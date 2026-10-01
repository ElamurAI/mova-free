//! Symbolic core: an expression is a tree. Rational constants, symbols (x, y…), constants (π, e),
//! operations, functions (sin, cos, exp, ln, sqrt…), aggregates and probabilities; units and absolute
//! temperatures are nodes of the same tree. Numerical evaluation (`eval`) is only one of the operations on
//! the tree; others are printing, derivative (`diff`), simplification, dimension checking.

use std::fmt;

use crate::rat::Rat;
use crate::units::{Mono, Scale};

/// Symbol-variable: index in the name table (`x`, `y`, `z`, `t`, `u`, `v`, `w`, `n`).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Var(pub u8);

pub const VAR_NAMES: [&str; 8] = ["x", "y", "z", "t", "u", "v", "w", "n"];

impl Var {
    pub fn name(self) -> &'static str {
        VAR_NAMES.get(self.0 as usize).copied().unwrap_or("?")
    }
    pub fn parse(s: &str) -> Option<Var> {
        VAR_NAMES.iter().position(|&n| n == s).map(|i| Var(i as u8))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Const {
    Pi,
    E,
}

/// Functions: elementary (symbolic) and aggregates/percentages (over quantities).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Func {
    Sin,
    Cos,
    Tan,
    Exp,
    Ln,
    Sqrt,
    Abs,
    Sum,
    Mean,
    Median,
    Min,
    Max,
    Range,
    /// p % of x.
    PctOf,
    /// Percentage change from a to b: (b − a)/a.
    PctChange,
    /// What share (%) a is of b.
    PctRatio,
    /// Absolute difference: «difference between a and b».
    AbsDiff,
}

impl Func {
    pub fn name(self) -> &'static str {
        match self {
            Func::Sin => "sin",
            Func::Cos => "cos",
            Func::Tan => "tan",
            Func::Exp => "exp",
            Func::Ln => "ln",
            Func::Sqrt => "sqrt",
            Func::Abs => "abs",
            Func::Sum => "sum",
            Func::Mean => "mean",
            Func::Median => "median",
            Func::Min => "min",
            Func::Max => "max",
            Func::Range => "range",
            Func::PctOf => "pct_of",
            Func::PctChange => "pct_change",
            Func::PctRatio => "pct_ratio",
            Func::AbsDiff => "abs_diff",
        }
    }
    pub fn is_elementary(self) -> bool {
        matches!(self, Func::Sin | Func::Cos | Func::Tan | Func::Exp | Func::Ln | Func::Sqrt | Func::Abs)
    }
}

/// Probability for independent events with probabilities pᵢ (each — `k` times).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum ProbOp {
    /// 1 − ∏(1 − pᵢ)
    AtLeastOnce,
    /// ∏(1 − pᵢ)
    NoneOf,
    /// ∏ pᵢ
    AllOf,
}

