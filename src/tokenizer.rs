use crate::token::{Token, TokenType};

extern crate regex;
use regex::Regex;

fn is_keyword_alone(str: &String) -> Option<TokenType> {
    if str.eq("dec") {
        return Some(TokenType::Dec);
    }
    else if str.eq("fn") {
        return Some(TokenType::Fn);
    }
    else if str.eq("ret") {
        return Some(TokenType::Ret);
    }
    else if str.eq("if") {
        return Some(TokenType::If);
    }
    else if str.eq("else") {
        return Some(TokenType::Else);
    }
    else if str.eq("elif") {
        return Some(TokenType::Elif);
    }
    else if str.eq("int") {
        return Some(TokenType::IntKeyword);
    }
    else if str.eq("uint") {
        return Some(TokenType::UIntKeyword);
    }
    else if str.eq("char") {
        return Some(TokenType::CharKeyword);
    }
    else if str.eq("str") {
        return Some(TokenType::StrKeyword);
    }
    else if str.eq("boolean") {
        return Some(TokenType::BooleanKeyword);
    }
    else if str.eq("double") {
        return Some(TokenType::DoubleKeyword);
    }
    else if str.eq("\"") {
        return Some(TokenType::Quo);
    }
    else if str.eq("\'") {
        return Some(TokenType::Div);
    }
    else if str.eq("(") {
        return Some(TokenType::OpeningPar);
    }
    else if str.eq(")") {
        return Some(TokenType::ClosingPar);
    }
    else if str.eq("{") {
        return Some(TokenType::OpeningCur);
    }
    else if str.eq("}") {
        return Some(TokenType::ClosingCur);
    }
    else if str.eq("[") {
        return Some(TokenType::OpeningBra);
    }
    else if str.eq("]") {
        return Some(TokenType::ClosingBra);
    }
    else if str.eq(",") {
        return Some(TokenType::Comma);
    }
    else if str.eq(";") {
        return Some(TokenType::Semicolon);
    }
    else if str.eq(":") {
        return Some(TokenType::Colon);
    }
    else if str.eq("=") {
        return Some(TokenType::Equal);
    }
    else if str.eq("==") {
        return Some(TokenType::EqualEqual);
    }
    else if str.eq(">=") {
        return Some(TokenType::GreaterEqual);
    }
    else if str.eq("=>") {
        return Some(TokenType::GreaterEqual);
    }
    else if str.eq(">") {
        return Some(TokenType::Greater);
    }
    else if str.eq("<=") {
        return Some(TokenType::SmallerEqual);
    }
    else if str.eq("=<") {
        return Some(TokenType::SmallerEqual);
    }
    else if str.eq("<") {
        return Some(TokenType::Smaller);
    }
    else if str.eq("-") {
        return Some(TokenType::Minus);
    }
    else if str.eq("+") {
        return Some(TokenType::Plus);
    }
    else if str.eq("*") {
        return Some(TokenType::Multi);
    }
    else if str.eq("/") {
        return Some(TokenType::Divison);
    }
    None
}

const NAME_LITERAL_REGEX_PATTERN: &str = r"^[a-zA-Z_][a-zA-Z0-9_]*$";
const INT_LITERAL_REGEX_PATTERN: &str = r"^[0-9]+$";
const DOUBLE_LITERAL_REGEX_PATTERN: &str = r"^[0-9]+\.[0-9]+$";

fn is_name_literal(buffer: &String) -> bool {
    let regex = Regex::new(NAME_LITERAL_REGEX_PATTERN).expect("Error creating regex for name literals match");
    regex.is_match(buffer.as_str())
}

fn is_int(buffer: &String) -> bool {
    let regex = Regex::new(INT_LITERAL_REGEX_PATTERN).expect("Error creating regex for int literals match");
    regex.is_match(buffer.as_str())
}

fn is_double(buffer: &String) -> bool {
    let regex = Regex::new(DOUBLE_LITERAL_REGEX_PATTERN).expect("Error creating regex for double literals match");
    regex.is_match(buffer.as_str())
}


fn get_token_from_identifier(index: usize, identifier: char, buffer: &String) -> Option<(usize, Token)> {
    let mut index = index;
    while index < buffer.len() {
        let current_char = buffer.chars().nth(index).expect("No closer was found");
        
        let mut data_buffer = String::new(); 
        
        // if start is a string / char
        if current_char == identifier {
            // skip the first "
            index += 1;
            while (buffer.chars().nth(index).expect("No closer was found") != identifier) && (index < buffer.len()) {
                data_buffer.push(buffer.chars().nth(index).expect("No closer was found"));
                index += 1;
            }
            // if found end of string, push token and increment index
            if buffer.chars().nth(index).expect("No closer was found") == identifier {
                index += 1;
                if identifier == '\'' {
                    return Some((index, Token { Type: TokenType::CharLiteral, Data: Some(data_buffer) }));
                }
                else if identifier == '\"' {
                    return Some((index, Token { Type: TokenType::StrLiteral, Data: Some(data_buffer) }));
                }
            }
            // end of thing, not found the end of string
            else {
                panic!("Couldn't find end of literal");
            }
        }
        
        else {
            index += 1;
        }
    }
    None
}

fn is_not_continue_of_name(next_char: char) -> bool {
    let list_of_seperates: Vec<char> = vec!['[', ']', '(' , ')', '\'', '\"', '{', '}', ',', ';', ':'];
    list_of_seperates.contains(&next_char)
}

