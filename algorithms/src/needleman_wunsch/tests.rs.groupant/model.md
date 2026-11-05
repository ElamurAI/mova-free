# algorithm-needleman-wunsch-tests

## descriptant

### algorithm-needleman-wunsch-tests

Run bounded independent oracle fixtures, edge witnesses, and executable reconstruction invariants; assertions certify only the exercised inputs.

All oracle cases are generated deterministically; comparison or reconstruction must assert, not merely execute.

Partial Rust syntax projection excludes macro expansion and semantic resolution. Finite tests do not prove general correctness or linear cost. No executable approximation profile: outputs are discrete exact objects and no useful certified surrogate metric is proposed.

## model

# algorithm-needleman-wunsch-tests

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

15 binary words through length 3 per side, 64 scoring triples: 14,400 independent complete-path oracle comparisons. Every returned alignment is reconstructed and rescored. Budget equality, empty input, signed overflow and tie ordering covered. Mutation changes deletion > to >=; deterministic_ties fails.

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

15 binary words through length 3 per side, 64 scoring triples: 14,400 independent complete-path oracle comparisons. Every returned alignment is reconstructed and rescored. Budget equality, empty input, signed overflow and tie ordering covered. Mutation changes deletion > to >=; deterministic_ties fails.

Partial Rust syntax projection excludes macro expansion and semantic resolution. Finite tests do not prove general correctness or linear cost. No executable approximation profile: outputs are discrete exact objects and no useful certified surrogate metric is proposed.

## History

### 2026-10-30 · descriptant

Prospective model saved before implementation in $MOVA_DATA/runs/octopus-wave18-strings-2026-10-30/prospective.md.

Observed 2026-10-30: 15 binary words through length 3 per side, 64 scoring triples: 14,400 independent complete-path oracle comparisons. Every returned alignment is reconstructed and rescored. Budget equality, empty input, signed overflow and tie ordering covered. Mutation changes deletion > to >=; deterministic_ties fails.

### 2026-10-30 · evidence

Heading `### Evidence, 2026-10-30` stays in evidence as `### Evidence`.

Source SHA-256: 611ade5ac6b3fa1f7add40b216078cc8638b3d10a9e1035e0afe8140b8d8f64f.

Logs under $MOVA_DATA/runs/octopus-wave18-strings-2026-10-30:

- needleman_wunsch-debug-1.log: 8d4acb18f47f99c70061ace93c309cde5b567b56829beeb55873314f84af8a94

- needleman_wunsch-debug-2.log: 8d4acb18f47f99c70061ace93c309cde5b567b56829beeb55873314f84af8a94

- needleman_wunsch-release-1.log: f97285beb3e11f9353ddbd41d02fecdd1e434e13150b658af956dc357be81cdd

- needleman_wunsch-release-2.log: 232e9b8d5e579914f5ea7eb082d36abdb2c8f32e7d50fd946ef33f8c7d2f3f76

- needleman_wunsch-mutant.log: 43d7781101026853ae32690e310a90f4724109bb06c5b9a84294e787362141c5

- needleman_wunsch-restored.log: 8d4acb18f47f99c70061ace93c309cde5b567b56829beeb55873314f84af8a94

- cargo-needleman_wunsch-debug-1.log: 01a44dab7b3e00376d0ce9e0142deaa327a1fac39c52e4538101b891baf2a7ed

- cargo-needleman_wunsch-debug-2.log: 01a44dab7b3e00376d0ce9e0142deaa327a1fac39c52e4538101b891baf2a7ed

- cargo-needleman_wunsch-release-1.log: 11d11331bc49b186089748e5b7b9e0550dc59ff424dffb200589b87f41882bf4

- cargo-needleman_wunsch-release-2.log: e773725f308c3f8e737ab40cb95a48d0389618ef883b7bd6a7dd9a0b7ec182fa

Cargo focused suites also passed twice in debug and twice in release with matching normalized outcomes. Strict library Clippy passed. Temporary exports restored. Clippy log SHA-256: a5668d2636de8c7e7c3d3a1b6aea1da32174b64bee750b6b6cdfca8afc01a77b.
