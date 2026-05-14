use crate::i18n::{clipboard_error, copy_hint, hint_groups, mode_name, HintItem};
use crate::render_tab::TabView;
use crate::width::{strip_ansi, truncate_to_width, visible_width};
use zellij_tile::prelude::{CopyDestination, InputMode};

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

    let active_tab_summary = active_tab
        .map(|tab| format!("标签 {} {}", tab.position + 1, tab.name))
        .unwrap_or_else(|| "无活动标签".to_owned());
    let notice_text = match notice {
        Some(StatusNotice::Copied(destination)) => format!(" │ {}", copy_hint(destination)),
        Some(StatusNotice::ClipboardError) => format!(" │ {}", clipboard_error()),
        None => String::new(),
    };
    let pane_summary = active_tab
        .map(|tab| format!(" │ {}窗格", tab.pane_count))
        .unwrap_or_default();
    let first = truncate_to_width(
        &format!(
            " {} {} │ {} {}{}{}",
            paint("◆", "36"),
            paint(mode_name(mode), mode_color(mode)),
            paint("◉", "33"),
            active_tab_summary,
            pane_summary,
            notice_text
        ),
        cols,
    );

    if rows == 1 {
        let compact = format!(
            "{} │ {}",
            paint(mode_name(mode), mode_color(mode)),
            render_hint_line(mode, cols)
        );
        return truncate_ansi_to_width(&compact, cols);
    }

    let second = render_hint_line(mode, cols);
    format!("{}\n{}", first, second)
}

pub fn render_permission_pending(cols: usize) -> String {
    truncate_to_width(" 等待授权读取 Zellij 状态，按 y 授权后显示中文界面", cols)
}

pub fn active_tab(tabs: &[TabView]) -> Option<&TabView> {
    tabs.iter().find(|tab| tab.active)
}

fn render_hint_line(mode: InputMode, cols: usize) -> String {
    if cols == 0 {
        return String::new();
    }

    for max_priority in [1, 2, 3] {
        let line = render_hint_line_with_priority(mode, max_priority);
        if visible_width(&line) <= cols {
            return line;
        }
    }

    truncate_ansi_to_width(&render_hint_line_with_priority(mode, 3), cols)
}

fn render_hint_line_with_priority(mode: InputMode, max_priority: u8) -> String {
    hint_groups(mode)
        .iter()
        .filter_map(|group| {
            let items = group
                .items
                .iter()
                .filter(|item| item.priority <= max_priority)
                .map(render_hint_item)
                .collect::<Vec<_>>();
            if items.is_empty() {
                None
            } else if let Some(prefix) = group.prefix {
                Some(format!(
                    " {} {}  {}",
                    prefix_symbol(prefix),
                    paint(prefix, "36"),
                    items.join("  ")
                ))
            } else {
                Some(format!(
                    " {}  {}",
                    paint(group.title, "36"),
                    items.join("  ")
                ))
            }
        })
        .collect::<Vec<_>>()
        .join(" │")
}

fn render_hint_item(item: &HintItem) -> String {
    format!("{} {}", paint(item.key, "1;33"), item.label)
}

fn prefix_symbol(prefix: &str) -> &'static str {
    match prefix {
        "Ctrl" => "⌃",
        "Alt" => "⌥",
        _ => "•",
    }
}

fn mode_color(mode: InputMode) -> &'static str {
    match mode {
        InputMode::Normal => "1;37",
        InputMode::Pane | InputMode::Tab | InputMode::Move | InputMode::Resize => "1;32",
        InputMode::Scroll | InputMode::Search | InputMode::EnterSearch => "1;34",
        InputMode::Locked | InputMode::Prompt => "1;31",
        _ => "1;35",
    }
}

fn paint(text: &str, code: &str) -> String {
    format!("\x1b[{code}m{text}\x1b[0m")
}

fn truncate_ansi_to_width(text: &str, max_width: usize) -> String {
    if visible_width(text) <= max_width {
        return text.to_owned();
    }
    truncate_to_width(&strip_ansi(text), max_width)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn renders_two_line_status_for_normal_mode() {
        let tab = TabView {
            position: 0,
            name: "开发".into(),
            active: true,
            pane_count: 2,
        };
        let status = render_status_bar(InputMode::Normal, Some(&tab), None, 2, 80);
        let plain = strip_ansi(&status);
        assert!(plain.contains("◆ 普通"));
        assert!(plain.contains("⌃ Ctrl  p 窗格"));
        assert_eq!(status.lines().count(), 2);
    }

    #[test]
    fn renders_compact_status_for_one_row() {
        let status = render_status_bar(InputMode::Tab, None, None, 1, 28);
        assert!(visible_width(&status) <= 28);
        assert!(strip_ansi(&status).contains("标签"));
    }

    #[test]
    fn renders_search_hint_in_chinese() {
        let status = render_status_bar(InputMode::Search, None, None, 2, 80);
        let plain = strip_ansi(&status);
        assert!(plain.contains("搜索"));
        assert!(plain.contains("关键词"));
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
            for line in status.lines() {
                assert!(visible_width(line) <= cols, "line exceeded {cols}: {line}");
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
        let plain = strip_ansi(&status);
        assert!(plain.contains("◆ 窗格"));
        assert!(plain.contains("窗格  d 下分屏"));
        assert!(plain.contains("r 右分屏"));
    }

    #[test]
    fn normal_status_uses_ctrl_prefix_and_colored_output() {
        let status = render_status_bar(InputMode::Normal, None, None, 2, 120);
        let plain = strip_ansi(&status);
        assert!(status.contains("\x1b["));
        assert!(plain.contains("⌃ Ctrl  p 窗格"));
        assert!(plain.contains("h 移动"));
        assert!(plain.contains("⌥ Alt  n 新窗格"));
    }
}
