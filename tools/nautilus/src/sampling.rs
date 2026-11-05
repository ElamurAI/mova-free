//! The Wikipedia layer by sampling (source `wikipedia-examples`): which words Nautilus asks about, in what order, and
//! which of a word's sampled examples a sense question shows. Natively, no teacher.
//!
//! - **Words** come from the large corpus's tier-1 word frequencies (`<corpus>/tier1/words.tsv`, written by the
//!   private `wikicorpus words`: `lemma|UPOS`, occurrences, shards). A word is a lower-case content lemma (NOUN,
//!   VERB, ADJ, ADV; `layer::admissible`, not `senses::junk`) with at least `min_count` occurrences over its content
//!   UPOS; words of the `skip` layers (earlier layers already worked) are left out.
//! - **Order** (`order`): with `f` the word's tier-1 occurrences over its content UPOS and `s` the number of numbered
//!   senses its meaning has in the pupils' dictionary (0 when it has none), the group is 0 for `s >= 2`, 1 for
//!   `s == 1` and 2 for `s == 0`; within a group the interest is `ln(1 + f) * max(s, 1)`, larger first; ties by the
//!   word's bytes. Ambiguous common words come first; words the pupils' dictionary does not have come last.
//! - **Examples** come from the corpus's own sampler (`wikicorpus examples-batch`: up to 100 a word, varied by
//!   relation, head lemma, article and topic, near-duplicates dropped), kept whole in the examples file
//!   (`<out>.examples.jsonl`, one JSON line a word, appended, so a rerun fetches only what is missing).
//! - **Choice** (`choose`): of those, the `ROW_EXAMPLES` the row keeps, best first (the question shows the first two,
//!   cut around the word). Only an example that uses the word by Mova's lemmas, is not boilerplate and is 20 to 220
//!   characters long counts. The first is the clearest: length (whole within the excerpt's 160 characters: 1, else
//!   0.5; a list, with two semicolons or four double quotes, loses 1), a clear sense (+0.5: it shares at least two
//!   content lemmas with one numbered sense and more than with any other) and the sampler's own order
//!   (`0.5 / (1 + i/10)`). Each next one adds variety: a clear sense the chosen ones do not show (+1.5), a new UPOS
//!   (+1: a verb use beside a noun use), a new article (+1), a new article category (+0.5), a new head lemma (+0.5),
//!   and loses three times its largest word overlap (Jaccard of content lemmas) and twice its largest topic overlap
//!   (`topic_sets`: the Jaccard of the content lemmas expanded by their co-occurrences among the word's examples)
//!   with a chosen one; an example overlapping a chosen one by 0.6 or more in words is a near-duplicate and never
//!   chosen.
//!
//! Wikipedia is CC BY-SA 4.0: the examples file and the layer stay outside the repository (`animal::LayerConfig::check`
//! refuses an output inside a Git work tree) and never ship.
//!
//! Deterministic: maps are `BTreeMap`s, floats are compared with `total_cmp` and ties broken by index or bytes.
use crate::layer::{self, Base, Row};
use crate::senses;
use anyhow::{Context, Result, ensure};
use serde::Deserialize;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use std::path::{Path, PathBuf};

/// Examples a row keeps (the question shows the first `senses::EXAMPLES`).
pub const ROW_EXAMPLES: usize = 3;
/// Examples fetched per word, at most.
pub const PER_WORD_MAX: usize = 100;
/// Words fetched per call of the sampler.
pub const CHUNK: usize = 200;
/// Overlap at which an example is a near-duplicate of a chosen one.
pub const NEAR_DUPLICATE: f64 = 0.6;

/// One sampled example (a line of `wikicorpus examples`; unknown fields are ignored).
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Example {
    pub title: String,
    pub sent_id: String,
    pub text: String,
    #[serde(default)]
    pub upos: String,
    #[serde(default)]
    pub head: String,
    #[serde(default)]
    pub category: String,
}

