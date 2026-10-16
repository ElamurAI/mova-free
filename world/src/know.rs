//! Knowledge: the hands for general statements.
//!
//! The story reader knows events — someone moved, gave, took. The worlds speak otherwise: in definitions («A prime is a
//! natural number…»), properties, abilities («Fish cannot live out of water») and habits («People usually eat three
//! meals a day»). This module reads such sentences along their dependency trees into facts — is a, is, has, can,
//! cannot, does (with how often), used for, made of, lives in, how many in — each with its sentence as the source, and
//! answers simple questions from them. Deterministic: ordered maps, the facts in sentence order.
//!
//!   world know "<sentence>"                 — the facts of one sentence
//!   world know --ask "<question>" [world]   — an answer from the facts of the central worlds (or of one)

use std::collections::BTreeMap;

use anyhow::Result;

use crate::tree::{Rel, Tree, UPos};

/// The rules of the hands — what the small ones can change and learn (`Rules of my knowledge` in the brain). The
/// defaults are what the skeleton was written with.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KnowRules {
    /// auxiliaries that make a clause a supposition, not a fact
    pub suppose: Vec<String>,
    /// subordinators that make a clause a condition, not a fact
    pub conditions: Vec<String>,
    /// «has / is N units» also reads as a count
    pub measures: bool,
    /// a subject inside quotation marks is an example of language
    pub quotes_are_examples: bool,
    /// a fact with more words than asked answers only «under a condition»
    pub conditional_answers: bool,
    /// definitions «is a/an» before descriptions «is the …»
    pub definitions_first: bool,
    /// what a verb (with its preposition, if any) means beyond «does»: «contain» → has, «consist of» → made of
    /// (verb, preposition, link, kind): «measure» → used for, only when the subject is a «device» (kind empty: always)
    pub verb_links: Vec<(String, String, Link, String)>,
    /// learned weights of the features of an answer (`RANK_FEATS`); none — the hand-set order
    pub rank_weights: Option<Vec<i64>>,
}

/// The features by which an answer is chosen among the facts that could give it.
pub const RANK_FEATS: [&str; 13] = ["is a", "used for or made of", "is", "has or counts", "does or can", "a description, not a definition", "the sentence does not open with the subject", "a short sentence", "negated", "the sentence's length in tens of characters", "the subject is exactly the one asked", "from a world of everyday things and living beings", "from a formal world (logic, mathematics, code, language, Mova herself)"];

/// The hand-set order as weights (the start of learning): the kind of link first, then definitions, then the
/// sentence that opens with the subject, then the shorter.
pub const HAND_WEIGHTS: [i64; 13] = [30, 20, 10, 0, 0, -4, -2, 0, 0, -1, 0, 0, 0];

/// The world each sentence of the central manuals comes from.
pub fn world_of(sentence: &str) -> &'static str {
    static M: std::sync::OnceLock<BTreeMap<String, &'static str>> = std::sync::OnceLock::new();
    M.get_or_init(|| crate::state::CENTRAL_MANUALS.iter().flat_map(|(id, _, m)| crate::nerves::sentences_of(m).into_iter().map(move |x| (x, *id))).collect()).get(sentence).copied().unwrap_or("")
}

impl Default for KnowRules {
    fn default() -> Self {
        KnowRules { suppose: ["would", "might", "should", "may"].map(String::from).to_vec(), conditions: ["if", "unless"].map(String::from).to_vec(), measures: true, quotes_are_examples: true, conditional_answers: true, definitions_first: true, verb_links: Vec::new(), rank_weights: None }
    }
}

/// Words the hands may learn to take into a list (tried as ideas).
const SUPPOSE_POOL: [&str; 6] = ["would", "might", "should", "may", "could", "must"];
const CONDITION_POOL: [&str; 6] = ["if", "unless", "when", "whenever", "until", "although"];

impl KnowRules {
    /// The rules as English sentences, one a line (the brain section).
    pub fn to_english(&self) -> String {
        let yn = |b: bool| if b { "yes" } else { "no" };
        format!(
            "- A supposition is marked by: {}.\n- A condition is marked by: {}.\n- A measure like «an hour has sixty minutes» is also a count: {}.\n- A subject in quotation marks is an example of language: {}.\n- A fact with more words than asked answers only under a condition: {}.\n- A definition comes before a description: {}.",
            self.suppose.join(", "), self.conditions.join(", "), yn(self.measures), yn(self.quotes_are_examples), yn(self.conditional_answers), yn(self.definitions_first)
        ) + &self.verb_links.iter().map(|(v, c, l, k)| format!("\n- The verb «{v}»{} means: {}{}.", if c.is_empty() { String::new() } else { format!(" with «{c}»") }, link_name(*l), if k.is_empty() { String::new() } else { format!(", when the subject is a «{k}»") })).collect::<String>()
            + &self.rank_weights.as_ref().map(|w| format!("\n- The weights of my answers: {}.", w.iter().map(i64::to_string).collect::<Vec<_>>().join(", "))).unwrap_or_default()
    }

    /// The rules read back from their English (missing lines keep the defaults).
    pub fn from_english(text: &str) -> KnowRules {
        let mut r = KnowRules::default();
        for l in text.lines().map(|l| l.trim().trim_start_matches("- ").trim_end_matches('.')) {
            let list = |x: &str| x.split(", ").map(str::trim).filter(|w| !w.is_empty()).map(String::from).collect::<Vec<_>>();
            let yes = |x: &str| x.trim() == "yes";
            if let Some(x) = l.strip_prefix("A supposition is marked by: ") { r.suppose = list(x) }
            else if let Some(x) = l.strip_prefix("A condition is marked by: ") { r.conditions = list(x) }
            else if let Some((_, x)) = l.split_once("is also a count: ") { r.measures = yes(x) }
            else if let Some(x) = l.strip_prefix("A subject in quotation marks is an example of language: ") { r.quotes_are_examples = yes(x) }
            else if let Some(x) = l.strip_prefix("A fact with more words than asked answers only under a condition: ") { r.conditional_answers = yes(x) }
            else if let Some(x) = l.strip_prefix("A definition comes before a description: ") { r.definitions_first = yes(x) }
            else if let Some(x) = l.strip_prefix("The weights of my answers: ") { r.rank_weights = Some(x.split(", ").filter_map(|n| n.trim().parse().ok()).collect()) }
            else if let Some(x) = l.strip_prefix("The verb «") {
                // «contain» means: has   |   «consist» with «of» means: made of
                let (v, rest) = x.split_once('»').unwrap_or(("", ""));
                let c = rest.strip_prefix(" with «").and_then(|r| r.split_once('»')).map(|x| x.0).unwrap_or("");
                let meaning = rest.split_once("means: ").map(|x| x.1.trim()).unwrap_or("");
                let (name, kind) = meaning.split_once(", when the subject is a «").map(|(n, k)| (n, k.trim_end_matches('»'))).unwrap_or((meaning, ""));
                if let Some(l) = link_of(name.trim()) {
                    r.verb_links.push((v.to_string(), c.to_string(), l, kind.to_string()));
                }
            }
        }
        r
    }

    /// Ideas drawn from the facts themselves: the most frequent verbs (with their preposition) still read as «does»,
    /// each offered every learnable meaning; a verb already given a meaning is offered to lose it.
    pub fn verb_ideas(&self, k: &Knowledge, top: usize) -> Vec<(String, KnowRules)> {
        let mut n: BTreeMap<(String, String), usize> = BTreeMap::new();
        for f in k.by_subj.values().flatten().filter(|f| f.link == Link::Does) {
            let ws: Vec<&str> = f.obj.split_whitespace().collect();
            if ws.len() < 2 {
                continue;
            }
            let prep = if ["in", "of", "for", "to", "on", "at", "as", "with", "from", "into"].contains(&ws[1]) { ws[1] } else { "" };
            *n.entry((ws[0].to_string(), prep.to_string())).or_default() += 1;
        }
        let mut v: Vec<((String, String), usize)> = n.into_iter().collect();
        v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        let mut out = Vec::new();
        for ((verb, prep), _) in v.into_iter().take(top) {
            if self.verb_links.iter().any(|(lv, lc, _, _)| *lv == verb && *lc == prep) {
                continue;
            }
            for l in LEARNABLE {
                let mut r = self.clone();
                r.verb_links.push((verb.clone(), prep.clone(), l, String::new()));
                out.push((format!("the verb «{verb}»{} means: {}", if prep.is_empty() { String::new() } else { format!(" with «{prep}»") }, link_name(l)), r));
            }
        }
        for (i, (verb, prep, l, kind)) in self.verb_links.iter().enumerate() {
            let mut r = self.clone();
            r.verb_links.remove(i);
            out.push((format!("the verb «{verb}»{} no longer means: {}", if prep.is_empty() { String::new() } else { format!(" with «{prep}»") }, link_name(*l)), r));
            // narrowing instead of dropping: the meaning only for the kinds of things that most often do the verb
            if kind.is_empty() {
                for k in kinds_doing(k, verb, 3) {
                    let mut r = self.clone();
                    r.verb_links[i].3 = k.clone();
                    out.push((format!("the verb «{verb}» means: {} only when the subject is a «{k}»", link_name(*l)), r));
                }
            }
        }
        out
    }

