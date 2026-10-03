//! Big samples of the tree store for learning on much larger texts than the gold treebanks. A sample is a set of
//! sentences from the store's books (by group), spread over all of them by a hash of the sentence number, parsed
//! afresh (annotator + hand repairs, no induced rules — the rules under test are applied on top). There is no gold:
//! the judge is the absurdity matrix (`induce::explore_big`).
//!
//! Samples: `big-tale` (tales and children's books, the tale domain) and `big-real` (fiction and legal texts, the
//! general domain); bucket 0 is for exploring, bucket 1 (disjoint) for the guard. The books of the tales test
//! (`tales-pg-*`, the source of the tale treebank) are never sampled.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

use anyhow::{Context, Result};

/// How many sentences one sample holds (`MOVA_BIG_N`, default 300000; about 1.3 GB parsed).
pub fn sample_size() -> usize {
    std::env::var("MOVA_BIG_N").ok().and_then(|x| x.parse().ok()).unwrap_or(300_000)
}

fn store_dir() -> PathBuf {
    std::env::var("MOVA_STORE").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| "data".into())).join("store/pd-en"))
}

/// The book groups of a sample's domain.
fn groups(domain: &str) -> &'static [&'static str] {
    if domain == "tale" { &["tales", "tales-more", "children"] } else { &["fiction", "legal"] }
}

/// Sentence ids of a sample: books of the domain's groups, hash bucket `bucket` of 16, then an even stride to `n`.
pub fn ids(domain: &str, bucket: u64, n: usize) -> Result<Vec<usize>> {
    let dir = store_dir();
    let st = crate::store::Store::open(&dir)?;
    let books = std::fs::read_to_string(dir.join("books.tsv")).context("books.tsv")?;
    let want = groups(domain);
    // the tales test comes from the tales-pg-* books: they are not in any group taken here, and checked by name
    let ok: BTreeMap<u32, bool> = books
        .lines()
        .skip(1)
        .filter_map(|l| {
            let c: Vec<&str> = l.split('\t').collect();
            Some((c.first()?.parse().ok()?, want.contains(c.get(3)?) && !c.get(3)?.starts_with("tales-pg")))
        })
        .collect();
    let mut cand = Vec::new();
    for i in 0..st.len() {
        let (b, _, _) = st.place(i);
        if !ok.get(&b).copied().unwrap_or(false) {
            continue;
        }
        let h = (i as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) >> 60;
        if h == bucket {
            cand.push(i);
        }
    }
    if cand.len() <= n {
        return Ok(cand);
    }
    let step = cand.len() as f64 / n as f64;
    Ok((0..n).map(|k| cand[(k as f64 * step) as usize]).collect())
}

/// Parse the sample (threads), keeping sentences of 5–40 tokens.
fn parse(domain: &str, bucket: u64, n: usize) -> Result<Vec<Vec<en::annotate::Word>>> {
    let ids = ids(domain, bucket, n)?;
    let st = crate::store::Store::open(&store_dir())?;
    let a = crate::tree::annotator()?;
    let m = crate::absurd::Matrix::global();
    let threads = std::thread::available_parallelism().map(|x| x.get()).unwrap_or(4).min(10);
    let next = std::sync::atomic::AtomicUsize::new(0);
    let out: Mutex<Vec<(usize, Vec<en::annotate::Word>)>> = Mutex::new(Vec::new());
    std::thread::scope(|sc| {
        for _ in 0..threads {
            sc.spawn(|| loop {
                let k = next.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                let Some(&i) = ids.get(k) else { break };
                let Ok(text) = st.text(i) else { continue };
                let forms: Vec<String> = a.tokenize(&text).into_iter().map(|t| t.form).collect();
                if !(5..=40).contains(&forms.len()) {
                    continue;
                }
                let mut ws = a.annotate(&forms);
                crate::rerank::hand_pass_without(&m, &mut ws, true, &[]);
                out.lock().unwrap().push((k, ws));
            });
        }
    });
    let mut v = out.into_inner().unwrap();
    v.sort_by_key(|x| x.0);
    Ok(v.into_iter().map(|x| x.1).collect())
}

