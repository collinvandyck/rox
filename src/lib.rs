#![allow(unused)]

pub mod class;
pub mod env;
pub mod expr;
pub mod func;
pub mod interpreter;
pub mod lox;
pub mod parser;
pub mod prelude;
pub mod scanner;
pub mod stmt;
pub mod value;

#[cfg(test)]
mod tests;
