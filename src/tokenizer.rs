use crate::token::{Token, TokenType};

pub fn tokenize(buffer: &mut String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let buffer_split : Vec<String> = buffer.split_whitespace().map(str::to_string).collect();
    // iterate over the splitted buffer (using whitespaces)
    for maybe_token in buffer_split {
        println!("current token: {}", &maybe_token);
        if maybe_token.eq("dec") {
            tokens.push(Token { Type: TokenType::Dec, Data: None });
        }
        else if maybe_token.eq("fn") {
            tokens.push(Token { Type: TokenType::Fn, Data: None });
        }
        else if maybe_token.eq("ret") {
            tokens.push(Token { Type: TokenType::Ret, Data: None });
        }
        else if maybe_token.eq("int") {
            tokens.push(Token { Type: TokenType::IntKeyword, Data: None });
        }
        else if maybe_token.eq("uint") {
            tokens.push(Token { Type: TokenType::UIntKeyword, Data: None });
        }
        else if maybe_token.eq("char") {
            tokens.push(Token { Type: TokenType::CharKeyword, Data: None });
        }
        else if maybe_token.eq("str") {
            tokens.push(Token { Type: TokenType::StrKeyword, Data: None });
        }
        else if maybe_token.eq("boolean") {
            tokens.push(Token { Type: TokenType::BooleanKeyword, Data: None });
        }
        else if maybe_token.eq("double") {
            tokens.push(Token { Type: TokenType::DoubleKeyword, Data: None });
        }
        // Probably using (, ), {, }, [, ], ", ', ;, or so. Could be literal than ; or = than literal, and more
        else {

        }
    } 
    tokens
}