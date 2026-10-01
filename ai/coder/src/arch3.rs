//! Architect's changes after round 2 (`runs/…/rounds/r2/architect.md`), each behind its own switch:
//! - `b1:procedure` — translating the description's imperative sentences into statements (counter, "from … to" loop, while,
//!   if/otherwise, the verbs increase/halve/double/square/multiply/swap/return, formulas "v = …");
//! - `b2:literal-repair` — literals with bracket balancing (matrices with ";"), wider cues for structural operators:
//!   "from highest to lowest", "each row and each column", replacing matrix elements by a condition;
//! - `b3:func-harness` — test-call arguments with bracket balancing, arity check, fixed argument.

use crate::lemma;
use crate::motiv::Model;
use crate::tasks::Public;
use crate::v0::Solution;
use crate::v1::{self, Step};

fn step(diff: &'static str, kind: &str, goal: &str, motive: &str, fast: Vec<String>, stepw: Option<Vec<String>>) -> Step {
    Step { diff, kind: kind.into(), goal: goal.into(), motive: motive.into(), why: "architect change (round 2)".into(), fast, step: stepw }
}

fn is_ident(s: &str) -> bool {
    !s.is_empty() && s.chars().next().is_some_and(|c| c.is_ascii_alphabetic()) && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn is_atom(s: &str) -> bool {
    is_ident(s) || s.parse::<f64>().is_ok() || (s.contains('(') && s.ends_with(')'))
}

/// English condition → mlab expression.
fn cond(c: &str) -> Option<String> {
    let c = c.trim().trim_end_matches([',', '.', ':']).trim();
    let w: Vec<&str> = c.split_whitespace().collect();
    let pats: &[(&[&str], &str)] = &[
        (&["is", "not", "equal", "to"], "~="),
        (&["is", "equal", "to"], "=="),
        (&["equals"], "=="),
        (&["is", "greater", "than", "or", "equal", "to"], ">="),
        (&["is", "less", "than", "or", "equal", "to"], "<="),
        (&["is", "greater", "than"], ">"),
        (&["is", "larger", "than"], ">"),
        (&["is", "more", "than"], ">"),
        (&["exceeds"], ">"),
        (&["is", "less", "than"], "<"),
        (&["is", "smaller", "than"], "<"),
        (&["is", "at", "least"], ">="),
        (&["is", "at", "most"], "<="),
        (&["is", "divisible", "by"], "mod"),
    ];
    for (p, op) in pats {
        for i in 1..w.len() {
            if w.len() >= i + p.len() + 1 && &w[i..i + p.len()] == *p {
                let a = w[..i].join(" ");
                let b = w[i + p.len()..].join(" ");
                if !is_atom(&a) || !is_atom(&b) {
                    return None;
                }
                return Some(if *op == "mod" { format!("mod({a}, {b}) == 0") } else { format!("{a} {op} {b}") });
            }
        }
    }
    if w.len() == 3 && w[1] == "is" && is_atom(w[0]) {
        return match w[2] {
            "even" => Some(format!("mod({}, 2) == 0", w[0])),
            "odd" => Some(format!("mod({}, 2) == 1", w[0])),
            "positive" => Some(format!("{} > 0", w[0])),
            "negative" => Some(format!("{} < 0", w[0])),
            "zero" => Some(format!("{} == 0", w[0])),
            "nonzero" | "non-zero" => Some(format!("{} ~= 0", w[0])),
            _ => None,
        };
    }
    // symbolic condition: "n > 1", "a(i) <= p"
    if [">", "<", ">=", "<=", "==", "~=", "!="].iter().any(|o| w.contains(o)) && mlab::parser::parse_program(&format!("x__ = {c};\n")).is_ok() {
        return Some(c.to_string());
    }
    None
}

/// One action clause → a statement.
fn action(c: &str) -> Option<String> {
    let c = c.trim().trim_end_matches(['.', ',', ';']).trim();
    let w: Vec<&str> = c.split_whitespace().collect();
    if w.is_empty() {
        return None;
    }
    let low: Vec<String> = w.iter().map(|x| x.to_lowercase()).collect();
    let l = |i: usize| low.get(i).map(String::as_str).unwrap_or("");
    // formula "v = expr"
    if let Some(p) = c.find(" = ") {
        let lhs = c[..p].split_whitespace().last().unwrap_or("");
        let rhs = c[p + 3..].trim();
        if (is_ident(lhs) || (lhs.contains('(') && lhs.ends_with(')'))) && mlab::parser::parse_program(&format!("{lhs} = {rhs};\n")).is_ok() {
            return Some(format!("{lhs} = {rhs};"));
        }
    }
    match l(0) {
        "increase" | "increment" | "decrease" | "decrement" => {
            let v = w.get(1)?;
            let k = if l(2) == "by" { w.get(3).copied().unwrap_or("1") } else { "1" };
            let op = if l(0).starts_with("inc") { "+" } else { "-" };
            if is_atom(v) && is_atom(k) {
                return Some(format!("{v} = {v} {op} {k};"));
            }
        }
        "add" if l(2) == "to" => {
            let (k, v) = (w.get(1)?, w.get(3)?);
            if is_atom(k) && is_atom(v) {
                return Some(format!("{v} = {v} + {k};"));
            }
        }
        "multiply" | "divide" if l(2) == "by" => {
            let (v, k) = (w.get(1)?, w.get(3)?);
            let op = if l(0) == "multiply" { "*" } else { "/" };
            if is_atom(v) && is_atom(k) {
                return Some(format!("{v} = {v} {op} {k};"));
            }
        }
        "halve" => return w.get(1).filter(|v| is_atom(v)).map(|v| format!("{v} = {v} / 2;")),
        "double" => return w.get(1).filter(|v| is_atom(v)).map(|v| format!("{v} = 2 * {v};")),
        "square" => return w.get(1).filter(|v| is_atom(v)).map(|v| format!("{v} = {v} ^ 2;")),
        "swap" => {
            let a = w.get(1)?;
            let b = w.get(3)?;
            if (l(2) == "and" || l(2) == "with") && a.contains('(') && b.contains('(') {
                let (xa, ia) = a.trim_end_matches(')').split_once('(')?;
                let (xb, ib) = b.trim_end_matches(')').split_once('(')?;
                if xa == xb {
                    return Some(format!("{xa}([{ia} {ib}]) = {xa}([{ib} {ia}]);"));
                }
            }
        }
        "return" => return w.get(1).filter(|v| is_atom(v)).map(|v| format!("out = {v};")),
        _ => {}
    }
    // «keep a counter c starting at 0», «start with s = 0», «set x to 5», «initialize x to 5»
    if let Some(i) = low.iter().position(|x| x == "counter" || x == "accumulator" || x == "variable") {
        if low.first().map(String::as_str) == Some("keep") || low.first().map(String::as_str) == Some("use") {
            let v = w.get(i + 1).map(|x| x.trim_end_matches([',', '.', ';'])).filter(|v| is_ident(v))?;
            // "starting at K" in the same clause
            if let Some(j) = low.iter().position(|x| x == "starting" || x == "initially") {
                let k = if low.get(j + 1).map(String::as_str) == Some("at") { w.get(j + 2) } else { w.get(j + 1) };
                if let Some(k) = k.map(|k| k.trim_end_matches([',', '.', ';'])).filter(|k| is_atom(k)) {
                    return Some(format!("{v} = {k};"));
                }
            }
            return Some(format!("__keep {v}"));
        }
    }
    if let Some(i) = low.iter().position(|x| x == "starting" || x == "initialized" || x == "initially") {
        let v = w[..i].iter().rev().find(|x| is_ident(x) && !["counter", "accumulator", "variable", "a", "an", "keep", "the"].contains(&x.to_lowercase().as_str()));
        let k = w.get(i + 2).or_else(|| w.get(i + 1))?;
        let k = k.trim_end_matches([',', '.']);
        if let (Some(v), true) = (v, is_atom(k)) {
            return Some(format!("{v} = {k};"));
        }
        if is_atom(k) {
            return Some(format!("__start {k}"));
        }
    }
    if let Some(i) = low.iter().position(|x| x == "starting" || x == "initialized" || x == "initially") {
        let v = w[..i].iter().rev().find(|x| is_ident(x) && !["counter", "accumulator", "variable", "a", "an", "keep", "the"].contains(&x.to_lowercase().as_str()))?;
        let k = w.get(i + 2).or_else(|| w.get(i + 1))?;
        let k = k.trim_end_matches([',', '.']);
        if is_atom(k) {
            return Some(format!("{v} = {k};"));
        }
    }
    if (l(0) == "set" || l(0) == "initialize" || l(0) == "initialise") && l(2) == "to" {
        let (v, k) = (w.get(1)?, w.get(3)?);
        if is_ident(v) && is_atom(k) {
            return Some(format!("{v} = {k};"));
        }
    }
    None
}

/// Block header: a loop or a condition. Returns (opening line, rest of the clause after the comma).
fn opener(c: &str) -> Option<(String, String)> {
    let c = c.trim();
    let low = c.to_lowercase();
    let (head, rest) = match c.find(", ") {
        Some(p) => (&c[..p], c[p + 2..].to_string()),
        None => (c, String::new()),
    };
    let hw: Vec<&str> = head.split_whitespace().collect();
    let hl: Vec<String> = hw.iter().map(|x| x.to_lowercase()).collect();
    // for each i from a to b / for i from a down to b / for i = a..b
    if hl.first().map(String::as_str) == Some("for") || low.starts_with("go through") || low.starts_with("scan") || low.starts_with("loop over") {
        if let Some(f) = hl.iter().position(|x| x == "from") {
            let v = hw.get(f.checked_sub(1)?)?;
            let a = hw.get(f + 1)?;
            let down = hl.get(f + 2).map(String::as_str) == Some("down");
            let b = hw.get(f + if down { 4 } else { 3 })?;
            if is_ident(v) && is_atom(a) && is_atom(b.trim_end_matches([',', ':'])) {
                let b = b.trim_end_matches([',', ':']);
                return Some((if down { format!("for {v} = {a}:-1:{b}") } else { format!("for {v} = {a}:{b}") }, rest));
            }
        }
        if let Some(e) = hw.iter().position(|x| *x == "=") {
            let v = hw.get(e.checked_sub(1)?)?;
            let r = hw.get(e + 1)?.trim_end_matches([',', ':']);
            if let Some((a, b)) = r.split_once("..") {
                if is_ident(v) && is_atom(a) && is_atom(b) {
                    return Some((format!("for {v} = {a}:{b}"), rest));
                }
            }
        }
    }
    if hl.first().map(String::as_str) == Some("while") {
        return cond(&hw[1..].join(" ")).map(|c| (format!("while {c}"), rest));
    }
    if matches!(hl.first().map(String::as_str), Some("if" | "whenever" | "when")) {
        return cond(&hw[1..].join(" ")).map(|c| (format!("if {c}"), rest));
    }
    None
}

const CUES: &[&str] = &[
    "counter", "accumulator", "starting at", "for each", "go through", "scan", "while", "whenever", "otherwise", "increase", "decrease",
    "increment", "decrement", "halve", "swap", "multiply", "return",
];

/// b1: imperative sentences → statements. None — not translated with confidence (≥ 2 recognised clauses, ≥ 70% of clauses with cues).
pub fn transcribe(text: &str) -> Option<(Vec<String>, usize, usize)> {
    // "(starting at 1)" → " starting at 1 "; "While n > 0: if …" → two clauses
    let norm = text.replace(" (", ", ").replace(") ", ", ").replace("),", ",").replace(": ", "; ");
    let text = norm.as_str();
    let low = text.to_lowercase();
    if CUES.iter().filter(|c| low.contains(*c)).count() < 2 {
        return None;
    }
    let mut lines: Vec<String> = Vec::new();
    let (mut ok, mut cue_clauses) = (0usize, 0usize);
    for sent in text.split(". ") {
        let mut depth = 0;
        let mut clauses: Vec<String> = Vec::new();
        for part in sent.split("; ") {
            for p in part.split(", then ").flat_map(|x| x.split(" then ")) {
                clauses.push(p.to_string());
            }
        }
        let mut i = 0;
        while i < clauses.len() {
            let c = clauses[i].trim().to_string();
            i += 1;
            let has_cue = CUES.iter().any(|q| c.to_lowercase().contains(q)) || c.contains(" = ");
            if has_cue {
                cue_clauses += 1;
            }
            let cl = c.to_lowercase();
            if cl.starts_with("otherwise") {
                if depth > 0 {
                    lines.push("else".into());
                    let rest = c["otherwise".len()..].trim_start_matches([',', ' ']).to_string();
                    let acts: Vec<String> = rest.split(" and ").filter_map(action).collect();
                    if !acts.is_empty() {
                        for a in acts {
                            lines.push(format!("  {a}"));
                        }
                        ok += 1;
                    }
                }
                continue;
            }
            if let Some((open, rest)) = opener(&c) {
                lines.push(open);
                depth += 1;
                ok += 1;
                if !rest.is_empty() {
                    for r in rest.split(" and ") {
                        if let Some(a) = action(r) {
                            lines.push(format!("  {a}"));
                        }
                    }
                }
                continue;
            }
            let parts: Vec<&str> = c.split(" and ").collect();
            let acts: Vec<String> = parts.iter().filter_map(|x| action(x)).collect();
            if !acts.is_empty() && acts.len() == parts.len() {
                for a in acts {
                    lines.push(if depth > 0 { format!("  {a}") } else { a });
                }
                ok += 1;
            } else if let Some(a) = action(&c) {
                lines.push(if depth > 0 { format!("  {a}") } else { a });
                ok += 1;
            }
        }
        for _ in 0..depth {
            lines.push("end".into());
        }
    }
    // «keep a counter l» + «starting at 0» → «l = 0;»
    let mut merged: Vec<String> = Vec::new();
    let mut pending: Option<String> = None;
    for l in lines {
        let t = l.trim().to_string();
        if let Some(v) = t.strip_prefix("__keep ") {
            pending = Some(v.to_string());
            continue;
        }
        if let Some(k) = t.strip_prefix("__start ") {
            if let Some(v) = pending.take() {
                merged.push(format!("{v} = {k};"));
            }
            continue;
        }
        if let Some(v) = pending.take() {
            merged.push(format!("{v} = 0;"));
        }
        merged.push(l);
    }
    let lines = merged;
    if ok < 2 || (ok as f64) < 0.7 * cue_clauses as f64 {
        return None;
    }
    let code = lines.join("\n") + "\n";
    if mlab::parser::parse_program(&code).is_err() {
        return None;
    }
    Some((lines, ok, cue_clauses))
}

/// b1 for a script: data from the description + translation + printing from the description.
pub fn procedure_task(t: &Public) -> Option<Solution> {
    let (lines, ok, cues) = transcribe(&t.description)?;
    let sp = v1::understand(&t.description);
    let mut steps = Vec::new();
    let given: Vec<String> = sp.data.iter().filter(|(n, _)| !lines.iter().any(|l| l.starts_with(&format!("{n} = ")))).map(|(n, v)| format!("{n} = {v};")).collect();
    if !given.is_empty() {
        steps.push(step("given", "Bind", "name the given data", "name the data first, so every later step can refer to it", given.clone(), Some(given)));
    }
    steps.push(step("repeat", "Iterate", "carry out the procedure exactly as the task states it", "the task spells out the steps, so each clause becomes one statement in the same order", lines.clone(), Some(lines)));
    let prints: Vec<String> = sp.lit.prints.iter().filter(|p| p.contains('\'')).map(|p| format!("{p};")).collect();
    if prints.is_empty() {
        return None;
    }
    steps.push(step("present", "Output", "print the answer in the requested format", "the answer must be shown exactly in the requested format", prints.clone(), Some(prints)));
    let mut sol = v1::finish(t, &sp, steps, vec!["b1:procedure".into()]);
    sol.explanation = format!("Procedure transcription: {ok} of {cues} procedural clauses turned into statements.\n{}", sol.explanation);
    Some(sol)
}

/// b3: call arguments with bracket balancing (lists separated by ",", "and", "then" at depth 0).
pub fn call_args(tail: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut d = 0i32;
    let mut q = false;
    let cs: Vec<char> = tail.chars().collect();
    let mut i = 0;
    while i < cs.len() {
        let c = cs[i];
        if c == '\'' {
            q = !q;
        }
        if !q {
            if c == '[' || c == '(' {
                d += 1;
            }
            if c == ']' || c == ')' {
                d -= 1;
            }
        }
        if d == 0 && !q {
            let rest: String = cs[i..].iter().take(6).collect();
            if c == ',' || rest.starts_with(" and ") || rest.starts_with(" then ") {
                out.push(cur.trim().to_string());
                cur.clear();
                i += if c == ',' { 1 } else if rest.starts_with(" and ") { 5 } else { 6 };
                continue;
            }
            if c == '.' && (i + 1 >= cs.len() || cs[i + 1] == ' ') && !cur.trim().chars().last().is_some_and(|x| x.is_ascii_digit() && cs.get(i + 1).is_some_and(|n| n.is_ascii_digit())) {
                break;
            }
        }
        cur.push(c);
        i += 1;
    }
    if !cur.trim().is_empty() {
        out.push(cur.trim().to_string());
    }
    out.into_iter()
        .map(|a| a.trim_start_matches("and ").trim_start_matches("then ").trim().trim_end_matches([',', '.']).to_string())
        .map(|a| a.split(" this way").next().unwrap_or("").trim().to_string())
        .filter(|a| a.starts_with('[') || a.starts_with('\'') || a.parse::<f64>().is_ok() || a.split('^').all(|x| x.trim().parse::<f64>().is_ok()) || (a.contains('=') && a.split('=').nth(1).is_some_and(|r| r.trim().parse::<f64>().is_ok() || r.trim().starts_with('['))))
        .collect()
}

/// b3: function calls with an arity check; fixed argument — a given value from the description.
pub fn harness_calls(desc: &str, name: &str, params: &[String], fmt: &str) -> Option<Vec<String>> {
    let low = desc.to_lowercase();
    let triggers = [" compute ", " test it on ", " call it on ", " apply it to ", " call it with ", " for the inputs ", " for the queries ", " on the inputs ", " to the values ", " call it for ", " test it with ", " on "];
    // cues by priority: the first one that yields a non-empty argument list
    let args = triggers.iter().filter_map(|t| low.rfind(t).map(|p| call_args(&desc[p + t.len()..]))).find(|a| !a.is_empty())?;
    let data: Vec<(String, String)> = lemma::literals(desc).named.into_iter().filter(|(n, _)| !params.contains(n)).collect();
    let mut calls = Vec::new();
    let args: Vec<String> = args
        .iter()
        .map(|a| a.split_once('=').map(|(_, r)| r.trim().to_string()).unwrap_or_else(|| a.clone()))
        .flat_map(|a| if params.len() == 2 && a.contains('^') { a.split('^').map(|x| x.trim().to_string()).collect::<Vec<_>>() } else { vec![a] })
        .collect();
    if params.len() == 1 {
        for a in &args {
            calls.push(format!("printf('{fmt}', {name}({a}));"));
        }
    } else if args.len() % params.len() == 0 && params.len() > 1 && !(params.len() == 2 && data.iter().any(|(_, v)| v.starts_with('['))) {
        for ch in args.chunks(params.len()) {
            calls.push(format!("printf('{fmt}', {name}({}));", ch.join(", ")));
        }
    } else if params.len() == 2 {
        // fixed first argument — a given value from the description (vector)
        let (dn, _) = data.iter().find(|(_, v)| v.starts_with('['))?;
        for a in &args {
            calls.push(format!("printf('{fmt}', {name}({dn}, {a}));"));
        }
    } else {
        return None;
    }
    Some(calls)
}
