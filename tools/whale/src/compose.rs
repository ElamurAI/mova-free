//! The Whale's description, written natively from its reading and its retained knowledge.
//!
//! Shape: an opening (title, size, parts, main topics) -> for every readable part, a lead that
//! connects it to the previous part and states its size and topics, then the part's sentences
//! restated as whole clauses (`clause.rs`) in source order, each term explained right after its
//! first use from the Whale's retained knowledge (`knowledge::explain`), sentences the Whale learned
//! to read through a question restated from that knowledge, and the numbers the part reports ->
//! a closing summary.
//!
//! Every sentence carries its basis: the source sentences it restates (`source`, `reading`), the
//! knowledge items it realizes (`explain`), or the counts and titles it states (`structure`). A
//! sentence the guard refuses (verbatim run over the cap anywhere in the paper, terminal-answer
//! words) or that nearly repeats an earlier one is not written. Nothing is padded: over the word
//! budget, every part keeps the same share of its restated sentences (evenly spaced); under the
//! minimum, the description stays short and the gate says so.
use crate::gates::{near_duplicate, Guard};
use crate::knowledge::{self, Item as KItem};
use crate::reader::{content, top, Reading, STOP};
use mova_octopus::realize::{list, sentence, thousands};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

/// The version of the native reader and writer (`reader.rs`, `clause.rs`, `compose.rs`, the answer
/// reader in `knowledge.rs`). A batch retries a paper that stopped unpassed only under a different
/// version (`batch.rs`): change it with every change that can change a description.
pub const WRITER: &str = "writer-2026-11-04d";

/// Words the structural templates and explanations add, besides their basis.
pub const FRAME: [&str; 64] = [
    "read", "has", "words", "sentences", "that", "could", "restate", "them", "parts", "are", "things", "speaks", "speak", "most", "mostly",
    "paper", "opens", "where", "about", "after", "turns", "which", "give", "gives", "numbers", "include", "sum", "across", "explained",
    "terms", "from", "what", "learned", "yet", "there", "part", "short", "understand", "take", "mean", "authors", "their", "theirs",
    "themselves", "the", "its", "and", "with", "this", "these", "not", "have", "had", "learnt", "now", "goes", "last", "closes", "first",
    "word", "sentence", "of", "shows", "asks",
];

/// One written sentence and its basis.
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct Item {
    pub text: String,
    /// `structure`, `source`, `explain`, `reading` or `world` (a clause restated from a text of
    /// the document's neighbourhood, `compose_epoch`).
    pub kind: &'static str,
    /// Sentence indices of the reading the clause restates (for `world`: of its neighbour text).
    pub sources: Vec<usize>,
    pub knowledge: Vec<String>,
    pub section: Option<usize>,
    /// For `structure`: the titles, topics and counts it states.
    pub basis: String,
    /// For `world`: the citation id of the neighbour text it restates.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cites: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Draft {
    pub text: String,
    pub items: Vec<Item>,
    pub words: usize,
    /// Words of all restatable clauses before the budget.
    pub available_words: usize,
}

/// What the writer may use from the Whale's knowledge for this paper.
#[derive(Default)]
pub struct Known {
    /// Normalized term -> its explanation sentences: (knowledge item, fact relation, sentence),
    /// at most one per relation.
    pub glosses: BTreeMap<String, Vec<(String, String, String)>>,
    /// Source sentence index -> (item id, restatements).
    pub readings: BTreeMap<usize, (String, Vec<String>)>,
    /// Section index -> (item id, restatements).
    pub sections: BTreeMap<usize, (String, Vec<String>)>,
}

