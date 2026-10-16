//! Event statistics of books — a primitive summary of who did what to whom, from parse trees, counts only.
//! "The fox said to the crow" → `fox say →crow`; "the crow was silent" → `crow be silent`; "the cheese fell" →
//! `cheese fall`; "the tree held the crow" → `tree hold crow`. No text and no trees are stored, so the output is
//! a statistic usable for books outside the public domain too (analysis logs only, never corpus text).
//!
//!   world events <out-dir> <book.txt>... [--threads N]
//!
//! Writes `<out-dir>/books/<book>.json` (actors, events, interactions, most frequent first),
//! `<out-dir>/books/<book>.events.tsv` (the search index: sentence id `<book>:<n>` → event; ids point into the
//! text without storing it),
//! `<out-dir>/events.tsv` (corpus-wide events: agent, verb, patient, recipient, count, books) and
//! `<out-dir>/cells.tsv` (observed verb × noun × role counts — the empirical side of the absurdity matrix).
//! Nouns only (NOUN/PROPN lemmas); pronouns are skipped until coreference is wired in.

use std::collections::BTreeMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

use anyhow::Result;
use en::annotate::Word;
use en::gram::{Rel, UPos};
use serde_json::json;

/// One event: verb (with "not " / particle / "be <adj>"), agent, patient, recipient (nouns, may be empty).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Event {
    pub agent: String,
    pub verb: String,
    pub patient: String,
    pub recipient: String,
}

impl Event {
    pub fn show(&self) -> String {
        let mut s = format!("{} {}", if self.agent.is_empty() { "_" } else { &self.agent }, self.verb);
        if !self.patient.is_empty() {
            s += &format!(" {}", self.patient);
        }
        if !self.recipient.is_empty() {
            s += &format!(" →{}", self.recipient);
        }
        s
    }
}

fn noun(w: &Word) -> Option<String> {
    matches!(w.upos, UPos::NOUN | UPos::PROPN).then(|| w.lemma.to_lowercase()).filter(|l| l.chars().all(|c| c.is_alphabetic() || c == '-'))
}

/// Events of one parsed sentence (words with 1-based heads).
pub fn events(w: &[Word]) -> Vec<Event> {
    let kids = |h: usize| (0..w.len()).filter(move |&i| w[i].head == h + 1);
    let mut out = Vec::new();
    for (p, pw) in w.iter().enumerate() {
        let is_verb = pw.upos == UPos::VERB;
        let is_adj_pred = pw.upos == UPos::ADJ && kids(p).any(|i| w[i].rel == Rel::Cop);
        if !is_verb && !is_adj_pred {
            continue;
        }
        let passive = kids(p).any(|i| w[i].rel == Rel::AuxPass);
        let neg = kids(p).any(|i| w[i].rel == Rel::Advmod && matches!(w[i].lemma.to_lowercase().as_str(), "not" | "never" | "n't"));
        let find = |pred: &dyn Fn(&Word) -> bool| kids(p).find(|&i| pred(&w[i])).and_then(|i| noun(&w[i]));
        let (agent, patient) = if passive {
            (find(&|x| x.rel == Rel::OblAgent), find(&|x| x.rel == Rel::NsubjPass || x.rel == Rel::Nsubj))
        } else {
            (find(&|x| x.rel == Rel::Nsubj), if is_verb { find(&|x| x.rel == Rel::Obj) } else { None })
        };
        // recipient: iobj, or an obl with "to" ("said to the crow", "gave it to the fox")
        let recipient = find(&|x| x.rel == Rel::Iobj).or_else(|| {
            kids(p).find(|&i| w[i].rel == Rel::Obl && kids(i).any(|c| w[c].rel == Rel::Case && w[c].lemma.eq_ignore_ascii_case("to"))).and_then(|i| noun(&w[i]))
        });
        if agent.is_none() && patient.is_none() {
            continue;
        }
        let mut verb = if is_verb { pw.lemma.to_lowercase() } else { format!("be {}", pw.lemma.to_lowercase()) };
        if let Some(prt) = kids(p).find(|&i| w[i].rel == Rel::CompoundPrt) {
            verb += &format!(" {}", w[prt].lemma.to_lowercase());
        }
        if neg {
            verb = format!("not {verb}");
        }
        out.push(Event { agent: agent.unwrap_or_default(), verb, patient: patient.unwrap_or_default(), recipient: recipient.unwrap_or_default() });
    }
    out
}

