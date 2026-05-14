use crate::width::{strip_ansi, truncate_to_width, visible_width};

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
    let prefix = format!(" {} {} │ ", paint("会话", "36"), session);
    let mut line = truncate_ansi_to_width(&prefix, cols);

    for tab in tabs {
        if visible_width(&line) >= cols {
            break;
        }
        let marker = if tab.active {
            paint("●", "1;33")
        } else {
            paint("○", "2;37")
        };
        let index = if tab.active {
            paint(&(tab.position + 1).to_string(), "1;33")
        } else {
            paint(&(tab.position + 1).to_string(), "2;37")
        };
        let tab_name = if tab.name.is_empty() {
            "未命名"
        } else {
            &tab.name
        };
        let raw = if tab.active {
            format!(
                "{} {} {} · {}窗格  ",
                marker, index, tab_name, tab.pane_count
            )
        } else {
            format!("{} {} {}  ", marker, index, tab_name)
        };
        let remaining = cols.saturating_sub(visible_width(&line));
        line.push_str(&truncate_ansi_to_width(&raw, remaining));
    }

    truncate_ansi_to_width(&line, cols)
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
    use crate::width::{strip_ansi, visible_width};

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
        let plain = strip_ansi(&line);
        assert!(plain.contains("会话 项目"));
        assert!(plain.contains("● 1 编辑 · 2窗格"));
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
        assert!(visible_width(&line) <= 20);
    }

    #[test]
    fn renders_polished_active_and_inactive_markers() {
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
        let plain = strip_ansi(&line);
        assert!(line.contains("\x1b["));
        assert!(plain.contains("会话 项目"));
        assert!(plain.contains("● 1 编辑 · 2窗格"));
        assert!(plain.contains("○ 2 测试"));
    }
}
