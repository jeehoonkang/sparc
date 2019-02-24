// TODO: what is `Arc` and what is `Rc`?

use std::cmp;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

use crate::arc_list::ArcList;
use crate::syntax::{BinaryOp, Ctor, Expr, Pattern, UnaryOp, Value as SynValue, Var};

#[derive(Debug, Clone)]
pub struct Res<T> {
    result: T,
    work: u64,
    span: u64,
}

#[derive(Debug, Clone)]
pub enum Err {
    InvalidIteCond {
        cond: Arc<Value>,
    },
    InvalidUnaryOpArgs {
        op: UnaryOp,
        inner: Arc<Value>,
    },
    InvalidBinaryOpArgs {
        op: BinaryOp,
        lhs: Arc<Value>,
        rhs: Arc<Value>,
    },
    InvalidAppArgs {
        inner: Arc<Value>,
    },
    CaseNoMatch {
        inner: Arc<Value>,
        patterns: Vec<Rc<Pattern>>,
    },
    EnvNotFound {
        var: Var,
    },
    PatternNotMatched {
        pattern: Pattern,
        value: Arc<Value>,
    },
    CtorNotMatched {
        ctor_pattern: Ctor,
        ctor_value: Ctor,
    },
}

type EResult<T> = Result<Res<T>, Err>;

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i64),
    Boolean(bool),
    Pair {
        lhs: Arc<Value>,
        rhs: Arc<Value>,
    },
    Ctor {
        ctor: Ctor,
        inner: Arc<Value>,
    },
    Lambda {
        pattern: Rc<Pattern>,
        expr: Rc<Expr>,
        env: Env,
    },
}

impl Value {
    pub fn coerce_integer(&self) -> Option<i64> {
        match self {
            Value::Integer(i) => Some(*i),
            _ => None,
        }
    }

    pub fn coerce_bool(&self) -> Option<bool> {
        match self {
            Value::Boolean(b) => Some(*b),
            _ => None,
        }
    }
}

pub type EnvPiece = HashMap<Var, Arc<Value>>;
pub type Env = ArcList<EnvPiece>;

impl Env {
    fn eval_var(&self, var: &Var) -> Result<Arc<Value>, Err> {
        for map in self.iter() {
            if let Some(res) = map.get(var) {
                return Ok(res.clone());
            }
        }

        Err(Err::EnvNotFound { var: var.clone() })
    }

    fn eval_value(&self, value: &SynValue) -> Result<Arc<Value>, Err> {
        match value {
            SynValue::Integer(inner) => Ok(Arc::new(Value::Integer(*inner))),
            SynValue::Boolean(inner) => Ok(Arc::new(Value::Boolean(*inner))),
            SynValue::Pair { lhs, rhs } => {
                let lhs = self.eval_value(lhs)?;
                let rhs = self.eval_value(rhs)?;
                Ok(Arc::new(Value::Pair { lhs, rhs }))
            }
            SynValue::Ctor { ctor, inner } => {
                let inner = self.eval_value(inner)?;
                Ok(Arc::new(Value::Ctor {
                    ctor: ctor.clone(),
                    inner,
                }))
            }
            SynValue::Lambda { pattern, expr } => Ok(Arc::new(Value::Lambda {
                pattern: pattern.clone(),
                expr: expr.clone(),
                env: self.clone(),
            })),
        }
    }

    fn eval_pattern_inner(
        &self,
        pattern: &Pattern,
        value: &Arc<Value>,
        env_piece: &mut EnvPiece,
    ) -> Result<(), Err> {
        match (pattern, &**value) {
            (Pattern::Var(var), _) => {
                env_piece.insert(var.clone(), value.clone());
                Ok(())
            }
            (
                Pattern::Pair {
                    lhs: lhs_pattern,
                    rhs: rhs_pattern,
                },
                Value::Pair {
                    lhs: lhs_value,
                    rhs: rhs_value,
                },
            ) => {
                self.eval_pattern_inner(lhs_pattern, lhs_value, env_piece)?;
                self.eval_pattern_inner(rhs_pattern, rhs_value, env_piece)?;
                Ok(())
            }
            (
                Pattern::Ctor {
                    ctor: ctor_pattern,
                    inner: inner_pattern,
                },
                Value::Ctor {
                    ctor: ctor_value,
                    inner: inner_value,
                },
            ) => {
                if ctor_pattern != ctor_value {
                    return Err(Err::CtorNotMatched {
                        ctor_pattern: ctor_pattern.clone(),
                        ctor_value: ctor_value.clone(),
                    });
                }
                self.eval_pattern_inner(inner_pattern, inner_value, env_piece)?;
                Ok(())
            }
            _ => Err(Err::PatternNotMatched {
                pattern: pattern.clone(),
                value: value.clone(),
            }),
        }
    }

    fn eval_pattern(&self, pattern: &Pattern, value: &Arc<Value>) -> Result<EnvPiece, Err> {
        let mut env_piece = EnvPiece::new();
        self.eval_pattern_inner(pattern, value, &mut env_piece)?;
        Ok(env_piece)
    }

    fn eval_unary_op(op: UnaryOp, inner: &Arc<Value>) -> Result<Value, Err> {
        match (op, &**inner) {
            (UnaryOp::Not, Value::Boolean(inner)) => Ok(Value::Boolean(!inner)),
            (UnaryOp::Neg, Value::Integer(inner)) => Ok(Value::Integer(-inner)),
            _ => Err(Err::InvalidUnaryOpArgs {
                op,
                inner: inner.clone(),
            }),
        }
    }

