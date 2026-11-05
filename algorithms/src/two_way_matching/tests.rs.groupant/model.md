# algorithm-two-way-matching-tests

## descriptant

### algorithm-two-way-matching-tests

Run bounded independent oracle fixtures, edge witnesses, and executable reconstruction invariants; assertions certify only the exercised inputs.

All oracle cases are generated deterministically; comparison or reconstruction must assert, not merely execute.

Partial Rust syntax projection excludes macro expansion and semantic resolution. Finite tests do not prove general correctness or linear cost. No executable approximation profile: outputs are discrete exact objects and no useful certified surrogate metric is proposed.

## model

# algorithm-two-way-matching-tests

## Forward

Run bounded independent oracle fixtures, edge witnesses, and executable reconstruction invariants; assertions certify only the exercised inputs.

## Invariant

All oracle cases are generated deterministically; comparison or reconstruction must assert, not merely execute.

## Cost

Finite exponential small-input oracle enumeration is intentional; production cost is modeled separately. Test allocation is ordinary and unbudgeted.

## nonbehavior

Passing fixtures do not establish universal correctness, asymptotic complexity, allocator exhaustion behavior or Unicode semantics. A compiler failure does not count as a killed mutation.

Domain: finite byte inputs and stated successful-allocation/representability premises. Observation: public call or named internal state. Status: source-derived exact target relation with bounded evidence, not a general proof.

## preimages

A named assertion failure identifies a violated fixture relation but may have many implementation causes. Passing observations retain all implementations agreeing on the enumerated finite domain.

Domain: finite byte inputs and stated successful-allocation/representability premises. Observation: public call or named internal state. Status: source-derived exact target relation with bounded evidence, not a general proof.

## oracle-independence

Binary pairs: 511 texts through length 8 times 127 patterns through length 6 = 64,897 independent window comparisons. Long periodic and nonperiodic cases, overlaps, empty boundaries and every byte. Mutation adds one to returned offsets; exhaustive_offsets fails.

Domain: finite byte inputs and stated successful-allocation/representability premises. Observation: public call or named internal state. Status: source-derived exact target relation with bounded evidence, not a general proof.

## mutation-sensitivity

The compiled intentional production mutation must reach its named assertion and fail; exact production bytes are restored and the positive suite reruns.

Domain: finite byte inputs and stated successful-allocation/representability premises. Observation: public call or named internal state. Status: source-derived exact target relation with bounded evidence, not a general proof.

## repeatability

Two serial debug runs and two serial optimized runs must have identical normalized test identities and outcomes. Timing fields are excluded; no speed claim follows.

Domain: finite byte inputs and stated successful-allocation/representability premises. Observation: public call or named internal state. Status: source-derived exact target relation with bounded evidence, not a general proof.

## evidence

### Evidence

Three tests for this algorithm passed twice in debug and twice with rustc -O, serial execution and identical normalized identities. Compiled mutation failed the named assertion; source restored byte exactly and positive suite passed again. Two source-bound partial syntax projections were byte identical.

Binary pairs: 511 texts through length 8 times 127 patterns through length 6 = 64,897 independent window comparisons. Long periodic and nonperiodic cases, overlaps, empty boundaries and every byte. Mutation adds one to returned offsets; exhaustive_offsets fails.

Partial Rust syntax projection excludes macro expansion and semantic resolution. Finite tests do not prove general correctness or linear cost. No executable approximation profile: outputs are discrete exact objects and no useful certified surrogate metric is proposed.

## History

### 2026-10-30 · descriptant

Prospective model saved before implementation in $MOVA_DATA/runs/octopus-wave18-strings-2026-10-30/prospective.md.

Observed 2026-10-30: Binary pairs: 511 texts through length 8 times 127 patterns through length 6 = 64,897 independent window comparisons. Long periodic and nonperiodic cases, overlaps, empty boundaries and every byte. Mutation adds one to returned offsets; exhaustive_offsets fails.

### 2026-10-30 · evidence

Heading `### Evidence, 2026-10-30` stays in evidence as `### Evidence`.

Source SHA-256: 27dee1b3a1fe564682f70131c1d14dbda70d7f8a380295de5a62cf1ebc0de5b1.

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
