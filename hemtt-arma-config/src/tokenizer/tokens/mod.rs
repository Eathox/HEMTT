use super::Rule;

mod keyword;
mod pair;
mod whitespace;

pub use self::{keyword::Keyword, pair::TokenPair, whitespace::Whitespace};

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

    pub fn size(&self) -> usize {
        match self {
            Self::Keyword(k) => k.size(),
            Self::Word(word) => word.len(),
            Self::EOI => 0,
            _ => 1,
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
