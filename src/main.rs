use std::collections::BTreeMap;
use zellij_cn_ui::render_status::{
    active_tab, render_permission_pending, render_status_bar, StatusNotice,
};
use zellij_cn_ui::render_tab::{render_tab_bar, TabView};
use zellij_tile::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BarKind {
    Tab,
    Status,
}

impl Default for BarKind {
    fn default() -> Self {
        Self::Status
    }
}

#[derive(Default)]
struct State {
    bar: BarKind,
    mode_info: Option<ModeInfo>,
    tabs: Vec<TabView>,
    notice: Option<StatusNotice>,
    permission_granted: bool,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, configuration: BTreeMap<String, String>) {
        self.bar = match configuration.get("bar").map(String::as_str) {
            Some("tab") => BarKind::Tab,
            _ => BarKind::Status,
        };
        set_selectable(false);
        subscribe(&[
            EventType::ModeUpdate,
            EventType::TabUpdate,
            EventType::PaneUpdate,
            EventType::CopyToClipboard,
            EventType::SystemClipboardFailure,
            EventType::InputReceived,
            EventType::PermissionRequestResult,
        ]);
        request_permission(&[PermissionType::ReadApplicationState]);
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::PermissionRequestResult(PermissionStatus::Granted) => {
                self.permission_granted = true;
                true
            }
            Event::PermissionRequestResult(PermissionStatus::Denied) => {
                self.permission_granted = false;
                true
            }
            Event::ModeUpdate(mode_info) => {
                self.mode_info = Some(mode_info);
                true
            }
            Event::TabUpdate(tabs) => {
                self.tabs = tabs
                    .into_iter()
                    .map(|tab| TabView {
                        position: tab.position,
                        name: tab.name,
                        active: tab.active,
                        pane_count: tab.selectable_tiled_panes_count
                            + tab.selectable_floating_panes_count,
                    })
                    .collect();
                true
            }
            Event::CopyToClipboard(destination) => {
                self.notice = Some(StatusNotice::Copied(destination));
                true
            }
            Event::SystemClipboardFailure => {
                self.notice = Some(StatusNotice::ClipboardError);
                true
            }
            Event::InputReceived => {
                if self.notice.take().is_some() {
                    return true;
                }
                false
            }
            Event::PaneUpdate(_) => true,
            _ => false,
        }
    }

    fn render(&mut self, rows: usize, cols: usize) {
        match self.bar {
            BarKind::Tab => {
                let session_name = self
                    .mode_info
                    .as_ref()
                    .and_then(|mode_info| mode_info.session_name.as_deref());
                print!("{}", render_tab_bar(session_name, &self.tabs, cols));
            }
            BarKind::Status => {
                if !self.permission_granted && self.mode_info.is_none() {
                    print!("{}", render_permission_pending(cols));
                    return;
                }
                let mode = self
                    .mode_info
                    .as_ref()
                    .map(|mode_info| mode_info.mode)
                    .unwrap_or(InputMode::Normal);
                print!(
                    "{}",
                    render_status_bar(mode, active_tab(&self.tabs), self.notice, rows, cols)
                );
            }
        }
    }
}
