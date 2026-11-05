//! A dictionary layer from annotated text units, natively: no teacher, no large model.
//!
//! Input: units (a lesson line, a corpus prompt, a sentence of an article), each with its id, its document (lesson,
//! prompt file, article), an optional class (a Wikipedia article's class) and its words as Mova annotated them
//! (form, lemma, UPOS). Output: the rows of a layer in the format of the school dictionaries (`$MOVA_DATA/school/dictionaries/*.tsv`)
//! (`# word	part of speech	meaning for a pupil	sense in the lessons	examples	note`), a draft:
//!
//! - **word**: the lemma of a content word (NOUN, VERB, ADJ, ADV), lower-cased; the `top` commonest by count, ties by
//!   the number of documents, then alphabetically;
//! - **part of speech** and **meaning for a pupil**: from the pupils' dictionary (`world/data/dictionary-improved.tsv`,
//!   ours) when it has the word, else the UPOS name and an empty meaning;
//! - **sense in the lessons**: what Mova counted in this source — uses, documents, the UPOS share, the commonest
//!   classes and the content words it is used with; it is not a written sense;
//! - **examples**: up to three units that use it, from different documents first, each `text «id»`;
//! - **note**: that the row is a native draft.
//!
//! Deterministic: every order is a `BTreeMap` order or an explicit sort with a total key.
use std::collections::{BTreeMap, BTreeSet};

/// One annotated word.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Word {
    pub form: String,
    pub lemma: String,
    pub upos: String,
}

/// One text unit of a source.
#[derive(Clone, Debug)]
pub struct Unit {
    pub id: String,
    pub doc: String,
    pub class: Option<String>,
    pub text: String,
    pub words: Vec<Word>,
}

/// The layer's column line (as the school layers have it).
pub const HEAD: &str = "# word\tpart of speech\tmeaning for a pupil\tsense in the lessons\texamples\tnote";
/// The parts of speech a layer counts.
pub const CONTENT: [&str; 4] = ["NOUN", "VERB", "ADJ", "ADV"];
/// Examples are units of at most this many characters (and at least `MIN_EXAMPLE`).
pub const MAX_EXAMPLE: usize = 220;
pub const MIN_EXAMPLE: usize = 20;

/// A row of a layer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Row {
    pub word: String,
    pub pos: String,
    pub meaning: String,
    pub sense: String,
    pub examples: Vec<String>,
    pub note: String,
}

impl Row {
    pub fn tsv(&self) -> String {
        let clean = |s: &str| s.replace(['\t', '\n', '\r'], " ");
        format!(
            "{}\t{}\t{}\t{}\t{}\t{}",
            clean(&self.word),
            clean(&self.pos),
            clean(&self.meaning),
            clean(&self.sense),
            self.examples.iter().map(|e| clean(e)).collect::<Vec<_>>().join(" | "),
            clean(&self.note)
        )
    }
}

/// What the layer is made from, for the sense and the draft line.
#[derive(Clone, Debug)]
pub struct Source {
    /// e.g. "the command course and its corpus"
    pub name: String,
    /// what a document is called: "lesson", "article"
    pub doc_noun: String,
    /// its plural: "lessons and prompts", "articles"
    pub doc_plural: String,
    /// the number of documents in the source
    pub docs: usize,
}

/// The pupils' dictionary: word → (part of speech, meaning for a pupil).
pub type Base = BTreeMap<String, (String, String)>;

/// Read the pupils' dictionary (`dictionary-improved.tsv` or a school layer): first column the word, second the part
/// of speech, third the meaning for a pupil; `#` lines skipped; the first record of a word kept.
pub fn read_base(text: &str) -> Base {
    let mut b = Base::new();
    for l in text.lines().filter(|l| !l.starts_with('#') && !l.trim().is_empty()) {
        let c: Vec<&str> = l.split('\t').collect();
        if c.len() >= 3 && !c[0].is_empty() {
            b.entry(c[0].to_lowercase()).or_insert_with(|| (c[1].to_string(), c[2].to_string()));
        }
    }
    b
}

fn upos_name(u: &str) -> &'static str {
    match u {
        "NOUN" => "noun",
        "VERB" => "verb",
        "ADJ" => "adjective",
        "ADV" => "adverb",
        _ => "word",
    }
}

/// A lemma a layer may hold: letters (and inner hyphens or apostrophes), at least two.
pub fn admissible(lemma: &str) -> bool {
    let l: Vec<char> = lemma.chars().collect();
    l.len() >= 2
        && l.iter().all(|c| c.is_alphabetic() || *c == '-' || *c == '\'')
        && l.first().is_some_and(|c| c.is_alphabetic())
        && l.last().is_some_and(|c| c.is_alphabetic())
}

struct Stat {
    count: usize,
    docs: BTreeSet<String>,
    upos: BTreeMap<String, usize>,
    classes: BTreeMap<String, usize>,
    with: BTreeMap<String, usize>,
    /// (unit index) in source order
    units: Vec<usize>,
}

