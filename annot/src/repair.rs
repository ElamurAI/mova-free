//! Deterministic repairs of Early Modern English annotations (L6 of `languages/en-x-emode/13-improvements.md`), on
//! CoNLL-U word lines: the drafts before the gates (`drafts_from`) and a whole parse database (`annot db-repair`).
//! No teacher: every rule is a table or a morphology lookup and says what it changed.
//!
//! - `table` — a word the EModE table (`en::emode`) knows with its tags: lemma and FEATS from the table (T1, T2, T4,
//!   F1, F2: «thee» → thou, «thine» → thy, «thou» never Person=3, `Style=Arch`), and an AUX that is not *be* is no
//!   copula;
//! - `ending` — an unknown «-eth»/«-est» verb with the tag of its ending: lemma and FEATS of the ending (T3, F3);
//! - `verb-lemma` — another VERB/AUX whose lemma is still its archaic form: the lemma `en::morph::archaic` gives
//!   (L1–L3, L5: «hath» → have, «saith» → say, «-eth» verbs);
//! - `lower-lemma` — a capitalised lemma of a function word or verb («Behold», «Whosoever», «Thy»: S7);
//! - `not` — a postverbal «not» on its lexical verb (S1).

use std::collections::BTreeMap;

use en::gram::{Feats, Rel, Tag, UPos};

/// Repairs one sentence's word lines (ten TAB-separated columns; ranges and empty nodes are left as they are).
/// Counts the changed words per rule into `counts`.
pub fn sentence(lines: &mut [String], counts: &mut BTreeMap<&'static str, usize>) {
    let idx: Vec<usize> = (0..lines.len())
        .filter(|&k| {
            lines[k]
                .split('\t')
                .next()
                .is_some_and(|c| c.chars().all(|x| x.is_ascii_digit()))
        })
        .collect();
    let mut cols: Vec<Vec<String>> = idx
        .iter()
        .map(|&k| lines[k].split('\t').map(String::from).collect())
        .collect();
    if cols.iter().any(|c| c.len() != 10) {
        return;
    }
    let forms: Vec<String> = cols.iter().map(|c| c[1].clone()).collect();
    let forms: Vec<&str> = forms.iter().map(String::as_str).collect();
    for i in 0..cols.len() {
        let c = &mut cols[i];
        let (Some(tag), Some(upos), Some(rel)) =
            (Tag::parse(&c[4]), UPos::parse(&c[3]), Rel::parse(&c[7]))
        else {
            continue;
        };
        let before = c.clone();
        let rule = if let Some((l, _, f, r)) =
            en::emode::fix(&forms, i, tag, upos, rel, Feats::parse(&c[5]).0).filter(|x| x.1 == upos)
        {
            c[2] = l;
            c[5] = f.to_string();
            if r != rel {
                c[7] = r.to_string();
            }
            if en::emode::tagged(&forms, i, tag).is_some() {
                "table"
            } else {
                "ending"
            }
        } else if matches!(upos, UPos::VERB | UPos::AUX)
            && c[2].to_lowercase() == c[1].to_lowercase()
        {
            // the tag of a verb; another tag on a VERB/AUX («art» AUX NN) is read as a present verb
            let t = if c[4].starts_with("VB") || tag == Tag::MD {
                tag
            } else {
                Tag::VBP
            };
            match en::morph::archaic(&c[1].to_lowercase(), t) {
                Some(l) => {
                    c[2] = l;
                    "verb-lemma"
                }
                None => continue,
            }
        } else if !matches!(
            upos,
            UPos::PROPN | UPos::NOUN | UPos::ADJ | UPos::NUM | UPos::X | UPos::SYM
        ) && c[2] != "I"
            && c[2].chars().next().is_some_and(char::is_uppercase)
            && c[2].chars().skip(1).all(|x| !x.is_uppercase())
        {
            c[2] = c[2].to_lowercase();
            "lower-lemma"
        } else {
            continue;
        };
        if *c != before {
            *counts.entry(rule).or_default() += 1;
        }
    }
    // S1 over the repaired words
    let upos: Vec<UPos> = cols
        .iter()
        .map(|c| UPos::parse(&c[3]).unwrap_or(UPos::X))
        .collect();
    let tags: Vec<Tag> = cols
        .iter()
        .map(|c| Tag::parse(&c[4]).unwrap_or(Tag::FW))
        .collect();
    let mut heads: Vec<usize> = cols.iter().map(|c| c[6].parse().unwrap_or(0)).collect();
    let mut rels: Vec<Rel> = cols
        .iter()
        .map(|c| Rel::parse(&c[7]).unwrap_or(Rel::Dep))
        .collect();
    let n = en::emode::postverbal_not(&forms, &upos, &tags, &mut heads, &mut rels);
    if n > 0 {
        *counts.entry("not").or_default() += n;
        for (i, c) in cols.iter_mut().enumerate() {
            if c[6] != heads[i].to_string() {
                c[6] = heads[i].to_string();
                c[7] = rels[i].to_string();
            }
        }
    }
    for (j, &k) in idx.iter().enumerate() {
        lines[k] = cols[j].join("\t");
    }
}

