//! coref — SLM (small language model) coreference (closed part of Mova): a deterministic multi-pass sieve over
//! UD trees, where every link carries the sieve id and evidence; reading and writing CorefUD (`Entity=` in MISC);
//! MUC, B³, CEAF-e, CoNLL F1 scorer with head-based matching (CRAC).
//!
//! Layers: `doc` (documents, trees, speaker, quotes) → `mention` (mentions from the tree) → `sieve` (sieves) →
//! `corefud` (output) and `score` (measurement). `eval` — measurements on GUM: baselines, sieves, pronouns by form.

pub mod corefud;
pub mod doc;
pub mod eval;
pub mod mention;
pub mod score;
pub mod sieve;

#[cfg(test)]
mod tests;