impl Known {
    /// Knowledge usable for this reading: term facts from any paper (the latest item per term),
    /// the paper's own acronym definitions, sentence and section readings of this paper.
    pub fn of(r: &Reading, paper: &str, items: &[KItem]) -> (Self, Vec<KItem>) {
        let mut k = Known::default();
        let mut native = vec![];
        for (acr, (exp, s)) in &r.definitions {
            let item = KItem {
                id: format!("p-{}-{acr}", s),
                whale: String::new(),
                kind: "term".into(),
                key: acr.clone(),
                paper: paper.into(),
                facts: vec![knowledge::Fact { relation: "stands_for".into(), subject: String::new(), verb: String::new(), object: exp.clone() }],
                provenance: serde_json::json!({"source":"paper","sentence":s}),
                unix: 0,
            };
            k.glosses.insert(knowledge::norm_term(acr), knowledge::explain(acr, &item.facts).into_iter().map(|e| (item.id.clone(), "stands_for".to_string(), e)).collect());
            native.push(item);
        }
        for it in items {
            match it.kind.as_str() {
                // Bare-category facts («a surname», «the label») are kept for review, never written.
                "term" if it.facts.iter().any(|f| !knowledge::bare(f)) => {
                    let facts: Vec<knowledge::Fact> = it.facts.iter().filter(|f| !knowledge::bare(f)).cloned().collect();
                    let display = r.forms.get(&knowledge::norm_term(&it.key)).cloned().unwrap_or_else(|| it.key.clone());
                    let key = knowledge::norm_term(&it.key);
                    let have = k.glosses.entry(key).or_default();
                    // The first fact of each relation stays (the paper's own expansion first);
                    // later items, of this Whale or of the pod, add only relations still missing,
                    // each sentence bound to its own item.
                    for f in &facts {
                        if have.iter().any(|(_, rel, _)| rel == &f.relation) {
                            continue;
                        }
                        if let Some(e) = knowledge::explain(&display, std::slice::from_ref(f)).into_iter().next() {
                            have.push((it.id.clone(), f.relation.clone(), e));
                        }
                    }
                }
                "sentence" if it.paper == paper => {
                    if let Some(i) = r.sentences.iter().position(|s| s.text == it.key) {
                        let v: Vec<String> = it.facts.iter().filter_map(knowledge::restate).collect();
                        if !v.is_empty() {
                            k.readings.insert(i, (it.id.clone(), v));
                        }
                    }
                }
                "section" if it.paper == paper => {
                    if let Some(i) = r.sections.iter().position(|s| s.title == it.key) {
                        let v: Vec<String> = it.facts.iter().filter_map(knowledge::restate).collect();
                        if !v.is_empty() {
                            k.sections.insert(i, (it.id.clone(), v));
                        }
                    }
                }
                _ => {}
            }
        }
        (k, native)
    }
}

fn words(s: &str) -> usize {
    crate::contract::word_count(s)
}

/// Numbers a part reports: decimals, percentages and figures of two or more digits, not years.
fn numbers(text: &str) -> Vec<String> {
    let mut out = vec![];
    for w in text.split_whitespace() {
        let w = w.trim_matches(|c: char| !(c.is_ascii_digit() || c == '%' || c == '.'));
        let w = w.trim_end_matches('.');
        if w.is_empty() || !w.chars().next().is_some_and(|c| c.is_ascii_digit()) || !w.chars().all(|c| c.is_ascii_digit() || c == '.' || c == '%') {
            continue;
        }
        let year = w.len() == 4 && (w.starts_with("19") || w.starts_with("20")) && w.chars().all(|c| c.is_ascii_digit());
        if !year && (w.contains('.') || w.contains('%') || w.len() >= 2) && !out.contains(&w.to_string()) {
            out.push(w.to_string());
        }
    }
    out
}

struct Writer<'a> {
    guard: &'a Guard,
    items: Vec<Item>,
    sets: Vec<BTreeSet<String>>,
    explained: BTreeSet<String>,
}

