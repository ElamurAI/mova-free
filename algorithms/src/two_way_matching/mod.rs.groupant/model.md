# algorithm-two-way-matching

## descriptant

### algorithm-two-way-matching

Return the ascending set {i | text[i..i+m]=pattern}; for m=0 return all n+1 byte boundaries. Critical factorization plus periodic memory implements Crochemore-Perrin (JACM 38, 1991).

Both ordered maximal suffix scans choose a critical cut. Right-half mismatch advances past impossible starts; after a complete right match, the left half certifies the candidate. A periodic shift retains only the prefix guaranteed by the pattern period.

Partial Rust syntax projection excludes macro expansion and semantic resolution. Finite tests do not prove general correctness or linear cost. No executable approximation profile: outputs are discrete exact objects and no useful certified surrogate metric is proposed.

## model

# algorithm-two-way-matching

## Forward

Return the ascending set {i | text[i..i+m]=pattern}; for m=0 return all n+1 byte boundaries. Critical factorization plus periodic memory implements Crochemore-Perrin (JACM 38, 1991).

## Invariant

Both ordered maximal suffix scans choose a critical cut. Right-half mismatch advances past impossible starts; after a complete right match, the left half certifies the candidate. A periodic shift retains only the prefix guaranteed by the pattern period.

## Cost

O(n+m+z) byte comparisons and output work; O(1) scratch excluding z offsets. Slice lengths fit isize, and shift/index sums are at most n+m, hence fit usize. Output allocation may panic/abort; no output budget.

## nonbehavior

No false or omitted offsets are allowed. Matching does not decode Unicode, fold case, or remove overlapping occurrences.

Domain: finite byte inputs and stated successful-allocation/representability premises. Observation: public call or named internal state. Status: source-derived exact target relation with bounded evidence, not a general proof.

## preimages

For fixed pattern and offsets, the fiber consists of texts with exactly those occurrences and no others. Empty offsets generally admit many texts; even one match does not recover bytes outside the matched intervals.

Domain: finite byte inputs and stated successful-allocation/representability premises. Observation: public call or named internal state. Status: source-derived exact target relation with bounded evidence, not a general proof.

## critical-cut

A cut and period retain patterns whose maximal suffix in one of two opposing byte orders attains that cut with the recorded period. They do not uniquely recover a pattern.

Domain: finite byte inputs and stated successful-allocation/representability premises. Observation: public call or named internal state. Status: source-derived exact target relation with bounded evidence, not a general proof.

## periodic-memory

If the critical prefix equals the prefix displaced by period, a successful right-half scan permits retaining m-period bytes after shifting. Memory is cleared on a right-half mismatch.

Domain: finite byte inputs and stated successful-allocation/representability premises. Observation: public call or named internal state. Status: source-derived exact target relation with bounded evidence, not a general proof.

## empty-long

Empty pattern has n+1 matches, including the end boundary. A longer nonempty pattern has no matches; these cases bypass factorization.

Domain: finite byte inputs and stated successful-allocation/representability premises. Observation: public call or named internal state. Status: source-derived exact target relation with bounded evidence, not a general proof.

## byte-renaming

Any bijection of byte values preserves occurrence offsets although it may change the critical factorization and intermediate shifts.

Domain: finite byte inputs and stated successful-allocation/representability premises. Observation: public call or named internal state. Status: source-derived exact target relation with bounded evidence, not a general proof.

## CPU measurement epoch wave18

Against KMP, repeated-prefix mismatch is 4.29–4.31x faster and dense periodic matching is about 1.10x faster on these fixtures. Random single-match and mismatch cases are about 1.05–1.06x and 1.26–1.27x slower, respectively. The linear asymptotic model predicted no universal speed ordering. See model.md#benchmark-wave18. These measurements calibrate constants on fixed fixtures; they do not prove an asymptotic bound or justify automatic production substitution.

## evidence

### Evidence

Three tests for this algorithm passed twice in debug and twice with rustc -O, serial execution and identical normalized identities. Compiled mutation failed the named assertion; source restored byte exactly and positive suite passed again. Two source-bound partial syntax projections were byte identical.

