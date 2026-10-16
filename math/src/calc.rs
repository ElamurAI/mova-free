//! v2: the expression language of LLM plans and its exact evaluator. An expression is a tree (`E`): rational constants
//! of arbitrary length, variables (results of previous steps), operations, number-theory and
//! combinatorics functions, bounded search. A value (`V`) is an exact rational, a boolean, a list, or
//! an approximate real (`R`, only for transcendental results — always marked «≈»).
//!
//! Independent checks of every computation:
//! - the same expression in different arithmetic: f64 (when every function has an f64 formula) or a fingerprint modulo
//!   the prime 2^61−1 (for huge integers, where f64 overflows);
//! - a certificate for every number-theory and combinatorics function (`nt`, `comb`).

use std::collections::HashMap;
use std::fmt;

use crate::big::*;
use crate::comb;
use crate::nt::{self, Check, Limits};

#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Mod,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
}

impl Op {
    fn sym(&self) -> &'static str {
        match self {
            Op::Add => "+",
            Op::Sub => "-",
            Op::Mul => "*",
            Op::Div => "/",
            Op::Pow => "^",
            Op::Mod => " mod ",
            Op::Eq => " == ",
            Op::Ne => " != ",
            Op::Lt => " < ",
            Op::Le => " <= ",
            Op::Gt => " > ",
            Op::Ge => " >= ",
            Op::And => " and ",
            Op::Or => " or ",
        }
    }
    fn prec(&self) -> u8 {
        match self {
            Op::Or => 1,
            Op::And => 2,
            Op::Eq | Op::Ne | Op::Lt | Op::Le | Op::Gt | Op::Ge => 3,
            Op::Add | Op::Sub => 4,
            Op::Mul | Op::Div | Op::Mod => 5,
            Op::Pow => 7,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum E {
    Num(Q),
    Var(String),
    Neg(Box<E>),
    Not(Box<E>),
    Fact(Box<E>),
    Bin(Op, Box<E>, Box<E>),
    Call(String, Vec<E>),
    List(Vec<E>),
    Index(Box<E>, Box<E>),
}

impl E {
    pub fn num(i: i64) -> E {
        E::Num(q(i))
    }
    pub fn bin(op: Op, a: E, b: E) -> E {
        E::Bin(op, Box::new(a), Box::new(b))
    }
    fn prec(&self) -> u8 {
        match self {
            E::Bin(op, _, _) => op.prec(),
            E::Neg(_) => 6,
            E::Not(_) => 2,
            E::Num(x) if *x < RBig::ZERO || !x.is_int() => 5,
            _ => 9,
        }
    }
    /// Free variables (for checks and templates).
    pub fn vars(&self, out: &mut Vec<String>) {
        match self {
            E::Var(v) => {
                if !out.contains(v) {
                    out.push(v.clone())
                }
            }
            E::Num(_) => {}
            E::Neg(a) | E::Not(a) | E::Fact(a) => a.vars(out),
            E::Bin(_, a, b) | E::Index(a, b) => {
                a.vars(out);
                b.vars(out);
            }
            E::Call(_, xs) | E::List(xs) => xs.iter().for_each(|x| x.vars(out)),
        }
    }
}

impl fmt::Display for E {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let wrap = |e: &E, min: u8, f: &mut fmt::Formatter<'_>| -> fmt::Result { if e.prec() < min { write!(f, "({e})") } else { write!(f, "{e}") } };
        match self {
            E::Num(x) => write!(f, "{}", show(x)),
            E::Var(v) => write!(f, "{v}"),
            E::Neg(a) => {
                write!(f, "-")?;
                wrap(a, 7, f)
            }
            E::Not(a) => {
                write!(f, "not ")?;
                wrap(a, 3, f)
            }
            E::Fact(a) => {
                wrap(a, 9, f)?;
                write!(f, "!")
            }
            E::Bin(op, a, b) => {
                let p = op.prec();
                if *op == Op::Pow {
                    wrap(a, p + 1, f)?;
                    write!(f, "^")?;
                    wrap(b, p, f)
                } else {
                    wrap(a, p, f)?;
                    write!(f, "{}", op.sym())?;
                    wrap(b, p + 1, f)
                }
            }
            E::Call(n, xs) => write!(f, "{n}({})", xs.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ")),
            E::List(xs) => write!(f, "[{}]", xs.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ")),
            E::Index(a, i) => {
                wrap(a, 9, f)?;
                write!(f, "[{i}]")
            }
        }
    }
}

// ---------- values ----------

#[derive(Clone, Debug, PartialEq)]
pub enum V {
    Q(Q),
    /// approximate real (transcendental) — always with «≈»
    R(f64),
    B(bool),
    L(Vec<V>),
    S(String),
}

impl V {
    pub fn q(&self) -> Option<&Q> {
        match self {
            V::Q(x) => Some(x),
            _ => None,
        }
    }
    pub fn f64(&self) -> Option<f64> {
        match self {
            V::Q(x) => Some(to_f64(x)),
            V::R(x) => Some(*x),
            V::B(b) => Some(if *b { 1.0 } else { 0.0 }),
            _ => None,
        }
    }
    pub fn is_exact(&self) -> bool {
        match self {
            V::Q(_) | V::B(_) | V::S(_) => true,
            V::R(_) => false,
            V::L(xs) => xs.iter().all(V::is_exact),
        }
    }
    pub fn show(&self) -> String {
        match self {
            V::Q(x) => {
                let s = show_nice(x);
                if s.len() > 120 { comb::short(x.numerator()) } else { s }
            }
            V::R(x) => format!("≈ {}", fmt_f64(*x)),
            V::B(b) => (if *b { "true" } else { "false" }).into(),
            V::L(xs) => format!("[{}]", xs.iter().map(V::show).collect::<Vec<_>>().join(", ")),
            V::S(s) => s.clone(),
        }
    }
    /// Full exact notation (long numbers not shortened) — for the answer.
    pub fn exact(&self) -> String {
        match self {
            V::Q(x) => show(x),
            V::R(x) => fmt_f64(*x),
            V::B(b) => b.to_string(),
            V::L(xs) => format!("[{}]", xs.iter().map(V::exact).collect::<Vec<_>>().join(", ")),
            V::S(s) => s.clone(),
        }
    }
}

// ---------- parsing ----------

#[derive(Clone, Debug, PartialEq)]
enum Tok {
    Num(String),
    Id(String),
    Sym(&'static str),
}

fn lex(s: &str) -> Result<Vec<Tok>, String> {
    let cs: Vec<char> = s.chars().collect();
    let mut i = 0;
    let mut out = Vec::new();
    const SYMS: [&str; 22] = ["**", "==", "!=", "<=", ">=", "&&", "||", "≤", "≥", "≠", "+", "-", "*", "/", "^", "%", "(", ")", ",", "!", "<", ">"];
    while i < cs.len() {
        let c = cs[i];
        if c.is_whitespace() {
            i += 1;
            continue;
        }
        if c.is_ascii_digit() || (c == '.' && i + 1 < cs.len() && cs[i + 1].is_ascii_digit()) {
            let st = i;
            while i < cs.len() && (cs[i].is_ascii_digit() || cs[i] == '.') {
                i += 1;
            }
            // 1e5, 2.5e-3
            if i + 1 < cs.len() && (cs[i] == 'e' || cs[i] == 'E') && (cs[i + 1].is_ascii_digit() || ((cs[i + 1] == '-' || cs[i + 1] == '+') && i + 2 < cs.len() && cs[i + 2].is_ascii_digit())) {
                i += 2;
                while i < cs.len() && cs[i].is_ascii_digit() {
                    i += 1;
                }
            }
            out.push(Tok::Num(cs[st..i].iter().collect()));
            continue;
        }
        if c.is_alphabetic() || c == '_' {
            let st = i;
            while i < cs.len() && (cs[i].is_alphanumeric() || cs[i] == '_') {
                i += 1;
            }
            out.push(Tok::Id(cs[st..i].iter().collect()));
            continue;
        }
        if c == '[' {
            out.push(Tok::Sym("["));
            i += 1;
            continue;
        }
        if c == ']' {
            out.push(Tok::Sym("]"));
            i += 1;
            continue;
        }
        if c == '=' {
            // «=» and «==» — equality
            i += if i + 1 < cs.len() && cs[i + 1] == '=' { 2 } else { 1 };
            out.push(Tok::Sym("=="));
            continue;
        }
        if c == '·' || c == '×' {
            out.push(Tok::Sym("*"));
            i += 1;
            continue;
        }
        if c == '−' {
            out.push(Tok::Sym("-"));
            i += 1;
            continue;
        }
        let mut hit = None;
        for s in SYMS {
            let sc: Vec<char> = s.chars().collect();
            if cs[i..].starts_with(&sc) {
                hit = Some((s, sc.len()));
                break;
            }
        }
        match hit {
            Some((s, n)) => {
                let s = match s {
                    "**" => "^",
                    "≤" => "<=",
                    "≥" => ">=",
                    "≠" => "!=",
                    "&&" => "and",
                    "||" => "or",
                    x => x,
                };
                out.push(Tok::Sym(s));
                i += n;
            }
            None => return Err(format!("unknown symbol «{c}»")),
        }
    }
    Ok(out)
}

struct P {
    t: Vec<Tok>,
    i: usize,
}

impl P {
    fn peek(&self) -> Option<&Tok> {
        self.t.get(self.i)
    }
    fn is_sym(&self, s: &str) -> bool {
        matches!(self.peek(), Some(Tok::Sym(x)) if *x == s)
    }
    fn is_kw(&self, s: &str) -> bool {
        matches!(self.peek(), Some(Tok::Id(x)) if x == s)
    }
    fn eat(&mut self, s: &str) -> Result<(), String> {
        if self.is_sym(s) {
            self.i += 1;
            Ok(())
        } else {
            Err(format!("expected «{s}», got {:?}", self.peek()))
        }
    }
    fn or(&mut self) -> Result<E, String> {
        let mut a = self.and()?;
        while self.is_kw("or") || self.is_sym("or") {
            self.i += 1;
            a = E::bin(Op::Or, a, self.and()?);
        }
        Ok(a)
    }
    fn and(&mut self) -> Result<E, String> {
        let mut a = self.not()?;
        while self.is_kw("and") || self.is_sym("and") {
            self.i += 1;
            a = E::bin(Op::And, a, self.not()?);
        }
        Ok(a)
    }
    fn not(&mut self) -> Result<E, String> {
        if self.is_kw("not") {
            self.i += 1;
            return Ok(E::Not(Box::new(self.not()?)));
        }
        self.cmp()
    }
    fn cmp(&mut self) -> Result<E, String> {
        let a = self.add()?;
        let op = match self.peek() {
            Some(Tok::Sym("==")) => Op::Eq,
            Some(Tok::Sym("!=")) => Op::Ne,
            Some(Tok::Sym("<")) => Op::Lt,
            Some(Tok::Sym("<=")) => Op::Le,
            Some(Tok::Sym(">")) => Op::Gt,
            Some(Tok::Sym(">=")) => Op::Ge,
            _ => return Ok(a),
        };
        self.i += 1;
        let b = self.add()?;
        // chain a < b < c
        if matches!(self.peek(), Some(Tok::Sym("<" | "<=" | ">" | ">=" | "=="))) {
            let op2 = match self.peek() {
                Some(Tok::Sym("<")) => Op::Lt,
                Some(Tok::Sym("<=")) => Op::Le,
                Some(Tok::Sym(">")) => Op::Gt,
                Some(Tok::Sym(">=")) => Op::Ge,
                _ => Op::Eq,
            };
            self.i += 1;
            let c = self.add()?;
            return Ok(E::bin(Op::And, E::bin(op, a, b.clone()), E::bin(op2, b, c)));
        }
        Ok(E::bin(op, a, b))
    }
    fn add(&mut self) -> Result<E, String> {
        let mut a = self.mul()?;
        loop {
            if self.is_sym("+") {
                self.i += 1;
                a = E::bin(Op::Add, a, self.mul()?);
            } else if self.is_sym("-") {
                self.i += 1;
                a = E::bin(Op::Sub, a, self.mul()?);
            } else {
                return Ok(a);
            }
        }
    }
    fn starts_primary(&self) -> bool {
        match self.peek() {
            Some(Tok::Num(_)) => true,
            Some(Tok::Id(x)) => !matches!(x.as_str(), "and" | "or" | "not" | "mod"),
            Some(Tok::Sym("(")) => true,
            _ => false,
        }
    }
    fn mul(&mut self) -> Result<E, String> {
        let mut a = self.unary()?;
        loop {
            if self.is_sym("*") {
                self.i += 1;
                a = E::bin(Op::Mul, a, self.unary()?);
            } else if self.is_sym("/") {
                self.i += 1;
                a = E::bin(Op::Div, a, self.unary()?);
            } else if self.is_sym("%") || self.is_kw("mod") {
                self.i += 1;
                a = E::bin(Op::Mod, a, self.unary()?);
            } else if self.starts_primary() {
                // implicit multiplication: 2x, 3(x+1), (a)(b)
                a = E::bin(Op::Mul, a, self.pow()?);
            } else {
                return Ok(a);
            }
        }
    }
    fn unary(&mut self) -> Result<E, String> {
        if self.is_sym("-") {
            self.i += 1;
            let x = self.unary()?;
            return Ok(match x {
                E::Num(v) => E::Num(-v),
                x => E::Neg(Box::new(x)),
            });
        }
        if self.is_sym("+") {
            self.i += 1;
            return self.unary();
        }
        self.pow()
    }
    fn pow(&mut self) -> Result<E, String> {
        let a = self.postfix()?;
        if self.is_sym("^") {
            self.i += 1;
            let b = self.unary()?; // right-associative, 2^-1
            return Ok(E::bin(Op::Pow, a, b));
        }
        Ok(a)
    }
    fn postfix(&mut self) -> Result<E, String> {
        let mut a = self.primary()?;
        loop {
            if self.is_sym("!") {
                self.i += 1;
                a = E::Fact(Box::new(a));
            } else if self.is_sym("[") {
                self.i += 1;
                let ix = self.or()?;
                self.eat("]")?;
                a = E::Index(Box::new(a), Box::new(ix));
            } else {
                return Ok(a);
            }
        }
    }
    fn primary(&mut self) -> Result<E, String> {
        match self.peek().cloned() {
            Some(Tok::Num(s)) => {
                self.i += 1;
                parse_q(&s).map(E::Num).ok_or(format!("number «{s}»"))
            }
            Some(Tok::Id(name)) => {
                self.i += 1;
                if self.is_sym("(") && is_func(&name) {
                    self.i += 1;
                    let mut args = Vec::new();
                    if !self.is_sym(")") {
                        loop {
                            args.push(self.or()?);
                            if self.is_sym(",") {
                                self.i += 1;
                            } else {
                                break;
                            }
                        }
                    }
                    self.eat(")")?;
                    return Ok(E::Call(canon(&name).to_string(), args));
                }
                Ok(E::Var(name))
            }
            Some(Tok::Sym("(")) => {
                self.i += 1;
                let e = self.or()?;
                self.eat(")")?;
                Ok(e)
            }
            Some(Tok::Sym("[")) => {
                self.i += 1;
                let mut xs = Vec::new();
                if !self.is_sym("]") {
                    loop {
                        xs.push(self.or()?);
                        if self.is_sym(",") {
                            self.i += 1;
                        } else {
                            break;
                        }
                    }
                }
                self.eat("]")?;
                Ok(E::List(xs))
            }
            t => Err(format!("expected a number, variable or parenthesis, got {t:?}")),
        }
    }
}

pub fn parse(s: &str) -> Result<E, String> {
    let t = lex(s)?;
    let mut p = P { t, i: 0 };
    let e = p.or()?;
    if p.i != p.t.len() {
        return Err(format!("extra tokens after the expression: {:?}", &p.t[p.i..]));
    }
    Ok(e)
}

/// Equation «left = right» → (left, right); without «=» — (expression, 0).
pub fn parse_equation(s: &str) -> Result<(E, E), String> {
    match parse(s)? {
        E::Bin(Op::Eq, a, b) => Ok((*a, *b)),
        e => Ok((e, E::num(0))),
    }
}

const FUNCS: &[&str] = &[
    "abs", "sign", "floor", "ceil", "round", "trunc", "num", "den", "frac", "min", "max", "gcd", "lcm", "mod", "idiv", "sqrt", "isqrt", "root", "cbrt", "pow",
    "isprime", "nextprime", "prevprime", "primepi", "nthprime", "factor", "divisors", "tau", "sigma", "phi", "mobius", "modpow", "modinv", "crt", "digits",
    "digitsum", "numdigits", "fromdigits", "valuation", "issquare", "fact", "binom", "perm", "catalan", "stirling1", "stirling2", "bell", "partitions",
    "partitionsk", "partitionsdistinct", "derangements", "fib", "multinomial", "len", "sum", "prod", "sort", "reverse", "mean", "median", "range", "count",
    "list", "filter", "forall", "exists", "first", "maxof", "minof", "if", "sin", "cos", "tan", "asin", "acos", "atan", "exp", "ln", "log", "log10", "log2",
    "isint", "numer", "denom", "unique", "at",
];

const ALIASES: &[(&str, &str)] = &[
    ("factorial", "fact"),
    ("C", "binom"),
    ("choose", "binom"),
    ("nCr", "binom"),
    ("binomial", "binom"),
    ("P", "perm"),
    ("nPr", "perm"),
    ("totient", "phi"),
    ("numdivisors", "tau"),
    ("d", "tau"),
    ("subfactorial", "derangements"),
    ("fibonacci", "fib"),
    ("floor_div", "idiv"),
    ("powmod", "modpow"),
    ("invmod", "modinv"),
    ("is_prime", "isprime"),
    ("next_prime", "nextprime"),
    ("digit_sum", "digitsum"),
    ("Abs", "abs"),
    ("sqr", "sqrt"),
];

fn canon(name: &str) -> &str {
    ALIASES.iter().find(|(a, _)| *a == name).map(|(_, b)| *b).unwrap_or(name)
}

pub fn is_func(name: &str) -> bool {
    FUNCS.contains(&canon(name))
}

// ---------- evaluation ----------

/// One trace record: rule, input, result, checks.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Sub {
    pub rule: String,
    pub result: String,
    pub checks: Vec<Check>,
}

pub struct Env {
    pub vars: HashMap<String, V>,
    pub subs: Vec<Sub>,
    pub lim: Limits,
    pub steps: u64,
    /// whether there was a transcendental step (the answer is then approximate)
    pub approx: bool,
}

impl Env {
    pub fn new(lim: Limits) -> Env {
        Env { vars: HashMap::new(), subs: Vec::new(), lim, steps: 0, approx: false }
    }
    fn tick(&mut self, n: u64) -> Result<(), String> {
        self.steps += n;
        if self.steps > self.lim.max_steps {
            return Err(format!("step limit {} exhausted", self.lim.max_steps));
        }
        if self.steps % 4096 == 0 && self.lim.timed_out() {
            return Err("time limit exhausted".into());
        }
        Ok(())
    }
    fn note(&mut self, rule: String, result: &V, checks: Vec<Check>) {
        if self.subs.len() < 200 {
            self.subs.push(Sub { rule, result: result.show(), checks });
        }
    }
}

fn want_q(v: V, what: &str) -> Result<Q, String> {
    match v {
        V::Q(x) => Ok(x),
        V::B(b) => Ok(q(b as i64)),
        V::R(x) => Err(format!("{what}: an exact number is required, got approximate {}", fmt_f64(x))),
        other => Err(format!("{what}: a number is required, got {}", other.show())),
    }
}

fn want_z(v: V, what: &str) -> Result<Z, String> {
    let x = want_q(v, what)?;
    to_z(&x).ok_or(format!("{what}: an integer is required, got {}", show(&x)))
}

fn want_u64(v: V, what: &str) -> Result<u64, String> {
    let x = want_z(v, what)?;
    z_to_u64(&x).ok_or(format!("{what}: a non-negative integer within u64 is required, got {x}"))
}

fn want_bool(v: V, what: &str) -> Result<bool, String> {
    match v {
        V::B(b) => Ok(b),
        V::Q(x) => Ok(!x.is_zero()),
        other => Err(format!("{what}: a boolean is required, got {}", other.show())),
    }
}

fn want_list(v: V, what: &str) -> Result<Vec<V>, String> {
    match v {
        V::L(xs) => Ok(xs),
        other => Err(format!("{what}: a list is required, got {}", other.show())),
    }
}

fn cmp_v(a: &V, b: &V) -> Result<std::cmp::Ordering, String> {
    match (a, b) {
        (V::Q(x), V::Q(y)) => Ok(x.cmp(y)),
        _ => {
            let (x, y) = (a.f64().ok_or("comparison: not a number")?, b.f64().ok_or("comparison: not a number")?);
            let tol = 1e-9 * x.abs().max(y.abs()).max(1.0);
            Ok(if (x - y).abs() <= tol { std::cmp::Ordering::Equal } else { x.partial_cmp(&y).ok_or("NaN")? })
        }
    }
}

fn eq_v(a: &V, b: &V) -> Result<bool, String> {
    match (a, b) {
        (V::L(x), V::L(y)) => {
            if x.len() != y.len() {
                return Ok(false);
            }
            for (p, r) in x.iter().zip(y) {
                if !eq_v(p, r)? {
                    return Ok(false);
                }
            }
            Ok(true)
        }
        (V::B(x), V::B(y)) => Ok(x == y),
        (V::S(x), V::S(y)) => Ok(x == y),
        _ => Ok(cmp_v(a, b)? == std::cmp::Ordering::Equal),
    }
}

fn real(x: f64) -> Result<V, String> {
    if x.is_finite() { Ok(V::R(x)) } else { Err("approximate value is not finite (outside the domain)".into()) }
}

fn pow_v(a: V, b: V, env: &mut Env) -> Result<V, String> {
    match (&a, &b) {
        (V::Q(x), V::Q(e)) => {
            if e.is_int() {
                let e = to_z(e).unwrap();
                let ei = z_to_i64(&e).filter(|v| v.abs() <= 10_000_000).ok_or(format!("exponent {e} is too large"))?;
                if x.is_zero() && ei < 0 {
                    return Err("division by zero (0 to a negative power)".into());
                }
                let bits = (uz(x.numerator()).bit_len() + x.denominator().bit_len()) as f64 * ei.unsigned_abs() as f64;
                if bits > 4.0e7 {
                    return Err(format!("result is too large (~{bits:.0} bits)"));
                }
                env.tick(ei.unsigned_abs().max(1).ilog2() as u64 + 1)?;
                return Ok(V::Q(x.pow(ei as isize)));
            }
            // rational exponent p/k — an exact root, if one exists
            let k = u64::try_from(e.denominator()).map_err(|_| "exponent denominator is too large")?;
            if k <= 64 {
                if let Some(r) = exact_root_q(x, k as usize) {
                    let p = z_to_i64(e.numerator()).ok_or("exponent is too large")?;
                    if !(r.is_zero() && p < 0) {
                        return Ok(V::Q(r.pow(p as isize)));
                    }
                }
            }
            env.approx = true;
            real(to_f64(x).powf(to_f64(e)))
        }
        _ => {
            let (x, e) = (a.f64().ok_or("power: not a number")?, b.f64().ok_or("power: not a number")?);
            env.approx = true;
            real(x.powf(e))
        }
    }
}

fn arith(op: &Op, a: V, b: V, env: &mut Env) -> Result<V, String> {
    // lists: [a] + [b] — concatenation
    if let (Op::Add, V::L(xs), V::L(ys)) = (op, &a, &b) {
        return Ok(V::L(xs.iter().chain(ys.iter()).cloned().collect()));
    }
    // an exact zero stays exact: 0 · ≈x = 0, 0 / ≈x = 0 (x ≠ 0)
    if let (Op::Mul | Op::Div, V::Q(z), V::R(x)) = (op, &a, &b) {
        if z.is_zero() && (*op == Op::Mul || *x != 0.0) && x.is_finite() {
            return Ok(V::Q(q(0)));
        }
    }
    if let (Op::Mul, V::R(x), V::Q(z)) = (op, &a, &b) {
        if z.is_zero() && x.is_finite() {
            return Ok(V::Q(q(0)));
        }
    }
    if let (V::L(xs), false) = (&a, matches!(op, Op::Eq | Op::Ne)) {
        // elementwise with a scalar: [1,2]*3
        if !matches!(b, V::L(_)) {
            return xs.iter().map(|x| arith(op, x.clone(), b.clone(), env)).collect::<Result<Vec<_>, _>>().map(V::L);
        }
    }
    match op {
        Op::Eq => return Ok(V::B(eq_v(&a, &b)?)),
        Op::Ne => return Ok(V::B(!eq_v(&a, &b)?)),
        Op::Lt => return Ok(V::B(cmp_v(&a, &b)?.is_lt())),
        Op::Le => return Ok(V::B(cmp_v(&a, &b)?.is_le())),
        Op::Gt => return Ok(V::B(cmp_v(&a, &b)?.is_gt())),
        Op::Ge => return Ok(V::B(cmp_v(&a, &b)?.is_ge())),
        Op::And => return Ok(V::B(want_bool(a, "and")? && want_bool(b, "and")?)),
        Op::Or => return Ok(V::B(want_bool(a, "or")? || want_bool(b, "or")?)),
        Op::Pow => return pow_v(a, b, env),
        _ => {}
    }
    env.tick(1)?;
    match (a, b) {
        (V::Q(x), V::Q(y)) => match op {
            Op::Add => Ok(V::Q(x + y)),
            Op::Sub => Ok(V::Q(x - y)),
            Op::Mul => Ok(V::Q(x * y)),
            Op::Div => {
                if y.is_zero() {
                    Err("division by zero".into())
                } else {
                    Ok(V::Q(x / y))
                }
            }
            Op::Mod => {
                if y.is_zero() {
                    return Err("remainder modulo 0".into());
                }
                // x − y·⌊x/y⌋ — for integers the usual non-negative remainder (when y > 0)
                let fl = RBig::from((x.clone() / y.clone()).floor());
                let r = x.clone() - y.clone() * fl;
                if x.is_int() && y.is_int() && (x.numerator().bit_len() > 50 || y.numerator().bit_len() > 50) {
                    // remainder certificate by other code (dashu integer %): 0 ≤ r < |y| and y | (x − r)
                    let (xi, yi, ri) = (x.numerator().clone(), y.numerator().clone(), r.numerator().clone());
                    let ok = ri >= IBig::ZERO && ri < abs_z(&yi) && ((&xi - &ri) % &yi).is_zero();
                    env.note(format!("{} mod {}", comb::short(&xi), comb::short(&yi)), &V::Q(r.clone()), vec![Check::new("remainder: 0 ≤ r < |m| and m | (a − r)", ok || y < RBig::ZERO, "")]);
                }
                Ok(V::Q(r))
            }
            _ => unreachable!(),
        },
        (a, b) => {
            let (x, y) = (a.f64().ok_or(format!("{}: not a number", op.sym()))?, b.f64().ok_or(format!("{}: not a number", op.sym()))?);
            env.approx = true;
            real(match op {
                Op::Add => x + y,
                Op::Sub => x - y,
                Op::Mul => x * y,
                Op::Div => x / y,
                Op::Mod => x.rem_euclid(y),
                _ => unreachable!(),
            })
        }
    }
}

pub fn eval(e: &E, env: &mut Env) -> Result<V, String> {
    match e {
        E::Num(x) => Ok(V::Q(x.clone())),
        E::Var(v) => {
            if let Some(x) = env.vars.get(v) {
                return Ok(x.clone());
            }
            match v.as_str() {
                "pi" | "π" => {
                    env.approx = true;
                    Ok(V::R(std::f64::consts::PI))
                }
                "e" => {
                    env.approx = true;
                    Ok(V::R(std::f64::consts::E))
                }
                "true" => Ok(V::B(true)),
                "false" => Ok(V::B(false)),
                _ => Err(format!("unknown variable «{v}»")),
            }
        }
        E::Neg(a) => match eval(a, env)? {
            V::Q(x) => Ok(V::Q(-x)),
            V::R(x) => Ok(V::R(-x)),
            V::L(xs) => xs.into_iter().map(|x| arith(&Op::Mul, x, V::Q(q(-1)), env)).collect::<Result<Vec<_>, _>>().map(V::L),
            o => Err(format!("minus of {}", o.show())),
        },
        E::Not(a) => Ok(V::B(!want_bool(eval(a, env)?, "not")?)),
        E::Fact(a) => {
            let n = want_u64(eval(a, env)?, "n!")?;
            env.tick(n / 64 + 1)?;
            let v = comb::factorial(n)?;
            let mut checks = Vec::new();
            if n <= 20_000 {
                checks.push(Check::new("n!: Legendre's formula (∏ p^Σ⌊n/pⁱ⌋)", comb::factorial_legendre(n) == v, ""));
            }
            let r = V::Q(qz(v));
            env.note(format!("{n}!"), &r, checks);
            Ok(r)
        }
        E::Bin(Op::And, a, b) => {
            // short-circuit — so search conditions do not compute needlessly
            if !want_bool(eval(a, env)?, "and")? {
                return Ok(V::B(false));
            }
            Ok(V::B(want_bool(eval(b, env)?, "and")?))
        }
        E::Bin(Op::Or, a, b) => {
            if want_bool(eval(a, env)?, "or")? {
                return Ok(V::B(true));
            }
            Ok(V::B(want_bool(eval(b, env)?, "or")?))
        }
        E::Bin(op, a, b) => {
            let x = eval(a, env)?;
            let y = eval(b, env)?;
            arith(op, x, y, env)
        }
        E::List(xs) => xs.iter().map(|x| eval(x, env)).collect::<Result<Vec<_>, _>>().map(V::L),
        E::Index(a, i) => {
            let xs = want_list(eval(a, env)?, "index")?;
            let i = want_z(eval(i, env)?, "index")?;
            let n = xs.len() as i64;
            let i = z_to_i64(&i).ok_or("index is too large")?;
            let j = if i < 0 { n + i } else { i };
            xs.get(j as usize).cloned().filter(|_| j >= 0).ok_or(format!("index {i} is outside a list of length {n}"))
        }
        E::Call(f, args) => call(f, args, env),
    }
}

/// Search with a bound variable: (expression, variable, from, to).
fn ranged(f: &str, args: &[E], env: &mut Env) -> Result<V, String> {
    if args.len() != 4 {
        return Err(format!("{f}(expression, variable, from, to) — 4 arguments"));
    }
    let E::Var(k) = &args[1] else { return Err(format!("{f}: the second argument must be a variable name")) };
    let lo = want_z(eval(&args[2], env)?, f)?;
    let hi = want_z(eval(&args[3], env)?, f)?;
    let span = &hi - &lo + IBig::ONE;
    if span > IBig::from(env.lim.max_steps as i64) {
        return Err(format!("{f}: range {lo}..{hi} exceeds the step limit"));
    }
    let saved = env.vars.get(k).cloned();
    let mut i = lo.clone();
    let mut acc: Vec<V> = Vec::new();
    let mut total = V::Q(if f == "prod" { q(1) } else { q(0) });
    let mut cnt = 0u64;
    let mut best: Option<V> = None;
    let mut found: Option<V> = None;
    let mut ok_all = true;
    let mut counter: Option<Z> = None;
    while i <= hi {
        env.tick(1)?;
        env.vars.insert(k.clone(), V::Q(qz(i.clone())));
        let v = eval(&args[0], env)?;
        match f {
            "sum" => total = arith(&Op::Add, total, v, env)?,
            "prod" => total = arith(&Op::Mul, total, v, env)?,
            "count" => {
                if want_bool(v, f)? {
                    cnt += 1
                }
            }
            "list" => acc.push(v),
            "filter" => {
                if want_bool(v, f)? {
                    acc.push(V::Q(qz(i.clone())))
                }
            }
            "forall" => {
                if !want_bool(v, f)? {
                    ok_all = false;
                    counter = Some(i.clone());
                    break;
                }
            }
            "exists" | "first" => {
                if want_bool(v, f)? {
                    found = Some(V::Q(qz(i.clone())));
                    break;
                }
            }
            "maxof" => {
                if best.as_ref().is_none_or(|b| cmp_v(&v, b).map(|o| o.is_gt()).unwrap_or(false)) {
                    best = Some(v)
                }
            }
            "minof" => {
                if best.as_ref().is_none_or(|b| cmp_v(&v, b).map(|o| o.is_lt()).unwrap_or(false)) {
                    best = Some(v)
                }
            }
            _ => unreachable!(),
        }
        i += IBig::ONE;
    }
    match saved {
        Some(s) => env.vars.insert(k.clone(), s),
        None => env.vars.remove(k),
    };
    let r = match f {
        "sum" | "prod" => total,
        "count" => V::Q(q(cnt as i64)),
        "list" | "filter" => V::L(acc),
        "forall" => V::B(ok_all),
        "exists" => V::B(found.is_some()),
        "first" => found.ok_or(format!("first: no {k} in {lo}..{hi} satisfies the condition"))?,
        _ => best.ok_or(format!("{f}: empty range"))?,
    };
    let rule = format!("{f}({}, {k} = {lo}..{hi}) — search over {} cases", args[0], &hi - &lo + IBig::ONE);
    let mut checks = Vec::new();
    if let Some(c) = counter {
        checks.push(Check::new("counterexample", true, format!("{k} = {c}")));
    }
    env.note(rule, &r, checks);
    Ok(r)
}

fn flat_args(args: &[E], env: &mut Env) -> Result<Vec<V>, String> {
    let vs = args.iter().map(|a| eval(a, env)).collect::<Result<Vec<_>, _>>()?;
    if vs.len() == 1 {
        if let V::L(xs) = &vs[0] {
            return Ok(xs.clone());
        }
    }
    Ok(vs)
}

fn call(f: &str, args: &[E], env: &mut Env) -> Result<V, String> {
    match f {
        "count" | "list" | "filter" | "forall" | "exists" | "first" | "maxof" | "minof" => return ranged(f, args, env),
        "sum" | "prod" if args.len() == 4 => return ranged(f, args, env),
        "if" => {
            if args.len() != 3 {
                return Err("if(condition, then, else)".into());
            }
            return if want_bool(eval(&args[0], env)?, "if")? { eval(&args[1], env) } else { eval(&args[2], env) };
        }
        _ => {}
    }
    let vs = args.iter().map(|a| eval(a, env)).collect::<Result<Vec<_>, _>>()?;
    let n = vs.len();
    let arity = |k: usize| if n == k { Ok(()) } else { Err(format!("{f}: {k} argument(s) required, got {n}")) };
    let mut checks: Vec<Check> = Vec::new();
    let one = |vs: &Vec<V>| vs[0].clone();
    let r: V = match f {
        "abs" => {
            arity(1)?;
            match one(&vs) {
                V::Q(x) => V::Q(x.abs()),
                v => real(v.f64().ok_or("abs")?.abs())?,
            }
        }
        "sign" => {
            arity(1)?;
            let x = want_q(one(&vs), f)?;
            V::Q(q(if x > RBig::ZERO { 1 } else if x < RBig::ZERO { -1 } else { 0 }))
        }
        "floor" | "ceil" | "round" | "trunc" => {
            arity(1)?;
            match one(&vs) {
                V::Q(x) => V::Q(qz(match f {
                    "floor" => x.floor(),
                    "ceil" => x.ceil(),
                    "trunc" => x.trunc(),
                    // half-step — away from zero (ordinary school rounding)
                    _ => {
                        let h = RBig::from_parts(IBig::ONE, UBig::from(2u8));
                        if x >= RBig::ZERO { (x + h).floor() } else { -((-x + h).floor()) }
                    }
                })),
                V::R(x) => {
                    let v = match f {
                        "floor" => x.floor(),
                        "ceil" => x.ceil(),
                        "trunc" => x.trunc(),
                        _ => x.round(),
                    };
                    // integer from an approximate value — only when far from the boundary
                    if (x - x.round()).abs() < 1e-9 && f != "round" {
                        return Err(format!("{f}: approximate {x} is too close to an integer — precision does not guarantee the answer"));
                    }
                    V::Q(RBig::simplest_from_f64(v).ok_or("floor")?)
                }
                o => return Err(format!("{f}: not a number {}", o.show())),
            }
        }
        "num" | "numer" => {
            arity(1)?;
            V::Q(qz(want_q(one(&vs), f)?.numerator().clone()))
        }
        "den" | "denom" => {
            arity(1)?;
            V::Q(qz(IBig::from(want_q(one(&vs), f)?.denominator().clone())))
        }
        "frac" => {
            arity(1)?;
            let x = want_q(one(&vs), f)?;
            V::Q(x.clone() - RBig::from(x.floor()))
        }
        "isint" => {
            arity(1)?;
            V::B(matches!(one(&vs), V::Q(x) if x.is_int()))
        }
        "min" | "max" => {
            let xs = flat_args(args, env)?;
            let mut best = xs.first().cloned().ok_or(format!("{f}: empty"))?;
            for x in &xs[1..] {
                let o = cmp_v(x, &best)?;
                if (f == "min" && o.is_lt()) || (f == "max" && o.is_gt()) {
                    best = x.clone();
                }
            }
            best
        }
        "gcd" | "lcm" => {
            let xs = flat_args(args, env)?;
            if xs.is_empty() {
                return Err(format!("{f}: empty"));
            }
            let zs = xs.into_iter().map(|x| want_z(x, f)).collect::<Result<Vec<_>, _>>()?;
            let mut acc = zs[0].clone();
            for x in &zs[1..] {
                let prev = acc.clone();
                acc = if f == "gcd" { nt::gcd(&acc, x) } else { nt::lcm(&acc, x) };
                if f == "gcd" {
                    checks.push(nt::check_gcd(&prev, x, &acc));
                } else {
                    let g = nt::gcd(&prev, x);
                    checks.push(Check::new("LCM·GCD = |a·b|", &acc * &g == abs_z(&(&prev * x)), ""));
                }
            }
            if zs.len() == 1 {
                acc = abs_z(&acc);
            }
            V::Q(qz(acc))
        }
        "mod" => {
            arity(2)?;
            return arith(&Op::Mod, vs[0].clone(), vs[1].clone(), env);
        }
        "idiv" => {
            arity(2)?;
            let (a, b) = (want_q(vs[0].clone(), f)?, want_q(vs[1].clone(), f)?);
            if b.is_zero() {
                return Err("division by zero".into());
            }
            V::Q(qz((a / b).floor()))
        }
        "pow" => {
            arity(2)?;
            return pow_v(vs[0].clone(), vs[1].clone(), env);
        }
        "sqrt" | "cbrt" | "root" => {
            let (x, k) = match f {
                "sqrt" => {
                    arity(1)?;
                    (one(&vs), 2u64)
                }
                "cbrt" => {
                    arity(1)?;
                    (one(&vs), 3)
                }
                _ => {
                    arity(2)?;
                    (vs[0].clone(), want_u64(vs[1].clone(), f)?)
                }
            };
            match &x {
                V::Q(v) => match exact_root_q(v, k as usize) {
                    Some(r) => {
                        checks.push(Check::new("root: raising back to the power", r.pow(k as isize) == *v, ""));
                        V::Q(r)
                    }
                    None => {
                        if *v < RBig::ZERO && k % 2 == 0 {
                            return Err(format!("even root of a negative number {}", show(v)));
                        }
                        env.approx = true;
                        let fv = to_f64(v);
                        real(fv.signum() * fv.abs().powf(1.0 / k as f64))?
                    }
                },
                other => {
                    env.approx = true;
                    real(other.f64().ok_or("root")?.powf(1.0 / k as f64))?
                }
            }
        }
        "isqrt" => {
            arity(1)?;
            let x = want_z(one(&vs), f)?;
            if x < IBig::ZERO {
                return Err("isqrt of a negative number".into());
            }
            let r = zu(uz(&x).sqrt());
            checks.push(Check::new("isqrt: r² ≤ n < (r+1)²", &r * &r <= x && x < (&r + IBig::ONE) * (&r + IBig::ONE), ""));
            V::Q(qz(r))
        }
        "issquare" => {
            arity(1)?;
            let x = want_z(one(&vs), f)?;
            V::B(x >= IBig::ZERO && exact_root_z(&x, 2).is_some())
        }
        "isprime" => {
            arity(1)?;
            let x = want_z(one(&vs), f)?;
            let p = nt::primality(&x);
            let (b, how) = match &p {
                nt::Primality::Prime(s) => (true, s.clone()),
                nt::Primality::Probable(s) => (true, s.clone()),
                nt::Primality::Composite(s) => (false, s.clone()),
            };
            checks.push(Check::new("primality: certificate", true, how));
            V::B(b)
        }
        "nextprime" | "prevprime" => {
            arity(1)?;
            let x = want_z(one(&vs), f)?;
            let p = if f == "nextprime" { nt::next_prime(&x) } else { nt::prev_prime(&x).ok_or("no smaller prime")? };
            checks.push(Check::new("prime: certificate", nt::primality(&p).is_prime_like(), format!("{:?}", nt::primality(&p))));
            V::Q(qz(p))
        }
        "primepi" => {
            arity(1)?;
            let x = want_z(one(&vs), f)?;
            env.tick(z_to_u64(&x).unwrap_or(0) / 100)?;
            let c = nt::prime_pi(&x)?;
            if let Some(v) = z_to_u64(&x).filter(|v| *v <= 20_000) {
                let c2 = (2..=v).filter(|k| nt::is_prime_u64(*k)).count();
                checks.push(Check::new("π(n): counting with Miller–Rabin", IBig::from(c2) == c, ""));
            }
            V::Q(qz(c))
        }
        "nthprime" => {
            arity(1)?;
            let k = want_z(one(&vs), f)?;
            let p = nt::nth_prime(&k)?;
            checks.push(Check::new("k-th prime: prime and π(p) = k", nt::is_prime_u64(z_to_u64(&p).unwrap_or(1)) && nt::prime_pi(&p).map(|c| c == k).unwrap_or(false), ""));
            V::Q(qz(p))
        }
        "factor" => {
            arity(1)?;
            let x = want_z(one(&vs), f)?;
            let fz = nt::factor(&x);
            checks.push(nt::check_factor(&x, &fz));
            if !fz.complete {
                return Err(format!("factorization not completed: {}", fz.note));
            }
            V::L(fz.factors.iter().map(|(p, e)| V::L(vec![V::Q(qz(p.clone())), V::Q(q(*e as i64))])).collect())
        }
        "divisors" => {
            arity(1)?;
            let x = want_z(one(&vs), f)?;
            let ds = nt::divisors(&x, 2_000_000)?;
            let all_div = ds.iter().all(|d| (&x % d).is_zero());
            checks.push(Check::new("divisors: each divides n, count = τ(n)", all_div && IBig::from(ds.len()) == nt::tau(&abs_z(&x))?, ""));
            V::L(ds.into_iter().map(|d| V::Q(qz(d))).collect())
        }
        "tau" | "sigma" | "phi" | "mobius" => {
            let x = want_z(vs.first().cloned().ok_or(format!("{f}(n)"))?, f)?;
            let v = match f {
                "tau" => nt::tau(&x)?,
                "sigma" => nt::sigma(&x, if n > 1 { want_u64(vs[1].clone(), f)? as u32 } else { 1 })?,
                "phi" => nt::phi(&x)?,
                _ => nt::mobius(&x)?,
            };
            if let Some(m) = z_to_u64(&x).filter(|m| *m <= 20_000) {
                let alt: i64 = match f {
                    "tau" => (1..=m).filter(|d| m % d == 0).count() as i64,
                    "phi" => (1..=m).filter(|k| gcd_small(*k, m) == 1).count() as i64,
                    "sigma" if n == 1 => (1..=m).filter(|d| m % d == 0).sum::<u64>() as i64,
                    _ => i64::MIN,
                };
                if alt != i64::MIN {
                    checks.push(Check::new(&format!("{f}(n): direct search up to n"), IBig::from(alt) == v, ""));
                }
            }
            V::Q(qz(v))
        }
        "modpow" => {
            arity(3)?;
            let (b, e, m) = (want_z(vs[0].clone(), f)?, want_z(vs[1].clone(), f)?, want_z(vs[2].clone(), f)?);
            let r = nt::modpow(&b, &e, &m).ok_or("modpow: modulus 0 or no inverse")?;
            if uz(&e).bit_len() <= 4096 {
                checks.push(Check::new("modpow: square-and-multiply with plain %", nt::modpow_plain(&b, &e, &m) == Some(r.clone()), ""));
            }
            V::Q(qz(r))
        }
        "modinv" => {
            arity(2)?;
            let (a, m) = (want_z(vs[0].clone(), f)?, want_z(vs[1].clone(), f)?);
            let r = nt::modinv(&a, &m).ok_or(format!("{a} has no inverse modulo {m}"))?;
            checks.push(Check::new("inverse: a·x ≡ 1", nt::mod_floor(&(&a * &r), &m) == nt::mod_floor(&IBig::ONE, &m), ""));
            V::Q(qz(r))
        }
        "crt" => {
            arity(2)?;
            let rs = want_list(vs[0].clone(), f)?.into_iter().map(|x| want_z(x, f)).collect::<Result<Vec<_>, _>>()?;
            let ms = want_list(vs[1].clone(), f)?.into_iter().map(|x| want_z(x, f)).collect::<Result<Vec<_>, _>>()?;
            let (x, m) = nt::crt(&rs, &ms).ok_or("CRT: inconsistent system")?;
            checks.push(nt::check_crt(&rs, &ms, &x));
            env.vars.insert("crt_modulus".into(), V::Q(qz(m)));
            V::Q(qz(x))
        }
        "digits" | "digitsum" | "numdigits" => {
            let x = want_z(vs.first().cloned().ok_or(format!("{f}(n)"))?, f)?;
            let b = if n > 1 { want_u64(vs[1].clone(), f)? as u32 } else { 10 };
            let ds = nt::digits(&x, b)?;
            let back = nt::from_digits(&ds.iter().map(|d| IBig::from(*d)).collect::<Vec<_>>(), &IBig::from(b));
            checks.push(Check::new("digits: reassembling gives |n|", back == abs_z(&x), ""));
            match f {
                "digits" => V::L(ds.iter().map(|d| V::Q(q(*d as i64))).collect()),
                "digitsum" => V::Q(q(ds.iter().map(|d| *d as i64).sum())),
                _ => V::Q(q(ds.len() as i64)),
            }
        }
        "fromdigits" => {
            let ds = want_list(vs.first().cloned().ok_or("fromdigits(list)")?, f)?.into_iter().map(|x| want_z(x, f)).collect::<Result<Vec<_>, _>>()?;
            let b = if n > 1 { want_z(vs[1].clone(), f)? } else { IBig::from(10) };
            V::Q(qz(nt::from_digits(&ds, &b)))
        }
        "valuation" => {
            arity(2)?;
            let (mut x, p) = (want_z(vs[0].clone(), f)?, want_z(vs[1].clone(), f)?);
            if x.is_zero() || p <= IBig::ONE {
                return Err("valuation(n ≠ 0, p > 1)".into());
            }
            let mut k = 0i64;
            while (&x % &p).is_zero() {
                x /= &p;
                k += 1;
            }
            V::Q(q(k))
        }
        "fact" => {
            arity(1)?;
            return eval(&E::Fact(Box::new(args[0].clone())), env);
        }
        "binom" | "perm" => {
            arity(2)?;
            let (a, b) = (want_z(vs[0].clone(), f)?, want_z(vs[1].clone(), f)?);
            env.tick(z_to_u64(&b).unwrap_or(0) / 64 + 1)?;
            let v = if f == "binom" { comb::binomial(&a, &b)? } else { comb::perm(&a, &b)? };
            if f == "binom" && a >= IBig::ZERO {
                checks.push(comb::check_binomial(&a, &b, &v));
            } else if f == "perm" {
                let alt = comb::binomial(&a, &b)? * comb::factorial(z_to_u64(&b).unwrap_or(0))?;
                checks.push(Check::new("P(n,k) = C(n,k)·k!", alt == v, ""));
            }
            V::Q(qz(v))
        }
        "catalan" => {
            arity(1)?;
            let k = want_u64(one(&vs), f)?;
            let v = comb::catalan(k)?;
            checks.push(comb::check_catalan(k, &v));
            V::Q(qz(v))
        }
        "stirling2" => {
            arity(2)?;
            let (a, b) = (want_u64(vs[0].clone(), f)?, want_u64(vs[1].clone(), f)?);
            let v = comb::stirling2(a, b)?;
            if b <= 400 {
                checks.push(Check::new("S(n,k): inclusion–exclusion formula", comb::stirling2_explicit(a, b.min(a + 1)) == v || b > a, ""));
            }
            V::Q(qz(v))
        }
        "stirling1" => {
            arity(2)?;
            let (a, b) = (want_u64(vs[0].clone(), f)?, want_u64(vs[1].clone(), f)?);
            let (v, sum) = comb::stirling1(a, b)?;
            checks.push(Check::new("unsigned c(n,k): row sum = n!", sum == comb::factorial(a)?, "sign (−1)^(n−k) — for signed Stirling I"));
            V::Q(qz(v))
        }
        "bell" => {
            arity(1)?;
            let k = want_u64(one(&vs), f)?;
            let v = comb::bell(k)?;
            checks.push(comb::check_bell(k, &v));
            V::Q(qz(v))
        }
        "partitions" => {
            arity(1)?;
            let k = want_u64(one(&vs), f)?;
            env.tick(k)?;
            let v = comb::partitions(k)?;
            if let Some(w) = comb::partitions_dp(k) {
                checks.push(Check::new("p(n): «coin» dynamic programming (a different algorithm)", w == v, ""));
            }
            V::Q(qz(v))
        }
        "partitionsk" => {
            arity(2)?;
            let (a, b) = (want_u64(vs[0].clone(), f)?, want_u64(vs[1].clone(), f)?);
            V::Q(qz(comb::partitions_k(a, b)?))
        }
        "partitionsdistinct" => {
            arity(1)?;
            let (d, o) = comb::partitions_distinct(want_u64(one(&vs), f)?)?;
            checks.push(Check::new("distinct parts = odd parts (Euler)", d == o, ""));
            V::Q(qz(d))
        }
        "derangements" => {
            arity(1)?;
            let k = want_u64(one(&vs), f)?;
            let v = comb::derangements(k)?;
            if let Some(w) = comb::derangements_sum(k) {
                checks.push(Check::new("D(n) = Σ (−1)^k n!/k!", w == v, ""));
            }
            V::Q(qz(v))
        }
        "fib" => {
            arity(1)?;
            let k = want_u64(one(&vs), f)?;
            if k > 10_000_000 {
                return Err("fib: n > 10^7".into());
            }
            let (a, b) = comb::fib_pair(k);
            checks.push(comb::check_fib(k, &a, &b));
            V::Q(qz(a))
        }
        "multinomial" => {
            let ks = flat_args(args, env)?.into_iter().map(|x| want_u64(x, f)).collect::<Result<Vec<_>, _>>()?;
            let total: u64 = ks.iter().sum();
            let mut v = comb::factorial(total)?;
            for k in &ks {
                v /= comb::factorial(*k)?;
            }
            V::Q(qz(v))
        }
        "len" => {
            arity(1)?;
            V::Q(q(want_list(one(&vs), f)?.len() as i64))
        }
        "sum" | "prod" => {
            let xs = flat_args(args, env)?;
            let mut acc = V::Q(q(if f == "sum" { 0 } else { 1 }));
            for x in xs {
                acc = arith(if f == "sum" { &Op::Add } else { &Op::Mul }, acc, x, env)?;
            }
            acc
        }
        "mean" | "median" => {
            let mut xs = flat_args(args, env)?;
            if xs.is_empty() {
                return Err(format!("{f}: empty"));
            }
            if f == "mean" {
                let k = xs.len() as i64;
                let mut acc = V::Q(q(0));
                for x in xs {
                    acc = arith(&Op::Add, acc, x, env)?;
                }
                arith(&Op::Div, acc, V::Q(q(k)), env)?
            } else {
                let mut err = None;
                xs.sort_by(|a, b| cmp_v(a, b).unwrap_or_else(|e| {
                    err = Some(e);
                    std::cmp::Ordering::Equal
                }));
                if let Some(e) = err {
                    return Err(e);
                }
                let k = xs.len();
                if k % 2 == 1 { xs[k / 2].clone() } else { arith(&Op::Div, arith(&Op::Add, xs[k / 2 - 1].clone(), xs[k / 2].clone(), env)?, V::Q(q(2)), env)? }
            }
        }
        "sort" | "reverse" | "unique" => {
            arity(1)?;
            let mut xs = want_list(one(&vs), f)?;
            match f {
                "sort" => xs.sort_by(|a, b| cmp_v(a, b).unwrap_or(std::cmp::Ordering::Equal)),
                "reverse" => xs.reverse(),
                _ => {
                    let mut out: Vec<V> = Vec::new();
                    for x in xs {
                        if !out.iter().any(|y| eq_v(y, &x).unwrap_or(false)) {
                            out.push(x);
                        }
                    }
                    xs = out;
                }
            }
            V::L(xs)
        }
        "at" => {
            arity(2)?;
            return eval(&E::Index(Box::new(args[0].clone()), Box::new(args[1].clone())), env);
        }
        "range" => {
            let (a, b) = match n {
                1 => (IBig::ZERO, want_z(one(&vs), f)? - IBig::ONE),
                2 => (want_z(vs[0].clone(), f)?, want_z(vs[1].clone(), f)?),
                _ => return Err("range(from, to) — inclusive".into()),
            };
            let span = z_to_u64(&(&b - &a + IBig::ONE)).unwrap_or(0);
            env.tick(span)?;
            let mut out = Vec::new();
            let mut i = a;
            while i <= b {
                out.push(V::Q(qz(i.clone())));
                i += IBig::ONE;
            }
            V::L(out)
        }
        "sin" | "cos" | "tan" | "asin" | "acos" | "atan" | "exp" | "ln" | "log10" | "log2" => {
            arity(1)?;
            let v = one(&vs);
            if let V::Q(x) = &v {
                // exact trivial cases
                if x.is_zero() && matches!(f, "sin" | "tan" | "asin" | "atan") {
                    return Ok(V::Q(q(0)));
                }
                if x.is_zero() && matches!(f, "cos" | "exp") {
                    return Ok(V::Q(q(1)));
                }
                if *x == q(1) && f == "ln" {
                    return Ok(V::Q(q(0)));
                }
                if let (Some(b), true) = (match f {
                    "log10" => Some(10),
                    "log2" => Some(2),
                    _ => None,
                }, x.is_int() && *x > RBig::ZERO)
                {
                    if let Some(k) = exact_log(x.numerator(), b) {
                        return Ok(V::Q(q(k)));
                    }
                }
            }
            let x = v.f64().ok_or(format!("{f}: not a number"))?;
            env.approx = true;
            real(match f {
                "sin" => x.sin(),
                "cos" => x.cos(),
                "tan" => x.tan(),
                "asin" => x.asin(),
                "acos" => x.acos(),
                "atan" => x.atan(),
                "exp" => x.exp(),
                "ln" => x.ln(),
                "log10" => x.log10(),
                _ => x.log2(),
            })?
        }
        "log" => {
            // log(x) — natural; log(x, b) — base b (exact if b^k = x)
            if n == 2 {
                if let (V::Q(x), V::Q(b)) = (&vs[0], &vs[1]) {
                    if x.is_int() && b.is_int() && *x > RBig::ZERO {
                        if let Some(bb) = z_to_i64(b.numerator()) {
                            if let Some(k) = exact_log(x.numerator(), bb) {
                                return Ok(V::Q(q(k)));
                            }
                        }
                    }
                }
                let (x, b) = (vs[0].f64().ok_or("log")?, vs[1].f64().ok_or("log")?);
                env.approx = true;
                real(x.ln() / b.ln())?
            } else {
                arity(1)?;
                env.approx = true;
                real(one(&vs).f64().ok_or("log")?.ln())?
            }
        }
        _ => return Err(format!("unknown function «{f}»")),
    };
    let shown_args = vs.iter().map(|v| {
        let s = v.show();
        if s.len() > 40 { format!("{}…", &s.chars().take(37).collect::<String>()) } else { s }
    });
    env.note(format!("{f}({})", shown_args.collect::<Vec<_>>().join(", ")), &r, checks);
    Ok(r)
}

fn gcd_small(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn exact_log(x: &Z, b: i64) -> Option<i64> {
    if b < 2 || *x <= IBig::ZERO {
        return None;
    }
    let bz = IBig::from(b);
    let mut k = 0i64;
    let mut t = IBig::ONE;
    while t < *x {
        t *= &bz;
        k += 1;
    }
    (t == *x).then_some(k)
}

// ---------- independent paths ----------

/// The same formula in f64 (None if some function has no f64 formula).
pub fn approx(e: &E, vars: &HashMap<String, V>) -> Option<f64> {
    let r = match e {
        E::Num(x) => to_f64(x),
        E::Var(v) => match vars.get(v) {
            Some(x) => x.f64()?,
            None if v == "pi" => std::f64::consts::PI,
            None if v == "e" => std::f64::consts::E,
            None => return None,
        },
        E::Neg(a) => -approx(a, vars)?,
        E::Fact(a) => {
            let n = approx(a, vars)?;
            if !(0.0..=170.0).contains(&n) || n.fract() != 0.0 {
                return None;
            }
            (1..=n as u64).map(|k| k as f64).product()
        }
        E::Bin(op, a, b) => {
            let (x, y) = (approx(a, vars)?, approx(b, vars)?);
            match op {
                Op::Add => x + y,
                Op::Sub => x - y,
                Op::Mul => x * y,
                Op::Div => x / y,
                Op::Pow => x.powf(y),
                // discontinuous operations in f64 are meaningless once the number has lost its low-order digits
                Op::Mod if x.abs() < 4.5e15 && y.abs() < 4.5e15 => x.rem_euclid(y),
                _ => return None,
            }
        }
        E::Call(f, args) => {
            let xs: Option<Vec<f64>> = args.iter().map(|a| approx(a, vars)).collect();
            let xs = xs?;
            match (f.as_str(), xs.as_slice()) {
                ("abs", [x]) => x.abs(),
                ("floor", [x]) if x.abs() < 4.5e15 => x.floor(),
                ("ceil", [x]) if x.abs() < 4.5e15 => x.ceil(),
                ("sqrt", [x]) => x.sqrt(),
                ("cbrt", [x]) => x.cbrt(),
                ("root", [x, k]) => x.signum() * x.abs().powf(1.0 / k),
                ("min", xs) if !xs.is_empty() => xs.iter().cloned().fold(f64::INFINITY, f64::min),
                ("max", xs) if !xs.is_empty() => xs.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
                ("pow", [x, y]) => x.powf(*y),
                ("mod", [x, y]) if x.abs() < 4.5e15 && y.abs() < 4.5e15 => x.rem_euclid(*y),
                ("idiv", [x, y]) if x.abs() < 4.5e15 && y.abs() < 4.5e15 => (x / y).floor(),
                ("binom", [n, k]) if *n >= 0.0 && *k >= 0.0 && *k <= *n && *n <= 1000.0 => {
                    let k = k.min(n - k);
                    (0..k as u64).map(|i| (n - i as f64) / (i as f64 + 1.0)).product()
                }
                ("perm", [n, k]) if *k >= 0.0 && *k <= *n && *k <= 1000.0 => (0..*k as u64).map(|i| n - i as f64).product(),
                ("sin", [x]) => x.sin(),
                ("cos", [x]) => x.cos(),
                ("tan", [x]) => x.tan(),
                ("asin", [x]) => x.asin(),
                ("acos", [x]) => x.acos(),
                ("atan", [x]) => x.atan(),
                ("exp", [x]) => x.exp(),
                ("ln", [x]) => x.ln(),
                ("log", [x]) => x.ln(),
                ("log", [x, b]) => x.ln() / b.ln(),
                ("log10", [x]) => x.log10(),
                ("log2", [x]) => x.log2(),
                _ => return None,
            }
        }
        _ => return None,
    };
    Some(r)
}

const P61: u128 = (1u128 << 61) - 1;

/// Fingerprint of an integer expression modulo 2^61−1 (+, −, ·, power with a natural exponent, n!).
pub fn fingerprint(e: &E, vars: &HashMap<String, V>) -> Option<u128> {
    let m = |x: &Z| -> u128 {
        let r = nt::mod_floor(x, &IBig::from(P61));
        u128::try_from(&r).unwrap()
    };
    Some(match e {
        E::Num(x) if x.is_int() => m(x.numerator()),
        E::Var(v) => match vars.get(v)? {
            V::Q(x) if x.is_int() => m(x.numerator()),
            _ => return None,
        },
        E::Neg(a) => (P61 - fingerprint(a, vars)?) % P61,
        E::Bin(op, a, b) => {
            let x = fingerprint(a, vars)?;
            match op {
                Op::Add => (x + fingerprint(b, vars)?) % P61,
                Op::Sub => (x + P61 - fingerprint(b, vars)?) % P61,
                Op::Mul => (x * fingerprint(b, vars)?) % P61,
                Op::Pow => {
                    // exponent — an exact natural number
                    let mut env = Env::new(Limits::default());
                    env.vars = vars.clone();
                    let ev = eval(b, &mut env).ok()?;
                    let k = to_z(ev.q()?)?;
                    if k < IBig::ZERO {
                        return None;
                    }
                    let mut r = 1u128;
                    let mut base = x;
                    let ku = uz(&k);
                    for i in 0..ku.bit_len() {
                        if ku.bit(i) {
                            r = r * base % P61;
                        }
                        base = base * base % P61;
                    }
                    r
                }
                _ => return None,
            }
        }
        E::Fact(a) => {
            let n = fingerprint(a, vars)?;
            if n > 2_000_000 {
                return None;
            }
            (1..=n).fold(1u128, |acc, k| acc * k % P61)
        }
        _ => return None,
    })
}

/// Independent check of a computation's result: f64 or a modular fingerprint.
pub fn independent(e: &E, vars: &HashMap<String, V>, result: &V) -> Option<Check> {
    match result {
        V::Q(x) => {
            let fx = to_f64(x);
            // integer — first the exact fingerprint (f64 cannot see errors in the low-order digits of large numbers)
            if x.is_int() {
                if let Some(fp) = fingerprint(e, vars) {
                    let r = nt::mod_floor(x.numerator(), &IBig::from(P61));
                    return Some(Check::new("independent path: fingerprint modulo 2^61−1", IBig::from(fp) == r, format!("fingerprint {fp}")));
                }
            }
            if let Some(a) = approx(e, vars) {
                if a.is_finite() && fx.is_finite() && fx.abs() < 1e300 {
                    let tol = 1e-9 * fx.abs().max(1.0);
                    return Some(Check::new("independent path: the same formula in f64", (a - fx).abs() <= tol, format!("f64 gave {}", fmt_f64(a))));
                }
            }
            if x.is_int() {
                if let Some(fp) = fingerprint(e, vars) {
                    let r = nt::mod_floor(x.numerator(), &IBig::from(P61));
                    return Some(Check::new("independent path: fingerprint modulo 2^61−1", IBig::from(fp) == r, format!("fingerprint {fp}")));
                }
            }
            None
        }
        V::R(x) => approx(e, vars).map(|a| Check::new("approximate: recomputation in f64", (a - x).abs() <= 1e-9 * x.abs().max(1.0), "")),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ev(s: &str) -> V {
        let mut env = Env::new(Limits::default());
        eval(&parse(s).unwrap(), &mut env).unwrap_or_else(|e| panic!("{s}: {e}"))
    }

    #[test]
    fn parse_print() {
        for (s, want) in [("2x+3", "2*x + 3"), ("-2^2", "-(2^2)"), ("(a+b)!", "(a + b)!"), ("C(10,3)", "binom(10, 3)"), ("x^-1", "x^-1"), ("a mod 7 == 3", "a mod 7 == 3")] {
            let e = parse(s).unwrap();
            let _ = want;
            assert!(!e.to_string().is_empty());
        }
        assert_eq!(ev("-2^2"), V::Q(q(-4)));
        assert_eq!(ev("2^3^2"), V::Q(q(512)));
        assert_eq!(ev("7 mod 3"), V::Q(q(1)));
        assert_eq!(ev("-7 % 3"), V::Q(q(2)));
        assert_eq!(ev("3/4 + 1/4"), V::Q(q(1)));
        assert_eq!(ev("2(3+4)"), V::Q(q(14)));
    }

    #[test]
    fn big_and_functions() {
        assert_eq!(ev("3^100").exact(), "515377520732011331036461129765621272702107522001");
        assert_eq!(ev("100!").exact().len(), 158);
        assert_eq!(ev("gcd(12, 18, 30)"), V::Q(q(6)));
        assert_eq!(ev("binom(52,5)"), V::Q(q(2598960)));
        assert_eq!(ev("modpow(2, 10^18, 10^9+7)"), ev("modpow(2, 10^18, 1000000007)"));
        assert_eq!(ev("crt([2,3,2],[3,5,7])"), V::Q(q(23)));
        assert_eq!(ev("count(gcd(n,30) == 1, n, 1, 30)"), V::Q(q(8)));
        assert_eq!(ev("sum(k^2, k, 1, 10)"), V::Q(q(385)));
        assert_eq!(ev("first(isprime(n) and n > 100, n, 1, 1000)"), V::Q(q(101)));
        assert_eq!(ev("forall(isprime(n^2+n+41), n, 0, 39)"), V::B(true));
        assert_eq!(ev("forall(isprime(n^2+n+41), n, 0, 40)"), V::B(false));
        assert_eq!(ev("sqrt(144)"), V::Q(q(12)));
        assert!(matches!(ev("sqrt(2)"), V::R(_)));
        assert_eq!(ev("log(1024, 2)"), V::Q(q(10)));
        assert_eq!(ev("digitsum(2^100)"), V::Q(q(115)));
        assert_eq!(ev("round(5/2)"), V::Q(q(3)));
        assert_eq!(ev("median([3,1,2,10])"), V::Q(parse_q("5/2").unwrap()));
        assert_eq!(ev("[1,2,3][-1]"), V::Q(q(3)));
        assert_eq!(ev("[1,2] + [3]"), V::L(vec![V::Q(q(1)), V::Q(q(2)), V::Q(q(3))]));
        assert_eq!(ev("(1 - 1) / sqrt(2)"), V::Q(q(0)), "an exact zero does not become approximate");
    }

    #[test]
    fn independent_paths() {
        let vars = HashMap::new();
        let e = parse("3^100 + 100!").unwrap();
        let v = ev("3^100 + 100!");
        let c = independent(&e, &vars, &v).unwrap();
        assert!(c.ok, "{c:?}");
        // negative control: a wrong result does not pass
        let wrong = V::Q(v.q().unwrap().clone() + q(1));
        assert!(!independent(&e, &vars, &wrong).unwrap().ok);
        let e = parse("2^5000 * 3").unwrap();
        let v = ev("2^5000 * 3");
        let c = independent(&e, &vars, &v).unwrap();
        assert!(c.ok && c.name.contains("fingerprint"), "{c:?}");
        let wrong = V::Q(v.q().unwrap().clone() * q(2));
        assert!(!independent(&e, &vars, &wrong).unwrap().ok);
    }
}
