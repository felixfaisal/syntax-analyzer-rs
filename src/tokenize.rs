#[allow(dead_code)]
use crate::parser::parse_object;

#[derive(PartialEq, Debug)]
pub enum TokenTypes {
    LeftBrace = 0,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
    Comma,
    String,
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
#[derive(PartialEq, Debug)]
pub enum CommentStates {
    _Start_ = 0,
    OpenComment,
}
#[derive(Debug)]
pub struct Token {
    pub token_type: TokenTypes,
    pub column: i32,
    pub row: i32,
    pub value: Option<Value>,
}
impl TokenTypes {
    pub fn parse_char(input: &char, row: i32, column: i32) -> Option<Token> {
        match input {
            '{' => Some(Token {
                token_type: TokenTypes::LeftBrace,
                column: column,
                row: row,
                value: None,
            }),
            '}' => Some(Token {
                token_type: TokenTypes::RightBrace,
                column: column,
                row: row,
                value: None,
            }),
            '[' => Some(Token {
                token_type: TokenTypes::LeftBracket,
                column: column,
                row: row,
                value: None,
            }),
            ']' => Some(Token {
                token_type: TokenTypes::RightBracket,
                column: column,
                row: row,
                value: None,
            }),
            ':' => Some(Token {
                token_type: TokenTypes::Colon,
                column: column,
                row: row,
                value: None,
            }),
            ',' => Some(Token {
                token_type: TokenTypes::Comma,
                column: column,
                row: row,
                value: None,
            }),
            _ => None,
        }
    }
    pub fn parse_string(input: &str, row: usize, column: &mut usize) -> Option<Token> {
        let mut buffer: String = String::new();
        let mut state = StringStates::_Start_;
        if *column + 2 < input.len()
            && format!(
                "{}{}{}",
                input.chars().nth(*column).unwrap(),
                input.chars().nth(*column + 1).unwrap(),
                input.chars().nth(*column + 2).unwrap()
            ) == "\"\"\""
        {
            return None;
        }
        if input.as_bytes()[*column] as char != '"' {
            return None;
        }
        while *column < input.len() {
            let return_string = buffer.clone();
            let character = input.as_bytes()[*column as usize] as char;
            match state {
                StringStates::_Start_ => {
                    if character == '"' {
                        state = StringStates::StartQuoteOrChar;
                        *column += 1;
                    }
                }
                StringStates::StartQuoteOrChar => {
                    if character == '\\' {
                        buffer += &character.to_string();
                        state = StringStates::Escape;
                    } else if character == '"' {
                        // return
                        return Some(Token {
                            token_type: TokenTypes::String,
                            row: row as i32,
                            column: *column as i32,
                            value: Some(Value::String(return_string)),
                        });
                    } else {
                        buffer += &character.to_string();
                        *column += 1;
                    }
                }
                StringStates::Escape => {
                    buffer += &character.to_string();
                    *column += 1;
                    state = StringStates::StartQuoteOrChar;
                }
            }
        }
        None
    }
    pub fn parse_multiline_string(
        input: &Vec<&str>,
        row: &mut usize,
        column: &mut usize,
    ) -> Option<Token> {
        let mut buffer: String = String::new();
        let mut state = StringStates::_Start_;
        while *column < input[*row].len() {
            let return_string = buffer.clone();
            //dbg!(input.chars());
            let character = input[*row].as_bytes()[*column as usize] as char;
            match state {
                StringStates::_Start_ => {
                    if *column + 2 < input[*row].len()
                        && format!(
                            "{}{}{}",
                            input[*row].chars().nth(*column).unwrap(),
                            input[*row].chars().nth(*column + 1).unwrap(),
                            input[*row].chars().nth(*column + 2).unwrap()
                        ) == "\"\"\""
                    {
                        state = StringStates::StartQuoteOrChar;
                        *column += 3;
                    } else {
                        return None;
                    }
                }
                StringStates::StartQuoteOrChar => {
                    if character == '\\' {
                        buffer += &character.to_string();
                        state = StringStates::Escape;
                    } else if *column + 2 < input[*row].len()
                        && format!(
                            "{}{}{}",
                            input[*row].chars().nth(*column).unwrap(),
                            input[*row].chars().nth(*column + 1).unwrap(),
                            input[*row].chars().nth(*column + 2).unwrap()
                        ) == "\"\"\""
                    {
                        // return
                        *column += 2;
                        return Some(Token {
                            token_type: TokenTypes::MultiLineString,
                            row: *row as i32,
                            column: *column as i32,
                            value: Some(Value::String(return_string)),
                        });
                    } else if *column + 1 == input[*row].len() {
                        //println!("Found multiline");
                        *row += 1;
                        *column = 0;
                        buffer.push_str("\n");
                    } else {
                        //dbg!(&buffer);
                        buffer += &character.to_string();
                        *column += 1;
                    }
                }
                StringStates::Escape => {
                    buffer += &character.to_string();
                    *column += 1;
                    state = StringStates::StartQuoteOrChar;
                }
            }
        }
        None
    }
    pub fn parse_comment(input: &str, row: usize, column: &mut usize) -> Option<Token> {
        let mut buffer: String = String::new();
        let mut state = CommentStates::_Start_;
        if *column + 1 >= input.len() {
            return None;
        }
        if format!(
            "{}{}",
            input.chars().nth(*column).unwrap(),
            input.chars().nth(*column + 1).unwrap()
        ) != "//"
        {
            return None;
        }
        while *column < input.len() {
            let character = input.as_bytes()[*column as usize] as char;
            match state {
                CommentStates::_Start_ => {
                    if format!(
                        "{}{}",
                        input.chars().nth(*column).unwrap(),
                        input.chars().nth(*column + 1).unwrap()
                    ) == "//"
                    {
                        *column += 2;
                        state = CommentStates::OpenComment;
                    } else {
                        return None;
                    }
                }
                CommentStates::OpenComment => {
                    buffer += &character.to_string();
                    *column += 1;
                }
            }
        }
        Some(Token {
            token_type: TokenTypes::Comment,
            row: row as i32,
            column: *column as i32,
            value: Some(Value::String(buffer)),
        })
    }
}
pub fn tokenize(data: String) -> bool {
    let lines: Vec<&str> = data.split("\n").collect();
    let mut tokens: Vec<Token> = vec![];
    let mut j: usize;
    let mut x: usize = 0;
    while x < lines.len() {
        j = 0;
        while j < lines[x].len() {
            let character = lines[x].as_bytes()[j] as char;
            //parse whitespace
            if character == ' ' {
                j += 1;
                continue;
            }
            if let Some(token) = TokenTypes::parse_char(&character, x as i32, j as i32){
                tokens.push(token);
            }
            else if let Some(token) = TokenTypes::parse_string(lines[x], x, &mut j){
                tokens.push(token);
            }
            else if let Some(token) = TokenTypes::parse_comment(lines[x], x, &mut j){
                tokens.push(token);
            }
            else if let Some(token) =  TokenTypes::parse_multiline_string(&lines, &mut x, &mut j){
                tokens.push(token);
            }
            else{
                panic!(
                    "{}",
                    format!("Unidentified Symbol {} at line {}:{}", character, x, j)
                ); 
            }
            j += 1;
        }
        x += 1;
    }
    let mut index: usize = 0;
    parse_object(&lines[0], &tokens, &mut index);
    if index == tokens.len() {
        true
    } else {
        false
    }
}