/// One word in the sampling order and why it is there.
#[derive(Clone, Debug, PartialEq)]
pub struct Candidate {
    pub word: String,
    /// tier-1 occurrences over the content UPOS
    pub f: u64,
    /// numbered senses in the pupils' dictionary
    pub senses: usize,
    /// 0: two senses or more; 1: one; 2: not in the pupils' dictionary
    pub group: u8,
    pub interest: f64,
    /// the UPOS the examples are asked for: the main one when it has 80% of the uses, else none (every UPOS)
    pub upos: Option<String>,
}

/// `ln(1 + f) * max(s, 1)`.
pub fn interest(f: u64, s: usize) -> f64 {
    (1.0 + f as f64).ln() * s.max(1) as f64
}

pub fn group(s: usize) -> u8 {
    match s {
        0 => 2,
        1 => 1,
        _ => 0,
    }
}

/// The content words of `words.tsv`: lemma → UPOS → occurrences.
pub fn read_words(text: &str) -> BTreeMap<String, BTreeMap<String, u64>> {
    let mut m: BTreeMap<String, BTreeMap<String, u64>> = BTreeMap::new();
    for l in text.lines() {
        let mut c = l.split('\t');
        let (Some(k), Some(n)) = (c.next(), c.next()) else { continue };
        let Some((lemma, upos)) = k.rsplit_once('|') else { continue };
        let Ok(n) = n.trim().parse::<u64>() else { continue };
        if !layer::CONTENT.contains(&upos) || lemma != lemma.to_lowercase() || !layer::admissible(lemma) || senses::junk(lemma).is_some() {
            continue;
        }
        *m.entry(lemma.to_string()).or_default().entry(upos.to_string()).or_default() += n;
    }
    m
}

/// The words of layer files (their first column), for `skip`.
pub fn layer_words(paths: &[PathBuf]) -> Result<BTreeSet<String>> {
    let mut s = BTreeSet::new();
    for p in paths {
        s.extend(layer::parse_rows(&std::fs::read_to_string(p).with_context(|| format!("{}", p.display()))?).into_iter().map(|r| r.word));
    }
    Ok(s)
}

/// The sampling order (module rule).
pub fn order(words: &BTreeMap<String, BTreeMap<String, u64>>, base: &Base, skip: &BTreeSet<String>, min_count: u64) -> Vec<Candidate> {
    let mut v: Vec<Candidate> = words
        .iter()
        .filter(|(w, _)| !skip.contains(*w))
        .filter_map(|(w, by)| {
            let f: u64 = by.values().sum();
            if f < min_count.max(1) {
                return None;
            }
            let (main, n) = by.iter().max_by(|a, b| a.1.cmp(b.1).then(b.0.cmp(a.0)))?;
            let s = base.get(w).map_or(0, |(_, m)| senses::parse(m).len());
            Some(Candidate { word: w.clone(), f, senses: s, group: group(s), interest: interest(f, s), upos: (*n * 5 >= f * 4).then(|| main.clone()) })
        })
        .collect();
    v.sort_by(|a, b| a.group.cmp(&b.group).then(b.interest.total_cmp(&a.interest)).then(a.word.as_bytes().cmp(b.word.as_bytes())));
    v
}

fn jaccard(a: &BTreeSet<String>, b: &BTreeSet<String>) -> f64 {
    let u = a.union(b).count();
    if u == 0 { 0.0 } else { a.intersection(b).count() as f64 / u as f64 }
}

