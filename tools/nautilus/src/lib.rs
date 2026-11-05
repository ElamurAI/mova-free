//! Nautilus (`mova-nautilus`): Mova's dictionary maker, one of Mova's animals.
//!
//! It grows a dictionary the way the nautilus grows its shell, chamber by chamber: a layer of entries from one
//! course or corpus at a time. Natively first (`layer`, `sources`: Mova's own lemmas, UPOS, counts and examples,
//! meanings from the pupils' dictionary); a teacher terminal writes only what the native draft cannot, and only
//! behind the operator's word (`teacher`), in a bounded standing loop (`work`). It understands English through Mova's native deputy and answers in
//! English (`animal`).

pub mod animal;
pub mod improve;
pub mod layer;
pub mod pool;
pub mod replay;
pub mod sampling;
pub mod senses;
pub mod sources;
pub mod teacher;
pub mod work;
