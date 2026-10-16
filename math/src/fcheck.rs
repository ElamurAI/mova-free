//! An independent computation path for the gate: the same tree, but different arithmetic — f64 in SI
//! (absolute temperature in kelvins, a percent as a fraction). Conversion does nothing here:
//! the physical quantity does not change, so the exact result after conversion must have the same
//! value in SI. A mismatch signals a bug in the exact path (or in this one).

use crate::expr::{Const, Expr, Func, ProbOp};
use crate::units::Scale;

fn scale_f(m: &crate::units::Mono) -> Option<f64> {
    Some(m.0.iter().fold(1.0, |acc, &(u, e)| acc * u.scale().to_f64().powi(e as i32)))
}

/// The value in SI, or None if the tree cannot be evaluated here.
pub fn eval_f64(e: &Expr) -> Option<f64> {
    Some(match e {
        Expr::Num(r) => r.to_f64(),
        Expr::Var(_) => return None,
        Expr::Const(Const::Pi) => std::f64::consts::PI,
        Expr::Const(Const::E) => std::f64::consts::E,
        Expr::Qty(x, m) => eval_f64(x)? * scale_f(m)?,
        Expr::Temp(x, s) => kelvin(eval_f64(x)?, *s),
        Expr::Neg(x) => -eval_f64(x)?,
        Expr::Add(xs) => xs.iter().map(eval_f64).sum::<Option<f64>>()?,
        Expr::Sub(a, b) => eval_f64(a)? - eval_f64(b)?,
        Expr::Mul(xs) => xs.iter().map(eval_f64).product::<Option<f64>>()?,
        Expr::Div(a, b) => eval_f64(a)? / eval_f64(b)?,
        Expr::Pow(a, b) => eval_f64(a)?.powf(eval_f64(b)?),
        Expr::Convert(x, _) => eval_f64(x)?,
        Expr::Call(f, args) => {
            let v: Vec<f64> = args.iter().map(eval_f64).collect::<Option<_>>()?;
            match f {
                Func::Sin => v[0].sin(),
                Func::Cos => v[0].cos(),
                Func::Tan => v[0].tan(),
                Func::Exp => v[0].exp(),
                Func::Ln => v[0].ln(),
                Func::Sqrt => v[0].sqrt(),
                Func::Abs => v[0].abs(),
                Func::Sum => v.iter().sum(),
                Func::Mean => v.iter().sum::<f64>() / v.len() as f64,
                Func::Median => {
                    let mut s = v.clone();
                    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
                    let n = s.len();
                    if n % 2 == 1 { s[n / 2] } else { (s[n / 2 - 1] + s[n / 2]) / 2.0 }
                }
                Func::Min => v.iter().cloned().fold(f64::INFINITY, f64::min),
                Func::Max => v.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
                Func::Range => v.iter().cloned().fold(f64::NEG_INFINITY, f64::max) - v.iter().cloned().fold(f64::INFINITY, f64::min),
                Func::PctOf => v[0] * v[1],
                Func::PctChange => (v[1] - v[0]) / v[0],
                Func::PctRatio => v[0] / v[1],
                Func::AbsDiff => (v[0] - v[1]).abs(),
            }
        }
        Expr::Prob(op, gs) => {
            let mut none = 1.0;
            let mut all = 1.0;
            for (p, k) in gs {
                let (p, k) = (eval_f64(p)?, eval_f64(k)?);
                none *= (1.0 - p).powf(k);
                all *= p.powf(k);
            }
            match op {
                ProbOp::AtLeastOnce => 1.0 - none,
                ProbOp::NoneOf => none,
                ProbOp::AllOf => all,
            }
        }
    })
}

/// Kelvins from scale degrees — own constants, not from units.rs (path independence).
fn kelvin(x: f64, s: Scale) -> f64 {
    match s {
        Scale::C => x + 273.15,
        Scale::F => (x - 32.0) * 5.0 / 9.0 + 273.15,
        Scale::K => x,
    }
}
