//! Pragmatics data: open texts with a commercial license — tales from the database (public domain) and
//! Tatoeba CC0. Selection of the first silver wave: whole tale paragraphs with dialogue and narration from different tales, and
//! single Tatoeba sentences; batches of 40–60 sentences for the LLM. Everything is deterministic (seeded).

use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

/// Where a sentence comes from.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Src {
    /// Project Gutenberg tales (public domain) — training
    Tale,
    /// Tatoeba CC0 — training
    Tatoeba,
    /// GUM (CC BY-NC-SA) — only for evaluating `form`, not used for training
    Gum,
}

impl Src {
    pub fn name(self) -> &'static str {
        match self {
            Src::Tale => "tale",
            Src::Tatoeba => "tatoeba",
            Src::Gum => "gum",
        }
    }

    /// Source in `en/data/train-licenses.tsv` for the commercial license gate.
    pub fn license_source(self) -> &'static str {
        match self {
            Src::Tale => "gutenberg-tales",
            Src::Tatoeba => "tatoeba-eng-cc0",
            Src::Gum => "gum",
        }
    }
}

/// Commercial license gate (`en::license::commercial_gate`): the pragmatics model trains only on
/// sources that allow commercial use. GUM (NC) — stop.
pub fn license_gate(srcs: &[Src]) -> Result<()> {
    let set: std::collections::BTreeSet<&str> = srcs.iter().map(|s| s.license_source()).collect();
    let paths: Vec<PathBuf> = set.iter().map(PathBuf::from).collect();
    en::license::commercial_gate(&paths)
}

/// Sentence for annotation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    /// `sent_id` in the database: `gutenberg:2591:hans-in-luck:5`, `tatoeba:9977229`
    pub key: String,
    pub src: Src,
    /// document key: `gutenberg:2591:hans-in-luck`, `tatoeba`
    pub doc: String,
    pub title: String,
    /// paragraph in the document (tales), 0 — outside paragraphs
    pub para: i64,
    /// sentence order in the document (from 1; Tatoeba — 0): adjacency for the previous sentence's act
    #[serde(default)]
    pub ord: i64,
    /// position in paragraph (from 0) and paragraph length in sentences
    pub pos: usize,
    pub plen: usize,
    pub text: String,
    /// LLM batch
    pub batch: usize,
    /// 1–2 sentences before the window (only on the window's first sentence) — context for the LLM
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ctx: Vec<String>,
}

/// Deterministic seeded string hash (FNV-1a + splitmix) — for shuffling without rand.
pub fn h64(s: &str, seed: u64) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325 ^ seed.wrapping_mul(0x9e37_79b9_7f4a_7c15);
    for b in s.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x0100_0000_01b3);
    }
    h ^= h >> 30;
    h = h.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    h ^= h >> 27;
    h = h.wrapping_mul(0x94d0_49bb_1331_11eb);
    h ^ (h >> 31)
}

// ── Database (DuckDB CLI, read-only) ───────────────────────────────────────────────────────────────

/// Run a SQL query file with `{out}` → TSV and read the rows (TAB-separated columns, NULL — `\N`).
pub fn duck_query(db: &Path, sql_template: &str, tmp: &Path) -> Result<Vec<Vec<String>>> {
    let bin = std::env::var("DUCKDB").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| "data".into())).join("bin/duckdb"));
    std::fs::create_dir_all(tmp)?;
    let out = tmp.join(format!("q-{}.tsv", std::process::id()));
    let o = out.to_string_lossy();
    if o.contains('\'') {
        bail!("path contains a quote: {o}");
    }
    let sql = sql_template.replace("{out}", &o);
    let mut child = Command::new(&bin)
        .args(["-bail", "-batch", "-readonly"])
        .arg(db)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("cannot start {} (DUCKDB)", bin.display()))?;
    std::io::Write::write_all(&mut child.stdin.take().context("stdin")?, sql.as_bytes())?;
    let r = child.wait_with_output()?;
    let err = String::from_utf8_lossy(&r.stderr);
    if !r.status.success() || err.contains("Error:") {
        bail!("duckdb {}: {}", db.display(), err.trim());
    }
    let text = std::fs::read_to_string(&out).with_context(|| out.display().to_string())?;
    let _ = std::fs::remove_file(&out);
    Ok(text.lines().map(|l| l.split('\t').map(str::to_string).collect()).collect())
}

