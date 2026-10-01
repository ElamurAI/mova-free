//! Quantity layer of the world (30.09): who has how much of what and what changes — for word problems.
//!
//! The story world holds individual objects; here — quantities: state `entity.thing → number`, changes are anchored to
//! sentences (`@N`) and labelled with a principle (`k=`), as advised by Chi, Feltovich, Glaser 1981 (an expert groups by
//! principle, not by surface). Executed by the exact core (`math::big`, rational numbers). The LLM writes a script,
//! the gate passes only one that gives the reference answer and takes numbers from the text; the MMM learns on verified scripts.
//!
//! ```text
//! @1 set janet.egg = 16            k=total
//! @2 sub janet.egg = 3             k=loss
//! @3 set janet.$ = janet.egg * 2   k=rate
//! @4 ask janet.$
//! ```

use std::collections::{BTreeMap, HashMap};

use anyhow::{Result, bail};
use math::big::*;

pub const KINDS: &[&str] = &["total", "gain", "loss", "transfer", "rate", "compare", "part", "convert", "combine", "unit", "given"];
pub const VERBS: &[&str] = &["set", "add", "sub", "mul", "div", "move", "ask"];
/// Constants usable without a number in the text (time units, percent, "half").
pub const IMPLIED: &[i64] = &[2, 3, 4, 7, 10, 12, 24, 30, 52, 60, 100, 365, 1000];

#[derive(Clone, Debug, PartialEq)]
pub enum Ex {
    Num(Q, String),
    Ref(String),
    Bin(char, Box<Ex>, Box<Ex>),
}

#[derive(Clone, Debug)]
pub struct Cmd {
    pub sent: usize,
    pub verb: String,
    pub target: String,
    pub from: Option<String>,
    pub expr: Ex,
    pub kind: String,
    pub line: String,
}

// ---------- expressions ----------

fn toks(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let cs: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < cs.len() {
        let c = cs[i];
        if c.is_whitespace() {
            i += 1;
        } else if "+-*/()".contains(c) {
            out.push(c.to_string());
            i += 1;
        } else {
            let st = i;
            while i < cs.len() && !cs[i].is_whitespace() && !"+-*/()".contains(cs[i]) {
                i += 1;
            }
            out.push(cs[st..i].iter().collect());
        }
    }
    out
}

struct P {
    t: Vec<String>,
    i: usize,
}

impl P {
    fn peek(&self) -> Option<&str> {
        self.t.get(self.i).map(|s| s.as_str())
    }
    fn expr(&mut self) -> Result<Ex> {
        let mut l = self.term()?;
        while let Some(op) = self.peek().filter(|o| *o == "+" || *o == "-").map(|o| o.chars().next().unwrap()) {
            self.i += 1;
            l = Ex::Bin(op, Box::new(l), Box::new(self.term()?));
        }
        Ok(l)
    }
    fn term(&mut self) -> Result<Ex> {
        let mut l = self.atom()?;
        while let Some(op) = self.peek().filter(|o| *o == "*" || *o == "/").map(|o| o.chars().next().unwrap()) {
            self.i += 1;
            l = Ex::Bin(op, Box::new(l), Box::new(self.atom()?));
        }
        Ok(l)
    }
    fn atom(&mut self) -> Result<Ex> {
        let Some(t) = self.t.get(self.i).cloned() else { bail!("expression cut off") };
        self.i += 1;
        if t == "(" {
            let e = self.expr()?;
            if self.peek() != Some(")") {
                bail!("missing «)»");
            }
            self.i += 1;
            return Ok(e);
        }
        if let Some(v) = parse_q(&t.replace(',', "")) {
            return Ok(Ex::Num(v, t));
        }
        if t.contains('.') && t.split('.').all(|p| !p.is_empty()) {
            return Ok(Ex::Ref(t.to_lowercase()));
        }
        bail!("unrecognized element «{t}»")
    }
}

pub fn parse_expr(s: &str) -> Result<Ex> {
    let mut p = P { t: toks(s), i: 0 };
    let e = p.expr()?;
    if p.i != p.t.len() {
        bail!("extra at end of expression: «{}»", p.t[p.i..].join(" "));
    }
    Ok(e)
}

