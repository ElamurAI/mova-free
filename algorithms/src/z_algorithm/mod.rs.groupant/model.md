# algorithm-z-algorithm

## descriptant

### z_algorithm mod.rs epoch

The mathematical combiner combines raw source, source-bound partial syntax, forward model, ordered typed obernants, invariants, cost and finite checks. The tests leaf is separate so a correct test model does not certify production. The first measured epoch supports the prediction on 9,841 bounded cases per algorithm and long constant fixtures; both compiled semantic mutants show red. No timing gain was measured. Future epochs must retain these observations and revise unsupported claims rather than convert them into proof.

Byte offsets deliberately permit spans inside UTF-8 encodings. Parent wiring belongs to integration. Macro bodies and semantic analysis remain gaps. All exact behavior is described directly; executable approximation profiles are absent.

## model

# z_algorithm mod.rs model

## Forward

For a byte sequence s of length n, return n lengths. z[0]=n when n>0; for i>0 z[i]=max{k<=n-i: s[0..k]=s[i..i+k]}. Empty input returns [].

## Invariant

Before center i, all earlier entries are maximal. The half-open interval [left,right) equals the prefix of length right-left and right is the greatest previously discovered endpoint. If i<right, copying min(z[i-left],right-i) is justified by the equal prefix. Extension compares only still unknown bytes; a completed entry either hits the end or the next bytes differ. Indices are guarded by z[i]<n-i.

## Cost

O(n) byte comparisons and O(n) output words, O(1) auxiliary words. Successful uncached comparisons advance the nondecreasing right boundary; each center contributes at most one unsuccessful comparison. No throughput measurement.

## nonbehavior

Type: behavior_complement. Universe: finite byte inputs paired with arbitrary usize vectors. Excluded outputs have incorrect length, z[0] != n when n>0, a radius beyond the suffix, an unequal byte inside a claimed match, or an extendable nonmaximal radius. This mathematical exclusion relation is not replaced by negative tests.

## compatible_antecedents

Type: reverse_reconstruction. Given vector v, compatible antecedents are all byte strings s with length n=v.len(), 0<=v[i]<=n-i for every index i, v[0]=n when nonempty, equalities s[j]=s[i+j] for j<v[i], and inequality s[v[i]]!=s[i+v[i]] whenever i+v[i]<n. Empty vector admits only empty input. Constraints can be inconsistent; symbol names are not recoverable uniquely.

## relabeling_boundaries

Type: metamorphic_relation. Every injective relabeling of byte symbols preserves the prefix vector. A constant string of length n yields [n,n-1,...,1].

## output_feasibility

Type: constraint_feasibility. Domain: Candidate vectors v of length n<=256, byte witnesses.

Relation: Require v[0]=n for n>0 and v[i]<=n-i. Quotient positions by j~i+j for j<v[i]. Add inequality edges (v[i],i+v[i]) when i+v[i]<n. Feasible iff no edge is a quotient self-loop: assign distinct bytes to classes. For n>256 general feasibility additionally requires quotient graph 256-colorability.

Assumptions: Ordinary successful Rust execution, sufficient allocation and exact byte equality. Mathematical sums; test sums fit usize. Finite fixtures do not establish unrestricted correctness.

Falsifiable check: equality_constraints_reconstruct_and_reject rejects [3,2,0] despite legal bounds and roundtrips five witnesses.

## border_period_duality

Type: structural_equivalence. Domain: Nonempty input s of length n, integers 1<=p<=n.

Relation: p is a period iff s[p..n]=s[0..n-p], iff p=n or z[p]=n-p. For p<n this is exactly a border of length n-p. Periods need not divide n; divisibility additionally characterizes whole-block repetition.

Assumptions: Ordinary successful Rust execution, sufficient allocation and exact byte equality. Mathematical sums; test sums fit usize. Finite fixtures do not establish unrestricted correctness.

Falsifiable check: borders_periods_and_prefix_occurrences compares each p to literal slices; ababa refutes divisibility.

## prefix_occurrence_projection

Type: observation_projection. Domain: Fixed input of length n, prefix length 1<=k<=n.

Relation: Occurrence starts of s[0..k] are exactly {i:z[i]>=k}. Count=sum_i 1[z[i]>=k]. Retaining positions recovers the occurrence observation, not original symbols.

Assumptions: Ordinary successful Rust execution, sufficient allocation and exact byte equality. Mathematical sums; test sums fit usize. Finite fixtures do not establish unrestricted correctness.

Falsifiable check: borders_periods_and_prefix_occurrences compares threshold positions with windows(k) on five fixtures.

## equality_partition_ambiguity

Type: reverse_ambiguity. Domain: Full Z observation over byte strings with latent symbol names.

Relation: Injective relabeling preserves z, but its fiber exceeds relabeling orbits: abb and abc both yield [3,0,0] although their equality partitions differ. Reconstruction cannot uniquely recover even the partition.

Assumptions: Ordinary successful Rust execution, sufficient allocation and exact byte equality. Mathematical sums; test sums fit usize. Finite fixtures do not establish unrestricted correctness.

Falsifiable check: equality_constraints_reconstruct_and_reject compares the actual outputs; relabeling alone is an incomplete inverse.

## sum_observation_loss

Type: information_loss. Domain: Z vectors for equal-length inputs, observed through h(z)=sum_i z[i].

Relation: h merges full-vector fibers: abca yields [4,0,0,1] and aabc yields [4,1,0,0], both sum five. h cannot recover positions or z. Neither input reversal nor noninjective symbol maps generally preserve z.

Assumptions: Ordinary successful Rust execution, sufficient allocation and exact byte equality. Mathematical sums; test sums fit usize. Finite fixtures do not establish unrestricted correctness.

Falsifiable check: compressed_observation_and_collapsing_symbols checks sum collision and reversal/noninjective countermodels.

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
