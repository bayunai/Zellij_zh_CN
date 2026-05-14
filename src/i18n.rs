use zellij_tile::prelude::{CopyDestination, InputMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HintItem {
    pub key: &'static str,
    pub label: &'static str,
    pub priority: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HintGroup {
    pub title: &'static str,
    pub prefix: Option<&'static str>,
    pub items: &'static [HintItem],
}

pub fn mode_name(mode: InputMode) -> &'static str {
    match mode {
        InputMode::Normal => "普通",
        InputMode::Locked => "锁定",
        InputMode::Pane => "窗格",
        InputMode::Tab => "标签",
        InputMode::Resize => "调整",
        InputMode::Move => "移动",
        InputMode::Search => "搜索",
        InputMode::Scroll => "滚动",
        InputMode::Session => "会话",
        InputMode::Tmux => "Tmux",
        InputMode::Prompt => "提示",
        InputMode::EnterSearch => "输入搜索",
        InputMode::RenameTab => "重命名标签",
        InputMode::RenamePane => "重命名窗格",
    }
}

pub fn mode_hint(mode: InputMode) -> &'static str {
    match mode {
        InputMode::Normal => "Ctrl+p 窗格 | Ctrl+t 标签 | Ctrl+n 调整 | Ctrl+h 移动 | Ctrl+s 滚动 | Ctrl+o 会话 | Alt+n 新窗格 | Alt+f 浮动 | Alt+i/o 移标签 | Ctrl+q 退出",
        InputMode::Locked => "界面已锁定 | Ctrl+g 解锁",
        InputMode::Pane => "d 下分屏 | r 右分屏 | n 自动窗格 | s 堆叠 | x 关闭 | f 全屏 | z 框架 | e 浮动/嵌入 | w 显示浮动 | c 重命名 | p 切焦点 | hjkl/方向 切换",
        InputMode::Tab => "n 新建 | x 关闭 | r 重命名 | s 同步 | 1-9 跳转 | Tab 最近标签 | h/l/←→ 切换 | [/] 拆分 | b 拆出",
        InputMode::Resize => "+/= 放大 | - 缩小 | hjkl/方向 调整 | Ctrl+n 返回",
        InputMode::Move => "n/Tab 下个位置 | p 上个位置 | hjkl/方向 移动 | Ctrl+h 返回",
        InputMode::Search | InputMode::EnterSearch => "s 输入关键词 | Enter 确认 | n 下一个 | p 上一个 | c 大小写 | w 全词 | o 循环 | Esc 返回",
        InputMode::Scroll => "s 搜索 | e 编辑回滚 | j/k/方向 滚动 | Ctrl+u/d 半页 | PageUp/Down 翻页 | Ctrl+c 底部",
        InputMode::Session => "d 分离 | w 会话管理 | p 插件管理 | c 配置 | l 布局 | a 关于 | Ctrl+o 返回",
        InputMode::Tmux => "c 新标签 | % 右分屏 | \" 下分屏 | , 重命名 | [ 滚动 | n/p 标签 | z 全屏",
        InputMode::RenameTab => "输入新标签名 | Enter 确认 | Esc 取消",
        InputMode::RenamePane => "输入新窗格名 | Enter 确认 | Esc 取消",
        InputMode::Prompt => "确认操作 | y 是 | n 否 | Esc 取消",
    }
}

const NORMAL_GROUPS: &[HintGroup] = &[
    HintGroup {
        title: "Ctrl",
        prefix: Some("Ctrl"),
        items: &[
            HintItem {
                key: "p",
                label: "窗格",
                priority: 1,
            },
            HintItem {
                key: "t",
                label: "标签",
                priority: 1,
            },
            HintItem {
                key: "n",
                label: "调整",
                priority: 1,
            },
            HintItem {
                key: "h",
                label: "移动",
                priority: 1,
            },
            HintItem {
                key: "s",
                label: "滚动",
                priority: 2,
            },
            HintItem {
                key: "o",
                label: "会话",
                priority: 2,
            },
            HintItem {
                key: "q",
                label: "退出",
                priority: 1,
            },
        ],
    },
    HintGroup {
        title: "Alt",
        prefix: Some("Alt"),
        items: &[
            HintItem {
                key: "n",
                label: "新窗格",
                priority: 1,
            },
            HintItem {
                key: "f",
                label: "浮动",
                priority: 2,
            },
            HintItem {
                key: "i/o",
                label: "移标签",
                priority: 3,
            },
        ],
    },
];

