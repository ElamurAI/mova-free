//! Domain modules of the working mode: slang, formal language, humour, hidden meaning. A domain is switched on by
//! level-1 evidence and says why; when it is on, it changes how words are read (slang senses for slang words).
//!
//!   world domains "<text>" [--genre tale]
//!
//! Evidence (docs/pragmatics-expressions.md, docs/absurdity.md):
//! - slang: a word or expression whose main sense is slang/informal/colloquial/vulgar/derogatory; with two or more
//!   such cues, other content words with a slang sense are read in that sense;
//! - formal: a main sense labelled formal, literary or archaic;
//! - humor: a main sense labelled humorous, jocular, ironic or sarcastic; or absurdity that survives the domain
//!   layer (the matrix in the genre's domain scores an argument 3–4: "the cheese ate the mouse" outside a tale);
//! - hidden meaning: a euphemistic or figurative main sense, or an idiom (its words do not mean what they say).

use std::collections::BTreeMap;

use anyhow::Result;
use en::annotate::Word;

use crate::absurd::{Matrix, judge};
use crate::sense::Node;

/// Formal-register thresholds (legal lexicon z, number of words), chosen on the domains dev set.
const LEGAL_Z: f32 = 20.0;
const LEGAL_K: usize = 3;
/// Register clash (mock-formal humour): children's/everyday register words with z ≥ this in a formal sentence.
const CLASH_Z: f32 = 5.0;

/// A switched-on domain with its reasons.
#[derive(Clone, Debug)]
pub struct Domain {
    pub name: &'static str,
    pub reasons: Vec<String>,
}

/// A word or expression read in a domain's sense.
#[derive(Clone, Debug)]
pub struct DomainReading {
    pub text: String,
    pub domain: &'static str,
    pub meaning: String,
}

#[derive(Clone, Debug, Default)]
pub struct Report {
    pub domains: Vec<Domain>,
    pub readings: Vec<DomainReading>,
    pub means: String,
}

fn push(ds: &mut Vec<Domain>, name: &'static str, why: String) {
    match ds.iter_mut().find(|d| d.name == name) {
        Some(d) => {
            if !d.reasons.contains(&why) {
                d.reasons.push(why)
            }
        }
        None => ds.push(Domain { name, reasons: vec![why] }),
    }
}

/// The slang sense of a register string ("informal: …; slang: …"), if any.
fn slang_sense(register: &str) -> Option<String> {
    register.split("; ").filter_map(|x| x.split_once(": ")).find(|(l, _)| global::register_domain(l.trim()) == Some("slang")).map(|(_, g)| g.to_string()).filter(|g| g.chars().filter(|c| c.is_alphabetic()).count() >= 3)
}

/// Word positions covered by multiword expressions.
fn lens_spans(words: &[Word]) -> Vec<usize> {
    let lemmas: Vec<String> = words.iter().map(|w| w.lemma.to_lowercase()).collect();
    let forms: Vec<String> = words.iter().map(|w| w.form.to_lowercase()).collect();
    let l: Vec<&str> = lemmas.iter().map(String::as_str).collect();
    let f: Vec<&str> = forms.iter().map(String::as_str).collect();
    global::expressions_in_sentence(&f, &l).into_iter().filter(|m| m.end - m.start > 1).flat_map(|m| m.start..m.end).collect()
}

/// Domains of an annotated sentence; `genre` is the matrix domain for the absurdity check ("real" or "tale").
/// The knobs of the domain modules as an abstract configuration (name → value) the SLM can analyse and change
/// (`world tune domains`); `default()` is the shipped setting.
pub fn knobs() -> Vec<(&'static str, f64, Vec<f64>)> {
    vec![
        ("legal_z", LEGAL_Z as f64, vec![6.0, 10.0, 15.0, 20.0, 25.0, 30.0, 40.0]),
        ("legal_k", LEGAL_K as f64, vec![1.0, 2.0, 3.0, 4.0]),
        ("clash_z", CLASH_Z as f64, vec![3.0, 5.0, 10.0, 20.0, 1000.0]),
        ("multiword_min_content", 2.0, vec![1.0, 2.0, 3.0]),
        ("single_word_content_only", 1.0, vec![0.0, 1.0]),
        ("main_sense_only", 1.0, vec![0.0, 1.0]),
        ("idiom_hidden", 1.0, vec![0.0, 1.0]),
        ("absurd_humor_min", 3.0, vec![2.0, 3.0, 4.0, 5.0]),
    ]
}

