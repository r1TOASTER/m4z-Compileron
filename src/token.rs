#[allow(dead_code)]
#[derive(Debug)]
pub enum TokenType {
    IntLiteral,
    UIntLiteral,
    CharLiteral,
    StrLiteral,
    DoubleLiteral,
    BooleanLiteral,
    NameLiteral,
    IntKeyword,
    UIntKeyword,
    CharKeyword,
    StrKeyword,
    DoubleKeyword,
    BooleanKeyword,
    Dec,
    Fn,
    Ret,
    Semicolon,
    OpeningPar,
    ClosingPar,
    OpeningCur,
    ClosingCur,
    OpeningBra,
    ClosingBra,
    Quo,
    Div,
    Equal,
    GreaterEqual,
    Greater,
    SmallerEqual,
    Smaller,
    Minus,
    Plus,
    Multi,
    Divison,
    Comma,
    Colon
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Token {
    pub Type: TokenType,
    pub Data: Option<String>,
}