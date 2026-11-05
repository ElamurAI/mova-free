//! Several segmentations, the parser chooses (Goldberg & Tsarfaty 2011, More et al. 2019, Tsarfaty et al. 2019).
//!
//! The segmenter commits to one split per written word, and end to end a wrong split costs the tree. Here a
//! sentence goes to the parser in several readings: the segmenter's best (the base), and for each written word with
//! another candidate close enough, the base with that one word cut the other way (`variants`). The parser scores
//! every reading (`gparse annotate` with GP_SCORE: the log-probability of its tree), and each word takes the
//! candidate whose reading beats the base by the most (`choose`):
//!
//!   gain = (tree score of the reading − tree score of the base) + λ · (log share of the candidate − log share of
//!          the best) + μ · (words added)
//!
//! μ pays for the extra word's own term in the tree score (a longer reading sums more negative terms); λ weighs the
//! segmenter's own preference. A word whose best gain is not above 0 keeps the base split. The words choose
//! independently (each reading changes one word), and the chosen split is written with `Lattice=` in MISC: which
//! candidate and its gain. The result is parsed once more as usual.

use std::collections::BTreeMap;

use crate::conllu::{Sent, Token};
use crate::seg::{Model, Piece, label, parse_label};

/// The options: candidates per word (`k`), the smallest share of a candidate against the best (`ratio`).
#[derive(Clone, Debug)]
pub struct Opts {
    pub k: usize,
    pub ratio: f64,
}

/// The candidates of a written word that go into readings: the first `k`, each at least `ratio` of the best's share.
fn alternatives(m: &Model, keys: &[String], i: usize, o: &Opts) -> Vec<(Vec<Piece>, f64)> {
    let c = m.candidates_in(keys, i);
    let best = c.first().map(|x| x.p).unwrap_or(1.0).max(1e-12);
    c.into_iter().take(o.k).enumerate().filter(|(i, x)| *i == 0 || x.p / best >= o.ratio).map(|(_, x)| (x.ps, (x.p.max(1e-12) / best).ln())).collect()
}

fn punct(t: &Token) -> bool {
    t.words.iter().all(|w| w.upos() == "PUNCT")
}

/// The readings of every sentence: the base (`# lattice = <sentence> base`), then one per other candidate of each
/// word (`# lattice = <sentence> <token> <label> <log share ratio>`), its words cut by `cut`.
pub fn variants(m: &Model, sents: &[Sent], o: &Opts, cut: &dyn Fn(&Model, &str, &[Piece]) -> Token) -> Vec<Sent> {
    let mut out = Vec::new();
    for (si, s) in sents.iter().enumerate() {
        let keys = m.keys(&s.tokens.iter().map(|t| t.form.as_str()).collect::<Vec<_>>());
        let alts: Vec<Vec<(Vec<Piece>, f64)>> = s.tokens.iter().enumerate().map(|(i, t)| if punct(t) { Vec::new() } else { alternatives(m, &keys, i, o) }).collect();
        let reading = |pick: Option<(usize, &[Piece])>| -> Vec<Token> {
            s.tokens.iter().enumerate().map(|(ti, t)| {
                if punct(t) {
                    return t.clone();
                }
                match pick {
                    Some((pi, ps)) if pi == ti => cut(m, &t.form, ps),
                    _ => cut(m, &t.form, &alts[ti][0].0),
                }
            }).collect()
        };
        let head = |extra: String| {
            let mut c: Vec<String> = s.comments.iter().filter(|c| c.starts_with("# sent_id") || c.starts_with("# text")).cloned().collect();
            c.push(format!("# lattice = {si} {extra}"));
            c
        };
        out.push(Sent { comments: head("base".into()), tokens: reading(None) });
        for (ti, a) in alts.iter().enumerate() {
            for (ps, lr) in a.iter().skip(1) {
                out.push(Sent { comments: head(format!("{ti} {} {lr:.4}", label(ps))), tokens: reading(Some((ti, ps))) });
            }
        }
    }
    out
}

/// One reading's place: the sentence, and the word and candidate it changes (None for the base).
#[derive(Debug, PartialEq)]
struct Place {
    sent: usize,
    change: Option<(usize, String, f64)>,
}

fn place(s: &Sent) -> Option<Place> {
    let v: Vec<&str> = s.comment("lattice")?.split_whitespace().collect();
    let sent = v.first()?.parse().ok()?;
    if v.get(1) == Some(&"base") {
        return Some(Place { sent, change: None });
    }
    Some(Place { sent, change: Some((v.get(1)?.parse().ok()?, v.get(2)?.to_string(), v.get(3)?.parse().ok()?)) })
}

