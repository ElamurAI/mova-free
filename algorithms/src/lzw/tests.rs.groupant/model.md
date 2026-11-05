# algorithm-lzw-tests

## descriptant

### Memory

Original bounded byte lzw. Reference: Welch (1984), Computer 17(6), A Technique for High-Performance Data Compression. Prospective equations preceded implementation. Exact semantics and observed checks are distinct from syntax binding. Round-trip cannot alone validate two mutually wrong codecs; retain hand-derived streams and malformed checks. No timing claim or percentage approximant.

## model

### Mathematical combiner

For source S, partial syntax P(S), forward relation F and typed relations O, the sidecar records (hash(S),P(S),F,O,I). Source identity binds descriptions to bytes but does not prove F. Tests intersect compatible implementations with finite observations. Composition with callers requires domain caps and exact model identity; no global Mova accuracy claim follows. Decoded bytes can have multiple code preimages: AAA admits [65,65,65] and [65,256]. Encode chooses one canonical stream; decoding then reencoding need not preserve arbitrary accepted codes.

## approximant

### Forward, invariant, and cost

Initial dictionary is all 256 single bytes. On a miss emit w and insert wb while size<4096. Decode appends first(next) to previous; next==size is allowed only with nonempty previous (KwKwK). Freeze dictionary at 4096.

O(n*L*log(4096)) encode, O(c*L+output) decode, O(4096*L+n) storage where L is maximum stored phrase length.

Input/codes cap 65536 first; per-code validity before remaining budget; output cap min(user,65536). Dictionary IDs <=4095; bounded phrase clones may allocate and panic/abort. Empty stream valid.

Invariant: Successful decode output differing from concatenated dictionary phrases is excluded. First code >=256 is invalid; a later code greater than current dictionary size is invalid.

Original implementation informed by Welch (1984), Computer 17(6), A Technique for High-Performance Data Compression; no copied implementation. Exact categorical semantics; no useful certified percentage surrogate is attached. Patience minimality is explicitly unsupported. Finite fixtures do not constitute universal proof.

## models

# Typed test relations

## behavior_complement
A passing transcript excludes any tested output failing the explicit expected value, reconstructed bytes, or independent oracle assertion. It excludes neither untested inputs nor environmental allocation failure. For production semantics: Successful decode output differing from concatenated dictionary phrases is excluded. First code >=256 is invalid; a later code greater than current dictionary size is invalid.

## reverse_reconstruction
A green transcript identifies the equivalence class of programs agreeing on the finite fixtures; many incorrect programs remain compatible outside those fixtures. Decoded bytes can have multiple code preimages: AAA admits [65,65,65] and [65,256]. Encode chooses one canonical stream; decoding then reencoding need not preserve arbitrary accepted codes.

## oracle_independence
Myers compares a full edit grid; LZ77 independently enumerates feasible lengths and nearest tie; codecs use round-trip plus hand-derived streams/malformed cases; BPE uses specified first-merge/tie values; patience checks both script projections and a nonminimal witness. Round-trip alone is insufficient.

## boundary_partition
Finite empty, cap, malformed-code, overlap and resource witnesses sample contract branches; exhaustive binary strings do not cover all 256 symbols or allocator behavior.

## mutation_sensitivity
A compiled production mutation must fail the named assertion; successful detection distinguishes that fault only. The original source is restored byte-for-byte and rerun.

## tests

Run cargo test -p mova-algorithms lzw::tests in debug/release twice. Preserve normalized transcripts, compiled negative control, and exact restoration under (see History). No benchmark evidence yet.

### Checks

## prospective

### Prospective finite checks
Independent oracle or enumerated fixtures, empty and invalid boundaries, reconstruction, and compiled production mutation; test success is finite evidence only.

## validation

### Observed validation

The five-module batch adds 14 native tests. All pass twice debug and twice release; sorted named test records match across all four runs. Initial integrated library run: 632 passed. Strict library Clippy passed. Each production file has a compiled mutant rejected by a named assertion, followed by byte-for-byte restoration and a passing module rerun: Myers score+1/exhaustive_dp; LZW flipped decoded byte/known_and_malformed; LZ77 match length-1/greedy_overlap_and_invalid; BPE tie comparison >=/ties_and_model_identity; patience anchor Insert instead of Equal/unique_anchor_and_nonminimal_witness.

## History

### 2026-10-30 · descriptant

Heading `### Epoch memory — 2026-10-30` stays in descriptant as `### Memory`.

### 2026-10-30 · tests

Run cargo test -p mova-algorithms lzw::tests in debug/release twice. Preserve normalized transcripts, compiled negative control, and exact restoration under $MOVA_DATA/runs/octopus-wave25-sequences-2026-10-30. No benchmark evidence yet.

### 2026-10-30 · validation

Heading `### Observed validation — 2026-10-30` stays in validation as `### Observed validation`.

Each of ten source/test files has paired identical partial syntax projections, refreshed after all mutants were restored. Receipts: $MOVA_DATA/runs/octopus-wave25-sequences-2026-10-30/{focused-debug-1.log,focused-debug-2.log,focused-release-1.log,focused-release-2.log,normalized-*,mutations.log,mutant-*,restored-*,syntax.log,clippy.log}. No timing or speedup evidence; allocator failures untested. These finite observations and lexical source bindings do not establish a semantic proof or global Mova quality improvement.
