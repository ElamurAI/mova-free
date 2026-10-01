//! Fixing the pilot for rule v2 (an explicit answer is a verbatim span of an anchor sentence).
//! Explicit pilot pairs that fail the span gate (`gate::span_reason`) go to the LLM in batches: paragraph,
//! question, old answer → a verbatim span of one anchor sentence. If no sentence states it
//! directly, the LLM writes `implicit`: the pair keeps its old answer but becomes implicit.
//!
//! The new rows are a separate database run (`db qa --replaces <pilot run>`) with the same document,
//! paragraph and question number; `replaces` points to the old row, the old row gets `replaced_by` and is not
//! deleted. The source of pairs is the pilot's `qa.tsv`, i.e. exactly what is in the database.
//!
//! Output gate (line `item anchors span`):
//! - batch (format, one retry): no lines; an item number outside 1..M; junk above 20 %; more than
//!   a quarter of items without an answer;
//! - item: not 3 columns; bad anchors; empty span; span not from an anchor sentence (the same
//!   reasons as in `gate`); `implicit` while the old answer is verbatim in one sentence; repeated item;
//! - no answer for an item — the old row stays in force.

use std::collections::{BTreeMap, HashMap};
use std::fmt::Write as _;
use std::path::Path;

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::gate::{Level, Reject, Sents, anchors, span_reason, verbatim, words};
use crate::lemma::{self, Lemmas};
use crate::report::{calls_json, kind, pct};
use crate::run::{Job, final_raw, retry_path};
use crate::schema::{ExIm, QType};
use crate::select::{Para, load_paras};

/// Header of `qa.tsv` (as written by `qagen gate` and read by `db qa`).
pub const QA_HEADER: &str = "doc_key\tpara\tk\tqtype\tex_or_im\tanchors\tquestion\tanswer";

/// Fix batch size — paragraphs per batch by default.
pub const BATCH: usize = 40;

/// A row of the pilot's `qa.tsv`.
#[derive(Clone, Debug)]
pub struct QaRow {
    pub doc: String,
    pub para: i64,
    pub k: usize,
    pub qtype: QType,
    pub exim: ExIm,
    /// `sent_id` of the anchor sentences
    pub anchors: Vec<String>,
    pub question: String,
    pub answer: String,
}

pub fn read_qa(path: &Path) -> Result<Vec<QaRow>> {
    let text = std::fs::read_to_string(path).with_context(|| path.display().to_string())?;
    let mut lines = text.lines();
    if lines.next() != Some(QA_HEADER) {
        bail!("{}: header is not {QA_HEADER:?}", path.display());
    }
    let mut out = Vec::new();
    for (i, l) in lines.enumerate().filter(|(_, l)| !l.is_empty()) {
        let at = || format!("{}:{}", path.display(), i + 2);
        let c: Vec<&str> = l.split('\t').collect();
        if c.len() != 8 {
            bail!("{}: {} columns", at(), c.len());
        }
        out.push(QaRow {
            doc: c[0].into(),
            para: c[1].parse().with_context(at)?,
            k: c[2].parse().with_context(at)?,
            qtype: QType::parse(c[3]).with_context(at)?,
            exim: ExIm::parse(c[4]).with_context(at)?,
            anchors: c[5].split(',').map(str::to_string).collect(),
            question: c[6].into(),
            answer: c[7].into(),
        });
    }
    Ok(out)
}

/// An explicit pilot pair to fix.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    pub doc: String,
    pub para: i64,
    pub k: usize,
    pub qtype: String,
    pub question: String,
    pub answer: String,
    /// paragraph sentences (from 1)
    pub anchors: Vec<usize>,
    /// why it failed the span gate
    pub reason: String,
    #[serde(default)]
    pub batch: usize,
}

impl Item {
    pub fn key(&self) -> String {
        format!("{}#{}", self.doc, self.para)
    }
}

fn anchor_nums(p: &Para, ids: &[String]) -> Result<Vec<usize>> {
    ids.iter()
        .map(|id| p.sents.iter().position(|s| &s.sent_id == id).map(|i| i + 1).with_context(|| format!("{}: anchor {id} is not from the paragraph", p.key())))
        .collect()
}

