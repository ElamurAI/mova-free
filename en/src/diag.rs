//! Parser diagnostics (`en parse-diag`): where the errors are, before any change to the parser.
//!
//! - **Distance to the root** (McDonald & Nivre 2007): labelled precision and recall of arcs by the depth of the
//!   dependent (1 = a child of the root); recall by gold depth, precision by predicted depth. Also by arc length.
//! - **Projectivity**: LAS of sentences with and without a non-projective gold arc, and of the non-projective arcs
//!   themselves (the arc-standard automaton cannot build them, so they bound its LAS from above).
//! - **Beam oracle** (Charniak & Johnson 2005): at each beam width, the LAS of the best tree and of the closest-to-gold
//!   tree among the distinct trees left in the final beam; the gap is the room a reranker has.
//! - **Cascading errors** (Ng & Curran 2015): for one class of relations at a time the gold arcs of that class are
//!   forced in the beam; the errors left outside the class show how many of them the class's own errors caused.
//!
//! Nothing here changes the parser: it only decodes with the trained weights.

use std::fmt::Write;

use crate::conllu::Sentence;
use crate::gram::{Rel, Tag};
use crate::parse::{Cons, Parser};

/// Error classes for constrained decoding (by the gold relation of the dependent).
pub const CLASSES: [&str; 10] = ["punct", "PP (obl, nmod)", "coordination", "clauses", "core arguments", "noun phrase", "function words", "modifiers", "root", "other"];

/// The class of a relation (an index into `CLASSES`).
pub fn class(r: Rel) -> usize {
    use Rel::*;
    match r {
        Punct => 0,
        Obl | OblAgent | OblNpmod | OblTmod | OblUnmarked | Nmod | NmodDesc | NmodNpmod | NmodTmod | NmodUnmarked => 1,
        Conj | Cc | CcPreconj => 2,
        Advcl | AdvclRelcl | Ccomp | Xcomp | Acl | AclRelcl | Csubj | CsubjOuter | CsubjPass | Parataxis => 3,
        Nsubj | NsubjOuter | NsubjPass | Obj | Iobj | Expl => 4,
        Det | DetPredet | Amod | Compound | Nummod | NmodPoss | Flat | FlatForeign | Appos => 5,
        Aux | AuxPass | Cop | Mark | Case | CompoundPrt => 6,
        Advmod | Discourse | Vocative | Dislocated => 7,
        Root => 8,
        _ => 9,
    }
}

/// Depth of each word (1..n, index 0 unused): steps up to the root; a cycle stops at n.
pub fn depths(heads: &[usize]) -> Vec<usize> {
    let n = heads.len();
    let mut d = vec![0usize; n + 1];
    for i in 1..=n {
        let (mut k, mut steps) = (i, 0usize);
        while k != 0 && steps <= n {
            k = heads[k - 1];
            steps += 1;
        }
        d[i] = steps;
    }
    d
}

/// Whether the arc of word `d` (1-based) is non-projective: some word between it and its head is not under the head.
pub fn nonprojective(heads: &[usize], d: usize) -> bool {
    let h = heads[d - 1];
    if h == 0 {
        return false;
    }
    let (lo, hi) = (h.min(d), h.max(d));
    (lo + 1..hi).any(|k| {
        let (mut x, mut steps) = (k, 0usize);
        while x != 0 && x != h && steps <= heads.len() {
            x = heads[x - 1];
            steps += 1;
        }
        x != h
    })
}

fn depth_bin(d: usize) -> usize {
    match d {
        0 | 1 => 0,
        2 => 1,
        3 | 4 => 2,
        _ => 3,
    }
}
const DEPTH_BINS: [&str; 4] = ["1 (root's child)", "2", "3–4", "5+"];

fn len_bin(l: usize) -> usize {
    match l {
        0 | 1 => 0,
        2 => 1,
        3..=6 => 2,
        _ => 3,
    }
}
const LEN_BINS: [&str; 4] = ["1", "2", "3–6", "7+"];

fn pct(a: usize, b: usize) -> f64 {
    100.0 * a as f64 / b.max(1) as f64
}

/// One test sentence: words, the tags the parser sees, the gold arcs.
struct Item<'a> {
    words: Vec<&'a str>,
    tags: Vec<Tag>,
    gold: Vec<(usize, Rel)>,
}

