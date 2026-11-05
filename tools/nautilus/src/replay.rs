//! Replay: measure the standing loop on copies, with the real answers of earlier teacher questions.
//!
//! `nautilus replay CONFIG OUT TERMINAL_ROOT…` reads every earlier question and answer of the given `mova-terminal`
//! roots (`tasks/<id>.md`, `answers/<id>.txt`), finds the configured layer each question was about (by its
//! dictionary name), and for each such layer writes a copy holding only the rows whose words were asked (`OUT/layers/`,
//! read only afterwards). It then runs the current loop over each copy (layers in name order, one shared sense store
//! `OUT/senses.jsonl`, no pacing) with a replay terminal: every word of a new question is answered from the word's real
//! answer for that layer — a draft's sense as `new: <sense>`, a layer's asked fields from the matching columns of the
//! old line, the pick `new` when a sense field is written, else `-` — and a word never answered before is left out.
//! Nothing reaches a teacher; the live folders are only read.
//!
//! The report: per layer and in all, the earlier questions (count, question bytes, words asked) against the replayed
//! ones (questions, bytes, words per question), the words Mova settled natively, the junk and thin words never asked,
//! and what the gates made of the replayed answers (picks kept as new senses, restatements of a numbered sense,
//! lines saying the word is absent, rejected answers).
use crate::animal::Config;
use crate::teacher::{self, PrivateTerminal, TerminalError};
use crate::work::Loop;
use anyhow::{Context, Result};
use serde_json::{Value, json};
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

/// One earlier question: the dictionary it named, its question bytes, its rendered task bytes and the words it asked.
#[derive(Clone, Debug)]
pub struct Earlier {
    pub name: String,
    pub question_bytes: usize,
    pub task_bytes: usize,
    pub words: Vec<String>,
}

/// The dictionary a question names: "… senses for Mova's NAME dictionary layer" (the earlier template), "… senses for
/// NAME, one of …", "… entries of NAME, one of …".
pub fn named(q: &str) -> Option<String> {
    let first = q.lines().next()?;
    for (a, b) in [("senses for Mova's ", " dictionary layer"), ("senses for ", ", one of"), ("entries of ", ", one of")] {
        if let Some(i) = first.find(a) {
            let rest = &first[i + a.len()..];
            if let Some(j) = rest.find(b) {
                return Some(rest[..j].to_string());
            }
        }
    }
    None
}

/// The earlier questions and their answers' lines (`(name, word)` → the answer's columns after the word).
pub fn earlier(roots: &[&Path]) -> Result<(Vec<Earlier>, BTreeMap<(String, String), Vec<String>>)> {
    let mut qs = Vec::new();
    let mut answers = BTreeMap::new();
    for root in roots {
        let mut files: Vec<_> = std::fs::read_dir(root.join("tasks")).with_context(|| format!("{}", root.display()))?.flatten().map(|e| e.path()).collect();
        files.sort();
        for f in files {
            let task = std::fs::read_to_string(&f)?;
            let Some(q) = task.split_once("\nQuestion:\n").map(|x| x.1) else { continue };
            let Some(name) = named(q) else { continue };
            let words: Vec<String> = q.lines().filter_map(|l| l.strip_prefix("word: ")).map(String::from).collect();
            let id = f.file_stem().and_then(|s| s.to_str()).unwrap_or("");
            if let Ok(a) = std::fs::read_to_string(root.join("answers").join(format!("{id}.txt"))) {
                for l in a.lines() {
                    let c: Vec<String> = l.split('\t').map(|x| x.trim().to_string()).collect();
                    if c.len() >= 2 && words.contains(&c[0]) {
                        answers.entry((name.clone(), c[0].clone())).or_insert_with(|| c[1..].to_vec());
                    }
                }
            }
            qs.push(Earlier { name, question_bytes: q.len(), task_bytes: task.len(), words });
        }
    }
    Ok((qs, answers))
}

/// The replay terminal: answers each new question from the earlier answers of one layer.
struct Replayer<'a> {
    id: String,
    name: String,
    /// the teacher fields of the layer's format (empty for a draft)
    fields: &'static [&'static str],
    answers: &'a BTreeMap<(String, String), Vec<String>>,
    asked: RefCell<Vec<(usize, usize)>>,
}