/// A configuration: knob name → value.
pub type Cfg = BTreeMap<String, f64>;

/// The shipped knob values, overridden by the hot layer: the file named by `WORLD_DOMAINS_CFG` (`name=value` lines),
/// read at start-up — a configuration found by `world tune` can work without recompiling.
pub fn default_cfg() -> Cfg {
    let mut c: Cfg = knobs().into_iter().map(|(k, v, _)| (k.to_string(), v)).collect();
    if let Some(t) = std::env::var("WORLD_DOMAINS_CFG").ok().and_then(|p| std::fs::read_to_string(p).ok()) {
        for l in t.lines().filter(|l| !l.starts_with('#')) {
            if let Some((k, v)) = l.split_once('=') {
                if let (true, Ok(v)) = (c.contains_key(k.trim()), v.trim().parse::<f64>()) {
                    c.insert(k.trim().to_string(), v);
                }
            }
        }
    }
    c
}

pub fn domains(words: &[Word], genre: &str) -> Report {
    domains_with(words, genre, &default_cfg())
}

/// Domains under a configuration.
pub fn domains_with(words: &[Word], genre: &str, cfg: &Cfg) -> Report {
    let k = |n: &str| cfg.get(n).copied().unwrap_or(0.0);
    let lens = prag::lens::lens(words);
    let mut r = Report { means: prag::lens::means(&lens), ..Default::default() };
    use en::gram::UPos;
    let content = |u: UPos| matches!(u, UPos::NOUN | UPos::VERB | UPos::ADJ | UPos::ADV | UPos::INTJ | UPos::PROPN);
    let lemmas_l: Vec<String> = words.iter().map(|w| w.lemma.to_lowercase()).collect();
    let forms_l: Vec<String> = words.iter().map(|w| w.form.to_lowercase()).collect();
    let lr: Vec<&str> = lemmas_l.iter().map(String::as_str).collect();
    let fr: Vec<&str> = forms_l.iter().map(String::as_str).collect();
    for m in global::expressions_in_sentence(&fr, &lr) {
        let span = &words[m.start..m.end];
        let text = span.iter().map(|w| w.form.as_str()).collect::<Vec<_>>().join(" ");
        let e = &m.entry;
        // single words: only content words carry a register ("her" has an informal sense, it is no cue)
        if span.len() == 1 && k("single_word_content_only") > 0.5 && !content(span[0].upos) {
            continue;
        }
        // multiword: at least two content words, or three or more words ("result in", "of a" are grammar)
        let contentful = span.len() >= 3 || span.iter().filter(|w| content(w.upos) && w.upos != UPos::ADV).count() as f64 >= k("multiword_min_content");
        if span.len() > 1 && !contentful {
            continue;
        }
        let main_only = k("main_sense_only") > 0.5;
        for l in e.register.split("; ").filter_map(|x| x.split_once(": ")).filter(|(_, g)| !main_only || *g == e.meaning).map(|(l, _)| l.trim()) {
            if let Some(d) = global::register_domain(l) {
                push(&mut r.domains, d, format!("«{text}» — {l}: {}", e.meaning.trim_end_matches('.')));
            }
        }
        if k("idiom_hidden") > 0.5 && span.len() > 1 && (e.kind == "idiom" || e.kind == "proverb") {
            push(&mut r.domains, "hidden-meaning", format!("«{text}» is an {} — {}", e.kind, e.meaning.trim_end_matches('.')));
        }
    }
    // formal register measured on corpora: content words much more frequent in legal and official texts than in
    // tales and children's books (global::register_z("legal"), log-odds z); thresholds chosen on the dev set
    let (lz, lk): (f32, usize) = (k("legal_z") as f32, k("legal_k") as usize);
    let legal: Vec<(&str, f32)> = words
        .iter()
        .filter(|w| content(w.upos) && w.upos != UPos::PROPN)
        .filter_map(|w| global::register_z("legal", &w.form.to_lowercase()).or_else(|| global::register_z("legal", &w.lemma.to_lowercase())).filter(|z| *z >= lz).map(|z| (w.form.as_str(), z)))
        .collect();
    if legal.len() >= lk {
        push(&mut r.domains, "formal", format!("{} words of the legal/official register (corpus log-odds z ≥ {lz}): {}", legal.len(), legal.iter().map(|(w, z)| format!("{w} {z:.0}")).collect::<Vec<_>>().join(", ")));
    }
    // register clash: a formal sentence about nursery things ("the toddler is hereby charged with the
    // distribution of crayon") — mock-formal humour; the clash words come from the children's register lexicon
    if r.domains.iter().any(|d| d.name == "formal") {
        let cz: f32 = k("clash_z") as f32;
        let homely: Vec<&str> = words
            .iter()
            .filter(|w| matches!(w.upos, UPos::NOUN | UPos::VERB | UPos::ADJ))
            .filter(|w| global::register_z("children", &w.lemma.to_lowercase()).is_some_and(|z| z >= cz) && global::register_z("legal", &w.lemma.to_lowercase()).is_none())
            .map(|w| w.form.as_str())
            .collect();
        if !homely.is_empty() {
            push(&mut r.domains, "humor", format!("register clash: formal language about everyday things ({})", homely.join(", ")));
        }
    }
    // words with a secondary slang sense: two or more switch slang on and are read that way
    let lemmas: Vec<String> = words.iter().map(|w| w.lemma.to_lowercase()).collect();
    let mut secondary: Vec<(usize, String)> = Vec::new();
    for (i, l) in lemmas.iter().enumerate() {
        if let Some(e) = global::expression(l).first() {
            if let Some(g) = slang_sense(e.register) {
                if global::register_domain(e.register.split(':').next().unwrap_or("").trim()) != Some("slang") || g != e.meaning {
                    secondary.push((i, g));
                }
            }
        }
    }
    // a strong slang context (two or more main-sense slang cues) re-reads content words with a secondary slang
    // sense, except words inside a found expression; one cue only switches the domain on (tried 02.10: re-reading
    // on any cue or on secondary senses alone read "fox", "cheese", "ate" as slang everywhere)
    let cues = r.domains.iter().find(|d| d.name == "slang").map_or(0, |d| d.reasons.len());
    let covered: Vec<usize> = lens_spans(words);
    if cues >= 2 {
        for (i, g) in &secondary {
            if !covered.contains(i) && matches!(words[*i].upos, en::gram::UPos::NOUN | en::gram::UPos::ADJ | en::gram::UPos::VERB | en::gram::UPos::ADV) {
                r.readings.push(DomainReading { text: words[*i].form.clone(), domain: "slang", meaning: g.clone() });
            }
        }
    }
    // absurdity that survives the genre's domain layer → humour or irony
    let m = Matrix::global_in(genre);
    let nodes: Vec<Node> = words.iter().map(|w| Node { form: w.form.clone(), lemma: w.lemma.to_lowercase(), upos: Some(w.upos), head: w.head, rel: w.rel }).collect();
    let (js, _) = judge(&m, &nodes);
    for j in js.iter().filter(|j| j.score as f64 >= k("absurd_humor_min")) {
        push(&mut r.domains, "humor", format!("absurd in the {genre} domain: {} as the {} of {} (score {})", words[j.noun].form, if j.role == 0 { "doer" } else { "object" }, words[j.verb].form, j.score));
    }
    r.domains.sort_by_key(|d| d.name);
    r
}

