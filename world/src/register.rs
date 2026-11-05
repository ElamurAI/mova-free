//! Register lexicons from corpus statistics: which words a register uses far more than a reference corpus —
//! log-odds with an informative Dirichlet prior (Monroe, Colaresi & Quinn 2008, "Fightin' Words"), z-scored.
//! Wiktionary marks few formal words (pursuant, notwithstanding, whereas have no label), so the register is
//! measured, not listed.
//!
//!   world register-stats <name> <register-books>... --ref <reference-books>... --out <words.tsv> [--z 3] [--min 20]
//!
//! Tokens: lowercase alphabetic words of the book bodies (Gutenberg markers stripped). Output: word, z, count in
//! the register corpus, count in the reference, per million in each — words with z ≥ `--z` and count ≥ `--min`.

use std::collections::BTreeMap;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::Result;

fn counts(books: &[PathBuf]) -> (BTreeMap<String, u64>, u64) {
    let mut m: BTreeMap<String, u64> = BTreeMap::new();
    let mut n = 0u64;
    for b in books {
        let Ok(t) = std::fs::read_to_string(b) else { continue };
        count_body(crate::events::book_body(&t), &mut m, &mut n);
    }
    (m, n)
}

/// `world register-stats`.
fn count_body(body: &str, m: &mut BTreeMap<String, u64>, n: &mut u64) {
    for w in body.split(|c: char| !c.is_alphabetic()).filter(|w| w.len() > 1) {
        *m.entry(w.to_lowercase()).or_default() += 1;
        *n += 1;
    }
}

/// Execute register statistics.
pub fn run(name: &str, reg: &[PathBuf], refs: &[PathBuf], out: &Path, zmin: f64, min: u64) -> Result<()> {
    let (a, na) = counts(reg);
    let (b, nb) = counts(refs);
    let rows = ranked_rows(&a, na, &b, nb, zmin, min);
    let mut f = std::io::BufWriter::new(std::fs::File::create(out)?);
    writeln!(f, "# register {name}: words used far more than in the reference (log-odds, informative Dirichlet prior; Monroe et al. 2008); world register-stats\n# register corpus {} books, {na} tokens; reference {} books, {nb} tokens\nword\tz\tcount\tref_count\tper_million\tref_per_million", reg.len(), refs.len())?;
    for (w, z, ya, yb) in &rows {
        writeln!(f, "{w}\t{z:.2}\t{ya}\t{yb}\t{:.1}\t{:.1}", *ya as f64 * 1e6 / na as f64, *yb as f64 * 1e6 / nb as f64)?;
    }
    f.flush()?;
    println!("{name}: {} words with z ≥ {zmin} (register {na} tokens, reference {nb} tokens); top: {}", rows.len(), rows.iter().take(40).map(|r| r.0.as_str()).collect::<Vec<_>>().join(" "));
    Ok(())
}

/// Pure ranking boundary; preserves IEEE degeneracies of the corpus formula.
fn ranked_rows(a: &BTreeMap<String, u64>, na: u64, b: &BTreeMap<String, u64>, nb: u64, zmin: f64, min: u64) -> Vec<(String, f64, u64, u64)> {
    // prior: the pooled corpus, scaled to a total of a0 pseudo-counts
    let a0 = 1000.0;
    let total = (na + nb) as f64;
    let mut rows: Vec<(String, f64, u64, u64)> = Vec::new();
    let mut words: Vec<&String> = a.keys().chain(b.keys()).collect();
    words.sort();
    words.dedup();
    for w in words {
        let (ca, cb) = (*a.get(w).unwrap_or(&0), *b.get(w).unwrap_or(&0));
        let (ya, yb) = (ca as f64, cb as f64);
        let alpha = a0 * (ya + yb) / total;
        let la = ((ya + alpha) / (na as f64 + a0 - ya - alpha)).ln();
        let lb = ((yb + alpha) / (nb as f64 + a0 - yb - alpha)).ln();
        let var = 1.0 / (ya + alpha) + 1.0 / (yb + alpha);
        let z = (la - lb) / var.sqrt();
        if z >= zmin && ca >= min {
            rows.push((w.clone(), z, ca, cb));
        }
    }
    rows.sort_by(|x, y| y.1.total_cmp(&x.1).then(x.0.cmp(&y.0)));
    rows
}