const LOCKED_GROUPS: &[HintGroup] = &[HintGroup {
    title: "锁定",
    prefix: None,
    items: &[HintItem {
        key: "Ctrl+g",
        label: "解锁",
        priority: 1,
    }],
}];

const PANE_GROUPS: &[HintGroup] = &[
    HintGroup {
        title: "窗格",
        prefix: None,
        items: &[
            HintItem {
                key: "d",
                label: "下分屏",
                priority: 1,
            },
            HintItem {
                key: "r",
                label: "右分屏",
                priority: 1,
            },
            HintItem {
                key: "n",
                label: "自动窗格",
                priority: 1,
            },
            HintItem {
                key: "s",
                label: "堆叠",
                priority: 2,
            },
            HintItem {
                key: "x",
                label: "关闭",
                priority: 1,
            },
            HintItem {
                key: "f",
                label: "全屏",
                priority: 1,
            },
            HintItem {
                key: "z",
                label: "框架",
                priority: 2,
            },
        ],
    },
    HintGroup {
        title: "更多",
        prefix: None,
        items: &[
            HintItem {
                key: "e",
                label: "浮动/嵌入",
                priority: 2,
            },
            HintItem {
                key: "w",
                label: "显示浮动",
                priority: 3,
            },
            HintItem {
                key: "c",
                label: "重命名",
                priority: 2,
            },
            HintItem {
                key: "p",
                label: "切焦点",
                priority: 3,
            },
            HintItem {
                key: "hjkl/方向",
                label: "切换",
                priority: 2,
            },
        ],
    },
];

const TAB_GROUPS: &[HintGroup] = &[
    HintGroup {
        title: "标签",
        prefix: None,
        items: &[
            HintItem {
                key: "n",
                label: "新建",
                priority: 1,
            },
            HintItem {
                key: "x",
                label: "关闭",
                priority: 1,
            },
            HintItem {
                key: "r",
                label: "重命名",
                priority: 1,
            },
            HintItem {
                key: "s",
                label: "同步",
                priority: 1,
            },
            HintItem {
                key: "1-9",
                label: "跳转",
                priority: 1,
            },
            HintItem {
                key: "Tab",
                label: "最近标签",
                priority: 2,
            },
        ],
    },
    HintGroup {
        title: "整理",
        prefix: None,
        items: &[
            HintItem {
                key: "h/l/←→",
                label: "切换",
                priority: 2,
            },
            HintItem {
                key: "[/]",
                label: "拆分",
                priority: 1,
            },
            HintItem {
                key: "b",
                label: "拆出",
                priority: 2,
            },
        ],
    },
];

const RESIZE_GROUPS: &[HintGroup] = &[HintGroup {
    title: "调整",
    prefix: None,
    items: &[
        HintItem {
            key: "+/=",
            label: "放大",
            priority: 1,
        },
        HintItem {
            key: "-",
            label: "缩小",
            priority: 1,
        },
        HintItem {
            key: "hjkl/方向",
            label: "调边界",
            priority: 1,
        },
        HintItem {
            key: "Ctrl+n",
            label: "返回",
            priority: 2,
        },
    ],
}];

