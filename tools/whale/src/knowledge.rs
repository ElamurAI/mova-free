//! A Whale's retained knowledge: what it learned from its own terminal's answers and from the
//! papers it read, kept as typed facts with provenance in its private state, reused in every later
//! round and for every later paper.
//!
//! An answer is never kept as text for writing. Mova re-reads it (`world::tree`) and keeps only
//! facts in its own representation:
//!
//! - `stands_for`: an acronym's expansion, found where the initials of consecutive words spell it;
//! - `is_a`: a copular clause about the term («X is a team name») -> its predicate noun phrase;
//! - `means`: «X means / refers to Y» -> Y's noun phrase;
//! - `does`: «X takes text» -> the verb lemma and its object's noun phrase;
//! - `reads_as` (sentence and section questions): subject, verb and object of an answer clause,
//!   kept only when every one of their words occurs in the source sentence or section, so the
//!   Whale can restate that source in the source's own words.
//!
//! Object phrases have at most `PHRASE_MAX` words. The answer text itself is stored privately
//! (`answers/SHA.txt`) only so that the anti-padding gate can prove it never reaches a
//! description. `asked.jsonl` holds every question key ever sent (answered or not): no question is
//! asked twice, across rounds and papers.
use crate::util::{error, hash, now, write_sync};
use crate::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::OpenOptions;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use world::tree::Tree;

pub const PHRASE_MAX: usize = 5;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Fact {
    /// `stands_for`, `is_a`, `means`, `does` or `reads_as`.
    pub relation: String,
    /// `reads_as`: the subject phrase; otherwise empty.
    #[serde(default)]
    pub subject: String,
    /// `does` / `reads_as`: the verb form; otherwise empty.
    #[serde(default)]
    pub verb: String,
    pub object: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Item {
    pub id: String,
    #[serde(alias = "shark")]
    pub whale: String,
    /// `term`, `sentence` or `section`.
    pub kind: String,
    /// The term, the source sentence text or the section title.
    pub key: String,
    pub paper: String,
    pub facts: Vec<Fact>,
    /// Where it came from: `{"source":"terminal", question, terminal_question, answer_sha256, round}`
    /// or `{"source":"paper", sentence}`.
    pub provenance: Value,
    pub unix: u64,
}

impl Item {
    /// The words an explanation realized from this item may use.
    pub fn basis(&self) -> String {
        let mut b = self.key.clone();
        for f in &self.facts {
            b.push(' ');
            b.push_str(&format!("{} {} {}", f.subject, f.verb, f.object));
            if f.relation == "does" {
                b.push(' ');
                b.push_str(&en::morph::Morph.inflect(&f.verb, en::gram::Tag::VBZ));
            }
        }
        b
    }
}

/// The Whale's knowledge directory: `items.jsonl`, `asked.jsonl`, `answers/`.
pub struct Store {
    pub dir: PathBuf,
    pub items: Vec<Item>,
    /// Question key -> its record (state, question id, paper, round).
    pub asked: BTreeMap<String, Value>,
}

fn append(path: &Path, v: &Value) -> Result<()> {
    let mut f = OpenOptions::new().create(true).append(true).mode(0o600).open(path)?;
    let mut line = v.to_string();
    line.push('\n');
    f.write_all(line.as_bytes())?;
    f.sync_data()?;
    Ok(())
}

fn lines(path: &Path) -> Result<Vec<Value>> {
    if !path.exists() {
        return Ok(vec![]);
    }
    std::fs::read_to_string(path)?.lines().filter(|l| !l.trim().is_empty()).map(|l| serde_json::from_str(l).map_err(|e| error(format!("{}: {e}", path.display())))).collect()
}

/// The key under which a question is remembered as asked.
pub fn question_key(kind: &str, paper: &str, subject: &str) -> String {
    match kind {
        // A term is asked once per Whale, whatever paper it meets it in, under its normalized form.
        "term" => format!("term:{}", norm_term(subject)),
        _ => format!("{kind}:{paper}:{}", hash(subject)),
    }
}

/// A term's normalized form: lower case, a plural acronym or noun in the singular («MRFs» ->
/// «mrf», «treebanks» -> «treebank», «Grammatical» -> «grammatical», «LSTMs»/«lstms» ->
/// «lstm»). Terms are counted, asked, kept and explained under it.
pub fn norm_term(t: &str) -> String {
    let t = t.trim();
    if let Some(core) = t.strip_suffix('s') {
        if core.chars().count() >= 2 && core.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit()) && core.chars().any(|c| c.is_ascii_uppercase()) {
            return core.to_lowercase();
        }
    }
    let lower = t.to_lowercase();
    if lower.chars().count() > 3 && lower.ends_with('s') && !["ss", "us", "is", "ics"].iter().any(|e| lower.ends_with(e)) && lower.chars().all(|c| c.is_alphabetic() || c == '-') {
        let m = en::morph::Morph;
        let l = m.lemmatize(&lower, en::gram::Tag::NNS);
        if !l.is_empty() && l.len() < lower.len() && m.inflect(&l, en::gram::Tag::NNS) == lower {
            return l;
        }
    }
    lower
}

