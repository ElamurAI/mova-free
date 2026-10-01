//! world — the world of a tale (closed part): the MMM module from, "Tale world:
//! entities, registries, cursor", "Minimal objects", "Immutable states — the alternative variant", "World-change language".
//! The LLM (Opus) writes change-language commands anchored to sentences; the MMM (this crate) executes them with gates, keeps
//! registries and immutable entity states, and answers questions from the state and history — without the LLM.

pub mod ans;
pub mod babi;
pub mod stepgame;
pub mod spartqa;
pub mod ftqa;
pub mod index;
pub mod lang;
pub mod md;
pub mod narr;
pub mod poly;
// 30.09: quantitative layer — who has how much of what and what changes (word problems).
pub mod quant;
// 30.09: level-1 (global) values as predicates over the story world.
pub mod values;
// 30.09: reading tales — paragraph summaries by the snake, checked by the LLM.
pub mod read;
pub mod qa;
pub mod qa3;
pub mod qframe;
pub mod query;
pub mod reader;
pub mod report;
pub mod rreport;
pub mod span;
pub mod story;
pub mod tree;
pub mod types;
pub mod vmm;
pub mod world;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests3;
