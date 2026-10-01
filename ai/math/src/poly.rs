//! v2: polynomials and rational functions of one variable over Q. Equation solving: rational
//! roots (rational root theorem), quadratics exactly with radicals (a ± b√d, d square-free,
//! d < 0 means complex), the rest numerically (Aberth) marked "≈". Multiplicities via Yun's
//! square-free factorization. Checks: exact substitution of each root (for radicals,
//! arithmetic in Q(√d)), completeness (sum of multiplicities = degree), product of factors = polynomial.

use crate::big::*;
use crate::calc::{E, Env, Op, V, eval};
use crate::nt::{self, Check};

#[derive(Clone, Debug, PartialEq)]
pub struct Poly(pub Vec<Q>);

impl Poly {
    pub fn zero() -> Poly {
        Poly(Vec::new())
    }
    pub fn c(x: Q) -> Poly {
        let mut p = Poly(vec![x]);
        p.trim();
        p
    }
    pub fn one() -> Poly {
        Poly(vec![q(1)])
    }
    pub fn x() -> Poly {
        Poly(vec![q(0), q(1)])
    }
    fn trim(&mut self) {
        while self.0.last().is_some_and(|c| c.is_zero()) {
            self.0.pop();
        }
    }
    pub fn deg(&self) -> Option<usize> {
        if self.0.is_empty() { None } else { Some(self.0.len() - 1) }
    }
    pub fn is_zero(&self) -> bool {
        self.0.is_empty()
    }
    pub fn lead(&self) -> Q {
        self.0.last().cloned().unwrap_or_else(|| q(0))
    }
    pub fn add(&self, o: &Poly) -> Poly {
        let n = self.0.len().max(o.0.len());
        let mut r = Vec::with_capacity(n);
        for i in 0..n {
            let a = self.0.get(i).cloned().unwrap_or_else(|| q(0));
            let b = o.0.get(i).cloned().unwrap_or_else(|| q(0));
            r.push(a + b);
        }
        let mut p = Poly(r);
        p.trim();
        p
    }
    pub fn neg(&self) -> Poly {
        Poly(self.0.iter().map(|c| -c.clone()).collect())
    }
    pub fn sub(&self, o: &Poly) -> Poly {
        self.add(&o.neg())
    }
    pub fn scale(&self, k: &Q) -> Poly {
        let mut p = Poly(self.0.iter().map(|c| c.clone() * k.clone()).collect());
        p.trim();
        p
    }
    pub fn mul(&self, o: &Poly) -> Poly {
        if self.is_zero() || o.is_zero() {
            return Poly::zero();
        }
        let mut r = vec![q(0); self.0.len() + o.0.len() - 1];
        for (i, a) in self.0.iter().enumerate() {
            if a.is_zero() {
                continue;
            }
            for (j, b) in o.0.iter().enumerate() {
                r[i + j] = r[i + j].clone() + a.clone() * b.clone();
            }
        }
        let mut p = Poly(r);
        p.trim();
        p
    }
    pub fn pow(&self, k: u32) -> Poly {
        let mut r = Poly::one();
        for _ in 0..k {
            r = r.mul(self);
        }
        r
    }
    pub fn divrem(&self, d: &Poly) -> (Poly, Poly) {
        assert!(!d.is_zero());
        let mut r = self.clone();
        let dd = d.deg().unwrap();
        let lc = d.lead();
        if r.deg().is_none_or(|x| x < dd) {
            return (Poly::zero(), r);
        }
        let mut quo = vec![q(0); r.0.len() - dd];
        while let Some(rd) = r.deg() {
            if rd < dd {
                break;
            }
            let k = r.lead() / lc.clone();
            let shift = rd - dd;
            quo[shift] = k.clone();
            for (i, c) in d.0.iter().enumerate() {
                r.0[i + shift] = r.0[i + shift].clone() - k.clone() * c.clone();
            }
            r.trim();
        }
        let mut qp = Poly(quo);
        qp.trim();
        (qp, r)
    }
    pub fn monic(&self) -> Poly {
        if self.is_zero() {
            return self.clone();
        }
        let l = self.lead();
        self.scale(&(q(1) / l))
    }
    pub fn gcd(a: &Poly, b: &Poly) -> Poly {
        let (mut x, mut y) = (a.clone(), b.clone());
        while !y.is_zero() {
            let (_, r) = x.divrem(&y);
            x = y;
            y = r;
        }
        x.monic()
    }
    pub fn deriv(&self) -> Poly {
        let mut p = Poly(self.0.iter().enumerate().skip(1).map(|(i, c)| c.clone() * q(i as i64)).collect());
        p.trim();
        p
    }
    pub fn eval(&self, x: &Q) -> Q {
        let mut r = q(0);
        for c in self.0.iter().rev() {
            r = r * x.clone() + c.clone();
        }
        r
    }
    pub fn eval_f64(&self, x: f64) -> f64 {
        self.0.iter().rev().fold(0.0, |r, c| r * x + to_f64(c))
    }
    pub fn eval_c(&self, z: (f64, f64)) -> (f64, f64) {
        self.0.iter().rev().fold((0.0, 0.0), |(a, b), c| (a * z.0 - b * z.1 + to_f64(c), a * z.1 + b * z.0))
    }
    /// Coprime integer coefficients of the same polynomial (up to a constant).
    pub fn primitive(&self) -> Vec<Z> {
        let mut l = IBig::ONE;
        for c in &self.0 {
            l = nt::lcm(&l, &IBig::from(c.denominator().clone()));
        }
        let ints: Vec<Z> = self.0.iter().map(|c| (c.clone() * RBig::from(l.clone())).numerator().clone()).collect();
        let mut g = IBig::ZERO;
        for c in &ints {
            g = nt::gcd(&g, c);
        }
        if g.is_zero() {
            return ints;
        }
        ints.into_iter().map(|c| c / &g).collect()
    }
    pub fn show(&self, var: &str) -> String {
        if self.is_zero() {
            return "0".into();
        }
        let mut s = String::new();
        for (i, c) in self.0.iter().enumerate().rev() {
            if c.is_zero() {
                continue;
            }
            let neg = *c < RBig::ZERO;
            let a = if neg { -c.clone() } else { c.clone() };
            if s.is_empty() {
                if neg {
                    s.push('-');
                }
            } else {
                s.push_str(if neg { " - " } else { " + " });
            }
            let coef = if a == q(1) && i > 0 { String::new() } else if i > 0 { format!("{}*", show_paren(&a)) } else { show(&a) };
            let mono = match i {
                0 => String::new(),
                1 => var.to_string(),
                k => format!("{var}^{k}"),
            };
            s.push_str(&coef);
            s.push_str(&mono);
        }
        s
    }
}