/// Selection: explicit pilot pairs that fail the span gate → `items.jsonl` and their paragraphs
/// `paras.jsonl` (batches of `per` paragraphs in text order); implicit pilot pairs with lemmas of a single
/// anchor → `doubt.tsv`. Returns the summary (`select.json`).
pub fn select(pilot: &Path, dir: &Path, per: usize, l: &dyn Lemmas) -> Result<Value> {
    if per == 0 {
        bail!("--batch 0");
    }
    if dir.join("raw").join("b000.txt").exists() {
        bail!("{}: LLM outputs already exist — a new selection would shift the batches", dir.display());
    }
    let paras = load_paras(pilot)?;
    let order: HashMap<String, usize> = paras.iter().enumerate().map(|(i, p)| (p.key(), i)).collect();
    let rows = read_qa(&pilot.join("qa.tsv"))?;
    let mut items: Vec<(usize, Item)> = Vec::new();
    let mut by_reason: BTreeMap<String, usize> = BTreeMap::new();
    let mut doubt = String::from("doc_key\tpara\tk\tanchor\tshare\tquestion\tanswer\n");
    let (mut explicit, mut implicit, mut doubts) = (0usize, 0usize, 0usize);
    let mut sents_of: HashMap<String, Sents> = HashMap::new();
    for r in &rows {
        let key = format!("{}#{}", r.doc, r.para);
        let &i = order.get(&key).with_context(|| format!("{key}: paragraph missing from the pilot's paras.jsonl"))?;
        let p = &paras[i];
        let anchors = anchor_nums(p, &r.anchors)?;
        match r.exim {
            ExIm::Explicit => {
                explicit += 1;
                let s = sents_of.entry(key).or_insert_with(|| Sents::of(p));
                if let Some(reason) = span_reason(&r.answer, &s.norm, &s.words, &anchors) {
                    *by_reason.entry(kind(&reason)).or_default() += 1;
                    let it = Item { doc: r.doc.clone(), para: r.para, k: r.k, qtype: r.qtype.name().into(), question: r.question.clone(), answer: r.answer.clone(), anchors, reason, batch: 0 };
                    items.push((i, it));
                }
            }
            ExIm::Implicit => {
                implicit += 1;
                let texts: Vec<&str> = p.sents.iter().map(|s| s.text.as_str()).collect();
                if let Some((k, share)) = lemma::doubt(l, &texts, &anchors, &r.answer) {
                    doubts += 1;
                    let _ = writeln!(doubt, "{}\t{}\t{}\t{k}\t{share:.2}\t{}\t{}", r.doc, r.para, r.k, r.question, r.answer);
                }
            }
        }
    }
    items.sort_by_key(|(i, it)| (*i, it.k));
    // batches: paragraphs with items, `per` at a time, in text order
    let mut fix_paras: Vec<Para> = Vec::new();
    for (i, it) in items.iter_mut() {
        if fix_paras.last().is_none_or(|p| p.key() != it.key()) {
            let mut p = paras[*i].clone();
            p.batch = fix_paras.len() / per;
            fix_paras.push(p);
        }
        it.batch = fix_paras.last().unwrap().batch;
    }
    std::fs::create_dir_all(dir)?;
    let mut out = String::new();
    for (_, it) in &items {
        out.push_str(&serde_json::to_string(it)?);
        out.push('\n');
    }
    std::fs::write(dir.join("items.jsonl"), out)?;
    let mut out = String::new();
    for p in &fix_paras {
        out.push_str(&serde_json::to_string(p)?);
        out.push('\n');
    }
    std::fs::write(dir.join("paras.jsonl"), out)?;
    std::fs::write(dir.join("doubt.tsv"), doubt)?;
    let v = json!({
        "pilot": pilot.display().to_string(),
        "explicit": explicit, "span_ok": explicit - items.len(), "span_ok_pct": pct(explicit - items.len(), explicit),
        "to_fix": items.len(), "by_reason": by_reason,
        "paragraphs": fix_paras.len(), "batches": fix_paras.len().div_ceil(per), "per_batch": per,
        "implicit": implicit, "implicit_doubt": doubts, "implicit_doubt_pct": pct(doubts, implicit),
    });
    std::fs::write(dir.join("select.json"), serde_json::to_string_pretty(&v)?)?;
    Ok(v)
}

