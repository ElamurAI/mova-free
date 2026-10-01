//! MMM coder v1 — motivated steps (design idea), two modes:
//! - **fast** — a ready-made tool at the top level: an mlab built-in (`sum`, `sort`, `find`…) or
//!   a verified program from memory (v0 template);
//! - **stepwise** — the same subgoals without a ready-made tool: loops, counters, insertion sort.
//!
//! Path (Pólya): understand (parse the description: data, conditions, reduction, format) → plan (differences between goal and state,
//! order — bigrams from mathematics; the move for each difference — a perceptron trained on the annotated
//! math v2 solutions) → carry out (each step is code with a "goal, motive" comment) → look back (two paths — an independent
//! check; if they agree, the gate is green; checking is done later by the control, no code is executed during training).

use crate::lemma::{self, Literals};
use crate::memory::Bm25;
use crate::motiv::Model;
use crate::tasks::Public;
use crate::v0::{self, Solution};

/// Switches for the architect's changes (each change is enabled separately — so the effect of each is visible).
#[derive(Clone, Debug, Default)]
pub struct Flags {
    pub names: Vec<String>,
}

impl Flags {
    pub fn on(&self, n: &str) -> bool {
        self.names.iter().any(|x| x == n)
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Step {
    pub(crate) diff: &'static str,
    pub(crate) kind: String,
    pub(crate) goal: String,
    pub(crate) motive: String,
    pub(crate) why: String,
    pub(crate) fast: Vec<String>,
    /// None — no stepwise variant (only a ready-made tool)
    pub(crate) step: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Val {
    Vec,
    Scalar,
    Str,
}

/// Understanding the task: what is given, what is required.
#[derive(Debug, Default, Clone)]
pub struct Spec {
    pub low: String,
    pub cues: Vec<String>,
    pub lit: Literals,
    pub data: Vec<(String, String)>,
    pub range: Option<(String, String)>,
    pub conds: Vec<(String, String)>,
    pub maps: Vec<(String, String)>,
    pub agg: Option<String>,
    pub order: Option<bool>,
    pub locate: Option<String>,
    pub strops: Vec<String>,
    pub parts: Option<String>,
    pub repeat: Option<String>,
    pub notes: Vec<String>,
}

fn num_after(words: &[String], i: usize) -> Option<String> {
    words.get(i).filter(|w| w.parse::<f64>().is_ok()).cloned()
}

fn has(low: &str, s: &str) -> bool {
    low.contains(s)
}

/// Words with numbers (for the patterns "from A to B", "divisible by K").
fn wordnums(text: &str) -> Vec<String> {
    let t = lemma::strip_literals(text).to_lowercase();
    t.split(|c: char| !(c.is_alphanumeric() || c == '.' || c == '-'))
        .map(|w| w.trim_end_matches('.').to_string())
        .filter(|w| !w.is_empty())
        .collect()
}

pub fn understand(desc: &str) -> Spec {
    let lit = lemma::literals(desc);
    let low = lemma::strip_literals(desc).to_lowercase();
    let cues = lemma::cues(desc);
    let w = wordnums(desc);
    let mut sp = Spec { low: low.clone(), cues, lit: lit.clone(), ..Default::default() };
    // data: named literals of the description
    for (name, val) in &lit.named {
        if val.starts_with('[') || val.starts_with('\'') || val.parse::<f64>().is_ok() {
            sp.data.push((name.clone(), val.clone()));
        }
    }
    if sp.data.is_empty() {
        if let Some(b) = lit.brackets.first() {
            sp.data.push(("v".into(), b.clone()));
        } else if let Some(s) = lit.strings.first() {
            sp.data.push(("s".into(), format!("'{s}'")));
        }
    }
    // range
    for i in 0..w.len() {
        let a = &w[i];
        if (a == "from" || a == "between") && i + 3 < w.len() {
            if let (Some(x), Some(y)) = (num_after(&w, i + 1), num_after(&w, i + 3)) {
                if w[i + 2] == "to" || w[i + 2] == "and" || w[i + 2] == "through" {
                    sp.range = Some((x, y));
                    sp.notes.push(format!("range «{a} {} {} {}»", w[i + 1], w[i + 2], w[i + 3]));
                    break;
                }
            }
        }
        if (a == "integers" || a == "numbers" || a == "values" || a == "range") && i + 3 < w.len() {
            if let (Some(x), Some(y)) = (num_after(&w, i + 1), num_after(&w, i + 3)) {
                if w[i + 2] == "to" || w[i + 2] == "through" {
                    sp.range = Some((x, y));
                    sp.notes.push(format!("range «{a} {} {} {}»", w[i + 1], w[i + 2], w[i + 3]));
                    break;
                }
            }
        }
        if a == "first" {
            if let Some(n) = num_after(&w, i + 1) {
                if w.get(i + 2).is_some_and(|x| x.starts_with("positive") || x.starts_with("natural") || x.starts_with("integer") || x.starts_with("number")) {
                    sp.range = Some(("1".into(), n.clone()));
                    sp.notes.push(format!("range «first {n}»"));
                    break;
                }
            }
        }
        if (a == "below" || a == "under") && sp.data.is_empty() {
            if let Some(n) = num_after(&w, i + 1) {
                let m = n.parse::<f64>().unwrap_or(1.0) - 1.0;
                sp.range = Some(("1".into(), format!("{m}")));
                sp.notes.push(format!("range «below {n}»"));
                break;
            }
        }
        if a == "up" && w.get(i + 1).is_some_and(|x| x == "to") && sp.data.is_empty() {
            if let Some(n) = num_after(&w, i + 2) {
                sp.range = Some(("1".into(), n.clone()));
                sp.notes.push(format!("range «up to {n}»"));
                break;
            }
        }
    }
    // range written as a:b
    if sp.range.is_none() && sp.data.is_empty() {
        for tok in desc.split(|c: char| c.is_whitespace() || c == ',' || c == '(' || c == ')') {
            let tok = tok.trim_end_matches('.');
            if let Some((a, b)) = tok.split_once(':') {
                if a.parse::<i64>().is_ok() && b.parse::<i64>().is_ok() {
                    sp.range = Some((a.into(), b.into()));
                    sp.notes.push(format!("range «{tok}»"));
                    break;
                }
            }
        }
    }
    // selection conditions
    let x = "X";
    if has(&low, "perfect square") {
        sp.conds.push(("perfect square".into(), format!("sqrt({x}) == floor(sqrt({x}))")));
    }
    for i in 0..w.len() {
        let a = w[i].as_str();
        let next_num = |k: usize| num_after(&w, i + k);
        match a {
            "even" if !has(&low, "even number of") => sp.conds.push(("even".into(), format!("mod({x}, 2) == 0"))),
            "odd" => sp.conds.push(("odd".into(), format!("mod({x}, 2) == 1"))),
            "divisible" | "multiples" | "multiple" => {
                let off = if a == "divisible" { 2 } else { 2 };
                if let Some(k) = next_num(off) {
                    let not = i > 0 && (w[i - 1] == "not" || w[i - 1] == "non");
                    if w.get(i + off + 1).is_some_and(|t| t == "or") {
                        if let Some(k2) = next_num(off + 2) {
                            sp.conds.push((format!("divisible by {k} or {k2}"), format!("mod({x}, {k}) == 0 | mod({x}, {k2}) == 0")));
                            continue;
                        }
                    }
                    if w.get(i + off + 1).is_some_and(|t| t == "and") {
                        if let Some(k2) = next_num(off + 2) {
                            sp.conds.push((format!("divisible by {k} and {k2}"), format!("mod({x}, {k}) == 0 & mod({x}, {k2}) == 0")));
                            continue;
                        }
                    }
                    if not {
                        sp.conds.push((format!("not divisible by {k}"), format!("mod({x}, {k}) ~= 0")));
                    } else {
                        sp.conds.push((format!("divisible by {k}"), format!("mod({x}, {k}) == 0")));
                    }
                }
            }
            "prime" | "primes" if !has(&low, "prime factor") => sp.conds.push(("prime".into(), format!("isprime({x})"))),
            "positive" if !has(&low, "positive integers") && !has(&low, "positive integer") => sp.conds.push(("positive".into(), format!("{x} > 0"))),
            "negative" => sp.conds.push(("negative".into(), format!("{x} < 0"))),
            "greater" | "larger" | "more" | "bigger" | "exceed" | "exceeds" | "above" => {
                let k = if a == "exceed" || a == "exceeds" || a == "above" { next_num(1) } else if w.get(i + 1).is_some_and(|t| t == "than") { next_num(2) } else { None };
                let or_eq = w.get(i + 2).is_some_and(|t| t == "or") && w.get(i + 3).is_some_and(|t| t == "equal");
                let k = if or_eq { num_after(&w, i + 6) } else { k };
                if let Some(k) = k {
                    if !sp.data.is_empty() || sp.range.is_some() {
                        let op = if or_eq { ">=" } else { ">" };
                        sp.conds.push((format!("{a} than {k}"), format!("{x} {op} {k}")));
                    }
                }
            }
            "less" | "smaller" | "fewer" | "below" | "under" => {
                let k = if a == "below" || a == "under" { next_num(1) } else if w.get(i + 1).is_some_and(|t| t == "than") { next_num(2) } else { None };
                let or_eq = w.get(i + 2).is_some_and(|t| t == "or") && w.get(i + 3).is_some_and(|t| t == "equal");
                let k = if or_eq { num_after(&w, i + 6) } else { k };
                if let Some(k) = k {
                    if !sp.data.is_empty() && sp.range.as_ref().is_none_or(|(_, b)| b != &k) {
                        let op = if or_eq { "<=" } else { "<" };
                        sp.conds.push((format!("{a} than {k}"), format!("{x} {op} {k}")));
                    }
                }
            }
            "least" if i > 0 && w[i - 1] == "at" => {
                if let Some(k) = next_num(1) {
                    sp.conds.push((format!("at least {k}"), format!("{x} >= {k}")));
                }
            }
            "most" if i > 0 && w[i - 1] == "at" => {
                if let Some(k) = next_num(1) {
                    sp.conds.push((format!("at most {k}"), format!("{x} <= {k}")));
                }
            }
            "nonzero" | "non-zero" => sp.conds.push(("nonzero".into(), format!("{x} ~= 0"))),
            _ => {}
        }
    }
    sp.conds.dedup();
    // transforming each
    let each = has(&low, "each") || has(&low, "every") || has(&low, "all ");
    if (has(&low, "squares") || has(&low, "squared") || (each && has(&low, "square"))) && !has(&low, "perfect square") && !has(&low, "square root") {
        sp.maps.push(("square".into(), "X.^2".into()));
    }
    if has(&low, "cubes") || has(&low, "cubed") {
        sp.maps.push(("cube".into(), "X.^3".into()));
    }
    if has(&low, "square roots") || (each && has(&low, "square root")) {
        sp.maps.push(("square root".into(), "sqrt(X)".into()));
    }
    if has(&low, "absolute value") {
        sp.maps.push(("absolute value".into(), "abs(X)".into()));
    }
    if has(&low, "reciprocal") {
        sp.maps.push(("reciprocal".into(), "1 ./ X".into()));
    }
    if has(&low, "doubled") || has(&low, "double each") || has(&low, "double every") {
        sp.maps.push(("double".into(), "2 * X".into()));
    }
    // reduction
    let agg_rules: &[(&[&str], &str)] = &[
        (&["how many", "count the", "count of", "number of elements", "number of integers", "number of values", "number of entries", "counts"], "count"),
        (&["average", "mean"], "mean"),
        (&["median"], "median"),
        (&["product"], "prod"),
        (&["sum", "total"], "sum"),
        (&["largest", "maximum", "greatest", "biggest", "highest"], "max"),
        (&["smallest", "minimum", "lowest"], "min"),
    ];
    for (keys, a) in agg_rules {
        if keys.iter().any(|k| has(&low, k)) {
            sp.agg = Some(a.to_string());
            break;
        }
    }
    // search
    if has(&low, "index") || has(&low, "position") {
        if matches!(sp.agg.as_deref(), Some("max")) {
            sp.locate = Some("argmax".into());
            sp.agg = None;
        } else if matches!(sp.agg.as_deref(), Some("min")) {
            sp.locate = Some("argmin".into());
            sp.agg = None;
        } else if has(&low, "first") {
            sp.locate = Some("first".into());
        } else if has(&low, "last") {
            sp.locate = Some("last".into());
        }
    }
    // order
    if has(&low, "sort") || has(&low, "ascending") || has(&low, "descending") || has(&low, "increasing order") || has(&low, "decreasing order") {
        sp.order = Some(!(has(&low, "descending") || has(&low, "decreasing") || has(&low, "largest to smallest") || has(&low, "high to low")));
    }
    // strings
    let is_str = sp.data.iter().any(|(_, v)| v.starts_with('\''));
    if is_str {
        for (k, op) in [
            ("palindrome", "palindrome"),
            ("vowel", "vowels"),
            ("upper", "upper"),
            ("capital", "upper"),
            ("lower", "lower"),
            ("revers", "reverse"),
            ("replace", "replace"),
            ("trim", "trim"),
            ("occurrence", "countchar"),
            ("how many times", "countchar"),
            ("length", "length"),
            ("number of characters", "length"),
        ] {
            if has(&low, k) && !sp.strops.contains(&op.to_string()) {
                sp.strops.push(op.into());
            }
        }
    }
    // parts of a number
    if has(&low, "digit") && !is_str {
        sp.parts = Some("digits".into());
    }
    // repetition
    for (k, r) in [("factorial", "factorial"), ("fibonacci", "fibonacci"), ("greatest common divisor", "gcd"), ("gcd", "gcd"), ("least common multiple", "lcm"), ("lcm", "lcm"), ("collatz", "collatz")] {
        if has(&low, k) {
            sp.repeat = Some(r.into());
            break;
        }
    }
    sp
}

/// Motives for the code: the general rule of the difference + a quote from mathematics (the cell medoid).
pub(crate) fn motive(model: &Model, diff: &str, kind: &str) -> (String, String) {
    let code = match diff {
        "given" => "name the data first, so every later step can refer to it",
        "candidates" => "the answer lies among a finite list of numbers, so list them",
        "select" => "only some items qualify, so keep just those before going on",
        "transform" => "every item needs the same change, so apply it to all at once",
        "aggregate" => "many values must become one number",
        "order" => "the items must be arranged by size",
        "locate" => "we need where an item is, not the item itself",
        "represent" => "the text is not yet in the requested form",
        "parts" => "the answer depends on the parts (digits) of the number",
        "repeat" => "the value is built by repeating one rule step after step",
        "present" => "the answer must be shown exactly in the requested format",
        "verify" => "a second, independent way should give the same result",
        _ => "this step removes the remaining difference to the goal",
    };
    let math = model
        .motive_of(diff, kind)
        .map(|(t, id, n, c)| format!("math: \"{t}\" [{id} s{n}; {c} steps with {diff}→{kind}]"))
        .unwrap_or_else(|| "math: no annotated step with this difference→move".into());
    (code.to_string(), math)
}

fn subst(tpl: &str, x: &str) -> String {
    tpl.replace('X', x)
}

fn fmt_line(fmt: &str, arg: &str) -> String {
    format!("printf('{fmt}', {arg});")
}

/// Plan and steps for one description (or a description segment). `start` — a given value already named earlier (function body,
/// the next goal in a multi-goal task). An error gives the reason why there is no plan.
pub(crate) fn plan(desc: &str, sp: &Spec, model: &Model, start: Option<(String, Val)>) -> Result<Vec<Step>, String> {
    let mut steps: Vec<Step> = Vec::new();
    let mut diffs: Vec<String> = Vec::new();
    if !sp.data.is_empty() {
        diffs.push("given".into());
    }
    if sp.range.is_some() {
        diffs.push("candidates".into());
    }
    if !sp.conds.is_empty() {
        diffs.push("select".into());
    }
    if !sp.maps.is_empty() {
        diffs.push("transform".into());
    }
    if sp.order.is_some() {
        diffs.push("order".into());
    }
    if sp.agg.is_some() && sp.repeat.is_none() && (sp.strops.is_empty() || sp.strops == ["countchar"] || sp.strops == ["vowels"]) {
        diffs.push("aggregate".into());
    }
    if sp.locate.is_some() {
        diffs.push("locate".into());
    }
    if !sp.strops.is_empty() {
        diffs.push("represent".into());
    }
    if sp.parts.is_some() {
        diffs.push("parts".into());
    }
    if sp.repeat.is_some() {
        diffs.push("repeat".into());
    }
    if start.is_some() {
        diffs.retain(|d| d != "given");
    }
    let understood = diffs.iter().any(|d| d != "given");
    if !understood {
        return Err("no known difference between the given data and the goal was recognised".into());
    }
    diffs.push("present".into());
    let order = model.order(diffs);
    // the value flowing between steps
    let mut cur = start.as_ref().map(|s| s.0.clone()).unwrap_or_default();
    let mut val = start.as_ref().map(|s| s.1.clone()).unwrap_or(Val::Vec);
    let (mut prev, mut prev2) = ("^".to_string(), "^".to_string());
    let needs_value = |d: &str| matches!(d, "select" | "transform" | "order" | "aggregate" | "locate" | "represent");
    let mut broken = false;
    for d in &order {
        let d: &'static str = crate::motiv::DIFFS.iter().copied().find(|x| x == d).unwrap_or("present");
        if needs_value(d) && cur.is_empty() {
            broken = true;
            break;
        }
        let allowed: &[&str] = match d {
            "given" => &["Bind"],
            "candidates" => &["Range"],
            "select" => &["Filter", "Search"],
            "transform" => &["Map", "Convert"],
            "order" => &["Sort"],
            "aggregate" => &["Reduce"],
            "locate" => &["Search"],
            "represent" => &["Convert", "Map"],
            "parts" => &["Decompose"],
            "repeat" => &["Iterate"],
            _ => &["Output"],
        };
        let (kind, why) = model.choose(d, &prev, &prev2, &crate::motiv::question_cues(desc), allowed);
        let (mcode, mmath) = motive(model, d, &kind);
        let mut st = Step { diff: d, kind: kind.clone(), goal: String::new(), motive: format!("{mcode} ({mmath})"), why, fast: vec![], step: None };
        match d {
            "given" => {
                for (name, v) in &sp.data {
                    st.fast.push(format!("{name} = {v};"));
                }
                let (n0, v0) = &sp.data[0];
                cur = n0.clone();
                val = if v0.starts_with('\'') { Val::Str } else if v0.starts_with('[') { Val::Vec } else { Val::Scalar };
                st.goal = format!("name the given data ({})", sp.data.iter().map(|(n, _)| n.as_str()).collect::<Vec<_>>().join(", "));
                st.step = Some(st.fast.clone());
            }
            "candidates" => {
                let (a, b) = sp.range.clone().unwrap();
                cur = "v".into();
                val = Val::Vec;
                st.goal = format!("list the candidates {a}..{b}");
                st.fast.push(format!("v = {a}:{b};"));
                st.step = Some(vec![format!("v = [];"), format!("for k = {a}:{b}"), "  v(end+1) = k;".into(), "end".into()]);
            }
            "select" => {
                let (names, conds): (Vec<String>, Vec<String>) = sp.conds.iter().cloned().unzip();
                let cond_v = conds.iter().map(|c| format!("({})", subst(c, &cur))).collect::<Vec<_>>().join(" & ");
                let cond_x = conds.iter().map(|c| format!("({})", subst(c, "x"))).collect::<Vec<_>>().join(" && ");
                st.goal = format!("keep only the items that are {}", names.join(" and "));
                if sp.locate.as_deref() == Some("first") || sp.locate.as_deref() == Some("last") {
                    // first/last search — the condition goes into the search step
                    st.goal = format!("mark the items that are {}", names.join(" and "));
                    st.fast.push(format!("mask = {cond_v};"));
                    st.step = Some(vec![format!("mask = false(size({cur}));"), format!("for k = 1:numel({cur})"), format!("  x = {cur}(k);"), format!("  mask(k) = {cond_x};"), "end".into()]);
                } else {
                    st.fast.push(format!("{cur} = {cur}({cond_v});"));
                    st.step = Some(vec![
                        "w = [];".into(),
                        format!("for x = {cur}"),
                        format!("  if {cond_x}"),
                        "    w(end+1) = x;".into(),
                        "  end".into(),
                        "end".into(),
                        format!("{cur} = w;"),
                    ]);
                }
            }
            "transform" => {
                let (names, maps): (Vec<String>, Vec<String>) = sp.maps.iter().cloned().unzip();
                st.goal = format!("take the {} of every item", names.join(", then "));
                let mut e = cur.clone();
                let mut ex = format!("{cur}(k)");
                for m in &maps {
                    e = subst(m, &e);
                    ex = subst(&m.replace(".^", "^").replace("./", "/"), &ex);
                }
                st.fast.push(format!("{cur} = {e};"));
                st.step = Some(vec![format!("for k = 1:numel({cur})"), format!("  {cur}(k) = {ex};"), "end".into()]);
            }
            "order" => {
                let asc = sp.order.unwrap_or(true);
                st.goal = format!("arrange the items in {} order", if asc { "ascending" } else { "descending" });
                st.fast.push(if asc { format!("{cur} = sort({cur});") } else { format!("{cur} = sort({cur}, 'descend');") });
                let cmp = if asc { ">" } else { "<" };
                st.step = Some(vec![
                    format!("for i = 2:numel({cur})"),
                    format!("  x = {cur}(i);"),
                    "  j = i - 1;".into(),
                    format!("  while j >= 1 && {cur}(j) {cmp} x"),
                    format!("    {cur}(j + 1) = {cur}(j);"),
                    "    j = j - 1;".into(),
                    "  end".into(),
                    format!("  {cur}(j + 1) = x;"),
                    "end".into(),
                ]);
            }
            "aggregate" => {
                let a = sp.agg.clone().unwrap();
                let src = if val == Val::Str && !sp.strops.is_empty() { "hits".to_string() } else { cur.clone() };
                st.goal = format!("reduce the items to one number: {a}");
                let (f, s): (String, Vec<String>) = match a.as_str() {
                    "sum" => (format!("r = sum({src});"), vec!["r = 0;".into(), format!("for x = {src}"), "  r = r + x;".into(), "end".into()]),
                    "prod" => (format!("r = prod({src});"), vec!["r = 1;".into(), format!("for x = {src}"), "  r = r * x;".into(), "end".into()]),
                    "count" => (format!("r = numel({src});"), vec!["r = 0;".into(), format!("for x = {src}"), "  r = r + 1;".into(), "end".into()]),
                    "mean" => (
                        format!("r = mean({src});"),
                        vec!["t = 0;".into(), format!("for x = {src}"), "  t = t + x;".into(), "end".into(), format!("r = t / numel({src});")],
                    ),
                    "max" | "min" => {
                        let c = if a == "max" { ">" } else { "<" };
                        (format!("r = {a}({src});"), vec![format!("r = {src}(1);"), format!("for x = {src}(2:end)"), format!("  if x {c} r"), "    r = x;".into(), "  end".into(), "end".into()])
                    }
                    _ => (format!("r = {a}({src});"), vec![]),
                };
                st.fast.push(f);
                st.step = if s.is_empty() { None } else { Some(s) };
                cur = "r".into();
                val = Val::Scalar;
            }
            "locate" => {
                let l = sp.locate.clone().unwrap();
                st.goal = format!("find the position ({l})");
                match l.as_str() {
                    "argmax" | "argmin" => {
                        let f = if l == "argmax" { "max" } else { "min" };
                        let c = if l == "argmax" { ">" } else { "<" };
                        st.fast.push(format!("[~, r] = {f}({cur});"));
                        st.step = Some(vec!["r = 1;".into(), format!("for k = 2:numel({cur})"), format!("  if {cur}(k) {c} {cur}(r)"), "    r = k;".into(), "  end".into(), "end".into()]);
                    }
                    _ => {
                        let has_mask = !sp.conds.is_empty();
                        let m = if has_mask { "mask".to_string() } else { format!("{cur} ~= 0") };
                        let dir = if l == "first" { "1" } else { "1, 'last'" };
                        st.fast.push(format!("r = find({m}, {dir});"));
                        let rng = if l == "first" { format!("1:numel({cur})") } else { format!("numel({cur}):-1:1") };
                        let test = if has_mask { "mask(k)".to_string() } else { format!("{cur}(k) ~= 0") };
                        st.step = Some(vec!["r = 0;".into(), format!("for k = {rng}"), format!("  if {test}"), "    r = k;".into(), "    break;".into(), "  end".into(), "end".into()]);
                    }
                }
                cur = "r".into();
                val = Val::Scalar;
            }
            "represent" => {
                let op = sp.strops[0].clone();
                st.goal = format!("bring the text to the requested form ({op})");
                match op.as_str() {
                    "upper" | "lower" => {
                        let (lo, hi, delta) = if op == "upper" { ("'a'", "'z'", "- 32") } else { ("'A'", "'Z'", "+ 32") };
                        st.fast.push(format!("{cur} = {op}({cur});"));
                        st.step = Some(vec![format!("for k = 1:numel({cur})"), format!("  if {cur}(k) >= {lo} && {cur}(k) <= {hi}"), format!("    {cur}(k) = char({cur}(k) {delta});"), "  end".into(), "end".into()]);
                        val = Val::Str;
                    }
                    "reverse" => {
                        st.fast.push(format!("{cur} = fliplr({cur});"));
                        st.step = Some(vec!["t = '';".into(), format!("for k = numel({cur}):-1:1"), format!("  t(end+1) = {cur}(k);"), "end".into(), format!("{cur} = t;")]);
                    }
                    "trim" => st.fast.push(format!("{cur} = strtrim({cur});")),
                    "replace" if sp.lit.strings.len() >= 3 => {
                        let a = &sp.lit.strings[1];
                        let b = &sp.lit.strings[2];
                        st.fast.push(format!("{cur} = strrep({cur}, '{a}', '{b}');"));
                    }
                    "palindrome" => {
                        st.fast.push(format!("r = isequal({cur}, fliplr({cur}));"));
                        st.step = Some(vec!["r = true;".into(), format!("n = numel({cur});"), "for k = 1:floor(n / 2)".into(), format!("  if {cur}(k) ~= {cur}(n + 1 - k)"), "    r = false;".into(), "    break;".into(), "  end".into(), "end".into()]);
                        cur = "r".into();
                        val = Val::Scalar;
                    }
                    "vowels" => {
                        st.fast.push(format!("t = lower({cur});"));
                        st.fast.push("hits = t(t == 'a' | t == 'e' | t == 'i' | t == 'o' | t == 'u');".into());
                        st.step = Some(vec![format!("t = lower({cur});"), "hits = '';".into(), "for c = t".into(), "  if any(c == 'aeiou')".into(), "    hits(end+1) = c;".into(), "  end".into(), "end".into()]);
                        if sp.agg.is_none() {
                            st.fast.push("r = numel(hits);".into());
                            cur = "r".into();
                            val = Val::Scalar;
                        }
                    }
                    "countchar" => {
                        let c = sp.lit.strings.get(1).cloned().unwrap_or_else(|| " ".into());
                        st.fast.push(format!("hits = {cur}({cur} == '{c}');"));
                        st.step = Some(vec!["hits = '';".into(), format!("for c = {cur}"), format!("  if c == '{c}'"), "    hits(end+1) = c;".into(), "  end".into(), "end".into()]);
                        if sp.agg.is_none() {
                            st.fast.push("r = numel(hits);".into());
                            cur = "r".into();
                            val = Val::Scalar;
                        }
                    }
                    "length" => {
                        st.fast.push(format!("r = length({cur});"));
                        cur = "r".into();
                        val = Val::Scalar;
                    }
                    _ => st.fast.push(format!("% (no operator for «{op}» yet)")),
                }
            }
            "parts" => {
                let n = if val == Val::Scalar && !cur.is_empty() { cur.clone() } else { sp.lit.numbers.first().map(|x| x.1.clone()).unwrap_or("0".into()) };
                st.goal = "split the number into its decimal digits".into();
                st.fast.push(format!("d = num2str({n}) - '0';"));
                st.step = Some(vec!["d = [];".into(), format!("m = {n};"), "while m > 0".into(), "  d = [mod(m, 10) d];".into(), "  m = floor(m / 10);".into(), "end".into()]);
                cur = "d".into();
                val = Val::Vec;
                if sp.agg.is_none() {
                    st.fast.push("r = numel(d);".into());
                }
            }
            "repeat" => {
                let r = sp.repeat.clone().unwrap();
                let nums: Vec<String> = sp.lit.numbers.iter().map(|x| x.1.clone()).collect();
                let a = if val == Val::Scalar && !cur.is_empty() { cur.clone() } else { nums.first().cloned().unwrap_or("1".into()) };
                let b = nums.get(1).cloned().unwrap_or("1".into());
                st.goal = format!("build the value by repeating one rule ({r})");
                match r.as_str() {
                    "factorial" => {
                        st.fast.push(format!("r = factorial({a});"));
                        st.step = Some(vec!["r = 1;".into(), format!("for k = 2:{a}"), "  r = r * k;".into(), "end".into()]);
                    }
                    "gcd" | "lcm" => {
                        st.fast.push(format!("r = {r}({a}, {b});"));
                        let mut s: Vec<String> = vec![format!("p = {a};"), format!("q = {b};"), "while q ~= 0".into(), "  t = mod(p, q);".into(), "  p = q;".into(), "  q = t;".into(), "end".into()];
                        s.push(if r == "gcd" { "r = p;".into() } else { format!("r = {a} * {b} / p;") });
                        st.step = Some(s);
                    }
                    "fibonacci" => {
                        let lines = vec!["f = [1 1];".into(), format!("for k = 3:{a}"), "  f(k) = f(k-1) + f(k-2);".into(), "end".into(), format!("r = f({a});")];
                        st.fast = lines.clone();
                        st.step = Some(lines);
                    }
                    "collatz" => {
                        let lines = vec![format!("m = {a};"), "r = 0;".into(), "while m ~= 1".into(), "  if mod(m, 2) == 0".into(), "    m = m / 2;".into(), "  else".into(), "    m = 3 * m + 1;".into(), "  end".into(), "  r = r + 1;".into(), "end".into()];
                        st.fast = lines.clone();
                        st.step = Some(lines);
                    }
                    _ => {}
                }
                cur = "r".into();
                val = Val::Scalar;
            }
            _ => {
                // presentation
                let fmt = sp.lit.formats.first().cloned();
                st.goal = "print the answer in the requested format".into();
                let call = sp.lit.prints.iter().find(|p| p.contains('\'')).cloned();
                let mut line = None;
                if let Some(c) = &call {
                    // printf('%d\n', count) — our result has the same name
                    let inner = c.split_once(',').map(|(_, r)| r.trim_end_matches(')').trim().to_string());
                    if let Some(arg) = inner {
                        if arg.chars().all(|ch| ch.is_alphanumeric() || ch == '_') && !arg.is_empty() && arg != cur && !cur.is_empty() {
                            st.fast.push(format!("{arg} = {cur};"));
                        }
                        if arg.chars().all(|ch| ch.is_alphanumeric() || ch == '_') {
                            line = Some(format!("{c};"));
                        }
                    }
                }
                let line = line.unwrap_or_else(|| {
                    let f = fmt.clone().unwrap_or_else(|| if val == Val::Str { "%s\\n".into() } else { "%d\\n".into() });
                    fmt_line(&f, &cur)
                });
                st.fast.push(line);
                st.step = Some(st.fast.clone());
            }
        }
        prev2 = prev;
        prev = kind;
        steps.push(st);
    }
    if broken {
        return Err("a step needs data, but no given data or range was recognised".into());
    }
    Ok(steps)
}

fn fallback(t: &Public, bm: &Bm25, why: &str) -> Solution {
    let mut s = v0::solve(t, bm);
    s.arch = "v1".into();
    s.program = format!("% No motivated plan: {why}.\n% Fast path only: nearest verified program from memory.\n{}", s.program);
    s.used = vec!["v1:fallback-v0".into()];
    s
}

pub fn solve(t: &Public, bm: &Bm25, model: &Model, flags: &Flags) -> Solution {
    if let Some(s) = crate::arch2::try_changes(t, bm, model, flags) {
        return s;
    }
    let sp = understand(&t.description);
    let steps = match plan(&t.description, &sp, model, None) {
        Ok(s) => s,
        Err(why) => return fallback(t, bm, &why),
    };
    finish(t, &sp, steps, vec!["v1:motivated".into()])
}

/// Assembling the two programs (fast and stepwise paths) from the steps, and the explanation.
pub(crate) fn finish(t: &Public, sp: &Spec, steps: Vec<Step>, mut used: Vec<String>) -> Solution {
    // assembling the two programs
    let render = |fast: bool| -> Option<String> {
        let mut out = String::new();
        out.push_str(&format!("% Goal: {}\n", t.description.chars().take(100).collect::<String>().replace('\n', " ")));
        for (i, s) in steps.iter().enumerate() {
            let lines = if fast { Some(&s.fast) } else { s.step.as_ref() };
            let lines = match lines {
                Some(l) => l,
                None if !fast => &s.fast,
                None => return None,
            };
            out.push_str(&format!("% Step {} ({} -> {}). Goal: {}. Motive: {}.\n", i + 1, s.diff, s.kind, s.goal, s.motive.split(" (math:").next().unwrap_or("")));
            for l in lines {
                out.push_str(l);
                out.push('\n');
            }
        }
        Some(out)
    };
    let fast = render(true).unwrap_or_default();
    let has_step = steps.iter().any(|s| s.step.as_ref().is_some_and(|l| l != &s.fast));
    let stepwise = if has_step { render(false) } else { None };
    let mut expl = String::from("Motivated plan (Polya: understand, plan, carry out, look back). Understood: ");
    expl.push_str(&sp.notes.join("; "));
    if !sp.conds.is_empty() {
        expl.push_str(&format!(" conditions: {};", sp.conds.iter().map(|c| c.0.clone()).collect::<Vec<_>>().join(", ")));
    }
    for (i, s) in steps.iter().enumerate() {
        expl.push_str(&format!("\n{}. [{} -> {}] goal: {}; motive: {}; choice: {}", i + 1, s.diff, s.kind, s.goal, s.motive, s.why));
    }
    expl.push_str(&format!(
        "\nLook back: {}",
        if stepwise.is_some() { "a second, stepwise path (loops instead of ready builtins) computes the same subgoals; both must agree." } else { "no independent stepwise path for these steps." }
    ));
    if stepwise.is_some() {
        used.push("v1:two-paths".into());
    }
    Solution { task: t.id.clone(), arch: "v1".into(), program: fast, explanation: expl, stepwise, used }
}