/// Count the content lemmas of the units: lemma → statistics.
fn count(units: &[Unit]) -> BTreeMap<String, Stat> {
    let mut m: BTreeMap<String, Stat> = BTreeMap::new();
    for (ui, u) in units.iter().enumerate() {
        let here: BTreeSet<String> = u
            .words
            .iter()
            .filter(|w| CONTENT.contains(&w.upos.as_str()))
            .map(|w| w.lemma.to_lowercase())
            .filter(|l| admissible(l))
            .collect();
        for w in u.words.iter().filter(|w| CONTENT.contains(&w.upos.as_str())) {
            let l = w.lemma.to_lowercase();
            if !admissible(&l) {
                continue;
            }
            let s = m.entry(l.clone()).or_insert_with(|| Stat {
                count: 0,
                docs: BTreeSet::new(),
                upos: BTreeMap::new(),
                classes: BTreeMap::new(),
                with: BTreeMap::new(),
                units: Vec::new(),
            });
            s.count += 1;
            *s.upos.entry(w.upos.clone()).or_default() += 1;
            if s.docs.insert(u.doc.clone()) {
                if let Some(c) = &u.class {
                    *s.classes.entry(c.clone()).or_default() += 1;
                }
            }
            if s.units.last() != Some(&ui) {
                s.units.push(ui);
                for o in here.iter().filter(|o| **o != l) {
                    *s.with.entry(o.clone()).or_default() += 1;
                }
            }
        }
    }
    m
}

/// The `k` largest entries of a count map, ties alphabetically.
fn top(m: &BTreeMap<String, usize>, k: usize) -> Vec<(String, usize)> {
    let mut v: Vec<(String, usize)> = m.iter().map(|(a, b)| (a.clone(), *b)).collect();
    v.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    v.truncate(k);
    v
}

/// Up to `k` example units: within the length bounds, a new document first, in source order.
fn examples(units: &[Unit], idx: &[usize], k: usize) -> Vec<String> {
    let fits = |i: &usize| (MIN_EXAMPLE..=MAX_EXAMPLE).contains(&units[*i].text.chars().count());
    let mut out: Vec<usize> = Vec::new();
    let mut docs = BTreeSet::new();
    for i in idx.iter().filter(|i| fits(i)) {
        if out.len() < k && docs.insert(units[*i].doc.clone()) {
            out.push(*i);
        }
    }
    for i in idx.iter().filter(|i| fits(i)) {
        if out.len() < k && !out.contains(i) {
            out.push(*i);
        }
    }
    out.iter().map(|i| format!("{} «{}»", units[*i].text.split_whitespace().collect::<Vec<_>>().join(" "), units[*i].id)).collect()
}

/// The draft rows: the `top` commonest content lemmas with their counted sense and examples.
pub fn build(units: &[Unit], base: &Base, src: &Source, top_n: usize) -> Vec<Row> {
    let stats = count(units);
    let mut order: Vec<(&String, &Stat)> = stats.iter().collect();
    order.sort_by(|a, b| b.1.count.cmp(&a.1.count).then_with(|| b.1.docs.len().cmp(&a.1.docs.len())).then_with(|| a.0.cmp(b.0)));
    order
        .into_iter()
        .take(top_n)
        .map(|(lemma, s)| {
            let (main, n_main) = top(&s.upos, 1).into_iter().next().unwrap_or_default();
            let (pos, meaning) = base.get(lemma).cloned().unwrap_or_else(|| (upos_name(&main).to_string(), String::new()));
            let mut sense = format!(
                "Counted by Mova: {} {} in {} of the {} {} of {}; as {} {} in {} of {}.",
                s.count,
                if s.count == 1 { "use" } else { "uses" },
                s.docs.len(),
                src.docs,
                src.doc_plural,
                src.name,
                if upos_name(&main).starts_with(['a', 'e', 'i', 'o', 'u']) { "an" } else { "a" },
                upos_name(&main),
                n_main,
                s.count
            );
            let classes = top(&s.classes, 4);
            if !classes.is_empty() {
                sense.push_str(&format!(
                    " By the class of the {}: {}.",
                    src.doc_noun,
                    classes.iter().map(|(c, n)| format!("{c} {n}")).collect::<Vec<_>>().join(", ")
                ));
            }
            let with = top(&s.with, 5);
            if !with.is_empty() {
                sense.push_str(&format!(" Most often with: {}.", with.iter().map(|(w, _)| format!("«{w}»")).collect::<Vec<_>>().join(", ")));
            }
            Row {
                word: lemma.clone(),
                pos,
                meaning,
                sense,
                examples: examples(units, &s.units, 3),
                note: format!(
                    "Draft: made natively by Nautilus from {}; the counts and examples are Mova's own, the meaning is the pupils' dictionary's; no sense of this source is written yet.",
                    src.name
                ),
            }
        })
        .collect()
}

