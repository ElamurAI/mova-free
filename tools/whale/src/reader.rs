//! A Whale reads a paper through Mova and writes its own description.
//!
//! Reading: the Markdown text is split into sections (headings) and paragraphs, paragraphs into
//! sentences (`world::quant::sentences`); every sentence is parsed by Mova's annotator into a UD
//! tree (`world::tree::Tree::parse`) and reduced to its clause skeletons (`world::read::skeleton`:
//! subject, modal, negation, verb, complements — the reading Mova uses for book summaries). Words
//! Mova's dictionary does not know (`en::morph::known_form`) are counted; they and the sentences
//! without a skeleton are what the Whale did not understand.
//!
//! Writing: an opening from the reading's own counts and most frequent nouns, then, section by
//! section, the skeletons realized as plain sentences, each bound to the source sentence it came
//! from. When more is read than the word budget allows, skeletons are taken evenly from every
//! section; a short paper is not padded. No language model writes or rewrites any of it.
//!
//! Checks: section coverage, a traceability check (every content word of a written sentence occurs
//! in its source sentence) and the longest verbatim run shared with the source.
use crate::Result;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use world::tree::Tree;

/// Sections whose text is not read as content. Appendices are read: they are part of the paper
/// (system descriptions, analyses), and their tables are left out like any other table.
const SKIPPED: [&str; 5] = ["references", "bibliography", "acknowledgments", "acknowledgements", "acknowledgment"];
pub const STOP: [&str; 40] = [
    "the", "and", "for", "that", "this", "with", "from", "are", "was", "were", "which", "have", "has", "been", "their", "they",
    "these", "those", "its", "our", "can", "not", "but", "also", "such", "than", "then", "there", "into", "each", "use", "used",
    "using", "one", "two", "may", "more", "most", "other", "all",
];

