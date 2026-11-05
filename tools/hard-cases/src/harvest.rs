//! Validated Fish replies of the lane's waves → review-only silver trees per track and split.
//!
//! A reply counts only when the pager's outer receipt and the Fish executor's inner receipt both say
//! `validated_review_only` (`review_only: true`, `training_authorized: false`), the reply binds the
//! job's task, and each target's tree keeps the target's id and text. A target annotated twice keeps
//! its first validated reply (waves in order). Held-out texts are dropped again.
//!
//! Writes `<lane_root>/learn/`: `annotations.jsonl` (one row per target, sorted), and per track
//! (`shipped`, `internal`, `measure`) and split (`train`, `dev`, `measure`) `<track>/<split>.conllu`
//! and `<track>/<split>-prag.tsv`, plus `manifest.json`. Rewritten whole each time; deterministic
//! for the same pager outputs.
use crate::config::Config;
use crate::wave::Selected;
use crate::{read_json, sha, write_atomic};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Annotation {
    pub target: String,
    pub key: String,
    pub wave: String,
    pub job: String,
    pub track: String,
    pub split: String,
    pub source: String,
    pub source_id: String,
    pub licence: String,
    /// The validated tree, with its id and text.
    pub tree: String,
    /// The validated pragmatics row (`sent_id form act indirect hedge polarity voice teacher`).
    pub prag: String,
    pub unresolved: usize,
    pub receipt_sha256: String,
}

pub const PRAG_HEADER: &str = "sent_id\tform\tact\tindirect\thedge\tpolarity\tvoice\tteacher\n";

/// The jobs under a pager root: `pages/<intent>/run/jobs/<job>/`.
fn jobs(root: &Path) -> Vec<std::path::PathBuf> {
    let mut out = vec![];
    let Ok(pages) = std::fs::read_dir(root.join("pages")) else { return out };
    let mut pages: Vec<_> = pages.filter_map(|e| e.ok()).map(|e| e.path()).collect();
    pages.sort();
    for p in pages {
        let Ok(js) = std::fs::read_dir(p.join("run/jobs")) else { continue };
        let mut js: Vec<_> = js.filter_map(|e| e.ok()).map(|e| e.path()).collect();
        js.sort();
        out.extend(js);
    }
    out
}

/// One job's validated (target, tree, prag row), or why not.
pub fn read_job(job: &Path) -> Result<(Value, Vec<(String, String, String, usize)>)> {
    let outer = read_json(&job.join("receipt.json"))?;
    anyhow::ensure!(outer["status"] == "validated_review_only" && outer["review_only"] == true && outer["training_authorized"] == false, "outer receipt not validated_review_only");
    let inner = read_json(&job.join("output/receipt.json"))?;
    anyhow::ensure!(inner["status"] == "validated_review_only" && inner["training_authorized"] == false, "inner receipt not validated_review_only");
    let task = read_json(&job.join("task.json"))?;
    let reply = read_json(&job.join("output/reply.json"))?;
    anyhow::ensure!(reply["task_id"] == task["id"], "reply binds another task");
    let texts: BTreeMap<String, String> = task["context"].as_array().into_iter().flatten().filter_map(|c| Some((c["id"].as_str()?.to_string(), c["text"].as_str()?.to_string()))).collect();
    let trees: BTreeMap<String, String> = crate::blocks(reply["ast_conllu"].as_str().unwrap_or("")).into_iter().filter_map(|b| Some((crate::block_id(&b)?.to_string(), b))).collect();
    let prags: BTreeMap<String, String> = reply["pragmatics_tsv"].as_str().unwrap_or("").lines().skip(1).filter_map(|l| Some((l.split('\t').next()?.to_string(), l.to_string()))).collect();
    let unresolved = reply["unresolved"].as_array().map_or(0, Vec::len);
    let mut out = vec![];
    for t in task["target_ids"].as_array().into_iter().flatten().filter_map(|x| x.as_str()) {
        let (Some(tree), Some(row)) = (trees.get(t), prags.get(t)) else { anyhow::bail!("reply lacks target {t}") };
        anyhow::ensure!(crate::block_text(tree) == texts.get(t).map(String::as_str), "target {t}: text differs from its context line");
        out.push((t.to_string(), tree.clone(), row.clone(), unresolved));
    }
    Ok((outer, out))
}