/// Category words that say only what kind of thing a term is, not what it is: a fact whose object
/// is one of them (with at most a determiner and an adjective such as «family») is kept for review,
/// never for writing («Goldberg is a surname», «the label»).
const BARE: [&str; 40] = [
    "name", "surname", "names", "label", "labels", "term", "word", "words", "abbreviation", "acronym", "author", "authors", "person", "researcher",
    "scientist", "symbol", "variable", "value", "thing", "concept", "token", "tag", "placeholder", "reference", "citation", "number", "notation",
    "field", "string", "identifier", "keyword", "entity", "category", "type", "kind", "unit", "code", "abbreviations", "lastname", "noun",
];

/// Whether a fact names only a bare category («the family name», «a surname», «the label»).
pub fn bare(f: &Fact) -> bool {
    if !matches!(f.relation.as_str(), "is_a" | "means") {
        return false;
    }
    let w: Vec<String> = f.object.split_whitespace().map(str::to_lowercase).collect();
    let content: Vec<&String> = w.iter().filter(|x| !matches!(x.as_str(), "a" | "an" | "the" | "this" | "that" | "its" | "their" | "his" | "her" | "one")).collect();
    match content.as_slice() {
        [x] => BARE.contains(&x.as_str()),
        [m, x] => BARE.contains(&x.as_str()) && matches!(m.as_str(), "family" | "last" | "first" | "given" | "proper" | "personal" | "author" | "person" | "short" | "special" | "technical" | "common"),
        _ => false,
    }
}

/// Split a batched answer into its numbered parts («1. …», «2) …», «**3.** …»): part n is the
/// text after the n-th number up to the next. A single question takes the whole answer when it
/// is not numbered.
pub fn split_answer(answer: &str, n: usize) -> Vec<String> {
    let mut parts = vec![String::new(); n];
    let mut cur: Option<usize> = None;
    for line in answer.lines() {
        let l = line.trim_start().trim_start_matches(['*', '#', ' ']);
        let digits: String = l.chars().take_while(|c| c.is_ascii_digit()).collect();
        let rest = &l[digits.len()..];
        let numbered = !digits.is_empty() && (rest.starts_with('.') || rest.starts_with(')') || rest.starts_with(':'));
        if numbered {
            if let Ok(k) = digits.parse::<usize>() {
                if (1..=n).contains(&k) {
                    cur = Some(k - 1);
                    let body = rest[1..].trim_start_matches(['*', ' ']);
                    parts[k - 1].push_str(body);
                    parts[k - 1].push(' ');
                    continue;
                }
            }
        }
        if let Some(k) = cur {
            parts[k].push_str(line.trim());
            parts[k].push(' ');
        }
    }
    if n == 1 && parts[0].trim().is_empty() {
        parts[0] = answer.to_string();
    }
    parts.into_iter().map(|p| p.trim().to_string()).collect()
}

