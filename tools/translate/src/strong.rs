//! Strong's numbers on the English words: eBible's KJV writes each translated word with the Strong's number of the
//! Hebrew word it renders (`\w beginning|strong="H7225"\w*`); MACULA writes the same numbers on the Hebrew words
//! (`Strong=7225`). Read back verse by verse and laid on the tokens of the standard stack, they give a near-gold
//! word alignment between the Hebrew and that English.
//!
//! The verses are cut as `school-sents usfm` cuts them without `--hebrew-verses`: headings, titles (`\d`), footnotes
//! and cross references are not verse text.

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::Result;

use crate::bible::{VKey, book_of};

/// A Strong's number without its leading zeros and letter: «H0871», «0871a» → 871.
pub fn number(s: &str) -> Option<u32> {
    let d: String = s.trim_start_matches('H').chars().take_while(char::is_ascii_digit).collect();
    d.parse().ok().filter(|&n| n > 0)
}

/// The words of one verse's text with their Strong's numbers.
fn words(text: &str) -> Vec<(String, Option<u32>)> {
    let mut out = Vec::new();
    let mut rest = text;
    let mut plain = String::new();
    let flush = |plain: &mut String, out: &mut Vec<(String, Option<u32>)>| {
        for w in plain.split_whitespace() {
            out.push((w.to_string(), None));
        }
        plain.clear();
    };
    while let Some(i) = rest.find('\\') {
        plain.push_str(&rest[..i]);
        let tail = &rest[i + 1..];
        let name: String = tail.chars().take_while(|c| c.is_ascii_alphanumeric() || *c == '+' || *c == '*').collect();
        let after = &tail[name.len()..];
        if name == "w" || name == "+w" {
            flush(&mut plain, &mut out);
            let close = format!("\\{name}*");
            let end = after.find(&close).unwrap_or(after.len());
            let inner = after[..end].trim_start();
            let (ws, attrs) = inner.split_once('|').unwrap_or((inner, ""));
            let n = attrs.split_once("strong=\"").and_then(|(_, x)| x.split(['"', ' ']).next()).and_then(number);
            for w in ws.split_whitespace() {
                out.push((w.to_string(), n));
            }
            rest = &after[(end + close.len()).min(after.len())..];
        } else {
            rest = after.strip_prefix(' ').filter(|_| !name.ends_with('*')).unwrap_or(after);
        }
    }
    plain.push_str(rest);
    flush(&mut plain, &mut out);
    out
}

/// The verses of a USFM directory with each word's Strong's number (English verse numbers).
pub fn read(dir: &Path) -> Result<BTreeMap<VKey, Vec<(String, Option<u32>)>>> {
    let files = crate::bible::usfm_files(dir)?;
    let mut out = BTreeMap::new();
    for f in files {
        let text = crate::bible::read_usfm(&f)?;
        // footnotes and cross references go whole
        let mut clean = String::new();
        let mut rest = text.as_str();
        while let Some(i) = rest.find('\\') {
            clean.push_str(&rest[..i]);
            let tail = &rest[i..];
            if tail.starts_with("\\f ") || tail.starts_with("\\x ") || tail.starts_with("\\fe ") {
                let m = &tail[..tail.find(' ').unwrap()];
                let end = tail.find(&format!("{m}*")).map(|j| j + m.len() + 1).unwrap_or(tail.len());
                rest = &tail[end..];
            } else {
                clean.push('\\');
                rest = &tail[1..];
            }
        }
        clean.push_str(rest);
        let (mut book, mut c) = (None, 0u16);
        let mut cur: Option<(VKey, String)> = None;
        let push = |cur: &mut Option<(VKey, String)>, out: &mut BTreeMap<VKey, Vec<(String, Option<u32>)>>| {
            if let Some((k, t)) = cur.take() {
                out.insert(k, words(&t));
            }
        };
        for line in clean.lines() {
            let l = line.trim();
            let (mark, body) = l.split_once(' ').unwrap_or((l, ""));
            match mark {
                "\\id" => book = body.split_whitespace().next().and_then(book_of),
                "\\c" => {
                    push(&mut cur, &mut out);
                    c = body.trim().parse().unwrap_or(c);
                }
                "\\v" => {
                    push(&mut cur, &mut out);
                    let (n, txt) = body.trim().split_once(' ').unwrap_or((body.trim(), ""));
                    if let (Some(b), Ok(v)) = (book, n.parse::<u16>()) {
                        cur = Some(((b, c, v), txt.to_string()));
                    }
                }
                m if m.starts_with("\\h") || m.starts_with("\\toc") || m.starts_with("\\mt") || m.starts_with("\\s") || m.starts_with("\\ms") || m.starts_with("\\r") || m.starts_with("\\d") => {}
                _ => {
                    if let Some((_, t)) = cur.as_mut() {
                        t.push(' ');
                        t.push_str(if mark.starts_with('\\') { body } else { l });
                    }
                }
            }
        }
        push(&mut cur, &mut out);
    }
    Ok(out)
}

