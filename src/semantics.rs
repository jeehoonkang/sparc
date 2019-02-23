use std::cmp;
use std::collections::HashMap;
use std::sync::Arc;

use crate::arc_list::ArcList;
use crate::syntax::{Err, Expr, Value, Var};

struct Res<T> {
    result: T,
    work: u64,
    span: u64,
}

type EResult<T> = Result<Res<T>, Err>;

pub type EnvPiece = HashMap<Var, Arc<Value>>;
pub type Env = ArcList<EnvPiece>;

impl Env {
    fn lookup(&self, var: &Var) -> Result<Arc<Value>, Err> {
        for map in self.iter() {
            if let Some(res) = map.get(var) {
                return Ok(res.clone());
            }
        }

        Err(Err::EnvNotFound { var: var.clone() })
    }

    fn eval(&self, expr: &Expr) -> EResult<Arc<Value>> {
        match expr {
            Expr::Var(var) => Ok(Res {
                result: self.lookup(var)?,
                work: 1,
                span: 1,
            }),
            Expr::Value(value) => Ok(Res {
                result: value.clone(), // TODO: should make a closure
                work: 1,
                span: 1,
            }),
            Expr::UnaryOp { op, inner } => {
                let inner = self.eval(inner)?;
                let res = op.eval(&inner.result)?;

                Ok(Res {
                    result: Arc::new(res),
                    work: inner.work + 1,
                    span: inner.span + 1,
                })
            }
            Expr::BinaryOp { op, lhs, rhs } => {
                let lhs = self.eval(lhs)?;
                let rhs = self.eval(rhs)?;
                let res = op.eval(&lhs.result, &rhs.result)?;

                Ok(Res {
                    result: Arc::new(res),
                    work: lhs.work + rhs.work + 1,
                    span: cmp::max(lhs.span, rhs.span) + 1,
                })
            }
            Expr::SeqPair { lhs, rhs } => {
                let lhs = self.eval(lhs)?;
                let rhs = self.eval(rhs)?;

                Ok(Res {
                    result: Arc::new(Value::Pair {
                        lhs: lhs.result,
                        rhs: rhs.result,
                    }),
                    work: lhs.work + rhs.work + 1,
                    span: lhs.work + rhs.work + 1,
                })
            }
            Expr::ParPair { lhs, rhs } => {
                let lhs = self.eval(lhs)?;
                let rhs = self.eval(rhs)?;

                Ok(Res {
                    result: Arc::new(Value::Pair {
                        lhs: lhs.result,
                        rhs: rhs.result,
                    }),
                    work: lhs.work + rhs.work + 1,
                    span: cmp::max(lhs.work, rhs.work) + 1,
                })
            }
            Expr::Case { inner, patterns } => {
                let inner = self.eval(inner)?;

                for (pattern, expr) in patterns.iter() {
                    if let Ok(env_piece) = pattern.pattern_match(&inner.result) {
                        let env = self.clone().insert(env_piece);
                        let inner = env.eval(expr)?;

                        return Ok(Res {
                            result: inner.result,
                            work: inner.work + 1,
                            span: inner.span + 1,
                        });
                    }
                }

                Err(Err::CaseNoMatch {
                    inner: inner.result,
                    patterns: patterns.iter().map(|(p, _)| p.clone()).collect(),
                })
            }
            Expr::Ite { cond, lhs, rhs } => {
                let cond = self.eval(cond)?;
                let cond_result = cond
                    .result
                    .coerce_bool()
                    .ok_or(Err::InvalidIteCond { cond: cond.result })?;

                let body = if cond_result { lhs } else { rhs };
                let body = self.eval(body)?;

                Ok(Res {
                    result: body.result,
                    work: cond.work + body.work + 1,
                    span: cond.span + body.span + 1,
                })
            }
            Expr::App { lhs, rhs } => {
                let lhs = self.eval(lhs)?;
                let rhs = self.eval(rhs)?;

                let (lhs_pattern, lhs_expr) = match &*lhs.result {
                    Value::Lambda { pattern, expr } => (pattern, expr),
                    _ => Err(Err::InvalidAppArgs { inner: lhs.result })?,
                };

                let env_piece = lhs_pattern.pattern_match(&rhs.result)?;
                let env = self.clone().insert(env_piece);
                let app = env.eval(lhs_expr)?;

                Ok(Res {
                    result: app.result,
                    work: lhs.work + rhs.work + app.work + 1,
                    span: cmp::max(lhs.span, rhs.span) + app.span + 1,
                })
            }
            Expr::Let { binds, expr } => {
                let mut env_piece = HashMap::new();
                let mut work = 0;
                let mut span = 0;
                for binding in binds.iter() {
                    let res = self.eval(&binding.expr)?;
                    env_piece.insert(binding.var.clone(), res.result);
                    work += res.work;
                    span += res.span;
                }

                // TODO: should use the closure's environment
                let env = Env::new().insert(env_piece);
                let res = env.eval(expr)?;

                Ok(Res {
                    result: res.result,
                    work: work + res.work + 1,
                    span: span + res.span + 1,
                })
            }
        }
    }
}