/// The indices of the examples a row keeps, best first (module rule).
pub fn choose(word: &str, examples: &[Example], numbered: &[(u32, String)], k: usize) -> Vec<usize> {
    struct C {
        i: usize,
        base: f64,
        best: Option<u32>,
        set: BTreeSet<String>,
    }
    let cands: Vec<C> = examples
        .iter()
        .enumerate()
        .filter(|(_, e)| {
            let n = e.text.chars().count();
            (layer::MIN_EXAMPLE..=layer::MAX_EXAMPLE).contains(&n) && !senses::boilerplate(&e.text) && senses::find(&e.text, word).is_some()
        })
        .map(|(i, e)| {
            let n = e.text.chars().count();
            // a list or a quotation chain (titles, credits) reads badly as a sense example
            let listy = e.text.matches(';').count() >= 2 || e.text.matches('"').count() >= 4;
            let len_q = if n <= senses::EXCERPT { 1.0 } else { 0.5 } - if listy { 1.0 } else { 0.0 };
            let mut sc = senses::score(word, std::slice::from_ref(&e.text), numbered);
            sc.sort_by(|a, b| b.score.cmp(&a.score).then(a.sense.cmp(&b.sense)));
            let top = sc.first().map_or(0, |s| s.score);
            let next = sc.get(1).map_or(0, |s| s.score);
            // a clear sense: at least two shared lemmas and more than the next (one shared word is noise)
            let best = (top >= senses::PICK_MIN && top > next).then(|| sc[0].sense);
            let clear = if best.is_some() { 0.5 } else { 0.0 };
            C { i, base: len_q + clear + 0.5 / (1.0 + i as f64 / 10.0), best, set: senses::content(&e.text, word) }
        })
        .collect();
    let topics = topic_sets(&cands.iter().map(|c| &c.set).collect::<Vec<_>>());
    let mut chosen: Vec<usize> = Vec::new();
    while chosen.len() < k {
        let mut pick: Option<(f64, usize)> = None;
        for (ci, c) in cands.iter().enumerate() {
            if chosen.contains(&ci) {
                continue;
            }
            let over = chosen.iter().map(|&j| jaccard(&c.set, &cands[j].set)).fold(0.0, f64::max);
            if over >= NEAR_DUPLICATE {
                continue;
            }
            let e = &examples[c.i];
            let topic = chosen.iter().map(|&j| jaccard(&topics[ci], &topics[j])).fold(0.0, f64::max);
            let mut s = c.base - 3.0 * over - 2.0 * topic;
            if !chosen.is_empty() {
                let was = |f: &dyn Fn(&Example) -> &str| chosen.iter().any(|&j| f(&examples[cands[j].i]) == f(e));
                if c.best.is_some() && chosen.iter().all(|&j| cands[j].best != c.best) {
                    s += 1.5;
                }
                if !was(&|x: &Example| x.upos.as_str()) {
                    s += 1.0;
                }
                if !was(&|x: &Example| x.title.as_str()) {
                    s += 1.0;
                }
                if !was(&|x: &Example| x.category.as_str()) {
                    s += 0.5;
                }
                if !was(&|x: &Example| x.head.as_str()) {
                    s += 0.5;
                }
            }
            if pick.is_none_or(|(b, _)| s > b) {
                pick = Some((s, ci));
            }
        }
        let Some((_, ci)) = pick else { break };
        chosen.push(ci);
    }
    chosen.into_iter().map(|ci| cands[ci].i).collect()
}

/// Each example's topic: its content lemmas that are not common among the word's examples (in at most 30% of them),
/// with every such lemma that occurs together with one of them in at least two examples. Examples of one sense share
/// their topics (a football pitch: ball, goal, club, stadium) even when they share no word.
pub fn topic_sets(sets: &[&BTreeSet<String>]) -> Vec<BTreeSet<String>> {
    let cap = ((sets.len() as f64) * 0.3).ceil().max(2.0) as usize;
    let mut df: BTreeMap<&str, usize> = BTreeMap::new();
    for s in sets {
        for w in s.iter() {
            *df.entry(w.as_str()).or_default() += 1;
        }
    }
    let keep: Vec<Vec<&str>> = sets.iter().map(|s| s.iter().map(String::as_str).filter(|w| df[w] <= cap).collect()).collect();
    let mut cooc: BTreeMap<(&str, &str), usize> = BTreeMap::new();
    for k in &keep {
        for a in k {
            for b in k {
                if a != b {
                    *cooc.entry((*a, *b)).or_default() += 1;
                }
            }
        }
    }
    let mut near: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for ((a, b), n) in &cooc {
        if *n >= 2 {
            near.entry(*a).or_default().insert(*b);
        }
    }
    keep.iter()
        .map(|k| {
            let mut t: BTreeSet<String> = k.iter().map(|w| w.to_string()).collect();
            for w in k {
                t.extend(near.get(w).into_iter().flatten().map(|x| x.to_string()));
            }
            t
        })
        .collect()
}