fn alnum(s: &str) -> Vec<char> {
    s.chars().filter(|c| c.is_alphanumeric()).flat_map(char::to_lowercase).collect()
}

/// The Strong's number of each token, by laying the tokens' letters on the verse's letters in order (punctuation and
/// spacing do not matter). A token whose letters are not found next gets none.
pub fn on_tokens(forms: &[String], ws: &[(String, Option<u32>)]) -> Vec<Option<u32>> {
    let mut chars: Vec<(char, Option<u32>)> = Vec::new();
    for (w, n) in ws {
        chars.extend(alnum(w).into_iter().map(|c| (c, *n)));
    }
    let mut pos = 0;
    forms
        .iter()
        .map(|f| {
            let t = alnum(f);
            if t.is_empty() {
                return None;
            }
            // the token's letters at the pointer, or within the next 40 letters
            for start in pos..(pos + 40).min(chars.len()) {
                if start + t.len() <= chars.len() && (0..t.len()).all(|i| chars[start + i].0 == t[i]) {
                    pos = start + t.len();
                    return chars[start].1;
                }
            }
            None
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strong_words_and_tokens() {
        let w = words("And \\w God|strong=\"H0430\"\\w* \\w saw|strong=\"H7200\"\\w* the \\w light|strong=\"H0216\"\\w*, \\add it was\\add* \\w good|strong=\"H2896\"\\w*:");
        assert_eq!(w[0], ("And".into(), None));
        assert_eq!(w[1], ("God".into(), Some(430)));
        assert_eq!(w[4], ("light".into(), Some(216)));
        assert_eq!(w[5], (",".into(), None));
        assert!(w.iter().any(|x| x.0 == "it" && x.1.is_none()));
        let forms: Vec<String> = ["And", "God", "saw", "the", "light", ",", "it", "was", "good", ":"].iter().map(|s| s.to_string()).collect();
        let n = on_tokens(&forms, &w);
        assert_eq!(n, vec![None, Some(430), Some(7200), None, Some(216), None, None, None, Some(2896), None]);
        assert_eq!(number("0871a"), Some(871));
        assert_eq!(number("H7225"), Some(7225));
    }
}
#[cfg(test)]
mod wave32 {
    use super::*;
    #[test]
    fn window_and_numeric_boundaries() {
        assert_eq!(number("HH0001tail"), Some(1));
        for s in ["H0", "h12", "4294967296", ""] {
            assert_eq!(number(s), None);
        }
        let forms = vec!["!".into(), "z".into(), "q".into(), "z".into()];
        let ws = vec![("x".repeat(39), None), ("z".into(), Some(7))];
        assert_eq!(on_tokens(&forms, &ws), vec![None, Some(7), None, None]);
        let ws = vec![("x".repeat(40), None), ("z".into(), Some(7))];
        assert_eq!(on_tokens(&["z".into()], &ws), vec![None]);
        assert_eq!(
            words("\\w alpha beta|strong=\"H2\""),
            vec![("alpha".into(), Some(2)), ("beta".into(), Some(2))]
        );
    }
}
