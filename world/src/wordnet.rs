//! Open English WordNet (CC BY 4.0) read for the school's runtime: a word's senses and its kind («ate» → consume).
//! The explanatory dictionaries themselves are built by a separate, closed tool.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

pub fn oewn_dir() -> PathBuf {
    PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| format!("{}/mova-data", std::env::var("HOME").unwrap_or_default()))).join("raw/en-oewn/json")
}

/// A sense: part of speech, definition, examples, the words of its kind (hypernym), its synonyms.
#[derive(Clone, Debug)]
pub struct Sense {
    pub pos: String,
    pub definition: String,
    pub examples: Vec<String>,
    pub kind: Vec<String>,
    pub synonyms: Vec<String>,
    /// its synset's id
    pub synset: String,
}

pub struct WordNet {
    /// lemma (lower case) → senses in order
    pub senses: BTreeMap<String, Vec<Sense>>,
    /// synset id → (its hypernyms' ids, its first member): the way up by meaning, not by word
    pub up: BTreeMap<String, (Vec<String>, String)>,
}

fn pos_name(p: &str) -> &'static str {
    match p {
        "n" => "noun",
        "v" => "verb",
        "a" | "s" => "adjective",
        "r" => "adverb",
        _ => "word",
    }
}

impl WordNet {
    pub fn load(dir: &Path) -> Result<WordNet> {
        let mut files: Vec<PathBuf> = std::fs::read_dir(dir).with_context(|| format!("{}", dir.display()))?.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "json")).collect();
        files.sort();
        // synsets: id → (pos, definition, examples, hypernyms, members)
        let mut syn: BTreeMap<String, (String, String, Vec<String>, Vec<String>, Vec<String>)> = BTreeMap::new();
        let mut entries: Vec<(String, String, Vec<String>)> = Vec::new();
        let strs = |v: &serde_json::Value| -> Vec<String> { v.as_array().map(|a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect()).unwrap_or_default() };
        for f in &files {
            let name = f.file_name().unwrap().to_string_lossy().to_string();
            let v: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(f)?)?;
            let Some(obj) = v.as_object() else { continue };
            if name.starts_with("entries-") {
                for (lemma, by_pos) in obj {
                    for (pos, e) in by_pos.as_object().into_iter().flatten() {
                        let ids: Vec<String> = e["sense"].as_array().map(|a| a.iter().filter_map(|s| s["synset"].as_str().map(String::from)).collect()).unwrap_or_default();
                        entries.push((lemma.to_lowercase(), pos.clone(), ids));
                    }
                }
            } else if name != "frames.json" {
                for (id, s) in obj {
                    syn.insert(id.clone(), (s["partOfSpeech"].as_str().unwrap_or("").to_string(), strs(&s["definition"]).join("; "), strs(&s["example"]), strs(&s["hypernym"]), strs(&s["members"])));
                }
            }
        }
        let mut senses: BTreeMap<String, Vec<Sense>> = BTreeMap::new();
        for (lemma, pos, ids) in entries {
            for id in ids {
                let Some((p, def, ex, hyp, members)) = syn.get(&id) else { continue };
                let kind: Vec<String> = hyp.iter().filter_map(|h| syn.get(h)).flat_map(|x| x.4.iter().take(2).cloned()).take(3).collect();
                let synonyms: Vec<String> = members.iter().filter(|m| m.to_lowercase() != lemma).take(5).cloned().collect();
                senses.entry(lemma.clone()).or_default().push(Sense { pos: pos_name(if p.is_empty() { &pos } else { p }).into(), definition: def.clone(), examples: ex.iter().take(2).cloned().collect(), kind, synonyms, synset: id.clone() });
            }
        }
        let up = syn.iter().map(|(id, x)| (id.clone(), (x.3.clone(), x.4.first().cloned().unwrap_or_default().to_lowercase()))).collect();
        Ok(WordNet { senses, up })
    }
}