fn show_paren(x: &Q) -> String {
    if x.is_int() { show(x) } else { format!("({})", show(x)) }
}

// ---------- expression → rational function ----------

pub struct RatFn {
    pub num: Poly,
    pub den: Poly,
    /// all denominators encountered (their roots lie outside the domain)
    pub dens: Vec<Poly>,
}

fn has_var(e: &E, var: &str) -> bool {
    let mut vs = Vec::new();
    e.vars(&mut vs);
    vs.iter().any(|v| v == var)
}

pub fn ratfunc(e: &E, var: &str, env: &mut Env) -> Result<RatFn, String> {
    let mut dens = Vec::new();
    let (n, d) = rf(e, var, env, &mut dens)?;
    Ok(RatFn { num: n, den: d, dens })
}

fn rf(e: &E, var: &str, env: &mut Env, dens: &mut Vec<Poly>) -> Result<(Poly, Poly), String> {
    if !has_var(e, var) {
        let v = eval(e, env)?;
        return match v {
            V::Q(x) => Ok((Poly::c(x), Poly::one())),
            V::R(x) => Err(format!("constant {} is approximate: exact solving is impossible (expression {e})", fmt_f64(x))),
            o => Err(format!("constant is not a number: {}", o.show())),
        };
    }
    match e {
        E::Var(v) if v == var => Ok((Poly::x(), Poly::one())),
        E::Neg(a) => {
            let (n, d) = rf(a, var, env, dens)?;
            Ok((n.neg(), d))
        }
        E::Bin(op @ (Op::Add | Op::Sub), a, b) => {
            let (n1, d1) = rf(a, var, env, dens)?;
            let (n2, d2) = rf(b, var, env, dens)?;
            let (x, y) = (n1.mul(&d2), n2.mul(&d1));
            Ok(reduce(if *op == Op::Add { x.add(&y) } else { x.sub(&y) }, d1.mul(&d2)))
        }
        E::Bin(Op::Mul, a, b) => {
            let (n1, d1) = rf(a, var, env, dens)?;
            let (n2, d2) = rf(b, var, env, dens)?;
            Ok(reduce(n1.mul(&n2), d1.mul(&d2)))
        }
        E::Bin(Op::Div, a, b) => {
            let (n1, d1) = rf(a, var, env, dens)?;
            let (n2, d2) = rf(b, var, env, dens)?;
            if n2.is_zero() {
                return Err("division by identically zero".into());
            }
            if n2.deg().unwrap_or(0) > 0 {
                dens.push(n2.clone());
            }
            Ok(reduce(n1.mul(&d2), d1.mul(&n2)))
        }
        E::Bin(Op::Pow, a, b) => {
            if has_var(b, var) {
                return Err(format!("variable in the exponent ({e}): not a polynomial"));
            }
            let k = eval(b, env)?;
            let k = k.q().and_then(to_i64).ok_or(format!("exponent {} is not an integer: not a polynomial", k.show()))?;
            if k.abs() > 200 {
                return Err("exponent too large".into());
            }
            let (n, d) = rf(a, var, env, dens)?;
            if k >= 0 {
                Ok((n.pow(k as u32), d.pow(k as u32)))
            } else {
                if n.deg().unwrap_or(0) > 0 {
                    dens.push(n.clone());
                }
                Ok((d.pow((-k) as u32), n.pow((-k) as u32)))
            }
        }
        _ => Err(format!("\"{e}\" is not a rational function of {var}")),
    }
}

