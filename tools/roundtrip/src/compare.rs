//! `roundtrip compare <before> <after>`: two runs' `residual.tsv` and `normal.tsv` side by side — bits per sentence
//! by set, layer and class, and the normal mode's exact %, chrF and divergent words per 100 tokens by start and
//! class. Only what changed is listed, with the layer totals and every start's chrF always.

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::{Context, Result};

/// (set, layer, class) → (bits, sentences)
type Res = BTreeMap<(String, String, String), (f64, f64)>;
/// (set, start) → (sentences, exact, chrF sum, tokens, class → items)
type Norm = BTreeMap<(String, String), (f64, f64, f64, f64, BTreeMap<String, f64>)>;

fn rows(p: &Path) -> Result<Vec<Vec<String>>> {
    let s = std::fs::read_to_string(p).with_context(|| format!("{}", p.display()))?;
    Ok(s.lines().skip(1).filter(|l| !l.is_empty()).map(|l| l.split('\t').map(str::to_string).collect()).collect())
}

fn num(s: &str) -> f64 {
    s.parse().unwrap_or(0.0)
}

fn residual(dir: &Path) -> Result<Res> {
    let mut m = Res::new();
    for r in rows(&dir.join("residual.tsv"))? {
        if r.len() >= 7 {
            m.insert((r[0].clone(), r[1].clone(), r[2].clone()), (num(&r[4]), num(&r[5])));
        }
    }
    Ok(m)
}

fn normal(dir: &Path) -> Result<Norm> {
    let mut m = Norm::new();
    for r in rows(&dir.join("normal.tsv"))? {
        if r.len() >= 8 {
            let e = m.entry((r[0].clone(), r[1].clone())).or_insert((num(&r[2]), num(&r[3]), num(&r[4]), num(&r[5]), BTreeMap::new()));
            *e.4.entry(r[6].clone()).or_default() += num(&r[7]);
        }
    }
    Ok(m)
}

pub fn compare(a: &Path, b: &Path) -> Result<String> {
    let (ra, rb) = (residual(a)?, residual(b)?);
    let (na, nb) = (normal(a)?, normal(b)?);
    let mut out = format!("# Round trip compared\n\nbefore `{}`, after `{}`\n\n", a.display(), b.display());
    let sets: Vec<String> = {
        let mut v: Vec<String> = Vec::new();
        for k in ra.keys().map(|k| &k.0).chain(na.keys().map(|k| &k.0)) {
            if !v.contains(k) {
                v.push(k.clone());
            }
        }
        v
    };
    for set in &sets {
        out += &format!("## {set}\n\nResidual, bits per sentence (before → after), classes that changed and the layer totals without meta:\n\n| layer | class | before | after | change |\n|---|---|---:|---:|---:|\n");
        let mut keys: Vec<(String, String)> = ra.keys().chain(rb.keys()).filter(|k| &k.0 == set).map(|k| (k.1.clone(), k.2.clone())).collect();
        // Include the full key so equal-class unknown layers become adjacent before dedup.
        keys.sort_by_key(|(l, c)| (["tokens", "tree", "frames", "world"].iter().position(|x| x == l).unwrap_or(9), c.clone(), l.clone()));
        keys.dedup();
        let per = |m: &Res, l: &str, c: &str| m.get(&(set.clone(), l.to_string(), c.to_string())).map_or(0.0, |(b, s)| if *s > 0.0 { b / s } else { 0.0 });
        let mut tot: BTreeMap<String, (f64, f64)> = BTreeMap::new();
        for (l, c) in &keys {
            let (x, y) = (per(&ra, l, c), per(&rb, l, c));
            if c != "meta" {
                let t = tot.entry(l.clone()).or_default();
                t.0 += x;
                t.1 += y;
            }
            if (x - y).abs() >= 0.05 {
                out += &format!("| {l} | {c} | {x:.1} | {y:.1} | {:+.1} |\n", y - x);
            }
        }
        for l in ["tokens", "tree", "frames", "world"] {
            if let Some((x, y)) = tot.get(l) {
                out += &format!("| **{l}** | **all** | {x:.1} | {y:.1} | {:+.1} |\n", y - x);
            }
        }
        out += "\nNormal mode: exact % / chrF, and divergent words per 100 tokens of the classes that changed:\n\n| start | exact % | chrF | classes |\n|---|---|---|---|\n";
        for ((s, start), x) in na.iter().filter(|(k, _)| &k.0 == set) {
            let Some(y) = nb.get(&(s.clone(), start.clone())) else { continue };
            let pct = |v: f64, n: f64| if n > 0.0 { 100.0 * v / n } else { 0.0 };
            let mut cls = Vec::new();
            let names: Vec<&String> = {
                let mut v: Vec<&String> = x.4.keys().chain(y.4.keys()).collect();
                v.sort();
                v.dedup();
                v
            };
            for c in names {
                let (p, q) = (pct(*x.4.get(c).unwrap_or(&0.0), x.3), pct(*y.4.get(c).unwrap_or(&0.0), y.3));
                if (p - q).abs() >= 0.05 {
                    cls.push(format!("{c} {p:.1} → {q:.1}"));
                }
            }
            out += &format!("| {start} | {:.1} → {:.1} | {:.2} → {:.2} | {} |\n", pct(x.1, x.0), pct(y.1, y.0), 100.0 * x.2 / x.0.max(1.0), 100.0 * y.2 / y.0.max(1.0), cls.join(", "));
        }
        out += "\n";
    }
    Ok(out)
}
#[cfg(test)]
mod wave32_tests {
    use super::*;
    #[test]
    fn lossy_rows_and_before_domain() {
        // WAVE32_TEMP names a fresh directory; without it the test uses its own one under the system temp dir.
        let root = std::env::var_os("WAVE32_TEMP").map(std::path::PathBuf::from).unwrap_or_else(|| {
            std::env::temp_dir().join(format!("roundtrip-wave32-{}", std::process::id()))
        });
        let a = root.join("a");
        let b = root.join("b");
        std::fs::create_dir_all(&a).unwrap();
        std::fs::create_dir_all(&b).unwrap();
        for p in [&a, &b] {
            std::fs::write(p.join("normal.tsv"), "header\n").unwrap();
        }
        std::fs::write(
            a.join("residual.tsv"),
            "header\ns\ttokens\tspacing\t0\t4\t2\tx\n",
        )
        .unwrap();
        std::fs::write(
            b.join("residual.tsv"),
            "header\ns\ttokens\tspacing\t0\t8\t2\tx\nonly-after\ttokens\tspacing\t0\t9\t1\tx\n",
        )
        .unwrap();
        let text = compare(&a, &b).unwrap();
        assert!(text.contains("| tokens | spacing | 2.0 | 4.0 | +2.0 |"));
        assert!(!text.contains("only-after"));
        assert_eq!(num("bad"), 0.0);
        assert!(num("NaN").is_nan());
        std::fs::remove_dir_all(root).unwrap();
    }
}

