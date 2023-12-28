#[macro_export]
macro_rules! regex {
    ($raw:expr) => {{
        static REGEX: once_cell::sync::OnceCell<::regex::Regex> = once_cell::sync::OnceCell::new();
        REGEX.get_or_init(|| ::regex::Regex::new($raw).unwrap())
    }};
}