#[derive(Clone, Debug, Serialize)]
pub struct Sentence {
    pub index: usize,
    pub section: usize,
    pub text: String,
    pub admitted: bool,
    pub skeleton: Vec<String>,
    pub unknown: Vec<String>,
    /// Whole clauses restated in the describer's perspective (`clause.rs`).
    pub clauses: Vec<crate::clause::Clause>,
    /// Noun lemmas of the sentence (lower case, stop words left out).
    pub nouns: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Section {
    pub title: String,
    pub skipped: bool,
    pub sentences: Vec<usize>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Reading {
    pub title: String,
    pub words: usize,
    pub sections: Vec<Section>,
    pub sentences: Vec<Sentence>,
    /// Unknown word -> occurrences, over read sections.
    pub unknown: BTreeMap<String, usize>,
    /// Noun lemma -> occurrences, over read sections.
    pub nouns: BTreeMap<String, usize>,
    /// Acronyms the paper defines itself («Grammatical Error Correction (GEC)»): acronym ->
    /// (expansion, sentence index of the first definition).
    pub definitions: BTreeMap<String, (String, usize)>,
    /// Normalized unknown term (`knowledge::norm_term`) -> the form the paper uses most.
    pub forms: BTreeMap<String, String>,
    /// Sentences of read parts left out by the content filter (`furniture`), by reason; and
    /// restated clauses the grammar gate refused (`ungrammatical`).
    pub filtered: BTreeMap<String, usize>,
    /// Author and cited names (lower case): never unknown terms.
    pub names: BTreeSet<String>,
}

pub fn alpha_share(s: &str) -> f64 {
    let n = s.chars().filter(|c| !c.is_whitespace()).count().max(1);
    s.chars().filter(|c| c.is_alphabetic()).count() as f64 / n as f64
}

/// A plain text line: not markup, not a page marker, not the library's source/licence line.
fn content_line(l: &str) -> bool {
    let t = l.trim();
    !(t.is_empty() || t.starts_with("<!--") || t.starts_with("Джерело:") || t.starts_with("Source:") || t.starts_with('|'))
}

/// The default verbatim cap of a realized clause, in alphanumeric tokens.
pub const VERBATIM_CAP: usize = 12;

/// Read a paper's Markdown text through Mova.
pub fn read(markdown: &str) -> Result<Reading> {
    read_with(markdown, VERBATIM_CAP)
}

/// A short standalone line that names a part of the paper: «Edit Extraction», «4.2 Results».
/// PDF extraction keeps such subsection titles as one-line paragraphs without Markdown marks.
pub fn heading_line(l: &str) -> bool {
    let t = l.trim();
    let words: Vec<&str> = t.split_whitespace().collect();
    let first = t.chars().next();
    (1..=8).contains(&words.len())
        && first.is_some_and(|c| c.is_uppercase() || c.is_ascii_digit())
        && !t.ends_with(['.', ',', ';', ':', '?', '!'])
        && !t.contains("://")
        && !t.contains('@')
        && !t.starts_with('*')
        && alpha_share(t) >= 0.75
        && words.iter().filter(|w| w.chars().any(char::is_alphabetic)).all(|w| w.chars().next().is_some_and(|c| c.is_uppercase() || !c.is_alphabetic()) || matches!(*w, "of" | "and" | "for" | "the" | "a" | "an" | "in" | "on" | "to" | "with" | "vs" | "by"))
}

/// Read a paper's Markdown text through Mova; realized clauses keep at most `cap` words in a row
/// from their source sentence.
pub fn read_with(markdown: &str, cap: usize) -> Result<Reading> {
    let a = world::tree::annotator().map_err(|e| crate::util::error(e.to_string()))?;
    let repaired = repair(markdown);
    let markdown = repaired.as_str();
    let mut title = String::new();
    let mut sections = vec![Section { title: "Front matter".into(), skipped: true, sentences: vec![] }];
    let mut paragraphs: Vec<(usize, String)> = vec![];
    let mut cur = String::new();
    let flush = |cur: &mut String, paragraphs: &mut Vec<(usize, String)>, section: usize| {
        if !cur.trim().is_empty() {
            paragraphs.push((section, std::mem::take(cur)));
        }
        cur.clear();
    };
    let lines: Vec<&str> = markdown.lines().collect();
    for (li, line) in lines.iter().enumerate() {
        let t = line.trim();
        let alone = li > 0 && lines[li - 1].trim().is_empty() && lines.get(li + 1).is_none_or(|n| n.trim().is_empty());
        // A heading stands between prose: the nearest non-empty lines on both sides are sentences
        // (tables and lists of codes have short lines next to each other).
        let prose = |l: Option<&&str>| l.is_some_and(|l| { let l = l.trim(); l.split_whitespace().count() >= 8 && l.ends_with(['.', ':', ',']) || l.starts_with('#') });
        let near_prose = prose(lines[..li].iter().rev().find(|l| !l.trim().is_empty() && !l.trim().starts_with("<!--"))) && prose(lines[li + 1..].iter().find(|l| !l.trim().is_empty() && !l.trim().starts_with("<!--")));
        let alone = alone && near_prose;
        if !t.starts_with('#') && alone && sections.len() > 1 && heading_line(t) {
            flush(&mut cur, &mut paragraphs, sections.len() - 1);
            let lower = t.to_lowercase();
            let skipped = SKIPPED.iter().any(|s| lower.trim_start_matches(|c: char| c.is_ascii_digit() || c == '.' || c == ' ').starts_with(s));
            // A skipped part (references) ends only at a Markdown heading.
            if !sections.last().is_some_and(|s| s.skipped) || skipped {
                sections.push(Section { title: t.to_string(), skipped, sentences: vec![] });
            }
            continue;
        }
        if let Some(h) = t.strip_prefix('#') {
            flush(&mut cur, &mut paragraphs, sections.len() - 1);
            let level = 1 + h.chars().take_while(|c| *c == '#').count();
            let name = h.trim_start_matches('#').trim().to_string();
            if level == 1 && title.is_empty() {
                title = name;
                continue;
            }
            let lower = name.to_lowercase();
            let skipped = SKIPPED.iter().any(|s| lower.trim_start_matches(|c: char| c.is_ascii_digit() || c == '.' || c == ' ').starts_with(s));
            sections.push(Section { title: name, skipped, sentences: vec![] });
        } else if content_line(t) {
            cur.push_str(t);
            cur.push(' ');
        } else {
            flush(&mut cur, &mut paragraphs, sections.len() - 1);
        }
    }
    flush(&mut cur, &mut paragraphs, sections.len() - 1);
    let names = names(&paragraphs);
    let mut sentences = vec![];
    let (mut unknown, mut nouns) = (BTreeMap::new(), BTreeMap::new());
    let mut form_counts: BTreeMap<String, BTreeMap<String, usize>> = BTreeMap::new();
    let mut filtered: BTreeMap<String, usize> = BTreeMap::new();
    let mut definitions: BTreeMap<String, (String, usize)> = BTreeMap::new();
    let mut words = 0;
    // A bibliography under a content heading: from the first reference entry of a part, its short
    // paragraphs (continuations split by a page break: venues, titles, author lines) are the
    // bibliography too, until a long prose paragraph or the next part.
    let mut tail: Option<usize> = None;
    let mut entries: Vec<usize> = vec![];
    // `WHALE_READ_DEBUG`: every filtered sentence and refused clause on stderr.
    let debug = std::env::var_os("WHALE_READ_DEBUG").is_some();
    if debug {
        for s in &sections {
            eprintln!("SECTION [{}] {}", if s.skipped { "skip" } else { "read" }, s.title);
        }
    }
    for (sec, p) in paragraphs {
        words += p.split_whitespace().count();
        if sections[sec].skipped {
            continue;
        }
        if tail.is_some_and(|t| t != sec) {
            tail = None;
        }
        if reference_entry(&p) {
            // Two author-year entries in a part open its bibliography tail; a venue or page
            // header alone does not (PDF page headers sit inside content parts).
            if author_year(&p).is_some() {
                entries.push(sec);
                if entries.iter().filter(|&&x| x == sec).count() >= 2 {
                    tail = Some(sec);
                }
            }
            *filtered.entry("bibliography".into()).or_insert(0) += 1;
            if debug {
                eprintln!("BIBLIOGRAPHY: {p}");
            }
            continue;
        }
        if tail == Some(sec) {
            if p.split_whitespace().count() <= 60 || furniture(&p).is_some() {
                *filtered.entry("bibliography".into()).or_insert(0) += 1;
                if debug {
                    eprintln!("BIBLIOGRAPHY TAIL: {p}");
                }
                continue;
            }
            tail = None;
        }
        for s in join_abbreviations(world::quant::sentences(&p)) {
            let n = s.split_whitespace().count();
            if !(4..=80).contains(&n) || alpha_share(&s) < 0.6 || residue(&s) {
                continue;
            }
            if let Some(why) = furniture(&s) {
                *filtered.entry(why.into()).or_insert(0) += 1;
                if debug {
                    eprintln!("FILTER {why}: {s}");
                }
                continue;
            }
            let toks = a.tokenize(&s);
            let orig: Vec<String> = toks.iter().map(|t| t.form.clone()).collect();
            let t = Tree::parse(a, &s);
            let admitted = t.root().is_some() && t.w.len() == orig.len();
            let skeleton = if admitted { world::read::skeleton(&t, &orig) } else { vec![] };
            let clauses: Vec<crate::clause::Clause> = if let Some(c) = caption(&s) {
                // A table or figure caption is restated as what the table or figure shows.
                let toks = a.tokenize(&c);
                crate::clause::clauses(&Tree::parse(a, &c), &toks, cap)
            } else if admitted {
                let c = crate::clause::clauses(&t, &toks, cap);
                // A question is restated as the paper's question, never as a statement.
                if s.trim_end().ends_with('?') {
                    crate::clause::question(&t, &toks, cap).into_iter().collect()
                } else {
                    uncovered_segments(a, &toks, c, cap)
                }
            } else {
                vec![]
            };
            // The grammar gate: a restated clause that Mova, reading it again on its own, does not
            // parse as a whole clause is not used.
            let before = clauses.len();
            let clauses: Vec<crate::clause::Clause> = clauses
                .into_iter()
                .filter(|c| match crate::clause::grammatical(a, &c.text) {
                    Ok(()) => true,
                    Err(why) => {
                        if debug {
                            eprintln!("UNGRAMMATICAL {why}: {} <= {s}", c.text);
                        }
                        false
                    }
                })
                .collect();
            if clauses.len() < before {
                *filtered.entry("ungrammatical".into()).or_insert(0) += before - clauses.len();
            }
            for (acr, exp) in acronym_definitions(&orig) {
                definitions.entry(acr).or_insert((exp, sentences.len()));
            }
            let mut unk = vec![];
            let mut snouns = vec![];
            for (i, f) in orig.iter().enumerate() {
                let lower = f.to_lowercase();
                if f.chars().count() >= 3 && f.chars().all(char::is_alphabetic) && !en::morph::known_form(&lower) && !citation_name(&orig, i) && !names.contains(&lower) {
                    let key = crate::knowledge::norm_term(f);
                    if !en::morph::known_form(&key) {
                        *unknown.entry(key.clone()).or_insert(0) += 1;
                        *form_counts.entry(key.clone()).or_default().entry(f.clone()).or_insert(0) += 1;
                        if !unk.contains(&key) {
                            unk.push(key);
                        }
                    }
                }
                if admitted && t.w[i].upos == en::gram::UPos::NOUN && t.w[i].lemma.len() >= 3 && !STOP.contains(&t.w[i].lemma.as_str()) {
                    *nouns.entry(t.w[i].lemma.to_lowercase()).or_insert(0) += 1;
                    snouns.push(t.w[i].lemma.to_lowercase());
                }
            }
            let index = sentences.len();
            sections[sec].sentences.push(index);
            sentences.push(Sentence { index, section: sec, text: s, admitted, skeleton, unknown: unk, clauses, nouns: snouns });
        }
    }
    // The form a term is asked and explained under: the paper's most frequent spelling.
    let forms = form_counts.into_iter().map(|(k, m)| (k, top(&m, 1).first().map(|(f, _)| (*f).clone()).unwrap_or_default())).collect();
    Ok(Reading { title, words, sections, sentences, unknown, nouns, definitions, forms, filtered, names })
}

/// PDF small capitals split into a capital and the rest («S TACK P TR», «A BSTRACT»): rejoined
/// where the joined word is a word Mova knows or a split that repeats in the text, and next to
/// such a join on the same line (a run of small capitals: «S TACK P TR» -> «STACK PTR»). A lone
/// «A» or «I» before an acronym («A CNN») stays unless the joined word is known.
pub fn repair(text: &str) -> String {
    // Small-capital names repeat («S TACK P TR», «B I AF», «P ROBLE M ATHIC»): a split pair or
    // chain that occurs at least twice in the text is joined even when Mova does not know the
    // word, unless it starts with the words «A» or «I» («A CNN»).
    let strip = |x: &str| x.trim_start_matches(['(', '[', '“', '"', '\'']).to_string();
    let caps = |x: &str| {
        let r: String = x.chars().take_while(|c| c.is_ascii_alphabetic()).collect();
        (r.len(), !r.is_empty() && r.chars().all(|c| c.is_ascii_uppercase()))
    };
    let single = |x: &str| {
        let c = strip(x);
        c.len() == 1 && c.chars().all(|ch| ch.is_ascii_uppercase())
    };
    let mut seen: BTreeMap<String, usize> = BTreeMap::new();
    for line in text.lines() {
        let w: Vec<&str> = line.split(' ').collect();
        for i in 0..w.len().saturating_sub(1) {
            if single(w[i]) && caps(w[i + 1]).1 && caps(w[i + 1]).0 >= 2 {
                let rest: String = w[i + 1].chars().take_while(|c| c.is_ascii_alphabetic()).collect();
                *seen.entry(format!("{}{rest}", strip(w[i]))).or_insert(0) += 1;
                if i > 0 && single(w[i - 1]) {
                    *seen.entry(format!("{}{}{rest}", strip(w[i - 1]), strip(w[i]))).or_insert(0) += 1;
                }
            }
        }
    }
    let repeated = |head: &str, word: &str| !matches!(head, "A" | "I") && seen.get(word).is_some_and(|&n| n >= 2);
    let mut out = String::with_capacity(text.len());
    for (n, line) in text.split('\n').enumerate() {
        if n > 0 {
            out.push('\n');
        }
        let w: Vec<&str> = line.split(' ').collect();
        // The capital may follow an opening bracket or quote: «(S TACK P TR )».
        let letter = |x: &str| -> bool {
            let core = x.trim_start_matches(['(', '[', '“', '"', '\'']);
            core.len() == 1 && core.chars().all(|c| c.is_ascii_uppercase())
        };
        let split = |i: usize| -> bool {
            i + 1 < w.len() && letter(w[i]) && {
                let rest: String = w[i + 1].chars().take_while(|c| c.is_ascii_alphabetic()).collect();
                rest.len() >= 2 && rest.chars().all(|c| c.is_ascii_uppercase())
            }
        };
        let known = |i: usize| -> bool {
            let rest: String = w[i + 1].chars().take_while(|c| c.is_ascii_alphabetic()).collect();
            let head = w[i].trim_start_matches(['(', '[', '“', '"', '\'']);
            en::morph::known_form(&format!("{head}{rest}").to_lowercase()) || repeated(head, &format!("{head}{rest}"))
        };
        let mut join = vec![false; w.len()];
        for i in 0..w.len() {
            join[i] = split(i) && known(i);
        }
        // A capital before a capital that opens a split pair: «B I AF» (BiAF) -> the pair first,
        // then the capital before it (right to left, below).
        let chain = |i: usize| -> bool { i + 2 < w.len() && letter(w[i]) && letter(w[i + 1]) && split(i + 1) };
        // Neighbours of a known join on the same line belong to the same run.
        loop {
            let mut changed = false;
            for i in 0..w.len() {
                if !join[i] && split(i) && ((i >= 2 && join[i - 2]) || (i >= 3 && join[i - 3] && !split(i - 2) && caps(w[i - 1]).1) || join.get(i + 2).copied().unwrap_or(false)) {
                    join[i] = true;
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        // A chain «B I AF» joins whole when its pair joins or occurs again on the line.
        let mut whole = vec![false; w.len()];
        for i in 0..w.len() {
            if chain(i) {
                let head = strip(w[i]);
                let rest: String = w[i + 2].chars().take_while(|c| c.is_ascii_alphabetic()).collect();
                whole[i] = join[i + 1] || repeated(&head, &format!("{head}{}{rest}", strip(w[i + 1])));
            }
        }
        let mut i = 0;
        let mut first = true;
        while i < w.len() {
            if !first {
                out.push(' ');
            }
            first = false;
            if whole[i] {
                out.push_str(w[i]);
                out.push_str(w[i + 1]);
                out.push_str(w[i + 2]);
                i += 3;
            } else if join[i] {
                out.push_str(w[i]);
                out.push_str(w[i + 1]);
                i += 2;
            } else {
                out.push_str(w[i]);
                i += 1;
            }
        }
    }
    out
}

/// The quality of a paper's text extraction, measured before a paper is assigned.
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct Extraction {
    /// Alphabetic tokens.
    pub words: usize,
    /// Capital letters split from the rest of their word that `repair` cannot rejoin, per 1,000
    /// words.
    pub spacing_residue_per_1000: f64,
    /// Words of at least three ASCII letters, lower case or capitals (capitalized names left out),
    /// that Mova does not know after `repair`, as a share of all such words.
    pub unknown_share: f64,
    /// Prose paragraphs cut off mid-sentence (no closing punctuation, the next prose paragraph
    /// starts a new sentence), as a share of prose paragraphs.
    pub truncated_share: f64,
    pub prose_paragraphs: usize,
    /// Words before the first section heading (`##`), as a share of all words: the reader takes
    /// them as front matter and does not read them, so a text whose headings were lost in
    /// extraction is mostly unread.
    pub front_share: f64,
    /// Up to eight distinct residue examples («P ROBLE»), for the re-extraction record.
    pub residue_examples: Vec<String>,
}

/// Limits above which a paper's extraction counts as broken.
#[derive(Clone, Debug, serde::Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ExtractionLimits {
    #[serde(default = "d_residue")]
    pub max_spacing_residue_per_1000: f64,
    #[serde(default = "d_unknown")]
    pub max_unknown_share: f64,
    #[serde(default = "d_truncated")]
    pub max_truncated_share: f64,
    #[serde(default = "d_front")]
    pub max_front_share: f64,
}
fn d_front() -> f64 {
    0.4
}
fn d_residue() -> f64 {
    2.0
}
fn d_unknown() -> f64 {
    0.12
}
fn d_truncated() -> f64 {
    0.3
}
impl Default for ExtractionLimits {
    fn default() -> Self {
        Self { max_spacing_residue_per_1000: d_residue(), max_unknown_share: d_unknown(), max_truncated_share: d_truncated(), max_front_share: d_front() }
    }
}

impl Extraction {
    /// The limits this extraction exceeds (empty: the text is fit to read).
    pub fn broken(&self, l: &ExtractionLimits) -> Vec<&'static str> {
        let mut v = vec![];
        if self.spacing_residue_per_1000 > l.max_spacing_residue_per_1000 {
            v.push("spacing_residue");
        }
        if self.unknown_share > l.max_unknown_share {
            v.push("unknown_words");
        }
        if self.truncated_share > l.max_truncated_share {
            v.push("truncated_paragraphs");
        }
        if self.front_share > l.max_front_share {
            v.push("no_section_headings");
        }
        v
    }
}

/// Measure a paper's extracted Markdown (no parsing; a few milliseconds).
pub fn extraction(markdown: &str) -> Extraction {
    let repaired = repair(markdown);
    let mut words = 0usize;
    let mut residue = 0usize;
    let mut examples: Vec<String> = vec![];
    let (mut lower, mut unknown) = (0usize, 0usize);
    let mut front = 0usize;
    let mut headed = false;
    for line in repaired.lines() {
        if line.trim_start().starts_with("##") {
            headed = true;
        }
        if !headed && content_line(line) && !line.trim_start().starts_with('#') {
            front += line.split_whitespace().filter(|x| x.chars().any(char::is_alphabetic)).count();
        }
    }
    for line in repaired.lines().filter(|l| content_line(l)) {
        let w: Vec<&str> = line.split_whitespace().collect();
        for (i, x) in w.iter().enumerate() {
            let alpha: String = x.chars().filter(|c| c.is_alphabetic()).collect();
            if alpha.is_empty() {
                continue;
            }
            words += 1;
            let core = x.trim_start_matches(['(', '[', '“', '"', '\'']);
            if core.len() == 1 && core.chars().all(|c| c.is_ascii_uppercase()) && w.get(i + 1).is_some_and(|n| n.chars().take_while(|c| c.is_ascii_alphabetic()).count() >= 2 && n.chars().take_while(|c| c.is_ascii_alphabetic()).all(|c| c.is_ascii_uppercase())) {
                residue += 1;
                let ex = format!("{x} {}", w[i + 1]);
                if examples.len() < 8 && !examples.contains(&ex) {
                    examples.push(ex);
                }
            }
            // Words of three or more letters, any case (fragments of split words are mostly
            // capitals), looked up in lower case; a capitalized word only when it is not a name
            // (it does not start a capitalized pair «Yoav Goldberg» and is not before «et al.»).
            let t = x.trim_matches(|c: char| !c.is_alphabetic());
            let name = t.chars().next().is_some_and(char::is_uppercase) && t.chars().skip(1).any(char::is_lowercase);
            if t.chars().count() >= 3 && t.chars().all(|c| c.is_ascii_alphabetic()) && !name {
                lower += 1;
                if !en::morph::known_form(&t.to_lowercase()) {
                    unknown += 1;
                }
            }
        }
    }
    // Paragraphs: blocks of content lines between empty lines.
    let mut paras: Vec<String> = vec![];
    let mut cur = String::new();
    for line in repaired.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') || t.starts_with("<!--") || t.starts_with('|') {
            if !cur.trim().is_empty() {
                paras.push(std::mem::take(&mut cur));
            }
            cur.clear();
        } else {
            cur.push_str(t);
            cur.push(' ');
        }
    }
    if !cur.trim().is_empty() {
        paras.push(cur);
    }
    let prose: Vec<&String> = paras.iter().filter(|p| p.split_whitespace().count() >= 15 && alpha_share(p) >= 0.7 && !reference_entry(p)).collect();
    let mut truncated = 0;
    for k in 0..prose.len() {
        let p = prose[k].trim_end();
        let open = p.chars().last().is_some_and(|c| c.is_alphabetic());
        let next_new = prose.get(k + 1).and_then(|n| n.trim_start().chars().next()).is_some_and(|c| c.is_uppercase());
        if open && next_new {
            truncated += 1;
        }
    }
    let per = |n: usize, d: usize| if d == 0 { 0.0 } else { n as f64 / d as f64 };
    let round = |x: f64| (x * 10000.0).round() / 10000.0;
    Extraction {
        words,
        spacing_residue_per_1000: round(per(residue * 1000, words)),
        unknown_share: round(per(unknown, lower)),
        truncated_share: round(per(truncated, prose.len())),
        prose_paragraphs: prose.len(),
        front_share: round(per(front, words)),
        residue_examples: examples,
    }
}

/// Why a sentence of a read part is not content, or None: page furniture (addresses, venue and
/// page lines, footnote markers), a list of names (an author line or a bibliography entry cut by
/// a page break), a table row (mostly numbers or codes), maths residue (formula symbols, single
/// letters), a citation list (mostly bracketed).
pub fn furniture(s: &str) -> Option<&'static str> {
    let t = s.trim();
    let w: Vec<&str> = t.split_whitespace().collect();
    if w.is_empty() {
        return Some("empty");
    }
    if t.contains("://") || t.contains("www.") || (t.contains('@') && t.contains('.')) || t.contains("arXiv:") || t.contains("Proceedings of") || t.contains("Association for Computational Linguistics") || t.contains("doi.org") {
        return Some("page_furniture");
    }
    if w.windows(2).any(|p| matches!(p[0], "pages" | "pp." | "Pages") && p[1].chars().next().is_some_and(|c| c.is_ascii_digit())) {
        return Some("page_furniture");
    }
    let first = t.chars().next().unwrap_or(' ');
    if first.is_ascii_digit() && t.chars().nth(w[0].len() + 1).is_some_and(|c| "♢∗*†‡§¶".contains(c)) {
        return Some("page_furniture");
    }
    // Names: capitalized words, commas, «and», initials; no lower-case content word.
    let alpha: Vec<&str> = w.iter().map(|x| x.trim_matches(|c: char| !c.is_alphabetic())).filter(|x| !x.is_empty()).collect();
    let particles = ["and", "de", "van", "von", "der", "den", "la", "le", "di", "da", "du", "del", "et", "al", "y"];
    let capital = alpha.iter().filter(|x| x.chars().next().is_some_and(char::is_uppercase)).count();
    let lowercase_content = alpha.iter().filter(|x| x.chars().next().is_some_and(char::is_lowercase) && !particles.contains(x)).count();
    let joins = t.matches(',').count() + w.iter().filter(|x| **x == "and").count();
    if alpha.len() >= 3 && lowercase_content == 0 && joins >= 1 && capital * 10 >= alpha.len() * 7 && !t.ends_with('?') {
        return Some("name_list");
    }
    let n = w.len() as f64;
    let digits = w.iter().filter(|x| x.chars().any(|c| c.is_ascii_digit()) && !x.chars().any(char::is_alphabetic)).count() as f64;
    let caps = w.iter().filter(|x| { let a: String = x.chars().filter(|c| c.is_alphabetic()).collect(); a.chars().count() >= 2 && a.chars().all(char::is_uppercase) }).count() as f64;
    if w.len() >= 4 && (digits / n >= 0.35 || caps / n >= 0.6) {
        return Some("table_row");
    }
    // Formula symbols (arrows are left out: papers write examples as «eat → consume»).
    const MATH: &str = "∑∏∫√∈∉≤≥≠≈±×÷∀∃ϕφθλμσαβγδεηρπ⊕⊗∥∂∇̸̂⋅·∗′";
    let symbols = t.chars().filter(|c| MATH.contains(*c)).count();
    let singles = w.iter().filter(|x| x.chars().count() == 1 && x.chars().all(char::is_alphabetic) && !matches!(**x, "a" | "A" | "I")).count() as f64;
    if symbols >= 2 || (w.len() >= 5 && singles / n >= 0.35) || (t.contains(" = ") && singles >= 3.0) {
        return Some("maths");
    }
    // Mostly bracketed citations: brackets hold most of the words, and they cite (years, «et al.»).
    let mut depth = 0i32;
    let mut inside = 0usize;
    let mut cites = 0usize;
    for x in &w {
        let opens = x.matches('(').count() as i32;
        let closes = x.matches(')').count() as i32;
        if depth > 0 || opens > 0 {
            inside += 1;
            let y: String = x.chars().filter(char::is_ascii_digit).collect();
            if (y.len() == 4 && (y.starts_with("19") || y.starts_with("20"))) || *x == "al." {
                cites += 1;
            }
        }
        depth = (depth + opens - closes).max(0);
    }
    if w.len() >= 6 && inside * 10 >= w.len() * 6 && cites >= 2 {
        return Some("citation_list");
    }
    None
}

