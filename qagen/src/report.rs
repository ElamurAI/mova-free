//! Collection after a run: gates over the final raw outputs → `qa.tsv` (for `db qa`), `rejects.tsv`
//! (log), `params.json` (accounting and distributions — into the database's `runs.params`); a sample of pairs for eyeballing — `sample.md`.
//! Rule v2: rejections by the span gate are counted separately (share of explicit ones that passed on the first try), and
//! implicit pairs made of the lemmas of a single anchor (`lemma::doubt`) go into the log with level `doubt`.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::Path;

use anyhow::{Context, Result, ensure};
use serde_json::{Value, json};

use prag::data::h64;
use prag::opus::read_calls;

use crate::gate::{Gated, Level, Pair, Reject, gate, words};
use crate::lemma::{self, Lemmas};
use crate::run::{batches, retry_path, try_final_raw};
use crate::schema::{ExIm, QType, Rule};
use crate::select::{Para, Pick};

/// Accepted pairs with paragraphs.
pub struct Collected<'a> {
    pub pairs: Vec<(&'a Para, Pair)>,
    pub gated: Vec<(usize, Vec<&'a Para>, Gated)>,
    pub missing: Vec<usize>,
}

pub fn gather<'a>(dir: &Path, paras: &'a [Para], rule: Rule) -> Result<Collected<'a>> {
    let mut c = Collected {
        pairs: Vec::new(),
        gated: Vec::new(),
        missing: Vec::new(),
    };
    for (b, its) in batches(paras) {
        let Some(path) = try_final_raw(dir, b)? else {
            c.missing.push(b);
            continue;
        };
        let text = std::fs::read_to_string(&path).with_context(|| path.display().to_string())?;
        let g = gate(&text, &its, rule);
        c.pairs
            .extend(g.ok.iter().map(|x| (its[x.p - 1], x.clone())));
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
    let pool: std::collections::HashSet<String> = x
        .anchors
        .iter()
        .flat_map(|&k| words(&p.sents[k - 1].text))
        .collect();
    a.iter().filter(|w| pool.contains(*w)).count() as f64 / a.len() as f64
}

pub fn pct(a: usize, b: usize) -> f64 {
    if b == 0 {
        0.0
    } else {
        (1000.0 * a as f64 / b as f64).round() / 10.0
    }
}

/// Call accounting of a dir (`calls.jsonl`): count, retries, time, tokens, cost estimate.
pub fn calls_json(dir: &Path, retries: usize) -> Result<Value> {
    let calls = read_calls(&dir.join("calls.jsonl"))?;
    let sum = |name: &str, f: fn(&prag::opus::Call) -> u64| -> Result<u64> {
        calls.iter().try_fold(0u64, |total, call| {
            total
                .checked_add(f(call))
                .with_context(|| format!("call accounting {name} total exceeds u64"))
        })
    };
    let finite_sum = |name: &str, f: fn(&prag::opus::Call) -> f64| -> Result<f64> {
        calls.iter().try_fold(0.0, |total, call| {
            let next = total + f(call);
            ensure!(
                next.is_finite(),
                "call accounting {name} total is not finite"
            );
            Ok(next)
        })
    };
    let secs = finite_sum("seconds", |x| x.secs)?;
    let cost = finite_sum("cost", |x| x.cost_usd)?;
    Ok(
        json!({"n": calls.len(), "retries": retries, "secs_sum": rounded(secs, 10.0),
              "secs_max": calls.iter().map(|x| x.secs).fold(0.0, f64::max).round(),
              "input_tokens": sum("input_tokens", |x| x.input_tokens)?, "output_tokens": sum("output_tokens", |x| x.output_tokens)?,
              "cache_read_tokens": sum("cache_read_tokens", |x| x.cache_read_tokens)?, "cache_write_tokens": sum("cache_write_tokens", |x| x.cache_write_tokens)?,
              "cost_usd_estimate": rounded(cost, 100.0)}),
    )
}

/// Preserve historical decimal rounding without overflowing its scaling step.
/// At this magnitude f64 spacing already exceeds one, so rounding is identity.
fn rounded(value: f64, scale: f64) -> f64 {
    if value.abs() > f64::MAX / scale {
        value
    } else {
        (value * scale).round() / scale
    }
}

