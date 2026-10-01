//! Bridge to `math::Expr` (feature `math`) for a subset: numbers, variables, arithmetic, fractions, roots,
//! functions; separately, a chain of relations (`a = b < c`).
//!
//! The LaTeX tree is presentational (atoms in a row), so operator precedence is added here:
//! `+ −` < `· × * / ÷` and implicit multiplication < unary minus < power (a script in the tree) < atom.
//! Brackets `( )`, `[ ]`, `\left( \right)` group; `|x|` is absolute value; `(x+1)^2`: in TeX the script attaches
//! to `)`, so a closing bracket with a script closes the group and raises it to the power.
//! Function rule: `\sin 2x` = sin(2x), `\sin x \cos x` = sin(x)·cos(x), `\sin^2 x` = (sin x)².
//! Literals: `\frac{1}{2}` (an irreducible fraction of integers) is a rational number; `-3` is a negative number;
//! `e` is Euler's constant; `\pi` is π. Variables are only those that exist in `math` (x y z t u v w n).
//!
//! The reverse side, `from_expr`, prints an `Expr` as a LaTeX tree (steps and answers of `math`); the round trip
//! `to_expr(parse(to_latex(from_expr(e)))) == e` is checked by a test (with normalization `Div(1, 2)` → `1/2`).

use std::fmt;

use math::expr::{Const, Expr, Func, Var};
use math::rat::Rat;

use crate::ast::{Node, Scripts};
use crate::table::{Class, Font, FracKind, Sym};

/// Why a formula does not convert to `Expr`: kind and detail.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unsupported {
    kind: &'static str,
    detail: String,
}

impl Unsupported {
    fn new(kind: &'static str, detail: impl Into<String>) -> Unsupported {
        Unsupported { kind, detail: detail.into() }
    }
    /// Kind of reason (for statistics): "relation", "variable", "script", "function", "construct"…
    pub fn kind(&self) -> &'static str {
        self.kind
    }
}

impl fmt::Display for Unsupported {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.detail)
    }
}

type R<X> = Result<X, Unsupported>;

/// A relation in a chain.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rel {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Approx,
}

impl Rel {
    fn of(s: Sym) -> Option<Rel> {
        Some(match s {
            Sym::Eq => Rel::Eq,
            Sym::Ne => Rel::Ne,
            Sym::Lt => Rel::Lt,
            Sym::Le | Sym::Leqslant | Sym::Leqq => Rel::Le,
            Sym::Gt => Rel::Gt,
            Sym::Ge | Sym::Geqslant | Sym::Geqq => Rel::Ge,
            Sym::Approx => Rel::Approx,
            _ => return None,
        })
    }
    pub fn sym(self) -> Sym {
        match self {
            Rel::Eq => Sym::Eq,
            Rel::Ne => Sym::Ne,
            Rel::Lt => Sym::Lt,
            Rel::Le => Sym::Le,
            Rel::Gt => Sym::Gt,
            Rel::Ge => Sym::Ge,
            Rel::Approx => Sym::Approx,
        }
    }
}

/// Expression (without relations) → `math::Expr`.
pub fn to_expr(n: &Node) -> R<Expr> {
    let items = n.items();
    if let Some(r) = items.iter().find_map(|x| match x {
        Node::Sym(s) if s.class() == Class::Rel => Some(*s),
        _ => None,
    }) {
        return Err(Unsupported::new("relation", r.latex()));
    }
    seq(items)
}

/// Chain of relations `a = b ≤ c` → the first expression and pairs (relation, expression).
pub fn to_relation(n: &Node) -> R<(Expr, Vec<(Rel, Expr)>)> {
    let items = n.items();
    let mut parts: Vec<&[Node]> = Vec::new();
    let mut rels = Vec::new();
    let mut start = 0;
    for (i, x) in items.iter().enumerate() {
        if let Node::Sym(s) = x {
            if s.class() == Class::Rel {
                let r = Rel::of(*s).ok_or_else(|| Unsupported::new("relation", s.latex()))?;
                parts.push(&items[start..i]);
                rels.push(r);
                start = i + 1;
            }
        }
    }
    parts.push(&items[start..]);
    if rels.is_empty() {
        return Err(Unsupported::new("syntax", "no relation"));
    }
    let first = seq(parts[0])?;
    let mut rest = Vec::new();
    for (r, p) in rels.into_iter().zip(&parts[1..]) {
        rest.push((r, seq(p)?));
    }
    Ok((first, rest))
}

