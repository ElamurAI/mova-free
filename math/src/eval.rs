//! Numeric evaluation of a tree over quantities: exact (rationals) where possible, approximate (f64,
//! marked "≈") only for transcendental operations. Every operation records a step (rule, formula,
//! substituted values, result) and, where possible, an independent check — that check is the gate.

use std::cmp::Ordering;

use crate::err::{Error, R, TempMisuse};
use crate::expr::{Const, Expr, Func, ProbOp, Target};
use crate::rat::Rat;
use crate::units::{Dim, Mono, Scale, Unit, factor};

/// Number: exact rational or approximate.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Num {
    Exact(Rat),
    Approx(f64),
}

impl Num {
    pub fn int(n: i128) -> Num {
        Num::Exact(Rat::int(n))
    }
    pub fn to_f64(self) -> f64 {
        match self {
            Num::Exact(r) => r.to_f64(),
            Num::Approx(x) => x,
        }
    }
    pub fn is_exact(self) -> bool {
        matches!(self, Num::Exact(_))
    }
    pub fn exact(self) -> Option<Rat> {
        match self {
            Num::Exact(r) => Some(r),
            Num::Approx(_) => None,
        }
    }
    fn bin(self, o: Num, fr: fn(Rat, Rat) -> R<Rat>, ff: fn(f64, f64) -> f64) -> R<Num> {
        match (self, o) {
            (Num::Exact(a), Num::Exact(b)) => Ok(Num::Exact(fr(a, b)?)),
            _ => Ok(Num::Approx(ff(self.to_f64(), o.to_f64()))),
        }
    }
    pub fn add(self, o: Num) -> R<Num> {
        self.bin(o, Rat::add, |a, b| a + b)
    }
    pub fn sub(self, o: Num) -> R<Num> {
        self.bin(o, Rat::sub, |a, b| a - b)
    }
    pub fn mul(self, o: Num) -> R<Num> {
        self.bin(o, Rat::mul, |a, b| a * b)
    }
    pub fn div(self, o: Num) -> R<Num> {
        if o.to_f64() == 0.0 {
            return Err(Error::DivZero);
        }
        self.bin(o, Rat::div, |a, b| a / b)
    }
    pub fn neg(self) -> R<Num> {
        match self {
            Num::Exact(r) => Ok(Num::Exact(r.neg()?)),
            Num::Approx(x) => Ok(Num::Approx(-x)),
        }
    }
    pub fn mul_rat(self, r: Rat) -> R<Num> {
        self.mul(Num::Exact(r))
    }
    pub fn cmp(self, o: Num) -> Ordering {
        match (self, o) {
            (Num::Exact(a), Num::Exact(b)) => a.cmp(&b),
            _ => self.to_f64().partial_cmp(&o.to_f64()).unwrap_or(Ordering::Equal),
        }
    }
}

/// Quantity: a number in units `u`; an absolute temperature is a `point` with a scale (then `u` is empty).
#[derive(Clone, Debug, PartialEq)]
pub struct Q {
    pub n: Num,
    pub u: Mono,
    pub point: Option<Scale>,
}

impl Q {
    pub fn plain(n: Num) -> Q {
        Q { n, u: Mono::one(), point: None }
    }
    pub fn with(n: Num, u: Mono) -> Q {
        Q { n, u, point: None }
    }
    pub fn temp(n: Num, s: Scale) -> Q {
        Q { n, u: Mono::one(), point: Some(s) }
    }
    pub fn dim(&self) -> Dim {
        if self.point.is_some() { Dim::TH } else { self.u.dim() }
    }
    pub fn is_plain(&self) -> bool {
        self.point.is_none() && self.u.is_one()
    }
    /// Dimensionless: a number or a percentage.
    pub fn is_ratio(&self) -> bool {
        self.point.is_none() && self.u.dim().is_none()
    }
    /// Value in SI (absolute temperature in kelvins).
    pub fn base(&self) -> R<Num> {
        match self.point {
            Some(s) => match self.n {
                Num::Exact(r) => Ok(Num::Exact(s.to_kelvin(r)?)),
                Num::Approx(x) => Ok(Num::Approx(s.to_kelvin_f64(x))),
            },
            None => self.n.mul_rat(self.u.scale()?),
        }
    }
    /// Unit for printing.
    pub fn unit_str(&self) -> String {
        match self.point {
            Some(s) => s.sym().to_string(),
            None if self.u.is_one() => String::new(),
            None => self.u.to_string(),
        }
    }
}

/// Number for humans: exact terminating — as is; non-terminating — "≈" with rounding and the exact fraction.
pub fn fmt_num(n: Num) -> String {
    match n {
        Num::Exact(r) => match r.exact_decimal(8) {
            Some(s) => s,
            None => format!("≈ {} (exactly {r})", r.round_decimal(places_for(r.to_f64()))),
        },
        Num::Approx(x) => format!("≈ {}", fmt_f64(x)),
    }
}

/// Short form: for substituted values in steps.
pub fn fmt_short(n: Num) -> String {
    match n {
        Num::Exact(r) => match r.exact_decimal(8) {
            Some(s) => s,
            None => r.to_string(),
        },
        Num::Approx(x) => format!("≈{}", fmt_f64(x)),
    }
}

fn places_for(x: f64) -> u32 {
    // at least 6 significant digits, at least 4 decimal places, at most 12
    let mag = if x == 0.0 { 0 } else { x.abs().log10().floor() as i32 };
    (5 - mag).clamp(4, 12) as u32
}

fn fmt_f64(x: f64) -> String {
    let s = format!("{:.*}", places_for(x) as usize + 2, x);
    if s.contains('.') { s.trim_end_matches('0').trim_end_matches('.').to_string() } else { s }
}

