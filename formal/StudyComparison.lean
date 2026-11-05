import Mathlib
/-!
A leaf binding model and the parent score-comparison model.
The Boolean guards abstract Rust identity/cardinality/passivity/row checks.
This does not prove SHA collision resistance, parser accuracy or the whole IO loop.
-/
namespace MovaStudyComparison
def binding (model assessment total noReads noUpdates rows : Bool) : Bool :=
  model && assessment && total && noReads && noUpdates && rows

theorem binding_requires_assessment (m a t r u rows : Bool)
    (h : binding m a t r u rows = true) : a = true := by
  simp only [binding, Bool.and_eq_true] at h
  exact h.1.1.1.1.2

theorem wrong_assessment_is_excluded (m t r u rows : Bool) :
    binding m false t r u rows = false := by simp [binding]

def recommend (before after oldBefore oldAfter : Nat) : Bool :=
  decide (before < after ∧ oldBefore ≤ oldAfter)

theorem no_gain_is_not_promotion (before oldBefore oldAfter : Nat) :
    recommend before before oldBefore oldAfter = false := by simp [recommend]

theorem regression_blocks_promotion (before after oldBefore oldAfter : Nat)
    (h : oldAfter < oldBefore) : recommend before after oldBefore oldAfter = false := by
  simp [recommend, Nat.not_le_of_lt h]

#print axioms binding_requires_assessment
#print axioms wrong_assessment_is_excluded
#print axioms no_gain_is_not_promotion
#print axioms regression_blocks_promotion

#eval do
  IO.println "# before\tafter\told_before\told_after\trecommend"
  for a in List.range 4 do
    for b in List.range 4 do
      for c in List.range 4 do
        for d in List.range 4 do
          IO.println s!"{a}\t{b}\t{c}\t{d}\t{recommend a b c d}"
end MovaStudyComparison
