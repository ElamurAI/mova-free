//! Generator of the in-code dictionary (`en lexicon`): a full run of AGID forms (all paradigms of 112k lemmas)
//! plus "form — tag — lemma" pairs with UD frequencies → two text files in the repo:
//! - `data/lemmas.tsv` — the shared lemma heap: number = line (from zero), **append-only** — numbers
//!   are frozen, so the graph on disk stays readable. New lemmas — by UD frequency, then alphabetically.
//! - `data/forms.tsv` — `form \t tag \t lemma number \t UD frequency \t source`: `u` — UD, `a` — the main
//!   form of an AGID paradigm cell, `r` — a clean form for reverse lookup, `b` — a stem (for the tagger).
//!
//! `build.rs` compiles both files into static hash tables inside the binary (`dict.rs`).

use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

use anyhow::{Context, Result};

use crate::conllu::Sentence;
use crate::gram::Tag;
use crate::hash::{FastMap, FastSet};
use crate::morph::Slot;

/// The first form from an AGID field and whether it is "clean" (no `!` and `?`); variants are comma-separated.
fn agid_form(field: &str) -> Option<(String, bool)> {
    let mut first: Option<(String, bool)> = None;
    for raw in field.split(',') {
        let raw = raw.trim();
        let word: String = raw.split_whitespace().next().unwrap_or("").chars().filter(|c| !matches!(c, '~' | '<' | '!' | '?')).collect();
        if word.is_empty() {
            continue;
        }
        let clean = !(raw.contains('!') || raw.contains('?'));
        if clean {
            return Some((word, true));
        }
        first.get_or_insert((word, false));
    }
    first
}

/// A row of the forms dictionary.
struct Row {
    form: String,
    tag: Tag,
    lemma: String,
    count: u32,
    src: &'static str,
}

pub struct Stats {
    pub lemmas: usize,
    pub new_lemmas: usize,
    pub forms: usize,
    pub rows: usize,
}

