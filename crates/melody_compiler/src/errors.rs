#[derive(Debug, Clone)]
pub struct ParseError {
    /// the unrecognized token responsible for the [ParseError]
    pub token: String,
    /// the line in which an unrecognized token was encountered
    pub line: String,
    /// 0 based index of the line in which an unrecognized token was encountered
    pub line_index: usize,
}

impl ParseError {
    pub fn new(slice: &str, source: &str, line: u16) -> Self {
        let line_index = usize::from(line);
        let line_source = source.split('\n').nth(line_index).unwrap();
        Self {
            token: String::from(slice),
            line: String::from(line_source),
            line_index,
        }
    }
}
