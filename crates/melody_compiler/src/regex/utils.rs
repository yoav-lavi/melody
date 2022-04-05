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
    // - \p{...}
    // - \P{...}
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
