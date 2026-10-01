//! Selection of fairy-tale paragraphs from the database: prose only (`p`, not verse or heading), 2+ sentences, 40–250 words.
//! Books — proportionally to their size in sentences (largest remainders); within a book — half paragraphs with
//! dialogue (quotes) and half without, round-robin over tales in a seed-shuffled order, so different tales are taken.
//! Everything is deterministic (seed). Paragraph context — up to the last two sentences of the tale's previous paragraph.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::Path;

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

use prag::data::{duck_query, h64, has_quote};

const PARAS_SQL: &str = include_str!("../sql/paras.sql");

/// A paragraph sentence.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sent {
    /// `sent_id` in the database: `gutenberg:2591:the-golden-bird:3`
    pub sent_id: String,
    pub text: String,
}

/// A fairy-tale paragraph.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Para {
    /// document key: `gutenberg:2591:the-golden-bird`
    pub doc: String,
    /// book (Gutenberg collection): `gutenberg:2591`
    pub book: String,
    pub title: String,
    pub author: String,
    /// paragraph in the document (`sentences.para`, from 1)
    pub para: i64,
    /// block kind: p | lg | head
    pub block: String,
    pub sents: Vec<Sent>,
    /// whether there is direct speech (quotes)
    pub dialogue: bool,
    pub words: usize,
    /// up to the last two sentences of the previous paragraph — context for the LLM
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ctx: Vec<String>,
    /// LLM batch
    #[serde(default)]
    pub batch: usize,
}

impl Para {
    pub fn key(&self) -> String {
        format!("{}#{}", self.doc, self.para)
    }
}

/// Selection filters and size.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pick {
    pub paras: usize,
    pub batch: usize,
    pub seed: u64,
    pub min_words: usize,
    pub max_words: usize,
    pub min_sents: usize,
    pub max_sents: usize,
    /// run rule (`schema::Rule`); the pilot's `pick.json` has no such field — v1
    #[serde(default = "rule_v1")]
    pub rule: u8,
    /// `paras.jsonl` of an already finished run: its paragraphs are not taken
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude: Vec<String>,
}

fn rule_v1() -> u8 {
    1
}

impl Default for Pick {
    fn default() -> Pick {
        Pick { paras: 300, batch: 20, seed: 26, min_words: 40, max_words: 250, min_sents: 2, max_sents: 16, rule: 2, exclude: Vec::new() }
    }
}

impl Pick {
    pub fn rule(&self) -> anyhow::Result<crate::schema::Rule> {
        crate::schema::Rule::of(self.rule).with_context(|| format!("unknown rule {}", self.rule))
    }

    pub fn eligible(&self, p: &Para) -> bool {
        p.block == "p" && (self.min_sents..=self.max_sents).contains(&p.sents.len()) && (self.min_words..=self.max_words).contains(&p.words)
    }
}

/// All fairy-tale paragraphs from the database (`sql/paras.sql`), in text order. Every document's license must be
/// public domain, otherwise stop.
pub fn read_paras(db: &Path, tmp: &Path) -> Result<Vec<Para>> {
    let rows = duck_query(db, PARAS_SQL, tmp)?;
    let mut out: Vec<Para> = Vec::new();
    for r in rows {
        // columns: key title author license para block ord sent_id text
        if r.len() != 9 {
            bail!("paras.sql: {} columns instead of 9", r.len());
        }
        if !r[3].starts_with("public domain") {
            bail!("{}: license «{}» — not public domain", r[0], r[3]);
        }
        if r[4] == "\\N" {
            bail!("{}: sentence {} without a paragraph", r[0], r[7]);
        }
        let para: i64 = r[4].parse().with_context(|| format!("{}: paragraph «{}»", r[0], r[4]))?;
        let sent = Sent { sent_id: r[7].clone(), text: r[8].clone() };
        match out.last_mut() {
            Some(p) if p.doc == r[0] && p.para == para => p.sents.push(sent),
            _ => {
                let book = r[0].split(':').take(2).collect::<Vec<_>>().join(":");
                out.push(Para { doc: r[0].clone(), book, title: r[1].clone(), author: r[2].clone(), para, block: r[5].clone(), sents: vec![sent], dialogue: false, words: 0, ctx: Vec::new(), batch: 0 });
            }
        }
    }
    for i in 0..out.len() {
        let p = &out[i];
        let dialogue = p.sents.iter().any(|s| has_quote(&s.text));
        let words = p.sents.iter().map(|s| s.text.split_whitespace().count()).sum();
        let ctx = match i.checked_sub(1).map(|j| &out[j]) {
            Some(q) if q.doc == p.doc => q.sents.iter().rev().take(2).rev().map(|s| s.text.clone()).collect(),
            _ => Vec::new(),
        };
        let p = &mut out[i];
        (p.dialogue, p.words, p.ctx) = (dialogue, words, ctx);
    }
    Ok(out)
}

