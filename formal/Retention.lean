import Mathlib

/-!
Closed-book retention contract and amortized cost, independent of lesson recall.
The encode/decode law is an assumption to test against Rust snapshots, not a
proof of the serializer or of semantic correctness.
-/
namespace MovaRetention
theorem restored_answer {S B Q A : Type} (encode : S → B) (decode : B → S)
    (restore : ∀ s, decode (encode s) = s) (answer : S → Q → A) (s : S) (q : Q) :
    answer (decode (encode s)) q = answer s q := by rw [restore]

/-- An examination emits an observation but preserves learned state. -/
def exam {S Q A : Type} (answer : S → Q → A) (s : S) (q : Q) : S × A :=
  (s, answer s q)

theorem exam_preserves_state {S Q A : Type} (answer : S → Q → A) (s : S) (q : Q) :
    (exam answer s q).1 = s := rfl

/-- Difference between repeated lesson replay and one-time retention. -/
theorem amortized_saving (replay load evaluate save repeats : ℝ) :
    repeats * (replay + evaluate) -
      (replay + save + repeats * (load + evaluate)) =
    (repeats - 1) * replay - repeats * load - save := by ring

theorem repeat_cost_derivative (load evaluate initial repeats : ℝ) :
    HasDerivAt (fun r : ℝ => initial + r * (load + evaluate))
      (load + evaluate) repeats := by
  convert ((hasDerivAt_id repeats).mul_const (load + evaluate)).const_add initial using 1 <;> simp

#print axioms restored_answer
#print axioms exam_preserves_state
#print axioms amortized_saving
#print axioms repeat_cost_derivative
end MovaRetention