fn eval(e: &Ex, st: &HashMap<String, Q>) -> Result<Q> {
    Ok(match e {
        Ex::Num(v, _) => v.clone(),
        Ex::Ref(r) => match st.get(r) {
            Some(v) => v.clone(),
            None => bail!("«{r}» has no value yet"),
        },
        Ex::Bin(op, a, b) => {
            let (x, y) = (eval(a, st)?, eval(b, st)?);
            match op {
                '+' => x + y,
                '-' => x - y,
                '*' => x * y,
                _ => {
                    if y == q(0) {
                        bail!("division by zero");
                    }
                    x / y
                }
            }
        }
    })
}

fn nums_of(e: &Ex, out: &mut Vec<(Q, String)>) {
    match e {
        Ex::Num(v, s) => out.push((v.clone(), s.clone())),
        Ex::Ref(_) => {}
        Ex::Bin(_, a, b) => {
            nums_of(a, out);
            nums_of(b, out);
        }
    }
}

// ---------- script ----------

pub fn parse_line(line: &str) -> Result<Cmd> {
    let l = line.trim();
    let Some(rest) = l.strip_prefix('@') else { bail!("line without «@N»") };
    let (n, rest) = rest.split_once(char::is_whitespace).unwrap_or((rest, ""));
    let sent: usize = n.parse().map_err(|_| anyhow::anyhow!("«@{n}» is not a sentence number"))?;
    let rest = rest.trim();
    // k= at the end (optional for ask)
    let (body, kind) = match rest.rsplit_once(" k=") {
        Some((b, k)) => (b.trim(), k.trim().to_string()),
        None => (rest, String::new()),
    };
    let (verb, body) = body.split_once(char::is_whitespace).unwrap_or((body, ""));
    if !VERBS.contains(&verb) {
        bail!("unknown action «{verb}»");
    }
    if verb == "ask" {
        return Ok(Cmd { sent, verb: verb.into(), target: String::new(), from: None, expr: parse_expr(body)?, kind, line: l.into() });
    }
    if !KINDS.contains(&kind.as_str()) {
        bail!("principle k=«{kind}» is not in the set {KINDS:?}");
    }
    let Some((lhs, rhs)) = body.split_once('=') else { bail!("missing «=»") };
    let lhs = lhs.trim().to_lowercase();
    let (from, target) = if verb == "move" {
        let Some((a, b)) = lhs.split_once("->") else { bail!("move: need «A.x -> B.x»") };
        (Some(a.trim().to_string()), b.trim().to_string())
    } else {
        (None, lhs)
    };
    for r in from.iter().chain(std::iter::once(&target)) {
        if !(r.contains('.') && r.split('.').all(|p| !p.is_empty())) {
            bail!("«{r}»: need «entity.thing»");
        }
    }
    Ok(Cmd { sent, verb: verb.into(), target, from, expr: parse_expr(rhs.trim())?, kind, line: l.into() })
}

/// Sentences of the problem (boundary: «. ? !» before a space or at the end; «$2.50» is not split).
pub fn sentences(text: &str) -> Vec<String> {
    let cs: Vec<char> = text.chars().collect();
    let mut out = Vec::new();
    let mut cur = String::new();
    for (i, &c) in cs.iter().enumerate() {
        cur.push(c);
        if matches!(c, '.' | '?' | '!') && cs.get(i + 1).is_none_or(|n| n.is_whitespace()) {
            let s = cur.trim().to_string();
            if !s.is_empty() {
                out.push(s);
            }
            cur.clear();
        }
    }
    if !cur.trim().is_empty() {
        out.push(cur.trim().to_string());
    }
    out
}

/// Numbers in a sentence's text (digits, words, percent as a fraction).
pub fn sent_numbers(s: &str) -> Vec<Q> {
    math::arith::quantities(s).0.into_iter().filter(|x| !x.name.starts_with("const")).flat_map(|x| {
        // «20%» yields both 0.2 and 20 — the script may write either
        let mut v = vec![x.exact.clone()];
        if x.name.ends_with('%') {
            v.push(x.exact.clone() * q(100));
        }
        v
    }).collect()
}

#[derive(Debug)]
pub struct Run {
    pub answer: Option<Q>,
    pub state: BTreeMap<String, Q>,
    pub errors: Vec<String>,
}

