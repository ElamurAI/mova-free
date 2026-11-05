//! Abstract Whale workers. A pinned immutable task queue, bounded source context, at most one
//! teacher call per task through `mova-teacher`, strict validation against a configuration-
//! supplied output contract and versioned bundles with receipts; pool/swarm scheduling and a
//! Crab external-role handler, and the English-speaking service (`animal`). Concrete prompts,
//! schemas, gates and deployments are external.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub mod agent;
pub mod animal;
pub mod claims;
pub mod clause;
pub mod compose;
pub mod answer;
pub mod batch;
pub mod contract;
pub mod dossier;
pub mod epoch;
pub mod gates;
pub mod knowledge;
pub mod learn;
pub mod originality;
pub mod paper;
pub mod pod;
pub mod pool;
pub mod reader;
pub mod review;
pub mod roles;
pub mod seaspider;
pub mod whale;
pub mod source;
pub mod spec;
pub mod swarm;
pub mod terminal;
pub mod topic;
pub mod util;
pub use roles::WhaleRoles;
/// Tests that spawn processes or observe lifetime flock leases run one at a time: a child
/// forked by a concurrent test briefly inherits every open descriptor until exec.
#[cfg(test)]
pub(crate) fn serial() -> std::sync::MutexGuard<'static, ()> {
    static SERIAL: std::sync::Mutex<()> = std::sync::Mutex::new(());
    SERIAL.lock().unwrap_or_else(|e| e.into_inner())
}
