//! The parse database (15.10): one best tree per sentence, outside the repo (`CYCLE_DB`, default
//! `$MOVA_BIG/parsedb`, $MOVA_BIG as in `crate::mova_big`). `current.conllu` holds the trees, `log.tsv` how each one got there (time, sentence,
//! source, added or replaced). Every Haiku result improves it; the GPU SLM trains on it.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};

use crate::{ids_of, R};

pub fn dir() -> PathBuf {
    PathBuf::from(std::env::var("CYCLE_DB").unwrap_or_else(|_| format!("{}/parsedb", crate::mova_big())))
}

/// Every database the one model learns from (operator, 17.10: «all corpora train at once — the weights and the
/// skeleton are one and open»): this pipeline's own (`dir`) first, then `CYCLE_TRAIN_DBS` (colon-separated), each once.
pub fn train_dbs() -> Vec<PathBuf> {
    let mut v = vec![dir()];
    for d in std::env::var("CYCLE_TRAIN_DBS").unwrap_or_default().split(':').filter(|x| !x.is_empty()) {
        let p = PathBuf::from(d);
        if !v.contains(&p) {
            v.push(p);
        }
    }
    v
}

/// Blocks of a CoNLL-U text keyed by sentence id, in file order.
pub(crate) fn blocks(text: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut body = String::new();
    let mut id = None;
    let mut last_word_end = 0;
    for line in text.lines() {
        let next = line.strip_prefix("# sent_id = ");
        if line.is_empty() || (next.is_some() && id.is_some()) {
            if let Some(old) = id.take() {
                // A joined sentence may have newdoc/version/other comment preambles before sent_id.
                let pending = if next.is_some() && last_word_end > 0 { body.split_off(last_word_end) } else { String::new() };
                out.push((old, format!("{}\n\n", body.trim_end())));
                body = pending;
                last_word_end = 0;
            } else if line.is_empty() { body.clear(); last_word_end = 0; }
        }
        if !line.is_empty() {
            if let Some(next) = next { id = Some(next.to_string()); }
            body.push_str(line);
            body.push('\n');
            if !line.starts_with('#') { last_word_end = body.len(); }
        }
    }
    if let Some(id) = id { out.push((id, format!("{}\n\n", body.trim_end()))); }
    out
}

/// The database's trees followed by the trees of `drafts` it does not have yet (a new document's sentences, so the
/// temporary world of a first pass already sees what came before them in this round).
pub fn with_drafts(db_text: &str, drafts: &str) -> String {
    let known: std::collections::BTreeSet<String> = blocks(db_text).into_iter().map(|(id, _)| id).collect();
    let mut out = db_text.trim_end().to_string();
    out.push_str("\n\n");
    for (id, b) in blocks(drafts) {
        if !known.contains(&id) {
            out.push_str(&b);
        }
    }
    out
}

/// Put the trees of `file` into the database: a known sentence is replaced, a new one appended.
pub fn update(db: &Path, file: &Path, source: &str) -> R<(usize, usize)> {
    crate::reviewed::ready(db)?;
    let selected = crate::reviewed::policy(db)?;
    let incoming = fs::read_to_string(file).map_err(|e| e.to_string())?;
    if let Some(ids) = selected {
        let current = crate::reviewed::trees(&fs::read_to_string(db.join("reviewed.conllu")).map_err(|e| e.to_string())?)?;
        for (id, block) in crate::reviewed::trees(&incoming)? {
            if ids.contains(&id) && current.get(&id) != Some(&block) { return Err(format!("update conflicts with reviewed selection {id}")); }
        }
    }
    fs::create_dir_all(db).map_err(|e| e.to_string())?;
    let cur = db.join("current.conllu");
    let mut order: Vec<(String, String)> = blocks(&fs::read_to_string(&cur).unwrap_or_default());
    let mut at: BTreeMap<String, usize> = order.iter().enumerate().map(|(i, (id, _))| (id.clone(), i)).collect();
    let (mut added, mut replaced) = (0, 0);
    let mut log = String::new();
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    for (id, b) in blocks(&incoming) {
        match at.get(&id) {
            Some(&i) => {
                order[i].1 = b;
                replaced += 1;
                log.push_str(&format!("{now}\t{id}\t{source}\treplaced\n"));
            }
            None => {
                at.insert(id.clone(), order.len());
                order.push((id.clone(), b));
                added += 1;
                log.push_str(&format!("{now}\t{id}\t{source}\tadded\n"));
            }
        }
    }
    let tmp = db.join("current.conllu.tmp");
    fs::write(&tmp, order.iter().map(|(_, b)| b.as_str()).collect::<String>()).map_err(|e| e.to_string())?;
    fs::rename(&tmp, &cur).map_err(|e| e.to_string())?;
    // every version is kept (design note, 15.10: the SLM sees many parses of a sentence, from different models at different
    // times, and agrees them): `history.conllu`, each block marked with its source and time
    let mut hist = String::new();
    for (_, b) in blocks(&incoming) {
        hist.push_str(&format!("# version = {source} {now}\n{b}"));
    }
    let mut h = fs::OpenOptions::new().create(true).append(true).open(db.join("history.conllu")).map_err(|e| e.to_string())?;
    h.write_all(hist.as_bytes()).map_err(|e| e.to_string())?;
    let mut f = fs::OpenOptions::new().create(true).append(true).open(db.join("log.tsv")).map_err(|e| e.to_string())?;
    f.write_all(log.as_bytes()).map_err(|e| e.to_string())?;
    Ok((added, replaced))
}

