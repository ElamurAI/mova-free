//! Collection after a run: gates over the final raw outputs → `qa.tsv` (for `db qa`), `rejects.tsv`
//! (log), `params.json` (accounting and distributions — into the database's `runs.params`); a sample of pairs for eyeballing — `sample.md`.
//! Rule v2: rejections by the span gate are counted separately (share of explicit ones that passed on the first try), and
//! implicit pairs made of the lemmas of a single anchor (`lemma::doubt`) go into the log with level `doubt`.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::Path;

use anyhow::{Context, Result};
use serde_json::{Value, json};

use prag::data::h64;
use prag::opus::read_calls;

use crate::gate::{Gated, Level, Pair, Reject, gate, words};
use crate::lemma::{self, Lemmas};
use crate::run::{batches, final_raw, retry_path};
use crate::schema::{ExIm, QType, Rule};
use crate::select::{Para, Pick};

/// Accepted pairs with paragraphs.
pub struct Collected<'a> {
    pub pairs: Vec<(&'a Para, Pair)>,
    pub gated: Vec<(usize, Vec<&'a Para>, Gated)>,
    pub missing: Vec<usize>,
}

pub fn gather<'a>(dir: &Path, paras: &'a [Para], rule: Rule) -> Result<Collected<'a>> {
    let mut c = Collected { pairs: Vec::new(), gated: Vec::new(), missing: Vec::new() };
    for (b, its) in batches(paras) {
        let Some(path) = final_raw(dir, b) else {
            c.missing.push(b);
            continue;
        };
        let text = std::fs::read_to_string(&path).with_context(|| path.display().to_string())?;
        let g = gate(&text, &its, rule);
        c.pairs.extend(g.ok.iter().map(|x| (its[x.p - 1], x.clone())));
        c.gated.push((b, its, g));
    }
    Ok(c)
}

/// Reason without numbers and quotes — for counting rejections by kind.
pub fn kind(reason: &str) -> String {
    let mut out = String::new();
    let mut quoted = false;
    for ch in reason.chars() {
        match ch {
            '«' => {
                quoted = true;
                out.push_str("«…");
            }
            '»' => {
                quoted = false;
                out.push('»');
            }
            _ if quoted => {}
            c if c.is_ascii_digit() => {
                if !out.ends_with('#') {
                    out.push('#');
                }
            }
            c => out.push(c),
        }
    }
    out
}

/// Share of the answer's words present in the anchor sentences (0..1).
pub fn support(p: &Para, x: &Pair) -> f64 {
    let a = words(&x.answer);
    if a.is_empty() {
        return 0.0;
    }
    let pool: std::collections::HashSet<String> = x.anchors.iter().flat_map(|&k| words(&p.sents[k - 1].text)).collect();
    a.iter().filter(|w| pool.contains(*w)).count() as f64 / a.len() as f64
}

pub fn pct(a: usize, b: usize) -> f64 {
    if b == 0 { 0.0 } else { (1000.0 * a as f64 / b as f64).round() / 10.0 }
}

/// Call accounting of a dir (`calls.jsonl`): count, retries, time, tokens, cost estimate.
pub fn calls_json(dir: &Path, retries: usize) -> Result<Value> {
    let calls = read_calls(&dir.join("calls.jsonl"))?;
    let sum = |f: fn(&prag::opus::Call) -> u64| calls.iter().map(f).sum::<u64>();
    let secs: f64 = calls.iter().map(|x| x.secs).sum();
    Ok(json!({"n": calls.len(), "retries": retries, "secs_sum": (secs * 10.0).round() / 10.0,
              "secs_max": calls.iter().map(|x| x.secs).fold(0.0, f64::max).round(),
              "input_tokens": sum(|x| x.input_tokens), "output_tokens": sum(|x| x.output_tokens),
              "cache_read_tokens": sum(|x| x.cache_read_tokens), "cache_write_tokens": sum(|x| x.cache_write_tokens),
              "cost_usd_estimate": (calls.iter().map(|x| x.cost_usd).sum::<f64>() * 100.0).round() / 100.0}))
}

/// "Doubtful implicit" marks for accepted implicit pairs: (batch, log entry).
pub fn doubts(l: &dyn Lemmas, c: &Collected) -> Vec<(usize, Reject)> {
    let mut out = Vec::new();
    for (b, its, g) in &c.gated {
        for x in g.ok.iter().filter(|x| x.exim == ExIm::Implicit) {
            let p = its[x.p - 1];
            let sents: Vec<&str> = p.sents.iter().map(|s| s.text.as_str()).collect();
            if let Some((k, share)) = lemma::doubt(l, &sents, &x.anchors, &x.answer) {
                let reason = format!("doubtful implicit: lemmas of sentence {k} cover {:.0} % of the answer", 100.0 * share);
                let line = format!("{}\t{}\t{}\t{}\t{}", x.k, x.qtype.name(), x.anchors.iter().map(usize::to_string).collect::<Vec<_>>().join(","), x.question, x.answer);
                out.push((*b, Reject { p: Some(x.p), level: Level::Doubt, reason, line }));
            }
        }
    }
    out
}

