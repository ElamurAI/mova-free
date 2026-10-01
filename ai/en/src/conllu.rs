//! CoNLL-U (Universal Dependencies): sentences with text and tokens. Multiword tokens (`1-2`) and
//! empty nodes (`8.1`) are skipped: the graph is built on syntactic words. Tag and relation become
//! enums right away; an unknown relation is an error with the line number (fail-fast).
//!
//! Full lossless representation — `Doc` (below): for dialect converters (`en::convert`).

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::{Context, Result, bail};

use crate::gram::{Feats, Rel, Tag, UPos};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Token {
    pub form: String,
    pub lemma: String,
    /// PTB tag; `None` — treebank without PTB tags ("_" or another system).
    pub tag: Option<Tag>,
    /// Universal part of speech; `None` — "_".
    pub upos: Option<UPos>,
    /// Morphological features (pairs outside the `Feat` list are dropped).
    pub feats: Feats,
    /// Head index (0 — root).
    pub head: usize,
    pub rel: Rel,
    /// Whether a space follows the token in the text (`SpaceAfter=No` — no).
    pub space_after: bool,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Sentence {
    pub id: String,
    pub text: String,
    pub tokens: Vec<Token>,
    /// Layers the treebank actually annotates (determined over the whole file): FEATS, lemmas, word forms.
    /// ESLSpok has neither FEATS nor lemmas; CHILDES lacks FEATS; GUMReddit lacks the words themselves (Reddit text removed).
    pub has_feats: bool,
    pub has_lemmas: bool,
    pub has_forms: bool,
}

impl Sentence {
    /// All tokens have a PTB tag — the sentence is usable for training the tagger and parser.
    pub fn tagged(&self) -> bool {
        self.tokens.iter().all(|t| t.tag.is_some())
    }
}

pub fn read(path: &Path) -> Result<Vec<Sentence>> {
    let mut out = Vec::new();
    let mut cur = Sentence::default();
    let reader = BufReader::new(File::open(path).with_context(|| format!("{}", path.display()))?);
    // the space after a multiword token is set by the range line (`3-4 … SpaceAfter=No`)
    let mut range_end: usize = 0;
    let mut range_space = true;
    for (no, line) in reader.lines().enumerate() {
        let line = line?;
        if line.is_empty() {
            if !cur.tokens.is_empty() {
                out.push(std::mem::take(&mut cur));
            }
            continue;
        }
        if let Some(rest) = line.strip_prefix("# sent_id = ") {
            cur.id = rest.to_string();
            continue;
        }
        if let Some(rest) = line.strip_prefix("# text = ") {
            cur.text = rest.to_string();
            continue;
        }
        if line.starts_with('#') {
            continue;
        }
        let cols: Vec<&str> = line.split('\t').collect();
        if cols.len() < 10 {
            continue;
        }
        if let Some((_, b)) = cols[0].split_once('-') {
            range_end = b.parse().unwrap_or(0);
            range_space = !cols[9].contains("SpaceAfter=No");
            continue;
        }
        if cols[0].contains('.') {
            continue;
        }
        let id: usize = cols[0].parse().unwrap_or(0);
        let mut space_after = !cols[9].contains("SpaceAfter=No");
        if range_end > 0 && id < range_end {
            space_after = false; // no spaces inside a multiword token
        } else if range_end > 0 && id == range_end {
            space_after = range_space;
            range_end = 0;
        }
        let rel = match Rel::parse(cols[7]).or_else(|| cols[7].split_once(':').and_then(|(b, _)| Rel::parse(b))) {
            Some(r) => r,
            None => bail!("{}:{}: unknown UD relation '{}'", path.display(), no + 1, cols[7]),
        };
        cur.tokens.push(Token {
            form: cols[1].to_string(),
            lemma: cols[2].to_string(),
            tag: Tag::parse(cols[4]),
            upos: UPos::parse(cols[3]),
            feats: Feats::parse(cols[5]).0,
            head: cols[6].parse().unwrap_or(0),
            rel,
            space_after,
        });
    }
    if !cur.tokens.is_empty() {
        out.push(cur);
    }
    // file layers: present if at least 5% of tokens have a value other than "_"
    let (mut n, mut f, mut l, mut w) = (0usize, 0usize, 0usize, 0usize);
    for t in out.iter().flat_map(|s| &s.tokens) {
        n += 1;
        f += !t.feats.is_empty() as usize;
        l += (t.lemma != "_") as usize;
        w += (t.form != "_") as usize;
    }
    for s in &mut out {
        s.has_feats = f * 20 >= n;
        s.has_lemmas = l * 20 >= n;
        s.has_forms = w * 20 >= n;
    }
    Ok(out)
}

// ── Lossless CoNLL-U ───────────────────────────────────────────────────────────────────────────
// `read` above gives a view for training: without multiword tokens, empty nodes, DEPS, MISC and unknown
// features. Dialect converters (`en::convert`) must carry over everything a rule does not touch, so here is
// the full representation: comments and all lines with raw columns. Writing without edits gives the same bytes.

/// CoNLL-U column.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
#[repr(u8)]
pub enum Col {
    Id,
    Form,
    Lemma,
    Upos,
    Xpos,
    Feats,
    Head,
    Deprel,
    Deps,
    Misc,
}

impl Col {
    pub const ALL: [Col; 10] = [Col::Id, Col::Form, Col::Lemma, Col::Upos, Col::Xpos, Col::Feats, Col::Head, Col::Deprel, Col::Deps, Col::Misc];
    pub fn name(self) -> &'static str {
        ["ID", "FORM", "LEMMA", "UPOS", "XPOS", "FEATS", "HEAD", "DEPREL", "DEPS", "MISC"][self as usize]
    }
}

