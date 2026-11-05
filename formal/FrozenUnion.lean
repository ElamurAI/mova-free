import Mathlib.Data.Set.Basic
/-!
Frozen-source specification: A is the current assessment source set, P the
parent's frozen source set, and O the resulting frozen set. Arbitrary sets,
not only the three-ID domain of study.rs's finite correspondence tests.
This proves the set relation, not Rust parsing, collection or execution.
-/
namespace MovaFrozenUnion

theorem union_preimage {α : Type*} (A P O : Set α) :
    A ∪ P = O ↔ A ⊆ O ∧ O \ A ⊆ P ∧ P ⊆ O := by
  classical
  constructor
  · intro h
    constructor
    · intro x hx
      rw [← h]
      exact Or.inl hx
    · constructor
      · intro x hx
        have hu : x ∈ A ∪ P := by rw [h]; exact hx.1
        rcases hu with ha | hp
        · exact False.elim (hx.2 ha)
        · exact hp
      · intro x hx
        rw [← h]
        exact Or.inr hx
  · rintro ⟨ha, hmissing, hp⟩
    apply Set.ext
    intro x
    constructor
    · intro hx
      rcases hx with hx | hx
      · exact ha hx
      · exact hp hx
    · intro hx
      by_cases hxa : x ∈ A
      · exact Or.inl hxa
      · exact Or.inr (hmissing ⟨hx, hxa⟩)

#print axioms union_preimage
end MovaFrozenUnion
