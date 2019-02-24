use std::sync::Arc;

use crate::parser::ExprParser;
use crate::semantics::Env;
use crate::semantics::{Res, Value};

pub struct Executor {
    parser: ExprParser,
    env: Env,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            parser: ExprParser::new(),
            env: Env::new(),
        }
    }

    pub fn exec(&self, file: &str) -> Result<Res<Arc<Value>>, String> {
        let expr = self
            .parser
            .parse(file)
            .map_err(|e| format!("Parse error: {}", e))?;

        let result = self
            .env
            .eval_expr(&expr)
            .map_err(|e| format!("Runtime error: {:?}", e))?;

        Ok(result)
    }
}
