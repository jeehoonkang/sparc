// TODO: "In SPARC, variables, type constructors, and data constructors are given a name, or an
// identifier. An identifer consist of only alphabetic and numeric characters (a-z, A-Z, 0-9), the
// underscore character (“ ”), and optionally end with some number of “primes”. Example identifiers
// include, x′, x1, xl, myV ar, myT ype, myData, and my data."

use crate::syntax::*;

lalrpop_mod!(parser_inner); // synthesized by LALRPOP

pub use parser_inner::ExprParser;