/// Cited and author names of a paper (lower case): the author part of its bibliography entries
/// (before «. 2016.»), name lists, and names before «et al.», «and X (2016)» or a year.
pub fn names(paragraphs: &[(usize, String)]) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    fn add(out: &mut BTreeSet<String>, part: &str) {
        for x in part.split(|c: char| !(c.is_alphabetic() || c == '-' || c == '\'' || c == '’')) {
            let x = x.trim_matches(|c: char| c == '-' || c == '\'' || c == '’');
            if x.chars().count() >= 2 && x.chars().next().is_some_and(char::is_uppercase) && x.chars().skip(1).any(char::is_lowercase) {
                out.insert(x.to_lowercase());
            }
        }
    }
    for (_, p) in paragraphs {
        if reference_entry(p) || author_year(p).is_some() {
            if let Some(k) = author_year(p) {
                add(&mut out, &p[..k]);
            }
            continue;
        }
        for s in world::quant::sentences(p) {
            if furniture(&s) == Some("name_list") {
                add(&mut out, &s);
            }
        }
        let w: Vec<String> = p.split_whitespace().map(|x| x.trim_matches(|c: char| !(c.is_alphanumeric() || c == '-')).to_string()).collect();
        let orig: Vec<String> = p.split_whitespace().map(String::from).collect();
        for i in 0..w.len() {
            let year = |x: Option<&String>| x.is_some_and(|y| y.chars().take(4).filter(|c| c.is_ascii_digit()).count() == 4 && (y.starts_with("19") || y.starts_with("20")));
            let cited = orig.get(i + 1).is_some_and(|x| x == "et") || year(w.get(i + 1)) || (orig.get(i).is_some_and(|x| x.ends_with(',')) && year(w.get(i + 1)));
            let pair = orig.get(i + 1).is_some_and(|x| x == "and" || x == "&") && w.get(i + 2).is_some_and(|x| x.chars().next().is_some_and(char::is_uppercase)) && year(w.get(i + 3));
            if (cited || pair) && w[i].chars().next().is_some_and(char::is_uppercase) && w[i].chars().skip(1).any(char::is_lowercase) {
                out.insert(w[i].to_lowercase());
                if pair {
                    out.insert(w[i + 2].to_lowercase());
                }
            }
        }
    }
    out
}

