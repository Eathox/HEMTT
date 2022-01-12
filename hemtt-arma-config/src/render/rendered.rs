use std::collections::HashMap;

use crate::tokenizer::tokens::{Token, TokenPair};

pub type LineMap = Vec<(usize, usize, String, Token)>;

pub struct Rendered<'a> {
    tokens: &'a [TokenPair<'a>],
    map: HashMap<usize, LineMap>,
}

impl<'a> Rendered<'a> {
    #[must_use]
    pub const fn new(tokens: &'a [TokenPair<'a>], map: HashMap<usize, LineMap>) -> Self {
        Self { tokens, map }
    }

    #[must_use]
    pub const fn tokens(&self) -> &[TokenPair<'a>] {
        self.tokens
    }

    #[must_use]
    pub const fn map(&self) -> &HashMap<usize, LineMap> {
        &self.map
    }

    #[must_use]
    pub fn export(&self) -> String {
        let mut content = String::new();
        for token in self.tokens {
            content.push_str(&token.to_string());
        }
        content
    }
}