fn reduce(n: Poly, d: Poly) -> (Poly, Poly) {
    if n.is_zero() {
        return (n, Poly::one());
    }
    let g = Poly::gcd(&n, &d);
    let (n2, _) = n.divrem(&g);
    let (d2, _) = d.divrem(&g);
    // the denominator is monic (leading coefficient 1)
    let l = d2.lead();
    (n2.scale(&(q(1) / l.clone())), d2.scale(&(q(1) / l)))
}

// ---------- roots ----------

/// a + b√d (d square-free, d ≠ 0, 1; d < 0 means complex).
#[derive(Clone, Debug, PartialEq)]
pub struct Surd {
    pub a: Q,
    pub b: Q,
    pub d: Z,
}

impl Surd {
    fn mul(&self, o: &Surd) -> Surd {
        Surd { a: self.a.clone() * o.a.clone() + self.b.clone() * o.b.clone() * qz(self.d.clone()), b: self.a.clone() * o.b.clone() + self.b.clone() * o.a.clone(), d: self.d.clone() }
    }
    fn add_q(&self, c: &Q) -> Surd {
        Surd { a: self.a.clone() + c.clone(), b: self.b.clone(), d: self.d.clone() }
    }
    pub fn f64(&self) -> (f64, f64) {
        let d = z_f64(&self.d);
        if d >= 0.0 { (to_f64(&self.a) + to_f64(&self.b) * d.sqrt(), 0.0) } else { (to_f64(&self.a), to_f64(&self.b) * (-d).sqrt()) }
    }
    pub fn show(&self) -> String {
        let rad = if self.d < IBig::ZERO {
            if self.d == IBig::from(-1) { "i".to_string() } else { format!("i√{}", -self.d.clone()) }
        } else {
            format!("√{}", self.d)
        };
        let bneg = self.b < RBig::ZERO;
        let babs = if bneg { -self.b.clone() } else { self.b.clone() };
        let bpart = if babs == q(1) { rad } else { format!("{}{}", show_paren(&babs), rad) };
        if self.a.is_zero() { format!("{}{}", if bneg { "-" } else { "" }, bpart) } else { format!("{} {} {}", show(&self.a), if bneg { "-" } else { "+" }, bpart) }
    }
}

