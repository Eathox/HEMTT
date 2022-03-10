#[macro_use]
extern crate pest_derive;

#[macro_use]
extern crate log;

mod ast;
pub use ast::parse;

mod preprocess;
pub use preprocess::Preprocessor;

mod render;
pub use render::*;

pub mod tokenizer;

mod error;
mod linter;
pub mod rapify;
pub mod resolver;
pub mod simplify;

pub use error::ArmaConfigError;
pub use linter::{InheritanceStyle, LinterOptions};
