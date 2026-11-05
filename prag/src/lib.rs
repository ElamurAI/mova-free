//! prag — Mova pragmatics: the first cut of the schema (docs/context.md,
//! "Pragmatics: what was meant") — form, speech act, indirectness, hedging, evaluation, voice and
//! "what was meant". Silver comes from the teacher (Codex CLI through mova-teacher) on open texts with a commercial licence (tales in
//! the public domain, Tatoeba CC0); the SLM is transparent Rust models over features from the UD tree (`en`),
//! words and context: an averaged perceptron and memory-based models (IGTree, k nearest neighbours — TiMBL).
//! The common model interface is `model::Classifier`, so several types work side by side.

pub mod agree;
pub mod data;
pub mod eval;
pub mod fbench;
pub mod feats;
pub mod frames;
pub mod frames_md;
pub mod infer;
pub mod invariants;
pub mod irony;
pub mod lens;
pub mod model;
pub mod opus;
pub mod roles;
pub mod rsa;
pub mod rules;
pub mod schema;
pub mod silver;
pub mod vn;
pub mod vnconv;

/// Transport used by the compatibility-named `opus` module, for run provenance.
pub const TEACHER_TRANSPORT: &str = "Codex CLI (mova-teacher)";

#[cfg(test)]
mod wave17_models {
    use super::*;
    fn item() -> data::Item {
        data::Item {
            key: "tatoeba:1".into(),
            src: data::Src::Tatoeba,
            doc: "tatoeba".into(),
            title: String::new(),
            para: 0,
            ord: 0,
            pos: 0,
            plen: 1,
            text: "Hello.".into(),
            batch: 0,
            ctx: vec![],
        }
    }
    #[test]
    fn schema_rows_compose_with_silver_closed_domains() {
        let it = item();
        for field in schema::Field::ALL {
            for class in 0..field.classes().len() {
                let mut labels = schema::Labels::default();
                field.set(&mut labels, class);
                let tsv = schema::labels_tsv(&labels);
                assert_eq!(
                    schema::parse_labels(&tsv.split('\t').collect::<Vec<_>>()).unwrap(),
                    labels
                );
                let g = silver::gate(&format!("1\t{tsv}\n"), &[&it]);
                if labels.voice == schema::Voice::None {
                    assert_eq!(g.ok.get(&1), Some(&labels));
                    assert!(g.bad.is_empty());
                } else {
                    assert!(g.ok.is_empty());
                    assert_eq!(g.bad.len(), 1);
                }
            }
        }
        let row = schema::labels_tsv(&schema::Labels::default());
        assert!(
            silver::gate(&format!("1\t{row}\n1\t{row}\n"), &[&it])
                .ok
                .is_empty()
        );
    }
    #[test]
    fn public_words_compose_roles_and_world_state() {
        use en::gram::{Rel, Tag, UPos};
        let words = vec![
            en::annotate::Word {
                form: "Alice".into(),
                lemma: "alice".into(),
                upos: UPos::PROPN,
                tag: Tag::NNP,
                feats: Default::default(),
                head: 2,
                rel: Rel::Nsubj,
            },
            en::annotate::Word {
                form: "runs".into(),
                lemma: "run".into(),
                upos: UPos::VERB,
                tag: Tag::VBZ,
                feats: Default::default(),
                head: 0,
                rel: Rel::Root,
            },
        ];
        assert_eq!(roles::roles(words.clone()), vec![(1, 2, "agent")]);
        let input = vec![
            ("d:2".into(), words.clone()),
            ("other:1".into(), words.clone()),
            ("d:1".into(), words),
        ];
        let rs = roles::roles_docs(input.clone());
        let ws = roles::world_docs(input, 1);
        assert_eq!(
            rs.iter().map(|x| &x.0).collect::<Vec<_>>(),
            ws.iter().map(|x| &x.0).collect::<Vec<_>>()
        );
        assert!(ws[0].1.contains("alice run"));
        assert!(ws[1].1.contains("none yet"));
        assert!(ws[2].1.contains("none yet"));
    }
}
