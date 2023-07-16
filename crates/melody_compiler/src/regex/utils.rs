pub fn wrap_quantified(value: String) -> String {
    if directly_quantifiable(&value) {
        value
    } else {
        format!("(?:{value})")
    }
}

fn directly_quantifiable(value: &str) -> bool {
    let value_char_count = value.chars().count();

    // missing (and currently unsupported):
    // - \xYY
    // - \ddd
    // - \uYYYY
    match value_char_count {
        // single char values
        1 => true,
        // escaped single char values
        2 => value.starts_with('\\'),
        // groups and character classes
        _ => match value.chars().next() {
            Some('(') => value.ends_with(')'),
            Some('[') => value.ends_with(']'),
            Some('\\') => {
                let has_unicode_group_prefix = value.starts_with("\\p{") || value.starts_with("\\P{");
                let has_unicode_group_suffix = value.ends_with('}');
                has_unicode_group_prefix && has_unicode_group_suffix
            }
            _ => false,
        },
    }
}

pub fn mark_lazy(quantifier: String, lazy: bool) -> String {
    if lazy {
        format!("{quantifier}?")
    } else {
        quantifier
    }
}
