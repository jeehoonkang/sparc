use codespan::{CodeMap, Span};
use codespan_reporting::termcolor::StandardStream;
use codespan_reporting::{emit, ColorArg, Diagnostic, Label, Severity};
use std::fmt;
use std::str::FromStr;

use crate::parser::ExprParser;
use crate::semantics::Env;

/// SPARC expression executor.
pub struct Executor {
    parser: ExprParser,
    env: Env,
}

impl fmt::Debug for Executor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Executor")
    }
}

impl Executor {
    /// Creates a new executor.
    pub fn new() -> Self {
        Self {
            parser: ExprParser::new(),
            env: Env::new(),
        }
    }

    /// Executes an expression.
    pub fn exec(&self, input: &str) {
        let expr = match self.parser.parse(input) {
            Ok(expr) => expr,
            Err(e) => {
                let mut codemap = CodeMap::new();
                let _filemap = codemap.add_filemap("input".into(), input.into());
                let writer = StandardStream::stderr(ColorArg::from_str("auto").unwrap().into());
                let error = match e {
                    lalrpop_util::ParseError::InvalidToken { location } => {
                        Diagnostic::new(Severity::Error, "Invalid token").with_label(
                            Label::new_primary(Span::from_offset(
                                (location as u32).into(),
                                1.into(),
                            )),
                        )
                    }
                    lalrpop_util::ParseError::UnrecognizedToken { token, .. } => {
                        let error = Diagnostic::new(Severity::Error, "Unrecognized token");
                        if let Some((start, _, end)) = token {
                            error.with_label(Label::new_primary(Span::new(
                                ((start + 1) as u32).into(),
                                ((end + 1) as u32).into(),
                            )))
                        } else {
                            error.with_label(Label::new_primary(Span::from_offset(
                                (input.len() as u32).into(),
                                1.into(),
                            )))
                        }
                    }
                    _ => Diagnostic::new(Severity::Error, format!("Unknown parse error: {}", e)),
                };
                emit(&mut writer.lock(), &codemap, &error).unwrap();
                return;
            }
        };

        let result = match self.env.eval_expr(&expr) {
            Ok(result) => result,
            Err(e) => {
                eprintln!("Runtime error: {:?}", e);
                return;
            }
        };

        println!("Result: {:?}\nWork: {}\nSpan: {}", result.result, result.work, result.span);
    }
}