fn eval_surd(p: &Poly, s: &Surd) -> Surd {
    let mut r = Surd { a: q(0), b: q(0), d: s.d.clone() };
    for c in p.0.iter().rev() {
        r = r.mul(s).add_q(c);
    }
    r
}

#[derive(Clone, Debug)]
pub enum Root {
    Q(Q),
    Surd(Surd),
    /// approximate real
    Approx(f64),
    /// approximate complex
    ApproxC(f64, f64),
}

impl Root {
    pub fn is_real(&self) -> bool {
        match self {
            Root::Q(_) | Root::Approx(_) => true,
            Root::Surd(s) => s.d > IBig::ZERO,
            Root::ApproxC(_, im) => im.abs() < 1e-12,
        }
    }
    pub fn f64(&self) -> (f64, f64) {
        match self {
            Root::Q(x) => (to_f64(x), 0.0),
            Root::Surd(s) => s.f64(),
            Root::Approx(x) => (*x, 0.0),
            Root::ApproxC(a, b) => (*a, *b),
        }
    }
    pub fn show(&self) -> String {
        match self {
            Root::Q(x) => show(x),
            Root::Surd(s) => format!("{} (≈ {})", s.show(), fmt_f64(s.f64().0)),
            Root::Approx(x) => format!("≈ {}", fmt_f64(*x)),
            Root::ApproxC(a, b) => format!("≈ {} {} {}i", fmt_f64(*a), if *b < 0.0 { "-" } else { "+" }, fmt_f64(b.abs())),
        }
    }
    pub fn value(&self) -> V {
        match self {
            Root::Q(x) => V::Q(x.clone()),
            r => {
                let (a, b) = r.f64();
                if b.abs() < 1e-12 { V::R(a) } else { V::S(r.show()) }
            }
        }
    }
}

/// Yun's factorization: f = ∏ aᵢ^i, aᵢ square-free and pairwise coprime.
pub fn squarefree(f: &Poly) -> Vec<(Poly, u32)> {
    let mut out = Vec::new();
    if f.deg().unwrap_or(0) == 0 {
        return out;
    }
    let fp = f.deriv();
    let a0 = Poly::gcd(f, &fp);
    let mut b = f.divrem(&a0).0;
    let mut c = fp.divrem(&a0).0;
    let mut d = c.sub(&b.deriv());
    let mut i = 1;
    while b.deg().unwrap_or(0) > 0 {
        let a = Poly::gcd(&b, &d);
        if a.deg().unwrap_or(0) > 0 {
            out.push((a.clone(), i));
        }
        b = b.divrem(&a).0;
        c = d.divrem(&a).0;
        d = c.sub(&b.deriv());
        i += 1;
        if i > 400 {
            break;
        }
    }
    out
}

/// Rational roots of a square-free polynomial (rational root theorem;
/// for large coefficients, candidates near the numeric roots).
fn rational_roots(p: &Poly) -> Vec<Q> {
    let ints = p.primitive();
    let mut roots = Vec::new();
    let mut work = p.clone();
    // root 0
    if ints.first().is_some_and(|c| c.is_zero()) {
        roots.push(q(0));
        work = work.divrem(&Poly::x()).0;
    }
    let ints = work.primitive();
    let (a0, an) = (ints.first().cloned().unwrap_or_default(), ints.last().cloned().unwrap_or_default());
    if work.deg().unwrap_or(0) == 0 {
        return roots;
    }
    let small = |x: &Z| abs_z(x) <= IBig::from(1_000_000_000_000i64);
    if small(&a0) && small(&an) {
        if let (Ok(ps), Ok(qs)) = (nt::divisors(&a0, 20_000), nt::divisors(&an, 20_000)) {
            for pn in &ps {
                for qd in &qs {
                    for sgn in [1i64, -1] {
                        let cand = frac(pn.clone() * IBig::from(sgn), qd.clone()).unwrap();
                        if !roots.contains(&cand) && work.eval(&cand).is_zero() {
                            roots.push(cand);
                        }
                    }
                }
            }
            return roots;
        }
    }
    // large coefficients: the root's denominator divides an; candidates round(r·q)/q near the numeric roots
    let qs = nt::divisors(&an, 2000).unwrap_or_else(|_| vec![IBig::ONE]);
    for (re, im) in numeric_roots(&work) {
        if im.abs() > 1e-6 * re.abs().max(1.0) {
            continue;
        }
        for qd in &qs {
            let pn = (re * z_f64(qd)).round();
            if !pn.is_finite() {
                continue;
            }
            if let Some(cand) = RBig::simplest_from_f64(pn).and_then(|pq| to_z(&pq)).and_then(|pz| frac(pz, qd.clone())) {
                if !roots.contains(&cand) && work.eval(&cand).is_zero() {
                    roots.push(cand);
                }
            }
        }
    }
    roots
}

