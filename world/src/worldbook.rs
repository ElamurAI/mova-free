//! World books: a detailed world of one topic (the Titanic first) written by Mova in plain English, built from a
//! private corpus, with a citation on almost every sentence — where the thought comes from and how it was drawn out.
//!
//! - **Reading.** Every sentence of the sources that names the topic (a lexicon of topic words) is parsed. A fact
//!   is the sentence compressed along its tree: the subject phrase, the predicate with its auxiliaries, negation and
//!   particle, the object, the recipient and the obliques (time, place, manner); relative clauses, parentheses,
//!   discourse words and most modifiers are dropped. The compression is the world's sentence; the source sentence
//!   is its citation.
//! - **Merging.** Facts with the same subject, predicate and object lemmas are one fact with several sources (the
//!   support); facts with the same subject and predicate but different numbers are listed as disagreements.
//! - **Writing.** Sections: the topic itself, then the people and things most often named, then a timeline (facts
//!   with a date or a clock time), then disagreements. Every sentence ends with a footnote: the source (title,
//!   author, year, sentence id), the source sentence when the source is public domain (only the reference
//!   otherwise), and what was kept. The world book is for people only: Mova reads it back like any English text,
//!   with its ordinary NLP tools (parser, world reader, pragmatics), to solve tasks on it.
//! - **Release, not versions.** A world is not versioned on its own: its files carry no version and its header names
//!   the release it was made for (`RELEASE`). The full world and a mini world (the best-supported facts, with a link
//!   to the full one).
//!
//!   world worldbook <topic> <sources.tsv> <out dir> [--release R] [--mini K]
//!
//! `sources.tsv`: path, title, author, year, licence (`pd` — the source sentence may be quoted; anything else —
//! reference only).

use std::collections::BTreeMap;
use std::path::Path;

/// The release the worlds are made for (a world has no version of its own).
pub const RELEASE: &str = "0.4";

use anyhow::{Context, Result, bail};
use en::annotate::Word;
use en::gram::{Rel, UPos};

/// Topic lexicons: lowercase words; a sentence belongs to the topic when it has one.
fn topic_words(topic: &str) -> Result<&'static [&'static str]> {
    Ok(match topic {
        "titanic" | "titanic-principles" => &[
            "titanic", "iceberg", "lifeboat", "lifeboats", "carpathia", "californian", "olympic", "wireless", "marconi", "bridge", "boat", "boats",
            "deck", "steerage", "survivors", "survivor", "rescue", "rescued", "collision", "struck", "sank", "sinking", "foundered", "lookout", "crow's-nest",
            "captain", "smith", "ismay", "andrews", "murdoch", "lightoller", "boxhall", "lowe", "pitman", "fleet", "phillips", "bride", "rostron", "astor",
            "guggenheim", "straus", "harland", "wolff", "southampton", "queenstown", "cherbourg", "liner", "passengers", "crew", "funnel", "funnels", "hull",
            "bulkhead", "bulkheads", "watertight",
        ],
        "disasters" => &[
            "ship", "steamer", "steamship", "steamboat", "vessel", "boat", "boats", "lifeboat", "lifeboats", "wreck", "wrecked", "sank", "sinking", "foundered",
            "capsized", "collision", "drowned", "drowning", "crew", "passengers", "survivors", "survivor", "rescue", "rescued", "captain", "fire", "flames",
            "burned", "burning", "explosion", "exploded", "flood", "dam", "earthquake", "shock", "hurricane", "storm", "mine", "miners", "colliery", "bridge",
            "train", "collapsed", "victims", "dead", "killed", "injured", "bodies", "inquest", "inquiry", "coroner", "testimony", "witness", "epidemic",
            "influenza", "theatre", "panic",
        ],
        "time" => &[
            "time", "before", "after", "earlier", "later", "past", "future", "present", "now", "moment", "instant", "duration", "second", "seconds",
            "minute", "minutes", "hour", "hours", "day", "days", "year", "years", "clock", "clocks", "calendar", "date", "tense", "aspect", "event",
            "events", "cause", "causes", "effect", "effects", "order", "sequence", "during", "while", "until", "since", "begins", "ends", "beginning",
            "end", "cycle", "cycles", "processor", "command", "commands", "instruction", "program", "latency", "parallel", "pipeline", "utc",
            "leap", "month", "week", "rhythm", "yesterday", "today", "tomorrow", "memory", "memories", "log", "timestamp",
        ],
        "mathematics" => &[
            "theorem", "lemma", "proof", "prove", "proved", "axiom", "definition", "defined", "set", "sets", "subset", "element", "elements",
            "function", "functions", "map", "maps", "number", "numbers", "integer", "integers", "prime", "primes", "real", "rational", "complex",
            "group", "groups", "subgroup", "ring", "rings", "field", "fields", "vector", "vectors", "space", "spaces", "matrix", "matrices",
            "linear", "basis", "dimension", "eigenvalue", "sequence", "series", "limit", "converges", "convergent", "continuous", "derivative",
            "integral", "differentiable", "open", "closed", "compact", "connected", "metric", "topology", "topological", "probability", "random",
            "variable", "expected", "variance", "distribution", "independent", "graph", "graphs", "vertex", "vertices", "edge", "edges", "tree",
            "polynomial", "polynomials", "equation", "equations", "triangle", "angle", "circle", "line", "lines", "point", "points", "plane",
            "algorithm", "finite", "infinite", "countable", "bijection", "injective", "surjective", "homomorphism", "isomorphism", "isomorphic",
            "category", "functor", "order", "relation", "equivalence", "divides", "divisor", "modulo", "congruent", "bounded", "every", "exists",
        ],
        "physics" => &[
            "physics", "physical", "force", "forces", "mass", "energy", "momentum", "velocity", "acceleration", "motion", "gravity",
            "gravitational", "field", "fields", "charge", "charges", "current", "voltage", "electric", "magnetic", "electromagnetic", "wave",
            "waves", "frequency", "wavelength", "light", "photon", "photons", "electron", "electrons", "proton", "protons", "neutron", "neutrons",
            "nucleus", "nuclei", "atom", "atoms", "particle", "particles", "quantum", "spin", "temperature", "heat", "entropy", "pressure",
            "thermodynamics", "thermal", "gas", "liquid", "solid", "fluid", "crystal", "relativity", "spacetime", "speed", "inertial", "frame",
            "oscillator", "oscillation", "pendulum", "lagrangian", "hamiltonian", "conservation", "conserved", "symmetry", "radiation",
            "decay", "fusion", "fission", "star", "stars", "galaxy", "galaxies", "universe", "cosmic", "planet", "orbit", "measurement",
            "unit", "units", "metre", "meter", "kilogram", "joule", "newton", "watt", "kelvin", "interaction", "quark", "quarks", "laser",
        ],
        // a central world without its own list: the topic is what its manual talks about — the words of four letters
        // or more the manual uses at least twice, without the common words of any text
        _ => match crate::state::CENTRAL_MANUALS.iter().find(|(t, _, _)| *t == topic) {
            Some((_, _, manual)) => {
                const COMMON: [&str; 40] = ["that", "this", "with", "from", "they", "their", "there", "which", "when", "what", "have", "been", "were", "will", "into", "than", "then", "them", "more", "most", "some", "only", "also", "each", "every", "other", "such", "same", "does", "about", "many", "much", "very", "first", "after", "before", "where", "while", "mova", "manual"];
                let mut n: BTreeMap<String, usize> = BTreeMap::new();
                for w in manual.split(|c: char| !c.is_alphabetic()).filter(|w| w.chars().count() >= 4) {
                    *n.entry(w.to_lowercase()).or_default() += 1;
                }
                let words: Vec<&'static str> = n.into_iter().filter(|(w, k)| *k >= 2 && !COMMON.contains(&w.as_str())).map(|(w, _)| &*Box::leak(w.into_boxed_str())).collect();
                Box::leak(words.into_boxed_slice())
            }
            None => bail!("no lexicon for topic {topic} (known: titanic, disasters, time, mathematics, physics, and the central worlds)"),
        },
    })
}

/// One source of the corpus.
#[derive(Clone)]
pub struct Source {
    pub key: String,
    pub path: String,
    pub title: String,
    pub author: String,
    pub year: String,
    pub pd: bool,
    /// the licence as written in the source's metadata (shown in the Sources section)
    pub licence: String,
}

fn read_sources(p: &Path) -> Result<Vec<Source>> {
    let t = std::fs::read_to_string(p).with_context(|| format!("{}", p.display()))?;
    Ok(t.lines()
        .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
        .filter_map(|l| {
            let c: Vec<&str> = l.split('\t').collect();
            (c.len() >= 5).then(|| Source {
                key: Path::new(c[0]).file_stem().map(|x| x.to_string_lossy().into_owned()).unwrap_or_default().replace(".turns", ""),
                path: c[0].to_string(),
                title: c[1].to_string(),
                author: c[2].to_string(),
                year: c[3].to_string(),
                pd: c[4].trim() == "pd" || c[4].starts_with("public domain") || c[4].starts_with("CC BY") || c[4].starts_with("ours"),
                licence: c[4].trim().to_string(),
            })
        })
        .collect())
}

/// A fact: the compressed sentence and its parts, with its sources.
#[derive(Clone, Debug)]
pub struct Fact {
    pub text: String,
    pub subject: String,
    pub predicate: String,
    pub object: String,
    /// (source key, sentence number, source sentence)
    pub cites: Vec<(String, usize, String)>,
    pub kept: String,
    pub time: Option<String>,
    pub numbers: Vec<String>,
    /// what Mova sees about the candidate (for her learned selection)
    pub attrs: BTreeMap<String, String>,
    /// content lemmas of the source sentence (for grouping sentences that say the same)
    pub lemmas: Vec<String>,
    /// the nouns among them (a section is named by a noun)
    pub nouns: Vec<String>,
}

/// The whole subtree of `i`, as surface text in order.
fn subtree(w: &[Word], i: usize) -> String {
    let under = |mut x: usize| {
        for _ in 0..w.len() {
            if x == i {
                return true;
            }
            match w[x].head {
                0 => return false,
                h => x = h - 1,
            }
        }
        false
    };
    (0..w.len()).filter(|&j| under(j) && w[j].upos != UPos::PUNCT).map(|j| w[j].form.as_str()).collect::<Vec<_>>().join(" ")
}

/// The subtree of `i` restricted to the relations worth keeping in a phrase (determiners, compounds, names,
/// numbers, adjectives, possessives, case markers), as surface text in order.
fn phrase(w: &[Word], i: usize) -> String {
    let keep = |r: Rel| matches!(r, Rel::Det | Rel::Compound | Rel::Flat | Rel::Nummod | Rel::Amod | Rel::NmodPoss | Rel::Case | Rel::DetPredet | Rel::FlatForeign);
    let mut ids = vec![i];
    let mut k = 0;
    while k < ids.len() {
        let h = ids[k];
        for (j, x) in w.iter().enumerate() {
            if x.head == h + 1 && keep(x.rel) && !ids.contains(&j) {
                ids.push(j);
            }
        }
        // "of" modifiers of the head ("the captain of the ship") stay too, one level
        if h == i {
            for (j, x) in w.iter().enumerate() {
                if x.head == h + 1 && x.rel == Rel::Nmod && w.iter().any(|c| c.head == j + 1 && c.rel == Rel::Case && c.lemma.eq_ignore_ascii_case("of")) && !ids.contains(&j) {
                    ids.push(j);
                }
            }
        }
        k += 1;
    }
    ids.sort();
    ids.iter().map(|&j| w[j].form.as_str()).collect::<Vec<_>>().join(" ").replace(" 's", "'s").replace(" ,", ",")
}

const MONTHS: &[&str] = &["january", "february", "march", "april", "may", "june", "july", "august", "september", "october", "november", "december"];

fn is_time(s: &str) -> bool {
    let l = s.to_lowercase();
    MONTHS.iter().any(|m| l.split_whitespace().any(|x| x == *m)) || l.contains("o'clock") || l.contains("p.m") || l.contains("a.m") || l.contains("midnight") || l.split(|c: char| !c.is_ascii_digit() && c != '.' && c != ':').any(|x| x.contains(':') || (x.len() == 4 && x.starts_with("19")))
}

