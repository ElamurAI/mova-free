//! en — Mova's English graph compressor: text → graph (deterministic parser) → text
//! (deterministic generator). Training is on UD trees, like neural nets, but with tables and
//! Markov models. The graph is Rust structures: tags and relations are enums, lemmas are numbers.
//! Domain versions (weather) are separate crates on top of this library.

pub mod annotate;
pub mod conllu;
pub mod convert;
pub mod ctx;
pub mod ctxeval;
pub mod dict;
pub mod expert;
pub mod explain;
pub mod gram;
pub mod hash;
pub mod lexgen;
pub mod lin;
pub mod model;
pub mod morph;
pub mod parse;
pub mod ptag;
pub mod store;
pub mod tag;
pub mod text;
pub mod tok;
pub mod ud;
pub mod udreg;
pub mod license;
