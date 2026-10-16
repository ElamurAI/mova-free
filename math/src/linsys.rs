//! v2: systems of linear equations — Gauss–Jordan elimination over rationals (exact). The check is
//! substituting the solution into every equation; for inconsistent ones — a witness row «0 = c ≠ 0».

use crate::big::*;
use crate::calc::{E, Env, Op, V, eval};
use crate::nt::Check;

/// Linear form Σ cᵢ·xᵢ + c₀ (an error if the expression is nonlinear in the variables).
pub fn linear_form(e: &E, vars: &[String], env: &mut Env) -> Result<(Vec<Q>, Q), String> {
    let n = vars.len();
    let mut vs = Vec::new();
    e.vars(&mut vs);
    if !vs.iter().any(|v| vars.contains(v)) {
        return match eval(e, env)? {
            V::Q(x) => Ok((vec![q(0); n], x)),
            o => Err(format!("constant {} is not exact", o.show())),
        };
    }
    match e {
        E::Var(v) => {
            let i = vars.iter().position(|x| x == v).unwrap();
            let mut c = vec![q(0); n];
            c[i] = q(1);
            Ok((c, q(0)))
        }
        E::Neg(a) => {
            let (c, k) = linear_form(a, vars, env)?;
            Ok((c.into_iter().map(|x| -x).collect(), -k))
        }
        E::Bin(op @ (Op::Add | Op::Sub), a, b) => {
            let (ca, ka) = linear_form(a, vars, env)?;
            let (cb, kb) = linear_form(b, vars, env)?;
            let s = if *op == Op::Add { q(1) } else { q(-1) };
            Ok((ca.into_iter().zip(cb).map(|(x, y)| x + s.clone() * y).collect(), ka + s * kb))
        }
        E::Bin(Op::Mul, a, b) => {
            let (ca, ka) = linear_form(a, vars, env)?;
            let (cb, kb) = linear_form(b, vars, env)?;
            let a_const = ca.iter().all(|x| x.is_zero());
            let b_const = cb.iter().all(|x| x.is_zero());
            if a_const {
                Ok((cb.into_iter().map(|x| x * ka.clone()).collect(), ka * kb))
            } else if b_const {
                Ok((ca.into_iter().map(|x| x * kb.clone()).collect(), ka * kb))
            } else {
                Err(format!("product of variables in «{e}» — the system is not linear"))
            }
        }
        E::Bin(Op::Div, a, b) => {
            let (ca, ka) = linear_form(a, vars, env)?;
            let (cb, kb) = linear_form(b, vars, env)?;
            if !cb.iter().all(|x| x.is_zero()) {
                return Err(format!("variable in the denominator in «{e}» — the system is not linear"));
            }
            if kb.is_zero() {
                return Err("division by zero".into());
            }
            Ok((ca.into_iter().map(|x| x / kb.clone()).collect(), ka / kb))
        }
        _ => Err(format!("«{e}» — not a linear expression")),
    }
}

#[derive(Clone, Debug)]
pub enum LinSol {
    Unique(Vec<Q>),
    /// a particular solution and the free variables (indices)
    Infinite { particular: Vec<Q>, free: Vec<usize> },
    /// index of the equation witnessing inconsistency (after reduction)
    Inconsistent(String),
}

/// Gauss–Jordan: A·x = b.
pub fn gauss(a: &[Vec<Q>], b: &[Q]) -> LinSol {
    let m = a.len();
    let n = a.first().map(|r| r.len()).unwrap_or(0);
    let mut t: Vec<Vec<Q>> = a.iter().zip(b).map(|(r, bi)| r.iter().cloned().chain(std::iter::once(bi.clone())).collect()).collect();
    let mut pivots = Vec::new();
    let mut row = 0;
    for col in 0..n {
        let Some(p) = (row..m).find(|&r| !t[r][col].is_zero()) else { continue };
        t.swap(row, p);
        let pv = t[row][col].clone();
        for x in t[row].iter_mut() {
            *x = x.clone() / pv.clone();
        }
        for r in 0..m {
            if r != row && !t[r][col].is_zero() {
                let f = t[r][col].clone();
                for c in 0..=n {
                    let v = t[r][c].clone() - f.clone() * t[row][c].clone();
                    t[r][c] = v;
                }
            }
        }
        pivots.push(col);
        row += 1;
        if row == m {
            break;
        }
    }
    for r in row..m {
        if !t[r][n].is_zero() {
            return LinSol::Inconsistent(format!("after reduction: 0 = {}", show(&t[r][n])));
        }
    }
    let mut x = vec![q(0); n];
    for (r, &c) in pivots.iter().enumerate() {
        x[c] = t[r][n].clone();
    }
    if pivots.len() == n {
        LinSol::Unique(x)
    } else {
        LinSol::Infinite { particular: x, free: (0..n).filter(|c| !pivots.contains(c)).collect() }
    }
}

/// Substitute the solution into every equation (exact).
pub fn check_solution(a: &[Vec<Q>], b: &[Q], x: &[Q]) -> Check {
    let mut bad = Vec::new();
    for (i, (r, bi)) in a.iter().zip(b).enumerate() {
        let lhs = r.iter().zip(x).fold(q(0), |s, (c, v)| s + c.clone() * v.clone());
        if lhs != *bi {
            bad.push(format!("equation {}: {} ≠ {}", i + 1, show(&lhs), show(bi)));
        }
    }
    Check::new("substitution into every equation", bad.is_empty(), bad.join("; "))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calc::parse_equation;
    use crate::nt::Limits;

    fn system(eqs: &[&str], vars: &[&str]) -> (Vec<Vec<Q>>, Vec<Q>) {
        let vars: Vec<String> = vars.iter().map(|s| s.to_string()).collect();
        let mut env = Env::new(Limits::default());
        let mut a = Vec::new();
        let mut b = Vec::new();
        for e in eqs {
            let (l, r) = parse_equation(e).unwrap();
            let (c, k) = linear_form(&E::bin(Op::Sub, l, r), &vars, &mut env).unwrap();
            a.push(c);
            b.push(-k);
        }
        (a, b)
    }

    #[test]
    fn unique_infinite_none() {
        let (a, b) = system(&["2x + 3y = 7", "x - y = 1"], &["x", "y"]);
        let LinSol::Unique(x) = gauss(&a, &b) else { panic!() };
        assert_eq!(x, vec![parse_q("2").unwrap(), parse_q("1").unwrap()]);
        assert!(check_solution(&a, &b, &x).ok);
        assert!(!check_solution(&a, &b, &[q(2), q(2)]).ok, "negative control");
        let (a, b) = system(&["x + y = 2", "2x + 2y = 4"], &["x", "y"]);
        assert!(matches!(gauss(&a, &b), LinSol::Infinite { .. }));
        let (a, b) = system(&["x + y = 2", "x + y = 3"], &["x", "y"]);
        assert!(matches!(gauss(&a, &b), LinSol::Inconsistent(_)));
        let (a, b) = system(&["x/2 + y/3 + z = 1", "x - y + 2z = 0", "3x + y/2 - z/4 = 5"], &["x", "y", "z"]);
        let LinSol::Unique(x) = gauss(&a, &b) else { panic!() };
        assert!(check_solution(&a, &b, &x).ok);
    }
}
