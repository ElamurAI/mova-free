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
        Pick {
            paras: 300,
            batch: 20,
            seed: 26,
            min_words: 40,
            max_words: 250,
            min_sents: 2,
            max_sents: 16,
            rule: 2,
            exclude: Vec::new(),
        }
    }
}

impl Pick {
    pub fn rule(&self) -> anyhow::Result<crate::schema::Rule> {
        crate::schema::Rule::of(self.rule).with_context(|| format!("unknown rule {}", self.rule))
    }

    pub fn eligible(&self, p: &Para) -> bool {
        p.block == "p"
            && (self.min_sents..=self.max_sents).contains(&p.sents.len())
            && (self.min_words..=self.max_words).contains(&p.words)
    }
}

/// All fairy-tale paragraphs from the database (`sql/paras.sql`), in text order. Every document's license must be
/// public domain, otherwise stop.
pub fn read_paras(db: &Path, tmp: &Path) -> Result<Vec<Para>> {
    paras_from_rows(duck_query(db, PARAS_SQL, tmp)?)
}

// Pure normalization seam: callers supply SQL-shaped rows in text order.
fn paras_from_rows(rows: Vec<Vec<String>>) -> Result<Vec<Para>> {
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
        let para: i64 = r[4]
            .parse()
            .with_context(|| format!("{}: paragraph «{}»", r[0], r[4]))?;
        let sent = Sent {
            sent_id: r[7].clone(),
            text: r[8].clone(),
        };
        match out.last_mut() {
            Some(p) if p.doc == r[0] && p.para == para => p.sents.push(sent),
            _ => {
                let book = r[0].split(':').take(2).collect::<Vec<_>>().join(":");
                out.push(Para {
                    doc: r[0].clone(),
                    book,
                    title: r[1].clone(),
                    author: r[2].clone(),
                    para,
                    block: r[5].clone(),
                    sents: vec![sent],
                    dialogue: false,
                    words: 0,
                    ctx: Vec::new(),
                    batch: 0,
                });
            }
        }
    }
    for i in 0..out.len() {
        let p = &out[i];
        let dialogue = p.sents.iter().any(|s| has_quote(&s.text));
        let words = p
            .sents
            .iter()
            .map(|s| s.text.split_whitespace().count())
            .sum();
        let ctx = match i.checked_sub(1).map(|j| &out[j]) {
            Some(q) if q.doc == p.doc => q
                .sents
                .iter()
                .rev()
                .take(2)
                .rev()
                .map(|s| s.text.clone())
                .collect(),
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
    let text = std::fs::read_to_string(&p)
        .with_context(|| format!("{} — run select first", p.display()))?;
    text.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| Ok(serde_json::from_str(l)?))
        .collect()
}

/// Selection parameters of a run dir (`pick.json`).
pub fn load_pick(dir: &Path) -> Result<Pick> {
    let p = dir.join("pick.json");
    Ok(serde_json::from_str(
        &std::fs::read_to_string(&p).with_context(|| p.display().to_string())?,
    )?)
}