pub fn load_items(dir: &Path) -> Result<Vec<Item>> {
    let p = dir.join("items.jsonl");
    let text = std::fs::read_to_string(&p).with_context(|| format!("{} — run qagen fix select first", p.display()))?;
    text.lines().filter(|l| !l.trim().is_empty()).map(|l| Ok(serde_json::from_str(l)?)).collect()
}

/// Fix rules.
pub const FIX_RULES: &str = "Below are paragraphs of classic fairy tales and fables. After each paragraph come reading-comprehension items: a question with its type, anchor sentences and current answer. Each answer should be EXPLICIT, in the style of the FairytaleQA dataset: a verbatim span of the text. The current answers are not spans yet. For each item, rewrite the answer as a verbatim span:
- copy a contiguous part of ONE sentence of the paragraph: the same words in the same order and form, nothing changed, added, reordered or skipped inside the span; you may start and end the span anywhere in the sentence;
- choose the shortest span that fully answers the question and keeps the meaning of the current answer; at most 15 words when the sentence allows;
- do not replace pronouns with names inside the span, do not join parts of two sentences.
If no single sentence states the answer (the current answer is inferred, or it combines several sentences), write implicit instead of a span: the item keeps its current answer as an implicit (inferred) answer.

Output one line per item, 3 TAB-separated columns:
item\tanchors\tspan
item is the item number; anchors is the number of the sentence the span is copied from, optionally followed by other supporting sentences of the same paragraph, comma-separated (for example 3 or 3,2); span is the copied words, or the single word implicit (then write - for anchors).

Example (format only, not part of the task):
=== P1 - \"The Miller's Sons\", paragraph 1
[1] Next to the old mill lived a poor miller and his three sons.
[2] One morning the miller found that half of his flour had been stolen.
Item 1 (action; anchors 2): What did the miller find one morning? | current answer: someone stole half his flour
Item 2 (character; anchors 1,2): Who is the story about? | current answer: a poor miller who was robbed
Output:
1\t2\thalf of his flour had been stolen
2\t-\timplicit

Output ONLY these lines: no header, no explanations, no code fences.
";

/// Fix batches: paragraphs and items in batch order (items follow their paragraphs).
pub struct Fixes<'a> {
    pub all: BTreeMap<usize, (Vec<&'a Para>, Vec<&'a Item>)>,
}

impl<'a> Fixes<'a> {
    pub fn new(paras: &'a [Para], items: &'a [Item]) -> Fixes<'a> {
        let mut by_key: HashMap<String, Vec<&Item>> = HashMap::new();
        for it in items {
            by_key.entry(it.key()).or_default().push(it);
        }
        let mut all: BTreeMap<usize, (Vec<&Para>, Vec<&Item>)> = BTreeMap::new();
        for p in paras {
            let e = all.entry(p.batch).or_default();
            e.0.push(p);
            e.1.extend(by_key.get(&p.key()).into_iter().flatten().copied());
        }
        Fixes { all }
    }
}

/// Batch prompt: paragraphs with sentences and, under each, its items, numbered 1..M throughout.
pub fn prompt(paras: &[&Para], items: &[&Item]) -> String {
    let mut p = String::from(FIX_RULES);
    p.push_str("\nParagraphs and items:\n");
    let mut n = 0;
    for (i, a) in paras.iter().enumerate() {
        let by = if a.author.is_empty() { String::new() } else { format!(" ({})", a.author) };
        let _ = write!(p, "\n=== P{} - \"{}\"{by}, paragraph {}\n", i + 1, a.title, a.para);
        for (k, s) in a.sents.iter().enumerate() {
            let _ = writeln!(p, "[{}] {}", k + 1, s.text);
        }
        for it in items.iter().filter(|it| it.key() == a.key()) {
            n += 1;
            let anchors: Vec<String> = it.anchors.iter().map(usize::to_string).collect();
            let _ = writeln!(p, "Item {n} ({}; anchors {}): {} | current answer: {}", it.qtype, anchors.join(","), it.question, it.answer);
        }
    }
    p
}

