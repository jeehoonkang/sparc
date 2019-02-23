#![warn(missing_debug_implementations)]

#[macro_use]
extern crate nom;

extern crate rayon;

pub mod parser; // TODO: should be private
mod semantics;
mod syntax;