/// Conversion target: a multiplicative unit or a temperature scale (the kind — point or difference — comes from the source).
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum Target {
    Mono(Mono),
    Scale(Scale),
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Target::Mono(m) => write!(f, "{m}"),
            Target::Scale(s) => f.write_str(s.sym()),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Expr {
    Num(Rat),
    Var(Var),
    Const(Const),
    /// Quantity: expression × product of units (5 km, 10 m/s, 12 Δ°F).
    Qty(Box<Expr>, Mono),
    /// Absolute temperature — a point of an affine scale (72 °F).
    Temp(Box<Expr>, Scale),
    Neg(Box<Expr>),
    Add(Vec<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Vec<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Call(Func, Vec<Expr>),
    Convert(Box<Expr>, Target),
    /// Probability: groups (p, how many times).
    Prob(ProbOp, Vec<(Expr, Expr)>),
}

pub fn num(n: i128) -> Expr {
    Expr::Num(Rat::int(n))
}

pub fn rat(r: Rat) -> Expr {
    Expr::Num(r)
}

impl Expr {
    pub fn bx(self) -> Box<Expr> {
        Box::new(self)
    }
    pub fn add(a: Expr, b: Expr) -> Expr {
        Expr::Add(vec![a, b])
    }
    pub fn sub(a: Expr, b: Expr) -> Expr {
        Expr::Sub(a.bx(), b.bx())
    }
    pub fn mul(a: Expr, b: Expr) -> Expr {
        Expr::Mul(vec![a, b])
    }
    pub fn div(a: Expr, b: Expr) -> Expr {
        Expr::Div(a.bx(), b.bx())
    }
    pub fn pow(a: Expr, b: Expr) -> Expr {
        Expr::Pow(a.bx(), b.bx())
    }
    pub fn call(f: Func, args: Vec<Expr>) -> Expr {
        Expr::Call(f, args)
    }

    /// Precedence for printing: the larger, the tighter.
    fn prec(&self) -> u8 {
        match self {
            Expr::Add(_) | Expr::Sub(..) => 1,
            Expr::Mul(_) | Expr::Div(..) => 2,
            Expr::Neg(_) => 3,
            Expr::Pow(..) => 4,
            Expr::Num(r) if r.is_neg() => 0,
            Expr::Num(r) if !r.is_int() => 2,
            Expr::Qty(..) | Expr::Temp(..) => 2,
            Expr::Convert(..) => 0,
            _ => 5,
        }
    }

    fn wrap(&self, f: &mut fmt::Formatter, min: u8) -> fmt::Result {
        if self.prec() < min { write!(f, "({self})") } else { write!(f, "{self}") }
    }

    /// Whether it contains the variable.
    pub fn has_var(&self, v: Var) -> bool {
        match self {
            Expr::Var(x) => *x == v,
            Expr::Num(_) | Expr::Const(_) => false,
            Expr::Qty(e, _) | Expr::Temp(e, _) | Expr::Neg(e) | Expr::Convert(e, _) => e.has_var(v),
            Expr::Add(xs) | Expr::Mul(xs) | Expr::Call(_, xs) => xs.iter().any(|x| x.has_var(v)),
            Expr::Sub(a, b) | Expr::Div(a, b) | Expr::Pow(a, b) => a.has_var(v) || b.has_var(v),
            Expr::Prob(_, gs) => gs.iter().any(|(p, k)| p.has_var(v) || k.has_var(v)),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Num(r) => {
                match r.exact_decimal(6) {
                    Some(s) => f.write_str(&s),
                    None => write!(f, "{r}"),
                }
            }
            Expr::Var(v) => f.write_str(v.name()),
            Expr::Const(Const::Pi) => f.write_str("π"),
            Expr::Const(Const::E) => f.write_str("e"),
            Expr::Qty(e, m) => {
                if matches!(**e, Expr::Num(_)) { write!(f, "{e}")? } else { e.wrap(f, 3)? }
                if m.is_pct() { write!(f, "%") } else { write!(f, " {m}") }
            }
            Expr::Temp(e, s) => {
                if matches!(**e, Expr::Num(_)) { write!(f, "{e}")? } else { e.wrap(f, 3)? }
                write!(f, " {}", s.sym())
            }
            Expr::Neg(e) => {
                f.write_str("−")?;
                e.wrap(f, 3)
            }
            Expr::Add(xs) => {
                for (i, x) in xs.iter().enumerate() {
                    if i > 0 {
                        f.write_str(" + ")?;
                    }
                    x.wrap(f, 1)?;
                }
                Ok(())
            }
            Expr::Sub(a, b) => {
                a.wrap(f, 1)?;
                f.write_str(" − ")?;
                b.wrap(f, 2)
            }
            Expr::Mul(xs) => {
                for (i, x) in xs.iter().enumerate() {
                    if i > 0 {
                        f.write_str(" · ")?;
                    }
                    x.wrap(f, 2)?;
                }
                Ok(())
            }
            Expr::Div(a, b) => {
                a.wrap(f, 2)?;
                f.write_str(" / ")?;
                b.wrap(f, 3)
            }
            Expr::Pow(a, b) => {
                a.wrap(f, 5)?;
                f.write_str("^")?;
                b.wrap(f, 5)
            }
            Expr::Call(fun, xs) => {
                write!(f, "{}(", fun.name())?;
                for (i, x) in xs.iter().enumerate() {
                    if i > 0 {
                        f.write_str(", ")?;
                    }
                    write!(f, "{x}")?;
                }
                f.write_str(")")
            }
            Expr::Convert(e, t) => write!(f, "{e} → {t}"),
            Expr::Prob(op, gs) => {
                let name = match op {
                    ProbOp::AtLeastOnce => "P(at least once)",
                    ProbOp::NoneOf => "P(none of)",
                    ProbOp::AllOf => "P(every time)",
                };
                write!(f, "{name}[")?;
                for (i, (p, k)) in gs.iter().enumerate() {
                    if i > 0 {
                        f.write_str("; ")?;
                    }
                    write!(f, "p={p}, n={k}")?;
                }
                f.write_str("]")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::Unit;

    #[test]
    fn prints_with_precedence() {
        let e = Expr::add(num(3), Expr::mul(num(4), num(2)));
        assert_eq!(e.to_string(), "3 + 4 · 2");
        let e = Expr::mul(Expr::add(num(3), num(4)), num(2));
        assert_eq!(e.to_string(), "(3 + 4) · 2");
        let e = Expr::sub(Expr::Temp(num(72).bx(), Scale::F), Expr::Temp(num(60).bx(), Scale::F));
        assert_eq!(e.to_string(), "72 °F − 60 °F");
        let e = Expr::Convert(Expr::Qty(num(10).bx(), Mono::of(Unit::Mi)).bx(), Target::Mono(Mono::of(Unit::Km)));
        assert_eq!(e.to_string(), "10 mi → km");
        let e = Expr::pow(Expr::Var(Var(0)), num(2));
        assert_eq!(e.to_string(), "x^2");
    }
}