pub fn generate(agid: &Path, train: &[Sentence], out: &Path) -> Result<Stats> {
    // 1) UD: (form, tag, lemma) triples in order of first appearance, lemma frequencies
    let mut ud_rows: Vec<Row> = Vec::new();
    let mut ud_idx: FastMap<(String, Tag, String), usize> = FastMap::default();
    let mut ud_freq: FastMap<String, u32> = FastMap::default();
    let mut known: FastSet<String> = FastSet::default();
    for s in train {
        for t in &s.tokens {
            let Some(tag) = t.tag else { continue };
            if t.lemma == "_" {
                continue;
            }
            let k = (t.form.to_lowercase(), tag, t.lemma.clone());
            match ud_idx.get(&k) {
                Some(&i) => ud_rows[i].count += 1,
                None => {
                    ud_idx.insert(k, ud_rows.len());
                    ud_rows.push(Row { form: t.form.to_lowercase(), tag, lemma: t.lemma.clone(), count: 1, src: "u" });
                }
            }
            *ud_freq.entry(t.lemma.clone()).or_default() += 1;
            known.insert(t.lemma.to_lowercase());
        }
    }
    // 2) AGID: main forms of cells (first per lemma and cell), clean ones — for reverse lookup, stems
    let mut agid_rows: Vec<Row> = Vec::new();
    let mut agid_lemmas: Vec<String> = Vec::new();
    let mut main_seen: FastSet<(String, Slot)> = FastSet::default();
    let f = File::open(agid).with_context(|| format!("{}", agid.display()))?;
    for line in BufReader::new(f).lines() {
        let line = line?;
        let Some((head, rest)) = line.split_once(": ") else { continue };
        let mut hp = head.split_whitespace();
        let (Some(lemma), Some(pos)) = (hp.next(), hp.next()) else { continue };
        // "V?", "N?" in the header — the whole entry is doubtful: it does not go into the reverse index
        let questionable = pos.ends_with('?');
        let pos = pos.trim_end_matches('?');
        let fields: Vec<&str> = rest.split('|').collect();
        let slots: &[Slot] = match pos {
            "V" if fields.len() >= 4 => &[Slot::Past, Slot::PastPart, Slot::PresPart, Slot::ThirdSg],
            "V" if fields.len() == 3 => &[Slot::Past, Slot::PresPart, Slot::ThirdSg],
            "N" => &[Slot::Plural],
            "A" => &[Slot::Comp, Slot::Super],
            _ => &[],
        };
        let lemma = lemma.to_lowercase();
        agid_lemmas.push(lemma.clone());
        if !questionable {
            known.insert(lemma.clone());
        }
        let base: &[Tag] = match pos {
            "N" => &[Tag::NN],
            "V" => &[Tag::VB, Tag::VBP],
            "A" => &[Tag::JJ],
            _ => &[],
        };
        for &t in base {
            agid_rows.push(Row { form: lemma.clone(), tag: t, lemma: lemma.clone(), count: 0, src: "b" });
        }
        let mut put = |sl: Slot, form: &str, clean: bool| {
            let form = form.to_lowercase();
            let main = main_seen.insert((lemma.clone(), sl));
            let src = match (main, clean) {
                (true, true) => "ar",
                (true, false) => "a",
                (false, true) => "r",
                (false, false) => return,
            };
            agid_rows.push(Row { form, tag: sl.tag(), lemma: lemma.clone(), count: 0, src });
        };
        for (i, &sl) in slots.iter().enumerate() {
            let Some((form, clean)) = fields.get(i).and_then(|f| agid_form(f)) else { continue };
            let clean = clean && !questionable;
            put(sl, &form, clean);
            if fields.len() == 3 && sl == Slot::Past {
                put(Slot::PastPart, &form, clean);
            }
        }
    }
    // 3) lemma heap: the existing file (numbers frozen) + new ones by UD frequency, then alphabetically
    let lemmas_path = out.join("lemmas.tsv");
    let mut heap: Vec<String> = Vec::new();
    if lemmas_path.exists() {
        for line in BufReader::new(File::open(&lemmas_path)?).lines() {
            let line = line?;
            heap.push(line.split('\t').next().unwrap_or("").to_string());
        }
    }
    let before = heap.len();
    let mut ids: FastMap<String, u32> = heap.iter().enumerate().map(|(i, s)| (s.clone(), i as u32)).collect();
    let mut by_freq: Vec<(&String, &u32)> = ud_freq.iter().collect();
    by_freq.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    for (l, _) in &by_freq {
        push(&mut heap, &mut ids, l);
        push(&mut heap, &mut ids, &l.to_lowercase());
    }
    let mut rest: Vec<&String> = agid_lemmas.iter().collect();
    rest.sort();
    rest.dedup();
    for l in rest {
        push(&mut heap, &mut ids, l);
    }
    // 4) writing
    std::fs::create_dir_all(out)?;
    let mut w = BufWriter::new(File::create(&lemmas_path)?);
    for l in &heap {
        writeln!(w, "{l}\t{}", if known.contains(l) { "k" } else { "-" })?;
    }
    w.flush()?;
    let mut w = BufWriter::new(File::create(out.join("forms.tsv"))?);
    let mut forms: FastSet<&str> = FastSet::default();
    for r in ud_rows.iter().chain(&agid_rows) {
        writeln!(w, "{}\t{}\t{}\t{}\t{}", r.form, r.tag, ids[&r.lemma], r.count, r.src)?;
        forms.insert(&r.form);
    }
    w.flush()?;
    Ok(Stats { lemmas: heap.len(), new_lemmas: heap.len() - before, forms: forms.len(), rows: ud_rows.len() + agid_rows.len() })
}

/// Adds a lemma to the heap if it is not there yet (number — the next one).
fn push(heap: &mut Vec<String>, ids: &mut FastMap<String, u32>, s: &str) {
    if !ids.contains_key(s) {
        ids.insert(s.to_string(), heap.len() as u32);
        heap.push(s.to_string());
    }
}
