pub fn wrap_quantified(value: String) -> String {
    let is_grouped = value.starts_with('(') && value.ends_with(')');
    if !is_grouped && value.chars().count() > 1 {
        format!("(?:{value})")
    } else {
        value
    }
}
