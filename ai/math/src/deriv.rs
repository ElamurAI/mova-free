//! v2: symbolic derivative with rule steps and a numerical check — the first step of the analysis level.
//! Every applied rule is written into the steps; the result is checked against the central difference
//! (f(x+h) − f(x−h)) / 2h at several points.

use std::collections::HashMap;

use crate::big::*;
use crate::calc::{E, Op, V, approx};
use crate::nt::Check;

fn has(e: &E, x: &str) -> bool {
    let mut vs = Vec::new();
    e.vars(&mut vs);
    vs.iter().any(|v| v == x)
}

fn n(i: i64) -> E {
    E::num(i)
}

fn is_num(e: &E, v: i64) -> bool {
    matches!(e, E::Num(x) if *x == q(v))
}

/// Simplification: constant folding, x·1, x+0, x·0, x^1.
pub fn simplify(e: E) -> E {
    match e {
        E::Bin(op, a, b) => {
            let a = simplify(*a);
            let b = simplify(*b);
            if let (E::Num(x), E::Num(y)) = (&a, &b) {
                match op {
                    Op::Add => return E::Num(x.clone() + y.clone()),
                    Op::Sub => return E::Num(x.clone() - y.clone()),
                    Op::Mul => return E::Num(x.clone() * y.clone()),
                    Op::Div if !y.is_zero() => return E::Num(x.clone() / y.clone()),
                    Op::Pow if y.is_int() && to_i64(y).is_some_and(|k| k.abs() <= 64) && !(x.is_zero() && *y < RBig::ZERO) => return E::Num(x.pow(to_i64(y).unwrap() as isize)),
                    _ => {}
                }
            }
            match op {
                Op::Add if is_num(&a, 0) => b,
                Op::Add | Op::Sub if is_num(&b, 0) => a,
                Op::Sub if is_num(&a, 0) => simplify(E::Neg(Box::new(b))),
                Op::Mul if is_num(&a, 0) || is_num(&b, 0) => n(0),
                Op::Mul if is_num(&a, 1) => b,
                Op::Mul if is_num(&b, 1) => a,
                Op::Mul if is_num(&a, -1) => simplify(E::Neg(Box::new(b))),
                Op::Div if is_num(&b, 1) => a,
                Op::Div if is_num(&a, 0) => n(0),
                Op::Pow if is_num(&b, 1) => a,
                Op::Pow if is_num(&b, 0) => n(1),
                _ => E::bin(op, a, b),
            }
        }
        E::Neg(a) => match simplify(*a) {
            E::Num(x) => E::Num(-x),
            E::Neg(b) => *b,
            b => E::Neg(Box::new(b)),
        },
        E::Call(f, xs) => E::Call(f, xs.into_iter().map(simplify).collect()),
        e => e,
    }
}

fn call(f: &str, a: E) -> E {
    E::Call(f.into(), vec![a])
}

