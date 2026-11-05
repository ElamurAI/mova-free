//! Sea Spider (`mova-seaspider`): Mova's polite link downloader.
//!
//! The abstract robot MovaBot fetches a queue of URLs, honours robots.txt (RFC 9309), keeps at
//! most one request per resolved IP every 2 seconds, re-checks every redirect, stores raw bodies
//! as zstd frames in append-only segments with a JSONL journal, and records outgoing links.
//! Adapted from the family Spider; see README.md for what was taken. As one of Mova's animals
//! (`animal`, `answer`) it takes ordinary English through Mova's native deputy and answers in
//! English.

pub mod animal;
pub mod answer;
pub mod config;
pub mod crawl;
pub mod fetch;
pub mod links;
pub mod polite;
pub mod robots;
pub mod singleton;
pub mod store;
