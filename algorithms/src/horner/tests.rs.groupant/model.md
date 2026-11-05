# algorithm-horner-tests

## descriptant

### horner tests.rs

Executable finite checks for horner: power_oracle independently computes powers and differentiated powers, degrees 0..14 over 17 dyadic points; invalid/overflow cases.

Original std-only implementation. Literature: see module-level Clenshaw (1955), Horner (1819), Neumaier (1974), or Lyness (1969) citation as applicable; citations are algorithm references, not copied code.

## model

## behavior_complement

For fixed fixtures, complement consists of assertion traces differing from the required exact errors, statuses or tolerance comparisons; implementations agreeing only here remain possible. A passing trace excludes the compiled mutant but not arbitrary bugs outside fixtures.

## reverse_reconstruction

Pass reconstructs the set of implementations consistent with these finite assertions; it does not identify one implementation. Untested inputs remain arbitrary. At x=0 value=c0 and derivative=c1; all higher coefficients remain free. [2,3,7] and [2,3,99] have identical outputs. With fixed degree and x, real fiber is intersection of two affine constraints; dependent constraints and f64 rounding require separate treatment.

## oracle_independence

power_oracle independently computes powers and differentiated powers, coefficient lengths 0..14 over 17 dyadic points; invalid/overflow cases.

## mutation_sensitivity

Replace derivative +value with -value; power_oracle fails

## finite_scope

Finite fixture agreement supplies empirical observations, not a semantic proof or universal error cap. Different implementations with the same fixture outputs remain in the pass fiber.


## reference_observation_equivalence

Compare original per-step and final gates on 133332 combinations (coefficient lengths 0..4, ten values, twelve arguments), two invalid-coefficient cases and derivative-only overflow. Compare both success bit patterns including signed zero, and exact error variants. The shared recurrence reference is complemented by the independent power oracle.

## approximant

### Forward model and invariant

Three tests combine independent power/derivative observations, boundary/fiber witnesses and full-width comparison with original gate placement. Success bits and error variants remain distinct.

Invariant: 133332 deterministic loop cases compare success bits and exact errors. The missing-derivative-gate mutant must fail W20-HORNER-DERIVATIVE-GATE.

Cost: Fixed corpus: 133332 comparisons with at most four coefficients, 255 independent power-oracle cases of lengths 0..14, and boundaries. Oracle cost is distinct from production.

IEEE-754 assumptions; no universal real accuracy guarantee. Wave20 is exact in its Result observation, not an approximate surrogate.

## tests

### Evidence

power_oracle independently computes powers and differentiated powers, degrees 0..14 over 17 dyadic points; invalid/overflow cases.

## wave20-evidence

### Horner optimization evidence

[Native receipts](../../../../models/algorithm-library.groupant/model.md#wave20-evidence) and [actual CPU report](../../../../models/algorithm-library.groupant/model.md#wave20-benchmark) distinguish the scoped absorption argument, finite observations and measured work. Historical wave19 measurements remain unchanged.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: independent polynomial/basis/integer fixtures and explicit finite-error partitions. Simpson hidden bump demonstrates limits of sampled estimates. An initial correction-denominator mutant survived tight-tolerance tests; a loose-tolerance quartic assertion was added to isolate the correction, and that mutant was then rejected. No speed claim; no percentages inferred from finite observations. Rust syntax projection is partial, without macro expansion, type resolution or semantic proof.

### 2026-10-30 · tests

Compiled production negative control: Replace derivative +value with -value; power_oracle fails. Original bytes restored and named positive test rerun. Full algorithms library passes twice debug and twice release with identical sorted test-name/results; receipts bind logs under $MOVA_DATA/runs/octopus-wave19-numeric-2026-10-30. Source projection generated twice and compared before installation. Strict crate all-target Clippy attempted; outcome in receipt.

### 2026-10-30 · wave20-evidence

Heading `### Horner optimization evidence — 2026-10-30` stays in wave20-evidence as `### Horner optimization evidence`.

One final finite-pair gate replaces per-coefficient output gates after finite input validation. Under ordinary IEEE-754 arithmetic, nonfinite recurrence states cannot recover to an admitted finite final pair. Successful arithmetic order and Result variants are retained; failure can traverse more coefficients. Root's compiled production mutant rejects; restored code passes four integrated algorithm profiles. Source SHA-256 751f5de9220401aea97bd6d76e421eb99102a06168dd1bb848757d5b8d26f0a5.
