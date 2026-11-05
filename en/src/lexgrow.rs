//! Generator of the lexicon extension (`en lexgrow`): ordinary words the in-code dictionary (`data/forms.tsv`, AGID +
//! UD) lacks, from sources whose licences allow shipping them with Mova, with the provenance of every row:
//!
//! - **Open English WordNet 2025** (`o`, CC BY 4.0, the JSON edition, *not* the `-plus` edition, which takes entries
//!   from Wiktionary): lemmas of nouns, verbs, adjectives and adverbs, and the irregular forms it lists;
//! - **the SPECIALIST Lexicon 2026** (`s`, NLM, SPECIALIST NLP Tools Terms and Conditions: a U.S. Government product,
//!   free to use and redistribute with attribution and the release): every inflected form of a noun, verb, adjective
//!   or adverb (`LRAGR`), with its EUI, and the acronyms with their expansions (`LEXICON`, `acronym_of=`);
//! - **2010 U.S. Census surnames** (`c`, a U.S. Government work, public domain): surnames borne by at least
//!   `SURNAME_MIN` people, as proper nouns;
//! - **the Names Corpus** 1.3 (`k`, Mark Kantrowitz, 1991, with additions by Bill Ross: free for any purpose with
//!   credit, redistributable with its README, which the output's header carries): given names, as proper nouns.
//!
//! What a row adds is only what the dictionary does not already have: a form the dictionary knows with the same tag
//! is left out. A form the dictionary knows with other tags keeps the new tag as a row (*directional* JJ next to the
//! dictionary's NN), so derivations on top of it (*bidirectional*) get the class right; the taggers still take a
//! known form's tags from the dictionary only.
//!
//! Inflection is taken, never guessed, where a source gives it: SPECIALIST lists every form (an adjective gets
//! -er/-est only when SPECIALIST lists them; *lexical* has none). A WordNet lemma SPECIALIST does not have gets the
//! regular forms of `morph` (no consonant doubling; nouns their -s plural, verbs -s/-ed/-ing), or, when WordNet lists
//! irregular forms, those forms only (for nouns as NNS, for adjectives as JJR/JJS by their ending; a verb's
//! irregular forms carry no slot in WordNet, so such a verb gets its base only). AGID's `dontuse` list (forms its
//! author marked wrong: *bookshelfs*) is never written.
//!
//! Proper nouns are conservative: a capitalised WordNet noun or SPECIALIST noun is NNP (no plural), a surname only
//! when no source has the form as a common word. Never Wiktionary or Wikipedia: they are CC BY-SA.
//!
//! Output (`data/`): `lexicon-ext.tsv` (`form \t tag \t lemma \t sources`, the lemma `=` when it is the form) and `acronyms.tsv` (`acronym \t
//! expansion \t source`), sorted, compiled into the binary by `build.rs` (`lexext.rs` reads them).

use std::collections::{BTreeMap, BTreeSet};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

use crate::dict;
use crate::gram::Tag;

/// Fewest bearers of a 2010 Census surname for it to be a proper noun here.
pub const SURNAME_MIN: u64 = 1000;
/// Most expansions kept for one acronym (SPECIALIST order).
pub const MAX_EXPANSIONS: usize = 8;

/// Inputs of a run.
pub struct Sources {
    /// The OEWN JSON folder (`entries-*.json`).
    pub oewn: Option<PathBuf>,
    /// SPECIALIST `LEXICON` (units) and `LRAGR` (inflections).
    pub specialist: Option<(PathBuf, PathBuf)>,
    /// `Names_2010Census.csv`.
    pub census: Option<PathBuf>,
    /// The Kantrowitz Names Corpus folder (`male.txt`, `female.txt`, `README`).
    pub first_names: Option<PathBuf>,
    /// AGID's `dontuse` list.
    pub dontuse: Option<PathBuf>,
}