/// The typed restatement of a sentence answer: «In the paper's words: X» -> X (the frame is the
/// question's, not the answer's content).
pub fn paper_words(answer: &str) -> String {
    let mut out = String::new();
    for s in world::quant::sentences(answer) {
        let t = s.trim();
        let body = ["In the paper's words:", "In the paper’s words:", "In the paper's words,", "In the paper’s words,"].iter().find_map(|p| t.strip_prefix(p)).unwrap_or(t);
        out.push_str(body.trim());
        out.push(' ');
    }
    out.trim().to_string()
}

impl Store {
    pub fn open(dir: &Path) -> Result<Self> {
        crate::util::private_dir(dir)?;
        std::fs::create_dir_all(dir.join("answers"))?;
        let items = lines(&dir.join("items.jsonl"))?.into_iter().map(serde_json::from_value).collect::<std::result::Result<Vec<Item>, _>>()?;
        let mut asked = BTreeMap::new();
        for v in lines(&dir.join("asked.jsonl"))? {
            if let Some(k) = v["key"].as_str() {
                asked.insert(k.to_string(), v.clone());
                // A term asked under an earlier, unnormalized key counts under its normal form.
                if let Some(t) = k.strip_prefix("term:") {
                    asked.entry(format!("term:{}", norm_term(t))).or_insert_with(|| v.clone());
                }
            }
        }
        Ok(Self { dir: dir.to_path_buf(), items, asked })
    }
    pub fn retain(&mut self, item: Item) -> Result<()> {
        append(&self.dir.join("items.jsonl"), &serde_json::to_value(&item)?)?;
        self.items.push(item);
        Ok(())
    }
    pub fn mark_asked(&mut self, key: &str, record: Value) -> Result<()> {
        let mut v = record;
        v["key"] = json!(key);
        v["unix"] = json!(now());
        append(&self.dir.join("asked.jsonl"), &v)?;
        self.asked.insert(key.to_string(), v);
        Ok(())
    }
    /// Record a fact flagged for review (`review.jsonl`), not kept as knowledge.
    pub fn review(&self, v: &Value) -> Result<()> {
        let mut v = v.clone();
        v["unix"] = json!(now());
        append(&self.dir.join("review.jsonl"), &v)
    }
    /// Keep an answer's text privately (for the gate that keeps it out of descriptions).
    pub fn keep_answer(&self, text: &str) -> Result<String> {
        let h = hash(text);
        let p = self.dir.join("answers").join(format!("{h}.txt"));
        if !p.exists() {
            write_sync(&p, text.as_bytes())?;
        }
        Ok(h)
    }
    /// Every answer text the Whale has received.
    pub fn answers(&self) -> Result<Vec<String>> {
        let mut v: Vec<PathBuf> = std::fs::read_dir(self.dir.join("answers"))?.filter_map(|e| e.ok().map(|e| e.path())).collect();
        v.sort();
        v.iter().map(|p| Ok(std::fs::read_to_string(p)?)).collect()
    }
    /// Terms the Whale has facts for (from any paper), and the item ids.
    pub fn known_terms(&self) -> BTreeSet<String> {
        self.items.iter().filter(|i| i.kind == "term" && i.facts.iter().any(|f| !bare(f))).map(|i| norm_term(&i.key)).collect()
    }
    pub fn next_id(&self) -> String {
        format!("k{:05}", self.items.len() + 1)
    }
}

fn is_acronym(t: &str) -> bool {
    let core = t.trim_end_matches('s');
    (2..=8).contains(&core.chars().count()) && core.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit()) && core.chars().any(|c| c.is_ascii_uppercase())
}