/// The byte offset of «. 2016.» after an author list at the head of a paragraph (a bibliography
/// entry, with or without its venue), or None.
pub fn author_year(p: &str) -> Option<usize> {
    let head: String = p.chars().take(400).collect();
    let bytes = head.as_bytes();
    for (k, _) in head.match_indices(". ") {
        let rest = &head[k + 2..];
        let y: String = rest.chars().take(4).collect();
        let after = rest.chars().nth(4);
        if y.len() == 4 && y.chars().all(|c| c.is_ascii_digit()) && (y.starts_with("19") || y.starts_with("20")) && after.is_some_and(|c| c == '.' || c.is_ascii_lowercase()) {
            let authors = &head[..k];
            let _ = bytes;
            if furniture(&format!("{authors}.")) == Some("name_list") || authors.split_whitespace().count() <= 3 && authors.chars().next().is_some_and(char::is_uppercase) {
                return Some(k);
            }
            return None;
        }
    }
    None
}

/// «*Table 2: The list of error categories.*» -> «Table 2 shows the list of error categories.»
pub fn caption(s: &str) -> Option<String> {
    let t = s.trim().trim_matches('*').trim();
    let (head, rest) = t.split_once(':')?;
    let mut w = head.split_whitespace();
    let (kind, num) = (w.next()?, w.next()?);
    if w.next().is_some() || !matches!(kind, "Table" | "Figure" | "Fig.") || !num.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let rest = rest.trim().trim_end_matches(['.', '*']).trim();
    let first = rest.chars().next()?;
    let rest = if first.is_uppercase() && rest.split_whitespace().next().is_some_and(|x| x.chars().skip(1).all(|c| c.is_lowercase())) { format!("{}{}", first.to_lowercase(), &rest[first.len_utf8()..]) } else { rest.to_string() };
    (rest.split_whitespace().count() >= 3).then(|| format!("{} {num} shows {rest}.", if kind == "Table" { "Table" } else { "Figure" }))
}

