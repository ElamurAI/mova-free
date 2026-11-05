//! One world for all languages: when an entity named in Hebrew and one named in English are one entity.
//!
//! Two links, and no other: **the verse** — a Hebrew piece that is a verse of the Hebrew Bible (its letters looked up
//! in an index of the WLC) and an English piece of the same volume that is that verse in JPS 1917 (looked up in an
//! index of its sentences): a Hebrew entity of the one and an English entity of the other are one when the English id
//! is among the dictionary's English words for the Hebrew lemma (`tanakh-en.tsv`: MACULA's commonest glosses and the
//! OpenScriptures definition); **the name** — a Hebrew name and an English name anywhere in the volume, by the same
//! dictionary. A Hebrew lemma joins at most one English id, and an English id at most one Hebrew lemma. Otherwise the
//! two stay two entities.
//!
//! Measured against MACULA's own gloss of the word in that verse (the gold trees' `Gloss=`, read in context, not the
//! dictionary's commonest glosses): a link is right when the gloss of a word with the Hebrew lemma in the verse holds
//! the English id. A link from a piece with no verse (Mishnaic or Modern Hebrew) is not verified.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use anyhow::{Context, Result};

use crate::frames::F;

/// The resources of linking, and the gold to measure it.
#[derive(Default)]
pub struct Lex {
    /// the letters of a WLC verse → the verse
    wlc: BTreeMap<String, String>,
    /// a JPS sentence (spaces collapsed) → the verse
    jps: BTreeMap<String, String>,
    /// the consonants of a Hebrew lemma → English words of its glosses and definition
    gloss: BTreeMap<String, BTreeSet<String>>,
    /// a verse → its words: consonants of the lemma, MACULA's gloss words
    gold: BTreeMap<String, Vec<(String, BTreeSet<String>)>>,
}

const STOP: [&str; 24] = ["the", "a", "an", "of", "to", "and", "in", "on", "at", "by", "for", "with", "from", "be", "is", "he", "she", "it", "they", "his", "her", "that", "this", "x"];

/// English words of a gloss or a definition, with their lemmas from `en`'s dictionary (men → man).
fn words_of(s: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for w in s.split(|c: char| !c.is_alphabetic()).map(str::to_lowercase).filter(|w| !w.is_empty() && !STOP.contains(&w.as_str())) {
        if let Some(f) = en::dict::form(&w) {
            for r in en::dict::analyses(f) {
                let l = en::dict::text(r.lemma()).to_lowercase();
                if !l.is_empty() {
                    out.insert(l);
                }
            }
        }
        out.insert(w);
    }
    out
}

/// The letters of a Hebrew text, a space between words.
pub fn letters(s: &str) -> String {
    let mut out = String::new();
    for w in s.split(|c: char| c.is_whitespace() || c == '\u{05BE}') {
        let l = macula::consonants(w);
        if !l.is_empty() {
            if !out.is_empty() {
                out.push(' ');
            }
            out.push_str(&l);
        }
    }
    out
}

pub fn collapse(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

impl Lex {
    pub fn load(judaism: &Path) -> Result<Lex> {
        let d = judaism.join("dict");
        let mut lex = Lex::default();
        let mut files: Vec<_> = std::fs::read_dir(d.join("wlc-trees")).context("wlc-trees")?.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "conllu")).collect();
        files.sort();
        for f in files {
            let mut verse = String::new();
            for l in std::fs::read_to_string(&f)?.lines() {
                if let Some(v) = l.strip_prefix("# verse = ") {
                    verse = v.split_whitespace().next().unwrap_or("").to_string();
                } else if let Some(t) = l.strip_prefix("# text = ") {
                    lex.wlc.entry(letters(t)).or_insert_with(|| verse.clone());
                } else if !l.starts_with('#') && !l.is_empty() {
                    let c: Vec<&str> = l.split('\t').collect();
                    if c.len() == 10 && !c[0].contains('-') {
                        if let Some(g) = c[9].split('|').find_map(|kv| kv.strip_prefix("Gloss=")) {
                            lex.gold.entry(verse.clone()).or_default().push((macula::consonants(c[2]), words_of(g)));
                        }
                    }
                }
            }
        }
        let verses: BTreeMap<String, String> = std::fs::read_to_string(d.join("tanakh-jps1917.tsv.verses"))?.lines().filter_map(|l| l.split_once('\t')).map(|(a, b)| (a.to_string(), b.to_string())).collect();
        for l in std::fs::read_to_string(d.join("tanakh-jps1917.tsv"))?.lines() {
            if let Some((id, t)) = l.split_once('\t') {
                if let Some(v) = verses.get(id) {
                    lex.jps.entry(collapse(t)).or_insert_with(|| v.clone());
                }
            }
        }
        for l in std::fs::read_to_string(d.join("tanakh-en.tsv"))?.lines().skip(1) {
            let c: Vec<&str> = l.split('\t').collect();
            if c.len() >= 5 {
                let g: String = c[3].split(';').map(|x| x.split('(').next().unwrap_or("")).collect::<Vec<_>>().join(" ");
                let e = lex.gloss.entry(macula::consonants(c[1])).or_default();
                e.extend(words_of(&g));
                e.extend(words_of(c[4]));
            }
        }
        Ok(lex)
    }

    pub fn wlc_verse(&self, hebrew: &str) -> Option<&String> {
        self.wlc.get(&letters(hebrew))
    }

    pub fn jps_verse(&self, english: &str) -> Option<&String> {
        self.jps.get(&collapse(english))
    }
}

