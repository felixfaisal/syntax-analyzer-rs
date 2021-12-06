use crate::tokenize::*;
pub enum ObjectStates {
    _Start_ = 0,
    OpenObject,
    Property,
    Comma,
}
pub enum PropertyStates {
    _Start_ = 0,
    Key,
    Colon,
}
pub enum ArrayStates {
    _Start_ = 0,
    OpenArray,
    Value,
    Comma,
}
pub fn parse_object(input: &str, token_list: &Vec<Token>, index: &mut usize) -> bool {
    let mut state = ObjectStates::_Start_;
    while *index < token_list.len() {
        let mut token = &token_list[*index];
        match state {
            ObjectStates::_Start_ => {
                if token.token_type == TokenTypes::LeftBrace {
                    state = ObjectStates::OpenObject;
                    *index += 1;
                } else {
                    break;
                }
            }
            ObjectStates::OpenObject => {
                if token.token_type == TokenTypes::RightBrace {
                    return true;
                } else if token.token_type == TokenTypes::Comment {
                    *index += 1;
                } else {
                    parse_property(input, token_list, index);
                    state = ObjectStates::Property;
                }
            }
            ObjectStates::Property => {
                if token.token_type == TokenTypes::RightBrace {
                    *index += 1;
                    return true;
                } else if token.token_type == TokenTypes::Comma {
                    state = ObjectStates::Comma;
                    *index += 1;
                } else if token.token_type == TokenTypes::Comment {
                    *index += 1;
                } else {
                    panic!("Unexpected Token");
                }
            }
            ObjectStates::Comma => {
                parse_property(input, token_list, index);
                token = &token_list[*index];
                if token.token_type == TokenTypes::RightBrace {
                    *index += 1;
                    return true;
                } else if token.token_type == TokenTypes::Comment {
                    *index += 1;
                } else if token.token_type == TokenTypes::Comma {
                    state = ObjectStates::Comma;
                    *index += 1;
                } else {
                    panic!("Unexpected Token");
                }
            }
        }
    }
    false
}
pub fn parse_property(input: &str, token_list: &Vec<Token>, index: &mut usize) {
    let mut state = PropertyStates::_Start_;
    while *index < token_list.len() {
        let token = &token_list[*index];
        match state {
            PropertyStates::_Start_ => {
                if token.token_type == TokenTypes::String {
                    // We extract key
                    state = PropertyStates::Key;
                    *index += 1;
                } else if token.token_type == TokenTypes::MultiLineString {
                    panic!(
                        "{}",
                        format!(
                            "Key cannot be multiline string {}:{}",
                            token.row, token.column
                        )
                    );
                } else {
                    break;
                }
            }
            PropertyStates::Key => {
                if token.token_type == TokenTypes::Colon {
                    state = PropertyStates::Colon;
                    *index += 1;
                } else if token.token_type == TokenTypes::Comment {
                    *index += 1;
                } else {
                    panic!("Unexpect Token in extracting value");
                }
            }
            PropertyStates::Colon => {
                parse_value(input, token_list, index);
                break;
            }
        }
    }
}
pub fn parse_value(input: &str, token_list: &Vec<Token>, index: &mut usize) -> bool {
    let value = parse_literal(input, token_list, index)
        || parse_object(input, token_list, index)
        || parse_array(input, token_list, index);
    value
}
pub fn parse_literal(_input: &str, token_list: &Vec<Token>, index: &mut usize) -> bool {
    let token = &token_list[*index];
    if token.token_type == TokenTypes::String || token.token_type == TokenTypes::MultiLineString {
        *index += 1;
        return true;
    }
    false
}
pub fn parse_array(input: &str, token_list: &Vec<Token>, index: &mut usize) -> bool {
    let mut state = ArrayStates::_Start_;
    while *index < token_list.len() {
        let token = &token_list[*index];
        match state {
            ArrayStates::_Start_ => {
                if token.token_type == TokenTypes::LeftBracket {
                    state = ArrayStates::OpenArray;
                    *index += 1;
                } else {
                    break;
                }
            }
            ArrayStates::OpenArray => {
                if token.token_type == TokenTypes::RightBracket {
                    *index += 1;
                    return true;
                } else {
                    parse_value(input, token_list, index);
                    state = ArrayStates::Value;
                }
            }
            ArrayStates::Value => {
                if token.token_type == TokenTypes::RightBracket {
                    *index += 1;
                    return true;
                } else if token.token_type == TokenTypes::Comma {
                    state = ArrayStates::Comma;
                    *index += 1;
                } else {
                    panic!("Unexpected Token");
                }
            }
            ArrayStates::Comma => {
                if parse_value(input, token_list, index) == false {
                    *index += 1;
                }
                state = ArrayStates::Value;
            }
        }
    }
    false
}