/// Tale sentence from the database.
#[derive(Clone, Debug)]
pub struct TaleSent {
    pub doc: String,
    pub title: String,
    pub para: i64,
    pub ord: i64,
    pub sent_id: String,
    pub text: String,
}

/// All tale sentences (`sql/tales.sql`), by document in text order. Each document's license must be
/// public domain, otherwise stop.
pub fn read_tales(db: &Path, tmp: &Path) -> Result<Vec<TaleSent>> {
    let rows = duck_query(db, include_str!("../sql/tales.sql"), tmp)?;
    let mut out = Vec::with_capacity(rows.len());
    for r in rows {
        // columns: key source title license para ord sent_id text
        if r.len() != 8 {
            bail!("tales.sql: {} columns instead of 8", r.len());
        }
        if !r[3].starts_with("public domain") {
            bail!("{}: license '{}' is not public domain", r[0], r[3]);
        }
        // no paragraph — the sentence is its own paragraph (negative number, so it does not merge with real paragraphs)
        let para = if r[4] == "\\N" { -r[5].parse::<i64>()? } else { r[4].parse()? };
        out.push(TaleSent { doc: r[0].clone(), title: r[2].clone(), para, ord: r[5].parse()?, sent_id: r[6].clone(), text: r[7].clone() });
    }
    Ok(out)
}

/// Whether the text has direct speech: opening quotes (“, ", ‘ or ' at the start of a word).
pub fn has_quote(text: &str) -> bool {
    let cs: Vec<char> = text.chars().collect();
    cs.iter().enumerate().any(|(i, &c)| {
        c == '“' || c == '"' || (matches!(c, '‘' | '\'') && (i == 0 || !cs[i - 1].is_alphanumeric()) && cs.get(i + 1).is_some_and(|x| x.is_alphabetic()))
    })
}

/// Tale selection by windows: a window is consecutive whole paragraphs of one tale, `win.0..=win.1` sentences (a short tale —
/// whole), a paragraph — up to `max_para` sentences; a window naturally has both dialogue and narration. Books (Gutenberg
/// collections) — proportionally to their size, so short fables do not crowd out tales. Tales — in
/// seed-shuffled order, the first pass takes one window per tale, then the next ones. In total — up to `n`
/// sentences. Output — windows (paragraphs → sentence indices in `all`) in `all` order.
pub fn select_tales(all: &[TaleSent], n: usize, seed: u64, win: (usize, usize), max_para: usize) -> Vec<Vec<Vec<usize>>> {
    let book = |d: &str| d.split(':').take(2).collect::<Vec<_>>().join(":");
    // document → consecutive paragraphs; book → sentence count
    let mut docs: BTreeMap<&str, Vec<Vec<usize>>> = BTreeMap::new();
    let mut size: BTreeMap<String, usize> = BTreeMap::new();
    for (i, s) in all.iter().enumerate() {
        *size.entry(book(&s.doc)).or_default() += 1;
        let ps = docs.entry(s.doc.as_str()).or_default();
        match ps.last_mut() {
            Some(p) if all[p[0]].para == s.para => p.push(i),
            _ => ps.push(vec![i]),
        }
    }
    let quota: BTreeMap<String, usize> = size.iter().map(|(b, &k)| (b.clone(), (n as f64 * k as f64 / all.len().max(1) as f64).round() as usize)).collect();
    let mut taken: BTreeMap<String, usize> = BTreeMap::new();
    let mut order: Vec<(u64, &str)> = docs.keys().map(|d| (h64(d, seed), *d)).collect();
    order.sort();
    let mut total = 0usize;
    let mut used: HashSet<usize> = HashSet::new();
    let mut out: Vec<Vec<Vec<usize>>> = Vec::new();
    'passes: for pass in 0..4u64 {
        for &(_, d) in &order {
            if total >= n {
                break 'passes;
            }
            let b = book(d);
            let room = quota[&b].saturating_sub(taken.get(&b).copied().unwrap_or(0)).min(n - total);
            let ps = &docs[d];
            let ok = |k: usize, used: &HashSet<usize>| ps[k].len() <= max_para && !ps[k].iter().any(|i| used.contains(i));
            let start = (h64(&format!("{d}#{pass}"), seed) % ps.len() as u64) as usize;
            let Some(s0) = (start..ps.len()).chain(0..start).find(|&k| ok(k, &used)) else { continue };
            let (mut lo, mut hi, mut len) = (s0, s0 + 1, ps[s0].len());
            while len < win.0 && hi < ps.len() && ok(hi, &used) && len + ps[hi].len() <= win.1 {
                len += ps[hi].len();
                hi += 1;
            }
            while len < win.0 && lo > 0 && ok(lo - 1, &used) && len + ps[lo - 1].len() <= win.1 {
                lo -= 1;
                len += ps[lo].len();
            }
            let mut w: Vec<Vec<usize>> = ps[lo..hi].to_vec();
            // no more than the book quota and n: drop extra paragraphs from the end of the window
            while !w.is_empty() && w.iter().map(Vec::len).sum::<usize>() > room {
                w.pop();
            }
            let len: usize = w.iter().map(Vec::len).sum();
            if len == 0 {
                continue;
            }
            total += len;
            *taken.entry(b).or_default() += len;
            used.extend(w.iter().flatten().copied());
            out.push(w);
        }
    }
    out.sort_by_key(|w| w[0][0]);
    out
}