/// The weight of a version's source in the vote: Opus 8, Sonnet 4, Haiku (a round) 2, the agreed seed 1.
pub fn weight(source: &str) -> u32 {
    if source.ends_with("-opus") {
        8
    } else if source.ends_with("-sonnet") {
        4
    } else if source.starts_with("round") {
        2
    } else {
        1
    }
}

/// Word lines of a block: (columns).
fn words(b: &str) -> Vec<Vec<String>> {
    b.lines().filter(|l| !l.starts_with('#') && !l.is_empty()).map(|l| l.split('\t').map(String::from).collect()).filter(|c: &Vec<String>| c.len() == 10).collect()
}

/// One root, no cycles, heads in range.
pub(crate) fn tree_ok(heads: &[usize]) -> bool {
    let n = heads.len();
    if heads.iter().filter(|&&h| h == 0).count() != 1 || heads.iter().enumerate().any(|(i, &h)| h > n || h == i + 1) {
        return false;
    }
    (1..=n).all(|i| {
        let mut j = i;
        for _ in 0..=n {
            j = heads[j - 1];
            if j == 0 {
                return true;
            }
        }
        false
    })
}

/// Agree all the versions of every sentence (`history.conllu`): per word, a weighted vote over (head, relation) —
/// Opus 8, Sonnet 4, Haiku 2, the seed 1, a tie to the newest; tags, lemmas and features from the strongest newest
/// version. A voted tree that is not a tree falls back to that version. Rewrites `current.conllu`; returns (sentences
/// with more than one version, words the vote set differently from the strongest version).
pub fn agree(db: &Path) -> R<(usize, usize)> {
    agree_with(db, std::env::var("CYCLE_VOTE_MISC").is_ok_and(|v| v == "1"))
}

/// `agree`; `margin` — keep each word's vote in MISC (`CYCLE_VOTE_MISC=1`).
pub fn agree_with(db: &Path, margin: bool) -> R<(usize, usize)> {
    crate::reviewed::ready(db)?;
    crate::reviewed::policy(db)?;
    let hist = fs::read_to_string(db.join("history.conllu")).map_err(|e| e.to_string())?;
    // id → versions (weight, time, order, block)
    let mut vs: BTreeMap<String, Vec<(u32, u64, usize, String)>> = BTreeMap::new();
    for (k, (_, b)) in blocks(&hist).into_iter().enumerate() {
        let Some(v) = b.lines().find_map(|l| l.strip_prefix("# version = ")) else { continue };
        let (src, when) = v.rsplit_once(' ').unwrap_or((v, "0"));
        let Some(id) = ids_of(&b).into_iter().next() else { continue };
        let body: String = b.lines().filter(|l| !l.starts_with("# version = ")).map(|l| format!("{l}\n")).collect();
        vs.entry(id).or_default().push((weight(src), when.parse().unwrap_or(0), k, body));
    }
    let cur = db.join("current.conllu");
    let mut order: Vec<(String, String)> = blocks(&fs::read_to_string(&cur).unwrap_or_default());
    let (mut multi, mut moved) = (0, 0);
    for (id, b) in order.iter_mut() {
        let Some(v) = vs.get(id) else { continue };
        let best = v.iter().max_by_key(|x| (x.0, x.1, x.2)).expect("non-empty");
        let mut base = words(&best.3);
        if v.len() < 2 || base.is_empty() {
            continue;
        }
        multi += 1;
        let n = base.len();
        let mut votes: Vec<BTreeMap<(String, String), (u32, u64, usize)>> = vec![BTreeMap::new(); n];
        for (w, t, k, blk) in v {
            let ws = words(blk);
            if ws.len() != n {
                continue;
            }
            for (i, c) in ws.iter().enumerate() {
                let e = votes[i].entry((c[6].clone(), c[7].clone())).or_insert((0, 0, 0));
                e.0 += w;
                e.1 = e.1.max(*t);
                e.2 = e.2.max(*k);
            }
        }
        let pick: Vec<(String, String)> = votes.iter().map(|m| m.iter().max_by_key(|(_, s)| **s).map(|(x, _)| x.clone()).unwrap_or_default()).collect();
        let heads: Vec<usize> = pick.iter().map(|(h, _)| h.parse().unwrap_or(usize::MAX)).collect();
        if tree_ok(&heads) {
            for (c, (h, r)) in base.iter_mut().zip(&pick) {
                if (&c[6], &c[7]) != (h, r) {
                    moved += 1;
                    c[6] = h.clone();
                    c[7] = r.clone();
                }
            }
        }
        // CYCLE_VOTE_MISC=1: the vote's margin stays with the tree (whichever tree was kept), in MISC `Vote`,
        // `VoteHead`, `VoteAlt` of every word line; off (the default), `current.conllu` is as before byte for byte —
        // reviewed promotion pins the hashes of current trees, so the margin is an explicit choice of a database
        if margin {
            for (c, m) in base.iter_mut().zip(&votes) {
                if !c[0].contains(['-', '.']) {
                    c[9] = with_vote(&c[9], &vote_misc(m, &c[6], &c[7]));
                }
            }
        } else if !tree_ok(&heads) {
            *b = best.3.clone();
            continue;
        }
        let comments: String = best.3.lines().filter(|l| l.starts_with('#')).map(|l| format!("{l}\n")).collect();
        *b = format!("{comments}{}\n", base.iter().map(|c| format!("{}\n", c.join("\t"))).collect::<String>());
    }
    crate::reviewed::overlay(db, &mut order)?;
    let tmp = db.join("current.conllu.tmp");
    fs::write(&tmp, order.iter().map(|(_, b)| b.as_str()).collect::<String>()).map_err(|e| e.to_string())?;
    fs::rename(&tmp, &cur).map_err(|e| e.to_string())?;
    Ok((multi, moved))
}

