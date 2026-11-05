//! roundtrip as a library: the layers of Mova (text ↔ tokens ↔ UD tree ↔ frames ↔ world), their decoders and the
//! residual of each, for the `roundtrip` binary and for tools that compose with these layers (`tools/translate`).

pub mod blocks;
pub mod class;
pub mod compare;
pub mod diff;
pub mod frames;
pub mod mixed;
pub mod pipeline;
pub mod report;
pub mod tally;
pub mod tree;
pub mod vol;
pub mod world;

#[cfg(test)]
mod tests;

use std::path::PathBuf;

/// The `en` model: `$MOVA_DATA/en/models/en.bin` (`MOVA_DATA` defaults to `~/mova-data`).
pub fn model_path() -> PathBuf {
    PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| format!("{}/mova-data", std::env::var("HOME").unwrap_or_default()))).join("en/models/en.bin")
}