    /// The ideas: every rule changed by one step, in a fixed order — (what changes, the new rules).
    pub fn ideas(&self) -> Vec<(String, KnowRules)> {
        let mut out = Vec::new();
        let flags: [(&str, fn(&mut KnowRules) -> &mut bool); 4] = [
            ("measures as counts", |r| &mut r.measures),
            ("quoted subjects as examples", |r| &mut r.quotes_are_examples),
            ("conditional answers", |r| &mut r.conditional_answers),
            ("definitions first", |r| &mut r.definitions_first),
        ];
        for (name, f) in flags {
            let mut r = self.clone();
            let v = f(&mut r);
            *v = !*v;
            out.push((format!("{} {name}", if *v { "turn on" } else { "turn off" }), r));
        }
        for (pool, list, what) in [(&SUPPOSE_POOL[..], &self.suppose, "supposition"), (&CONDITION_POOL[..], &self.conditions, "condition")] {
            for w in pool {
                let mut r = self.clone();
                let l = if what == "supposition" { &mut r.suppose } else { &mut r.conditions };
                if list.iter().any(|x| x == w) {
                    l.retain(|x| x != w);
                    out.push((format!("«{w}» no longer marks a {what}"), r));
                } else {
                    l.push(w.to_string());
                    out.push((format!("«{w}» marks a {what}"), r));
                }
            }
        }
        out
    }
}

/// The name of a link in the rules' English.
pub fn link_name(l: Link) -> &'static str {
    match l {
        Link::IsA => "is a",
        Link::Is => "is",
        Link::Has => "has",
        Link::Can => "can",
        Link::Does => "does",
        Link::UsedFor => "used for",
        Link::MadeOf => "made of",
        Link::LivesIn => "lives in",
        Link::CountIn => "count in",
    }
}

fn link_of(name: &str) -> Option<Link> {
    [Link::IsA, Link::Is, Link::Has, Link::Can, Link::Does, Link::UsedFor, Link::MadeOf, Link::LivesIn, Link::CountIn].into_iter().find(|l| link_name(*l) == name)
}

/// The links a verb may be learned to mean.
const LEARNABLE: [Link; 5] = [Link::Has, Link::IsA, Link::MadeOf, Link::UsedFor, Link::LivesIn];

/// How a fact relates its subject to its object.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Link {
    IsA,
    Is,
    Has,
    Can,
    Does,
    UsedFor,
    MadeOf,
    LivesIn,
    /// «there are 60 minutes in an hour»: the object holds this many of the subject
    CountIn,
}

/// How often: a habit is not a law.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Often {
    Always,
    Generic,
    Usually,
    Often,
    Sometimes,
    Rarely,
    Never,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fact {
    pub subj: String,
    pub link: Link,
    pub obj: String,
    pub negated: bool,
    pub often: Often,
    /// the number of «there are N …»
    pub count: Option<String>,
    pub source: String,
}

impl Fact {
    pub fn show(&self) -> String {
        let not = if self.negated { "not " } else { "" };
        let often = match self.often {
            Often::Generic => String::new(),
            o => format!(" [{}]", format!("{o:?}").to_lowercase()),
        };
        match self.link {
            Link::CountIn => format!("{} {} in {}", self.count.as_deref().unwrap_or("some"), self.subj, self.obj),
            l => format!("{} {not}{} {}{often}", self.subj, format!("{l:?}").to_lowercase().replace("isa", "is a").replace("usedfor", "used for").replace("madeof", "made of").replace("livesin", "lives in"), self.obj),
        }
    }
}

const FREQ: [(&str, Often); 12] = [
    ("always", Often::Always), ("usually", Often::Usually), ("normally", Often::Usually), ("typically", Often::Usually),
    ("generally", Often::Usually), ("mostly", Often::Usually), ("often", Often::Often), ("frequently", Often::Often),
    ("sometimes", Often::Sometimes), ("occasionally", Often::Sometimes), ("rarely", Often::Rarely), ("seldom", Often::Rarely),
];

/// A noun phrase by its head: the head lemma with its compounds and adjectives before it («leap year», «solar day»).
fn phrase(t: &Tree, h: usize) -> String {
    let mut ids: Vec<usize> = t.kids(h).iter().copied().filter(|&k| k < h && matches!(t.base(k), Rel::Compound | Rel::Amod | Rel::Nummod | Rel::Flat) && matches!(t.upos(k), UPos::NOUN | UPos::ADJ | UPos::PROPN | UPos::NUM)).collect();
    ids.push(h);
    ids.sort_unstable();
    let mut words: Vec<String> = Vec::new();
    for &i in &ids {
        if i == h {
            // the head in its plain singular, also for words the parser does not know («zorps» → «zorp»)
            let l = t.lemma(i);
            words.push(if l == t.form(i) { crate::nerves::norm(l).unwrap_or_else(|| l.to_string()) } else { l.to_string() });
        } else if t.upos(i) == UPos::NUM {
            // a number with its hyphenated parts («twenty - four» → «twenty-four»)
            let sub = t.sub(i, &[]);
            words.push(sub.iter().map(|&k| t.form(k)).collect::<Vec<_>>().join(" ").replace(" - ", "-"));
        } else {
            words.push(t.form(i).to_string());
        }
    }
    words.join(" ")
}

/// A head and the heads coordinated with it («cat food, meat and fish»).
fn with_conj(t: &Tree, h: usize) -> Vec<usize> {
    let mut v = vec![h];
    v.extend(t.kids_of(h, Rel::Conj).into_iter().filter(|&c| matches!(t.upos(c), UPos::NOUN | UPos::PROPN)));
    v
}

/// The objects of a predicate: its direct objects (with their conjuncts), else its first oblique with its preposition.
fn objects(t: &Tree, v: usize) -> Vec<(String, Option<String>)> {
    if let Some(&o) = t.kids_of(v, Rel::Obj).first() {
        return with_conj(t, o).into_iter().map(|x| (phrase(t, x), None)).collect();
    }
    object(t, v).into_iter().collect()
}

/// The object of a predicate: its direct object, else its first oblique with the preposition named.
fn object(t: &Tree, v: usize) -> Option<(String, Option<String>)> {
    if let Some(&o) = t.kids_of(v, Rel::Obj).first() {
        return Some((phrase(t, o), None));
    }
    for &o in t.kids(v).iter().filter(|&&k| t.base(k) == Rel::Obl) {
        let case = t.kids_of(o, Rel::Case).first().map(|&c| t.lemma(c).to_string());
        return Some((phrase(t, o), case));
    }
    t.kids_of(v, Rel::Xcomp).first().map(|&x| (t.lemma(x).to_string(), Some("to".into())))
}

/// A word inside quotation marks: an example of language, not a thing the sentence is about.
fn quoted(t: &Tree, i: usize) -> bool {
    let q = |k: usize| matches!(t.form(k), "\"" | "“" | "”" | "«" | "»" | "'" | "``" | "''");
    (0..i).filter(|&k| q(k)).count() % 2 == 1
}

fn negated(t: &Tree, v: usize) -> bool {
    t.kids(v).iter().any(|&k| matches!(t.lemma(k), "not" | "n't" | "never" | "no") && matches!(t.base(k), Rel::Advmod | Rel::Det | Rel::Aux))
        || t.kids(v).iter().any(|&k| t.base(k) == Rel::Aux && t.form(k).ends_with("n't"))
        || t.kids(v).iter().any(|&k| t.base(k) == Rel::Aux && t.form(k) == "cannot")
}