/// A word's vote as MISC items: `Vote` — the share of the weight behind its (head, relation), `VoteHead` — behind
/// its head with any relation, `VoteAlt` — the other heads with their shares, largest first (at most 4, shares of at
/// least 0.01). Shares have two decimals. Training can keep only the words the vote is sure of (gparse `GP_PA`).
pub(crate) fn vote_misc(m: &BTreeMap<(String, String), (u32, u64, usize)>, head: &str, rel: &str) -> String {
    let total: u32 = m.values().map(|x| x.0).sum();
    if total == 0 {
        return String::new();
    }
    let share = |w: u32| w as f64 / total as f64;
    let pair = m.get(&(head.to_string(), rel.to_string())).map_or(0, |x| x.0);
    let mut by_head: BTreeMap<&str, u32> = BTreeMap::new();
    for ((h, _), x) in m {
        *by_head.entry(h.as_str()).or_default() += x.0;
    }
    let on_head = by_head.get(head).copied().unwrap_or(0);
    let mut alts: Vec<(&str, u32)> = by_head.into_iter().filter(|(h, w)| *h != head && share(*w) >= 0.005).collect();
    alts.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    let mut out = format!("Vote={:.2}|VoteHead={:.2}", share(pair), share(on_head));
    if !alts.is_empty() {
        out += "|VoteAlt=";
        out += &alts.iter().take(4).map(|(h, w)| format!("{h}:{:.2}", share(*w))).collect::<Vec<_>>().join(",");
    }
    out
}

/// MISC with the vote items replaced (`_` when nothing is left).
pub(crate) fn with_vote(misc: &str, vote: &str) -> String {
    let mut items: Vec<&str> = misc.split('|').filter(|x| !x.is_empty() && *x != "_" && !matches!(x.split_once('=').map(|k| k.0), Some("Vote" | "VoteHead" | "VoteAlt"))).collect();
    if !vote.is_empty() {
        items.extend(vote.split('|'));
    }
    if items.is_empty() { "_".to_string() } else { items.join("|") }
}

/// The pragmatic reading next to the trees: `prag.tsv` (sent_id + fields + what the teacher changed), one row per
/// sentence, the newest wins.
pub fn update_prag(db: &Path, file: &Path) -> R<usize> {
    crate::reviewed::ready(db)?;
    let selected = crate::reviewed::policy(db)?;
    let incoming = fs::read_to_string(file).map_err(|e| e.to_string())?;
    let new_rows = crate::reviewed::prag(&incoming)?;
    if let Some(ids) = selected {
        let rows = crate::reviewed::prag(&fs::read_to_string(db.join("reviewed-prag.tsv")).map_err(|e| e.to_string())?)?;
        for (id, row) in &new_rows {
            if ids.contains(id) && rows.get(id) != Some(row) { return Err(format!("pragmatic update conflicts with reviewed selection {id}")); }
        }
    }
    let cur = db.join("prag.tsv");
    let text = fs::read_to_string(&cur).unwrap_or_default();
    let new = incoming;
    let head = crate::reviewed::HEADER.to_string();
    if !text.is_empty() { crate::reviewed::prag(&text)?; }
    let mut rows: BTreeMap<String, String> = text.lines().skip(1).filter_map(|l| l.split('\t').next().map(|id| (id.to_string(), l.to_string()))).collect();
    let mut n = 0;
    for l in new.lines().skip(1) {
        if let Some(id) = l.split('\t').next() {
            rows.insert(id.to_string(), l.to_string());
            n += 1;
        }
    }
    let tmp = db.join("prag.tsv.tmp");
    fs::write(&tmp, format!("{head}\n{}", rows.values().map(|l| format!("{l}\n")).collect::<String>())).map_err(|e| e.to_string())?;
    fs::rename(&tmp, &cur).map_err(|e| e.to_string())?;
    Ok(n)
}

