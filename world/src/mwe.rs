//! Verbal multiword expressions found by the SLM, as a tunable component: contiguous matches of the expression
//! base (`global::expressions_in_sentence`) and pairs over dependency arcs (verb + object or particle in the idiom
//! base `global::idiom`) — the second finds discontinuous ones ("took an important decision"). Measured on PARSEME
//! 1.3 English (verbal idioms, light-verb constructions, verb-particle constructions; annotation CC BY 4.0, UD text
//! CC BY-SA 4.0) with the gold trees of the corpus.
//!
//! A predicted expression is right if it shares two or more tokens with a gold one (the verb among them); per
//! sentence score = right − wrong predictions (for the tuning loop); precision and recall are printed too.

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::Result;

pub struct Sent {
    pub form: Vec<String>,
    pub lemma: Vec<String>,
    pub upos: Vec<String>,
    /// Dense one-based parent positions; zero means root, missing or ambiguous parent.
    pub head: Vec<usize>,
    pub rel: Vec<String>,
    /// gold expressions: token index sets with their category
    pub gold: Vec<(Vec<usize>, String)>,
}

/// Read a `.cupt` file (CoNLL-U Plus with the PARSEME:MWE column).
pub fn read_cupt(path: &Path) -> Result<Vec<Sent>> {
    let mut out = Vec::new();
    let mut cur: Vec<Vec<String>> = Vec::new();
    let flush = |cur: &mut Vec<Vec<String>>, out: &mut Vec<Sent>| {
        if cur.is_empty() {
            return;
        }
        // CUpt IDs may be sparse after filtering range and empty-node rows.
        // Duplicate IDs cannot identify a unique parent and map to no arc.
        let mut positions = BTreeMap::new();
        for (i, c) in cur.iter().enumerate() {
            let id = c[0].parse::<usize>().unwrap();
            positions.entry(id).and_modify(|p| *p = 0).or_insert(i + 1);
        }
        let mut groups: BTreeMap<String, (Vec<usize>, String)> = BTreeMap::new();
        for (i, c) in cur.iter().enumerate() {
            for part in c[10].split(';').filter(|p| *p != "*" && *p != "_") {
                let (id, cat) = part.split_once(':').map_or((part, ""), |(a, b)| (a, b));
                let e = groups.entry(id.to_string()).or_insert((Vec::new(), String::new()));
                e.0.push(i);
                if !cat.is_empty() {
                    e.1 = cat.to_string();
                }
            }
        }
        out.push(Sent {
            form: cur.iter().map(|c| c[1].clone()).collect(),
            lemma: cur.iter().map(|c| c[2].to_lowercase()).collect(),
            upos: cur.iter().map(|c| c[3].clone()).collect(),
            head: cur.iter().map(|c| c[6].parse::<usize>().ok().and_then(|h| positions.get(&h).copied()).unwrap_or(0)).collect(),
            rel: cur.iter().map(|c| c[7].clone()).collect(),
            gold: groups.into_values().collect(),
        });
        cur.clear();
    };
    for l in std::fs::read_to_string(path)?.lines() {
        if l.trim().is_empty() {
            flush(&mut cur, &mut out);
        } else if !l.starts_with('#') {
            let c: Vec<String> = l.split('\t').map(String::from).collect();
            if c.len() >= 11 && c[0].bytes().all(|x| x.is_ascii_digit()) && c[0].parse::<usize>().is_ok_and(|id| id > 0) {
                cur.push(c);
            }
        }
    }
    flush(&mut cur, &mut out);
    Ok(out)
}

pub fn knobs() -> Vec<(&'static str, f64, Vec<f64>)> {
    vec![
        ("contiguous", 1.0, vec![0.0, 1.0]),
        ("kind_idiom", 1.0, vec![0.0, 1.0]),
        ("kind_verb_phrase", 1.0, vec![0.0, 1.0]),
        ("kind_phrasal_verb", 1.0, vec![0.0, 1.0]),
        ("kind_phrase", 0.0, vec![0.0, 1.0]),
        ("min_content", 2.0, vec![1.0, 2.0, 3.0]),
        ("arcs", 1.0, vec![0.0, 1.0]),
        ("arc_idiom", 1.0, vec![0.0, 1.0]),
        ("arc_light_verb", 1.0, vec![0.0, 1.0]),
        ("arc_collocation", 0.0, vec![0.0, 1.0]),
        ("arc_phrasal_verb", 1.0, vec![0.0, 1.0]),
        ("arc_candidate", 0.0, vec![0.0, 1.0]),
    ]
}

fn content(u: &str) -> bool {
    matches!(u, "NOUN" | "VERB" | "ADJ" | "ADV" | "PROPN")
}