/// Facts of one parsed sentence: one per main predicate with a subject.
fn facts_of(w: &[Word], key: &str, n: usize, sentence: &str) -> Vec<Fact> {
    let mut out = Vec::new();
    for (p, pw) in w.iter().enumerate() {
        let main = pw.head == 0 || (matches!(pw.rel, Rel::Conj) && w.get(pw.head - 1).is_some_and(|h| h.head == 0));
        if !main || !matches!(pw.upos, UPos::VERB | UPos::ADJ | UPos::NOUN | UPos::NUM) {
            continue;
        }
        let kids: Vec<usize> = (0..w.len()).filter(|&i| w[i].head == p + 1).collect();
        let Some(&s) = kids.iter().find(|&&i| matches!(w[i].rel, Rel::Nsubj | Rel::NsubjPass)) else { continue };
        // what Mova sees about the candidate: she learns from a teacher which ones make good world sentences
        let finite = matches!(pw.tag, en::gram::Tag::VBD | en::gram::Tag::VBZ | en::gram::Tag::VBP) || kids.iter().any(|&k| matches!(w[k].rel, Rel::Aux | Rel::AuxPass | Rel::Cop));
        let dropped_clause = kids.iter().any(|&k| matches!(w[k].rel, Rel::Ccomp | Rel::Csubj | Rel::Dep | Rel::Parataxis | Rel::Advcl));
        let sk = match w[s].upos {
            UPos::PROPN => "name",
            UPos::NOUN => "noun",
            UPos::PRON if w[s].lemma.eq_ignore_ascii_case("she") => "she",
            UPos::PRON => "pronoun",
            _ => "other",
        };
        let sp = phrase(w, s);
        let bin = |n: usize, cuts: &[usize]| -> String { format!("{}", cuts.iter().filter(|c| n > **c).count()) };
        let mut attrs: BTreeMap<String, String> = BTreeMap::new();
        attrs.insert("finite predicate".into(), if finite { "yes" } else { "no" }.into());
        attrs.insert("clause dropped".into(), if dropped_clause { "yes" } else { "no" }.into());
        attrs.insert("subject".into(), sk.into());
        attrs.insert("subject length".into(), bin(sp.split_whitespace().count(), &[1, 3, 6]));
        attrs.insert("adverb in subject".into(), if w.iter().any(|x| x.head == s + 1 && x.upos == UPos::ADV) { "yes" } else { "no" }.into());
        attrs.insert("predicate".into(), format!("{:?}", pw.upos));
        attrs.insert("source starts upper".into(), if sentence.trim_start().chars().next().is_some_and(char::is_uppercase) { "yes" } else { "no" }.into());
        attrs.insert("source has quotes".into(), if sentence.contains(['"', '“', '”']) { "yes" } else { "no" }.into());
        attrs.insert("source length".into(), bin(w.len(), &[10, 20, 35]));
        let mut parts: Vec<(usize, String)> = vec![(s, sp)];
        let mut kept = vec![format!("subject «{}»", phrase(w, s))];
        // the predicate with auxiliaries, copula, negation, particle
        let mut pred = vec![p];
        for &k in &kids {
            if matches!(w[k].rel, Rel::Aux | Rel::AuxPass | Rel::Cop | Rel::CompoundPrt) || (w[k].rel == Rel::Advmod && matches!(w[k].lemma.to_lowercase().as_str(), "not" | "never" | "n't")) {
                pred.push(k);
            }
        }
        pred.sort();
        let pred_text = pred.iter().map(|&k| w[k].form.as_str()).collect::<Vec<_>>().join(" ");
        if pw.upos == UPos::VERB {
            parts.push((p, pred_text.clone()));
        } else {
            // a nominal or adjectival predicate: copula + the predicate phrase
            let cop: Vec<&str> = pred.iter().filter(|&&k| k != p).map(|&k| w[k].form.as_str()).collect();
            parts.push((p, format!("{} {}", cop.join(" "), phrase(w, p))));
        }
        kept.push(format!("predicate «{}»", parts.last().unwrap().1.trim()));
        let mut object = String::new();
        let mut time = None;
        let mut numbers = Vec::new();
        for &k in &kids {
            match w[k].rel {
                Rel::Obj | Rel::Iobj => {
                    let ph = phrase(w, k);
                    if w[k].rel == Rel::Obj {
                        object = ph.clone();
                    }
                    kept.push(format!("object «{ph}»"));
                    parts.push((k, ph));
                }
                Rel::Xcomp => {
                    // a controlled complement is kept whole ("devoted himself to warning people")
                    let ph = subtree(w, k);
                    kept.push(format!("complement «{ph}»"));
                    parts.push((k, ph));
                }
                Rel::Obl | Rel::OblTmod | Rel::OblUnmarked | Rel::OblAgent if w[k].upos != UPos::PRON => {
                    let ph = phrase(w, k);
                    if is_time(&ph) {
                        time = Some(ph.clone());
                    }
                    kept.push(format!("oblique «{ph}»"));
                    parts.push((k, ph));
                }
                _ => {}
            }
        }
        for (_, ph) in &parts {
            numbers.extend(ph.split_whitespace().filter(|t| t.chars().next().is_some_and(|c| c.is_ascii_digit())).map(|t| t.trim_matches(|c: char| !c.is_ascii_alphanumeric()).to_string()));
        }
        if parts.len() < 3 && object.is_empty() && pw.upos == UPos::VERB {
            // a bare "X sank" stays only with an oblique or when short
            if parts.len() < 2 {
                continue;
            }
        }
        parts.sort_by_key(|x| x.0);
        let mut text = parts.iter().map(|x| x.1.trim()).filter(|x| !x.is_empty()).collect::<Vec<_>>().join(" ");
        if let Some(c) = text.get(0..1) {
            text = c.to_uppercase() + &text[1..];
        }
        text = text.trim_end_matches([',', ';', ':']).to_string() + ".";
        if text.split_whitespace().count() < 3 {
            continue;
        }
        let kept_n = text.split_whitespace().count();
        attrs.insert("kept share".into(), format!("{}", [25usize, 50, 75].iter().filter(|c| kept_n * 100 / w.len().max(1) > **c).count()));
        attrs.insert("has object".into(), if object.is_empty() { "no" } else { "yes" }.into());
        attrs.insert("kind".into(), "compressed".into());
        // what the compression dropped from the predicate: the teacher's verdicts teach which losses hurt
        for (rel, name) in [(Rel::Conj, "dropped conjunct"), (Rel::Advcl, "dropped adverbial clause"), (Rel::Ccomp, "dropped clausal object"), (Rel::Advmod, "dropped adverb"), (Rel::Parataxis, "dropped parataxis")] {
            attrs.insert(name.into(), if kids.iter().any(|&k| w[k].rel == rel) { "yes" } else { "no" }.into());
        }
        attrs.insert("starts with".into(), w.first().map(|x| if matches!(x.upos, UPos::CCONJ | UPos::ADV | UPos::SCONJ) { "connective" } else if x.upos == UPos::PUNCT { "punctuation" } else { "word" }).unwrap_or("word").into());
        out.push(Fact {
            text,
            subject: w[s].form.clone(),
            predicate: pw.lemma.to_lowercase(),
            object: if object.is_empty() { String::new() } else { object.to_lowercase() },
            cites: vec![(key.to_string(), n, sentence.to_string())],
            kept: kept.join(", "),
            time,
            numbers,
            attrs,
            lemmas: Vec::new(),
            nouns: Vec::new(),
        });
    }
    out
}

/// Build the world: facts merged over all sources.
/// The sentences of a source: a testimony (`.turns.tsv`: seq, page, speaker, role, text) gives the answers and the
/// narrative with their speakers; any other text its body split into sentences.
fn source_sentences(src: &Source) -> Result<Vec<(String, String)>> {
    let t = std::fs::read_to_string(&src.path).with_context(|| src.path.clone())?;
    if src.path.ends_with(".turns.tsv") {
        let mut out = Vec::new();
        for l in t.lines().skip(1) {
            let c: Vec<&str> = l.splitn(5, '\t').collect();
            if c.len() == 5 && matches!(c[3], "A" | "N") {
                for s in crate::babi::split_sentences(c[4]).into_iter().filter(|s| (3..=60).contains(&s.split_whitespace().count())) {
                    out.push((s, c[2].trim().to_string()));
                }
            }
        }
        return Ok(out);
    }
    if src.path.ends_with(".turns.jsonl") {
        let mut out = Vec::new();
        for l in t.lines() {
            let Ok(v) = serde_json::from_str::<serde_json::Value>(l) else { continue };
            let speaker = v["speaker"].as_str().unwrap_or("").to_string();
            for s in crate::babi::split_sentences(v["text"].as_str().unwrap_or("")).into_iter().filter(|s| (3..=60).contains(&s.split_whitespace().count())) {
                out.push((s, speaker.clone()));
            }
        }
        return Ok(out);
    }
    Ok(crate::events::sentences(crate::events::book_body(&t)).into_iter().map(|s| (s, String::new())).collect())
}

/// Build the world: Mova's selection over all candidates.
pub fn build(topic: &str, sources: &[Source]) -> Result<Vec<Fact>> {
    let all = build_all(topic, sources)?;
    Ok(match read_selection_for(topic) {
        Some(l) => all.into_iter().filter(|f| crate::tabular::predict(&l, &f.attrs).0).collect(),
        None => all,
    })
}

/// All candidates: facts merged over all sources (sources read in parallel, merged in source order).
pub fn build_all(topic: &str, sources: &[Source]) -> Result<Vec<Fact>> {
    let words = topic_words(topic)?;
    let a = crate::tree::annotator()?;
    let learned = read_list(&compress_path());
    // token frequencies over the whole corpus (lowercase), for the language-free noise measure
    let freq: BTreeMap<String, u32> = {
        let next = std::sync::atomic::AtomicUsize::new(0);
        let total: std::sync::Mutex<BTreeMap<String, u32>> = std::sync::Mutex::new(BTreeMap::new());
        let threads = std::thread::available_parallelism().map(|x| x.get()).unwrap_or(4).min(10);
        std::thread::scope(|sc| {
            for _ in 0..threads {
                sc.spawn(|| {
                    let mut local: std::collections::HashMap<String, u32> = std::collections::HashMap::new();
                    loop {
                        let k = next.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                        let Some(src) = sources.get(k) else { break };
                        let Ok(t) = std::fs::read_to_string(&src.path) else { continue };
                        for tok in t.split_whitespace() {
                            let x = tok.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase();
                            if !x.is_empty() {
                                *local.entry(x).or_default() += 1;
                            }
                        }
                    }
                    let mut g = total.lock().unwrap();
                    for (k, v) in local {
                        *g.entry(k).or_default() += v;
                    }
                });
            }
        });
        total.into_inner().unwrap()
    };
    // how often each word appears in lower case: a word that almost never does is a name
    let lowerfreq: BTreeMap<String, u32> = {
        let next = std::sync::atomic::AtomicUsize::new(0);
        let total: std::sync::Mutex<BTreeMap<String, u32>> = std::sync::Mutex::new(BTreeMap::new());
        let threads = std::thread::available_parallelism().map(|x| x.get()).unwrap_or(4).min(10);
        std::thread::scope(|sc| {
            for _ in 0..threads {
                sc.spawn(|| {
                    let mut local: std::collections::HashMap<String, u32> = std::collections::HashMap::new();
                    loop {
                        let k = next.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                        let Some(src) = sources.get(k) else { break };
                        let Ok(t) = std::fs::read_to_string(&src.path) else { continue };
                        for tok in t.split_whitespace() {
                            let x = tok.trim_matches(|c: char| !c.is_alphanumeric());
                            if x.chars().next().is_some_and(char::is_lowercase) {
                                *local.entry(x.to_string()).or_default() += 1;
                            }
                        }
                    }
                    let mut g = total.lock().unwrap();
                    for (k, v) in local {
                        *g.entry(k).or_default() += v;
                    }
                });
            }
        });
        total.into_inner().unwrap()
    };
    let freq = &freq;
    let lowerfreq = &lowerfreq;
    let per_source: Vec<Vec<Fact>> = {
        let next = std::sync::atomic::AtomicUsize::new(0);
        let out: std::sync::Mutex<Vec<(usize, Vec<Fact>)>> = std::sync::Mutex::new(Vec::new());
        let threads = std::thread::available_parallelism().map(|x| x.get()).unwrap_or(4).min(10);
        std::thread::scope(|sc| {
            for _ in 0..threads {
                sc.spawn(|| loop {
                    let k = next.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    let Some(src) = sources.get(k) else { break };
                    let Ok(sents) = source_sentences(src) else { continue };
                    let mut facts = Vec::new();
                    for (n, (s, speaker)) in sents.iter().enumerate() {
                        let low = s.to_lowercase();
                        if !low.split(|c: char| !c.is_alphanumeric() && c != '\'' && c != '-').any(|x| words.contains(&x)) {
                            continue;
                        }
                        let clean = s.replace('_', "");
                        let forms: Vec<String> = a.tokenize(&clean).into_iter().map(|t| t.form).collect();
                        let mut w = a.annotate(&forms);
                        crate::rerank::repair_words(&mut w);
                        let mut found = match &learned {
                            Some(l) => learned_fact(l, &w, &src.key, n + 1, &clean).into_iter().collect::<Vec<_>>(),
                            None => facts_of(&w, &src.key, n + 1, &clean),
                        };
                        if let Some(f) = whole_fact(&w, &src.key, n + 1, &clean) {
                            found.push(f);
                        }
                        // scanned text: how much of the sentence looks like noise — tokens the whole corpus has seen fewer
                        // than twenty times (no lexicon of any language: OCR garbage is rare by nature)
                        let toks: Vec<String> = clean.split_whitespace().map(|t| t.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase()).filter(|t| !t.is_empty()).collect();
                        let rare = toks.iter().filter(|t| freq.get(t.as_str()).copied().unwrap_or(0) < 20).count();
                        let noise = format!("{}", [5usize, 12, 25].iter().filter(|c| rare * 100 / toks.len().max(1) > **c).count());
                        // character-level signs of a bad scan, language-free: letters mixed with digits inside a word,
                        // capitals in the middle of a word, odd symbols
                        let raw: Vec<&str> = clean.split_whitespace().collect();
                        let mixed = raw.iter().filter(|t| t.chars().any(|c| c.is_ascii_digit()) && t.chars().any(char::is_alphabetic)).count();
                        let midcap = raw.iter().filter(|t| t.chars().skip(1).any(char::is_uppercase) && t.chars().any(char::is_lowercase)).count();
                        let symbols = clean.chars().filter(|c| !c.is_alphanumeric() && !c.is_whitespace() && !",.;:!?'\"-()’“”".contains(*c)).count();
                        let bad_chars = format!("{}", [0usize, 1, 3].iter().filter(|c| mixed + midcap + symbols > **c).count());
                        // who the sentence is about: a named person, or people and things in general (the principles)
                        const NOT_PEOPLE: &[&str] = &["Titanic", "Carpathia", "Californian", "Olympic", "Lusitania", "Southampton", "Queenstown", "Cherbourg", "York", "Atlantic", "England", "America", "Halifax", "Harland", "Wolff", "White", "Star", "Line", "Senate", "Board", "Trade", "Britain", "Liverpool", "London", "God"];
                        const TITLES: &[&str] = &["Mr.", "Mrs.", "Miss", "Master", "Captain", "Colonel", "Col.", "Major", "Dr.", "Senator", "Officer", "Lord", "Lady", "Sir"];
                        let proper = |form: &str| form.chars().next().is_some_and(char::is_uppercase) && !NOT_PEOPLE.contains(&form) && {
                            let lower = form.to_lowercase();
                            let lc = lowerfreq.get(lower.as_str()).copied().unwrap_or(0);
                            let all_n = freq.get(lower.as_str()).copied().unwrap_or(0);
                            all_n >= 3 && lc * 10 < all_n
                        };
                        let person = w.iter().any(|x| matches!(x.rel, Rel::Nsubj | Rel::NsubjPass | Rel::Obj) && matches!(x.upos, UPos::PROPN | UPos::NOUN) && proper(&x.form)) || w.iter().enumerate().any(|(i, x)| (TITLES.contains(&x.form.as_str()) && w.get(i + 1).is_some_and(|y| y.form.chars().next().is_some_and(char::is_uppercase))) || (x.upos == UPos::PROPN && matches!(x.rel, Rel::Nsubj | Rel::NsubjPass) && !NOT_PEOPLE.contains(&x.form.as_str())));
                        let generic = w.iter().any(|x| matches!(x.rel, Rel::Nsubj | Rel::NsubjPass) && x.upos == UPos::NOUN && (x.tag == en::gram::Tag::NNS || matches!(x.lemma.to_lowercase().as_str(), "crew" | "steerage" | "class" | "ship" | "boat" | "rule" | "law" | "company")));
                        for f in found.iter_mut() {
                            f.attrs.insert("names a person".into(), if person { "yes" } else { "no" }.into());
                            f.attrs.insert("general subject".into(), if generic { "yes" } else { "no" }.into());
                            f.attrs.insert("scan damage".into(), bad_chars.clone());
                            f.attrs.insert("ocr noise".into(), noise.clone());
                            f.attrs.insert("source kind".into(), if src.path.ends_with(".turns.tsv") || src.path.ends_with(".turns.jsonl") { "testimony" } else if src.path.contains("newspaper") || src.path.contains("chronicling-america") { "newspaper" } else if src.path.contains("wikipedia") || src.path.contains("planetmath") { "encyclopedia" } else if src.path.contains("internet-archive") { "scanned book" } else { "book" }.into());
                            if !speaker.is_empty() {
                                f.kept = format!("{}; said by {speaker}", f.kept);
                            }
                        }
                        let content: Vec<String> = {
                            let mut v: Vec<String> = w.iter().filter(|x| matches!(x.upos, UPos::NOUN | UPos::VERB | UPos::ADJ | UPos::NUM) && !matches!(x.rel, Rel::Aux | Rel::Cop)).map(|x| x.lemma.to_lowercase()).filter(|l| l.len() > 2 && !matches!(l.as_str(), "be" | "have" | "do" | "say" | "make" | "get" | "go" | "come" | "take" | "see" | "know" | "think" | "one" | "two" | "other" | "same" | "such" | "many" | "much" | "time")).collect();
                            v.sort();
                            v.dedup();
                            v
                        };
                        let nouns: Vec<String> = w.iter().filter(|x| x.upos == UPos::NOUN).map(|x| x.lemma.to_lowercase()).filter(|l| content.contains(l)).collect();
                        for f in found.iter_mut() {
                            f.lemmas = content.clone();
                            f.nouns = nouns.clone();
                        }
                        facts.extend(found);
                    }
                    out.lock().unwrap().push((k, facts));
                });
            }
        });
        let mut v = out.into_inner().unwrap();
        v.sort_by_key(|x| x.0);
        v.into_iter().map(|x| x.1).collect()
    };
    let mut merged: BTreeMap<(String, String, String), Fact> = BTreeMap::new();
    for facts in per_source {
        for f in facts {
            let k = (f.subject.to_lowercase(), f.predicate.clone(), f.text.to_lowercase());
            match merged.get_mut(&k) {
                Some(m) => {
                    let c = f.cites[0].clone();
                    if !m.cites.iter().any(|x| x.0 == c.0 && x.1 == c.1) {
                        m.cites.push(c);
                    }
                }
                None => {
                    merged.insert(k, f);
                }
            }
        }
    }
    Ok(merged.into_values().collect())
}

