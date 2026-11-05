//! Testing grounds for world v3 (public domain, Project Gutenberg): cut a story out of a book, paragraphs,
//! sentences — the `en::tok` tokenizer and sentence boundaries as in `tools/db/src/tales.rs` (after . ! ? … and closing
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

/// Sentence boundaries over tokens (the same algorithm as in `tools/db/src/tales.rs::sentences`).
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
    let paras = story_paragraphs(&text, start, end)?;
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

/// Pure extraction before acquiring the parser or writing the output.
fn story_paragraphs(text: &str, start: &str, end: Option<&str>) -> Result<Vec<String>> {
    let text = text.trim_start_matches('\u{feff}');
    let lines: Vec<&str> = text.lines().map(|l| l.trim_end_matches('\r')).collect();
    let body_from = lines.iter().position(|l| l.starts_with("*** START OF")).context("no Gutenberg START marker")? + 1;
    let body_to = lines.iter().position(|l| l.starts_with("*** END OF")).context("no Gutenberg END marker")?;
    if body_from > body_to {
        bail!("Gutenberg END marker precedes START body");
    }
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
    Ok(paras)
}

#[cfg(test)]
mod bounded_models {
    use super::*;
    fn toks(forms: &[&str]) -> Vec<Tok> {
        forms.iter().map(|s| Tok { form: (*s).into(), space_after: true }).collect()
    }

    #[test]
    fn sentence_table_and_partition_oracle() {
        let cases: &[(&[&str], &[(usize, usize)])] = &[
            (&[], &[]),
            (&["Heading"], &[(0, 1)]),
            (&["A", ".", "B", "!"], &[(0, 2), (2, 4)]),
            (&["A", ".", "”", "b"], &[(0, 4)]),
            (&["A", "?!", ")", "B"], &[(0, 3), (3, 4)]),
            (&["A", ".", ",", "B"], &[(0, 4)]),
            (&["A", "Mr.", "B"], &[(0, 3)]),
            (&["A", "…", "—", "B"], &[(0, 4)]),
        ];
        for (forms, expected) in cases {
            assert_eq!(sentences(&toks(forms)), *expected, "{forms:?}");
        }
        // Exhaust the small token language; the oracle checks partition laws,
        // independently of the boundary scanner.
        let alphabet = ["A", "a", ".", "”", ",", ""];
        for n in 0..=4 {
            for mut code in 0..alphabet.len().pow(n as u32) {
                let mut forms = Vec::new();
                for _ in 0..n {
                    forms.push(alphabet[code % alphabet.len()]);
                    code /= alphabet.len();
                }
                let ranges = sentences(&toks(&forms));
                let flattened: Vec<usize> = ranges.iter().flat_map(|(a, b)| *a..*b).collect();
                assert_eq!(flattened, (0..n).collect::<Vec<_>>());
                assert!(ranges.iter().all(|(a, b)| a < b && *b <= n));
            }
        }
    }

    #[test]
    fn continuation_and_closer_oracle() {
        for next in ["a", "é", ",", ";", ":", "-", "–", "—"] {
            assert_eq!(sentences(&toks(&["A", ".", "”", next])), vec![(0, 4)]);
        }
        for next in ["A", "7", "É", "("] {
            assert_eq!(sentences(&toks(&["A", ".", "”", next])), vec![(0, 3), (3, 4)]);
        }
        for close in ["”", "’", "\"", "'", "''", ")", "]", "»", "..."] {
            assert_eq!(sentences(&toks(&["A", "!", close, "B"])), vec![(0, 3), (3, 4)]);
        }
        assert!(!terminal(""));
        assert!(!terminal(".x"));
        assert!(!closer("("));
    }

    #[test]
    fn extraction_headings_whitespace_and_bounds() {
        let raw = "\u{feff}front\r\n*** START OF BOOK\r\nTitle\r\n One\t two \r\nthree\r\n\r\nNested heading\r\n\r\nFour!\r\nStop\r\nignored\r\n*** END OF BOOK\r\nback";
        assert_eq!(story_paragraphs(raw, "Title", Some("Stop")).unwrap(),
            vec!["One two three", "Nested heading", "Four!"]);
        assert_eq!(story_paragraphs(raw, "Stop", None).unwrap(), vec!["ignored"]);
        assert!(story_paragraphs(raw, " Title", None).is_err());
        assert!(story_paragraphs(raw, "Stop", Some("Title")).is_err());
        let repeated = "*** START OF\nH\nfirst\nH\nlast\n*** END OF";
        assert_eq!(story_paragraphs(repeated, "H", Some("H")).unwrap(), vec!["first"]);
    }

    #[test]
    fn invalid_extraction_is_error_before_parser_or_output() {
        let bad = [
            ("*** END OF\n*** START OF\nTitle\ntext", "precedes START"),
            ("Title\ntext\n*** END OF", "no Gutenberg START"),
            ("*** START OF\nTitle\ntext", "no Gutenberg END"),
            ("*** START OF\nOther\ntext\n*** END OF", "no heading"),
            ("*** START OF\nTitle\n\n*** END OF", "no text"),
            ("*** START OF\n*** END OF", "no heading"),
        ];
        for (raw, message) in bad {
            let result = std::panic::catch_unwind(|| story_paragraphs(raw, "Title", None));
            assert!(result.is_ok(), "must return an error, not panic: {raw:?}");
            assert!(result.unwrap().unwrap_err().to_string().contains(message));
        }
        // Exercise the public I/O boundary only in a fresh test directory.
        let dir = std::env::temp_dir().join(format!("mova-poly-bounded-{}", std::process::id()));
        std::fs::create_dir(&dir).unwrap();
        let input = dir.join("input.txt");
        let output = dir.join("output.tsv");
        std::fs::write(&input, bad[0].0).unwrap();
        std::fs::write(&output, "sentinel").unwrap();
        assert!(prep(&input, "Title", None, &output).unwrap_err().to_string().contains("precedes START"));
        assert_eq!(std::fs::read_to_string(&output).unwrap(), "sentinel");
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn compiled_negative_control() {
        // Deliberately false boundary model: punctuation always splits.
        let wrong = vec![(0, 2), (2, 3)];
        assert_ne!(sentences(&toks(&["A", ".", "lower"])), wrong);
    }
}
