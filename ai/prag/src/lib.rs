//! prag — Mova pragmatics (closed part): the first cut of the schema (,
//! "Pragmatics: what was meant") — form, speech act, indirectness, hedging, evaluation, voice and
//! "what was meant". Silver comes from the LLM (Opus) on open texts with a commercial licence (tales in
//! the public domain, Tatoeba CC0); the MMM is transparent Rust models over features from the UD tree (`en`),
//! words and context: an averaged perceptron and memory-based models (IGTree, k nearest neighbours — TiMBL).
//! The common model interface is `model::Classifier`, so several types work side by side.

pub mod data;
pub mod eval;
pub mod feats;
pub mod model;
pub mod opus;
pub mod schema;
pub mod silver;