#[cfg(test)]
mod wave37_tests {
    use super::*;

    #[test]
    fn unknown_residual_layers_are_unique_and_ordered() {
        let root = std::env::temp_dir().join(format!(
            "mova-compare-unknown-layers-{}",
            std::process::id()
        ));
        let a = root.join("before");
        let b = root.join("after");
        for p in [&a, &b] {
            std::fs::create_dir_all(p).unwrap();
            std::fs::write(p.join("normal.tsv"), "header\n").unwrap();
        }
        let layers = ["alpha", "beta", "world", "tokens", "frames", "tree"];
        let mut reference = None;
        for reverse_before in [false, true] {
            for reverse_after in [false, true] {
                for (path, bits, reverse) in [(&a, 2, reverse_before), (&b, 6, reverse_after)] {
                    let mut rows: Vec<_> = layers
                        .iter()
                        .map(|layer| format!("s\t{layer}\tspacing\t0\t{bits}\t2\tx\n"))
                        .collect();
                    if reverse {
                        rows.reverse();
                    }
                    std::fs::write(path.join("residual.tsv"), format!("header\n{}", rows.concat()))
                        .unwrap();
                }
                let text = compare(&a, &b).unwrap();
                for layer in ["alpha", "beta"] {
                    let row = format!("| {layer} | spacing | 1.0 | 3.0 | +2.0 |");
                    assert_eq!(text.matches(&row).count(), 1, "{text}");
                }
                let changed: Vec<_> = text
                    .lines()
                    .filter(|line| line.contains("| spacing |"))
                    .collect();
                assert_eq!(
                    changed,
                    [
                        "| tokens | spacing | 1.0 | 3.0 | +2.0 |",
                        "| tree | spacing | 1.0 | 3.0 | +2.0 |",
                        "| frames | spacing | 1.0 | 3.0 | +2.0 |",
                        "| world | spacing | 1.0 | 3.0 | +2.0 |",
                        "| alpha | spacing | 1.0 | 3.0 | +2.0 |",
                        "| beta | spacing | 1.0 | 3.0 | +2.0 |",
                    ]
                );
                if let Some(expected) = &reference {
                    assert_eq!(&text, expected);
                } else {
                    reference = Some(text);
                }
            }
        }
        std::fs::remove_dir_all(root).unwrap();
    }
}
