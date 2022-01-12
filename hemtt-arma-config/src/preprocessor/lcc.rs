use std::fmt::{Error, Formatter};

use crate::tokenizer::tokens::{Token, TokenPair};

pub struct LineColCounter {
    pos: usize,
    line: usize,
    col: usize,

    newline: bool,
}

impl LineColCounter {
    #[must_use]
    pub fn new() -> Self {
        Self {
            line: 1,
            col: 1,
            pos: 1,
            newline: true,
        }
    }

    pub fn mod_cols(&mut self, t: &TokenPair) {
        t.set_start(self.pos_linecol());
        self.add_cols(t.token());
        t.set_end(self.pos_linecol());
    }

    pub fn add_cols(&mut self, t: &Token) {
        let c = t.size();
        self.col += c;
        self.pos += c;

        if self.newline && !t.is_whitespace() {
            self.newline = false;
        }
    }

    pub fn add_line(&mut self) {
        self.line += 1;
        self.newline = true;
        self.pos += 1;
        self.col = 1;
    }

    #[must_use]
    pub fn newline(&self) -> bool {
        self.newline
    }

    #[must_use]
    pub fn pos(&self) -> usize {
        self.pos
    }

    #[must_use]
    pub fn cols(&self) -> usize {
        self.col
    }

    #[must_use]
    pub fn line(&self) -> usize {
        self.line
    }

    #[must_use]
    pub fn pos_linecol(&self) -> (usize, (usize, usize)) {
        (self.pos, (self.line, self.col))
    }
}

impl std::fmt::Debug for LineColCounter {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}:{} ({})", self.line, self.col, self.pos)
    }
}
