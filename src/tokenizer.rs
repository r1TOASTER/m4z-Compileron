use crate::token::{Token, TokenType};

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

fn get_token_from_identifier(index: usize, identifier: char, buffer: &String) -> Option<(usize, Token)> {
    let mut index = index;
    while index < buffer.len() {
        let current_char = buffer.chars().nth(index).expect("No char found");
        
        let mut data_buffer = String::new(); 
        
        // if start of a string
        if current_char == identifier {
            // skip the first "
            index += 1;
            while (buffer.chars().nth(index).expect("No char found") != identifier) && (index < buffer.len()) {
                data_buffer.push(buffer.chars().nth(index).expect("No char found"));
                index += 1;
            }
            // if found end of string, push token and increment index
            if buffer.chars().nth(index).expect("No char found") == identifier {
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
                panic!("Couldn't find end of string");
            }
        }

        else {
            index += 1;
        }
    }
    None
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
            if maybe_token.chars().all(|b| matches!(b, 'A'..='Z' | 'a'..='z' | '0'..='9' | '_')) {
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
                // else, probably start with other token, name or [, {, (, ',', ', )}]
                while buffer_index < maybe_token.len() {
                    buffer_index += 1;
                } 
            }
        }
    } 
    tokens
}