/// Paragraphs of a run dir (`paras.jsonl`).
pub fn load_paras(dir: &Path) -> Result<Vec<Para>> {
    let p = dir.join("paras.jsonl");
    let text = std::fs::read_to_string(&p).with_context(|| format!("{} — run select first", p.display()))?;
    text.lines().filter(|l| !l.trim().is_empty()).map(|l| Ok(serde_json::from_str(l)?)).collect()
}

/// Selection parameters of a run dir (`pick.json`).
pub fn load_pick(dir: &Path) -> Result<Pick> {
    let p = dir.join("pick.json");
    Ok(serde_json::from_str(&std::fs::read_to_string(&p).with_context(|| p.display().to_string())?)?)
}

/// Book quotas: `n` proportional to `size` (largest remainders), at most `cap` per book; a shortfall
/// passes to the other books by the same rule.
pub fn quotas(size: &BTreeMap<String, usize>, cap: &BTreeMap<String, usize>, n: usize) -> BTreeMap<String, usize> {
    let mut q: BTreeMap<String, usize> = size.keys().map(|b| (b.clone(), 0)).collect();
    let mut left = n.min(cap.values().sum());
    while left > 0 {
        let open: Vec<&String> = size.keys().filter(|b| q[*b] < cap.get(*b).copied().unwrap_or(0)).collect();
        let total: usize = open.iter().map(|b| size[*b]).sum();
        if open.is_empty() || total == 0 {
            break;
        }
        // shares and remainders
        let mut share: Vec<(usize, f64, &String)> = open.iter().map(|b| {
            let x = left as f64 * size[*b] as f64 / total as f64;
            (x.floor() as usize, x - x.floor(), *b)
        }).collect();
        let mut rest = left - share.iter().map(|s| s.0).sum::<usize>();
        share.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.2.cmp(b.2)));
        for s in share.iter_mut() {
            if rest == 0 {
                break;
            }
            s.0 += 1;
            rest -= 1;
        }
        let mut given = 0;
        for (k, _, b) in share {
            let room = cap[b] - q[b];
            let k = k.min(room);
            *q.get_mut(b).unwrap() += k;
            given += k;
        }
        if given == 0 {
            break;
        }
        left -= given;
    }
    q
}

/// Round-robin over tales: within each tale its paragraphs are seed-shuffled; take first the first paragraph of each
/// tale (tales also in shuffled order), then the second… — the first `k`.
fn round_robin(all: &[Para], pool: &[usize], k: usize, seed: u64) -> Vec<usize> {
    let mut by_doc: HashMap<&str, Vec<(u64, usize)>> = HashMap::new();
    for &i in pool {
        by_doc.entry(all[i].doc.as_str()).or_default().push((h64(&all[i].key(), seed), i));
    }
    let mut ranked: Vec<(usize, u64, usize)> = Vec::new();
    for (doc, mut v) in by_doc {
        v.sort();
        let hd = h64(doc, seed);
        ranked.extend(v.into_iter().enumerate().map(|(r, (_, i))| (r, hd, i)));
    }
    ranked.sort();
    ranked.into_iter().take(k).map(|x| x.2).collect()
}

