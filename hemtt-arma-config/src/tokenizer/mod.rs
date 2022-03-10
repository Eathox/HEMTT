use pest::{error::Error, iterators::Pair, Parser};

mod keyword;
pub use keyword::Keyword;
mod token;
pub use token::{Token, TokenPair};
mod whitespace;
pub use whitespace::Whitespace;

#[derive(Parser)]
#[grammar = "tokenizer/token.pest"]
pub struct Tokenizer;

pub struct Tokens<'a> {
    path: String,
    source: &'a str,
    tokens: Vec<TokenPair<'a>>,
}

impl<'a> Tokens<'a> {
    pub fn new<S: Into<String> + Copy>(path: S, source: &'a str) -> Result<Self, Error<Rule>> {
        let mut tokens = Vec::new();
        let pairs = Tokenizer::parse(Rule::file, source)?;
        for pair in pairs {
            tokens.push(TokenPair::new(path, pair))
        }

        Ok(Tokens {
            path: path.into(),
            source,
            tokens,
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = &TokenPair> {
        self.tokens.iter()
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn source(&self) -> &str {
        self.source
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }
}

pub fn tokenize<'a>(source: &'a str, path: &str) -> Result<Tokens<'a>, Error<Rule>> {
    let mut tokens = Vec::new();

    let pairs = Tokenizer::parse(Rule::file, source)?;
    for pair in pairs {
        tokens.push(TokenPair::new(path, pair))
    }

    Ok(Tokens {
        path: path.to_string(),
        source,
        tokens,
    })
}

impl From<Pair<'_, Rule>> for Token {
    fn from(pair: Pair<Rule>) -> Token {
        match pair.as_rule() {
            Rule::word => Token::from_word(pair.as_str().to_string()),
            Rule::alpha => Token::Alpha(pair.as_str().chars().next().unwrap()),
            Rule::digit => Token::Digit(pair.as_str().parse::<u8>().unwrap()),
            Rule::underscore => Token::Underscore,
            Rule::dash => Token::Dash,
            Rule::assignment => Token::Assignment,
            Rule::left_brace => Token::LeftBrace,
            Rule::right_brace => Token::RightBrace,
            Rule::left_bracket => Token::LeftBracket,
            Rule::right_bracket => Token::RightBracket,
            Rule::left_parentheses => Token::LeftParenthesis,
            Rule::right_parentheses => Token::RightParenthesis,
            Rule::colon => Token::Colon,
            Rule::semicolon => Token::Semicolon,
            Rule::directive => Token::Directive,
            Rule::escape => Token::Escape,
            Rule::comma => Token::Comma,
            Rule::decimal => Token::Decimal,
            Rule::double_quote => Token::DoubleQuote,
            Rule::single_quote => Token::SingleQuote,
            Rule::char => Token::Char(pair.as_str().chars().next().unwrap()),

            Rule::newline => Token::Newline,
            Rule::space => Token::Whitespace(Whitespace::Space),
            Rule::tab => Token::Whitespace(Whitespace::Tab),
            Rule::WHITESPACE => Token::from(pair.into_inner().next().unwrap()),
            Rule::EOI => Token::EOI,

            Rule::file => panic!("Unexpected attempt to tokenize file"),
            Rule::COMMENT => panic!("Unexpected attempt to tokenize comment"),
            // _ => panic!("Unknown: {:?}", pair),
        }
    }
}