/// Comma, semicolon or colon segments of a sentence that its whole parse left (almost) unrestated
/// are parsed on their own: a wrong root over a long sentence often loses a whole stretch whose own
/// parse is sound (when the whole parse yields nothing, every segment is tried). The fronted
/// context of a segment is lost; its words are the source's own. Clauses already found stay first;
/// a segment's clauses follow in source order.
fn uncovered_segments(a: &en::annotate::Annotator, toks: &[en::tok::Tok], mut found: Vec<crate::clause::Clause>, cap: usize) -> Vec<crate::clause::Clause> {
    let alnum = |i: usize| toks[i].form.chars().any(char::is_alphanumeric);
    let cov: BTreeSet<usize> = found.iter().flat_map(|c| c.src.iter().copied()).collect();
    let mut start = 0;
    let mut extra = vec![];
    for end in (0..=toks.len()).filter(|&i| i == toks.len() || matches!(toks[i].form.as_str(), "," | ";" | ":")) {
        let seg: Vec<usize> = (start..end).filter(|&i| alnum(i)).collect();
        let open = seg.iter().filter(|i| !cov.contains(i)).count();
        if seg.len() >= 5 && end - start < toks.len() && open * 10 >= seg.len() * 8 {
            let mut text = String::new();
            for i in start..end {
                text.push_str(&toks[i].form);
                if toks[i].space_after && i + 1 < end {
                    text.push(' ');
                }
            }
            let st = a.tokenize(text.trim());
            let t = Tree::parse(a, text.trim());
            if t.w.len() == st.len() {
                let cs = crate::clause::clauses(&t, &st, cap);
                if cs.is_empty() {
                    // A subordinate clause cut off from the clause it modifies is attached to that
                    // clause again: the nearest clause after it, else the nearest before it.
                    if let Some((sub, srcs)) = crate::clause::subordinate(&t, &st, cap) {
                        let host = found.iter().position(|c| c.src.iter().min().is_some_and(|&m| m >= end)).or_else(|| found.iter().rposition(|c| c.src.iter().max().is_some_and(|&m| m < start)));
                        if let Some(k) = host.filter(|&k| found[k].text.split_whitespace().count() + sub.split_whitespace().count() <= 60 && matches!(found[k].kind, "main" | "conj" | "parataxis")) {
                            let c = &mut found[k];
                            c.text = format!("{}, {sub}.", c.text.trim_end_matches(['.', '!', '?']));
                            c.src.extend(srcs.iter().map(|&i| i + start));
                            c.kind = "attached";
                        }
                    }
                }
                for mut c in cs {
                    c.src = c.src.iter().map(|&i| i + start).collect();
                    extra.push(c);
                }
            }
        }
        start = end + 1;
    }
    found.extend(extra);
    found
}

/// Sentence splits after «et al.», «e.g.», «i.e.», «cf.», «Fig.», «Eq.», «Sec.» or «vs.» are not
/// sentence ends: such a piece is joined with the next one.
pub fn join_abbreviations(v: Vec<String>) -> Vec<String> {
    const ABBR: [&str; 9] = [" al.", "e.g.", "i.e.", " cf.", " Fig.", " Eq.", " Sec.", " vs.", " etc."];
    let mut out: Vec<String> = vec![];
    let mut open = false;
    for s in v {
        if open {
            if let Some(last) = out.last_mut() {
                last.push(' ');
                last.push_str(&s);
            }
        } else {
            out.push(s);
        }
        open = out.last().is_some_and(|l| ABBR.iter().any(|a| l.ends_with(a) || l == a.trim()));
    }
    out
}

/// Acronym definitions in a sentence's tokens: «… Grammatical Error Correction ( GEC ) …». The
/// words before the bracket whose initials spell the acronym (small function words may sit
/// between them) are its expansion.
pub fn acronym_definitions(orig: &[String]) -> Vec<(String, String)> {
    const SMALL: [&str; 7] = ["of", "and", "for", "the", "to", "in", "on"];
    let mut out = vec![];
    for i in 1..orig.len().saturating_sub(1) {
        let acr = orig[i].trim_end_matches('s');
        if orig[i - 1] != "(" || orig.get(i + 1).map(String::as_str) != Some(")") || !(2..=8).contains(&acr.chars().count()) || !acr.chars().all(|c| c.is_ascii_uppercase()) {
            continue;
        }
        let letters: Vec<char> = acr.chars().map(|c| c.to_ascii_lowercase()).collect();
        let mut need = letters.len();
        let mut j = i - 1;
        let mut words: Vec<&str> = vec![];
        while j > 0 && need > 0 && words.len() < letters.len() * 2 {
            j -= 1;
            let w = orig[j].as_str();
            let first = w.chars().next().map(|c| c.to_ascii_lowercase());
            if first == Some(letters[need - 1]) && w.chars().all(|c| c.is_alphabetic() || c == '-') {
                need -= 1;
                words.push(w);
            } else if SMALL.contains(&w.to_lowercase().as_str()) && !words.is_empty() {
                words.push(w);
            } else {
                break;
            }
        }
        if need == 0 {
            words.reverse();
            out.push((orig[i].clone(), words.join(" ")));
        }
    }
    out
}

