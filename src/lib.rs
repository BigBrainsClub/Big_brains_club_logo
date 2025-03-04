use std::sync::LazyLock;
use colored::Colorize;
use strip_ansi_escapes::strip_str;
use unicode_width::UnicodeWidthStr;

const VERTICAL_CHAR: &str = "║";
const HORIZONTAL_CHAR: &str = "═";
const LEFT_UP_CHAR: &str = "╔";
const LEFT_DOWN_CHAR: &str = "╚";
const RIGHT_UP_CHAR: &str = "╗";
const RIGHT_DOWN_CHAR: &str = "╝";
const LEFT_CHAR_INFO: &str = "[";
const RIGHT_CHAR_INFO: &str = "]";

pub static LOGO_PRINT: LazyLock<String> = LazyLock::new(|| {
    format!(
        "  ____  _         ____            _              _____ _       _     
 |  _ \\(_)       |  _ \\          (_)            / ____| |     | |    
 | |_) |_  __ _  | |_) |_ __ __ _ _ _ __  ___  | |    | |_   _| |__  
 |  _ <| |/ _` | |  _ <| '__/ _` | | '_ \\/ __| | |    | | | | | '_ \\ 
 | |_) | | (_| | | |_) | | | (_| | | | | \\__ \\ | |____| | |_| | |_) |
 |____/|_|\\__, | |____/|_|  \\__,_|_|_| |_|___/  \\_____|_|\\__,_|_.__/ 
           __/ |
          |___/")
        .bright_red()
        .to_string()
});

pub static BIG_DICKS_TO: LazyLock<String> = LazyLock::new(|| {
    format!(
        "
    ┌───────────────────┐
    │and big dicks to{} │
    └───────────────────┘",
        " √".green()
    )
    .bright_black()
    .to_string()
});

#[derive(Clone)]
pub struct LogoBuilder {
    information: Vec<(&'static str, Option<&'static str>)>,
    pub extra_info: Vec<(&'static str, Option<usize>)>,
    splitter: &'static str,
    custom_header: Option<&'static str>,
}

impl LogoBuilder {
    pub fn new(
        information: Vec<(&'static str, Option<&'static str>)>,
        extra_info: Vec<(&'static str, Option<usize>)>,
        splitter: &'static str,
    ) -> Self {
        Self {
            information,
            extra_info,
            splitter,
            custom_header: None,
        }
    }

    pub fn with_custom_header(mut self, text: &'static str) -> Self {
        self.custom_header = Some(text);
        self
    }

    fn calculate_visible_width(s: &str) -> usize {
        let stripped = strip_str(s);
        UnicodeWidthStr::width(&*stripped)
    }

    pub fn render(&self) -> String {
        let left_max_width = self
            .information
            .iter()
            .map(|(k, v)| match v {
                Some(value) => {
                    Self::calculate_visible_width(LEFT_CHAR_INFO)
                        + Self::calculate_visible_width(k)
                        + Self::calculate_visible_width(RIGHT_CHAR_INFO)
                        + Self::calculate_visible_width(self.splitter)
                        + Self::calculate_visible_width(value)
                        + 4
                }
                None => {
                    Self::calculate_visible_width(LEFT_CHAR_INFO)
                        + Self::calculate_visible_width(k)
                        + Self::calculate_visible_width(RIGHT_CHAR_INFO)
                        + 2
                }
            })
            .max()
            .unwrap_or(0);

        let horizontal_line = format!(
            "{}{}{}",
            LEFT_UP_CHAR,
            HORIZONTAL_CHAR.repeat(left_max_width),
            RIGHT_UP_CHAR
        );

        let bottom_line = format!(
            "{}{}{}",
            LEFT_DOWN_CHAR,
            HORIZONTAL_CHAR.repeat(left_max_width),
            RIGHT_DOWN_CHAR
        );

        let mut result = String::new();
        result.push_str(&horizontal_line);
        result.push('\n');

        if let Some(header) = self.custom_header {
            let visible_width = Self::calculate_visible_width(header);
            let total_padding = left_max_width.saturating_sub(visible_width);
            let left_padding = total_padding / 2;
            let right_padding = total_padding - left_padding;

            result.push_str(&format!(
                "{}{}{}{}{}\n",
                VERTICAL_CHAR,
                " ".repeat(left_padding),
                header,
                " ".repeat(right_padding),
                VERTICAL_CHAR
            ));
        }

        let total_rows = self.information.len().max(self.extra_info.len());

        for i in 0..total_rows {
            let left_content = if i < self.information.len() {
                let (key, value) = &self.information[i];
                let content = match value {
                    Some(v) => format!(
                        " {}{}{}{} {} ",
                        LEFT_CHAR_INFO, key, RIGHT_CHAR_INFO, self.splitter, v
                    ),
                    None => format!(" {}{}{} ", LEFT_CHAR_INFO, key, RIGHT_CHAR_INFO),
                };
                let visible_width = Self::calculate_visible_width(&content);
                let padding = left_max_width.saturating_sub(visible_width);
                format!("{}{}", content, " ".repeat(padding))
            } else {
                " ".repeat(left_max_width).to_string()
            };

            let right_content = if i < self.extra_info.len() {
                let (key, value) = &self.extra_info[i];
                match value {
                    Some(num) => format!(" {}: {}", key, num),
                    None => format!(" {}", key),
                }
            } else {
                String::new()
            };

            result.push_str(&format!(
                "{}{}{}  {}\n",
                VERTICAL_CHAR, left_content, VERTICAL_CHAR, right_content
            ));
        }

        result.push_str(&bottom_line);
        format!("{}{}\n{}", *LOGO_PRINT, *BIG_DICKS_TO, result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logo() {
        let logo = LogoBuilder::new(
            vec![
                ("telegram", Some("@MolodostVnutri")),
                ("Ссылка", None),
                ("Тема", Some("https://lolz.live/threads")),
            ],
            vec![
                ("тест", Some(623)),
                ("test", None),
            ],
            "=>"
        )
        .with_custom_header("TESTING");

        let output = logo.render();
        println!("{}", output);
        
        assert!(output.contains("[telegram] => @MolodostVnutri"));
        assert!(output.contains("[Ссылка]"));
        assert!(!output.contains("[Ссылка] =>"));
        
        assert!(output.contains("тест: 623"));
        assert!(!output.contains(" test"));
    }
}