use crate::tokenizer::{tokens::TokenPair, Tokens};

pub use self::defines::Defines;

mod defines;
mod functions;
mod ifstate;
mod lcc;

pub struct Preprocessor<'a> {
    source: &'a Tokens<'a>,
    output: Vec<TokenPair<'a>>,
}

impl<'a, 'b> Preprocessor<'a> {
    pub fn execute(root: &'a Tokens<'a>, defines: &'b Defines<'a>) -> Result<Self, String> {
        Ok(Self {
            output: functions::process_tokens(root.iter(), defines)?,
            source: root,
        })
    }

    pub fn output(&'a self) -> &'a Vec<TokenPair<'a>> {
        &self.output
    }

    pub fn source(&'a self) -> &'a Tokens<'a> {
        self.source
    }
}
