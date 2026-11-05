import Mathlib

/-!
Real-arithmetic specification of gparse/src/softmax.rs.
The calculus theorem derives the directional derivative from normalized exponentials.
The Jacobian theorem rewrites a quadratic matrix-vector product as one dot product
and one elementwise pass. These are mathematical identities, not CUDA/IEEE-754 proofs.
Sources: Goodfellow, Bengio & Courville (2016), section 6.2.2.3;
https://www.deeplearningbook.org/contents/mlp.html
-/

noncomputable section
namespace MovaSoftmax
open scoped BigOperators

def normalizer {n : ℕ} (z : Fin n → ℝ) : ℝ := ∑ j, Real.exp (z j)
def probability {n : ℕ} (z : Fin n → ℝ) (i : Fin n) : ℝ :=
  Real.exp (z i) / normalizer z

theorem normalizer_pos {n : ℕ} (z : Fin n → ℝ) (i : Fin n) :
    0 < normalizer z := by
  exact lt_of_lt_of_le (Real.exp_pos (z i))
    (Finset.single_le_sum (fun j _ => le_of_lt (Real.exp_pos (z j))) (Finset.mem_univ i))

theorem exp_line (z g : ℝ) :
    HasDerivAt (fun t : ℝ => Real.exp (z + t * g)) (Real.exp z * g) 0 := by
  simpa using ((hasDerivAt_const (0 : ℝ) z).add ((hasDerivAt_id (0 : ℝ)).mul_const g)).exp

theorem directional_derivative {n : ℕ} (z g : Fin n → ℝ) (i : Fin n) :
    HasDerivAt (fun t : ℝ => probability (fun j => z j + t * g j) i)
      (probability z i * (g i - ∑ j, probability z j * g j)) 0 := by
  have hd : HasDerivAt (fun t : ℝ => ∑ j, Real.exp (z j + t * g j))
      (∑ j, Real.exp (z j) * g j) 0 :=
    HasDerivAt.fun_sum (fun j _ => exp_line (z j) (g j))
  have hn : normalizer z ≠ 0 := ne_of_gt (normalizer_pos z i)
  have hquot := (exp_line (z i) (g i)).div hd (by simpa [normalizer] using hn)
  have hsum : (∑ j, probability z j * g j) =
      (∑ j, Real.exp (z j) * g j) / normalizer z := by
    simp only [probability, div_mul_eq_mul_div, Finset.sum_div]
  have algebra :
      (Real.exp (z i) * g i * normalizer z -
        Real.exp (z i) * (∑ j, Real.exp (z j) * g j)) / normalizer z ^ 2 =
        probability z i * (g i - ∑ j, probability z j * g j) := by
    rw [hsum]
    dsimp [probability]
    field_simp
  rw [← algebra]
  convert hquot using 1 <;> simp [probability, normalizer]
  funext t
  rfl

/-- Dense Jacobian multiplication equals the linear-work backward formula.
    No probability assumptions are needed for this algebraic factorization. -/
theorem jacobian_reduction {n : ℕ} (p g : Fin n → ℝ) (i : Fin n) :
    (∑ j, ((if i = j then p i else 0) - p i * p j) * g j) =
      p i * (g i - ∑ j, p j * g j) := by
  simp [sub_mul, Finset.sum_sub_distrib, Finset.mul_sum, mul_sub, mul_assoc]

/-- The transpose Jacobian product (reverse mode) has the same reduction. -/
theorem vjp_reduction {n : ℕ} (p g : Fin n → ℝ) (i : Fin n) :
    (∑ j, ((if j = i then p j else 0) - p j * p i) * g j) =
      p i * (g i - ∑ j, p j * g j) := by
  have symm (j : Fin n) :
      ((if j = i then p j else 0) - p j * p i) =
        ((if i = j then p i else 0) - p i * p j) := by
    by_cases h : i = j
    · subst j
      rfl
    · simp [h, Ne.symm h, mul_comm]
  simp_rw [symm]
  exact jacobian_reduction p g i

/-- A normalized row's backward components sum to zero. -/
theorem backward_zero_sum {n : ℕ} (p g : Fin n → ℝ) (hp : ∑ i, p i = 1) :
    (∑ i, p i * (g i - ∑ j, p j * g j)) = 0 := by
  simp_rw [mul_sub]
  rw [Finset.sum_sub_distrib, ← Finset.sum_mul, hp]
  ring

/-- Adding a constant upstream gradient leaves backward unchanged. -/
theorem upstream_shift {n : ℕ} (p g : Fin n → ℝ) (hp : ∑ i, p i = 1)
    (c : ℝ) (i : Fin n) :
    p i * ((g i + c) - ∑ j, p j * (g j + c)) =
      p i * (g i - ∑ j, p j * g j) := by
  simp_rw [mul_add]
  rw [Finset.sum_add_distrib, ← Finset.sum_mul, hp]
  ring

#print axioms directional_derivative
#print axioms jacobian_reduction
#print axioms vjp_reduction
#print axioms backward_zero_sum
#print axioms upstream_shift
end MovaSoftmax