#[derive(Default, Debug)]
pub struct Stats {
    pub rows: usize,
    pub forms: usize,
    pub new_forms: usize,
    pub lemmas: usize,
    pub acronyms: usize,
    pub by_source: BTreeMap<char, usize>,
}

/// A form the extension can hold: letters (any script), inner hyphens and apostrophes, at least two letters.
pub fn word_ok(s: &str) -> bool {
    let n = s.chars().filter(|c| c.is_alphabetic()).count();
    n >= 2 && s.chars().next().is_some_and(char::is_alphabetic) && !s.ends_with(['-', '\'']) && s.chars().all(|c| c.is_alphabetic() || c == '-' || c == '\'') && s.chars().count() <= 40
}

/// An acronym: no spaces, letters and digits, at least two capitals and no lower-case letter after the first two
/// characters run in capitals (*GPU, NLP, LHCb* yes; *Smith* no).
pub fn acronym_ok(s: &str) -> bool {
    let caps = s.chars().filter(|c| c.is_uppercase()).count();
    caps >= 2 && s.chars().count() <= 12 && s.chars().all(|c| c.is_alphanumeric() || c == '-') && s.chars().take(2).all(char::is_uppercase)
}

/// The dictionary already has this form with this tag.
fn in_core(lower: &str, tag: Tag) -> bool {
    dict::form(lower).is_some_and(|f| dict::analyses(f).any(|r| r.tag == tag))
}

/// The dictionary has the form with an open-class or proper tag.
fn core_known(lower: &str) -> bool {
    dict::form(lower).is_some()
}

/// Rows: (form lower, tag, lemma) → sources.
type Rows = BTreeMap<(String, usize, String), BTreeSet<String>>;

fn put(rows: &mut Rows, bad: &BTreeSet<String>, form: &str, tag: Tag, lemma: &str, src: String) {
    if !word_ok(form) || !word_ok(lemma) {
        return;
    }
    let low = form.to_lowercase();
    if bad.contains(&low) || in_core(&low, tag) {
        return;
    }
    // a form the dictionary knows gains only a class it lacks (*directional* JJ beside its NN; not RB beside RBR for
    // *faster*), never a proper-noun reading
    let cls = crate::derive::class(tag);
    if let Some(f) = dict::form(&low)
        && (cls == 0 || dict::analyses(f).any(|r| crate::derive::class(r.tag) == cls))
    {
        return;
    }
    rows.entry((low, tag.idx(), lemma.to_string())).or_default().insert(src);
}