fn seq(items: &[Node]) -> R<Expr> {
    if items.is_empty() {
        return Err(Unsupported::new("syntax", "empty"));
    }
    let mut p = P { items, i: 0 };
    let e = p.sum()?;
    if p.i < items.len() {
        return Err(Unsupported::new("syntax", format!("extra: {}", crate::print::to_latex(&items[p.i]))));
    }
    Ok(e)
}

struct P<'n> {
    items: &'n [Node],
    i: usize,
}

fn bx(e: Expr) -> Box<Expr> {
    Box::new(e)
}

fn int_lit(n: &Node) -> Option<i128> {
    match n {
        Node::Num(s) if !s.contains('.') => s.parse().ok(),
        _ => None,
    }
}

/// An irreducible fraction of integers `\frac{1}{2}` is a rational literal.
fn frac_lit(n: &Node) -> Option<Rat> {
    let Node::Frac(FracKind::Frac | FracKind::DFrac | FracKind::TFrac, a, b) = n else { return None };
    let (a, b) = (int_lit(a)?, int_lit(b)?);
    let r = Rat::new(a, b).ok()?;
    (b > 1 && r.num() == a && r.den() == b).then_some(r)
}

fn num(s: &str) -> R<Rat> {
    Rat::parse_decimal(s).ok_or_else(|| Unsupported::new("number", s.to_string()))
}

fn func(s: Sym) -> R<Func> {
    Ok(match s {
        Sym::Sin => Func::Sin,
        Sym::Cos => Func::Cos,
        Sym::Tan => Func::Tan,
        Sym::Exp => Func::Exp,
        Sym::Ln => Func::Ln,
        other => return Err(Unsupported::new("function", other.latex())),
    })
}

fn is_close(n: &Node, close: Sym) -> bool {
    match n {
        Node::Sym(s) => *s == close,
        Node::Scripts(s) => matches!(s.base, Node::Sym(b) if b == close),
        _ => false,
    }
}

/// Exponent of a closing bracket with a script: `)^2`.
fn close_power(n: &Node) -> R<Option<Expr>> {
    match n {
        Node::Scripts(s) => {
            if s.sub.is_some() || s.primes > 0 {
                return Err(Unsupported::new("script", "script on a bracket"));
            }
            s.sup.as_ref().map(to_expr).transpose()
        }
        _ => Ok(None),
    }
}