/// The full report for one test file. `tag` gives the tags the parser sees (ours or gold).
pub fn report(parser: &Parser, sents: &[Sentence], tag: &dyn Fn(&[&str], &Sentence) -> Vec<Tag>, widths: &[usize]) -> String {
    let items: Vec<Item> = sents
        .iter()
        .map(|s| {
            let words: Vec<&str> = s.tokens.iter().map(|t| t.form.as_str()).collect();
            let tags = tag(&words, s);
            Item { words, tags, gold: s.tokens.iter().map(|t| (t.head, t.rel)).collect() }
        })
        .collect();
    let preds: Vec<Vec<(usize, Rel)>> = items.iter().map(|it| parser.parse(&it.words, &it.tags)).collect();
    let mut out = String::new();
    let ntok: usize = items.iter().map(|it| it.gold.len()).sum();
    let las_of = |g: &[(usize, Rel)], p: &[(usize, Rel)]| g.iter().zip(p).filter(|(a, b)| a == b).count();
    let base_las: usize = items.iter().zip(&preds).map(|(it, p)| las_of(&it.gold, p)).sum();
    writeln!(out, "sentences {}, tokens {ntok}, beam {}: LAS {:.2}\n", items.len(), parser.beam, pct(base_las, ntok)).ok();

    // 1. distance to the root and arc length
    let (mut g_n, mut g_ok, mut p_n, mut p_ok) = ([0usize; 4], [0usize; 4], [0usize; 4], [0usize; 4]);
    let (mut l_n, mut l_ok) = ([0usize; 4], [0usize; 4]);
    for (it, p) in items.iter().zip(&preds) {
        let gh: Vec<usize> = it.gold.iter().map(|x| x.0).collect();
        let ph: Vec<usize> = p.iter().map(|x| x.0).collect();
        let (gd, pd) = (depths(&gh), depths(&ph));
        for i in 1..=gh.len() {
            let ok = it.gold[i - 1] == p[i - 1];
            let (gb, pb) = (depth_bin(gd[i]), depth_bin(pd[i]));
            g_n[gb] += 1;
            g_ok[gb] += ok as usize;
            p_n[pb] += 1;
            p_ok[pb] += ok as usize;
            let lb = len_bin(if gh[i - 1] == 0 { 1 } else { gh[i - 1].abs_diff(i) });
            l_n[lb] += 1;
            l_ok[lb] += ok as usize;
        }
    }
    writeln!(out, "| depth of the dependent | gold arcs | recall | predicted arcs | precision | F1 |\n|---|---:|---:|---:|---:|---:|").ok();
    for b in 0..4 {
        let (r, pr) = (pct(g_ok[b], g_n[b]), pct(p_ok[b], p_n[b]));
        writeln!(out, "| {} | {} | {r:.2} | {} | {pr:.2} | {:.2} |", DEPTH_BINS[b], g_n[b], p_n[b], 2.0 * r * pr / (r + pr).max(1e-9)).ok();
    }
    writeln!(out, "\n| gold arc length | arcs | LAS |\n|---|---:|---:|").ok();
    for b in 0..4 {
        writeln!(out, "| {} | {} | {:.2} |", LEN_BINS[b], l_n[b], pct(l_ok[b], l_n[b])).ok();
    }

    // 2. projectivity
    let (mut ps, mut ps_n, mut ps_ok, mut ns, mut ns_n, mut ns_ok, mut na_n, mut na_ok) = (0usize, 0usize, 0usize, 0usize, 0usize, 0usize, 0usize, 0usize);
    for (it, p) in items.iter().zip(&preds) {
        let gh: Vec<usize> = it.gold.iter().map(|x| x.0).collect();
        let np: Vec<bool> = (1..=gh.len()).map(|d| nonprojective(&gh, d)).collect();
        let ok = las_of(&it.gold, p);
        if np.iter().any(|&x| x) {
            ns += 1;
            ns_n += gh.len();
            ns_ok += ok;
        } else {
            ps += 1;
            ps_n += gh.len();
            ps_ok += ok;
        }
        for (i, &x) in np.iter().enumerate() {
            if x {
                na_n += 1;
                na_ok += (it.gold[i] == p[i]) as usize;
            }
        }
    }
    writeln!(out, "\n| gold sentences | sentences | tokens | LAS |\n|---|---:|---:|---:|").ok();
    writeln!(out, "| projective | {ps} | {ps_n} | {:.2} |", pct(ps_ok, ps_n)).ok();
    writeln!(out, "| with a non-projective arc | {ns} | {ns_n} | {:.2} |", pct(ns_ok, ns_n)).ok();
    writeln!(out, "| — the non-projective arcs themselves | — | {na_n} | {:.2} |", pct(na_ok, na_n)).ok();
    writeln!(out, "\nNon-projective gold arcs: {na_n} of {ntok} ({:.2}%). The parser's trees are projective, so at least one arc of each such crossing is lost: the ceiling is LAS ≥ {:.2}.", pct(na_n, ntok), pct(ntok - na_n, ntok)).ok();

    out.push_str(&pp_report(sents, &preds));

    // 3. beam oracle
    writeln!(out, "\n| beam width | 1-best LAS | oracle LAS | gap | distinct trees per sentence |\n|---:|---:|---:|---:|---:|").ok();
    for &k in widths {
        let (mut best, mut orc, mut trees) = (0usize, 0usize, 0usize);
        for it in &items {
            let kb = parser.parse_kbest(&it.words, &it.tags, k);
            trees += kb.len();
            best += kb.first().map_or(0, |(_, t)| las_of(&it.gold, t));
            orc += kb.iter().map(|(_, t)| las_of(&it.gold, t)).max().unwrap_or(0);
        }
        writeln!(out, "| {k} | {:.2} | {:.2} | {:.2} | {:.2} |", pct(best, ntok), pct(orc, ntok), pct(orc - best, ntok), trees as f64 / items.len().max(1) as f64).ok();
    }

    // 4. cascading errors: force one class at a time
    let width = parser.beam.max(1);
    let base_err: Vec<[usize; 2]> = (0..CLASSES.len())
        .map(|c| {
            let mut e = [0usize; 2];
            for (it, p) in items.iter().zip(&preds) {
                for (g, x) in it.gold.iter().zip(p) {
                    if g != x {
                        e[(class(g.1) != c) as usize] += 1;
                    }
                }
            }
            e
        })
        .collect();
    writeln!(
        out,
        "\n| class forced | gold arcs | its errors | other errors before | after | cascaded (fixed elsewhere) | share of other errors | LAS forced | no tree |\n|---|---:|---:|---:|---:|---:|---:|---:|---:|"
    )
    .ok();
    for (c, name) in CLASSES.iter().enumerate() {
        let (mut arcs, mut after, mut las, mut dead) = (0usize, 0usize, 0usize, 0usize);
        for (it, p) in items.iter().zip(&preds) {
            let mut arc = vec![None; it.gold.len() + 1];
            for (i, g) in it.gold.iter().enumerate() {
                if class(g.1) == c {
                    arc[i + 1] = Some(*g);
                    arcs += 1;
                }
            }
            let t = if arc.iter().all(Option::is_none) {
                p.clone()
            } else {
                match parser.parse_constrained(&it.words, &it.tags, &Cons::new(arc), width) {
                    Some(t) => t,
                    None => {
                        dead += 1;
                        p.clone()
                    }
                }
            };
            las += las_of(&it.gold, &t);
            after += it.gold.iter().zip(&t).filter(|(g, x)| g != x && class(g.1) != c).count();
        }
        let [own, other] = base_err[c];
        let fixed = other as i64 - after as i64;
        writeln!(out, "| {name} | {arcs} | {own} | {other} | {after} | {fixed} | {:.1}% | {:.2} | {dead} |", 100.0 * fixed as f64 / other.max(1) as f64, pct(las, ntok)).ok();
    }
    out
}