/// SPECIALIST `LRAGR`: `EUI|form|cat|agreement|base|citation|` → rows; returns the (lower-case) lemmas seen per class.
fn specialist_forms(path: &Path, bad: &BTreeSet<String>, rows: &mut Rows) -> Result<BTreeSet<(String, char)>> {
    let mut seen = BTreeSet::new();
    let f = BufReader::new(std::fs::File::open(path).with_context(|| format!("reading {}", path.display()))?);
    // (EUI, form, tag, base)
    let mut all: Vec<(String, String, Tag, String)> = Vec::new();
    for line in f.lines() {
        let line = line?;
        let c: Vec<&str> = line.split('|').collect();
        if c.len() < 6 {
            continue;
        }
        let (eui, form, cat, agr, base) = (c[0], c[1], c[2], c[3], c[4]);
        if form.contains(' ') || base.contains(' ') {
            continue;
        }
        let agr0 = agr.split('(').next().unwrap_or("");
        // a noun spelt with a capital anywhere is a name (*Kerala*, *iPhone*, *LHCb*); an adjective, adverb or verb
        // spelt with two capitals is an abbreviation's use (*OA* osteoarthritic), left out
        let proper = base.chars().any(char::is_uppercase) && cat == "noun";
        if cat != "noun" && base.chars().filter(|c| c.is_uppercase()).count() >= 2 {
            continue;
        }
        let tag = match (cat, agr0, agr) {
            ("noun", _, a) if a.contains("thr_plur") => {
                if proper {
                    continue;
                }
                Tag::NNS
            }
            ("noun", _, _) => {
                if proper {
                    Tag::NNP
                } else {
                    Tag::NN
                }
            }
            ("verb", "infinitive", _) => Tag::VB,
            ("verb", "pres", a) if a.contains("thr_sing") && !a.contains("fst") => Tag::VBZ,
            ("verb", "pres", _) => Tag::VBP,
            ("verb", "past", _) => Tag::VBD,
            ("verb", "past_part", _) => Tag::VBN,
            ("verb", "pres_part", _) => Tag::VBG,
            ("adj", "positive", _) | ("adj", "positive;periph", _) => Tag::JJ,
            ("adj", "comparative", _) => Tag::JJR,
            ("adj", "superlative", _) => Tag::JJS,
            ("adv", "positive", _) | ("adv", "positive;periph", _) => Tag::RB,
            ("adv", "comparative", _) => Tag::RBR,
            ("adv", "superlative", _) => Tag::RBS,
            _ => continue,
        };
        let class = match cat {
            "noun" => 'n',
            "verb" => 'v',
            "adj" => 'a',
            _ => 'r',
        };
        seen.insert((base.to_lowercase(), class));
        all.push((eui.to_string(), form.to_string(), tag, base.to_string()));
    }
    // a unit (EUI) the dictionary already has in its class, under any of its spellings and forms, keeps the
    // dictionary's forms: SPECIALIST lists the undoubled *occured* beside *occurred* and the variant *noticable*
    // beside *noticeable*. Such a noun may only gain a plural of a dictionary lemma (the Greco-Latin *vertices*
    // beside *vertexes*).
    let covered: BTreeSet<&str> = all
        .iter()
        .filter(|(_, form, tag, _)| {
            let cls = crate::derive::class(*tag);
            cls != 0 && dict::form(&form.to_lowercase()).is_some_and(|f| dict::analyses(f).any(|r| crate::derive::class(r.tag) == cls))
        })
        .map(|(eui, ..)| eui.as_str())
        .collect();
    for (eui, form, tag, base) in &all {
        if covered.contains(eui.as_str()) && !(*tag == Tag::NNS && in_core(&base.to_lowercase(), Tag::NN)) {
            continue;
        }
        put(rows, bad, form, *tag, base, format!("s:{eui}"));
    }
    Ok(seen)
}

/// SPECIALIST `LEXICON` units: `{base=…` … `acronym_of=<expansion>|<EUI>` … `}` → acronyms.
fn specialist_acronyms(path: &Path, out: &mut BTreeMap<String, Vec<(String, String)>>) -> Result<()> {
    let f = BufReader::new(std::fs::File::open(path).with_context(|| format!("reading {}", path.display()))?);
    let (mut base, mut eui) = (String::new(), String::new());
    for line in f.lines() {
        let line = line?;
        let t = line.trim();
        if let Some(b) = t.strip_prefix("{base=") {
            base = b.to_string();
            eui.clear();
        } else if let Some(e) = t.strip_prefix("entry=") {
            eui = e.to_string();
        } else if let Some(x) = t.strip_prefix("acronym_of=") {
            let exp = x.split('|').next().unwrap_or("").trim();
            if acronym_ok(&base) && !exp.is_empty() && exp.contains(' ') {
                let v = out.entry(base.clone()).or_default();
                if v.len() < MAX_EXPANSIONS && !v.iter().any(|(e, _)| e == exp) {
                    v.push((exp.to_string(), format!("s:{eui}")));
                }
            }
        }
    }
    Ok(())
}

/// Initials of an expansion, skipping short function words: «Universal Dependencies» → UD.
fn initials(exp: &str) -> String {
    exp.split([' ', '-'])
        .filter(|w| !w.is_empty() && !matches!(*w, "of" | "and" | "the" | "for" | "in" | "on" | "to" | "a" | "an" | "&"))
        .filter_map(|w| w.chars().next())
        .flat_map(char::to_uppercase)
        .collect()
}