/// The body of a Gutenberg text (between the START/END markers).
pub fn book_body(t: &str) -> &str {
    let start = t.find("*** START OF").and_then(|i| t[i..].find('\n').map(|j| i + j + 1)).unwrap_or(0);
    let end = t.find("*** END OF").unwrap_or(t.len());
    &t[start..end.max(start)]
}

/// Version of the sentence split; sentence ids `<book>:<n>` are stable only within one version.
pub const SPLIT: &str = "split-v2: CRLF normalised, paragraphs on blank lines, babi::split_sentences, 3-60 words, all-caps paragraphs skipped (v1 kept CRLF texts as one paragraph)";

/// Sentences of a book body (paragraphs, then `split_sentences`), 3–60 words, all-caps headings skipped.
pub fn sentences(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    // Gutenberg texts end lines with CRLF: without normalising, a blank line is "\r\n\r\n" and a whole book was one
    // paragraph (found by the tree store: "paragraph 1, sentence 1684")
    let body = body.replace("\r\n", "\n");
    for para in body.split("\n\n") {
        let para = para.split_whitespace().collect::<Vec<_>>().join(" ");
        if para.len() < 20 || para.chars().filter(|c| c.is_uppercase()).count() * 2 > para.len() {
            continue;
        }
        out.extend(crate::babi::split_sentences(&para).into_iter().filter(|s| (3..=60).contains(&s.split_whitespace().count())));
    }
    out
}

#[derive(Default)]
struct BookStats {
    sentences: usize,
    events: BTreeMap<Event, usize>,
    /// (sentence number, event) — the search index of the book
    postings: Vec<(usize, Event)>,
}

fn book_stats(a: &en::annotate::Annotator, path: &Path) -> Option<BookStats> {
    let t = std::fs::read_to_string(path).ok()?;
    let mut st = BookStats::default();
    for (k, s) in sentences(book_body(&t)).into_iter().enumerate() {
        let forms: Vec<String> = a.tokenize(&s).into_iter().map(|t| t.form).collect();
        if forms.is_empty() {
            continue;
        }
        st.sentences += 1;
        let mut ws = a.annotate(&forms);
        crate::rerank::repair_words(&mut ws);
        for e in events(&ws) {
            *st.events.entry(e.clone()).or_default() += 1;
            st.postings.push((k, e));
        }
    }
    Some(st)
}

fn top<K: Clone + Ord>(m: &BTreeMap<K, usize>, n: usize) -> Vec<(K, usize)> {
    let mut v: Vec<(K, usize)> = m.iter().map(|(k, c)| (k.clone(), *c)).collect();
    v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    v.truncate(n);
    v
}

fn book_json(name: &str, st: &BookStats) -> serde_json::Value {
    let mut actors: BTreeMap<String, usize> = BTreeMap::new();
    let mut inter: BTreeMap<String, usize> = BTreeMap::new();
    for (e, n) in &st.events {
        for x in [&e.agent, &e.patient, &e.recipient].into_iter().filter(|x| !x.is_empty()) {
            *actors.entry(x.clone()).or_default() += n;
        }
        for other in [&e.patient, &e.recipient].into_iter().filter(|x| !x.is_empty() && !e.agent.is_empty() && **x != e.agent) {
            let (a, b) = if e.agent < *other { (&e.agent, other) } else { (other, &e.agent) };
            *inter.entry(format!("{a} ~ {b}")).or_default() += n;
        }
    }
    json!({
        "book": name,
        "sentences": st.sentences,
        "events_total": st.events.values().sum::<usize>(),
        "actors": top(&actors, 30),
        "interactions": top(&inter, 30),
        "events": top(&st.events, 60).into_iter().map(|(e, n)| (e.show(), n)).collect::<Vec<_>>(),
    })
}

