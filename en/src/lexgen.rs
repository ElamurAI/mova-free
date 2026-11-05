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

/// Every source `en lexicon` reads, with the source letters of `data/forms.tsv` it yields and the name its citation
/// carries in `data/LICENSE.md`: the AGID file, then the UD training treebanks of the 2026-09-24 run. A new source
/// is added here and cited there before it is read.
pub const SOURCES: &[(&str, &str, &str)] = &[
    ("agid", "abr", "AGID"),
    ("ewt", "u", "UD English EWT"),
    ("eslspok", "u", "UD English ESLSpok"),
];

/// The sources the licence text does not cite, and the source letters of `forms` no source yields.
pub fn uncited(licence: &str, forms: &str) -> Vec<String> {
    let mut miss: Vec<String> = SOURCES.iter().filter(|s| !licence.contains(s.2)).map(|s| s.0.to_string()).collect();
    let mut letters = std::collections::BTreeSet::new();
    for l in forms.lines() {
        letters.extend(l.split('\t').nth(4).unwrap_or("").chars());
    }
    miss.extend(letters.into_iter().filter(|c| !SOURCES.iter().any(|s| s.1.contains(*c))).map(|c| format!("letter {c}")));
    miss
}

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
            if t.lemma == "_" || personal_token(&t.form) || personal_token(&t.lemma) {
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

/// A corpus token that is an address, not a word: an e-mail address (also the obfuscated `name...@host`
/// and the Lotus Notes `Name@DOMAIN` of the EWT e-mail genre), a link (`http://`, `https://`, `ftp://`,
/// `mailto:`, `www.`), an `@handle` or a North American telephone number (`713-853-1234`, `(713) 853-1234`,
/// `202.456.1111`, the seven-digit `853-1234`). Such tokens are personal data or page addresses, never
/// lexicon. Words that merely contain `@` stay: `@`, `s@p`, `e@tg`, a censored `assh@%$e`.
pub fn personal_token(s: &str) -> bool {
    let l = s.to_ascii_lowercase();
    if !l.is_empty() && l.chars().all(|c| c.is_ascii_digit() || "-.() +".contains(c)) {
        let g: Vec<usize> = l.split(|c: char| !c.is_ascii_digit()).filter(|x| !x.is_empty()).map(str::len).collect();
        let n: usize = g.iter().sum();
        return g.last() == Some(&4) && (g == [3, 4] || (g.len() >= 3 && (10..=11).contains(&n)));
    }
    if ["http://", "https://", "ftp://", "mailto:", "www."].iter().any(|p| l.starts_with(p)) {
        return true;
    }
    if let Some(h) = l.strip_prefix('@') {
        return h.len() >= 2 && h.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '.')) && h.chars().any(|c| c.is_ascii_alphabetic());
    }
    let Some((local, domain)) = l.rsplit_once('@') else { return false };
    (local.chars().filter(char::is_ascii_alphanumeric).count() >= 2 || domain.contains('.'))
        && domain.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-'))
        && domain.split(['.', '_', '-']).any(|p| p.len() >= 2 && p.chars().any(|c| c.is_ascii_alphabetic()))
}

/// What `redact` changed.
pub struct Redacted {
    pub lemmas: usize,
    pub rows: usize,
}

/// Removes personal tokens (`personal_token`) from an existing dictionary in `dir` without renumbering:
/// a personal lemma line becomes `<redacted-N>` with flag `-` (N is its own number, so heap keys stay unique
/// and every other lemma number keeps its meaning), and a form row whose form is personal or whose lemma
/// is redacted is dropped. Idempotent. The dictionary fingerprint changes, so models must be retrained.
pub fn redact(dir: &Path) -> Result<Redacted> {
    let lp = dir.join("lemmas.tsv");
    let fp = dir.join("forms.tsv");
    let lemmas = std::fs::read_to_string(&lp).with_context(|| format!("{}", lp.display()))?;
    let forms = std::fs::read_to_string(&fp).with_context(|| format!("{}", fp.display()))?;
    let mut gone: FastSet<String> = FastSet::default();
    let mut heap = String::with_capacity(lemmas.len());
    for (i, line) in lemmas.lines().enumerate() {
        let lemma = line.split('\t').next().unwrap_or("");
        if personal_token(lemma) {
            gone.insert(i.to_string());
            heap.push_str(&format!("<redacted-{i}>\t-\n"));
        } else {
            heap.push_str(line);
            heap.push('\n');
        }
    }
    let mut kept = String::with_capacity(forms.len());
    let mut rows = 0;
    for line in forms.lines() {
        let mut c = line.split('\t');
        let (form, lemma) = (c.next().unwrap_or(""), c.nth(1).unwrap_or(""));
        if personal_token(form) || gone.contains(lemma) {
            rows += 1;
            continue;
        }
        kept.push_str(line);
        kept.push('\n');
    }
    std::fs::write(&lp, heap)?;
    std::fs::write(&fp, kept)?;
    Ok(Redacted { lemmas: gone.len(), rows })
}

#[cfg(test)]
mod personal_tests {
    use super::*;

