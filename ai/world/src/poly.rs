//! Testing grounds for world v3 (public domain, Project Gutenberg): cut a story out of a book, paragraphs,
//! sentences — the `en::tok` tokenizer and sentence boundaries as in (after . ! ? … and closing
//! quotes, unless followed by a lowercase letter or sentence-internal punctuation). Output — TSV
//! `paragraph<TAB>sentence<TAB>text`, as for FairytaleQA tales (`ftqa::read_story`). Headings inside
//! the cut text (titles of nested stories) are separate paragraph-sentences: they open frames.

use std::fmt::Write as _;
use std::path::Path;

use anyhow::{Context, Result, bail};
use en::tok::{Tok, detokenize};

fn terminal(f: &str) -> bool {
    !f.is_empty() && f.chars().all(|c| matches!(c, '.' | '!' | '?' | '…'))
}

fn closer(f: &str) -> bool {
    matches!(f, "”" | "’" | "\"" | "'" | "''" | ")" | "]" | "»")
}

/// Sentence boundaries over tokens (the same algorithm as in::sentences`).
pub fn sentences(toks: &[Tok]) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    let (mut start, mut i) = (0usize, 0usize);
    while i < toks.len() {
        if terminal(&toks[i].form) {
            let mut j = i + 1;
            while j < toks.len() && (closer(&toks[j].form) || terminal(&toks[j].form)) {
                j += 1;
            }
            let next = toks.get(j).and_then(|t| t.form.chars().next());
            let continues = matches!(next, Some(c) if c.is_lowercase() || matches!(c, ',' | ';' | ':' | '-' | '–' | '—'));
            if !continues {
                out.push((start, j));
                start = j;
            }
            i = j;
        } else {
            i += 1;
        }
    }
    if start < toks.len() {
        out.push((start, toks.len()));
    }
    out
}

/// Cut out a story: after the heading line `start` (exact line match) up to the line `end` (exclusive;
/// none — to the end of the book). The Gutenberg header and footer are stripped. Returns the TSV and a summary.
pub fn prep(raw: &Path, start: &str, end: Option<&str>, out: &Path) -> Result<String> {
    let text = std::fs::read_to_string(raw).with_context(|| raw.display().to_string())?;
    let text = text.trim_start_matches('\u{feff}');
    let lines: Vec<&str> = text.lines().map(|l| l.trim_end_matches('\r')).collect();
    let body_from = lines.iter().position(|l| l.starts_with("*** START OF")).context("no Gutenberg START marker")? + 1;
    let body_to = lines.iter().position(|l| l.starts_with("*** END OF")).context("no Gutenberg END marker")?;
    let body = &lines[body_from..body_to];
    let s = body.iter().position(|l| *l == start).with_context(|| format!("no heading line {start:?}"))? + 1;
    let e = match end {
        Some(x) => s + body[s..].iter().position(|l| *l == x).with_context(|| format!("no end line {x:?}"))?,
        None => body.len(),
    };
    // paragraphs between blank lines, whitespace collapsed
    let mut paras: Vec<String> = Vec::new();
    let mut cur: Vec<&str> = Vec::new();
    for l in &body[s..e] {
        if l.trim().is_empty() {
            if !cur.is_empty() {
                paras.push(cur.iter().flat_map(|x| x.split_whitespace()).collect::<Vec<_>>().join(" "));
                cur.clear();
            }
        } else {
            cur.push(l);
        }
    }
    if !cur.is_empty() {
        paras.push(cur.iter().flat_map(|x| x.split_whitespace()).collect::<Vec<_>>().join(" "));
    }
    if paras.is_empty() {
        bail!("no text between {start:?} and {end:?}");
    }
    let ann = crate::tree::annotator()?;
    let mut o = format!("# sec\tsent\ttext  (source {}, from {start:?} to {}; paragraphs — between blank lines)\n", raw.display(), end.map(|x| format!("{x:?}")).unwrap_or("the end".into()));
    let (mut n, mut diff, mut words) = (0usize, 0usize, 0usize);
    for (pi, p) in paras.iter().enumerate() {
        let toks = ann.tokenize(p);
        if detokenize(&toks) != *p {
            diff += 1;
        }
        for (a, b) in sentences(&toks) {
            let sent = detokenize(&toks[a..b]);
            if sent.is_empty() {
                continue;
            }
            n += 1;
            words += toks[a..b].iter().filter(|t| t.form.chars().any(char::is_alphanumeric)).count();
            let _ = writeln!(o, "{}\t{n}\t{}", pi + 1, sent.replace('\t', " "));
        }
    }
    std::fs::write(out, &o)?;
    Ok(format!("{}: paragraphs {}, sentences {n}, words {words}; paragraphs where tokens do not reproduce the text verbatim: {diff}", out.display(), paras.len()))
}