/// Numeric roots (Aberth–Ehrlich) in complex f64.
pub fn numeric_roots(p: &Poly) -> Vec<(f64, f64)> {
    let n = match p.deg() {
        Some(n) if n > 0 => n,
        _ => return Vec::new(),
    };
    let c: Vec<f64> = p.0.iter().map(to_f64).collect();
    let lead = c[n];
    let c: Vec<f64> = c.iter().map(|x| x / lead).collect();
    let rbound = 1.0 + c[..n].iter().fold(0.0f64, |m, x| m.max(x.abs()));
    let mut z: Vec<(f64, f64)> = (0..n).map(|k| {
        let t = 2.0 * std::f64::consts::PI * (k as f64 + 0.25) / n as f64;
        (0.5 * rbound * t.cos(), 0.5 * rbound * t.sin())
    }).collect();
    let ev = |z: (f64, f64)| -> ((f64, f64), (f64, f64)) {
        let (mut pr, mut pi, mut dr, mut di) = (1.0, 0.0, 0.0, 0.0);
        for k in (0..n).rev() {
            // d = d·z + p ; p = p·z + c
            let (ndr, ndi) = (dr * z.0 - di * z.1 + pr, dr * z.1 + di * z.0 + pi);
            dr = ndr;
            di = ndi;
            let (npr, npi) = (pr * z.0 - pi * z.1 + c[k], pr * z.1 + pi * z.0);
            pr = npr;
            pi = npi;
        }
        ((pr, pi), (dr, di))
    };
    let div = |a: (f64, f64), b: (f64, f64)| {
        let m = b.0 * b.0 + b.1 * b.1;
        ((a.0 * b.0 + a.1 * b.1) / m, (a.1 * b.0 - a.0 * b.1) / m)
    };
    for _ in 0..500 {
        let mut maxstep: f64 = 0.0;
        for i in 0..n {
            let (pv, dv) = ev(z[i]);
            let ratio = div(pv, dv);
            let mut s = (0.0, 0.0);
            for j in 0..n {
                if i != j {
                    let dz = (z[i].0 - z[j].0, z[i].1 - z[j].1);
                    let inv = div((1.0, 0.0), dz);
                    s = (s.0 + inv.0, s.1 + inv.1);
                }
            }
            // w = ratio / (1 − ratio·s)
            let rs = (ratio.0 * s.0 - ratio.1 * s.1, ratio.0 * s.1 + ratio.1 * s.0);
            let w = div(ratio, (1.0 - rs.0, -rs.1));
            if w.0.is_finite() && w.1.is_finite() {
                z[i] = (z[i].0 - w.0, z[i].1 - w.1);
                maxstep = maxstep.max((w.0 * w.0 + w.1 * w.1).sqrt());
            }
        }
        if maxstep < 1e-15 {
            break;
        }
    }
    z
}

/// Quadratic trinomial ax²+bx+c without rational roots → a₀ ± b₀√d.
fn quadratic_surds(p: &Poly) -> Option<(Surd, Surd)> {
    let (c, b, a) = (p.0[0].clone(), p.0[1].clone(), p.0[2].clone());
    let disc = b.clone() * b.clone() - q(4) * a.clone() * c;
    // √(N/M) = √(N·M)/M; N·M = s²·d
    let nm = disc.numerator().clone() * IBig::from(disc.denominator().clone());
    let m = IBig::from(disc.denominator().clone());
    let f = nt::factor(&abs_z(&nm));
    if !f.complete {
        return None;
    }
    let mut s = IBig::ONE;
    let mut d = IBig::ONE;
    for (pr, e) in &f.factors {
        s *= pr.pow((*e / 2) as usize);
        if e % 2 == 1 {
            d *= pr;
        }
    }
    if nm < IBig::ZERO {
        d = -d;
    }
    let a0 = -b / (q(2) * a.clone());
    let b0 = qz(s) / (q(2) * a * qz(m));
    Some((Surd { a: a0.clone(), b: b0.clone(), d: d.clone() }, Surd { a: a0, b: -b0, d }))
}