/// The expansion of an acronym among `words`: consecutive words whose initials spell it (small
/// function words may sit between them).
pub fn expansion(acronym: &str, words: &[String]) -> Option<String> {
    const SMALL: [&str; 7] = ["of", "and", "for", "the", "to", "in", "on"];
    let letters: Vec<char> = acronym.trim_end_matches('s').chars().filter(|c| c.is_ascii_alphabetic()).map(|c| c.to_ascii_lowercase()).collect();
    if letters.len() < 2 {
        return None;
    }
    for start in 0..words.len() {
        let mut k = 0;
        let mut out: Vec<&str> = vec![];
        let mut j = start;
        while j < words.len() && k < letters.len() {
            let w = words[j].as_str();
            if w.chars().next().map(|c| c.to_ascii_lowercase()) == Some(letters[k]) && w.chars().all(|c| c.is_alphabetic() || c == '-') && w.to_uppercase() != acronym {
                out.push(w);
                k += 1;
            } else if !out.is_empty() && SMALL.contains(&w.to_lowercase().as_str()) {
                out.push(w);
            } else {
                break;
            }
            j += 1;
        }
        if k == letters.len() && out.len() <= letters.len() * 2 {
            return Some(out.join(" "));
        }
    }
    None
}

/// A short noun phrase of a node: determiner, possessive, adjectives, compounds and the head,
/// in order; None when longer than `PHRASE_MAX` or not nominal.
fn noun_phrase(t: &Tree, i: usize, orig: &[String]) -> Option<String> {
    use en::gram::{Rel, UPos};
    if !matches!(t.w[i].upos, UPos::NOUN | UPos::PROPN | UPos::PRON | UPos::NUM) {
        return None;
    }
    let mut ix: Vec<usize> = t.kids(i).iter().copied().filter(|&k| matches!(t.base(k), Rel::Det | Rel::Amod | Rel::Compound | Rel::Nummod) || t.w[k].rel == Rel::NmodPoss).collect();
    ix.push(i);
    ix.sort();
    if ix.len() > PHRASE_MAX {
        return None;
    }
    // One «of/for …» modifier when the phrase still fits: «the percentage of tokens».
    if let Some(&m) = t.kids(i).iter().find(|&&k| t.w[k].rel == Rel::Nmod) {
        let sub: Vec<usize> = t.sub(m, &[]).into_iter().filter(|&k| t.w[k].upos != UPos::PUNCT).collect();
        if ix.len() + sub.len() <= PHRASE_MAX && sub.iter().all(|&k| k > i) {
            ix.extend(sub);
            ix.sort();
        }
    }
    Some(ix.iter().map(|&k| orig[k].clone()).collect::<Vec<_>>().join(" "))
}

fn mentions(t: &Tree, i: usize, orig: &[String], term: &str) -> bool {
    let sub = t.sub(i, &[]);
    let key = norm_term(term);
    sub.iter().any(|&k| orig[k].eq_ignore_ascii_case(term) || norm_term(&orig[k]) == key) || (sub.len() == 1 && matches!(t.lemma(i), "it" | "this"))
}

