import Mathlib

/-!
Projection conditions for exact lesson-local memoization.
Michie (1968), doi:10.1038/218019a0.
The world, structural candidate set and deterministic tie policy are fixed.
Rust compares the complete Ask weight block, not only the winning features.
These theorems do not prove that the Rust reader has no other dependencies;
that correspondence is checked by code review and trajectory comparisons.
-/
noncomputable section
namespace MovaExerciseReuse

theorem score_projection_eq {n : ℕ} (active : Finset (Fin n))
    (w v phi : Fin n → ℝ)
    (same : ∀ i ∈ active, w i = v i)
    (outside : ∀ i ∉ active, phi i = 0) :
    ∑ i, w i * phi i = ∑ i, v i * phi i := by
  apply Finset.sum_congr rfl
  intro i _
  by_cases h : i ∈ active
  · rw [same i h]
  · simp [outside i h]

/-- Any deterministic score-based selection, including ties, is preserved. -/
theorem selection_projection_eq {n m : ℕ} (active : Finset (Fin n))
    (w v : Fin n → ℝ) (phi : Fin m → Fin n → ℝ)
    (select : (Fin m → ℝ) → Fin m)
    (same : ∀ i ∈ active, w i = v i)
    (outside : ∀ j i, i ∉ active → phi j i = 0) :
    select (fun j => ∑ i, w i * phi j i) =
      select (fun j => ∑ i, v i * phi j i) := by
  apply congrArg select
  funext j
  exact score_projection_eq active w v (phi j) same (outside j)

/-- Perturbations outside the used features have zero directional derivative. -/
theorem inactive_direction_derivative {n : ℕ} (w phi direction : Fin n → ℝ)
    (inactive : ∀ i, direction i * phi i = 0) (point : ℝ) :
    HasDerivAt (fun t : ℝ => ∑ i, (w i + t * direction i) * phi i) 0 point := by
  have equal : (fun t : ℝ => ∑ i, (w i + t * direction i) * phi i) =
      (fun _ : ℝ => ∑ i, w i * phi i) := by
    funext t
    apply Finset.sum_congr rfl
    intro i _
    calc
      (w i + t * direction i) * phi i = w i * phi i + t * (direction i * phi i) := by ring
      _ = w i * phi i := by rw [inactive i]; ring
  rw [equal]
  exact hasDerivAt_const point _

#print axioms score_projection_eq
#print axioms selection_projection_eq
#print axioms inactive_direction_derivative
end MovaExerciseReuse
