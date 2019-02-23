use std::rc::Rc;

pub type Var = String;
pub type Ctor = String;

#[derive(Debug, Clone)]
pub enum Pattern {
    Var(Var),
    Pair {
        lhs: Box<Pattern>,
        rhs: Box<Pattern>,
    },
    Ctor {
        ctor: Ctor,
        inner: Box<Pattern>,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Not,
    Neg,
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

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i64),
    Boolean(bool),
    Pair {
        lhs: Box<Value>,
        rhs: Box<Value>,
    },
    Ctor {
        ctor: Ctor,
        inner: Box<Value>,
    },
    Lambda {
        pattern: Rc<Pattern>,
        expr: Rc<Expr>,
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

#[derive(Debug, Clone)]
pub enum Expr {
    Var(Var),
    Value(Box<Value>),
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
        patterns: Vec<(Rc<Pattern>, Box<Expr>)>,
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
        binds: Vec<Bind>,
        expr: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub struct Bind {
    pub var: Var,
    pub expr: Box<Expr>,
}