fn score(s: &Sent) -> Option<f64> {
    s.comment("gp_score")?.parse().ok()
}

/// Every reading against the base, for study: (sentence, token, label, tree score gain, log share ratio, words
/// added, gain of the mean score per word).
pub fn gains(parsed: &[Sent]) -> Vec<(usize, usize, String, f64, f64, i64, f64)> {
    let mut base: BTreeMap<usize, (f64, usize)> = BTreeMap::new();
    for s in parsed {
        if let (Some(Place { sent, change: None }), Some(sc)) = (place(s), score(s)) {
            base.insert(sent, (sc, s.n_words()));
        }
    }
    parsed.iter().filter_map(|s| {
        let Some(Place { sent, change: Some((ti, l, lr)) }) = place(s) else { return None };
        let (b, bn) = base.get(&sent)?;
        let sc = score(s)?;
        Some((sent, ti, l, sc - b, lr, s.n_words() as i64 - *bn as i64, sc / s.n_words().max(1) as f64 - b / (*bn).max(1) as f64))
    }).collect()
}

/// What each word chose: sentence → token → (label, gain).
pub type Choice = BTreeMap<usize, BTreeMap<usize, (String, f64)>>;

/// From the parsed readings, the candidate each word takes (only the words that leave the base).
pub fn choose(parsed: &[Sent], lambda: f64, mu: f64) -> Result<Choice, String> {
    let mut base: BTreeMap<usize, (f64, usize)> = BTreeMap::new();
    for s in parsed {
        let p = place(s).ok_or_else(|| format!("{}: no lattice comment", s.id()))?;
        if p.change.is_none() {
            base.insert(p.sent, (score(s).ok_or_else(|| format!("{}: no gp_score (GP_SCORE=1)", s.id()))?, s.n_words()));
        }
    }
    let mut out: Choice = BTreeMap::new();
    for s in parsed {
        let Some(Place { sent, change: Some((ti, l, lr)) }) = place(s) else { continue };
        let (b, bn) = base[&sent];
        let gain = score(s).unwrap_or(f64::MIN) - b + lambda * lr + mu * (s.n_words() as f64 - bn as f64);
        if gain > 0. {
            let e = out.entry(sent).or_default();
            if e.get(&ti).is_none_or(|x| gain > x.1) {
                e.insert(ti, (l, gain));
            }
        }
    }
    Ok(out)
}