// ---- Principles: what many sources say, not what one person did ---------------------------------------------------
//
// Sentences that name no person and are not a first-person story are grouped by what they talk about: every set of
// three content lemmas they share. A set that at least `min` different sources talk about is a principle; its
// sentence is the plainest member (an encyclopedia or a report first, a whole short sentence, the fewest words), and
// its footnotes list the sources that say it — the thought was drawn out of all of them.

pub fn principles(topic: &str, sources: &[Source], min: usize) -> Result<Vec<Fact>> {
    let all = build_all(topic, sources)?;
    let ok = |f: &Fact| {
        let a = |k: &str| f.attrs.get(k).map(String::as_str).unwrap_or("");
        a("names a person") != "yes" && a("first person") != "yes" && matches!(a("ocr noise"), "" | "0") && matches!(a("scan damage"), "" | "0" | "1") && a("subject") != "pronoun"
    };
    let idx: Vec<usize> = (0..all.len()).filter(|&i| ok(&all[i]) && all[i].lemmas.len() >= 3).collect();
    let mut groups: BTreeMap<(String, String, String), (std::collections::BTreeSet<String>, Vec<usize>)> = BTreeMap::new();
    for &i in &idx {
        let l: Vec<&String> = all[i].lemmas.iter().take(8).collect();
        for a in 0..l.len() {
            for b in a + 1..l.len() {
                for c in b + 1..l.len() {
                    let e = groups.entry((l[a].clone(), l[b].clone(), l[c].clone())).or_default();
                    e.0.insert(all[i].cites[0].0.clone());
                    e.1.push(i);
                }
            }
        }
    }
    // a principle is said by different kinds of sources (a newspaper and a testimony, a book and an encyclopedia)
    let kinds = |v: &[usize]| v.iter().filter_map(|&i| all[i].attrs.get("source kind").cloned()).collect::<std::collections::BTreeSet<_>>().len();
    let mut frequent: Vec<((String, String, String), (std::collections::BTreeSet<String>, Vec<usize>))> = groups.into_iter().filter(|(_, v)| v.0.len() >= min && kinds(&v.1) >= 2).collect();
    frequent.sort_by(|a, b| b.1 .0.len().cmp(&a.1 .0.len()).then(a.0.cmp(&b.0)));
    let rank = |f: &Fact| -> (u8, u8, usize) {
        let kind = match f.attrs.get("source kind").map(String::as_str) { Some("encyclopedia") => 0, Some("book") => 1, Some("testimony") => 2, Some("scanned book") => 3, _ => 4 };
        let whole = u8::from(f.attrs.get("kind").map(String::as_str) != Some("whole"));
        (kind, whole, f.text.split_whitespace().count())
    };
    let mut used: std::collections::BTreeSet<usize> = std::collections::BTreeSet::new();
    let mut out = Vec::new();
    for ((x, y, z), (srcs, members)) in frequent {
        // the sentence that stands for the group must be clean: a capital at the start, a full stop at the end, no
        // letters and digits mixed in a word, no word broken by a hyphen and a space, no scan damage
        let clean = |f: &Fact| {
            let t = f.text.trim_start_matches(['"', '“', ' ']);
            let last = f.text.trim_end_matches(['"', '”']).split_whitespace().last().unwrap_or("");
            t.chars().next().is_some_and(|c| c.is_uppercase()) && f.text.trim_end_matches(['"', '”']).ends_with(['.', '!'])
                && !["Mr.", "Mrs.", "Dr.", "No.", "St.", "Capt.", "Col.", "Messrs.", "Nos."].contains(&last)
                && !f.text.split_whitespace().any(|w| w.chars().any(|c| c.is_ascii_digit()) && w.chars().any(char::is_alphabetic))
                && !f.text.contains("- ") && f.attrs.get("scan damage").map(String::as_str) == Some("0") && f.attrs.get("ocr noise").map(String::as_str) == Some("0")
                // a scanned newspaper corroborates a principle but does not word it (its scan breaks words into
                // ordinary-looking pieces no frequency can catch)
                && !matches!(f.attrs.get("source kind").map(String::as_str), Some("newspaper" | "scanned book"))
                // a principle is worded by a whole source sentence, never by a compression
                && f.attrs.get("kind").map(String::as_str) == Some("whole")
                && stands_alone(&f.text)
                // formulas word nothing for a reader of English (LaTeX, or a PDF's broken display math)
                && !f.text.contains(['$', '\\', '=', '∈', '⊂', '→'])
        };
        let Some(&rep) = members.iter().filter(|&&i| !used.contains(&i) && clean(&all[i]) && (6..=35).contains(&all[i].text.split_whitespace().count())).min_by_key(|&&i| rank(&all[i])) else { continue };
        // one citation per source, the representative's first
        let mut cites = vec![all[rep].cites[0].clone()];
        for &i in &members {
            if cites.len() >= 8 {
                break;
            }
            let c = &all[i].cites[0];
            if !cites.iter().any(|d| d.0 == c.0) {
                cites.push(c.clone());
            }
        }
        for &i in &members {
            used.insert(i);
        }
        let mut f = all[rep].clone();
        f.cites = cites;
        f.kept = format!("said in {} sources — they share «{x}», «{y}», «{z}»", srcs.len());
        // the section: the group's most telling word (the rarest of the three in the corpus is too narrow; the
        // first noun-like one reads best)
        f.subject = [x.clone(), y.clone(), z.clone()].into_iter().find(|w| all[rep].nouns.contains(w)).unwrap_or_else(|| [x, y, z].into_iter().max_by_key(|w| w.len()).unwrap_or_default());
        out.push(f);
    }
    Ok(out)
}

/// A principle must stand alone: no opener pointing back to the sentence before («This», «Hence», «Similarly»), no
/// numbered example or page of its book («as in (33)», «(p. 275)»), no example announced, no pointer to its book's
/// chapters or to what was said before («Moreover», «in this case», «a later chapter»), no prayer or verse («Thee»).
pub fn stands_alone(text: &str) -> bool {
    let first = text.split_whitespace().next().unwrap_or("").trim_end_matches(',').to_lowercase();
    let back = ["this", "these", "that", "those", "such", "hence", "thus", "therefore", "similarly", "likewise", "finally", "specifically", "also", "however", "here", "below", "above", "example", "it", "they", "he", "she", "then"];
    let numbered = text.split('(').skip(1).any(|r| {
        let inner = r.split(')').next().unwrap_or("");
        let inner = inner.trim_start_matches("p. ").trim_start_matches("pp. ").trim_start_matches("ex. ");
        !inner.is_empty() && inner.chars().all(|c| c.is_ascii_digit() || matches!(c, 'a'..='e' | '–' | '-' | ','))
    });
    let verse = text.split(|c: char| !c.is_alphabetic()).any(|w| matches!(w, "Thee" | "Thou" | "Thy" | "Thine" | "thee" | "thou" | "thy"));
    let lower = text.to_lowercase();
    let pointing = ["moreover", "furthermore", "in that case", "in this case", "in this way", "chapter", "section", "of this", "as above", "as before", "here ", "below", "above", "part 2", "part 1"];
    !back.contains(&first.as_str()) && !numbered && !verse && !lower.contains("example") && !pointing.iter().any(|p| lower.contains(p))
}

/// Principles of one part of a world that go into the brain.
const BRAIN_PER_PART: usize = 12;

/// The world as a part of the brain: the world's principles under their parts, each with its first source and how
/// many sources say it (read back from the written world, so the brain shows the same parts as the book).
pub fn brain_body(world: &str) -> (String, usize) {
    // the footnotes: number → (the source's title, how many sources say it)
    let mut notes: BTreeMap<String, (String, String)> = BTreeMap::new();
    for l in world.lines().filter(|l| l.starts_with("[^")) {
        let (n, rest) = l.split_once("]: ").unwrap_or(("", ""));
        let title = rest.split('*').nth(1).unwrap_or("").to_string();
        let said = rest.split("said in ").nth(1).and_then(|r| r.split(' ').next()).unwrap_or("").to_string();
        notes.insert(n.trim_start_matches("[^").to_string(), (title, said));
    }
    let mut md = String::new();
    let mut part = String::new();
    let (mut n, mut in_part) = (0, 0);
    for l in world.lines().skip_while(|l| !l.starts_with("## ")) {
        if let Some(h) = l.strip_prefix("## ") {
            if h == "Sources" {
                break;
            }
            part = h.to_string();
            continue;
        }
        let Some((text, refs)) = l.split_once(" [^") else { continue };
        let (title, said) = notes.get(refs.split(']').next().unwrap_or("")).cloned().unwrap_or_default();
        if !part.is_empty() {
            md += &format!("\n### {}\n\n", part);
            part.clear();
            in_part = 0;
        }
        n += 1;
        // the brain keeps the best-supported principles of each part (the world book keeps them all)
        in_part += 1;
        if in_part > BRAIN_PER_PART {
            continue;
        }
        md += &format!("{text} ({title}{})\n", if said.is_empty() { String::new() } else { format!("; {said} sources") });
    }
    (md.trim().to_string(), n)
}

/// A world gathered by a member from the material it is given (the sources of a topic): the principles many sources
/// share, written as a world book in `dir`, returned as a brain part.
pub fn gather(topic: &str, sources_tsv: &Path, dir: &Path) -> Result<(String, usize)> {
    let sources = read_sources(sources_tsv)?;
    let facts = principles(topic, &sources, 5)?;
    write(topic, &facts, &sources, dir, RELEASE, None, None)?;
    let world = std::fs::read_to_string(dir.join(format!("{topic}.world.md")))?;
    Ok(brain_body(&world))
}

/// `world worldbook-assemble <title line> <parts dir> <out.md>`: one manual from its parts (`*.md` in name order,
/// each a run of «## » parts), dense: a line said before (the same words in the same order, case and punctuation
/// aside) is said once, a part named before continues under its first heading, empty parts go.
pub fn assemble_args(args: &[String]) -> Result<()> {
    let usage = "usage: world worldbook-assemble <title line> <parts dir> <out.md>";
    let title = args.get(1).context(usage)?;
    let dir = Path::new(args.get(2).context(usage)?);
    let out = Path::new(args.get(3).context(usage)?);
    let mut files: Vec<std::path::PathBuf> = std::fs::read_dir(dir)?.filter_map(|e| e.ok().map(|e| e.path())).filter(|p| p.extension().is_some_and(|x| x == "md")).collect();
    files.sort();
    let norm = |l: &str| l.to_lowercase().chars().filter(|c| c.is_alphanumeric() || *c == ' ').collect::<String>().split_whitespace().collect::<Vec<_>>().join(" ");
    let mut seen = std::collections::BTreeSet::new();
    let mut parts: Vec<(String, Vec<String>)> = Vec::new();
    let mut preface: Vec<String> = Vec::new();
    let (mut lines, mut dropped) = (0, 0);
    // the internal kitchen (`--private-out <file> --private-words a,b,c`): a line naming any of the words goes to the
    // private manual, kept under the same part headings, and never into the public one
    let opt = |k: &str| args.iter().position(|a| a == k).and_then(|i| args.get(i + 1)).cloned();
    let private_words: Vec<String> = opt("--private-words").map(|w| w.split(',').map(|x| x.trim().to_lowercase()).filter(|x| !x.is_empty()).collect()).unwrap_or_default();
    let is_private = |l: &str| {
        let low = l.to_lowercase();
        private_words.iter().any(|w| low.match_indices(w.as_str()).any(|(i, _)| {
            let before = low[..i].chars().next_back().is_none_or(|c| !c.is_alphanumeric());
            let after = low[i + w.len()..].chars().next().is_none_or(|c| !c.is_alphanumeric());
            before && after
        }))
    };
    let mut private_parts: Vec<(String, Vec<String>)> = Vec::new();
    for f in &files {
        let mut cur: Option<usize> = None;
        let mut code = false;
        for l in std::fs::read_to_string(f)?.lines() {
            // fenced code is kept whole, line for line (a «}» in every example is no repeat)
            if l.trim_start().starts_with("```") || code {
                if l.trim_start().starts_with("```") {
                    code = !code;
                }
                if let Some(i) = cur {
                    parts[i].1.push(l.to_string());
                }
                continue;
            }
            if let Some(h) = l.strip_prefix("## ") {
                let key = norm(h);
                cur = Some(parts.iter().position(|(n, _)| norm(n) == key).unwrap_or_else(|| {
                    parts.push((h.trim().to_string(), Vec::new()));
                    parts.len() - 1
                }));
                continue;
            }
            if l.starts_with("# ") || l.trim().is_empty() {
                continue;
            }
            if !seen.insert(norm(l)) {
                dropped += 1;
                continue;
            }
            lines += 1;
            // a line under a private heading is private as a whole
            let private_part = cur.is_some_and(|i| is_private(&parts[i].0));
            if !private_words.is_empty() && (private_part || is_private(l)) {
                let h = cur.map(|i| parts[i].0.clone()).unwrap_or_else(|| "Overview".into());
                match private_parts.iter_mut().find(|(n, _)| *n == h) {
                    Some(p) => p.1.push(l.to_string()),
                    None => private_parts.push((h, vec![l.to_string()])),
                }
                continue;
            }
            match cur {
                Some(i) => parts[i].1.push(l.to_string()),
                None => preface.push(l.to_string()),
            }
        }
    }
    // the part on how Mova uses the world closes the manual
    parts.sort_by_key(|(n, _)| n.starts_with("How "));
    let mut md = format!("{title}\n\n{}\n", preface.join("\n"));
    for (n, ls) in parts.iter().filter(|(_, ls)| !ls.is_empty()) {
        md += &format!("\n## {n}\n\n{}\n", ls.join("\n"));
    }
    std::fs::write(out, &md)?;
    println!("{} → {} files, {} parts, {lines} lines ({dropped} repeated lines dropped), {} words", out.display(), files.len(), parts.len(), md.split_whitespace().count());
    if let Some(po) = opt("--private-out") {
        let mut pm = String::from("# The internal kitchen (closed build only)\n\nWhat the public Mova does not tell: the internal system, its siblings, its costs and its private arrangements. Built only with the cargo feature «tera»; never exported.\n");
        for (n, ls) in &private_parts {
            pm += &format!("\n## {n}\n\n{}\n", ls.join("\n"));
        }
        std::fs::create_dir_all(Path::new(&po).parent().unwrap_or(Path::new(".")))?;
        std::fs::write(&po, &pm)?;
        println!("{po} → {} private lines in {} parts", private_parts.iter().map(|p| p.1.len()).sum::<usize>(), private_parts.len());
    }
    Ok(())
}