fn often(t: &Tree, v: usize) -> Often {
    for &k in t.kids(v) {
        let l = t.lemma(k);
        if l == "never" {
            return Often::Never;
        }
        if let Some(f) = FREQ.iter().find(|f| f.0 == l) {
            return f.1;
        }
    }
    Often::Generic
}

/// The facts of one clause headed by `r` whose subject is `subj` (inherited by a coordinated predicate).
fn clause(t: &Tree, r: usize, inherited: Option<usize>, source: &str, rules: &KnowRules, out: &mut Vec<Fact>) {
    let subj_id = t.kids(r).iter().copied().find(|&k| t.base(k) == Rel::Nsubj).or(inherited);
    let neg = negated(t, r);
    let oft = often(t, r);
    let fact = |subj: String, link: Link, obj: String, count: Option<String>| Fact { subj, link, obj, negated: neg && oft != Often::Never, often: oft, count, source: source.to_string() };
    // «There are 60 minutes in an hour»
    if t.kids_of(r, Rel::Expl).iter().any(|&e| t.lemma(e) == "there") {
        if let Some(s) = subj_id {
            let n = t.kids_of(s, Rel::Nummod).first().map(|&k| t.form(k).to_string());
            if let Some(&o) = t.kids(s).iter().chain(t.kids(r).iter()).find(|&&k| matches!(t.base(k), Rel::Obl | Rel::Nmod) && t.kids_of(k, Rel::Case).iter().any(|&c| matches!(t.lemma(c), "in" | "per"))) {
                out.push(fact(t.lemma(s).to_string(), Link::CountIn, phrase(t, o), n));
            }
        }
    } else if let Some(s) = subj_id.filter(|&s| !(rules.quotes_are_examples && quoted(t, s))) {
        let subj = phrase(t, s);
        let aux: Vec<&str> = t.kids(r).iter().filter(|&&k| t.base(k) == Rel::Aux).map(|&k| t.lemma(k)).collect();
        // «would be strange», «might break»: a supposition, not a fact
        if aux.iter().any(|a| rules.suppose.iter().any(|x| x == a)) || t.kids(r).iter().any(|&k| t.base(k) == Rel::Mark && rules.conditions.iter().any(|x| x == t.lemma(k))) {
            return;
        }
        if !t.kids_of(r, Rel::Cop).is_empty() {
            // a copula: a noun predicate is a kind, an adjective a property; «made of», «used for» in passive form
            match t.upos(r) {
                UPos::NOUN | UPos::PROPN => out.push(fact(subj, Link::IsA, phrase(t, r), None)),
                UPos::ADJ => out.push(fact(subj, Link::Is, t.lemma(r).to_string(), None)),
                _ => {}
            }
        } else if t.upos(r) == UPos::VERB {
            let verb = t.lemma(r).to_string();
            let objs = objects(t, r);
            let objs: Vec<Option<(String, Option<String>)>> = if objs.is_empty() { vec![None] } else { objs.into_iter().map(Some).collect() };
            for obj in objs {
            let passive = t.kids(r).iter().any(|&k| t.w[k].rel == Rel::AuxPass) || t.kids(r).iter().any(|&k| t.w[k].rel == Rel::NsubjPass);
            let can = aux.iter().any(|a| matches!(*a, "can" | "could" | "cannot"));
            match (verb.as_str(), obj) {
                ("make", Some((o, Some(c)))) if passive && c == "of" => out.push(fact(subj.clone(), Link::MadeOf, o, None)),
                ("use", Some((o, Some(c)))) if passive && (c == "for" || c == "to") => out.push(fact(subj.clone(), Link::UsedFor, o, None)),
                ("have", Some((o, _))) if !can => out.push(fact(subj.clone(), Link::Has, o, None)),
                ("live", Some((o, Some(c)))) if matches!(c.as_str(), "in" | "on" | "at" | "under") => out.push(fact(subj.clone(), Link::LivesIn, o, None)),
                (v, Some((o, c))) if !can && rules.verb_links.iter().any(|(lv, lc, _, k)| k.is_empty() && lv == v && lc.as_str() == c.as_deref().unwrap_or("")) => {
                    let l = rules.verb_links.iter().find(|(lv, lc, _, k)| k.is_empty() && lv == v && lc.as_str() == c.as_deref().unwrap_or("")).map(|x| x.2).unwrap_or(Link::Does);
                    out.push(fact(subj.clone(), l, o, None));
                }
                (_, o) => {
                    let what = match o {
                        Some((o, Some(c))) => format!("{verb} {c} {o}"),
                        Some((o, None)) => format!("{verb} {o}"),
                        None => verb.clone(),
                    };
                    out.push(fact(subj.clone(), if can { Link::Can } else { Link::Does }, what, None));
                }
            }
            }
        }
    }
    // coordinated predicates share the subject; coordinated clauses bring their own
    for &c in t.kids(r).iter().filter(|&&k| t.base(k) == Rel::Conj) {
        if matches!(t.upos(c), UPos::VERB | UPos::ADJ | UPos::NOUN) {
            clause(t, c, subj_id, source, rules, out);
        }
    }
}

/// «An hour has sixty minutes», «an hour is sixty minutes»: a measure — sixty minutes in an hour.
fn measures(out: &mut Vec<Fact>) {
    let extra: Vec<Fact> = out
        .iter()
        .filter(|f| matches!(f.link, Link::Has | Link::IsA) && !f.negated)
        .filter_map(|f| {
            let mut ws = f.obj.split_whitespace();
            let n = ws.next()?;
            let unit: Vec<&str> = ws.collect();
            let is_num = n.chars().all(|c| c.is_ascii_digit() || c == ',' || c == '.') || crate::triggers::number_word(n).is_some();
            (is_num && !unit.is_empty()).then(|| Fact { subj: unit.join(" "), link: Link::CountIn, obj: f.subj.clone(), negated: false, often: f.often, count: Some(n.to_string()), source: f.source.clone() })
        })
        .collect();
    out.extend(extra);
}

/// The facts of one sentence, by the skeleton's rules.
pub fn facts(sentence: &str) -> Result<Vec<Fact>> {
    facts_with(sentence, &KnowRules::default())
}

/// The facts of one sentence, by the given rules.
pub fn facts_with(sentence: &str, rules: &KnowRules) -> Result<Vec<Fact>> {
    let a = crate::tree::annotator()?;
    let t = Tree::parse(a, sentence);
    if std::env::var("KNOW_DEBUG").is_ok() {
        for (i, w) in t.w.iter().enumerate() {
            eprintln!("  {} {} {} {:?} {} {:?}", i + 1, w.form, w.lemma, w.upos, w.head, w.rel);
        }
    }
    let mut out = Vec::new();
    if let Some(r) = t.root() {
        clause(&t, r, None, sentence, rules, &mut out);
    }
    if rules.measures {
        measures(&mut out);
    }
    Ok(out)
}

/// Facts of many sentences, in their order (split across threads, put back by index: the same on every machine).
pub fn facts_of_all(sentences: &[String]) -> Vec<Fact> {
    facts_of_all_with(sentences, &KnowRules::default())
}

pub fn facts_of_all_with(sentences: &[String], rules: &KnowRules) -> Vec<Fact> {
    let threads = std::thread::available_parallelism().map(|x| x.get()).unwrap_or(4).min(10);
    let chunk = sentences.len().div_ceil(threads).max(1);
    let parts: Vec<Vec<Fact>> = std::thread::scope(|s| {
        let hs: Vec<_> = sentences.chunks(chunk).map(|c| s.spawn(move || c.iter().flat_map(|x| facts_with(x, rules).unwrap_or_default()).collect::<Vec<_>>())).collect();
        hs.into_iter().map(|h| h.join().unwrap_or_default()).collect()
    });
    parts.into_iter().flatten().collect()
}

/// The kinds whose members most often do a verb: the «is a» objects of the subjects of facts that use it.
pub fn kinds_doing(k: &Knowledge, verb: &str, n: usize) -> Vec<String> {
    let mut count: BTreeMap<String, usize> = BTreeMap::new();
    for fs in k.by_subj.values() {
        if !fs.iter().any(|f| f.obj.split_whitespace().next() == Some(verb) || f.source.to_lowercase().contains(&format!(" {verb}"))) {
            continue;
        }
        for f in fs.iter().filter(|f| f.link == Link::IsA && !f.negated && !f.obj.contains(' ')) {
            *count.entry(f.obj.clone()).or_default() += 1;
        }
    }
    let mut v: Vec<(String, usize)> = count.into_iter().filter(|x| x.1 >= 2).collect();
    v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    v.into_iter().take(n).map(|x| x.0).collect()
}

