#[derive(PartialEq,Debug)]
pub enum TokenTypes {
    LeftBrace = 0,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
    Comma,
    String,
    Number,
    True,
    False,
    Null,
    MultiLineString,
    Comment,
}
#[derive(Debug)]
pub enum Value {
    String(String),
}
#[derive(PartialEq, Debug)]
pub enum StringStates {
    _Start_ = 0,
    StartQuoteOrChar,
    Escape,
}
#[derive(Debug)]
pub struct Token {
    pub token_type: TokenTypes,
    pub column: i32,
    pub row: i32,
    pub value: Option<Value>,
}

