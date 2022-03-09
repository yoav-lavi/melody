#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GroupKind {
    Match,
    Capture,
    Either,
}

#[derive(Debug, Clone)]
pub enum AssertionKind {
    Ahead,
    Behind,
}

#[derive(Debug, Clone)]
pub enum QuantifierKind {
    Range { start: String, end: String },
    Some,
    Any,
    Over(usize),
    Option,
    Amount(String),
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Quantifier {
    pub kind: QuantifierKind,
    pub lazy: bool,
    pub expression: Box<Expression>,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Atom(String),
    Group(Group),
    Symbol(Symbol),
    Range(Range),
    NegativeCharClass(String),
}

#[derive(Debug, Clone)]
pub enum Range {
    AsciiRange(AsciiRange),
    NumericRange(NumericRange),
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AsciiRange {
    pub negative: bool,
    pub start: char,
    pub end: char,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct NumericRange {
    pub negative: bool,
    pub start: char,
    pub end: char,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub kind: SymbolKind,
    pub negative: bool,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum SpecialSymbol {
    Start,
    End,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Group {
    pub ident: Option<String>,
    pub kind: GroupKind,
    pub statements: Vec<Node>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct VariableInvocation {
    pub statements: Vec<Node>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Assertion {
    pub kind: AssertionKind,
    pub statements: Vec<Node>,
    pub negative: bool,
}

#[derive(Debug, Clone)]
pub enum Node {
    Group(Group),
    Assertion(Assertion),
    Quantifier(Quantifier),
    Atom(String),
    Range(Range),
    Symbol(Symbol),
    SpecialSymbol(SpecialSymbol),
    NegativeCharClass(String),
    VariableInvocation(VariableInvocation),
    Empty,
}
