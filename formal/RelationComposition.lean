import Complementer
/-!
Abstract relation algebra. Composition means first R, then S. No theorem here
certifies Rust extraction, floating-point behavior, or measured approximation.
-/
namespace MovaRelationComposition

def compose {X Y Z : Type} (R : X → Y → Prop) (S : Y → Z → Prop)
    (x : X) (z : Z) : Prop := ∃ y, R x y ∧ S y z

def converse {X Y : Type} (R : X → Y → Prop) (y : Y) (x : X) : Prop := R x y

def identity {X : Type} (x y : X) : Prop := x = y

-- The universe is a relation, so excluded pairs are explicit even for nonrectangular scopes.
def complement {X Y : Type} (U R : X → Y → Prop) (x : X) (y : Y) : Prop :=
  U x y ∧ ¬ R x y

theorem converse_composition {X Y Z : Type} (R : X → Y → Prop)
    (S : Y → Z → Prop) (z : Z) (x : X) :
    converse (compose R S) z x ↔ compose (converse S) (converse R) z x := by
  simp only [converse, compose]
  exact exists_congr fun _ => and_comm

theorem converse_involution {X Y : Type} (R : X → Y → Prop) :
    converse (converse R) = R := rfl

theorem composition_associative {W X Y Z : Type} (R : W → X → Prop)
    (S : X → Y → Prop) (T : Y → Z → Prop) (w : W) (z : Z) :
    compose (compose R S) T w z ↔ compose R (compose S T) w z := by
  constructor
  · rintro ⟨y, ⟨x, hr, hs⟩, ht⟩
    exact ⟨x, hr, y, hs, ht⟩
  · rintro ⟨x, hr, y, hs, ht⟩
    exact ⟨y, ⟨x, hr, hs⟩, ht⟩

theorem identity_left {X Y : Type} (R : X → Y → Prop) (x : X) (y : Y) :
    compose identity R x y ↔ R x y := by simp [compose, identity]

theorem identity_right {X Y : Type} (R : X → Y → Prop) (x : X) (y : Y) :
    compose R identity x y ↔ R x y := by simp [compose, identity]

theorem complement_double_scope {X Y : Type} (U R : X → Y → Prop) (x : X) (y : Y) :
    complement U (complement U R) x y ↔ U x y ∧ R x y := by
  classical
  constructor
  · rintro ⟨hu, hn⟩
    refine ⟨hu, ?_⟩
    by_contra hr
    exact hn ⟨hu, hr⟩
  · rintro ⟨hu, hr⟩
    exact ⟨hu, fun hn => hn.2 hr⟩

theorem complement_involution {X Y : Type} (U R : X → Y → Prop)
    (within : ∀ x y, R x y → U x y) (x : X) (y : Y) :
    complement U (complement U R) x y ↔ R x y := by
  rw [complement_double_scope]
  exact ⟨And.right, fun h => ⟨within x y h, h⟩⟩

-- Reuse the original function-preimage model, with exactly the same candidate semantics.
theorem function_preimage_composition {X Y Z : Type} (f : X → Y) (g : Y → Z)
    (z : Z) (x : X) :
    MovaComplementer.antecedent (g ∘ f) z x ↔
      compose (MovaComplementer.antecedent g) (MovaComplementer.antecedent f) z x :=
  MovaComplementer.composition_reverses f g z x

-- On Bool, identity ; identity is identity, but inequality ; inequality is identity too.
-- At (false,false), complement(identity ; identity) is false and the composed
-- complements are true (with the intermediate witness true).
theorem complement_composition_counterexample :
    ¬ (∀ x z : Bool,
      complement (fun _ _ => True) (compose identity identity) x z ↔
      compose (complement (fun _ _ => True) identity)
        (complement (fun _ _ => True) identity) x z) := by
  intro h
  have bad := (h false false).mpr (show compose
    (complement (fun _ _ => True) identity)
    (complement (fun _ _ => True) identity) false false from
      ⟨true, by simp [complement, identity], by simp [complement, identity]⟩)
  exact bad.2 ⟨false, rfl, rfl⟩

-- A first-stage error needs a downstream sensitivity bound before it can be added.
-- These premises must hold on the actual intermediate states, including modeled ones.
theorem composition_error_budget (f fApprox g gApprox : ℝ → ℝ)
    (x firstError secondError amplification : ℝ)
    (ha : 0 ≤ amplification)
    (hf : |fApprox x - f x| ≤ firstError)
    (hg : |gApprox (fApprox x) - g (fApprox x)| ≤ secondError)
    (sensitivity : |g (fApprox x) - g (f x)| ≤ amplification * |fApprox x - f x|) :
    |gApprox (fApprox x) - g (f x)| ≤ secondError + amplification * firstError := by
  exact MovaComplementer.true_input_survives_error_budget
    (g (fApprox x)) (gApprox (fApprox x)) (g (f x))
    secondError (amplification * firstError) hg
    (sensitivity.trans (mul_le_mul_of_nonneg_left hf ha))

#print axioms converse_composition
#print axioms converse_involution
#print axioms composition_associative
#print axioms identity_left
#print axioms identity_right
#print axioms complement_double_scope
#print axioms complement_involution
#print axioms function_preimage_composition
#print axioms complement_composition_counterexample
#print axioms composition_error_budget
end MovaRelationComposition