/// Sentence line — ten raw columns as in the file: a word, a multiword token (`3-4`) or an empty node (`8.1`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Row(pub [String; 10]);

impl Row {
    pub fn get(&self, c: Col) -> &str {
        &self.0[c as usize]
    }
    pub fn set(&mut self, c: Col, v: impl Into<String>) {
        self.0[c as usize] = v.into();
    }
    /// Syntactic word: ID is an integer (not `3-4` and not `8.1`).
    pub fn is_word(&self) -> bool {
        !self.0[0].contains(['-', '.'])
    }
}

/// Lossless sentence. The truth is `comments` and `rows`; `sent` is a view for rules (`en::expert`, `en::convert`):
/// syntactic words only, as `read` gives. After editing `rows` — `refresh()`.
#[derive(Clone, Debug, Default)]
pub struct FullSentence {
    /// Comments as in the file, with "#".
    pub comments: Vec<String>,
    pub rows: Vec<Row>,
    /// Word number k+1 is `rows[words[k]]`.
    pub words: Vec<usize>,
    pub sent: Sentence,
    /// Number of blank lines after the sentence (1 in UD files).
    pub blank_after: usize,
    /// Line number of the sentence's first line in the file (from 1) — for messages.
    pub line: usize,
}

impl FullSentence {
    /// Line of word number k+1.
    pub fn word(&self, k: usize) -> &Row {
        &self.rows[self.words[k]]
    }
    pub fn word_mut(&mut self, k: usize) -> &mut Row {
        let i = self.words[k];
        &mut self.rows[i]
    }
    /// sent_id or line number — for messages.
    pub fn label(&self) -> String {
        if self.sent.id.is_empty() { format!("line {}", self.line) } else { self.sent.id.clone() }
    }

    /// Rebuild `sent` and `words` from `rows` — the same way `read` builds a sentence (has_* layers stay).
    /// Word IDs must be consecutive from 1: the head number is an index into `sent.tokens`.
    pub fn refresh(&mut self) -> Result<()> {
        let mut s = Sentence { has_feats: self.sent.has_feats, has_lemmas: self.sent.has_lemmas, has_forms: self.sent.has_forms, ..Default::default() };
        for c in &self.comments {
            if let Some(rest) = c.strip_prefix("# sent_id = ") {
                s.id = rest.to_string();
            } else if let Some(rest) = c.strip_prefix("# text = ") {
                s.text = rest.to_string();
            }
        }
        self.words.clear();
        let (mut range_end, mut range_space) = (0usize, true);
        for (k, row) in self.rows.iter().enumerate() {
            let c = &row.0;
            if let Some((_, b)) = c[0].split_once('-') {
                range_end = b.parse().unwrap_or(0);
                range_space = !c[9].contains("SpaceAfter=No");
                continue;
            }
            if c[0].contains('.') {
                continue;
            }
            let id: usize = c[0].parse().unwrap_or(0);
            if id != s.tokens.len() + 1 {
                bail!("{}: word with ID '{}' out of place (expected {})", s.id, c[0], s.tokens.len() + 1);
            }
            let mut space_after = !c[9].contains("SpaceAfter=No");
            if range_end > 0 && id < range_end {
                space_after = false;
            } else if range_end > 0 && id == range_end {
                space_after = range_space;
                range_end = 0;
            }
            let rel = match Rel::parse(&c[7]).or_else(|| c[7].split_once(':').and_then(|(b, _)| Rel::parse(b))) {
                Some(r) => r,
                None => bail!("{}: unknown UD relation '{}'", s.id, c[7]),
            };
            s.tokens.push(Token {
                form: c[1].clone(),
                lemma: c[2].clone(),
                tag: Tag::parse(&c[4]),
                upos: UPos::parse(&c[3]),
                feats: Feats::parse(&c[5]).0,
                head: c[6].parse().unwrap_or(0),
                rel,
                space_after,
            });
            self.words.push(k);
        }
        self.sent = s;
        Ok(())
    }
}