impl Writer<'_> {
    /// Keep a sentence unless the guard refuses it or it nearly repeats an earlier one.
    fn push(&mut self, it: Item) -> bool {
        let set: BTreeSet<String> = content(&it.text).into_iter().collect();
        let admitted = if it.kind == "structure" { self.guard.admits_structure(&it.text) } else { self.guard.admits(&it.text) };
        // Restated sentences pass the grammar gate (`clause::grammatical`) before they are written.
        let admitted = admitted && (!matches!(it.kind, "source" | "reading" | "world") || self.guard.grammatical(&it.text));
        if it.text.trim().is_empty() || !admitted || self.sets.iter().any(|s| near_duplicate(s, &set, self.guard.params.near_duplicate)) || self.items.iter().any(|x| x.text.eq_ignore_ascii_case(&it.text)) {
            return false;
        }
        self.sets.push(set);
        self.items.push(it);
        true
    }
    fn structure(&mut self, text: String, basis: String, section: Option<usize>) {
        self.push(Item { text: sentence(&text), kind: "structure", sources: vec![], knowledge: vec![], section, basis, cites: vec![] });
    }
    /// Explain the terms of a written sentence that are not explained yet.
    fn explain(&mut self, text: &str, known: &Known, section: Option<usize>) {
        let toks: Vec<&str> = text.split(|c: char| !(c.is_alphanumeric() || c == '-')).filter(|w| !w.is_empty()).collect();
        for w in toks {
            let key = knowledge::norm_term(w);
            if self.explained.contains(&key) {
                continue;
            }
            if let Some(ex) = known.glosses.get(&key) {
                self.explained.insert(key);
                for (id, _, e) in ex {
                    self.push(Item { text: e.clone(), kind: "explain", sources: vec![], knowledge: vec![id.clone()], section, basis: String::new(), cites: vec![] });
                }
            }
        }
    }
}

fn section_nouns(r: &Reading, sec: usize, n: usize) -> Vec<String> {
    let mut m: BTreeMap<String, usize> = BTreeMap::new();
    for &i in &r.sections[sec].sentences {
        for x in &r.sentences[i].nouns {
            if !STOP.contains(&x.as_str()) && x.chars().all(|c| c.is_alphabetic() || c == '-') {
                *m.entry(x.clone()).or_insert(0) += 1;
            }
        }
    }
    top(&m, n).into_iter().filter(|(_, c)| **c >= 2).map(|(k, _)| k.clone()).collect()
}

