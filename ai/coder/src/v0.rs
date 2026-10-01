//! MMM coder v0 — templates: nearest memory example (BM25 over lemmas) → adaptation (names, constants, format,
//! built-ins of the same type) → a small search over variants with static scoring (no execution).

use serde::{Deserialize, Serialize};

use crate::lemma;
use crate::memory::Bm25;
use crate::tasks::Public;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Solution {
    pub task: String,
    /// architecture version (not shown to the judge)
    pub arch: String,
    pub program: String,
    pub explanation: String,
    /// stepwise path (v1): motivated steps without a ready-made tool at the top level
    #[serde(default)]
    pub stepwise: Option<String>,
    /// which components fired (for tracking the architect's changes)
    #[serde(default)]
    pub used: Vec<String>,
}

/// Description words → mlab built-ins (transparent table, shared by v0 and v1).
pub const LEX: &[(&str, &[&str])] = &[
    ("sum", &["sum"]), ("total", &["sum"]), ("add", &["sum"]), ("product", &["prod"]), ("multiply", &["prod"]),
    ("average", &["mean"]), ("mean", &["mean"]), ("median", &["median"]), ("maximum", &["max"]), ("max", &["max"]),
    ("large", &["max"]), ("largest", &["max"]), ("big", &["max"]), ("greatest", &["max"]), ("minimum", &["min"]),
    ("min", &["min"]), ("small", &["min"]), ("smallest", &["min"]), ("least", &["min"]), ("count", &["nnz", "numel"]),
    ("number", &["numel"]), ("many", &["nnz", "numel"]), ("sort", &["sort"]), ("ascend", &["sort"]), ("ascending", &["sort"]),
    ("descend", &["sort"]), ("descending", &["sort"]), ("order", &["sort"]), ("reverse", &["fliplr"]), ("unique", &["unique"]),
    ("distinct", &["unique"]), ("index", &["find"]), ("position", &["find"]), ("find", &["find"]), ("even", &["mod"]),
    ("odd", &["mod"]), ("divisible", &["mod"]), ("remainder", &["mod"]), ("multiple", &["mod"]), ("prime", &["isprime"]),
    ("root", &["sqrt"]), ("sqrt", &["sqrt"]), ("absolute", &["abs"]), ("round", &["round"]), ("floor", &["floor"]),
    ("ceil", &["ceil"]), ("factorial", &["factorial"]), ("upper", &["upper"]), ("uppercase", &["upper"]),
    ("capital", &["upper"]), ("lower", &["lower"]), ("lowercase", &["lower"]), ("replace", &["strrep"]), ("trim", &["strtrim"]),
    ("length", &["length"]), ("cumulative", &["cumsum"]), ("running", &["cumsum"]), ("digit", &["num2str"]),
    ("string", &["sprintf"]), ("transpose", &["transpose"]), ("determinant", &["det"]), ("inverse", &["inv"]),
    ("table", &["table"]), ("group", &["groupsummary"]), ("row", &["sum"]), ("column", &["sum"]), ("gcd", &["gcd"]),
    ("divisor", &["gcd", "mod"]), ("lcm", &["lcm"]), ("power", &["power"]), ("square", &["sqrt"]), ("standard", &["std"]),
    ("deviation", &["std"]), ("substring", &["strfind"]), ("occurrence", &["strfind"]), ("character", &["double"]),
];

/// Families of built-ins of the same type: substitution within a family.
pub const FAMILIES: &[&[&str]] = &[
    &["sum", "prod", "max", "min", "mean", "median", "numel", "nnz", "std", "cumsum", "cumprod"],
    &["sqrt", "abs", "floor", "ceil", "round", "exp", "log", "fix"],
    &["upper", "lower", "fliplr", "strtrim"],
    &["sort", "unique"],
];

pub fn lex_builtins(lemmas: &[String]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for l in lemmas {
        for (w, bs) in LEX {
            if l == w {
                for b in *bs {
                    if !out.iter().any(|x| x == b) {
                        out.push(b.to_string());
                    }
                }
            }
        }
    }
    out
}

fn fmt_num(v: f64) -> String {
    if v.fract() == 0.0 && v.abs() < 1e15 { format!("{}", v as i64) } else { format!("{v}") }
}

/// Numeric literals of the code in order of first appearance (via the lexer).
fn code_numbers(code: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    if let Ok(toks) = mlab::lexer::lex(code) {
        for t in toks {
            if let mlab::lexer::Tok::Num(v, _) = t.tok {
                let s = fmt_num(v);
                if !out.contains(&s) {
                    out.push(s);
                }
            }
        }
    }
    out
}

/// Replacing a whole word (identifier or number) outside strings.
pub fn replace_word(code: &str, from: &str, to: &str) -> String {
    let mut out = String::new();
    let cs: Vec<char> = code.chars().collect();
    let f: Vec<char> = from.chars().collect();
    let mut i = 0;
    let mut inq = false;
    while i < cs.len() {
        let c = cs[i];
        if c == '\'' && (i == 0 || !(cs[i - 1].is_alphanumeric() || cs[i - 1] == ')' || cs[i - 1] == ']' || cs[i - 1] == '\'')) {
            inq = true;
            out.push(c);
            i += 1;
            continue;
        }
        if inq {
            if c == '\'' {
                inq = false;
            }
            out.push(c);
            i += 1;
            continue;
        }
        let word_start = i == 0 || !(cs[i - 1].is_alphanumeric() || cs[i - 1] == '_' || cs[i - 1] == '.');
        if word_start && cs[i..].starts_with(&f) {
            let e = i + f.len();
            let word_end = e >= cs.len() || !(cs[e].is_alphanumeric() || cs[e] == '_' || (cs[e] == '.' && e + 1 < cs.len() && cs[e + 1].is_ascii_digit()));
            if word_end {
                out.push_str(to);
                i = e;
                continue;
            }
        }
        out.push(c);
        i += 1;
    }
    out
}