/// "Doubtful implicit" marks for accepted implicit pairs: (batch, log entry).
pub fn doubts(l: &dyn Lemmas, c: &Collected) -> Vec<(usize, Reject)> {
    let mut out = Vec::new();
    for (b, its, g) in &c.gated {
        for x in g.ok.iter().filter(|x| x.exim == ExIm::Implicit) {
            let p = its[x.p - 1];
            let sents: Vec<&str> = p.sents.iter().map(|s| s.text.as_str()).collect();
            if let Some((k, share)) = lemma::doubt(l, &sents, &x.anchors, &x.answer) {
                let reason = format!(
                    "doubtful implicit: lemmas of sentence {k} cover {:.0} % of the answer",
                    100.0 * share
                );
                let line = format!(
                    "{}\t{}\t{}\t{}\t{}",
                    x.k,
                    x.qtype.name(),
                    x.anchors
                        .iter()
                        .map(usize::to_string)
                        .collect::<Vec<_>>()
                        .join(","),
                    x.question,
                    x.answer
                );
                out.push((
                    *b,
                    Reject {
                        p: Some(x.p),
                        level: Level::Doubt,
                        reason,
                        line,
                    },
                ));
            }
        }
    }
    out
}

/// Gates, files and accounting. Returns the run's `params`.
pub fn collect(dir: &Path, paras: &[Para], pick: &Pick) -> Result<Value> {
    let rule = pick.rule()?;
    // Validate accounting before model loading or replacing any report artifact.
    let retries = batches(paras)
        .keys()
        .try_fold(0usize, |n, &b| -> Result<usize> {
            let path = retry_path(dir, b);
            Ok(n + usize::from(
                path.try_exists()
                    .with_context(|| path.display().to_string())?,
            ))
        })?;
    let calls = calls_json(dir, retries)?;
    let c = gather(dir, paras, rule)?;
    // v2: "doubtful implicit" marks — lemmas from `en`
    let doubt = if rule >= Rule::V2 {
        doubts(&lemma::load()?, &c)
    } else {
        Vec::new()
    };
    // qa.tsv for `db qa`
    let mut qa = String::from("doc_key\tpara\tk\tqtype\tex_or_im\tanchors\tquestion\tanswer\n");
    for (p, x) in &c.pairs {
        let anchors: Vec<&str> = x
            .anchors
            .iter()
            .map(|&k| p.sents[k - 1].sent_id.as_str())
            .collect();
        let _ = writeln!(
            qa,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            p.doc,
            p.para,
            x.k,
            x.qtype.name(),
            x.exim.name(),
            anchors.join(","),
            x.question,
            x.answer
        );
    }
    // log
    let mut rej = String::from("batch\tdoc_key\tpara\tlevel\treason\tline\n");
    let mut by_level: BTreeMap<&str, usize> = BTreeMap::new();
    let mut by_kind: BTreeMap<String, usize> = BTreeMap::new();
    for (b, its, g) in &c.gated {
        let mine = doubt.iter().filter(|(db, _)| db == b).map(|(_, r)| r);
        for r in g.rej.iter().chain(mine) {
            let (doc, para) =
                r.p.map(|p| (its[p - 1].doc.as_str(), its[p - 1].para.to_string()))
                    .unwrap_or(("-", "-".into()));
            let _ = writeln!(
                rej,
                "{b}\t{doc}\t{para}\t{}\t{}\t{}",
                r.level.name(),
                r.reason,
                r.line.replace('\t', " | ")
            );
            *by_level.entry(r.level.name()).or_default() += 1;
            *by_kind
                .entry(format!("{}: {}", r.level.name(), kind(&r.reason)))
                .or_default() += 1;
        }
    }
    // explicit ones that reached the span gate: accepted and rejected by it
    let span_rej = c
        .gated
        .iter()
        .flat_map(|(_, _, g)| &g.rej)
        .filter(|r| r.level == Level::Pair && r.reason.starts_with("explicit:"))
        .count();
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
        let im = c
            .pairs
            .iter()
            .filter(|(_, x)| x.qtype == t && x.exim == ExIm::Implicit)
            .count();
        let dial = c
            .pairs
            .iter()
            .filter(|(p, x)| x.qtype == t && p.dialogue)
            .count();
        types.insert(t.name().into(), json!({"n": all, "pct": pct(all, n), "explicit": all - im, "implicit": im, "in_dialogue_paras": dial}));
    }
    let implicit = c
        .pairs
        .iter()
        .filter(|(_, x)| x.exim == ExIm::Implicit)
        .count();
    let mut books: BTreeMap<String, (usize, usize, usize)> = BTreeMap::new();
    for p in paras {
        books
            .entry(format!("{} {}", p.book, p.author))
            .or_default()
            .0 += 1;
    }
    for (p, _) in &c.pairs {
        books
            .entry(format!("{} {}", p.book, p.author))
            .or_default()
            .1 += 1;
    }
    for p in paras.iter().filter(|p| p.dialogue) {
        books
            .entry(format!("{} {}", p.book, p.author))
            .or_default()
            .2 += 1;
    }
    let dial_pairs = c.pairs.iter().filter(|(p, _)| p.dialogue).count();
    let explicit: Vec<f64> = c
        .pairs
        .iter()
        .filter(|(_, x)| x.exim == ExIm::Explicit)
        .map(|(p, x)| support(p, x))
        .collect();
    let anchors_mean = if n == 0 {
        0.0
    } else {
        c.pairs.iter().map(|(_, x)| x.anchors.len()).sum::<usize>() as f64 / n as f64
    };
    let mut params = json!({
        "model": mova_teacher::model("PRAG_MODEL"),
        "effort": std::env::var("PRAG_EFFORT").unwrap_or_else(|_| "medium".into()),
        "how": "Codex CLI via mova-teacher",
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
        "calls": calls,
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
        params["pairs"]["implicit_doubt"] =
            json!({"n": doubt.len(), "of": implicit, "pct": pct(doubt.len(), implicit)});
    }
    let params_text = serde_json::to_string_pretty(&params)?;
    // All fallible preparation precedes writes; filesystem writes are not a transaction.
    std::fs::write(dir.join("qa.tsv"), qa)?;
    std::fs::write(dir.join("rejects.tsv"), rej)?;
    std::fs::write(dir.join("params.json"), params_text)?;
    Ok(params)
}

