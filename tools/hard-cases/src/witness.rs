//! The three open parsers the Eel's tree agreement (`treeagree agree`) needs — Stanza, Trankit and
//! spaCy trf — run on a track's harvested training trees with the existing neural runner, under the
//! shared GPU lease. The runner's output has no `# text`; it is attached from the reference only
//! after the id and every token form agree (the same adapter the Eel's database export used).
//! Records any parser failed to align with are listed as missing and left out of the agreed input:
//! the engine never manufactures a witness.
//!
//! Writes `<lane_root>/learn/<track>/`: `agreed-input.conllu`, `parsers/out/<parser>.conllu`,
//! `witness.json` (bound to the training file's SHA-256: an unchanged file is not run again).
use crate::config::Config;
use crate::{read_json, sha, write_atomic};
use anyhow::{bail, ensure, Context, Result};
use serde_json::{json, Value};
use std::os::unix::io::AsRawFd;
use std::path::Path;
use std::process::Command;
use std::time::{Duration, Instant};

pub const PARSERS: [&str; 3] = ["stanza", "trankit", "spacy-trf"];

/// The forms of a block, in order.
pub fn forms(b: &str) -> Vec<String> {
    crate::rows(b).iter().map(|c| c[1].to_string()).collect()
}

/// `in/ids.txt` and `in/tokens.txt` for the runner.
pub fn runner_input(blocks: &[String], dir: &Path) -> Result<()> {
    for d in ["in", "out", "log"] {
        std::fs::create_dir_all(dir.join(d))?;
    }
    let ids: String = blocks.iter().map(|b| format!("{}\n", crate::block_id(b).unwrap_or(""))).collect();
    let toks: String = blocks.iter().map(|b| forms(b).join(" ") + "\n").collect();
    std::fs::write(dir.join("in/ids.txt"), ids)?;
    std::fs::write(dir.join("in/tokens.txt"), toks)?;
    Ok(())
}

/// The witness blocks aligned to the reference: (aligned blocks with `# text` attached, by index;
/// ids of the records that do not align).
pub fn align(reference: &[String], witness: &str) -> (Vec<Option<String>>, Vec<String>) {
    let w = crate::blocks(witness);
    let mut out = vec![];
    let mut missing = vec![];
    for (i, r) in reference.iter().enumerate() {
        let id = crate::block_id(r).unwrap_or("");
        match w.get(i) {
            Some(b) if crate::block_id(b) == Some(id) && forms(b) == forms(r) => {
                let text = crate::block_text(r).unwrap_or("");
                let mut s = String::new();
                for l in b.lines() {
                    s += l;
                    s.push('\n');
                    if l.starts_with("# sent_id = ") && crate::block_text(b).is_none() {
                        s += &format!("# text = {text}\n");
                    }
                }
                out.push(Some(s));
            }
            _ => {
                out.push(None);
                missing.push(id.to_string());
            }
        }
    }
    (out, missing)
}

