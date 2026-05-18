use crate::width::{display_width, truncate_to_width};
use zellij_tile::ui_components::{serialize_ribbon_with_coordinates, serialize_text_with_coordinates, Text};

/// Zellij 主题 emphasis_0：高亮（快捷键、活动标记）
pub const EMPHASIS_ACCENT: usize = 0;
/// emphasis_1：青色标签（Ctrl / 会话 / ◆）
pub const EMPHASIS_CYAN: usize = 1;
/// emphasis_2：绿色（模式名等）
pub const EMPHASIS_GREEN: usize = 2;
/// emphasis_3：蓝紫色
pub const EMPHASIS_VIOLET: usize = 3;
/// emphasis_4：暗淡（非活动标签）
pub const EMPHASIS_DIM: usize = 4;

/// ribbon 箭头在布局上占用的额外列宽。
pub const RIBBON_CHEVRON_PAD: usize = 4;

/// ribbon 文本末尾留白，避免最后一个字被 chevron 盖住。
pub const RIBBON_TEXT_TRAIL: usize = 2;

/// 模式 ribbon 至少保留的列宽（` ◆ 普通 ` / ` ◆ 锁定 ` 等）。
pub const RIBBON_MODE_MIN_WIDTH: usize = 12;

/// 带字符区间着色的行，最终输出 Zellij 不透明 UI 文本（整行背景）。
#[derive(Debug, Default, Clone)]
pub struct StyledLine {
    pub(crate) content: String,
    pub(crate) ranges: Vec<(usize, usize, usize)>,
}

impl StyledLine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, text: &str) {
        self.push_styled(text, None);
    }

    pub fn push_styled(&mut self, text: &str, level: Option<usize>) {
        let start = self.char_len();
        self.content.push_str(text);
        let end = self.char_len();
        if let Some(level) = level {
            self.ranges.push((level, start, end));
        }
    }

    pub fn visible_width(&self) -> usize {
        display_width(&self.content)
    }

    pub fn plain(&self) -> &str {
        &self.content
    }

    pub fn char_len(&self) -> usize {
        self.content.chars().count()
    }

    pub fn push_range(&mut self, level: usize, start: usize, end: usize) {
        if start < end {
            self.ranges.push((level, start, end));
        }
    }

    pub fn append_styled_line(&mut self, other: &StyledLine) {
        let offset = self.char_len();
        self.content.push_str(other.plain());
        for &(level, start, end) in &other.ranges {
            self.push_range(level, offset + start, offset + end);
        }
    }

    pub fn into_opaque_line(self, cols: usize, y: usize) -> String {
        if cols == 0 {
            return String::new();
        }

        let content = truncate_to_width(&self.content, cols);
        let char_len = content.chars().count();
        let padded = if display_width(&content) < cols {
            pad_plain_to_width(&content, cols)
        } else {
            content
        };

        let mut text = Text::new(padded).opaque();
        for (level, start, end) in self.ranges {
            if start >= char_len {
                continue;
            }
            let end = end.min(char_len);
            if start < end {
                text = text.color_range(level, start..end);
            }
        }

        serialize_text_with_coordinates(&text, 0, y, Some(cols), Some(1))
    }

    /// 转为 ribbon 片段（带主题背景），返回 Text 与占用列宽（含箭头预留）。
    pub fn into_ribbon(self, selected: bool, max_width: usize) -> (Text, usize) {
        let text_budget = max_width.saturating_sub(RIBBON_CHEVRON_PAD);
        let content = truncate_to_width(
            &self.content,
            text_budget.saturating_sub(RIBBON_TEXT_TRAIL),
        );
        let char_len = content.chars().count();
        let padded = pad_plain_to_width(
            &content,
            display_width(&content) + RIBBON_TEXT_TRAIL,
        );
        let content_width = display_width(&padded);
        let mut text = Text::new(padded);
        if selected {
            text = text.selected();
        }
        for (level, start, end) in self.ranges {
            if start >= char_len {
                continue;
            }
            let end = end.min(char_len);
            if start < end {
                text = text.color_range(level, start..end);
            }
        }
        let render_width = (content_width + RIBBON_CHEVRON_PAD).min(max_width);
        (text, render_width)
    }

    pub fn into_ribbon_min(self, selected: bool, max_width: usize, min_width: usize) -> (Text, usize) {
        let (text, width) = self.into_ribbon(selected, max_width);
        let width = width.max(min_width.min(max_width));
        (text, width)
    }
}