/// Sentences of a pool the database has no tree for, whose text it already has under another id (operator, 17.10:
/// «align them»): the cycle sends a text to the teacher once, so a verse repeated word for word — Exodus 25–31 and
/// 35–40, «And the LORD spoke unto Moses, saying:» — got no tree under its own address. Each gets a copy of the tree
/// (and the pragmatic row) of the first sentence with the same text, under its own id; logged as `same-text`.
/// Returns (filled, still without a tree).
pub fn same_text(db: &Path, pool: &Path) -> R<(usize, usize)> {
    let cur = fs::read_to_string(db.join("current.conllu")).map_err(|e| e.to_string())?;
    let have: BTreeMap<String, String> = blocks(&cur).into_iter().collect();
    let mut by_text: BTreeMap<String, String> = BTreeMap::new();
    for (id, b) in &have {
        if let Some(t) = b.lines().find_map(|l| l.strip_prefix("# text = ")) {
            by_text.entry(t.split_whitespace().collect::<Vec<_>>().join(" ")).or_insert(id.clone());
        }
    }
    let prag_text = fs::read_to_string(db.join("prag.tsv")).unwrap_or_default();
    let prag: BTreeMap<&str, &str> = prag_text.lines().skip(1).filter_map(|l| l.split_once('\t').map(|(id, _)| (id, l))).collect();
    let (mut trees, mut rows, mut left) = (String::new(), String::new(), 0usize);
    let mut filled = 0;
    for l in fs::read_to_string(pool).map_err(|e| e.to_string())?.lines() {
        let Some((id, text)) = l.split_once('\t') else { continue };
        if have.contains_key(id) {
            continue;
        }
        let Some(src) = by_text.get(&text.split_whitespace().collect::<Vec<_>>().join(" ")) else {
            left += 1;
            continue;
        };
        // the copy under its own id, with a note of where it came from among the comments
        for x in have[src].lines().filter(|x| !x.is_empty()) {
            if x.starts_with("# sent_id = ") {
                trees.push_str(&format!("# sent_id = {id}\n# same_text_as = {src}\n"));
            } else {
                trees.push_str(x);
                trees.push('\n');
            }
        }
        trees.push('\n');
        if let Some(p) = prag.get(src.as_str()) {
            rows.push_str(&format!("{id}{}\n", &p[src.len()..]));
        }
        filled += 1;
    }
    if filled > 0 {
        let f = db.join("same-text.conllu");
        fs::write(&f, trees).map_err(|e| e.to_string())?;
        update(db, &f, "same-text")?;
        let _ = fs::remove_file(&f);
        if !rows.is_empty() {
            let p = db.join("same-text-prag.tsv");
            fs::write(&p, format!("{}\n{rows}", prag_text.lines().next().unwrap_or("sent_id"))).map_err(|e| e.to_string())?;
            update_prag(db, &p)?;
            let _ = fs::remove_file(&p);
        }
    }
    Ok((filled, left))
}

/// Normalized sentence text for held-out matching: the lowercase letters and digits only, so spacing, punctuation,
/// quote style and case do not let a dev sentence back into training under another id; a text without any letter or
/// digit keeps its non-space characters.
pub fn norm_text(s: &str) -> String {
    let n: String = s.chars().flat_map(char::to_lowercase).filter(|c| c.is_alphanumeric()).collect();
    if n.is_empty() {
        s.split_whitespace().collect()
    } else {
        n
    }
}