/// Predicted expressions of a sentence under a configuration (token index sets).
pub fn predict(s: &Sent, cfg: &BTreeMap<String, f64>) -> Vec<Vec<usize>> {
    let k = |n: &str| cfg.get(n).copied().unwrap_or(0.0) > 0.5;
    let num = |n: &str| cfg.get(n).copied().unwrap_or(0.0);
    let mut out: Vec<Vec<usize>> = Vec::new();
    if k("contiguous") {
        let fr: Vec<String> = s.form.iter().map(|f| f.to_lowercase()).collect();
        let f: Vec<&str> = fr.iter().map(String::as_str).collect();
        let l: Vec<&str> = s.lemma.iter().map(String::as_str).collect();
        for m in global::expressions_in_sentence(&f, &l) {
            if m.end - m.start < 2 {
                continue;
            }
            let kind_ok = match m.entry.kind {
                "idiom" => k("kind_idiom"),
                "verb-phrase" => k("kind_verb_phrase"),
                "phrasal-verb" => k("kind_phrasal_verb"),
                "phrase" => k("kind_phrase"),
                _ => false,
            };
            let span: Vec<usize> = (m.start..m.end).collect();
            let has_verb = span.iter().any(|&i| s.upos[i] == "VERB");
            if kind_ok && has_verb && span.iter().filter(|&&i| content(&s.upos[i])).count() as f64 >= num("min_content").min(span.len() as f64) {
                out.push(span);
            }
        }
    }
    if k("arcs") {
        for i in 0..s.form.len() {
            let h = s.head[i];
            if h == 0 || h > s.upos.len() || h == i + 1 || !matches!(s.rel[i].as_str(), "obj" | "compound:prt" | "iobj") || s.upos[h - 1] != "VERB" {
                continue;
            }
            let ok = match global::idiom(&s.lemma[h - 1], &s.lemma[i]) {
                Some("idiom") => k("arc_idiom"),
                Some("light-verb") => k("arc_light_verb"),
                Some("collocation") => k("arc_collocation"),
                Some("phrasal-verb") => k("arc_phrasal_verb"),
                Some("candidate") => k("arc_candidate"),
                _ => false,
            };
            let mut pair = vec![h - 1, i];
            pair.sort();
            if ok && !out.iter().any(|o| pair.iter().all(|x| o.contains(x))) {
                out.push(pair);
            }
        }
    }
    out
}

/// (right, wrong, gold) for a sentence. Greedy in prediction and gold order;
/// each gold can be used once. Overlap requires two distinct valid indices,
/// including a shared VERB. This is not maximum bipartite matching.
pub fn judge(s: &Sent, pred: &[Vec<usize>]) -> (usize, usize, usize) {
    let mut used = vec![false; s.gold.len()];
    let (mut right, mut wrong) = (0, 0);
    for p in pred {
        let hit = s.gold.iter().enumerate().find(|(gi, (g, _))| {
            let common: std::collections::BTreeSet<usize> = p.iter().copied()
                .filter(|i| *i < s.upos.len() && g.contains(i)).collect();
            !used[*gi] && common.len() >= 2 && common.iter().any(|&i| s.upos[i] == "VERB")
        });
        match hit {
            Some((gi, _)) => {
                used[gi] = true;
                right += 1;
            }
            None => wrong += 1,
        }
    }
    (right, wrong, s.gold.len())
}

pub fn score(data: &[Sent], cfg: &BTreeMap<String, f64>) -> Vec<i64> {
    data.iter().map(|s| {
        let (r, w, _) = judge(s, &predict(s, cfg));
        r as i64 - w as i64
    }).collect()
}

/// Precision and recall under a configuration.
pub fn pr(data: &[Sent], cfg: &BTreeMap<String, f64>) -> (f64, f64) {
    let (mut r, mut w, mut g) = (0usize, 0usize, 0usize);
    for s in data {
        let (a, b, c) = judge(s, &predict(s, cfg));
        r += a;
        w += b;
        g += c;
    }
    (100.0 * r as f64 / (r + w).max(1) as f64, 100.0 * r as f64 / g.max(1) as f64)
}

#[cfg(test)]
mod wave10_models {
    use super::*;
    use std::collections::BTreeSet;

    fn sent(gold: Vec<Vec<usize>>) -> Sent {
        Sent {
            form: vec!["v".into(), "a".into(), "b".into(), "c".into()],
            lemma: vec!["v".into(), "a".into(), "b".into(), "c".into()],
            upos: vec!["VERB".into(), "NOUN".into(), "NOUN".into(), "NOUN".into()],
            head: vec![0, 1, 1, 1],
            rel: vec!["root".into(), "obj".into(), "obj".into(), "obj".into()],
            gold: gold.into_iter().map(|g| (g, "VID".into())).collect(),
        }
    }

