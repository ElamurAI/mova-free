//! Closing the loop between reading failures and learning.
//!
//! Mova's corpus readers keep the sentences they read badly on one shared list
//! (`mova-hard-case-list`). This crate turns that list into learning:
//!
//! 1. `policy` — what each case may become: its licence decides its training track (`shipped`,
//!    `internal`, `measure`), its document decides its split (`train`, `dev`, `measure`).
//! 2. `wave` — the Fish priority lane: open cases become source-bound Fish tasks in their documents'
//!    context (pinned pages, ready manifests, pager configurations), ahead of the generic frontier.
//! 3. `harvest` — validated Fish replies become review-only silver trees per track and split.
//! 4. `witness` — the three open parsers the Eel's tree agreement needs, on those trees.
//! 5. `eel` — the Eel's training data for a hard-case round (the public Eel engine, unchanged gates).
//! 6. `judge` — the list re-checked with each new gparse model, and the before/after pass rate of a
//!    fixed sample.
//! 7. `control` — the lane's priority (generic pagers yield while lane pagers work) and its share of
//!    the shared limit guard.
//!
//! No step calls a teacher or trains by itself; the Fish pagers and the Eel do, under their own
//! controls. Nothing here marks an annotation gold.
pub mod config;
pub mod control;
pub mod eel;
pub mod harvest;
pub mod judge;
pub mod policy;
pub mod wave;
pub mod witness;

pub use anyhow::Result;
use anyhow::Context;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::path::Path;

/// Lowercase hex SHA-256.
pub fn sha(b: impl AsRef<[u8]>) -> String {
    format!("{:x}", Sha256::digest(b.as_ref()))
}

/// SHA-256 of a file's bytes.
pub fn file_sha(p: &Path) -> Result<String> {
    Ok(sha(std::fs::read(p).with_context(|| format!("{}", p.display()))?))
}

/// Writes a file through a temporary name and a rename, synced.
pub fn write_atomic(p: &Path, b: &[u8]) -> Result<()> {
    use std::io::Write;
    if let Some(d) = p.parent() {
        std::fs::create_dir_all(d)?;
    }
    let tmp = p.with_extension(format!("{}.part", p.extension().and_then(|e| e.to_str()).unwrap_or("")));
    let mut f = std::fs::File::create(&tmp)?;
    f.write_all(b)?;
    f.sync_all()?;
    std::fs::rename(&tmp, p)?;
    Ok(())
}

/// Writes a new file; refuses to overwrite one that exists.
pub fn write_new(p: &Path, b: &[u8]) -> Result<()> {
    use std::io::Write;
    if let Some(d) = p.parent() {
        std::fs::create_dir_all(d)?;
    }
    let mut f = std::fs::OpenOptions::new().write(true).create_new(true).open(p).with_context(|| format!("{} exists", p.display()))?;
    f.write_all(b)?;
    f.sync_all()?;
    Ok(())
}

pub fn read_json(p: &Path) -> Result<Value> {
    serde_json::from_slice(&std::fs::read(p).with_context(|| format!("{}", p.display()))?).with_context(|| format!("{}", p.display()))
}

/// The id of a sentence block (`# sent_id = …`).
pub fn block_id(b: &str) -> Option<&str> {
    b.lines().find_map(|l| l.strip_prefix("# sent_id = "))
}

/// The text of a sentence block (`# text = …`).
pub fn block_text(b: &str) -> Option<&str> {
    b.lines().find_map(|l| l.strip_prefix("# text = "))
}

/// The sentence blocks of a CoNLL-U text, each ending in one newline (no blank line).
pub fn blocks(conllu: &str) -> Vec<String> {
    conllu.split("\n\n").map(str::trim).filter(|b| !b.is_empty()).map(|b| format!("{b}\n")).collect()
}

/// The word rows (id, form, …) of a block, multiword ranges and empty nodes left out.
pub fn rows(block: &str) -> Vec<Vec<&str>> {
    block
        .lines()
        .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
        .map(|l| l.split('\t').collect::<Vec<_>>())
        .filter(|c| c.len() == 10 && c[0].parse::<usize>().is_ok())
        .collect()
}

/// The block with its `# sent_id` replaced.
pub fn with_id(block: &str, id: &str) -> String {
    block.lines().map(|l| if l.starts_with("# sent_id = ") { format!("# sent_id = {id}\n") } else { format!("{l}\n") }).collect()
}

/// A lowercase SHA-256 of the text with whitespace runs collapsed (the held-out alias check).
pub fn norm_sha(text: &str) -> String {
    sha(text.split_whitespace().collect::<Vec<_>>().join(" "))
}

/// The document's lane id: `hard:<first 16 hex of SHA-256(source + NUL + document id)>`. Fish ids
/// admit only ASCII letters, digits and `._:-`; Wikipedia titles do not.
pub fn doc_id(source: &str, document: &str) -> String {
    format!("hard:{}", &sha(format!("{source}\0{document}"))[..16])
}