pub fn fmt_q(q: &Q) -> String {
    let u = q.unit_str();
    let n = fmt_num(q.n);
    if u.is_empty() {
        n
    } else if u == "%" {
        match n.split_once(" (exactly ") {
            Some((a, b)) => format!("{a} % (exactly {} %)", b.trim_end_matches(')')),
            None => format!("{n} %"),
        }
    } else {
        match n.split_once(" (exactly ") {
            Some((a, b)) => format!("{a} {u} (exactly {} {u})", b.trim_end_matches(')')),
            None => format!("{n} {u}"),
        }
    }
}

fn short_q(q: &Q) -> String {
    let u = q.unit_str();
    let n = fmt_short(q.n);
    if u.is_empty() {
        n
    } else if u == "%" {
        format!("{n}%")
    } else {
        format!("{n} {u}")
    }
}

/// Step: rule, formula, substituted values, result.
#[derive(Clone, Debug)]
pub struct Step {
    pub rule: &'static str,
    pub formula: String,
    pub subst: String,
    pub result: String,
}

/// Independent check of the result.
#[derive(Clone, Debug)]
pub struct Check {
    pub what: &'static str,
    pub ok: bool,
    pub detail: String,
}

#[derive(Default, Debug)]
pub struct Ctx {
    pub steps: Vec<Step>,
    pub checks: Vec<Check>,
    /// Assumptions, stated out loud.
    pub notes: Vec<String>,
    /// Variable values.
    pub vars: Vec<(crate::expr::Var, Num)>,
}

impl Ctx {
    fn step(&mut self, rule: &'static str, formula: impl Into<String>, subst: impl Into<String>, result: &Q) {
        self.steps.push(Step { rule, formula: formula.into(), subst: subst.into(), result: fmt_q(result) });
    }
    fn check(&mut self, what: &'static str, ok: bool, detail: impl Into<String>) {
        self.checks.push(Check { what, ok, detail: detail.into() });
    }
    fn note(&mut self, s: &str) {
        if !self.notes.iter().any(|n| n == s) {
            self.notes.push(s.to_string());
        }
    }
}

// ---------- operations on quantities ----------

fn temp_delta_to(q: &Q, s: Scale, op: &'static str) -> R<Num> {
    // convert the difference q (a Δ unit) into the Δ of scale s
    match q.u.temp_delta() {
        Some(_) => q.n.mul_rat(factor(&q.u, &Mono::of(s.delta()), op)?),
        None if q.is_plain() => Err(Error::Temp(TempMisuse::MixedWithPlain)),
        None => Err(Error::Dim { op, a: Dim::TH, b: q.dim() }),
    }
}

pub fn q_add(a: &Q, b: &Q, op: &'static str) -> R<Q> {
    match (a.point, b.point) {
        (Some(_), Some(_)) => Err(Error::Temp(TempMisuse::AddPoints)),
        (Some(s), None) => Ok(Q::temp(a.n.add(temp_delta_to(b, s, op)?)?, s)),
        (None, Some(s)) => Ok(Q::temp(b.n.add(temp_delta_to(a, s, op)?)?, s)),
        (None, None) => {
            if a.dim() != b.dim() {
                return Err(Error::Dim { op, a: a.dim(), b: b.dim() });
            }
            let f = factor(&b.u, &a.u, op)?;
            Ok(Q::with(a.n.add(b.n.mul_rat(f)?)?, a.u.clone()))
        }
    }
}

pub fn q_sub(a: &Q, b: &Q) -> R<Q> {
    let op = "subtraction";
    match (a.point, b.point) {
        (Some(sa), Some(sb)) => {
            // point − point = difference, in the Δ of the first scale
            let ka = Q::temp(a.n, sa).base()?;
            let kb = Q::temp(b.n, sb).base()?;
            let d = ka.sub(kb)?.div(Num::Exact(sa.delta().scale()))?;
            Ok(Q::with(d, Mono::of(sa.delta())))
        }
        (Some(s), None) => Ok(Q::temp(a.n.sub(temp_delta_to(b, s, op)?)?, s)),
        (None, Some(_)) => {
            if a.u.temp_delta().is_some() {
                Err(Error::Temp(TempMisuse::DeltaMinusPoint))
            } else if a.is_plain() {
                Err(Error::Temp(TempMisuse::MixedWithPlain))
            } else {
                Err(Error::Dim { op, a: a.dim(), b: Dim::TH })
            }
        }
        (None, None) => {
            if a.dim() != b.dim() {
                return Err(Error::Dim { op, a: a.dim(), b: b.dim() });
            }
            let f = factor(&b.u, &a.u, op)?;
            Ok(Q::with(a.n.sub(b.n.mul_rat(f)?)?, a.u.clone()))
        }
    }
}

/// After multiplication: a dimensionless product of different units (km/m) and a percentage in a mixed product
/// collapse into a number.
fn normalize(q: Q) -> R<Q> {
    if q.u.is_one() || q.u.is_pct() {
        return Ok(q);
    }
    if q.u.dim().is_none() {
        let s = q.u.scale()?;
        return Ok(Q::plain(q.n.mul_rat(s)?));
    }
    if q.u.0.iter().any(|x| x.0 == Unit::Pct) {
        let e = q.u.0.iter().find(|x| x.0 == Unit::Pct).unwrap().1;
        let n = q.n.mul_rat(Unit::Pct.scale().pow(e as i64)?)?;
        let u = Mono(q.u.0.iter().filter(|x| x.0 != Unit::Pct).copied().collect());
        return Ok(Q::with(n, u));
    }
    Ok(q)
}

pub fn q_mul(a: &Q, b: &Q) -> R<Q> {
    if a.point.is_some() || b.point.is_some() {
        return Err(Error::Temp(TempMisuse::ScalePoint));
    }
    normalize(Q::with(a.n.mul(b.n)?, a.u.mul(&b.u)))
}

pub fn q_div(a: &Q, b: &Q) -> R<Q> {
    if a.point.is_some() || b.point.is_some() {
        return Err(Error::Temp(TempMisuse::ScalePoint));
    }
    normalize(Q::with(a.n.div(b.n)?, a.u.mul(&b.u.inv())))
}

