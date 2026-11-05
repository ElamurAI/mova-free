//! Topic graphs and closed topic corpora for the paper-describing Whales (our Owls).
//!
//! A scientific paper is a set of topics, authors, references and citing works; the texts of that
//! neighbourhood are the topic's corpus. This crate builds the neighbourhood from a paper library
//! (`graph`), queues the most-cited missing open copies for the library's download driver
//! (`missing`), annotates the neighbourhood's texts with Mova into a closed corpus that a Whale reads
//! natively before it asks its consultant (`corpus`, `seed`), adds pages fetched by Sea Spider
//! (`web`), cites every statement of a description (`cite`) and turns a consultant's draft article
//! into claims whose citations are followed to the originals (`sol`). Everything in a closed corpus
//! is internal; only open-licensed library texts may feed public outputs (`licence`).
pub mod cite;
pub mod config;
pub mod corpus;
pub mod graph;
pub mod hardcases;
pub mod library;
pub mod licence;
pub mod missing;
pub mod seed;
pub mod sol;
pub mod web;
pub mod world;

pub use anyhow::Result;
