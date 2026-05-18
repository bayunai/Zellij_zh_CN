use crate::i18n::{clipboard_error, copy_hint, hint_groups, mode_name, HintItem};
use crate::render_tab::TabView;
use crate::style::{
    opaque_filler, opaque_plain_line, StyledLine, RIBBON_MODE_MIN_WIDTH, EMPHASIS_ACCENT,
    EMPHASIS_CYAN, EMPHASIS_GREEN, EMPHASIS_VIOLET,
};
use crate::width::truncate_to_width;
use zellij_tile::prelude::{CopyDestination, InputMode};
use zellij_tile::ui_components::serialize_ribbon_with_coordinates;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StatusNotice {
    Copied(CopyDestination),
    ClipboardError,
}

pub fn render_status_bar(
    mode: InputMode,
    active_tab: Option<&TabView>,
    notice: Option<StatusNotice>,
    rows: usize,
    cols: usize,
) -> String {
    if cols == 0 || rows == 0 {
        return String::new();
    }

    if rows == 1 {
        return render_compact_status_bar(mode, cols);
    }

    format!(
        "{}{}",
        render_status_info_row(mode, active_tab, notice, cols, 0),
        render_status_hint_row(mode, cols, 1)
    )
}

pub fn render_permission_pending(cols: usize) -> String {
    opaque_plain_line(" 等待授权读取 Zellij 状态，按 y 授权后显示中文界面", cols, 0)
}

pub fn active_tab(tabs: &[TabView]) -> Option<&TabView> {
    tabs.iter().find(|tab| tab.active)
}

fn render_status_info_row(
    mode: InputMode,
    active_tab: Option<&TabView>,
    notice: Option<StatusNotice>,
    cols: usize,
    y: usize,
) -> String {
    let mut segments: Vec<StatusRibbonSegment> = Vec::new();

    let mode_seg = mode_status_segment(mode);
    segments.push((mode_seg, true, Some(mode)));

    let active_tab_summary = active_tab
        .map(|tab| format!("标签 {} {}", tab.position + 1, tab.name))
        .unwrap_or_else(|| "无活动标签".to_owned());
    let mut tab_seg = StyledLine::new();
    tab_seg.push(" ");
    tab_seg.push_styled("◉", Some(EMPHASIS_ACCENT));
    tab_seg.push(&format!(" {active_tab_summary} "));
    segments.push((tab_seg, tab_ribbon_selected(mode), None));

    if let Some(tab) = active_tab {
        let mut pane_seg = StyledLine::new();
        pane_seg.push(&format!(" {}窗格 ", tab.pane_count));
        segments.push((pane_seg, false, None));
    }

    if let Some(notice) = notice {
        let mut notice_seg = StyledLine::new();
        notice_seg.push(" ");
        match notice {
            StatusNotice::Copied(destination) => {
                notice_seg.push(&format!("{} ", copy_hint(destination)));
            }
            StatusNotice::ClipboardError => {
                notice_seg.push(&format!("{} ", clipboard_error()));
            }
        }
        segments.push((notice_seg, false, None));
    }

    render_status_ribbon_row(&segments, cols, y)
}

fn render_status_hint_row(mode: InputMode, cols: usize, y: usize) -> String {
    // 整行一条 ribbon，避免按 │ 拆分后末段宽度不足导致「退出」「新窗格」被裁切。
    let line = build_hint_line(mode, cols);
    render_status_ribbon_row(&[(line, false, None)], cols, y)
}

fn mode_status_segment(mode: InputMode) -> StyledLine {
    let mut seg = StyledLine::new();
    seg.push(" ");
    seg.push_styled("◆", Some(EMPHASIS_CYAN));
    seg.push(" ");
    seg.push_styled(mode_name(mode), Some(mode_emphasis(mode)));
    seg.push(" ");
    seg
}

fn render_compact_status_bar(mode: InputMode, cols: usize) -> String {
    let mut segments = vec![(mode_status_segment(mode), true, Some(mode))];
    let hint = build_hint_line(mode, cols);
    if !hint.plain().is_empty() {
        segments.push((hint, false, None));
    }

    render_status_ribbon_row(&segments, cols, 0)
}

