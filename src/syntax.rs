//! Umut A. Acar || Guy E. Blelloch.  Algorithm Design: Parallel and Sequential.  Definition 3.4
//! [SPARC expressions] (page 30)

// TODO: what is `Arc` and what is `Box`?

use std::sync::Arc;

#[derive(Debug)]
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
}

pub type Id = String;
pub type Var = Id;
pub type Ctor = Id;

#[derive(Debug, Clone)]
pub enum Pattern {
    Var(Var),
    Pair {
        lhs: Box<Pattern>,
        rhs: Box<Pattern>,
    },
    DPat {
        ctor: Ctor,
        inner: Box<Pattern>,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Not,
    Neg,
}

impl UnaryOp {
    pub fn eval(self, inner: &Value) -> Result<Value, Err> {
        match (self, inner) {
            (UnaryOp::Not, Value::Boolean(inner)) => Ok(Value::Boolean(!inner)),
            (UnaryOp::Neg, Value::Integer(inner)) => Ok(Value::Integer(-inner)),
            (_, _) => Err(Err::InvalidUnaryOpArgs {
                op: self,
                inner: Arc::new(inner.clone()),
            }),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOp {
    Or,
    And,
    Xor,

    Plus,
    Minus,
    Times,
    Over,

    Equal,
    Less,
    Le,
}

impl BinaryOp {
    pub fn eval(self, lhs: &Value, rhs: &Value) -> Result<Value, Err> {
        match (self, lhs, rhs) {
            (BinaryOp::Or, Value::Boolean(lhs), Value::Boolean(rhs)) => Ok(Value::Boolean(*lhs || *rhs)),
            (BinaryOp::And, Value::Boolean(lhs), Value::Boolean(rhs)) => Ok(Value::Boolean(*lhs && *rhs)),
            (BinaryOp::Xor, Value::Boolean(lhs), Value::Boolean(rhs)) => Ok(Value::Boolean(*lhs ^ *rhs)),

            (BinaryOp::Plus, Value::Integer(lhs), Value::Integer(rhs)) => Ok(Value::Integer(*lhs + *rhs)),
            (BinaryOp::Minus, Value::Integer(lhs), Value::Integer(rhs)) => Ok(Value::Integer(*lhs - *rhs)),
            (BinaryOp::Times, Value::Integer(lhs), Value::Integer(rhs)) => Ok(Value::Integer(*lhs * *rhs)),
            (BinaryOp::Over, Value::Integer(lhs), Value::Integer(rhs)) => Ok(Value::Integer(*lhs / *rhs)),

            (BinaryOp::Equal, Value::Integer(lhs), Value::Integer(rhs)) => Ok(Value::Boolean(*lhs == *rhs)),
            (BinaryOp::Less, Value::Integer(lhs), Value::Integer(rhs)) => Ok(Value::Boolean(*lhs < *rhs)),
            (BinaryOp::Le, Value::Integer(lhs), Value::Integer(rhs)) => Ok(Value::Boolean(*lhs <= *rhs)),

            (_, _, _) => Err(Err::InvalidBinaryOpArgs {
                op: self,
                lhs: Arc::new(lhs.clone()),
                rhs: Arc::new(rhs.clone()),
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i64),
    Boolean(bool),
    Pair { lhs: Arc<Value>, rhs: Arc<Value> },
    Ctor { ctor: Ctor, inner: Arc<Value> },
    Lambda { pattern: Pattern, expr: Box<Expr> },
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

#[derive(Debug, Clone)]
pub enum Expr {
    Var(Var),
    Value(Arc<Value>),
    UnaryOp {
        op: UnaryOp,
        inner: Box<Expr>,
    },
    BinaryOp {
        op: BinaryOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    SeqPair {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    ParPair {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Case {
        inner: Box<Expr>,
        patterns: Vec<(Pattern, Box<Expr>)>,
    },
    Ite {
        cond: Box<Expr>,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    App {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Let {
        bindings: Vec<Binding>,
        expr: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub struct Binding {
    pub var: Var,
    pub expr: Box<Expr>,
}