fn q_pow(a: &Q, e: &Q) -> R<Q> {
    if a.point.is_some() {
        return Err(Error::Temp(TempMisuse::NegPoint));
    }
    if !e.is_plain() {
        return Err(Error::Dim { op: "power", a: Dim::NONE, b: e.dim() });
    }
    match (a.n, e.n) {
        (Num::Exact(x), Num::Exact(k)) if k.is_int() => {
            let k = i64::try_from(k.num()).map_err(|_| Error::Overflow("power"))?;
            let ku = i8::try_from(k).map_err(|_| Error::Overflow("unit power"));
            let u = if a.u.is_one() { Mono::one() } else { a.u.pow(ku?) };
            Ok(Q::with(Num::Exact(x.pow(k)?), u))
        }
        (Num::Exact(x), Num::Exact(k)) if a.u.is_one() => {
            // rational exponent: exact root, if there is one
            let root = u32::try_from(k.den()).ok().and_then(|d| x.root(d));
            match root {
                Some(r) => {
                    let p = i64::try_from(k.num()).map_err(|_| Error::Overflow("power"))?;
                    Ok(Q::plain(Num::Exact(r.pow(p)?)))
                }
                None if x.is_neg() => Err(Error::Domain("fractional power of a negative number")),
                None => Ok(Q::plain(Num::Approx(x.to_f64().powf(k.to_f64())))),
            }
        }
        _ if a.u.is_one() => {
            let v = a.n.to_f64().powf(e.n.to_f64());
            if v.is_nan() { Err(Error::Domain("power")) } else { Ok(Q::plain(Num::Approx(v))) }
        }
        _ => Err(Error::Dim { op: "fractional power of a quantity with units", a: a.dim(), b: Dim::NONE }),
    }
}

/// Bring a list to the common unit of the first element; points to the scale of the first one.
fn same_kind(xs: &[Q], op: &'static str) -> R<()> {
    let first = &xs[0];
    for x in &xs[1..] {
        match (first.point.is_some(), x.point.is_some()) {
            (true, true) => {}
            (false, false) => {
                if first.dim() != x.dim() {
                    return Err(Error::Dim { op, a: first.dim(), b: x.dim() });
                }
            }
            _ => {
                let other = if first.point.is_some() { x } else { first };
                return Err(if other.is_plain() { Error::Temp(TempMisuse::MixedWithPlain) } else { Error::Dim { op, a: first.dim(), b: x.dim() } });
            }
        }
    }
    Ok(())
}

/// Value of x in the units (or scale) of the first element.
fn in_unit_of(x: &Q, first: &Q) -> R<Num> {
    match first.point {
        Some(s) => {
            let k = x.base()?;
            match k {
                Num::Exact(r) => Ok(Num::Exact(s.from_kelvin(r)?)),
                Num::Approx(v) => Ok(Num::Approx(s.from_kelvin_f64(v))),
            }
        }
        None => x.n.mul_rat(factor(&x.u, &first.u, "conversion to a common unit")?),
    }
}

fn rebuild(n: Num, like: &Q) -> Q {
    Q { n, u: like.u.clone(), point: like.point }
}

fn convert(q: &Q, t: &Target, ctx: &mut Ctx) -> R<Q> {
    let op = "conversion";
    match t {
        Target::Scale(s) => match q.point {
            Some(s0) => {
                let k = q.base()?;
                let n = match k {
                    Num::Exact(r) => Num::Exact(s.from_kelvin(r)?),
                    Num::Approx(v) => Num::Approx(s.from_kelvin_f64(v)),
                };
                let out = Q::temp(n, *s);
                let formula = temp_formula(s0, *s);
                ctx.step("temperature conversion (affine scale)", formula, format!("{} → {}", short_q(q), s.sym()), &out);
                // check: the reverse conversion gives back the original number
                if let (Num::Exact(r), Num::Exact(orig)) = (n, q.n) {
                    let back = s0.from_kelvin(s.to_kelvin(r)?)?;
                    ctx.check("reverse conversion", back == orig, format!("{} {} → {} = {}", fmt_short(n), s.sym(), s0.sym(), fmt_short(Num::Exact(back))));
                }
                Ok(out)
            }
            None if q.u.temp_delta().is_some() => {
                ctx.note("this is a temperature difference, not an absolute temperature: converted by the factor only, without shifting zero");
                convert(q, &Target::Mono(Mono::of(s.delta())), ctx)
            }
            None if q.is_plain() => Err(Error::Temp(TempMisuse::MixedWithPlain)),
            None => Err(Error::Dim { op, a: q.dim(), b: Dim::TH }),
        },
        Target::Mono(m) => {
            if q.point.is_some() {
                return Err(if m.dim() == Dim::TH { Error::Temp(TempMisuse::KindMismatch) } else { Error::Dim { op, a: Dim::TH, b: m.dim() } });
            }
            let f = factor(&q.u, m, op)?;
            let out = Q::with(q.n.mul_rat(f)?, m.clone());
            let from = if q.u.is_one() { "1".to_string() } else { q.u.to_string() };
            ctx.step("conversion", format!("x {from} · {} ({m} per 1 {from})", fmt_short(Num::Exact(f))), format!("{} · {}", fmt_short(q.n), fmt_short(Num::Exact(f))), &out);
            if let (Num::Exact(r), Num::Exact(orig)) = (out.n, q.n) {
                let back = r.mul(factor(m, &q.u, op)?)?;
                ctx.check("reverse conversion", back == orig, format!("{} {m} → {} = {}", fmt_short(out.n), from, fmt_short(Num::Exact(back))));
            }
            Ok(out)
        }
    }
}