/// Every wave's selection, by target.
pub fn selections(lane: &Path) -> Result<BTreeMap<String, Selected>> {
    let mut m = BTreeMap::new();
    for s in crate::wave::ledger(lane)? {
        if s.status == "queued" {
            m.entry(s.target.clone()).or_insert(s);
        }
    }
    Ok(m)
}

pub fn harvest(cfg: &Config) -> Result<Value> {
    let lane = &cfg.lane_root;
    let sel = selections(lane)?;
    let held = crate::wave::heldout(cfg)?;
    let mut got: BTreeMap<String, Annotation> = BTreeMap::new();
    let (mut jobs_seen, mut jobs_valid, mut dropped) = (0, 0, 0);
    let mut not_valid: BTreeMap<String, usize> = BTreeMap::new();
    let mut waves: Vec<_> = std::fs::read_dir(lane).map(|d| d.filter_map(|e| e.ok()).map(|e| e.path()).filter(|p| p.join("wave.json").exists()).collect()).unwrap_or_default();
    waves.sort();
    for w in waves {
        let wj = read_json(&w.join("wave.json"))?;
        for pc in wj["pagers"].as_array().into_iter().flatten().filter_map(|x| x.as_str()) {
            let root = read_json(Path::new(pc))?["root"].as_str().unwrap_or("").to_string();
            for job in jobs(Path::new(&root)) {
                jobs_seen += 1;
                match read_job(&job) {
                    Err(e) => *not_valid.entry(format!("{e:#}").chars().take(80).collect()).or_default() += 1,
                    Ok((outer, rows)) => {
                        jobs_valid += 1;
                        for (t, tree, row, unresolved) in rows {
                            let Some(s) = sel.get(&t) else { dropped += 1; continue };
                            let text = crate::block_text(&tree).unwrap_or("");
                            if held.0.contains(&sha(text)) || held.1.contains(&crate::norm_sha(text)) {
                                dropped += 1;
                                continue;
                            }
                            got.entry(t.clone()).or_insert(Annotation {
                                target: t,
                                key: s.key.clone(),
                                wave: s.wave.clone(),
                                job: job.file_name().unwrap_or_default().to_string_lossy().into_owned(),
                                track: s.track.clone(),
                                split: s.split.clone(),
                                source: s.source.clone(),
                                source_id: s.source_id.clone(),
                                licence: s.licence.clone(),
                                tree,
                                prag: row,
                                unresolved,
                                receipt_sha256: sha(serde_json::to_vec(&outer)?),
                            });
                        }
                    }
                }
            }
        }
    }
    write(lane, &got)?;
    let mut by: BTreeMap<String, usize> = BTreeMap::new();
    for a in got.values() {
        *by.entry(format!("{}/{}", a.track, a.split)).or_default() += 1;
    }
    Ok(json!({"jobs": jobs_seen, "validated_jobs": jobs_valid, "not_validated": not_valid, "annotations": got.len(), "by_track_split": by, "dropped": dropped, "learn": lane.join("learn")}))
}

