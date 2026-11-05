//! The construction gate (`en construct-eval`): recall of the key arcs of hard constructions on our own held-out
//! sentences (`data/constructions.tsv`), per construction, after Rimell, Clark & Steedman (2009). LAS on a treebank
//! can rise while a rare construction breaks; the gate fails a model whose recall on any construction falls below
//! the floor recorded for the last accepted model (`data/constructions-floor.tsv`).

use std::collections::BTreeMap;
use std::fmt::Write;
use std::path::Path;

use anyhow::{Context, Result, bail};

use crate::gram::Rel;

/// One sentence of the set: its construction, tokens and target arcs (head, dependent, relation; 1-based, 0 = root).
pub struct Item {
    pub cx: String,
    pub words: Vec<String>,
    pub arcs: Vec<(usize, usize, Rel)>,
}

/// `form-i` → i, checking that token i is that form.
fn node(spec: &str, words: &[String], line: usize) -> Result<usize> {
    let Some((form, i)) = spec.rsplit_once('-') else { bail!("line {line}: '{spec}' is not form-index") };
    let i: usize = i.parse().with_context(|| format!("line {line}: index in '{spec}'"))?;
    if i == 0 && form.eq_ignore_ascii_case("root") {
        return Ok(0);
    }
    if words.get(i.wrapping_sub(1)).is_none_or(|w| w != form) {
        bail!("line {line}: token {i} is not '{form}'");
    }
    Ok(i)
}

/// Reads the set: `construction <TAB> tokens <TAB> rel(head-i,dep-j) …`; `#` lines are comments.
pub fn load(path: &Path) -> Result<Vec<Item>> {
    let text = std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let mut out = Vec::new();
    for (k, l) in text.lines().enumerate() {
        let line = k + 1;
        if l.trim().is_empty() || l.starts_with('#') {
            continue;
        }
        let f: Vec<&str> = l.split('\t').collect();
        if f.len() != 3 {
            bail!("{}:{line}: three tab-separated fields expected", path.display());
        }
        let words: Vec<String> = f[1].split(' ').filter(|w| !w.is_empty()).map(str::to_string).collect();
        let mut arcs = Vec::new();
        for a in f[2].split(' ').filter(|a| !a.is_empty()) {
            let Some((rel, rest)) = a.split_once('(') else { bail!("line {line}: '{a}' is not rel(head-i,dep-j)") };
            let Some((h, d)) = rest.trim_end_matches(')').split_once(',') else { bail!("line {line}: '{a}' needs two nodes") };
            let r = Rel::parse(rel).with_context(|| format!("line {line}: unknown relation '{rel}'"))?;
            arcs.push((node(h, &words, line)?, node(d, &words, line)?, r));
        }
        if arcs.is_empty() {
            bail!("line {line}: no target arcs");
        }
        out.push(Item { cx: f[0].to_string(), words, arcs });
    }
    Ok(out)
}

/// Recall per construction: (right arcs, all arcs). An arc is right when the dependent has the target head and relation.
pub fn recall(items: &[Item], parse: &dyn Fn(&[&str]) -> Vec<(usize, Rel)>) -> BTreeMap<String, (usize, usize)> {
    let mut r: BTreeMap<String, (usize, usize)> = BTreeMap::new();
    for it in items {
        let words: Vec<&str> = it.words.iter().map(String::as_str).collect();
        let tree = parse(&words);
        let e = r.entry(it.cx.clone()).or_default();
        for &(h, d, rel) in &it.arcs {
            e.0 += (tree[d - 1] == (h, rel)) as usize;
            e.1 += 1;
        }
    }
    r
}

/// The missed target arcs, one line each: construction, sentence, target, what the parser gave.
pub fn misses(items: &[Item], parse: &dyn Fn(&[&str]) -> Vec<(usize, Rel)>) -> Vec<String> {
    let mut out = Vec::new();
    for it in items {
        let words: Vec<&str> = it.words.iter().map(String::as_str).collect();
        let tree = parse(&words);
        let at = |i: usize| if i == 0 { "ROOT".to_string() } else { format!("{}-{i}", words[i - 1]) };
        for &(h, d, rel) in &it.arcs {
            let (ph, pr) = tree[d - 1];
            if (ph, pr) != (h, rel) {
                out.push(format!("{}\t{}\t{rel}({},{})\tgot {pr}({},{})", it.cx, it.words.join(" "), at(h), at(d), at(ph), at(d)));
            }
        }
    }
    out
}

/// Floor file: `construction <TAB> recall %`.
pub fn read_floor(path: &Path) -> Result<BTreeMap<String, f64>> {
    let text = std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    text.lines()
        .filter(|l| !l.trim().is_empty() && !l.starts_with('#'))
        .map(|l| {
            let (c, v) = l.split_once('\t').with_context(|| format!("floor line '{l}'"))?;
            Ok((c.to_string(), v.trim().parse::<f64>().with_context(|| format!("floor value '{v}'"))?))
        })
        .collect()
}

pub fn pct((ok, n): (usize, usize)) -> f64 {
    (10000.0 * ok as f64 / n.max(1) as f64).round() / 100.0
}

/// The table, and the constructions below their floor (empty = the gate is green).
pub fn report(rec: &BTreeMap<String, (usize, usize)>, floor: Option<&BTreeMap<String, f64>>) -> (String, Vec<String>) {
    let mut out = String::from("| construction | arcs | right | recall | floor | gate |\n|---|---:|---:|---:|---:|---|\n");
    let mut red = Vec::new();
    let (mut ok, mut n) = (0usize, 0usize);
    for (c, &(a, b)) in rec {
        ok += a;
        n += b;
        let f = floor.and_then(|f| f.get(c)).copied();
        let below = f.is_some_and(|f| pct((a, b)) < f);
        if below {
            red.push(c.clone());
        }
        let fs = f.map_or("—".to_string(), |f| format!("{f:.2}"));
        writeln!(out, "| {c} | {b} | {a} | {:.2} | {fs} | {} |", pct((a, b)), if f.is_none() { "" } else if below { "RED" } else { "ok" }).ok();
    }
    writeln!(out, "| all | {n} | {ok} | {:.2} | | |", pct((ok, n))).ok();
    (out, red)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_shipped_set_reads() {
        let items = load(&Path::new(env!("CARGO_MANIFEST_DIR")).join("data/constructions.tsv")).expect("set");
        assert!(items.len() >= 100);
        let kinds: std::collections::BTreeSet<&str> = items.iter().map(|i| i.cx.as_str()).collect();
        assert!(kinds.len() >= 10);
    }

    #[test]
    fn a_wrong_index_is_an_error() {
        let words: Vec<String> = ["The", "dog", "barks"].iter().map(|s| s.to_string()).collect();
        assert_eq!(node("dog-2", &words, 1).unwrap(), 2);
        assert!(node("dog-3", &words, 1).is_err());
    }
}

#[cfg(test)]
mod wave27 {
    use super::*;
    #[test]
    fn bounded_contract() {
        let mut rec = BTreeMap::new();
        rec.insert("synthetic".into(), (1, 3));
        let mut floor = BTreeMap::new();
        floor.insert("synthetic".into(), 33.33);
        assert!(report(&rec, Some(&floor)).1.is_empty());
        floor.insert("synthetic".into(), 33.34);
        assert_eq!(report(&rec, Some(&floor)).1, vec!["synthetic"]);
        assert_eq!(pct((0, 0)), 0.0);
        let w = vec!["a-b".into()];
        assert_eq!(node("a-b-1", &w, 1).unwrap(), 1);
        assert!(node("a-b-0", &w, 1).is_err());
    }
}