/// Run the script and check the gates: the sentence exists; every number in an expression comes from the text of this or
/// earlier sentences (or from the constant set, or 1/2… as a fraction of them); only existing state is changed; there is an ask.
pub fn run(cmds: &[Cmd], sents: &[String]) -> Run {
    let mut st: HashMap<String, Q> = HashMap::new();
    let mut errors = Vec::new();
    let mut answer = None;
    let mut seen: Vec<Q> = Vec::new();
    let mut upto = 0;
    for c in cmds {
        if c.sent == 0 || c.sent > sents.len() {
            errors.push(format!("{}: sentence @{} does not exist (there are {})", c.line, c.sent, sents.len()));
            continue;
        }
        while upto < c.sent {
            seen.extend(sent_numbers(&sents[upto]));
            upto += 1;
        }
        let mut ns = Vec::new();
        nums_of(&c.expr, &mut ns);
        for (v, s) in &ns {
            let ok = seen.contains(v) || IMPLIED.iter().any(|k| q(*k) == *v) || *v == q(1) || *v == q(0) || IMPLIED.iter().any(|k| q(1) / q(*k) == *v);
            if !ok {
                errors.push(format!("{}: number {s} not from the text up to sentence @{}", c.line, c.sent));
            }
        }
        let val = match eval(&c.expr, &st) {
            Ok(v) => v,
            Err(e) => {
                errors.push(format!("{}: {e}", c.line));
                continue;
            }
        };
        let cur = |k: &str, st: &HashMap<String, Q>| st.get(k).cloned();
        match c.verb.as_str() {
            "ask" => answer = Some(val),
            "set" => {
                st.insert(c.target.clone(), val);
            }
            "move" => {
                let f = c.from.clone().unwrap();
                match cur(&f, &st) {
                    Some(x) => {
                        st.insert(f, x - val.clone());
                    }
                    None => errors.push(format!("{}: transfer from «{f}», which does not exist yet", c.line)),
                }
                let t = cur(&c.target, &st).unwrap_or(q(0));
                st.insert(c.target.clone(), t + val);
            }
            v => match cur(&c.target, &st) {
                None => errors.push(format!("{}: «{}» has no value yet — set first", c.line, c.target)),
                Some(x) => {
                    let y = match v {
                        "add" => x + val,
                        "sub" => x - val,
                        "mul" => x * val,
                        _ => {
                            if val == q(0) {
                                errors.push(format!("{}: division by zero", c.line));
                                continue;
                            }
                            x / val
                        }
                    };
                    st.insert(c.target.clone(), y);
                }
            },
        }
    }
    if answer.is_none() {
        errors.push("no ask".into());
    }
    Run { answer, state: st.into_iter().collect(), errors }
}

pub fn parse_script(text: &str) -> (Vec<Cmd>, Vec<String>) {
    let mut cmds = Vec::new();
    let mut errs = Vec::new();
    for l in text.lines() {
        let t = l.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        match parse_line(t) {
            Ok(c) => cmds.push(c),
            Err(e) => errs.push(format!("{t}: {e}")),
        }
    }
    (cmds, errs)
}

/// Check a script against the reference: (answer, errors). The script is green when there are no errors and answer = reference.
pub fn check(script: &str, question: &str, gold: &Q) -> (Option<Q>, Vec<String>) {
    let (cmds, mut errs) = parse_script(script);
    let sents = sentences(question);
    let r = run(&cmds, &sents);
    errs.extend(r.errors);
    if let Some(a) = &r.answer {
        if a != gold {
            errs.push(format!("answer {} ≠ reference {}", show(a), show(gold)));
        }
    }
    (r.answer, errs)
}

// ---------- LLM prompt ----------

pub const RULES: &str = r#"You translate math word problems into a QUANTITY WORLD script. The script is executed exactly by a program; you never compute numbers yourself.

State: named quantities `entity.thing` (lowercase, no spaces; e.g. `janet.egg`, `robe.blue_bolt`, `house.value`, `tom.$`, `trip.hour`). Each command is anchored to the sentence it comes from (`@N`, sentences numbered below) and labelled with the PRINCIPLE it applies (`k=`).

Commands (one per line):
  @N set   E.x = EXPR           k=KIND   -- E has/is EXPR of x (introduce or overwrite)
  @N add   E.x = EXPR           k=KIND   -- E.x increases by EXPR
  @N sub   E.x = EXPR           k=KIND   -- E.x decreases by EXPR
  @N mul   E.x = EXPR           k=KIND   -- E.x is multiplied by EXPR
  @N div   E.x = EXPR           k=KIND   -- E.x is divided by EXPR
  @N move  A.x -> B.x = EXPR    k=transfer -- EXPR of x goes from A to B
  @N ask   EXPR                          -- the question (last line)
EXPR: numbers, `entity.thing` references, + - * / and parentheses.