impl Job for Fixes<'_> {
    fn step(&self) -> &'static str {
        "qagen-fix"
    }

    fn batches(&self) -> Vec<usize> {
        self.all.keys().copied().collect()
    }

    fn prompt(&self, b: usize) -> String {
        let (paras, items) = &self.all[&b];
        prompt(paras, items)
    }

    fn size(&self, b: usize) -> usize {
        self.all[&b].0.iter().map(|p| p.sents.len()).sum()
    }

    fn check(&self, b: usize, text: &str) -> (Option<String>, String) {
        let (paras, items) = &self.all[&b];
        let g = gate(text, paras, items);
        let span = g.ok.iter().filter(|f| f.exim == ExIm::Explicit).count();
        let rej = g.rej.iter().filter(|r| r.level == Level::Pair).count();
        (g.format, format!("items {}, spans {span}, became implicit {}, rejected {rej}", items.len(), g.ok.len() - span))
    }
}

/// An accepted fix for item `i` (from 1 in the batch).
#[derive(Clone, Debug, PartialEq)]
pub struct Fix {
    pub i: usize,
    pub exim: ExIm,
    pub anchors: Vec<usize>,
    pub answer: String,
}

pub struct FixGated {
    pub ok: Vec<Fix>,
    pub rej: Vec<Reject>,
    pub junk: usize,
    pub format: Option<String>,
}

/// Gate for the LLM output on a fix batch.
pub fn gate(text: &str, paras: &[&Para], items: &[&Item]) -> FixGated {
    let m = items.len();
    let mut by_i: BTreeMap<usize, Vec<(String, Vec<String>)>> = BTreeMap::new();
    let (mut junk, mut total) = (0usize, 0usize);
    let mut outside: Vec<usize> = Vec::new();
    for line in text.lines() {
        let l = line.trim_end_matches('\r');
        let t = l.trim();
        if t.is_empty() || t.starts_with("```") {
            continue;
        }
        let cols: Vec<String> = l.split('\t').map(str::to_string).collect();
        let c0 = cols[0].trim();
        if c0.eq_ignore_ascii_case("item") {
            continue; // header
        }
        // "Item 3" instead of "3" is the same unambiguous label
        let num = if c0.len() > 4 && c0[..4].eq_ignore_ascii_case("item") { c0[4..].trim() } else { c0 };
        let Ok(i) = num.parse::<usize>() else {
            junk += 1;
            continue;
        };
        if i == 0 || i > m {
            outside.push(i);
            continue;
        }
        total += 1;
        by_i.entry(i).or_default().push((l.to_string(), cols));
    }
    let missing = (1..=m).filter(|i| !by_i.contains_key(i)).count();
    let format = if total == 0 {
        Some("no lines at all".to_string())
    } else if !outside.is_empty() {
        Some(format!("item numbers outside 1..{m}: {outside:?} — alignment in doubt"))
    } else if junk * 5 > total + junk {
        Some(format!("lines without a number: {junk} of {}", total + junk))
    } else if missing * 4 > m {
        Some(format!("items without an answer: {missing} of {m} — truncated output?"))
    } else {
        None
    };
    if let Some(f) = format {
        let rej = vec![Reject { p: None, level: Level::Batch, reason: f.clone(), line: String::new() }];
        return FixGated { ok: Vec::new(), rej, junk, format: Some(f) };
    }
    let mut ok = Vec::new();
    let mut rej = Vec::new();
    let pos: HashMap<String, usize> = paras.iter().enumerate().map(|(i, p)| (p.key(), i + 1)).collect();
    for (i, it) in items.iter().enumerate().map(|(j, it)| (j + 1, it)) {
        let pn = pos[&it.key()];
        let para = paras[pn - 1];
        let Some(lines) = by_i.get(&i) else {
            rej.push(Reject { p: Some(pn), level: Level::Pair, reason: "no answer".into(), line: format!("{i}\t{}", it.question) });
            continue;
        };
        let sents = Sents::of(para);
        for (j, (line, cols)) in lines.iter().enumerate() {
            let r = if j > 0 { Err("item repeated".to_string()) } else { check(cols, &sents, it) };
            match r {
                Ok((exim, anchors, answer)) => ok.push(Fix { i, exim, anchors, answer }),
                Err(reason) => rej.push(Reject { p: Some(pn), level: Level::Pair, reason, line: line.clone() }),
            }
        }
    }
    FixGated { ok, rej, junk, format: None }
}