/// The WordNet, loaded once. Missing, unreadable or empty OEWN files stop the process with a message that names
/// the directory and how to install it (`./install.sh`). Only with `MOVA_OEWN=optional` does the reader run on
/// an empty WordNet instead, after one warning on stderr: kinds, synonyms and senses are then absent.
pub fn wordnet() -> &'static WordNet {
    static W: std::sync::OnceLock<WordNet> = std::sync::OnceLock::new();
    W.get_or_init(|| {
        let dir = oewn_dir();
        let optional = std::env::var("MOVA_OEWN").is_ok_and(|v| v == "optional");
        match loaded_or_missing(&dir, WordNet::load(&dir), optional) {
            Ok(w) => w,
            Err(msg) => panic!("{msg}"),
        }
    })
}

/// The loaded WordNet, or — when the load failed or found no entries — the explanation of what is missing:
/// an error, or with `optional` an empty WordNet after a warning on stderr.
fn loaded_or_missing(dir: &Path, r: Result<WordNet>, optional: bool) -> std::result::Result<WordNet, String> {
    let why = match r {
        Ok(w) if !w.senses.is_empty() => return Ok(w),
        Ok(_) => "no WordNet entries there".to_string(),
        Err(e) => format!("{e:#}"),
    };
    let msg = format!(
        "Open English WordNet is missing: {} ({why}). The school's world model reads kinds, synonyms and senses from it. \
         Install it with ./install.sh (Open English WordNet 2025 JSON, CC BY 4.0) or point MOVA_DATA at a data directory \
         that has raw/en-oewn/json; set MOVA_OEWN=optional to run without it on purpose (results that need it degrade).",
        dir.display()
    );
    if optional {
        eprintln!("warning: {msg}");
        return Ok(WordNet { senses: BTreeMap::new(), up: BTreeMap::new() });
    }
    Err(msg)
}

/// A word's kind in WordNet — the first hypernym of its first sense of that part of speech («ate» → consume,
/// «peaches» → edible fruit): the plain form found by taking off the endings, irregular forms by the mentions' stems.
pub fn kind_of(word: &str, pos: &str) -> Option<String> {
    let wn = wordnet();
    plain_forms(word).into_iter().find_map(|t| wn.senses.get(&t).and_then(|ss| ss.iter().find(|s| s.pos == pos)).and_then(|s| s.kind.first().cloned()))
}

/// The other words of a word's first sense of that part of speech, lower case («doc» → doctor, physician, md …).
pub fn synonyms_of(word: &str, pos: &str) -> Vec<String> {
    first_sense(word, pos).map(|s| s.synonyms.iter().map(|x| x.to_lowercase()).collect()).unwrap_or_default()
}

/// A word's kinds all the way up, by meaning: the first sense's synset, then its hypernyms' (their first members,
/// lower case) — «dog» → canine → carnivore → placental → mammal → vertebrate …; going up by the word instead would
/// read «canine» in its own first sense, the tooth.
pub fn kinds_up(word: &str, pos: &str) -> Vec<String> {
    first_sense(word, pos).map(|s| synset_up(&s.synset)).unwrap_or_default()
}

/// The kinds all the way up of each sense of a word of that part of speech, in the senses' order.
pub fn senses_up(word: &str, pos: &str) -> Vec<Vec<String>> {
    let wn = wordnet();
    plain_forms(word).into_iter().find_map(|t| wn.senses.get(&t).filter(|ss| ss.iter().any(|s| s.pos == pos)))
        .map(|ss| ss.iter().filter(|s| s.pos == pos).map(|s| synset_up(&s.synset)).collect()).unwrap_or_default()
}

/// A synset's kinds all the way up: every hypernym, level by level, the nearer first («dog» is a canine and a domestic
/// animal: both, then theirs), capped at the first 40 distinct member labels.
fn synset_up(synset: &str) -> Vec<String> {
    synset_up_in(wordnet(), synset)
}

fn synset_up_in(wn: &WordNet, synset: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut level: Vec<String> = vec![synset.to_string()];
    let mut seen: std::collections::BTreeSet<String> = level.iter().cloned().collect();
    while !level.is_empty() && out.len() < 40 {
        let mut next = Vec::new();
        for i in &level {
            for h in wn.up.get(i).map(|x| x.0.clone()).unwrap_or_default() {
                if seen.insert(h.clone()) {
                    if let Some((_, m)) = wn.up.get(&h) {
                        if !out.contains(m) {
                            out.push(m.clone());
                            if out.len() == 40 {
                                return out;
                            }
                        }
                    }
                    next.push(h);
                }
            }
        }
        level = next;
    }
    out
}

