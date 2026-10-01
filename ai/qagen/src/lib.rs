//! qagen — training questions and answers from the LLM over fairy-tale paragraphs (closed part;
//! — "Training questions from the LLM"). Design note: Opus itself
//! can generate plenty of questions and answers per paragraph for training.
//!
//! Per paragraph — 3–6 questions of FairytaleQA types (character, setting, action, feeling, causal, outcome,
//! prediction), an explicit/implicit mark and anchors to the paragraph's sentences. Texts are fairy tales from the database in
//! the public domain (gate `en::license::commercial_gate`, source `gutenberg-tales`). The LLM call is
//! `prag::opus`; gates — `gate`; into the database — `db qa` (table `qa`, migration v3).
//!
//! Rule v2: an explicit answer is a verbatim span of a single anchor sentence, as in FairytaleQA
//! (`gate::span_reason`); an implicit one composed of the lemmas of a single anchor is marked "doubtful implicit"
//! (`lemma`, lemmas from `en`). Pilot v1 is corrected by `fix`: new rows replace the old ones (`db qa --replaces`).

pub mod fix;
pub mod gate;
pub mod lemma;
pub mod prompt;
pub mod report;
pub mod run;
pub mod schema;
pub mod select;