/// One word's sampled examples as kept in the examples file.
#[derive(Clone, Debug, PartialEq)]
pub struct Sampled {
    pub word: String,
    pub examples: Vec<Example>,
}

/// One line of the examples file (unknown fields ignored).
#[derive(Deserialize)]
struct Line {
    lemma: String,
    #[serde(default)]
    examples: Vec<Example>,
}

/// Stream the examples file: `f` sees each word's examples once (the first line of a word wins). A missing file is
/// empty; a broken line fails. The file may be large (about 35 kB a word), so it is never held whole.
pub fn scan(path: &Path, mut f: impl FnMut(Sampled) -> Result<()>) -> Result<usize> {
    use std::io::BufRead;
    let file = match std::fs::File::open(path) {
        Ok(x) => x,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(0),
        Err(e) => return Err(e.into()),
    };
    let mut seen = BTreeSet::new();
    for (n, l) in std::io::BufReader::new(file).lines().enumerate() {
        let l = l?;
        if l.trim().is_empty() {
            continue;
        }
        let x: Line = serde_json::from_str(&l).with_context(|| format!("{}:{}: a bad line", path.display(), n + 1))?;
        if seen.insert(x.lemma.clone()) {
            f(Sampled { word: x.lemma, examples: x.examples })?;
        }
    }
    Ok(seen.len())
}

/// The examples file whole: word → its examples (small files and tests).
pub fn read_examples(path: &Path) -> Result<BTreeMap<String, Sampled>> {
    let mut m = BTreeMap::new();
    scan(path, |s| {
        m.insert(s.word.clone(), s);
        Ok(())
    })?;
    Ok(m)
}

/// How the sampler is called.
#[derive(Clone, Debug)]
pub struct Sampler {
    /// the `wikicorpus` binary
    pub bin: PathBuf,
    /// the corpus root (`WIKICORPUS_ROOT`)
    pub root: PathBuf,
    pub per_word: usize,
    pub seed: u64,
    pub deep: u64,
    pub jobs: usize,
}

impl Sampler {
    /// Fetch the examples of `words` (in chunks of `CHUNK`) and append each chunk's lines to `path` once the sampler
    /// has answered for all of it. A failed call stops (no retry); what was appended stays.
    pub fn fetch(&self, words: &[&Candidate], path: &Path) -> Result<usize> {
        ensure!((1..=PER_WORD_MAX).contains(&self.per_word), "per_word is 1..{PER_WORD_MAX}");
        let mut n = 0;
        for chunk in words.chunks(CHUNK) {
            let input: String = chunk.iter().map(|c| format!("{}\t{}\n", c.word, c.upos.as_deref().unwrap_or(""))).collect();
            let mut child = std::process::Command::new(&self.bin)
                .args(["examples-batch", "--n", &self.per_word.to_string(), "--seed", &self.seed.to_string(), "--deep", &self.deep.to_string(), "--jobs", &self.jobs.max(1).to_string()])
                .env("WIKICORPUS_ROOT", &self.root)
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()
                .with_context(|| format!("{}", self.bin.display()))?;
            child.stdin.take().context("stdin")?.write_all(input.as_bytes())?;
            let out = child.wait_with_output()?;
            ensure!(out.status.success(), "the sampler failed: {}", String::from_utf8_lossy(&out.stderr).trim());
            let text = String::from_utf8(out.stdout)?;
            let lines: Vec<&str> = text.lines().filter(|l| !l.trim().is_empty()).collect();
            ensure!(lines.len() == chunk.len(), "the sampler answered {} lines for {} words", lines.len(), chunk.len());
            for (l, c) in lines.iter().zip(chunk) {
                let v: Value = serde_json::from_str(l)?;
                ensure!(v["lemma"].as_str() == Some(c.word.as_str()), "the sampler answered «{}» for «{}»", v["lemma"], c.word);
            }
            if let Some(d) = path.parent() {
                std::fs::create_dir_all(d)?;
            }
            let mut f = std::fs::OpenOptions::new().create(true).append(true).open(path)?;
            f.write_all(lines.iter().map(|l| format!("{l}\n")).collect::<String>().as_bytes())?;
            n += chunk.len();
        }
        Ok(n)
    }
}