/// Facts about `term` in an answer, read by Mova.
pub fn term_facts(a: &en::annotate::Annotator, term: &str, answer: &str) -> Vec<Fact> {
    use en::gram::{Rel, UPos};
    let mut out: Vec<Fact> = vec![];
    let all: Vec<String> = a.tokenize(answer).into_iter().map(|t| t.form).collect();
    if is_acronym(term) {
        if let Some(e) = expansion(term, &all) {
            out.push(Fact { relation: "stands_for".into(), subject: String::new(), verb: String::new(), object: e });
        }
    }
    for (n, s) in world::quant::sentences(answer).into_iter().enumerate().take(4) {
        let orig: Vec<String> = a.tokenize(&s).into_iter().map(|t| t.form).collect();
        let t = Tree::parse(a, &s);
        let Some(root) = t.root() else { continue };
        if t.w.len() != orig.len() {
            continue;
        }
        let Some(subj) = t.kids(root).iter().copied().find(|&c| matches!(t.base(c), Rel::Nsubj)) else { continue };
        if !mentions(&t, subj, &orig, term) || (n > 1 && !orig.iter().any(|w| w.eq_ignore_ascii_case(term) || norm_term(w) == norm_term(term))) {
            continue;
        }
        let fact = if t.kids(root).iter().any(|&c| t.w[c].rel == Rel::Cop) {
            noun_phrase(&t, root, &orig).map(|o| Fact { relation: "is_a".into(), subject: String::new(), verb: String::new(), object: o })
        } else if t.w[root].upos == UPos::VERB {
            let obj = t.kids(root).iter().copied().find(|&c| matches!(t.base(c), Rel::Obj | Rel::Obl)).and_then(|o| noun_phrase(&t, o, &orig));
            match (t.lemma(root), obj) {
                ("mean" | "refer" | "denote" | "stand", Some(o)) => Some(Fact { relation: "means".into(), subject: String::new(), verb: String::new(), object: o }),
                (_, Some(o)) if t.kids_of(root, Rel::Obj).first().is_some() => Some(Fact { relation: "does".into(), subject: String::new(), verb: t.lemma(root).to_string(), object: o }),
                _ => None,
            }
        } else {
            None
        };
        if let Some(f) = fact {
            // «GEC means grammatical error correction» repeats the expansion: not a new fact.
            let repeats = out.iter().any(|x| x.relation == "stands_for" && (x.object.to_lowercase().starts_with(&f.object.to_lowercase()) || f.object.to_lowercase().starts_with(&x.object.to_lowercase())));
            if !out.iter().any(|x| x.relation == f.relation) && !repeats {
                out.push(f);
            }
        }
    }
    out.truncate(3);
    out
}

fn tokens(s: &str) -> Vec<String> {
    s.split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()).map(str::to_lowercase).collect()
}

/// The version of the answer readers. A Whale re-reads its kept answers when it changes
/// (`learn::reread`): the same answers, read better, never asked again.
pub const READER: &str = "reading-facts-2";