/// Selection: indices into `all`, in text order. Paragraphs from `skip` (keys `Para::key`) are not taken.
pub fn select(all: &[Para], o: &Pick, skip: &HashSet<String>) -> Vec<usize> {
    let mut size: BTreeMap<String, usize> = BTreeMap::new();
    let mut cap: BTreeMap<String, usize> = BTreeMap::new();
    let mut dial: BTreeMap<&str, Vec<usize>> = BTreeMap::new();
    let mut narr: BTreeMap<&str, Vec<usize>> = BTreeMap::new();
    for (i, p) in all.iter().enumerate() {
        *size.entry(p.book.clone()).or_default() += p.sents.len();
        if o.eligible(p) && !skip.contains(&p.key()) {
            *cap.entry(p.book.clone()).or_default() += 1;
            let pools = if p.dialogue { &mut dial } else { &mut narr };
            pools.entry(p.book.as_str()).or_default().push(i);
        }
    }
    let q = quotas(&size, &cap, o.paras);
    let mut out = Vec::new();
    for (book, &k) in &q {
        let d_pool = dial.get(book.as_str()).map(Vec::as_slice).unwrap_or(&[]);
        let n_pool = narr.get(book.as_str()).map(Vec::as_slice).unwrap_or(&[]);
        // half and half; where one kind is short — fill up with the other
        let mut d = (k / 2).min(d_pool.len());
        let n = (k - d).min(n_pool.len());
        d = (k - n).min(d_pool.len());
        out.extend(round_robin(all, d_pool, d, o.seed));
        out.extend(round_robin(all, n_pool, n, o.seed ^ 0x5eed));
    }
    out.sort();
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn para(book: &str, doc: &str, para: i64, n: usize, words_each: usize, dialogue: bool) -> Para {
        let text = if dialogue { format!("‘Hi,’ {}", "w ".repeat(words_each.saturating_sub(1))) } else { "w ".repeat(words_each) };
        let sents = (0..n).map(|k| Sent { sent_id: format!("{doc}:{}", k + 1), text: text.trim().to_string() }).collect();
        Para { doc: doc.into(), book: book.into(), title: "T".into(), author: "A".into(), para, block: "p".into(), sents, dialogue, words: n * words_each, ctx: vec![], batch: 0 }
    }

    #[test]
    fn quotas_are_proportional_capped_and_exact() {
        let size: BTreeMap<String, usize> = [("a", 600), ("b", 300), ("c", 100)].iter().map(|(b, n)| (b.to_string(), *n)).collect();
        let big: BTreeMap<String, usize> = size.keys().map(|b| (b.clone(), 1000)).collect();
        let q = quotas(&size, &big, 10);
        assert_eq!((q["a"], q["b"], q["c"]), (6, 3, 1));
        // book cap: the shortfall c passes to the others, in total exactly n
        let cap: BTreeMap<String, usize> = [("a", 1000), ("b", 1000), ("c", 0)].iter().map(|(b, n)| (b.to_string(), *n)).collect();
        let q = quotas(&size, &cap, 10);
        assert_eq!((q["a"] + q["b"], q["c"]), (10, 0));
        // fewer paragraphs than requested — all of them
        let small: BTreeMap<String, usize> = size.keys().map(|b| (b.clone(), 2)).collect();
        assert_eq!(quotas(&size, &small, 10).values().sum::<usize>(), 6);
    }

    #[test]
    fn select_spreads_tales_and_mixes_dialogue() {
        let mut all = Vec::new();
        for d in 0..6 {
            for p in 1..=6 {
                all.push(para("g:1", &format!("g:1:t{d}"), p, 3, 20, p % 2 == 0));
            }
        }
        // negative controls of the filter: short, single sentence, verse
        all.push(para("g:1", "g:1:t9", 1, 3, 5, false));
        all.push(para("g:1", "g:1:t9", 2, 1, 60, false));
        let mut verse = para("g:1", "g:1:t9", 3, 3, 20, false);
        verse.block = "lg".into();
        all.push(verse);
        let o = Pick { paras: 12, ..Pick::default() };
        let got = select(&all, &o, &HashSet::new());
        assert_eq!(got.len(), 12);
        assert!(got.iter().all(|&i| all[i].doc != "g:1:t9"));
        assert_eq!(got.iter().filter(|&&i| all[i].dialogue).count(), 6);
        // round-robin: each of the six tales gives exactly two paragraphs
        let mut per: BTreeMap<&str, usize> = BTreeMap::new();
        for &i in &got {
            *per.entry(all[i].doc.as_str()).or_default() += 1;
        }
        assert!(per.len() == 6 && per.values().all(|&v| v == 2), "{per:?}");
        // deterministic
        assert_eq!(select(&all, &o, &HashSet::new()), got);
        // the rest: without the already selected — all other eligible ones, no repeats
        let skip: HashSet<String> = got.iter().map(|&i| all[i].key()).collect();
        let rest = select(&all, &Pick { paras: 1000, ..o.clone() }, &skip);
        assert_eq!(rest.len(), 36 - 12);
        assert!(rest.iter().all(|&i| !skip.contains(&all[i].key()) && all[i].doc != "g:1:t9"));
    }
}
