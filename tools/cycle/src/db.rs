//! The parse database (15.10): one best tree per sentence, outside the repo (`CYCLE_DB`, default
//! `$MOVA_BIG/parsedb`, $MOVA_BIG defaults to /data/mova). `current.conllu` holds the trees, `log.tsv` how each one got there (time, sentence,
//! source, added or replaced). Every Haiku result improves it; the GPU MMM trains on it.

use std::collections::BTreeMap;
use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};

use crate::{ids_of, R};

pub fn dir() -> PathBuf {
    PathBuf::from(std::env::var("CYCLE_DB").unwrap_or_else(|_| format!("{}/parsedb", std::env::var("MOVA_BIG").unwrap_or_else(|_| "/data/mova".into()))))
}

/// Blocks of a CoNLL-U text keyed by sentence id, in file order.
fn blocks(text: &str) -> Vec<(String, String)> {
    text.split("\n\n")
        .filter(|b| !b.trim().is_empty())
        .filter_map(|b| ids_of(b).into_iter().next().map(|id| (id, format!("{}\n\n", b.trim_end()))))
        .collect()
}

/// Put the trees of `file` into the database: a known sentence is replaced, a new one appended.
pub fn update(db: &Path, file: &Path, source: &str) -> R<(usize, usize)> {
    fs::create_dir_all(db).map_err(|e| e.to_string())?;
    let cur = db.join("current.conllu");
    let mut order: Vec<(String, String)> = blocks(&fs::read_to_string(&cur).unwrap_or_default());
    let mut at: BTreeMap<String, usize> = order.iter().enumerate().map(|(i, (id, _))| (id.clone(), i)).collect();
    let (mut added, mut replaced) = (0, 0);
    let mut log = String::new();
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    for (id, b) in blocks(&fs::read_to_string(file).map_err(|e| format!("{}: {e}", file.display()))?) {
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
    // every version is kept (design note, 15.10: the MMM sees many parses of a sentence, from different models at different
    // times, and agrees them): `history.conllu`, each block marked with its source and time
    let mut hist = String::new();
    for (_, b) in blocks(&fs::read_to_string(file).map_err(|e| e.to_string())?) {
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
fn tree_ok(heads: &[usize]) -> bool {
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
    let hist = fs::read_to_string(db.join("history.conllu")).map_err(|e| e.to_string())?;
    // id → versions (weight, time, order, block)
    let mut vs: BTreeMap<String, Vec<(u32, u64, usize, String)>> = BTreeMap::new();
    for (k, b) in hist.split("\n\n").filter(|b| !b.trim().is_empty()).enumerate() {
        let Some(v) = b.lines().find_map(|l| l.strip_prefix("# version = ")) else { continue };
        let (src, when) = v.rsplit_once(' ').unwrap_or((v, "0"));
        let Some(id) = ids_of(b).into_iter().next() else { continue };
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
        if !tree_ok(&heads) {
            *b = best.3.clone();
            continue;
        }
        for (c, (h, r)) in base.iter_mut().zip(&pick) {
            if (&c[6], &c[7]) != (h, r) {
                moved += 1;
                c[6] = h.clone();
                c[7] = r.clone();
            }
        }
        let comments: String = best.3.lines().filter(|l| l.starts_with('#')).map(|l| format!("{l}\n")).collect();
        *b = format!("{comments}{}\n", base.iter().map(|c| format!("{}\n", c.join("\t"))).collect::<String>());
    }
    let tmp = db.join("current.conllu.tmp");
    fs::write(&tmp, order.iter().map(|(_, b)| b.as_str()).collect::<String>()).map_err(|e| e.to_string())?;
    fs::rename(&tmp, &cur).map_err(|e| e.to_string())?;
    Ok((multi, moved))
}

/// The pragmatic reading next to the trees: `prag.tsv` (sent_id + fields + what the teacher changed), one row per
/// sentence, the newest wins.
pub fn update_prag(db: &Path, file: &Path) -> R<usize> {
    let cur = db.join("prag.tsv");
    let text = fs::read_to_string(&cur).unwrap_or_default();
    let new = fs::read_to_string(file).map_err(|e| e.to_string())?;
    let head = new.lines().next().unwrap_or("").to_string();
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

/// The database's trees minus held-out sentences, as a training file for the GPU MMM.
pub fn training_file(db: &Path, exclude: &std::collections::BTreeSet<String>) -> R<PathBuf> {
    let out = db.join("train-db.conllu");
    let text = fs::read_to_string(db.join("current.conllu")).map_err(|e| e.to_string())?;
    let keep: String = blocks(&text).into_iter().filter(|(id, _)| !exclude.contains(id)).map(|(_, b)| b).collect();
    fs::write(&out, keep).map_err(|e| e.to_string())?;
    Ok(out)
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
    fn replace_and_append() {
        let d = std::env::temp_dir().join(format!("parsedb-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        let a = d.join("a.conllu");
        fs::create_dir_all(&d).unwrap();
        fs::write(&a, "# sent_id = s1\n1\tA\ta\tX\tX\t_\t0\troot\t_\t_\n\n# sent_id = s2\n1\tB\tb\tX\tX\t_\t0\troot\t_\t_\n\n").unwrap();
        assert_eq!(update(&d.join("db"), &a, "seed").unwrap(), (2, 0));
        fs::write(&a, "# sent_id = s2\n1\tB2\tb\tX\tX\t_\t0\troot\t_\t_\n\n# sent_id = s3\n1\tC\tc\tX\tX\t_\t0\troot\t_\t_\n\n").unwrap();
        assert_eq!(update(&d.join("db"), &a, "round1").unwrap(), (1, 1));
        let cur = fs::read_to_string(d.join("db/current.conllu")).unwrap();
        assert_eq!(ids_of(&cur), vec!["s1", "s2", "s3"]);
        assert!(cur.contains("B2") && !cur.contains("\tB\t"));
        // negative control: held-out ids stay out of the training file
        let ex: std::collections::BTreeSet<String> = ["s2".to_string()].into();
        let t = fs::read_to_string(training_file(&d.join("db"), &ex).unwrap()).unwrap();
        assert_eq!(ids_of(&t), vec!["s1", "s3"]);
        assert_eq!(fs::read_to_string(d.join("db/log.tsv")).unwrap().lines().count(), 4);
        fs::remove_dir_all(&d).ok();
    }
}
