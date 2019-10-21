//! Модуль по парсингу графа из GraphML формата.

#[path = "errors.rs"]
pub mod errors;

#[path = "types.rs"]
pub mod types;

#[path = "parser_impl.rs"]
pub mod parser_impl;

#[macro_use]
extern crate mdo;

/// Парсер графа из GraphML формата.
pub mod parser {
    pub use crate::errors::*;
    pub use crate::parser_impl::read_graphml;
    pub use crate::types::*;
}