fn define_type_using_name(keyword: &String) -> TokenType {
    match is_keyword_alone(keyword) {
        Some(token_type) => token_type,
        None => panic!("wasn't a keyword, error"),
    }
}

fn is_keyword_contained(index: usize, buffer: &String) -> Option<(usize, Token)> {
    let list_of_keywords: Vec<&str> = vec!["dec", "ret", "if", "elif", "else", "int", "uint", "char", "str", "boolean", "double", ">=", "=>", "<=", "=<", "=="];
    
    for &prob_keyword in &list_of_keywords {
        if let Some(mut i) = buffer.find(prob_keyword) {
            // if that's the start of the buffer
            if i == index {
                i += prob_keyword.len();
                // if there is a continue to the buffer 
                if i <= (buffer.len() - 1) {
                    // check if the next chars are seperates or its a name
                    if is_not_continue_of_name(buffer.chars().nth(i).expect("Not a char")) {
                        i -= 1;
                        return Some((i, Token { Type: define_type_using_name(&prob_keyword.to_owned()) , Data: None }));
                    }
                }
                // no continue, it's the whole token
                else {
                    i -= 1;
                    return Some((i, Token { Type: define_type_using_name(&prob_keyword.to_owned()) , Data: None }));
                }
            }
        }
    }

    // not a keyword contained from this index
    None
}

// can be literal / name
fn push_literal(tokens: &mut Vec<Token>, literal: &mut String) {
    let mut token_type: TokenType = TokenType::NotInitiallized;
    if is_name_literal(&literal) {
        token_type = TokenType::NameLiteral;
    }
    // if it's a literal like true / false / int / uint / double (regex to check that as well) 
    else {
        if (literal == "true") || (literal == "false") {
            token_type = TokenType::BooleanLiteral;
        }
        else if is_int(&literal) {
            token_type = TokenType::IntLiteral;
        }
        else if is_double(&literal) {
            token_type = TokenType::DoubleLiteral;
        }
    }
    tokens.push(Token { Type: token_type, Data: Some(literal.to_string()) });
    literal.clear();
}

pub fn tokenize(buffer: &mut String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let buffer_split : Vec<String> = buffer.split_whitespace().map(str::to_string).collect();
    // iterate over the splitted buffer (using whitespaces)
    for maybe_token in buffer_split {
        println!("current token: {}", &maybe_token);
        // match if returned - if the standalone is a keyword
        let is_keyword = match is_keyword_alone(&maybe_token) {
            Some(token_type) => {
                tokens.push(Token { Type: token_type, Data: None });
                true
            },
            None => {
                false
            }
        };
        
        // if isn't a stand alone token - keyword
        if !is_keyword {
            // if that's an entire word (name like variable)
            if is_name_literal(&maybe_token) {
                tokens.push(Token { Type: TokenType::NameLiteral, Data: Some(maybe_token.to_owned()) });
            }
            else {
                let mut buffer_index = 0;
                // if a string at the start, get it and return the next char index (where the string ends)
                match get_token_from_identifier(0, '\"', &maybe_token) {
                    Some((index, token)) => {
                        tokens.push(token);
                        buffer_index = index;
                    },
                    None => {
                        // if not a string at the start, check for a char
                        match get_token_from_identifier(0, '\'', &maybe_token) {
                            Some((index, token)) => {
                                tokens.push(token);
                                buffer_index = index;
                            },
                            None => {},
                        }
                    },
                }

                let mut literal_buffer = String::new();

                // else, probably start with other token, name or [ { ( , : ;  
                while buffer_index < maybe_token.len() {

                    // if a single char is a keyword ( ) [ ] { } , : ; 
                    let is_single_char_keyword_alone = match is_keyword_alone(&(maybe_token.chars().nth(buffer_index).expect("No first char found").to_string().to_owned())) {
                        Some(token_type) => {
                            if !literal_buffer.is_empty() { push_literal(&mut tokens, &mut literal_buffer); }
                            tokens.push(Token { Type: token_type, Data: None });
                            true
                        },
                        None => {
                            false
                        }
                    };

                    // else, try to see if thats a string or char in the middle
                    if !is_single_char_keyword_alone {
                        match get_token_from_identifier(buffer_index, '\"', &maybe_token) {
                            Some((index, token)) => {
                                if !literal_buffer.is_empty() { push_literal(&mut tokens, &mut literal_buffer); }
                                tokens.push(token);
                                buffer_index = index;
                            },
                            None => {
                                // if not a string at the start, check for a char
                                match get_token_from_identifier(buffer_index, '\'', &maybe_token) {
                                    Some((index, token)) => {
                                        if !literal_buffer.is_empty() { push_literal(&mut tokens, &mut literal_buffer); }
                                        tokens.push(token);
                                        buffer_index = index;
                                    },
                                    None => {},
                                }
                            },
                        }
                        // try to see if that's a keyword like str or something like that
                        match is_keyword_contained(buffer_index, &maybe_token) {
                            Some((index, token)) => {
                                if !literal_buffer.is_empty() { push_literal(&mut tokens, &mut literal_buffer); }
                                tokens.push(token);
                                buffer_index = index;
                            },
                            None => {
                                literal_buffer.push(maybe_token.chars().nth(buffer_index).expect("No char was found"));
                            },
                        }  
                    }
                    buffer_index += 1;
                } 
            }
        }
    } 
    tokens
}