/// A bibliography entry in a part that is not marked as references (PDF extraction often puts the
/// reference list under the last heading): «Name, Name, and Name. 2016. Title. In Proceedings of
/// …», a venue line «In Proceedings of ACL-2015 … pages 344–354.», an arXiv line.
pub fn reference_entry(p: &str) -> bool {
    let t = p.trim();
    let head: String = t.chars().take(200).collect();
    let year = head.split(|c: char| !c.is_ascii_alphanumeric()).enumerate().any(|(k, w)| k > 0 && w.len() == 4 && (w.starts_with("19") || w.starts_with("20")) && w.chars().all(|c| c.is_ascii_digit()))
        && (head.contains(". 19") || head.contains(". 20") || head.contains(", 19") || head.contains(", 20"));
    let venue = t.contains("In Proceedings") || t.contains("In Proceeding of") || t.contains("arXiv preprint") || t.contains("arXiv:") || t.contains("Association for Computational Linguistics") || t.contains("Transactions of the") || t.contains("Journal of");
    let pages = t.split_whitespace().collect::<Vec<_>>().windows(2).any(|w| w[0] == "pages" && w[1].chars().next().is_some_and(|c| c.is_ascii_digit()));
    let short = t.split_whitespace().count() <= 80;
    short && ((year && (venue || pages)) || (t.starts_with("In ") && (venue || pages)) || (venue && pages) || author_year(t).is_some())
}

/// Extraction residue rather than prose: table rows (few distinct words), proceedings and page
/// headers, citation lists.
fn residue(s: &str) -> bool {
    let w: Vec<String> = s.split_whitespace().map(str::to_lowercase).collect();
    let distinct = w.iter().collect::<BTreeSet<_>>().len();
    distinct * 2 < w.len() || s.contains("Proceedings of") || s.matches("et al.").count() >= 3
}

/// A capitalized word used as a cited author: «Nivre et al.», «Nivre, 2016», «(Nivre 2016)».
fn citation_name(orig: &[String], i: usize) -> bool {
    let year = |x: Option<&String>| x.is_some_and(|y| y.len() == 4 && y.chars().all(|c| c.is_ascii_digit()) && (y.starts_with("19") || y.starts_with("20")));
    orig[i].chars().next().is_some_and(char::is_uppercase)
        && (orig.get(i + 1).is_some_and(|x| x == "et" || x == "and" || x == "&") || year(orig.get(i + 1)) || (orig.get(i + 1).is_some_and(|x| x == ",") && year(orig.get(i + 2))))
}

pub fn tokens(s: &str) -> Vec<String> {
    s.split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()).map(str::to_lowercase).collect()
}
pub fn content(s: &str) -> Vec<String> {
    tokens(s).into_iter().filter(|w| w.chars().count() >= 3 && !STOP.contains(&w.as_str())).collect()
}

/// One written sentence and the source sentence it came from (None: the Whale's own counts).
#[derive(Clone, Debug, Serialize)]
pub struct Written {
    pub text: String,
    pub source: Option<usize>,
    pub section: Option<usize>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Description {
    pub text: String,
    pub sentences: Vec<Written>,
    pub words: usize,
    pub available_words: usize,
}

fn realize(skel: &str) -> String {
    mova_octopus::realize::sentence(skel.trim_end_matches([',', ';', ':']))
}

pub fn top<'a>(m: &'a BTreeMap<String, usize>, n: usize) -> Vec<(&'a String, &'a usize)> {
    let mut v: Vec<_> = m.iter().collect();
    v.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    v.truncate(n);
    v
}

/// Skeletons worth writing: 4..=40 words, mostly letters, not seen before.
fn candidates(r: &Reading) -> Vec<Vec<(usize, String)>> {
    let mut seen = BTreeSet::new();
    let mut out = vec![vec![]; r.sections.len()];
    for s in &r.sentences {
        for k in &s.skeleton {
            let n = k.split_whitespace().count();
            if (4..=40).contains(&n) && alpha_share(k) >= 0.7 && seen.insert(k.to_lowercase()) {
                out[s.section].push((s.index, k.clone()));
            }
        }
    }
    out
}

/// Write the description: at most `max` words; under `min` when the reading has less.
pub fn write(r: &Reading, max: usize) -> Description {
    let parsed = r.sentences.iter().filter(|s| s.admitted).count();
    let read_sections = r.sections.iter().filter(|s| !s.skipped && !s.sentences.is_empty()).count();
    let nouns: Vec<String> = top(&r.nouns, 8).into_iter().map(|(k, _)| k.clone()).collect();
    let mut sentences = vec![
        Written { text: mova_octopus::realize::sentence(&format!("I read «{}»", if r.title.is_empty() { "an untitled paper" } else { &r.title })), source: None, section: None },
        Written {
            text: mova_octopus::realize::sentence(&format!(
                "It has {} words; I read {} of its sections and parsed {} of its {} sentences",
                mova_octopus::realize::thousands(r.words as u64),
                read_sections,
                parsed,
                r.sentences.len()
            )),
            source: None,
            section: None,
        },
    ];
    if !nouns.is_empty() {
        sentences.push(Written { text: mova_octopus::realize::sentence(&format!("The things it speaks of most are {}", mova_octopus::realize::list(&nouns))), source: None, section: None });
    }
    let cands = candidates(r);
    let available: usize = cands.iter().flatten().map(|(_, k)| k.split_whitespace().count()).sum();
    let mut used: usize = sentences.iter().map(|w| w.text.split_whitespace().count()).sum();
    let headers: usize = cands.iter().enumerate().filter(|(_, c)| !c.is_empty()).map(|(i, _)| r.sections[i].title.split_whitespace().count() + 6).sum();
    // The share of every section's skeletons that fits the budget, the same for all sections.
    let room = max.saturating_sub(used + headers);
    let share = if available <= room { 1.0 } else { room as f64 / available as f64 };
    for (sec, c) in cands.iter().enumerate() {
        if c.is_empty() {
            continue;
        }
        let keep = ((c.len() as f64 * share).floor() as usize).max(1).min(c.len());
        let head = Written { text: mova_octopus::realize::sentence(&format!("In the part «{}», the paper says this", r.sections[sec].title)), source: None, section: Some(sec) };
        let mut part = vec![head];
        // Evenly spaced picks keep the whole section in view, deterministically.
        for j in 0..keep {
            let (src, k) = &c[j * c.len() / keep];
            part.push(Written { text: realize(k), source: Some(*src), section: Some(sec) });
        }
        let n: usize = part.iter().map(|w| w.text.split_whitespace().count()).sum();
        if used + n > max {
            // Take this part only up to the budget, whole sentences only.
            for w in part {
                let m = w.text.split_whitespace().count();
                if used + m > max {
                    break;
                }
                used += m;
                sentences.push(w);
            }
            break;
        }
        used += n;
        sentences.extend(part);
    }
    let mut text = String::new();
    let mut last = usize::MAX;
    for w in &sentences {
        let sec = w.section.unwrap_or(usize::MAX - 1);
        if !text.is_empty() {
            text.push_str(if sec != last { "\n\n" } else { " " });
        }
        last = sec;
        text.push_str(&w.text);
    }
    text.push('\n');
    let words = crate::contract::word_count(&text);
    Description { text, sentences, words, available_words: available }
}