/// OEWN entries: lemma → part of speech → {form, sense}.
fn oewn(dir: &Path, bad: &BTreeSet<String>, specialist: &BTreeSet<(String, char)>, rows: &mut Rows, acr: &mut BTreeMap<String, Vec<(String, String)>>) -> Result<()> {
    let mut files: Vec<PathBuf> = std::fs::read_dir(dir)?.filter_map(|e| e.ok().map(|e| e.path())).collect();
    files.sort();
    // synset → members, for acronyms (an acronym and a multi-word member with its initials)
    let mut synsets: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for p in &files {
        let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if !name.ends_with(".json") || !name.starts_with("entries-") {
            continue;
        }
        let v: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(p)?).with_context(|| format!("parsing {}", p.display()))?;
        let Some(m) = v.as_object() else { bail!("{}: not an object", p.display()) };
        for (lemma, poses) in m {
            let Some(poses) = poses.as_object() else { continue };
            for (pos, e) in poses {
                for s in e.get("sense").and_then(|s| s.as_array()).into_iter().flatten() {
                    if let Some(id) = s.get("synset").and_then(|x| x.as_str()) {
                        synsets.entry(id.to_string()).or_default().push(lemma.clone());
                    }
                }
                if lemma.contains(' ') || !word_ok(lemma) {
                    continue;
                }
                let forms: Vec<String> = e.get("form").and_then(|f| f.as_array()).into_iter().flatten().filter_map(|x| x.as_str().map(str::to_string)).collect();
                let class = pos.chars().next().unwrap_or(' ');
                let class = if class == 's' { 'a' } else { class };
                let low = lemma.to_lowercase();
                let src = "o".to_string();
                let has_s = specialist.contains(&(low.clone(), class));
                let m = crate::morph::Morph;
                match class {
                    'n' if lemma.chars().any(char::is_uppercase) => put(rows, bad, lemma, Tag::NNP, lemma, src),
                    'n' => {
                        put(rows, bad, lemma, Tag::NN, lemma, src.clone());
                        if has_s {
                            continue;
                        }
                        // a regular plural only where one is plausible: not for -s nouns (*memetics*), -ism, -ness
                        if forms.is_empty() && !low.ends_with('s') && !low.ends_with("ism") {
                            put(rows, bad, &m.inflect(&low, Tag::NNS), Tag::NNS, lemma, src);
                        } else {
                            for f in &forms {
                                put(rows, bad, f, Tag::NNS, lemma, src.clone());
                            }
                        }
                    }
                    'v' => {
                        put(rows, bad, lemma, Tag::VB, lemma, src.clone());
                        put(rows, bad, lemma, Tag::VBP, lemma, src.clone());
                        if has_s || !forms.is_empty() {
                            continue;
                        }
                        for t in [Tag::VBZ, Tag::VBD, Tag::VBN, Tag::VBG] {
                            put(rows, bad, &m.inflect(&low, t), t, lemma, src.clone());
                        }
                    }
                    'a' => {
                        put(rows, bad, lemma, Tag::JJ, lemma, src.clone());
                        if has_s {
                            continue;
                        }
                        for f in &forms {
                            let t = if f.ends_with("est") {
                                Tag::JJS
                            } else if f.ends_with("er") {
                                Tag::JJR
                            } else {
                                continue;
                            };
                            put(rows, bad, f, t, lemma, src.clone());
                        }
                    }
                    'r' => put(rows, bad, lemma, Tag::RB, lemma, src),
                    _ => {}
                }
            }
        }
    }
    for (id, members) in &synsets {
        for a in members.iter().filter(|a| acronym_ok(a)) {
            for exp in members.iter().filter(|e| e.contains(' ') && initials(e) == a.replace('-', "")) {
                let v = acr.entry(a.clone()).or_default();
                if v.len() < MAX_EXPANSIONS && !v.iter().any(|(e, _)| e == exp) {
                    v.push((exp.clone(), format!("o:{id}")));
                }
            }
        }
    }
    Ok(())
}

