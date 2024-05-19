#![allow(unused)]

pub mod env;
pub mod expr;
pub mod func;
pub mod interpret;
pub mod lox;
pub mod parse;
pub mod prelude;
pub mod resolver;
pub mod scanner;
pub mod stmt;
pub mod tree;

#[cfg(test)]
mod tests;
