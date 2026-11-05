import Mathlib.Data.Set.Basic
import Mathlib.Tactic.Tauto

-- Intentionally false: this file MUST fail kernel-checked compilation.
def carry (A B C : Set Unit) : Set Unit := (A \ C) ∪ (B ∩ C)
example : ∀ A B C : Set Unit, carry A B C = B := by
  intro A B C
  ext x
  simp only [carry, Set.mem_union, Set.mem_sdiff, Set.mem_inter_iff]
  tauto
