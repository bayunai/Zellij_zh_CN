use zellij_tile::prelude::PaneInfo;

/// 只统计用户可操作的终端窗格，排除插件栏（tab/status）等 `is_plugin` 窗格。
pub fn content_pane_count(panes: &[PaneInfo]) -> usize {
    panes.iter().filter(|pane| counts_as_content_pane(pane)).count()
}

fn counts_as_content_pane(pane: &PaneInfo) -> bool {
    !pane.is_suppressed && !pane.is_plugin
}

#[cfg(test)]
mod tests {
    use super::*;

    fn terminal_pane() -> PaneInfo {
        PaneInfo {
            is_plugin: false,
            is_suppressed: false,
            ..Default::default()
        }
    }

    fn plugin_pane() -> PaneInfo {
        PaneInfo {
            is_plugin: true,
            is_suppressed: false,
            is_selectable: false,
            ..Default::default()
        }
    }

    #[test]
    fn excludes_plugin_panes() {
        let panes = vec![plugin_pane(), terminal_pane(), plugin_pane()];
        assert_eq!(content_pane_count(&panes), 1);
    }

    #[test]
    fn counts_multiple_terminals() {
        let panes = vec![terminal_pane(), terminal_pane()];
        assert_eq!(content_pane_count(&panes), 2);
    }

    #[test]
    fn ignores_suppressed_panes() {
        let mut suppressed = terminal_pane();
        suppressed.is_suppressed = true;
        let panes = vec![terminal_pane(), suppressed];
        assert_eq!(content_pane_count(&panes), 1);
    }
}