/// Measurements of a description against its reading.
pub fn measure(r: &Reading, d: &Description) -> serde_json::Value {
    let readable: Vec<usize> = (0..r.sections.len()).filter(|&i| !r.sections[i].skipped && !r.sections[i].sentences.is_empty()).collect();
    let described: BTreeSet<usize> = d.sentences.iter().filter(|w| w.source.is_some()).filter_map(|w| w.section).collect();
    let mut traceable = 0;
    let mut sourced = 0;
    let mut runs = vec![];
    for w in d.sentences.iter().filter(|w| w.source.is_some()) {
        sourced += 1;
        let src = &r.sentences[w.source.unwrap_or(0)].text;
        let have: BTreeSet<String> = tokens(src).into_iter().collect();
        if content(&w.text).iter().all(|t| have.contains(t)) {
            traceable += 1;
        }
        runs.push(longest_run(&tokens(&w.text), &tokens(src)));
    }
    runs.sort();
    let unparsed = r.sentences.iter().filter(|s| !s.admitted).count();
    let empty = r.sentences.iter().filter(|s| s.admitted && s.skeleton.is_empty()).count();
    serde_json::json!({
        "paper_words": r.words, "sentences_read": r.sentences.len(), "sentences_unparsed": unparsed, "sentences_without_skeleton": empty,
        "description_words": d.words, "available_skeleton_words": d.available_words,
        "sections_readable": readable.len(), "sections_described": described.len(),
        "section_coverage": if readable.is_empty() { 0.0 } else { described.len() as f64 / readable.len() as f64 },
        "sentences_used": sourced, "source_sentences_used": d.sentences.iter().filter_map(|w| w.source).collect::<BTreeSet<_>>().len(),
        "traceable_sentences": traceable, "traceability": if sourced == 0 { 0.0 } else { traceable as f64 / sourced as f64 },
        "traceability_method": "every content word (>=3 letters, not a stop word) of a written sentence occurs in its bound source sentence",
        "verbatim_run_max": runs.last().copied().unwrap_or(0), "verbatim_run_median": runs.get(runs.len() / 2).copied().unwrap_or(0),
        "sentences_with_verbatim_run_ge_12": runs.iter().filter(|&&n| n >= 12).count(),
        "unknown_word_types": r.unknown.len(), "unknown_word_tokens": r.unknown.values().sum::<usize>()
    })
}

/// Longest run of consecutive tokens of `a` that also occurs consecutively in `b`.
pub fn longest_run(a: &[String], b: &[String]) -> usize {
    let mut best = 0;
    let mut prev = vec![0usize; b.len() + 1];
    for x in a {
        let mut cur = vec![0usize; b.len() + 1];
        for (j, y) in b.iter().enumerate() {
            if x == y {
                cur[j + 1] = prev[j] + 1;
                best = best.max(cur[j + 1]);
            }
        }
        prev = cur;
    }
    best
}

