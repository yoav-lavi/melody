use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct MultipleExprs<'a>(pub Vec<Expr<'a>>);

impl<'a> From<Vec<Expr<'a>>> for MultipleExprs<'a> {
    fn from(v: Vec<Expr<'a>>) -> Self {
        Self(v)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expr<'a> {
    Literal(&'a str),
    CharRange {
        start: char,
        end: char,
    },
    DigitRange {
        start: u8,
        end: u8,
    },
    Keyword(Keyword),
    Quantifier {
        range: Range,
        expr: Box<Expr<'a>>,
    },
    Capture {
        capture_name: Option<&'a str>,
        capture_exprs: MultipleExprs<'a>,
    },
    Match {
        match_name: Option<&'a str>,
        match_exprs: MultipleExprs<'a>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Range {
    Exact(u64),
    Numeric(std::ops::Range<u64>),
    Over(std::ops::RangeFrom<u64>),
    Upto(std::ops::RangeTo<u64>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Keyword {
    NewLine,        // "<newline>"
    TabSymbol,      // "<tab>"
    ReturnSymbol,   // "<return>"
    FeedSymbol,     // "<feed>"
    NullSymbol,     // "<null>"
    DigitSymbol,    // "<digit>"
    WordSymbol,     // "<word>"
    VerticalSymbol, // "<vertical>"
    SpaceSymbol,    // "<space>"
    AnyChar,        // "char"
    AnchorStart,    // "start"
    AnchorEnd,      //  "end"
}

impl<T> From<T> for Keyword
where
    T: AsRef<str>,
{
    fn from(s: T) -> Self {
        use Keyword::*;
        match s.as_ref() {
            "<newline>" => NewLine,
            "<tab>" => TabSymbol,
            "<return>" => ReturnSymbol,
            "<feed>" => FeedSymbol,
            "<null>" => NullSymbol,
            "<digit>" => DigitSymbol,
            "<word>" => WordSymbol,
            "<vertical>" => VerticalSymbol,
            "<space>" => SpaceSymbol,
            "char" => AnyChar,
            "start" => AnchorStart,
            "end" => AnchorEnd,
            _ => panic!("unkown keyword {}", s.as_ref()),
        }
    }
}

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sym = match self {
            Keyword::NewLine => "\\n",
            Keyword::TabSymbol => "\\t",
            Keyword::ReturnSymbol => "\\r",
            Keyword::FeedSymbol => "\\f",
            Keyword::NullSymbol => "\\0",
            Keyword::DigitSymbol => "\\d",
            Keyword::WordSymbol => "\\w",
            Keyword::VerticalSymbol => "\\v",
            Keyword::SpaceSymbol => "\\s",
            Keyword::AnyChar => ".",
            Keyword::AnchorStart => "^",
            Keyword::AnchorEnd => "$",
        };
        write!(f, "{}", sym)
    }
}