/// `Names_2010Census.csv`: `name,rank,count,…` (upper case) → NNP rows for surnames no source has as a common word.
fn census(path: &Path, rows: &mut Rows) -> Result<()> {
    let common: BTreeSet<String> = rows.keys().filter(|(_, t, _)| *t != Tag::NNP.idx()).map(|(f, _, _)| f.clone()).collect();
    let text = std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    for line in text.lines().skip(1) {
        let c: Vec<&str> = line.split(',').collect();
        let (Some(name), Some(count)) = (c.first(), c.get(2).and_then(|x| x.parse::<u64>().ok())) else { continue };
        if count < SURNAME_MIN || *name == "ALL OTHER NAMES" {
            continue;
        }
        let low = name.to_lowercase();
        if common.contains(&low) || core_known(&low) || !word_ok(&low) {
            continue;
        }
        let mut ch = low.chars();
        let title: String = ch.next().map(|f| f.to_uppercase().collect::<String>() + ch.as_str()).unwrap_or_default();
        put(rows, &BTreeSet::new(), &low, Tag::NNP, &title, "c".into());
    }
    Ok(())
}

/// Given names of the Kantrowitz Names Corpus (one per line) → NNP rows for names no source has as a common word
/// and the dictionary does not know.
fn first_names(dir: &Path, rows: &mut Rows) -> Result<()> {
    let common: BTreeSet<String> = rows.keys().filter(|(_, t, _)| *t != Tag::NNP.idx()).map(|(f, _, _)| f.clone()).collect();
    for file in ["female.txt", "male.txt"] {
        let p = dir.join(file);
        let text = std::fs::read_to_string(&p).with_context(|| format!("reading {}", p.display()))?;
        for name in text.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')) {
            let low = name.to_lowercase();
            if common.contains(&low) || core_known(&low) || !word_ok(&low) || low.chars().count() < 3 {
                continue;
            }
            put(rows, &BTreeSet::new(), &low, Tag::NNP, name, "k".into());
        }
    }
    Ok(())
}

