//! The English texts besides the Bibles, as units with keys like verses: the school lessons by register (the book
//! lines, the conversational test questions, the formal exam), the tales, the Talmud's English. Each kind says how
//! its units split into train, dev and test, and which units share a discourse (frames are read through it).

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::{Context, Result};

use crate::bible::{Split, VKey, split_of};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// verses: split by book, a book is one discourse
    Bible,
    /// (group, document, unit): split by document, a document is one discourse
    Docs,
    /// training text only (the standard's)
    TrainOnly,
}

impl Kind {
    pub fn split(self, k: VKey) -> Split {
        match self {
            Kind::Bible => split_of(k.0),
            Kind::Docs => match k.1 % 10 {
                3 | 8 => Split::Test,
                5 => Split::Dev,
                _ => Split::Train,
            },
            Kind::TrainOnly => Split::Train,
        }
    }
    /// The discourse a unit belongs to.
    pub fn group(self, k: VKey) -> (u8, u16) {
        match self {
            Kind::Bible => (k.0, 0),
            _ => (k.0, k.1),
        }
    }
    /// A held-out document is measured if it is one of every `n` (the school's dev and test are large).
    pub fn sampled(self, k: VKey, n: u16) -> bool {
        match self {
            Kind::Bible | Kind::TrainOnly => true,
            Kind::Docs => (k.1 / 10) % n == 0,
        }
    }
}

/// The registers of the school lessons.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Register {
    /// `explain:` and `fact:` lines: the book voice
    Plain,
    /// the questions of `test:` lines: the conversational voice, half of it slang of a generation and a place
    Talk,
    /// the questions of `exam:` lines: the formal voice
    Exam,
}

/// Courses whose lines are not English prose (grammars of other languages, the Hebrew Torah, the queue).
fn english_course(name: &str) -> bool {
    !(name.starts_with("grammar-") && !name.starts_with("grammar-en")) && !matches!(name, "torah-he" | "queue" | "lessons")
}

/// The lines of one register of the school: key (course, lesson in the course, line in the lesson).
pub fn lessons(dir: &Path, reg: Register) -> Result<BTreeMap<VKey, String>> {
    let mut courses: Vec<_> = std::fs::read_dir(dir).with_context(|| format!("{}", dir.display()))?.flatten().map(|e| e.path()).filter(|p| p.is_dir()).collect();
    courses.sort();
    let mut out = BTreeMap::new();
    for (ci, c) in courses.iter().enumerate() {
        let name = c.file_name().map(|x| x.to_string_lossy().to_string()).unwrap_or_default();
        if !english_course(&name) {
            continue;
        }
        let mut files: Vec<_> = std::fs::read_dir(c)?.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "lesson")).collect();
        files.sort();
        for (li, f) in files.iter().enumerate() {
            let text = std::fs::read_to_string(f)?;
            for (n, line) in text.lines().enumerate() {
                let Some((key, val)) = line.split_once(": ") else { continue };
                let t = match (reg, key) {
                    (Register::Plain, "explain" | "fact") => val.to_string(),
                    (Register::Talk, "test") | (Register::Exam, "exam") => val.split(" | ").next().unwrap_or("").trim_start_matches(['★', ' ']).to_string(),
                    _ => continue,
                };
                if !t.trim().is_empty() {
                    out.insert((ci as u8, li as u16, n as u16), t);
                }
            }
        }
    }
    Ok(out)
}

/// The sentences of a treebank (`# text`, `# sent_id = <src>:<book>:<n>`): key (0, book, sentence in the book).
pub fn treebank_texts(path: &Path) -> Result<BTreeMap<VKey, String>> {
    let text = std::fs::read_to_string(path).with_context(|| format!("{}", path.display()))?;
    let mut books: BTreeMap<String, u16> = BTreeMap::new();
    let mut count: BTreeMap<u16, u16> = BTreeMap::new();
    let mut cur_text = String::new();
    let mut out = BTreeMap::new();
    for line in text.lines() {
        if let Some(t) = line.strip_prefix("# text = ") {
            cur_text = t.to_string();
        } else if let Some(id) = line.strip_prefix("# sent_id = ") {
            let book = id.rsplit_once(':').map(|x| x.0).unwrap_or(id).to_string();
            let nb = books.len() as u16;
            let b = *books.entry(book).or_insert(nb);
            let c = count.entry(b).or_default();
            out.insert((0u8, b, *c), std::mem::take(&mut cur_text));
            *c += 1;
        }
    }
    Ok(out)
}

/// A pool (`id<TAB>text`) as training text: key (0, n / 1000, n % 1000).
pub fn pool_texts(path: &Path) -> Result<BTreeMap<VKey, String>> {
    let text = std::fs::read_to_string(path).with_context(|| format!("{}", path.display()))?;
    Ok(text.lines().filter_map(|l| l.split_once('\t')).enumerate().map(|(n, (_, t))| ((0u8, (n / 1000) as u16, (n % 1000) as u16), t.to_string())).collect())
}

#[cfg(test)]
mod wave32 {
    use super::*;
    #[test]
    fn split_sample_and_course() {
        let k = (3, 15, 2);
        assert_eq!(Kind::Bible.split(k), Split::Test);
        assert_eq!(Kind::Docs.split(k), Split::Dev);
        assert_eq!(Kind::TrainOnly.split(k), Split::Train);
        assert_eq!(Kind::Bible.group(k), (3, 0));
        assert_eq!(Kind::Docs.group(k), (3, 15));
        assert!(!Kind::Docs.sampled(k, 2));
        assert!(Kind::Bible.sampled(k, 0));
        assert!(english_course("grammar-en"));
        assert!(!english_course("grammar-fr"));
        assert!(!english_course("queue"));
    }
}