/// Verb meanings bound to a kind, applied once all facts are read: a «does» fact whose subject is of that kind.
fn apply_kind_links(all: &mut [Fact], rules: &KnowRules) {
    let bound: Vec<&(String, String, Link, String)> = rules.verb_links.iter().filter(|x| !x.3.is_empty()).collect();
    if bound.is_empty() {
        return;
    }
    let mut kinds: BTreeMap<String, std::collections::BTreeSet<String>> = BTreeMap::new();
    for f in all.iter().filter(|f| f.link == Link::IsA && !f.negated) {
        kinds.entry(f.subj.clone()).or_default().insert(f.obj.clone());
    }
    for f in all.iter_mut().filter(|f| f.link == Link::Does) {
        let ws: Vec<String> = f.obj.split_whitespace().map(String::from).collect();
        let Some(verb) = ws.first() else { continue };
        let (prep, rest) = if ws.len() > 2 && ["in", "of", "for", "to", "on", "at", "as", "with", "from", "into"].contains(&ws[1].as_str()) { (ws[1].clone(), ws[2..].join(" ")) } else { (String::new(), ws[1..].join(" ")) };
        if rest.is_empty() {
            continue;
        }
        if let Some((_, _, l, _)) = bound.iter().find(|(v, p, _, k)| v == verb && *p == prep && kinds.get(&f.subj).is_some_and(|ks| ks.contains(k))) {
            f.link = *l;
            // «used for» keeps the deed («used for measure temperature»), the other links keep the thing
            if *l != Link::UsedFor {
                f.obj = rest;
            }
        }
    }
}

/// The facts by subject (lower case).
pub struct Knowledge {
    pub by_subj: BTreeMap<String, Vec<Fact>>,
    pub n: usize,
    pub rules: KnowRules,
}

impl Knowledge {
    pub fn of(sentences: &[String]) -> Knowledge {
        Knowledge::of_with(sentences, KnowRules::default())
    }

    pub fn of_with(sentences: &[String], rules: KnowRules) -> Knowledge {
        let mut all = facts_of_all_with(sentences, &rules);
        apply_kind_links(&mut all, &rules);
        let derived = derive(&all);
        all.extend(derived);
        let n = all.len();
        let mut by_subj: BTreeMap<String, Vec<Fact>> = BTreeMap::new();
        for f in all {
            by_subj.entry(f.subj.to_lowercase()).or_default().push(f);
        }
        Knowledge { by_subj, n, rules }
    }

    /// Facts about a subject: the exact phrase, else facts whose subject ends with its head word.
    pub fn about(&self, subj: &str) -> Vec<&Fact> {
        let s = subj.to_lowercase();
        if let Some(v) = self.by_subj.get(&s) {
            return v.iter().collect();
        }
        let head = s.split_whitespace().last().unwrap_or("").to_string();
        self.by_subj.iter().filter(|(k, _)| k.split_whitespace().last() == Some(head.as_str())).flat_map(|(_, v)| v.iter()).collect()
    }

    /// The features of a fact as an answer (`RANK_FEATS`).
    pub fn rank_feats(&self, f: &Fact, asked: &str) -> [i64; 13] {
        let src = f.source.to_lowercase();
        let head = f.subj.split_whitespace().last().unwrap_or("").to_string();
        let opens = src.starts_with(&format!("a {head}")) || src.starts_with(&format!("an {head}")) || src.starts_with(&format!("the {head}")) || src.starts_with(&head);
        let definition = src.contains(" is a ") || src.contains(" is an ") || src.contains(" are ");
        let b = |x: bool| i64::from(x);
        [
            b(f.link == Link::IsA),
            b(matches!(f.link, Link::UsedFor | Link::MadeOf)),
            b(f.link == Link::Is),
            b(matches!(f.link, Link::Has | Link::CountIn)),
            b(matches!(f.link, Link::Does | Link::Can)),
            b(f.link == Link::IsA && !definition),
            b(!opens),
            b(f.source.len() < 80),
            b(f.negated || f.often == Often::Never),
            (f.source.len() / 10) as i64,
            b(!asked.is_empty() && (f.subj == asked || (f.link == Link::CountIn && f.obj == asked))),
            b(matches!(world_of(&f.source), "everyday" | "biology" | "body-health" | "geography" | "mind" | "time" | "technology")),
            b(matches!(world_of(&f.source), "logic" | "math" | "programming" | "ai" | "hardware" | "language" | "self")),
        ]
    }

    /// The facts in the order they answer: by the learned weights, or by the hand-set order.
    pub fn order(&self, fs: &mut [&Fact], asked: &str) {
        if let Some(w) = self.rules.rank_weights.as_ref().filter(|w| w.len() == RANK_FEATS.len()) {
            let sc = |f: &Fact| self.rank_feats(f, asked).iter().zip(w).map(|(a, b)| a * b).sum::<i64>();
            fs.sort_by(|a, b| sc(b).cmp(&sc(a)).then(a.source.len().cmp(&b.source.len())).then(a.source.cmp(&b.source)));
            return;
        }
        let prio = |l: Link| match l { Link::IsA => 0, Link::UsedFor | Link::MadeOf => 1, Link::Is => 2, _ => 3 };
        let opens = |f: &Fact| self.rank_feats(f, asked)[6] == 1;
        // a definition says «is a/an», a description «is the most …»
        let definite = |f: &Fact| self.rules.definitions_first && self.rank_feats(f, asked)[5] == 1;
        fs.sort_by(|a, b| prio(a.link).cmp(&prio(b.link)).then(definite(a).cmp(&definite(b))).then(opens(a).cmp(&opens(b))).then(a.source.len().cmp(&b.source.len())).then(a.source.cmp(&b.source)));
    }

    /// The facts that could answer a question that is answered by choosing among them («what is», «what does … have»,
    /// «how many»), with the word that opens a positive answer.
    pub fn pool(&self, q: &str) -> Option<(Vec<&Fact>, &'static str, String)> {
        let ws: Vec<String> = q.split(|c: char| !(c.is_alphanumeric() || c == '\'')).filter(|w| !w.is_empty()).map(str::to_lowercase).collect();
        let art = |w: &str| matches!(w, "a" | "an" | "the");
        let sing = |w: &str| crate::nerves::norm(w).unwrap_or_else(|| w.to_string());
        let subject = |from: usize, upto: usize| -> Option<String> {
            let from = from + usize::from(ws.get(from).is_some_and(|w| art(w)));
            let runs: Vec<String> = (from + 1..=upto.min(ws.len())).rev().map(|e| ws[from..e].iter().map(|w| sing(w)).collect::<Vec<_>>().join(" ")).collect();
            runs.iter().find(|s| self.by_subj.contains_key(*s)).or_else(|| runs.iter().find(|s| !self.about(s).is_empty())).cloned()
        };
        let first = ws.first().map(String::as_str).unwrap_or("");
        if first == "how" && ws.get(1).map(String::as_str) == Some("many") {
            let unit = ws.get(2).map(|w| sing(w)).unwrap_or_default();
            let inside = ws.iter().position(|w| w == "in").map(|i| ws[i + 1..].iter().filter(|w| !art(w)).map(|w| sing(w)).collect::<Vec<_>>().join(" "));
            let holder = match (ws.iter().position(|w| w == "does" || w == "do"), ws.last().map(String::as_str)) {
                (Some(d), Some("have")) => Some(ws[d + 1..ws.len() - 1].iter().filter(|w| !art(w)).map(|w| sing(w)).collect::<Vec<_>>().join(" ")),
                _ => None,
            };
            let inside = inside.or(holder).unwrap_or_default();
            return Some((self.about(&unit).into_iter().filter(|f| f.link == Link::CountIn && (inside.is_empty() || f.obj.ends_with(&inside))).collect(), "So many", inside));
        }
        if first == "what" && matches!(ws.get(1).map(String::as_str), Some("is" | "are")) {
            let subj = subject(2, ws.len())?;
            let about = self.about(&subj);
            let mut fs: Vec<&Fact> = about.iter().copied().filter(|f| f.link == Link::IsA).collect();
            fs.extend(about.iter().copied().filter(|f| matches!(f.link, Link::UsedFor | Link::MadeOf | Link::Does | Link::Is)));
            return Some((fs, "It is", subj));
        }
        if first == "what" && matches!(ws.get(1).map(String::as_str), Some("does" | "do")) && ws.last().map(String::as_str) == Some("have") {
            let subj = subject(2, ws.len() - 1)?;
            return Some((self.about(&subj).into_iter().filter(|f| f.link == Link::Has && !f.negated).collect(), "It has", subj));
        }
        None
    }

