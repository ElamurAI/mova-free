//! Commercial-license gate for training data. Design note: "The main thing is that the data we train
//! the closed model on has the right license for commercial use". A model that we save or use to annotate the corpus
//! is trained only on sources from `data/train-licenses.tsv` that allow commercial use. NC or an unknown
//! source — stop. Experiments with NC — only `en ud-eval`, without saving the model.

use std::path::{Path, PathBuf};

use anyhow::{Result, bail};

use crate::hash::FastMap;

/// Licenses of training sources (`data/train-licenses.tsv`, embedded in the binary).
pub const TRAIN_LICENSES: &str = include_str!("../data/train-licenses.tsv");

/// Source of a training file: `en_gum-ud-train.conllu` → `gum`, `gum-train.conllu` (meta) → `gum`.
pub fn train_source(p: &Path) -> String {
    let stem = p.file_stem().map(|x| x.to_string_lossy().into_owned()).unwrap_or_default();
    let s = stem.strip_prefix("en_").unwrap_or(&stem);
    ["-ud-train", "-ud-dev", "-ud-test", "-train", "-dev", "-test"].iter().fold(s.to_string(), |acc, suf| acc.strip_suffix(suf).map(str::to_string).unwrap_or(acc))
}

/// Commercial-license gate for `en train`: the model we save is trained only on data that
/// allows commercial use ("the main thing is that the data we train the closed model on has the right
/// license for commercial use"). NC or an unknown source — stop. Experiments with NC — only `ud-eval`, without
/// saving the model.
pub fn commercial_gate(train: &[PathBuf]) -> Result<()> {
    let table: FastMap<String, (String, bool)> = TRAIN_LICENSES
        .lines()
        .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
        .filter_map(|l| {
            let c: Vec<&str> = l.split('\t').collect();
            (c.len() >= 3).then(|| (c[0].to_string(), (c[1].to_string(), c[2].trim() == "yes")))
        })
        .collect();
    for p in train {
        let src = train_source(p);
        match table.get(&src) {
            Some((_, true)) => {}
            Some((lic, false)) => bail!("{}: source '{src}' is under {lic} — no commercial use; not allowed for training the closed model (experiments — ud-eval)", p.display()),
            None => bail!("{}: source '{src}' is unknown — add it with its license to en/data/train-licenses.tsv", p.display()),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commercial_gate_blocks_nc() {
        assert_eq!(train_source(Path::new("/x/UD_English-GUM/en_gum-ud-train.conllu")), "gum");
        assert_eq!(train_source(Path::new("/x/ud-mova/gum-train.conllu")), "gum");
        assert_eq!(train_source(Path::new("/x/corpus/mova-silver-en.conllu")), "mova-silver-en");
        assert!(commercial_gate(&[PathBuf::from("en_ewt-ud-train.conllu"), PathBuf::from("eslspok-train.conllu")]).is_ok());
        // negative controls: NC and an unknown source stop training
        assert!(commercial_gate(&[PathBuf::from("en_ewt-ud-train.conllu"), PathBuf::from("en_gum-ud-train.conllu")]).is_err());
        assert!(commercial_gate(&[PathBuf::from("gentle-test.conllu")]).is_err());
        assert!(commercial_gate(&[PathBuf::from("mystery.conllu")]).is_err());
    }
}