/// A CoNLL-U block's text: its `# text = ` comment, else its word forms joined by spaces.
pub fn block_text(b: &str) -> String {
    if let Some(t) = b.lines().find_map(|l| l.strip_prefix("# text = ")) {
        return t.to_string();
    }
    b.lines()
        .filter(|l| !l.starts_with('#'))
        .filter_map(|l| {
            let c: Vec<&str> = l.split('\t').collect();
            (c.len() > 1 && c[0].parse::<usize>().is_ok()).then(|| c[1])
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Every development and held-out sentence the training export must not reach (gparse-leak, 2026-11-05: the
/// school dev set had entered the training files): their ids and their normalized texts (`norm_text`). One shared,
/// ordered set; `hits` is the one test every training input of the cycle goes through.
#[derive(Default, Debug)]
pub struct Heldout {
    pub ids: BTreeSet<String>,
    pub texts: BTreeSet<String>,
    /// (source, sentences read from it); a default source that is absent is listed with `absent`
    pub sources: Vec<(String, String)>,
}

impl Heldout {
    /// Only these ids (no texts), for tests.
    #[cfg(test)]
    pub fn of_ids(ids: BTreeSet<String>) -> Heldout {
        Heldout { ids, ..Heldout::default() }
    }

    /// Add every sentence of a CoNLL-U text; returns how many.
    pub fn add_conllu(&mut self, text: &str) -> usize {
        let mut n = 0;
        for (id, b) in blocks(text) {
            self.ids.insert(id);
            self.texts.insert(norm_text(&block_text(&b)));
            n += 1;
        }
        n
    }

    /// Read the sources: a CoNLL-U file, or a Wikipedia corpus directory (`school/split.tsv` and
    /// `articles/<title>/ast.conllu`: every article of every split — `cv-a`, `cv-b` decide, `test` is held out).
    /// `required` sources must exist; others that do not are recorded as absent.
    pub fn load(sources: &[(String, bool)]) -> R<Heldout> {
        let mut h = Heldout::default();
        for (s, required) in sources {
            let p = Path::new(s);
            let split = p.join("school/split.tsv");
            let n = if split.is_file() {
                let titles: BTreeSet<String> = fs::read_to_string(&split)
                    .map_err(|e| format!("{}: {e}", split.display()))?
                    .lines()
                    .skip(1)
                    .filter_map(|l| l.split('\t').next().map(|t| t.replace(' ', "_")))
                    .collect();
                let mut n = 0;
                for t in titles {
                    let f = p.join("articles").join(&t).join("ast.conllu");
                    n += h.add_conllu(&fs::read_to_string(&f).map_err(|e| format!("{}: {e}", f.display()))?);
                }
                n
            } else if p.is_file() {
                h.add_conllu(&fs::read_to_string(p).map_err(|e| format!("{s}: {e}"))?)
            } else if *required {
                return Err(format!("held-out source {s} does not exist"));
            } else {
                h.sources.push((s.clone(), "absent".into()));
                continue;
            };
            h.sources.push((s.clone(), n.to_string()));
        }
        Ok(h)
    }

    /// Whether a sentence (its id and CoNLL-U block) is a development or held-out sentence.
    pub fn hits(&self, id: &str, block: &str) -> bool {
        self.ids.contains(id) || self.texts.contains(&norm_text(&block_text(block)))
    }

    /// The blocks of a CoNLL-U text that are not held out, and how many were dropped.
    pub fn filter(&self, text: &str) -> (String, usize) {
        let (mut keep, mut dropped) = (String::new(), 0);
        for (id, b) in blocks(text) {
            if self.hits(&id, &b) {
                dropped += 1;
            } else {
                keep.push_str(&b);
            }
        }
        (keep, dropped)
    }

    /// One line for the logs: the sources and their sizes.
    pub fn describe(&self) -> String {
        let s: Vec<String> = self.sources.iter().map(|(p, n)| format!("{p} ({n})")).collect();
        format!("{} held-out ids, {} texts from {}", self.ids.len(), self.texts.len(), s.join(", "))
    }
}

/// Copies of training files (CoNLL-U) without held-out sentences, in `dir` (`<i>/<file name>`: the file name is kept,
/// so `en`'s licence gate still reads the source from it), with each file's kept and dropped counts logged; what every
/// training input of the cycle passes through (`gparse train`, and the annotator's `CYCLE_BASE`).
pub fn filtered_inputs(held: &Heldout, files: &[String], dir: &Path) -> R<Vec<String>> {
    Ok(filtered_counts(held, files, dir)?.into_iter().map(|(o, _, _)| o).collect())
}

/// `filtered_inputs` with (copy, sentences kept, sentences dropped) per file.
pub fn filtered_counts(held: &Heldout, files: &[String], dir: &Path) -> R<Vec<(String, usize, usize)>> {
    let mut out = Vec::new();
    for (i, f) in files.iter().enumerate() {
        let text = fs::read_to_string(f).map_err(|e| format!("{f}: {e}"))?;
        let (keep, dropped) = held.filter(&text);
        let kept = blocks(&keep).len();
        let name = Path::new(f).file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        let sub = dir.join(i.to_string());
        fs::create_dir_all(&sub).map_err(|e| e.to_string())?;
        let o = sub.join(&name);
        fs::write(&o, keep).map_err(|e| e.to_string())?;
        eprintln!("held-out: {f}: {kept} kept, {dropped} dev/held-out sentences dropped → {}", o.display());
        out.push((o.display().to_string(), kept, dropped));
    }
    Ok(out)
}

/// The database's trees minus development and held-out sentences, as a training file for the GPU SLM
/// (`train-db.conllu` in the database).
pub fn training_file(db: &Path, held: &Heldout) -> R<PathBuf> {
    let out = db.join("train-db.conllu");
    training_file_at(db, held, &out)?;
    Ok(out)
}

/// `training_file` into a given path; returns (sentences written, held-out sentences dropped). The drop is logged.
pub fn training_file_at(db: &Path, held: &Heldout, out: &Path) -> R<(usize, usize)> {
    // the trees of every training database (`train_dbs`), this one's first
    let mut dbs = vec![db.to_path_buf()];
    dbs.extend(train_dbs().into_iter().filter(|d| d != db));
    training_file_from(&dbs, held, out)
}

/// The export over an explicit list of databases, the first one primary; an id already taken is not taken again.
pub fn training_file_from(dbs: &[PathBuf], held: &Heldout, out: &Path) -> R<(usize, usize)> {
    let mut seen = BTreeSet::new();
    let mut keep = String::new();
    let (mut n, mut dropped) = (0, 0);
    let strict = crate::reviewed::policy(&dbs[0])?.is_some();
    for d in dbs {
        crate::reviewed::ready(d)?;
        let allow = crate::reviewed::training_allow(d)?;
        if strict && allow.is_none() { return Err(format!("unreviewed secondary training DB {}", d.display())); }
        let text = fs::read_to_string(d.join("current.conllu")).map_err(|e| format!("{}: {e}", d.display()))?;
        for (id, b) in blocks(&text) {
            if !allow.as_ref().is_none_or(|a| a.contains(&id)) || seen.contains(&id) {
                continue;
            }
            if held.hits(&id, &b) {
                dropped += 1;
                continue;
            }
            seen.insert(id);
            keep.push_str(&b);
            n += 1;
        }
    }
    fs::write(out, keep).map_err(|e| e.to_string())?;
    eprintln!("train-db: {n} sentences → {}; {dropped} dev/held-out sentences dropped", out.display());
    Ok((n, dropped))
}

/// A fresh database: the seed corpus, then every finished round in order (the agreed trees after Haiku for the old
/// post rounds, Haiku's trees for the pre rounds).
pub fn init(db: &Path, seed: &Path, root: &Path) -> R<()> {
    if db.join("current.conllu").exists() {
        return Err(format!("{} already exists", db.join("current.conllu").display()));
    }
    let (a, _) = update(db, seed, "seed")?;
    eprintln!("db: seed {a} sentences from {}", seed.display());
    let mut rounds: Vec<(usize, PathBuf)> = fs::read_dir(root)
        .map_err(|e| e.to_string())?
        .filter_map(|e| e.ok())
        .filter_map(|e| e.file_name().to_str().and_then(|n| n.strip_prefix("round")).and_then(|k| k.parse().ok()).map(|k| (k, e.path())))
        .collect();
    rounds.sort();
    for (k, d) in rounds {
        let f = if d.join("verify/agreed.conllu").exists() { d.join("verify/agreed.conllu") } else { d.join("final.conllu") };
        if f.exists() {
            let (a, r) = update(db, &f, &format!("round{k}"))?;
            eprintln!("db: round {k}: +{a} new, {r} replaced");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drafts_add_only_new_sentences() {
        let db = "# sent_id = s1\n1\tA\ta\tX\tX\t_\t0\troot\t_\t_\n\n";
        let drafts = "# sent_id = s1\n1\tZ\tz\tX\tX\t_\t0\troot\t_\t_\n\n# sent_id = b:1\n1\tB\tb\tX\tX\t_\t0\troot\t_\t_\n\n";
        let w = with_drafts(db, drafts);
        assert!(w.contains("b:1") && w.contains("\tA\t"));
        // negative control: the database's version wins over the draft of a known sentence
        assert!(!w.contains("\tZ\t"));
    }

    #[test]
    fn replace_and_append() {
        let d = std::env::temp_dir().join(format!("parsedb-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        let a = d.join("a.conllu");
        fs::create_dir_all(&d).unwrap();
        // the training file joins every training database (`train_dbs`); point them all at the test's own, never at
        // the machine's real parse database
        std::env::set_var("CYCLE_DB", d.join("db"));
        std::env::remove_var("CYCLE_TRAIN_DBS");
        fs::write(&a, "# sent_id = s1\n1\tA\ta\tX\tX\t_\t0\troot\t_\t_\n\n# sent_id = s2\n1\tB\tb\tX\tX\t_\t0\troot\t_\t_\n\n").unwrap();
        assert_eq!(update(&d.join("db"), &a, "seed").unwrap(), (2, 0));
        fs::write(&a, "# sent_id = s2\n1\tB2\tb\tX\tX\t_\t0\troot\t_\t_\n\n# sent_id = s3\n1\tC\tc\tX\tX\t_\t0\troot\t_\t_\n\n").unwrap();
        assert_eq!(update(&d.join("db"), &a, "round1").unwrap(), (1, 1));
        let cur = fs::read_to_string(d.join("db/current.conllu")).unwrap();
        assert_eq!(ids_of(&cur), vec!["s1", "s2", "s3"]);
        assert!(cur.contains("B2") && !cur.contains("\tB\t"));
        // negative control: held-out ids stay out of the training file
        let ex = Heldout::of_ids(["s2".to_string()].into());
        let t = fs::read_to_string(training_file(&d.join("db"), &ex).unwrap()).unwrap();
        assert_eq!(ids_of(&t), vec!["s1", "s3"]);
        assert_eq!(fs::read_to_string(d.join("db/log.tsv")).unwrap().lines().count(), 4);
        fs::remove_dir_all(&d).ok();
    }
}
#[cfg(test)]
mod vote_tests {
    use super::*;

    fn tree(head2: &str, rel2: &str, misc1: &str) -> String {
        format!("1\tA\ta\tX\tX\t_\t0\troot\t_\t{misc1}\n2\tB\tb\tX\tX\t_\t{head2}\t{rel2}\t_\t_\n3\tC\tc\tX\tX\t_\t1\tdep\t_\t_\n\n")
    }

    #[test]
    fn vote_shares_go_to_misc() {
        let mut m: BTreeMap<(String, String), (u32, u64, usize)> = BTreeMap::new();
        m.insert(("1".into(), "obj".into()), (8, 0, 0));
        m.insert(("1".into(), "iobj".into()), (2, 0, 0));
        m.insert(("3".into(), "dep".into()), (2, 0, 0));
        assert_eq!(vote_misc(&m, "1", "obj"), "Vote=0.67|VoteHead=0.83|VoteAlt=3:0.17");
        assert_eq!(vote_misc(&BTreeMap::new(), "1", "obj"), "");
        assert_eq!(with_vote("SpaceAfter=No|Vote=0.5|VoteAlt=2:0.5", "Vote=1.00|VoteHead=1.00"), "SpaceAfter=No|Vote=1.00|VoteHead=1.00");
        assert_eq!(with_vote("_", "Vote=1.00|VoteHead=1.00"), "Vote=1.00|VoteHead=1.00");
        assert_eq!(with_vote("Vote=0.2", ""), "_");
    }

    /// db-agree keeps the tree it kept before and adds the margin; a second run gives the same file; a sentence with one
    /// version gets no vote (it is trusted as it is).
    #[test]
    fn agree_writes_the_margin_and_is_stable() {
        let d = std::env::temp_dir().join(format!("parsedb-vote-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(&d).unwrap();
        let db = d.join("db");
        let f = d.join("in.conllu");
        let one = "# sent_id = solo\n1\tZ\tz\tX\tX\t_\t0\troot\t_\t_\n\n";
        fs::write(&f, format!("# sent_id = s\n{}{one}", tree("1", "obj", "SpaceAfter=No"))).unwrap();
        update(&db, &f, "seed").unwrap();
        fs::write(&f, format!("# sent_id = s\n{}", tree("3", "dep", "_"))).unwrap();
        update(&db, &f, "round1").unwrap();
        fs::write(&f, format!("# sent_id = s\n{}", tree("1", "obj", "_"))).unwrap();
        update(&db, &f, "round2").unwrap();
        // negative control: off, the database is as before (no vote anywhere)
        assert_eq!(agree_with(&db, false).unwrap(), (1, 0));
        let before = fs::read_to_string(db.join("current.conllu")).unwrap();
        assert!(!before.contains("Vote"));
        assert_eq!(agree_with(&db, true).unwrap(), (1, 0));
        let cur = fs::read_to_string(db.join("current.conllu")).unwrap();
        // the same trees: only MISC gained the vote
        let strip = |t: &str| -> Vec<String> {
            t.lines().map(|l| {
                let mut c: Vec<String> = l.split('\t').map(String::from).collect();
                if c.len() == 10 {
                    c[9] = with_vote(&c[9], "");
                }
                c.join("\t")
            }).collect()
        };
        assert_eq!(strip(&cur), strip(&before));
        // round2 (2) + seed (1) for 1/obj against round1 (2) for 3/dep
        assert!(cur.contains("2\tB\tb\tX\tX\t_\t1\tobj\t_\tVote=0.60|VoteHead=0.60|VoteAlt=3:0.40\n"), "{cur}");
        assert!(cur.contains("\troot\t_\tVote=1.00|VoteHead=1.00\n"), "{cur}");
        assert!(cur.contains("1\tZ\tz\tX\tX\t_\t0\troot\t_\t_\n"), "a single version gets no vote");
        agree_with(&db, true).unwrap();
        assert_eq!(fs::read_to_string(db.join("current.conllu")).unwrap(), cur, "a second agreement changes nothing");
        fs::remove_dir_all(&d).ok();
    }
}

#[cfg(test)]
mod wave33_tests {
    use super::*;
    #[test]
    fn voting_boundaries_and_snapshot_drafts() {
        for (s, w) in [
            ("round1-opus", 8),
            ("round2-sonnet", 4),
            ("round3", 2),
            ("seed", 1),
            ("opus", 1),
        ] {
            assert_eq!(weight(s), w);
        }
        for (h, ok) in [
            (vec![], false),
            (vec![0], true),
            (vec![0, 1], true),
            (vec![0, 0], false),
            (vec![2, 1], false),
            (vec![0, 3, 2], false),
            (vec![4, 0], false),
        ] {
            assert_eq!(tree_ok(&h), ok, "{h:?}");
        }
        let draft = "# sent_id = a\n\n# sent_id = a\n\n";
        assert_eq!(ids_of(&with_drafts("", draft)), vec!["a", "a"]);
        assert_eq!(with_drafts("", ""), "\n\n");
    }
}

#[cfg(test)]
mod heldout_tests {
    use super::*;

    fn sent(id: &str, text: &str) -> String {
        let words: Vec<&str> = text.split(' ').collect();
        let mut b = format!("# sent_id = {id}\n# text = {text}\n");
        for (i, w) in words.iter().enumerate() {
            b.push_str(&format!("{}\t{w}\t{w}\tX\tX\t_\t{}\t{}\t_\t_\n", i + 1, if i == 0 { 0 } else { 1 }, if i == 0 { "root" } else { "dep" }));
        }
        b.push('\n');
        b
    }

    #[test]
    fn normalized_text_ignores_case_spacing_and_punctuation() {
        assert_eq!(norm_text("The cat sat."), norm_text("the  cat sat !"));
        assert_eq!(norm_text("\u{201c}Yes,\u{201d} she said."), norm_text("\"yes\" she said"));
        assert_ne!(norm_text("The cat sat."), norm_text("The cat sat down."));
        assert_eq!(norm_text("..."), "...");
        assert_eq!(block_text("# sent_id = a\n1\tHi\thi\tX\tX\t_\t0\troot\t_\t_\n2\tyou\tyou\tX\tX\t_\t1\tdep\t_\t_\n\n"), "Hi you");
    }

    /// The negative control of gparse-leak (2026-11-05): a development sentence that reaches the training export, under
    /// its own id or under another id with the same normalized text, turns this red.
    #[test]
    fn dev_sentences_never_reach_the_training_export() {
        let d = std::env::temp_dir().join(format!("parsedb-heldout-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(&d).unwrap();
        // explicit database list: other tests change CYCLE_DB concurrently
        let dbs = [d.join("db")];
        let dev = d.join("dev.conllu");
        fs::write(&dev, format!("{}{}", sent("school:dev:1", "The fox ran home ."), sent("ewt:dev:9", "Thanks a lot !"))).unwrap();
        let seed = d.join("seed.conllu");
        // s1 trains; school:dev:1 is the dev id itself; book:7 is the dev text under another id, spaced and cased
        // differently; s4 differs by a word and stays
        fs::write(&seed, format!("{}{}{}{}", sent("s1", "A cat sat ."), sent("school:dev:1", "The fox ran home ."), sent("book:7", "the fox ran home"), sent("s4", "The fox ran home again ."))).unwrap();
        update(&d.join("db"), &seed, "seed").unwrap();
        let held = Heldout::load(&[(dev.display().to_string(), true), (d.join("absent.conllu").display().to_string(), false)]).unwrap();
        assert_eq!((held.ids.len(), held.texts.len()), (2, 2));
        assert_eq!(held.sources[1].1, "absent");
        let out = d.join("train.conllu");
        assert_eq!(training_file_from(&dbs, &held, &out).unwrap(), (2, 2));
        let t = fs::read_to_string(&out).unwrap();
        assert_eq!(ids_of(&t), vec!["s1", "s4"]);
        for (id, b) in blocks(&t) {
            assert!(!held.hits(&id, &b), "dev sentence {id} reached the training export");
        }
        // the control can show red: without the exclusion both dev sentences reach the export
        let open = d.join("open.conllu");
        assert_eq!(training_file_from(&dbs, &Heldout::default(), &open).unwrap(), (4, 0));
        let leaked = blocks(&fs::read_to_string(&open).unwrap()).into_iter().filter(|(id, b)| held.hits(id, b)).count();
        assert_eq!(leaked, 2);
        // a required source that is missing stops the export instead of weakening it
        assert!(Heldout::load(&[(d.join("absent.conllu").display().to_string(), true)]).is_err());
        // the same filter guards the other training inputs (base files, GP_ALWAYS)
        let f = filtered_inputs(&held, &[seed.display().to_string()], &d.join("inputs")).unwrap();
        assert_eq!(ids_of(&fs::read_to_string(&f[0]).unwrap()), vec!["s1", "s4"]);
        // the copy keeps the file name (the licence gate of `en` reads the source from it)
        assert!(f[0].ends_with("/0/seed.conllu"), "{}", f[0]);
        fs::remove_dir_all(&d).ok();
    }

    #[test]
    fn wikipedia_corpus_contributes_every_split() {
        let d = std::env::temp_dir().join(format!("wiki-heldout-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(d.join("school")).unwrap();
        fs::write(d.join("school/split.tsv"), "title\tclass\tsplit\nNew York\tcity\ttest\nNew York\tcity\ttest\nOslo\tcity\tcv-a\n").unwrap();
        for (t, s) in [("New_York", "New York is big ."), ("Oslo", "Oslo is cold .")] {
            fs::create_dir_all(d.join("articles").join(t)).unwrap();
            fs::write(d.join("articles").join(t).join("ast.conllu"), sent(&format!("wiki:{t}:1"), s)).unwrap();
        }
        let h = Heldout::load(&[(d.display().to_string(), true)]).unwrap();
        assert_eq!(h.sources[0].1, "2");
        assert!(h.hits("other", &sent("other", "new york is big")));
        fs::remove_dir_all(&d).ok();
    }
}
