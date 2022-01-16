use std::collections::HashMap;

use crate::tokenizer::tokens::{Token, TokenPair};

pub use self::rendered::Rendered;

mod rendered;

pub fn render<'a>(source: &'a [&'a TokenPair<'a>]) -> Rendered {
    let mut map = HashMap::new();
    let mut line = Vec::new();
    let mut lc = 1;
    let mut cc = 1;
    for token in source {
        if token.token() == &Token::Newline {
            map.insert(lc, line);
            lc += 1;
            cc = 1;
            line = Vec::new();
        } else {
            line.push((
                cc,
                token.to_string().len(),
                token.path().to_owned(),
                token.token().clone(),
            ));
        }
    }
    Rendered::new(source, map)
}
