# algorithm-z-algorithm-tests

## descriptant

### z_algorithm tests.rs epoch

The mathematical combiner combines raw source, source-bound partial syntax, forward model, ordered typed obernants, invariants, cost and finite checks. The tests leaf is separate so a correct test model does not certify production. The first measured epoch supports the prediction on 9,841 bounded cases per algorithm and long constant fixtures; both compiled semantic mutants show red. No timing gain was measured. Future epochs must retain these observations and revise unsupported claims rather than convert them into proof.

Byte offsets deliberately permit spans inside UTF-8 encodings. Parent wiring belongs to integration. Macro bodies and semantic analysis remain gaps. All exact behavior is described directly; executable approximation profiles are absent.

## model

# z_algorithm tests.rs model

## Forward

Execute 9,841 ternary sequences of lengths 0 through 8 against an independent naive oracle, plus explicit empty/singleton/repetition/UTF-8 and 100,000-byte constant-input fixtures; succeed only when all assertions hold.

## Invariant

Base-three decoding enumerates each length-n ternary sequence once. The oracle shares neither the rightmost-box invariant nor mirror reuse with production. Assertions compare complete vectors; the count assertion protects enumeration scope. Tests cannot establish all-input correctness.

## Cost

Bounded exhaustive oracle work O(sum(n^2*3^n), n=0..8), O(n) temporary space; long fixture O(100000).

## nonbehavior

Type: behavior_complement. Within the universe of these test runs paired with pass/fail observations, pass is excluded if any specified assertion is false. Fail is excluded when every assertion completes true in the ordinary execution model. Untested inputs and compiler/runtime defects remain outside the observation.

## compatible_antecedents

Type: reverse_reconstruction. A passing observation admits every candidate implementation agreeing with all checked fixture observations; it cannot identify the source or constrain all untested inputs. A failing observation admits executions with at least one violated assertion; the assertion trace narrows the compatible implementations.

## relabeling_boundaries

Type: metamorphic_relation. Fixture-preserving injective byte relabeling should preserve tested output; long constant fixtures exercise maximal radii.

## constraint_witness_gate

Type: constraint_correspondence. Domain: Candidate vectors on named fixtures with at most 256 positions.

Relation: Reconstruction succeeds iff bounds hold and no required inequality lies inside an equality class; returned witnesses must reproduce the candidate through production. Joint constraints, not individual bounds, determine feasibility.

Assumptions: Ordinary successful Rust execution, sufficient allocation and exact byte equality. Mathematical sums; test sums fit usize. Finite fixtures do not establish unrestricted correctness.

Falsifiable check: Run equality_constraints_reconstruct_and_reject or joint_parity_constraints_reconstruct_and_reject; reject in-bounds contradictory vectors.

## observation_loss_gate

Type: information_loss. Domain: Named distinct-output fixture pairs and deterministic projection assertions.

Relation: Projection equality admits distinct full vectors: the sum observation cannot identify its antecedent. Passing interval/position assertions additionally constrains production at every named interval/threshold.

Assumptions: Ordinary successful Rust execution, sufficient allocation and exact byte equality. Mathematical sums; test sums fit usize. Finite fixtures do not establish unrestricted correctness.

Falsifiable check: Run compressed_observation_and_collapsing_symbols or interval_membership_count_and_lossy_projection; full-vector equality is a rejected alternative.

## Current check scope

Baseline nonbehavior and compatible_antecedents use manifest domains and ordinary execution assumptions. Their falsifiable checks are the independent exhaustive oracle and bounded reconstruction tests. Relabeling requires an injective byte map; collapsing symbols is a rejected countermodel. No executable approximate profile is introduced: supported error caps remain at most 10%, and exact fixtures imply no approximation-error or speed claim.

## evidence

### Evidence

Three focused tests per algorithm pass in two debug runs and one release run. Each run checks 9,841 exhaustive ternary strings through length 8 against an independent naive oracle, explicit empty/singleton/repeated/mixed and UTF-8-byte cases, injective relabeling and a 100,000-byte constant fixture. Z uses zipped direct prefix comparison; palindrome testing enumerates intervals and checks reversal, without mirrored radii.

The external production-source mutant changes z[0]=n to zero compiles with rustc --edition 2024 --test and is rejected by tests::boundaries_and_utf8_bytes with exit 101. Production source is unchanged. Strict cargo clippy -p mova-algorithms --offline --all-targets -- -D warnings passes. Focused commands use cargo test -p mova-algorithms --offline z_algorithm::tests::, repeated with --release. Temporary lib.rs exports support local checks and are not committed.

Source-bound syntax is generated by installed mova-modelcheck syntax and generated twice for equality. The projection is partial: macro expansion and semantic resolution are absent. Finite evidence supplies no universal proof, timing claim, downstream integration claim or approximation certificate. No approximate executable profile is declared for these exact algorithms; 1/2/5/10 percent caps do not turn exact code into an approximation.

## prospective

### Prospective

Before implementation the planned API fixed byte semantics, z[0]=n, and palindrome centers with explicit odd/even spans. Expected O(n) comparisons/storage from a monotone rightmost match/palindrome interval. Independent exhaustive ternary cases, boundary and UTF-8 bytes, long constant inputs, strict Clippy, debug/release checks and compiled source mutations were planned. These are finite checks, not proofs. The mathematical invariants and exact observation domain are recorded in model.md.

## History

### 2026-10-30 · descriptant

2026-10-30. Repository search found no z_algorithm or manacher implementation in algorithms/src. Original std-only code, no unsafe, new dependencies or external solution imports. API and mathematical target were fixed before implementation: maximal prefix lengths or odd/even palindrome radii over bytes, with a monotone right frontier and linear predicted cost.

#### Constraint and observation epoch, 2026-10-30

Constraints from different positions must be solved jointly: legal coordinate bounds can still be impossible. Distinct-class byte witnesses are complete only in the stated at-most-256-position domain; larger domains require quotient-graph coloring. Projection collisions preserve reverse ambiguity. Named Rust tests connect these models to production outputs without changing its API. Earlier exhaustive evidence remains useful. No runtime bug or performance improvement is claimed.

### 2026-10-30 · evidence

Heading `### Evidence 2026-10-30` stays in evidence as `### Evidence`.

#### Constraint and projection checks, 2026-10-30

Six focused tests per algorithm pass in two fresh debug executions with byte-identical ordered assertion-result records. The complete algorithm library passes 143 unit tests in debug and release; strict Clippy with --all-targets -- -D warnings passes. Three new tests per algorithm check constructive witnesses and impossible outputs, derived period/interval observations, and projection or transformation countermodels. The witness routines are tested on the named finite fixtures, not exhaustively over candidate vectors.

Two independently compiled source mutants fail new tests: changing Z's first entry from n to n.saturating_sub(1) fails equality_constraints_reconstruct_and_reject; reducing stored even palindrome radii by one fails interval_membership_count_and_lossy_projection. Each rustc test binary compiled successfully and reported one failed selected test. Production implementations remain unchanged.

All four source syntax projections were generated twice and compared byte-for-byte. External logs and deterministic results are under $MOVA_DATA/runs/octopus-deep-strings-2026-10-30. Structural checks do not prove semantics; no downstream school result, approximate executable certificate or throughput gain is claimed.

### 2026-10-30 · prospective

Heading `### Prospective epoch 2026-10-30` stays in prospective as `### Prospective`.
