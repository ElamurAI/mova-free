//! world — the world of a tale: the SLM module from docs/context.md, "Tale world:
//! entities, registries, cursor", "Minimal objects", "Immutable states — the alternative variant", "World-change language".
//! The LLM (Opus) writes change-language commands anchored to sentences; the SLM (this crate) executes them with gates, keeps
//! registries and immutable entity states, and answers questions from the state and history — without the LLM.

pub mod absurd;
pub mod ans;
pub mod babi;
pub mod big;
pub mod bookans;
pub mod stepgame;
pub mod spartqa;
pub mod derive;
pub mod domains;
pub mod events;
pub mod family;
pub mod ftqa;
pub mod idioms;
pub mod index;
pub mod induce;
#[cfg(feature = "gpu")]
pub mod induce_gpu;
pub mod lang;
pub mod md;
pub mod mwe;
pub mod mind;
pub mod narr;
pub mod poly;
// 30.09: quantitative layer — who has how much of what and what changes (word problems).
pub mod quant;
// 30.09: level-1 (global) values as predicates over the story world.
pub mod values;
// 30.09: reading tales — paragraph summaries by the snake, checked by the LLM.
pub mod read;
pub mod rerank;
pub mod qa;
pub mod qa3;
pub mod qframe;
pub mod qud;
pub mod query;
pub mod reader;
pub mod register;
pub mod report;
pub mod rreport;
// 02.10: sense check of dependency trees from level-1 knowledge (cheap parse-error detector).
pub mod sense;
pub mod span;
pub mod store;
pub mod state;
pub mod story;
pub mod tree;
pub mod tabular;
pub mod worldbook;
pub mod triggers;
pub mod nerves;
pub mod know;
pub mod ideas;
pub mod principles;
pub mod school;
pub mod numbers;
pub mod mentions;
pub mod treeread;
pub mod exec;
pub mod opchoice;
pub mod primers;
pub mod lessons;
pub mod meanings;
pub mod knowledge;
pub mod fbench;
pub mod prules;
pub mod frames;
pub mod dictionary;
pub mod schooltools;
pub mod talk;
pub mod tune;
pub mod types;
pub mod vmm;
pub mod world;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests3;
#[cfg(test)]
mod tests03;
