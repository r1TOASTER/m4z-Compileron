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

            }
        }
    } 
    tokens
}