KIND (the principle, not the surface words):
  given    -- a quantity stated directly
  total    -- sum of parts / altogether
  gain     -- someone gets more
  loss     -- someone uses, eats, spends, loses
  transfer -- goes from one owner to another
  rate     -- per unit: price, speed, per day/each (quantity = rate * count)
  compare  -- more/less than, times as many, half as much
  part     -- fraction or percent of a whole
  convert  -- change of unit (hours->minutes, dozens->items)
  combine  -- other combination of known quantities
  unit     -- unit value from total and count (total / count)

Rules:
- Every number in EXPR must appear in the text of that sentence or an earlier one (digits or words like "three", "half" = 1/2 or 2, "twice" = 2, "20%" = 20/100), or be a unit constant 2 3 4 7 10 12 24 30 52 60 100 365 1000.
- Anchor each command to the sentence whose words justify it. Several commands per sentence are fine; a sentence may have none.
- Prefer small steps: one principle per command. Don't collapse the whole solution into one expression.
- Name quantities by meaning so a reader can follow the world.
- The final line is `ask` with an expression over the state.

Example (invented problem):
Problem X
[1] Mia bakes 24 cookies every day.
[2] She gives 5 to her brother and eats 3.
[3] She sells the rest for $1.50 each.
[4] How much money does she earn in a week?
=== X
@1 set mia.cookie = 24                      k=given
@2 move mia.cookie -> brother.cookie = 5    k=transfer
@2 sub mia.cookie = 3                        k=loss
@3 set mia.$ = mia.cookie * 1.50            k=rate
@4 set mia.week_$ = mia.$ * 7               k=convert
@4 ask mia.week_$

Answer ONLY with blocks `=== <id>` followed by the script, one block per problem, nothing else.
"#;

pub fn prompt(items: &[(String, String)]) -> String {
    let mut s = String::from(RULES);
    s.push_str("\nProblems:\n");
    for (id, qn) in items {
        s.push_str(&format!("\nProblem {id}\n"));
        for (i, t) in sentences(qn).iter().enumerate() {
            s.push_str(&format!("[{}] {t}\n", i + 1));
        }
    }
    s
}

/// Split the LLM reply into `=== id` blocks.
pub fn split_blocks(out: &str) -> HashMap<String, String> {
    let mut m = HashMap::new();
    let mut cur: Option<String> = None;
    let mut buf = String::new();
    for l in out.lines() {
        if let Some(id) = l.trim().strip_prefix("===") {
            if let Some(c) = cur.take() {
                m.insert(c, std::mem::take(&mut buf));
            }
            cur = Some(id.trim().to_string());
        } else if cur.is_some() && !l.trim().starts_with("```") {
            buf.push_str(l);
            buf.push('\n');
        }
    }
    if let Some(c) = cur {
        m.insert(c, buf);
    }
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    const Q: &str = "Mia bakes 24 cookies every day. She gives 5 to her brother and eats 3. She sells the rest for $1.50 each. How much money does she earn in a week?";

    #[test]
    fn example_green() {
        let s = RULES.split("=== X\n").nth(1).unwrap().split("\n\n").next().unwrap();
        let (a, e) = check(s, Q, &q(168));
        assert!(e.is_empty(), "{e:?}");
        assert_eq!(a, Some(q(168)));
    }

    #[test]
    fn negative_controls_red() {
        // foreign number
        let (_, e) = check("@1 set mia.cookie = 25 k=given\n@4 ask mia.cookie", Q, &q(25));
        assert!(e.iter().any(|x| x.contains("not from the text")), "{e:?}");
        // number from a later sentence
        let (_, e) = check("@1 set mia.cookie = 5 k=given\n@4 ask mia.cookie", Q, &q(5));
        assert!(e.iter().any(|x| x.contains("not from the text")), "{e:?}");
        // wrong answer
        let (_, e) = check("@1 set mia.cookie = 24 k=given\n@4 ask mia.cookie", Q, &q(168));
        assert!(e.iter().any(|x| x.contains("≠ reference")), "{e:?}");
        // changing something that does not exist
        let (_, e) = check("@2 sub mia.cookie = 3 k=loss\n@4 ask mia.cookie", Q, &q(0));
        assert!(e.iter().any(|x| x.contains("set first")), "{e:?}");
        // unknown principle
        let (_, e) = check("@1 set mia.cookie = 24 k=magic\n@4 ask mia.cookie", Q, &q(24));
        assert!(!e.is_empty());
        // no ask
        let (_, e) = check("@1 set mia.cookie = 24 k=given", Q, &q(24));
        assert!(e.iter().any(|x| x.contains("no ask")));
    }
}

// ---------- datasets and LLM calls ----------