/// 在一行内横向排列多个 ribbon，末尾用不透明填充补齐。
pub fn render_ribbon_row(segments: &[(StyledLine, bool)], cols: usize, y: usize) -> String {
    if cols == 0 {
        return String::new();
    }

    let mut output = String::new();
    let mut x = 0;

    for (segment, selected) in segments {
        if x >= cols {
            break;
        }
        let remaining = cols - x;
        let (ribbon, width) = segment.clone().into_ribbon(*selected, remaining);
        if width == 0 {
            break;
        }
        output.push_str(&serialize_ribbon_with_coordinates(
            &ribbon, x, y, Some(width), Some(1),
        ));
        x += width;
    }

    if x < cols {
        output.push_str(&opaque_filler(x, cols - x, y));
    }

    output
}

fn pad_plain_to_width(text: &str, target_width: usize) -> String {
    let width = display_width(text);
    if width >= target_width {
        return text.to_owned();
    }
    format!("{text}{}", " ".repeat(target_width - width))
}

/// 在指定列位置填充不透明背景（用于 ribbon 行尾补齐）。
pub fn opaque_filler(x: usize, width: usize, y: usize) -> String {
    if width == 0 {
        return String::new();
    }
    let text = Text::new(" ".repeat(width)).opaque();
    serialize_text_with_coordinates(&text, x, y, Some(width), Some(1))
}

/// 将已有纯文本行渲染为不透明 UI 行（无额外着色）。
pub fn opaque_plain_line(line: &str, cols: usize, y: usize) -> String {
    if cols == 0 {
        return String::new();
    }
    let truncated = truncate_to_width(line, cols);
    let padded = if display_width(&truncated) < cols {
        pad_plain_to_width(&truncated, cols)
    } else {
        truncated
    };
    let text = Text::new(padded).opaque();
    serialize_text_with_coordinates(&text, 0, y, Some(cols), Some(1))
}

/// 从 DCS 载荷解码可见文本（仅测试用）。
#[cfg(test)]
pub fn decode_ui_payload(payload: &str) -> String {
    let mut payload = payload.trim_end_matches("\u{1b}\\");
    if let Some(rest) = payload.strip_prefix('z') {
        payload = rest;
    }
    if let Some(rest) = payload.strip_prefix('x') {
        payload = rest;
    }
    let byte_part = payload.rsplit('$').next().unwrap_or(payload);
    let bytes: Vec<u8> = byte_part
        .split(',')
        .filter_map(|b| b.trim().parse().ok())
        .collect();
    String::from_utf8(bytes).unwrap_or_default()
}

/// 从单行不透明文本 UI 序列中提取可见文本（仅测试用）。
#[cfg(test)]
pub fn extract_ui_plain(serialized: &str) -> String {
    let Some(payload) = serialized.split(';').nth(2) else {
        return String::new();
    };
    decode_ui_payload(payload)
}

/// 从 UI 输出（text + ribbon）提取全部可见文本（仅测试用）。
#[cfg(test)]
pub fn extract_ui_components_plain(serialized: &str) -> String {
    let mut plain = String::new();
    for tag in ["Pztext;", "Pzribbon;"] {
        let mut search = 0;
        while let Some(pos) = serialized[search..].find(tag) {
            let abs = search + pos;
            if let Some(payload) = serialized[abs..].split(';').nth(2) {
                let payload = payload.split("\u{1b}P").next().unwrap_or(payload);
                plain.push_str(&decode_ui_payload(payload));
            }
            search = abs + tag.len();
        }
    }
    plain
}

/// 从标签栏输出提取可见文本（仅测试用）。
#[cfg(test)]
pub fn extract_tab_bar_plain(serialized: &str) -> String {
    extract_ui_components_plain(serialized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn styled_line_renders_opaque_ui() {
        let mut line = StyledLine::new();
        line.push_styled(" 会话", Some(EMPHASIS_CYAN));
        line.push(" test");
        let out = line.into_opaque_line(20, 0);
        assert!(out.contains("Pztext"));
        assert!(out.contains(";z"));
        assert_eq!(display_width(&extract_ui_plain(&out)), 20);
    }

    #[test]
    fn opaque_plain_line_fills_width() {
        let out = opaque_plain_line("普通", 10, 0);
        assert!(out.contains("Pztext"));
        assert_eq!(display_width(&extract_ui_plain(&out)), 10);
    }

    #[test]
    fn render_ribbon_row_outputs_ribbons() {
        let mut mode = StyledLine::new();
        mode.push_styled(" ◆ 普通 ", Some(EMPHASIS_CYAN));
        let out = render_ribbon_row(&[(mode, true)], 20, 0);
        assert!(out.contains("Pzribbon"));
        assert!(out.contains(";x"));
    }
}