pub fn parses(code: &str) -> bool {
    mlab::parser::parse_program(code).is_ok()
}

struct Cand {
    code: String,
    doc: usize,
    swaps: Vec<String>,
    score: f64,
    why: String,
}

/// v0: solution of a task with an explanation.
pub fn solve(t: &Public, bm: &Bm25) -> Solution {
    let lem = lemma::lemmas(&t.description);
    let lit = lemma::literals(&t.description);
    let builtins = lex_builtins(&lem);
    let mut query = lem.clone();
    query.extend(builtins.iter().cloned());
    let hits = bm.search(&query, 5);
    let task_nums: Vec<String> = lit.numbers.iter().map(|(v, _, _)| fmt_num(*v)).collect();
    let mut cands: Vec<Cand> = Vec::new();
    for h in hits.iter().take(3) {
        let d = &bm.docs[h.doc];
        for swap in [false, true] {
            let mut code = d.code.clone();
            let mut notes: Vec<String> = Vec::new();
            // constants: code numbers → task numbers in order of appearance
            let cn = code_numbers(&code);
            for (a, b) in cn.iter().zip(task_nums.iter()) {
                if a != b {
                    code = replace_word(&code, a, &format!("__N{b}__"));
                    notes.push(format!("{a}→{b}"));
                }
            }
            code = code.replace("__N", "").replace("__", "");
            // vector and string from the description
            if let (Some(tb), Some(p)) = (lit.brackets.first(), code.find('[')) {
                if let Some(e) = code[p..].find(']') {
                    let old = code[p..p + e + 1].to_string();
                    code = code.replacen(&old, tb, 1);
                    notes.push(format!("{old}→{tb}"));
                }
            }
            if let Some(fmt) = lit.formats.first() {
                for key in ["printf('", "fprintf('"] {
                    if let Some(p) = code.find(key) {
                        let s = p + key.len();
                        if let Some(e) = code[s..].find('\'') {
                            let old = code[s..s + e].to_string();
                            if old != *fmt {
                                code = format!("{}{}{}", &code[..s], fmt, &code[s + e..]);
                                notes.push(format!("format '{old}'→'{fmt}'"));
                            }
                            break;
                        }
                    }
                }
            }
            let mut swaps = Vec::new();
            if swap {
                for fam in FAMILIES {
                    let used: Vec<&str> = fam.iter().copied().filter(|b| d.idents.iter().any(|x| x == b)).collect();
                    let want: Vec<&str> = fam.iter().copied().filter(|b| builtins.iter().any(|x| x == b)).collect();
                    if let (Some(u), Some(w)) = (used.first(), want.first()) {
                        if u != w && !used.contains(w) {
                            code = replace_word(&code, u, w);
                            swaps.push(format!("{u}→{w}"));
                        }
                    }
                }
                if swaps.is_empty() {
                    continue;
                }
            }
            let ok = parses(&code);
            let ids = crate::memory::idents(&code);
            let cov_b = builtins.iter().filter(|b| ids.contains(b)).count() as f64;
            let cov_n = task_nums.iter().filter(|n| code_numbers(&code).contains(n)).count() as f64;
            let score = h.score + 2.0 * cov_b + 1.0 * cov_n + if ok { 3.0 } else { 0.0 };
            let why = format!(
                "example {} ({}; BM25 {:.2}: {}); substitutions: {}; built-in swaps: {}; parses {}; coverage: built-ins {}/{}, constants {}/{}",
                d.id,
                d.source,
                h.score,
                h.parts.iter().take(4).map(|(t, s)| format!("{t} {s:.1}")).collect::<Vec<_>>().join(", "),
                if notes.is_empty() { "—".into() } else { notes.join(", ") },
                if swaps.is_empty() { "—".into() } else { swaps.join(", ") },
                if ok { "yes" } else { "no" },
                cov_b,
                builtins.len(),
                cov_n,
                task_nums.len()
            );
            cands.push(Cand { code, doc: h.doc, swaps, score, why });
        }
    }
    let n_c = cands.len();
    let best = cands.into_iter().max_by(|a, b| a.score.total_cmp(&b.score).then(b.doc.cmp(&a.doc)));
    match best {
        Some(c) => Solution {
            task: t.id.clone(),
            arch: "v0".into(),
            program: c.code,
            explanation: format!("Template: {} Variants tried: {n_c}; chosen by static score {:.2}.", c.why, c.score),
            stepwise: None,
            used: if c.swaps.is_empty() { vec!["v0:template".into()] } else { vec!["v0:template".into(), "v0:swap".into()] },
        },
        None => Solution {
            task: t.id.clone(),
            arch: "v0".into(),
            program: "% no similar example in memory\n".into(),
            explanation: "No memory example matched the task lemmas.".into(),
            stepwise: None,
            used: vec!["v0:none".into()],
        },
    }
}