fn temp_formula(a: Scale, b: Scale) -> &'static str {
    match (a, b) {
        (Scale::F, Scale::C) => "°C = (°F − 32) · 5/9",
        (Scale::C, Scale::F) => "°F = °C · 9/5 + 32",
        (Scale::C, Scale::K) => "K = °C + 273.15",
        (Scale::K, Scale::C) => "°C = K − 273.15",
        (Scale::F, Scale::K) => "K = (°F + 459.67) · 5/9",
        (Scale::K, Scale::F) => "°F = K · 9/5 − 459.67",
        _ => "same scale",
    }
}

fn prob_value(q: &Q) -> R<Num> {
    if !q.is_ratio() {
        return Err(Error::Dim { op: "probability", a: Dim::NONE, b: q.dim() });
    }
    let p = q.base()?;
    if p.cmp(Num::int(0)) == Ordering::Less || p.cmp(Num::int(1)) == Ordering::Greater {
        return Err(Error::Domain("probability outside [0, 1]"));
    }
    Ok(p)
}

fn as_display(p: Num, pct: bool) -> R<Q> {
    if pct { Ok(Q::with(p.mul(Num::int(100))?, Mono::of(Unit::Pct))) } else { Ok(Q::plain(p)) }
}

fn prob(op: ProbOp, gs: &[(Q, Q)], ctx: &mut Ctx) -> R<Q> {
    let pct = gs.iter().any(|g| g.0.u.is_pct());
    let mut ps = Vec::new();
    for (p, k) in gs {
        let pv = prob_value(p)?;
        let kv = match k.n {
            Num::Exact(r) if k.is_plain() && r.is_int() && !r.is_neg() => r.num(),
            _ => return Err(Error::Domain("the number of repetitions must be a non-negative integer")),
        };
        ps.push((pv, kv));
    }
    ctx.note("I treat the events (days) as independent — otherwise the formula does not hold");
    let total: i128 = ps.iter().map(|x| x.1).sum();
    let mut none = Num::int(1);
    let mut all = Num::int(1);
    let mut none_parts = Vec::new();
    let mut all_parts = Vec::new();
    for (i, (p, k)) in ps.iter().enumerate() {
        let q = Num::int(1).sub(*p)?;
        if op != ProbOp::AllOf {
            ctx.step("complement", "1 − p", format!("1 − {}", short_q(&gs[i].0)), &as_display(q, pct)?);
        }
        let kk = i64::try_from(*k).map_err(|_| Error::Overflow("power"))?;
        let qn = match q {
            Num::Exact(r) => Num::Exact(r.pow(kk)?),
            Num::Approx(v) => Num::Approx(v.powi(kk as i32)),
        };
        let pn = match p {
            Num::Exact(r) => Num::Exact(r.pow(kk)?),
            Num::Approx(v) => Num::Approx(v.powi(kk as i32)),
        };
        none = none.mul(qn)?;
        all = all.mul(pn)?;
        let pw = |x: Num| if *k == 1 { fmt_short(x) } else { format!("{}^{k}", fmt_short(x)) };
        none_parts.push(pw(q));
        all_parts.push(pw(*p));
    }
    let res = match op {
        ProbOp::AllOf => {
            let out = as_display(all, pct)?;
            ctx.step("every time (independent events)", "∏ pᵢ", all_parts.join(" · "), &out);
            all
        }
        _ => {
            let n_out = as_display(none, pct)?;
            ctx.step("never (independent events)", "∏ (1 − pᵢ)", none_parts.join(" · "), &n_out);
            if op == ProbOp::AtLeastOnce {
                let al = Num::int(1).sub(none)?;
                let out = as_display(al, pct)?;
                ctx.step("at least once", "1 − ∏ (1 − pᵢ)", format!("1 − {}", fmt_short(none)), &out);
                al
            } else {
                none
            }
        }
    };
    // independent check: enumerate all 2^N outcomes (exactly), if N is small
    if total <= 16 && ps.iter().all(|x| x.0.is_exact()) {
        let mut flat = Vec::new();
        for (p, k) in &ps {
            for _ in 0..*k {
                flat.push(p.exact().unwrap());
            }
        }
        let n = flat.len();
        let (mut s_none, mut s_all, mut s_total) = (Rat::ZERO, Rat::ZERO, Rat::ZERO);
        for mask in 0u32..(1u32 << n) {
            let mut pr = Rat::ONE;
            for (i, p) in flat.iter().enumerate() {
                pr = pr.mul(if mask >> i & 1 == 1 { *p } else { Rat::ONE.sub(*p)? })?;
            }
            s_total = s_total.add(pr)?;
            if mask == 0 {
                s_none = s_none.add(pr)?;
            }
            if mask == (1u32 << n) - 1 {
                s_all = s_all.add(pr)?;
            }
        }
        let want = match op {
            ProbOp::AtLeastOnce => s_total.sub(s_none)?,
            ProbOp::NoneOf => s_none,
            ProbOp::AllOf => s_all,
        };
        ctx.check(
            "enumeration of all outcomes",
            Num::Exact(want) == res && s_total == Rat::ONE,
            format!("2^{n} = {} outcomes, sum of probabilities {}, target {}", 1u64 << n, s_total, fmt_short(Num::Exact(want))),
        );
    }
    as_display(res, pct)
}