impl<'n> P<'n> {
    fn peek(&self) -> Option<&'n Node> {
        self.items.get(self.i)
    }
    fn peek_sym(&self) -> Option<Sym> {
        match self.peek() {
            Some(Node::Sym(s)) => Some(*s),
            _ => None,
        }
    }

    fn sum(&mut self) -> R<Expr> {
        let mut acc = self.term()?;
        let mut open = false;
        loop {
            match self.peek_sym() {
                Some(Sym::Plus) => {
                    self.i += 1;
                    let t = self.term()?;
                    acc = match acc {
                        Expr::Add(mut v) if open => {
                            v.push(t);
                            Expr::Add(v)
                        }
                        a => Expr::Add(vec![a, t]),
                    };
                    open = true;
                }
                Some(Sym::Minus) => {
                    self.i += 1;
                    let t = self.term()?;
                    acc = Expr::Sub(bx(acc), bx(t));
                    open = false;
                }
                Some(Sym::Pm | Sym::Mp) => return Err(Unsupported::new("construct", "±")),
                _ => return Ok(acc),
            }
        }
    }

    fn term(&mut self) -> R<Expr> {
        let mut acc = self.unary()?;
        let mut open = false;
        let mul = |acc: Expr, f: Expr, open: &mut bool| {
            let r = match acc {
                Expr::Mul(mut v) if *open => {
                    v.push(f);
                    Expr::Mul(v)
                }
                a => Expr::Mul(vec![a, f]),
            };
            *open = true;
            r
        };
        loop {
            match self.peek() {
                Some(Node::Sym(Sym::Cdot | Sym::Times | Sym::Ast)) => {
                    self.i += 1;
                    let f = self.unary()?;
                    acc = mul(acc, f, &mut open);
                }
                Some(Node::Sym(Sym::Slash | Sym::Div)) => {
                    self.i += 1;
                    let f = self.unary()?;
                    acc = Expr::Div(bx(acc), bx(f));
                    open = false;
                }
                Some(n) if starts_factor(n) => {
                    if matches!(n, Node::Num(_)) && matches!(self.items.get(self.i - 1), Some(Node::Num(_))) {
                        return Err(Unsupported::new("syntax", "two numbers in a row"));
                    }
                    let f = self.power()?;
                    acc = mul(acc, f, &mut open);
                }
                _ => return Ok(acc),
            }
        }
    }

    fn unary(&mut self) -> R<Expr> {
        match self.peek_sym() {
            Some(Sym::Minus) => {
                self.i += 1;
                // `-3`, `-\frac{1}{2}` — negative literal
                match self.peek() {
                    Some(Node::Num(s)) => {
                        self.i += 1;
                        let r = num(s)?.neg().map_err(|_| Unsupported::new("number", "overflow"))?;
                        Ok(Expr::Num(r))
                    }
                    Some(n) if frac_lit(n).is_some() => {
                        self.i += 1;
                        Ok(Expr::Num(frac_lit(n).unwrap().neg().map_err(|_| Unsupported::new("number", "overflow"))?))
                    }
                    _ => Ok(Expr::Neg(bx(self.unary()?))),
                }
            }
            Some(Sym::Plus) => {
                self.i += 1;
                self.unary()
            }
            _ => self.power(),
        }
    }

    /// Factor: atom, bracketed group, absolute value, function application (the script is already in the tree).
    fn power(&mut self) -> R<Expr> {
        let Some(n) = self.peek() else { return Err(Unsupported::new("syntax", "missing operand")) };
        self.i += 1;
        match n {
            Node::Sym(s) if matches!(s.class(), Class::Fn | Class::FnLim) => {
                let f = func(*s)?;
                let arg = self.func_arg()?;
                Ok(Expr::Call(f, vec![arg]))
            }
            Node::Scripts(sc) if matches!(sc.base, Node::Sym(s) if matches!(s.class(), Class::Fn | Class::FnLim)) => {
                let Node::Sym(s) = sc.base else { unreachable!() };
                let f = func(s)?;
                if sc.sub.is_some() || sc.primes > 0 {
                    return Err(Unsupported::new("script", s.latex()));
                }
                let e = to_expr(sc.sup.as_ref().unwrap())?;
                if e == Expr::Neg(bx(Expr::Num(Rat::ONE))) || e == Expr::Num(Rat::int(-1)) {
                    return Err(Unsupported::new("function", "inverse function ^{-1}"));
                }
                let arg = self.func_arg()?;
                Ok(Expr::Pow(bx(Expr::Call(f, vec![arg])), bx(e)))
            }
            Node::Sym(open @ (Sym::LParen | Sym::LBrack)) => {
                let close = if *open == Sym::LParen { Sym::RParen } else { Sym::RBrack };
                self.group(*open, close)
            }
            Node::Sym(Sym::Vert) => self.group(Sym::Vert, Sym::Vert),
            Node::Sym(Sym::Lvert) => self.group(Sym::Lvert, Sym::Rvert),
            Node::Scripts(sc) => {
                if let Node::Sym(op) = sc.base {
                    if matches!(op.class(), Class::Op | Class::OpNoLim) {
                        return Err(Unsupported::new("construct", op.latex()));
                    }
                }
                if sc.sub.is_some() {
                    return Err(Unsupported::new("script", crate::print::to_latex(n)));
                }
                if sc.primes > 0 {
                    return Err(Unsupported::new("construct", "prime"));
                }
                if matches!(sc.base, Node::Sym(Sym::LParen | Sym::LBrack | Sym::Vert)) {
                    return Err(Unsupported::new("syntax", "script on an opening bracket"));
                }
                let b = atom(&sc.base)?;
                let e = to_expr(sc.sup.as_ref().unwrap())?;
                Ok(Expr::Pow(bx(b), bx(e)))
            }
            other => atom(other),
        }
    }

    /// After an opening bracket: the content up to the matching closing one; `)^2` is a power of the group.
    fn group(&mut self, open: Sym, close: Sym) -> R<Expr> {
        let start = self.i;
        let mut depth = 0usize;
        let mut j = start;
        while j < self.items.len() {
            let n = &self.items[j];
            if open != close && matches!(n, Node::Sym(s) if *s == open) {
                depth += 1;
            } else if is_close(n, close) {
                if depth == 0 {
                    break;
                }
                depth -= 1;
            }
            j += 1;
        }
        if j >= self.items.len() {
            return Err(Unsupported::new("syntax", format!("{} without a match", open.latex())));
        }
        let inner = seq(&self.items[start..j])?;
        let inner = if matches!(open, Sym::Vert | Sym::Lvert) { Expr::Call(Func::Abs, vec![inner]) } else { inner };
        let e = match close_power(&self.items[j])? {
            Some(p) => Expr::Pow(bx(inner), bx(p)),
            None => inner,
        };
        self.i = j + 1;
        Ok(e)
    }

    /// A function argument without brackets: product of factors up to the next operator or function.
    fn func_arg(&mut self) -> R<Expr> {
        match self.peek() {
            None => Err(Unsupported::new("syntax", "function without an argument")),
            Some(Node::Sym(Sym::LParen | Sym::LBrack)) | Some(Node::LeftRight(..)) => self.power(),
            Some(_) => {
                let mut acc = self.power()?;
                let mut open = false;
                while let Some(n) = self.peek() {
                    let is_fn = matches!(n, Node::Sym(s) if matches!(s.class(), Class::Fn | Class::FnLim))
                        || matches!(n, Node::Scripts(s) if matches!(s.base, Node::Sym(b) if matches!(b.class(), Class::Fn | Class::FnLim)));
                    if is_fn || !starts_factor(n) {
                        break;
                    }
                    let f = self.power()?;
                    acc = match acc {
                        Expr::Mul(mut v) if open => {
                            v.push(f);
                            Expr::Mul(v)
                        }
                        a => Expr::Mul(vec![a, f]),
                    };
                    open = true;
                }
                Ok(acc)
            }
        }
    }
}

