//! Umut A. Acar || Guy E. Blelloch.  Algorithm Design: Parallel and Sequential.  Definition 3.4
//! [SPARC expressions] (page 30)

type Id = String;
type Var = Id;
type TCon = Id;
type DCon = Id;

enum Pattern {
    Var(Var),
    Paren {
        inner: Box<Pattern>,
    },
    Pair {
        left: Box<Pattern>,
        right: Box<Pattern>,
    },
    DPat {
        dcon: DCon,
        inner: Box<Pattern>,
    },
}

enum Typ {
    Integer,
    Boolean,
    Product { inner: Vec<Box<Typ>> },
    Fun { lhs: Box<Typ>, rhs: Box<Typ> },
    TCon(TCon),
    DTyp { cons: Vec<(DCon, Box<Typ>)> },
}

enum UnaryOp {
    Not,
    Neg,
}

enum BinaryOp {
    And,
    Or,
    Xor,
    Plus,
    Minus,
    Times,
}

enum Value {
    Integer(i64),
    Boolean(bool),
    UnaryOp(UnaryOp),
    BinaryOp(BinaryOp),
    Pair { left: Box<Value>, right: Box<Value> },
    Paren { inner: Box<Value> },
    DCon { dcon: DCon, inner: Box<Value> },
    Lambda { var: Var, expr: Box<Expr> },
}

enum InfixOp {
    // TODO ??
}

enum Expr {
    Var(Var),
    Value(Box<Value>),
    Infix {
        op: InfixOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    SeqPair {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    ParPair {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Paren {
        inner: Box<Expr>,
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

enum Binding {
    VarBind { var: Var, expr: Box<Expr> },
    TypBind { var: TCon, typ: Box<Typ> },
}