#[derive(Clone, Debug)]
pub struct Item {
    pub id: String,
    pub question: String,
    pub gold: Q,
}

fn raw_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| "data".into())).join("raw")
}

fn gold_q(v: &serde_json::Value) -> Option<Q> {
    match v {
        serde_json::Value::Number(n) => parse_q(&n.to_string()),
        serde_json::Value::String(s) => parse_q(&s.replace(',', "")),
        _ => None,
    }
}

/// Problem set: `svamp-train`, `svamp-test` (MIT), `gsm8k-train`, `gsm8k-test` (MIT, from the Scientist's socratic copy).
pub fn load_set(name: &str) -> Result<Vec<Item>> {
    let (file, kind) = match name {
        "svamp-train" => ("mwpdata-svamp/svamp-train.json", "svamp"),
        "svamp-test" => ("mwpdata-svamp/svamp-test.json", "svamp"),
        "gsm8k-train" => ("mwpdata-gsm8k-socratic/gsm8k-socratic-train.json", "gsm8k"),
        "gsm8k-test" => ("mwpdata-gsm8k-socratic/gsm8k-socratic-test.json", "gsm8k"),
        _ => bail!("unknown set «{name}»"),
    };
    let v: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(raw_dir().join(file))?)?;
    let arr = v.as_array().ok_or_else(|| anyhow::anyhow!("{file}: not an array"))?;
    Ok(arr.iter().enumerate().filter_map(|(i, x)| match kind {
        "svamp" => {
            let body = x["Body"].as_str()?.trim();
            let sep = if body.ends_with(['.', '?', '!']) { " " } else { ". " };
            Some(Item { id: format!("{name}-{}", x["ID"].as_str().unwrap_or(&i.to_string())), question: format!("{body}{sep}{}", x["Question"].as_str()?.trim()), gold: gold_q(&x["Answer"])? })
        }
        _ => {
            let a = x["answer"].as_str()?;
            Some(Item { id: format!("{name}-{i}"), question: x["question"].as_str()?.to_string(), gold: parse_q(&a.rsplit("####").next()?.trim().replace(',', ""))? })
        }
    }).collect())
}

/// Script records: one JSON line per problem.
pub fn load_done(path: &std::path::Path) -> HashMap<String, serde_json::Value> {
    std::fs::read_to_string(path).unwrap_or_default().lines().filter_map(|l| serde_json::from_str::<serde_json::Value>(l).ok()).filter_map(|v| Some((v["id"].as_str()?.to_string(), v))).collect()
}