// ---- Perception: how the small ones see a world ------------------------------------------------------------------
//
// A world is read the way the small ones read: every sentence through the story reader (`babi::Story`, the same one the
// family members hear with) and through the parser. The report says what share is understood, why the rest is not
// (the reasons grouped, with examples), which names, numbers with units and verbs the world talks about, and which
// sentences have no verb — the places where the skeleton needs eyes (recognisers) and hands (operations).

/// What one world looks like to the reader.
pub struct Perception {
    pub sentences: usize,
    pub understood: usize,
    /// sentences the story reader misses but the hands read as general knowledge (`know`)
    pub as_knowledge: usize,
    /// reason → (count, up to three example sentences)
    pub gaps: BTreeMap<String, (usize, Vec<String>)>,
    pub names: BTreeMap<String, usize>,
    pub units: BTreeMap<String, usize>,
    pub verbs: BTreeMap<String, usize>,
    pub verbless: usize,
}

pub fn perceive(text: &str, max: usize) -> Result<Perception> {
    let lines: Vec<&str> = text.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')).take(max).collect();
    let mut st = crate::babi::Story::default();
    let mut p = Perception { sentences: 0, understood: 0, as_knowledge: 0, gaps: BTreeMap::new(), names: BTreeMap::new(), units: BTreeMap::new(), verbs: BTreeMap::new(), verbless: 0 };
    let (ok, gaps) = crate::mind::tell(&mut st, &lines.join("\n"));
    p.understood = ok;
    p.sentences = ok + gaps.len();
    // what the story reader misses, the hands for general statements may still read as knowledge
    let gap_sents: Vec<String> = gaps.iter().map(|g| g.0.clone()).collect();
    let known: std::collections::BTreeSet<String> = crate::know::facts_of_all(&gap_sents).into_iter().map(|f| f.source).collect();
    p.as_knowledge = known.len();
    let gaps: Vec<(String, String)> = gaps.into_iter().filter(|g| !known.contains(&g.0)).collect();
    for (sent, why) in gaps {
        // the reason without its sentence-specific tail («no verb class for "orbit"» → «no verb class»)
        let mut key = String::new();
        let mut quoted = false;
        for c in why.chars() {
            match c {
                '«' | '“' => {
                    quoted = true;
                    key.push('…');
                }
                '»' | '”' => quoted = false,
                '"' => {
                    if !quoted {
                        key.push('…');
                    }
                    quoted = !quoted;
                }
                _ if !quoted => key.push(c),
                _ => {}
            }
        }
        let key: String = key.split(':').next().unwrap_or("").trim().chars().take(70).collect();
        let e = p.gaps.entry(key).or_default();
        e.0 += 1;
        if e.1.len() < 3 {
            e.1.push(sent);
        }
    }
    let a = crate::tree::annotator()?;
    for line in &lines {
        for sent in crate::babi::split_sentences(line) {
            let forms: Vec<String> = a.tokenize(&sent).into_iter().map(|t| t.form).collect();
            if forms.is_empty() {
                continue;
            }
            let mut ws = a.annotate(&forms);
            crate::rerank::repair_words(&mut ws);
            let mut has_verb = false;
            for (i, w) in ws.iter().enumerate() {
                match w.upos {
                    en::gram::UPos::PROPN => *p.names.entry(w.form.clone()).or_default() += 1,
                    en::gram::UPos::VERB => {
                        has_verb = true;
                        *p.verbs.entry(w.lemma.to_lowercase()).or_default() += 1;
                    }
                    en::gram::UPos::AUX => has_verb = true,
                    en::gram::UPos::NUM => {
                        if let Some(n) = ws.get(i + 1).filter(|n| n.upos == en::gram::UPos::NOUN) {
                            *p.units.entry(n.lemma.to_lowercase()).or_default() += 1;
                        }
                    }
                    _ => {}
                }
            }
            if !has_verb {
                p.verbless += 1;
            }
        }
    }
    Ok(p)
}

fn top(m: &BTreeMap<String, usize>, k: usize) -> String {
    let mut v: Vec<(&String, &usize)> = m.iter().collect();
    v.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    v.into_iter().take(k).map(|(w, n)| format!("{w} {n}")).collect::<Vec<_>>().join(", ")
}

/// `world worldbook-perceive <out dir> <name=file.md>... [--max N]`: a report per world and a summary table.
pub fn perceive_args(args: &[String]) -> Result<()> {
    let usage = "usage: world worldbook-perceive <out dir> <name=file.md>... [--max N]";
    let max: usize = args.iter().position(|a| a == "--max").and_then(|i| args.get(i + 1)).and_then(|x| x.parse().ok()).unwrap_or(usize::MAX);
    let out = Path::new(args.get(1).context(usage)?);
    std::fs::create_dir_all(out)?;
    let mut summary = String::from("world\tsentences\tunderstood %\tknowledge %\tverbless %\tnames\tmain gap\n");
    for spec in args.iter().skip(2).filter(|a| a.contains('=')) {
        let (name, file) = spec.split_once('=').unwrap();
        let t0 = std::time::Instant::now();
        let p = perceive(&std::fs::read_to_string(file)?, max)?;
        let pct = |a: usize| 100.0 * a as f64 / p.sentences.max(1) as f64;
        let mut gaps: Vec<(&String, &(usize, Vec<String>))> = p.gaps.iter().collect();
        gaps.sort_by(|a, b| b.1 .0.cmp(&a.1 .0).then(a.0.cmp(b.0)));
        let mut md = format!("# How the small ones see the world of {name}\n\n{} sentences: {:.1}% understood by the story reader, {:.1}% more read as general knowledge, {:.1}% without a verb. Read in {:.0} s.\n\n## Why the rest is not understood\n\n", p.sentences, pct(p.understood), pct(p.as_knowledge), pct(p.verbless), t0.elapsed().as_secs_f64());
        for (why, (n, ex)) in gaps.iter().take(25) {
            md += &format!("- {why} — {n} ({:.1}%): «{}»\n", pct(*n), ex.join("» | «"));
        }
        md += &format!("\n## Names\n\n{}\n\n## Numbers with units\n\n{}\n\n## Verbs\n\n{}\n", top(&p.names, 60), top(&p.units, 60), top(&p.verbs, 80));
        std::fs::write(out.join(format!("{name}.md")), md)?;
        summary += &format!("{name}\t{}\t{:.1}\t{:.1}\t{:.1}\t{}\t{}\n", p.sentences, pct(p.understood), pct(p.as_knowledge), pct(p.verbless), p.names.len(), gaps.first().map(|g| format!("{} ({})", g.0, g.1 .0)).unwrap_or_default());
        println!("{name}: {} sentences, {:.1}% understood, +{:.1}% as knowledge, {:.1}% verbless, {:.0} s", p.sentences, pct(p.understood), pct(p.as_knowledge), pct(p.verbless), t0.elapsed().as_secs_f64());
    }
    std::fs::write(out.join("summary.tsv"), summary)?;
    Ok(())
}

// ---- The site: the central worlds as pages ---------------------------------------------------------------------

fn esc(t: &str) -> String {
    t.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

const SITE_CSS: &str = ":root{--bg:#f7f5f0;--ink:#1c1d21;--muted:#62646c;--line:#e3dfd6;--accent:#0f766e;--panel:#fff;--bar:#cfe9e5}
@media (prefers-color-scheme:dark){:root{--bg:#141518;--ink:#ebeae6;--muted:#a1a1aa;--line:#2e3036;--accent:#2dd4bf;--panel:#1c1d21;--bar:#134e4a}}
body{margin:0;background:var(--bg);color:var(--ink);font:16px/1.6 system-ui,-apple-system,\"Segoe UI\",Roboto,sans-serif}
.wrap{max-width:1080px;margin:0 auto;padding:40px 16px}a{color:var(--accent);text-decoration:none}a:hover{text-decoration:underline}
h1{margin:0 0 .2em}h2{margin:2em 0 .5em;padding-top:.5em;border-top:1px solid var(--line)}p.lead,.muted{color:var(--muted)}
.stats{display:flex;gap:12px;flex-wrap:wrap;margin:20px 0}.stat{background:var(--panel);border:1px solid var(--line);border-radius:12px;padding:10px 16px}
.stat b{display:block;font-size:1.5em}.grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(250px,1fr));gap:12px}
.card{background:var(--panel);border:1px solid var(--line);border-radius:14px;padding:14px 16px;display:flex;flex-direction:column;gap:6px}
.card h3{margin:0;font-size:1.05em}.card small{color:var(--muted)}.bar{height:6px;background:var(--line);border-radius:3px;overflow:hidden}
.bar i{display:block;height:100%;background:var(--accent)}.toc{columns:3 220px;font-size:.92em;background:var(--panel);border:1px solid var(--line);border-radius:12px;padding:14px 18px}
.toc a{display:block}ul.s{padding-left:1.1em}ul.s li{margin:.15em 0}.cite{color:var(--muted);font-size:.85em}
pre{background:var(--panel);border:1px solid var(--line);border-radius:10px;padding:10px 14px;overflow:auto;font-size:.88em}
.tag{display:inline-block;font-size:.75em;padding:1px 8px;border-radius:9px;background:var(--bar);color:var(--ink)}";

fn page(title: &str, body: &str) -> String {
    format!("<!doctype html>\n<html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n<meta name=\"robots\" content=\"noindex, nofollow, noarchive\"><title>{} — Mova</title>\n<style>{SITE_CSS}</style></head>\n<body><div class=\"wrap\">\n{body}\n</div></body></html>\n", esc(title))
}

/// A sentence with its bracketed citations set apart.
fn line_html(l: &str) -> String {
    let mut out = String::new();
    let mut rest = l;
    while let Some(i) = rest.find(" [") {
        let Some(j) = rest[i..].find(']') else { break };
        out += &esc(&rest[..i]);
        out += &format!(" <span class=\"cite\">{}</span>", esc(&rest[i + 1..i + j + 1]));
        rest = &rest[i + j + 1..];
    }
    out + &esc(rest)
}

/// `world worldbook-site <out dir>`: the central worlds as pages, with what the small ones gathered and derived.
pub fn site_args(args: &[String]) -> Result<()> {
    let out = Path::new(args.get(1).context("usage: world worldbook-site <out dir>")?);
    std::fs::create_dir_all(out.join("gathered"))?;
    let hot = crate::state::root();
    // the newest gathered world book of each topic
    let mut gathered: BTreeMap<String, (std::time::SystemTime, std::path::PathBuf)> = BTreeMap::new();
    // shared/<topic>-<key>/ (gathered once, shared) and <state>/<topic>/ (gathered before the cache)
    let mut dirs: Vec<(String, std::path::PathBuf)> = Vec::new();
    for top in std::fs::read_dir(hot.join("worlds")).map(|r| r.flatten().map(|e| e.path()).collect::<Vec<_>>()).unwrap_or_default() {
        let shared = top.file_name().is_some_and(|n| n == "shared");
        for e in std::fs::read_dir(&top).map(|r| r.flatten().collect::<Vec<_>>()).unwrap_or_default() {
            let name = e.file_name().to_string_lossy().to_string();
            let topic = if shared { name.rsplit_once('-').map(|x| x.0.to_string()).unwrap_or(name.clone()) } else { name.clone() };
            dirs.push((topic, e.path()));
        }
    }
    dirs.sort();
    {
        for (topic, dir) in dirs {
            let f = dir.join(format!("{topic}.world.md"));
            if let Ok(m) = std::fs::metadata(&f).and_then(|m| m.modified()) {
                if gathered.get(&topic).is_none_or(|g| g.0 < m) {
                    gathered.insert(topic, (m, f));
                }
            }
        }
    }
    let max_kb = crate::state::CENTRAL_MANUALS.iter().map(|m| m.2.len()).max().unwrap_or(1);
    let (mut total_kb, mut total_words, mut total_sent) = (0, 0, 0);
    let mut cards = String::new();
    for (id, name, manual) in crate::state::CENTRAL_MANUALS.iter() {
        let sents = crate::nerves::sentences_of(manual);
        let parts: Vec<&str> = manual.lines().filter_map(|l| l.strip_prefix("## ")).collect();
        let words = manual.split_whitespace().count();
        total_kb += manual.len() / 1024;
        total_words += words;
        total_sent += sents.len();
        // the page of the world
        let mut body = format!("<p><a href=\"./\">← all worlds</a></p><h1>The world of {}</h1><p class=\"lead\">{} KB · {} parts · {} sentences · {} words. The manual Mova was given; each sentence one thought, cited where it comes from a work.</p>", esc(name), manual.len() / 1024, parts.len(), sents.len(), words);
        if let Some((_, g)) = gathered.get(*id) {
            let dest = out.join("gathered").join(format!("{id}.world.md"));
            std::fs::copy(g, &dest)?;
            body += &format!("<p><span class=\"tag\">gathered by the small ones</span> <a href=\"gathered/{id}.world.md\">the world they gathered from the corpus</a> ({} KB)</p>", std::fs::metadata(&dest)?.len() / 1024);
        }
        body += "<div class=\"toc\">";
        for (i, p) in parts.iter().enumerate() {
            body += &format!("<a href=\"#p{i}\">{}</a>", esc(p));
        }
        body += "</div>";
        let mut pi = 0;
        let mut open = false;
        let mut code = false;
        for raw in manual.lines() {
            let l = raw.trim();
            if l.starts_with("```") {
                body += if code { "</pre>" } else { "<pre>" };
                code = !code;
                continue;
            }
            if code {
                body += &format!("{}\n", esc(raw));
                continue;
            }
            if let Some(h) = l.strip_prefix("## ") {
                if open {
                    body += "</ul>";
                }
                body += &format!("<h2 id=\"p{pi}\">{}</h2><ul class=\"s\">", esc(h));
                pi += 1;
                open = true;
            } else if !l.is_empty() && !l.starts_with('#') && open {
                body += &format!("<li>{}</li>", line_html(l));
            }
        }
        if open {
            body += "</ul>";
        }
        std::fs::write(out.join(format!("{id}.html")), page(&format!("The world of {name}"), &body))?;
        cards += &format!(
            "<div class=\"card\"><h3><a href=\"{id}.html\">{}</a></h3><small>{} KB · {} parts · {} sentences</small><div class=\"bar\"><i style=\"width:{}%\"></i></div>{}</div>",
            esc(name),
            manual.len() / 1024,
            parts.len(),
            sents.len(),
            (manual.len() * 100 / max_kb).max(2),
            if gathered.contains_key(*id) { "<small><span class=\"tag\">+ gathered world</span></small>" } else { "" }
        );
    }
    let derived = hot.join("worlds").join("derived").join("derived.world.md");
    let derived_link = if derived.exists() {
        std::fs::copy(&derived, out.join("gathered").join("derived.world.md"))?;
        "<p><span class=\"tag\">derived</span> <a href=\"gathered/derived.world.md\">What I derived</a> — the principles the small ones derived themselves and their teacher found true.</p>"
    } else {
        ""
    };
    let body = format!(
        "<p><a href=\"../\">← Mova</a></p><h1>The central worlds of Mova</h1><p class=\"lead\">What every brain of the small ones is given: dense English manuals, one thought a line. From them the small ones read facts, derive new ones and check what is absurd; from the open corpora they gather their own bigger worlds.</p><div class=\"stats\"><div class=\"stat\"><b>{}</b>worlds</div><div class=\"stat\"><b>{:.1} MB</b>manuals</div><div class=\"stat\"><b>{}</b>sentences</div><div class=\"stat\"><b>{}</b>words</div><div class=\"stat\"><b>{}</b>gathered worlds</div></div>{derived_link}<div class=\"grid\">{cards}</div>",
        crate::state::CENTRAL_MANUALS.len(),
        total_kb as f64 / 1024.0,
        total_sent,
        total_words,
        gathered.len()
    );
    std::fs::write(out.join("index.html"), page("The central worlds of Mova", &body))?;
    println!("{} → {} worlds, {:.1} MB of manuals, {} gathered worlds", out.display(), crate::state::CENTRAL_MANUALS.len(), total_kb as f64 / 1024.0, gathered.len());
    Ok(())
}

/// `world worldbook-brain <world.md>`: print a written world as a part of the brain.
pub fn brain_args(args: &[String]) -> Result<()> {
    let world = std::fs::read_to_string(args.get(1).context("usage: world worldbook-brain <world.md>")?)?;
    let (body, n) = brain_body(&world);
    println!("{body}");
    eprintln!("{n} principles");
    Ok(())
}

/// `world worldbook-principles <topic> <sources.tsv> <out dir> [--min-sources 3] [--release R]`.
pub fn principles_args(args: &[String]) -> Result<()> {
    let usage = "usage: world worldbook-principles <topic> <sources.tsv> <out dir> [--min-sources 3] [--release R] [--full-link URL]";
    let opt = |k: &str| args.iter().position(|a| a == k).and_then(|i| args.get(i + 1)).cloned();
    let topic = args.get(1).context(usage)?;
    let sources = read_sources(Path::new(args.get(2).context(usage)?))?;
    let out = Path::new(args.get(3).context(usage)?);
    let min: usize = opt("--min-sources").and_then(|x| x.parse().ok()).unwrap_or(5);
    let facts = principles(topic, &sources, min)?;
    let version = opt("--release").unwrap_or_else(|| RELEASE.into());
    write(topic, &facts, &sources, out, &version, None, None)?;
    Ok(())
}

fn selection_path() -> std::path::PathBuf {
    std::env::var("MOVA_HOT_ROOT").map(std::path::PathBuf::from).unwrap_or_else(|_| std::path::PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| format!("{}/mova-data", std::env::var("HOME").unwrap_or_default()))).join("hot")).join("worldbook-select.tsv")
}

