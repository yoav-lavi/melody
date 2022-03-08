#[derive(Debug, PartialEq, Eq)]
pub enum GroupKind {
    Match,
    Capture,
    Either,
}

#[derive(Debug)]
pub enum AssertionKind {
    Ahead,
    Behind,
}

#[derive(Debug)]
pub enum QuantifierKind {
    Range { start: String, end: String },
    Some,
    Any,
    Over(String),
    Option,
    Amount(String),
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Quantifier {
    pub kind: QuantifierKind,
    pub lazy: bool,
    pub expression: Box<Expression>,
}

#[derive(Debug)]
pub enum Expression {
    Atom(String),
    Group(Group),
    Symbol(Symbol),
    Range(Range),
    NegativeCharClass(String),
}

#[derive(Debug)]
pub enum Range {
    AsciiRange(AsciiRange),
    NumericRange(NumericRange),
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct AsciiRange {
    pub negative: bool,
    pub start: char,
    pub end: char,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct NumericRange {
    pub negative: bool,
    pub start: char,
    pub end: char,
}

#[derive(Debug)]
pub struct Symbol {
    pub kind: SymbolKind,
    pub negative: bool,
}

#[derive(Debug)]
pub enum SymbolKind {
    Space,
    Newline,
    Vertical,
    Return,
    Tab,
    Null,
    Whitespace,
    Alphabetic,
    Alphanumeric,
    Char,
    Digit,
    Word,
    Feed,
    Backspace,
    Boundary,
}

#[derive(Debug)]
pub enum SpecialSymbol {
    Start,
    End,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Group {
    pub ident: Option<String>,
    pub kind: GroupKind,
    pub statements: Vec<Node>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Assertion {
    pub kind: AssertionKind,
    pub statements: Vec<Node>,
    pub negative: bool,
}

#[derive(Debug)]
pub enum Node {
    Group(Group),
    Assertion(Assertion),
    Quantifier(Quantifier),
    Atom(String),
    Range(Range),
    Symbol(Symbol),
    SpecialSymbol(SpecialSymbol),
    NegativeCharClass(String),
    EndOfInput,
}