fn check(cols: &[String], s: &Sents, it: &Item) -> Result<(ExIm, Vec<usize>, String), String> {
    if cols.len() != 3 {
        return Err(format!("{} columns instead of 3", cols.len()));
    }
    let span = cols[2].trim();
    if span.eq_ignore_ascii_case("implicit") {
        if let Some(k) = verbatim(&it.answer, &s.words) {
            return Err(format!("implicit, but the old answer is verbatim in sentence {k}"));
        }
        return Ok((ExIm::Implicit, it.anchors.clone(), it.answer.clone()));
    }
    let anchors = anchors(&cols[1], s.words.len())?;
    if words(span).is_empty() {
        return Err("empty span".into());
    }
    if let Some(why) = span_reason(span, &s.norm, &s.words, &anchors) {
        return Err(why);
    }
    Ok((ExIm::Explicit, anchors, span.to_string()))
}

/// Collection: gates over the final outputs → `qa.tsv` (new rows for `db qa --replaces`), `rejects.tsv`,
/// `changes.md` (before → after), `params.json`.
pub fn collect(dir: &Path, replaces: &str, l: &dyn Lemmas) -> Result<Value> {
    let paras = load_paras(dir)?;
    let items = load_items(dir)?;
    let fx = Fixes::new(&paras, &items);
    let mut qa = format!("{QA_HEADER}\n");
    let mut rej = String::from("batch\tdoc_key\tpara\tlevel\treason\tline\n");
    let mut changes = format!("# qagen fix — explicit pilot pairs → verbatim spans (replacing run {replaces})\n");
    let (mut span, mut implicit, mut doubts, mut retries) = (0usize, 0usize, 0usize, 0usize);
    let mut by_level: BTreeMap<&str, usize> = BTreeMap::new();
    let mut by_kind: BTreeMap<String, usize> = BTreeMap::new();
    let mut missing = Vec::new();
    let mut fixed_reason: BTreeMap<String, (usize, usize)> = BTreeMap::new();
    for (&b, (ps, its)) in &fx.all {
        let Some(path) = final_raw(dir, b) else {
            missing.push(b);
            continue;
        };
        retries += retry_path(dir, b).exists() as usize;
        let text = std::fs::read_to_string(&path).with_context(|| path.display().to_string())?;
        let g = gate(&text, ps, its);
        let mut log: Vec<Reject> = g.rej;
        for f in &g.ok {
            let it = its[f.i - 1];
            let p = ps.iter().find(|p| p.key() == it.key()).unwrap();
            let ids: Vec<&str> = f.anchors.iter().map(|&k| p.sents[k - 1].sent_id.as_str()).collect();
            let _ = writeln!(qa, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}", it.doc, it.para, it.k, it.qtype, f.exim.name(), ids.join(","), it.question, f.answer);
            let e = fixed_reason.entry(kind(&it.reason)).or_default();
            match f.exim {
                ExIm::Explicit => {
                    span += 1;
                    e.0 += 1;
                }
                ExIm::Implicit => {
                    implicit += 1;
                    e.1 += 1;
                    let texts: Vec<&str> = p.sents.iter().map(|s| s.text.as_str()).collect();
                    if let Some((k, share)) = lemma::doubt(l, &texts, &f.anchors, &f.answer) {
                        doubts += 1;
                        let reason = format!("doubtful implicit: lemmas of sentence {k} cover {:.0} % of the answer", 100.0 * share);
                        log.push(Reject { p: ps.iter().position(|q| q.key() == it.key()).map(|x| x + 1), level: Level::Doubt, reason, line: format!("{}\t{}", it.question, f.answer) });
                    }
                }
            }
            let _ = write!(
                changes,
                "\n## {} \"{}\", paragraph {}, question {} ({})\n\n- Q: {}\n- before: {} · anchors {:?} · {}\n- after: {} · anchors {:?} · {}\n",
                it.doc,
                p.title,
                it.para,
                it.k,
                it.qtype,
                it.question,
                it.answer,
                it.anchors,
                it.reason,
                f.answer,
                f.anchors,
                f.exim.name()
            );
        }
        for r in &log {
            let (doc, para) = r.p.map(|p| (ps[p - 1].doc.as_str(), ps[p - 1].para.to_string())).unwrap_or(("-", "-".into()));
            let _ = writeln!(rej, "{b}\t{doc}\t{para}\t{}\t{}\t{}", r.level.name(), r.reason, r.line.replace('\t', " | "));
            *by_level.entry(r.level.name()).or_default() += 1;
            *by_kind.entry(format!("{}: {}", r.level.name(), kind(&r.reason))).or_default() += 1;
        }
    }
    std::fs::write(dir.join("qa.tsv"), qa)?;
    std::fs::write(dir.join("rejects.tsv"), rej)?;
    std::fs::write(dir.join("changes.md"), changes)?;
    let fixed = span + implicit;
    let params = json!({
        "model": std::env::var("PRAG_MODEL").unwrap_or_else(|_| "claude-opus-5-5".into()),
        "effort": std::env::var("PRAG_EFFORT").unwrap_or_else(|_| "medium".into()),
        "how": "claude -p --settings {\"env\":{\"CLAUDE_CODE_EFFORT_LEVEL\":…}}, prag::opus; qagen fix",
        "rule": 2,
        "replaces": replaces,
        "license": "gutenberg-tales (en/data/train-licenses.tsv): public domain",
        "items": items.len(),
        "fixed": {"n": fixed, "pct": pct(fixed, items.len()), "span": span, "implicit": implicit, "implicit_doubt": doubts,
                  "by_old_reason": fixed_reason.iter().map(|(k, (s, i))| (k.clone(), json!({"span": s, "implicit": i}))).collect::<serde_json::Map<_, _>>()},
        "unfixed": items.len() - fixed,
        "rejects": {"by_level": by_level, "by_kind": by_kind},
        "batches_missing": missing,
        "calls": calls_json(dir, retries)?,
    });
    std::fs::write(dir.join("params.json"), serde_json::to_string_pretty(&params)?)?;
    Ok(params)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::select::Sent;

    fn para(doc: &str, n: i64, texts: &[&str]) -> Para {
        Para {
            doc: doc.into(),
            book: "g:1".into(),
            title: "T".into(),
            author: String::new(),
            para: n,
            block: "p".into(),
            sents: texts.iter().enumerate().map(|(i, t)| Sent { sent_id: format!("{doc}:{n}{}", i + 1), text: t.to_string() }).collect(),
            dialogue: false,
            words: 0,
            ctx: vec![],
            batch: 0,
        }
    }

    fn item(doc: &str, n: i64, k: usize, anchors: &[usize], q: &str, a: &str) -> Item {
        Item { doc: doc.into(), para: n, k, qtype: "action".into(), question: q.into(), answer: a.into(), anchors: anchors.to_vec(), reason: "explicit: not a span of an anchor sentence".into(), batch: 0 }
    }

    fn setup() -> (Vec<Para>, Vec<Item>) {
        let paras = vec![
            para("g:1:a", 1, &["A certain king had a beautiful garden.", "Every night one of the golden apples was gone."]),
            para("g:1:b", 3, &["‘I will catch the thief,’ said the eldest son.", "At midnight he fell asleep."]),
        ];
        let items = vec![
            item("g:1:a", 1, 2, &[2], "What happened every night?", "an apple vanished"),
            item("g:1:a", 1, 4, &[1], "What did the king own?", "a lovely garden"),
            item("g:1:b", 3, 1, &[1], "What did the eldest son promise?", "to catch whoever steals"),
            item("g:1:b", 3, 5, &[1, 2], "Why did the son fail?", "he slept at midnight instead of watching"),
        ];
        (paras, items)
    }

    #[test]
    fn prompt_numbers_items_under_paragraphs() {
        let (paras, items) = setup();
        let fx = Fixes::new(&paras, &items);
        let (ps, its) = &fx.all[&0];
        assert_eq!((ps.len(), its.len()), (2, 4));
        let p = prompt(ps, its);
        assert!(p.starts_with(FIX_RULES));
        assert!(p.contains("=== P1 - \"T\", paragraph 1\n[1] A certain king had a beautiful garden.\n[2] Every night one of the golden apples was gone.\nItem 1 (action; anchors 2): What happened every night? | current answer: an apple vanished\nItem 2 (action; anchors 1): "));
        assert!(p.contains("[2] At midnight he fell asleep.\nItem 3 (action; anchors 1): What did the eldest son promise?"));
        assert!(p.contains("Item 4 (action; anchors 1,2): Why did the son fail?"));
    }

    /// Good output passes; negative controls: span not from an anchor, paraphrase, repeated item,
    /// "implicit" with a verbatim old answer, foreign anchors, no answer; batch format.
    #[test]
    fn fix_gate_and_negative_controls() {
        let (paras, items) = setup();
        let fx = Fixes::new(&paras, &items);
        let (ps, its) = &fx.all[&0];
        let good = "1\t2\tone of the golden apples was gone.\nItem 2\t1\t“a beautiful garden”\n3\t1,2\tI will catch the thief\n4\t-\timplicit\n";
        let g = gate(good, ps, its);
        assert!(g.format.is_none() && g.rej.is_empty(), "{:?}", g.rej);
        assert_eq!(g.ok.len(), 4);
        assert_eq!(g.ok[0], Fix { i: 1, exim: ExIm::Explicit, anchors: vec![2], answer: "one of the golden apples was gone.".into() });
        assert_eq!(g.ok[3], Fix { i: 4, exim: ExIm::Implicit, anchors: vec![1, 2], answer: "he slept at midnight instead of watching".into() });
        let bad = [
            ("1\t1\tone of the golden apples was gone\n", "explicit: span of sentence 2, which is not an anchor"),
            ("1\t2\tan apple was gone\n", "explicit: not a span of an anchor sentence"),
            ("1\t2\tone of the golden apples, was gone\n", "explicit: consecutive words in sentence 2, but different punctuation"),
            ("1\t3\tone of the golden apples was gone\n", "anchor 3 outside the paragraph (2 sentences)"),
            ("1\t2\t\n", "empty span"),
            ("1\t2\n", "2 columns instead of 3"),
        ];
        let rest = "2\t1\ta beautiful garden\n3\t1\tI will catch the thief\n4\t-\timplicit\n";
        for (line, why) in bad {
            let g = gate(&format!("{line}{rest}"), ps, its);
            assert!(g.format.is_none(), "{line}");
            assert_eq!(g.ok.len(), 3, "{line}");
            assert_eq!(g.rej.len(), 1, "{line}: {:?}", g.rej);
            assert_eq!(g.rej[0].reason, why, "{line}");
        }
        // repeated item: the first line counts, the second is rejected
        let g = gate(&format!("{good}1\t2\tthe golden apples\n"), ps, its);
        assert_eq!((g.ok.len(), g.rej.len(), g.rej[0].reason.as_str()), (4, 1, "item repeated"));
        // no answer for item 4: logged, the rest is accepted
        let g = gate("1\t2\tone of the golden apples was gone\n2\t1\ta beautiful garden\n3\t1\tI will catch the thief\n", ps, its);
        assert_eq!((g.ok.len(), g.rej[0].reason.as_str()), (3, "no answer"));
        // "implicit" while the old answer is verbatim in a sentence
        let mut items2 = items.clone();
        items2[3].answer = "he fell asleep".into();
        let fx2 = Fixes::new(&paras, &items2);
        let (ps2, its2) = &fx2.all[&0];
        let g = gate(good, ps2, its2);
        assert_eq!(g.rej[0].reason, "implicit, but the old answer is verbatim in sentence 2");
        // format: empty, number outside the batch, truncated output, junk
        assert!(gate("", ps, its).format.is_some());
        assert!(gate(&format!("{good}5\t1\tthe king\n"), ps, its).format.is_some());
        assert!(gate("1\t2\tone of the golden apples was gone\n", ps, its).format.is_some());
        assert!(gate(&format!("{good}Sure, here:\nNote:\n"), ps, its).format.is_some());
    }

    #[test]
    fn read_qa_checks_header() {
        let dir = std::env::temp_dir().join(format!("qagen-fix-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let f = dir.join("qa.tsv");
        std::fs::write(&f, format!("{QA_HEADER}\ng:1:a\t1\t2\taction\texplicit\tg:1:a:12\tWhat?\tan apple\n")).unwrap();
        let r = read_qa(&f).unwrap();
        assert_eq!((r[0].k, r[0].exim, r[0].anchors.as_slice()), (2, ExIm::Explicit, &["g:1:a:12".to_string()][..]));
        std::fs::write(&f, "doc\tpara\n").unwrap();
        assert!(read_qa(&f).is_err());
        std::fs::write(&f, format!("{QA_HEADER}\ng:1:a\t1\t2\tcausal\texplicit\tg:1:a:12\tWhat?\tan apple\n")).unwrap();
        assert!(read_qa(&f).is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