/// A word's first sense of that part of speech: the plain form found by taking off the endings, irregular forms by the
/// mentions' stems.
fn first_sense(word: &str, pos: &str) -> Option<&'static Sense> {
    first_sense_in(wordnet(), word, pos)
}

fn first_sense_in<'a>(wn: &'a WordNet, word: &str, pos: &str) -> Option<&'a Sense> {
    plain_forms(word).into_iter().find_map(|t| wn.senses.get(&t).and_then(|ss| ss.iter().find(|s| s.pos == pos)))
}

/// The forms a word may have in the dictionary: itself, with the endings taken off, its irregular stem.
fn plain_forms(word: &str) -> Vec<String> {
    let w = word.to_lowercase();
    let irregular = crate::mentions::stem(&w);
    let mut tries = vec![w.clone()];
    for (suf, add) in [("ies", "y"), ("es", ""), ("s", ""), ("ed", ""), ("ed", "e"), ("d", ""), ("ing", ""), ("ing", "e")] {
        if let Some(b) = w.strip_suffix(suf) {
            tries.push(format!("{b}{add}"));
        }
    }
    tries.push(irregular);
    tries
}

#[cfg(test)]
mod wave10_models {
    use super::*;
    use std::collections::BTreeSet;

    fn empty() -> WordNet {
        WordNet {
            senses: BTreeMap::new(),
            up: BTreeMap::new(),
        }
    }

    /// A missing or empty OEWN directory is an error naming the directory and install.sh, not a silent empty
    /// WordNet (red with the earlier `unwrap_or(empty)`); `optional` gives the empty WordNet on purpose.
    #[test]
    fn missing_oewn_is_an_error_unless_optional() {
        let dir = std::env::temp_dir().join(format!("mova-oewn-missing-{}", std::process::id()));
        let msg = loaded_or_missing(&dir, WordNet::load(&dir), false).err().expect("missing OEWN must be an error");
        assert!(msg.contains(&dir.display().to_string()) && msg.contains("./install.sh") && msg.contains("MOVA_OEWN=optional"), "{msg}");
        assert!(loaded_or_missing(&dir, Ok(empty()), false).is_err(), "an empty directory is missing data too");
        assert!(loaded_or_missing(&dir, WordNet::load(&dir), true).unwrap().senses.is_empty());
        let mut one = empty();
        one.senses.insert("dog".into(), Vec::new());
        assert!(loaded_or_missing(&dir, Ok(one), false).is_ok());
    }

    #[test]
    fn ancestry_exact_output_cap() {
        let mut wn = empty();
        let ids: Vec<String> = (0..55).map(|i| format!("h{i:02}")).collect();
        wn.up.insert("root".into(), (ids.clone(), "root".into()));
        for id in &ids {
            wn.up.insert(id.clone(), (vec![], id.clone()));
        }
        assert_eq!(synset_up_in(&wn, "root"), ids[..40]);
    }

    #[test]
    fn ancestry_finite_reachability_oracle() {
        // All 512 directed graphs on three labeled nodes, including cycles.
        for mask in 0..512 {
            let mut wn = empty();
            let mut reach = [[false; 3]; 3];
            for i in 0..3 {
                let mut edges = vec![];
                for j in 0..3 {
                    reach[i][j] = mask & (1 << (i * 3 + j)) != 0;
                    if reach[i][j] {
                        edges.push(j.to_string());
                    }
                }
                wn.up.insert(i.to_string(), (edges, i.to_string()));
            }
            // Transitive closure is independent of the production frontier traversal.
            for k in 0..3 {
                for i in 0..3 {
                    for j in 0..3 {
                        reach[i][j] |= reach[i][k] && reach[k][j];
                    }
                }
            }
            for start in 0..3 {
                let want: BTreeSet<String> = (0..3)
                    .filter(|&j| j != start && reach[start][j])
                    .map(|j| j.to_string())
                    .collect();
                let got = synset_up_in(&wn, &start.to_string());
                assert_eq!(
                    got.iter().cloned().collect::<BTreeSet<_>>(),
                    want,
                    "{mask}/{start}"
                );
                assert_eq!(got.len(), want.len(), "unique labels");
            }
        }
    }