/// A topic's own selection (`worldbook-select-<topic>.tsv`); a topic without one starts from the shared one (what
/// another world learned — the worlds copy from each other).
fn topic_selection_path(topic: &str) -> std::path::PathBuf {
    selection_path().with_file_name(format!("worldbook-select-{topic}.tsv"))
}

fn read_selection_for(topic: &str) -> Option<crate::tabular::List> {
    let own = topic_selection_path(topic);
    let shared = selection_path();
    read_list(if own.exists() { &own } else { &shared })
}

/// The learned selection: `attr=value&attr=value<TAB>yes|no<TAB>good<TAB>bad` lines, `*<TAB>yes|no` the default.
#[allow(dead_code)]
fn read_selection() -> Option<crate::tabular::List> {
    let t = std::fs::read_to_string(selection_path()).ok()?;
    let mut rules = Vec::new();
    let mut default = true;
    for l in t.lines().filter(|l| !l.starts_with('#')) {
        let c: Vec<&str> = l.split('\t').collect();
        if c.len() < 2 {
            continue;
        }
        if c[0] == "*" {
            default = c[1] == "yes";
            continue;
        }
        let cond: Vec<(String, String)> = c[0].split('&').filter_map(|x| x.split_once('=')).map(|(a, b)| (a.to_string(), b.to_string())).collect();
        rules.push((cond, c[1] == "yes", c.get(2).and_then(|x| x.parse().ok()).unwrap_or(0), c.get(3).and_then(|x| x.parse().ok()).unwrap_or(0)));
    }
    Some(crate::tabular::List { rules, default })
}

/// `world worldbook-teach <topic> <sources.tsv> <n>`: the teacher (Opus) judges `n` candidates — a good world
/// sentence is grammatical English that says only what its source sentence says —, Mova learns her selection from
/// the judgments (a decision list, settings chosen by 10-fold cross-validation) and keeps it in the hot layer.
pub fn teach(topic: &str, sources: &[Source], n: usize) -> Result<()> {
    let judge_only = std::env::var("WORLDBOOK_JUDGE_ONLY").is_ok();
    let sel = if topic == "titanic" { selection_path() } else { topic_selection_path(topic) };
    if topic.ends_with("-principles") && !sel.exists() {
        // a principles world starts empty-handed: no person-centred selection is borrowed
        std::fs::write(&sel, "*\tyes\n")?;
    }
    let saved = std::fs::read_to_string(&sel).ok();
    if !judge_only {
        let _ = std::fs::remove_file(&sel);
    }
    let cands = build(topic, sources);
    if let Some(t) = saved {
        std::fs::write(&sel, t)?;
    }
    let cands = cands?;
    let step = (cands.len() / n.max(1)).max(1);
    let sample: Vec<&Fact> = cands.iter().step_by(step).take(n).collect();
    let dir = std::path::PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| format!("{}/mova-data", std::env::var("HOME").unwrap_or_default()))).join("runs/worldbook-teach");
    let opus = prag::opus::Opus::from_env(dir.join("cwd"));
    let mut labels: Vec<(BTreeMap<String, String>, bool)> = Vec::new();
    let mut log = String::new();
    for (bi, batch) in sample.chunks(25).enumerate() {
        let mut prompt = if topic == "time" {
            String::from("For each item you get a SOURCE sentence and a CANDIDATE sentence made from it (the same or with words dropped). Answer «good» only if the candidate (1) is grammatical English, (2) says only what the source says, and (3) states a general fact about time: order, before and after, cause and effect, duration, clocks, calendars, tense and aspect in language, how people perceive time, or time in computing (commands, clock cycles, waiting, parallel work). A sentence about something else, or a story of a particular person, is «bad». Answer only with lines «<number> good|bad».\n\n")
        } else if topic == "titanic" || topic.ends_with("-principles") {
            String::from("For each item you get a SOURCE sentence and a CANDIDATE sentence made from it (the same or with words dropped). Answer «good» only if the candidate (1) is grammatical English, (2) says only what the source says, and (3) states a general principle or a general fact about the event — about people, groups, classes, the crew, the boats, the ship, rules or what usually happened — NOT about a specific named person. A sentence about a named person, or a personal first-person story, is «bad». Answer only with lines «<number> good|bad».\n\n")
        } else {
            String::from("For each item you get a SOURCE sentence and a CANDIDATE sentence made from it by dropping words. Answer «good» if the candidate is grammatical English and says only what the source says (no changed meaning, nothing missing that makes it false or odd); otherwise «bad». Answer only with lines «<number> good|bad».\n\n")
        };
        for (k, f) in batch.iter().enumerate() {
            prompt += &format!("{}. SOURCE: {}\n   CANDIDATE: {}\n", k + 1, f.cites[0].2, f.text);
        }
        let (ans, call) = opus.ask("worldbook-teach", bi, batch.len(), &prompt)?;
        prag::opus::append_call(&dir.join("calls.jsonl"), &call)?;
        for l in ans.lines() {
            let mut it = l.split_whitespace();
            let (Some(k), Some(v)) = (it.next().and_then(|x| x.trim_end_matches('.').parse::<usize>().ok()), it.next()) else { continue };
            if let Some(f) = batch.get(k.wrapping_sub(1)) {
                let good = v.eq_ignore_ascii_case("good");
                labels.push((f.attrs.clone(), good));
                log += &format!("{}\t{}\t{}\n", if good { "good" } else { "bad" }, f.text, f.cites[0].2);
            }
        }
    }
    std::fs::create_dir_all(&dir)?;
    std::fs::write(dir.join(format!("{topic}-labels.tsv")), &log)?;
    // the teacher's verdicts accumulate across runs (evidence is not thrown away): every earlier verdict whose
    // candidate still exists joins the training data, with the candidate's current attributes
    let all_path = dir.join(format!("{topic}-verdicts.tsv"));
    let mut verdicts: BTreeMap<String, bool> = std::fs::read_to_string(&all_path).unwrap_or_default().lines().filter_map(|l| {
        let (v, t) = l.split_once('\t')?;
        Some((t.split('\t').next().unwrap_or("").to_string(), v == "good"))
    }).collect();
    for l in log.lines() {
        if let Some((v, rest)) = l.split_once('\t') {
            verdicts.insert(rest.split('\t').next().unwrap_or("").to_string(), v == "good");
        }
    }
    std::fs::write(&all_path, verdicts.iter().map(|(t, g)| format!("{}\t{t}\n", if *g { "good" } else { "bad" })).collect::<String>())?;
    if !judge_only {
        let by_text: BTreeMap<&str, &Fact> = cands.iter().map(|f| (f.text.as_str(), f)).collect();
        labels = verdicts.iter().filter_map(|(t, g)| by_text.get(t.as_str()).map(|f| (f.attrs.clone(), *g))).collect();
        println!("verdicts accumulated: {} in all, {} match today's candidates", verdicts.len(), labels.len());
    }
    let good = labels.iter().filter(|x| x.1).count();
    println!("teacher: {} judged, {good} good, {} bad", labels.len(), labels.len() - good);
    if judge_only {
        // the world as it is (with Mova's selection), judged on a fresh sample: nothing is relearned
        println!("the world's sentences: {:.1}% good by the teacher", 100.0 * good as f64 / labels.len().max(1) as f64);
        return Ok(());
    }
    // Mova learns: settings by 10-fold cross-validation, scored by accuracy
    let mut best = (0usize, 5usize, 1usize);
    for k in 1..=2 {
        for min in [3usize, 5, 10] {
            let mut ok = 0;
            for fold in 0..10 {
                let tr: Vec<_> = labels.iter().enumerate().filter(|(i, _)| i % 10 != fold).map(|(_, x)| x.clone()).collect();
                let l = crate::tabular::learn_k(&tr, min, k);
                ok += labels.iter().enumerate().filter(|(i, _)| i % 10 == fold).filter(|(_, x)| crate::tabular::predict(&l, &x.0).0 == x.1).count();
            }
            if ok > best.0 {
                best = (ok, min, k);
            }
        }
    }
    let mut l = crate::tabular::learn_k(&labels, best.1, best.2);
    if labels.iter().filter(|x| crate::tabular::predict(&l, &x.0).0).count() == 0 {
        // good sentences are rare here: keep only what Mova is sure of (precision ≥ 0.6), drop the rest
        l = crate::tabular::learn_rare(&labels, 5, 2, 0.6);
        println!("good sentences are rare: Mova keeps only what she is sure of (precision ≥ 0.6)");
    }
    let kept: Vec<&(BTreeMap<String, String>, bool)> = labels.iter().filter(|x| crate::tabular::predict(&l, &x.0).0).collect();
    println!("Mova's selection: CV accuracy {:.1}% (min {}, up to {} attribute(s)); on the judged set it keeps {} of which {} good", 100.0 * best.0 as f64 / labels.len().max(1) as f64, best.1, best.2, kept.len(), kept.iter().filter(|x| x.1).count());
    let mut out = String::from("# Mova's world-book selection, learned from the teacher's judgments (world worldbook-teach)\n");
    for (c, v, p, q) in &l.rules {
        out += &format!("{}\t{}\t{p}\t{q}\n", c.iter().map(|(a, b)| format!("{a}={b}")).collect::<Vec<_>>().join("&"), if *v { "yes" } else { "no" });
        println!("  if {} → {} ({p} good, {q} bad)", c.iter().map(|(a, b)| format!("{a} = {b}")).collect::<Vec<_>>().join(" and "), if *v { "keep" } else { "drop" });
    }
    out += &format!("*\t{}\n", if l.default { "yes" } else { "no" });
    println!("  otherwise → {}", if l.default { "keep" } else { "drop" });
    std::fs::write(&sel, out)?;
    Ok(())
}

