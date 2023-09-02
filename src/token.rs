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
    If,
    Else,
    Elif,
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
    EqualEqual,
    GreaterEqual,
    Greater,
    SmallerEqual,
    Smaller,
    Minus,
    Plus,
    Multi,
    Divison,
    Comma,
    Colon,
    NotInitiallized
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Token {
    pub Type: TokenType,
    pub Data: Option<String>,
}

impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        (self.Type == other.Type) && (self.Data == other.Data)
    }
}