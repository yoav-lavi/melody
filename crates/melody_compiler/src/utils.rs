pub fn format_regex(regex: &str, flags: Option<String>) -> String {
    format!("/{regex}/{}", flags.unwrap_or_default())
}