/// `world domains`.
pub fn run(text: &str, genre: &str) -> Result<()> {
    let a = crate::tree::annotator()?;
    let forms: Vec<String> = a.tokenize(text).into_iter().map(|t| t.form).collect();
    let mut words = a.annotate(&forms);
    crate::rerank::repair_words_with(&mut words, "induced", genre);
    let r = domains(&words, genre);
    if r.domains.is_empty() {
        println!("domains: - (literal reading)");
    }
    for d in &r.domains {
        println!("domain {}:", d.name);
        for why in &d.reasons {
            println!("  because {why}");
        }
    }
    for rd in &r.readings {
        println!("read «{}» in the {} sense: {}", rd.text, rd.domain, rd.meaning);
    }
    println!("means: {}", r.means);
    Ok(())
}

/// A labelled domain set annotated once: (words, genre, gold domains).
pub type Labelled = Vec<(Vec<Word>, String, Vec<String>)>;

pub fn load_set(path: &std::path::Path) -> Result<Labelled> {
    let a = crate::tree::annotator()?;
    let mut out = Vec::new();
    for l in std::fs::read_to_string(path)?.lines() {
        let v: serde_json::Value = serde_json::from_str(l)?;
        let genre = if v["genre"] == "tale" { "tale" } else { "real" };
        let gold: Vec<String> = v["domains"].as_array().map(|x| x.iter().filter_map(|d| d.as_str().map(String::from)).collect()).unwrap_or_default();
        let forms: Vec<String> = a.tokenize(v["text"].as_str().unwrap_or("")).into_iter().map(|t| t.form).collect();
        let mut words = a.annotate(&forms);
        crate::rerank::repair_words_with(&mut words, "induced", genre);
        out.push((words, genre.to_string(), gold));
    }
    Ok(out)
}