/// Writes the learn directory from the annotations.
pub fn write(lane: &Path, got: &BTreeMap<String, Annotation>) -> Result<()> {
    let d = lane.join("learn");
    let rows: String = got.values().map(|a| serde_json::to_string(a).unwrap() + "\n").collect();
    write_atomic(&d.join("annotations.jsonl"), rows.as_bytes())?;
    let mut files = BTreeMap::new();
    for track in crate::policy::TRACKS {
        for split in ["train", "dev", "measure"] {
            let mine: Vec<&Annotation> = got.values().filter(|a| a.track == track && a.split == split).collect();
            let conllu: String = mine
                .iter()
                .map(|a| {
                    let mut b = String::new();
                    for l in a.tree.lines() {
                        b += l;
                        b.push('\n');
                        if l.starts_with("# text = ") {
                            b += &format!("# hard_case = {} {} {}\n", a.key, a.source, a.wave);
                        }
                    }
                    b + "\n"
                })
                .collect();
            let prag = String::from(PRAG_HEADER) + &mine.iter().map(|a| a.prag.clone() + "\n").collect::<String>();
            let p = d.join(track).join(format!("{split}.conllu"));
            write_atomic(&p, conllu.as_bytes())?;
            write_atomic(&d.join(track).join(format!("{split}-prag.tsv")), prag.as_bytes())?;
            files.insert(format!("{track}/{split}"), json!({"trees": mine.len(), "sha256": sha(conllu.as_bytes())}));
        }
    }
    write_atomic(&d.join("manifest.json"), &serde_json::to_vec_pretty(&json!({"schema": "mova-hard-learn-v1", "annotations": got.len(), "files": files, "review_only": true, "training_authorized": false, "semantic_gold": false}))?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn job(dir: &Path, status: &str, text_ok: bool) -> std::path::PathBuf {
        let j = dir.join("pages/h0001-p0000/run/jobs/bjob");
        std::fs::create_dir_all(j.join("output")).unwrap();
        std::fs::write(j.join("receipt.json"), json!({"status": status, "review_only": true, "training_authorized": false}).to_string()).unwrap();
        std::fs::write(j.join("output/receipt.json"), json!({"status": "validated_review_only", "training_authorized": false}).to_string()).unwrap();
        std::fs::write(j.join("task.json"), json!({"id": "bjob", "target_ids": ["hard:abc:2"], "context": [{"id": "hard:abc:1", "text": "Before."}, {"id": "hard:abc:2", "text": "Dogs bark."}]}).to_string()).unwrap();
        let text = if text_ok { "Dogs bark." } else { "Cats bark." };
        let tree = format!("# sent_id = hard:abc:2\n# text = {text}\n1\tDogs\tdog\tNOUN\tNNS\t_\t2\tnsubj\t_\t_\n2\tbark\tbark\tVERB\tVBP\t_\t0\troot\t_\tSpaceAfter=No\n3\t.\t.\tPUNCT\t.\t_\t2\tpunct\t_\t_\n");
        std::fs::write(j.join("output/reply.json"), json!({"task_id": "bjob", "ast_conllu": tree, "pragmatics_tsv": format!("{PRAG_HEADER}hard:abc:2\tdecl\tassert\t0\t0\tnone\tnarrator\tt\n"), "unresolved": ["x"]}).to_string()).unwrap();
        j
    }

    #[test]
    fn only_bound_validated_replies_count() {
        let d = std::env::temp_dir().join(format!("hard-harvest-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        let j = job(&d, "validated_review_only", true);
        let (_, rows) = read_job(&j).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].0, "hard:abc:2");
        assert!(rows[0].2.starts_with("hard:abc:2\tdecl"));
        // red: a quarantined job, and a tree whose text is not its target's
        let j = job(&d, "completed_quarantined", true);
        assert!(read_job(&j).is_err());
        let j = job(&d, "validated_review_only", false);
        assert!(read_job(&j).unwrap_err().to_string().contains("text differs"));
        let _ = std::fs::remove_dir_all(d);
    }

    #[test]
    fn learn_files_split_by_track_and_split() {
        let d = std::env::temp_dir().join(format!("hard-learn-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        let a = |t: &str, track: &str, split: &str| Annotation { target: t.into(), key: "k".into(), wave: "wave-0001".into(), job: "b".into(), track: track.into(), split: split.into(), source: "wikipedia".into(), source_id: "wiki:X:1".into(), licence: "CC BY-SA 4.0".into(), tree: format!("# sent_id = {t}\n# text = A\n1\tA\ta\tDET\tDT\t_\t0\troot\t_\t_\n"), prag: format!("{t}\tdecl\tassert\t0\t0\tnone\tnarrator\tt"), unresolved: 0, receipt_sha256: "0".repeat(64) };
        let got: BTreeMap<String, Annotation> = [a("hard:a:1", "internal", "train"), a("hard:b:1", "internal", "dev"), a("hard:c:1", "shipped", "train")].into_iter().map(|x| (x.target.clone(), x)).collect();
        write(&d, &got).unwrap();
        let t = std::fs::read_to_string(d.join("learn/internal/train.conllu")).unwrap();
        assert!(t.contains("# sent_id = hard:a:1\n# text = A\n# hard_case = k wikipedia wave-0001\n"));
        assert!(!t.contains("hard:c:1"));
        assert!(std::fs::read_to_string(d.join("learn/shipped/train.conllu")).unwrap().contains("hard:c:1"));
        assert_eq!(std::fs::read_to_string(d.join("learn/shipped/dev.conllu")).unwrap(), "");
        let m = read_json(&d.join("learn/manifest.json")).unwrap();
        assert_eq!(m["files"]["internal/dev"]["trees"], 1);
        let _ = std::fs::remove_dir_all(d);
    }
}