/// How entities are linked: the rules above, none, or the negative control (the gloss alone, any Hebrew piece with
/// any English piece of the volume, no verse needed).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinkMode {
    Align,
    None,
    Any,
}

/// A piece as linking sees it.
pub struct PieceInfo<'a> {
    pub heb: bool,
    /// the verses of the piece, found by the indexes
    pub verses: BTreeSet<String>,
    /// the gold verse of the piece's item (for measuring)
    pub gold_verse: Option<String>,
    pub frames: Vec<&'a F>,
}

#[derive(Clone, Debug, Default)]
pub struct Stats {
    pub verse_links: u64,
    pub name_links: u64,
    pub right: u64,
    pub wrong: u64,
    pub unverified: u64,
    pub missed: u64,
    pub pairs: u64,
    pub examples: Vec<String>,
}

impl Stats {
    pub fn merge(&mut self, o: &Stats) {
        self.verse_links += o.verse_links;
        self.name_links += o.name_links;
        self.right += o.right;
        self.wrong += o.wrong;
        self.unverified += o.unverified;
        self.missed += o.missed;
        self.pairs += o.pairs;
        for e in &o.examples {
            if self.examples.len() < 1000 {
                self.examples.push(e.clone());
            }
        }
    }
}

/// The Hebrew id → English id links of a volume, and their measure.
pub fn link(lex: &Lex, pieces: &[PieceInfo], mode: LinkMode) -> (BTreeMap<String, String>, Stats) {
    let mut st = Stats::default();
    let mut map: BTreeMap<String, String> = BTreeMap::new();
    let mut taken: BTreeSet<String> = BTreeSet::new();
    if mode == LinkMode::None {
        return (map, st);
    }
    let ents = |p: &PieceInfo| -> Vec<(String, bool)> { p.frames.iter().flat_map(|f| f.ents()).map(|e| (e.id.clone(), e.named)).collect() };
    let gold_ok = |verse: &Option<String>, h: &str, e: &str| -> Option<bool> {
        let v = verse.as_ref()?;
        let ws = lex.gold.get(v)?;
        let c = macula::consonants(h);
        Some(ws.iter().any(|(l, g)| *l == c && g.contains(&e.to_lowercase())))
    };
    let mut try_link = |h: &str, e: &str, verse: &Option<String>, rule: &str, map: &mut BTreeMap<String, String>, st: &mut Stats| {
        if map.contains_key(h) || taken.contains(e) {
            return;
        }
        map.insert(h.to_string(), e.to_string());
        taken.insert(e.to_string());
        if rule == "verse" {
            st.verse_links += 1;
        } else {
            st.name_links += 1;
        }
        let ok = gold_ok(verse, h, e);
        match ok {
            Some(true) => st.right += 1,
            Some(false) => st.wrong += 1,
            None => st.unverified += 1,
        }
        if st.examples.len() < 1000 {
            st.examples.push(format!("{rule}: {h} = {e} ({}) in {}", match ok {
                Some(true) => "right",
                Some(false) => "wrong",
                None => "not verified",
            }, verse.clone().unwrap_or_else(|| "-".into())));
        }
    };
    // the verse: pairs of a Hebrew and an English piece of one verse (the control: any pair)
    for (i, hp) in pieces.iter().enumerate().filter(|(_, p)| p.heb) {
        for ep in pieces.iter().filter(|p| !p.heb) {
            let aligned = mode == LinkMode::Any || hp.verses.intersection(&ep.verses).next().is_some();
            if !aligned {
                continue;
            }
            st.pairs += 1;
            let ees = ents(ep);
            for (h, _) in ents(hp) {
                let Some(g) = lex.gloss.get(&macula::consonants(&h)) else { continue };
                if let Some((e, _)) = ees.iter().find(|(e, _)| g.contains(&e.to_lowercase())) {
                    try_link(&h, e, &hp.gold_verse, "verse", &mut map, &mut st);
                }
            }
            // what the gold says should have been joined in this pair
            if mode == LinkMode::Align {
                if let Some(ws) = hp.gold_verse.as_ref().and_then(|v| lex.gold.get(v)) {
                    let hs: BTreeSet<String> = ents(hp).into_iter().map(|(h, _)| macula::consonants(&h)).collect();
                    for (l, g) in ws {
                        if !hs.contains(l) {
                            continue;
                        }
                        if let Some((e, _)) = ees.iter().find(|(e, _)| g.contains(&e.to_lowercase())) {
                            let linked = map.iter().any(|(h, x)| macula::consonants(h) == *l && x == e);
                            if !linked {
                                st.missed += 1;
                            }
                        }
                    }
                }
            }
            let _ = i;
        }
    }
    // the name: a Hebrew name and an English name anywhere in the volume
    for hp in pieces.iter().filter(|p| p.heb) {
        for (h, named) in ents(hp) {
            if !named {
                continue;
            }
            let Some(g) = lex.gloss.get(&macula::consonants(&h)) else { continue };
            for ep in pieces.iter().filter(|p| !p.heb) {
                if let Some((e, _)) = ents(ep).into_iter().find(|(e, n)| *n && g.contains(&e.to_lowercase())) {
                    try_link(&h, &e, &hp.gold_verse, "name", &mut map, &mut st);
                }
            }
        }
    }
    (map, st)
}

