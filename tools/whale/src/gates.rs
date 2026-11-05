//! Anti-padding gates for a Whale's own description. Each gate shows red on its negative control
//! (tests below):
//!
//! - `words`: the description has `min..=max` words (the deployment's 4,000..5,000);
//! - `near_duplicates`: no two sentences are the same or nearly the same (Jaccard similarity of
//!   their content-word sets at least `near_duplicate`, sets of at least 4 words);
//! - `verbatim`: no sentence shares more than `verbatim_max` alphanumeric tokens in a row with
//!   anywhere in the paper (a structural sentence's «…» quotation of a title or part name is
//!   exempt; it names, it does not restate);
//! - `answer_overlap`: no `answer_ngram`-token sequence of the description occurs in any answer the
//!   Whale's terminal ever gave, unless the paper itself has it (then the paper is its source);
//! - `traceability`: every content word of every sentence is a word of its basis — the source
//!   sentences it restates, the knowledge item it realizes, or the counts and titles a structural
//!   sentence states — or a word of the fixed frame vocabulary; and every sentence has a basis;
//! - `structure_share`: structural sentences (counts, titles, transitions) are at most
//!   `structure_share_max` of the words;
//! - `grammar`: every restated sentence (`source`, `reading`), parsed again by Mova on its own, is a
//!   whole clause (`clause::grammatical`: a subject, a finite verb, no fragment such as «This
//!   architecture all the previously derived subtrees, while maintaining.»).
use crate::compose::{Draft, FRAME};
use crate::knowledge::Item;
use crate::reader::{content, tokens, Reading};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Params {
    #[serde(default = "d_verbatim")]
    pub verbatim_max: usize,
    #[serde(default = "d_ngram")]
    pub answer_ngram: usize,
    #[serde(default = "d_dup")]
    pub near_duplicate: f64,
    #[serde(default = "d_share")]
    pub structure_share_max: f64,
}
fn d_verbatim() -> usize {
    12
}
fn d_ngram() -> usize {
    6
}
fn d_dup() -> f64 {
    0.8
}
fn d_share() -> f64 {
    0.2
}
impl Default for Params {
    fn default() -> Self {
        Self { verbatim_max: d_verbatim(), answer_ngram: d_ngram(), near_duplicate: d_dup(), structure_share_max: d_share() }
    }
}
impl Params {
    pub fn validate(&self) -> crate::Result<()> {
        if !(4..=40).contains(&self.verbatim_max) || !(3..=20).contains(&self.answer_ngram) || !(0.5..=1.0).contains(&self.near_duplicate) || !(0.0..=1.0).contains(&self.structure_share_max) {
            return Err(crate::util::error("gates: verbatim_max 4..40, answer_ngram 3..20, near_duplicate 0.5..1, structure_share_max 0..1"));
        }
        Ok(())
    }
}

/// The paper's tokens and the terminal's answers, indexed once per round.
pub struct Guard {
    pub params: Params,
    positions: BTreeMap<String, Vec<usize>>,
    paper_grams: BTreeSet<String>,
    answer_grams: BTreeSet<String>,
}

fn grams(t: &[String], n: usize) -> impl Iterator<Item = String> + '_ {
    t.windows(n).map(|w| w.join(" "))
}

impl Guard {
    pub fn new(paper_text: &str, answers: &[String], params: Params) -> Self {
        let pt = tokens(paper_text);
        let mut positions: BTreeMap<String, Vec<usize>> = BTreeMap::new();
        for (i, w) in pt.iter().enumerate() {
            positions.entry(w.clone()).or_default().push(i);
        }
        let n = params.answer_ngram;
        let paper_grams = grams(&pt, n).collect();
        let answer_grams = answers.iter().flat_map(|a| grams(&tokens(a), n).collect::<Vec<_>>()).collect();
        Self { params, positions, paper_grams, answer_grams }
    }
    /// Longest run of the sentence's tokens found consecutively anywhere in the paper.
    pub fn verbatim(&self, s: &str) -> usize {
        let t = tokens(s);
        let mut best = 0;
        let mut prev: BTreeMap<usize, usize> = BTreeMap::new();
        for w in &t {
            let mut cur = BTreeMap::new();
            for &p in self.positions.get(w).map(Vec::as_slice).unwrap_or(&[]) {
                let n = p.checked_sub(1).and_then(|q| prev.get(&q)).copied().unwrap_or(0) + 1;
                best = best.max(n);
                cur.insert(p, n);
            }
            prev = cur;
        }
        best
    }
    /// Sequences of the sentence that occur in a terminal answer and not in the paper.
    pub fn answer_overlap(&self, s: &str) -> Vec<String> {
        grams(&tokens(s), self.params.answer_ngram).filter(|g| self.answer_grams.contains(g) && !self.paper_grams.contains(g)).collect()
    }
    /// The grammar gate's verdict on one restated sentence (Mova's annotator; refused when the
    /// model cannot be loaded).
    pub fn grammatical(&self, s: &str) -> bool {
        world::tree::annotator().is_ok_and(|a| crate::clause::grammatical(a, s).is_ok())
    }
    /// The writer's own check before it keeps a sentence.
    pub fn admits(&self, s: &str) -> bool {
        self.verbatim(s) <= self.params.verbatim_max && self.answer_overlap(s).is_empty()
    }
    /// A structural sentence may quote a title or a part's name in «…» whole; the rest of it is
    /// checked like any sentence.
    pub fn admits_structure(&self, s: &str) -> bool {
        self.admits(&unquote(s))
    }
}

