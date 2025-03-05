use indexmap::IndexMap;

pub mod default;
pub mod static_data;
pub mod render;

#[derive(Clone)]
pub struct LogoBuilder {
    information: Vec<(&'static str, Option<&'static str>)>,
    extra_info: IndexMap<&'static str, Option<usize>>,
    splitter: &'static str,
    custom_header: Option<&'static str>,
    vertical_char: &'static str,
    horizontal_char: &'static str,
    left_up_char: &'static str,
    left_down_char: &'static str,
    right_up_char: &'static str,
    right_down_char: &'static str,
    left_info_char: &'static str,
    right_info_char: &'static str
}

#[cfg(test)]
mod tests {
    use indexmap::IndexMap;

    use crate::LogoBuilder;


    #[test]
    fn test_logo() {
        let mut extra_info = IndexMap::new();
        extra_info.insert("тест", Some(623));
        extra_info.insert("test", None);

        let mut logo = LogoBuilder::default();
        logo.entry_info(vec![
            ("telegram", Some("@MolodostVnutri")),
            ("Ссылка", None),
            ("Ссылка", Some("Ссылка")),
            ("Тема", Some("https://lolz.live/threads"))
        ])
        
        .with_custom_header("TESTING")
        .entry_extra("test", 623);
        

        let output = logo.render();
        println!("{}", output);
        
        assert!(output.contains("[telegram] => @MolodostVnutri"));
        assert!(output.contains("[Ссылка]"));
        assert!(output.contains("[Ссылка] =>"));
        
        assert!(output.contains("test: 623"));
        assert!(output.contains(" test"));
    }
}