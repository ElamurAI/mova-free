//! Idioms, light-verb constructions and phrasal verbs from corpus statistics, the absurdity matrix and the
//! English Wiktionary — the base that keeps idioms out of the absurdity alarms ("open fire", "take place").
//!
//!   world idioms <cells.tsv> <events.tsv> <wikt-multiword.tsv> [--mwe <mwe-en-layer.tsv>] --out <dir> [--min 20] [--books 10]
//!
//! Signals for a verb–object pair (from `world events` over parse trees: syntax-aware association, after
//! Bogdanova 2026, PARSEME 2.0; association over frequency, Dunn 2019; PMI, Church & Hanks 1990):
//! - PMI of verb and object over dependency arcs (count ≥ `--min` in ≥ `--books` books);
//! - the matrix: the literal reading is odd or absurd (OBJ ≥ 2) — a frequent, associated, literally absurd pair
//!   is an idiom or a light-verb construction;
//! - Wiktionary: a multiword entry "verb (det|one's)? noun" (CC BY-SA).
//! Kinds: `idiom` (Wiktionary idiom, or absurd + associated), `light-verb` (make/take/give/have/do/pay/get/
//! put + a noun whose literal reading is odd), `collocation` (associated, literal), `phrasal-verb` (verb +
//! particle, from the events), each with its evidence.

use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use std::path::Path;

use anyhow::{Context, Result};

use crate::absurd::Matrix;

const LIGHT: [&str; 8] = ["make", "take", "give", "have", "do", "pay", "get", "put"];

fn read(path: &Path) -> Result<String> {
    std::fs::read_to_string(path).with_context(|| format!("{}", path.display()))
}

/// Wiktionary multiword verb entries → (verb, noun) → (title, kind, gloss); "open fire", "take place",
/// "make one's way", "kick the bucket".
fn wikt_pairs(text: &str) -> BTreeMap<(String, String), (String, String, String)> {
    let mut out = BTreeMap::new();
    let fillers = ["the", "a", "an", "one's", "someone's", "somebody's", "no", "any", "some", "his", "her", "its", "their"];
    for l in text.lines().skip(1) {
        let c: Vec<&str> = l.split('\t').collect();
        if c.len() < 4 {
            continue;
        }
        // verbal entries only: a verb phrase, an idiom, a phrase (not "ice cream", "post office")
        if !matches!(c[2], "Verb" | "Phrase" | "Idiom") {
            continue;
        }
        let ws: Vec<&str> = c[0].split(' ').collect();
        if ws.len() < 2 || ws.len() > 4 || !ws.iter().all(|w| w.chars().all(|ch| ch.is_alphabetic() || ch == '\'')) {
            continue;
        }
        // verb + optional filler + noun (the last word); only lowercase titles
        let rest: Vec<&str> = ws[1..].iter().copied().filter(|w| !fillers.contains(w)).collect();
        if rest.len() != 1 || c[0] != c[0].to_lowercase() {
            continue;
        }
        let key = (ws[0].to_string(), rest[0].to_string());
        let rank = |k: &str| match k { "idiom" => 0, "verb-phrase" => 1, "phrase" => 2, _ => 3 };
        let better = out.get(&key).is_none_or(|(_, k, _): &(String, String, String)| rank(c[1]) < rank(k));
        if better {
            out.insert(key, (c[0].to_string(), c[1].to_string(), c[3].to_string()));
        }
    }
    out
}

/// The merged multiword layer from the Scientist (`mwe-en-layer.tsv`: expression, type, all_types, n_sources,
/// sources, licence_flags): idioms, LVCs and VPCs whose licence flags include an open or copyleft source (unknown
/// and NC sources are for measurement only) → (verb, object or particle) → (expression, kind, sources).
fn mwe_pairs(text: &str) -> BTreeMap<(String, String), (String, String, String)> {
    let mut out = BTreeMap::new();
    let fillers = ["the", "a", "an", "one's", "someone's", "somebody's", "no", "any", "some", "his", "her", "its", "their", "one"];
    for l in text.lines().skip(1) {
        let c: Vec<&str> = l.split('\t').collect();
        if c.len() < 6 || !(c[5].contains("open") || c[5].contains("copyleft")) {
            continue;
        }
        let kind = match c[1] {
            "idiom" => "idiom",
            "LVC" => "light-verb",
            "VPC" => "phrasal-verb",
            _ => continue,
        };
        let ws: Vec<&str> = c[0].split(' ').filter(|w| !fillers.contains(w)).collect();
        if ws.len() != 2 || !ws.iter().all(|w| w.chars().all(|ch| ch.is_ascii_lowercase())) {
            continue;
        }
        out.entry((ws[0].to_string(), ws[1].to_string())).or_insert((c[0].to_string(), kind.to_string(), c[4].to_string()));
    }
    out
}

