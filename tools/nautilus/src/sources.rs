//! The sources Nautilus reads, as annotated units (`layer::Unit`).
//!
//! - **The command course** (`courses/commands/*.lesson` of the school data, ours): what a lesson teaches — its `explain`,
//!   `fact` and `exercise` lines and an exercise's options — is a unit, id `commands:<lesson>:<field>:<n>` (`n` from
//!   1 per field and lesson; an option is `<field>-option`), its document the lesson. Assessment is never read: the
//!   `test` and `exam` lines and their options are skipped, and a lesson file whose name marks it held out is not
//!   opened. The command corpus (`$MOVA_DATA/corpus/commands/*.jsonl`, the
//!   prompts Crab read, anonymized): every line's `text` is a unit, id `commands-corpus:<file stem>:<line>`, its
//!   document the prompt. Both are annotated by Mova's own model (`en.bin`): its tokenizer, lemmas and UPOS.
//! - **School courses** (`courses/<course>/*.lesson` of the school data, ours): one or more course folders; a lesson's
//!   `explain`, `fact` and `exercise` lines (and an exercise's options) are units, id `<course>:<lesson>:<field>:<n>`,
//!   the document the lesson. Assessment is never read: the `test` and `exam` lines are skipped, a course folder whose
//!   name marks it held out (a word `test`, `exam`, `control` or `heldout`, or `held-out`) is not opened, nor a lesson
//!   file whose name says `held-out` or `heldout` (lessons *about* tests keep their topical names). Annotated by
//!   `en.bin` like the command course.
//! - **The Wikipedia corpus** (`$MOVA_DATA/corpus/wikipedia/articles/<slug>/ast.conllu`, made by the Wikipedia school;
//!   CC BY-SA): the trees are Mova's already, so nothing is annotated again; a sentence is a unit with its `sent_id`,
//!   its document the article, its class from `meta.json`.
use crate::layer::{Unit, Word};
use anyhow::{Context, Result};
use std::path::Path;

/// The fields of a lesson every source reads: what is taught, never what is assessed (`test`, `exam`).
pub const LEARN_FIELDS: [&str; 3] = ["explain", "fact", "exercise"];

/// A lesson file name that marks it held out.
pub fn held_out_file(name: &str) -> bool {
    let n = name.to_lowercase();
    n.contains("held-out") || n.contains("heldout") || n.contains("held_out")
}

/// A course folder name that marks held-out material: never opened by a course source.
pub fn held_out(name: &str) -> bool {
    let n = name.to_lowercase();
    n.split(|c: char| !c.is_ascii_alphanumeric()).any(|w| matches!(w, "test" | "tests" | "exam" | "exams" | "control" | "heldout"))
        || n.contains("held-out")
        || n.contains("held_out")
}

/// The taught units of one lesson file: (id, doc, text); the lesson number from its `lesson:` line. Assessment lines
/// (`test`, `exam`) and their options are never units.
pub fn lesson_units(course: &str, text: &str) -> Vec<(String, String, String)> {
    lesson_units_of(course, text, &LEARN_FIELDS)
}

/// The units of a lesson's `fields` only.
pub fn lesson_units_of(course: &str, text: &str, fields: &[&str]) -> Vec<(String, String, String)> {
    let mut n_lesson = String::new();
    let mut per: std::collections::BTreeMap<&str, usize> = std::collections::BTreeMap::new();
    let mut out = Vec::new();
    for line in text.lines().map(str::trim) {
        let Some((k, v)) = line.split_once(':') else { continue };
        let (k, v) = (k.trim(), v.trim());
        if k == "lesson" {
            n_lesson = v.to_string();
            continue;
        }
        let Some(field) = fields.iter().find(|f| **f == k) else { continue };
        let n = per.entry(field).or_default();
        *n += 1;
        let doc = format!("{course}:{n_lesson}");
        let mut parts = v.split(" | ");
        if let Some(q) = parts.next() {
            out.push((format!("{course}:{n_lesson}:{field}:{n}"), doc.clone(), q.trim().to_string()));
        }
        for (k, o) in parts.enumerate() {
            let o = o.trim().trim_start_matches('*').trim();
            if !o.is_empty() {
                out.push((format!("{course}:{n_lesson}:{field}-option:{n}.{}", k + 1), doc.clone(), o.to_string()));
            }
        }
    }
    out
}