fn starts_factor(n: &Node) -> bool {
    match n {
        Node::Sym(s) => {
            matches!(s, Sym::LParen | Sym::LBrack | Sym::Vert | Sym::Lvert | Sym::Pi)
                || matches!(s.class(), Class::Fn | Class::FnLim | Class::Ord)
        }
        Node::Scripts(s) => !matches!(s.base, Node::Sym(b) if matches!(b.class(), Class::Bin | Class::Rel | Class::Close | Class::Punct)),
        Node::Row(_) | Node::Middle(_) => false,
        _ => true,
    }
}

fn atom(n: &Node) -> R<Expr> {
    match n {
        Node::Num(s) => Ok(Expr::Num(num(s)?)),
        Node::Char('e') => Ok(Expr::Const(Const::E)),
        Node::Char(c) => {
            let mut b = [0u8; 4];
            Var::parse(c.encode_utf8(&mut b))
                .map(Expr::Var)
                .ok_or_else(|| Unsupported::new("variable", format!("{c} (math has only x y z t u v w n)")))
        }
        Node::Sym(Sym::Pi) => Ok(Expr::Const(Const::Pi)),
        Node::Sym(s) => Err(Unsupported::new(
            match s.class() {
                Class::Ord => "symbol",
                Class::Op | Class::OpNoLim => "construct",
                _ => "syntax",
            },
            s.latex(),
        )),
        Node::Group(g) => to_expr(g),
        Node::Frac(FracKind::Frac | FracKind::DFrac | FracKind::TFrac | FracKind::CFrac, a, b) => match frac_lit(n) {
            Some(r) => Ok(Expr::Num(r)),
            None => Ok(Expr::Div(bx(to_expr(a)?), bx(to_expr(b)?))),
        },
        Node::Frac(k, ..) => Err(Unsupported::new("construct", format!("{k:?}").to_lowercase())),
        Node::Sqrt(None, b) => Ok(Expr::Call(Func::Sqrt, vec![to_expr(b)?])),
        Node::Sqrt(Some(i), b) => {
            let e = match int_lit(i) {
                Some(k) if k > 0 => Expr::Num(Rat::new(1, k).map_err(|_| Unsupported::new("number", "root index"))?),
                _ => Expr::Div(bx(Expr::Num(Rat::ONE)), bx(to_expr(i)?)),
            };
            Ok(Expr::Pow(bx(to_expr(b)?), bx(e)))
        }
        Node::LeftRight(Some(Sym::LParen | Sym::LBrack), b, Some(Sym::RParen | Sym::RBrack)) => to_expr(b),
        Node::LeftRight(Some(Sym::Vert | Sym::Lvert), b, Some(Sym::Vert | Sym::Rvert)) => Ok(Expr::Call(Func::Abs, vec![to_expr(b)?])),
        Node::Font(Font::Rm | Font::It | Font::Normal, b) if matches!(**b, Node::Char('e')) => Ok(Expr::Const(Const::E)),
        Node::Class(_, b) => to_expr(b),
        Node::Unknown(u) => Err(Unsupported::new("unknown command", format!("\\{}", u.name))),
        Node::Row(v) => seq(v),
        other => Err(Unsupported::new("construct", kind_name(other))),
    }
}

