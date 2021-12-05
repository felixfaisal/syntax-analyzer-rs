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
pub fn parse_object(input: &str, token_list: &Vec<Token>, index: &mut usize) -> bool {
    let mut start_token: &Token;
    let mut state = ObjectStates::_Start_;
    while *index < token_list.len() {
        let token = &token_list[*index];
        match state {
            ObjectStates::_Start_ => {
                if token.token_type == TokenTypes::LeftBrace {
                    start_token = token;
                    state = ObjectStates::OpenObject;
                    *index += 1;
                    println!("Start Parsing Object");
                } else {
                    // return null
                }
            }
            ObjectStates::OpenObject => {
                if token.token_type == TokenTypes::RightBrace {
                    //break;
                    return true;
                } else if token.token_type == TokenTypes::Comment {
                    *index += 1;
                }
                else {
                    parse_property(input, token_list, index);
                    println!("Parsed Property Sucesfully");
                    state = ObjectStates::Property;
                    *index += 1;
                }
            }
            ObjectStates::Property => {
                if token.token_type == TokenTypes::RightBrace {
                    // return object
                    println!("Object prased!");
                    //break;
                    return true;
                } else if token.token_type == TokenTypes::Comma {
                    state = ObjectStates::Comma;
                    *index += 1;
                    println!("Property parsed!");
                } 
                else if token.token_type == TokenTypes::Comment {
                    *index += 1;
                }
                else {
                    panic!("Unexpected Token");
                }
            }
            ObjectStates::Comma => {
                // parse property
                parse_property(input, token_list, index);
                // if property then append to children
                if token.token_type == TokenTypes::RightBrace {
                    println!("Object Parsed");
                    //break;
                    return true;
                    // return object and allowing trailing commas
                }
                else if token.token_type == TokenTypes::Comment {
                    *index += 1;
                } 
                else {
                    panic!("Unexpected Token");
                }
            }
        }
    }
    false
}
pub fn parse_property(input: &str, token_list: &Vec<Token>, index: &mut usize) {
    let mut start_token: &Token;
    let mut state = PropertyStates::_Start_;
    while *index < token_list.len() {
        let token = &token_list[*index];
        match state {
            PropertyStates::_Start_ => {
                if token.token_type == TokenTypes::String {
                    // We extract key
                    println!("Extracting key");
                    state = PropertyStates::Key;
                    *index += 1;
                } else if token.token_type == TokenTypes::MultiLineString {
                    panic!("{}",format!("Key cannot be multiline string {}:{}",token.row,token.column));
                }
                else {
                    // Null
                    break;
                }
            }
            PropertyStates::Key => {
                if token.token_type == TokenTypes::Colon {
                    state = PropertyStates::Colon;
                    *index += 1;
                } else if token.token_type == TokenTypes::Comment {
                    *index += 1;
                } 
                else {
                    panic!("Unexpect Token");
                }
            }
            PropertyStates::Colon => {
                // parse value
                println!("Extracted Value");
                parse_value(input, token_list, index);
                break;
            }
        }
    }
}
pub fn parse_value(input: &str, token_list: &Vec<Token>, index: &mut usize) {
    let token = &token_list[*index];
    // Parse value
    let value = parse_literal(input, token_list, index) || parse_object(input, token_list, index);
    //dbg!(&value);
    if value {
        println!("Parsed Value Succesfully");
    } else {
        panic!("Unexpected Token");
    }
    // if value, we return value
    // else we say error
}
pub fn parse_literal(input: &str, token_list: &Vec<Token>, index: &mut usize) -> bool {
    let token = &token_list[*index];
    if token.token_type == TokenTypes::String {
        *index += 1;
        return true;
    }
    false
}
 