pub struct Solved {
    pub roots: Vec<(Root, u32)>,
    pub checks: Vec<Check>,
    pub notes: Vec<String>,
}

/// All complex roots of a polynomial with multiplicities and checks.
pub fn solve_poly(p: &Poly) -> Result<Solved, String> {
    let deg = p.deg().ok_or("equation 0 = 0: any value")?;
    let mut roots: Vec<(Root, u32)> = Vec::new();
    let mut checks = Vec::new();
    let mut notes = Vec::new();
    if deg == 0 {
        return Ok(Solved { roots, checks: vec![Check::new("constant ≠ 0: no solutions", true, "")], notes });
    }
    let mut rebuilt = Poly::c(p.lead());
    for (f, m) in squarefree(p) {
        let mut rest = f.monic();
        for r in rational_roots(&rest) {
            let ok = p.eval(&r).is_zero();
            checks.push(Check::new(&format!("substitution x = {}", show(&r)), ok, "exact"));
            roots.push((Root::Q(r.clone()), m));
            rest = rest.divrem(&Poly(vec![-r.clone(), q(1)])).0;
            rebuilt = rebuilt.mul(&Poly(vec![-r, q(1)]).pow(m));
        }
        match rest.deg().unwrap_or(0) {
            0 => {}
            2 => {
                let (s1, s2) = quadratic_surds(&rest).ok_or("failed to extract a square from the discriminant")?;
                for s in [s1, s2] {
                    let v = eval_surd(p, &s);
                    checks.push(Check::new(&format!("substitution x = {} in Q(√{})", s.show(), s.d), v.a.is_zero() && v.b.is_zero(), "exact"));
                    roots.push((Root::Surd(s), m));
                }
                rebuilt = rebuilt.mul(&rest.pow(m));
            }
            k => {
                notes.push(format!("factor of degree {k} without rational roots: {}; roots computed numerically (Cardano/Ferrari radicals not wired in)", rest.show("x")));
                for (re, im) in numeric_roots(&rest) {
                    let (vr, vi) = p.eval_c((re, im));
                    let scale: f64 = p.0.iter().enumerate().map(|(i, c)| to_f64(c).abs() * (re * re + im * im).sqrt().powi(i as i32)).sum();
                    let ok = (vr * vr + vi * vi).sqrt() <= 1e-8 * scale.max(1.0);
                    checks.push(Check::new("substitution of approximate root: |p(z)| ≈ 0", ok, format!("|p(z)| = {:.2e}", (vr * vr + vi * vi).sqrt())));
                    roots.push((if im.abs() <= 1e-10 * re.abs().max(1.0) { Root::Approx(re) } else { Root::ApproxC(re, im) }, m));
                }
                rebuilt = rebuilt.mul(&rest.pow(m));
            }
        }
    }
    let total: u32 = roots.iter().map(|(_, m)| *m).sum();
    checks.push(Check::new("completeness: sum of multiplicities = degree", total as usize == deg, format!("{total} of {deg}")));
    checks.push(Check::new("product of factors = polynomial", rebuilt == *p, ""));
    Ok(Solved { roots, checks, notes })
}

/// Factorization over Q: leading coefficient and irreducible (proven for degrees ≤ 3) factors with multiplicities.
pub fn factor_q(p: &Poly) -> (Q, Vec<(Poly, u32)>, Check) {
    let lead = p.lead();
    let mut out = Vec::new();
    for (f, m) in squarefree(p) {
        let mut rest = f.monic();
        for r in rational_roots(&rest) {
            let lin = Poly(vec![-r.clone(), q(1)]);
            rest = rest.divrem(&lin).0;
            out.push((lin, m));
        }
        if rest.deg().unwrap_or(0) > 0 {
            out.push((rest, m));
        }
    }
    let rebuilt = out.iter().fold(Poly::c(lead.clone()), |acc, (f, m)| acc.mul(&f.pow(*m)));
    let c = Check::new("factorization: product of factors = polynomial", rebuilt == *p, "");
    (lead, out, c)
}

