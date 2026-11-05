//! Document dossiers and one global world for Whale epochs.
//!
//! A dossier (`dossier`) keeps every text written about a document, epoch after epoch, append-only:
//! the original, Mova's drafts, a consultant's descriptions, earlier descriptions and web pages, with
//! licence, provenance, inputs and citation counts. The global world (`world`) reads every text once
//! with Mova (through `mova-topic-corpus`) into one incremental graph: entities with every mention,
//! statements with their sentence and document, a word → sentence → document index and document
//! edges; `world(e)` is the graph over the texts of epochs before `e`. Queries (`query`) give a
//! writer explanations, related documents, a document's nearest neighbourhood with all its content
//! and context packs, all with citations. Everything is internal by default (`dossier::public_ok`).
//! The texts export (`export`) writes the finished descriptions, their drafts and sources into an
//! editorial texts repository, idempotently and without overwriting edits.
pub mod config;
pub mod dossier;
pub mod export;
pub mod query;
pub mod world;

pub use anyhow::Result;

#[cfg(test)]
mod tests;