/// Per-sentence score under a configuration: correct decisions over the four domains (0–4).
pub fn score_set(set: &Labelled, cfg: &Cfg) -> Vec<i64> {
    let names = ["slang", "formal", "humor", "hidden-meaning"];
    set.iter()
        .map(|(w, g, gold)| {
            let pred: Vec<&str> = domains_with(w, g, cfg).domains.iter().map(|d| d.name).collect();
            names.iter().filter(|n| gold.iter().any(|x| x == *n) == pred.contains(n)).count() as i64
        })
        .collect()
}

/// `world domains-eval <set.jsonl>`: per-domain precision and recall on a labelled set (`{"text", "domains",
/// "genre"}`), the false-alarm rate on literal sentences (negative control), and the misses and false alarms.
pub fn eval(path: &std::path::Path, show: usize) -> Result<()> {
    let a = crate::tree::annotator()?;
    let names = ["slang", "formal", "humor", "hidden-meaning"];
    let mut tp = [0usize; 4];
    let mut fp = [0usize; 4];
    let mut fnn = [0usize; 4];
    let (mut lit, mut lit_alarm) = (0usize, 0usize);
    let mut errs: Vec<String> = Vec::new();
    for l in std::fs::read_to_string(path)?.lines() {
        let v: serde_json::Value = serde_json::from_str(l)?;
        let text = v["text"].as_str().unwrap_or("");
        let genre = if v["genre"] == "tale" { "tale" } else { "real" };
        let gold: Vec<&str> = v["domains"].as_array().map(|x| x.iter().filter_map(|d| d.as_str()).collect()).unwrap_or_default();
        let forms: Vec<String> = a.tokenize(text).into_iter().map(|t| t.form).collect();
        let mut words = a.annotate(&forms);
        crate::rerank::repair_words_with(&mut words, "induced", genre);
        let r = domains(&words, genre);
        let pred: Vec<&str> = r.domains.iter().map(|d| d.name).collect();
        if gold.is_empty() {
            lit += 1;
            lit_alarm += !pred.is_empty() as usize;
        }
        for (k, n) in names.iter().enumerate() {
            let (g, p) = (gold.contains(n), pred.contains(n));
            tp[k] += (g && p) as usize;
            fp[k] += (!g && p) as usize;
            fnn[k] += (g && !p) as usize;
            if g != p && errs.len() < show {
                let why = r.domains.iter().find(|d| d.name == *n).map(|d| d.reasons.join("; ")).unwrap_or_default();
                errs.push(format!("{} {n}: {text}{}", if p { "FALSE ALARM" } else { "MISS" }, if why.is_empty() { String::new() } else { format!("\n    because {why}") }));
            }
        }
    }
    let pct = |a: usize, b: usize| if b == 0 { 0.0 } else { 100.0 * a as f64 / b as f64 };
    println!("domain           precision        recall");
    for (k, n) in names.iter().enumerate() {
        println!("{n:16} {:5.1}% ({}/{})  {:5.1}% ({}/{})", pct(tp[k], tp[k] + fp[k]), tp[k], tp[k] + fp[k], pct(tp[k], tp[k] + fnn[k]), tp[k], tp[k] + fnn[k]);
    }
    println!("literal sentences with a false alarm: {lit_alarm}/{lit} = {:.1}% (negative control)", pct(lit_alarm, lit));
    for e in errs {
        println!("{e}");
    }
    Ok(())
}
