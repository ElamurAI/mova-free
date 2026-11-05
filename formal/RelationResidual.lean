import RelationComposition
/-!
Schröder disjointness equivalences and the right residual of composition.
Abstract typed relations only: no theorem here certifies Rust extraction,
finite evaluator correspondence, floating-point behavior or approximation bounds.
Composition, converse and scoped complement come from RelationComposition.
-/
namespace MovaRelationResidual
open MovaRelationComposition

/-- Two relations with the same endpoints share no pair. -/
def disjointRel {X Z : Type} (A B : X → Z → Prop) : Prop :=
  ∀ x z, A x z → B x z → False

def subsetRel {X Z : Type} (A B : X → Z → Prop) : Prop :=
  ∀ x z, A x z → B x z

/-- The full rectangle of a typed endpoint pair. -/
def full (X Z : Type) : X → Z → Prop := fun _ _ => True

-- R : X → Y, S : Y → Z, T : X → Z. (R;S) ∩ T = ∅ ↔ (R°;T) ∩ S = ∅.
theorem schroeder_converse_left {X Y Z : Type} (R : X → Y → Prop)
    (S : Y → Z → Prop) (T : X → Z → Prop) :
    disjointRel (compose R S) T ↔ disjointRel (compose (converse R) T) S := by
  constructor
  · rintro h y z ⟨x, hr, ht⟩ hs
    exact h x z ⟨y, hr, hs⟩ ht
  · rintro h x z ⟨y, hr, hs⟩ ht
    exact h y z ⟨x, hr, ht⟩ hs

-- (R;S) ∩ T = ∅ ↔ (T;S°) ∩ R = ∅.
theorem schroeder_converse_right {X Y Z : Type} (R : X → Y → Prop)
    (S : Y → Z → Prop) (T : X → Z → Prop) :
    disjointRel (compose R S) T ↔ disjointRel (compose T (converse S)) R := by
  constructor
  · rintro h x y ⟨z, ht, hs⟩ hr
    exact h x z ⟨y, hr, hs⟩ ht
  · rintro h x z ⟨y, hr, hs⟩ ht
    exact h x y ⟨z, ht, hs⟩ hr

theorem schroeder_three_way {X Y Z : Type} (R : X → Y → Prop)
    (S : Y → Z → Prop) (T : X → Z → Prop) :
    (disjointRel (compose (converse R) T) S ↔
      disjointRel (compose T (converse S)) R) :=
  (schroeder_converse_left R S T).symm.trans (schroeder_converse_right R S T)

-- Right residual in the explicit full rectangles X × Z and Y × Z:
-- R;S ⊆ T ↔ S ⊆ ¬(R° ; ¬T). The reverse direction is classical.
theorem right_residual_full {X Y Z : Type} (R : X → Y → Prop)
    (S : Y → Z → Prop) (T : X → Z → Prop) :
    subsetRel (compose R S) T ↔
      subsetRel S (complement (full Y Z) (compose (converse R) (complement (full X Z) T))) := by
  classical
  constructor
  · intro h y z hs
    refine ⟨trivial, ?_⟩
    rintro ⟨x, hr, -, hnt⟩
    exact hnt (h x z ⟨y, hr, hs⟩)
  · rintro h x z ⟨y, hr, hs⟩
    by_contra hnt
    exact (h y z hs).2 ⟨x, hr, trivial, hnt⟩

-- Scoped residual: T is complemented only inside U, and the bound is stated inside V.
-- Needs R;S ⊆ U (outputs lie in the declared rectangle) and S ⊆ V.
theorem right_residual_scoped {X Y Z : Type} (U : X → Z → Prop) (V : Y → Z → Prop)
    (R : X → Y → Prop) (S : Y → Z → Prop) (T : X → Z → Prop)
    (withinU : subsetRel (compose R S) U) (withinV : subsetRel S V) :
    subsetRel (compose R S) T ↔
      subsetRel S (complement V (compose (converse R) (complement U T))) := by
  classical
  constructor
  · intro h y z hs
    refine ⟨withinV y z hs, ?_⟩
    rintro ⟨x, hr, -, hnt⟩
    exact hnt (h x z ⟨y, hr, hs⟩)
  · rintro h x z ⟨y, hr, hs⟩
    by_contra hnt
    exact (h y z hs).2 ⟨x, hr, withinU x z ⟨y, hr, hs⟩, hnt⟩

-- Omitting the converse is false even on one type. On Bool, with R = {(false,true)},
-- S = {(false,false)} and T = {(true,false)}: R;S is empty, but R;T contains
-- (false,false) through witness true, and that pair lies in S.
theorem omitted_converse_counterexample :
    ¬ (∀ R S T : Bool → Bool → Prop,
      disjointRel (compose R S) T ↔ disjointRel (compose R T) S) := by
  intro h
  let R : Bool → Bool → Prop := fun x y => x = false ∧ y = true
  let S : Bool → Bool → Prop := fun y z => y = false ∧ z = false
  let T : Bool → Bool → Prop := fun x z => x = true ∧ z = false
  have lhs : disjointRel (compose R S) T := by
    rintro x z ⟨y, ⟨-, hy⟩, ⟨hy', -⟩⟩ -
    rw [hy] at hy'
    exact Bool.noConfusion hy'
  exact (h R S T).mp lhs false false ⟨true, ⟨rfl, rfl⟩, ⟨rfl, rfl⟩⟩ ⟨rfl, rfl⟩

#print axioms schroeder_converse_left
#print axioms schroeder_converse_right
#print axioms schroeder_three_way
#print axioms right_residual_full
#print axioms right_residual_scoped
#print axioms omitted_converse_counterexample
end MovaRelationResidual
