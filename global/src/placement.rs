//! Nonexclusive knowledge retention and activation (docs/knowledge.md#the-knowledge-contract).
//! The global graph preserves admitted knowledge. The important graph is its
//! useful resident working set. Compiled mechanisms are a separate learning axis.
//! Metadata recommendations do not implement storage, RAM eviction or Rust edits.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scope {
    General,
    Domain,
    Instance,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Candidate {
    pub scope: Scope,
    /// Understanding/inference mechanisms or foundational world models eligible
    /// for slower skeleton/hard-parameter updates; not every reusable fact.
    pub mechanism_or_foundation: bool,
    pub contested: bool,
    /// Operational usefulness, supplied by the caller; not certainty.
    pub important: bool,
    /// Evidence and applicability reviewed, without claiming permanence.
    pub reviewed: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Decision {
    /// Canonical knowledge/provenance survives both compilation and RAM eviction.
    pub record_in_global_graph: bool,
    /// Eligible for the active RAM graph, subject to a separate memory budget.
    pub retain_in_important_graph: bool,
    /// A proposed slower code/parameter training change, not an automatic edit.
    pub skeleton_candidate: bool,
    /// Conflict metadata is independent of storage/residency decisions.
    pub preserve_alternatives: bool,
}

/// Input candidates are already admitted for knowledge/provenance recording.
/// Unverified claims remain explicitly unverified; recording is not endorsement.
pub fn recommend(c: Candidate) -> Decision {
    Decision {
        record_in_global_graph: true,
        retain_in_important_graph: c.important,
        skeleton_candidate: c.scope != Scope::Instance
            && c.mechanism_or_foundation
            && c.reviewed
            && !c.contested,
        preserve_alternatives: c.contested,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn detail() -> Candidate {
        Candidate {
            scope: Scope::Instance,
            mechanism_or_foundation: false,
            contested: false,
            important: false,
            reviewed: true,
        }
    }
    #[test]
    fn useful_story_detail_is_resident_without_losing_durable_knowledge() {
        let cold = recommend(detail());
        assert!(cold.record_in_global_graph);
        assert!(!cold.retain_in_important_graph && !cold.skeleton_candidate);
        let hot = recommend(Candidate {
            important: true,
            ..detail()
        });
        assert!(hot.record_in_global_graph && hot.retain_in_important_graph);
        assert!(!hot.skeleton_candidate);
        // Deactivation changes eligibility, not canonical storage.
        assert!(recommend(detail()).record_in_global_graph);
    }
    #[test]
    fn contested_does_not_mean_resident_and_compilation_is_not_exclusive() {
        let contested = recommend(Candidate {
            contested: true,
            ..detail()
        });
        assert!(contested.record_in_global_graph && contested.preserve_alternatives);
        assert!(!contested.retain_in_important_graph);
        let mechanism = Candidate {
            scope: Scope::General,
            mechanism_or_foundation: true,
            important: true,
            ..detail()
        };
        let compiled = recommend(mechanism);
        assert!(
            compiled.skeleton_candidate
                && compiled.retain_in_important_graph
                && compiled.record_in_global_graph
        );
        let revision = recommend(Candidate {
            contested: true,
            ..mechanism
        });
        assert!(!revision.skeleton_candidate);
        assert!(
            revision.preserve_alternatives
                && revision.record_in_global_graph
                && revision.retain_in_important_graph
        );
    }
    #[test]
    fn exhaustive_metadata_space_keeps_storage_activation_and_conflict_separate() {
        let mut count = 0;
        for scope in [Scope::General, Scope::Domain, Scope::Instance] {
            for bits in 0..16 {
                let c = Candidate {
                    scope,
                    mechanism_or_foundation: bits & 1 != 0,
                    contested: bits & 2 != 0,
                    important: bits & 4 != 0,
                    reviewed: bits & 8 != 0,
                };
                let d = recommend(c);
                assert!(d.record_in_global_graph);
                assert_eq!(d.retain_in_important_graph, c.important);
                assert_eq!(d.preserve_alternatives, c.contested);
                if d.skeleton_candidate {
                    assert_ne!(scope, Scope::Instance);
                    assert!(c.mechanism_or_foundation && c.reviewed && !c.contested);
                }
                let toggled = recommend(Candidate {
                    important: !c.important,
                    ..c
                });
                assert_eq!(d.skeleton_candidate, toggled.skeleton_candidate);
                assert_eq!(d.preserve_alternatives, toggled.preserve_alternatives);
                assert!(toggled.record_in_global_graph);
                count += 1;
            }
        }
        assert_eq!(count, 48);
    }
}