/// PP attachment read off the predicted tree (Atterer & Schütze 2007), not from gold (v, n1, p, n2) tuples.
/// A PP is a gold `obl` or `nmod` (not `nmod:poss`) word with a `case` child tagged IN or TO whose gold head is a noun
/// or a verb. The parser **found** the PP if its tree also has that preposition as `case` of the word and the word as
/// `obl` or `nmod`; otherwise the case is **missed**. Attachment is right when the word's predicted head is the gold
/// head. Three accuracies, as the paper asks: missed cases counted as errors, given the noun attachment, discarded.
/// The floors (Volk 2006, Rosenthal 2010): the share of noun attachments (always-noun), and «of goes to the noun».
pub fn pp_report(sents: &[Sentence], preds: &[Vec<(usize, Rel)>]) -> String {
    use crate::gram::UPos;
    // rows: all, of, other prepositions, gold head noun, gold head verb
    const ROWS: [&str; 5] = ["all PPs", "*of*", "other prepositions", "gold head a noun", "gold head a verb"];
    // n, missed, right among found, nominal gold head, nominal gold head among missed
    let mut c = [[0usize; 5]; 5];
    let mut of_rule = [0usize; 2];
    for (s, p) in sents.iter().zip(preds) {
        let tk = &s.tokens;
        for (i, t) in tk.iter().enumerate() {
            let w = i + 1;
            if !matches!(t.rel.base(), Rel::Obl | Rel::Nmod) || t.rel == Rel::NmodPoss || t.head == 0 {
                continue;
            }
            let Some(prep) = (0..tk.len()).find(|&k| tk[k].head == w && tk[k].rel == Rel::Case && matches!(tk[k].tag, Some(Tag::IN) | Some(Tag::TO))) else { continue };
            let hu = tk[t.head - 1].upos;
            let noun = matches!(hu, Some(UPos::NOUN) | Some(UPos::PROPN) | Some(UPos::PRON) | Some(UPos::NUM));
            let verb = matches!(hu, Some(UPos::VERB));
            if !noun && !verb {
                continue;
            }
            let found = p[prep] == (w, Rel::Case) && matches!(p[i].1.base(), Rel::Obl | Rel::Nmod) && p[i].1 != Rel::NmodPoss;
            let right = found && p[i].0 == t.head;
            let is_of = tk[prep].form.eq_ignore_ascii_case("of");
            if is_of {
                of_rule[0] += 1;
                of_rule[1] += noun as usize;
            }
            for (r, on) in [true, is_of, !is_of, noun, verb].into_iter().enumerate() {
                if on {
                    c[r][0] += 1;
                    c[r][1] += (!found) as usize;
                    c[r][2] += right as usize;
                    c[r][3] += noun as usize;
                    c[r][4] += (!found && noun) as usize;
                }
            }
        }
    }
    let mut out = String::from(
        "\n| PP attachment (predicted tree) | PPs | missed | right, missed = error | right, missed → noun | right, missed discarded | always-noun floor |\n|---|---:|---:|---:|---:|---:|---:|\n",
    );
    for (r, name) in ROWS.iter().enumerate() {
        let [n, miss, right, nouns, miss_noun] = c[r];
        writeln!(out, "| {name} | {n} | {miss} | {:.2} | {:.2} | {:.2} | {:.2} |", pct(right, n), pct(right + miss_noun, n), pct(right, n - miss), pct(nouns, n)).ok();
    }
    writeln!(out, "\n«*of* goes to the noun» is right for {:.2}% of the {} *of*-PPs.", pct(of_rule[1], of_rule[0]), of_rule[0]).ok();
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn depth_and_projectivity() {
        // 1 ← 2 → 3 (root 2); then the same with 4 hanging on 1 across 2 and 3: non-projective
        assert_eq!(depths(&[2, 0, 2]), vec![0, 2, 1, 2]);
        assert!(!(1..=3).any(|d| nonprojective(&[2, 0, 2], d)));
        let h = [2, 0, 2, 1];
        assert!(nonprojective(&h, 4));
        assert!(!nonprojective(&h, 3));
    }
}

#[cfg(test)]
mod wave27 {
    use super::*;
    #[test]
    fn cycles_empty_and_bins() {
        assert_eq!(depths(&[]), [0]);
        assert_eq!(depths(&[1]), [0, 2]);
        assert!(!nonprojective(&[0], 1));
        assert_eq!(class(Rel::NmodPoss), 5);
        assert_eq!(class(Rel::Root), 8);
        assert_eq!(pct(0, 0), 0.0);
        assert_eq!(
            (depth_bin(4), depth_bin(5), len_bin(6), len_bin(7)),
            (2, 3, 2, 3)
        );
    }
}