/// Gates, files and accounting. Returns the run's `params`.
pub fn collect(dir: &Path, paras: &[Para], pick: &Pick) -> Result<Value> {
    let rule = pick.rule()?;
    let c = gather(dir, paras, rule)?;
    // v2: "doubtful implicit" marks — lemmas from `en`
    let doubt = if rule >= Rule::V2 { doubts(&lemma::load()?, &c) } else { Vec::new() };
    // qa.tsv for `db qa`
    let mut qa = String::from("doc_key\tpara\tk\tqtype\tex_or_im\tanchors\tquestion\tanswer\n");
    for (p, x) in &c.pairs {
        let anchors: Vec<&str> = x.anchors.iter().map(|&k| p.sents[k - 1].sent_id.as_str()).collect();
        let _ = writeln!(qa, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}", p.doc, p.para, x.k, x.qtype.name(), x.exim.name(), anchors.join(","), x.question, x.answer);
    }
    std::fs::write(dir.join("qa.tsv"), qa)?;
    // log
    let mut rej = String::from("batch\tdoc_key\tpara\tlevel\treason\tline\n");
    let mut by_level: BTreeMap<&str, usize> = BTreeMap::new();
    let mut by_kind: BTreeMap<String, usize> = BTreeMap::new();
    for (b, its, g) in &c.gated {
        let mine = doubt.iter().filter(|(db, _)| db == b).map(|(_, r)| r);
        for r in g.rej.iter().chain(mine) {
            let (doc, para) = r.p.map(|p| (its[p - 1].doc.as_str(), its[p - 1].para.to_string())).unwrap_or(("-", "-".into()));
            let _ = writeln!(rej, "{b}\t{doc}\t{para}\t{}\t{}\t{}", r.level.name(), r.reason, r.line.replace('\t', " | "));
            *by_level.entry(r.level.name()).or_default() += 1;
            *by_kind.entry(format!("{}: {}", r.level.name(), kind(&r.reason))).or_default() += 1;
        }
    }
    // explicit ones that reached the span gate: accepted and rejected by it
    let span_rej = c.gated.iter().flat_map(|(_, _, g)| &g.rej).filter(|r| r.level == Level::Pair && r.reason.starts_with("explicit:")).count();
    std::fs::write(dir.join("rejects.tsv"), rej)?;
    // distributions
    let sent_paras: usize = c.gated.iter().map(|(_, its, _)| its.len()).sum();
    let mut per_para: BTreeMap<String, usize> = BTreeMap::new();
    for (p, _) in &c.pairs {
        *per_para.entry(p.key()).or_default() += 1;
    }
    let mut hist: BTreeMap<usize, usize> = BTreeMap::new();
    for n in per_para.values() {
        *hist.entry(*n).or_default() += 1;
    }
    let with_implicit = {
        let mut s = std::collections::HashSet::new();
        for (p, x) in &c.pairs {
            if x.exim == ExIm::Implicit {
                s.insert(p.key());
            }
        }
        s.len()
    };
    let n = c.pairs.len();
    let mut types = serde_json::Map::new();
    for t in QType::ALL {
        let all = c.pairs.iter().filter(|(_, x)| x.qtype == t).count();
        let im = c.pairs.iter().filter(|(_, x)| x.qtype == t && x.exim == ExIm::Implicit).count();
        let dial = c.pairs.iter().filter(|(p, x)| x.qtype == t && p.dialogue).count();
        types.insert(t.name().into(), json!({"n": all, "pct": pct(all, n), "explicit": all - im, "implicit": im, "in_dialogue_paras": dial}));
    }
    let implicit = c.pairs.iter().filter(|(_, x)| x.exim == ExIm::Implicit).count();
    let mut books: BTreeMap<String, (usize, usize, usize)> = BTreeMap::new();
    for p in paras {
        books.entry(format!("{} {}", p.book, p.author)).or_default().0 += 1;
    }
    for (p, _) in &c.pairs {
        books.entry(format!("{} {}", p.book, p.author)).or_default().1 += 1;
    }
    for p in paras.iter().filter(|p| p.dialogue) {
        books.entry(format!("{} {}", p.book, p.author)).or_default().2 += 1;
    }
    let dial_pairs = c.pairs.iter().filter(|(p, _)| p.dialogue).count();
    let explicit: Vec<f64> = c.pairs.iter().filter(|(_, x)| x.exim == ExIm::Explicit).map(|(p, x)| support(p, x)).collect();
    let anchors_mean = if n == 0 { 0.0 } else { c.pairs.iter().map(|(_, x)| x.anchors.len()).sum::<usize>() as f64 / n as f64 };
    let retries = batches(paras).keys().filter(|&&b| retry_path(dir, b).exists()).count();
    let mut params = json!({
        "model": std::env::var("PRAG_MODEL").unwrap_or_else(|_| "claude-opus-5-5".into()),
        "effort": std::env::var("PRAG_EFFORT").unwrap_or_else(|_| "medium".into()),
        "how": "claude -p --settings {\"env\":{\"CLAUDE_CODE_EFFORT_LEVEL\":…}}, prag::opus",
        "pick": pick,
        "license": "gutenberg-tales (en/data/train-licenses.tsv): public domain",
        "paragraphs": {"selected": paras.len(), "sent": sent_paras, "with_pairs": per_para.len(), "with_implicit": with_implicit,
                        "dialogue": paras.iter().filter(|p| p.dialogue).count(), "pairs_per_paragraph": hist},
        "pairs": {"n": n, "explicit": n - implicit, "implicit": implicit, "implicit_pct": pct(implicit, n), "in_dialogue_paras": dial_pairs,
                  "anchors_mean": (anchors_mean * 100.0).round() / 100.0,
                  "explicit_answer_words_in_anchors": {
                      "all": explicit.iter().filter(|&&s| s >= 1.0).count(), "under_half": explicit.iter().filter(|&&s| s < 0.5).count(), "of": explicit.len()}},
        "types": types,
        "books": books.iter().map(|(b, (p, q, d))| (b.clone(), json!({"paragraphs": p, "dialogue": d, "pairs": q}))).collect::<serde_json::Map<_, _>>(),
        "rejects": {"by_level": by_level, "by_kind": by_kind},
        "batches_missing": c.missing,
        "calls": calls_json(dir, retries)?,
    });
    if rule >= Rule::V2 {
        let offered = n - implicit + span_rej;
        params["rule"] = json!({
            "n": rule.num(),
            "explicit": "verbatim span of one anchor sentence (case, quotes, spaces, edge punctuation normalized)",
            "doubt": format!("implicit answer: >= {:.0} % of content words from the lemmas of one anchor (en {})", 100.0 * lemma::DOUBT,
                             lemma::model_path().file_name().unwrap_or_default().to_string_lossy()),
        });
        params["pairs"]["explicit_span"] = json!({"offered": offered, "passed": n - implicit, "rejected": span_rej, "passed_pct": pct(n - implicit, offered)});
        params["pairs"]["implicit_doubt"] = json!({"n": doubt.len(), "of": implicit, "pct": pct(doubt.len(), implicit)});
    }
    std::fs::write(dir.join("params.json"), serde_json::to_string_pretty(&params)?)?;
    Ok(params)
}