/// A row of the sampled layer: the pupils' part of speech and meaning, the sampling evidence, the chosen examples.
pub fn row(c: &Candidate, s: &Sampled, base: &Base) -> Row {
    let numbered = base.get(&c.word).map(|(_, m)| senses::parse(m)).unwrap_or_default();
    let pick = choose(&c.word, &s.examples, &numbered, ROW_EXAMPLES);
    let (pos, meaning) = base.get(&c.word).cloned().unwrap_or_else(|| (c.upos.as_deref().map_or("word", upos_name).to_string(), String::new()));
    let articles: BTreeSet<&str> = s.examples.iter().map(|e| e.title.as_str()).collect();
    Row {
        word: c.word.clone(),
        pos,
        meaning,
        sense: format!(
            "Sampled by Mova from English Wikipedia (tier 1): {} {} in tier 1; {} numbered {} in the pupils' dictionary; interest {:.2} (group {}); {} {} sampled from {} {}, {} kept for the question.",
            c.f,
            if c.f == 1 { "occurrence" } else { "occurrences" },
            c.senses,
            if c.senses == 1 { "sense" } else { "senses" },
            c.interest,
            c.group,
            s.examples.len(),
            if s.examples.len() == 1 { "example" } else { "examples" },
            articles.len(),
            if articles.len() == 1 { "article" } else { "articles" },
            pick.len()
        ),
        examples: pick.iter().map(|&i| format!("{} «{}»", s.examples[i].text.split_whitespace().collect::<Vec<_>>().join(" "), s.examples[i].sent_id)).collect(),
        note: "Draft: sampled natively by Nautilus from the English Wikipedia corpus (examples by wikicorpus, chosen for the sense question by Nautilus); the meaning is the pupils' dictionary's; no sense of this source is written yet.".into(),
    }
}

fn upos_name(u: &str) -> &'static str {
    match u {
        "NOUN" => "noun",
        "VERB" => "verb",
        "ADJ" => "adjective",
        "ADV" => "adverb",
        _ => "word",
    }
}

