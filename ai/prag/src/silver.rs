//! Pragmatics silver from the LLM: batch prompt (tales as paragraphs with 1–2 preceding sentences,
//! Tatoeba as single sentences), output is a compact TSV `n form act indirect hedge polarity voice
//! means`, gates and a rejects log. No blind retries: a rejection goes to the log, the call is not repeated;
//! a call error stops the run (finished batches stay, a rerun takes only the unfinished ones).

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use anyhow::{Context, Result, bail};

use crate::data::{Item, Src, has_quote};
use crate::opus::{Opus, append_call};
use crate::schema::{Labels, Voice, parse_labels};

/// Wave ceiling (brief: the first wave is at most 3000 sentences).
pub const WAVE_MAX: usize = 3000;

/// Annotation rules — the same for all batches (definitions of the first cut).
pub const RULES: &str = "You are an expert in English pragmatics. For each NUMBERED sentence, label what the speaker does and means. Use ONLY the listed values.

Output one line per numbered sentence, in order, 8 TAB-separated columns:
n	form	act	indirect	hedge	polarity	voice	means

form - sentence type of the labeled utterance:
  decl declarative | q yes/no or alternative question (also tag questions) | wh wh-question | imp imperative (also let's, don't) | excl exclamative (What a...! How...!) | frag fragment without a main finite clause (also vocative or answer fragments) | intj interjection only (Oh! Alas! Hello!) | other
act - speech act, what the speaker does:
  assert state, inform, explain, give an opinion (not story events) | narrate the narrator tells story events or describes the story world | ask seek information | request get the addressee to do something (order, command, beg, invite) | offer offer to do or give something | suggest propose an action (let's, you should, why don't we) | promise commit oneself to act (also vows) | thank | apologize | greet (also farewell, welcome) | agree assent, accept, consent, say yes | disagree refuse, reject, contradict, say no | evaluate praise, criticize, complain, judge good or bad | express feelings: joy, sorrow, surprise, wish, lament | warn warn, threaten, caution | other