/// Lossless CoNLL-U file: `Doc::parse(t)?.text() == t` byte for byte — blank lines, end of file,
/// comments, multiword tokens, empty nodes, DEPS, MISC, features outside `Feat`.
#[derive(Clone, Debug, Default)]
pub struct Doc {
    /// Number of blank lines before the first sentence.
    pub blank_before: usize,
    pub sents: Vec<FullSentence>,
    /// Whether the file ends with a newline.
    pub final_newline: bool,
}

impl Doc {
    pub fn read(path: &Path) -> Result<Doc> {
        let text = std::fs::read_to_string(path).with_context(|| format!("{}", path.display()))?;
        Doc::parse(&text).with_context(|| format!("{}", path.display()))
    }

    /// A sentence is consecutive non-blank lines: comments first, then lines with ten TAB-separated columns.
    /// Anything else is an error with the line number (fail-fast): CR, comment in the middle of a sentence, not 10 columns,
    /// non-consecutive word IDs, unknown relation.
    pub fn parse(text: &str) -> Result<Doc> {
        let mut doc = Doc::default();
        let body = match text.strip_suffix('\n') {
            Some(b) => {
                doc.final_newline = true;
                b
            }
            None if text.is_empty() => return Ok(doc),
            None => text,
        };
        let mut cur: Option<FullSentence> = None;
        for (no, line) in body.split('\n').enumerate() {
            if line.is_empty() {
                if let Some(s) = cur.take() {
                    doc.sents.push(s);
                }
                match doc.sents.last_mut() {
                    Some(s) => s.blank_after += 1,
                    None => doc.blank_before += 1,
                }
                continue;
            }
            if line.contains('\r') {
                bail!("line {}: CR — CoNLL-U lines end with LF only", no + 1);
            }
            let s = cur.get_or_insert_with(|| FullSentence { line: no + 1, ..Default::default() });
            if line.starts_with('#') {
                if !s.rows.is_empty() {
                    bail!("line {}: comment in the middle of a sentence", no + 1);
                }
                s.comments.push(line.to_string());
                continue;
            }
            let cols: Vec<String> = line.split('\t').map(str::to_string).collect();
            let n = cols.len();
            let cols: [String; 10] = cols.try_into().map_err(|_| anyhow::anyhow!("line {}: {n} columns, expected 10", no + 1))?;
            s.rows.push(Row(cols));
        }
        if let Some(s) = cur.take() {
            doc.sents.push(s);
        }
        for s in &mut doc.sents {
            let line = s.line;
            s.refresh().with_context(|| format!("sentence at line {line}"))?;
        }
        // file layers — as in `read`: present if at least 5% of words have a value other than "_"
        let (mut n, mut f, mut l, mut w) = (0usize, 0usize, 0usize, 0usize);
        for t in doc.sents.iter().flat_map(|s| &s.sent.tokens) {
            n += 1;
            f += !t.feats.is_empty() as usize;
            l += (t.lemma != "_") as usize;
            w += (t.form != "_") as usize;
        }
        for s in &mut doc.sents {
            s.sent.has_feats = f * 20 >= n;
            s.sent.has_lemmas = l * 20 >= n;
            s.sent.has_forms = w * 20 >= n;
        }
        Ok(doc)
    }