/// Frames with the linked Hebrew ids renamed (and back).
pub fn rename(fs: &[F], map: &BTreeMap<String, String>) -> Vec<F> {
    let r = |id: &mut String| {
        if let Some(x) = map.get(id) {
            *id = x.clone();
        }
    };
    fs.iter()
        .map(|f| {
            let mut f = f.clone();
            for e in [&mut f.agent, &mut f.patient, &mut f.recipient, &mut f.place].into_iter().flatten() {
                r(&mut e.id);
                e.and.iter_mut().for_each(r);
            }
            for (_, e) in f.obls.iter_mut() {
                r(&mut e.id);
                e.and.iter_mut().for_each(r);
            }
            f
        })
        .collect()
}

pub fn inverse(map: &BTreeMap<String, String>) -> BTreeMap<String, String> {
    map.iter().map(|(a, b)| (b.clone(), a.clone())).collect()
}
#[cfg(test)]
mod wave32_tests {
    use super::*;
    #[test]
    fn greedy_links_and_inverse() {
        let mut lex = Lex::default();
        lex.gloss.insert("אב".into(), ["father".into()].into());
        let h = F {
            agent: Some(crate::frames::E {
                id: "אב".into(),
                named: true,
                ..Default::default()
            }),
            ..Default::default()
        };
        let e = F {
            agent: Some(crate::frames::E {
                id: "father".into(),
                named: true,
                ..Default::default()
            }),
            ..Default::default()
        };
        let pieces = [
            PieceInfo {
                heb: true,
                verses: BTreeSet::new(),
                gold_verse: None,
                frames: vec![&h],
            },
            PieceInfo {
                heb: false,
                verses: BTreeSet::new(),
                gold_verse: None,
                frames: vec![&e],
            },
        ];
        assert!(link(&lex, &pieces, LinkMode::None).0.is_empty());
        let (map, stats) = link(&lex, &pieces, LinkMode::Align);
        assert_eq!(map.get("אב").map(String::as_str), Some("father"));
        assert_eq!(
            (stats.name_links, stats.verse_links, stats.unverified),
            (1, 0, 1)
        );
        assert_eq!(rename(&rename(&[h.clone()], &map), &inverse(&map)), vec![h]);
        assert_eq!(collapse(" a\t b\n"), "a b");
        assert_eq!(letters("אָב־אָב"), "אב אב");
    }
}
