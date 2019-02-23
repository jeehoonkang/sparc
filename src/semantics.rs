use std::cmp;
use std::collections::HashMap;
use std::sync::Arc;

use crate::syntax::{Err, Expr, Id, Value};

struct Res<T> {
    result: T,
    work: u64,
    span: u64,
}

type EResult<T> = Result<Res<T>, Err>;

struct Env {
    values: HashMap<Id, Arc<Value>>,
}

impl Env {
    fn lookup(&self, id: &Id) -> Result<Arc<Value>, Err> {
        unimplemented!()
    }

    fn eval(&self, expr: &Expr) -> EResult<Arc<Value>> {
        match expr {
            Expr::Var(var) => Ok(Res {
                result: self.lookup(var)?,
                work: 1,
                span: 1,
            }),
            Expr::Value(value) => Ok(Res {
                result: value.clone(),
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
            Expr::Case { inner, patterns } => unimplemented!(),
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

                let app: Res<Arc<Value>> = unimplemented!();

                Ok(Res {
                    result: app.result,
                    work: lhs.work + rhs.work + app.work + 1,
                    span: cmp::max(lhs.span, rhs.span) + app.span + 1,
                })
            }
            Expr::Let { bindings, expr } => {
                let mut values = HashMap::new();
                let mut work = 0;
                let mut span = 0;
                for binding in bindings.iter() {
                    let res = self.eval(&binding.expr)?;
                    values.insert(binding.var.clone(), res.result);
                    work += res.work;
                    span += res.span;
                }

                let env = Env { values };
                // TODO: chain environments
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
