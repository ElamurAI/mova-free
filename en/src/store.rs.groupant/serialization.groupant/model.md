# en-store-serialization

## descriptant

### store: serialization

Serde primitive payloads are LE fixed width; variants/lengths use LEB128; bool/Option use0/1; structs omit names and compound end adds no bytes. Unknown sequence lengths fail before emission of their length.

## approximant

### serialization forward scope

Serde primitive payloads are LE fixed width; variants/lengths use LEB128; bool/Option use0/1; structs omit names and compound end adds no bytes. Unknown sequence lengths fail before emission of their length.

Composition rejoins the parent at its explicit state/representation boundaries; guarantees apply only to this byte range, not unrelated parent branches.

## invariant

### State and cost

Ser grows output bytes; De retains borrowed bytes and cursor; Items tracks remaining typed elements. Header44 bytes binds magic/version/enum sizes/dictionary fingerprint/body length/checksum.

Invalid bool/Option tags cannot deserialize. Trailing bytes cannot be silently accepted by from_bytes. Wrong checksum prevents body parsing. Format is not self-describing and cannot skip arbitrary values. Checksum is not authentication, and varint parser does not enforce canonical encodings or reject all tenth-byte overflow patterns.

O(serialized bytes plus typed element work), output allocation proportional to body; pack copies body into header buffer. Deserialization borrows primitive strings/bytes where visitor permits but vectors allocate advertised lengths. No size cap or allocation-failure Result guarantee.

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

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31. Exact byte range/hash in manifest. Partial lexical AST is shared with parent; no macro expansion, semantic proof or executable percent surrogate. Parent tests exercise selected reachable paths, not every statement in this fragment.

### 2026-10-31 · tests

Actual source test: store::wave27::bounded_contract. Command: cargo test -p en --lib wave27 -- --test-threads=1 (debug twice, release twice). External evidence: $MOVA_DATA/runs/octopus-wave27-structure-2026-10-31. Nine focused tests, each in its owned production module; no copied implementation.