/// Write the world book (markdown) and its facts file; `limit` keeps the best-supported facts (the mini world).
pub fn write(topic: &str, facts: &[Fact], sources: &[Source], out: &Path, version: &str, limit: Option<usize>, full_link: Option<&str>) -> Result<usize> {
    let by_src: BTreeMap<&str, &Source> = sources.iter().map(|s| (s.key.as_str(), s)).collect();
    let mut fs: Vec<&Fact> = facts.iter().collect();
    // best supported first: more sources, then more citations, then the text
    fs.sort_by(|a, b| {
        let sa = a.cites.iter().map(|c| &c.0).collect::<std::collections::BTreeSet<_>>().len();
        let sb = b.cites.iter().map(|c| &c.0).collect::<std::collections::BTreeSet<_>>().len();
        sb.cmp(&sa).then(b.cites.len().cmp(&a.cites.len())).then(a.text.cmp(&b.text))
    });
    if let Some(k) = limit {
        fs.truncate(k);
    }
    // sections: the topic, then the most named subjects, then the timeline
    let pronoun = |x: &str| matches!(x.to_lowercase().as_str(), "i" | "we" | "he" | "she" | "it" | "they" | "you" | "this" | "that" | "there" | "who" | "which" | "one" | "some" | "all" | "none");
    let mut subj_count: BTreeMap<&str, usize> = BTreeMap::new();
    for f in fs.iter().filter(|f| !pronoun(&f.subject)) {
        *subj_count.entry(f.subject.as_str()).or_default() += 1;
    }
    let mut subjects: Vec<(&str, usize)> = subj_count.into_iter().filter(|x| x.1 >= 2).collect();
    // one section per name whatever its case ("Boat" at a sentence start and "boat" are one)
    let mut seen_lower = std::collections::BTreeSet::new();
    subjects.retain(|x| seen_lower.insert(x.0.to_lowercase()));
    subjects.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    subjects.retain(|x| x.0.chars().next().is_some_and(char::is_uppercase) || x.1 >= 4);
    let base = topic.trim_end_matches("-principles");
    let name = base[..1].to_uppercase() + &base[1..];
    let mut md = if base == "titanic" { format!("# The world of the {name}\n\n") } else { format!("# The world of {}\n\n", base.to_lowercase()) };
    md += &format!("Release {version}{}. Written by Mova from {} sources: {} facts, each with its source. ", if limit.is_some() { " (mini)" } else { "" }, sources.len(), fs.len());
    md += if base == "titanic" { "It tells how things went in general — the ship, the boats, the crew, the passengers and their classes, the rules, what usually happened — not the stories of named people. " } else { "It tells what holds in general, not the stories of named people. " };
    md += "Every sentence comes from a source sentence — kept whole when it is short and plain, otherwise compressed along its parse tree — and Mova chose it with what she learned from a teacher's judgments; its footnote names the source sentence. Where several sources say the same, the sentence lists them all.";
    if let Some(l) = full_link {
        md += &format!(" The full world: {l}.");
    }
    md += "\n\n";
    let mut notes: Vec<String> = Vec::new();
    let mut cite = |f: &Fact, notes: &mut Vec<String>| -> String {
        let mut refs = Vec::new();
        let mini = limit.is_some();
        for (k, n, s) in f.cites.iter().take(if mini { 1 } else { 3 }) {
            let src = by_src.get(k.as_str());
            let who = src.map(|s| format!("{}, *{}* ({})", s.author, s.title, s.year)).unwrap_or_else(|| k.clone());
            let quote = if src.is_some_and(|s| s.pd) {
                if mini && s.chars().count() > 90 { format!(" — «{}…»", s.chars().take(90).collect::<String>()) } else { format!(" — «{s}»") }
            } else {
                String::new()
            };
            notes.push(if mini { format!("[^{}]: {who}, {k}:{n}{quote}", notes.len() + 1) } else { format!("[^{}]: {who}, sentence {k}:{n}{quote}. Kept: {}.", notes.len() + 1, f.kept) });
            refs.push(format!("[^{}]", notes.len()));
        }
        refs.join("")
    };
    let mut used = std::collections::BTreeSet::new();
    // first-person sentences are testimonies: one section, each with its book in the footnote
    let voices: Vec<&&Fact> = fs.iter().filter(|f| matches!(f.subject.to_lowercase().as_str(), "i" | "we")).collect();
    if !voices.is_empty() {
        md += "## Voices of the people who were there\n\n";
        for f in voices {
            md += &format!("{} {}\n", f.text, cite(f, &mut notes));
            used.insert(f.text.clone());
        }
        md += "\n";
    }
    for (subj, _) in subjects.iter().take(if limit.is_some() { 10 } else { 40 }) {
        md += &format!("## {}\n\n", subj.chars().next().map(|c| c.to_uppercase().collect::<String>() + &subj[c.len_utf8()..]).unwrap_or_default());
        let mine: Vec<&&Fact> = fs.iter().filter(|f| f.subject.eq_ignore_ascii_case(subj) && !used.contains(&f.text)).collect();
        for f in mine {
            md += &format!("{} {}\n", f.text, cite(f, &mut notes));
            used.insert(f.text.clone());
        }
        md += "\n";
    }
    let mut timeline: Vec<&&Fact> = fs.iter().filter(|f| f.time.is_some() && !used.contains(&f.text)).collect();
    timeline.sort_by(|a, b| a.time.cmp(&b.time).then(a.text.cmp(&b.text)));
    if !timeline.is_empty() {
        md += "## Times and dates\n\n";
        for f in timeline {
            md += &format!("{} {}\n", f.text, cite(f, &mut notes));
            used.insert(f.text.clone());
        }
        md += "\n";
    }
    let rest: Vec<&&Fact> = fs.iter().filter(|f| !used.contains(&f.text)).collect();
    if !rest.is_empty() {
        md += "## More\n\n";
        for f in rest {
            md += &format!("{} {}\n", f.text, cite(f, &mut notes));
        }
        md += "\n";
    }
    // disagreements: same subject and predicate, different numbers
    let mut groups: BTreeMap<(String, String), Vec<&Fact>> = BTreeMap::new();
    for f in facts.iter().filter(|f| !f.numbers.is_empty()) {
        groups.entry((f.subject.clone(), f.predicate.clone())).or_default().push(f);
    }
    let dis: Vec<_> = groups.into_iter().filter(|(_, v)| v.iter().map(|f| f.numbers.join(",")).collect::<std::collections::BTreeSet<_>>().len() > 1).collect();
    if !dis.is_empty() && limit.is_none() {
        md += "## Where the sources disagree\n\n";
        for ((s, p), v) in dis.iter().take(40) {
            md += &format!("- *{s} — {p}*: {}\n", v.iter().map(|f| format!("{} {}", f.text, cite(f, &mut notes))).collect::<Vec<_>>().join(" / "));
        }
        md += "\n";
    }
    md += "## Sources\n\n";
    let cited: std::collections::BTreeSet<&str> = notes.iter().filter_map(|n| n.split(", ").find_map(|p| p.split(':').next().filter(|k| by_src.contains_key(k)))).collect();
    for s in sources.iter().filter(|s| limit.is_none() || cited.contains(s.key.as_str())) {
        md += &format!("- {}, *{}* ({}){}\n", s.author, s.title, s.year, if s.pd { format!(" — {}", if s.licence == "pd" { "public domain" } else { s.licence.as_str() }) } else { ", cited by reference only".to_string() });
    }
    md += "\n";
    md += &notes.join("\n");
    md += "\n";
    std::fs::create_dir_all(out)?;
    let stem = if limit.is_some() { format!("{topic}.mini.world") } else { format!("{topic}.world") };
    std::fs::write(out.join(format!("{stem}.md")), &md)?;
    println!("{} → {} facts, {} footnotes, {} KB", out.join(format!("{stem}.md")).display(), fs.len(), notes.len(), md.len() / 1024);
    Ok(md.len())
}

/// `world worldbook-sources <dir>...`: a sources list from the corpus folders' `meta.jsonl` / `*.meta.jsonl` (file,
/// title or paper, author, date, licence); a testimony with a `.turns.tsv` next to its text is read by turns.
pub fn sources_args(args: &[String]) -> Result<()> {
    for d in args.iter().skip(1) {
        let dir = Path::new(d);
        let mut metas: Vec<std::path::PathBuf> = std::fs::read_dir(dir)?.filter_map(|e| e.ok().map(|e| e.path())).filter(|p| { let n = p.to_string_lossy(); n.ends_with("meta.jsonl") || n.ends_with("metadata.jsonl") }).collect();
        // a corpus of sub-corpora keeps a metadata file in each (corpus-math: books/, planetmath/, wikipedia-en/ …)
        if metas.is_empty() {
            metas = walk_files(dir, "metadata.jsonl");
        }
        metas.sort();
        let mut by_key: BTreeMap<String, serde_json::Value> = BTreeMap::new();
        // a metadata line that names its file by a path relative to the corpus (section files whose stems repeat)
        let mut by_path: BTreeMap<std::path::PathBuf, serde_json::Value> = BTreeMap::new();
        let mut default: Option<serde_json::Value> = None;
        for m in &metas {
            for l in std::fs::read_to_string(m)?.lines() {
                let Ok(v) = serde_json::from_str::<serde_json::Value>(l) else { continue };
                let stem = m.file_name().map(|x| x.to_string_lossy().replace(".meta.jsonl", "")).unwrap_or_default();
                let key = v["file"].as_str().map(|f| Path::new(f).file_stem().map(|x| x.to_string_lossy().into_owned()).unwrap_or_default()).or_else(|| v["id"].as_str().map(String::from)).unwrap_or(stem);
                if default.is_none() {
                    default = Some(v.clone());
                }
                if let Some(rel) = v["path"].as_str() {
                    by_path.insert(dir.join(rel), v.clone());
                }
                by_key.insert(key, v);
            }
        }
        // every text of the folder, newest naming conventions included (a file whose metadata line is missing takes
        // the folder's licence)
        let mut files: Vec<std::path::PathBuf> = walk_txt(dir);
        files.sort();
        for path in files {
            let stem = path.file_stem().map(|x| x.to_string_lossy().into_owned()).unwrap_or_default();
            let Some(v) = by_path.get(&path).or_else(|| by_key.get(&stem)).or(default.as_ref()) else { continue };
            let g = |k: &str| v[k].as_str().map(String::from).or_else(|| v[k].as_array().map(|a| a.iter().filter_map(|x| x.as_str()).collect::<Vec<_>>().join(", "))).unwrap_or_default().replace('\t', " ");
            let mut p = path.clone();
            for ext in ["turns.tsv", "turns.jsonl"] {
                let t = path.with_extension(ext);
                if t.exists() {
                    p = t;
                }
            }
            let mut title = [g("title"), g("paper"), g("newspaper"), g("source_title"), g("page_title"), g("entry_title")].into_iter().find(|x| !x.is_empty()).unwrap_or_else(|| stem.clone());
            let section = g("section_title");
            if !section.is_empty() && !section.eq_ignore_ascii_case("lead") && section != "(lead)" && !title.to_lowercase().ends_with(&section.to_lowercase()) {
                title = format!("{title}, «{section}»");
            }
            // the year: the date, the year, the edition's dump date or the entry's creation
            let year: String = [g("date"), g("year"), g("created"), g("edition").split_whitespace().find(|w| w.len() == 10 && w.starts_with("20")).unwrap_or("").to_string()].into_iter().find(|x| x.len() >= 4).unwrap_or_else(|| if by_key.contains_key(&stem) { String::new() } else { stem.chars().skip_while(|c| !c.is_ascii_digit()).take(10).collect() });
            let author = [g("author"), g("authors")].into_iter().find(|x| !x.is_empty()).unwrap_or_default();
            println!("{}\t{}\t{}\t{}\t{}", p.display(), title, author, year.get(..4).unwrap_or(&year), g("licence"));
        }
    }
    Ok(())
}

/// The files of a folder (recursively) with this name.
fn walk_files(dir: &Path, name: &str) -> Vec<std::path::PathBuf> {
    let mut out = Vec::new();
    if let Ok(rd) = std::fs::read_dir(dir) {
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() {
                out.extend(walk_files(&p, name));
            } else if p.file_name().is_some_and(|x| x == name) {
                out.push(p);
            }
        }
    }
    out
}

/// The .txt files of a folder (recursively), wikitext and turns excluded.
fn walk_txt(dir: &Path) -> Vec<std::path::PathBuf> {
    let mut out = Vec::new();
    if let Ok(rd) = std::fs::read_dir(dir) {
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() {
                out.extend(walk_txt(&p));
            } else if p.extension().is_some_and(|x| x == "txt") {
                out.push(p);
            }
        }
    }
    out
}

/// `world worldbook-teach`.
pub fn teach_args(args: &[String]) -> Result<()> {
    let usage = "usage: world worldbook-teach <topic> <sources.tsv> <n>";
    let sources = read_sources(Path::new(args.get(2).context(usage)?))?;
    teach(args.get(1).context(usage)?, &sources, args.get(3).and_then(|x| x.parse().ok()).unwrap_or(200))
}

/// `world worldbook`.
pub fn main_args(args: &[String]) -> Result<()> {
    let usage = "usage: world worldbook <topic> <sources.tsv> <out dir> [--release R] [--mini K] [--full-link URL]";
    let opt = |k: &str| args.iter().position(|a| a == k).and_then(|i| args.get(i + 1)).cloned();
    let topic = args.get(1).context(usage)?;
    let sources = read_sources(Path::new(args.get(2).context(usage)?))?;
    let out = Path::new(args.get(3).context(usage)?);
    let version = opt("--release").unwrap_or_else(|| RELEASE.into());
    let facts = build(topic, &sources)?;
    write(topic, &facts, &sources, out, &version, None, None)?;
    // the mini world fits a byte budget (tens of kilobytes, for the repository): the most facts that fit
    let budget: usize = opt("--mini-kb").and_then(|x| x.parse().ok()).unwrap_or(40) * 1024;
    let (mut lo, mut hi) = (10usize, opt("--mini").and_then(|x| x.parse().ok()).unwrap_or(1000).min(facts.len().max(10)));
    while lo < hi {
        let mid = (lo + hi).div_ceil(2);
        if write(topic, &facts, &sources, out, &version, Some(mid), opt("--full-link").as_deref())? <= budget { lo = mid } else { hi = mid - 1 }
    }
    write(topic, &facts, &sources, out, &version, Some(lo), opt("--full-link").as_deref())?;
    Ok(())
}

// ---- Learned compression: Mova learns which words of a sentence to keep from the teacher's deletions ----------
//
// The teacher (Opus) compresses a sample of topic sentences by deletion only; every word becomes an example with
// what Mova sees of it in the tree (its relation, part of speech, its head's, depth, leaf or not, side of the head,
// a preposition under it, a negation word) and the label keep/drop. Mova learns a decision list over these; to
// compress, she walks the tree from the root and keeps a word only if the list keeps it and its head is kept — the
// result is always a connected tree, read in the original order (deletion-based compression on dependency trees,
// after Filippova & Altun 2013, outside the library).

fn depth(w: &[Word], i: usize) -> usize {
    let (mut d, mut x) = (0, i);
    while w[x].head > 0 && d < w.len() {
        x = w[x].head - 1;
        d += 1;
    }
    d
}

/// What Mova sees about word `i` for the keep/drop decision.
pub fn word_attrs(w: &[Word], i: usize) -> BTreeMap<String, String> {
    let x = &w[i];
    let h = if x.head > 0 { Some(&w[x.head - 1]) } else { None };
    let kids: Vec<&Word> = w.iter().filter(|y| y.head == i + 1).collect();
    let mut a = BTreeMap::new();
    a.insert("relation".to_string(), x.rel.to_string());
    a.insert("word".to_string(), format!("{:?}", x.upos));
    a.insert("head word".to_string(), h.map(|h| format!("{:?}", h.upos)).unwrap_or_else(|| "root".into()));
    a.insert("head relation".to_string(), h.map(|h| h.rel.to_string()).unwrap_or_else(|| "root".into()));
    a.insert("depth".to_string(), depth(w, i).min(4).to_string());
    a.insert("leaf".to_string(), if kids.is_empty() { "yes" } else { "no" }.into());
    a.insert("side".to_string(), if x.head == 0 { "root".into() } else if i + 1 < x.head { "before head".into() } else { "after head".to_string() });
    a.insert("preposition".to_string(), kids.iter().find(|k| k.rel == Rel::Case).map(|k| k.lemma.to_lowercase()).filter(|l| ["of", "in", "on", "at", "to", "from", "by", "with", "for", "into"].contains(&l.as_str())).unwrap_or_else(|| if kids.iter().any(|k| k.rel == Rel::Case) { "other".into() } else { "none".into() }));
    a.insert("negation".to_string(), if matches!(x.lemma.to_lowercase().as_str(), "not" | "never" | "n't" | "no") { "yes" } else { "no" }.into());
    a.insert("own subject".to_string(), if kids.iter().any(|k| matches!(k.rel, Rel::Nsubj | Rel::NsubjPass)) { "yes" } else { "no" }.into());
    a.insert("relation pair".to_string(), format!("{} under {}", x.rel, h.map(|h| h.rel.to_string()).unwrap_or_else(|| "root".into())));
    a.insert("children".to_string(), kids.len().min(3).to_string());
    a.insert("half".to_string(), if i * 2 < w.len() { "first" } else { "second" }.into());
    // function words by themselves (a closed set: what a word is matters more than its tag)
    let l = x.lemma.to_lowercase();
    if matches!(x.upos, UPos::ADP | UPos::CCONJ | UPos::SCONJ | UPos::AUX | UPos::PART | UPos::DET | UPos::PRON) || matches!(l.as_str(), "say" | "tell" | "ask" | "reply" | "answer") {
        a.insert("lemma".to_string(), l);
    }
    a
}

