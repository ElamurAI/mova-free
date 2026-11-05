# algorithm-convex-hull-tests

## descriptant

### convex_hull tests.rs mathematical combiner

AST and source hash bind the mathematical relations to this file. Forward behavior, behavior complement and reconstruction fibers constrain different observations; none replaces the others. Composition requires the parent to connect public calls, shared Point representation and tests. Partial syntax omits semantic resolution and macro expansion.

Epoch memory: coordinates are i32 deliberately; full-domain i64 would require wider cross/distance handling. Duplicate points must be handled before closest-pair packing arguments. Collinear hull boundary points are deliberately discarded. No runtime acceleration or universal correctness claim follows from finite tests.

## model

# convex_hull tests.rs

## Forward

Run deterministic assertions comparing production outputs with independent geometric or exhaustive-pair certificates and stated transformations.

## Invariant

Fixture generation stays representable; assertions compare deterministic values and never depend on timing.

## Cost

Finite 512 grid subsets; certificate O(n³+nh) per fixture with n<=9, plus bounded extras.

## nonbehavior

Type: behavior_complement. Within deterministic fixture runs, exclude a passing assertion when its compared oracle/result values differ. Passing output excludes only exercised counterexamples; it does not imply correctness on unenumerated coordinates or allocator failures.

Domain: Deterministic finite fixtures in tests.rs; ordinary terminating allocation-success execution. Source-derived relation, not a universal proof.

## preimages

Type: reverse_reconstruction. A passing test transcript admits every implementation agreeing with all exercised observations, including incorrect implementations differing elsewhere. Exact test output therefore cannot reconstruct source or universal behavior; independent oracles narrow this equivalence class.

Domain: Deterministic finite fixtures in tests.rs; ordinary terminating allocation-success execution. Source-derived relation, not a universal proof.

## oracle

Type: oracle_relation. Independent shoelace determinants, source membership, strict CCW turns, support directions normal to all input pairs and degenerate endpoint checks certify 512 subsets of a 3x3 grid. Support maximization uses no production turn helper.

Domain: Deterministic finite fixtures in tests.rs; ordinary terminating allocation-success execution. Source-derived relation, not a universal proof.

## transform

Type: metamorphic_relation. Permutation, duplicates, interior/boundary insertion, positive scaling and translation retain predicted hull output; applying hull twice is unchanged. These relations intentionally admit ambiguous input fibers.

Domain: Deterministic finite fixtures in tests.rs; ordinary terminating allocation-success execution. Source-derived relation, not a universal proof.

## boundary

Type: boundary_partition. The full i32 3x3 extremal grid exercises wide cross products; empty, singleton, collinear and duplicate cases exercise dimensional collapse. The mutant retaining collinear vertices must fail endpoint/strict-turn assertions.

Domain: Deterministic finite fixtures in tests.rs; ordinary terminating allocation-success execution. Source-derived relation, not a universal proof.

## evidence

### Evidence

Source-bound validation: debug and release each run twice with normalized test outcomes compared; strict Clippy and rustfmt checks; two identical installed mova-modelcheck syntax projections. Focused compiled negative control must compile, fail a real assertion and then be restored. See the external geometry run directory for raw logs. Finite checks do not establish universal correctness or timing bounds.

nonbehavior, preimages, oracle, transform, boundary are source-derived observation models. Independent shoelace determinants, source membership, strict CCW turns, support directions normal to all input pairs and degenerate endpoint checks certify 512 subsets of a 3x3 grid. Support maximization uses no production turn helper.

## prospective

### Prospective model

Before validation: predict exact agreement with independent certificates and pair enumeration; canonical ties survive inclusive strip boundaries. Full i32 differences and squared sums fit the stated widened bounds. Debug and release normalized assertions should agree. Changing <=0 turn popping to <0 should retain forbidden collinear vertices. No performance speedup is predicted.

## History

### 2026-10-30 · descriptant

Original std-only code and fixtures for Mova, 2026-10-30. The test group combines independent certificates and metamorphic relations; agreeing fixtures cannot prove all possible inputs. No external implementation was copied.

### 2026-10-30 · evidence

Heading `### Evidence — 2026-10-30` stays in evidence as `### Evidence`.

Measured 2026-10-30: four full algorithms runs each passed 175 unit tests plus one doctest (176 normalized passing lines), identical across debug/release twice. Eight new geometry tests passed. Strict Clippy with -D warnings and rustfmt check passed. Each source projection matched byte-for-byte twice. Both compiled mutants were rejected by assertion failures; restored full suite passed. Hull mutant changed <=0 to <0 in stack popping; closest-pair mutant dropped the y-square distance term. These controls establish local test sensitivity, not proof.
