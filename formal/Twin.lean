import Mathlib

/-!
Real-arithmetic properties of the cost twin's prediction correction.
Sherman & Morrison (1950), doi:10.1214/aoms/1177729893.
Kalman (1960), doi:10.1115/1.3662552.
With h = xᵀPx ≥ 0, the RLS update changes the current prediction by
h/(1+h) times its residual. These theorems prove the scalar consequence,
not floating-point stability, matrix positive definiteness, or forecast accuracy.
-/
noncomputable section
namespace MovaTwin

def corrected (prediction observation leverage : ℝ) : ℝ :=
  prediction + leverage / (1 + leverage) * (observation - prediction)

theorem residual_identity (p y h : ℝ) (hh : 0 ≤ h) :
    y - corrected p y h = (y - p) / (1 + h) := by
  have hn : 1 + h ≠ 0 := by linarith
  unfold corrected
  field_simp
  <;> ring

theorem residual_contracts (p y h : ℝ) (hh : 0 ≤ h) :
    |y - corrected p y h| ≤ |y - p| := by
  rw [residual_identity p y h hh, abs_div, abs_of_pos (by linarith : 0 < 1 + h)]
  apply (div_le_iff₀ (by linarith : 0 < 1 + h)).2
  nlinarith [abs_nonneg (y-p)]

/-- Gradient of squared prediction error, used when exploring local surrogates. -/
theorem loss_derivative (y p : ℝ) :
    HasDerivAt (fun q : ℝ => (y-q)^2/2) (p-y) p := by
  have d := (((hasDerivAt_const p y).sub (hasDerivAt_id p)).pow 2).div_const 2
  convert d using 1
  · funext t
    simp
  · simp
    ring

/-- A local linear/quadratic cost hypothesis has an analytic sensitivity. -/
theorem cost_derivative (a b c x : ℝ) :
    HasDerivAt (fun t : ℝ => a + b*t + c*t^2) (b+2*c*x) x := by
  have d := ((hasDerivAt_const x a).add ((hasDerivAt_id x).const_mul b)).add
    (((hasDerivAt_id x).pow 2).const_mul c)
  convert d using 1
  · funext t
    simp
  · simp
    ring

#print axioms residual_identity
#print axioms residual_contracts
#print axioms loss_derivative
#print axioms cost_derivative
end MovaTwin