    /// A fact said as an answer.
    pub fn say(f: &Fact, yes: &str) -> String {
        format!("{}: «{}» (from «{}»)", if f.negated || f.often == Often::Never { "No" } else { yes }, f.show(), f.source)
    }

    /// A simple question: «What is X?», «Can X V?», «Does X V Y?», «What does X have?», «How many X are in a Y?».
    /// Questions are the skeleton's own interface: they are read by their words (the parser still misreads many
    /// questions), the subject found as the longest run of words the knowledge has facts about.
    pub fn ask(&self, q: &str) -> String {
        let ws: Vec<String> = q.split(|c: char| !(c.is_alphanumeric() || c == '\'')).filter(|w| !w.is_empty()).map(str::to_lowercase).collect();
        let art = |w: &str| matches!(w, "a" | "an" | "the");
        let sing = |w: &str| crate::nerves::norm(w).unwrap_or_else(|| w.to_string());
        let answer = |mut fs: Vec<&Fact>, yes: &str, none: &str, asked: &str| -> String {
            self.order(&mut fs, asked);
            match fs.first() {
                Some(f) => Knowledge::say(f, yes),
                None => none.to_string(),
            }
        };
        let unknown = "I do not know: no fact in my worlds says it.";
        if let Some((fs, yes, asked)) = self.pool(q) {
            return answer(fs, yes, unknown, &asked);
        }
        // the subject: words after the question word, as long as the knowledge knows them
        // the subject: the longest run of words that is exactly a known subject, else the longest the head-word
        // fallback knows («people fly» must not become a «fly» the insect)
        let subject = |from: usize, upto: usize| -> Option<(String, usize)> {
            let from = from + usize::from(ws.get(from).is_some_and(|w| art(w)));
            let runs: Vec<(String, usize)> = (from + 1..=upto.min(ws.len())).rev().map(|e| (ws[from..e].iter().map(|w| sing(w)).collect::<Vec<_>>().join(" "), e)).collect();
            runs.iter().find(|(s, _)| self.by_subj.contains_key(s)).or_else(|| runs.iter().find(|(s, _)| !self.about(s).is_empty())).cloned()
        };
        let first = ws.first().map(String::as_str).unwrap_or("");
        if first == "how" && ws.get(1).map(String::as_str) == Some("many") {
            let unit = ws.get(2).map(|w| sing(w)).unwrap_or_default();
            let inside = ws.iter().position(|w| w == "in").map(|i| ws[i + 1..].iter().filter(|w| !art(w)).map(|w| sing(w)).collect::<Vec<_>>().join(" "));
            // «how many legs does a spider have»: the holder stands between «does» and «have»
            let holder = match (ws.iter().position(|w| w == "does" || w == "do"), ws.last().map(String::as_str)) {
                (Some(d), Some("have")) => Some(ws[d + 1..ws.len() - 1].iter().filter(|w| !art(w)).map(|w| sing(w)).collect::<Vec<_>>().join(" ")),
                _ => None,
            };
            let inside = inside.or(holder).unwrap_or_default();
            let fs: Vec<&Fact> = self.about(&unit).into_iter().filter(|f| f.link == Link::CountIn && (inside.is_empty() || f.obj.ends_with(&inside))).collect();
            return answer(fs, "So many", unknown, "");
        }
        if first == "what" && matches!(ws.get(1).map(String::as_str), Some("is" | "are")) {
            let Some((subj, _)) = subject(2, ws.len()) else { return unknown.into() };
            let about = self.about(&subj);
            let mut fs: Vec<&Fact> = about.iter().copied().filter(|f| f.link == Link::IsA).collect();
            fs.extend(about.iter().copied().filter(|f| matches!(f.link, Link::UsedFor | Link::MadeOf | Link::Does | Link::Is)));
            return answer(fs, "It is", unknown, "");
        }
        if first == "what" && matches!(ws.get(1).map(String::as_str), Some("does" | "do")) && ws.last().map(String::as_str) == Some("have") {
            let Some((subj, _)) = subject(2, ws.len() - 1) else { return unknown.into() };
            return answer(self.about(&subj).into_iter().filter(|f| f.link == Link::Has && !f.negated).collect(), "It has", unknown, "");
        }
        if matches!(first, "can" | "could" | "does" | "do") {
            // «can a fish fly», «does a cat eat mice»: subject, then the verb, then what
            let Some((subj, e)) = subject(1, ws.len().saturating_sub(1)) else { return unknown.into() };
            let verb = ws.get(e).cloned().unwrap_or_default();
            let what: Vec<String> = ws[(e + 1).min(ws.len())..].iter().filter(|w| !art(w)).map(|w| sing(w)).collect();
            let link = if matches!(first, "can" | "could") { Link::Can } else { Link::Does };
            let about = self.about(&subj);
            let same_verb: Vec<&Fact> = about.iter().copied().filter(|f| f.link == link && f.obj.split_whitespace().next() == Some(verb.as_str())).collect();
            let mut exact: Vec<&Fact> = same_verb.iter().copied().filter(|f| what.iter().all(|w| f.obj.split_whitespace().any(|x| sing(x) == *w))).collect();
            // a fact that holds only under a condition («cannot fly with a broken wing») does not answer the plain question
            let extra = |f: &Fact| if self.rules.conditional_answers { f.obj.split_whitespace().count().saturating_sub(1 + what.len()) } else { 0 };
            exact.sort_by(|a, b| extra(a).cmp(&extra(b)).then(a.source.len().cmp(&b.source.len())).then(a.source.cmp(&b.source)));
            if let Some(f) = exact.first() {
                if extra(f) == 0 {
                    return answer(vec![f], "Yes", unknown, "");
                }
                return format!("Only under a condition: «{}» (from «{}»)", f.show(), f.source);
            }
            if let Some(f) = same_verb.first() {
                return format!("I do not know that; I know only «{}» (from «{}»)", f.show(), f.source);
            }
            return unknown.into();
        }
        "I can answer «what is», «can», «does», «what does … have» and «how many … in» for now.".into()
    }
}

// ---- Derivation: new facts from two known ones ---------------------------------------------------------------------
//
// «People do not have wings» and «birds fly with their wings» give «people cannot fly with wings»: something that lacks
// what an action is done with cannot do the action with it. Every derived fact names the two facts it came from.

/// Facts derived from the facts: (lacking an instrument) A has no B, and something does V with B → A cannot V with B.
pub fn derive(facts: &[Fact]) -> Vec<Fact> {
    // what is done with each thing: thing → (verb, source)
    let mut done_with: BTreeMap<String, Vec<(String, String)>> = BTreeMap::new();
    for f in facts.iter().filter(|f| matches!(f.link, Link::Does | Link::Can) && !f.negated) {
        let ws: Vec<&str> = f.obj.split_whitespace().collect();
        if let Some(i) = ws.iter().position(|w| *w == "with") {
            if i >= 1 && i + 1 < ws.len() {
                done_with.entry(ws[i + 1..].join(" ")).or_default().push((ws[0].to_string(), f.source.clone()));
            }
        }
    }
    let mut out = inherit(facts);
    let mut seen = std::collections::BTreeSet::new();
    for f in facts.iter().filter(|f| f.link == Link::Has && f.negated) {
        for (verb, src) in done_with.get(&f.obj).into_iter().flatten() {
            if seen.insert((f.subj.clone(), verb.clone(), f.obj.clone())) {
                out.push(Fact { subj: f.subj.clone(), link: Link::Can, obj: format!("{verb} with {}", f.obj), negated: true, often: Often::Generic, count: None, source: format!("derived from «{}» and «{}»", f.source, src) });
            }
        }
    }
    out
}