/// The sentences cut with the chosen candidates (the base elsewhere); a word that left the base carries
/// `Lattice=<label>:<gain>` in its first word's MISC.
pub fn apply(m: &Model, sents: &[Sent], ch: &Choice, o: &Opts, cut: &dyn Fn(&Model, &str, &[Piece]) -> Token) -> Vec<Sent> {
    sents.iter().enumerate().map(|(si, s)| {
        let mut s2 = s.clone();
        let keys = m.keys(&s.tokens.iter().map(|t| t.form.as_str()).collect::<Vec<_>>());
        s2.comments.retain(|c| c.starts_with("# sent_id") || c.starts_with("# text"));
        for (ti, t) in s2.tokens.iter_mut().enumerate() {
            if punct(t) {
                continue;
            }
            *t = match ch.get(&si).and_then(|x| x.get(&ti)) {
                Some((l, g)) => {
                    let mut nt = cut(m, &t.form, &parse_label(l));
                    let w = &mut nt.words[0].cols[9];
                    *w = format!("{w}|Lattice={l}:{g:.3}");
                    nt
                }
                None => cut(m, &t.form, &alternatives(m, &keys, ti, o)[0].0),
            };
        }
        s2
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conllu::read;

    #[test]
    fn readings_and_choice() {
        let mk = |form: &str, ps: &[Piece]| Token { form: form.into(), misc: String::new(), words: Model::cut(form, ps).into_iter().map(|(f, _)| crate::conllu::Word::new(&f)).collect() };
        let cut = |_: &Model, f: &str, ps: &[Piece]| mk(f, ps);
        // a word seen split two ways, the commoner chosen
        let one = "1-2\tבבית\t_\t_\t_\t_\t_\t_\t_\t_\n1\tב\t_\tADP\t_\t_\t2\tcase\t_\t_\n2\tבית\t_\tNOUN\t_\t_\t0\troot\t_\t_\n\n";
        let two = "1-3\tבבית\t_\t_\t_\t_\t_\t_\t_\t_\n1\tב\t_\tADP\t_\t_\t3\tcase\t_\t_\n2\tה\t_\tDET\t_\t_\t3\tdet\t_\tMId=xה\n3\tבית\t_\tNOUN\t_\t_\t0\troot\t_\t_\n\n";
        let (m, _) = Model::learn(&read(&format!("{one}{one}{two}")), crate::text::Level::Unpointed, crate::seg::Opts { min: 1, ..Default::default() });
        let gold = read(&format!("# sent_id = a\n{two}"));
        let o = Opts { k: 3, ratio: 0.1 };
        let v = variants(&m, &gold, &o, &cut);
        assert_eq!(v.len(), 2);
        assert_eq!(v[0].comment("lattice"), Some("0 base"));
        assert_eq!(v[1].comment("lattice").map(|c| c.split(' ').take(3).collect::<Vec<_>>().join(" ")), Some("0 0 1.H.3".into()));
        assert_eq!((v[0].n_words(), v[1].n_words()), (2, 3));
        // the parser likes the reading with the article better: it is chosen; then the sentence is cut so
        let mut parsed = v.clone();
        parsed[0].comments.push("# gp_score = -2.0".into());
        parsed[1].comments.push("# gp_score = -1.5".into());
        let ch = choose(&parsed, 0.5, 0.0).unwrap();
        assert_eq!(ch[&0][&0].0, "1.H.3");
        let out = apply(&m, &gold, &ch, &o, &cut);
        assert_eq!(out[0].n_words(), 3);
        assert!(out[0].tokens[0].words[0].cols[9].contains("Lattice=1.H.3"));
        // negative control: a reading the parser likes less keeps the base
        parsed[1].comments.retain(|c| !c.starts_with("# gp_score"));
        parsed[1].comments.push("# gp_score = -2.5".into());
        let ch = choose(&parsed, 0.5, 0.0).unwrap();
        assert!(ch.is_empty());
        assert_eq!(apply(&m, &gold, &ch, &o, &cut)[0].n_words(), 2);
        // and parsed readings without scores are refused
        assert!(choose(&v, 1.0, 0.0).is_err());
    }
}
#[cfg(test)]
mod wave32 {
    use super::*;
    use crate::{cut_with, mark_token};
    fn reading(place: &str, score: Option<&str>, n: usize) -> Sent {
        let mut comments = vec![format!("# lattice = {place}")];
        if let Some(sc) = score {
            comments.push(format!("# gp_score = {sc}"));
        }
        Sent {
            comments,
            tokens: (0..n).map(|_| mark_token("x")).collect(),
        }
    }
    #[test]
    fn supplied_readings_ties_missing_and_apply() {
        let m = Model::load("# hbo segmenter\tlevel=unpointed\nlex\tאב\t1.1\t2\nlex\tאב\t2\t1\n");
        let s = crate::conllu::read("1\tאב\t_\tNOUN\t_\t_\t0\troot\t_\t_");
        let opts = Opts { k: 2, ratio: 0.5 };
        let v = variants(&m, &s, &opts, &cut_with);
        assert_eq!(v.len(), 2);
        assert_eq!((v[0].n_words(), v[1].n_words()), (2, 1));
        let parsed = vec![
            reading("0 base", Some("-2"), 2),
            reading("0 0 2 0", Some("-1"), 1),
            reading("0 0 1.1 0", Some("-1"), 2),
        ];
        let ch = choose(&parsed, 0., 0.).unwrap();
        assert_eq!(ch[&0][&0], ("2".into(), 1.));
        let out = apply(&m, &s, &ch, &opts, &cut_with);
        assert_eq!(out[0].n_words(), 1);
        assert!(out[0].tokens[0].words[0].cols[9].contains("Lattice=2:1.000"));
        assert_eq!(gains(&parsed)[0].5, -1);
        assert!(choose(&[reading("0 base", None, 0)], 0., 0.).is_err());
        assert!(
            choose(
                &[reading("0 base", Some("0"), 0), reading("0 0 2 0", None, 1)],
                0.,
                0.
            )
            .unwrap()
            .is_empty()
        );
        assert!(
            std::panic::catch_unwind(|| choose(&[reading("0 0 2 0", Some("1"), 1)], 0., 0.))
                .is_err()
        );
        assert!(
            std::panic::catch_unwind(|| variants(&m, &s, &Opts { k: 0, ratio: 0. }, &cut_with))
                .is_err()
        );
    }
}