    fn eval_binary_op(op: BinaryOp, lhs: &Arc<Value>, rhs: &Arc<Value>) -> Result<Value, Err> {
        match (op, &**lhs, &**rhs) {
            (BinaryOp::Or, Value::Boolean(lhs), Value::Boolean(rhs)) => {
                Ok(Value::Boolean(*lhs || *rhs))
            }
            (BinaryOp::And, Value::Boolean(lhs), Value::Boolean(rhs)) => {
                Ok(Value::Boolean(*lhs && *rhs))
            }
            (BinaryOp::Xor, Value::Boolean(lhs), Value::Boolean(rhs)) => {
                Ok(Value::Boolean(*lhs ^ *rhs))
            }

            (BinaryOp::Plus, Value::Integer(lhs), Value::Integer(rhs)) => {
                Ok(Value::Integer(*lhs + *rhs))
            }
            (BinaryOp::Minus, Value::Integer(lhs), Value::Integer(rhs)) => {
                Ok(Value::Integer(*lhs - *rhs))
            }
            (BinaryOp::Times, Value::Integer(lhs), Value::Integer(rhs)) => {
                Ok(Value::Integer(*lhs * *rhs))
            }
            (BinaryOp::Over, Value::Integer(lhs), Value::Integer(rhs)) => {
                Ok(Value::Integer(*lhs / *rhs))
            }

            (BinaryOp::Equal, Value::Integer(lhs), Value::Integer(rhs)) => {
                Ok(Value::Boolean(*lhs == *rhs))
            }
            (BinaryOp::Less, Value::Integer(lhs), Value::Integer(rhs)) => {
                Ok(Value::Boolean(*lhs < *rhs))
            }
            (BinaryOp::Le, Value::Integer(lhs), Value::Integer(rhs)) => {
                Ok(Value::Boolean(*lhs <= *rhs))
            }

            (_, _, _) => Err(Err::InvalidBinaryOpArgs {
                op,
                lhs: lhs.clone(),
                rhs: rhs.clone(),
            }),
        }
    }

    pub fn eval_expr(&self, expr: &Expr) -> EResult<Arc<Value>> {
        match expr {
            Expr::Var(var) => Ok(Res {
                result: self.eval_var(var)?,
                work: 1,
                span: 1,
            }),
            Expr::Value(value) => Ok(Res {
                result: self.eval_value(value)?,
                work: 1,
                span: 1,
            }),
            Expr::UnaryOp { op, inner } => {
                let inner = self.eval_expr(inner)?;
                let res = Self::eval_unary_op(*op, &inner.result)?;

                Ok(Res {
                    result: Arc::new(res),
                    work: inner.work + 1,
                    span: inner.span + 1,
                })
            }
            Expr::BinaryOp { op, lhs, rhs } => {
                let lhs = self.eval_expr(lhs)?;
                let rhs = self.eval_expr(rhs)?;
                let res = Self::eval_binary_op(*op, &lhs.result, &rhs.result)?;

                Ok(Res {
                    result: Arc::new(res),
                    work: lhs.work + rhs.work + 1,
                    span: cmp::max(lhs.span, rhs.span) + 1,
                })
            }
            Expr::SeqPair { lhs, rhs } => {
                let lhs = self.eval_expr(lhs)?;
                let rhs = self.eval_expr(rhs)?;

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
                let lhs = self.eval_expr(lhs)?;
                let rhs = self.eval_expr(rhs)?;

                Ok(Res {
                    result: Arc::new(Value::Pair {
                        lhs: lhs.result,
                        rhs: rhs.result,
                    }),
                    work: lhs.work + rhs.work + 1,
                    span: cmp::max(lhs.work, rhs.work) + 1,
                })
            }
            Expr::Case { inner, arms } => {
                let inner = self.eval_expr(inner)?;

                for (pattern, expr) in arms.iter() {
                    if let Ok(env_piece) = self.eval_pattern(pattern, &inner.result) {
                        let env = self.clone().insert(env_piece);
                        let inner = env.eval_expr(expr)?;

                        return Ok(Res {
                            result: inner.result,
                            work: inner.work + 1,
                            span: inner.span + 1,
                        });
                    }
                }

                Err(Err::CaseNoMatch {
                    inner: inner.result,
                    patterns: arms.iter().map(|(p, _)| p.clone()).collect(),
                })
            }
            Expr::Ite { cond, lhs, rhs } => {
                let cond = self.eval_expr(cond)?;
                let cond_result = cond
                    .result
                    .coerce_bool()
                    .ok_or(Err::InvalidIteCond { cond: cond.result })?;

                let body = if cond_result { lhs } else { rhs };
                let body = self.eval_expr(body)?;

                Ok(Res {
                    result: body.result,
                    work: cond.work + body.work + 1,
                    span: cond.span + body.span + 1,
                })
            }
            Expr::App { lhs, rhs } => {
                let lhs = self.eval_expr(lhs)?;
                let rhs = self.eval_expr(rhs)?;

                let (lhs_pattern, lhs_expr, lhs_env) = match &*lhs.result {
                    Value::Lambda { pattern, expr, env } => (pattern, expr, env),
                    _ => Err(Err::InvalidAppArgs { inner: lhs.result })?,
                };

                let env_piece = self.eval_pattern(lhs_pattern, &rhs.result)?;
                let env = lhs_env.clone().insert(env_piece);
                let app = env.eval_expr(lhs_expr)?;

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
                for bind in binds.iter() {
                    let res = self.eval_expr(&bind.expr)?;
                    env_piece.insert(bind.var.clone(), res.result);
                    work += res.work;
                    span += res.span;
                }

                let env = self.clone().insert(env_piece);
                let res = env.eval_expr(expr)?;

                Ok(Res {
                    result: res.result,
                    work: work + res.work + 1,
                    span: span + res.span + 1,
                })
            }
        }
    }
}
