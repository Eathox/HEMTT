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
    pub fn from_word<S: Into<String>>(word: S) -> Self {
        let word = word.into();
        match word.as_str() {
            "class" => Self::Keyword(Keyword::Class),
            "delete" => Self::Keyword(Keyword::Delete),
            "enum" => Self::Keyword(Keyword::Enum),
            _ => Self::Word(word),
        }
    }

    #[must_use]
    pub const fn is_whitespace(&self) -> bool {
        matches!(&self, Self::Whitespace(_))
    }

    #[must_use]
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
            Self::Keyword(k) => k.to_string(),
            Self::Word(w) => w.clone(),
            Self::Alpha(c) => c.to_string(),
            Self::Digit(d) => d.to_string(),
            Self::Underscore => "_".to_string(),
            Self::Dash => "-".to_string(),
            Self::Assignment => "=".to_string(),
            Self::LeftBrace => "{".to_string(),
            Self::RightBrace => "}".to_string(),
            Self::LeftBracket => "[".to_string(),
            Self::RightBracket => "]".to_string(),
            Self::LeftParenthesis => "(".to_string(),
            Self::RightParenthesis => ")".to_string(),
            Self::Colon => ":".to_string(),
            Self::Semicolon => ";".to_string(),
            Self::Directive => "#".to_string(),
            Self::Escape => "\\".to_string(),
            Self::Comma => ",".to_string(),
            Self::Decimal => ".".to_string(),
            Self::DoubleQuote => "\"".to_string(),
            Self::SingleQuote => "'".to_string(),
            Self::Char(c) => c.to_string(),
            Self::Newline => "\n".to_string(),
            Self::Whitespace(w) => w.to_string(),
            Self::EOI => "".to_string(),
        }
    }
}