    #[test]
    fn personal_tokens_are_addresses_not_words() {
        for s in [
            "jane.q.public@example.com",
            "JohnDoe@EXAMPLECOMMUNICATIONS",
            "John_Doe%EXAMPLE@example.net",
            "jdoe...@example.com",
            "j...@example.net",
            "jane.doe@ex",
            "mailto:jane.roe@example.org",
            "http://www.example.org/home/default.asp",
            "https://example.org",
            "www.example.com",
            "@jdoe",
            "@j.d.o.e",
            "555-555-0100",
            "555.555.0101",
            "(555) 555-0123",
            "1-800-555-0199",
            "555-0142",
        ] {
            assert!(personal_token(s), "{s}");
        }
        for s in ["@", "s@p", "e@tg", "assh@%$e", "the", "example", "a.m.", "@2", "AT&T", "1990-1995", "3.14", "12", "2-3", "10:30"] {
            assert!(!personal_token(s), "{s}");
        }
    }

    #[test]
    fn redact_keeps_numbers_and_is_idempotent() {
        let dir = std::env::temp_dir().join(format!("mova-lexredact-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("lemmas.tsv"), "the\tk\nJo.Doe@corp.com\t-\njo.doe@corp.com\tk\nrun\tk\n").unwrap();
        std::fs::write(
            dir.join("forms.tsv"),
            "the\tDT\t0\t9\tu\njo.doe@corp.com\tADD\t1\t1\tu\nruns\tVBZ\t3\t0\tar\nodd\tNN\t2\t1\tu\n",
        )
        .unwrap();
        let r = redact(&dir).unwrap();
        assert_eq!((r.lemmas, r.rows), (2, 2));
        let lemmas = std::fs::read_to_string(dir.join("lemmas.tsv")).unwrap();
        assert_eq!(lemmas, "the\tk\n<redacted-1>\t-\n<redacted-2>\t-\nrun\tk\n");
        assert_eq!(std::fs::read_to_string(dir.join("forms.tsv")).unwrap(), "the\tDT\t0\t9\tu\nruns\tVBZ\t3\t0\tar\n");
        let again = redact(&dir).unwrap();
        assert_eq!((again.lemmas, again.rows), (0, 0));
        assert_eq!(std::fs::read_to_string(dir.join("lemmas.tsv")).unwrap(), lemmas);
        std::fs::remove_dir_all(dir).unwrap();
    }

    /// The gate on the shipped dictionary: red while any e-mail address, link or handle is in it.
    #[test]
    fn shipped_dictionary_holds_no_personal_tokens() {
        let d = Path::new(env!("CARGO_MANIFEST_DIR")).join("data");
        let mut bad = Vec::new();
        for (f, col) in [("lemmas.tsv", 0), ("forms.tsv", 0)] {
            let Ok(t) = std::fs::read_to_string(d.join(f)) else { continue };
            for (i, l) in t.lines().enumerate() {
                if personal_token(l.split('\t').nth(col).unwrap_or("")) {
                    bad.push(format!("{f}:{}", i + 1));
                }
            }
        }
        assert!(bad.is_empty(), "{} personal tokens, first {:?}", bad.len(), &bad[..bad.len().min(5)]);
    }

    /// Every source the generator reads is cited in `data/LICENSE.md`, and every source letter of the shipped
    /// `forms.tsv` belongs to one of them.
    #[test]
    fn licence_cites_every_source() {
        let d = Path::new(env!("CARGO_MANIFEST_DIR")).join("data");
        let forms = std::fs::read_to_string(d.join("forms.tsv")).unwrap_or_default();
        assert_eq!(uncited(crate::dict::LICENCE, &forms), Vec::<String>::new());
        // negative controls: a licence without AGID, and a row with a source letter no source yields
        assert_eq!(uncited(&crate::dict::LICENCE.replace("AGID", "A-G-I-D"), ""), ["agid"]);
        assert_eq!(uncited(crate::dict::LICENCE, "x\tNN\t0\t0\tq\n"), ["letter q"]);
    }
}

#[cfg(test)]
mod wave27_tests {
    use super::*;
    #[test]
    fn synthetic_files_preserve_ids_and_source_flags() {
        let root = std::env::temp_dir().join(format!("mova-wave27-lexgen-{}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        let agid = root.join("tiny.agid");
        let out = root.join("out");
        std::fs::create_dir_all(&out).unwrap();
        std::fs::write(out.join("lemmas.tsv"), "frozen\t-\n").unwrap();
        std::fs::write(&agid, "zorb V: zorbed | zorbing | zorbs\nwug N?: wugs\n").unwrap();
        let s = generate(&agid, &[], &out).unwrap();
        assert_eq!((s.lemmas, s.new_lemmas, s.forms, s.rows), (3, 2, 6, 8));
        assert_eq!(
            std::fs::read_to_string(out.join("lemmas.tsv")).unwrap(),
            "frozen\t-\nwug\t-\nzorb\tk\n"
        );
        let forms = std::fs::read_to_string(out.join("forms.tsv")).unwrap();
        assert!(forms.contains("zorbed\tVBN\t2\t0\tar\n"));
        assert!(forms.contains("wugs\tNNS\t1\t0\ta\n"));
        assert_eq!(generate(&agid, &[], &out).unwrap().new_lemmas, 0);
        let before = std::fs::read(out.join("forms.tsv")).unwrap();
        assert!(generate(&root.join("absent"), &[], &out).is_err());
        assert_eq!(std::fs::read(out.join("forms.tsv")).unwrap(), before);
        std::fs::remove_dir_all(root).unwrap();
        assert_eq!(agid_form("bad?, clean"), Some(("clean".into(), true)));
    }
}
