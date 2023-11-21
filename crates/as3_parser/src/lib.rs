#![feature(decl_macro)]
#![feature(try_blocks)]
#![feature(arbitrary_self_types)]

pub mod character_validation;
pub(crate) mod compiler_options;
pub(crate) mod comment;
pub(crate) mod diagnostics;
pub(crate) mod diagnostics_defaults;
pub(crate) mod errors;
pub(crate) mod operator;
pub(crate) mod operator_precedence;
pub(crate) mod parser;
pub(crate) mod token;
pub(crate) mod tokenizer;
pub(crate) mod location;
pub(crate) mod source;
pub mod ast;
pub mod keywords;
pub mod util;

pub use comment::*;
pub use compiler_options::*;
pub use diagnostics::*;
pub use errors::*;
pub use operator::*;
pub use operator_precedence::*;
pub use parser::*;
pub use token::*;
pub use tokenizer::*;
pub use location::*;
pub use source::*;