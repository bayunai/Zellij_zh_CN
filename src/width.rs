use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

pub fn display_width(text: &str) -> usize {
    UnicodeWidthStr::width(text)
}

pub fn truncate_to_width(text: &str, max_width: usize) -> String {
    if display_width(text) <= max_width {
        return text.to_owned();
    }
    if max_width == 0 {
        return String::new();
    }

    let ellipsis = "…";
    let ellipsis_width = display_width(ellipsis);
    if max_width <= ellipsis_width {
        return ellipsis.to_owned();
    }

    let target = max_width - ellipsis_width;
    let mut width = 0;
    let mut output = String::new();
    for ch in text.chars() {
        let ch_width = UnicodeWidthChar::width(ch).unwrap_or(0);
        if width + ch_width > target {
            break;
        }
        output.push(ch);
        width += ch_width;
    }
    output.push_str(ellipsis);
    output
}

pub fn pad_to_width(text: &str, target_width: usize) -> String {
    let width = display_width(text);
    if width >= target_width {
        return text.to_owned();
    }
    format!("{}{}", text, " ".repeat(target_width - width))
}

pub fn strip_ansi(text: &str) -> String {
    let mut output = String::new();
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\x1b' && chars.peek() == Some(&'[') {
            chars.next();
            for next in chars.by_ref() {
                if ('@'..='~').contains(&next) {
                    break;
                }
            }
        } else {
            output.push(ch);
        }
    }

    output
}

pub fn visible_width(text: &str) -> usize {
    display_width(&strip_ansi(text))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_chinese_as_double_width() {
        assert_eq!(display_width("普通"), 4);
        assert_eq!(display_width("Tab 标签"), 8);
    }

    #[test]
    fn truncates_mixed_text_without_exceeding_width() {
        let truncated = truncate_to_width("标签-开发环境-main", 10);
        assert!(display_width(&truncated) <= 10);
        assert!(truncated.ends_with('…'));
    }

    #[test]
    fn pads_text_to_display_width() {
        assert_eq!(display_width(&pad_to_width("普通", 6)), 6);
    }

    #[test]
    fn strips_ansi_before_measuring_visible_width() {
        let colored = "\x1b[36mCtrl\x1b[0m  p 窗格";
        assert_eq!(strip_ansi(colored), "Ctrl  p 窗格");
        assert_eq!(visible_width(colored), display_width("Ctrl  p 窗格"));
    }
}