    #[test]
    fn ancestry_preserves_levels_and_label_ambiguity() {
        let mut wn = empty();
        for (id, edges, label) in [
            ("r", vec!["a", "b", "missing"], "root"),
            ("a", vec!["c"], "same"),
            ("b", vec!["d"], "same"),
            ("c", vec!["r"], "deep-c"),
            ("d", vec![], "deep-d"),
        ] {
            wn.up.insert(
                id.into(),
                (edges.into_iter().map(String::from).collect(), label.into()),
            );
        }
        assert_eq!(synset_up_in(&wn, "r"), vec!["same", "deep-c", "deep-d"]);
        assert!(synset_up_in(&wn, "absent").is_empty());
    }

    #[test]
    fn normalization_and_first_sense_fibers() {
        let sense = |id: &str, pos: &str| Sense {
            pos: pos.into(),
            definition: id.into(),
            examples: vec![],
            kind: vec![],
            synonyms: vec![],
            synset: id.into(),
        };
        let mut wn = empty();
        wn.senses.insert("cats".into(), vec![sense("v", "verb")]);
        wn.senses
            .insert("cat".into(), vec![sense("n1", "noun"), sense("n2", "noun")]);
        for spelling in ["cats", "CATS", "Cats"] {
            assert_eq!(first_sense_in(&wn, spelling, "noun").unwrap().synset, "n1");
            assert_eq!(first_sense_in(&wn, spelling, "verb").unwrap().synset, "v");
        }
        assert!(first_sense_in(&wn, "cat", "NOUN").is_none());
        assert_eq!(plain_forms("TRIES")[..4], ["tries", "try", "tri", "trie"]);
        assert_eq!(plain_forms("")[0], "");
    }

    #[test]
    fn supplied_json_snapshot_projection() {
        let dir = std::env::temp_dir().join(format!("mova-wordnet-wave10-{}", std::process::id()));
        std::fs::create_dir(&dir).unwrap();
        std::fs::write(
            dir.join("entries-a.json"),
            r#"{"Alpha":{"n":{"sense":[{"synset":"s"},{"synset":"missing"}]}}}"#,
        )
        .unwrap();
        std::fs::write(
            dir.join("a.json"),
            r#"{"s":{"definition":["old"],"members":["Alpha"]}}"#,
        )
        .unwrap();
        std::fs::write(
            dir.join("z.json"),
            r#"{
            "s":{"partOfSpeech":"n","definition":["new","second"],"example":["a","b","c"],
                 "hypernym":["h","j"],"members":["Alpha","ALPHA","Beta","Gamma","D","E","F","G"]},
            "h":{"members":["H1","H2","H3"]},
            "j":{"members":["J1","J2"]}
        }"#,
        )
        .unwrap();
        std::fs::write(dir.join("frames.json"), r#"{"s":{"members":["wrong"]}}"#).unwrap();
        let wn = WordNet::load(&dir).unwrap();
        // Loading a changed directory does not mutate an already returned snapshot.
        std::fs::write(dir.join("z.json"), "{}").unwrap();
        let changed = WordNet::load(&dir).unwrap();
        std::fs::remove_dir_all(&dir).unwrap();
        let s = first_sense_in(&wn, "ALPHA", "noun").unwrap();
        assert_eq!(wn.senses["alpha"].len(), 1);
        assert_eq!(s.definition, "new; second");
        assert_eq!(s.examples, ["a", "b"]);
        assert_eq!(s.kind, ["H1", "H2", "J1"]);
        assert_eq!(s.synonyms, ["Beta", "Gamma", "D", "E", "F"]);
        assert_eq!(wn.up["s"].1, "alpha");
        assert_eq!(
            first_sense_in(&changed, "alpha", "noun")
                .unwrap()
                .definition,
            "old"
        );
    }
}