    #[test]
    fn judge_shared_verb_and_distinct_overlap() {
        let s = sent(vec![vec![0, 1, 2]]);
        assert_eq!(judge(&s, &[vec![1, 2]]), (0, 1, 1));
        assert_eq!(judge(&s, &[vec![0, 0]]), (0, 1, 1));
        assert_eq!(judge(&s, &[vec![0, 1]]), (1, 0, 1));
    }

    #[test]
    fn judge_finite_set_oracle() {
        for gm in 0..16 {
            for pm in 0..16 {
                let g: Vec<usize> = (0..4).filter(|i| gm & (1 << i) != 0).collect();
                let p: Vec<usize> = (0..4).filter(|i| pm & (1 << i) != 0).collect();
                let common: BTreeSet<_> = g
                    .iter()
                    .copied()
                    .collect::<BTreeSet<_>>()
                    .intersection(&p.iter().copied().collect())
                    .copied()
                    .collect();
                let hit = usize::from(common.len() >= 2 && common.contains(&0));
                let s = sent(vec![g]);
                for duplicate in [false, true] {
                    let mut q = p.clone();
                    if duplicate {
                        q.extend_from_slice(&p);
                    }
                    assert_eq!(judge(&s, &[q]), (hit, 1 - hit, 1), "gm={gm} pm={pm}");
                }
            }
        }
    }

    #[test]
    fn judge_greedy_order_and_denominators() {
        let s = sent(vec![vec![0, 1], vec![0, 2]]);
        assert_eq!(judge(&s, &[vec![0, 1, 2], vec![0, 1]]), (1, 1, 2));
        assert_eq!(judge(&s, &[vec![0, 1], vec![0, 1, 2]]), (2, 0, 2));
        assert_eq!(judge(&s, &[vec![0, 1], vec![0, 1], vec![9]]), (1, 2, 2));
        let data = vec![s, sent(vec![])];
        assert_eq!(score(&data, &BTreeMap::new()), vec![0, 0]);
        assert_eq!(pr(&data, &BTreeMap::new()), (0.0, 0.0));
        assert_eq!(pr(&[], &BTreeMap::new()), (0.0, 0.0));
    }

    #[test]
    fn cupt_head_ids_are_dense_positions() {
        let path =
            std::env::temp_dir().join(format!("mova-mwe-wave10-{}.cupt", std::process::id()));
        // IDs are not vector positions; range/decimal rows are deliberately ignored.
        let text = "# supplied fixture\n10\tTook\tTAKE\tVERB\t_\t_\t0\troot\t_\t_\t1:VID\n10-20\tx\tx\tX\t_\t_\t0\t_\t_\t_\t*\n20\tAction\tACTION\tNOUN\t_\t_\t10\tobj\t_\t_\t1\n20.1\tx\tx\tX\t_\t_\t10\t_\t_\t_\t*\n30\tx\tx\tNOUN\t_\t_\t99\tobj\t_\t_\t*";
        std::fs::write(&path, text).unwrap();
        let got = read_cupt(&path);
        std::fs::remove_file(path).unwrap();
        let got = got.unwrap();
        assert_eq!(got.len(), 1);
        assert_eq!(got[0].head, vec![0, 1, 0]);
        assert_eq!(got[0].lemma, vec!["take", "action", "x"]);
        assert_eq!(got[0].gold, vec![(vec![0, 1], "VID".into())]);
    }

    #[test]
    fn cupt_ambiguous_and_invalid_ids_do_not_attach() {
        let path =
            std::env::temp_dir().join(format!("mova-mwe-ids-wave10-{}.cupt", std::process::id()));
        let mut text = String::new();
        for (id, head) in [
            ("7", "0"),
            ("7", "7"),
            ("9", "7"),
            ("0", "9"),
            ("", "9"),
            ("999999999999999999999999999", "9"),
        ] {
            text.push_str(&format!("{id}\tx\tX\tNOUN\t_\t_\t{head}\tobj\t_\t_\t*\n"));
        }
        std::fs::write(&path, text).unwrap();
        let got = read_cupt(&path);
        std::fs::remove_file(path).unwrap();
        let got = got.unwrap();
        assert_eq!(got[0].head, [0, 0, 0]);
        assert_eq!(got[0].form.len(), 3);
    }

    #[test]
    fn malformed_arc_head_is_ignored() {
        let mut s = sent(vec![]);
        s.head[1] = usize::MAX;
        s.head[2] = 0;
        s.head[3] = 0;
        assert!(predict(&s, &BTreeMap::from([("arcs".into(), 1.0)])).is_empty());
    }
}
