import Mathlib.Data.Set.Basic
import Mathlib.Tactic.Tauto

/- Abstract set model only. This does not verify Rust, filesystem semantics,
   path normalization, selection predicates, serialization, or runtime cost. -/
namespace SampleCarry
variable {α : Type*}

def carry (A B C : Set α) : Set α := (A \ C) ∪ (B ∩ C)
def delta (A B : Set α) : Set α := (A \ B) ∪ (B \ A)

theorem carry_eq_iff_outside (A B C : Set α) :
    carry A B C = B ↔ ∀ x, x ∉ C → (x ∈ A ↔ x ∈ B) := by
  classical
  constructor
  · intro h x hx
    have he := Set.ext_iff.mp h x
    simp only [carry, Set.mem_union, Set.mem_sdiff, Set.mem_inter_iff] at he
    tauto
  · intro h
    apply Set.ext
    intro x
    have hx := h x
    simp only [carry, Set.mem_union, Set.mem_sdiff, Set.mem_inter_iff]
    tauto

theorem outside_iff_delta_subset (A B C : Set α) :
    (∀ x, x ∉ C → (x ∈ A ↔ x ∈ B)) ↔ delta A B ⊆ C := by
  classical
  constructor
  · intro h x hx
    have he := h x
    simp only [delta, Set.mem_union, Set.mem_sdiff] at hx
    tauto
  · intro h x hx
    have he : ((x ∈ A ∧ x ∉ B) ∨ (x ∈ B ∧ x ∉ A)) → x ∈ C := fun hd => h hd
    tauto

theorem carry_eq_iff_delta_subset (A B C : Set α) :
    carry A B C = B ↔ delta A B ⊆ C :=
  (carry_eq_iff_outside A B C).trans (outside_iff_delta_subset A B C)

theorem carry_empty (A B : Set α) : carry A B ∅ = A := by
  ext x
  simp [carry]

theorem carry_full (A B : Set α) : carry A B Set.univ = B := by
  ext x
  simp [carry]

/- For fixed refreshed selection B and changed set C, this characterizes all
   compatible predecessor selections A for an observed result R. Inside C,
   A is unconstrained; unique reconstruction generally fails. -/
theorem compatible_preimage (A B C R : Set α) :
    carry A B C = R ↔
      (∀ x, x ∉ C → (x ∈ A ↔ x ∈ R)) ∧
      (∀ x, x ∈ C → (x ∈ B ↔ x ∈ R)) := by
  classical
  constructor
  · intro h
    constructor
    · intro x hx
      have he := Set.ext_iff.mp h x
      simp only [carry, Set.mem_union, Set.mem_sdiff, Set.mem_inter_iff] at he
      tauto
    · intro x hx
      have he := Set.ext_iff.mp h x
      simp only [carry, Set.mem_union, Set.mem_sdiff, Set.mem_inter_iff] at he
      tauto
  · rintro ⟨ho, hi⟩
    apply Set.ext
    intro x
    have hox := ho x
    have hix := hi x
    simp only [carry, Set.mem_union, Set.mem_sdiff, Set.mem_inter_iff]
    tauto

theorem unconditional_equality_is_false :
    ¬ (∀ A B C : Set Unit, carry A B C = B) := by
  intro h
  have he := Set.ext_iff.mp (h Set.univ ∅ ∅) ()
  simp [carry] at he

#print axioms carry_eq_iff_outside
#print axioms carry_eq_iff_delta_subset
#print axioms compatible_preimage
end SampleCarry