/// Book quotas: `n` proportional to `size` (largest remainders), at most `cap` per book; a shortfall
/// passes to the other books by the same rule. Equal remainders use lexical book order.
/// Missing caps are zero; extra cap keys never receive seats. Zero-weight books receive
/// no seats, so their capacity may remain unused when positive weights are exhausted.
pub fn quotas(
    size: &BTreeMap<String, usize>,
    cap: &BTreeMap<String, usize>,
    n: usize,
) -> BTreeMap<String, usize> {
    let mut q: BTreeMap<String, usize> = size.keys().map(|b| (b.clone(), 0)).collect();
    // u128 holds sums and products of addressable 32/64-bit usize inputs.
    let mut left = (n as u128).min(cap.values().map(|&c| c as u128).sum()) as usize;
    while left > 0 {
        let open: Vec<&String> = size
            .keys()
            .filter(|b| q[*b] < cap.get(*b).copied().unwrap_or(0))
            .collect();
        let total: u128 = open.iter().map(|b| size[*b] as u128).sum();
        if open.is_empty() || total == 0 {
            break;
        }
        // shares and remainders
        let mut share: Vec<(usize, u128, &String)> = open
            .iter()
            .map(|b| {
                let numerator = left as u128 * size[*b] as u128;
                ((numerator / total) as usize, numerator % total, *b)
            })
            .collect();
        let mut rest = left - share.iter().map(|s| s.0).sum::<usize>();
        share.sort_by(|a, b| b.1.cmp(&a.1).then(a.2.cmp(b.2)));
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
        by_doc
            .entry(all[i].doc.as_str())
            .or_default()
            .push((h64(&all[i].key(), seed), i));
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
        let text = if dialogue {
            format!("‘Hi,’ {}", "w ".repeat(words_each.saturating_sub(1)))
        } else {
            "w ".repeat(words_each)
        };
        let sents = (0..n)
            .map(|k| Sent {
                sent_id: format!("{doc}:{}", k + 1),
                text: text.trim().to_string(),
            })
            .collect();
        Para {
            doc: doc.into(),
            book: book.into(),
            title: "T".into(),
            author: "A".into(),
            para,
            block: "p".into(),
            sents,
            dialogue,
            words: n * words_each,
            ctx: vec![],
            batch: 0,
        }
    }

    #[test]
    fn quotas_are_proportional_capped_and_exact() {
        let size: BTreeMap<String, usize> = [("a", 600), ("b", 300), ("c", 100)]
            .iter()
            .map(|(b, n)| (b.to_string(), *n))
            .collect();
        let big: BTreeMap<String, usize> = size.keys().map(|b| (b.clone(), 1000)).collect();
        let q = quotas(&size, &big, 10);
        assert_eq!((q["a"], q["b"], q["c"]), (6, 3, 1));
        // book cap: the shortfall c passes to the others, in total exactly n
        let cap: BTreeMap<String, usize> = [("a", 1000), ("b", 1000), ("c", 0)]
            .iter()
            .map(|(b, n)| (b.to_string(), *n))
            .collect();
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
        let o = Pick {
            paras: 12,
            ..Pick::default()
        };
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
        let rest = select(
            &all,
            &Pick {
                paras: 1000,
                ..o.clone()
            },
            &skip,
        );
        assert_eq!(rest.len(), 36 - 12);
        assert!(
            rest.iter()
                .all(|&i| !skip.contains(&all[i].key()) && all[i].doc != "g:1:t9")
        );
    }
}