/// Write with the clauses `keep` admits (sentence index, clause index).
fn write_with(r: &Reading, known: &Known, guard: &Guard, keep: &dyn Fn(usize, usize) -> bool) -> Vec<Item> {
    let mut w = Writer { guard, items: vec![], sets: vec![], explained: BTreeSet::new() };
    let readable: Vec<usize> = (0..r.sections.len()).filter(|&i| !r.sections[i].skipped && !r.sections[i].sentences.is_empty()).collect();
    let restatable = r.sentences.iter().filter(|s| r.sections[s.section].skipped || !s.clauses.is_empty() || known.readings.contains_key(&s.index)).count();
    let titles: Vec<String> = readable.iter().map(|&i| format!("«{}»", r.sections[i].title)).collect();
    let title = if r.title.is_empty() { "an untitled paper".to_string() } else { format!("«{}»", r.title) };
    let nouns: Vec<String> = top(&r.nouns, 8).into_iter().map(|(k, _)| k.clone()).collect();
    w.structure(format!("I read {title}"), r.title.clone(), None);
    w.structure(
        format!("It has {} words; of the {} sentences I read in it, I could restate {}", thousands(r.words as u64), r.sentences.len(), restatable),
        format!("{} {} {}", thousands(r.words as u64), r.sentences.len(), restatable),
        None,
    );
    if !titles.is_empty() {
        w.structure(format!("The parts I read are {}", list(&titles)), titles.join(" "), None);
    }
    if !nouns.is_empty() {
        w.structure(format!("The things it speaks of most are {}", list(&nouns)), nouns.join(" "), None);
    }
    let mut prev: Option<usize> = None;
    for &sec in &readable {
        let s = &r.sections[sec];
        let topics = section_nouns(r, sec, 3);
        let about = if topics.is_empty() { String::new() } else { format!(", mostly about {}", list(&topics)) };
        let lead = match prev {
            None => format!("The paper opens with «{}», where I read {} sentences{about}", s.title, s.sentences.len()),
            Some(p) => format!("After «{}», the paper turns to «{}»: {} sentences{about}", r.sections[p].title, s.title, s.sentences.len()),
        };
        let basis = format!("{} {} {} {}", s.title, prev.map(|p| r.sections[p].title.as_str()).unwrap_or(""), s.sentences.len(), topics.join(" "));
        w.structure(lead, basis, Some(sec));
        prev = Some(sec);
        let mut used: Vec<usize> = vec![];
        for &i in &s.sentences {
            let st = &r.sentences[i];
            let mut wrote = false;
            for (c, cl) in st.clauses.iter().enumerate() {
                if !keep(i, c) {
                    continue;
                }
                if w.push(Item { text: cl.text.clone(), kind: "source", sources: vec![i], knowledge: vec![], section: Some(sec), basis: String::new(), cites: vec![] }) {
                    wrote = true;
                    w.explain(&cl.text, known, Some(sec));
                }
            }
            if !wrote {
                if let Some((id, v)) = known.readings.get(&i) {
                    for t in v {
                        if w.push(Item { text: t.clone(), kind: "reading", sources: vec![i], knowledge: vec![id.clone()], section: Some(sec), basis: String::new(), cites: vec![] }) {
                            wrote = true;
                            w.explain(t, known, Some(sec));
                        }
                    }
                }
            }
            if wrote {
                used.push(i);
            }
        }
        if let Some((id, v)) = known.sections.get(&sec) {
            let all: Vec<usize> = s.sentences.clone();
            for t in v {
                w.push(Item { text: t.clone(), kind: "reading", sources: all.clone(), knowledge: vec![id.clone()], section: Some(sec), basis: String::new(), cites: vec![] });
            }
        }
        let text: String = used.iter().map(|&i| r.sentences[i].text.as_str()).collect::<Vec<_>>().join(" ");
        let nums: Vec<String> = numbers(&text).into_iter().take(6).collect();
        if nums.len() >= 2 {
            w.structure(format!("The numbers «{}» gives include {}", s.title, list(&nums)), format!("{} {}", s.title, nums.join(" ")), Some(sec));
        }
    }
    if readable.len() >= 2 {
        let explained = w.items.iter().filter(|i| i.kind == "explain").flat_map(|i| i.knowledge.iter()).collect::<BTreeSet<_>>().len();
        let last = &r.sections[*readable.last().unwrap_or(&0)].title;
        let main: Vec<String> = nouns.iter().take(4).cloned().collect();
        w.structure(format!("The paper closes with «{last}»; across its {} parts, it speaks mostly about {}", readable.len(), list(&main)), format!("{last} {} {}", readable.len(), main.join(" ")), None);
        w.structure(
            format!("I explained {explained} of its terms from what I have learned, and {} of its {} sentences I could not restate yet", r.sentences.len() - restatable, r.sentences.len()),
            format!("{explained} {} {}", r.sentences.len() - restatable, r.sentences.len()),
            None,
        );
    }
    w.items
}

fn total(items: &[Item]) -> usize {
    items.iter().map(|i| words(&i.text)).sum()
}

