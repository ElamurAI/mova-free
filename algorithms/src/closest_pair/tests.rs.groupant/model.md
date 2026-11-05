# algorithm-closest-pair-tests

## descriptant

### closest_pair tests.rs mathematical combiner

AST and source hash bind the mathematical relations to this file. Forward behavior, behavior complement and reconstruction fibers constrain different observations; none replaces the others. Composition requires the parent to connect public calls, shared Point representation and tests. Partial syntax omits semantic resolution and macro expansion.

Epoch memory: coordinates are i32 deliberately; full-domain i64 would require wider cross/distance handling. Duplicate points must be handled before closest-pair packing arguments. Collinear hull boundary points are deliberately discarded. No runtime acceleration or universal correctness claim follows from finite tests.

## model

# closest_pair tests.rs

## Forward

Run deterministic assertions comparing production outputs with independent geometric or exhaustive-pair certificates and stated transformations.

## Invariant

Fixture generation stays representable; assertions compare deterministic values and never depend on timing.

## Cost

5461 small sequences with O(n²) brute enumeration and bounded clouds up to 257 points; 10000 equal points exercise duplicate shortcut without brute oracle.

## nonbehavior

Type: behavior_complement. Within deterministic fixture runs, exclude a passing assertion when its compared oracle/result values differ. Passing output excludes only exercised counterexamples; it does not imply correctness on unenumerated coordinates or allocator failures.

Domain: Deterministic finite fixtures in tests.rs; ordinary terminating allocation-success execution. Source-derived relation, not a universal proof.

## preimages

Type: reverse_reconstruction. A passing test transcript admits every implementation agreeing with all exercised observations, including incorrect implementations differing elsewhere. Exact test output therefore cannot reconstruct source or universal behavior; independent oracles narrow this equivalence class.

Domain: Deterministic finite fixtures in tests.rs; ordinary terminating allocation-success execution. Source-derived relation, not a universal proof.

## oracle

Type: oracle_relation. Independent all-pairs enumeration uses i64 unsigned absolute differences and u128 powers, minimizes (distance,index tuple), and compares every sequence of length <=6 over four points (5461 sequences). It does not use production square_delta, candidate or better.

Domain: Deterministic finite fixtures in tests.rs; ordinary terminating allocation-success execution. Source-derived relation, not a universal proof.

## transform

Type: metamorphic_relation. Translation with quarter-turn preserves selected result; scaling by three multiplies distance by nine; a different-length fixture has the same result. These test selected members of the ambiguous preimage relation.

Domain: Deterministic finite fixtures in tests.rs; ordinary terminating allocation-success execution. Source-derived relation, not a universal proof.

## boundary

Type: boundary_partition. Extremal coordinate pairs require a 65-bit squared distance. Rotated 25-point extremal grids, deterministic full-width clouds, vertical lines and multiple duplicate groups exercise index ties and strip recursion. A wrong distance metric must fail independent oracle assertions.

Domain: Deterministic finite fixtures in tests.rs; ordinary terminating allocation-success execution. Source-derived relation, not a universal proof.

## evidence

### Evidence

Source-bound validation: debug and release each run twice with normalized test outcomes compared; strict Clippy and rustfmt checks; two identical installed mova-modelcheck syntax projections. Focused compiled negative control must compile, fail a real assertion and then be restored. See the external geometry run directory for raw logs. Finite checks do not establish universal correctness or timing bounds.

nonbehavior, preimages, oracle, transform, boundary are source-derived observation models. Independent all-pairs enumeration uses i64 unsigned absolute differences and u128 powers, minimizes (distance,index tuple), and compares every sequence of length <=6 over four points (5461 sequences). It does not use production square_delta, candidate or better.

## prospective

### Prospective model

Before validation: predict exact agreement with independent certificates and pair enumeration; canonical ties survive inclusive strip boundaries. Full i32 differences and squared sums fit the stated widened bounds. Debug and release normalized assertions should agree. Changing squared-distance sum to one coordinate should disagree with exhaustive pair assertions. No performance speedup is predicted.

## History

### 2026-10-30 · descriptant

Original std-only code and fixtures for Mova, 2026-10-30. The test group combines independent certificates and metamorphic relations; agreeing fixtures cannot prove all possible inputs. No external implementation was copied.

### 2026-10-30 · evidence

Heading `### Evidence — 2026-10-30` stays in evidence as `### Evidence`.

Measured 2026-10-30: four full algorithms runs each passed 175 unit tests plus one doctest (176 normalized passing lines), identical across debug/release twice. Eight new geometry tests passed. Strict Clippy with -D warnings and rustfmt check passed. Each source projection matched byte-for-byte twice. Both compiled mutants were rejected by assertion failures; restored full suite passed. Hull mutant changed <=0 to <0 in stack popping; closest-pair mutant dropped the y-square distance term. These controls establish local test sensitivity, not proof.