#[cfg(test)]
mod wave17_models {
    use super::*;
    fn row(doc: &str, p: &str, ord: &str, text: &str) -> Vec<String> {
        [
            doc,
            "Title",
            "Author",
            "public domain",
            p,
            "p",
            ord,
            ord,
            text,
        ]
        .into_iter()
        .map(str::to_string)
        .collect()
    }
    #[test]
    fn normalization_groups_context_and_rejects_bad_rows() {
        let rows = vec![
            row("g:1:d", "1", "1", "One two"),
            row("g:1:d", "1", "2", "Three"),
            row("g:1:d", "1", "3", "Four five"),
            row("g:1:d", "2", "4", "‘Hello!’"),
            row("g:1:e", "1", "1", "End"),
        ];
        let got = paras_from_rows(rows.clone()).unwrap();
        assert_eq!(got.len(), 3);
        assert_eq!(got[0].words, 5);
        assert_eq!(got[0].book, "g:1");
        assert!(got[0].ctx.is_empty());
        assert_eq!(got[1].ctx, vec!["Three", "Four five"]);
        assert!(got[1].dialogue);
        assert!(got[2].ctx.is_empty());
        let mut other = rows;
        other[0][6] = "arbitrary".into();
        assert_eq!(
            serde_json::to_value(paras_from_rows(other).unwrap()).unwrap(),
            serde_json::to_value(got).unwrap()
        );
        assert!(paras_from_rows(vec![vec![]]).is_err());
        for bad in ["\\N", "invalid"] {
            assert!(paras_from_rows(vec![row("d", bad, "1", "x")]).is_err());
        }
        let mut bad = row("d", "1", "1", "x");
        bad[3] = "restricted".into();
        assert!(paras_from_rows(vec![bad]).is_err());
    }
    fn map(v: &[usize]) -> BTreeMap<String, usize> {
        v.iter()
            .enumerate()
            .map(|(i, &n)| (format!("b{i}"), n))
            .collect()
    }
    // Small independent oracle uses repeated subtraction and maximum scans.
    fn oracle(w: &[usize], caps: &[usize], n: usize) -> Vec<usize> {
        let mut q = vec![0; w.len()];
        let mut left = n.min(caps.iter().sum());
        while left > 0 {
            let total: usize = (0..w.len()).filter(|&i| q[i] < caps[i]).map(|i| w[i]).sum();
            if total == 0 {
                break;
            }
            let mut give = vec![0; w.len()];
            let mut rem = vec![None; w.len()];
            for i in 0..w.len() {
                if q[i] < caps[i] {
                    let mut numerator = left * w[i];
                    while numerator >= total {
                        give[i] += 1;
                        numerator -= total
                    }
                    rem[i] = Some(numerator);
                }
            }
            let mut extra = left - give.iter().sum::<usize>();
            while extra > 0 {
                let winner = (0..w.len())
                    .filter(|&i| rem[i].is_some())
                    .max_by(|&a, &b| rem[a].cmp(&rem[b]).then(b.cmp(&a)))
                    .unwrap();
                give[winner] += 1;
                rem[winner] = None;
                extra -= 1;
            }
            let mut used = 0;
            for i in 0..w.len() {
                let k = give[i].min(caps[i] - q[i]);
                q[i] += k;
                used += k;
            }
            if used == 0 {
                break;
            }
            left -= used;
        }
        q
    }
    #[test]
    fn quota_small_exhaustive_rational_oracle() {
        for state in 0..4096usize {
            let mut x = state;
            let mut w = [0; 3];
            let mut c = [0; 3];
            for a in w.iter_mut().chain(c.iter_mut()) {
                *a = x % 4;
                x /= 4;
            }
            for n in 0..=10 {
                let got = quotas(&map(&w), &map(&c), n);
                assert_eq!(
                    got.values().copied().collect::<Vec<_>>(),
                    oracle(&w, &c, n),
                    "w={w:?} c={c:?} n={n}"
                );
            }
        }
    }
    #[test]
    fn quota_exact_above_f64_integer_precision() {
        if usize::BITS < 64 {
            return;
        }
        let a = (1u64 << 53) as usize;
        assert_eq!(quotas(&map(&[a, a + 1]), &map(&[2, 2]), 1), map(&[0, 1]));
    }
    #[test]
    fn quota_capacity_and_weight_sums_exceed_usize() {
        let m = usize::MAX;
        assert_eq!(
            quotas(&map(&[m, m]), &map(&[m, m]), m),
            map(&[m / 2 + 1, m / 2])
        );
    }
    #[test]
    fn quota_missing_extra_zero_and_capped_domains() {
        let mut c = map(&[2, 2, 9]);
        c.insert("unused".into(), usize::MAX);
        assert_eq!(quotas(&map(&[0, 3, 0]), &c, 10), map(&[0, 2, 0]));
        assert_eq!(
            quotas(&map(&[1, 2]), &BTreeMap::new(), usize::MAX),
            map(&[0, 0])
        );
        assert_eq!(
            quotas(&map(&[9, 2, 1]), &map(&[1, 5, 5]), 8),
            map(&[1, 4, 3])
        );
        assert!(quotas(&BTreeMap::new(), &c, usize::MAX).is_empty());
    }
    fn para(doc: &str, i: i64, dialogue: bool) -> Para {
        Para {
            doc: doc.into(),
            book: "b".into(),
            title: "T".into(),
            author: "A".into(),
            para: i,
            block: "p".into(),
            sents: vec![
                Sent {
                    sent_id: format!("{doc}:{i}"),
                    text: "x".into()
                };
                2
            ],
            dialogue,
            words: 40,
            ctx: vec![],
            batch: 0,
        }
    }
    #[test]
    fn selection_order_filter_skip_balance_and_rounds() {
        let all: Vec<_> = (0..4)
            .flat_map(|d| (0..4).map(move |i| para(&format!("d{d}"), i, i % 2 == 0)))
            .collect();
        let pick = Pick {
            paras: 8,
            ..Pick::default()
        };
        let skip = HashSet::new();
        let got = select(&all, &pick, &skip);
        assert_eq!(got.len(), 8);
        assert!(got.windows(2).all(|x| x[0] < x[1]));
        assert_eq!(got.iter().filter(|&&i| all[i].dialogue).count(), 4);
        for d in 0..4 {
            assert_eq!(
                got.iter()
                    .filter(|&&i| all[i].doc == format!("d{d}"))
                    .count(),
                2
            );
        }
        for _ in 0..8 {
            assert_eq!(select(&all, &pick, &skip), got);
        }
        let skip = got.iter().map(|&i| all[i].key()).collect();
        let remaining = select(
            &all,
            &Pick {
                paras: usize::MAX,
                ..pick.clone()
            },
            &skip,
        );
        assert_eq!(remaining.len(), 8);
        assert!(remaining.iter().all(|i| !got.contains(i)));
        assert!(
            select(
                &all,
                &Pick {
                    min_words: 41,
                    ..pick
                },
                &HashSet::new()
            )
            .is_empty()
        );
        let mut other = all.clone();
        other[0].title = "Other".into();
        assert_eq!(
            select(
                &other,
                &Pick {
                    paras: 8,
                    ..Pick::default()
                },
                &HashSet::new()
            ),
            got
        );
    }
}