pub fn show_factorization(lead: &Q, fs: &[(Poly, u32)], var: &str) -> String {
    let mut parts = Vec::new();
    if *lead != q(1) || fs.is_empty() {
        parts.push(show_paren(lead));
    }
    for (f, m) in fs {
        let s = format!("({})", f.show(var));
        parts.push(if *m > 1 { format!("{s}^{m}") } else { s });
    }
    parts.join("·")
}

// ---------- inequalities ----------

/// Solution of the inequality N/D ⋈ 0 on the real line: intervals with exact endpoints where possible.
pub fn solve_ineq(rf: &RatFn, op: &Op) -> Result<(String, Vec<Check>), String> {
    let mut crit: Vec<(f64, String, bool)> = Vec::new(); // (value, notation, whether from the numerator)
    let mut collect = |p: &Poly, from_num: bool| -> Result<(), String> {
        if p.deg().unwrap_or(0) == 0 {
            return Ok(());
        }
        for (r, _) in solve_poly(p)?.roots {
            if r.is_real() {
                let v = r.f64().0;
                if !crit.iter().any(|(x, _, n)| (x - v).abs() < 1e-12 * v.abs().max(1.0) && *n == from_num) {
                    crit.push((v, match &r {
                        Root::Q(x) => show(x),
                        Root::Surd(s) => s.show(),
                        _ => format!("≈{}", fmt_f64(v)),
                    }, from_num));
                }
            }
        }
        Ok(())
    };
    collect(&rf.num, true)?;
    let mut all_den = rf.den.clone();
    for d in &rf.dens {
        all_den = all_den.mul(d);
    }
    collect(&all_den, false)?;
    crit.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let holds = |x: f64| -> Option<bool> {
        let (n, d) = (rf.num.eval_f64(x), rf.den.eval_f64(x));
        if d == 0.0 || rf.dens.iter().any(|p| p.eval_f64(x) == 0.0) {
            return None;
        }
        let v = n / d;
        Some(match op {
            Op::Lt => v < 0.0,
            Op::Le => v <= 0.0,
            Op::Gt => v > 0.0,
            Op::Ge => v >= 0.0,
            Op::Ne => v != 0.0,
            _ => v == 0.0,
        })
    };
    let mut pieces: Vec<String> = Vec::new();
    let mut checks = Vec::new();
    let bounds: Vec<f64> = crit.iter().map(|c| c.0).collect();
    let n = bounds.len();
    let mut inside = vec![false; n + 1];
    for i in 0..=n {
        let lo = if i == 0 { f64::NEG_INFINITY } else { bounds[i - 1] };
        let hi = if i == n { f64::INFINITY } else { bounds[i] };
        let t = match (lo.is_finite(), hi.is_finite()) {
            (false, false) => 0.0,
            (false, true) => hi - 1.0,
            (true, false) => lo + 1.0,
            _ => (lo + hi) / 2.0,
        };
        // the test point is rational, so we check exactly
        let tq = RBig::simplest_from_f64(t).unwrap_or_else(|| q(0));
        let exact = {
            let d = rf.den.eval(&tq);
            if d.is_zero() { None } else { Some(rf.num.eval(&tq) / d) }
        };
        inside[i] = holds(t).unwrap_or(false);
        if let Some(v) = &exact {
            let ok_exact = match op {
                Op::Lt => *v < RBig::ZERO,
                Op::Le => *v <= RBig::ZERO,
                Op::Gt => *v > RBig::ZERO,
                Op::Ge => *v >= RBig::ZERO,
                _ => v.is_zero(),
            };
            checks.push(Check::new(&format!("sign at test point {}", show(&tq)), ok_exact == inside[i], "exact"));
        }
    }
    let at_crit = |j: usize| holds(bounds[j]) == Some(true);
    let mut i = 0;
    while i <= n {
        if inside[i] {
            // merge adjacent intervals across critical points that also satisfy
            let st = i;
            while i < n && inside[i + 1] && at_crit(i) {
                i += 1;
            }
            let l = if st == 0 { "(-∞".to_string() } else { format!("{}{}", if at_crit(st - 1) { "[" } else { "(" }, crit[st - 1].1) };
            let h = if i == n { "+∞)".to_string() } else { format!("{}{}", crit[i].1, if at_crit(i) { "]" } else { ")" }) };
            pieces.push(format!("{l}, {h}"));
        } else if i < n && at_crit(i) && !inside[i + 1] {
            pieces.push(format!("{{{}}}", crit[i].1));
        }
        i += 1;
    }
    let s = if pieces.is_empty() { "∅".to_string() } else { pieces.join(" ∪ ") };
    Ok((s, checks))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calc::parse;
    use crate::nt::Limits;

    fn rfn(s: &str) -> RatFn {
        let mut env = Env::new(Limits::default());
        let (a, b) = crate::calc::parse_equation(s).unwrap();
        ratfunc(&E::bin(Op::Sub, a, b), "x", &mut env).unwrap()
    }

    #[test]
    fn solve_quadratics_and_more() {
        let r = rfn("x^2 - 5x + 6 = 0");
        let s = solve_poly(&r.num).unwrap();
        assert_eq!(s.roots.len(), 2);
        assert!(s.checks.iter().all(|c| c.ok));
        let r = rfn("x^2 - 2x - 1 = 0");
        let s = solve_poly(&r.num).unwrap();
        assert!(s.checks.iter().all(|c| c.ok), "{:?}", s.checks);
        assert!(matches!(&s.roots[0].0, Root::Surd(x) if x.d == IBig::from(2)));
        let r = rfn("x^2 + x + 1 = 0");
        let s = solve_poly(&r.num).unwrap();
        assert!(matches!(&s.roots[0].0, Root::Surd(x) if x.d == IBig::from(-3)));
        assert!(s.checks.iter().all(|c| c.ok));
        let r = rfn("(x-1)^3*(x+2) = 0");
        let s = solve_poly(&r.num).unwrap();
        assert!(s.roots.iter().any(|(r, m)| matches!(r, Root::Q(v) if *v == q(1)) && *m == 3));
        let r = rfn("x^3 - 2 = 0");
        let s = solve_poly(&r.num).unwrap();
        assert!(s.checks.iter().all(|c| c.ok), "{:?}", s.checks);
        let r = rfn("6x^3 - 11x^2 + 6x - 1 = 0");
        let s = solve_poly(&r.num).unwrap();
        assert_eq!(s.roots.iter().filter(|(r, _)| matches!(r, Root::Q(_))).count(), 3);
    }

    #[test]
    fn rational_equation_excludes_holes() {
        // (x^2-1)/(x-1) = 2 → x+1 = 2 → x = 1, but x = 1 is outside the domain
        let r = rfn("(x^2-1)/(x-1) = 2");
        let s = solve_poly(&r.num).unwrap();
        assert!(s.roots.iter().any(|(r, _)| matches!(r, Root::Q(v) if *v == q(1))), "root 1 found in the numerator");
        assert!(r.dens.iter().any(|d| d.eval(&q(1)).is_zero()), "denominator x − 1 recorded; the plan will reject it");
    }

    #[test]
    fn factor_and_ineq() {
        let mut env = Env::new(Limits::default());
        let rf = ratfunc(&parse("x^4 - 5x^2 + 4").unwrap(), "x", &mut env).unwrap();
        let (l, fs, c) = factor_q(&rf.num);
        assert!(c.ok);
        assert_eq!(fs.len(), 4);
        assert_eq!(l, q(1));
        let rf = ratfunc(&parse("x^2 - 4").unwrap(), "x", &mut env).unwrap();
        let (s, cs) = solve_ineq(&rf, &Op::Lt).unwrap();
        assert_eq!(s, "(-2, 2)");
        assert!(cs.iter().all(|c| c.ok));
        let (s, _) = solve_ineq(&rf, &Op::Ge).unwrap();
        assert_eq!(s, "(-∞, -2] ∪ [2, +∞)");
    }
}