/// The prompts of a command corpus file (JSONL with `text`): (id, doc, text).
pub fn corpus_units(stem: &str, text: &str) -> Vec<(String, String, String)> {
    text.lines()
        .enumerate()
        .filter_map(|(i, l)| {
            let v: serde_json::Value = serde_json::from_str(l).ok()?;
            let t = v["text"].as_str()?.trim().to_string();
            (!t.is_empty()).then(|| {
                let id = format!("commands-corpus:{stem}:{}", i + 1);
                (id.clone(), id, t)
            })
        })
        .collect()
}

/// Annotate (id, doc, text) units with Mova's model.
pub fn annotate(a: &en::annotate::Annotator, raw: Vec<(String, String, String)>) -> Vec<Unit> {
    raw.into_iter()
        .map(|(id, doc, text)| {
            let forms: Vec<String> = a.tokenize(&text).into_iter().map(|t| t.form).collect();
            let words = if forms.is_empty() {
                vec![]
            } else {
                a.annotate(&forms).into_iter().map(|w| Word { form: w.form, lemma: w.lemma, upos: w.upos.name().to_string() }).collect()
            };
            Unit { id, doc, class: None, text, words }
        })
        .collect()
}

/// The sorted `*.ext` files of a directory.
fn files(dir: &Path, ext: &str) -> Result<Vec<std::path::PathBuf>> {
    let mut v: Vec<_> = std::fs::read_dir(dir)
        .with_context(|| format!("{}", dir.display()))?
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == ext))
        .collect();
    v.sort();
    Ok(v)
}

/// The command course and its corpus, raw: (units, lessons, prompts).
pub fn commands_raw(course: &Path, corpus: Option<&Path>) -> Result<(Vec<(String, String, String)>, usize, usize)> {
    let mut raw = Vec::new();
    let lessons: Vec<std::path::PathBuf> = files(course, "lesson")?.into_iter().filter(|f| !f.file_name().and_then(|s| s.to_str()).is_some_and(held_out_file)).collect();
    for f in &lessons {
        raw.extend(lesson_units("commands", &std::fs::read_to_string(f)?));
    }
    let mut prompts = 0;
    if let Some(c) = corpus {
        for f in files(c, "jsonl")? {
            let stem = f.file_stem().and_then(|s| s.to_str()).unwrap_or("corpus").to_string();
            let u = corpus_units(&stem, &std::fs::read_to_string(&f)?);
            prompts += u.len();
            raw.extend(u);
        }
    }
    Ok((raw, lessons.len(), prompts))
}

/// School courses, raw: (units, lessons, the files opened). A course folder or lesson file with a held-out name is
/// not opened; only the taught fields are read. A lesson whose header declares CC BY-SA is reported in `cc_by_sa`.
pub struct Courses {
    pub raw: Vec<(String, String, String)>,
    pub lessons: usize,
    pub opened: Vec<std::path::PathBuf>,
    pub cc_by_sa: Vec<std::path::PathBuf>,
}

pub fn courses_raw(courses: &[std::path::PathBuf]) -> Result<Courses> {
    let mut c = Courses { raw: vec![], lessons: 0, opened: vec![], cc_by_sa: vec![] };
    for dir in courses {
        let name = dir.file_name().and_then(|s| s.to_str()).unwrap_or("course").to_string();
        if held_out(&name) {
            continue;
        }
        for f in files(dir, "lesson")? {
            if f.file_name().and_then(|s| s.to_str()).is_some_and(held_out_file) {
                continue;
            }
            let text = std::fs::read_to_string(&f)?;
            c.opened.push(f.clone());
            if crate::improve::declares_cc_by_sa(&text) {
                c.cc_by_sa.push(f.clone());
            }
            c.raw.extend(lesson_units_of(&name, &text, &LEARN_FIELDS));
            c.lessons += 1;
        }
    }
    Ok(c)
}

