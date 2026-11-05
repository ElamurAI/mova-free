# algorithm-run-length-encoding

## descriptant

### algorithm-run-length-encoding

Encode bytes into their unique maximal constant-byte runs. Decode only positive counts with differing adjacent bytes; check the count sum and explicit byte budget before allocating output. Classical lossless run-length coding, original implementation.

Encoding runs cover the consumed input exactly, counts are positive, and neighboring run bytes differ. Decoding accumulates a checked sum before output allocation, then each resize extends precisely one validated run.

Partial Rust syntax projection excludes macro expansion and semantic resolution. Finite tests do not prove general correctness or linear cost. No executable approximation profile: outputs are discrete exact objects and no useful certified surrogate metric is proposed.

## model

# algorithm-run-length-encoding

## Forward

Encode bytes into their unique maximal constant-byte runs. Decode only positive counts with differing adjacent bytes; check the count sum and explicit byte budget before allocating output. Classical lossless run-length coding, original implementation.

## Invariant

Encoding runs cover the consumed input exactly, counts are positive, and neighboring run bytes differ. Decoding accumulates a checked sum before output allocation, then each resize extends precisely one validated run.

## Cost

Encoding O(n) time and O(r) output, with ordinary Vec allocation failure behavior. Decoding O(r+n) time and O(n) output; checked sum, explicit max_output, and fallible reservation. A run count cannot overflow during encoding because it is bounded by slice length.

## nonbehavior

Decoding rejects zero counts and neighboring equal byte runs rather than silently canonicalizing malformed input. It cannot exceed max_output on success or return a partial decoded prefix.

Domain: finite byte inputs and stated successful-allocation/representability premises. Observation: public call or named internal state. Status: source-derived exact target relation with bounded evidence, not a general proof.

## preimages

On canonical runs, encode(decode(runs))=runs; on byte strings, decode(encode(bytes))=bytes with a sufficient budget. Thus both successful maps are injective on their stated domains. Error outputs have many antecedents.

Domain: finite byte inputs and stated successful-allocation/representability premises. Observation: public call or named internal state. Status: source-derived exact target relation with bounded evidence, not a general proof.

## validation-order

At each run test zero count then neighboring equality then checked length addition. Check output budget only after every run validates. Consequently later malformed runs take precedence over an earlier budget excess.

Domain: finite byte inputs and stated successful-allocation/representability premises. Observation: public call or named internal state. Status: source-derived exact target relation with bounded evidence, not a general proof.

## prefix-sum

After r runs the accumulated length equals their sum and previous records the last byte. These observations retain many different run prefixes; total length alone cannot recover their byte order.

Domain: finite byte inputs and stated successful-allocation/representability premises. Observation: public call or named internal state. Status: source-derived exact target relation with bounded evidence, not a general proof.

## concatenation

Encoding x followed by y concatenates their run lists, merging exactly the boundary pair when both are nonempty and the final x byte equals the first y byte.

Domain: finite byte inputs and stated successful-allocation/representability premises. Observation: public call or named internal state. Status: source-derived exact target relation with bounded evidence, not a general proof.

## canonical-uniqueness

For a fixed byte string every transition starts exactly one new run, so positive maximal run representation is unique; allowing zero or adjacent-equal runs would destroy this uniqueness.

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

Source SHA-256: e4ff9e057aa445e4491ee6fce21a8168a6f7452c6b8ef86734969ff349930729.

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
