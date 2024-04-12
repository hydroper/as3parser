//! Defines the parser and the tokenizer.
//!
//! Using the methods of the `ParserFacade` structure is the most common way of parsing
//! programs until end-of-file.

mod character_validator;
pub use character_validator::*;
mod contexts;
pub use contexts::*;
mod as3_reserved_word;
pub use as3_reserved_word::*;
mod parser;
pub use parser::*;
mod parsing_failure;
pub use parsing_failure::*;
mod token;
pub use token::*;
mod tokenizer;
pub use tokenizer::*;