/// Tatoeba sentences (`id \t text`): a single sentence (no internal boundary), 1–25 words, distinct texts;
/// seed-shuffled, first `n`.
pub fn select_tatoeba(path: &Path, n: usize, seed: u64) -> Result<Vec<(String, String)>> {
    let text = std::fs::read_to_string(path).with_context(|| path.display().to_string())?;
    let mut cand: Vec<(u64, String, String)> = Vec::new();
    for l in text.lines() {
        let Some((id, t)) = l.split_once('\t') else { continue };
        let t = t.trim();
        let words = t.split_whitespace().count();
        if !(1..=25).contains(&words) || multi_sentence(t) || t.contains('\t') {
            continue;
        }
        cand.push((h64(id, seed), id.to_string(), t.to_string()));
    }
    cand.sort();
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for (_, id, t) in cand {
        if seen.insert(t.to_lowercase()) {
            out.push((id, t));
            if out.len() == n {
                break;
            }
        }
    }
    Ok(out)
}

/// Several sentences in one record: `. ! ?` (possibly followed by a quote or bracket), a space and a capital letter or
/// an opening quote.
pub fn multi_sentence(t: &str) -> bool {
    let cs: Vec<char> = t.chars().collect();
    for i in 0..cs.len() {
        if matches!(cs[i], '.' | '!' | '?') {
            let mut j = i + 1;
            while j < cs.len() && matches!(cs[j], '"' | '”' | '’' | ')' | '\'') {
                j += 1;
            }
            if j < cs.len() && cs[j] == ' ' && j + 1 < cs.len() && (cs[j + 1].is_uppercase() || matches!(cs[j + 1], '"' | '“' | '‘')) {
                // "Mr. Smith", "Dr. Who" — abbreviations, not a boundary
                let word: String = cs[..i].iter().rev().take_while(|c| c.is_alphabetic()).collect::<Vec<_>>().into_iter().rev().collect();
                if !matches!(word.as_str(), "Mr" | "Mrs" | "Ms" | "Dr" | "St" | "Jr" | "Sr" | "Prof" | "Mt" | "vs") {
                    return true;
                }
            }
        }
    }
    false
}