/// The layer file of the sampled rows.
pub fn render(rows: &[Row], fetched: usize, licence: &str) -> String {
    let mut s = format!(
        "{}\n# draft: sampled natively by Nautilus (tools/nautilus) from the English Wikipedia corpus's tier 1, {} words with examples, {} rows in the sampling order (ambiguous common words first); not reviewed; {licence}\n",
        layer::HEAD,
        fetched,
        rows.len()
    );
    for r in rows {
        s.push_str(&r.tsv());
        s.push('\n');
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    fn ex(title: &str, n: usize, text: &str, head: &str, cat: &str) -> Example {
        Example { title: title.into(), sent_id: format!("wiki:{}:{n}", title.replace(' ', "_")), text: text.into(), upos: "NOUN".into(), head: head.into(), category: cat.into() }
    }
    #[test]
    fn the_order_puts_ambiguous_common_words_first() {
        let words = read_words("bank|NOUN\t20000\t100\nbank|VERB\t700\t99\nbank|AUX\t7\t7\ncell|NOUN\t30000\t100\nzebra|NOUN\t900\t80\nqi|NOUN\t500\t9\nParis|PROPN\t9000\t100\nrare|ADJ\t3\t3\nlobby|NOUN\t40\t30\nlobby|VERB\t40\t30\n");
        // AUX and PROPN are not content words; a two-letter word is junk
        assert_eq!(words.keys().collect::<Vec<_>>(), ["bank", "cell", "lobby", "rare", "zebra"]);
        assert_eq!(words["bank"].values().sum::<u64>(), 20700);
        let base = layer::read_base("bank\tnoun\t1) A bank keeps money. 2) A bank is the side of a river.\ncell\tnoun\tA cell is a small room.\nlobby\tnoun\t1) A hall. 2) To try to sway lawmakers.\n");
        let o = order(&words, &base, &BTreeSet::new(), 20);
        let w: Vec<&str> = o.iter().map(|c| c.word.as_str()).collect();
        // two senses first (bank before lobby by interest), then one sense (cell), then none (zebra); rare is under the minimum
        assert_eq!(w, ["bank", "lobby", "cell", "zebra"]);
        assert_eq!((o[0].group, o[0].senses, o[0].f), (0, 2, 20700));
        assert!((o[0].interest - 2.0 * 20701f64.ln()).abs() < 1e-9);
        // the main UPOS when it has 80% of the uses, else every UPOS
        assert_eq!((o[0].upos.as_deref(), o[1].upos.as_deref()), (Some("NOUN"), None));
        // negative control: a skipped word (an earlier layer's) is not in the order
        let skip: BTreeSet<String> = ["bank".to_string()].into();
        assert!(order(&words, &base, &skip, 20).iter().all(|c| c.word != "bank"));
        // deterministic: the same input, the same order
        assert_eq!(o, order(&words, &base, &BTreeSet::new(), 20));
    }
    #[test]
    fn the_choice_shows_different_senses_and_no_near_duplicates() {
        let numbered = senses::parse("1) A bank is a business that keeps money and lends loans. 2) A bank is the land along the side of a river.");
        let e = vec![
            ex("Moscow", 3, "The bank lends money and keeps savings for its customers in the city.", "lend", "Banks"),
            ex("Moscow", 4, "The bank lends money and keeps savings for its customers in the town.", "lend", "Banks"),
            ex("Prague", 9, "The castle stands on the left bank of the river Vltava near the old land.", "stand", "Rivers"),
            ex("Kyiv", 2, "The central bank raised its rates in the spring of that year, as the money supply grew.", "raise", "Banks"),
            ex("Nowhere", 1, "Banking.", "x", "y"),
            ex("Licence", 1, "See the bank of https://example.org for more about the bank and its money.", "see", "z"),
            ex("Charts", 5, "Bank; 2005; \"Money\" (4); \"Bank Holiday\"; more money.", "x", "Songs"),
        ];
        let pick = choose("bank", &e, &numbered, 3);
        // the clearest first, then the river sense (a new sense, a new article), never the near-duplicate (index 1)
        assert_eq!(pick[..2], [0, 2]);
        assert!(!pick.contains(&1), "{pick:?}");
        // even when every other example is taken, the near-duplicate never is
        let all = choose("bank", &e, &numbered, 10);
        assert!(all.len() >= 3 && !all.contains(&1), "{all:?}");
        // negative controls: too short, boilerplate and examples without the word are never chosen
        assert!(!pick.contains(&4) && !pick.contains(&5));
        // a list is not chosen while prose is left
        assert!(!pick.contains(&6), "{pick:?}");
        assert!(choose("zebra", &e, &numbered, 3).is_empty());
        // deterministic
        assert_eq!(pick, choose("bank", &e, &numbered, 3));
    }
    #[test]
    fn topics_join_examples_of_one_sense() {
        let set = |ws: &[&str]| ws.iter().map(|w| w.to_string()).collect::<BTreeSet<String>>();
        let sets = [set(&["ball", "goal"]), set(&["ball", "goal", "club"]), set(&["club", "stadium"]), set(&["note", "sound"]), set(&["note", "sound", "high"]), set(&["year"]), set(&["year"]), set(&["year"]), set(&["year"]), set(&["year"])];
        let refs: Vec<&BTreeSet<String>> = sets.iter().collect();
        let t = topic_sets(&refs);
        // «stadium» shares no word with «ball, goal», but «club» joins them through the second example
        assert!(jaccard(&sets[0], &sets[2]) == 0.0 && jaccard(&t[0], &t[2]) == 0.0 && jaccard(&t[1], &t[2]) > 0.0);
        assert!(t[0].contains("ball") && t[3].contains("note") && jaccard(&t[0], &t[3]) == 0.0);
        // negative control: a lemma in more than 30% of the examples is no topic
        assert!(t[5].is_empty());
    }
    #[test]
    fn rows_and_the_examples_file() {
        let d = std::env::temp_dir().join(format!("nautilus-sampled-{}-{}", std::process::id(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        std::fs::create_dir_all(&d).unwrap();
        let p = d.join("w.examples.jsonl");
        let line = |w: &str, t: &str| serde_json::json!({"lemma": w, "upos": "NOUN", "n": 2, "examples": [
            {"title": "A", "sent_id": "wiki:A:1", "text": t, "lemma": w, "upos": "NOUN", "form": w, "rel": "obj", "head": "see", "tokens": 9, "class": "city", "category": "C", "rank": 3, "from": "tier1"},
            {"title": "B", "sent_id": "wiki:B:2", "text": format!("Another {w} stood by the road in the old town."), "head": "stand", "category": "D"}]}).to_string();
        std::fs::write(&p, format!("{}\n{}\n", line("bank", "We saw the bank near the river at noon."), line("bank", "A later bank line is ignored here."))).unwrap();
        let m = read_examples(&p).unwrap();
        assert_eq!(m["bank"].examples.len(), 2);
        assert_eq!(m["bank"].examples[0].text, "We saw the bank near the river at noon.");
        let base = layer::read_base("bank\tnoun\t1) A bank keeps money. 2) The side of a river.\n");
        let c = &order(&read_words("bank|NOUN\t50\t3\n"), &base, &BTreeSet::new(), 1)[0];
        let r = row(c, &m["bank"], &base);
        assert_eq!(r.examples.len(), 2);
        assert!(r.examples[0].ends_with("«wiki:A:1»") || r.examples[0].ends_with("«wiki:B:2»"), "{:?}", r.examples);
        assert!(r.sense.contains("50 occurrences") && r.sense.contains("2 numbered senses") && r.sense.contains("2 examples sampled from 2 articles"), "{}", r.sense);
        // the row reads back and plans as a question with two examples
        let text = render(&[r.clone()], 1, "CC BY-SA 4.0");
        assert_eq!(layer::parse_rows(&text), vec![r.clone()]);
        let plan = senses::plan(&r.word, &r.examples, 2, senses::parse(&r.meaning), None, true);
        assert_eq!(plan.uses.len(), 2);
        // negative control: a missing examples file is empty; a broken line fails
        assert!(read_examples(&d.join("none.jsonl")).unwrap().is_empty());
        std::fs::write(&p, "not json\n").unwrap();
        assert!(read_examples(&p).is_err());
        let _ = std::fs::remove_dir_all(&d);
    }
    #[test]
    fn a_failing_sampler_appends_nothing() {
        let d = std::env::temp_dir().join(format!("nautilus-sampler-{}-{}", std::process::id(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        std::fs::create_dir_all(&d).unwrap();
        let p = d.join("x.examples.jsonl");
        let c = Candidate { word: "bank".into(), f: 9, senses: 2, group: 0, interest: 1.0, upos: None };
        // `false` exits 1: the call fails and nothing is appended (no retry)
        let s = Sampler { bin: "/bin/false".into(), root: d.clone(), per_word: 100, seed: 1, deep: 0, jobs: 1 };
        assert!(s.fetch(&[&c], &p).is_err());
        assert!(!p.exists());
        // `true` exits 0 with no lines: a short answer fails too
        let s = Sampler { bin: "/bin/true".into(), ..s };
        assert!(s.fetch(&[&c], &p).is_err());
        assert!(!p.exists());
        // over the cap is refused before any call
        assert!(Sampler { per_word: 101, ..s }.fetch(&[&c], &p).is_err());
        let _ = std::fs::remove_dir_all(&d);
    }
}