/// A sample, parsed once and kept while in use: `big-tale` / `big-real` (bucket 0), `big-tale-guard` /
/// `big-real-guard` (bucket 1), `big-tale@<b>` / `big-real@<b>` (bucket b, 2..15 — fresh text for each pass of
/// exploring). At most four samples stay in memory; the oldest exploring one is dropped first, guards stay.
pub fn sample(name: &str) -> Result<std::sync::Arc<Vec<Vec<en::annotate::Word>>>> {
    type Cache = Mutex<Vec<(String, std::sync::Arc<Vec<Vec<en::annotate::Word>>>)>>;
    static CACHE: OnceLock<Cache> = OnceLock::new();
    let c = CACHE.get_or_init(|| Mutex::new(Vec::new()));
    if let Some(v) = c.lock().unwrap().iter().find(|x| x.0 == name).map(|x| x.1.clone()) {
        return Ok(v);
    }
    let domain = domain_of(name);
    let bucket = if name.ends_with("-guard") { 1 } else { name.split_once('@').and_then(|x| x.1.parse().ok()).unwrap_or(0) };
    let t = std::time::Instant::now();
    let v = std::sync::Arc::new(parse(domain, bucket, sample_size())?);
    eprintln!("big sample {name}: {} sentences parsed in {:.0} s", v.len(), t.elapsed().as_secs_f64());
    let mut g = c.lock().unwrap();
    while g.len() >= 4 {
        let i = g.iter().position(|x| !x.0.ends_with("-guard")).unwrap_or(0);
        g.remove(i);
    }
    g.push((name.to_string(), v.clone()));
    Ok(v)
}

/// The domain of a big sample's name.
pub fn domain_of(name: &str) -> &'static str {
    if name.starts_with("big-tale") { "tale" } else { "real" }
}

/// The teacher's check of a big idea: a sample of the words it relabels, each shown with its head and the two labels
/// (old and new, in alternating order so the position tells nothing), judged by Opus (subscription, medium) as A, B
/// or N (neither). Returns (new right, old right, neither); every call is accounted in
/// `data/runs/family-judge/calls.jsonl`. A failed call is an error (fail-fast, no retry).
pub fn judge_changes(items: &[(Vec<String>, usize, usize, en::gram::Rel, en::gram::Rel)]) -> Result<(usize, usize, usize)> {
    let mut prompt = String::from("You check dependency labels of Universal Dependencies (English, UD 2.x). For each item you get a sentence with one word marked [[like this]], its head word, and two labels A and B for the relation of the marked word to that head. Say which label is correct: A, B, or N if neither fits. Answer only with lines «<number> <A|B|N>», one per item, nothing else.\n\n");
    for (k, (forms, i, head, old, new)) in items.iter().enumerate() {
        let text: Vec<String> = forms.iter().enumerate().map(|(j, f)| if j == *i { format!("[[{f}]]") } else { f.clone() }).collect();
        let h = if *head > 0 { forms[head - 1].clone() } else { "ROOT".into() };
        let (a, b) = if k % 2 == 0 { (old, new) } else { (new, old) };
        prompt += &format!("{}. {}\n   head: {h}; A = {a}; B = {b}\n", k + 1, text.join(" "));
    }
    let dir = PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| "data".into())).join("runs/family-judge");
    let o = prag::opus::Opus::from_env(dir.join("cwd"));
    let (ans, call) = o.ask("family-judge", 0, items.len(), &prompt)?;
    prag::opus::append_call(&dir.join("calls.jsonl"), &call)?;
    let (mut new_right, mut old_right, mut neither) = (0, 0, 0);
    for l in ans.lines() {
        let mut w = l.split_whitespace();
        let (Some(n), Some(v)) = (w.next().and_then(|x| x.trim_end_matches('.').parse::<usize>().ok()), w.next()) else { continue };
        if n == 0 || n > items.len() {
            continue;
        }
        let old_is_a = (n - 1) % 2 == 0;
        match (v, old_is_a) {
            ("A", true) | ("B", false) => old_right += 1,
            ("A", false) | ("B", true) => new_right += 1,
            _ => neither += 1,
        }
    }
    Ok((new_right, old_right, neither))
}
