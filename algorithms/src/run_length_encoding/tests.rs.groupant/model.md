# algorithm-run-length-encoding-tests

## descriptant

### algorithm-run-length-encoding-tests

Run bounded independent oracle fixtures, edge witnesses, and executable reconstruction invariants; assertions certify only the exercised inputs.

All oracle cases are generated deterministically; comparison or reconstruction must assert, not merely execute.

Partial Rust syntax projection excludes macro expansion and semantic resolution. Finite tests do not prove general correctness or linear cost. No executable approximation profile: outputs are discrete exact objects and no useful certified surrogate metric is proposed.

## model

# algorithm-run-length-encoding-tests

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

9,841 ternary words through length 8 checked for roundtrip, transition-count canonicality and exact total lengths. Every byte and a 10,000-byte run checked. Zero, adjacent equal, usize overflow, budget and capacity errors checked. Mutation increments a run count by two; exhaustive_canonical_roundtrips fails.

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

9,841 ternary words through length 8 checked for roundtrip, transition-count canonicality and exact total lengths. Every byte and a 10,000-byte run checked. Zero, adjacent equal, usize overflow, budget and capacity errors checked. Mutation increments a run count by two; exhaustive_canonical_roundtrips fails.

Partial Rust syntax projection excludes macro expansion and semantic resolution. Finite tests do not prove general correctness or linear cost. No executable approximation profile: outputs are discrete exact objects and no useful certified surrogate metric is proposed.

## History

### 2026-10-30 · descriptant

Prospective model saved before implementation in $MOVA_DATA/runs/octopus-wave18-strings-2026-10-30/prospective.md.

Observed 2026-10-30: 9,841 ternary words through length 8 checked for roundtrip, transition-count canonicality and exact total lengths. Every byte and a 10,000-byte run checked. Zero, adjacent equal, usize overflow, budget and capacity errors checked. Mutation increments a run count by two; exhaustive_canonical_roundtrips fails.

### 2026-10-30 · evidence

Heading `### Evidence, 2026-10-30` stays in evidence as `### Evidence`.

Source SHA-256: bf2bebfd9c291d5415f32b8e36bebb56bc5494288b84721ba2947655fc16e066.

Logs under $MOVA_DATA/runs/octopus-wave18-strings-2026-10-30:

- run_length_encoding-debug-1.log: 48211eecae569f5c4f0b5b6b05d645b19ff145c359cf1ab3ac80c450f7788887

- run_length_encoding-debug-2.log: 48211eecae569f5c4f0b5b6b05d645b19ff145c359cf1ab3ac80c450f7788887

- run_length_encoding-release-1.log: 86fa04f7f772160ec25c896cff78fb89fd34717424ddca73fac7bb5ce5cc2ca4

- run_length_encoding-release-2.log: 86fa04f7f772160ec25c896cff78fb89fd34717424ddca73fac7bb5ce5cc2ca4

- run_length_encoding-mutant.log: 1e9116050511572e86c1307f5bc7e6c0b1173e236e01e7257f6991911bb18b5b

- run_length_encoding-restored.log: 48211eecae569f5c4f0b5b6b05d645b19ff145c359cf1ab3ac80c450f7788887

- cargo-run_length_encoding-debug-1.log: 95ab64b766d891e271f1620e86f5d57400af36e67d92c54e0b17d14a62f20618

- cargo-run_length_encoding-debug-2.log: ca418df8875f4248ba22455550a81126f70c15e66bb84d53b0a43d155ee0c027

- cargo-run_length_encoding-release-1.log: 47a578747c1a354b9ca9be32f8a1ca72d1e4857174f8d591458489c24f3454f5

- cargo-run_length_encoding-release-2.log: 47a578747c1a354b9ca9be32f8a1ca72d1e4857174f8d591458489c24f3454f5

Cargo focused suites also passed twice in debug and twice in release with matching normalized outcomes. Strict library Clippy passed. Temporary exports restored. Clippy log SHA-256: a5668d2636de8c7e7c3d3a1b6aea1da32174b64bee750b6b6cdfca8afc01a77b.