/// The text without its «…» quotations (titles and part names in structural sentences).
pub fn unquote(s: &str) -> String {
    let mut out = String::new();
    let mut depth = 0;
    for c in s.chars() {
        match c {
            '«' => {
                depth += 1;
                out.push_str(" | ");
            }
            '»' => depth = (depth - 1).max(0),
            _ if depth == 0 => out.push(c),
            _ => {}
        }
    }
    out
}

/// Content-word set used for near-duplicate detection.
fn word_set(s: &str) -> BTreeSet<String> {
    content(s).into_iter().collect()
}
pub fn similarity(a: &BTreeSet<String>, b: &BTreeSet<String>) -> f64 {
    let inter = a.intersection(b).count();
    let union = a.union(b).count();
    if union == 0 { 0.0 } else { inter as f64 / union as f64 }
}
pub fn near_duplicate(a: &BTreeSet<String>, b: &BTreeSet<String>, threshold: f64) -> bool {
    a == b || (a.len() >= 4 && b.len() >= 4 && similarity(a, b) >= threshold)
}

/// Run every gate. `knowledge` resolves the items explanations cite.
pub fn check(d: &Draft, r: &Reading, knowledge: &[Item], guard: &Guard, min: usize, max: usize) -> Value {
    let p = &guard.params;
    let frame: BTreeSet<&str> = FRAME.iter().copied().collect();
    let by_id: BTreeMap<&str, &Item> = knowledge.iter().map(|k| (k.id.as_str(), k)).collect();
    let sets: Vec<BTreeSet<String>> = d.items.iter().map(|i| word_set(&i.text)).collect();
    let mut dups = vec![];
    let mut exact: BTreeSet<String> = BTreeSet::new();
    for (i, it) in d.items.iter().enumerate() {
        if !exact.insert(it.text.to_lowercase()) {
            dups.push(json!([i, i]));
        }
        for j in 0..i {
            if near_duplicate(&sets[i], &sets[j], p.near_duplicate) {
                dups.push(json!([j, i]));
            }
        }
    }
    let mut verbatim_max = 0;
    let mut verbatim_over = vec![];
    let mut overlap = vec![];
    let mut untraceable = vec![];
    let mut ungrammatical = vec![];
    let mut structure_words = 0;
    let annotator = world::tree::annotator().ok();
    for (i, it) in d.items.iter().enumerate() {
        if matches!(it.kind, "source" | "reading") {
            let verdict = annotator.map_or(Err("no model"), |a| crate::clause::grammatical(a, &it.text));
            if let Err(why) = verdict {
                ungrammatical.push(json!({"item":i,"text":it.text,"why":why}));
            }
        }
        let v = guard.verbatim(&if it.kind == "structure" { unquote(&it.text) } else { it.text.clone() });
        verbatim_max = verbatim_max.max(v);
        if v > p.verbatim_max {
            verbatim_over.push(json!({"item":i,"run":v}));
        }
        let o = guard.answer_overlap(&it.text);
        if !o.is_empty() {
            overlap.push(json!({"item":i,"grams":o.len(),"first":o[0]}));
        }
        let basis = match it.kind {
            "source" | "reading" if !it.sources.is_empty() && it.sources.iter().all(|&s| s < r.sentences.len()) => {
                let mut b: String = it.sources.iter().map(|&s| r.sentences[s].text.clone()).collect::<Vec<_>>().join(" ");
                for k in &it.knowledge {
                    if let Some(item) = by_id.get(k.as_str()) {
                        b.push(' ');
                        b.push_str(&item.basis());
                    }
                }
                Some(b)
            }
            "explain" if !it.knowledge.is_empty() => {
                let ks: Vec<String> = it.knowledge.iter().filter_map(|k| by_id.get(k.as_str()).map(|i| i.basis())).collect();
                (ks.len() == it.knowledge.len()).then(|| ks.join(" "))
            }
            "structure" => {
                structure_words += crate::contract::word_count(&it.text);
                Some(it.basis.clone())
            }
            _ => None,
        };
        // A term may be written in another form of the same normalized term («MRFs» for «MRF»).
        let single = |w: &str| -> String { if w.len() > 3 && w.ends_with('s') && !w.ends_with("ss") { w[..w.len() - 1].to_string() } else { w.to_string() } };
        let ok = basis.is_some_and(|b| {
            let have: BTreeSet<String> = tokens(&b).into_iter().flat_map(|w| [crate::knowledge::norm_term(&w), single(&w), w]).collect();
            content(&it.text).iter().all(|w| have.contains(w) || have.contains(&crate::knowledge::norm_term(w)) || have.contains(&single(w)) || frame.contains(w.as_str()))
        });
        if !ok {
            untraceable.push(json!({"item":i,"kind":it.kind,"text":it.text}));
        }
    }
    let words = d.words;
    let share = if words == 0 { 0.0 } else { structure_words as f64 / words as f64 };
    let traceability = if d.items.is_empty() { 0.0 } else { 1.0 - untraceable.len() as f64 / d.items.len() as f64 };
    let g_words = (min..=max).contains(&words);
    let g_dup = dups.is_empty();
    let g_verb = verbatim_over.is_empty();
    let g_over = overlap.is_empty();
    let g_trace = untraceable.is_empty();
    let g_share = share <= p.structure_share_max;
    let g_grammar = ungrammatical.is_empty();
    let failed: Vec<&str> = [("words", g_words), ("near_duplicates", g_dup), ("verbatim", g_verb), ("answer_overlap", g_over), ("traceability", g_trace), ("structure_share", g_share), ("grammar", g_grammar)]
        .iter()
        .filter(|g| !g.1)
        .map(|g| g.0)
        .collect();
    json!({
        "pass": failed.is_empty(), "failed": failed,
        "words": words, "words_min": min, "words_max": max,
        "near_duplicates": dups.len(), "near_duplicate_pairs": dups.into_iter().take(8).collect::<Vec<_>>(),
        "verbatim_max": verbatim_max, "verbatim_cap": p.verbatim_max, "verbatim_over": verbatim_over.into_iter().take(8).collect::<Vec<_>>(),
        "answer_overlap": overlap.len(), "answer_overlap_items": overlap.into_iter().take(8).collect::<Vec<_>>(), "answer_ngram": p.answer_ngram,
        "traceability": traceability, "untraceable": untraceable.into_iter().take(8).collect::<Vec<_>>(),
        "structure_words": structure_words, "structure_share": share, "structure_share_max": p.structure_share_max,
        "ungrammatical": ungrammatical.len(), "ungrammatical_items": ungrammatical.into_iter().take(8).collect::<Vec<_>>()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compose::Item as W;
    use crate::reader::{Section, Sentence};

    fn reading(sentences: &[&str]) -> Reading {
        Reading {
            title: "T".into(),
            words: 100,
            sections: vec![Section { title: "Method".into(), skipped: false, sentences: (0..sentences.len()).collect() }],
            sentences: sentences.iter().enumerate().map(|(i, s)| Sentence { index: i, section: 0, text: s.to_string(), admitted: true, skeleton: vec![], unknown: vec![], clauses: vec![], nouns: vec![] }).collect(),
            unknown: Default::default(),
            nouns: Default::default(),
            definitions: Default::default(),
            forms: Default::default(),
            filtered: Default::default(),
            names: Default::default(),
        }
    }
    fn draft(items: Vec<W>) -> Draft {
        let text = items.iter().map(|i| i.text.clone()).collect::<Vec<_>>().join(" ");
        Draft { words: crate::contract::word_count(&text), text, items, available_words: 0 }
    }
    fn src(text: &str, s: usize) -> W {
        W { text: text.into(), kind: "source", sources: vec![s], knowledge: vec![], section: Some(0), basis: String::new(), cites: vec![] }
    }
    const PAPER: [&str; 2] = [
        "The parser reads every word of the sentence once and attaches it to the head chosen by the zorbic rule of the method.",
        "Zorbic parsing is faster than the baseline on long sentences.",
    ];

    #[test]
    fn a_clean_draft_passes_and_every_gate_has_a_red_control() {
        // The grammar gate reads every restated sentence with Mova's model.
        if !crate::learn::tests::model_ok() {
            return;
        }
        let r = reading(&PAPER);
        let answer = "A zorbic rule is a simple rule that attaches every single word to exactly one head word.".to_string();
        let g = Guard::new(&PAPER.join(" "), &[answer.clone()], Params::default());
        let k = vec![Item { id: "k00001".into(), whale: "whale-1".into(), kind: "term".into(), key: "zorbic".into(), paper: "p".into(), facts: vec![crate::knowledge::Fact { relation: "is_a".into(), subject: "".into(), verb: "".into(), object: "a simple rule".into() }], provenance: json!({}), unix: 0 }];
        let good = vec![
            src("The parser reads every word of the sentence once.", 0),
            src("It attaches it to the head chosen by the zorbic rule.", 0),
            W { text: "I understand «zorbic» as a simple rule.".into(), kind: "explain", sources: vec![], knowledge: vec!["k00001".into()], section: Some(0), basis: String::new(), cites: vec![] },
            src("Zorbic parsing is faster than the baseline on long sentences.", 1),
        ];
        let ok = check(&draft(good.clone()), &r, &k, &g, 10, 200);
        assert_eq!(ok["pass"], true, "{ok}");
        // words: one word short of the bound.
        let words = check(&draft(good.clone()), &r, &k, &g, ok["words"].as_u64().unwrap() as usize + 1, 200);
        assert_eq!(words["failed"], json!(["words"]));
        // near duplicates: the same sentence twice, and a near copy.
        let mut dup = good.clone();
        dup.push(src("The parser reads every word of the sentence once.", 0));
        assert_eq!(check(&draft(dup), &r, &k, &g, 10, 200)["failed"], json!(["near_duplicates"]));
        let mut near = good.clone();
        near.push(src("Once, the parser reads every word of the sentence.", 0));
        assert!(check(&draft(near), &r, &k, &g, 10, 200)["failed"].as_array().unwrap().contains(&json!("near_duplicates")));
        // verbatim: 13 words in a row from the paper.
        let mut copy = good.clone();
        copy.push(src("The parser reads every word of the sentence once and attaches it to the head.", 0));
        let v = check(&draft(copy), &r, &k, &g, 10, 200);
        assert!(v["failed"].as_array().unwrap().contains(&json!("verbatim")), "{v}");
        assert!(v["verbatim_max"].as_u64().unwrap() >= 13);
        // answer overlap: the terminal's words pasted into an explanation.
        let mut pasted = good.clone();
        pasted.push(W { text: "A zorbic rule is a simple rule that attaches every single word.".into(), kind: "explain", sources: vec![], knowledge: vec!["k00001".into()], section: Some(0), basis: String::new(), cites: vec![] });
        assert!(check(&draft(pasted), &r, &k, &g, 10, 200)["failed"].as_array().unwrap().contains(&json!("answer_overlap")));
        // traceability: a sentence with words from nowhere, and one with no basis at all.
        let mut invented = good.clone();
        invented.push(src("The parser is remarkably elegant and widely admired.", 1));
        assert_eq!(check(&draft(invented), &r, &k, &g, 10, 200)["failed"], json!(["traceability"]));
        let mut orphan = good.clone();
        orphan.push(W { text: "Zorbic parsing matters.".into(), kind: "source", sources: vec![], knowledge: vec![], section: None, basis: String::new(), cites: vec![] });
        assert_eq!(check(&draft(orphan), &r, &k, &g, 10, 200)["failed"], json!(["traceability"]));
        // structure share: counts and titles may not carry the description.
        let mut padded = good;
        padded.push(W { text: "I read «T» with its part «Method» and its sentences about the parser.".into(), kind: "structure", sources: vec![], knowledge: vec![], section: None, basis: "T Method parser".into(), cites: vec![] });
        assert!(check(&draft(padded), &r, &k, &g, 10, 200)["failed"].as_array().unwrap().contains(&json!("structure_share")));
    }
    /// The grammar gate is red on the fragments the writer of 2026-11-04c let through on
    /// ma-2018 and green on whole clauses of the same paper.
    #[test]
    fn grammar_gate_refuses_fragments() {
        if !crate::learn::tests::model_ok() {
            return;
        }
        let r = reading(&["This architecture makes it possible to capture information from the whole sentence and all the previously derived subtrees, while maintaining a number of parsing steps linear in the sentence length."]);
        let g = Guard::new(&r.sentences[0].text, &[], Params::default());
        let good = vec![src("This architecture makes it possible to capture information from the whole sentence.", 0)];
        let ok = check(&draft(good.clone()), &r, &[], &g, 1, 200);
        assert_eq!(ok["pass"], true, "{ok}");
        for frag in ["This architecture all the previously derived subtrees, while maintaining.", "A number of parsing steps linear in the sentence length."] {
            let mut bad = good.clone();
            bad.push(src(frag, 0));
            let v = check(&draft(bad), &r, &[], &g, 1, 200);
            assert_eq!(v["failed"], json!(["grammar"]), "{frag}: {v}");
            assert!(!g.grammatical(frag), "{frag}");
        }
    }
    #[test]
    fn guard_counts_runs_anywhere_in_the_paper() {
        let g = Guard::new("a b c d e f. x y z a b c", &[], Params::default());
        assert_eq!(g.verbatim("q a b c d r"), 4);
        assert_eq!(g.verbatim("q r s"), 0);
        assert!(Params { verbatim_max: 50, ..Params::default() }.validate().is_err());
    }
}
