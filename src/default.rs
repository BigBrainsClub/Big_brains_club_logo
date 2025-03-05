use indexmap::IndexMap;

use crate::LogoBuilder;

impl Default for LogoBuilder {
    fn default() -> Self {
        Self {
            information: vec![],
            extra_info: IndexMap::new(),
            splitter: " =>",
            custom_header: None,
            vertical_char: "║",
            horizontal_char: "═",
            left_up_char: "╔",
            left_down_char: "╚",
            right_down_char: "╝",
            right_up_char: "╗",
            left_info_char: "[",
            right_info_char: "]"
        }
    }
}