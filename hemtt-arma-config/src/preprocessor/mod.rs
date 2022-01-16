use crate::tokenizer::{tokens::TokenPair, Tokens};

pub use self::defines::Defines;
use self::{holder::Holder, lcc::LineColCounter};

mod defines;
mod functions;
mod holder;
mod ifstate;
mod lcc;

pub struct Preprocessor<'a> {
    source: &'a Tokens<'a>,
    output: Vec<&'a TokenPair<'a>>,
    holder: Holder<'a>,
}

impl<'a, 'b> Preprocessor<'a> {
    pub fn execute(root: &'a Tokens<'a>, defines: &'b mut Defines<'a>) -> Result<Self, String> {
        let mut lcc = LineColCounter::new();
        let mut p = Self {
            output: Vec::new(),
            source: root,
            holder: Holder::new(),
        };
        p.output = functions::process_tokens(root.iter(), &mut lcc, defines, &p.holder)?;
        Ok(p)
    }

    #[must_use]
    pub const fn output(&'a self) -> &'a Vec<&'a TokenPair<'a>> {
        &self.output
    }

    #[must_use]
    pub const fn source(&'a self) -> &'a Tokens<'a> {
        self.source
    }
}
