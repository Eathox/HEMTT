use std::{collections::HashMap, sync::Mutex};

use crate::tokenizer::TokenPair;

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

    pub fn remove(&self, name: &str) {
        self.defines.lock().unwrap().remove(name);
    }

    pub fn defined(&self, name: &str) -> bool {
        self.defines.lock().unwrap().contains_key(name)
    }

    pub fn new_word(&self, word: &str, statement: Vec<&'a TokenPair<'a>>) -> Option<Define<'a>> {
        debug!("new word: {}", word);
        let mut defines = self.defines.lock().unwrap();
        defines.insert(word.to_string(), Define::Word(statement))
    }

    pub fn new_function(
        &self,
        word: &str,
        args: DefineArgs<'a>,
        statement: Vec<&'a TokenPair<'a>>,
    ) -> Option<Define<'a>> {
        debug!("new function: {}", word);
        let mut defines = self.defines.lock().unwrap();
        defines.insert(
            word.to_string(),
            Define::Function(FunctionMacro::new(args, statement)),
        )
    }

    pub fn new_define(&self, word: &str, define: Define<'a>) -> Option<Define<'a>> {
        debug!("new define: {}", word);
        let mut defines = self.defines.lock().unwrap();
        defines.insert(word.to_string(), define)
    }

    #[must_use]
    pub fn get(&self, word: &str) -> Option<Define<'a>> {
        let defines = self.defines.lock().unwrap();
        defines.get(word).map(|d| d.to_owned())
    }

    #[must_use]
    pub fn all(&self) -> Vec<(String, Define<'a>)> {
        let defines = self.defines.lock().unwrap();
        defines
            .iter()
            .map(|(k, v)| (k.to_owned(), v.to_owned()))
            .collect()
    }
}

impl std::fmt::Debug for Defines<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let defines = self.defines.lock().unwrap();
        f.debug_map().entries(defines.iter()).finish()
    }
}

#[derive(Clone, Debug)]
pub enum Define<'a> {
    Word(Vec<&'a TokenPair<'a>>),
    Function(FunctionMacro<'a>),
}

impl<'a> Define<'a> {
    pub fn statement(&self) -> Vec<&'a TokenPair<'a>> {
        match self {
            Define::Word(statement) => statement.clone(),
            Define::Function(function) => function.statement.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct FunctionMacro<'a> {
    args: DefineArgs<'a>,
    statement: Vec<&'a TokenPair<'a>>,
}

impl<'a> FunctionMacro<'a> {
    pub fn new(args: DefineArgs<'a>, statement: Vec<&'a TokenPair<'a>>) -> Self {
        FunctionMacro { args, statement }
    }

    pub fn args(&self) -> &DefineArgs<'a> {
        &self.args
    }

    pub fn statement(&self) -> &Vec<&'a TokenPair<'a>> {
        &self.statement
    }
}

// pub struct Define<'a> {
//     args: Option<DefineArgs<'a>>,
//     statement: Vec<&'a TokenPair<'a>>,
// }

// impl<'a> Define<'a> {
//     #[must_use]
//     pub fn new_word(statement: Vec<&'a TokenPair<'a>>) -> Self {
//         Define {
//             args: None,
//             statement,
//         }
//     }

//     #[must_use]
//     pub fn new_function(args: DefineArgs<'a>, statement: Vec<&'a TokenPair<'a>>) -> Self {
//         Define {
//             args: Some(args),
//             statement,
//         }
//     }

//     #[must_use]
//     pub fn args(&self) -> Option<&Vec<Vec<&'a TokenPair>>> {
//         self.args.as_ref()
//     }

//     #[must_use]
//     pub fn statement(&self) -> Vec<TokenPair<'a>> {
//         self.statement
//             .clone()
//             .into_iter()
//             .map(|t| t.to_owned())
//             .collect()
//     }

//     pub fn statement_ref(&self) -> Vec<&'a TokenPair<'a>> {
//         self.statement.clone()
//     }
// }

// #[cfg(test)]
// mod tests {}