indirect - 1 if the act is done through a form not typical of it: a question or statement used as a request, offer or suggestion (\"Can you open the window?\", \"I'd appreciate a reply\", \"It's cold in here\" = close the window), a rhetorical question used as a statement or evaluation, irony (literal meaning opposite to what is meant). Otherwise 0; explicit performatives (\"I promise...\", \"Thank you\") are 0.
hedge - 1 if the speaker softens or qualifies commitment: maybe, perhaps, I think, I suppose, I guess, it seems, probably, possibly, sort of, kind of, rather, a little, might or could of possibility, I'm afraid. Otherwise 0.
polarity - the speaker's evaluation or emotion: pos | neg | none (neutral facts). Grammatical negation alone is not neg (\"He did not come\" is none).
voice - whose utterance is labeled: character (direct speech of a character; also a quote inside a standalone sentence) | narrator (narration of a story, also reported indirect speech) | none (a standalone sentence outside any story).
  If a sentence contains direct speech framed by narration (\"'Pay me my wages,' said Hans.\"), label the quoted speech (voice character) and ignore the frame. Speech that continues from the previous sentence without new quote marks is still character. If an utterance has several clauses of different types, take the form of the clause that carries the main act.
means - what the speaker means beyond the literal words (indirect act, implicature, irony, presupposition), plain English, at most 15 words; for an implied request say who should do what. Write - if there is nothing beyond the literal content.

Examples (format only, not part of the task):
1	q	request	1	0	none	none	asks the listener to open the window
2	decl	assert	0	1	none	none	-
3	frag	evaluate	1	0	neg	character	the speaker is annoyed that it is Monday again
4	decl	narrate	0	0	none	narrator	-

Output ONLY these lines: no header, no explanations, no code fences. Every numbered sentence gets exactly one line; context sentences get none.
";

/// Batch prompt: rules and sentences numbered 1..N in batch order.
pub fn prompt(items: &[&Item]) -> String {
    let mut p = String::from(RULES);
    p.push_str("\nSentences:\n");
    let mut n = 0usize;
    let mut prev: Option<&Item> = None;
    let mut tat_header = false;
    for it in items {
        match it.src {
            Src::Tale | Src::Gum => {
                if it.pos == 0 {
                    let _ = write!(p, "\n=== Tale \"{}\" - paragraph {}\n", it.title, it.para);
                    // context only if the previous paragraph does not come right before in this same batch
                    let adjacent = prev.is_some_and(|q| q.doc == it.doc && q.para + 1 == it.para);
                    if !it.ctx.is_empty() && !adjacent {
                        let _ = writeln!(p, "context (previous sentences, do not label): {}", it.ctx.join(" "));
                    }
                }
            }
            Src::Tatoeba => {
                if !tat_header {
                    p.push_str("\n=== Standalone sentences (Tatoeba examples), no surrounding context\n");
                    tat_header = true;
                }
            }
        }
        n += 1;
        let _ = writeln!(p, "{n}\t{}", it.text);
        prev = Some(it);
    }
    p
}

/// Result of the batch gates: labels by number (from 1) and rejections (number, or None for the whole batch).
pub struct Gated {
    pub ok: BTreeMap<usize, Labels>,
    pub bad: Vec<(Option<usize>, String)>,
    /// lines that did not start with a number (junk)
    pub junk: usize,
}

/// Gates for the answer to a batch of `n` sentences:
/// - a line is a number and 7 TAB-separated columns, values from closed sets (`schema::parse_labels`);
/// - every sentence has a line (missing — that sentence is rejected);
/// - an extra or repeated number means the alignment is in doubt: the whole batch is rejected;
/// - voice consistent with the source: tale — narrator|character, Tatoeba without quotes — none.
pub fn gate(text: &str, items: &[&Item]) -> Gated {
    let n = items.len();
    let mut rows: BTreeMap<usize, Vec<Result<Labels, String>>> = BTreeMap::new();
    let mut junk = 0usize;
    let mut extra: Vec<String> = Vec::new();
    for line in text.lines() {
        let l = line.trim_end_matches('\r');
        if l.trim().is_empty() || l.trim_start().starts_with("```") {
            continue;
        }
        let cols: Vec<&str> = l.split('\t').collect();
        let Ok(k) = cols[0].trim().parse::<usize>() else {
            junk += 1;
            continue;
        };
        if k == 0 || k > n {
            extra.push(k.to_string());
            continue;
        }
        rows.entry(k).or_default().push(parse_labels(&cols[1..]));
    }
    let mut ok = BTreeMap::new();
    let mut bad = Vec::new();
    let dup: Vec<usize> = rows.iter().filter(|(_, v)| v.len() > 1).map(|(k, _)| *k).collect();
    if !extra.is_empty() || !dup.is_empty() {
        bad.push((None, format!("batch: extra numbers [{}], repeated [{}] — alignment in doubt", extra.join(","), dup.iter().map(usize::to_string).collect::<Vec<_>>().join(","))));
        return Gated { ok, bad, junk };
    }
    for (k, it) in items.iter().enumerate().map(|(i, it)| (i + 1, it)) {
        match rows.remove(&k).and_then(|mut v| v.pop()) {
            None => bad.push((Some(k), "no line".into())),
            Some(Err(e)) => bad.push((Some(k), e)),
            Some(Ok(l)) => match voice_gate(it, &l) {
                Err(e) => bad.push((Some(k), e)),
                Ok(()) => {
                    ok.insert(k, l);
                }
            },
        }
    }
    Gated { ok, bad, junk }
}

/// Voice consistent with the source.
fn voice_gate(it: &Item, l: &Labels) -> Result<(), String> {
    match it.src {
        Src::Tale if l.voice == Voice::None => Err("voice=none in a tale".into()),
        Src::Tatoeba if l.voice != Voice::None && !has_quote(&it.text) => Err(format!("voice={} in a Tatoeba sentence without quotes", l.voice)),
        _ => Ok(()),
    }
}

/// Batches by number, in sentence order.
pub fn batches(items: &[Item]) -> BTreeMap<usize, Vec<&Item>> {
    let mut b: BTreeMap<usize, Vec<&Item>> = BTreeMap::new();
    for it in items {
        b.entry(it.batch).or_default().push(it);
    }
    b
}

fn raw_path(dir: &Path, b: usize) -> PathBuf {
    dir.join("raw").join(format!("b{b:03}.txt"))
}

/// LLM run over batches `which` (empty — all), `jobs` calls in parallel. A batch that already has
/// raw output is skipped. A call error stops the run: no new batches are taken.
pub fn run(dir: &Path, items: &[Item], which: &[usize], jobs: usize) -> Result<()> {
    if items.len() > WAVE_MAX {
        bail!("{} sentences > {WAVE_MAX} — wave ceiling", items.len());
    }
    let all = batches(items);
    let todo: Vec<usize> = all.keys().copied().filter(|b| (which.is_empty() || which.contains(b)) && !raw_path(dir, *b).exists()).collect();
    std::fs::create_dir_all(dir.join("raw"))?;
    let opus = Opus::from_env(dir.join("cwd"));
    eprintln!("LLM {} ({}): batches to call {}, in parallel {jobs}", opus.model, opus.effort, todo.len());
    let next = AtomicUsize::new(0);
    let stop = AtomicBool::new(false);
    let errors: Mutex<Vec<String>> = Mutex::new(Vec::new());
    let log = Mutex::new(());
    std::thread::scope(|s| {
        for _ in 0..jobs.max(1) {
            s.spawn(|| {
                loop {
                    if stop.load(Ordering::SeqCst) {
                        break;
                    }
                    let i = next.fetch_add(1, Ordering::SeqCst);
                    let Some(&b) = todo.get(i) else { break };
                    let its = &all[&b];
                    let p = prompt(its);
                    let r = (|| -> Result<()> {
                        std::fs::write(dir.join("raw").join(format!("b{b:03}.prompt.txt")), &p)?;
                        let (text, call) = opus.ask("prag-silver", b, its.len(), &p)?;
                        let tmp = raw_path(dir, b).with_extension("part");
                        std::fs::write(&tmp, &text)?;
                        std::fs::rename(&tmp, raw_path(dir, b))?;
                        let _g = log.lock().unwrap();
                        append_call(&dir.join("calls.jsonl"), &call)?;
                        let g = gate(&text, its);
                        eprintln!("batch {b}: sentences {}, accepted {}, rejected {}; {:.0} s, output {} tok.", its.len(), g.ok.len(), its.len() - g.ok.len(), call.secs, call.output_tokens);
                        Ok(())
                    })();
                    if let Err(e) = r {
                        stop.store(true, Ordering::SeqCst);
                        errors.lock().unwrap().push(format!("batch {b}: {e:#}"));
                        break;
                    }
                }
            });
        }
    });
    let errors = errors.into_inner().unwrap();
    if !errors.is_empty() {
        bail!("stopped (fail-fast, no retries): {}", errors.join(" | "));
    }
    Ok(())
}

/// Gates over all raw outputs: `silver.tsv` (key and labels), `rejects.tsv` (key, batch,
/// reason). Returns (accepted, rejected, batches without output).
pub fn collect(dir: &Path, items: &[Item]) -> Result<(usize, usize, usize)> {
    let mut silver = String::from("sent_id\tform\tact\tindirect\thedge\tpolarity\tvoice\tmeans\n");
    let mut rej = String::from("sent_id\tbatch\treason\n");
    let (mut ok, mut bad, mut missing) = (0usize, 0usize, 0usize);
    for (b, its) in batches(items) {
        let p = raw_path(dir, b);
        if !p.exists() {
            missing += 1;
            continue;
        }
        let text = std::fs::read_to_string(&p).with_context(|| p.display().to_string())?;
        let g = gate(&text, &its);
        for (k, l) in &g.ok {
            let _ = writeln!(silver, "{}\t{}", its[k - 1].key, crate::schema::labels_tsv(l));
            ok += 1;
        }
        for (k, why) in &g.bad {
            match k {
                Some(k) => {
                    let _ = writeln!(rej, "{}\t{b}\t{why}", its[k - 1].key);
                    bad += 1;
                }
                None => {
                    for it in &its {
                        let _ = writeln!(rej, "{}\t{b}\t{why}", it.key);
                        bad += 1;
                    }
                }
            }
        }
        if g.junk > 0 {
            eprintln!("batch {b}: lines without a number {}", g.junk);
        }
    }
    std::fs::write(dir.join("silver.tsv"), silver)?;
    std::fs::write(dir.join("rejects.tsv"), rej)?;
    Ok((ok, bad, missing))
}

/// Read `silver.tsv`: key → labels.
pub fn read_silver(path: &Path) -> Result<BTreeMap<String, Labels>> {
    let text = std::fs::read_to_string(path).with_context(|| path.display().to_string())?;
    let mut m = BTreeMap::new();
    for (i, l) in text.lines().enumerate().skip(1) {
        let c: Vec<&str> = l.split('\t').collect();
        let lab = parse_labels(&c[1..]).map_err(|e| anyhow::anyhow!("{}:{}: {e}", path.display(), i + 1))?;
        m.insert(c[0].to_string(), lab);
    }
    Ok(m)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(key: &str, src: Src, para: i64, pos: usize, text: &str) -> Item {
        Item { key: key.into(), src, doc: "d".into(), title: "T".into(), para, ord: pos as i64 + 1, pos, plen: 2, text: text.into(), batch: 0, ctx: vec!["Before.".into()] }
    }

    #[test]
    fn gates_catch_misalignment_and_values() {
        let a = item("d:1", Src::Tale, 1, 0, "‘Can you open the window?’ said he.");
        let b = item("d:2", Src::Tale, 1, 1, "She opened it.");
        let c = item("tatoeba:1", Src::Tatoeba, 0, 0, "I think it might rain.");
        let its = vec![&a, &b, &c];
        let p = prompt(&its);
        assert!(p.contains("1\t‘Can you open") && p.contains("3\tI think") && p.contains("context (previous sentences, do not label): Before."));
        let good = "1\tq\trequest\t1\t0\tnone\tcharacter\tasks her to open the window\n2\tdecl\tnarrate\t0\t0\tnone\tnarrator\t-\n3\tdecl\tassert\t0\t1\tnone\tnone\t-\n";
        let g = gate(good, &its);
        assert_eq!((g.ok.len(), g.bad.len()), (3, 0));
        // negative controls
        // out-of-set value and a missing line — only those sentences are rejected
        let g = gate("1\tquestion\trequest\t1\t0\tnone\tcharacter\tx\n3\tdecl\tassert\t0\t1\tnone\tnone\t-\n", &its);
        assert_eq!((g.ok.len(), g.bad.len()), (1, 2));
        // extra number — the whole batch is rejected
        let g = gate(&format!("{good}4\tdecl\tassert\t0\t0\tnone\tnone\t-\n"), &its);
        assert_eq!((g.ok.len(), g.bad.len(), g.bad[0].0), (0, 1, None));
        // repeated number — same
        let g = gate(&format!("{good}2\tdecl\tnarrate\t0\t0\tnone\tnarrator\t-\n"), &its);
        assert_eq!(g.ok.len(), 0);
        // voice inconsistent with the source: tale with none, Tatoeba without quotes with narrator
        let g = gate("1\tq\trequest\t1\t0\tnone\tnone\tx\n2\tdecl\tnarrate\t0\t0\tnone\tnarrator\t-\n3\tdecl\tassert\t0\t1\tnone\tnarrator\t-\n", &its);
        assert_eq!((g.ok.len(), g.bad.len()), (1, 2));
    }
}