#[cfg(test)]
mod corpus_models {
    use super::*;
    fn histogram(xs: &[(&str, u64)]) -> BTreeMap<String, u64> {
        xs.iter().map(|(w, n)| (w.to_string(), *n)).collect()
    }
    #[test]
    fn unicode_byte_length_and_case_expansion() {
        let (mut m, mut n) = (BTreeMap::new(), 0);
        count_body("A I é É İ hello HELLO co-op 123", &mut m, &mut n);
        assert_eq!(n, 7);
        assert_eq!(m, histogram(&[("é", 2), ("i\u{307}", 1), ("hello", 2), ("co", 1), ("op", 1)]));
    }
    #[test]
    fn pooled_prior_matches_independent_odds_and_swap() {
        for x in 1..9 {
            for y in 1..9 {
                let a = histogram(&[("x", x), ("rest", 10 - x)]);
                let b = histogram(&[("x", y), ("rest", 10 - y)]);
                let rows = ranked_rows(&a, 10, &b, 10, f64::NEG_INFINITY, 0);
                let z = rows.iter().find(|r| r.0 == "x").unwrap().1;
                let prior = 50.0 * (x + y) as f64;
                let aa = x as f64 + prior;
                let bb = y as f64 + prior;
                let expected = (aa.ln() - (1010.0 - aa).ln() - bb.ln() + (1010.0 - bb).ln())
                    / (aa.recip() + bb.recip()).sqrt();
                assert!((z - expected).abs() < 1e-12);
                let swapped = ranked_rows(&b, 10, &a, 10, f64::NEG_INFINITY, 0);
                assert!((z + swapped.iter().find(|r| r.0 == "x").unwrap().1).abs() < 1e-12);
            }
        }
    }
    #[test]
    fn exact_thresholds_and_lexical_ties() {
        let a = histogram(&[("z", 2), ("a", 2)]);
        assert_eq!(ranked_rows(&a, 4, &a, 4, 0.0, 2).iter().map(|r| r.0.as_str()).collect::<Vec<_>>(), vec!["a", "z"]);
        assert!(ranked_rows(&a, 4, &a, 4, f64::EPSILON, 0).is_empty());
        assert!(ranked_rows(&a, 4, &a, 4, 0.0, 3).is_empty());
        assert!(ranked_rows(&a, 4, &a, 4, f64::NAN, 0).is_empty());
    }
    #[test]
    fn empty_and_singleton_degeneracies_are_excluded() {
        let empty = BTreeMap::new();
        let one = histogram(&[("only", 10)]);
        assert!(ranked_rows(&empty, 0, &empty, 0, f64::NEG_INFINITY, 0).is_empty());
        assert!(ranked_rows(&one, 10, &one, 10, f64::NEG_INFINITY, 0).is_empty());
        assert!(ranked_rows(&one, 10, &empty, 0, f64::NEG_INFINITY, 0).is_empty());
        let two = histogram(&[("a", 2), ("b", 8)]);
        let rows = ranked_rows(&two, 10, &empty, 0, f64::NEG_INFINITY, 0);
        assert_eq!(rows.len(), 2);
        assert!(rows.iter().all(|r| r.1.is_finite() && r.3 == 0));
    }
    #[test]
    fn segmentation_accumulation_and_body_boundary() {
        let (mut m, mut n) = (BTreeMap::new(), 0);
        let t = "ignored words\n*** START OF BOOK\nAlpha beta.\n*** END OF BOOK\nignored words";
        count_body(crate::events::book_body(t), &mut m, &mut n);
        count_body("beta\tgamma", &mut m, &mut n);
        assert_eq!((m, n), (histogram(&[("alpha", 1), ("beta", 2), ("gamma", 1)]), 4));
    }
    #[test]
    fn large_singleton_rounding_can_admit_zero_score() {
        let na = (1u64 << 53) + 1;
        let nb = (1u64 << 53) + 2;
        let a = histogram(&[("only", na)]);
        let b = histogram(&[("only", nb)]);
        let alpha = 1000.0 * (na as f64 + nb as f64) / (na + nb) as f64;
        assert!(alpha < 1000.0);
        assert!(na as f64 + 1000.0 - na as f64 - alpha > 0.0);
        assert!(nb as f64 + 1000.0 - nb as f64 - alpha > 0.0);
        let rows = ranked_rows(&a, na, &b, nb, 0.0, 0);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].0, "only");
        assert_eq!(rows[0].1, 0.0);
        assert_eq!(rows[0].2, na);
        assert_eq!(rows[0].3, nb);
    }

    #[test]
    fn large_integer_count_thresholds_remain_exact() {
        for ca in [(1u64 << 53) + 1, (1u64 << 53) + 3] {
            let a = histogram(&[("large", ca), ("rest", 1024)]);
            let total = ca + 1024;
            let admitted = ranked_rows(&a, total, &a, total, 0.0, ca);
            assert_eq!(admitted.len(), 1);
            assert_eq!(admitted[0].0, "large");
            assert_eq!(admitted[0].2, ca);
            assert_eq!(admitted[0].3, ca);
            assert!(ranked_rows(&a, total, &a, total, 0.0, ca + 1).is_empty());
        }
    }

}