/// The whole layer file: the column line, the draft line, the rows.
pub fn render(rows: &[Row], src: &Source, licence: &str) -> String {
    let mut s = format!(
        "{HEAD}\n# draft: made natively by Nautilus (tools/nautilus) from {}, {} {}; the {} commonest content words; not reviewed; {licence}\n",
        src.name,
        src.docs,
        src.doc_plural,
        rows.len()
    );
    for r in rows {
        s.push_str(&r.tsv());
        s.push('\n');
    }
    s
}

/// The rows of a layer file (the inverse of `render` for its rows; `#` lines skipped, missing columns empty).
pub fn parse_rows(text: &str) -> Vec<Row> {
    text.lines()
        .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
        .map(|l| {
            let c: Vec<&str> = l.split('\t').collect();
            let g = |i: usize| c.get(i).copied().unwrap_or("").to_string();
            Row { word: g(0), pos: g(1), meaning: g(2), sense: g(3), examples: g(4).split(" | ").map(str::trim).filter(|e| !e.is_empty()).map(String::from).collect(), note: g(5) }
        })
        .collect()
}

/// What a layer file holds: its rows (words), whether it is a draft, and its examples.
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize)]
pub struct Stats {
    pub entries: usize,
    pub examples: usize,
    pub with_meaning: usize,
    pub draft: bool,
}

pub fn stats(text: &str) -> Stats {
    let mut st = Stats { draft: text.lines().any(|l| l.starts_with("# draft:")), ..Default::default() };
    for l in text.lines().filter(|l| !l.starts_with('#') && !l.trim().is_empty()) {
        let c: Vec<&str> = l.split('\t').collect();
        st.entries += 1;
        if c.get(2).is_some_and(|m| !m.is_empty()) {
            st.with_meaning += 1;
        }
        st.examples += c.get(4).map_or(0, |e| e.split(" | ").filter(|x| !x.trim().is_empty()).count());
    }
    st
}

#[cfg(test)]
mod tests {
    use super::*;
    fn u(id: &str, doc: &str, text: &str, ws: &[(&str, &str)]) -> Unit {
        Unit {
            id: id.into(),
            doc: doc.into(),
            class: Some("city".into()),
            text: text.into(),
            words: ws.iter().map(|(l, p)| Word { form: l.to_string(), lemma: l.to_string(), upos: p.to_string() }).collect(),
        }
    }
    #[test]
    fn counts_ranks_and_renders() {
        let units = vec![
            u("a:1", "a", "Pass the salt to Dad, please.", &[("pass", "VERB"), ("the", "DET"), ("salt", "NOUN"), ("dad", "NOUN")]),
            u("a:2", "a", "Pass it back to me now.", &[("pass", "VERB"), ("it", "PRON"), ("back", "ADV"), ("now", "ADV")]),
            u("b:1", "b", "Dad wants the salt passed.", &[("dad", "NOUN"), ("want", "VERB"), ("salt", "NOUN"), ("pass", "VERB")]),
            u("b:2", "b", "x", &[("x", "NOUN"), ("1st", "ADJ")]),
        ];
        let base = read_base("# head\npass\tverb (pass, passes)\tTo give something to someone.\n");
        let src = Source { name: "the test".into(), doc_noun: "lesson".into(), doc_plural: "lessons".into(), docs: 2 };
        let rows = build(&units, &base, &src, 3);
        let words: Vec<&str> = rows.iter().map(|r| r.word.as_str()).collect();
        // pass 3 uses; dad and salt 2 uses in 2 lessons, alphabetically; "x" and "1st" not admissible; DET not counted
        assert_eq!(words, ["pass", "dad", "salt"]);
        assert_eq!(rows[0].pos, "verb (pass, passes)");
        assert_eq!(rows[1].pos, "noun");
        assert!(rows[1].meaning.is_empty());
        assert!(rows[0].sense.starts_with("Counted by Mova: 3 uses in 2 of the 2 lessons of the test; as a verb in 3 of 3."), "{}", rows[0].sense);
        assert!(rows[0].sense.contains("Most often with: «dad», «salt»"), "{}", rows[0].sense);
        assert!(rows[0].sense.contains("By the class of the lesson: city 2."));
        // a new lesson first, then source order
        assert_eq!(rows[0].examples, ["Pass the salt to Dad, please. «a:1»", "Dad wants the salt passed. «b:1»", "Pass it back to me now. «a:2»"]);
        let text = render(&rows, &src, "ours");
        assert!(text.starts_with(HEAD));
        let st = stats(&text);
        assert_eq!(st, Stats { entries: 3, examples: 7, with_meaning: 1, draft: true });
        // the rows read back as they were written
        assert_eq!(parse_rows(&text), rows);
        // deterministic: the same input, the same bytes
        assert_eq!(text, render(&build(&units, &base, &src, 3), &src, "ours"));
    }
    #[test]
    fn admissible_lemmas() {
        assert!(admissible("salt") && admissible("well-known") && admissible("o'clock"));
        assert!(!admissible("a") && !admissible("1st") && !admissible("-x") && !admissible("x-") && !admissible("km2"));
    }
}
