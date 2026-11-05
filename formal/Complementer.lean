import Mathlib
/-!
Complementer baseline: converse/preimage of a forward model. These theorems concern
the declared functions, not extraction from Rust or identification of real causes.
-/
namespace MovaComplementer

def antecedent {X Y : Type} (forward : X → Y) (y : Y) (x : X) : Prop :=
  forward x = y

theorem original_is_candidate {X Y : Type} (f : X → Y) (x : X) :
    antecedent f (f x) x := rfl

theorem unique_for_injective {X Y : Type} (f : X → Y)
    (h : Function.Injective f) (x x' : X) :
    antecedent f (f x) x' ↔ x' = x := by
  constructor
  · exact fun hx => h hx
  · intro hx
    subst x'
    rfl

theorem composition_reverses {X Y Z : Type} (f : X → Y) (g : Y → Z)
    (z : Z) (x : X) :
    antecedent (g ∘ f) z x ↔
      ∃ y, antecedent g z y ∧ antecedent f y x := by
  constructor
  · intro h
    exact ⟨f x, h, rfl⟩
  · rintro ⟨y, hg, hf⟩
    change g (f x) = z
    change f x = y at hf
    rw [hf]
    exact hg

theorem left_inverse_needs_injectivity {X Y : Type} (f : X → Y)
    (undo : Y → X) (h : ∀ x, undo (f x) = x) : Function.Injective f := by
  intro a b hab
  calc
    a = undo (f a) := (h a).symm
    _ = undo (f b) := congrArg undo hab
    _ = b := h b

def project (p : Bool × Bool) : Bool := p.1

theorem projection_has_two_candidates :
    antecedent project false (false, false) ∧
    antecedent project false (false, true) := by simp [antecedent, project]

theorem projection_has_no_left_inverse :
    ¬ ∃ undo : Bool → Bool × Bool, ∀ x, undo (project x) = x := by
  rintro ⟨undo, h⟩
  have hi := left_inverse_needs_injectivity project undo h
  have eq : (false, false) = (false, true) := hi rfl
  have bad : false = true := congrArg Prod.snd eq
  cases bad

theorem swap_is_its_own_inverse (p : Bool × Bool) :
    p.swap.swap = p := by cases p; rfl

theorem unreachable_has_no_candidate :
    ¬ ∃ x : Nat, antecedent Nat.succ 0 x := by
  simp [antecedent]

-- Forward-model and observation error budgets add. This only bounds the
-- output residual; it does not bound distance to the original input.
theorem true_input_survives_error_budget
    (actual modeled observed modelError observationError : ℝ)
    (hm : |modeled - actual| ≤ modelError)
    (ho : |actual - observed| ≤ observationError) :
    |modeled - observed| ≤ modelError + observationError := by
  calc
    |modeled - observed| = |(modeled - actual) + (actual - observed)| := by congr 1; ring
    _ ≤ |modeled - actual| + |actual - observed| := abs_add_le _ _
    _ ≤ modelError + observationError := add_le_add hm ho

#print axioms original_is_candidate
#print axioms unique_for_injective
#print axioms composition_reverses
#print axioms left_inverse_needs_injectivity
#print axioms projection_has_two_candidates
#print axioms projection_has_no_left_inverse
#print axioms swap_is_its_own_inverse
#print axioms unreachable_has_no_candidate
#print axioms true_input_survives_error_budget
end MovaComplementer