/// The sentences of one CoNLL-U text as units of document `doc`.
pub fn conllu_units(doc: &str, class: Option<&str>, text: &str) -> Vec<Unit> {
    let mut out = Vec::new();
    let mut cur = Unit { id: String::new(), doc: doc.into(), class: class.map(String::from), text: String::new(), words: vec![] };
    for l in text.lines().chain(std::iter::once("")) {
        if l.trim().is_empty() {
            if !cur.words.is_empty() {
                let next = Unit { id: String::new(), doc: doc.into(), class: class.map(String::from), text: String::new(), words: vec![] };
                out.push(std::mem::replace(&mut cur, next));
            }
            continue;
        }
        if let Some(v) = l.strip_prefix("# sent_id = ") {
            cur.id = v.trim().to_string();
        } else if let Some(v) = l.strip_prefix("# text = ") {
            cur.text = v.trim().to_string();
        } else if !l.starts_with('#') {
            let c: Vec<&str> = l.split('\t').collect();
            // multiword ranges and empty nodes are not words of their own
            if c.len() >= 4 && !c[0].contains(['-', '.']) {
                cur.words.push(Word { form: c[1].into(), lemma: c[2].into(), upos: c[3].into() });
            }
        }
    }
    out
}

/// The Wikipedia corpus: every article's sentences (articles in name order, at most `limit` of them); returns the
/// units and the number of articles read. Two layouts are read: the Wikipedia school's folders
/// (`articles/<slug>/ast.conllu`, `meta.json`) and the large corpus's tier 1 (`tier1/index.tsv`, the articles in
/// title order, each a zstd frame of a JSON record with `conllu` and `class` in `tier1/shards/<k>.zst`; written by
/// the private `wikicorpus` tool).
pub fn wikipedia(corpus: &Path, limit: Option<usize>) -> Result<(Vec<Unit>, usize)> {
    if corpus.join("tier1/index.tsv").is_file() {
        return wikipedia_tier1(corpus, limit);
    }
    let dir = corpus.join("articles");
    let mut arts: Vec<_> = std::fs::read_dir(&dir).with_context(|| format!("{}", dir.display()))?.flatten().map(|e| e.path()).filter(|p| p.join("ast.conllu").is_file()).collect();
    arts.sort();
    if let Some(k) = limit {
        arts.truncate(k);
    }
    let mut units = Vec::new();
    for a in &arts {
        let meta: serde_json::Value = std::fs::read(a.join("meta.json")).ok().and_then(|b| serde_json::from_slice(&b).ok()).unwrap_or_default();
        let doc = a.file_name().and_then(|s| s.to_str()).unwrap_or("").to_string();
        let text = std::fs::read_to_string(a.join("ast.conllu"))?;
        units.extend(conllu_units(&doc, meta["class"].as_str(), &text));
    }
    Ok((units, arts.len()))
}