/// What the Whale did not understand, most frequent first: unknown terms seen at least twice,
/// then the longest sentences it could not reduce to a skeleton.
pub fn gaps(r: &Reading, terms: usize, sentences: usize) -> (Vec<(String, usize)>, Vec<usize>) {
    // Terms met in at least two sentences; acronyms (2..8 capitals) first, then by frequency.
    let spread = |k: &str| r.sentences.iter().filter(|s| s.unknown.iter().any(|u| u == k)).count();
    let acronym = |k: &str| {
        let f = r.forms.get(k).map(String::as_str).unwrap_or(k);
        let f = f.strip_suffix('s').unwrap_or(f);
        (2..=8).contains(&f.chars().count()) && f.chars().all(|c| c.is_ascii_uppercase())
    };
    let mut t: Vec<(String, usize)> = top(&r.unknown, usize::MAX).into_iter().filter(|(k, _)| spread(k) >= 2).map(|(k, n)| (k.clone(), *n)).collect();
    t.sort_by(|a, b| acronym(&b.0).cmp(&acronym(&a.0)).then(b.1.cmp(&a.1)).then(a.0.cmp(&b.0)));
    t.truncate(terms);
    let mut s: Vec<&Sentence> = r.sentences.iter().filter(|s| !s.admitted || s.skeleton.is_empty()).collect();
    s.sort_by(|a, b| b.text.split_whitespace().count().cmp(&a.text.split_whitespace().count()).then(a.index.cmp(&b.index)));
    (t, s.into_iter().take(sentences).map(|s| s.index).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn runs_and_tokens() {
        let t = |s: &str| tokens(s);
        assert_eq!(longest_run(&t("a b c d"), &t("x b c d y")), 3);
        assert_eq!(longest_run(&t("a"), &t("b")), 0);
        assert_eq!(content("The parser uses UD trees."), ["parser", "uses", "trees"]);
        assert!(!content_line("<!-- с. 1 -->") && !content_line("Джерело: https://x") && content_line("Text."));
        assert!(residue("TP FP FN TP FP FN TP FP FN") && residue("In Proceedings of the 56th Annual Meeting.") && !residue("The parser reads each word once."));
        let o: Vec<String> = ["as", "in", "Nivre", "et", "al", "and", "Zeman", ",", "2018", "and", "UDPipe", "tags"].iter().map(|s| s.to_string()).collect();
        assert!(citation_name(&o, 2) && citation_name(&o, 6) && !citation_name(&o, 10));
    }
}

#[cfg(test)]
mod writer_tests {
    /// The reader's restatements of 2026-11-04: questions stay questions, a fronted subordinate
    /// clause the parse cut off is attached to its main clause, a coordinated base form keeps its
    /// shared subject, «Note that» points at its clause, and a reference list under a content
    /// heading is not read.
    #[test]
    fn questions_subordinates_and_reference_lists() {
        if !crate::learn::tests::model_ok() {
            return;
        }
        let md = "# T\n\n## Method\n\nDo both sides of the edit have the same lemma?\n\nAs the Levenshtein distance only aligns individual tokens however, they also merged all adjacent non-matches in an effort to capture multi-token edits.\n\nTransition-based dependency parsers read words sequentially and build dependency trees incrementally by making series of multiple choice decisions.\n\nNote that this is a representative example of detailed error type performance.\n\nDaniel Andor, Chris Alberti, and Michael Collins. 2016. Globally normalized transition-based neural networks. In Proceedings of ACL-2016. Berlin, Germany, pages 2442–2452.\n";
        let r = super::read(md).unwrap();
        let all: Vec<&str> = r.sentences.iter().flat_map(|s| s.clauses.iter().map(|c| c.text.as_str())).collect();
        assert!(all.contains(&"The paper asks: do both sides of the edit have the same lemma?"), "{all:?}");
        // Red control: the question is never asserted as a statement.
        assert!(!all.iter().any(|c| c.starts_with("Both sides of the edit have")), "{all:?}");
        assert!(all.iter().any(|c| c.contains(", as the Levenshtein distance only aligns individual tokens")), "{all:?}");
        assert!(all.contains(&"Transition-based dependency parsers build dependency trees incrementally by making series of multiple choice decisions."), "{all:?}");
        assert!(all.contains(&"This is a representative example of detailed error type performance."), "{all:?}");
        assert!(!r.sentences.iter().any(|s| s.text.contains("Globally normalized")), "a reference entry was read");
        assert!(super::reference_entry("Daniel Andor and Michael Collins. 2016. Globally normalized networks. In Proceedings of ACL-2016.") && !super::reference_entry("In 2016, the authors reported 91% in their experiments on the treebank."));
    }
}

#[cfg(test)]
mod dev {
    /// Development probe: clause words available per pilot paper (`WHALE_DEV_PAPERS`, paths joined by ':').
    #[test]
    #[ignore]
    fn clause_words() {
        std::env::set_var("WORLD_EN_MODEL", std::env::var("MOVA_OCTOPUS_TEST_MODEL").unwrap());
        for p in std::env::var("WHALE_DEV_PAPERS").unwrap().split(':') {
            let r = super::read(&std::fs::read_to_string(p).unwrap()).unwrap();
            let cl: usize = r.sentences.iter().map(|s| s.clauses.len()).sum();
            let w: usize = r.sentences.iter().flat_map(|s| s.clauses.iter()).map(|c| c.text.split_whitespace().count()).sum();
            let none = r.sentences.iter().filter(|s| s.clauses.is_empty()).count();
            println!("{p}\n  sections {} sentences {} clauses {cl} words {w} sentences-without-clause {none} defs {:?}", r.sections.iter().filter(|s| !s.skipped).count(), r.sentences.len(), r.definitions);
            let with: Vec<&super::Sentence> = r.sentences.iter().filter(|s| !s.clauses.is_empty()).collect();
            let src_w: usize = with.iter().map(|s| s.text.split_whitespace().count()).sum();
            let cl_w: usize = with.iter().flat_map(|s| s.clauses.iter()).map(|c| c.text.split_whitespace().count()).sum();
            let dropped: usize = with.iter().flat_map(|s| s.clauses.iter()).map(|c| c.dropped).sum();
            let prose_no: Vec<&super::Sentence> = r.sentences.iter().filter(|s| s.clauses.is_empty() && s.text.split_whitespace().count() >= 8 && super::alpha_share(&s.text) >= 0.8).collect();
            println!("  restated sentences {} source words {src_w} clause words {cl_w} dropped subtrees {dropped}; unrestated prose {} words {}", with.len(), prose_no.len(), prose_no.iter().map(|s| s.text.split_whitespace().count()).sum::<usize>());
            if std::env::var_os("WHALE_DEV_UNRESTATED").is_some() { for s in &prose_no { println!("    U {}", s.text); } }
            if std::env::var_os("WHALE_DEV_LOSS").is_some() {
                // Source words of restated sentences no clause covers, by the relation of the top uncovered word.
                let a = world::tree::annotator().unwrap();
                let mut by: std::collections::BTreeMap<String, usize> = Default::default();
                for s in &with {
                    let t = world::tree::Tree::parse(a, &s.text);
                    let cov: std::collections::BTreeSet<usize> = s.clauses.iter().flat_map(|c| c.src.iter().copied()).collect();
                    if std::env::var_os("WHALE_DEV_SPANS").is_some() {
                        let mut d = 0i32; let mut line = String::new();
                        for i in 0..t.w.len() { match t.form(i) { "(" | "[" => d += 1, ")" | "]" => d = (d - 1).max(0), _ => {} }
                            if cov.contains(&i) || d > 0 { line.push_str(t.form(i)); } else { line.push_str(&format!("_{}_", t.form(i))); } line.push(' '); }
                        println!("    S {line}");
                        for c in &s.clauses { println!("      = {} ({})", c.text, c.kind); }
                    }
                    let mut depth = 0i32;
                    for i in 0..t.w.len() {
                        match t.form(i) { "(" | "[" => depth += 1, ")" | "]" => depth = (depth - 1).max(0), _ => {} }
                        if depth > 0 || cov.contains(&i) || !t.form(i).chars().any(char::is_alphanumeric) { continue; }
                        let mut top = i;
                        while let Some(hd) = t.head(top) { if cov.contains(&hd) { break; } top = hd; }
                        let key = if t.head(top).is_none() { "ROOT".to_string() } else { format!("{:?}", t.w[top].rel) };
                        if std::env::var_os("WHALE_DEV_TOP").is_some_and(|v| v.to_str() == Some(key.as_str())) && top == i {
                            let hd = t.head(top).unwrap_or(0);
                            println!("    T {key} [{}] head {} ({:?} {:?}) | {}", t.sub(top, &[]).iter().map(|&k| t.form(k)).collect::<Vec<_>>().join(" "), t.form(hd), t.w[hd].rel, t.w[hd].upos, s.text);
                        }
                        *by.entry(key).or_insert(0) += 1;
                    }
                }
                let mut v: Vec<_> = by.into_iter().collect(); v.sort_by(|a, b| b.1.cmp(&a.1));
                println!("  loss total (outside brackets) {}", v.iter().map(|x| x.1).sum::<usize>());
                println!("  loss by relation {:?}", &v[..v.len().min(14)]);
            }
            for s in r.sections.iter() { println!("   [{}] {} {}", if s.skipped {"skip"} else {"read"}, s.title, s.sentences.len()); }
            for s in r.sentences.iter().take(40) { for c in &s.clauses { println!("    - {} ({} v{} d{})", c.text, c.kind, c.verbatim, c.dropped); } if s.clauses.is_empty() { println!("    X {}", s.text); } }
        }
    }
}

#[cfg(test)]
mod filter_tests {
    use super::*;
    /// The content filter, the small-capitals repair and the extraction check, each with a
    /// sentence that must stay (red control against a filter that takes everything).
    #[test]
    fn furniture_repair_and_extraction() {
        assert_eq!(furniture("Miguel Ballesteros, Yoav Goldberg, Chris Dyer, and Noah A."), Some("name_list"));
        assert_eq!(furniture("Eliyahu Kiperwasser and Yoav Goldberg."), Some("name_list"));
        assert_eq!(furniture("Association for Computational Linguistics, Lisbon, Portugal, pages 461–470."), Some("page_furniture"));
        assert_eq!(furniture("Codebase: https://github.com/x/y is open."), Some("page_furniture"));
        assert_eq!(furniture("PRP VBP TO VB PRP DT NN IN DT NNS RB ."), Some("table_row"));
        assert_eq!(furniture("Simple Adv Train 1313 613 60 12 7"), Some("table_row"));
        assert_eq!(furniture("e ti = score(h t , s i ) a t = softmax (e t )"), Some("maths"));
        assert_eq!(furniture("The learning rate η is annealed by a fixed decay rate ρ = 0.75 when performance stops."), Some("maths"));
        for keep in ["The parser reads each word of the sentence once.", "Grammatical Error Correction (GEC) is the task of correcting errors in text.", "ERRANT will merge edits such as [eat → eaten] into one edit.", "Results for each team are shown in Table 9 and Table 10."] {
            assert_eq!(furniture(keep), None, "{keep}");
        }
        assert!(reference_entry("Gabor Angeli, Melvin Jose Johnson Premkumar, and Christopher D. Manning. 2015. Leveraging linguistic structure for open domain information extraction."));
        assert!(!reference_entry("In 2015, the authors reported 91% on the treebank. The parser was fast."));
        let names = names(&[(1, "Miguel Ballesteros, Yoav Goldberg, and Noah A. Smith. 2016. Training with exploration. In Proceedings of EMNLP-2016.".into()), (1, "As shown by Nivre et al. (2016) and Zeman and Popel (2018), treebanks differ.".into())]);
        for n in ["goldberg", "ballesteros", "smith", "nivre", "zeman", "popel"] {
            assert!(names.contains(n), "{n}: {names:?}");
        }
        assert!(!names.contains("treebanks") && !names.contains("training"), "{names:?}");
        // Small capitals: known words, repeated names, a run; «A CNN» stays.
        let fixed = repair("We propose stack-pointer networks (S TACK P TR ). S TACK P TR beats B I AF and B I AF.\nA CNN reads it.\nC UTTING T HROUGH THE N OISE");
        assert!(fixed.contains("(STACK PTR )") && fixed.contains("STACK PTR beats BIAF and BIAF"), "{fixed}");
        assert!(fixed.contains("A CNN reads it") && fixed.contains("CUTTING THROUGH THE NOISE"), "{fixed}");
        // Extraction: a text whose sections have no headings is refused; one with headings is not.
        let body = "The parser reads each word of the sentence once and attaches it to its head word. ".repeat(30);
        let headed = extraction(&format!("# T\n\n## Method\n\n{body}\n"));
        let flat = extraction(&format!("# T\n\n{body}\n\n## References\n\nA. B. 2020. C.\n"));
        let l = ExtractionLimits::default();
        assert!(headed.broken(&l).is_empty(), "{headed:?}");
        assert_eq!(flat.broken(&l), vec!["no_section_headings"], "{flat:?}");
        assert!(extraction("## M\n\nS TACK and P ROB and X YZW and Q RST are here in the text now.\n").spacing_residue_per_1000 > 0.0);
    }
}