/// Write the description within `max` words.
pub fn compose(r: &Reading, known: &Known, guard: &Guard, max: usize) -> Draft {
    let all = write_with(r, known, guard, &|_, _| true);
    let available: usize = all.iter().filter(|i| i.kind == "source").map(|i| words(&i.text)).sum();
    let mut items = all.clone();
    if total(&all) > max {
        // The same share of every part's clauses, evenly spaced.
        let fixed = total(&all) - available;
        let share = max.saturating_sub(fixed) as f64 / available.max(1) as f64;
        let mut chosen: BTreeSet<(usize, usize)> = BTreeSet::new();
        for sec in &r.sections {
            let cl: Vec<(usize, usize)> = sec.sentences.iter().flat_map(|&i| (0..r.sentences[i].clauses.len()).map(move |c| (i, c))).collect();
            if cl.is_empty() {
                continue;
            }
            let keep = ((cl.len() as f64 * share).floor() as usize).max(1).min(cl.len());
            for j in 0..keep {
                chosen.insert(cl[j * cl.len() / keep]);
            }
        }
        items = write_with(r, known, guard, &|i, c| chosen.contains(&(i, c)));
        // Whole restated sentences come off the end until the budget holds.
        while total(&items) > max {
            match items.iter().rposition(|i| i.kind == "source") {
                Some(p) => {
                    items.remove(p);
                }
                None => break,
            }
        }
    }
    let mut text = String::new();
    let mut last = None;
    for it in &items {
        if !text.is_empty() {
            text.push_str(if it.section != last { "\n\n" } else { " " });
        }
        last = it.section;
        text.push_str(&it.text);
    }
    text.push('\n');
    Draft { words: words(&text), text, items, available_words: available }
}

/// The version of the epoch writer (`compose_epoch`): the description above, then what the
/// document's neighbourhood says about the document's things.
pub const EPOCH_WRITER: &str = "epoch-writer-2026-11-05";

/// A text of the document's neighbourhood, read by Mova, with its citation id.
pub struct Neighbour<'a> {
    pub cite: String,
    pub title: String,
    /// How it relates to the document: `own:<kind>` (the document's own earlier texts), `cites`,
    /// `cited_by`, `similar`, `same_author`, `shared_entity`, or anything else («near»).
    pub relation: String,
    pub reading: &'a Reading,
}

/// The paragraph numbers of neighbour texts start here (the paper's parts come first).
const NEIGHBOUR_SECTION: usize = 100_000;

/// The document's key terms: its most frequent nouns, its own acronyms and the terms Mova did not
/// know (normalized, `knowledge::norm_term`).
pub fn key_terms(r: &Reading) -> BTreeSet<String> {
    let mut k: BTreeSet<String> = top(&r.nouns, 40).into_iter().map(|(w, _)| knowledge::norm_term(w)).collect();
    k.extend(r.definitions.keys().map(|a| knowledge::norm_term(a)));
    k.extend(r.forms.keys().cloned());
    k.retain(|w| !STOP.contains(&w.as_str()) && w.chars().count() >= 3);
    k
}

/// How many distinct key terms a sentence speaks of.
pub fn relevance(s: &crate::reader::Sentence, key: &BTreeSet<String>) -> usize {
    let mut hit: BTreeSet<String> = s.nouns.iter().map(|n| knowledge::norm_term(n)).filter(|n| key.contains(n)).collect();
    for w in s.text.split(|c: char| !(c.is_alphanumeric() || c == '-')).filter(|w| !w.is_empty()) {
        let t = knowledge::norm_term(w);
        if key.contains(&t) {
            hit.insert(t);
        }
    }
    hit.len()
}

fn relation_phrase(rel: &str) -> &'static str {
    match rel {
        "own:sol" => "an earlier description of this paper",
        "own:mova" => "an earlier text of mine on this paper",
        r if r.starts_with("own:") => "a description of this paper",
        "cites" => "a work this paper cites",
        "cited_by" => "a work that cites this paper",
        "similar" => "a work on a similar subject",
        "same_author" => "a work by the same authors",
        "shared_entity" => "a work that speaks of the same things",
        _ => "a work near this paper",
    }
}