/// The large corpus's tier 1: index rows `title, shard, offset, length, …`, one zstd frame per record.
fn wikipedia_tier1(corpus: &Path, limit: Option<usize>) -> Result<(Vec<Unit>, usize)> {
    use std::io::{Read, Seek, SeekFrom};
    let idx = std::fs::read_to_string(corpus.join("tier1/index.tsv"))?;
    let mut units = Vec::new();
    let mut n = 0;
    for l in idx.lines() {
        if limit.is_some_and(|k| n >= k) {
            break;
        }
        let c: Vec<&str> = l.split('\t').collect();
        anyhow::ensure!(c.len() >= 4, "tier1/index.tsv: bad row «{l}»");
        let (off, len): (u64, usize) = (c[2].parse()?, c[3].parse()?);
        let path = corpus.join("tier1/shards").join(format!("{}.zst", c[1]));
        let mut f = std::fs::File::open(&path).with_context(|| format!("{}", path.display()))?;
        f.seek(SeekFrom::Start(off))?;
        let mut b = vec![0u8; len];
        f.read_exact(&mut b)?;
        let r: serde_json::Value = serde_json::from_slice(&zstd::decode_all(&b[..])?)?;
        let doc = c[0].replace(' ', "_").replace('/', "%2F");
        units.extend(conllu_units(&doc, r["class"].as_str(), r["conllu"].as_str().unwrap_or("")));
        n += 1;
    }
    Ok((units, n))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lesson_lines_become_units() {
        let l = "course: commands\nlesson: 7\ntitle: T\nexplain: Do it now.\nfact: A fact.\nexercise: Why? | *Because | Not | No\ntest: Zorblax? | A | *B\nexam: Quuxite now. | *C | D\n";
        let u = lesson_units("commands", l);
        let ids: Vec<&str> = u.iter().map(|x| x.0.as_str()).collect();
        // taught lines only: the test and exam lines and their options are never units
        assert_eq!(ids, ["commands:7:explain:1", "commands:7:fact:1", "commands:7:exercise:1", "commands:7:exercise-option:1.1", "commands:7:exercise-option:1.2", "commands:7:exercise-option:1.3"]);
        assert!(u.iter().all(|x| !x.2.contains("Zorblax") && !x.2.contains("Quuxite") && x.2 != "B" && x.2 != "C"));
        // the star of the right option is not text
        assert_eq!(u[3].2, "Because");
        assert!(u.iter().all(|x| x.1 == "commands:7"));
    }
    #[test]
    fn the_command_course_reads_only_what_is_taught() {
        let d = std::env::temp_dir().join(format!("nautilus-commands-{}-{}", std::process::id(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        std::fs::create_dir_all(&d).unwrap();
        std::fs::write(d.join("001-stop.lesson"), "course: commands\nlesson: 1\nexplain: Stop means halt.\nexercise: Stop now? | *Yes | No\ntest: Is zorblax a command? | *No | Yes\nexam: Say quuxite.\n").unwrap();
        std::fs::write(d.join("002-heldout.lesson"), "course: commands\nlesson: 2\nexplain: The heldoutword is here.\n").unwrap();
        let (raw, lessons, prompts) = commands_raw(&d, None).unwrap();
        let text: String = raw.iter().map(|u| u.2.clone() + "\n").collect();
        assert!(text.contains("Stop means halt.") && text.contains("Stop now?"), "{text}");
        assert!(!text.contains("zorblax") && !text.contains("quuxite") && !text.contains("heldoutword"), "{text}");
        assert!(raw.iter().all(|u| !u.0.contains(":test") && !u.0.contains(":exam")));
        assert_eq!((lessons, prompts), (1, 0));
    }
    #[test]
    fn courses_read_only_what_is_taught() {
        let d = std::env::temp_dir().join(format!("nautilus-courses-{}-{}", std::process::id(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        for sub in ["biology", "biology-test", "history"] {
            std::fs::create_dir_all(d.join(sub)).unwrap();
        }
        std::fs::write(d.join("biology/001-cell.lesson"), "course: biology\nlesson: 1\nexplain: A cell is small.\nexercise: Is a cell small? | *Yes | No\ntest: Is a zorblax a cell? | *No | Yes\nexam: Name the quuxite.\n").unwrap();
        std::fs::write(d.join("biology/002-heldout-review.lesson"), "course: biology\nlesson: 2\nexplain: The heldoutword is here.\n").unwrap();
        std::fs::write(d.join("biology-test/001.lesson"), "course: biology\nlesson: 1\nexplain: The secretword is here.\n").unwrap();
        std::fs::write(d.join("history/001.lesson"), "# License: CC BY-SA 4.0\ncourse: history\nlesson: 1\nfact: Rome grew.\n").unwrap();
        let c = courses_raw(&[d.join("biology"), d.join("biology-test"), d.join("history")]).unwrap();
        let text: String = c.raw.iter().map(|u| u.2.clone() + "\n").collect();
        // taught lines only; the test and exam lines, the held-out folder and file are never read
        assert!(text.contains("A cell is small.") && text.contains("Rome grew."), "{text}");
        assert!(!text.contains("zorblax") && !text.contains("quuxite") && !text.contains("secretword") && !text.contains("heldoutword"), "{text}");
        assert!(c.opened.iter().all(|p| !p.to_string_lossy().contains("test") && !p.to_string_lossy().contains("heldout")), "{:?}", c.opened);
        assert_eq!((c.lessons, c.cc_by_sa.len()), (2, 1));
        assert_eq!(c.raw[0].0, "biology:1:explain:1");
    }
    #[test]
    fn tier1_records_are_read() {
        let d = std::env::temp_dir().join(format!("nautilus-tier1-{}", std::process::id()));
        std::fs::create_dir_all(d.join("tier1/shards")).unwrap();
        let rec = |t: &str, s: &str| zstd::encode_all(serde_json::json!({"title": t, "class": "city", "conllu": format!("# sent_id = wiki:{t}:1\n# text = {s}\n1\t{s}\t{s}\tPROPN\tNNP\t_\t0\troot\t_\t_\n\n")}).to_string().as_bytes(), 3).unwrap();
        let (a, b) = (rec("Aa", "Aa"), rec("Bb", "Bb"));
        std::fs::write(d.join("tier1/shards/00000.zst"), [a.clone(), b.clone()].concat()).unwrap();
        std::fs::write(d.join("tier1/index.tsv"), format!("Aa\t00000\t0\t{}\t1\t1\t1\nBb\t00000\t{}\t{}\t2\t2\t1\n", a.len(), a.len(), b.len())).unwrap();
        let (u, n) = wikipedia(&d, Some(1)).unwrap();
        assert_eq!((n, u.len(), u[0].text.as_str(), u[0].class.as_deref()), (1, 1, "Aa", Some("city")));
        let (u, n) = wikipedia(&d, None).unwrap();
        assert_eq!((n, u[1].id.as_str()), (2, "wiki:Bb:1"));
        // negative control: a broken index row fails
        std::fs::write(d.join("tier1/index.tsv"), "Aa\t00000\n").unwrap();
        assert!(wikipedia(&d, None).is_err());
        let _ = std::fs::remove_dir_all(&d);
    }
    #[test]
    fn corpus_and_conllu() {
        let c = corpus_units("seed", "{\"text\":\"Stop Eel.\\n\"}\nnot json\n{\"text\":\"\"}\n{\"text\":\"Is Eel running?\"}\n");
        assert_eq!(c, vec![("commands-corpus:seed:1".into(), "commands-corpus:seed:1".into(), "Stop Eel.".into()), ("commands-corpus:seed:4".into(), "commands-corpus:seed:4".into(), "Is Eel running?".into())]);
        let t = "# sent_id = wiki:X:1\n# text = X is big.\n1\tX\tX\tPROPN\tNNP\t_\t3\tnsubj\t_\t_\n2-3\tisn't\t_\t_\t_\t_\t_\t_\t_\t_\n2\tis\tbe\tAUX\tVBZ\t_\t3\tcop\t_\t_\n3\tbig\tbig\tADJ\tJJ\t_\t0\troot\t_\t_\n\n# sent_id = wiki:X:2\n# text = Y.\n1\tY\tY\tPROPN\tNNP\t_\t0\troot\t_\t_\n";
        let u = conllu_units("X", Some("city"), t);
        assert_eq!(u.len(), 2);
        assert_eq!((u[0].id.as_str(), u[0].text.as_str(), u[0].words.len()), ("wiki:X:1", "X is big.", 3));
        assert_eq!(u[1].class.as_deref(), Some("city"));
    }
}
