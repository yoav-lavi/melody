#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "fuzzer", derive(arbitrary::Arbitrary))]
pub enum GroupKind {
    Match,
    Capture,
    Either,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "fuzzer", derive(arbitrary::Arbitrary))]
pub enum AssertionKind {
    Ahead,
    Behind,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "fuzzer", derive(arbitrary::Arbitrary))]
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
#[cfg_attr(feature = "fuzzer", derive(arbitrary::Arbitrary))]
pub struct Quantifier {
    pub kind: QuantifierKind,
    pub lazy: bool,
    pub expression: Box<Expression>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "fuzzer", derive(arbitrary::Arbitrary))]
pub enum Expression {
    Atom(String),
    Group(Group),
    Symbol(Symbol),
    UnicodeCategory(UnicodeCategory),
    Range(Range),
    NegativeCharClass(String),
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "fuzzer", derive(arbitrary::Arbitrary))]
pub enum Range {
    AsciiRange(AsciiRange),
    NumericRange(NumericRange),
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
#[cfg_attr(feature = "fuzzer", derive(arbitrary::Arbitrary))]
pub struct AsciiRange {
    pub negative: bool,
    pub start: char,
    pub end: char,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
#[cfg_attr(feature = "fuzzer", derive(arbitrary::Arbitrary))]
pub struct NumericRange {
    pub negative: bool,
    pub start: char,
    pub end: char,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "fuzzer", derive(arbitrary::Arbitrary))]
pub struct Symbol {
    pub kind: SymbolKind,
    pub negative: bool,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "fuzzer", derive(arbitrary::Arbitrary))]
pub struct UnicodeCategory {
    pub kind: UnicodeCategoryKind,
    pub negative: bool,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "fuzzer", derive(arbitrary::Arbitrary))]
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
#[cfg_attr(feature = "fuzzer", derive(arbitrary::Arbitrary))]
pub enum UnicodeCategoryKind {
    CasedLetter,
    ClosePunctuation,
    ConnectorPunctuation,
    Control,
    CurrencySymbol,
    DashPunctuation,
    DecimalDigitNumber,
    EnclosingMark,
    FinalPunctuation,
    Format,
    InitialPunctuation,
    LetterNumber,
    Letter,
    LineSeparator,
    LowercaseLetter,
    Mark,
    MathSymbol,
    ModifierLetter,
    ModifierSymbol,
    NonSpacingMark,
    Number,
    OpenPunctuation,
    OtherLetter,
    OtherNumber,
    OtherPunctuation,
    OtherSymbol,
    Other,
    ParagraphSeparator,
    PrivateUse,
    Punctuation,
    Separator,
    SpaceSeparator,
    SpacingCombiningMark,
    Surrogate,
    Symbol,
    TitlecaseLetter,
    Unassigned,
    UppercaseLetter,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "fuzzer", derive(arbitrary::Arbitrary))]
pub enum SpecialSymbolKind {
    Start,
    End,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
#[cfg_attr(feature = "fuzzer", derive(arbitrary::Arbitrary))]
pub struct Group {
    pub ident: Option<String>,
    pub kind: GroupKind,
    pub statements: Box<MelodyAst>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
#[cfg_attr(feature = "fuzzer", derive(arbitrary::Arbitrary))]
pub struct VariableInvocation {
    pub statements: Box<MelodyAst>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
#[cfg_attr(feature = "fuzzer", derive(arbitrary::Arbitrary))]
pub struct Assertion {
    pub kind: AssertionKind,
    pub statements: Box<MelodyAst>,
    pub negative: bool,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "fuzzer", derive(arbitrary::Arbitrary))]
pub enum MelodyAstNode {
    Group(Group),
    Assertion(Assertion),
    Quantifier(Quantifier),
    Atom(String),
    Range(Range),
    Symbol(Symbol),
    SpecialSymbol(SpecialSymbolKind),
    UnicodeCategory(UnicodeCategory),
    NegativeCharClass(String),
    VariableInvocation(VariableInvocation),
    Skip,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "fuzzer", derive(arbitrary::Arbitrary))]
pub enum MelodyAst {
    Root(Vec<MelodyAstNode>),
    Empty,
}