/// Batches: tale windows stay whole, a batch closes when the next window does not fit in `max`; Tatoeba — in groups of
/// `tat_batch`. Context (1–2 sentences before the window) — on the window's first sentence. Batch number — in `Item::batch`.
pub fn make_items(all: &[TaleSent], wins: &[Vec<Vec<usize>>], tat: &[(String, String)], max: usize, tat_batch: usize) -> Vec<Item> {
    let mut items = Vec::new();
    let (mut batch, mut cur) = (0usize, 0usize);
    for w in wins {
        let wl: usize = w.iter().map(Vec::len).sum();
        if cur > 0 && cur + wl > max {
            batch += 1;
            cur = 0;
        }
        let first = w[0][0];
        let ctx: Vec<String> = (first.saturating_sub(2)..first).filter(|&i| all[i].doc == all[first].doc).map(|i| all[i].text.clone()).collect();
        for p in w {
            for (k, &i) in p.iter().enumerate() {
                let s = &all[i];
                items.push(Item {
                    key: s.sent_id.clone(),
                    src: Src::Tale,
                    doc: s.doc.clone(),
                    title: s.title.clone(),
                    para: s.para,
                    ord: s.ord,
                    pos: k,
                    plen: p.len(),
                    text: s.text.clone(),
                    batch,
                    ctx: if i == first { ctx.clone() } else { Vec::new() },
                });
            }
        }
        cur += wl;
    }
    if cur > 0 {
        batch += 1;
    }
    for (k, (id, t)) in tat.iter().enumerate() {
        items.push(Item {
            key: format!("tatoeba:{id}"),
            src: Src::Tatoeba,
            doc: "tatoeba".into(),
            title: String::new(),
            para: 0,
            ord: 0,
            pos: 0,
            plen: 1,
            text: t.clone(),
            batch: batch + k / tat_batch.max(1),
            ctx: Vec::new(),
        });
    }
    items
}

pub fn write_items(path: &Path, items: &[Item]) -> Result<()> {
    let mut s = String::new();
    for it in items {
        s.push_str(&serde_json::to_string(it)?);
        s.push('\n');
    }
    std::fs::write(path, s).with_context(|| path.display().to_string())
}

pub fn read_items(path: &Path) -> Result<Vec<Item>> {
    std::fs::read_to_string(path)
        .with_context(|| path.display().to_string())?
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| Ok(serde_json::from_str(l)?))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quotes_and_sentence_bounds() {
        assert!(has_quote("At last he said, ‘Master, my time is up.’"));
        assert!(has_quote("“Come here,” said the fox."));
        assert!(!has_quote("The dogs’ bones and don’t were apostrophes."));
        assert!(multi_sentence("I came. He left."));
        assert!(multi_sentence("\"Hi!\" He waved."));
        assert!(!multi_sentence("Mr. Smith is here."));
        assert!(!multi_sentence("Where are you going?"));
    }

    #[test]
    fn license_gate_blocks_gum() {
        assert!(license_gate(&[Src::Tale, Src::Tatoeba]).is_ok());
        // negative control: GUM (NC) in training — stop
        assert!(license_gate(&[Src::Tale, Src::Gum]).is_err());
    }

    #[test]
    fn selection_is_whole_paragraphs_in_windows() {
        let mut all = Vec::new();
        for d in 0..6 {
            let mut ord = 0;
            for p in 1..=5 {
                for s in 0..(p % 3 + 1) {
                    ord += 1;
                    let text = if (d + p) % 2 == 0 { format!("“Hi {s},” said {d}.") } else { format!("Story {d} {p} {s}.") };
                    all.push(TaleSent { doc: format!("d{d}"), title: String::new(), para: p as i64, ord, sent_id: format!("d{d}:{p}:{s}"), text });
                }
            }
        }
        let wins = select_tales(&all, 20, 7, (4, 6), 14);
        let total: usize = wins.iter().flatten().map(Vec::len).sum();
        assert!(total <= 20 && total >= 15, "{total}");
        for w in &wins {
            assert!(w.iter().map(Vec::len).sum::<usize>() <= 6);
            for p in w {
                // whole paragraph: all sentences of one paragraph
                let (d, a) = (&all[p[0]].doc, all[p[0]].para);
                assert_eq!(p.len(), all.iter().filter(|s| &s.doc == d && s.para == a).count());
            }
            // window paragraphs are consecutive
            let idx: Vec<usize> = w.iter().flatten().copied().collect();
            assert!(idx.windows(2).all(|x| x[1] == x[0] + 1));
        }
        let items = make_items(&all, &wins, &[("1".into(), "Hello!".into())], 7, 50);
        // a batch does not split windows and is no larger than max
        let mut by: BTreeMap<usize, usize> = BTreeMap::new();
        for it in &items {
            *by.entry(it.batch).or_default() += 1;
        }
        assert!(items.iter().filter(|x| x.src == Src::Tale).all(|x| by[&x.batch] <= 7));
        assert_eq!(items.last().unwrap().key, "tatoeba:1");
    }
}
