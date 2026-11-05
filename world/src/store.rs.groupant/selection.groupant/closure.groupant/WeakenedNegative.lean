import SemanticDelta
open SemanticDelta
/- Expected rejection: coarse completeness cannot discharge full completeness.
SemanticDelta.coarse_carry_can_fail supplies the actual counterexample. -/
theorem weakened_condition
    (P0 P1 : Unit → Projection Nat Nat Nat) (C : Set Unit)
    (h : Complete (fun i => (P0 i).structuralPart)
      (fun i => (P1 i).structuralPart) C)
    (query : Projection Nat Nat Nat → Prop) :
    SampleCarry.carry (selected P0 query) (selected P1 query) C =
      selected P1 query := by
  exact carry_eq_of_complete P0 P1 C h query