impl PrivateTerminal for Replayer<'_> {
    fn id(&self) -> &str {
        &self.id
    }
    fn ask(&self, q: &str) -> std::result::Result<String, TerminalError> {
        let mut out = String::new();
        let mut words = 0;
        let mut word: Option<String> = None;
        let flush = |w: &str, answer: Option<&str>, out: &mut String| {
            let Some(old) = self.answers.get(&(self.name.clone(), w.to_string())) else { return };
            match answer {
                // a draft: the earlier sense as a new one
                None => out.push_str(&format!("{w}\tnew: {}\n", old.first().map_or("", |s| s.as_str()))),
                Some(names) => {
                    let asked: Vec<&str> = names.split(", ").skip(1).collect();
                    let vals: Vec<String> = asked
                        .iter()
                        .map(|n| self.fields.iter().position(|f| f == n).and_then(|i| old.get(i)).cloned().unwrap_or_else(|| "-".into()))
                        .collect();
                    let sense = asked.iter().zip(&vals).any(|(n, v)| (n.starts_with("sense") || *n == "interpretation") && v != "-");
                    out.push_str(&format!("{w}\t{}\t{}\n", if sense { "new" } else { "-" }, vals.join("\t")));
                }
            }
        };
        for l in q.lines() {
            if let Some(w) = l.strip_prefix("word: ") {
                if let Some(prev) = word.take() {
                    if self.fields.is_empty() {
                        flush(&prev, None, &mut out);
                    }
                }
                word = Some(w.to_string());
                words += 1;
            } else if let Some(names) = l.strip_prefix("answer: ") {
                if let Some(w) = word.take() {
                    flush(&w, Some(names), &mut out);
                }
            }
        }
        if let Some(w) = word.take() {
            if self.fields.is_empty() {
                flush(&w, None, &mut out);
            }
        }
        self.asked.borrow_mut().push((q.len(), words));
        Ok(out)
    }
}