fn kind_name(n: &Node) -> String {
    match n {
        Node::Scripts(_) => "script".into(),
        Node::OpName(..) => "\\operatorname".into(),
        Node::LeftRight(..) => "\\left…\\right".into(),
        Node::Env(_) => "table".into(),
        Node::Font(f, _) => f.latex().into(),
        Node::Text(..) => "\\text".into(),
        Node::Accent(a, _) => a.latex().into(),
        Node::Stack(..) => "\\overset".into(),
        Node::XArrow(..) => "\\xrightarrow".into(),
        Node::Not(_) => "\\not".into(),
        other => crate::print::to_latex(other),
    }
}

// ---------------------------------------------------------------- Expr → LaTeX tree

/// `math::Expr` → LaTeX tree (printing steps and answers: `to_latex(&from_expr(&e)?)`).
pub fn from_expr(e: &Expr) -> R<Node> {
    let mut v = Vec::new();
    emit(e, &mut v)?;
    Ok(crate::ast::norm(v))
}

fn paren(n: Node) -> Node {
    Node::LeftRight(Some(Sym::LParen), Box::new(n), Some(Sym::RParen))
}

fn node(e: &Expr) -> R<Node> {
    from_expr(e)
}

fn is_neg_lit(e: &Expr) -> bool {
    matches!(e, Expr::Num(r) if r.is_neg())
}

fn emit_wrapped(e: &Expr, wrap: bool, v: &mut Vec<Node>) -> R<()> {
    if wrap {
        v.push(paren(node(e)?));
        Ok(())
    } else {
        emit(e, v)
    }
}

fn lit(r: Rat) -> Node {
    let a = r.abs();
    if a.is_int() {
        Node::Num(a.num().to_string().into())
    } else {
        Node::Frac(FracKind::Frac, Box::new(Node::Num(a.num().to_string().into())), Box::new(Node::Num(a.den().to_string().into())))
    }
}

