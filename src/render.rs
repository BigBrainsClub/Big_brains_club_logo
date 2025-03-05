use strip_ansi_escapes::strip_str;
use unicode_width::UnicodeWidthStr;

use crate::{static_data::{BIG_DICKS_TO, LOGO_PRINT}, LogoBuilder};


impl LogoBuilder {
    pub fn entry_extra(&mut self, key: &str, new_value: Option<usize>) -> &mut Self {
        if let Some(new_value) = new_value {
            match self.extra_info.get_mut(key) {
                Some(value) => {
                    match value {
                        Some(value) => *value += new_value,
                        None => *value = Some(new_value)
                    }
                },
                None => {
                    self.extra_info.insert(key.to_string().leak(), Some(new_value));
                }
            }
        } else {
            self.extra_info.insert(key.to_string().leak(), None);
        }
        self
    }
    pub fn entry_info(&mut self, info: Vec<(&'static str, Option<&'static str>)>) -> &mut Self {
        self.information.extend(info);
        self
    }

    fn calculate_visible_width(s: &str) -> usize {
        let stripped = strip_str(s);
        UnicodeWidthStr::width(&*stripped)
    }

    pub fn with_custom_header(&mut self, text: &'static str) -> &mut Self {
        self.custom_header = Some(text);
        self
    }

    fn calculate_left_max_width(&self) -> usize {
        self.information
            .iter()
            .map(|(k, v)| match v {
                Some(value) => {
                    Self::calculate_visible_width(self.left_info_char)
                    + Self::calculate_visible_width(k)
                    + Self::calculate_visible_width(self.right_info_char)
                    + Self::calculate_visible_width(self.splitter)
                    + Self::calculate_visible_width(value)
                    + 4
                },
                None => {
                    Self::calculate_visible_width(self.left_info_char)
                    + Self::calculate_visible_width(k)
                    + Self::calculate_visible_width(self.right_info_char)
                    + 2
                }
            })
            .max()
            .unwrap_or(0)
    }
    fn horizontal_line(&self) -> String {
        format!(
            "{}{}{}",
            self.left_up_char,
            self.horizontal_char.repeat(self.calculate_left_max_width()),
            self.right_up_char
        )
    }
    fn botton_line(&self) -> String {
        format!(
            "{}{}{}",
            self.left_down_char,
            self.horizontal_char.repeat(self.calculate_left_max_width()),
            self.right_down_char
        )
    }
    fn custom_header_builder(&self, result: &mut String) {
        if let Some(header) = self.custom_header {
            let visible_width = Self::calculate_visible_width(header);
            let total_padding = self.calculate_left_max_width().saturating_sub(visible_width);
            let left_padding = total_padding / 2;
            let rigth_padding = total_padding - left_padding;

            result.push_str(&format!(
                "{}{}{}{}{}\n",
                self.vertical_char,
                " ".repeat(left_padding),
                header,
                " ".repeat(rigth_padding),
                self.vertical_char
            ));
        }
    }

    pub fn custom_info_chars(&mut self, left_char: &'static str, right_char: &'static str) -> &mut Self {
        self.left_info_char = left_char;
        self.right_info_char = right_char;
        self
    }

    pub fn render(&self) -> String {
        let mut result = String::new();

        result.push_str(&self.horizontal_line());
        result.push('\n');
        self.custom_header_builder(&mut result);
        self.build_information_and_extra_data(&mut result)
    }

    fn build_information_and_extra_data(&self, result: &mut String) -> String {
        let total_rows = self.information.len().max(self.extra_info.len());

        for i in 0..total_rows {
            let left_content = if i < self.information.len() {
                let (key, value)  = &self.information[i];
                let content = match value {
                    Some(v) => format!(
                        " {}{}{}{} {} ",
                        self.left_info_char,
                        key,
                        self.right_info_char,
                        self.splitter,
                        v
                    ),
                    None => format!(
                        " {}{}{} ",
                        self.left_info_char,
                        key,
                        self.right_info_char
                    )
                };
                let visible_width = Self::calculate_visible_width(&content);
                let padding = self.calculate_left_max_width().saturating_sub(visible_width);
                format!("{}{}", content, " ".repeat(padding))
            } else {
                " ".repeat(self.calculate_left_max_width()).to_string()
            };
            let rigth_content = self.extra_info
                .get_index(i)
                .map(|(key, value)| match value {
                    Some(num) => format!(" {}: {}", key, num),
                    None => format!(" {}", key)
                })
                .unwrap_or_default();
            result.push_str(&format!(
                "{}{}{}  {}\n",
                self.vertical_char,
                left_content,
                self.vertical_char,
                rigth_content
            ));
        }
        result.push_str(&self.botton_line());
        format!("{}{}\n{}", *LOGO_PRINT, *BIG_DICKS_TO, result)
    }
}