fn compress_path() -> std::path::PathBuf {
    selection_path().with_file_name("worldbook-compress.tsv")
}

fn read_list(p: &std::path::Path) -> Option<crate::tabular::List> {
    let t = std::fs::read_to_string(p).ok()?;
    let mut rules = Vec::new();
    let mut default = true;
    for l in t.lines().filter(|l| !l.starts_with('#')) {
        let c: Vec<&str> = l.split('\t').collect();
        if c.len() < 2 {
            continue;
        }
        if c[0] == "*" {
            default = c[1] == "yes";
            continue;
        }
        let cond = c[0].split('&').filter_map(|x| x.split_once('=')).map(|(a, b)| (a.to_string(), b.to_string())).collect();
        rules.push((cond, c[1] == "yes", c.get(2).and_then(|x| x.parse().ok()).unwrap_or(0), c.get(3).and_then(|x| x.parse().ok()).unwrap_or(0)));
    }
    Some(crate::tabular::List { rules, default })
}

fn write_list(p: &std::path::Path, l: &crate::tabular::List, what: &str) -> Result<()> {
    let mut out = format!("# {what}\n");
    for (c, v, a, b) in &l.rules {
        out += &format!("{}\t{}\t{a}\t{b}\n", c.iter().map(|(x, y)| format!("{x}={y}")).collect::<Vec<_>>().join("&"), if *v { "yes" } else { "no" });
    }
    out += &format!("*\t{}\n", if l.default { "yes" } else { "no" });
    std::fs::write(p, out)?;
    Ok(())
}

/// Compress a sentence with a learned keep/drop list, top-down (a word stays only under a kept head).
pub fn compress_with(l: &crate::tabular::List, w: &[Word]) -> String {
    let mut keep = vec![false; w.len()];
    let mut order: Vec<usize> = (0..w.len()).collect();
    order.sort_by_key(|&i| depth(w, i));
    for i in order {
        let head_kept = w[i].head == 0 || keep[w[i].head - 1];
        keep[i] = head_kept && (w[i].head == 0 || crate::tabular::predict(l, &word_attrs(w, i)).0);
    }
    // grammar, not taste: a kept word keeps its function words (case, determiner, auxiliaries, copula, particle,
    // negation, name parts) — "resound from the bridge", never "resound the bridge"
    for i in 0..w.len() {
        if w[i].head > 0 && keep[w[i].head - 1] && (matches!(w[i].rel, Rel::Case | Rel::Det | Rel::Aux | Rel::AuxPass | Rel::Cop | Rel::CompoundPrt | Rel::Flat) || matches!(w[i].lemma.to_lowercase().as_str(), "not" | "n't" | "never")) {
            keep[i] = true;
        }
    }
    let mut s = (0..w.len()).filter(|&i| keep[i]).map(|i| w[i].form.as_str()).collect::<Vec<_>>().join(" ").replace(" ,", ",").replace(" .", ".").replace(" 's", "'s").replace(" n't", "n't");
    s = s.trim_end_matches([',', ';', ':', ' ']).to_string();
    if !s.ends_with(['.', '!', '?']) {
        s.push('.');
    }
    s
}

/// `world worldbook-compress-teach <topic> <sources.tsv> <n>`: the teacher compresses `n` topic sentences by
/// deletion; Mova learns her keep/drop list (settings by 10-fold cross-validation over sentences) and keeps it.
pub fn compress_teach(topic: &str, sources: &[Source], n: usize) -> Result<()> {
    if std::env::var("WORLDBOOK_FROM_LOG").is_ok() {
        return compress_relearn(topic);
    }
    let words = topic_words(topic)?;
    let a = crate::tree::annotator()?;
    let mut sents: Vec<Vec<Word>> = Vec::new();
    for src in sources {
        let t = std::fs::read_to_string(&src.path)?;
        for s in crate::events::sentences(crate::events::book_body(&t)) {
            let low = s.to_lowercase();
            if low.split(|c: char| !c.is_alphanumeric() && c != '\'' && c != '-').any(|x| words.contains(&x)) && (8..=40).contains(&s.split_whitespace().count()) {
                let clean = s.replace('_', "");
                let forms: Vec<String> = a.tokenize(&clean).into_iter().map(|t| t.form).collect();
                let mut w = a.annotate(&forms);
                crate::rerank::repair_words(&mut w);
                sents.push(w);
            }
        }
    }
    let step = (sents.len() / n.max(1)).max(1);
    let sample: Vec<&Vec<Word>> = sents.iter().step_by(step).take(n).collect();
    let dir = std::path::PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| format!("{}/mova-data", std::env::var("HOME").unwrap_or_default()))).join("runs/worldbook-teach");
    let opus = prag::opus::Opus::from_env(dir.join("cwd"));
    // per sentence: the words with their keep labels
    let mut taught: Vec<(usize, Vec<bool>)> = Vec::new();
    let mut log = String::new();
    for (bi, batch) in sample.chunks(10).enumerate() {
        let mut prompt = String::from("Compress each sentence by DELETION ONLY: keep the words (by number) that, in their original order, form one short grammatical English sentence stating the main fact. Keep names, numbers, times and places that matter; drop asides, relative clauses, discourse words, most adjectives and adverbs. Answer only with lines «<item>: <numbers separated by spaces>».\n\n");
        for (k, w) in batch.iter().enumerate() {
            prompt += &format!("{}. {}\n", k + 1, w.iter().enumerate().map(|(i, x)| format!("{}:{}", i + 1, x.form)).collect::<Vec<_>>().join(" "));
        }
        let (ans, call) = opus.ask("worldbook-compress-teach", bi, batch.len(), &prompt)?;
        prag::opus::append_call(&dir.join("calls.jsonl"), &call)?;
        for l in ans.lines() {
            let Some((item, nums)) = l.split_once(':') else { continue };
            let Ok(item) = item.trim().trim_end_matches('.').parse::<usize>() else { continue };
            let Some(w) = batch.get(item.wrapping_sub(1)) else { continue };
            let keep_ids: Vec<usize> = nums.split(|c: char| !c.is_ascii_digit()).filter_map(|x| x.parse().ok()).collect();
            let labels: Vec<bool> = (0..w.len()).map(|i| keep_ids.contains(&(i + 1))).collect();
            log += &format!("{}\t{}\n", w.iter().map(|x| x.form.as_str()).collect::<Vec<_>>().join(" "), w.iter().zip(&labels).filter(|x| *x.1).map(|x| x.0.form.as_str()).collect::<Vec<_>>().join(" "));
            taught.push((bi * 10 + item - 1, labels));
        }
    }
    std::fs::create_dir_all(&dir)?;
    std::fs::write(dir.join(format!("{topic}-compressions.tsv")), log)?;
    let ex = |idx: &[usize]| -> Vec<(BTreeMap<String, String>, bool)> {
        idx.iter().flat_map(|&t| {
            let (si, labels) = &taught[t];
            let w = sample[*si];
            (0..w.len()).filter(|&i| w[i].head != 0).map(move |i| (word_attrs(w, i), labels[i])).collect::<Vec<_>>()
        }).collect()
    };
    let all: Vec<usize> = (0..taught.len()).collect();
    let mut best = (0usize, 10usize, 1usize, 0usize);
    for k in 1..=2 {
        for min in [5usize, 10, 20] {
            let (mut ok, mut tot) = (0, 0);
            for fold in 0..10 {
                let tr: Vec<usize> = all.iter().copied().filter(|t| t % 10 != fold).collect();
                let te: Vec<usize> = all.iter().copied().filter(|t| t % 10 == fold).collect();
                let l = crate::tabular::learn_k(&ex(&tr), min, k);
                for (f, y) in ex(&te) {
                    ok += usize::from(crate::tabular::predict(&l, &f).0 == y);
                    tot += 1;
                }
            }
            if ok > best.0 {
                best = (ok, min, k, tot);
            }
        }
    }
    let l = crate::tabular::learn_k(&ex(&all), best.1, best.2);
    let kept_share = ex(&all).iter().filter(|x| x.1).count() as f64 / ex(&all).len().max(1) as f64;
    println!("teacher compressed {} sentences ({:.0}% of words kept); Mova's keep/drop list: CV word accuracy {:.1}% (min {}, up to {}), {} rules", taught.len(), 100.0 * kept_share, 100.0 * best.0 as f64 / best.3.max(1) as f64, best.1, best.2, l.rules.len());
    write_list(&compress_path(), &l, "Mova's keep/drop list for world-book compression, learned from the teacher's deletions")?;
    for w in sample.iter().take(6) {
        println!("  {}\n    → {}", w.iter().map(|x| x.form.as_str()).collect::<Vec<_>>().join(" "), compress_with(&l, w));
    }
    Ok(())
}

/// Relearn the keep/drop list from the teacher's saved compressions (no new calls): the kept words are matched to the
/// sentence in order.
fn compress_relearn(topic: &str) -> Result<()> {
    let dir = std::path::PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| format!("{}/mova-data", std::env::var("HOME").unwrap_or_default()))).join("runs/worldbook-teach");
    let log = std::fs::read_to_string(dir.join(format!("{topic}-compressions.tsv")))?;
    let a = crate::tree::annotator()?;
    let mut data: Vec<(Vec<Word>, Vec<bool>)> = Vec::new();
    for l in log.lines() {
        let Some((sent, kept)) = l.split_once('\t') else { continue };
        let forms: Vec<String> = sent.split(' ').map(String::from).collect();
        let mut w = a.annotate(&forms);
        crate::rerank::repair_words(&mut w);
        let k: Vec<&str> = kept.split(' ').collect();
        let (mut j, mut labels) = (0, vec![false; w.len()]);
        for (i, f) in forms.iter().enumerate() {
            if j < k.len() && k[j] == f {
                labels[i] = true;
                j += 1;
            }
        }
        data.push((w, labels));
    }
    let ex = |idx: &[usize]| -> Vec<(BTreeMap<String, String>, bool)> {
        idx.iter().flat_map(|&t| {
            let (w, labels) = &data[t];
            (0..w.len()).filter(|&i| w[i].head != 0).map(|i| (word_attrs(w, i), labels[i])).collect::<Vec<_>>()
        }).collect()
    };
    let all: Vec<usize> = (0..data.len()).collect();
    let mut best = (0usize, 10usize, 1usize, 0usize);
    for k in 1..=2 {
        for min in [5usize, 10, 20, 40] {
            let (mut ok, mut tot) = (0, 0);
            for fold in 0..10 {
                let tr: Vec<usize> = all.iter().copied().filter(|t| t % 10 != fold).collect();
                let te: Vec<usize> = all.iter().copied().filter(|t| t % 10 == fold).collect();
                let l = crate::tabular::learn_kc(&ex(&tr), min, k, 60);
                for (f, y) in ex(&te) {
                    ok += usize::from(crate::tabular::predict(&l, &f).0 == y);
                    tot += 1;
                }
            }
            println!("  min {min:>2}, up to {k}: CV word accuracy {:.1}%", 100.0 * ok as f64 / tot.max(1) as f64);
            if ok > best.0 {
                best = (ok, min, k, tot);
            }
        }
    }
    let l = crate::tabular::learn_kc(&ex(&all), best.1, best.2, 60);
    println!("relearned from {} compressions: CV word accuracy {:.1}% (min {}, up to {}), {} rules", data.len(), 100.0 * best.0 as f64 / best.3.max(1) as f64, best.1, best.2, l.rules.len());
    write_list(&compress_path(), &l, "Mova's keep/drop list for world-book compression, learned from the teacher's deletions")?;
    for (w, _) in data.iter().take(6) {
        println!("  {}\n    → {}", w.iter().map(|x| x.form.as_str()).collect::<Vec<_>>().join(" "), compress_with(&l, w));
    }
    Ok(())
}

/// The whole source sentence as a candidate (5–25 words, a statement with a subject), with what Mova sees about it.
fn whole_fact(w: &[Word], key: &str, n: usize, sentence: &str) -> Option<Fact> {
    let words = w.iter().filter(|x| x.upos != UPos::PUNCT).count();
    let root = w.iter().position(|x| x.head == 0)?;
    let subj = w.iter().position(|x| x.head == root + 1 && matches!(x.rel, Rel::Nsubj | Rel::NsubjPass))?;
    if !(5..=25).contains(&words) || sentence.trim_end().ends_with('?') {
        return None;
    }
    let mut text = sentence.replace(['“', '”'], "\"").replace(" n't", "n't").replace(" n’t", "n’t").replace("do n’t", "don’t").trim().to_string();
    // a quote mark that opens or closes nothing in the sentence is dropped
    if text.matches('"').count() % 2 == 1 {
        text = text.replacen('"', "", 1).trim().to_string();
    }
    let mut attrs = BTreeMap::new();
    attrs.insert("kind".to_string(), "whole".to_string());
    attrs.insert("source length".to_string(), format!("{}", [10usize, 20, 35].iter().filter(|c| w.len() > **c).count()));
    attrs.insert("source has quotes".to_string(), if sentence.contains(['"', '“', '”']) { "yes" } else { "no" }.into());
    attrs.insert("subject".to_string(), match w[subj].upos { UPos::PROPN => "name", UPos::NOUN => "noun", UPos::PRON => "pronoun", _ => "other" }.into());
    attrs.insert("starts with".to_string(), w.first().map(|x| if matches!(x.upos, UPos::CCONJ | UPos::ADV | UPos::SCONJ) { "connective" } else if x.upos == UPos::PUNCT { "punctuation" } else { "word" }).unwrap_or("word").into());
    attrs.insert("first person".to_string(), if w.iter().any(|x| matches!(x.lemma.to_lowercase().as_str(), "i" | "we" | "my" | "our")) { "yes" } else { "no" }.into());
    Some(Fact {
        text,
        subject: w[subj].form.clone(),
        predicate: w[root].lemma.to_lowercase(),
        object: String::from("(whole)"),
        cites: vec![(key.to_string(), n, sentence.to_string())],
        kept: "the whole sentence".into(),
        lemmas: Vec::new(),
        nouns: Vec::new(),
        time: sentence.split(',').find(|p| is_time(p)).map(|p| p.trim().to_string()),
        numbers: sentence.split_whitespace().filter(|t| t.chars().next().is_some_and(|c| c.is_ascii_digit())).map(String::from).collect(),
        attrs,
    })
}