/// Inheritance: «a penguin is a bird» and «birds have feathers» give «a penguin has feathers». A kind passes on what it
/// has, can and does (its own facts, not derived ones) to what is of that kind; a thing's own fact about the same verb
/// or the same part wins («penguins cannot fly» over «birds can fly»). At most 12 facts a thing, kinds of one word with
/// at least three facts — a kind known well enough to pass anything on.
pub fn inherit(facts: &[Fact]) -> Vec<Fact> {
    let mut by_subj: BTreeMap<&str, Vec<&Fact>> = BTreeMap::new();
    for f in facts {
        by_subj.entry(f.subj.as_str()).or_default().push(f);
    }
    let key = |f: &Fact| (f.link, f.obj.split_whitespace().next().unwrap_or("").to_string());
    // a sentence that opens with its subject («A bird has …», «Birds can …»): a general statement, not a passing remark
    let opens = |f: &Fact| {
        let src = f.source.to_lowercase();
        let head = f.subj.split_whitespace().last().unwrap_or("");
        ["a ", "an ", "the ", ""].iter().any(|a| src.starts_with(&format!("{a}{head}")))
    };
    // a definition: «A penguin is a bird.» — opens with the subject, says «is a/an» or «are», no number in the subject
    let definition = |f: &Fact| opens(f) && (f.source.contains(" is a ") || f.source.contains(" is an ") || f.source.contains(" are ")) && !f.subj.chars().any(|c| c.is_ascii_digit());
    let mut out = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    for f in facts.iter().filter(|f| f.link == Link::IsA && !f.negated && f.subj != f.obj && !f.obj.contains(' ') && definition(f)) {
        let Some(kind) = by_subj.get(f.obj.as_str()) else { continue };
        // what a kind passes on: what it has and what it can or cannot do, said of the kind itself
        // and only within one world: a «tree» of programming does not inherit from a «tree» of biology
        let passed: Vec<&&Fact> = kind.iter().filter(|g| matches!(g.link, Link::Has | Link::Can) && g.often != Often::Never && opens(g) && world_of(&g.source) == world_of(&f.source)).collect();
        if passed.len() < 3 {
            continue;
        }
        let own: std::collections::BTreeSet<(Link, String)> = by_subj.get(f.subj.as_str()).into_iter().flatten().map(|g| key(g)).collect();
        // a negated own fact about the same verb also blocks («cannot fly» blocks «can fly»)
        let own_verbs: std::collections::BTreeSet<String> = by_subj.get(f.subj.as_str()).into_iter().flatten().map(|g| g.obj.split_whitespace().next().unwrap_or("").to_string()).collect();
        let mut n = 0;
        for g in passed {
            if n >= 12 {
                break;
            }
            let k = key(g);
            if own.contains(&k) || (matches!(g.link, Link::Can | Link::Does) && own_verbs.contains(&k.1)) {
                continue;
            }
            if seen.insert((f.subj.clone(), g.link, g.obj.clone())) {
                out.push(Fact { subj: f.subj.clone(), link: g.link, obj: g.obj.clone(), negated: g.negated, often: g.often, count: None, source: format!("derived from «{}» and «{}»", f.source, g.source) });
                n += 1;
            }
        }
    }
    out
}

fn central_sentences() -> &'static Vec<String> {
    static S: std::sync::OnceLock<Vec<String>> = std::sync::OnceLock::new();
    S.get_or_init(|| crate::state::CENTRAL_MANUALS.iter().flat_map(|m| crate::nerves::sentences_of(m.2)).collect())
}

/// The knowledge of the central worlds by the given rules (the last one kept: rules change rarely).
pub fn knowledge_with(rules: &KnowRules) -> std::sync::Arc<Knowledge> {
    static K: std::sync::Mutex<Option<std::sync::Arc<Knowledge>>> = std::sync::Mutex::new(None);
    let mut g = K.lock().unwrap();
    if let Some(k) = g.as_ref().filter(|k| &k.rules == rules) {
        return k.clone();
    }
    let k = std::sync::Arc::new(Knowledge::of_with(central_sentences(), rules.clone()));
    *g = Some(k.clone());
    k
}

/// The knowledge by the rules of the current state's brain.
pub fn knowledge() -> std::sync::Arc<Knowledge> {
    knowledge_with(&current_rules())
}

/// The rules of the hands written in the current state's brain (the defaults if none).
pub fn current_rules() -> KnowRules {
    let brain = crate::state::brain_read_of(&crate::state::current_id());
    let sec: String = brain.lines().skip_while(|l| *l != "## Rules of my knowledge").skip(1).take_while(|l| !l.starts_with("## ")).collect::<Vec<_>>().join("\n");
    KnowRules::from_english(&sec)
}

// ---- Thinking ahead: answers prepared before the questions come ------------------------------------------------
//
// Reacting to a question from nothing is slow; a mind that has thought ahead looks up what it prepared. Between rounds
// the small ones take the things their worlds say most about and answer the likely questions about them — what it
// is, what it has, what it can and does — into a cache keyed by the rules of their knowledge. The chat looks there
// first. The answers are a determinant of the rules, so a prepared answer stays right until the rules change.

/// A question reduced to its key: lower case, plain singular words, no articles or punctuation.
pub fn question_key(q: &str) -> String {
    q.split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()).map(str::to_lowercase).filter(|w| !matches!(w.as_str(), "a" | "an" | "the")).map(|w| crate::nerves::norm(&w).unwrap_or(w)).collect::<Vec<_>>().join(" ")
}

fn anticipated_path() -> std::path::PathBuf {
    crate::state::root().join("anticipated.tsv")
}

/// The likely questions about the things the knowledge says most about, answered: (key, question, answer).
pub fn anticipate(k: &Knowledge, subjects: usize) -> Vec<(String, String, String)> {
    let mut subj: Vec<(&String, &Vec<Fact>)> = k.by_subj.iter().filter(|(s, _)| s.chars().all(|c| c.is_alphabetic() || c == ' ') && s.len() > 2).collect();
    subj.sort_by(|a, b| b.1.len().cmp(&a.1.len()).then(a.0.cmp(b.0)));
    let mut out = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    for (s, fs) in subj.into_iter().take(subjects) {
        let mut qs = vec![format!("What is a {s}?"), format!("What does a {s} have?")];
        for f in fs.iter().filter(|f| matches!(f.link, Link::Can | Link::Does)).take(6) {
            let mut ws = f.obj.split_whitespace();
            let verb = ws.next().unwrap_or("");
            let obj: Vec<&str> = ws.take(2).collect();
            qs.push(format!("{} a {s} {verb}{}?", if f.link == Link::Can { "Can" } else { "Does" }, if obj.is_empty() { String::new() } else { format!(" {}", obj.join(" ")) }));
        }
        for q in qs {
            let key = question_key(&q);
            if !seen.insert(key.clone()) {
                continue;
            }
            let a = k.ask(&q);
            if !a.starts_with("I do not know") && !a.starts_with("I can answer") {
                out.push((key, q, a));
            }
        }
    }
    out
}

/// Prepare the answers again when the rules of knowledge changed since the last time (or never were): how many.
pub fn anticipate_if_stale(subjects: usize) -> Option<usize> {
    let rules = current_rules();
    let stamp = format!("{:016x}", crate::induce::fnv(&format!("{}{}", rules.to_english(), crate::induce::code_key())));
    let p = anticipated_path();
    let old = std::fs::read_to_string(&p).unwrap_or_default();
    if old.lines().next() == Some(&format!("# rules {stamp}")) {
        return None;
    }
    let k = knowledge_with(&rules);
    let v = anticipate(&k, subjects);
    let body: String = std::iter::once(format!("# rules {stamp}\n")).chain(v.iter().map(|(key, q, a)| format!("{key}\t{q}\t{}\n", a.replace(['\t', '\n'], " ")))).collect();
    let _ = crate::store::write_atomic(&p, body.as_bytes());
    Some(v.len())
}

/// A prepared answer for a question, if the small ones thought of it ahead.
pub fn prepared(q: &str) -> Option<String> {
    let key = question_key(q);
    std::fs::read_to_string(anticipated_path()).ok()?.lines().skip(1).find_map(|l| {
        let mut c = l.splitn(3, '\t');
        (c.next()? == key).then(|| c.nth(1).map(String::from)).flatten()
    })
}