/// Holds the shared GPU lease (an exclusive `flock`), waiting for it up to `wait`.
pub fn gpu_lease(path: &Path, wait: Duration) -> Result<std::fs::File> {
    let f = std::fs::OpenOptions::new().create(true).truncate(false).write(true).open(path)?;
    let t0 = Instant::now();
    loop {
        if unsafe { libc::flock(f.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } == 0 {
            return Ok(f);
        }
        if t0.elapsed() > wait {
            bail!("GPU lease {} still held after {} s", path.display(), wait.as_secs());
        }
        std::thread::sleep(Duration::from_secs(5));
    }
}

pub fn run(cfg: &Config, track: &str) -> Result<Value> {
    let w = cfg.witness.as_ref().context("the configuration has no witness section")?;
    let dir = cfg.lane_root.join("learn").join(track);
    let train = std::fs::read_to_string(dir.join("train.conllu")).unwrap_or_default();
    let reference = crate::blocks(&train);
    if reference.is_empty() {
        return Ok(json!({"track": track, "skipped": "no training trees"}));
    }
    let train_sha = sha(train.as_bytes());
    if let Ok(v) = read_json(&dir.join("witness.json")) {
        if v["train_sha256"] == train_sha.as_str() {
            return Ok(v);
        }
    }
    let run = dir.join(format!("witness-run-{}", &train_sha[..16]));
    let _ = std::fs::remove_dir_all(&run);
    runner_input(&reference, &run)?;
    {
        let _lease = gpu_lease(&w.gpu_lock, Duration::from_secs(w.lease_wait_seconds))?;
        for p in PARSERS {
            let py = if p == "trankit" { &w.venv310 } else { &w.venv };
            let out = Command::new(py).arg(&w.neural).arg(p).arg(&run).output().with_context(|| format!("{p}"))?;
            std::fs::write(run.join("log").join(format!("{p}.log")), [out.stdout, out.stderr].concat())?;
            ensure!(out.status.success(), "{p} failed (log {})", run.join("log").join(format!("{p}.log")).display());
        }
    }
    let mut aligned = vec![];
    let mut missing = std::collections::BTreeSet::new();
    for p in PARSERS {
        let (a, m) = align(&reference, &std::fs::read_to_string(run.join("out").join(format!("{p}.conllu")))?);
        missing.extend(m);
        aligned.push(a);
    }
    let keep: Vec<usize> = (0..reference.len()).filter(|&i| aligned.iter().all(|a| a[i].is_some())).collect();
    let agreed: String = keep.iter().map(|&i| reference[i].clone() + "\n").collect();
    write_atomic(&dir.join("agreed-input.conllu"), agreed.as_bytes())?;
    let mut parsers = serde_json::Map::new();
    for (p, a) in PARSERS.iter().zip(&aligned) {
        let s: String = keep.iter().map(|&i| a[i].clone().unwrap() + "\n").collect();
        let f = dir.join("parsers/out").join(format!("{p}.conllu"));
        write_atomic(&f, s.as_bytes())?;
        parsers.insert(p.to_string(), json!(sha(s.as_bytes())));
    }
    let v = json!({"track": track, "train_sha256": train_sha, "agreed_input_sha256": sha(agreed.as_bytes()), "records": reference.len(), "fully_witnessed": keep.len(), "missing": missing, "parsers": parsers, "run": run});
    write_atomic(&dir.join("witness.json"), &serde_json::to_vec_pretty(&v)?)?;
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_is_attached_only_to_aligned_records() {
        let r = vec!["# sent_id = hard:a:1\n# text = Dogs bark.\n1\tDogs\t_\t_\t_\t_\t2\tnsubj\t_\t_\n2\tbark.\t_\t_\t_\t_\t0\troot\t_\t_\n".to_string(), "# sent_id = hard:a:2\n# text = Hi\n1\tHi\t_\t_\t_\t_\t0\troot\t_\t_\n".to_string()];
        // the witness split «bark.» differently in the first record, and agrees on the second
        let w = "# sent_id = hard:a:1\n1\tDogs\t_\tNOUN\tNNS\t_\t2\tnsubj\t_\t_\n2\tbark\t_\tVERB\tVBP\t_\t0\troot\t_\t_\n3\t.\t_\tPUNCT\t.\t_\t2\tpunct\t_\t_\n\n# sent_id = hard:a:2\n1\tHi\t_\tINTJ\tUH\t_\t0\troot\t_\t_\n\n";
        let (a, m) = align(&r, w);
        assert_eq!(m, vec!["hard:a:1".to_string()]);
        assert!(a[0].is_none());
        assert_eq!(a[1].as_deref(), Some("# sent_id = hard:a:2\n# text = Hi\n1\tHi\t_\tINTJ\tUH\t_\t0\troot\t_\t_\n"));
        // a witness with the wrong id never aligns
        let (_, m) = align(&r[1..], "# sent_id = hard:a:9\n1\tHi\t_\tINTJ\tUH\t_\t0\troot\t_\t_\n");
        assert_eq!(m.len(), 1);
    }
}
