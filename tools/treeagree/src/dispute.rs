//! Disputes for a teacher model (design note, 15.10: "deltas, a compressed vocabulary, fewer thinking tokens — find a couple of
//! fixes and stop"). Instead of the whole tree, the teacher sees each sentence once as numbered words and, for every
//! word whose head our parsers dispute (the agreed draft, Stanza, Trankit), a choice: «17 called: A → 3 word (acl) |
//! B → 9 is (ccomp)». It answers «<n> <word> <letter>» only where it would change the draft; silence keeps the draft.
//! Punctuation is not asked (its heads are convention, not meaning).
//!
//!   treeagree dispute-ask <draft.conllu> <parsers dir> <prompts.jsonl> [sentences per prompt]
//!   treeagree dispute-apply <draft.conllu> <prompts.jsonl> <answers.jsonl> <out.conllu>

use std::collections::BTreeMap;
use std::fmt::Write as _;

use en::gram::Rel;
use en::udreg::Registry;

use crate::tree::{self, Sent, Wit};

/// One disputed word: its options, the first is the draft's.
#[derive(Clone, Debug, PartialEq)]
pub struct Dispute {
    pub word: usize,
    pub opts: Vec<(usize, Rel)>,
}

/// A parser relation as a UD relation (unknown → its universal part → None).
fn ud(r: &str) -> Option<Rel> {
    Rel::parse(r).or_else(|| Rel::parse(r.split(':').next().unwrap_or(r)))
}

/// The disputed words of a sentence: a different head from Stanza or Trankit than the draft's, punctuation left out.
pub fn disputes(s: &Sent, wits: &[&[Wit]]) -> Vec<Dispute> {
    let mut out = Vec::new();
    for (k, t) in s.toks.iter().enumerate() {
        if t.rel == Rel::Punct {
            continue;
        }
        let mut opts = vec![(t.head, t.rel)];
        for w in wits {
            let Some(x) = w.get(k) else { continue };
            if x.head != t.head && !opts.iter().any(|o| o.0 == x.head) && x.rel != "punct" {
                if let Some(r) = ud(&x.rel) {
                    opts.push((x.head, if x.head == 0 { Rel::Root } else { r }));
                }
            }
        }
        if opts.len() > 1 {
            out.push(Dispute { word: k + 1, opts });
        }
    }
    out
}

const LETTERS: [char; 4] = ['A', 'B', 'C', 'D'];

/// The fixed instructions (cached by the server as a prompt prefix).
pub const FIXED: &str = "You check disputed attachments in English dependency trees (Universal Dependencies). Each sentence is given as numbered words. Below it, each disputed word has options: which word it depends on (0 = it is the root) and with which relation. Option A is our current choice. For each disputed word answer only if A is wrong: output one line <sentence number> <word number> <letter>. If A is right, output nothing for it. If every A is right, output the single line OK. No explanations.\n\n";

/// The questions of a group of sentences, and the map back: (sentence number in the prompt → sent id).
pub fn prompt(group: &[(&Sent, Vec<Dispute>)]) -> String {
    let mut v = String::new();
    for (n, (s, ds)) in group.iter().enumerate() {
        let _ = writeln!(v, "S{} {}", n + 1, s.toks.iter().enumerate().map(|(i, t)| format!("{}:{}", i + 1, t.form())).collect::<Vec<_>>().join(" "));
        for d in ds {
            let form = s.toks[d.word - 1].form();
            let opts: Vec<String> = d
                .opts
                .iter()
                .enumerate()
                .map(|(i, (h, r))| {
                    let to = if *h == 0 { "ROOT".to_string() } else { format!("{h}:{}", s.toks[h - 1].form()) };
                    format!("{} {to} {}", LETTERS[i], r.name())
                })
                .collect();
            let _ = writeln!(v, "  {} {form}: {}", d.word, opts.join(" | "));
        }
        v.push('\n');
    }
    v
}

/// Answers «<sentence number> <word> <letter>» → (sentence number, word, option index).
pub fn parse_answers(text: &str) -> Vec<(usize, usize, usize)> {
    text.lines()
        .filter_map(|l| {
            let c: Vec<&str> = l.split_whitespace().collect();
            if c.len() != 3 {
                return None;
            }
            let n: usize = c[0].trim_start_matches('S').parse().ok()?;
            let w: usize = c[1].parse().ok()?;
            let k = LETTERS.iter().position(|x| c[2].trim_matches(|ch: char| !ch.is_alphabetic()).starts_with(*x))?;
            Some((n, w, k))
        })
        .collect()
}

/// Apply the chosen options to a sentence; the whole sentence is kept only if it passes the gates no worse than the
/// draft (else the draft stays). Returns how many words changed.
pub fn apply(s: &mut Sent, ds: &[Dispute], picks: &[(usize, usize)], reg: &Registry) -> usize {
    let mut heads: Vec<usize> = s.toks.iter().map(|t| t.head).collect();
    let mut rels: Vec<Rel> = s.toks.iter().map(|t| t.rel).collect();
    let base = tree::violations(&s.toks, &heads, &rels, reg);
    let mut n = 0;
    for &(w, k) in picks {
        if let Some(d) = ds.iter().find(|d| d.word == w) {
            if let Some(&(h, r)) = d.opts.get(k).filter(|_| k > 0) {
                heads[w - 1] = h;
                rels[w - 1] = r;
                n += 1;
            }
        }
    }
    if n == 0 || tree::no_worse(&s.toks, &heads, &rels, &base, reg).is_err() {
        return 0;
    }
    for (t, (h, r)) in s.toks.iter_mut().zip(heads.into_iter().zip(rels)) {
        t.head = h;
        t.rel = r;
    }
    n
}

/// Groups of sentences with disputes: (prompt text, the sent ids in order).
pub fn groups(sents: &[Sent], wits: &[Vec<Vec<Wit>>], per: usize) -> Vec<(String, Vec<String>)> {
    let mut with: Vec<(&Sent, Vec<Dispute>)> = Vec::new();
    for (i, s) in sents.iter().enumerate() {
        let ws: Vec<&[Wit]> = wits.iter().map(|w| w[i].as_slice()).collect();
        let ds = disputes(s, &ws);
        if !ds.is_empty() {
            with.push((s, ds));
        }
    }
    with.chunks(per).map(|g| (prompt(g), g.iter().map(|(s, _)| s.id.clone()).collect())).collect()
}

/// Remember the disputes by id for applying later.
pub fn by_id(sents: &[Sent], wits: &[Vec<Vec<Wit>>]) -> BTreeMap<String, Vec<Dispute>> {
    sents
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let ws: Vec<&[Wit]> = wits.iter().map(|w| w[i].as_slice()).collect();
            (s.id.clone(), disputes(s, &ws))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_parse_and_silence_keeps_the_draft() {
        assert_eq!(parse_answers("S1 17 B\n2 4 C.\nOK\nnonsense line here x\n"), vec![(1, 17, 1), (2, 4, 2)]);
        assert!(parse_answers("OK").is_empty());
    }
}
