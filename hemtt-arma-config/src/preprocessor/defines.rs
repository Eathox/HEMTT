use std::{collections::HashMap, sync::Mutex};

use crate::tokenizer::tokens::TokenPair;

pub type DefineArgs<'a> = Vec<Vec<&'a TokenPair<'a>>>;

#[derive(Default)]
pub struct Defines<'a> {
    defines: Mutex<HashMap<String, Define<'a>>>,
}

impl<'a> Defines<'a> {
    pub fn new() -> Self {
        Defines {
            defines: Mutex::new(HashMap::new()),
        }
    }

    pub fn new_word(&self, word: &str, statement: Vec<&'a TokenPair<'a>>) -> Option<Define<'a>> {
        let mut defines = self.defines.lock().unwrap();
        defines.insert(word.to_string(), Define::new_word(statement))
    }

    pub fn remove(&self, word: &str) -> Option<Define<'a>> {
        let mut defines = self.defines.lock().unwrap();
        defines.remove(word)
    }

    pub fn new_function(
        &self,
        word: &str,
        args: DefineArgs<'a>,
        statement: Vec<&'a TokenPair<'a>>,
    ) -> Option<Define<'a>> {
        let mut defines = self.defines.lock().unwrap();
        defines.insert(word.to_string(), Define::new_function(args, statement))
    }

    #[must_use]
    pub fn get(&self, word: &str) -> Option<Define<'a>> {
        let defines = self.defines.lock().unwrap();
        defines.get(word).map(|d| d.to_owned())
    }

    #[must_use]
    pub fn contains(&self, word: &str) -> bool {
        let defines = self.defines.lock().unwrap();
        defines.contains_key(word)
    }
}

#[derive(Clone)]
pub struct Define<'a> {
    args: Option<DefineArgs<'a>>,
    statement: Vec<&'a TokenPair<'a>>,
}

impl<'a> Define<'a> {
    #[must_use]
    pub fn new_word(statement: Vec<&'a TokenPair<'a>>) -> Self {
        Define {
            args: None,
            statement,
        }
    }

    #[must_use]
    pub fn new_function(args: DefineArgs<'a>, statement: Vec<&'a TokenPair<'a>>) -> Self {
        Define {
            args: Some(args),
            statement,
        }
    }

    #[must_use]
    pub fn args(&self) -> Option<&Vec<Vec<&'a TokenPair>>> {
        self.args.as_ref()
    }

    #[must_use]
    pub fn statement(&self) -> Vec<TokenPair<'a>> {
        self.statement
            .clone()
            .into_iter()
            .map(|t| t.to_owned())
            .collect()
    }

    pub fn statement_ref(&self) -> Vec<&'a TokenPair<'a>> {
        self.statement.clone()
    }
}

#[cfg(test)]
mod tests {}
