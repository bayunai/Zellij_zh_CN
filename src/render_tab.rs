use crate::style::{
    opaque_filler, StyledLine, EMPHASIS_ACCENT, EMPHASIS_CYAN, RIBBON_CHEVRON_PAD,
    RIBBON_TEXT_TRAIL,
};
use crate::width::display_width;
use zellij_tile::ui_components::{serialize_ribbon_with_coordinates, Text};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TabView {
    pub position: usize,
    pub name: String,
    pub active: bool,
    pub pane_count: usize,
}

pub fn render_tab_bar(session_name: Option<&str>, tabs: &[TabView], cols: usize) -> String {
    if cols == 0 {
        return String::new();
    }

    let session = session_name
        .filter(|name| !name.is_empty())
        .unwrap_or("未命名会话");

    let mut prefix = StyledLine::new();
    prefix.push(" ");
    prefix.push_styled("会话", Some(EMPHASIS_CYAN));
    prefix.push(&format!(" {session} │ "));
    let prefix_w = prefix.visible_width().min(cols);

    let mut output = prefix.into_opaque_line(prefix_w, 0);
    let mut x = prefix_w;

    for tab in tabs {
        if x >= cols {
            break;
        }
        let remaining = cols - x;
        let (label, width) = tab_ribbon_label(tab, remaining);
        if width == 0 {
            break;
        }
        let ribbon = tab_ribbon_text(tab, &label);
        output.push_str(&serialize_ribbon_with_coordinates(
            &ribbon, x, 0, Some(width), Some(1),
        ));
        x += width;
    }

    if x < cols {
        output.push_str(&opaque_filler(x, cols - x, 0));
    }

    output
}

fn tab_ribbon_label(tab: &TabView, max_width: usize) -> (String, usize) {
    if max_width == 0 {
        return (String::new(), 0);
    }

    let tab_name = if tab.name.is_empty() {
        "未命名".to_owned()
    } else {
        tab.name.clone()
    };
    let index = (tab.position + 1).to_string();

    let full = if tab.active {
        format!("● {index} {tab_name} · {}窗格 ", tab.pane_count)
    } else {
        format!("○ {index} {tab_name} ")
    };

    let text_budget =
        max_width.saturating_sub(RIBBON_CHEVRON_PAD + RIBBON_TEXT_TRAIL);
    if text_budget == 0 {
        return (String::new(), 0);
    }

    let content = crate::width::truncate_to_width(&full, text_budget);
    let label = format!("{content}{}", " ".repeat(RIBBON_TEXT_TRAIL));
    let width = (display_width(&label) + RIBBON_CHEVRON_PAD).min(max_width);
    (label, width)
}

fn tab_ribbon_text(tab: &TabView, label: &str) -> Text {
    let index = (tab.position + 1).to_string();
    let mut text = Text::new(label.to_owned());

    if tab.active {
        text = text.selected();
        text = text.color_substring(EMPHASIS_ACCENT, "●");
        text = text.color_substring(EMPHASIS_ACCENT, &index);
    }

    text
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::extract_tab_bar_plain;
    use crate::width::visible_width;

    #[test]
    fn renders_session_and_active_tab() {
        let tabs = vec![
            TabView {
                position: 0,
                name: "编辑".into(),
                active: true,
                pane_count: 2,
            },
            TabView {
                position: 1,
                name: "测试".into(),
                active: false,
                pane_count: 1,
            },
        ];
        let line = render_tab_bar(Some("项目"), &tabs, 80);
        let plain = extract_tab_bar_plain(&line);
        assert!(plain.contains("会话 项目"));
        assert!(plain.contains("● 1 编辑 · 2窗格"));
        assert!(plain.contains("○ 2 测试"));
    }

    #[test]
    fn truncates_long_tab_names_to_terminal_width() {
        let tabs = vec![TabView {
            position: 0,
            name: "非常非常长的中文标签名称".into(),
            active: true,
            pane_count: 3,
        }];
        let line = render_tab_bar(Some("项目"), &tabs, 20);
        assert!(visible_width(&extract_tab_bar_plain(&line)) <= 20);
    }

    #[test]
    fn renders_ribbon_tab_bar() {
        let tabs = vec![
            TabView {
                position: 0,
                name: "编辑".into(),
                active: true,
                pane_count: 2,
            },
            TabView {
                position: 1,
                name: "测试".into(),
                active: false,
                pane_count: 1,
            },
        ];

        let line = render_tab_bar(Some("项目"), &tabs, 80);
        assert!(line.contains("Pztext") && line.contains(";z"));
        assert!(line.contains("Pzribbon"));
        assert!(line.contains(";x"), "active tab ribbon should be selected");
        let plain = extract_tab_bar_plain(&line);
        assert!(plain.contains("会话 项目"));
        assert!(plain.contains("● 1 编辑 · 2窗格"));
        assert!(plain.contains("○ 2 测试"));
    }

    #[test]
    fn tab_ribbon_reserves_space_for_chevron() {
        let tab = TabView {
            position: 0,
            name: "编辑".into(),
            active: true,
            pane_count: 2,
        };

        let (label, width) = tab_ribbon_label(&tab, 80);
        assert!(width > crate::width::display_width(&label));
    }
}