/// A fact by Mova's learned compression: the compressed sentence, its root and subject.
fn learned_fact(l: &crate::tabular::List, w: &[Word], key: &str, n: usize, sentence: &str) -> Option<Fact> {
    let root = w.iter().position(|x| x.head == 0)?;
    let subj = w.iter().position(|x| x.head == root + 1 && matches!(x.rel, Rel::Nsubj | Rel::NsubjPass))?;
    let text = compress_with(l, w);
    let text = text.get(..1).map(|c| c.to_uppercase() + &text[1..]).unwrap_or(text);
    if text.split_whitespace().count() < 3 {
        return None;
    }
    let mut attrs = BTreeMap::new();
    attrs.insert("by".to_string(), "learned compression".to_string());
    Some(Fact {
        time: w.iter().map(|x| x.form.as_str()).collect::<Vec<_>>().join(" ").split(',').find(|p| is_time(p)).map(|p| p.trim().to_string()),
        numbers: text.split_whitespace().filter(|t| t.chars().next().is_some_and(|c| c.is_ascii_digit())).map(String::from).collect(),
        subject: w[subj].form.clone(),
        predicate: w[root].lemma.to_lowercase(),
        object: String::new(),
        cites: vec![(key.to_string(), n, sentence.to_string())],
        kept: "words chosen by Mova's learned keep/drop list".into(),
        text,
        attrs,
        lemmas: Vec::new(),
        nouns: Vec::new(),
    })
}

/// `world worldbook-compress-teach`.
pub fn compress_teach_args(args: &[String]) -> Result<()> {
    let usage = "usage: world worldbook-compress-teach <topic> <sources.tsv> <n>";
    let sources = read_sources(Path::new(args.get(2).context(usage)?))?;
    compress_teach(args.get(1).context(usage)?, &sources, args.get(3).and_then(|x| x.parse().ok()).unwrap_or(200))
}

// ---- Reading a world back: who survived, from Mova's own world book ---------------------------------------------
//
// A reading test with known answers (the Kaggle Titanic training passengers): Mova reads the world book like any
// English text with her ordinary tools (sentences, the parser, lemmas) and, for each passenger, looks for sentences
// that name them (family name with a given name or a title) and say what happened to them — rescued, saved, in a
// boat, picked up (lived); lost, drowned, perished, went down, body found (died). The mini world first; the full
// world only for those the mini one does not know. Never used for a submission: that would be reading the answer.

fn world_sentences(md: &str) -> Vec<String> {
    md.lines()
        .filter(|l| !l.starts_with('#') && !l.starts_with("[^") && !l.starts_with("- ") && !l.trim().is_empty())
        .map(|l| l.split("[^").next().unwrap_or(l).trim().to_string())
        .filter(|l| l.split_whitespace().count() >= 3)
        .collect()
}

/// What a sentence says happened to its subject: Some(true) lived, Some(false) died, None nothing clear.
fn outcome(w: &[Word]) -> Option<bool> {
    let lem: Vec<String> = w.iter().map(|x| x.lemma.to_lowercase()).collect();
    let has = |x: &str| lem.iter().any(|l| l == x);
    let neg = has("not") || has("never") || has("n't");
    let lived = has("rescue") || has("save") || has("survive") || has("survivor") || has("pick") && has("up") || has("lifeboat") || (has("boat") && (has("put") || has("get") || has("leave") || has("lower"))) || has("carpathia");
    let died = has("drown") || has("perish") || has("die") || has("dead") || has("body") || has("bury") || (has("lose") && lem.iter().any(|l| l == "be")) || (has("go") && has("down")) || has("victim");
    match (lived, died) {
        (true, false) => Some(!neg),
        (false, true) => Some(neg),
        _ => None,
    }
}

/// `world worldbook-read <mini.md> <full.md> <train.csv>`: the reading test.
pub fn read_args(args: &[String]) -> Result<()> {
    let usage = "usage: world worldbook-read <mini.md> <full.md> <train.csv>";
    let mini = std::fs::read_to_string(args.get(1).context(usage)?)?;
    let full = std::fs::read_to_string(args.get(2).context(usage)?)?;
    let table = crate::tabular::read_csv(Path::new(args.get(3).context(usage)?))?;
    let col = |n: &str| table.head.iter().position(|h| h == n);
    let (ni, si) = (col("Name").context("no Name")?, col("Survived").context("no Survived")?);
    let a = crate::tree::annotator()?;
    let parse = |s: &str| {
        let forms: Vec<String> = a.tokenize(s).into_iter().map(|t| t.form).collect();
        a.annotate(&forms)
    };
    let (ms, fs) = (world_sentences(&mini), world_sentences(&full));
    let titles = |t: &str| match t { "Mr" => vec!["Mr.", "Mr"], "Mrs" => vec!["Mrs.", "Mrs"], "Miss" => vec!["Miss"], "Master" => vec!["Master"], "Col" => vec!["Colonel", "Col."], "Dr" => vec!["Dr.", "Doctor"], "Rev" => vec!["Rev.", "Reverend"], "Capt" => vec!["Captain", "Capt."], "Major" => vec!["Major"], _ => vec![] };
    let (mut found_m, mut found_f, mut right_m, mut right_f) = (0, 0, 0, 0);
    let mut shown = 0;
    for r in &table.rows {
        let name = &r[ni];
        let truth = r[si] == "1";
        let Some((fam, rest)) = name.split_once(", ") else { continue };
        let (title, given) = rest.split_once(". ").unwrap_or(("", rest));
        let given_first = given.split(|c: char| c == ' ' || c == '(' || c == '"').find(|x| x.len() > 1).unwrap_or("");
        let names_them = |s: &str| {
            s.split(|c: char| !c.is_alphanumeric() && c != '\'').any(|t| t == fam) && (s.contains(given_first) && !given_first.is_empty() || titles(title).iter().any(|t| s.contains(&format!("{t} {fam}"))))
        };
        let decide = |sents: &[String]| -> Option<(bool, String)> {
            let mut votes = (0, 0, String::new());
            for s in sents.iter().filter(|s| names_them(s)) {
                if let Some(o) = outcome(&parse(s)) {
                    if o { votes.0 += 1 } else { votes.1 += 1 }
                    if votes.2.is_empty() {
                        votes.2 = s.clone();
                    }
                }
            }
            (votes.0 != votes.1).then(|| (votes.0 > votes.1, votes.2))
        };
        if let Some((o, why)) = decide(&ms) {
            found_m += 1;
            right_m += usize::from(o == truth);
            if shown < 4 {
                shown += 1;
                println!("  {name}: {} — «{}» (truth: {})", if o { "lived" } else { "died" }, why.chars().take(140).collect::<String>(), if truth { "lived" } else { "died" });
            }
        } else if let Some((o, _)) = decide(&fs) {
            found_f += 1;
            right_f += usize::from(o == truth);
        }
    }
    let n = table.rows.len();
    println!("passengers {n}: the mini world tells {found_m} ({right_m} right, {:.0}%); the full world tells {found_f} more ({right_f} right, {:.0}%); unknown {}", 100.0 * right_m as f64 / found_m.max(1) as f64, 100.0 * right_f as f64 / found_f.max(1) as f64, n - found_m - found_f);
    Ok(())
}

// ---- The small world is made from the big one --------------------------------------------------------------------
//
// Mova reads her big world book back (its sentences, the footnotes each sentence carries, the section it stands in)
// and keeps, within a byte budget, the sentences best supported (most footnotes = most sources saying it), taking
// them round-robin from the sections so the small world still covers every part; footnotes are renumbered, quotes
// shortened, only cited sources listed.

/// `world worldbook-mini <big.md> <out.md> [--kb 40] [--full-link URL]`.
pub fn mini_args(args: &[String]) -> Result<()> {
    let usage = "usage: world worldbook-mini <big.md> <out.md> [--kb 40] [--full-link URL]";
    let opt = |k: &str| args.iter().position(|a| a == k).and_then(|i| args.get(i + 1)).cloned();
    let big = std::fs::read_to_string(args.get(1).context(usage)?)?;
    let out = Path::new(args.get(2).context(usage)?);
    let budget = opt("--kb").and_then(|x| x.parse::<usize>().ok()).unwrap_or(40) * 1024;
    // footnotes of the big world
    let notes: BTreeMap<String, String> = big.lines().filter_map(|l| l.strip_prefix("[^").and_then(|r| r.split_once("]: ")).map(|(k, v)| (k.to_string(), v.to_string()))).collect();
    // sections and their sentences with the footnote ids they cite
    let mut sections: Vec<(String, Vec<(String, Vec<String>)>)> = Vec::new();
    let mut title = String::new();
    for l in big.lines() {
        if let Some(t) = l.strip_prefix("# ") {
            title = t.to_string();
        } else if let Some(h) = l.strip_prefix("## ") {
            if h != "Sources" {
                sections.push((h.to_string(), Vec::new()));
            } else {
                break;
            }
        } else if !l.trim().is_empty() && !l.starts_with("[^") && !l.starts_with("Version ") {
            if let Some(sec) = sections.last_mut() {
                let ids: Vec<String> = l.match_indices("[^").filter_map(|(i, _)| l[i + 2..].split(']').next().map(String::from)).collect();
                let text = l.split("[^").next().unwrap_or(l).trim().to_string();
                if !text.is_empty() && !ids.is_empty() {
                    sec.1.push((text, ids));
                }
            }
        }
    }
    for sec in sections.iter_mut() {
        // most sources first; otherwise the order of the big world (which is already by support and quality)
        sec.1.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
    }
    let shorten = |n: &str| -> String {
        match n.split_once(" — «") {
            Some((who, q)) => {
                let q = q.trim_end_matches(|c| c == '.' || c == '»').split(". Kept:").next().unwrap_or(q);
                let q: String = q.chars().take(90).collect();
                format!("{} — «{q}…»", who.replace(", sentence ", ", "))
            }
            None => n.split(". Kept:").next().unwrap_or(n).replace(", sentence ", ", "),
        }
    };
    let render = |k: usize| -> String {
        let mut md = format!("# {title} (small)\n\nThe small world, made by Mova from her big world: the best-supported sentences of every part, one footnote each.{}\n\n", opt("--full-link").map(|l| format!(" The big world: {l}.")).unwrap_or_default());
        let mut foot = Vec::new();
        let mut taken = 0;
        let mut depth = 0;
        let mut body: Vec<(usize, String)> = Vec::new();
        while taken < k {
            let mut any = false;
            for (si, (_, sents)) in sections.iter().enumerate() {
                if taken >= k {
                    break;
                }
                if let Some((text, ids)) = sents.get(depth) {
                    any = true;
                    taken += 1;
                    let n = notes.get(&ids[0]).map(|x| shorten(x)).unwrap_or_default();
                    foot.push(n);
                    body.push((si, format!("{text} [^{}]", foot.len())));
                }
            }
            if !any {
                break;
            }
            depth += 1;
        }
        for (si, (name, _)) in sections.iter().enumerate() {
            let lines: Vec<&String> = body.iter().filter(|x| x.0 == si).map(|x| &x.1).collect();
            if !lines.is_empty() {
                md += &format!("## {name}\n\n");
                for l in lines {
                    md += &format!("{l}\n");
                }
                md += "\n";
            }
        }
        for (i, n) in foot.iter().enumerate() {
            md += &format!("[^{}]: {n}\n", i + 1);
        }
        md
    };
    let (mut lo, mut hi) = (5usize, sections.iter().map(|s| s.1.len()).sum::<usize>().max(5));
    while lo < hi {
        let mid = (lo + hi).div_ceil(2);
        if render(mid).len() <= budget { lo = mid } else { hi = mid - 1 }
    }
    let md = render(lo);
    std::fs::write(out, &md)?;
    println!("{} → {lo} sentences from {} sections, {} KB", out.display(), sections.len(), md.len() / 1024);
    Ok(())
}

/// `world worldbook-judge <world.md> <n> [--principles]`: the teacher judges `n` sentences of a written world (evenly
/// spread), with the source sentence from the footnote when it is quoted. Prints the share of good sentences.
pub fn judge_args(args: &[String]) -> Result<()> {
    let usage = "usage: world worldbook-judge <world.md> <n> [--principles]";
    let md = std::fs::read_to_string(args.get(1).context(usage)?)?;
    let n: usize = args.get(2).and_then(|x| x.parse().ok()).unwrap_or(100);
    let principled = args.iter().any(|a| a == "--principles");
    let notes: BTreeMap<String, String> = md.lines().filter_map(|l| l.strip_prefix("[^").and_then(|r| r.split_once("]: ")).map(|(k, v)| (k.to_string(), v.to_string()))).collect();
    let items: Vec<(String, String)> = md
        .lines()
        .filter(|l| !l.starts_with('#') && !l.starts_with("[^") && !l.starts_with("- ") && l.contains("[^"))
        .map(|l| {
            let text = l.split("[^").next().unwrap_or(l).trim().to_string();
            let id = l.split("[^").nth(1).and_then(|r| r.split(']').next()).unwrap_or("");
            let src = notes.get(id).and_then(|v| v.split_once(" — «").map(|x| x.1.trim_end_matches(|c| c == '.' || c == '»' || c == '…').to_string())).unwrap_or_default();
            (text, src)
        })
        .collect();
    let step = (items.len() / n.max(1)).max(1);
    let sample: Vec<&(String, String)> = items.iter().step_by(step).take(n).collect();
    let dir = std::path::PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| format!("{}/mova-data", std::env::var("HOME").unwrap_or_default()))).join("runs/worldbook-teach");
    let opus = prag::opus::Opus::from_env(dir.join("cwd"));
    let (mut good, mut judged) = (0, 0);
    for (bi, batch) in sample.chunks(25).enumerate() {
        let mut prompt = if principled {
            String::from("Each item is a SENTENCE of a written world about an event, with the SOURCE sentence it comes from when known. Answer «good» only if the sentence is grammatical English, agrees with its source, and states a general principle or a general fact about the event (people, groups, classes, crew, boats, the ship, rules, what usually happened) — not a story about a specific named person. Otherwise «bad». Answer only with lines «<number> good|bad».\n\n")
        } else {
            String::from("Each item is a SENTENCE of a written world with the SOURCE sentence it comes from when known. Answer «good» if the sentence is grammatical English and agrees with its source; otherwise «bad». Answer only with lines «<number> good|bad».\n\n")
        };
        for (k, (t, src)) in batch.iter().enumerate() {
            prompt += &format!("{}. SENTENCE: {t}\n   SOURCE: {}\n", k + 1, if src.is_empty() { "(not quoted)" } else { src.as_str() });
        }
        let (ans, call) = opus.ask("worldbook-judge", bi, batch.len(), &prompt)?;
        prag::opus::append_call(&dir.join("calls.jsonl"), &call)?;
        for l in ans.lines() {
            let mut it = l.split_whitespace();
            if let (Some(_), Some(v)) = (it.next(), it.next()) {
                judged += 1;
                good += usize::from(v.eq_ignore_ascii_case("good"));
            }
        }
    }
    println!("{}: {judged} judged, {good} good ({:.1}%)", args[1], 100.0 * good as f64 / judged.max(1) as f64);
    Ok(())
}