Binary pairs: 511 texts through length 8 times 127 patterns through length 6 = 64,897 independent window comparisons. Long periodic and nonperiodic cases, overlaps, empty boundaries and every byte. Mutation adds one to returned offsets; exhaustive_offsets fails.

Partial Rust syntax projection excludes macro expansion and semantic resolution. Finite tests do not prove general correctness or linear cost. No executable approximation profile: outputs are discrete exact objects and no useful certified surrogate metric is proposed.

## benchmark-wave18

### Bounded CPU cost observation

Against KMP, repeated-prefix mismatch is 4.29–4.31x faster and dense periodic matching is about 1.10x faster on these fixtures. Random single-match and mismatch cases are about 1.05–1.06x and 1.26–1.27x slower, respectively. The linear asymptotic model predicted no universal speed ordering.

## History

### 2026-10-30 · descriptant

Prospective model saved before implementation in $MOVA_DATA/runs/octopus-wave18-strings-2026-10-30/prospective.md.

Observed 2026-10-30: Binary pairs: 511 texts through length 8 times 127 patterns through length 6 = 64,897 independent window comparisons. Long periodic and nonperiodic cases, overlaps, empty boundaries and every byte. Mutation adds one to returned offsets; exhaustive_offsets fails.

### 2026-10-30 · evidence

Heading `### Evidence, 2026-10-30` stays in evidence as `### Evidence`.

Source SHA-256: 59243a65cbe79def72d53aa7e6a929fdb4bc53bab1b5736cde5217e6d481acef.

Logs under $MOVA_DATA/runs/octopus-wave18-strings-2026-10-30:

- two_way_matching-debug-1.log: de33458663ba04f1a6bd8396f80554f5ff651da16e05609af4a5752f98b67558

- two_way_matching-debug-2.log: de33458663ba04f1a6bd8396f80554f5ff651da16e05609af4a5752f98b67558

- two_way_matching-release-1.log: 2d08e9e9e1dae2e45b32aac537d93883f2cc86d0af32511656d560749d4bc2f0

- two_way_matching-release-2.log: 2d08e9e9e1dae2e45b32aac537d93883f2cc86d0af32511656d560749d4bc2f0

- two_way_matching-mutant.log: 07f1789e82142e5f44d86d6d3e5cb1f551bbf9d7a77df93b285f28270b33f5dd

- two_way_matching-restored.log: de33458663ba04f1a6bd8396f80554f5ff651da16e05609af4a5752f98b67558

- cargo-two_way_matching-debug-1.log: 82a7bd56891aa322f06f4bb39815df7d42d6e5914151485cc5d3d10a9c1cb006

- cargo-two_way_matching-debug-2.log: f0ea76ed5c031b20fb37025370d152cabc7dc2c44c4bd41c26ef664aadcaf8e0

- cargo-two_way_matching-release-1.log: 79594969a49e754fa88079c8660a09c9a34ce1ba68482b73cba904107e117af9

- cargo-two_way_matching-release-2.log: 3f2ed2a26c495cfade5eb554da26fed9479e3fea84ce5abc4a80f9eff492e714

Cargo focused suites also passed twice in debug and twice in release with matching normalized outcomes. Strict library Clippy passed. Temporary exports restored. Clippy log SHA-256: a5668d2636de8c7e7c3d3a1b6aea1da32174b64bee750b6b6cdfca8afc01a77b.

### 2026-10-30 · benchmark-wave18

Heading `### Bounded CPU cost observation — 2026-10-30` stays in benchmark-wave18 as `### Bounded CPU cost observation`.

Source SHA-256 59243a65cbe79def72d53aa7e6a929fdb4bc53bab1b5736cde5217e6d481acef. Harness SHA-256 20b57c991f61192b2b05e085edf8303d56e43290fe10b7e1dfd36f10ffc9cccf. See [full fixtures, timings, limitations and prospective model](../../../../models/algorithm-library.groupant/model.md#wave18-benchmark). Two executions retain exact outputs/checksums and all seven samples per method. This is an exact-API CPU comparison, not a Rust approximation profile or Mova training claim.