/// The LLM writes scripts in batches; the gate checks each; green and red go to `scripts.jsonl`, accounting to `calls.jsonl`.
/// One retry per batch — only for the red ones, with the errors explained.
pub fn vmm_run(set: &str, dir: &std::path::Path, limit: usize, batch: usize, par: usize, cap: f64) -> Result<()> {
    use prag::opus::{Opus, append_call, read_calls};
    use std::sync::Mutex;
    std::fs::create_dir_all(dir)?;
    let out = dir.join("scripts.jsonl");
    let calls = dir.join("calls.jsonl");
    let done = load_done(&out);
    let items: Vec<Item> = load_set(set)?.into_iter().filter(|x| !done.contains_key(&x.id)).take(limit).collect();
    let spent0: f64 = read_calls(&calls).map(|c| c.iter().map(|x| x.cost_usd).sum()).unwrap_or(0.0);
    eprintln!("{set}: {} problems; spent before start ${spent0:.2} of cap ${cap:.2}", items.len());
    let opus = Opus::from_env(dir.join("cwd"));
    let lock = Mutex::new(spent0);
    let chunks: Vec<&[Item]> = items.chunks(batch).collect();
    let next = std::sync::atomic::AtomicUsize::new(0);
    let (green, red) = (std::sync::atomic::AtomicUsize::new(0), std::sync::atomic::AtomicUsize::new(0));
    std::thread::scope(|s| {
        for _ in 0..par {
            s.spawn(|| loop {
                let k = next.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                let Some(ch) = chunks.get(k) else { break };
                if *lock.lock().unwrap() >= cap {
                    eprintln!("cap ${cap:.2} reached — stopping");
                    break;
                }
                let pairs: Vec<(String, String)> = ch.iter().map(|x| (x.id.clone(), x.question.clone())).collect();
                let res = opus.ask(&format!("quant {set}"), k, ch.len(), &prompt(&pairs));
                let (text, call) = match res {
                    Ok(r) => r,
                    Err(e) => {
                        eprintln!("batch {k}: call error — {e}");
                        continue;
                    }
                };
                let _ = append_call(&calls, &call);
                *lock.lock().unwrap() += call.cost_usd;
                let blocks = split_blocks(&text);
                let mut first: Vec<(Item, String, Vec<String>)> = Vec::new();
                for it in *ch {
                    let sc = blocks.get(&it.id).cloned().unwrap_or_default();
                    let (_, errs) = check(&sc, &it.question, &it.gold);
                    let errs = if sc.trim().is_empty() { vec!["no script".to_string()] } else { errs };
                    first.push((it.clone(), sc, errs));
                }
                // retry only the red ones
                let bad: Vec<&(Item, String, Vec<String>)> = first.iter().filter(|x| !x.2.is_empty()).collect();
                let mut fixed: HashMap<String, (String, Vec<String>)> = HashMap::new();
                if !bad.is_empty() {
                    let mut p = prompt(&bad.iter().map(|x| (x.0.id.clone(), x.0.question.clone())).collect::<Vec<_>>());
                    p.push_str("\nYour previous scripts for these problems failed the checker (the true answer is not shown). Errors:\n");
                    for (it, sc, errs) in &bad {
                        p.push_str(&format!("=== {}\n{}-- errors: {}\n", it.id, sc, errs.iter().filter(|e| !e.contains("reference")).cloned().chain(errs.iter().any(|e| e.contains("reference")).then(|| "the final answer is wrong".to_string())).collect::<Vec<_>>().join("; ")));
                    }
                    p.push_str("\nWrite corrected scripts (same format, all problems above).\n");
                    if let Ok((t2, c2)) = opus.ask(&format!("quant {set} retry"), k, bad.len(), &p) {
                        let _ = append_call(&calls, &c2);
                        *lock.lock().unwrap() += c2.cost_usd;
                        for (id, sc) in split_blocks(&t2) {
                            if let Some(it) = bad.iter().find(|x| x.0.id == id) {
                                let (_, errs) = check(&sc, &it.0.question, &it.0.gold);
                                fixed.insert(id, (sc, errs));
                            }
                        }
                    }
                }
                let mut text_out = String::new();
                for (it, sc, errs) in first {
                    let (sc, errs, round) = match fixed.remove(&it.id) {
                        Some((s2, e2)) if e2.is_empty() => (s2, e2, 2),
                        _ => (sc, errs, 1),
                    };
                    if errs.is_empty() { green.fetch_add(1, std::sync::atomic::Ordering::SeqCst); } else { red.fetch_add(1, std::sync::atomic::Ordering::SeqCst); }
                    let rec = serde_json::json!({"id": it.id, "set": set, "question": it.question, "gold": show(&it.gold), "script": sc, "ok": errs.is_empty(), "errors": errs, "round": round});
                    text_out.push_str(&rec.to_string());
                    text_out.push('\n');
                }
                let _g = lock.lock().unwrap();
                use std::io::Write;
                if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&out) {
                    let _ = f.write_all(text_out.as_bytes());
                }
            });
        }
    });
    let spent: f64 = read_calls(&calls).map(|c| c.iter().map(|x| x.cost_usd).sum()).unwrap_or(0.0);
    println!("{set}: green {}, red {}; spent ${:.2} (this run ${:.2})", green.into_inner(), red.into_inner(), spent, spent - spent0);
    Ok(())
}

/// Summary: green scripts by principle and command verb.
pub fn stats(dir: &std::path::Path) -> Result<()> {
    let done = load_done(&dir.join("scripts.jsonl"));
    let mut by_set: BTreeMap<String, (usize, usize)> = BTreeMap::new();
    let mut kinds: BTreeMap<String, usize> = BTreeMap::new();
    let mut verbs: BTreeMap<String, usize> = BTreeMap::new();
    for v in done.values() {
        let ok = v["ok"].as_bool().unwrap_or(false);
        let e = by_set.entry(v["set"].as_str().unwrap_or("").to_string()).or_default();
        e.1 += 1;
        if ok {
            e.0 += 1;
            for c in parse_script(v["script"].as_str().unwrap_or("")).0 {
                *kinds.entry(c.kind.clone()).or_default() += 1;
                *verbs.entry(c.verb.clone()).or_default() += 1;
            }
        }
    }
    for (s, (g, n)) in &by_set {
        println!("{s}: green {g}/{n}");
    }
    println!("principles: {kinds:?}");
    println!("actions: {verbs:?}");
    Ok(())
}
