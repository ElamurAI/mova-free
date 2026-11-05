# algorithm-matrix-chain-tests

## descriptant

### matrix_chain tests groupant

Original code; no external implementation was copied. The mathematical recurrence is standard dynamic programming. Raw Rust is authoritative outside this sidecar. Syntax is partial and omits expansion and semantic resolution.

Composition: source identity, forward recurrence, invariant and typed relations jointly constrain observations. behavior_complement excludes outcomes; reverse_reconstruction retains compatible antecedents. They do not supply a global inverse; leaf checks do not prove caller wiring.

## approximant

### Forward, invariant and cost

Iterate the bounded deterministic corpus and compare independent oracle outcomes. Invariant: every visited case satisfies its assertion before advancing. Failures stop the corresponding test. Recursive exhaustive oracles are exponential and intentionally bounded.

## models

# Typed relations

Subject: tests.rs, source SHA-256 in manifest.json. Successful-result relations assume storage success.

## behavior_complement

Universe: implementation/test-outcome pairs. On the fixed corpus a pass excludes disagreement at any asserted observation, but excludes no unseen defect. 3,280 dimension vectors of length 0..7 over 1..3 against exhaustive parenthesization costs. Tests cover empty and zero cases, product overflow, sum overflow, an overflowing split with exact u128::MAX optimum, classical cost 15125, reversal, cubic scaling, constant dimensions and singleton preimages.

## reverse_reconstruction

A passing run admits all implementations agreeing on tested observations, including potentially incorrect programs outside the corpus. A failing run admits defect hypotheses, not a uniquely recovered cause. The compiled mutant supplies one concrete failing antecedent.

## oracle_relation

The oracle enumerates every tree cost recursively without memoization or intermediate minimization; only the outer assertion minimizes. Small dimensions ensure ordinary u128 arithmetic remains exact.

## metamorphic_relation

Paired inputs must satisfy the equality or ordering laws stated in the implementation models. Restricted successful small inputs avoid resource ambiguity. These checks supplement the independent oracle.

## mutation_separation

Replacing candidate comparison cost<old with cost>old in compiled production code must fail unchanged real assertions; it did. This separates maximizing from minimizing behavior.

## tests

### Checks and limits

Four tests passed in each of two debug and two optimized standalone rustc --edition 2024 --test runs. Single-thread outputs matched after removing timing from the footer. Strict clippy-driver --test -D warnings and rustfmt passed. Compiled copied production mutant (maximize candidate cost) failed unchanged assertion checks. No mutation remains in repository source. Syntax extraction ran twice and matched bytes; source hashes are bound in manifest.json.

No allocator-refusal injection, benchmark, machine-domain proof or workspace integration is claimed. Parent integration owns exports and central registry wiring.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: 3,280 dimension vectors of length 0..7 over 1..3 against exhaustive parenthesization costs. Tests cover empty and zero cases, product overflow, sum overflow, an overflowing split with exact u128::MAX optimum, classical cost 15125, reversal, cubic scaling, constant dimensions and singleton preimages. Four tests passed twice each in debug and optimized standalone builds, with equal normalized outcomes. Strict lint passed and compiled mutants failed real assertions. Source syntax was identical twice. Reviewer correction replaced a non-overflowing chain witness with [MAX,MAX,1,2], and expanded coin cost accounting to include initialization and validation. Initial smoke tests preceded this recorded evidence; no prospective speedup claim was made. Finite observations are not universal proof. Allocation refusal, huge actual slices and complete preimage execution remain gaps. Preserve these observations in later epochs.

### 2026-10-30 · tests

2026-10-30: 3,280 dimension vectors of length 0..7 over 1..3 against exhaustive parenthesization costs. Tests cover empty and zero cases, product overflow, sum overflow, an overflowing split with exact u128::MAX optimum, classical cost 15125, reversal, cubic scaling, constant dimensions and singleton preimages.
