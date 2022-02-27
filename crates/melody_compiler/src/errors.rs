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
    /// creates a new [ParseError].
    /// panics if line_index is larger than source.len()
    pub fn new(slice: &str, source: &str, line_index: usize) -> Self {
        let line_source = source
            .split('\n')
            .nth(line_index)
            .expect("Invalid line_index used to create a ParseError");
        Self {
            token: String::from(slice),
            line: String::from(line_source),
            line_index,
        }
    }
}
