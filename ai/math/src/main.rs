//! CLI of the math module:
//!   math eval "<question>"    — expression, steps, checks, answer;
//!   math test <file.tsv>      — run a set (id, question, expected), summary by category;
//!   math parse "<sentence>"…  — UD tree over placeholders (debugging);
//! `en` model: MATH_EN_MODEL or data/en/models/ud-ewt-eslspok.bin.

use std::path::PathBuf;
use std::time::Instant;

use anyhow::{Context, Result, bail};
use math::eval::{answer, fmt_q};
use math::suite::{self, Verdict};
use math::understand::{MODEL, Understander};

fn model() -> PathBuf {
    PathBuf::from(std::env::var("MATH_EN_MODEL").unwrap_or_else(|_| MODEL.to_string()))
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(cmd) = args.first() else { bail!("usage: math eval \"<question>\" | math test <file.tsv> | math parse \"<sentence>\"") };
    let t0 = Instant::now();
    let u = Understander::load(&model()).with_context(|| format!("model {}", model().display()))?;
    let load = t0.elapsed();
    match cmd.as_str() {
        "eval" => {
            for q in &args[1..] {
                eval_one(&u, q);
            }
        }
        "test" => {
            let path = args.get(1).context("math test <file.tsv>")?;
            test(&u, path)?;
            eprintln!("model: {:.2} s", load.as_secs_f64());
        }
        "parse" => {
            for q in &args[1..] {
                let toks: Vec<String> = q.split_whitespace().map(String::from).collect();
                let s = u.parse_sentence(&toks);
                println!("# {q}");
                for (k, it) in s.items.iter().enumerate() {
                    println!("{k}\t{}\t{:?}\thead={:?}\t{}\t{}\tcase={:?}{}\tcond={}\tdelta={}", it.text, it.kind, it.head, it.rel, it.upos, it.case, if it.case_tree { "(tree)" } else { "" }, it.cond, it.delta);
                }
            }
        }
        _ => bail!("unknown command {cmd}"),
    }
    Ok(())
}

fn eval_one(u: &Understander, q: &str) {
    println!("question: {q}");
    let parsed = match u.understand(q) {
        Ok(p) => p,
        Err(e) => {
            println!("answer: {e}");
            return;
        }
    };
    println!("expr:    {}", parsed.expr);
    match answer(&parsed.expr) {
        Ok(a) => {
            println!("steps:");
            for (i, s) in a.steps.iter().enumerate() {
                println!("  {}. {}: {}  |  {}  |  = {}", i + 1, s.rule, s.formula, s.subst, s.result);
            }
            println!("checks:");
            for c in &a.checks {
                println!("  {} {}{}", if c.ok { "✓" } else { "✗" }, c.what, if c.detail.is_empty() { String::new() } else { format!(" — {}", c.detail) });
            }
            for n in &a.notes {
                println!("assumption: {n}");
            }
            let kind = if a.q.u.temp_delta().is_some() { "  (temperature difference)" } else { "" };
            println!("answer: {}{kind}", fmt_q(&a.q));
        }
        Err(math::err::Error::Overflow(op)) => {
            // v2: i128 overflowed — the exact core computes the same expression without units on big integers
            let src = parsed.expr.to_string();
            let big = math::calc::parse(&src).and_then(|e| {
                let mut env = math::calc::Env::new(math::nt::Limits::with_secs(10.0));
                let v = math::calc::eval(&e, &mut env)?;
                Ok((math::calc::independent(&e, &env.vars, &v), v))
            });
            match big {
                Ok((check, v)) => {
                    println!("steps:\n  1. i128 overflowed ({op}) → exact core v2 (big integers and rationals, dashu)");
                    if let Some(c) = check {
                        println!("checks:\n  {} {}{}", if c.ok { "✓" } else { "✗" }, c.name, if c.detail.is_empty() { String::new() } else { format!(" — {}", c.detail) });
                    }
                    println!("answer: {}", v.exact());
                }
                Err(_) => println!("answer: {}", math::err::Error::Overflow(op)),
            }
        }
        Err(e) => println!("answer: {e}"),
    }
    for t in &parsed.trace {
        println!("trace: {t}");
    }
    println!();
}

fn test(u: &Understander, path: &str) -> Result<()> {
    let text = std::fs::read_to_string(path).with_context(|| format!("reading {path}"))?;
    let t0 = Instant::now();
    let rep = suite::run(u, &text)?;
    for r in &rep.rows {
        let mark = match r.verdict {
            Verdict::Hit => "✓",
            Verdict::HonestNu => "?",
            Verdict::Wrong => "✗",
        };
        println!("{mark}\t{}\t{}\texpected {}\tgot {}", r.id, r.question, r.want, r.got);
    }
    println!("\n# category\thit\thonest «not understood»\twrong");
    for (cat, v) in &rep.by_cat {
        println!("# {cat}\t{}\t{}\t{}", v[0], v[1], v[2]);
    }
    println!("# TOTAL {}\t{}\t{}\t{}", rep.rows.len(), rep.total[0], rep.total[1], rep.total[2]);
    eprintln!("run: {} questions in {:.2} s", rep.rows.len(), t0.elapsed().as_secs_f64());
    Ok(())
}