/// 状态栏 ribbon 行：`(内容, 是否选中, 可选模式用于锁定/提示着色)`
type StatusRibbonSegment = (StyledLine, bool, Option<InputMode>);

fn render_status_ribbon_row(segments: &[StatusRibbonSegment], cols: usize, y: usize) -> String {
    if cols == 0 {
        return String::new();
    }

    let mut output = String::new();
    let mut x = 0;

    for (line, selected, accent_mode) in segments {
        if x >= cols {
            break;
        }
        let remaining = cols - x;
        let min_width = if accent_mode.is_some() {
            RIBBON_MODE_MIN_WIDTH
        } else {
            0
        };
        let (mut text, render_width) = line
            .clone()
            .into_ribbon_min(*selected, remaining, min_width);
        if matches!(
            accent_mode,
            Some(InputMode::Locked) | Some(InputMode::Prompt)
        ) {
            text = text.error_color_all();
        }
        if render_width == 0 {
            break;
        }
        output.push_str(&serialize_ribbon_with_coordinates(
            &text,
            x,
            y,
            Some(render_width),
            Some(1),
        ));
        x += render_width;
    }

    if x < cols {
        output.push_str(&opaque_filler(x, cols - x, y));
    }

    output
}

fn tab_ribbon_selected(mode: InputMode) -> bool {
    matches!(mode, InputMode::Normal)
}

fn build_hint_line(mode: InputMode, cols: usize) -> StyledLine {
    if cols == 0 {
        return StyledLine::new();
    }

    for max_priority in [1, 2, 3] {
        let line = build_hint_line_with_priority(mode, max_priority);
        if line.visible_width() <= cols {
            return line;
        }
    }

    let line = build_hint_line_with_priority(mode, 3);
    let plain = truncate_to_width(line.plain(), cols);
    StyledLine {
        content: plain,
        ranges: Vec::new(),
    }
}

fn build_hint_line_with_priority(mode: InputMode, max_priority: u8) -> StyledLine {
    let mut line = StyledLine::new();
    for group in hint_groups(mode) {
        let items: Vec<_> = group
            .items
            .iter()
            .filter(|item| item.priority <= max_priority)
            .collect();
        if items.is_empty() {
            continue;
        }
        if !line.plain().is_empty() {
            line.push(" │");
        }
        if let Some(prefix) = group.prefix {
            line.push(" ");
            line.push(prefix_symbol(prefix));
            line.push(" ");
            line.push_styled(prefix, Some(EMPHASIS_CYAN));
            line.push("  ");
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    line.push("  ");
                }
                append_hint_item(&mut line, item);
            }
        } else {
            line.push(" ");
            line.push_styled(group.title, Some(EMPHASIS_CYAN));
            line.push("  ");
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    line.push("  ");
                }
                append_hint_item(&mut line, item);
            }
        }
    }
    line
}

fn append_hint_item(line: &mut StyledLine, item: &HintItem) {
    line.push_styled(item.key, Some(EMPHASIS_ACCENT));
    line.push(&format!(" {}", item.label));
}

fn prefix_symbol(prefix: &str) -> &'static str {
    match prefix {
        "Ctrl" => "⌃",
        "Alt" => "⌥",
        _ => "•",
    }
}