/// d/dx e; steps go into `steps`.
pub fn derive(e: &E, x: &str, steps: &mut Vec<String>) -> Result<E, String> {
    if !has(e, x) {
        return Ok(n(0));
    }
    let d = |a: &E, steps: &mut Vec<String>| derive(a, x, steps);
    let r = match e {
        E::Var(_) => n(1),
        E::Neg(a) => E::Neg(Box::new(d(a, steps)?)),
        E::Bin(Op::Add, a, b) => {
            steps.push(format!("sum: ({a} + {b})' = ({a})' + ({b})'"));
            E::bin(Op::Add, d(a, steps)?, d(b, steps)?)
        }
        E::Bin(Op::Sub, a, b) => {
            steps.push(format!("difference: ({a} − {b})' = ({a})' − ({b})'"));
            E::bin(Op::Sub, d(a, steps)?, d(b, steps)?)
        }
        E::Bin(Op::Mul, a, b) => {
            if !has(a, x) {
                steps.push(format!("constant factor: ({a}·u)' = {a}·u'"));
                E::bin(Op::Mul, (**a).clone(), d(b, steps)?)
            } else if !has(b, x) {
                steps.push(format!("constant factor: (u·{b})' = u'·{b}"));
                E::bin(Op::Mul, d(a, steps)?, (**b).clone())
            } else {
                steps.push(format!("product: (u·v)' = u'·v + u·v', u = {a}, v = {b}"));
                E::bin(Op::Add, E::bin(Op::Mul, d(a, steps)?, (**b).clone()), E::bin(Op::Mul, (**a).clone(), d(b, steps)?))
            }
        }
        E::Bin(Op::Div, a, b) => {
            if !has(b, x) {
                steps.push(format!("division by a constant: (u/{b})' = u'/{b}"));
                E::bin(Op::Div, d(a, steps)?, (**b).clone())
            } else {
                steps.push(format!("quotient: (u/v)' = (u'·v − u·v')/v², u = {a}, v = {b}"));
                E::bin(Op::Div, E::bin(Op::Sub, E::bin(Op::Mul, d(a, steps)?, (**b).clone()), E::bin(Op::Mul, (**a).clone(), d(b, steps)?)), E::bin(Op::Pow, (**b).clone(), n(2)))
            }
        }
        E::Bin(Op::Pow, a, b) => {
            if !has(b, x) {
                steps.push(format!("power and chain: (u^k)' = k·u^(k−1)·u', u = {a}, k = {b}"));
                let k1 = simplify(E::bin(Op::Sub, (**b).clone(), n(1)));
                E::bin(Op::Mul, E::bin(Op::Mul, (**b).clone(), E::bin(Op::Pow, (**a).clone(), k1)), d(a, steps)?)
            } else if !has(a, x) {
                steps.push(format!("exponential: (c^v)' = c^v·ln c·v', c = {a}"));
                E::bin(Op::Mul, E::bin(Op::Mul, e.clone(), call("ln", (**a).clone())), d(b, steps)?)
            } else {
                steps.push(format!("u^v = e^(v·ln u): (u^v)' = u^v·(v'·ln u + v·u'/u)"));
                E::bin(Op::Mul, e.clone(), E::bin(Op::Add, E::bin(Op::Mul, d(b, steps)?, call("ln", (**a).clone())), E::bin(Op::Div, E::bin(Op::Mul, (**b).clone(), d(a, steps)?), (**a).clone())))
            }
        }
        E::Call(f, args) if args.len() == 1 => {
            let u = &args[0];
            let du = d(u, steps)?;
            let (outer, rule): (E, &str) = match f.as_str() {
                "sin" => (call("cos", u.clone()), "(sin u)' = cos u·u'"),
                "cos" => (E::Neg(Box::new(call("sin", u.clone()))), "(cos u)' = −sin u·u'"),
                "tan" => (E::bin(Op::Div, n(1), E::bin(Op::Pow, call("cos", u.clone()), n(2))), "(tan u)' = u'/cos²u"),
                "exp" => (call("exp", u.clone()), "(e^u)' = e^u·u'"),
                "ln" | "log" => (E::bin(Op::Div, n(1), u.clone()), "(ln u)' = u'/u"),
                "sqrt" => (E::bin(Op::Div, n(1), E::bin(Op::Mul, n(2), call("sqrt", u.clone()))), "(√u)' = u'/(2√u)"),
                "asin" => (E::bin(Op::Div, n(1), call("sqrt", E::bin(Op::Sub, n(1), E::bin(Op::Pow, u.clone(), n(2))))), "(arcsin u)' = u'/√(1−u²)"),
                "acos" => (E::Neg(Box::new(E::bin(Op::Div, n(1), call("sqrt", E::bin(Op::Sub, n(1), E::bin(Op::Pow, u.clone(), n(2))))))), "(arccos u)' = −u'/√(1−u²)"),
                "atan" => (E::bin(Op::Div, n(1), E::bin(Op::Add, n(1), E::bin(Op::Pow, u.clone(), n(2)))), "(arctan u)' = u'/(1+u²)"),
                _ => return Err(format!("derivative of {f} not supported")),
            };
            steps.push(format!("chain: {rule}, u = {u}"));
            E::bin(Op::Mul, outer, du)
        }
        _ => return Err(format!("derivative of «{e}» not supported")),
    };
    Ok(simplify(r))
}

/// Numerical check of a derivative by central difference at several points.
pub fn check_derivative(f: &E, df: &E, x: &str, env: &HashMap<String, V>) -> Check {
    let pts: [f64; 8] = [0.3, 0.7, 1.1, 1.9, 2.6, -0.4, -1.3, 3.7];
    let mut used = 0;
    let mut worst: f64 = 0.0;
    let mut bad = Vec::new();
    for &p in &pts {
        let at = |t: f64| {
            let mut vars = env.clone();
            vars.insert(x.to_string(), V::R(t));
            approx(f, &vars)
        };
        let h = 1e-5 * p.abs().max(1.0);
        let (Some(a), Some(b)) = (at(p + h), at(p - h)) else { continue };
        let mut vars = env.clone();
        vars.insert(x.to_string(), V::R(p));
        let Some(dv) = approx(df, &vars) else { continue };
        if !(a.is_finite() && b.is_finite() && dv.is_finite()) {
            continue;
        }
        let num = (a - b) / (2.0 * h);
        let err = (num - dv).abs() / dv.abs().max(1.0);
        used += 1;
        worst = worst.max(err);
        if err > 1e-5 {
            bad.push(format!("x = {p}: difference {num:.8}, derivative {dv:.8}"));
        }
    }
    Check::new("derivative: central difference at points", used >= 3 && bad.is_empty(), if used < 3 { format!("only {used} points in the domain") } else if bad.is_empty() { format!("{used} points, largest relative error {worst:.1e}") } else { bad.join("; ") })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calc::parse;

    #[test]
    fn derivatives_with_checks() {
        for s in ["x^3*sin(x)", "(x^2+1)/(x-5)", "exp(2x)*ln(x^2+1)", "sqrt(1+x^2)", "3^x", "x^x", "atan(x/2)"] {
            let f = parse(s).unwrap();
            let mut steps = Vec::new();
            let df = derive(&f, "x", &mut steps).unwrap();
            let c = check_derivative(&f, &df, "x", &HashMap::new());
            assert!(c.ok, "{s}: {df} — {c:?}");
            assert!(!steps.is_empty());
        }
        // negative control: a wrong derivative does not pass
        let f = parse("x^3*sin(x)").unwrap();
        let wrong = parse("3*x^2*cos(x)").unwrap();
        assert!(!check_derivative(&f, &wrong, "x", &HashMap::new()).ok);
    }
}