/// Run all sources and write `lexicon-ext.tsv` and `acronyms.tsv` into `out`.
pub fn generate(src: &Sources, out: &Path) -> Result<Stats> {
    let bad: BTreeSet<String> = match &src.dontuse {
        Some(p) => std::fs::read_to_string(p)?.lines().map(|l| l.trim().to_lowercase()).filter(|l| !l.is_empty()).collect(),
        None => BTreeSet::new(),
    };
    let mut rows = Rows::new();
    let mut acr: BTreeMap<String, Vec<(String, String)>> = BTreeMap::new();
    let mut spec = BTreeSet::new();
    if let Some((_, lragr)) = &src.specialist {
        spec = specialist_forms(lragr, &bad, &mut rows)?;
    }
    // acronyms: WordNet's general expansions first, then SPECIALIST's (mostly biomedical)
    if let Some(d) = &src.oewn {
        oewn(d, &bad, &spec, &mut rows, &mut acr)?;
    }
    if let Some((lexicon, _)) = &src.specialist {
        specialist_acronyms(lexicon, &mut acr)?;
    }
    if let Some(p) = &src.census {
        census(p, &mut rows)?;
    }
    if let Some(d) = &src.first_names {
        first_names(d, &mut rows)?;
    }
    let mut st = Stats::default();
    let mut w = std::io::BufWriter::new(std::fs::File::create(out.join("lexicon-ext.tsv"))?);
    writeln!(w, "# The lexicon extension of en (en/src/lexgrow.rs, `en lexgrow`): form, PTB tag, lemma (`=`: the form itself), sources.")?;
    writeln!(w, "# Sources: o = Open English WordNet 2025 (McCrae et al.; CC BY 4.0, crediting Princeton WordNet and the OEWN team);")?;
    writeln!(w, "# s:<EUI> = the SPECIALIST Lexicon, 2026 release (U.S. National Library of Medicine, SPECIALIST NLP Tools Terms and Conditions),")?;
    writeln!(w, "#   rows selected from LRAGR (single-word nouns, verbs, adjectives, adverbs) and mapped to PTB tags;")?;
    writeln!(w, "# c = 2010 U.S. Census surnames (public domain), as proper nouns; k = given names of the Names Corpus (Mark Kantrowitz,")?;
    writeln!(w, "#   its README below), as proper nouns. Only rows the dictionary (data/forms.tsv) lacks.")?;
    if let Some(d) = &src.first_names {
        writeln!(w, "# The Names Corpus README, as its terms ask for when the list is redistributed:")?;
        for l in std::fs::read_to_string(d.join("README"))?.lines() {
            writeln!(w, "#   {l}")?;
        }
    }
    writeln!(w, "# License: CC BY 4.0 for the OEWN-derived rows; the rest public domain or SPECIALIST terms; selection ours.")?;
    let mut forms = BTreeSet::new();
    let mut lemmas = BTreeSet::new();
    for ((form, tag, lemma), s) in &rows {
        // a common-word reading of three letters or fewer only from WordNet: SPECIALIST's short entries are mostly
        // clipped forms and abbreviations (*sed, ves, ful*); proper nouns and acronyms are gated by case at run time
        let short = form.chars().filter(|c| c.is_alphabetic()).count() <= 3;
        if short && !matches!(Tag::ALL[*tag], Tag::NNP | Tag::NNPS) && !s.iter().any(|x| x.starts_with('o')) {
            continue;
        }
        let s: Vec<&str> = s.iter().map(String::as_str).collect();
        // one SPECIALIST EUI per row is enough provenance: the first in order
        let mut keep: Vec<&str> = Vec::new();
        for x in s {
            let k = x.chars().next().unwrap_or(' ');
            if !keep.iter().any(|y| y.starts_with(k)) {
                keep.push(x);
            }
        }
        for x in &keep {
            *st.by_source.entry(x.chars().next().unwrap_or(' ')).or_default() += 1;
        }
        // the lemma is written `=` when it is the form itself
        let l = if lemma == form { "=" } else { lemma.as_str() };
        writeln!(w, "{form}\t{}\t{l}\t{}", Tag::ALL[*tag].name(), keep.join(","))?;
        st.rows += 1;
        if forms.insert(form.clone()) && !core_known(form) {
            st.new_forms += 1;
        }
        lemmas.insert(lemma.clone());
    }
    w.flush()?;
    st.forms = forms.len();
    st.lemmas = lemmas.len();
    let mut w = std::io::BufWriter::new(std::fs::File::create(out.join("acronyms.tsv"))?);
    writeln!(w, "# Acronyms and their expansions (en/src/lexgrow.rs): acronym, expansion, source.")?;
    writeln!(w, "# Sources: s:<EUI> = SPECIALIST Lexicon 2026 (NLM), `acronym_of`; o:<synset> = Open English WordNet 2025 (CC BY 4.0),")?;
    writeln!(w, "#   an acronym and a multi-word member of the same synset whose initials it is. At most {MAX_EXPANSIONS} expansions each.")?;
    for (a, v) in &acr {
        for (e, s) in v {
            writeln!(w, "{a}\t{e}\t{s}")?;
        }
        st.acronyms += 1;
    }
    w.flush()?;
    Ok(st)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acronyms_and_words() {
        for a in ["GPU", "NLP", "LHCb", "UD", "COVID-19"] {
            assert!(acronym_ok(a), "{a}");
        }
        for a in ["Smith", "gpu", "A", "Ab"] {
            assert!(!acronym_ok(a), "{a}");
        }
        assert!(word_ok("grammatical") && word_ok("self-attention") && word_ok("o'clock"));
        assert!(!word_ok("a") && !word_ok("x2") && !word_ok("-ish") && !word_ok("two words"));
        assert_eq!(initials("Universal Dependencies"), "UD");
        assert_eq!(initials("National Aeronautics and Space Administration"), "NASA");
    }
}