/// Replay every layer of `cfg` that the earlier questions were about; see the module.
pub fn run(cfg: &Config, out: &Path, roots: &[&Path]) -> Result<Value> {
    let (qs, answers) = earlier(roots)?;
    let mut report = BTreeMap::new();
    let mut total: BTreeMap<&str, f64> = BTreeMap::new();
    let wrapper: f64 = qs.iter().map(|q| (q.task_bytes - q.question_bytes) as f64).sum::<f64>() / qs.len().max(1) as f64;
    std::fs::create_dir_all(out.join("layers"))?;
    let store = out.join("senses.jsonl");
    for (key, l) in &cfg.layers {
        let mine: Vec<&Earlier> = qs.iter().filter(|q| q.name == l.name).collect();
        if mine.is_empty() {
            continue;
        }
        let asked: BTreeSet<&str> = mine.iter().flat_map(|q| q.words.iter().map(String::as_str)).collect();
        let file = l.rows_file().context("no layer file")?;
        let text = std::fs::read_to_string(file).with_context(|| format!("{}", file.display()))?;
        let copy: String = text.lines().filter(|r| r.starts_with('#') || r.split('\t').next().is_some_and(|w| asked.contains(w))).map(|r| format!("{r}\n")).collect();
        let copy_path = out.join("layers").join(format!("{key}.tsv"));
        std::fs::write(&copy_path, &copy)?;
        let mut c = cfg.clone();
        c.senses = Some(store.clone());
        let lc = c.layers.get_mut(key).context("layer")?;
        if lc.source == "layer" {
            lc.file = Some(copy_path.clone());
        } else {
            lc.out = Some(copy_path.clone());
        }
        let job = c.job(key, 0, 1)?;
        let state = out.join("state").join(key);
        let profile = teacher::Profile {
            name: "replay".into(),
            provider: "codex".into(),
            model: "replay".into(),
            terminal: format!("replay-{key}"),
            terminal_profile: out.join("replay-terminal.json"),
            purposes: vec!["dictionary".into()],
        };
        let t = teacher::Config { journal: state.join("teacher.jsonl"), profiles: vec![profile], operator_word: Some("replay of earlier answers, nothing is sent".into()) };
        let fields = match l.format.as_deref() {
            Some(f) => crate::improve::teacher_fields(f),
            None => &[],
        };
        let term = Replayer { id: format!("replay-{key}"), name: l.name.clone(), fields, answers: &answers, asked: RefCell::new(vec![]) };
        let lp = Loop { state: state.clone(), teachers: &t, profile: "replay".into(), min_gap_secs: 0, frozen: l.frozen(), ledger: None, member: "replay".into() };
        let mut steps = 0;
        loop {
            let v = lp.step(job.as_ref(), &term, 1_000 + steps)?;
            steps += 1;
            if matches!(v["event"].as_str(), Some("idle" | "stopped")) || steps > 10_000 {
                break;
            }
        }
        // what the proposals hold
        let (mut native, mut junk, mut thin, mut new, mut restates, mut picked, mut same) = (0, 0, 0, 0, 0, 0, 0);
        // a native pick against the earlier teacher's text: the sense that text shares most lemmas with
        let base = crate::layer::read_base(&std::fs::read_to_string(&l.base).unwrap_or_default());
        let (mut judged, mut agree) = (0, 0);
        let props = state.join("proposals");
        let mut names: Vec<_> = std::fs::read_dir(&props).map(|d| d.flatten().map(|e| e.path()).collect()).unwrap_or_default();
        names.sort();
        for p in &names {
            let text = std::fs::read_to_string(p)?;
            for line in text.lines().filter_map(|x| x.strip_prefix("# pick\t")) {
                let c: Vec<&str> = line.split('\t').collect();
                match (c.get(1).copied(), c.get(3).copied(), c.get(4).copied()) {
                    (pick, Some("native"), _) => {
                        native += 1;
                        let senses = base.get(c[0]).map(|b| crate::senses::parse(&b.1)).unwrap_or_default();
                        if let Some(old) = answers.get(&(l.name.clone(), c[0].to_string())) {
                            let mut sc = crate::senses::score(c[0], &[old.join(" ")], &senses);
                            sc.sort_by(|a, b| b.score.cmp(&a.score).then(a.sense.cmp(&b.sense)));
                            if sc.first().is_some_and(|b| b.score > 0 && sc.get(1).is_none_or(|n| n.score < b.score)) {
                                judged += 1;
                                agree += usize::from(pick == Some(sc[0].sense.to_string().as_str()));
                            }
                        }
                    }
                    (_, _, Some(e)) if e.contains("restates sense") => restates += 1,
                    (Some("new"), _, _) => new += 1,
                    (Some("same"), _, _) => same += 1,
                    _ => picked += 1,
                }
            }
            junk += text.lines().filter(|x| x.starts_with("# not asked\t") && !x.contains("example(s) use")).count();
            thin += text.lines().filter(|x| x.starts_with("# not asked\t") && x.contains("example(s) use")).count();
        }
        let sum = crate::work::summary(&state)?;
        let calls = term.asked.borrow();
        let (n_new, b_new, w_new) = (calls.len(), calls.iter().map(|c| c.0).sum::<usize>(), calls.iter().map(|c| c.1).sum::<usize>());
        let (n_old, b_old, w_old) = (mine.len(), mine.iter().map(|q| q.question_bytes).sum::<usize>(), mine.iter().map(|q| q.words.len()).sum::<usize>());
        let rows = copy.lines().filter(|r| !r.starts_with('#')).count();
        let v = json!({
            "rows_copied": rows,
            "earlier": {"questions": n_old, "question_bytes": b_old, "words_asked": w_old,
                "bytes_per_question": b_old as f64 / n_old.max(1) as f64, "words_per_question": w_old as f64 / n_old.max(1) as f64},
            "replayed": {"questions": n_new, "question_bytes": b_new, "words_asked": w_new,
                "bytes_per_question": b_new as f64 / n_new.max(1) as f64, "words_per_question": w_new as f64 / n_new.max(1) as f64,
                "bytes_per_word": b_new as f64 / w_new.max(1) as f64},
            "native_picks": native, "native_judged": judged, "native_agree": agree, "junk": junk, "thin": thin,
            "teacher": {"numbered": picked, "same": same, "new": new, "restates": restates},
            "events": sum["events"], "reasons": sum["rejection_and_hold_reasons"],
        });
        for (k, x) in [("rows", rows), ("old_q", n_old), ("old_b", b_old), ("old_w", w_old), ("new_q", n_new), ("new_b", b_new), ("new_w", w_new),
            ("native", native), ("judged", judged), ("agree", agree), ("junk", junk), ("thin", thin), ("numbered", picked), ("same", same), ("new", new), ("restates", restates)] {
            *total.entry(k).or_default() += x as f64;
        }
        report.insert(key.clone(), v);
    }
    let t = |k: &str| total.get(k).copied().unwrap_or(0.0);
    Ok(json!({
        "layers": report,
        "total": {
            "rows": t("rows"),
            "earlier": {"questions": t("old_q"), "words_asked": t("old_w"), "bytes_per_question": t("old_b") / t("old_q").max(1.0),
                "rendered_bytes_per_question": t("old_b") / t("old_q").max(1.0) + wrapper, "words_per_question": t("old_w") / t("old_q").max(1.0),
                "bytes_per_word": t("old_b") / t("old_w").max(1.0)},
            "replayed": {"questions": t("new_q"), "words_asked": t("new_w"), "bytes_per_question": t("new_b") / t("new_q").max(1.0),
                "rendered_bytes_per_question": t("new_b") / t("new_q").max(1.0) + wrapper, "words_per_question": t("new_w") / t("new_q").max(1.0),
                "bytes_per_word": t("new_b") / t("new_w").max(1.0)},
            "settled_natively": t("native"), "junk": t("junk"), "thin": t("thin"),
            "native_vs_earlier_teacher": {"judged": t("judged"), "agree": t("agree"),
                "note": "a native pick agrees when the earlier teacher's text shares the most lemmas with the same sense (a strict maximum); proxy, not gold"},
            "native_share_of_planned_words": t("native") / (t("native") + t("new_w")).max(1.0),
            "teacher": {"numbered": t("numbered"), "same": t("same"), "new": t("new"), "restates": t("restates")},
            "wrapper_bytes": wrapper,
        },
        "note": "replayed on copies with the earlier real answers; nothing was sent to a teacher"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dictionary_names_of_both_templates() {
        assert_eq!(named("You are writing senses for Mova's the commands dictionary dictionary layer, for a").as_deref(), Some("the commands dictionary"));
        assert_eq!(named("You are choosing senses for the commands dictionary, one of Mova's dictionary layers").as_deref(), Some("the commands dictionary"));
        assert_eq!(named("You are improving entries of the Judaism dictionary, one of Mova's dictionaries").as_deref(), Some("the Judaism dictionary"));
        assert!(named("Rewrite this lesson.").is_none());
    }
}
