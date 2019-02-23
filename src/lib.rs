//! The SPARC language
//!
//! Reference: Umut A. Acar || Guy E. Blelloch.  Algorithm Design: Parallel and Sequential (Chapter
//! 3)

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

#[macro_use]
extern crate nom;

extern crate rayon;

mod arc_list;
pub mod parser;
mod semantics;
mod syntax;