/// The long text of an epoch: the description of the paper (`compose`), then what the texts of
/// its neighbourhood (nearest first) say about its key terms, each restated clause cited to its
/// text. Neighbour sentences are taken in rounds — every neighbour's most relevant sentence
/// first, then every neighbour's second, … — so the nearest texts all speak before any one
/// dominates; within a neighbour they are written in source order after a lead naming the text
/// and its relation. Nothing inside a text is capped except by the word budget `max`; a sentence
/// that speaks of fewer than `min_relevance` key terms is not taken. Every clause passes the same
/// guard (verbatim cap against the paper and every neighbour text, terminal-answer overlap, the
/// grammar gate) and the near-duplicate check against everything written before it.
pub fn compose_epoch(r: &Reading, known: &Known, guard: &Guard, neighbours: &[Neighbour], max: usize, min_relevance: usize) -> Draft {
    let base = compose(r, known, guard, max);
    let mut w = Writer { guard, items: base.items.clone(), sets: base.items.iter().map(|i| content(&i.text).into_iter().collect()).collect(), explained: BTreeSet::new() };
    let key = key_terms(r);
    let ranked: Vec<Vec<usize>> = neighbours
        .iter()
        .map(|n| {
            let mut v: Vec<(usize, usize)> = n.reading.sentences.iter().filter(|s| !n.reading.sections[s.section].skipped && !s.clauses.is_empty()).map(|s| (relevance(s, &key), s.index)).filter(|(sc, _)| *sc >= min_relevance.max(1)).collect();
            v.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
            v.into_iter().map(|x| x.1).collect()
        })
        .collect();
    let leads: Vec<String> = neighbours.iter().map(|n| sentence(&format!("I also read «{}», {}", n.title, relation_phrase(&n.relation)))).collect();
    let intro = sentence(&format!("Next, I read {} texts near this paper, and I restate what they say about its things", neighbours.len()));
    let mut used = total(&w.items);
    let mut opened = vec![false; neighbours.len()];
    let mut any_open = false;
    let mut world: Vec<Vec<Item>> = vec![vec![]; neighbours.len()];
    let rounds = ranked.iter().map(Vec::len).max().unwrap_or(0);
    'all: for round in 0..rounds {
        for (k, v) in ranked.iter().enumerate() {
            let Some(&i) = v.get(round) else { continue };
            if used + 8 > max {
                break 'all;
            }
            let n = &neighbours[k];
            for cl in &n.reading.sentences[i].clauses {
                let extra = if opened[k] { 0 } else { words(&leads[k]) } + if any_open { 0 } else { words(&intro) };
                let it = Item { text: cl.text.clone(), kind: "world", sources: vec![i], knowledge: vec![], section: Some(NEIGHBOUR_SECTION + k), basis: String::new(), cites: vec![n.cite.clone()] };
                let cost = words(&it.text) + extra;
                if used + cost > max {
                    continue;
                }
                if w.push(it) {
                    let it = w.items.pop().expect("pushed");
                    world[k].push(it);
                    used += cost;
                    opened[k] = true;
                    any_open = true;
                }
            }
        }
    }
    let mut items = w.items;
    if any_open {
        items.push(Item { text: intro, kind: "structure", sources: vec![], knowledge: vec![], section: Some(NEIGHBOUR_SECTION - 1), basis: neighbours.len().to_string(), cites: vec![] });
    }
    for (k, mut v) in world.into_iter().enumerate() {
        if v.is_empty() {
            continue;
        }
        v.sort_by_key(|it| it.sources[0]);
        items.push(Item { text: leads[k].clone(), kind: "structure", sources: vec![], knowledge: vec![], section: Some(NEIGHBOUR_SECTION + k), basis: neighbours[k].title.clone(), cites: vec![neighbours[k].cite.clone()] });
        items.extend(v);
    }
    let mut text = String::new();
    let mut last = None;
    for it in &items {
        if !text.is_empty() {
            text.push_str(if it.section != last { "\n\n" } else { " " });
        }
        last = it.section;
        text.push_str(&it.text);
    }
    text.push('\n');
    Draft { words: words(&text), text, items, available_words: base.available_words }
}
