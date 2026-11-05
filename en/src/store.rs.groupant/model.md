# en-store

## descriptant

### store

Ser grows output bytes; De retains borrowed bytes and cursor; Items tracks remaining typed elements. Header44 bytes binds magic/version/enum sizes/dictionary fingerprint/body length/checksum.

Primitive integers/floats fixed-width little endian; lengths/variants/char LEB128; fields concatenate without names; options/bools use0/1. from_bytes deserializes known type then rejects trailing bytes. unpack checks short/magic, version, enum shape, fingerprint, body length, checksum, then typed body. Chunk arrays require byte length divisible by element width.

Partial source-bound model, not full proof. No useful executable percentage surrogate established for these categorical decisions/effects; budgets1/2/5/10% require a future observation metric. No corpus/API/GPU/teacher, allocation fault injection, adversarial size exhaustion or broad accuracy claim.

## approximant

### Forward relation

Primitive integers/floats fixed-width little endian; lengths/variants/char LEB128; fields concatenate without names; options/bools use0/1. from_bytes deserializes known type then rejects trailing bytes. unpack checks short/magic, version, enum shape, fingerprint, body length, checksum, then typed body. Chunk arrays require byte length divisible by element width.

Domain: compatible parallel arrays and valid symbol indices; API-specific omissions/invalid-input behavior in model.md#models. Finite machine arithmetic follows Rust source; no real-number replacement guarantee.

## invariant

### State and cost

Ser grows output bytes; De retains borrowed bytes and cursor; Items tracks remaining typed elements. Header44 bytes binds magic/version/enum sizes/dictionary fingerprint/body length/checksum.

Invalid bool/Option tags cannot deserialize. Trailing bytes cannot be silently accepted by from_bytes. Wrong checksum prevents body parsing. Format is not self-describing and cannot skip arbitrary values. Checksum is not authentication, and varint parser does not enforce canonical encodings or reject all tenth-byte overflow patterns.

O(serialized bytes plus typed element work), output allocation proportional to body; pack copies body into header buffer. Deserialization borrows primitive strings/bytes where visitor permits but vectors allocate advertised lengths. No size cap or allocation-failure Result guarantee.

## composition

### Scoped composition

Parent includes helpers, public API and test declarations; fragments below focus on distinct mechanisms and share the whole-file partial lexical projection. They are not separately registered sources.

- serialization: bytes[1675,7483) delegates this source's mechanism; forward composition: Primitive integers/floats fixed-width little endian; lengths/variants/char LEB128; fields concatenate without names; options/bools use0/1. from_bytes deserializes known type then rejects trailing bytes. unpack checks short/magic, version, enum shape, fingerprint, body length, checksum, then typed body. Chunk arrays require byte length divisible by element width.
- deserialization: bytes[7483,14935) delegates this source's mechanism; forward composition: Primitive integers/floats fixed-width little endian; lengths/variants/char LEB128; fields concatenate without names; options/bools use0/1. from_bytes deserializes known type then rejects trailing bytes. unpack checks short/magic, version, enum shape, fingerprint, body length, checksum, then typed body. Chunk arrays require byte length divisible by element width.
- envelope: bytes[18768,21426) delegates this source's mechanism; forward composition: Primitive integers/floats fixed-width little endian; lengths/variants/char LEB128; fields concatenate without names; options/bools use0/1. from_bytes deserializes known type then rejects trailing bytes. unpack checks short/magic, version, enum shape, fingerprint, body length, checksum, then typed body. Chunk arrays require byte length divisible by element width.

## models

## behavior_complement

Invalid bool/Option tags cannot deserialize. Trailing bytes cannot be silently accepted by from_bytes. Wrong checksum prevents body parsing. Format is not self-describing and cannot skip arbitrary values. Checksum is not authentication, and varint parser does not enforce canonical encodings or reject all tenth-byte overflow patterns.

## reverse_reconstruction

For fixed type T and compatible dictionary/format, unpack(pack(x)) recovers serializable state under serde contracts; accepted bytes can be noncanonical and map pair iteration can differ on rewrite. Output value's compatible byte set includes equivalent map orders and varints; therefore unique original byte reconstruction is unsupported.

