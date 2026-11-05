# algorithm-manacher

## descriptant

### manacher mod.rs epoch

The mathematical combiner combines raw source, source-bound partial syntax, forward model, ordered typed obernants, invariants, cost and finite checks. The tests leaf is separate so a correct test model does not certify production. The first measured epoch supports the prediction on 9,841 bounded cases per algorithm and long constant fixtures; both compiled semantic mutants show red. No timing gain was measured. Future epochs must retain these observations and revise unsupported claims rather than convert them into proof.

Byte offsets deliberately permit spans inside UTF-8 encodings. Parent wiring belongs to integration. Macro bodies and semantic analysis remain gaps. All exact behavior is described directly; executable approximation profiles are absent.

## model

# manacher mod.rs model

## Forward

For byte sequence s of length n return odd/even vectors of length n. odd[i] is maximal r>=1 with palindrome s[i+1-r..i+r]; even[i] is maximal r>=0 with palindrome s[i-r..i+r]. The even center is immediately before i. Empty input yields two empty vectors.

## Invariant

For each parity pass, earlier radii are maximal and [left,right) is a known palindrome with the greatest discovered right endpoint. A center inside it copies the reflected radius clipped at right-i; symmetry guarantees this lower bound. Further comparisons extend it maximally. Odd left=i+1-radius and even left=i-radius never underflow; right=i+radius<=n. Mirror indices use left+(right-1-i) or left+(right-i), avoiding overflowing left+right.

## Cost

Two linear passes: O(n) byte comparisons, O(n) output words, O(1) auxiliary words. Each successful comparison beyond the mirror lower bound advances the right frontier; at most one failed comparison per center. No throughput measurement.

## nonbehavior

Type: behavior_complement. Universe: finite byte inputs paired with arbitrary pairs of usize vectors. Excluded outputs have wrong lengths, zero odd radii, out-of-range spans, unequal symmetric byte pairs in a claimed span, or a radius whose next valid outer pair still matches. The model describes excluded behavior independently of negative tests.

## compatible_antecedents

Type: reverse_reconstruction. Given two radius vectors of equal length n, compatible inputs are exactly byte strings of length n satisfying 1<=odd[i]<=min(i+1,n-i) and 0<=even[i]<=min(i,n-i), every symmetric equality inside every claimed span, and inequality of the next outer byte pair whenever that pair exists. Empty vectors admit empty input. Inconsistent equalities/inequalities admit no input; injective symbol relabelings exhibit nonunique antecedents.

## relabeling_boundaries

Type: metamorphic_relation. Injective byte relabeling preserves both vectors. A constant string has odd[i]=min(i+1,n-i), even[i]=min(i,n-i). UTF-8 bytes need not share the character-level palindrome structure.

## joint_output_feasibility

Type: constraint_feasibility. Domain: Radius vectors of common length n<=256, byte witnesses.

Relation: Require 1<=o[i]<=min(i+1,n-i), 0<=e[i]<=min(i,n-i). Union symmetric pairs in both parities and add inequality edges for next outer pairs when in bounds. Feasible iff no edge collapses to one class; distinct class bytes construct a witness. Larger n additionally requires 256-colorability; the bounded routine does not decide this.

Assumptions: Ordinary successful Rust execution, sufficient allocation and exact byte equality. Mathematical sums; test sums fit usize. Finite fixtures do not establish unrestricted correctness.

Falsifiable check: joint_parity_constraints_reconstruct_and_reject rejects o=[1,1,1],e=[0,1,1], despite legal bounds.

## reversal_centers

Type: metamorphic_relation. Domain: Input s of length n and t=reverse(s); empty case vacuous.

Relation: o_t[i]=o_s[n-1-i]. e_t[0]=0 and e_t[i]=e_s[n-i] for 1<=i<n. Even centers are gaps; simply reversing the even vector is incorrect.

Assumptions: Ordinary successful Rust execution, sufficient allocation and exact byte equality. Mathematical sums; test sums fit usize. Finite fixtures do not establish unrestricted correctness.

Falsifiable check: reversal_centers_and_injective_symbols checks five fixtures including aab, where naive reversal fails.

## interval_membership

Type: observation_projection. Domain: Fixed input and nonempty half-open intervals [a,b) within its length.

Relation: Let L=b-a,c=a+floor(L/2). Palindrome membership iff L even and e[c]>=L/2, or L odd and o[c]>=floor(L/2)+1. Complete radii encode every nonempty interval's membership, not bytes.

Assumptions: Ordinary successful Rust execution, sufficient allocation and exact byte equality. Mathematical sums; test sums fit usize. Finite fixtures do not establish unrestricted correctness.

Falsifiable check: interval_membership_count_and_lossy_projection compares every interval with two-ended equality on five fixtures.

## palindrome_count_loss

Type: information_loss. Domain: Full radii observed through C=sum(o)+sum(e).

Relation: C counts all nonempty palindromic intervals: each radius level contributes one distinct interval at its center/parity. aab and abb have equal C=4 but different even vectors, so count cannot recover location or radii.

Assumptions: Ordinary successful Rust execution, sufficient allocation and exact byte equality. Mathematical sums; test sums fit usize. Finite fixtures do not establish unrestricted correctness.

Falsifiable check: interval_membership_count_and_lossy_projection checks direct counts and the collision.

## maximal_boundary_constraints

Type: boundary_exclusion. Domain: Fixed input, center and in-bounds candidate radius in either parity.

Relation: Span symmetry alone does not imply maximality. The next outer pair must differ if present; otherwise a boundary stops expansion. Odd zero is excluded. Injective byte maps preserve equalities and inequalities; collapsing symbols can lengthen palindromes.

Assumptions: Ordinary successful Rust execution, sufficient allocation and exact byte equality. Mathematical sums; test sums fit usize. Finite fixtures do not establish unrestricted correctness.

Falsifiable check: joint_parity_constraints_reconstruct_and_reject checks maximality contradictions; reversal_centers_and_injective_symbols rejects ab->aa invariance.

## Current check scope

Baseline nonbehavior and compatible_antecedents use manifest domains and ordinary execution assumptions. Their falsifiable checks are the independent exhaustive oracle and bounded reconstruction tests. Relabeling requires an injective byte map; collapsing symbols is a rejected countermodel. No executable approximate profile is introduced: supported error caps remain at most 10%, and exact fixtures imply no approximation-error or speed claim.

## evidence

### Evidence

Three focused tests per algorithm pass in two debug runs and one release run. Each run checks 9,841 exhaustive ternary strings through length 8 against an independent naive oracle, explicit empty/singleton/repeated/mixed and UTF-8-byte cases, injective relabeling and a 100,000-byte constant fixture. Z uses zipped direct prefix comparison; palindrome testing enumerates intervals and checks reversal, without mirrored radii.

The external production-source mutant replaces the stored even radius with zero compiles with rustc --edition 2024 --test and is rejected by tests::boundaries_and_utf8_bytes with exit 101. Production source is unchanged. Strict cargo clippy -p mova-algorithms --offline --all-targets -- -D warnings passes. Focused commands use cargo test -p mova-algorithms --offline manacher::tests::, repeated with --release. Temporary lib.rs exports support local checks and are not committed.

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