fn emit(e: &Expr, v: &mut Vec<Node>) -> R<()> {
    match e {
        Expr::Num(r) => {
            if r.is_neg() {
                v.push(Node::Sym(Sym::Minus));
            }
            v.push(lit(*r));
        }
        Expr::Var(x) => v.push(Node::Char(x.name().chars().next().unwrap_or('?'))),
        Expr::Const(Const::Pi) => v.push(Node::Sym(Sym::Pi)),
        Expr::Const(Const::E) => v.push(Node::Char('e')),
        Expr::Neg(x) => {
            v.push(Node::Sym(Sym::Minus));
            let bare = matches!(**x, Expr::Var(_) | Expr::Const(_) | Expr::Pow(..) | Expr::Call(..))
                || matches!(&**x, Expr::Div(a, b) if !(matches!(**a, Expr::Num(_)) && matches!(**b, Expr::Num(_))));
            emit_wrapped(x, !bare, v)?;
        }
        Expr::Add(xs) => {
            for (i, x) in xs.iter().enumerate() {
                if i > 0 {
                    v.push(Node::Sym(Sym::Plus));
                }
                let wrap = if i == 0 {
                    matches!(x, Expr::Add(_))
                } else {
                    matches!(x, Expr::Add(_) | Expr::Sub(..) | Expr::Neg(_)) || is_neg_lit(x)
                };
                emit_wrapped(x, wrap, v)?;
            }
        }
        Expr::Sub(a, b) => {
            emit(a, v)?;
            v.push(Node::Sym(Sym::Minus));
            let wrap = matches!(**b, Expr::Add(_) | Expr::Sub(..) | Expr::Neg(_)) || is_neg_lit(b);
            emit_wrapped(b, wrap, v)?;
        }
        Expr::Mul(xs) => {
            for (i, x) in xs.iter().enumerate() {
                if i > 0 {
                    v.push(Node::Sym(Sym::Cdot));
                }
                let wrap = matches!(x, Expr::Add(_) | Expr::Sub(..) | Expr::Neg(_) | Expr::Mul(_)) || is_neg_lit(x);
                emit_wrapped(x, wrap, v)?;
            }
        }
        Expr::Div(a, b) => v.push(Node::Frac(FracKind::Frac, Box::new(node(a)?), Box::new(node(b)?))),
        Expr::Pow(a, b) => {
            // base without brackets only for an atom that will not otherwise merge with the script
            let bare = match &**a {
                Expr::Var(_) | Expr::Const(_) | Expr::Div(..) => true,
                Expr::Num(r) => !r.is_neg(),
                Expr::Call(Func::Sqrt | Func::Abs, _) => true,
                _ => false,
            };
            let mut base = Vec::new();
            emit_wrapped(a, !bare, &mut base)?;
            let base = crate::ast::norm(base);
            let base = if matches!(base, Node::Row(_)) { paren(base) } else { base };
            v.push(Node::Scripts(Box::new(Scripts { base, sub: None, sup: Some(node(b)?), primes: 0 })));
        }
        Expr::Call(f, args) => {
            let [x] = args.as_slice() else { return Err(Unsupported::new("function", format!("{} with {} arguments", f.name(), args.len()))) };
            match f {
                Func::Sqrt => v.push(Node::Sqrt(None, Box::new(node(x)?))),
                Func::Abs => v.push(Node::LeftRight(Some(Sym::Vert), Box::new(node(x)?), Some(Sym::Vert))),
                Func::Sin | Func::Cos | Func::Tan | Func::Exp | Func::Ln => {
                    let s = match f {
                        Func::Sin => Sym::Sin,
                        Func::Cos => Sym::Cos,
                        Func::Tan => Sym::Tan,
                        Func::Exp => Sym::Exp,
                        _ => Sym::Ln,
                    };
                    v.push(Node::Sym(s));
                    v.push(paren(node(x)?));
                }
                other => return Err(Unsupported::new("function", other.name())),
            }
        }
        other => return Err(Unsupported::new("construct", format!("{other}"))),
    }
    Ok(())
}

/// Normalization for comparison: `Div(p, q)` of irreducible integers ≥ 0 is the literal `p/q`.
pub fn fold_literals(e: &Expr) -> Expr {
    let f = |x: &Expr| Box::new(fold_literals(x));
    match e {
        Expr::Div(a, b) => {
            if let (Expr::Num(p), Expr::Num(q)) = (&**a, &**b) {
                if p.is_int() && q.is_int() && !p.is_neg() && q.num() > 1 {
                    if let Ok(r) = Rat::new(p.num(), q.num()) {
                        if r.num() == p.num() {
                            return Expr::Num(r);
                        }
                    }
                }
            }
            Expr::Div(f(a), f(b))
        }
        Expr::Neg(x) => Expr::Neg(f(x)),
        Expr::Add(xs) => Expr::Add(xs.iter().map(fold_literals).collect()),
        Expr::Mul(xs) => Expr::Mul(xs.iter().map(fold_literals).collect()),
        Expr::Sub(a, b) => Expr::Sub(f(a), f(b)),
        Expr::Pow(a, b) => Expr::Pow(f(a), f(b)),
        Expr::Call(g, xs) => Expr::Call(*g, xs.iter().map(fold_literals).collect()),
        other => other.clone(),
    }
}

/// Round trip of the bridge: `Expr` → LaTeX → tree → `Expr` (with literal normalization).
pub fn roundtrip(e: &Expr) -> R<bool> {
    let printed = crate::print::to_latex(&from_expr(e)?);
    let tree = crate::parse::parse(&printed).map_err(|err| Unsupported::new("syntax", err.to_string()))?;
    let back = to_expr(&tree)?;
    Ok(fold_literals(&back) == fold_literals(e))
}