/// `world idioms`.
pub fn run(cells: &Path, events: &Path, wikt: &Path, mwe: Option<&Path>, out: &Path, min: usize, min_books: usize) -> Result<()> {
    let m = Matrix::global();
    let mut wk = wikt_pairs(&read(wikt)?);
    if let Some(p) = mwe {
        // the merged layer confirms a pair as a dictionary entry; an idiom from it outranks a Wiktionary verb phrase
        for (k, (e, kind, src)) in mwe_pairs(&read(p)?) {
            let ev = (e, if kind == "light-verb" { "verb-phrase".to_string() } else if kind == "phrasal-verb" { "verb-phrase".to_string() } else { kind }, format!("mwe layer: {src}"));
            match wk.get(&k) {
                Some((_, wkind, _)) if wkind == "idiom" => {}
                _ if ev.1 == "idiom" || !wk.contains_key(&k) => {
                    wk.insert(k, ev);
                }
                _ => {}
            }
        }
    }
    // verb–object counts over arcs
    let mut vo: BTreeMap<(String, String), (usize, usize)> = BTreeMap::new();
    let (mut verb_n, mut noun_n): (BTreeMap<String, usize>, BTreeMap<String, usize>) = (BTreeMap::new(), BTreeMap::new());
    let mut total = 0usize;
    for l in read(cells)?.lines().skip(1) {
        let c: Vec<&str> = l.split('\t').collect();
        if c.len() != 5 || c[2] != "OBJ" || c[0].contains(' ') || !c[1].chars().all(char::is_alphabetic) {
            continue;
        }
        let (n, b): (usize, usize) = (c[3].parse().unwrap_or(0), c[4].parse().unwrap_or(0));
        total += n;
        *verb_n.entry(c[0].to_string()).or_default() += n;
        *noun_n.entry(c[1].to_string()).or_default() += n;
        vo.insert((c[0].to_string(), c[1].to_string()), (n, b));
    }
    std::fs::create_dir_all(out)?;
    let mut rows: Vec<(String, String, &'static str, usize, usize, f64, Option<u8>, String)> = Vec::new();
    for ((v, n), (c, b)) in &vo {
        let w = wk.get(&(v.clone(), n.clone()));
        if (*c < min || *b < min_books) && w.is_none() {
            continue;
        }
        if *c < 3 {
            continue;
        }
        let pmi = ((*c as f64 * total as f64) / (verb_n[v] as f64 * noun_n[n] as f64)).log2();
        let o = m.score(v, n, 1);
        let literal_odd = o.is_some_and(|o| o >= 2);
        let kind = match (w.map(|x| x.1.as_str()), literal_odd, LIGHT.contains(&v.as_str())) {
            (Some("idiom"), _, _) => "idiom",
            (_, true, true) => "light-verb",
            (_, true, false) if pmi >= 3.0 => "idiom",
            (Some(_), _, true) => "light-verb",
            (Some(_), _, _) => "collocation",
            (None, false, _) if pmi >= 5.0 => "collocation",
            _ => continue,
        };
        let ev = w.map_or(String::new(), |(t, k, g)| if g.starts_with("mwe layer") { format!("{g}: {t} ({k})") } else { format!("wiktionary: {t} ({k}) — {g}") });
        rows.push((v.clone(), n.clone(), kind, *c, *b, pmi, o, ev));
    }
    // phrasal verbs from the events: "fall down", "give up"
    let mut prt: BTreeMap<String, (usize, BTreeSet<usize>)> = BTreeMap::new();
    for (k, l) in read(events)?.lines().skip(1).enumerate() {
        let c: Vec<&str> = l.split('\t').collect();
        if c.len() != 6 {
            continue;
        }
        let vs: Vec<&str> = c[1].split(' ').collect();
        if vs.len() == 2 && vs[0] != "not" && vs[0] != "be" {
            let e = prt.entry(c[1].to_string()).or_default();
            e.0 += c[4].parse::<usize>().unwrap_or(0);
            e.1.insert(k % 1); // books are summed per event line below
        }
    }
    let mut f = std::io::BufWriter::new(std::fs::File::create(out.join("idioms.tsv"))?);
    writeln!(f, "verb\tobject\tkind\tcount\tbooks\tpmi\tmatrix_obj\tevidence")?;
    rows.sort_by(|a, b| a.2.cmp(b.2).then(b.3.cmp(&a.3)).then(a.0.cmp(&b.0)).then(a.1.cmp(&b.1)));
    let mut kinds: BTreeMap<&str, usize> = BTreeMap::new();
    for (v, n, k, c, b, p, o, ev) in &rows {
        *kinds.entry(k).or_default() += 1;
        writeln!(f, "{v}\t{n}\t{k}\t{c}\t{b}\t{p:.2}\t{}\t{ev}", o.map_or("-".into(), |x| x.to_string()))?;
    }
    let mut pv: Vec<(&String, usize)> = prt.iter().map(|(k, v)| (k, v.0)).filter(|x| x.1 >= min).collect();
    pv.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    for (v, c) in &pv {
        let (verb, part) = v.split_once(' ').unwrap_or((v, ""));
        let w = wk.get(&(verb.to_string(), part.to_string()));
        writeln!(f, "{verb}\t{part}\tphrasal-verb\t{c}\t-\t-\t-\t{}", w.map_or(String::new(), |(t, k, g)| format!("wiktionary: {t} ({k}) — {g}")))?;
    }
    f.flush()?;
    // the level-1 data file (global/data/idioms.tsv): dictionary-confirmed kinds; corpus-only pairs → candidate
    let mut g = std::io::BufWriter::new(std::fs::File::create(out.join("global-idioms.tsv"))?);
    writeln!(g, "# Verbal multiword expressions (docs/absurdity.md, world idioms): verb, object or particle, kind (idiom | light-verb | collocation | phrasal-verb | candidate — a corpus-only pair, not confirmed by a dictionary, never suppresses an alarm), corpus count, source.")?;
    writeln!(g, "# From 5.78 M sentences of public-domain books (syntax-aware PMI over dependency arcs) + the absurdity matrix + English Wiktionary entries + open/copyleft MWE resources (MAGPIE, PARSEME 1.3 EN, STREUSLE, SemEval-2022 Task 2, WordNet, ConceptNet).")?;
    writeln!(g, "# License: CC BY-SA 4.0 (derived from Wiktionary, STREUSLE, ConceptNet; MAGPIE and PARSEME annotations CC BY 4.0; WordNet licence); free, part of the open release.")?;
    let rank = |k: &str| match k { "phrasal-verb" => 0, "idiom" => 1, "light-verb" => 2, "collocation" => 3, _ => 4 };
    let mut gl: BTreeMap<(String, String), (String, String, String)> = BTreeMap::new();
    for (v, n, k, c, _, _, _, ev) in &rows {
        let src = if ev.starts_with("wiktionary") { "wiktionary".to_string() } else if let Some(x) = ev.strip_prefix("mwe layer: ") { x.split(':').next().unwrap_or("").replace(',', "+") } else { "corpus".into() };
        let kind = if src == "corpus" { "candidate".to_string() } else { k.to_string() };
        if gl.get(&(v.clone(), n.clone())).is_none_or(|x| rank(&kind) < rank(&x.0)) {
            gl.insert((v.clone(), n.clone()), (kind, c.to_string(), src));
        }
    }
    for (v, c) in &pv {
        let (verb, part) = v.split_once(' ').unwrap_or((v, ""));
        gl.insert((verb.to_string(), part.to_string()), ("phrasal-verb".into(), c.to_string(), "corpus".into()));
    }
    for ((v, n), (k, c, s)) in &gl {
        if v.chars().all(char::is_alphabetic) && n.chars().all(char::is_alphabetic) {
            writeln!(g, "{v}\t{n}\t{k}\t{c}\t{s}")?;
        }
    }
    g.flush()?;
    println!("pairs {}  wiktionary verb–noun entries {}  → {}", vo.len(), wk.len(), out.join("idioms.tsv").display());
    for (k, n) in &kinds {
        println!("  {k:12} {n}");
    }
    println!("  phrasal-verb {}", pv.len());
    Ok(())
}