/// A whole CoNLL-U text: comments and blank lines as they are, every sentence repaired.
pub fn text(src: &str, counts: &mut BTreeMap<&'static str, usize>) -> String {
    let mut out = String::with_capacity(src.len());
    let mut block: Vec<String> = Vec::new();
    let flush =
        |block: &mut Vec<String>, out: &mut String, counts: &mut BTreeMap<&'static str, usize>| {
            let mut words: Vec<String> = block
                .iter()
                .filter(|l| !l.starts_with('#'))
                .cloned()
                .collect();
            sentence(&mut words, counts);
            let mut w = words.into_iter();
            for l in block.drain(..) {
                out.push_str(&if l.starts_with('#') {
                    l
                } else {
                    w.next().unwrap_or(l)
                });
                out.push('\n');
            }
        };
    for l in src.lines() {
        if l.trim().is_empty() {
            flush(&mut block, &mut out, counts);
            out.push('\n');
        } else {
            block.push(l.to_string());
        }
    }
    flush(&mut block, &mut out, counts);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(rows: &str) -> (String, BTreeMap<&'static str, usize>) {
        let mut c = BTreeMap::new();
        (text(rows, &mut c), c)
    }

    #[test]
    fn repairs_with_negative_controls() {
        if en::dict::N_LEMMAS == 0 {
            return;
        }
        let src = "# text = I know not the day ; he giveth thee bread .\n\
            1\tI\tI\tPRON\tPRP\tCase=Nom|Number=Sing|Person=1|PronType=Prs\t2\tnsubj\t_\t_\n\
            2\tknow\tknow\tVERB\tVBP\tMood=Ind|Number=Sing|Person=1|Tense=Pres|VerbForm=Fin\t0\troot\t_\t_\n\
            3\tnot\tnot\tPART\tRB\tPolarity=Neg\t5\tadvmod\t_\t_\n\
            4\tthe\tthe\tDET\tDT\tDefinite=Def|PronType=Art\t5\tdet\t_\t_\n\
            5\tday\tday\tNOUN\tNN\tNumber=Sing\t2\tobj\t_\t_\n\
            6\t;\t;\tPUNCT\t:\t_\t8\tpunct\t_\t_\n\
            7\the\the\tPRON\tPRP\tCase=Nom|Gender=Masc|Number=Sing|Person=3|PronType=Prs\t8\tnsubj\t_\t_\n\
            8\tgiveth\tgiveth\tVERB\tVBZ\tMood=Ind|Number=Sing|Person=3|Tense=Pres|VerbForm=Fin\t2\tparataxis\t_\t_\n\
            9\tthee\tthee\tPRON\tPRP\tCase=Acc|Gender=Neut|Number=Sing|Person=3|PronType=Prs\t8\tiobj\t_\t_\n\
            10\tbread\tbread\tNOUN\tNN\tNumber=Sing\t8\tobj\t_\t_\n\
            11\t.\t.\tPUNCT\t.\t_\t2\tpunct\t_\t_\n\n";
        let (out, c) = run(src);
        let col = |n: usize, k: usize| {
            out.lines()
                .filter(|l| !l.starts_with('#') && !l.is_empty())
                .nth(n - 1)
                .unwrap()
                .split('\t')
                .nth(k)
                .unwrap()
                .to_string()
        };
        assert_eq!(col(9, 2), "thou");
        assert_eq!(
            col(9, 5),
            "Case=Acc|Number=Sing|Person=2|PronType=Prs|Style=Arch"
        );
        assert_eq!(col(8, 2), "give");
        assert_eq!(
            (col(3, 6), col(3, 7)),
            ("2".to_string(), "advmod".to_string())
        );
        assert_eq!(c.get("table"), Some(&1));
        assert_eq!(c.get("ending"), Some(&1));
        assert_eq!(c.get("not"), Some(&1));
        // negative controls: modern words, «I», the comment line, a second run changes nothing
        assert_eq!(col(1, 2), "I");
        assert_eq!(col(5, 2), "day");
        assert!(out.starts_with("# text = I know not"));
        let (again, c2) = run(&out);
        assert_eq!(again, out);
        assert!(c2.is_empty(), "{c2:?}");
        // a capitalised interjection lemma is lowered; a proper noun's is not
        let (o, c) = run(
            "1\tAmen\tAmen\tINTJ\tUH\t_\t0\troot\t_\t_\n2\tJacob\tJacob\tPROPN\tNNP\tNumber=Sing\t1\tvocative\t_\t_\n\n",
        );
        assert!(o.contains("\tAmen\tamen\tINTJ") && o.contains("\tJacob\tJacob\tPROPN"));
        assert_eq!(c.get("lower-lemma"), Some(&1));
        assert_eq!(c.get("table"), None);
        // «art» made AUX with a noun tag gets the lemma «be» (the old AUX repair of drafts_from)
        let (o, c) = run(
            "1\tthou\tthou\tPRON\tPRP\tCase=Nom|Number=Sing|Person=2|PronType=Prs|Style=Arch\t3\tnsubj\t_\t_\n2\tart\tart\tAUX\tNN\t_\t3\tcop\t_\t_\n3\tking\tking\tNOUN\tNN\tNumber=Sing\t0\troot\t_\t_\n\n",
        );
        assert!(o.contains("\tart\tbe\tAUX"), "{o}");
        assert_eq!(c.get("verb-lemma"), Some(&1));
    }
}

#[cfg(test)]
mod wave27_tests {
    use super::*;
    #[test]
    fn malformed_word_aborts_before_repair_and_ranges_survive() {
        let row = "1\tAmen\tAmen\tINTJ\tUH\t_\t0\troot\t_\t_";
        let mut lines = vec![row.to_string(), "2\tmalformed".into()];
        let original = lines.clone();
        let mut counts = BTreeMap::new();
        sentence(&mut lines, &mut counts);
        assert_eq!(lines, original);
        assert!(counts.is_empty());
        let range = "1-2\tignored".to_string();
        lines = vec![row.into(), range.clone()];
        sentence(&mut lines, &mut counts);
        assert!(lines[0].contains("\tAmen\tamen\tINTJ"));
        assert_eq!(lines[1], range);
        assert_eq!(counts.get("lower-lemma"), Some(&1));
        sentence(&mut lines, &mut counts);
        assert_eq!(counts.get("lower-lemma"), Some(&1));
    }
}
