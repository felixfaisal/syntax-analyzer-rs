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
                } else {
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
                } else {
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
                } else {
                    panic!("Unexpected Token");
                }
            }
        }
    }
    false
}
