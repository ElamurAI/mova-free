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
        for w in crate::events::book_body(&t).split(|c: char| !c.is_alphabetic()).filter(|w| w.len() > 1) {
            *m.entry(w.to_lowercase()).or_default() += 1;
            n += 1;
        }
    }
    (m, n)
}

/// `world register-stats`.
pub fn run(name: &str, reg: &[PathBuf], refs: &[PathBuf], out: &Path, zmin: f64, min: u64) -> Result<()> {
    let (a, na) = counts(reg);
    let (b, nb) = counts(refs);
    // prior: the pooled corpus, scaled to a total of a0 pseudo-counts
    let a0 = 1000.0;
    let total = (na + nb) as f64;
    let mut rows: Vec<(String, f64, u64, u64)> = Vec::new();
    let mut words: Vec<&String> = a.keys().chain(b.keys()).collect();
    words.sort();
    words.dedup();
    for w in words {
        let (ya, yb) = (*a.get(w).unwrap_or(&0) as f64, *b.get(w).unwrap_or(&0) as f64);
        let alpha = a0 * (ya + yb) / total;
        let la = ((ya + alpha) / (na as f64 + a0 - ya - alpha)).ln();
        let lb = ((yb + alpha) / (nb as f64 + a0 - yb - alpha)).ln();
        let var = 1.0 / (ya + alpha) + 1.0 / (yb + alpha);
        let z = (la - lb) / var.sqrt();
        if z >= zmin && ya as u64 >= min {
            rows.push((w.clone(), z, ya as u64, yb as u64));
        }
    }
    rows.sort_by(|x, y| y.1.total_cmp(&x.1).then(x.0.cmp(&y.0)));
    let mut f = std::io::BufWriter::new(std::fs::File::create(out)?);
    writeln!(f, "# register {name}: words used far more than in the reference (log-odds, informative Dirichlet prior; Monroe et al. 2008); world register-stats\n# register corpus {} books, {na} tokens; reference {} books, {nb} tokens\nword\tz\tcount\tref_count\tper_million\tref_per_million", reg.len(), refs.len())?;
    for (w, z, ya, yb) in &rows {
        writeln!(f, "{w}\t{z:.2}\t{ya}\t{yb}\t{:.1}\t{:.1}", *ya as f64 * 1e6 / na as f64, *yb as f64 * 1e6 / nb as f64)?;
    }
    f.flush()?;
    println!("{name}: {} words with z ≥ {zmin} (register {na} tokens, reference {nb} tokens); top: {}", rows.len(), rows.iter().take(40).map(|r| r.0.as_str()).collect::<Vec<_>>().join(" "));
    Ok(())
}