/// `n` random (seeded) accepted pairs with paragraphs — for eyeballing → `sample.md`. With `weak` — only explicit pairs
/// where the share of the answer's words in the anchors is below `weak` (suspected wrong anchor) → `weak.md`.
pub fn sample(
    dir: &Path,
    paras: &[Para],
    rule: Rule,
    n: usize,
    seed: u64,
    weak: Option<f64>,
) -> Result<String> {
    let c = gather(dir, paras, rule)?;
    let keep =
        |p: &Para, x: &Pair| weak.is_none_or(|t| x.exim == ExIm::Explicit && support(p, x) < t);
    let mut order: Vec<(u64, usize)> = c
        .pairs
        .iter()
        .enumerate()
        .filter(|(_, (p, x))| keep(p, x))
        .map(|(i, (p, x))| (h64(&format!("{}#{}", p.key(), x.k), seed), i))
        .collect();
    order.sort();
    let mut out = match weak {
        None => format!("# qagen — {n} random pairs (seed {seed})\n"),
        Some(t) => format!(
            "# qagen — explicit pairs with answer support in anchors < {:.0} %: {} (showing up to {n})\n",
            100.0 * t,
            order.len()
        ),
    };
    for (j, &(_, i)) in order.iter().take(n).enumerate() {
        let (p, x) = &c.pairs[i];
        let _ = write!(
            out,
            "\n## {}. {} — «{}», paragraph {}{}\n\n",
            j + 1,
            p.doc,
            p.title,
            p.para,
            if p.dialogue { " (dialogue)" } else { "" }
        );
        for (k, s) in p.sents.iter().enumerate() {
            let mark = if x.anchors.contains(&(k + 1)) {
                "**→**"
            } else {
                ""
            };
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
    let path = dir.join(if weak.is_some() {
        "weak.md"
    } else {
        "sample.md"
    });
    std::fs::write(&path, &out)?;
    Ok(path.display().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reason_kinds_drop_numbers_and_quotes() {
        assert_eq!(
            kind("anchor 12 outside the paragraph (3 sentences)"),
            "anchor # outside the paragraph (# sentences)"
        );
        assert_eq!(
            kind("type «causal» not in the set"),
            "type «…» not in the set"
        );
    }

    #[test]
    fn levels_have_names() {
        assert_eq!(
            [
                Level::Batch,
                Level::Para,
                Level::Pair,
                Level::Note,
                Level::Doubt
            ]
            .map(Level::name),
            ["batch", "para", "pair", "note", "doubt"]
        );
    }
}

#[cfg(test)]
mod wave18_models {
    use super::*;
    use prag::opus::{Call, append_call};
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicUsize, Ordering};
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    struct Scratch(PathBuf);
    impl Scratch {
        fn new() -> Self {
            let p = std::env::temp_dir().join(format!(
                "mova-report-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            std::fs::create_dir(&p).unwrap();
            Self(p)
        }
        fn calls(&self, calls: &[Call]) {
            for c in calls {
                append_call(&self.0.join("calls.jsonl"), c).unwrap();
            }
        }
    }
    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
    #[test]
    fn token_overflow_is_error_in_every_column() {
        for column in 0..4 {
            let d = Scratch::new();
            let mut c = Call::default();
            match column {
                0 => c.input_tokens = u64::MAX,
                1 => c.output_tokens = u64::MAX,
                2 => c.cache_read_tokens = u64::MAX,
                _ => c.cache_write_tokens = u64::MAX,
            }
            d.calls(&[c.clone(), c]);
            assert!(
                calls_json(&d.0, 0).is_err(),
                "column {column} overflow must be an error"
            );
        }
    }
    #[test]
    fn large_finite_rounding_stays_numeric() {
        let d = Scratch::new();
        d.calls(&[Call {
            secs: f64::MAX,
            cost_usd: f64::MAX,
            ..Call::default()
        }]);
        let v = calls_json(&d.0, 0).unwrap();
        assert_eq!(v["secs_sum"].as_f64(), Some(f64::MAX));
        assert_eq!(v["cost_usd_estimate"].as_f64(), Some(f64::MAX));
    }
    #[test]
    fn finite_negative_history_and_max_tokens_remain_readable() {
        let d = Scratch::new();
        d.calls(&[Call {
            secs: -1.25,
            cost_usd: -2.125,
            input_tokens: u64::MAX,
            ..Call::default()
        }]);
        let v = calls_json(&d.0, 0).unwrap();
        assert_eq!(v["secs_sum"], json!(-1.3));
        assert_eq!(v["secs_max"], json!(0.0));
        assert_eq!(v["cost_usd_estimate"], json!(-2.13));
        assert_eq!(v["input_tokens"], json!(u64::MAX));
    }
    #[test]
    fn floating_total_overflow_is_error() {
        for cost in [false, true] {
            let d = Scratch::new();
            let c = if cost {
                Call {
                    cost_usd: f64::MAX,
                    ..Call::default()
                }
            } else {
                Call {
                    secs: f64::MAX,
                    ..Call::default()
                }
            };
            d.calls(&[c.clone(), c]);
            assert!(
                calls_json(&d.0, 0).is_err(),
                "nonrepresentable floating total must fail"
            );
        }
    }
    #[test]
    fn collect_accounting_failure_preserves_artifacts() {
        for malformed in [true, false] {
            let d = Scratch::new();
            for f in ["qa.tsv", "rejects.tsv", "params.json"] {
                std::fs::write(d.0.join(f), "sentinel").unwrap();
            }
            if malformed {
                std::fs::write(d.0.join("calls.jsonl"), "{bad json}\n").unwrap();
            } else {
                d.calls(&[
                    Call {
                        input_tokens: u64::MAX,
                        ..Call::default()
                    },
                    Call {
                        input_tokens: 1,
                        ..Call::default()
                    },
                ]);
            }
            let pick = Pick {
                rule: 1,
                ..Pick::default()
            };
            assert!(collect(&d.0, &[], &pick).is_err());
            for f in ["qa.tsv", "rejects.tsv", "params.json"] {
                assert_eq!(
                    std::fs::read_to_string(d.0.join(f)).unwrap(),
                    "sentinel",
                    "preflight failure changed {f}"
                );
            }
        }
    }
    #[test]
    fn small_accounting_matches_independent_integer_oracle() {
        for n in 0..=12u64 {
            let d = Scratch::new();
            let calls: Vec<_> = (0..n)
                .map(|k| Call {
                    input_tokens: k,
                    output_tokens: 2 * k,
                    cache_read_tokens: 3 * k,
                    cache_write_tokens: 4 * k,
                    secs: k as f64 / 4.0,
                    cost_usd: k as f64 / 8.0,
                    ..Call::default()
                })
                .collect();
            d.calls(&calls);
            let v = calls_json(&d.0, 3).unwrap();
            let triangular = n.saturating_sub(1) * n / 2;
            for (key, mul) in [
                ("input_tokens", 1),
                ("output_tokens", 2),
                ("cache_read_tokens", 3),
                ("cache_write_tokens", 4),
            ] {
                assert_eq!(v[key], json!(mul * triangular));
            }
            assert_eq!(v["n"], json!(n));
            assert_eq!(v["retries"], json!(3));
            assert_eq!(
                v["secs_sum"],
                json!((triangular as f64 * 2.5).round() / 10.0)
            );
            assert_eq!(
                v["cost_usd_estimate"],
                json!((triangular as f64 * 12.5).round() / 100.0)
            );
        }
    }
    #[test]
    fn valid_collect_writes_matching_params_without_models() {
        let d = Scratch::new();
        let p = collect(
            &d.0,
            &[],
            &Pick {
                rule: 1,
                ..Pick::default()
            },
        )
        .unwrap();
        let saved: Value =
            serde_json::from_str(&std::fs::read_to_string(d.0.join("params.json")).unwrap())
                .unwrap();
        assert_eq!(p, saved);
        assert_eq!(p["calls"]["n"], json!(0));
        assert_eq!(p["pairs"]["n"], json!(0));
        assert!(
            std::fs::read_to_string(d.0.join("qa.tsv"))
                .unwrap()
                .starts_with("doc_key\t")
        );
    }
}