/// `reads_as` facts of an answer about a source sentence or section: every clause of every answer
/// sentence (main, coordinated, complement and adverbial clauses) whose subject, verb group and
/// object or prepositional complement are all words of the source. The verb group keeps its
/// auxiliaries and negation («can not be selected»), so modality and polarity stay as the answer
/// read them; a coordinated verb without a subject shares its governor's. Pronoun subjects and
/// the answer talking about the text itself («This sentence …») are not facts. At most four.
pub fn reading_facts(a: &en::annotate::Annotator, source: &str, answer: &str) -> Vec<Fact> {
    use en::gram::{Rel, UPos};
    let have: BTreeSet<String> = tokens(source).into_iter().collect();
    let inside = |p: &str| !p.is_empty() && tokens(p).iter().all(|w| have.contains(w));
    let mut out: Vec<Fact> = vec![];
    for s in world::quant::sentences(answer) {
        let orig: Vec<String> = a.tokenize(&s).into_iter().map(|t| t.form).collect();
        let t = Tree::parse(a, &s);
        if t.root().is_none() || t.w.len() != orig.len() {
            continue;
        }
        for v in 0..t.w.len() {
            let cop = t.kids(v).iter().copied().find(|&c| t.w[c].rel == Rel::Cop);
            if t.w[v].upos != UPos::VERB && cop.is_none() {
                continue;
            }
            if !(t.head(v).is_none() || matches!(t.base(v), Rel::Conj | Rel::Ccomp | Rel::Advcl | Rel::Parataxis)) {
                continue;
            }
            // The subject: its own, else (a coordinated verb) its governor's.
            let mut g = v;
            let subj = loop {
                if let Some(c) = t.kids(g).iter().copied().find(|&c| matches!(t.base(c), Rel::Nsubj)) {
                    break Some(c);
                }
                match t.head(g) {
                    Some(h) if t.w[g].rel == Rel::Conj => g = h,
                    _ => break None,
                }
            };
            let Some(subj) = subj.and_then(|c| noun_phrase(&t, c, &orig)) else { continue };
            if matches!(subj.to_lowercase().as_str(), "it" | "this" | "they" | "these" | "that" | "which" | "who" | "he" | "she" | "we" | "the sentence" | "this sentence" | "the passage" | "this passage" | "the paper" | "this paper" | "the text" | "the claim" | "this part" | "the part") {
                continue;
            }
            // The verb group in source order: auxiliaries, negation, the verb (or the copula).
            let mut group: Vec<usize> = t.kids(v).iter().copied().filter(|&c| t.base(c) == Rel::Aux || matches!(t.lemma(c), "not" | "never") || t.form(c) == "n't").filter(|&c| c < v).collect();
            let (verb_ix, object) = match cop {
                Some(c) => {
                    group.retain(|&x| x != c);
                    group.push(c);
                    let pred = if t.w[v].upos == UPos::ADJ { (t.kids(v).iter().all(|&k| matches!(t.base(k), Rel::Cop | Rel::Nsubj | Rel::Punct | Rel::Advmod | Rel::Aux))).then(|| orig[v].clone()) } else { noun_phrase(&t, v, &orig) };
                    (c, pred)
                }
                None => {
                    group.push(v);
                    let obj = t.kids(v).iter().copied().find(|&c| t.base(c) == Rel::Obj).and_then(|c| noun_phrase(&t, c, &orig)).or_else(|| {
                        t.kids(v).iter().copied().find(|&c| t.base(c) == Rel::Obl && c > v).and_then(|c| {
                            let case = t.kids_of(c, Rel::Case).first().map(|&k| orig[k].clone())?;
                            noun_phrase(&t, c, &orig).map(|np| format!("{case} {np}"))
                        })
                    });
                    (v, obj)
                }
            };
            let _ = verb_ix;
            group.sort();
            group.dedup();
            let verb = group.iter().map(|&k| orig[k].clone()).collect::<Vec<_>>().join(" ");
            let Some(object) = object else { continue };
            if inside(&subj) && inside(&verb) && inside(&object) {
                let f = Fact { relation: "reads_as".into(), subject: subj, verb, object };
                if !out.contains(&f) {
                    out.push(f);
                }
            }
        }
    }
    out.truncate(4);
    out
}

/// An explanation of a term in the Whale's own words, realized from its facts.
pub fn explain(term: &str, facts: &[Fact]) -> Vec<String> {
    let mut out = vec![];
    for f in facts {
        let s = match f.relation.as_str() {
            "stands_for" => format!("«{term}» is short for {}", f.object),
            "is_a" => format!("I understand «{term}» as {}", f.object),
            "means" => format!("I take «{term}» to mean {}", f.object),
            "does" => format!("As I understand it, «{term}» {} {}", en::morph::Morph.inflect(&f.verb, en::gram::Tag::VBZ), f.object),
            _ => continue,
        };
        out.push(mova_octopus::realize::sentence(&s));
    }
    out
}

/// A source restated from a `reads_as` fact.
pub fn restate(f: &Fact) -> Option<String> {
    (f.relation == "reads_as").then(|| mova_octopus::realize::sentence(&format!("{} {} {}", f.subject, f.verb, f.object)))
}

