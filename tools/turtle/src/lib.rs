//! Sea Turtle (`mova-turtle`): Mova's school maker, one of Mova's animals.
//!
//! The long-lived teacher that comes back every year: it checks courses against the school's own rules (`check`),
//! makes native questions and draft lessons (`make`: gap questions from a 🐚 Nautilus dictionary layer), and hands
//! the writing of lessons to a teacher terminal only behind the operator's word (`nautilus::teacher`, purpose
//! `questions`). It understands English through Mova's native deputy and answers in English (`animal`).

pub mod animal;
pub mod check;
pub mod fit;
pub mod make;
pub mod work;