fn elementary(f: Func, x: &Q) -> R<Q> {
    if f == Func::Abs {
        return Ok(rebuild(if x.n.cmp(Num::int(0)) == Ordering::Less { x.n.neg()? } else { x.n }, x));
    }
    if f == Func::Sqrt && x.point.is_none() && !x.u.is_one() {
        if x.u.0.iter().all(|u| u.1 % 2 == 0) {
            let half = Mono(x.u.0.iter().map(|&(u, e)| (u, e / 2)).collect());
            let inner = elementary(f, &Q::plain(x.n))?;
            return Ok(Q::with(inner.n, half));
        }
        return Err(Error::Dim { op: "root", a: x.dim(), b: Dim::NONE });
    }
    if !x.is_plain() {
        return Err(Error::Dim { op: f.name(), a: x.dim(), b: Dim::NONE });
    }
    let v = x.n.to_f64();
    let exact = x.n.exact();
    let out = match f {
        Func::Sqrt => {
            if x.n.cmp(Num::int(0)) == Ordering::Less {
                return Err(Error::Domain("square root of a negative number"));
            }
            match exact.and_then(|r| r.root(2)) {
                Some(r) => Num::Exact(r),
                None => Num::Approx(v.sqrt()),
            }
        }
        Func::Ln => {
            if v <= 0.0 {
                return Err(Error::Domain("logarithm of a non-positive number"));
            }
            if exact == Some(Rat::ONE) { Num::int(0) } else { Num::Approx(v.ln()) }
        }
        Func::Exp => {
            if exact == Some(Rat::ZERO) { Num::int(1) } else { Num::Approx(v.exp()) }
        }
        Func::Sin => {
            if exact == Some(Rat::ZERO) { Num::int(0) } else { Num::Approx(v.sin()) }
        }
        Func::Cos => {
            if exact == Some(Rat::ZERO) { Num::int(1) } else { Num::Approx(v.cos()) }
        }
        Func::Tan => {
            if exact == Some(Rat::ZERO) { Num::int(0) } else { Num::Approx(v.tan()) }
        }
        _ => unreachable!(),
    };
    Ok(Q::plain(out))
}

fn aggregate(f: Func, xs: &[Q], ctx: &mut Ctx) -> R<Q> {
    if xs.is_empty() {
        return Err(Error::NotUnderstood(format!("{}: empty list", f.name())));
    }
    let op = f.name();
    same_kind(xs, op)?;
    let first = &xs[0];
    let vals: Vec<Num> = xs.iter().map(|x| in_unit_of(x, first)).collect::<R<_>>()?;
    let n = xs.len();
    let listed = vals.iter().map(|v| fmt_short(*v)).collect::<Vec<_>>();
    match f {
        Func::Sum => {
            if first.point.is_some() {
                return Err(Error::Temp(TempMisuse::SumPoints));
            }
            let s = vals.iter().try_fold(Num::int(0), |a, b| a.add(*b))?;
            let out = rebuild(s, first);
            ctx.step("sum", "x₁ + … + xₙ", listed.join(" + "), &out);
            Ok(out)
        }
        Func::Mean => {
            let s = vals.iter().try_fold(Num::int(0), |a, b| a.add(*b))?;
            let m = s.div(Num::int(n as i128))?;
            let out = rebuild(m, first);
            let note = if first.point.is_some() { " (mean of absolute temperatures — an affine combination, valid)" } else { "" };
            ctx.step("mean", "(x₁ + … + xₙ) / n", format!("({}) / {n} = {} / {n}{note}", listed.join(" + "), fmt_short(s)), &out);
            // check: deviations from the mean sum to zero; min ≤ mean ≤ max
            let dev = vals.iter().try_fold(Num::int(0), |a, b| a.add(b.sub(m)?))?;
            let lo = vals.iter().any(|v| v.cmp(m) != Ordering::Greater);
            let hi = vals.iter().any(|v| v.cmp(m) != Ordering::Less);
            let ok = dev.to_f64().abs() <= 1e-9 * (1.0 + s.to_f64().abs()) && (!dev.is_exact() || dev == Num::int(0)) && lo && hi;
            ctx.check("sum of deviations from the mean = 0, min ≤ mean ≤ max", ok, format!("Σ(xᵢ − x̄) = {}", fmt_short(dev)));
            Ok(out)
        }
        Func::Median | Func::Min | Func::Max | Func::Range => {
            let mut sorted = vals.clone();
            sorted.sort_by(|a, b| a.cmp(*b));
            let sorted_s = sorted.iter().map(|v| fmt_short(*v)).collect::<Vec<_>>().join(", ");
            match f {
                Func::Median => {
                    let m = if n % 2 == 1 { sorted[n / 2] } else { sorted[n / 2 - 1].add(sorted[n / 2])?.div(Num::int(2))? };
                    let out = rebuild(m, first);
                    let how = if n % 2 == 1 { format!("sorted: {sorted_s}; middle of {n} — #{}", n / 2 + 1) } else { format!("sorted: {sorted_s}; even count — mean of the two middle ones ({} + {}) / 2", fmt_short(sorted[n / 2 - 1]), fmt_short(sorted[n / 2])) };
                    ctx.step("median", "middle element of the sorted list", how, &out);
                    let le = vals.iter().filter(|v| v.cmp(m) != Ordering::Greater).count();
                    let ge = vals.iter().filter(|v| v.cmp(m) != Ordering::Less).count();
                    ctx.check("at least half ≤ median and at least half ≥", 2 * le >= n && 2 * ge >= n, format!("≤: {le} of {n}, ≥: {ge} of {n}"));
                    Ok(out)
                }
                Func::Min | Func::Max => {
                    let m = if f == Func::Min { sorted[0] } else { sorted[n - 1] };
                    let out = rebuild(m, first);
                    ctx.step(if f == Func::Min { "minimum" } else { "maximum" }, "extreme element of the sorted list", format!("sorted: {sorted_s}"), &out);
                    let ok = vals.contains(&m) && vals.iter().all(|v| if f == Func::Min { v.cmp(m) != Ordering::Less } else { v.cmp(m) != Ordering::Greater });
                    ctx.check("a list element and not exceeded by any", ok, String::new());
                    Ok(out)
                }
                _ => {
                    let (lo, hi) = (rebuild(sorted[0], first), rebuild(sorted[n - 1], first));
                    let out = q_sub(&hi, &lo)?;
                    ctx.step("range", "max − min", format!("{} − {}", short_q(&hi), short_q(&lo)), &out);
                    let back = q_add(&lo, &out, "check")?;
                    ctx.check("min + range = max", back.n.cmp(hi.n) == Ordering::Equal, format!("{} + {} = {}", short_q(&lo), short_q(&out), short_q(&back)));
                    Ok(out)
                }
            }
        }
        _ => unreachable!(),
    }
}