/// Words the explanation templates add (the gate's frame vocabulary for explanations).
pub const TEMPLATE: [&str; 9] = ["is", "short", "for", "understand", "as", "take", "to", "mean", "it"];

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn expansions_and_keys() {
        let w = |s: &str| s.split(' ').map(String::from).collect::<Vec<_>>();
        assert_eq!(expansion("GEC", &w("GEC means Grammatical Error Correction : the task")).as_deref(), Some("Grammatical Error Correction"));
        assert_eq!(expansion("UAS", &w("the unlabeled attachment score of a parser")).as_deref(), Some("unlabeled attachment score"));
        assert_eq!(expansion("FCE", &w("First Certificate in English exam")).as_deref(), Some("First Certificate in English"));
        assert_eq!(expansion("GEC", &w("good evening")), None);
        assert_eq!(question_key("term", "p1", "GEC"), question_key("term", "p2", "GEC"), "a term is asked once per Whale");
        assert_ne!(question_key("sentence", "p1", "S."), question_key("sentence", "p2", "S."));
        assert_eq!(explain("GEC", &[Fact { relation: "stands_for".into(), subject: "".into(), verb: "".into(), object: "grammatical error correction".into() }]), ["«GEC» is short for grammatical error correction."]);
    }
}
#[cfg(test)]
mod reading_tests {
    /// Every clause of an answer is read, its verb group keeps auxiliaries and negation, and only
    /// clauses made of the source's own words become facts (red control: a paraphrase).
    #[test]
    fn answer_clauses_keep_modality_and_the_source_words() {
        if !crate::learn::tests::model_ok() {
            return;
        }
        let a = world::tree::annotator().unwrap();
        let src = "The decoder can not select a child twice, and the parser removes the child from the list of available words.";
        let ans = "The decoder can not select a child twice. The parser removes the child from the list. Graph parsers search all possible trees.";
        let f = super::reading_facts(a, src, ans);
        let said: Vec<String> = f.iter().filter_map(super::restate).collect();
        assert!(said.iter().any(|x| x.starts_with("The decoder can not select a child")), "{said:?}");
        assert!(said.iter().any(|x| x.starts_with("The parser removes the child")), "{said:?}");
        assert!(!said.iter().any(|x| x.contains("search")), "a paraphrase with words outside the source became a fact: {said:?}");
        assert!(super::reading_facts(a, src, "Graph parsers search all possible trees.").is_empty());
    }
}
#[cfg(test)]
mod norm_tests {
    use super::*;
    #[test]
    fn terms_bare_facts_and_batched_answers() {
        for (a, b) in [("MRFs", "mrf"), ("MRF", "mrf"), ("treebanks", "treebank"), ("Treebank", "treebank"), ("Grammatical", "grammatical"), ("lstms", "lstm"), ("LSTM", "lstm"), ("corpus", "corpus"), ("analysis", "analysis")] {
            assert_eq!(norm_term(a), b, "{a}");
        }
        assert_eq!(question_key("term", "p", "MRFs"), question_key("term", "q", "mrf"));
        let f = |rel: &str, o: &str| Fact { relation: rel.into(), subject: String::new(), verb: String::new(), object: o.into() };
        assert!(bare(&f("is_a", "the family name")) && bare(&f("is_a", "a surname")) && bare(&f("means", "the label")));
        assert!(!bare(&f("is_a", "a type of neural network")) && !bare(&f("is_a", "the percentage of words")) && !bare(&f("stands_for", "name")));
        let parts = split_answer("1. GEC stands for grammatical error correction.\n2) In the paper's words: the parser reads each word.\nIt is greedy.\n**3.** QZW is a record.", 3);
        assert_eq!(parts[0], "GEC stands for grammatical error correction.");
        assert_eq!(parts[1], "In the paper's words: the parser reads each word. It is greedy.");
        assert_eq!(parts[2], "QZW is a record.");
        assert_eq!(split_answer("No numbers here.", 1), vec!["No numbers here."]);
        assert_eq!(split_answer("1. Only one.", 2)[1], "");
        assert_eq!(paper_words("In the paper's words: the parser reads each word. It stores heads."), "the parser reads each word. It stores heads.");
    }
}
