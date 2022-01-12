use std::collections::HashMap;

use crate::tokenizer::tokens::{Token, TokenPair};

pub type LineMap = Vec<(usize, usize, String, Token)>;

pub struct Rendered<'a> {
    tokens: &'a [TokenPair<'a>],
    map: HashMap<usize, LineMap>,
}

impl<'a> Rendered<'a> {
    pub fn new(tokens: &'a [TokenPair<'a>], map: HashMap<usize, LineMap>) -> Self {
        Self { tokens, map }
    }

    pub fn tokens(&self) -> &[TokenPair<'a>] {
        self.tokens
    }

    pub fn map(&self) -> &HashMap<usize, LineMap> {
        &self.map
    }

    pub fn export(&self) -> String {
        let mut content = String::new();
        for token in self.tokens {
            content.push_str(&token.to_string());
        }
        content
    }
}