fn mode_emphasis(mode: InputMode) -> usize {
    match mode {
        InputMode::Normal => EMPHASIS_ACCENT,
        InputMode::Pane | InputMode::Tab | InputMode::Move | InputMode::Resize => EMPHASIS_GREEN,
        InputMode::Scroll | InputMode::Search | InputMode::EnterSearch => EMPHASIS_CYAN,
        InputMode::Locked | InputMode::Prompt => EMPHASIS_ACCENT,
        _ => EMPHASIS_VIOLET,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::extract_ui_components_plain;
    fn status_rows(status: &str) -> Vec<String> {
        let mut rows = Vec::new();
        let mut search = 0;
        while let Some(pos) = status[search..]
            .find("Pzribbon;")
            .map(|p| search + p)
        {
            let y = status[pos..]
                .strip_prefix("Pzribbon;")
                .and_then(|s| s.split('/').nth(1))
                .and_then(|y| y.parse::<usize>().ok())
                .unwrap_or(rows.len());
            while rows.len() <= y {
                rows.push(String::new());
            }
            let chunk_end = status[pos..]
                .find("\u{1b}P")
                .map(|i| pos + i)
                .unwrap_or(status.len());
            rows[y].push_str(&extract_ui_components_plain(&status[pos..chunk_end]));
            search = chunk_end;
        }
        rows
    }

    #[test]
    fn renders_two_line_status_for_normal_mode() {
        let tab = TabView {
            position: 0,
            name: "开发".into(),
            active: true,
            pane_count: 2,
        };
        let status = render_status_bar(InputMode::Normal, Some(&tab), None, 2, 80);
        assert!(status.contains("Pzribbon"));
        let rows = status_rows(&status);
        assert!(rows.len() >= 2, "expected two ribbon rows: {rows:?}");
        assert!(rows[0].contains("◆ 普通"));
        assert!(rows[1].contains("⌃ Ctrl  p 窗格"));
    }

    #[test]
    fn renders_compact_status_for_one_row() {
        let status = render_status_bar(InputMode::Tab, None, None, 1, 80);
        assert!(status.contains("Pzribbon"));
        let plain = extract_ui_components_plain(&status);
        assert!(plain.contains("标签"));
    }

    #[test]
    fn renders_search_hint_in_chinese() {
        let status = render_status_bar(InputMode::Search, None, None, 2, 80);
        let rows = status_rows(&status);
        assert!(rows[1].contains("搜索"));
        assert!(rows[1].contains("关键词"));
    }

    #[test]
    fn status_bar_respects_common_terminal_widths() {
        let tab = TabView {
            position: 0,
            name: "很长的中文开发标签".into(),
            active: true,
            pane_count: 3,
        };

        for cols in [40, 80, 120] {
            let status = render_status_bar(InputMode::Pane, Some(&tab), None, 2, cols);
            for row in status_rows(&status) {
                assert!(
                    crate::width::display_width(&row) <= cols,
                    "row exceeded {cols}: {row}"
                );
            }
        }
    }

    #[test]
    fn status_bar_uses_compact_professional_grouping() {
        let tab = TabView {
            position: 0,
            name: "开发".into(),
            active: true,
            pane_count: 2,
        };
        let status = render_status_bar(InputMode::Pane, Some(&tab), None, 2, 100);
        let rows = status_rows(&status);
        assert!(rows[0].contains("◆ 窗格"));
        assert!(rows[1].contains("窗格  d 下分屏"));
        assert!(rows[1].contains("r 右分屏"));
    }

    #[test]
    fn normal_status_uses_ctrl_prefix_and_colored_output() {
        let status = render_status_bar(InputMode::Normal, None, None, 2, 120);
        let rows = status_rows(&status);
        assert!(status.contains("Pzribbon"));
        assert!(rows[1].contains("⌃ Ctrl  p 窗格"));
        assert!(rows[1].contains("h 移动"));
        assert!(rows[1].contains("q 退出"));
        assert!(rows[1].contains("⌥ Alt  n 新窗格"));
    }

    #[test]
    fn locked_mode_shows_lock_label_and_unlock_hint() {
        let status = render_status_bar(InputMode::Locked, None, None, 2, 80);
        let rows = status_rows(&status);
        assert!(rows[0].contains("◆ 锁定"));
        assert!(rows[1].contains("解锁"));
        assert!(rows[1].contains('g'));
    }

    #[test]
    fn status_bar_uses_ribbon_background() {
        let status = render_status_bar(InputMode::Normal, None, None, 2, 80);
        assert!(status.matches("Pzribbon").count() >= 2);
        assert!(status.contains(";x") || status.contains(";z"));
    }
}
