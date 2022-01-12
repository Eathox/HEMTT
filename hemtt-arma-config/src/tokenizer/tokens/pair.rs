use std::sync::RwLock;

use pest::iterators::Pair;

use super::{Rule, Token};

#[derive(Debug)]
pub struct TokenPair<'a> {
    path: String,
    pair: Option<Pair<'a, Rule>>,
    token: Token,
    start: RwLock<Option<(usize, (usize, usize))>>,
    end: RwLock<Option<(usize, (usize, usize))>>,
}

impl<'a> TokenPair<'a> {
    pub fn new<S: Into<String>>(path: S, pair: Pair<'a, Rule>) -> Result<Self, String> {
        Ok(Self {
            path: path.into(),
            pair: Some(pair.clone()),
            token: Token::try_from(pair)?,
            start: RwLock::new(None),
            end: RwLock::new(None),
        })
    }

    #[must_use]
    pub fn anon(token: Token) -> Self {
        Self {
            path: String::new(),
            token,
            pair: None,
            start: RwLock::new(None),
            end: RwLock::new(None),
        }
    }

    pub fn with_pos(token: Token, token_pair: &Self) -> Self {
        Self {
            path: token_pair.path().to_string(),
            token,
            pair: token_pair.pair.clone(),
            start: RwLock::new(None),
            end: RwLock::new(None),
        }
    }

    pub fn start(&self) -> Result<(usize, (usize, usize)), String> {
        if let Ok(lock) = self.start.read() {
            if let Some(pos) = *lock {
                return Ok(pos);
            }
        }
        let pos = self.pair.as_ref().map_or((0, (0, 0)), |p| {
            (
                p.as_span().start_pos().pos(),
                p.as_span().start_pos().line_col(),
            )
        });
        self.start.write().map_err(|e| e.to_string())?.replace(pos);
        Ok(pos)
    }

    pub fn set_start(&self, pos: (usize, (usize, usize))) {
        self.start.write().unwrap().replace(pos);
    }

    pub fn end(&self) -> Result<(usize, (usize, usize)), String> {
        if let Ok(lock) = self.end.read() {
            if let Some(pos) = *lock {
                return Ok(pos);
            }
        }

        let pos = self.pair.as_ref().map_or((0, (0, 0)), |p| {
            (
                p.as_span().end_pos().pos(),
                p.as_span().end_pos().line_col(),
            )
        });
        self.end.write().map_err(|e| e.to_string())?.replace(pos);
        Ok(pos)
    }

    pub fn set_end(&self, pos: (usize, (usize, usize))) {
        self.end.write().unwrap().replace(pos);
    }

    pub fn pair(&self) -> Option<Pair<Rule>> {
        self.pair.clone()
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub const fn token(&self) -> &Token {
        &self.token
    }

    pub fn into_token(self) -> Token {
        self.token
    }
}

impl Clone for TokenPair<'_> {
    fn clone(&self) -> Self {
        Self {
            path: self.path.clone(),
            pair: self.pair.clone(),
            token: self.token.clone(),
            start: RwLock::new(None),
            end: RwLock::new(None),
        }
    }
}

impl ToString for TokenPair<'_> {
    fn to_string(&self) -> String {
        self.token().to_string()
    }
}
