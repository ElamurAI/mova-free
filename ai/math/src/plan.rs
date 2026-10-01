//! v2: the LLM plan language → execution and checking by the MMM. A plan is JSON: steps `{id, op, …}` and an answer.
//! Operations: compute, solve, factor, expand, derive, enumerate, check, claim. Every step yields traces with
//! checks; the gate passes the answer only when all checks are green, `check` steps are true,
//! `claim` hypotheses survived enumeration, and the answer type and the chosen option (for tests) match
//! the computed value.

use std::collections::HashMap;
use std::time::Instant;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::big::*;
use crate::calc::{self, E, Env, Op, V, eval, parse, parse_equation};
use crate::deriv;
use crate::linsys::{self, LinSol};
use crate::nt::{self, Check, Limits};
use crate::poly::{self, Root};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Step {
    #[serde(default)]
    pub id: Option<String>,
    pub op: String,
    #[serde(default)]
    pub expr: Option<String>,
    #[serde(default)]
    pub eqs: Vec<String>,
    #[serde(default)]
    pub vars: Vec<Value>,
    #[serde(default)]
    pub var: Option<String>,
    #[serde(default)]
    pub domain: Option<String>,
    #[serde(default, rename = "where")]
    pub cond: Option<String>,
    #[serde(default)]
    pub agg: Option<String>,
    #[serde(default)]
    pub at: Option<Value>,
    #[serde(default)]
    pub why: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Plan {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub method: String,
    #[serde(default)]
    pub steps: Vec<Step>,
    #[serde(default)]
    pub answer: Value,
    #[serde(default, rename = "type")]
    pub ty: Option<String>,
    #[serde(default)]
    pub option: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TraceStep {
    pub id: String,
    pub op: String,
    pub input: String,
    pub result: String,
    pub checks: Vec<Check>,
    pub subs: Vec<calc::Sub>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Trace {
    pub steps: Vec<TraceStep>,
    /// answer — exact form (None — not reached)
    pub answer: Option<String>,
    /// answer is approximate (transcendental)
    pub approx: bool,
    pub option: Option<String>,
    /// gate rejection reasons (empty — verified)
    pub fails: Vec<String>,
    pub secs: f64,
    /// checks passed / total
    pub checks_ok: usize,
    pub checks_total: usize,
}

impl Trace {
    pub fn verified(&self) -> bool {
        self.fails.is_empty() && self.answer.is_some()
    }
}

/// What the MMM knows about the problem in advance (plausibility gates).
#[derive(Clone, Debug, Default)]
pub struct Expect {
    /// the answer must be an integer (a count of items etc.)
    pub integer: bool,
    /// options (letter → content) for multiple-choice questions
    pub options: Vec<(String, String)>,
}

fn val_str(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

fn range_spec(v: &Value) -> Result<(String, String, String), String> {
    match v {
        Value::Array(xs) if xs.len() == 3 => Ok((val_str(&xs[0]), val_str(&xs[1]), val_str(&xs[2]))),
        other => Err(format!("range must be [name, from, to], got {other}")),
    }
}

/// Run a plan. `lim_secs` is the time limit for the whole plan. A panic inside the MMM is not a crash of the run but
/// a gate failure with a reason.
pub fn run(plan: &Plan, expect: &Expect, lim_secs: f64) -> Trace {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| run_inner(plan, expect, lim_secs))) {
        Ok(t) => t,
        Err(e) => {
            let msg = e.downcast_ref::<String>().cloned().or_else(|| e.downcast_ref::<&str>().map(|s| s.to_string())).unwrap_or_default();
            Trace { steps: Vec::new(), answer: None, approx: false, option: plan.option.clone(), fails: vec![format!("internal MMM error (panic): {msg}")], secs: 0.0, checks_ok: 0, checks_total: 0 }
        }
    }
}

fn run_inner(plan: &Plan, expect: &Expect, lim_secs: f64) -> Trace {
    let t0 = Instant::now();
    let lim = Limits::with_secs(lim_secs);
    let mut env = Env::new(lim);
    let mut steps = Vec::new();
    let mut fails = Vec::new();
    for (k, st) in plan.steps.iter().enumerate() {
        let id = st.id.clone().unwrap_or_else(|| format!("_{}", k + 1));
        env.subs.clear();
        let r = exec_step(st, &id, &mut env);
        let subs = std::mem::take(&mut env.subs);
        match r {
            Ok((input, value, mut checks)) => {
                for s in &subs {
                    checks.extend(s.checks.iter().cloned());
                }
                for c in &checks {
                    if !c.ok {
                        fails.push(format!("step {id} ({}): check failed «{}» {}", st.op, c.name, c.detail));
                    }
                }
                let result = value.as_ref().map(V::show).unwrap_or_default();
                if let Some(v) = value {
                    if !id.starts_with('_') {
                        env.vars.insert(id.clone(), v);
                    }
                }
                steps.push(TraceStep { id, op: st.op.clone(), input, result, checks, subs, error: None });
            }
            Err(e) => {
                fails.push(format!("step {id} ({}): {e}", st.op));
                steps.push(TraceStep { id, op: st.op.clone(), input: step_input(st), result: String::new(), checks: Vec::new(), subs, error: Some(e) });
                break;
            }
        }
    }
    let mut answer = None;
    let mut approx = false;
    if fails.is_empty() {
        let a = val_str(&plan.answer);
        match parse(&a).and_then(|e| {
            env.approx = false;
            let v = eval(&e, &mut env)?;
            Ok((e, v))
        }) {
            Ok((e, v)) => {
                approx = !v.is_exact() || env.approx;
                let mut checks = Vec::new();
                if let Some(c) = calc::independent(&e, &env.vars, &v) {
                    checks.push(c);
                }
                gate_answer(&v, plan, expect, &mut checks, &mut fails);
                answer = Some(v.exact());
                for c in &checks {
                    if !c.ok && !fails.iter().any(|f| f.contains(&c.name)) {
                        fails.push(format!("answer: check failed «{}» {}", c.name, c.detail));
                    }
                }
                steps.push(TraceStep { id: "answer".into(), op: "answer".into(), input: a, result: v.show(), checks, subs: std::mem::take(&mut env.subs), error: None });
            }
            Err(e) => fails.push(format!("answer «{a}»: {e}")),
        }
    }
    let checks_total = steps.iter().map(|s| s.checks.len()).sum();
    let checks_ok = steps.iter().map(|s| s.checks.iter().filter(|c| c.ok).count()).sum();
    Trace { steps, answer, approx, option: plan.option.clone(), fails, secs: t0.elapsed().as_secs_f64(), checks_ok, checks_total }
}

fn step_input(st: &Step) -> String {
    let mut parts = Vec::new();
    if let Some(e) = &st.expr {
        parts.push(e.clone());
    }
    if !st.eqs.is_empty() {
        parts.push(st.eqs.join("; "));
    }
    if !st.vars.is_empty() {
        parts.push(format!("vars {}", st.vars.iter().map(val_str).collect::<Vec<_>>().join(", ")));
    }
    if let Some(w) = &st.cond {
        parts.push(format!("where {w}"));
    }
    if let Some(a) = &st.agg {
        parts.push(format!("agg {a}"));
    }
    parts.join(" | ")
}

/// Numeric value of an option's content («$$14$$», «\frac{3}{4}», «25%», «$1,200»).
pub fn option_value(s: &str) -> Option<Q> {
    let mut t = s.replace("$$", "").replace(['$', '\\', ' ', '~', '＄', '€', '£', '¥', '\u{a0}'], "").replace("{,}", "");
    t = t.replace("dfrac", "frac").replace("textbf", "").replace("text", "");
    if let Some(r) = t.strip_prefix("frac{") {
        let (a, rest) = r.split_once("}{")?;
        let b = rest.strip_suffix('}')?;
        return Some(parse_q(a)? / parse_q(b)?);
    }
    let pct = t.ends_with('%');
    let t = t.trim_end_matches('%').trim_end_matches('.').to_string();
    let v = parse_q(&t)?;
    Some(if pct { v / q(100) } else { v })
}

fn gate_answer(v: &V, plan: &Plan, expect: &Expect, checks: &mut Vec<Check>, fails: &mut Vec<String>) {
    let ty = plan.ty.as_deref().unwrap_or("");
    let want_int = expect.integer || ty == "integer";
    if want_int {
        let ok = matches!(v, V::Q(x) if x.is_int());
        checks.push(Check::new("type: answer is an integer", ok, if ok { String::new() } else { format!("got {}", v.show()) }));
    }
    if !expect.options.is_empty() {
        let vals: Vec<(String, Option<Q>)> = expect.options.iter().map(|(k, s)| (k.clone(), option_value(s))).collect();
        let chosen = plan.option.clone().unwrap_or_default().trim().trim_matches(['(', ')', '.']).to_uppercase();
        let matching: Vec<&String> = match v {
            V::Q(x) => vals.iter().filter(|(_, o)| o.as_ref() == Some(x)).map(|(k, _)| k).collect(),
            V::R(x) => vals.iter().filter(|(_, o)| o.as_ref().is_some_and(|o| (to_f64(o) - x).abs() <= 1e-6 * x.abs().max(1.0))).map(|(k, _)| k).collect(),
            _ => Vec::new(),
        };
        let numeric_opts = vals.iter().filter(|(_, o)| o.is_some()).count();
        if chosen.is_empty() {
            fails.push("test: option letter not given (option)".into());
        } else if !expect.options.iter().any(|(k, _)| *k == chosen) {
            fails.push(format!("test: no option «{chosen}»"));
        } else if numeric_opts == vals.len() || !matching.is_empty() {
            let ok = matching.iter().any(|k| **k == chosen);
            checks.push(Check::new("test: computed value matches the chosen option", ok, if ok { String::new() } else if matching.is_empty() { format!("{} matches no option", v.show()) } else { format!("{} matches option {}, but {chosen} was chosen", v.show(), matching.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(",")) }));
        } else {
            checks.push(Check::new("test: options are not numeric — choice not checked numerically", true, ""));
        }
    }
}

type StepOut = (String, Option<V>, Vec<Check>);

fn exec_step(st: &Step, id: &str, env: &mut Env) -> Result<StepOut, String> {
    let input = step_input(st);
    match st.op.as_str() {
        "compute" | "let" => {
            let src = st.expr.as_deref().ok_or("compute without expr")?;
            let e = parse(src)?;
            let v = eval(&e, env)?;
            let mut checks = Vec::new();
            if let Some(c) = calc::independent(&e, &env.vars, &v) {
                checks.push(c);
            }
            Ok((input, Some(v), checks))
        }
        "check" => {
            let src = st.expr.as_deref().ok_or("check without expr")?;
            let e = parse(src)?;
            let v = eval(&e, env)?;
            let ok = matches!(v, V::B(true));
            let why = st.why.clone().unwrap_or_default();
            Ok((input, None, vec![Check::new(&format!("LLM check: {src}"), ok, if ok { why } else { format!("false ({why})") })]))
        }
        "claim" => claim(st, env).map(|c| (input, None, vec![c])),
        "enumerate" => enumerate(st, env).map(|(v, c)| (input, Some(v), c)),
        "solve" => solve(st, id, env).map(|(v, c)| (input, Some(v), c)),
        "factor" => factor(st, env).map(|(v, c)| (input, Some(v), c)),
        "expand" => {
            let e = parse(st.expr.as_deref().ok_or("expand without expr")?)?;
            let x = pick_var(&e, st, env)?;
            let rf = poly::ratfunc(&e, &x, env)?;
            if rf.den != poly::Poly::one() {
                return Err("expand: expression is not a polynomial (has a denominator)".into());
            }
            let s = rf.num.show(&x);
            // check: values at three rational points match the original expression
            let mut ok = true;
            for t in [parse_q("2/7").unwrap(), q(3), q(-5)] {
                let mut env2 = Env::new(Limits::default());
                env2.vars = env.vars.clone();
                env2.vars.insert(x.clone(), V::Q(t.clone()));
                match eval(&e, &mut env2) {
                    Ok(V::Q(v)) => ok &= v == rf.num.eval(&t),
                    _ => ok = false,
                }
            }
            let coeffs = V::L(rf.num.0.iter().map(|c| V::Q(c.clone())).collect());
            Ok((format!("{input} → {s}"), Some(coeffs), vec![Check::new("expansion: agrees at points 2/7, 3, −5", ok, s)]))
        }
        "derive" => {
            let e = parse(st.expr.as_deref().ok_or("derive without expr")?)?;
            let x = pick_var(&e, st, env)?;
            let mut steps = Vec::new();
            let d = deriv::derive(&e, &x, &mut steps)?;
            let mut checks = vec![deriv::check_derivative(&e, &d, &x, &env.vars)];
            for s in &steps {
                checks.push(Check::new(&format!("rule: {s}"), true, ""));
            }
            let value = match &st.at {
                Some(at) => {
                    let pt = eval(&parse(&val_str(at))?, env)?;
                    let saved = env.vars.insert(x.clone(), pt);
                    let v = eval(&d, env);
                    match saved {
                        Some(s) => env.vars.insert(x.clone(), s),
                        None => env.vars.remove(&x),
                    };
                    v?
                }
                None => V::S(d.to_string()),
            };
            Ok((format!("{input} → d/d{x} = {d}"), Some(value), checks))
        }
        other => Err(format!("unknown operation «{other}»")),
    }
}

fn pick_var(e: &E, st: &Step, env: &Env) -> Result<String, String> {
    if let Some(v) = &st.var {
        return Ok(v.clone());
    }
    let mut vs = Vec::new();
    e.vars(&mut vs);
    let free: Vec<String> = vs.into_iter().filter(|v| !env.vars.contains_key(v) && v != "pi" && v != "e").collect();
    match free.as_slice() {
        [x] => Ok(x.clone()),
        [] => Err("no free variable".into()),
        _ => Err(format!("several free variables {free:?} — specify var")),
    }
}

fn parse_ranges(st: &Step) -> Result<Vec<(String, E, E)>, String> {
    st.vars.iter().map(|v| {
        let (n, a, b) = range_spec(v)?;
        Ok((n, parse(&a)?, parse(&b)?))
    }).collect()
}

/// Nested enumeration over ranges (bounds may depend on earlier variables).
fn walk(ranges: &[(String, E, E)], env: &mut Env, f: &mut dyn FnMut(&mut Env) -> Result<bool, String>) -> Result<bool, String> {
    let Some(((name, lo, hi), rest)) = ranges.split_first() else { return f(env) };
    let lo = eval(lo, env)?;
    let hi = eval(hi, env)?;
    let (lo, hi) = (lo.q().and_then(to_z).ok_or("enumeration bound is not an integer")?, hi.q().and_then(to_z).ok_or("enumeration bound is not an integer")?);
    if &hi - &lo > IBig::from(env.lim.max_steps as i64) {
        return Err(format!("range {name} = {lo}..{hi} is too large"));
    }
    let saved = env.vars.get(name).cloned();
    let mut i = lo;
    let mut go_on = true;
    while i <= hi && go_on {
        env.vars.insert(name.clone(), V::Q(qz(i.clone())));
        go_on = walk(rest, env, f)?;
        i += IBig::ONE;
    }
    match saved {
        Some(s) => env.vars.insert(name.clone(), s),
        None => env.vars.remove(name),
    };
    Ok(go_on)
}

fn enumerate(st: &Step, env: &mut Env) -> Result<(V, Vec<Check>), String> {
    let ranges = parse_ranges(st)?;
    if ranges.is_empty() {
        return Err("enumerate: vars is a list of [name, from, to]".into());
    }
    let cond = st.cond.as_deref().map(parse).transpose()?;
    let agg = st.agg.clone().unwrap_or_else(|| "count".into());
    let (kind, body) = match agg.split_once(':') {
        Some((k, b)) => (k.trim().to_string(), Some(parse(b.trim())?)),
        None => (agg.trim().to_string(), None),
    };
    let mut count = 0u64;
    let mut visited = 0u64;
    let mut acc: Vec<V> = Vec::new();
    let mut total = V::Q(q(0));
    let mut best: Option<V> = None;
    let mut first: Option<V> = None;
    let max_steps = env.lim.max_steps;
    let t0 = Instant::now();
    walk(&ranges, env, &mut |env| {
        visited += 1;
        if visited > max_steps || (visited % 4096 == 0 && env.lim.timed_out()) {
            return Err(format!("enumeration exceeded the limit ({visited} cases, {:.1} s)", t0.elapsed().as_secs_f64()));
        }
        if let Some(c) = &cond {
            if !matches!(eval(c, env)?, V::B(true)) {
                return Ok(true);
            }
        }
        count += 1;
        let val = match &body {
            Some(b) => Some(eval(b, env)?),
            None => None,
        };
        match kind.as_str() {
            "count" => {}
            "sum" => total = arith_add(total.clone(), val.clone().ok_or("sum: expression required sum:<expr>")?, env)?,
            "list" => acc.push(val.clone().unwrap_or_else(|| V::L(ranges.iter().map(|(n, _, _)| env.vars[n].clone()).collect()))),
            "min" | "max" => {
                let v = val.clone().ok_or("min/max: expression required")?;
                let better = match &best {
                    None => true,
                    Some(b) => {
                        let c = eval(&E::bin(if kind == "min" { Op::Lt } else { Op::Gt }, lit(&v)?, lit(b)?), env)?;
                        matches!(c, V::B(true))
                    }
                };
                if better {
                    best = Some(v);
                }
            }
            "first" => {
                first = Some(val.clone().unwrap_or_else(|| V::L(ranges.iter().map(|(n, _, _)| env.vars[n].clone()).collect())));
                return Ok(false);
            }
            other => return Err(format!("agg «{other}» unknown (count, sum:, min:, max:, list:, first:)")),
        }
        if acc.len() > 100_000 {
            return Err("list: more than 10^5 elements".into());
        }
        Ok(true)
    })?;
    let v = match kind.as_str() {
        "count" => V::Q(q(count as i64)),
        "sum" => total,
        "list" => V::L(acc),
        "min" | "max" => best.ok_or("min/max: no case satisfied the condition")?,
        _ => first.ok_or("first: no case satisfied the condition")?,
    };
    Ok((v, vec![Check::new("enumeration completed fully within limits", true, format!("{visited} cases, {count} satisfied the condition"))]))
}

fn lit(v: &V) -> Result<E, String> {
    match v {
        V::Q(x) => Ok(E::Num(x.clone())),
        V::R(x) => RBig::simplest_from_f64(*x).map(E::Num).ok_or("NaN".into()),
        _ => Err("min/max: value is not a number".into()),
    }
}

fn arith_add(a: V, b: V, env: &mut Env) -> Result<V, String> {
    let mut env2 = Env::new(env.lim);
    env2.vars.insert("_a".into(), a);
    env2.vars.insert("_b".into(), b);
    eval(&parse("_a + _b")?, &mut env2)
}

fn claim(st: &Step, env: &mut Env) -> Result<Check, String> {
    let ranges = parse_ranges(st)?;
    let e = parse(st.expr.as_deref().ok_or("claim without expr")?)?;
    let cond = st.cond.as_deref().map(parse).transpose()?;
    let mut cases = 0u64;
    let mut counter: Option<String> = None;
    walk(&ranges, env, &mut |env| {
        if let Some(c) = &cond {
            if !matches!(eval(c, env)?, V::B(true)) {
                return Ok(true);
            }
        }
        cases += 1;
        if cases > env.lim.max_steps {
            return Err("claim: too many cases".into());
        }
        let ok = match eval(&e, env) {
            Ok(V::B(b)) => b,
            Ok(o) => return Err(format!("claim: expression must be yes/no, got {}", o.show())),
            Err(er) => return Err(er),
        };
        if !ok {
            counter = Some(ranges.iter().map(|(n, _, _)| format!("{n} = {}", env.vars[n].show())).collect::<Vec<_>>().join(", "));
            return Ok(false);
        }
        Ok(true)
    })?;
    let why = st.why.clone().unwrap_or_default();
    Ok(match counter {
        Some(c) => Check::new(&format!("hypothesis «{}» — enumeration of small cases", st.expr.as_deref().unwrap_or("")), false, format!("counterexample: {c} ({why})")),
        None => Check::new(&format!("hypothesis «{}» — enumeration of small cases", st.expr.as_deref().unwrap_or("")), true, format!("survived {cases} cases (this is not a proof)")),
    })
}

fn factor(st: &Step, env: &mut Env) -> Result<(V, Vec<Check>), String> {
    let e = parse(st.expr.as_deref().ok_or("factor without expr")?)?;
    let mut vs = Vec::new();
    e.vars(&mut vs);
    let free: Vec<String> = vs.into_iter().filter(|v| !env.vars.contains_key(v)).collect();
    if free.is_empty() && st.var.is_none() {
        let n = eval(&e, env)?;
        let n = n.q().and_then(to_z).ok_or("factor: integer")?;
        let f = nt::factor(&n);
        let c = nt::check_factor(&n, &f);
        if !f.complete {
            return Err(format!("factorization incomplete: {}", f.note));
        }
        let v = V::L(f.factors.iter().map(|(p, k)| V::L(vec![V::Q(qz(p.clone())), V::Q(q(*k as i64))])).collect());
        return Ok((v, vec![c, Check::new("factorization", true, nt::show_factored(&f))]));
    }
    let x = pick_var(&e, st, env)?;
    let rf = poly::ratfunc(&e, &x, env)?;
    if rf.den != poly::Poly::one() {
        return Err("factor: not a polynomial".into());
    }
    let (lead, fs, c) = poly::factor_q(&rf.num);
    let s = poly::show_factorization(&lead, &fs, &x);
    Ok((V::S(s.clone()), vec![c, Check::new("factorization over Q", true, s)]))
}

fn solve(st: &Step, id: &str, env: &mut Env) -> Result<(V, Vec<Check>), String> {
    let vars: Vec<String> = if !st.vars.is_empty() { st.vars.iter().map(val_str).collect() } else { st.var.clone().into_iter().collect() };
    if st.eqs.is_empty() {
        return Err("solve: eqs is empty".into());
    }
    let parsed: Vec<E> = st.eqs.iter().map(|s| parse(s)).collect::<Result<_, _>>()?;
    let domain = st.domain.clone().unwrap_or_else(|| "real".into());
    // inequality in one variable
    if parsed.len() == 1 {
        if let E::Bin(op @ (Op::Lt | Op::Le | Op::Gt | Op::Ge | Op::Ne), a, b) = &parsed[0] {
            let x = vars.first().cloned().ok_or("solve: vars")?;
            let rf = poly::ratfunc(&E::bin(Op::Sub, (**a).clone(), (**b).clone()), &x, env)?;
            let (s, checks) = poly::solve_ineq(&rf, op)?;
            return Ok((V::S(s), checks));
        }
    }
    let sides: Vec<(E, E)> = st.eqs.iter().map(|s| parse_equation(s)).collect::<Result<_, _>>()?;
    let vars = if vars.is_empty() {
        let mut vs = Vec::new();
        for (a, b) in &sides {
            a.vars(&mut vs);
            b.vars(&mut vs);
        }
        vs.into_iter().filter(|v| !env.vars.contains_key(v) && !calc::is_func(v) && v != "pi").collect()
    } else {
        vars
    };
    if vars.is_empty() {
        return Err("solve: no unknowns".into());
    }
    // linear system?
    let mut a = Vec::new();
    let mut b = Vec::new();
    let mut linear = true;
    for (l, r) in &sides {
        match linsys::linear_form(&E::bin(Op::Sub, l.clone(), r.clone()), &vars, env) {
            Ok((c, k)) => {
                a.push(c);
                b.push(-k);
            }
            Err(_) => {
                linear = false;
                break;
            }
        }
    }
    if linear {
        return match linsys::gauss(&a, &b) {
            LinSol::Unique(x) => {
                let c = linsys::check_solution(&a, &b, &x);
                // substitution into the original equations (independent of the linear form)
                let mut env2 = Env::new(env.lim);
                env2.vars = env.vars.clone();
                for (n, v) in vars.iter().zip(&x) {
                    env2.vars.insert(n.clone(), V::Q(v.clone()));
                }
                let mut orig_ok = true;
                for s in &parsed {
                    let e = match s {
                        E::Bin(Op::Eq, _, _) => s.clone(),
                        other => E::bin(Op::Eq, other.clone(), E::num(0)),
                    };
                    orig_ok &= matches!(eval(&e, &mut env2), Ok(V::B(true)));
                }
                let checks = vec![c, Check::new("substitution into the original equations", orig_ok, "")];
                if !domain_ok(&x, &domain) {
                    return Err(format!("unique solution {:?} outside the domain «{domain}»", x.iter().map(show).collect::<Vec<_>>()));
                }
                for (n, v) in vars.iter().zip(&x) {
                    env.vars.insert(n.clone(), V::Q(v.clone()));
                }
                Ok((V::L(x.into_iter().map(V::Q).collect()), checks))
            }
            LinSol::Infinite { particular, free } => Err(format!("system has infinitely many solutions (free: {}; particular {:?}) — add a condition", free.iter().map(|i| vars[*i].clone()).collect::<Vec<_>>().join(", "), particular.iter().map(show).collect::<Vec<_>>())),
            LinSol::Inconsistent(w) => Err(format!("system is inconsistent: {w}")),
        };
    }
    if vars.len() != 1 || sides.len() != 1 {
        return Err("nonlinear system of several equations: reduce it to one equation in one variable (by substitution) or to enumeration (enumerate)".into());
    }
    let x = &vars[0];
    let (l, r) = &sides[0];
    let rf = poly::ratfunc(&E::bin(Op::Sub, l.clone(), r.clone()), x, env)?;
    if rf.num.is_zero() {
        return Err("the equation is an identity: any value from the domain".into());
    }
    let solved = poly::solve_poly(&rf.num)?;
    let mut checks = solved.checks.clone();
    for n in &solved.notes {
        checks.push(Check::new("note", true, n.clone()));
    }
    let mut out = Vec::new();
    let mut shown = Vec::new();
    for (root, _m) in &solved.roots {
        // outside the domain (root of the denominator)
        if let Root::Q(v) = root {
            if rf.dens.iter().chain(std::iter::once(&rf.den)).any(|d| d.eval(v).is_zero()) {
                checks.push(Check::new(&format!("x = {} rejected: denominator 0", show(v)), true, ""));
                continue;
            }
        }
        let keep = match domain.as_str() {
            "complex" | "all" => true,
            "real" => root.is_real(),
            "integer" => matches!(root, Root::Q(v) if v.is_int()),
            "positive" => root.is_real() && root.f64().0 > 0.0,
            "nonneg" | "nonnegative" => root.is_real() && root.f64().0 >= 0.0,
            "positive_integer" | "natural" => matches!(root, Root::Q(v) if v.is_int() && *v > RBig::ZERO),
            "rational" => matches!(root, Root::Q(_)),
            other => return Err(format!("domain «{other}» unknown (real, complex, integer, positive, nonneg, positive_integer, rational)")),
        };
        if keep && !out.iter().any(|v: &V| *v == root.value()) {
            out.push(root.value());
            shown.push(root.show());
        }
    }
    checks.push(Check::new(&format!("roots ({domain})"), true, shown.join(", ")));
    if out.len() == 1 {
        env.vars.insert(x.clone(), out[0].clone());
    }
    let _ = id;
    Ok((V::L(out), checks))
}

fn domain_ok(x: &[Q], domain: &str) -> bool {
    match domain {
        "integer" => x.iter().all(|v| v.is_int()),
        "positive" => x.iter().all(|v| *v > RBig::ZERO),
        "nonneg" | "nonnegative" => x.iter().all(|v| *v >= RBig::ZERO),
        "positive_integer" | "natural" => x.iter().all(|v| v.is_int() && *v > RBig::ZERO),
        _ => true,
    }
}

/// Parse the LLM response: JSON objects per line (``` fences and extra surrounding text are allowed).
pub fn parse_plans(text: &str) -> Vec<Result<Plan, String>> {
    let mut out = Vec::new();
    for obj in json_objects(text) {
        out.push(serde_json::from_str::<Plan>(&obj).map_err(|e| format!("plan JSON: {e}: {}", obj.chars().take(200).collect::<String>())));
    }
    out
}

/// Extract top-level JSON objects from text (by brace balance, aware of strings).
pub fn json_objects(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let cs: Vec<char> = text.chars().collect();
    let mut i = 0;
    while i < cs.len() {
        if cs[i] == '{' {
            let mut depth = 0;
            let mut in_str = false;
            let mut esc = false;
            let st = i;
            while i < cs.len() {
                let c = cs[i];
                if in_str {
                    if esc {
                        esc = false;
                    } else if c == '\\' {
                        esc = true;
                    } else if c == '"' {
                        in_str = false;
                    }
                } else if c == '"' {
                    in_str = true;
                } else if c == '{' {
                    depth += 1;
                } else if c == '}' {
                    depth -= 1;
                    if depth == 0 {
                        out.push(cs[st..=i].iter().collect());
                        break;
                    }
                }
                i += 1;
            }
        }
        i += 1;
    }
    out
}

pub fn show_trace(t: &Trace) -> String {
    let mut s = String::new();
    for st in &t.steps {
        s.push_str(&format!("  [{}] {} {} → {}\n", st.id, st.op, st.input, if let Some(e) = &st.error { format!("ERROR: {e}") } else { st.result.clone() }));
        for sub in &st.subs {
            s.push_str(&format!("      · {} = {}\n", sub.rule, sub.result));
        }
        for c in &st.checks {
            s.push_str(&format!("      {} {}{}\n", if c.ok { "✓" } else { "✗" }, c.name, if c.detail.is_empty() { String::new() } else { format!(" — {}", c.detail) }));
        }
    }
    if !t.fails.is_empty() {
        s.push_str(&format!("  GATE: {}\n", t.fails.join(" | ")));
    }
    s.push_str(&format!("  answer: {}{}\n", t.answer.clone().unwrap_or_else(|| "—".into()), if t.approx { " (approximate)" } else { "" }));
    s
}

pub fn plan_from_json(s: &str) -> Result<Plan, String> {
    serde_json::from_str(s).map_err(|e| e.to_string())
}

/// Negative control of the gate: replace the result of step `id` in the trace with a false value and
/// re-run the following steps with it — the checks must turn red.
pub fn run_with_tamper(plan: &Plan, expect: &Expect, tamper_id: &str, fake: &str) -> Trace {
    let mut p = plan.clone();
    for st in p.steps.iter_mut() {
        if st.id.as_deref() == Some(tamper_id) {
            // tampered result: the same id but a false value (as if the MMM «made a mistake»)
            let orig = st.expr.clone().unwrap_or_default();
            st.op = "tampered".into();
            st.expr = Some(format!("{fake}|{orig}"));
        }
    }
    run_tampered(&p, expect)
}

fn run_tampered(plan: &Plan, expect: &Expect) -> Trace {
    // «tampered»: the fake value is recorded as the result, while an independent check computes the original expression
    let lim = Limits::with_secs(10.0);
    let mut env = Env::new(lim);
    let mut steps = Vec::new();
    let mut fails = Vec::new();
    for (k, st) in plan.steps.iter().enumerate() {
        let id = st.id.clone().unwrap_or_else(|| format!("_{}", k + 1));
        if st.op == "tampered" {
            let src = st.expr.clone().unwrap_or_default();
            let (fake, orig) = src.split_once('|').unwrap_or((&src, ""));
            let fv = parse_q(fake).map(V::Q).unwrap_or(V::S(fake.to_string()));
            let oe = parse(orig).unwrap_or(E::num(0));
            let mut checks = Vec::new();
            if let Some(c) = calc::independent(&oe, &env.vars, &fv) {
                checks.push(c);
            }
            for c in &checks {
                if !c.ok {
                    fails.push(format!("step {id}: check failed «{}» {}", c.name, c.detail));
                }
            }
            env.vars.insert(id.clone(), fv.clone());
            steps.push(TraceStep { id, op: "compute".into(), input: orig.to_string(), result: fv.show(), checks, subs: Vec::new(), error: None });
            continue;
        }
        env.subs.clear();
        match exec_step(st, &id, &mut env) {
            Ok((input, value, checks)) => {
                for c in &checks {
                    if !c.ok {
                        fails.push(format!("step {id}: «{}» {}", c.name, c.detail));
                    }
                }
                if let Some(v) = value.clone() {
                    env.vars.insert(id.clone(), v);
                }
                steps.push(TraceStep { id, op: st.op.clone(), input, result: value.map(|v| v.show()).unwrap_or_default(), checks, subs: Vec::new(), error: None });
            }
            Err(e) => {
                fails.push(format!("step {id}: {e}"));
                break;
            }
        }
    }
    let mut answer = None;
    if let Ok(e) = parse(&val_str(&plan.answer)) {
        if let Ok(v) = eval(&e, &mut env) {
            let mut checks = Vec::new();
            gate_answer(&v, plan, expect, &mut checks, &mut fails);
            for c in &checks {
                if !c.ok {
                    fails.push(format!("answer: «{}» {}", c.name, c.detail));
                }
            }
            answer = Some(v.exact());
        }
    }
    Trace { steps, answer, approx: false, option: plan.option.clone(), fails, secs: 0.0, checks_ok: 0, checks_total: 0 }
}

/// Value as text for comparison with the reference.
pub fn answer_value(t: &Trace) -> Option<V> {
    let a = t.answer.as_ref()?;
    if let Some(x) = parse_q(a) {
        return Some(V::Q(x));
    }
    if let Ok(e) = parse(a) {
        let mut env = Env::new(Limits::default());
        if let Ok(v) = eval(&e, &mut env) {
            return Some(v);
        }
    }
    Some(V::S(a.clone()))
}

pub fn vars_of(plan: &Plan) -> HashMap<String, usize> {
    let mut m = HashMap::new();
    for (i, s) in plan.steps.iter().enumerate() {
        if let Some(id) = &s.id {
            m.insert(id.clone(), i);
        }
    }
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plan(s: &str) -> Plan {
        plan_from_json(s).unwrap()
    }

    #[test]
    fn gsm_like_plan_verifies() {
        let p = plan(r#"{"id":"t1","method":"arith","steps":[
            {"id":"left","op":"compute","expr":"16 - 3 - 4"},
            {"id":"money","op":"compute","expr":"left * 2"},
            {"op":"check","expr":"money / 2 + 3 + 4 == 16","why":"back-substitution"}],
            "answer":"money","type":"integer"}"#);
        let t = run(&p, &Expect { integer: true, ..Default::default() }, 5.0);
        assert!(t.verified(), "{}", show_trace(&t));
        assert_eq!(t.answer.as_deref(), Some("18"));
    }

    #[test]
    fn solve_system_and_quadratic() {
        let p = plan(r#"{"id":"t2","steps":[
            {"id":"s","op":"solve","eqs":["x + y = 10","x - y = 4"],"vars":["x","y"]},
            {"id":"r","op":"solve","eqs":["t^2 - 5t + 6 = 0"],"vars":["t"]},
            {"id":"m","op":"compute","expr":"x*y + max(r)"}],
            "answer":"m","type":"integer"}"#);
        let t = run(&p, &Expect::default(), 5.0);
        assert!(t.verified(), "{}", show_trace(&t));
        assert_eq!(t.answer.as_deref(), Some("24"));
    }

    #[test]
    fn gate_catches_wrong_things() {
        // false LLM check
        let p = plan(r#"{"id":"n1","steps":[{"id":"a","op":"compute","expr":"7*8"},{"op":"check","expr":"a == 54"}],"answer":"a"}"#);
        assert!(!run(&p, &Expect::default(), 5.0).verified());
        // false hypothesis: n^2+n+41 is prime for all n ≤ 50 — enumeration finds the counterexample n = 40
        let p = plan(r#"{"id":"n2","steps":[{"op":"claim","vars":[["n",0,50]],"expr":"isprime(n^2+n+41)"}],"answer":"1"}"#);
        let t = run(&p, &Expect::default(), 5.0);
        assert!(!t.verified());
        assert!(t.fails.iter().any(|f| f.contains("n = 40")), "{:?}", t.fails);
        // non-integer result where an integer is required
        let p = plan(r#"{"id":"n3","steps":[{"id":"a","op":"compute","expr":"7/2"}],"answer":"a"}"#);
        assert!(!run(&p, &Expect { integer: true, ..Default::default() }, 5.0).verified());
        // test: the chosen option does not match the computed value
        let opts = vec![("A".into(), "$$12$$".into()), ("B".into(), "$$14$$".into())];
        let p = plan(r#"{"id":"n4","steps":[{"id":"a","op":"compute","expr":"2*7"}],"answer":"a","option":"A"}"#);
        assert!(!run(&p, &Expect { integer: false, options: opts.clone() }, 5.0).verified());
        let p = plan(r#"{"id":"n4","steps":[{"id":"a","op":"compute","expr":"2*7"}],"answer":"a","option":"B"}"#);
        assert!(run(&p, &Expect { integer: false, options: opts }, 5.0).verified());
    }

    #[test]
    fn tampered_trace_is_caught() {
        // negative control: a false intermediate result is planted in the trace — the independent path catches it
        let p = plan(r#"{"id":"t","steps":[
            {"id":"a","op":"compute","expr":"3^40 + 17"},
            {"id":"b","op":"compute","expr":"a mod 1000"}],"answer":"b"}"#);
        let good = run(&p, &Expect::default(), 5.0);
        assert!(good.verified());
        let bad = run_with_tamper(&p, &Expect::default(), "a", "12157665459056928819"); // correct …818
        assert!(!bad.fails.is_empty(), "tampering not caught: {}", show_trace(&bad));
    }

    #[test]
    fn enumerate_and_derive() {
        let p = plan(r#"{"id":"e","steps":[
            {"id":"c","op":"enumerate","vars":[["a",1,6],["b",1,6]],"where":"(a+b) % 3 == 0","agg":"count"},
            {"id":"pr","op":"compute","expr":"c/36"},
            {"id":"d","op":"derive","expr":"x^3 - 2x","var":"x","at":"2"}],
            "answer":"pr + d"}"#);
        let t = run(&p, &Expect::default(), 5.0);
        assert!(t.verified(), "{}", show_trace(&t));
        assert_eq!(t.answer.as_deref(), Some("31/3"));
    }
}
