//! v2: a curriculum from small to large — level sets, sampling with a fixed seed,
//! comparison with the reference answer, LLM prompts (answer only; plan in the step language; correction after the gate).
//!
//! Levels (only sets with a clear licence):
//! 0. GSM8K test (MIT) — 200 random;
//! 1. algebra: TAL-SCQ5K-EN (MIT), topic "equations", easy + MMLU high_school_mathematics (MIT), algebra;
//! 2. number theory and combinatorics: TheoremQA (MIT), integer answers + TAL-SCQ5K-EN "number theory"/"counting";
//! 3. competition level: TAL-SCQ5K-EN, difficulty ≥ 2 (TAL competition problems, own, MIT).

use std::collections::HashSet;
use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::big::*;
use crate::calc::{Env, V, eval, parse};
use crate::nt::Limits;

pub const SEED: u64 = 20_260_926;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Problem {
    pub id: String,
    pub level: u8,
    pub source: String,
    pub license: String,
    /// dev — memory and learning; held — held-out sample of the level
    pub split: String,
    pub question: String,
    #[serde(default)]
    pub options: Vec<(String, String)>,
    pub gold: String,
    /// int | number | option
    pub gold_kind: String,
    #[serde(default)]
    pub topic: String,
}

fn splitmix(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = x;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

fn fnv(s: &str) -> u64 {
    s.bytes().fold(0xcbf2_9ce4_8422_2325u64, |h, b| (h ^ b as u64).wrapping_mul(0x0100_0000_01b3))
}

/// Deterministic sample of k from a set (seed + hash of id).
pub fn sample<T: Clone>(items: &[T], id: impl Fn(&T) -> String, k: usize, salt: &str) -> Vec<T> {
    let mut keyed: Vec<(u64, T)> = items.iter().map(|x| (splitmix(SEED ^ fnv(&format!("{salt}:{}", id(x)))), x.clone())).collect();
    keyed.sort_by_key(|(k, _)| *k);
    keyed.into_iter().take(k).map(|(_, x)| x).collect()
}

fn jsonl(path: &Path) -> Result<Vec<Value>> {
    let text = std::fs::read_to_string(path).with_context(|| path.display().to_string())?;
    text.lines().filter(|l| !l.trim().is_empty()).map(|l| Ok(serde_json::from_str(l)?)).collect()
}

fn s(v: &Value, k: &str) -> String {
    match &v[k] {
        Value::String(x) => x.clone(),
        Value::Null => String::new(),
        o => o.to_string(),
    }
}

fn tal_options(v: &Value) -> Vec<(String, String)> {
    v["answer_option_list"].as_array().map(|xs| xs.iter().filter_map(|o| {
        let o = o.as_array()?.first()?;
        Some((o["aoVal"].as_str()?.to_string(), o["content"].as_str()?.trim().to_string()))
    }).collect()).unwrap_or_default()
}

fn tal_routes(v: &Value) -> String {
    v["knowledge_point_routes"].as_array().map(|xs| xs.iter().filter_map(|x| x.as_str()).collect::<Vec<_>>().join(" | ")).unwrap_or_default()
}

const PHYSICS: &[&str] = &["velocity", "acceleration", "rad/s", "newton", "voltage", "joule", "momentum", "friction", "angular"];

fn split_dev_held(mut ps: Vec<Problem>) -> Vec<Problem> {
    let n = ps.len();
    for (i, p) in ps.iter_mut().enumerate() {
        p.split = if i < n / 2 { "dev".into() } else { "held".into() };
    }
    ps
}

/// Assemble the level sets from raw files (`data/raw/mathsolve`) into `out/level{N}.jsonl`.
pub fn prep(raw: &Path, out: &Path) -> Result<Vec<(u8, usize)>> {
    std::fs::create_dir_all(out)?;
    let mut counts = Vec::new();
    // level 0: GSM8K test
    let gsm = jsonl(&raw.join("gsm8k/test.jsonl"))?;
    let all0: Vec<Problem> = gsm.iter().enumerate().map(|(i, v)| {
        let a = s(v, "answer");
        let gold = a.rsplit("####").next().unwrap_or("").trim().replace(',', "");
        Problem { id: format!("gsm8k-test-{i}"), level: 0, source: "GSM8K test (openai/gsm8k)".into(), license: "MIT".into(), split: String::new(), question: s(v, "question"), options: Vec::new(), gold, gold_kind: "int".into(), topic: "word problem".into() }
    }).collect();
    let l0 = split_dev_held(sample(&all0, |p| p.id.clone(), 200, "L0"));
    // TAL-SCQ5K-EN
    let tal = jsonl(&raw.join("tal-scq5k/en-test.jsonl"))?;
    let talp: Vec<(Problem, i64, String)> = tal.iter().filter_map(|v| {
        let opts = tal_options(v);
        let q = s(v, "problem");
        if opts.is_empty() || PHYSICS.iter().any(|w| q.to_lowercase().contains(w)) {
            return None;
        }
        let d: i64 = s(v, "difficulty").parse().unwrap_or(0);
        let routes = tal_routes(v);
        Some((Problem { id: format!("tal-en-test-{}", s(v, "queId")), level: 0, source: "TAL-SCQ5K-EN test (math-eval/TAL-SCQ5K)".into(), license: "MIT".into(), split: String::new(), question: q, options: opts, gold: s(v, "answer_value"), gold_kind: "option".into(), topic: routes.clone() }, d, routes))
    }).collect();
    // MMLU
    let mmlu = jsonl(&raw.join("mmlu/hs-math.jsonl"))?;
    let letters = ["A", "B", "C", "D", "E"];
    let mmlu_p: Vec<Problem> = mmlu.iter().filter_map(|v| {
        let q = s(v, "question");
        let lq = q.to_lowercase();
        let alg = ["equation", "polynomial", "roots", "root of", "inequalit", "solve", "system", "quadratic", "zeros", "factor", "f(x)", "g(x)", "x^2", "x²"].iter().any(|w| lq.contains(w));
        if !alg {
            return None;
        }
        let ch = v["choices"].as_array()?;
        let opts: Vec<(String, String)> = ch.iter().enumerate().map(|(i, c)| (letters[i].to_string(), match c { Value::String(x) => x.clone(), o => o.to_string() })).collect();
        let ans = v["answer"].as_u64()? as usize;
        Some(Problem { id: format!("mmlu-{}", s(v, "id")), level: 1, source: "MMLU high_school_mathematics test (cais/mmlu)".into(), license: "MIT".into(), split: String::new(), question: q, options: opts, gold: letters[ans].into(), gold_kind: "option".into(), topic: "algebra".into() })
    }).collect();
    // level 1: algebra
    let tal_alg: Vec<Problem> = talp.iter().filter(|(_, d, r)| *d <= 1 && (r.contains("Equation") || r.contains("Operations through Formulas") || r.contains("Power"))).map(|(p, _, _)| Problem { level: 1, ..p.clone() }).collect();
    let mut l1 = sample(&tal_alg, |p| p.id.clone(), 40, "L1t");
    l1.extend(sample(&mmlu_p, |p| p.id.clone(), 20, "L1m"));
    let l1 = split_dev_held(sample(&l1, |p| p.id.clone(), l1.len(), "L1"));
    // level 2: number theory and combinatorics
    let tq = jsonl(&raw.join("theoremqa/test.jsonl"))?;
    let kw = ["prime", "divisor", "gcd", "greatest common", "modulo", "remainder", "congruen", "how many ways", "permutation", "combination", "partition", "stirling", "catalan", "binomial", "totient", "digits", "factorial", "divisible", "lcm", "subsets", "arrangements", "fermat", "euler"];
    let tq_p: Vec<Problem> = tq.iter().enumerate().filter_map(|(i, v)| {
        if v["has_picture"].as_bool() == Some(true) || s(v, "answer_type") != "integer" {
            return None;
        }
        let q = s(v, "question");
        let lq = q.to_lowercase();
        kw.iter().any(|w| lq.contains(w)).then(|| Problem { id: format!("theoremqa-{i}"), level: 2, source: "TheoremQA test (TIGER-Lab/TheoremQA)".into(), license: "MIT".into(), split: String::new(), question: q, options: Vec::new(), gold: s(v, "answer"), gold_kind: "number".into(), topic: "number theory / combinatorics".into() })
    }).collect();
    let tal_nt: Vec<Problem> = talp.iter().filter(|(_, d, r)| *d <= 1 && (r.contains("Number Theory") || r.contains("Counting"))).map(|(p, _, _)| Problem { level: 2, ..p.clone() }).collect();
    let mut l2 = sample(&tq_p, |p| p.id.clone(), 30, "L2q");
    l2.extend(sample(&tal_nt, |p| p.id.clone(), 30, "L2t"));
    let l2 = split_dev_held(sample(&l2, |p| p.id.clone(), l2.len(), "L2"));
    // level 3: TAL competition problems of difficulty ≥ 2 only from mathematical subtopics (without AP statistics,
    // biology and physics that occur in "Calculation Modules" without a subtopic) + MMLU college_mathematics
    let math_routes = ["Number Theory Modules->", "Counting Modules->", "Combinatorics->", "Geometry", "Operations with New Definition", "Sequences", "Equation"];
    let tal_hard: Vec<Problem> = talp.iter().filter(|(_, d, r)| *d >= 2 && math_routes.iter().any(|m| r.contains(m))).map(|(p, _, _)| Problem { level: 3, ..p.clone() }).collect();
    let col = jsonl(&raw.join("mmlu/college-math.jsonl"))?;
    let col_p: Vec<Problem> = col.iter().filter_map(|v| {
        let ch = v["choices"].as_array()?;
        let opts: Vec<(String, String)> = ch.iter().enumerate().map(|(i, c)| (letters[i].to_string(), match c { Value::String(x) => x.clone(), o => o.to_string() })).collect();
        let ans = v["answer"].as_u64()? as usize;
        Some(Problem { id: format!("mmlu-{}", s(v, "id")), level: 3, source: "MMLU college_mathematics test (cais/mmlu)".into(), license: "MIT".into(), split: String::new(), question: s(v, "question"), options: opts, gold: letters[ans].into(), gold_kind: "option".into(), topic: "college mathematics".into() })
    }).collect();
    let mut l3 = sample(&tal_hard, |p| p.id.clone(), 20, "L3t");
    l3.extend(sample(&col_p, |p| p.id.clone(), 20, "L3c"));
    let l3 = split_dev_held(sample(&l3, |p| p.id.clone(), l3.len(), "L3"));
    for (lvl, ps) in [(0u8, &l0), (1, &l1), (2, &l2), (3, &l3)] {
        let mut text = String::new();
        let mut seen = HashSet::new();
        for p in ps.iter() {
            if seen.insert(p.id.clone()) {
                let mut p = p.clone();
                p.level = lvl;
                text.push_str(&serde_json::to_string(&p)?);
                text.push('\n');
            }
        }
        std::fs::write(out.join(format!("level{lvl}.jsonl")), text)?;
        counts.push((lvl, ps.len()));
    }
    Ok(counts)
}

/// Training level 5: a sample of GSM8K train (MIT) — only for the SLM's memory, split `train`, not used in C measurements.
/// Raw file — `gsm8k/train.jsonl` (from `train.parquet` via DuckDB).
pub const TRAIN_LEVEL: u8 = 5;

pub fn prep_train(raw: &Path, out: &Path, n: usize) -> Result<usize> {
    let gsm = jsonl(&raw.join("gsm8k/train.jsonl"))?;
    let all: Vec<Problem> = gsm.iter().enumerate().map(|(i, v)| {
        let a = s(v, "answer");
        let gold = a.rsplit("####").next().unwrap_or("").trim().replace(',', "");
        Problem { id: format!("gsm8k-train-{i}"), level: TRAIN_LEVEL, source: "GSM8K train (openai/gsm8k)".into(), license: "MIT".into(), split: "train".into(), question: s(v, "question"), options: Vec::new(), gold, gold_kind: "int".into(), topic: "word problem".into() }
    }).collect();
    let ps = sample(&all, |p| p.id.clone(), n, "L5");
    let mut text = String::new();
    for p in &ps {
        text.push_str(&serde_json::to_string(p)?);
        text.push('\n');
    }
    std::fs::create_dir_all(out)?;
    std::fs::write(out.join(format!("level{TRAIN_LEVEL}.jsonl")), text)?;
    Ok(ps.len())
}

/// Levels from which the SLM takes memory: curriculum 0–3 and the training one.
pub const MEMORY_LEVELS: [u8; 5] = [0, 1, 2, 3, TRAIN_LEVEL];

pub fn load_level(dir: &Path, level: u8) -> Result<Vec<Problem>> {
    jsonl(&dir.join(format!("level{level}.jsonl")))?.into_iter().map(|v| Ok(serde_json::from_value(v)?)).collect()
}

// ---------- comparison with the reference ----------

fn norm_num(a: &str) -> Option<V> {
    let t = a.trim().trim_start_matches('$').replace([',', '$', ' '], "").replace("\\%", "%").replace("\\frac{", "frac{");
    let t = t.trim_end_matches('.').to_string();
    if let Some(r) = t.strip_prefix("frac{") {
        let (x, rest) = r.split_once("}{")?;
        let y = rest.strip_suffix('}')?;
        return Some(V::Q(parse_q(x)? / parse_q(y)?));
    }
    if let Some(x) = parse_q(&t) {
        return Some(V::Q(x));
    }
    // first number in the line ("18 dollars")
    let num: String = t.chars().skip_while(|c| !(c.is_ascii_digit() || *c == '-')).take_while(|c| c.is_ascii_digit() || *c == '.' || *c == '-' || *c == '/').collect();
    if let Some(x) = parse_q(&num) {
        return Some(V::Q(x));
    }
    let e = parse(&t).ok()?;
    let mut env = Env::new(Limits::default());
    eval(&e, &mut env).ok()
}

/// Whether the answer matches the reference. `answer` — number/expression; `option` — letter (for tests).
pub fn grade(p: &Problem, answer: Option<&str>, option: Option<&str>) -> bool {
    match p.gold_kind.as_str() {
        "option" => {
            let pick = |s: &str| s.trim().trim_start_matches(['(', '[']).chars().next().map(|c| c.to_ascii_uppercase().to_string());
            if let Some(o) = option.and_then(pick) {
                if p.options.iter().any(|(k, _)| *k == o) {
                    return o == p.gold;
                }
            }
            // a letter answer in the answer field
            if let Some(a) = answer {
                let t = a.trim();
                if t.len() <= 3 {
                    if let Some(o) = pick(t) {
                        if p.options.iter().any(|(k, _)| *k == o) {
                            return o == p.gold;
                        }
                    }
                }
                // number → the option with the same value
                if let Some(V::Q(x)) = norm_num(t) {
                    let hits: Vec<&String> = p.options.iter().filter(|(_, c)| crate::plan::option_value(c).as_ref() == Some(&x)).map(|(k, _)| k).collect();
                    return hits.len() == 1 && *hits[0] == p.gold;
                }
            }
            false
        }
        _ => {
            let Some(a) = answer else { return false };
            let (Some(got), Some(gold)) = (norm_num(a), norm_num(&p.gold)) else { return false };
            match (&got, &gold) {
                (V::Q(x), V::Q(y)) if p.gold_kind == "int" => x == y,
                _ => {
                    let (x, y) = (got.f64().unwrap_or(f64::NAN), gold.f64().unwrap_or(f64::NAN));
                    (x - y).abs() <= 1e-2 * y.abs().max(1e-9) || (x - y).abs() < 1e-9
                }
            }
        }
    }
}

// ---------- prompts ----------

fn problem_block(p: &Problem) -> String {
    let mut s = format!("### {}\n{}\n", p.id, p.question.trim());
    if !p.options.is_empty() {
        for (k, c) in &p.options {
            s.push_str(&format!("({k}) {c}\n"));
        }
    }
    s
}

/// Configuration A: the LLM alone — answer only.
pub fn prompt_answers(ps: &[Problem]) -> String {
    let mut s = String::from(
        "Solve each math problem below. Think as long as you need, but OUTPUT ONLY one line per problem, a JSON object:\n\
         {\"id\": \"<problem id>\", \"answer\": \"<final answer: a number, or for multiple choice the option letter>\"}\n\
         No explanations, no code fences.\n\n",
    );
    for p in ps {
        s.push_str(&problem_block(p));
        s.push('\n');
    }
    s
}

pub const PLAN_SPEC: &str = r#"You are the PLANNER in a two-part math solver. You write a PLAN in a small typed step language; a deterministic exact engine executes it and independently verifies every step. Do NOT compute numbers yourself: write expressions and let the engine compute exactly (big integers, exact fractions). Your job is the modelling: which quantities, which equations, which method.

OUTPUT ONLY one line per problem — a JSON object, no prose, no code fences:
{"id": "<problem id>", "method": "<METHOD>", "steps": [<step>, ...], "answer": "<expression>", "type": "integer|rational|real|list", "option": "<letter, only for multiple choice>"}

Steps run in order; a step's "id" becomes a variable for later steps (use short snake_case ids, never single letters that you also use as unknowns):
- {"id":"total","op":"compute","expr":"3*12 + 5"}
- {"id":"s","op":"solve","eqs":["2*x + 3*y = 7","x - y = 1"],"vars":["x","y"],"domain":"real"} — linear systems (any size) or ONE polynomial/rational equation in one unknown (exact roots, radicals allowed). After a unique solution the unknowns (x, y) become variables; s is the list of solutions. domain: real|complex|integer|positive|nonneg|positive_integer|rational. One inequality in one unknown: returns the solution set as text (to COUNT integer solutions use count(...)).
- {"id":"f","op":"factor","expr":"360"} → list of [prime, exponent]; a polynomial → factored form over Q
- {"id":"c","op":"expand","expr":"(1+2*x)^7","var":"x"} → coefficient list [c0, c1, ...] (so c[3] is the x^3 coefficient)
- {"id":"d","op":"derive","expr":"x^3*sin(x)","var":"x","at":"2"} → derivative (its value at x=2 when "at" is given)
- {"id":"k","op":"enumerate","vars":[["a",1,9],["b","a",9]],"where":"(a*b) % 6 == 0","agg":"count"} — brute force over integer ranges (bounds may use earlier loop vars); agg: count | sum:<expr> | min:<expr> | max:<expr> | list:<expr> | first:<expr>
- {"op":"check","expr":"<boolean>","why":"..."} — an INDEPENDENT verification you want the engine to test, e.g. plug the answer back into the original conditions of the problem (not a restatement of the same formula). Add at least one check to every plan where it is possible.
- {"op":"claim","vars":[["n",1,60]],"expr":"<boolean>","why":"..."} — a conjecture/pattern you rely on; the engine tests it on all listed cases and rejects the plan on a counterexample.

Expression syntax: + - * / ^ ! (factorial) mod or %, parentheses, == != < <= > >=, and/or/not, lists [a,b,c], indexing s[0] (s[-1] is the last), implicit multiplication 2x. Numbers are exact: 0.1+0.2 == 0.3, 1/3 stays a fraction; write decimals as in the problem. Percent: write p/100.
Functions: abs floor ceil round (half away from zero) trunc min max gcd lcm mod idiv (floor division) sqrt isqrt root(x,k) num den frac
 isprime nextprime prevprime primepi nthprime factor divisors tau sigma(n[,k]) phi mobius modpow(b,e,m) modinv(a,m) crt([r1,r2],[m1,m2]) digits(n[,base]) digitsum(n[,base]) numdigits fromdigits(list[,base]) valuation(n,p) issquare
 fact binom(n,k) perm(n,k) catalan stirling1 stirling2 bell partitions(n) partitionsk(n,k) partitionsdistinct derangements fib multinomial([k1,k2,...])
 len sum prod sort reverse unique mean median range(a,b)
 sum(expr,k,a,b) prod(expr,k,a,b) count(cond,k,a,b) filter(cond,k,a,b) list(expr,k,a,b) forall(cond,k,a,b) exists(cond,k,a,b) first(cond,k,a,b) maxof(expr,k,a,b) minof(expr,k,a,b) if(cond,a,b)
 sin cos tan asin acos atan exp ln log(x[,base]) pi e — approximate reals; avoid them when an exact form exists.
Limits: ≤ 2·10^7 elementary steps per plan; keep enumeration ranges modest.
Multiple choice: compute the value, and set "option" to the letter whose value equals your computed answer (the engine compares them). If options are not numbers, still compute what you can and choose the letter.
METHOD (pick one): arith, percent, ratio, rate, linear_eq, system, quadratic, polynomial, inequality, sequence, counting, casework_enum, number_theory, digits, probability, geometry_formula, calculus, logic, other.

Example:
{"id":"ex1","method":"arith","steps":[{"id":"eggs_left","op":"compute","expr":"16 - 3 - 4"},{"id":"income","op":"compute","expr":"eggs_left * 2"},{"op":"check","expr":"income / 2 + 3 + 4 == 16","why":"eggs sold + eaten + baked = laid"}],"answer":"income","type":"integer"}
"#;

/// Configuration B: the LLM writes a plan, the SLM executes and checks it.
pub fn prompt_plans(ps: &[Problem], hints: &[(String, String)]) -> String {
    let mut s = String::from(PLAN_SPEC);
    s.push_str("\nProblems:\n\n");
    for p in ps {
        s.push_str(&problem_block(p));
        if let Some((_, h)) = hints.iter().find(|(id, _)| *id == p.id) {
            s.push_str(&format!("Hint from the engine: {h}\n"));
        }
        s.push('\n');
    }
    s
}

/// Retry after the gate: the previous plan, reasons for rejection.
pub fn prompt_retry(items: &[(Problem, String, Vec<String>)]) -> String {
    let mut s = String::from(PLAN_SPEC);
    s.push_str("\nYour previous plans for the problems below FAILED the engine's verification. Read the reasons, find the modelling or syntax mistake, and write a corrected plan for each (same JSON-line format, same ids). Do not just delete the failing check — a failed check usually means the model of the problem is wrong.\n\n");
    for (p, plan, fails) in items {
        s.push_str(&problem_block(p));
        s.push_str(&format!("Previous plan: {plan}\nEngine verdict: {}\n\n", fails.join(" | ")));
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(kind: &str, gold: &str, opts: &[(&str, &str)]) -> Problem {
        Problem { id: "t".into(), level: 0, source: String::new(), license: String::new(), split: String::new(), question: String::new(), options: opts.iter().map(|(a, b)| (a.to_string(), b.to_string())).collect(), gold: gold.into(), gold_kind: kind.into(), topic: String::new() }
    }

    #[test]
    fn grading() {
        assert!(grade(&p("int", "18", &[]), Some("18"), None));
        assert!(grade(&p("int", "18", &[]), Some("$18.00"), None));
        assert!(!grade(&p("int", "18", &[]), Some("17"), None));
        assert!(grade(&p("number", "3.1416", &[]), Some("3.14159"), None));
        let o = [("A", "$$3$$ "), ("B", "$$4$$ ")];
        assert!(grade(&p("option", "B", &o), Some("4"), None));
        assert!(grade(&p("option", "B", &o), Some("x"), Some("B")));
        assert!(!grade(&p("option", "B", &o), Some("3"), None));
        let xs: Vec<u32> = (0..100).collect();
        let a = sample(&xs, |x| x.to_string(), 10, "s");
        assert_eq!(a, sample(&xs, |x| x.to_string(), 10, "s"), "sample is reproducible");
    }
}