fn pct_fn(f: Func, a: &Q, b: &Q, ctx: &mut Ctx) -> R<Q> {
    match f {
        Func::PctOf => {
            if !a.is_ratio() {
                return Err(Error::Dim { op: "percent of", a: a.dim(), b: Dim::NONE });
            }
            if b.point.is_some() {
                return Err(Error::Temp(TempMisuse::ScalePoint));
            }
            let frac = a.base()?;
            let out = Q::with(b.n.mul(frac)?, b.u.clone());
            ctx.step("percent of a number", "p% · x = p/100 · x", format!("{} · {} = {} · {}", short_q(a), short_q(b), fmt_short(frac), short_q(b)), &out);
            if !b.n.to_f64().eq(&0.0) {
                let back = out.n.div(b.n)?;
                ctx.check("inverse operation: result / x = p", back.cmp(frac) == Ordering::Equal, format!("{} / {} = {}", fmt_short(out.n), fmt_short(b.n), fmt_short(back)));
            }
            Ok(out)
        }
        Func::PctChange | Func::PctRatio => {
            if a.point.is_some() || b.point.is_some() {
                return Err(Error::Temp(TempMisuse::ScalePoint));
            }
            let bv = in_unit_of(b, a)?;
            let (ratio, formula, subst) = if f == Func::PctChange {
                let r = bv.sub(a.n)?.div(a.n)?;
                (r, "(new − old) / old · 100%", format!("({} − {}) / {}", fmt_short(bv), fmt_short(a.n), fmt_short(a.n)))
            } else {
                // share of a in b: a / b
                let r = a.n.div(bv)?;
                (r, "part / whole · 100%", format!("{} / {}", fmt_short(a.n), fmt_short(bv)))
            };
            let out = as_display(ratio, true)?;
            ctx.step(if f == Func::PctChange { "percent change" } else { "percentage share" }, formula, format!("{subst} = {}", fmt_short(ratio)), &out);
            if f == Func::PctChange {
                let back = a.n.mul(Num::int(1).add(ratio)?)?;
                ctx.check("old · (1 + change) = new", back.cmp(bv) == Ordering::Equal, format!("{} · (1 + {}) = {}", fmt_short(a.n), fmt_short(ratio), fmt_short(back)));
            } else {
                let back = ratio.mul(bv)?;
                ctx.check("share · whole = part", back.cmp(a.n) == Ordering::Equal, format!("{} · {} = {}", fmt_short(ratio), fmt_short(bv), fmt_short(back)));
            }
            Ok(out)
        }
        Func::AbsDiff => {
            let d = q_sub(a, b)?;
            let out = if d.n.cmp(Num::int(0)) == Ordering::Less { rebuild(d.n.neg()?, &d) } else { d };
            ctx.step("difference", "|a − b|", format!("|{} − {}|", short_q(a), short_q(b)), &out);
            if out.u.temp_delta().is_some() {
                ctx.note("a difference of absolute temperatures is a difference (Δ), not a temperature");
            }
            let hi = if a.base()?.cmp(b.base()?) == Ordering::Less { b } else { a };
            let lo = if std::ptr::eq(hi, a) { b } else { a };
            let back = q_add(lo, &out, "check")?;
            let ok = back.base()?.cmp(hi.base()?) == Ordering::Equal || !back.n.is_exact();
            ctx.check("smaller + difference = larger", ok, format!("{} + {} = {}", short_q(lo), short_q(&out), short_q(&back)));
            Ok(out)
        }
        _ => unreachable!(),
    }
}

