//! Answers about a book that is not public domain: a short summary in our own words, never a quotation.
//! The book stays internal; retrieval finds the relevant sentences (event index + word overlap), Sonnet
//! (`prag::opus` call, on the subscription; `BOOK_ANSWER_MODEL` overrides) writes a short answer in its own words, and a copy gate rejects any run
//! of 7 words shared with the book and any quoted phrase of 4+ words. One rewrite with the copied phrases
//! named, then fail. Sentence ids used go to the internal log only.
//!
//!   world book-answer <book.txt> <question> [--index <book.events.tsv>] [--log <answers.jsonl>]

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::io::Write;
use std::path::Path;

use anyhow::{Context, Result, bail};
use en::gram::UPos;
use serde_json::json;

use crate::events::{book_body, sentences};

/// Shared-run length that counts as copying.
pub const COPY_RUN: usize = 7;
/// Words inside quotation marks that count as a quotation.
pub const QUOTE_WORDS: usize = 4;

fn words(s: &str) -> Vec<String> {
    s.split(|c: char| !c.is_alphanumeric() && c != '\'').filter(|w| !w.is_empty()).map(|w| w.trim_matches('\'').to_lowercase()).filter(|w| !w.is_empty()).collect()
}

fn hash(ws: &[String]) -> u64 {
    // FNV-1a over the words; membership only, so the hash order never reaches the output
    let mut h: u64 = 0xcbf29ce484222325;
    for w in ws {
        for b in w.bytes().chain(std::iter::once(b' ')) {
            h ^= b as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
    }
    h
}

/// The copy gate: runs of `COPY_RUN` words shared with the book, and quoted phrases of `QUOTE_WORDS`+ words.
pub struct Gate {
    grams: HashSet<u64>,
}

impl Gate {
    pub fn new(book: &str) -> Gate {
        let ws = words(book);
        Gate { grams: ws.windows(COPY_RUN).map(hash).collect() }
    }

    /// Problems of an answer (empty = passes).
    pub fn check(&self, answer: &str) -> Vec<String> {
        let ws = words(answer);
        let mut out = Vec::new();
        let mut i = 0;
        while i + COPY_RUN <= ws.len() {
            if self.grams.contains(&hash(&ws[i..i + COPY_RUN])) {
                // extend to the whole shared run
                let mut j = i + COPY_RUN;
                while j < ws.len() && self.grams.contains(&hash(&ws[j + 1 - COPY_RUN..=j])) {
                    j += 1;
                }
                out.push(format!("copied from the book: \"{}\"", ws[i..j].join(" ")));
                i = j;
            } else {
                i += 1;
            }
        }
        for (open, close) in [('"', '"'), ('\u{201c}', '\u{201d}'), ('\u{ab}', '\u{bb}')] {
            let mut rest = answer;
            while let Some(a) = rest.find(open) {
                let after = &rest[a + open.len_utf8()..];
                let Some(b) = after.find(close) else { break };
                if words(&after[..b]).len() >= QUOTE_WORDS {
                    out.push(format!("quotation: {open}{}{close}", &after[..b]));
                }
                rest = &after[b + close.len_utf8()..];
            }
        }
        out
    }
}

/// Content lemmas of the question (nouns, proper nouns, verbs, adjectives) plus its lowercase words.
fn query_terms(q: &str) -> Result<(BTreeSet<String>, BTreeSet<String>)> {
    let a = crate::tree::annotator()?;
    let forms: Vec<String> = a.tokenize(q).into_iter().map(|t| t.form).collect();
    let stop = ["be", "have", "do", "what", "who", "whom", "which", "how", "why", "when", "where", "book", "story", "happen", "tell"];
    let lemmas: BTreeSet<String> = a
        .annotate(&forms)
        .into_iter()
        .filter(|w| matches!(w.upos, UPos::NOUN | UPos::PROPN | UPos::VERB | UPos::ADJ))
        .map(|w| w.lemma.to_lowercase())
        .filter(|l| !stop.contains(&l.as_str()))
        .collect();
    let ws: BTreeSet<String> = words(q).into_iter().filter(|w| w.len() > 2 && !stop.contains(&w.as_str())).collect();
    Ok((lemmas, ws))
}

/// Sentence numbers relevant to the question: event-index hits weigh 2, word hits 1; top 30 plus neighbours.
fn retrieve(sents: &[String], index: Option<&Path>, lemmas: &BTreeSet<String>, ws: &BTreeSet<String>) -> Result<Vec<usize>> {
    let mut score: BTreeMap<usize, usize> = BTreeMap::new();
    if let Some(ix) = index {
        for line in std::fs::read_to_string(ix).with_context(|| format!("{}", ix.display()))?.lines().filter(|l| !l.starts_with('#') && !l.starts_with("sent\t")) {
            let c: Vec<&str> = line.split('\t').collect();
            let Some(k) = c.first().and_then(|id| id.rsplit_once(':')).and_then(|(_, n)| n.parse::<usize>().ok()) else { continue };
            // Ignore stale or malformed index positions before ranking or neighbour arithmetic.
            if k >= sents.len() {
                continue;
            }
            let hits = c[1..].iter().flat_map(|f| f.split(' ')).filter(|t| lemmas.contains(*t)).count();
            *score.entry(k).or_default() += 2 * hits;
        }
    }
    let terms: BTreeSet<&String> = lemmas.iter().chain(ws.iter()).collect();
    for (k, s) in sents.iter().enumerate() {
        let sw: BTreeSet<String> = words(s).into_iter().collect();
        let hits = terms.iter().filter(|t| sw.contains(t.as_str())).count();
        if hits > 0 {
            *score.entry(k).or_default() += hits;
        }
    }
    let mut ranked: Vec<(usize, usize)> = score.into_iter().filter(|x| x.1 > 0).collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    let mut pick: BTreeSet<usize> = BTreeSet::new();
    for (k, _) in ranked.into_iter().take(30) {
        for j in k.saturating_sub(1)..=(k + 1).min(sents.len().saturating_sub(1)) {
            pick.insert(j);
        }
    }
    Ok(pick.into_iter().take(90).collect())
}

fn prompt(question: &str, book: &str, notes: &[(String, &String)]) -> String {
    let mut p = format!(
        "You answer a reader's question about a book that is under copyright ({book}). The excerpts below are your private notes; \
the reader never sees them. Write a short answer, at most 120 words, entirely in your own words: do not quote, do not copy \
any phrase of more than four words from the excerpts, do not reproduce dialogue, use no quotation marks. Summarize; do not \
retell scene by scene. If the excerpts do not answer the question, say briefly that the book does not seem to say. \
Answer with the answer text only.\n\nQuestion: {question}\n\nPrivate notes (excerpts with sentence ids):\n"
    );
    for (id, s) in notes {
        p += &format!("[{id}] {s}\n");
    }
    p
}

/// `world book-answer`.
pub fn run(book: &Path, question: &str, index: Option<&Path>, log: Option<&Path>) -> Result<()> {
    // direct mode: the call goes through the subscription, not the API key
    // SAFETY: set before any thread is spawned in this process
    unsafe { std::env::set_var("MOVA_SUBSCRIPTION", "1") };
    let name = book.file_stem().map(|x| x.to_string_lossy().to_string()).unwrap_or_default();
    let text = std::fs::read_to_string(book).with_context(|| format!("{}", book.display()))?;
    let body = book_body(&text);
    let sents = sentences(body);
    let (lemmas, ws) = query_terms(question)?;
    let picked = retrieve(&sents, index, &lemmas, &ws)?;
    if picked.is_empty() {
        println!("The book does not seem to say anything about that.");
        return Ok(());
    }
    let notes: Vec<(String, &String)> = picked.iter().map(|&k| (format!("{name}:{k}"), &sents[k])).collect();
    let gate = Gate::new(body);
    // user-facing answers run on Sonnet (BOOK_ANSWER_MODEL overrides)
    let mut op = prag::opus::Opus::from_env(std::env::temp_dir().join("book-answer"));
    op.model = mova_teacher::model("BOOK_ANSWER_MODEL");
    let mut p = prompt(question, &name, &notes);
    let mut tries = Vec::new();
    for attempt in 1..=2 {
        let (ans, _call) = op.ask("book-answer", attempt, notes.len(), &p)?;
        let ans = ans.trim().to_string();
        let problems = gate.check(&ans);
        tries.push(json!({"attempt": attempt, "problems": problems}));
        if problems.is_empty() {
            println!("{ans}");
            if let Some(l) = log {
                let mut f = std::fs::OpenOptions::new().create(true).append(true).open(l)?;
                writeln!(f, "{}", json!({"book": name, "question": question, "terms": lemmas, "sentences": picked.iter().map(|k| format!("{name}:{k}")).collect::<Vec<_>>(), "answer": ans, "gate": tries}))?;
            }
            return Ok(());
        }
        eprintln!("copy gate, attempt {attempt}: {}", problems.join("; "));
        p += &format!("\n\nYour previous answer was rejected by the copy gate:\n{}\nRewrite it fully in your own words.\n\nPrevious answer:\n{ans}\n", problems.iter().map(|x| format!("- {x}")).collect::<Vec<_>>().join("\n"));
    }
    bail!("the copy gate rejected both answers; nothing is shown")
}

#[cfg(test)]
mod tests {
    use super::*;

    const BOOK: &str = "Once upon a time a sly fox saw a crow sitting on a high branch with a piece of cheese in her beak. \
The fox said that her voice must be the sweetest in the whole forest.";

    #[test]
    fn gate_catches_copies_and_quotes() {
        let g = Gate::new(BOOK);
        // negative control: a verbatim copy and a quotation must go red
        let p = g.check("A sly fox saw a crow sitting on a high branch, and flattered her.");
        assert!(p.iter().any(|x| x.starts_with("copied from the book")), "{p:?}");
        let p = g.check("The fox praises her: \"the sweetest voice of all\".");
        assert!(p.iter().any(|x| x.starts_with("quotation")), "{p:?}");
        // a paraphrase passes
        assert!(g.check("A cunning fox flatters a crow holding cheese, praising her singing, so she drops it.").is_empty());
    }
}

#[cfg(test)]
mod wave8_models {
    use super::*;
    #[test]
    fn gate_threshold_and_normalization_fibers() {
        let g = Gate::new("Alpha beta gamma delta epsilon zeta eta theta");
        for n in 0..=8 {
            let ws = words("alpha beta gamma delta epsilon zeta eta theta");
            assert_eq!(!g.check(&ws[..n].join(" ")).is_empty(), n >= 7);
        }
        for a in ["ALPHA beta gamma delta epsilon zeta eta", "alpha,beta gamma delta epsilon zeta eta", "'alpha' beta gamma delta epsilon zeta eta"] {
            assert_eq!(g.check(a), vec!["copied from the book: \"alpha beta gamma delta epsilon zeta eta\""]);
        }
        assert_eq!(words("l'été 'FOX' 42"), vec!["l'été", "fox", "42"]);
    }
    #[test]
    fn gate_extends_runs_and_checks_quote_pairs() {
        let g = Gate::new("a b c d e f g h");
        assert_eq!(g.check("a b c d e f g h").len(), 1);
        for (a, b) in [("\"", "\""), ("“", "”"), ("«", "»")] {
            assert_eq!(g.check(&format!("{a}one two three{b}")).len(), 0);
            assert_eq!(g.check(&format!("{a}one two three four{b}")).len(), 1);
            assert!(g.check(&format!("{a}one two three four")).is_empty());
        }
    }
    #[test]
    fn retrieval_matches_independent_bounded_word_oracle() {
        for mask in 0..64 {
            let s: Vec<String> = (0..6).map(|i| if mask & (1 << i) != 0 { "fox fox" } else { "crow" }.into()).collect();
            let terms = BTreeSet::from(["fox".to_string()]);
            let actual = retrieve(&s, None, &terms, &terms).unwrap();
            let expected: Vec<_> = (0usize..6).filter(|j| (0usize..6).any(|i| mask & (1 << i) != 0 && i.abs_diff(*j) <= 1)).collect();
            assert_eq!(actual, expected);
        }
    }
    #[test]
    fn retrieval_top_thirty_ties_and_empty_domain() {
        let terms = BTreeSet::from(["fox".to_string()]);
        let s = vec!["fox".into(); 100];
        assert_eq!(retrieve(&s, None, &terms, &terms).unwrap(), (0..31).collect::<Vec<_>>());
        assert!(retrieve(&[], None, &terms, &terms).unwrap().is_empty());
        assert!(retrieve(&s, None, &BTreeSet::new(), &BTreeSet::new()).unwrap().is_empty());
    }
    #[test]
    fn retrieval_rejects_invalid_index_positions() {
        let path = std::env::temp_dir().join(format!("mova-wave8-bookans-{}.tsv", std::process::id()));
        std::fs::write(&path, format!("fixture:{}\tfox\nfixture:4\tfox\nfixture:0\tfox fox\n", usize::MAX)).unwrap();
        let s = vec!["crow".into(); 4];
        let terms = BTreeSet::from(["fox".to_string()]);
        let got = retrieve(&s, Some(&path), &terms, &BTreeSet::new());
        let empty = retrieve(&[], Some(&path), &terms, &BTreeSet::new());
        // Invalid high scores must not crowd the valid row out of the top thirty.
        let mut crowded = (4..44).map(|k| format!("fixture:{k}\tfox fox fox\n")).collect::<String>();
        crowded.push_str("fixture:1\tfox\n");
        std::fs::write(&path, crowded).unwrap();
        let selected = retrieve(&s, Some(&path), &terms, &BTreeSet::new());
        std::fs::remove_file(path).unwrap();
        assert_eq!(got.unwrap(), vec![0, 1]);
        assert!(empty.unwrap().is_empty());
        assert_eq!(selected.unwrap(), vec![0, 1, 2]);
    }
    #[test]
    fn prompt_keeps_note_order_and_ids() {
        let a = "invented first note".to_string();
        let b = "invented second note".to_string();
        let p = prompt("Which animal?", "fixture", &[("fixture:2".into(), &a), ("fixture:0".into(), &b)]);
        assert!(p.contains("at most 120 words"));
        assert!(p.ends_with("[fixture:2] invented first note\n[fixture:0] invented second note\n"));
        assert!(p.contains("Question: Which animal?"));
    }
}
