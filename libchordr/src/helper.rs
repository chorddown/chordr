use crate::tokenizer::{Token, TokenLine};

pub fn token_lines_to_tokens(token_lines: Vec<TokenLine>) -> Vec<Token> {
    let mut stream = vec![];
    for line in token_lines {
        for token in line {
            stream.push(token);
        }

        stream.push(Token::Newline);
    }

    stream
}