/// An answer for the family's chat: a trigger that computes (durations, cycles, leap years) with the world sentence
/// that explains it, else a fact of the worlds; none when neither knows.
pub fn family_answer(q: &str) -> Option<String> {
    for (t, said) in crate::triggers::fire(q) {
        if matches!(t.name, "duration" | "cycles" | "calendar") {
            let why = crate::triggers::explain(t, q, 1);
            return Some(match why.first() {
                Some(w) => format!("{said}. Because my world of {} says: «{w}»", t.world),
                None => format!("{said}."),
            });
        }
    }
    if let Some(a) = prepared(q) {
        return Some(format!("{a} — I had thought of it ahead."));
    }
    let a = knowledge().ask(q);
    (!a.starts_with("I do not know") && !a.starts_with("I can answer")).then_some(a)
}

/// The independent questions with their expected answers.
pub fn qa_rows() -> Vec<(String, String)> {
    [include_str!("../data/know-qa.tsv"), include_str!("../data/know-qa-2.tsv"), include_str!("../data/know-qa-3.tsv"), include_str!("../data/know-qa-4.tsv")].concat().lines().filter(|l| !l.starts_with('#') && !l.trim().is_empty()).filter_map(|l| l.split_once('\t')).map(|(q, e)| (q.to_string(), e.trim().to_lowercase())).collect()
}

/// Is an answer right for the expected one (yes, no, a number, or words separated by «|»)?
pub fn right_answer(ans: &str, exp: &str) -> bool {
    let a = ans.to_lowercase();
    match exp {
        "yes" => !a.starts_with("no") && !a.starts_with("only"),
        "no" => a.starts_with("no"),
        n if n.chars().all(|c| c.is_ascii_digit()) => {
            let words: Vec<f64> = a.split(|c: char| !(c.is_alphanumeric() || c == '-')).filter_map(|w| w.parse::<f64>().ok().or_else(|| crate::triggers::number_word(w))).collect();
            words.contains(&n.parse::<f64>().unwrap_or(-1.0))
        }
        w => w.split('|').any(|x| a.contains(x)),
    }
}

/// Learn the weights of the answers on the dev half: an averaged perceptron over the questions answered by choosing
/// (when the chosen fact is wrong and a right one is in the pool, the weights move towards the right one); integers,
/// a fixed order of questions and epochs — the same weights on every machine.
pub fn train_weights(k: &Knowledge, epochs: usize) -> Vec<i64> {
    let rows: Vec<(String, String)> = qa_rows().into_iter().enumerate().filter(|(i, _)| i % 2 == 0).map(|x| x.1).collect();
    let pools: Vec<(Vec<[i64; 13]>, Vec<bool>)> = rows
        .iter()
        .filter_map(|(q, exp)| {
            let (fs, yes, asked) = k.pool(q)?;
            let feats: Vec<[i64; 13]> = fs.iter().map(|f| k.rank_feats(f, &asked)).collect();
            let ok: Vec<bool> = fs.iter().map(|f| right_answer(&Knowledge::say(f, yes), exp)).collect();
            (ok.iter().any(|&x| x) && ok.iter().any(|&x| !x)).then_some((feats, ok))
        })
        .collect();
    let mut w = HAND_WEIGHTS;
    let mut sum = [0i64; 13];
    let score = |w: &[i64; 13], f: &[i64; 13]| f.iter().zip(w).map(|(a, b)| a * b).sum::<i64>();
    for _ in 0..epochs {
        for (feats, ok) in &pools {
            // ties go to the shorter sentence, as in `order`
            let best = |only_ok: bool| (0..feats.len()).filter(|&i| !only_ok || ok[i]).max_by(|&a, &b| score(&w, &feats[a]).cmp(&score(&w, &feats[b])).then(feats[b][9].cmp(&feats[a][9])).then(b.cmp(&a)));
            let (Some(p), Some(r)) = (best(false), best(true)) else { continue };
            if !ok[p] {
                for j in 0..13 {
                    w[j] += feats[r][j] - feats[p][j];
                }
            }
            for j in 0..13 {
                sum[j] += w[j];
            }
        }
    }
    sum.to_vec()
}

/// The check of the hands on an independent set of questions (`world/data/know-qa.tsv`): how many are answered, how
/// many of those rightly; a negative control (the expected answers shifted by one question) must score far lower.
pub fn check(k: &Knowledge) -> (usize, usize, usize, usize) {
    check_half(k, None)
}

/// The check on one half of the questions: `Some(0)` the dev half (even rows), `Some(1)` the guard half (odd rows).
pub fn check_half(k: &Knowledge, half: Option<usize>) -> (usize, usize, usize, usize) {
    let rows: Vec<(String, String)> = qa_rows().into_iter().enumerate().filter(|(i, _)| half.is_none_or(|h| i % 2 == h)).map(|x| x.1).collect();
    let right = |ans: &str, exp: &str| -> bool {
        let a = ans.to_lowercase();
        match exp {
            "yes" => !a.starts_with("no") && !a.starts_with("only"),
            "no" => a.starts_with("no"),
            n if n.chars().all(|c| c.is_ascii_digit()) => {
                let words: Vec<f64> = a.split(|c: char| !(c.is_alphanumeric() || c == '-')).filter_map(|w| w.parse::<f64>().ok().or_else(|| crate::triggers::number_word(w))).collect();
                words.contains(&n.parse::<f64>().unwrap_or(-1.0))
            }
            // a word, or several right words separated by «|»
            w => w.split('|').any(|x| a.contains(x)),
        }
    };
    let answers: Vec<String> = rows.iter().map(|(q, _)| k.ask(q)).collect();
    let answered = |a: &str| !a.starts_with("I do not know") && !a.starts_with("I can answer") && !a.starts_with("I do not");
    let (mut n_ans, mut n_right, mut n_ctrl) = (0, 0, 0);
    for (i, ((q, exp), a)) in rows.iter().zip(&answers).enumerate() {
        if answered(a) {
            n_ans += 1;
            if right(a, exp) {
                n_right += 1;
            }
            if right(a, &rows[(i + 1) % rows.len()].1) {
                n_ctrl += 1;
            }
        }
        if std::env::var("KNOW_CHECK_SHOW").is_ok() {
            eprintln!("{} {q} → {a}", if !answered(a) { "·" } else if right(a, exp) { "✓" } else { "✗" });
        }
    }
    (rows.len(), n_ans, n_right, n_ctrl)
}

/// The score of a check: right answers minus wrong ones (an honest «I do not know» costs nothing).
pub fn score(c: (usize, usize, usize, usize)) -> i64 {
    2 * c.2 as i64 - c.1 as i64
}

/// One step of learning the hands: try up to `k` ideas (from `offset`, in a fixed order) on the dev half; the best
/// that gains there, loses nothing on the guard half and does not lift the negative control is returned with its
/// numbers — (what changed, new rules, dev gain, guard gain).
pub fn learn_step(rules: &KnowRules, offset: usize, k: usize) -> Option<(String, KnowRules, i64, i64)> {
    let base = knowledge_with(rules);
    let (bd, bg) = (check_half(&base, Some(0)), check_half(&base, Some(1)));
    // the audit first: a learned verb meaning must earn its place on the guard half, or it goes
    for i in 0..rules.verb_links.len() {
        let mut without = rules.clone();
        without.verb_links.remove(i);
        let g_without = check_half(&knowledge_with(&without), Some(1));
        if score(g_without) >= score(bg) {
            let (v, p, l, k) = &rules.verb_links[i];
            let _ = knowledge_with(rules);
            return Some((format!("drop the meaning of «{v}»{} as {}{} — it did not earn its place on unseen questions", if p.is_empty() { String::new() } else { format!(" with «{p}»") }, link_name(*l), if k.is_empty() { String::new() } else { format!(" for a «{k}»") }), without, 0, score(g_without) - score(bg)));
        }
    }
    let mut ideas = rules.ideas();
    ideas.extend(rules.verb_ideas(&base, 30));
    // the weights of the answers, learned on the dev half (an idea like the others: kept only if the guard agrees)
    let mut lw = rules.clone();
    lw.rank_weights = Some(train_weights(&base, 10));
    if lw != *rules {
        ideas.insert(0, (format!("learn the weights of my answers ({})", lw.rank_weights.as_ref().unwrap().iter().map(i64::to_string).collect::<Vec<_>>().join(", ")), lw));
    }
    let n = ideas.len();
    let mut best: Option<(String, KnowRules, i64, i64)> = None;
    for j in 0..k.min(n) {
        let (what, r) = ideas[(offset + j) % n].clone();
        let kn = knowledge_with(&r);
        let (d, g) = (check_half(&kn, Some(0)), check_half(&kn, Some(1)));
        let (gd, gg) = (score(d) - score(bd), score(g) - score(bg));
        let control_ok = d.3 <= bd.3 && g.3 <= bg.3;
        // the gate: a gain on the dev half AND on the guard half (questions not seen while choosing); «no worse» is not
        // a reason to keep a rule
        if gd > 0 && gg > 0 && control_ok && best.as_ref().is_none_or(|b| gd + gg > b.2 + b.3) {
            best = Some((what, r, gd, gg));
        }
    }
    // the knowledge of the rules in force is kept for the family's answers
    let _ = knowledge_with(rules);
    best
}