const MOVE_GROUPS: &[HintGroup] = &[HintGroup {
    title: "移动",
    prefix: None,
    items: &[
        HintItem {
            key: "hjkl/方向",
            label: "移动",
            priority: 1,
        },
        HintItem {
            key: "n/Tab",
            label: "下个位置",
            priority: 1,
        },
        HintItem {
            key: "p",
            label: "上个位置",
            priority: 2,
        },
        HintItem {
            key: "Ctrl+h",
            label: "返回",
            priority: 2,
        },
    ],
}];

const SCROLL_GROUPS: &[HintGroup] = &[
    HintGroup {
        title: "滚动",
        prefix: None,
        items: &[
            HintItem {
                key: "j/k/方向",
                label: "滚动",
                priority: 1,
            },
            HintItem {
                key: "PageUp/Down",
                label: "翻页",
                priority: 1,
            },
            HintItem {
                key: "Ctrl+b/f",
                label: "整页",
                priority: 2,
            },
            HintItem {
                key: "u/d",
                label: "半页",
                priority: 2,
            },
        ],
    },
    HintGroup {
        title: "操作",
        prefix: None,
        items: &[
            HintItem {
                key: "s",
                label: "搜索",
                priority: 1,
            },
            HintItem {
                key: "e",
                label: "编辑回滚",
                priority: 2,
            },
            HintItem {
                key: "Ctrl+c",
                label: "到底部",
                priority: 2,
            },
        ],
    },
];

const SEARCH_GROUPS: &[HintGroup] = &[HintGroup {
    title: "搜索",
    prefix: None,
    items: &[
        HintItem {
            key: "s",
            label: "输入关键词",
            priority: 1,
        },
        HintItem {
            key: "Enter",
            label: "确认",
            priority: 1,
        },
        HintItem {
            key: "n/p",
            label: "下/上一个",
            priority: 1,
        },
        HintItem {
            key: "c",
            label: "大小写",
            priority: 2,
        },
        HintItem {
            key: "w",
            label: "全词",
            priority: 2,
        },
        HintItem {
            key: "o",
            label: "循环",
            priority: 3,
        },
    ],
}];

const SESSION_GROUPS: &[HintGroup] = &[HintGroup {
    title: "会话",
    prefix: None,
    items: &[
        HintItem {
            key: "d",
            label: "分离",
            priority: 1,
        },
        HintItem {
            key: "w",
            label: "会话管理",
            priority: 1,
        },
        HintItem {
            key: "p",
            label: "插件管理",
            priority: 1,
        },
        HintItem {
            key: "c",
            label: "配置",
            priority: 2,
        },
        HintItem {
            key: "l",
            label: "布局",
            priority: 2,
        },
        HintItem {
            key: "a",
            label: "关于",
            priority: 3,
        },
    ],
}];

const TMUX_GROUPS: &[HintGroup] = &[HintGroup {
    title: "Tmux",
    prefix: None,
    items: &[
        HintItem {
            key: "c",
            label: "新标签",
            priority: 1,
        },
        HintItem {
            key: "%",
            label: "右分屏",
            priority: 1,
        },
        HintItem {
            key: "\"",
            label: "下分屏",
            priority: 1,
        },
        HintItem {
            key: ",",
            label: "重命名",
            priority: 2,
        },
        HintItem {
            key: "[",
            label: "滚动",
            priority: 2,
        },
        HintItem {
            key: "n/p",
            label: "标签",
            priority: 2,
        },
        HintItem {
            key: "z",
            label: "全屏",
            priority: 2,
        },
    ],
}];

const RENAME_TAB_GROUPS: &[HintGroup] = &[HintGroup {
    title: "重命名标签",
    prefix: None,
    items: &[
        HintItem {
            key: "Enter",
            label: "确认",
            priority: 1,
        },
        HintItem {
            key: "Esc",
            label: "取消",
            priority: 1,
        },
        HintItem {
            key: "Ctrl+c",
            label: "返回普通",
            priority: 2,
        },
    ],
}];

