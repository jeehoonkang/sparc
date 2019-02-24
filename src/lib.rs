//! The SPARC language
//!
//! Reference: Umut A. Acar || Guy E. Blelloch.  Algorithm Design: Parallel and Sequential (Chapter
//! 3)

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

extern crate rayon;
#[macro_use]
extern crate lalrpop_util;

mod arc_list;
mod parser;
mod semantics;
mod syntax;

pub use parser::ExprParser;
pub use semantics::Env;