/// How many ideas the hands have now (their rules' own and those drawn from the facts).
pub fn idea_count() -> usize {
    let r = current_rules();
    let k = knowledge_with(&r);
    r.ideas().len() + r.verb_ideas(&k, 30).len()
}

/// `world know "<sentence>" | --ask "<question>" [world]`.
pub fn main_args(args: &[String]) -> Result<()> {
    if args.get(1).map(String::as_str) == Some("--check") {
        let (n, ans, right, ctrl) = check(&knowledge_with(&KnowRules::default()));
        println!("know check: {n} questions, {ans} answered, {right} right ({:.0}% of answered), negative control {ctrl} right", 100.0 * right as f64 / ans.max(1) as f64);
        return Ok(());
    }
    if args.get(1).map(String::as_str) == Some("--about") {
        let k = knowledge_with(&KnowRules::default());
        let want = args.get(3).cloned().unwrap_or_default();
        for f in k.about(args.get(2).map(String::as_str).unwrap_or("")).into_iter().filter(|f| want.is_empty() || f.obj.contains(&want)).take(15) {
            println!("{:?} {} | {}", f.link, f.show(), f.source.chars().take(120).collect::<String>());
        }
        return Ok(());
    }
    if args.get(1).map(String::as_str) == Some("--audit") {
        let r = current_rules();
        match learn_step(&r, 0, 0) {
            Some((what, _, gd, gg)) => println!("{what} (dev {gd:+}, guard {gg:+})"),
            None => println!("every learned verb meaning earns its place"),
        }
        return Ok(());
    }
    if args.get(1).map(String::as_str) == Some("--ahead") {
        let t0 = std::time::Instant::now();
        let k = knowledge_with(&KnowRules::default());
        let v = anticipate(&k, 3000);
        println!("{} answers prepared ahead in {:.0} s", v.len(), t0.elapsed().as_secs_f64());
        for (_, q, a) in v.iter().step_by((v.len() / 8).max(1)).take(8) {
            println!("  {q} → {}", a.chars().take(140).collect::<String>());
        }
        return Ok(());
    }
    if args.get(1).map(String::as_str) == Some("--weights") {
        let base = knowledge_with(&KnowRules::default());
        let w = train_weights(&base, 10);
        let mut r = KnowRules::default();
        r.rank_weights = Some(w.clone());
        let kn = knowledge_with(&r);
        for (name, x) in RANK_FEATS.iter().zip(&w) {
            println!("{x:>8}  {name}");
        }
        let (bd, bg) = (check_half(&base, Some(0)), check_half(&base, Some(1)));
        let (d, g) = (check_half(&kn, Some(0)), check_half(&kn, Some(1)));
        println!("hand order: dev {bd:?} guard {bg:?}\nlearned:    dev {d:?} guard {g:?}   (questions, answered, right, control)");
        return Ok(());
    }
    if args.get(1).map(String::as_str) == Some("--learn") {
        let mut rules = KnowRules::default();
        let ideas = rules.ideas().len();
        for step in 0..ideas {
            match learn_step(&rules, step * 4, 4) {
                Some((what, r, gd, gg)) => {
                    println!("step {step}: {what} (dev {gd:+}, guard {gg:+})");
                    rules = r;
                }
                None => println!("step {step}: nothing helps"),
            }
        }
        println!("{}", rules.to_english());
        return Ok(());
    }
    if args.get(1).map(String::as_str) == Some("--ask") {
        let q = args.get(2).cloned().unwrap_or_default();
        let only = args.get(3);
        let t0 = std::time::Instant::now();
        let sents: Vec<String> = crate::state::CENTRAL_MANUALS.iter().filter(|m| only.is_none_or(|o| m.0 == o)).flat_map(|m| crate::nerves::sentences_of(m.2)).collect();
        let k = Knowledge::of(&sents);
        eprintln!("{} facts from {} sentences in {:.0} s", k.n, sents.len(), t0.elapsed().as_secs_f64());
        println!("{}", k.ask(&q));
        return Ok(());
    }
    let s = args.iter().skip(1).cloned().collect::<Vec<_>>().join(" ");
    for f in facts(&s)? {
        println!("{}", f.show());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shown(s: &str) -> Vec<String> {
        facts(s).unwrap().iter().map(Fact::show).collect()
    }

    #[test]
    fn definitions_abilities_habits() {
        assert!(shown("A prime is a natural number.").iter().any(|x| x == "prime is a natural number"), "{:?}", shown("A prime is a natural number."));
        assert!(shown("Fish cannot live out of water.").iter().any(|x| x.starts_with("fish not can live")), "{:?}", shown("Fish cannot live out of water."));
        assert!(shown("People usually eat three meals a day.").iter().any(|x| x.contains("does eat") && x.ends_with("[usually]")), "{:?}", shown("People usually eat three meals a day."));
        assert!(shown("There are 60 minutes in an hour.").iter().any(|x| x == "60 minute in hour"), "{:?}", shown("There are 60 minutes in an hour."));
    }

    #[test]
    fn rules_survive_their_english() {
        let mut r = KnowRules::default();
        assert_eq!(KnowRules::from_english(&r.to_english()), r);
        r.verb_links = vec![("contain".into(), String::new(), Link::Has, String::new()), ("consist".into(), "of".into(), Link::MadeOf, String::new()), ("measure".into(), String::new(), Link::UsedFor, "device".into())];
        assert_eq!(KnowRules::from_english(&r.to_english()), r);
        let f = facts_with("A cell contains a nucleus.", &r).unwrap();
        assert!(f.iter().any(|x| x.link == Link::Has), "{:?}", f.iter().map(Fact::show).collect::<Vec<_>>());
        for (_, x) in r.ideas() {
            assert_eq!(KnowRules::from_english(&x.to_english()), x);
            assert_ne!(x, r, "every idea changes something");
        }
    }

    #[test]
    fn derived_people_cannot_fly_with_wings() {
        let k = Knowledge::of(&["People do not have wings.".to_string(), "Birds fly with their wings.".to_string()]);
        let a = k.ask("Can people fly with wings?");
        assert!(a.starts_with("No") && a.contains("derived from"), "{a}");
        // negative control: without the missing wings there is nothing to derive
        let k = Knowledge::of(&["Birds fly with their wings.".to_string()]);
        assert!(!k.ask("Can people fly with wings?").starts_with("No"));
    }

    #[test]
    fn inherited_and_own_facts() {
        // made-up words, so no sentence of the worlds can take part
        let k = Knowledge::of(&["A zorp is a blick.".to_string(), "Blicks have fins.".to_string(), "Blicks can glow.".to_string(), "Blicks have tails.".to_string(), "Zorps cannot glow.".to_string()]);
        let has = k.ask("What does a zorp have?");
        assert!(has.contains("fin") || has.contains("tail"), "{has}");
        assert!(k.ask("Can a zorp glow?").starts_with("No"), "own fact wins: {}", k.ask("Can a zorp glow?"));
    }

    #[test]
    fn negative_control_an_event_is_no_general_fact_of_its_kind() {
        // a story event still reads as «does», but never as a definition or an ability
        let f = facts("Mary went to the kitchen.").unwrap();
        assert!(f.iter().all(|x| !matches!(x.link, Link::IsA | Link::Can | Link::CountIn)), "{:?}", f.iter().map(Fact::show).collect::<Vec<_>>());
    }
}
