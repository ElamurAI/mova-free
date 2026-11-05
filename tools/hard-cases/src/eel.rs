//! The Eel's training data for a hard-case round: `mova_eel::TrainingData` over a track's harvested
//! annotations, run through the public Eel engine (`mova_eel::trainer::run`, once) with all its
//! gates unchanged: real tree agreement with the three open parsers, the held-out exclusions, the
//! EWT LAS anchor and floor, the snapshot development set (here the track's `dev` split: a
//! candidate must not lose LAS on hard cases it never trained on), the reload check and the guarded
//! installation.
//!
//! The track's engine configuration names `fresh_train_prefixes: ["hard:"]` (the engine admits only
//! `school:` ids otherwise) and its own `run_root`. The `internal` track must install into a model
//! directory other than the deployed one: an internal model is never the shipped model.
//!
//! The export is an immutable copy: `train.conllu` (every track/train tree), `dev.conllu`,
//! `agreed-input.conllu` and `parsers/out/*.conllu` (the fully witnessed training records), with a
//! generation hash over their bytes. A round is ready only when train, dev and witnesses are there
//! and the witnesses were run on the current training file.
use crate::config::Config;
use crate::{file_sha, read_json, sha};
use anyhow::{ensure, Context, Result};
use serde_json::{json, Value};
use std::path::{Path, PathBuf};

pub struct HardData {
    pub dir: PathBuf,
}

const FILES: [&str; 6] = ["train.conllu", "dev.conllu", "agreed-input.conllu", "parsers/out/stanza.conllu", "parsers/out/trankit.conllu", "parsers/out/spacy-trf.conllu"];

impl HardData {
    pub fn new(cfg: &Config, track: &str) -> Self {
        HardData { dir: cfg.lane_root.join("learn").join(track) }
    }

    /// Ready to train: (generation, missing witness ids), or why not.
    pub fn ready(&self) -> Result<(String, Value)> {
        let d = &self.dir;
        let train = std::fs::read_to_string(d.join("train.conllu")).unwrap_or_default();
        ensure!(!crate::blocks(&train).is_empty(), "no training trees in {}", d.display());
        ensure!(!crate::blocks(&std::fs::read_to_string(d.join("dev.conllu")).unwrap_or_default()).is_empty(), "no development trees in {}", d.display());
        let w = read_json(&d.join("witness.json")).context("witnesses not run (`mova-hard-cases witness`)")?;
        ensure!(w["train_sha256"] == sha(train.as_bytes()).as_str(), "witnesses were run on another training file");
        ensure!(w["fully_witnessed"].as_u64().unwrap_or(0) > 0, "no fully witnessed training record");
        let mut h = String::new();
        for f in FILES {
            h += &format!("{f}\t{}\n", file_sha(&d.join(f))?);
        }
        Ok((sha(h), w["missing"].clone()))
    }
}

impl mova_eel::TrainingData for HardData {
    fn status(&mut self, _c: &Value) -> mova_eel::Result<Value> {
        let (g, _) = self.ready().map_err(|e| e.to_string())?;
        Ok(json!({"generation_sha256": g}))
    }

    fn export(&mut self, _c: &Value, destination: &Path) -> mova_eel::Result<Value> {
        let (g, missing) = self.ready().map_err(|e| e.to_string())?;
        std::fs::create_dir_all(destination.join("parsers/out"))?;
        for f in FILES {
            std::fs::copy(self.dir.join(f), destination.join(f))?;
        }
        // The copy must be the generation it claims.
        let (again, _) = self.ready().map_err(|e| e.to_string())?;
        if again != g {
            return Err("hard-cases: learn files changed during export".into());
        }
        Ok(json!({
            "generation_sha256": g, "train_path": destination.join("train.conllu"), "dev_path": destination.join("dev.conllu"),
            "agreed_input_train_path": destination.join("agreed-input.conllu"), "parsers_dir": destination.join("parsers"),
            "missing_training_witness_ids": missing,
        }))
    }
}

/// One hard-case Eel round for a track, when its data is ready. The engine's receipt says what
/// happened (`installed`, `rejected`, `below_threshold`).
pub fn round(cfg: &Config, track: &str) -> Result<Value> {
    let c = cfg.eel.get(track).with_context(|| format!("no eel.{track} configuration"))?;
    let model = c["model_dir"].as_str().context("eel model_dir")?;
    if track == "internal" {
        ensure!(Path::new(model) != cfg.gparse.model_dir.as_path(), "the internal track must not install into the deployed model directory");
    }
    ensure!(c["fresh_train_prefixes"].as_array().is_some_and(|a| a.iter().any(|x| x == "hard:")), "eel.{track}: fresh_train_prefixes must admit hard:");
    let mut data = HardData::new(cfg, track);
    let g = match data.ready() {
        Ok((g, _)) => g,
        Err(e) => return Ok(json!({"track": track, "ready": false, "why": format!("{e:#}")})),
    };
    mova_eel::trainer::run(c, true, &mut data).map_err(|e| anyhow::anyhow!("{e}"))?;
    let run_root = PathBuf::from(c["run_root"].as_str().context("eel run_root")?);
    let receipt = read_json(&run_root.join("trainer/generations").join(&g).join("receipt.json")).unwrap_or(json!(null));
    Ok(json!({"track": track, "ready": true, "generation": g, "receipt": receipt}))
}

#[cfg(test)]
mod tests {
    use super::*;
    use mova_eel::TrainingData;

    #[test]
    fn ready_only_with_train_dev_and_current_witnesses() {
        let d = std::env::temp_dir().join(format!("hard-eel-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(d.join("parsers/out")).unwrap();
        let tree = "# sent_id = hard:a:1\n# text = Hi\n1\tHi\thi\tINTJ\tUH\t_\t0\troot\t_\t_\n\n";
        let mut h = HardData { dir: d.clone() };
        // red: nothing yet; then train without dev; then no witnesses
        assert!(h.ready().is_err());
        std::fs::write(d.join("train.conllu"), tree).unwrap();
        assert!(h.ready().unwrap_err().to_string().contains("development"));
        std::fs::write(d.join("dev.conllu"), tree.replace("hard:a:1", "hard:b:1")).unwrap();
        assert!(h.ready().unwrap_err().to_string().contains("witnesses"));
        std::fs::write(d.join("agreed-input.conllu"), tree).unwrap();
        for p in crate::witness::PARSERS {
            std::fs::write(d.join("parsers/out").join(format!("{p}.conllu")), tree).unwrap();
        }
        std::fs::write(d.join("witness.json"), json!({"train_sha256": sha("other"), "fully_witnessed": 1, "missing": []}).to_string()).unwrap();
        assert!(h.ready().unwrap_err().to_string().contains("another training file"));
        std::fs::write(d.join("witness.json"), json!({"train_sha256": sha(tree), "fully_witnessed": 1, "missing": []}).to_string()).unwrap();
        let (g, _) = h.ready().unwrap();
        assert_eq!(h.status(&json!({})).unwrap()["generation_sha256"], g.as_str());
        let out = d.join("export");
        let e = h.export(&json!({}), &out).unwrap();
        assert_eq!(e["generation_sha256"], g.as_str());
        assert_eq!(std::fs::read_to_string(out.join("parsers/out/trankit.conllu")).unwrap(), tree);
        // a changed training file is a new generation, not ready until witnessed again
        std::fs::write(d.join("train.conllu"), tree.to_string() + &tree.replace("hard:a:1", "hard:a:2")).unwrap();
        assert!(h.ready().is_err());
        let _ = std::fs::remove_dir_all(d);
    }
}