/// `world events`.
pub fn run(out: &Path, books: &[PathBuf], threads: usize) -> Result<()> {
    let a = crate::tree::annotator()?;
    std::fs::create_dir_all(out.join("books"))?;
    // corpus-wide: event → (count, books); cell (verb, noun, role) → (count, books)
    let all: Mutex<BTreeMap<Event, (usize, usize)>> = Mutex::new(BTreeMap::new());
    let cells: Mutex<BTreeMap<(String, String, &'static str), (usize, usize)>> = Mutex::new(BTreeMap::new());
    let (next, done, sents) = (AtomicUsize::new(0), AtomicUsize::new(0), AtomicUsize::new(0));
    std::thread::scope(|sc| {
        for _ in 0..threads.max(1) {
            sc.spawn(|| {
                loop {
                    let i = next.fetch_add(1, Ordering::SeqCst);
                    let Some(b) = books.get(i) else { break };
                    let name = b.file_stem().map(|x| x.to_string_lossy().to_string()).unwrap_or_default();
                    let dest = out.join("books").join(format!("{name}.json"));
                    let Some(st) = book_stats(a, b) else { continue };
                    let _ = std::fs::write(&dest, serde_json::to_string_pretty(&book_json(&name, &st)).unwrap_or_default());
                    let mut idx = format!("# sentence ids <book>:<n>, n = index in world::events::sentences ({SPLIT})\nsent\tagent\tverb\tpatient\trecipient\n");
                    for (k, e) in &st.postings {
                        idx += &format!("{name}:{k}\t{}\t{}\t{}\t{}\n", e.agent, e.verb, e.patient, e.recipient);
                    }
                    let _ = std::fs::write(out.join("books").join(format!("{name}.events.tsv")), idx);
                    sents.fetch_add(st.sentences, Ordering::SeqCst);
                    {
                        let mut all = all.lock().unwrap();
                        for (e, n) in &st.events {
                            let x = all.entry(e.clone()).or_default();
                            x.0 += n;
                            x.1 += 1;
                        }
                    }
                    let mut per: BTreeMap<(String, String, &'static str), usize> = BTreeMap::new();
                    for (e, n) in &st.events {
                        let v = e.verb.clone();
                        for (noun, role) in [(&e.agent, "SUBJ"), (&e.patient, "OBJ"), (&e.recipient, "TO")] {
                            if !noun.is_empty() {
                                *per.entry((v.clone(), noun.clone(), role)).or_default() += n;
                            }
                        }
                    }
                    {
                        let mut cells = cells.lock().unwrap();
                        for (k, n) in per {
                            let x = cells.entry(k).or_default();
                            x.0 += n;
                            x.1 += 1;
                        }
                    }
                    let d = done.fetch_add(1, Ordering::SeqCst) + 1;
                    if d % 25 == 0 {
                        eprintln!("[{d}/{}] books, {} sentences", books.len(), sents.load(Ordering::SeqCst));
                    }
                }
            });
        }
    });
    let all = all.into_inner().unwrap();
    let cells = cells.into_inner().unwrap();
    let mut ev: Vec<(&Event, &(usize, usize))> = all.iter().filter(|x| x.1.0 >= 2).collect();
    ev.sort_by(|a, b| b.1.0.cmp(&a.1.0).then(a.0.cmp(b.0)));
    let mut f = std::io::BufWriter::new(std::fs::File::create(out.join("events.tsv"))?);
    writeln!(f, "agent\tverb\tpatient\trecipient\tcount\tbooks")?;
    for (e, (n, b)) in &ev {
        writeln!(f, "{}\t{}\t{}\t{}\t{n}\t{b}", e.agent, e.verb, e.patient, e.recipient)?;
    }
    f.flush()?;
    let mut cv: Vec<_> = cells.iter().collect();
    cv.sort_by(|a, b| b.1.0.cmp(&a.1.0).then(a.0.cmp(b.0)));
    let mut f = std::io::BufWriter::new(std::fs::File::create(out.join("cells.tsv"))?);
    writeln!(f, "verb\tnoun\trole\tcount\tbooks")?;
    for ((v, n, r), (c, b)) in &cv {
        writeln!(f, "{v}\t{n}\t{r}\t{c}\t{b}")?;
    }
    f.flush()?;
    println!("books {}  sentences {}  distinct events {} ({} seen at least twice)  cells {}", done.load(Ordering::SeqCst), sents.load(Ordering::SeqCst), all.len(), ev.len(), cells.len());
    for (e, (n, b)) in ev.iter().take(25) {
        println!("  {n:6} {b:4} books  {}", e.show());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use en::gram::{Feats, Tag};

    fn w(form: &str, lemma: &str, upos: UPos, head: usize, rel: Rel) -> Word {
        Word { form: form.into(), lemma: lemma.into(), upos, tag: Tag::NN, feats: Feats::default(), head, rel }
    }

    #[test]
    fn fox_crow_cheese() {
        // "The fox said to the crow"
        let s = [w("The", "the", UPos::DET, 2, Rel::Det), w("fox", "fox", UPos::NOUN, 3, Rel::Nsubj), w("said", "say", UPos::VERB, 0, Rel::Root), w("to", "to", UPos::ADP, 6, Rel::Case), w("the", "the", UPos::DET, 6, Rel::Det), w("crow", "crow", UPos::NOUN, 3, Rel::Obl)];
        assert_eq!(events(&s).iter().map(Event::show).collect::<Vec<_>>(), vec!["fox say →crow"]);
        // "The crow was silent"
        let s = [w("The", "the", UPos::DET, 2, Rel::Det), w("crow", "crow", UPos::NOUN, 4, Rel::Nsubj), w("was", "be", UPos::AUX, 4, Rel::Cop), w("silent", "silent", UPos::ADJ, 0, Rel::Root)];
        assert_eq!(events(&s).iter().map(Event::show).collect::<Vec<_>>(), vec!["crow be silent"]);
        // "The cheese fell down"
        let s = [w("The", "the", UPos::DET, 2, Rel::Det), w("cheese", "cheese", UPos::NOUN, 3, Rel::Nsubj), w("fell", "fall", UPos::VERB, 0, Rel::Root), w("down", "down", UPos::ADP, 3, Rel::CompoundPrt)];
        assert_eq!(events(&s).iter().map(Event::show).collect::<Vec<_>>(), vec!["cheese fall down"]);
        // "The tree held the crow"; pronoun subject → no agent: "She did not eat the cheese" → "_ not eat cheese"
        let s = [w("tree", "tree", UPos::NOUN, 2, Rel::Nsubj), w("held", "hold", UPos::VERB, 0, Rel::Root), w("crow", "crow", UPos::NOUN, 2, Rel::Obj)];
        assert_eq!(events(&s)[0].show(), "tree hold crow");
        let s = [w("She", "she", UPos::PRON, 4, Rel::Nsubj), w("did", "do", UPos::AUX, 4, Rel::Aux), w("not", "not", UPos::PART, 4, Rel::Advmod), w("eat", "eat", UPos::VERB, 0, Rel::Root), w("cheese", "cheese", UPos::NOUN, 4, Rel::Obj)];
        assert_eq!(events(&s)[0].show(), "_ not eat cheese");
        // negative control: no noun argument → no event
        let s = [w("She", "she", UPos::PRON, 2, Rel::Nsubj), w("ran", "run", UPos::VERB, 0, Rel::Root)];
        assert!(events(&s).is_empty());
    }
}

/// `world events-show <book.txt> <n>...`: the sentences with the given numbers (ids `<book>:<n>` of the index),
/// for debugging the event statistics on public-domain books.
pub fn show(book: &Path, ns: &[usize]) -> Result<()> {
    let t = std::fs::read_to_string(book)?;
    let ss = sentences(book_body(&t));
    for &n in ns {
        println!("{n}\t{}", ss.get(n).map(String::as_str).unwrap_or("?"));
    }
    Ok(())
}