## failure_precedence

Header magic/size precedes version, shape, fingerprint, length, checksum, then serde error/trailing-byte check.

## bit_transport

f32/f64 carry raw bits including signed zero and NaN; equality of floats is weaker than bit preservation. Chunk arrays use exact LE chunks.

## resource_lengths

Unknown sequence length fails serialize. Decoder varint loop at most10 bytes; conversion to usize checked, take bounds checked. Typed visitor allocation remains uncontrolled.

## schema_scope

Struct names/field names omitted; enum indices and field order form wire schema. Same header does not prove target T matches payload schema.

## tests

### Evidence

Compiled source mutants cover construct rounded-floor boundary, parse root legality and store checksum gate. Other sources have no compiled source mutation claim. Preflight empty-model nonempty tagging failed in ptag; synthetic fixtures adjusted without production changes. Existing corpus/seeds tests not selected.

Partial source-bound model, not full proof. No useful executable percentage surrogate established for these categorical decisions/effects; budgets1/2/5/10% require a future observation metric. No corpus/API/GPU/teacher, allocation fault injection, adversarial size exhaustion or broad accuracy claim.

## prospective

### Prospective structure model — before edits

Read all nine sources. Annotation composes tagging, parse, repair, child flags, UPOS, two-pass features and morphology; complete and kbest omit repair. Save packs before mkdir/write/rename; no fsync or concurrent-writer guarantee. Graph encoding casts heads to u16 and local symbols deduplicate by linear search; lossless preserves token spacing flags, not arbitrary original whitespace. Explanation separately runs tag/parse and encode: duplicated work and no repair equivalence claim.

Parser maintains root-indexed stack/buffer, attachment and saturating child counts. Legal root attachment waits for exhausted buffer; 2n transitions; greedy ties favor later ACTS, beam ties stable with gold lower. Width zero clamps to one at step. Constraints require compatible sizes/heads; no input validation. Sparse table restores weights, not averaging accumulators. Linearizer uses MIN=3 side backoff, right on ties, subset DP through degree10 then greedy; depth>200 truncates traversal. Invalid forests/cycles may omit nodes. Punctuation first OnceLock wins, majority strictly >half.

UD child flags scan quadratically and propagate conjunction flags with bounded chains. UPOS/FEATS need count2, lower enum tie; sentence features refine verbs left-to-right unless EN_AGR=0. Scores compare base labels, content F1, and sorted function children. Expert binds distinct nodes, checks all require clauses then any unless; missing references rejected, unclosed blocks omitted. Construct loader checks exact token spelling, but allows root-0 dependent: recall requires d>=1 and sufficient tree length; pct zero denominator clamps to1, report compares rounded percentage strictly below floor. Store validates magic/size, version, enum shape, dictionary, length, checksum, then typed body; varint decoding is bounded but not canonical and high bits in tenth byte are not independently rejected.

Tests will use synthetic in-memory fixtures only: nine focused contracts, no corpus, GPU, API or live training. Model::train_with(empty,1) is bounded zero-data initialization. Compiled mutations target construct threshold, parser root legality and store checksum rejection; failures and restored positives retained. Expected costs: expert O(n falling-factorial m times predicate/require work), parser beam O(n*b*(features*labels+n)+n*b log b) due cloned configs; linearizer sum O(2^k*k^2) for k<=10, quadratic greedy otherwise; store linear bytes plus typed allocation; graph local intern worst quadratic strings. No executable percent surrogate is justified for these categorical/effectful APIs; keep explicit gap. Models and syntax remain partial.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: source read before tests/models. Synthetic checks characterize a small boundary, not whole NLP quality. Preflight empty-trained tagger panicked on nonempty input; model/explain fixtures now use two synthetic one-token sentences. This is an unmodified dependency/domain gap.

### 2026-10-31 · tests

Actual source test: store::wave27::bounded_contract. Command: cargo test -p en --lib wave27 -- --test-threads=1 (debug twice, release twice). External evidence: $MOVA_DATA/runs/octopus-wave27-structure-2026-10-31. Nine focused tests, each in its owned production module; no copied implementation.