/// `n` random (seeded) accepted pairs with paragraphs — for eyeballing → `sample.md`. With `weak` — only explicit pairs
/// where the share of the answer's words in the anchors is below `weak` (suspected wrong anchor) → `weak.md`.
pub fn sample(dir: &Path, paras: &[Para], rule: Rule, n: usize, seed: u64, weak: Option<f64>) -> Result<String> {
    let c = gather(dir, paras, rule)?;
    let keep = |p: &Para, x: &Pair| weak.is_none_or(|t| x.exim == ExIm::Explicit && support(p, x) < t);
    let mut order: Vec<(u64, usize)> =
        c.pairs.iter().enumerate().filter(|(_, (p, x))| keep(p, x)).map(|(i, (p, x))| (h64(&format!("{}#{}", p.key(), x.k), seed), i)).collect();
    order.sort();
    let mut out = match weak {
        None => format!("# qagen — {n} random pairs (seed {seed})\n"),
        Some(t) => format!("# qagen — explicit pairs with answer support in anchors < {:.0} %: {} (showing up to {n})\n", 100.0 * t, order.len()),
    };
    for (j, &(_, i)) in order.iter().take(n).enumerate() {
        let (p, x) = &c.pairs[i];
        let _ = write!(out, "\n## {}. {} — «{}», paragraph {}{}\n\n", j + 1, p.doc, p.title, p.para, if p.dialogue { " (dialogue)" } else { "" });
        for (k, s) in p.sents.iter().enumerate() {
            let mark = if x.anchors.contains(&(k + 1)) { "**→**" } else { "" };
            let _ = writeln!(out, "{mark}[{}] {}  ", k + 1, s.text);
        }
        let _ = write!(
            out,
            "\n- **{}** · {} · anchors {:?} · answer support in anchors {:.0} %\n- Q: {}\n- A: {}\n",
            x.qtype.name(),
            x.exim.name(),
            x.anchors,
            100.0 * support(p, x),
            x.question,
            x.answer
        );
    }
    let path = dir.join(if weak.is_some() { "weak.md" } else { "sample.md" });
    std::fs::write(&path, &out)?;
    Ok(path.display().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reason_kinds_drop_numbers_and_quotes() {
        assert_eq!(kind("anchor 12 outside the paragraph (3 sentences)"), "anchor # outside the paragraph (# sentences)");
        assert_eq!(kind("type «causal» not in the set"), "type «…» not in the set");
    }

    #[test]
    fn levels_have_names() {
        assert_eq!([Level::Batch, Level::Para, Level::Pair, Level::Note, Level::Doubt].map(Level::name), ["batch", "para", "pair", "note", "doubt"]);
    }
}
