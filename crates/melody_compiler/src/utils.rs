/// changes line comments to inline comments
/// while preserving line numbers
pub fn format_line_comments(source: &str) -> String {
    source
        .lines()
        .map(|line| {
            if line.trim_start().starts_with("//") {
                let comment = line.trim_start().trim_start_matches("//");
                format!("/*{comment}*/")
            } else {
                line.to_owned()
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}
