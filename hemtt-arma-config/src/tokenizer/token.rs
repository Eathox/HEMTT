use std::sync::Mutex;

use pest::iterators::Pair;

use super::{Keyword, Rule, Whitespace};

#[derive(Debug)]
pub struct TokenPair<'a> {
    path: String,
    pair: Option<Pair<'a, Rule>>,
    token: Token,
    start: Mutex<Option<(usize, (usize, usize))>>,
    end: Mutex<Option<(usize, (usize, usize))>>,
}

impl<'a> TokenPair<'a> {
    pub fn new<S: Into<String>>(path: S, pair: Pair<'a, Rule>) -> Self {
        Self {
            path: path.into(),
            pair: Some(pair.clone()),
            token: Token::from(pair),
            start: Mutex::new(None),
            end: Mutex::new(None),
        }
    }

    pub fn anon(token: Token) -> Self {
        Self {
            path: String::new(),
            token,
            pair: None,
            start: Mutex::new(None),
            end: Mutex::new(None),
        }
    }

    pub fn with_pos(token: Token, token_pair: &Self) -> Self {
        Self {
            path: token_pair.path().to_string(),
            token,
            pair: token_pair.pair.clone(),
            start: Mutex::new(None),
            end: Mutex::new(None),
        }
    }

    pub fn start(&self) -> (usize, (usize, usize)) {
        if let Some(pos) = *self.start.lock().unwrap() {
            return pos;
        }
        let pos = if let Some(p) = &self.pair {
            (
                p.as_span().start_pos().pos(),
                p.as_span().start_pos().line_col(),
            )
        } else {
            (0, (0, 0))
        };
        self.start.lock().unwrap().replace(pos);
        pos
    }

    pub fn set_start(&self, pos: (usize, (usize, usize))) {
        self.start.lock().unwrap().replace(pos);
    }

    pub fn end(&self) -> (usize, (usize, usize)) {
        if let Some(pos) = *self.end.lock().unwrap() {
            return pos;
        }
        let pos = if let Some(p) = &self.pair {
            (
                p.as_span().end_pos().pos(),
                p.as_span().end_pos().line_col(),
            )
        } else {
            (0, (0, 0))
        };
        self.end.lock().unwrap().replace(pos);
        pos
    }

    pub fn set_end(&self, pos: (usize, (usize, usize))) {
        self.end.lock().unwrap().replace(pos);
    }

    pub fn pair(&self) -> Option<Pair<Rule>> {
        self.pair.clone()
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn token(&self) -> &Token {
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
            start: Mutex::new(None),
            end: Mutex::new(None),
        }
    }
}

impl ToString for TokenPair<'_> {
    fn to_string(&self) -> String {
        self.token().to_string()
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "maps", derive(serde::Serialize))]
pub enum Token {
    Keyword(Keyword),
    Word(String),
    Alpha(char),
    Digit(u8),
    Underscore,
    Dash,
    Assignment,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    LeftParenthesis,
    RightParenthesis,
    Colon,
    Semicolon,
    Directive,
    Escape,
    Comma,
    Decimal,
    DoubleQuote,
    SingleQuote,
    Char(char),

    Newline,
    Whitespace(Whitespace),

    EOI,
}

impl Token {
    pub fn from_word<S: Into<String>>(word: S) -> Token {
        let word = word.into();
        match word.as_str() {
            "class" => Token::Keyword(Keyword::Class),
            "delete" => Token::Keyword(Keyword::Delete),
            "enum" => Token::Keyword(Keyword::Enum),
            _ => Token::Word(word),
        }
    }

    pub fn is_whitespace(&self) -> bool {
        matches!(&self, Self::Whitespace(_))
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Keyword(k) => k.len(),
            Self::Word(word) => word.len(),
            Self::EOI => 0,
            _ => 1,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::Keyword(_) => false,
            Self::Word(word) => word.is_empty(),
            Self::EOI => true,
            _ => false,
        }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Token::Keyword(k) => k.to_string(),
            Token::Word(w) => w.to_owned(),
            Token::Alpha(c) => c.to_string(),
            Token::Digit(d) => d.to_string(),
            Token::Underscore => "_".to_string(),
            Token::Dash => "-".to_string(),
            Token::Assignment => "=".to_string(),
            Token::LeftBrace => "{".to_string(),
            Token::RightBrace => "}".to_string(),
            Token::LeftBracket => "[".to_string(),
            Token::RightBracket => "]".to_string(),
            Token::LeftParenthesis => "(".to_string(),
            Token::RightParenthesis => ")".to_string(),
            Token::Colon => ":".to_string(),
            Token::Semicolon => ";".to_string(),
            Token::Directive => "#".to_string(),
            Token::Escape => "\\".to_string(),
            Token::Comma => ",".to_string(),
            Token::Decimal => ".".to_string(),
            Token::DoubleQuote => "\"".to_string(),
            Token::SingleQuote => "'".to_string(),
            Token::Char(c) => c.to_string(),
            Token::Newline => "\n".to_string(),
            Token::Whitespace(w) => w.to_string(),
            Token::EOI => "".to_string(),
        }
    }
}
