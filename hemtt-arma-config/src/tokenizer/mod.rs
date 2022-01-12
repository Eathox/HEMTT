use pest::{iterators::Pair, Parser};

use self::tokens::{Token, TokenPair, Whitespace};

pub mod tokens;

#[derive(Parser)]
#[grammar = "tokenizer/arma-config.pest"]
pub struct Tokenizer;

pub struct Tokens<'a> {
    path: &'a str,
    source: &'a str,
    tokens: Vec<TokenPair<'a>>,
}

impl<'a> Tokens<'a> {
    pub fn new(path: &'a str, source: &'a str) -> Result<Self, String> {
        let mut tokens = Vec::new();
        let pairs = Tokenizer::parse(Rule::file, source).map_err(|e| e.to_string())?;
        for pair in pairs {
            tokens.push(TokenPair::new(path, pair)?);
        }

        Ok(Tokens {
            path,
            source,
            tokens,
        })
    }

    #[must_use]
    pub fn from_vec(path: &'a str, source: &'a str, tokens: Vec<TokenPair<'a>>) -> Self {
        Tokens {
            path,
            source,
            tokens,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &TokenPair> {
        self.tokens.iter()
    }

    #[must_use]
    pub const fn path(&self) -> &str {
        self.path
    }

    #[must_use]
    pub const fn source(&self) -> &str {
        self.source
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }
}

pub fn tokenize<'a>(source: &'a str, path: &'a str) -> Result<Tokens<'a>, String> {
    let mut tokens = Vec::new();

    let pairs = Tokenizer::parse(Rule::file, source).map_err(|e| e.to_string())?;
    for pair in pairs {
        tokens.push(TokenPair::new(path, pair)?);
    }

    Ok(Tokens {
        path,
        source,
        tokens,
    })
}

impl TryFrom<Pair<'_, Rule>> for Token {
    type Error = String;
    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        Ok(match pair.as_rule() {
            Rule::word => Self::from_word(pair.as_str().to_string()),
            Rule::alpha => Self::Alpha(
                pair.as_str()
                    .chars()
                    .next()
                    .ok_or_else(|| String::from("No char"))?,
            ),
            Rule::digit => Self::Digit(
                pair.as_str()
                    .parse::<u8>()
                    .map_err(|_| String::from("Invalid digit"))?,
            ),
            Rule::underscore => Self::Underscore,
            Rule::dash => Self::Dash,
            Rule::assignment => Self::Assignment,
            Rule::left_brace => Self::LeftBrace,
            Rule::right_brace => Self::RightBrace,
            Rule::left_bracket => Self::LeftBracket,
            Rule::right_bracket => Self::RightBracket,
            Rule::left_parentheses => Self::LeftParenthesis,
            Rule::right_parentheses => Self::RightParenthesis,
            Rule::colon => Self::Colon,
            Rule::semicolon => Self::Semicolon,
            Rule::directive => Self::Directive,
            Rule::escape => Self::Escape,
            Rule::comma => Self::Comma,
            Rule::decimal => Self::Decimal,
            Rule::double_quote => Self::DoubleQuote,
            Rule::single_quote => Self::SingleQuote,
            Rule::char => Self::Char(
                pair.as_str()
                    .chars()
                    .next()
                    .ok_or_else(|| String::from("No char"))?,
            ),

            Rule::newline => Self::Newline,
            Rule::space => Self::Whitespace(Whitespace::Space),
            Rule::tab => Self::Whitespace(Whitespace::Tab),
            Rule::WHITESPACE => Self::try_from(
                pair.into_inner()
                    .next()
                    .ok_or_else(|| String::from("No inner"))?,
            )
            .unwrap(),
            Rule::EOI => Self::EOI,

            Rule::file => panic!("Unexpected attempt to tokenize file"),
            Rule::COMMENT => panic!("Unexpected attempt to tokenize comment"),
            // _ => panic!("Unknown: {:?}", pair),
        })
    }
}
