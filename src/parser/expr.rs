use std::fmt;

#[derive(Debug)]
pub enum Expr {
    EqualExpr(EqualExpr),
    NotEqualExpr(NotEqualExpr),
    LessExpr(LessExpr),
    GreaterExpr(GreaterExpr),
    LessEqualExpr(LessEqualExpr),
    GreaterEqualExpr(GreaterEqualExpr),

    AndExpr(AndExpr),
    XorExpr(XorExpr),
    OrExpr(OrExpr),
    NotExpr(NotExpr),

    AddExpr(AddExpr),
    SubExpr(SubExpr),
    MultExpr(MultExpr),
    DivExpr(DivExpr),
    PowerExpr(PowerExpr),
    UnarySubExpr(UnarySubExpr),

    NumberLit(f64),
    StringLit(String),
    BooleanLit(bool),

    Variable(String),
    PropertyLookup(PropertyLookup),
}

impl fmt::Display for Expr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::EqualExpr(expr) => write!(fmt, "{}={}", expr.lhs, expr.rhs),
            Self::NotEqualExpr(expr) => write!(fmt, "{}!={}", expr.lhs, expr.rhs),
            Self::LessExpr(expr) => write!(fmt, "{}<{}", expr.lhs, expr.rhs),
            Self::LessEqualExpr(expr) => write!(fmt, "{}<={}", expr.lhs, expr.rhs),
            Self::GreaterExpr(expr) => write!(fmt, "{}>{}", expr.lhs, expr.rhs),
            Self::GreaterEqualExpr(expr) => write!(fmt, "{}>={}", expr.lhs, expr.rhs),
            Self::AndExpr(expr) => write!(fmt, "{} AND {}", expr.lhs, expr.rhs),
            Self::XorExpr(expr) => write!(fmt, "{} XOR {}", expr.lhs, expr.rhs),
            Self::OrExpr(expr) => write!(fmt, "{} OR {}", expr.lhs, expr.rhs),
            Self::NotExpr(expr) => write!(fmt, "NOT {}", expr.child),
            Self::AddExpr(expr) => write!(fmt, "{}+{}", expr.lhs, expr.rhs),
            Self::SubExpr(expr) => write!(fmt, "{}-{}", expr.lhs, expr.rhs),
            Self::MultExpr(expr) => write!(fmt, "{}*{}", expr.lhs, expr.rhs),
            Self::DivExpr(expr) => write!(fmt, "{}/{}", expr.lhs, expr.rhs),
            Self::PowerExpr(expr) => write!(fmt, "{}^{}", expr.lhs, expr.rhs),
            Self::UnarySubExpr(expr) => write!(fmt, "-{}", expr.child),
            Self::NumberLit(expr) => write!(fmt, "{}", expr),
            Self::StringLit(expr) => write!(fmt, "{}", expr),
            Self::BooleanLit(expr) => write!(fmt, "{}", expr),
            Self::Variable(expr) => write!(fmt, "{}", expr),
            Self::PropertyLookup(expr) => write!(fmt, "{}.{}", expr.child, expr.prop_name),
        }
    }
}

#[derive(Debug)]
pub struct EqualExpr {
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

#[derive(Debug)]
pub struct NotEqualExpr {
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

#[derive(Debug)]
pub struct LessExpr {
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

#[derive(Debug)]
pub struct LessEqualExpr {
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

#[derive(Debug)]
pub struct GreaterExpr {
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

#[derive(Debug)]
pub struct GreaterEqualExpr {
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

#[derive(Debug)]
pub struct AndExpr {
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

#[derive(Debug)]
pub struct OrExpr {
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

#[derive(Debug)]
pub struct XorExpr {
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

#[derive(Debug)]
pub struct NotExpr {
    pub child: Box<Expr>,
}

#[derive(Debug)]
pub struct AddExpr {
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

#[derive(Debug)]
pub struct SubExpr {
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

#[derive(Debug)]
pub struct MultExpr {
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

#[derive(Debug)]
pub struct DivExpr {
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

#[derive(Debug)]
pub struct PowerExpr {
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

#[derive(Debug)]
pub struct UnarySubExpr {
    pub child: Box<Expr>,
}

#[derive(Debug)]
pub struct PropertyLookup {
    pub child: Box<Expr>,
    pub prop_name: String,
}