/// Evaluate a node with steps and checks.
pub fn eval(e: &Expr, ctx: &mut Ctx) -> R<Q> {
    match e {
        Expr::Num(r) => Ok(Q::plain(Num::Exact(*r))),
        Expr::Var(v) => ctx.vars.iter().find(|x| x.0 == *v).map(|x| Q::plain(x.1)).ok_or(Error::Domain("variable without a value")),
        Expr::Const(Const::Pi) => Ok(Q::plain(Num::Approx(std::f64::consts::PI))),
        Expr::Const(Const::E) => Ok(Q::plain(Num::Approx(std::f64::consts::E))),
        Expr::Qty(x, m) => {
            let q = eval(x, ctx)?;
            q_mul(&q, &Q::with(Num::int(1), m.clone()))
        }
        Expr::Temp(x, s) => {
            let q = eval(x, ctx)?;
            if !q.is_plain() {
                return Err(Error::Dim { op: "temperature", a: q.dim(), b: Dim::NONE });
            }
            Ok(Q::temp(q.n, *s))
        }
        Expr::Neg(x) => {
            let q = eval(x, ctx)?;
            if q.point.is_some() {
                return Err(Error::Temp(TempMisuse::NegPoint));
            }
            Ok(rebuild(q.n.neg()?, &q))
        }
        Expr::Add(xs) => {
            let qs: Vec<Q> = xs.iter().map(|x| eval(x, ctx)).collect::<R<_>>()?;
            let mut acc = qs[0].clone();
            for q in &qs[1..] {
                acc = q_add(&acc, q, "addition")?;
            }
            ctx.step("addition", e.to_string(), qs.iter().map(short_q).collect::<Vec<_>>().join(" + "), &acc);
            let mut back = acc.clone();
            for q in qs[1..].iter().rev() {
                back = q_sub(&back, q)?;
            }
            if back.n.is_exact() {
                ctx.check("inverse operation: sum − addends = first", back.base()? == qs[0].base()?, format!("{}", short_q(&back)));
            }
            Ok(acc)
        }
        Expr::Sub(a, b) => {
            let (qa, qb) = (eval(a, ctx)?, eval(b, ctx)?);
            let out = q_sub(&qa, &qb)?;
            ctx.step("subtraction", e.to_string(), format!("{} − {}", short_q(&qa), short_q(&qb)), &out);
            if out.n.is_exact() {
                let back = q_add(&qb, &out, "check")?;
                ctx.check("inverse operation: difference + subtrahend = minuend", back.base()? == qa.base()?, short_q(&back));
            }
            Ok(out)
        }
        Expr::Mul(xs) => {
            let qs: Vec<Q> = xs.iter().map(|x| eval(x, ctx)).collect::<R<_>>()?;
            let mut acc = qs[0].clone();
            for q in &qs[1..] {
                acc = q_mul(&acc, q)?;
            }
            ctx.step("multiplication", e.to_string(), qs.iter().map(short_q).collect::<Vec<_>>().join(" · "), &acc);
            Ok(acc)
        }
        Expr::Div(a, b) => {
            let (qa, qb) = (eval(a, ctx)?, eval(b, ctx)?);
            let out = q_div(&qa, &qb)?;
            ctx.step("division", e.to_string(), format!("{} / {}", short_q(&qa), short_q(&qb)), &out);
            if out.n.is_exact() {
                let back = q_mul(&out, &qb)?;
                ctx.check("inverse operation: quotient · divisor = dividend", back.base()? == qa.base()?, short_q(&back));
            }
            Ok(out)
        }
        Expr::Pow(a, b) => {
            let (qa, qb) = (eval(a, ctx)?, eval(b, ctx)?);
            let out = q_pow(&qa, &qb)?;
            ctx.step("power", e.to_string(), format!("{}^{}", short_q(&qa), short_q(&qb)), &out);
            Ok(out)
        }
        Expr::Call(f, args) => {
            let qs: Vec<Q> = args.iter().map(|x| eval(x, ctx)).collect::<R<_>>()?;
            match f {
                f if f.is_elementary() => {
                    if qs.len() != 1 {
                        return Err(Error::NotUnderstood(format!("{} takes one argument", f.name())));
                    }
                    let out = elementary(*f, &qs[0])?;
                    ctx.step(f.name(), e.to_string(), format!("{}({})", f.name(), short_q(&qs[0])), &out);
                    if *f == Func::Sqrt && out.is_plain() {
                        let sq = out.n.mul(out.n)?;
                        let ok = if sq.is_exact() && qs[0].n.is_exact() { sq == qs[0].n } else { (sq.to_f64() - qs[0].n.to_f64()).abs() <= 1e-9 * (1.0 + qs[0].n.to_f64().abs()) };
                        ctx.check("root squared = radicand", ok, format!("{}² = {}", fmt_short(out.n), fmt_short(sq)));
                    }
                    Ok(out)
                }
                Func::Sum | Func::Mean | Func::Median | Func::Min | Func::Max | Func::Range => aggregate(*f, &qs, ctx),
                _ => {
                    if qs.len() != 2 {
                        return Err(Error::NotUnderstood(format!("{} takes two arguments", f.name())));
                    }
                    pct_fn(*f, &qs[0], &qs[1], ctx)
                }
            }
        }
        Expr::Convert(x, t) => {
            let q = eval(x, ctx)?;
            convert(&q, t, ctx)
        }
        Expr::Prob(op, gs) => {
            let qs: Vec<(Q, Q)> = gs.iter().map(|(p, k)| Ok((eval(p, ctx)?, eval(k, ctx)?))).collect::<R<_>>()?;
            prob(*op, &qs, ctx)
        }
    }
}

/// Answer: value with unit, steps, checks, assumptions.
#[derive(Debug)]
pub struct Answer {
    pub q: Q,
    pub steps: Vec<Step>,
    pub checks: Vec<Check>,
    pub notes: Vec<String>,
}

/// Full evaluation with the gate: the exact path, an independent f64 path (`fcheck`) and per-operation checks.
/// Any failed check is an error, not an answer.
pub fn answer(e: &Expr) -> R<Answer> {
    let mut ctx = Ctx::default();
    let q = eval(e, &mut ctx)?;
    gate(e, q, ctx)
}

