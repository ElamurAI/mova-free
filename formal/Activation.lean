import Mathlib

/-!
A real-arithmetic activation specification. A derivative and a conditional
approximation-error bound, not a proof of a global GELU approximation bound.
Hendrycks & Gimpel (2016), https://arxiv.org/abs/1606.08415:
erf and tanh gates both have the form x/2 * (1 + gate(x)).
-/
noncomputable section
namespace MovaActivation

def gated (f : ℝ → ℝ) (x : ℝ) : ℝ := x / 2 * (1 + f x)

/-- Product rule for both exact and approximate GELU gates. -/
theorem gated_derivative (f : ℝ → ℝ) (x df : ℝ) (hf : HasDerivAt f df x) :
    HasDerivAt (gated f) ((1 + f x) / 2 + x / 2 * df) x := by
  have h := ((hasDerivAt_id x).div_const 2).mul ((hasDerivAt_const x 1).add hf)
  convert h using 1
  · funext y
    simp [gated]
  · simp
    ring

/-- A pointwise gate error epsilon transfers to at most |x|*epsilon/2.
    The hypothesis must come from a separate proof or a measured domain; the
    theorem does not assert an epsilon for tanh versus erf on all real inputs. -/
theorem gate_error_bound (f g : ℝ → ℝ) (x epsilon : ℝ)
    (h : |f x - g x| ≤ epsilon) :
    |gated f x - gated g x| ≤ |x| / 2 * epsilon := by
  have algebra : gated f x - gated g x = (x / 2) * (f x - g x) := by
    simp [gated]
    ring
  rw [algebra, abs_mul, abs_div]
  norm_num
  exact mul_le_mul_of_nonneg_left h (div_nonneg (abs_nonneg x) (by norm_num))

#print axioms gated_derivative
#print axioms gate_error_bound
end MovaActivation
