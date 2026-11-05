import Mathlib

/-!
Local school-learning model: a fixed right/chosen pair of feature witnesses.
Collins (2002), https://aclanthology.org/W02-1001/ (structured perceptron).
The Rust learner must use the witnesses of its active decision policy.
These real-arithmetic identities do not prove convergence of the max-score
policy, correctness of language understanding, or absence of i64 overflow.
-/
noncomputable section
namespace MovaSchool

def gap {n : ℕ} (w right chosen : Fin n → ℝ) : ℝ :=
  ∑ i, w i * (right i - chosen i)

def step {n : ℕ} (w right chosen : Fin n → ℝ) (rate : ℝ) : Fin n → ℝ :=
  fun i => w i + rate * (right i - chosen i)

theorem gap_update {n : ℕ} (w right chosen : Fin n → ℝ) (rate : ℝ) :
    gap (step w right chosen rate) right chosen =
      gap w right chosen + rate * ∑ i, (right i - chosen i)^2 := by
  simp only [gap, step, Finset.mul_sum, ← Finset.sum_add_distrib]
  apply Finset.sum_congr rfl
  intro i _
  ring

theorem gap_non_decreasing {n : ℕ} (w right chosen : Fin n → ℝ)
    (rate : ℝ) (h : 0 ≤ rate) :
    gap w right chosen ≤ gap (step w right chosen rate) right chosen := by
  rw [gap_update]
  have hs : 0 ≤ ∑ i, (right i - chosen i)^2 :=
    Finset.sum_nonneg (fun i _ => sq_nonneg (right i - chosen i))
  exact le_add_of_nonneg_right (mul_nonneg h hs)

theorem gap_derivative {n : ℕ} (w right chosen : Fin n → ℝ) (rate : ℝ) :
    HasDerivAt (fun t => gap (step w right chosen t) right chosen)
      (∑ i, (right i - chosen i)^2) rate := by
  simp_rw [gap_update]
  convert (hasDerivAt_id rate).mul_const (∑ i, (right i - chosen i)^2)
    |>.const_add (gap w right chosen) using 1 <;> simp

#print axioms gap_update
#print axioms gap_non_decreasing
#print axioms gap_derivative
end MovaSchool