/// Gate: the independent path and per-operation checks; separate from evaluation so that a negative control can
/// feed a deliberately wrong answer and see red.
pub fn gate(e: &Expr, q: Q, mut ctx: Ctx) -> R<Answer> {
    // independent check: different arithmetic (f64 in SI) over the same tree
    match crate::fcheck::eval_f64(e) {
        Some(fv) => {
            let b = q.base()?.to_f64();
            let ok = (b - fv).abs() <= 1e-9 * (1.0 + b.abs().max(fv.abs()));
            ctx.check("independent f64 path (SI)", ok, format!("exact {} ≈ {:.12}, f64 {:.12}", fmt_short(q.base()?), b, fv));
        }
        None => ctx.check("independent f64 path (SI)", false, "the f64 path gave no value, while the exact one did"),
    }
    let failed: Vec<String> = ctx.checks.iter().filter(|c| !c.ok).map(|c| format!("{}: {}", c.what, c.detail)).collect();
    if !failed.is_empty() {
        return Err(Error::CheckFailed(failed.join("; ")));
    }
    Ok(Answer { q, steps: ctx.steps, checks: ctx.checks, notes: ctx.notes })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{num, rat};

    fn qty(n: i128, u: Unit) -> Expr {
        Expr::Qty(num(n).bx(), Mono::of(u))
    }
    fn temp(n: i128, s: Scale) -> Expr {
        Expr::Temp(num(n).bx(), s)
    }
    fn pct(n: i128) -> Expr {
        qty(n, Unit::Pct)
    }

    #[test]
    fn brief_samples() {
        let a = answer(&Expr::call(Func::PctOf, vec![pct(15), num(80)])).unwrap();
        assert_eq!(a.q, Q::plain(Num::int(12)));
        let a = answer(&Expr::Convert(temp(72, Scale::F).bx(), Target::Scale(Scale::C))).unwrap();
        assert_eq!(a.q, Q::temp(Num::Exact(Rat::new(200, 9).unwrap()), Scale::C));
        assert_eq!(fmt_q(&a.q), "≈ 22.2222 °C (exactly 200/9 °C)");
        let a = answer(&Expr::Convert(qty(10, Unit::Mi).bx(), Target::Mono(Mono::of(Unit::Km)))).unwrap();
        assert_eq!(fmt_q(&a.q), "16.09344 km");
        let a = answer(&Expr::call(Func::Mean, vec![num(3), num(5), num(10)])).unwrap();
        assert_eq!(a.q.n, Num::int(6));
        let a = answer(&Expr::Prob(ProbOp::AtLeastOnce, vec![(pct(30), num(3))])).unwrap();
        assert_eq!(fmt_q(&a.q), "65.7 %");
        assert!(a.checks.iter().any(|c| c.what == "enumeration of all outcomes" && c.ok));
        let a = answer(&Expr::call(Func::AbsDiff, vec![temp(72, Scale::F), temp(60, Scale::F)])).unwrap();
        assert_eq!(fmt_q(&a.q), "12 Δ°F");
    }

    /// Negative controls: dimension errors, overflow, absolute versus difference.
    #[test]
    fn negative_controls() {
        let e = Expr::add(qty(5, Unit::Km), qty(3, Unit::Kg));
        assert!(matches!(answer(&e), Err(Error::Dim { .. })));
        let e = Expr::add(temp(72, Scale::F), temp(60, Scale::F));
        assert_eq!(answer(&e).unwrap_err(), Error::Temp(TempMisuse::AddPoints));
        let e = Expr::mul(temp(20, Scale::C), num(2));
        assert_eq!(answer(&e).unwrap_err(), Error::Temp(TempMisuse::ScalePoint));
        let e = Expr::pow(num(10), num(50));
        assert!(matches!(answer(&e), Err(Error::Overflow(_))));
        let e = Expr::div(num(5), num(0));
        assert_eq!(answer(&e).unwrap_err(), Error::DivZero);
        // a difference of 10 °F in Celsius ≠ an absolute 10 °F in Celsius
        let d = answer(&Expr::Convert(qty(10, Unit::DF).bx(), Target::Scale(Scale::C))).unwrap();
        let p = answer(&Expr::Convert(temp(10, Scale::F).bx(), Target::Scale(Scale::C))).unwrap();
        assert_eq!(fmt_q(&d.q), "≈ 5.55556 Δ°C (exactly 50/9 Δ°C)");
        assert_eq!(p.q, Q::temp(Num::Exact(Rat::new(-110, 9).unwrap()), Scale::C));
        assert_ne!(d.q.base().unwrap(), p.q.base().unwrap());
        // a point cannot be converted into a difference unit
        let e = Expr::Convert(temp(10, Scale::F).bx(), Target::Mono(Mono::of(Unit::DC)));
        assert_eq!(answer(&e).unwrap_err(), Error::Temp(TempMisuse::KindMismatch));
    }

    #[test]
    fn mixed_units_and_points() {
        let a = answer(&Expr::add(qty(5, Unit::Km), qty(300, Unit::M))).unwrap();
        assert_eq!(fmt_q(&a.q), "5.3 km");
        let a = answer(&Expr::div(qty(90, Unit::Km), qty(2, Unit::H))).unwrap();
        assert_eq!(fmt_q(&a.q), "45 km/h");
        let a = answer(&Expr::add(temp(20, Scale::C), qty(9, Unit::DF))).unwrap();
        assert_eq!(fmt_q(&a.q), "25 °C");
        let a = answer(&Expr::call(Func::Mean, vec![temp(72, Scale::F), temp(20, Scale::C)])).unwrap();
        assert_eq!(fmt_q(&a.q), "70 °F");
        let a = answer(&Expr::call(Func::Median, vec![num(1), num(2), num(3), num(4)])).unwrap();
        assert_eq!(a.q.n, Num::Exact(Rat::new(5, 2).unwrap()));
        let a = answer(&Expr::add(rat(Rat::parse("0.1").unwrap()), rat(Rat::parse("0.2").unwrap()))).unwrap();
        assert_eq!(fmt_q(&a.q), "0.3");
    }
}

#[cfg(test)]
mod gate_tests {
    use super::*;
    use crate::expr::num;

    /// Negative control of the gate: a deliberately wrong answer (16.09 km instead of 16.09344 km, 13 instead of 12)
    /// fails; the real one passes.
    #[test]
    fn gate_rejects_wrong_answers() {
        let e = Expr::Convert(Expr::Qty(num(10).bx(), Mono::of(Unit::Mi)).bx(), Target::Mono(Mono::of(Unit::Km)));
        let good = Q::with(Num::Exact(Rat::parse("16.09344").unwrap()), Mono::of(Unit::Km));
        let bad = Q::with(Num::Exact(Rat::parse("16.09").unwrap()), Mono::of(Unit::Km));
        assert!(gate(&e, good, Ctx::default()).is_ok());
        assert!(matches!(gate(&e, bad, Ctx::default()), Err(Error::CheckFailed(_))));
        let e = Expr::call(Func::PctOf, vec![Expr::Qty(num(15).bx(), Mono::of(Unit::Pct)), num(80)]);
        assert!(matches!(gate(&e, Q::plain(Num::int(13)), Ctx::default()), Err(Error::CheckFailed(_))));
        // the same physical quantity in other units is not an error (1609.344 m = 1 mi)
        let e = Expr::Convert(Expr::Qty(num(1).bx(), Mono::of(Unit::Mi)).bx(), Target::Mono(Mono::of(Unit::M)));
        assert!(gate(&e, Q::with(Num::Exact(Rat::parse("1609.344").unwrap()), Mono::of(Unit::M)), Ctx::default()).is_ok());
    }
}