const RENAME_PANE_GROUPS: &[HintGroup] = &[HintGroup {
    title: "重命名窗格",
    prefix: None,
    items: &[
        HintItem {
            key: "Enter",
            label: "确认",
            priority: 1,
        },
        HintItem {
            key: "Esc",
            label: "取消",
            priority: 1,
        },
        HintItem {
            key: "Ctrl+c",
            label: "返回普通",
            priority: 2,
        },
    ],
}];

const PROMPT_GROUPS: &[HintGroup] = &[HintGroup {
    title: "确认",
    prefix: None,
    items: &[
        HintItem {
            key: "y",
            label: "同意",
            priority: 1,
        },
        HintItem {
            key: "n",
            label: "拒绝",
            priority: 1,
        },
        HintItem {
            key: "Esc",
            label: "取消",
            priority: 2,
        },
    ],
}];

pub fn hint_groups(mode: InputMode) -> &'static [HintGroup] {
    match mode {
        InputMode::Normal => NORMAL_GROUPS,
        InputMode::Locked => LOCKED_GROUPS,
        InputMode::Pane => PANE_GROUPS,
        InputMode::Tab => TAB_GROUPS,
        InputMode::Resize => RESIZE_GROUPS,
        InputMode::Move => MOVE_GROUPS,
        InputMode::Scroll => SCROLL_GROUPS,
        InputMode::EnterSearch | InputMode::Search => SEARCH_GROUPS,
        InputMode::Session => SESSION_GROUPS,
        InputMode::Tmux => TMUX_GROUPS,
        InputMode::RenameTab => RENAME_TAB_GROUPS,
        InputMode::RenamePane => RENAME_PANE_GROUPS,
        InputMode::Prompt => PROMPT_GROUPS,
    }
}

pub fn copy_hint(destination: CopyDestination) -> &'static str {
    match destination {
        CopyDestination::Command => "已复制到命令",
        CopyDestination::Primary => "已复制到主选择区",
        CopyDestination::System => "已复制到系统剪贴板",
    }
}

pub fn clipboard_error() -> &'static str {
    "系统剪贴板不可用"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translates_major_input_modes() {
        assert_eq!(mode_name(InputMode::Normal), "普通");
        assert_eq!(mode_name(InputMode::Locked), "锁定");
        assert_eq!(mode_name(InputMode::Pane), "窗格");
        assert_eq!(mode_name(InputMode::Tab), "标签");
        assert_eq!(mode_name(InputMode::Resize), "调整");
        assert_eq!(mode_name(InputMode::Search), "搜索");
    }

    #[test]
    fn returns_chinese_hint_for_common_modes() {
        assert!(mode_hint(InputMode::Normal).contains("窗格"));
        assert!(mode_hint(InputMode::Tab).contains("n 新建"));
    }

    #[test]
    fn pane_mode_hint_includes_split_and_layout_shortcuts() {
        let hint = mode_hint(InputMode::Pane);
        assert!(hint.contains("d 下分屏"));
        assert!(hint.contains("r 右分屏"));
        assert!(hint.contains("n 自动窗格"));
    }

    #[test]
    fn tab_mode_hint_includes_navigation_and_split_shortcuts() {
        let hint = mode_hint(InputMode::Tab);
        assert!(hint.contains("1-9 跳转"));
        assert!(hint.contains("s 同步"));
        assert!(hint.contains("[/] 拆分"));
    }

    #[test]
    fn normal_mode_hint_includes_global_shortcuts() {
        let hint = mode_hint(InputMode::Normal);
        assert!(hint.contains("Ctrl+p 窗格"));
        assert!(hint.contains("Ctrl+h 移动"));
        assert!(hint.contains("Alt+n 新窗格"));
        assert!(hint.contains("Ctrl+q 退出"));
    }

    #[test]
    fn normal_hint_groups_use_ctrl_prefix_and_include_move() {
        let groups = hint_groups(InputMode::Normal);
        assert_eq!(groups[0].prefix, Some("Ctrl"));
        assert!(groups[0]
            .items
            .iter()
            .any(|item| item.key == "h" && item.label == "移动"));
    }
}
