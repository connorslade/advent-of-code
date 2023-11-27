use chrono::{Datelike, Utc};

macro_rules! selector {
    ($raw:expr) => {{
        static SELECTOR: once_cell::sync::OnceCell<scraper::Selector> =
            once_cell::sync::OnceCell::new();
        SELECTOR.get_or_init(|| scraper::Selector::parse($raw).unwrap())
    }};
}

macro_rules! regex {
    ($raw:expr) => {{
        static REGEX: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        REGEX.get_or_init(|| regex::Regex::new($raw).unwrap())
    }};
}

pub fn current_year() -> u16 {
    Utc::now().year() as u16
}