    /// File text: lines joined by LF, as they were read.
    pub fn text(&self) -> String {
        let mut out = String::new();
        let mut first = true;
        let mut nl = |out: &mut String| {
            if !first {
                out.push('\n');
            }
            first = false;
        };
        for _ in 0..self.blank_before {
            nl(&mut out);
        }
        for s in &self.sents {
            for c in &s.comments {
                nl(&mut out);
                out.push_str(c);
            }
            for r in &s.rows {
                nl(&mut out);
                for (i, c) in r.0.iter().enumerate() {
                    if i > 0 {
                        out.push('\t');
                    }
                    out.push_str(c);
                }
            }
            for _ in 0..s.blank_after {
                nl(&mut out);
            }
        }
        if self.final_newline {
            out.push('\n');
        }
        out
    }
}

#[cfg(test)]
mod full_tests {
    use super::*;

    /// A sentence with everything `read` drops: newdoc, multiword token, empty node, DEPS, MISC,
    /// features outside `Feat` (Aspect, Zzz).
    const S1: &str = "# newdoc id = d1\n# sent_id = a\n# text = Don't go.\n1-2\tDon't\t_\t_\t_\t_\t_\t_\t_\tSpaceAfter=No\n1\tDo\tdo\tAUX\tVBP\tAspect=Perf|Mood=Ind|Zzz=Q\t3\taux\t3:aux\t_\n2\tn't\tnot\tPART\tRB\t_\t3\tadvmod\t3:advmod\t_\n3\tgo\tgo\tVERB\tVB\tVerbForm=Inf\t0\troot\t0:root\tSpaceAfter=No\n3.1\tgo\tgo\tVERB\tVB\t_\t_\t_\t3:conj\tCopyOf=3\n4\t.\t.\tPUNCT\t.\t_\t3\tpunct\t3:punct\t_\n";
    const S2: &str = "# sent_id = b\n1\tHi\thi\tINTJ\tUH\t_\t0\troot\t0:root\t_\n";

    #[test]
    fn full_round_trip_synthetic() {
        let cases = [
            format!("{S1}\n{S2}\n"),
            format!("\n\n{S1}\n{S2}\n"),
            format!("{S1}\n\n\n{S2}\n\n"),
            format!("{S1}\n{S2}"),
            format!("{S1}\n{}", S2.trim_end_matches('\n')),
            String::new(),
            "\n".into(),
            "\n\n".into(),
        ];
        for t in &cases {
            let d = Doc::parse(t).unwrap();
            assert_eq!(&d.text(), t, "{t:?}");
        }
        let d = Doc::parse(&cases[0]).unwrap();
        assert_eq!(d.sents.len(), 2);
        assert_eq!((d.sents[0].rows.len(), d.sents[0].words.len(), d.sents[0].comments.len()), (6, 4, 3));
        // view — as in `read`: words only; the space after the token Don't comes from the multiword line
        let s = &d.sents[0].sent;
        assert_eq!((s.id.as_str(), s.text.as_str(), s.tokens.len()), ("a", "Don't go.", 4));
        assert_eq!(s.tokens.iter().map(|t| t.space_after).collect::<Vec<_>>(), [false, false, false, true]);
        assert!(s.tokens[0].feats.has(crate::gram::Feat::MoodInd));
        assert_eq!(d.sents[0].word(2).get(Col::Form), "go");
        // negative control: editing a raw column changes the text
        let mut e = d.clone();
        e.sents[0].rows[1].set(Col::Feats, "Mood=Ind");
        assert_ne!(e.text(), cases[0]);
        assert_eq!(e.text().len() + "Aspect=Perf||Zzz=Q".len(), cases[0].len());
    }

    #[test]
    fn full_bad_input_fails() {
        let err = |t: &str| format!("{:#}", Doc::parse(t).unwrap_err());
        assert!(err("# sent_id = a\r\n1\tHi\thi\tINTJ\tUH\t_\t0\troot\t_\t_\n").contains("CR"));
        assert!(err("1\tHi\thi\tINTJ\tUH\t_\t0\troot\t_\t_\n# x\n").contains("in the middle of"));
        assert!(err("1\tHi\thi\tINTJ\tUH\t_\t0\troot\t_\n").contains("9 columns"));
        assert!(err("2\tHi\thi\tINTJ\tUH\t_\t0\troot\t_\t_\n").contains("out of place"));
        assert!(err("1\tHi\thi\tINTJ\tUH\t_\t0\tzzz\t_\t_\n").contains("unknown UD relation"));
    }

}
