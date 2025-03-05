use std::sync::LazyLock;

use colored::Colorize;

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
