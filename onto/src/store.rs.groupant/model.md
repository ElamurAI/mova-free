# onto-store

## descriptant

### onto-store

pack fixed little-endian graph arrays under38-byte magic/version/enum-shape/length/checksum header. unpack gate order then typed arrays/pair lengths/full consumption/check. save direct write; load full read. R::take compares n<=len-p, avoiding addition overflow after retained old-code regression.

## approximant

### Forward relation

pack fixed little-endian graph arrays under38-byte magic/version/enum-shape/length/checksum header. unpack gate order then typed arrays/pair lengths/full consumption/check. save direct write; load full read. R::take compares n<=len-p, avoiding addition overflow after retained old-code regression.

#### Equations

decode(encode(g))=g for checked representable g. Reader invariant p0=0≤B; n≤B-p ⇒ p'=p+n≤B, so addition cannot overflow. ValidOffsets(o,r,t): |o|=r+1 ∧ o0=0 ∧ o_last=t ∧ monotone(o).

#### Domain

Accepted arrays/refs/UTF8 shape; no sortedness/unique keys/inverse edges/lexfile range/DAG proof. pack accepts malformed public graph; roundtrip requires checked graph. save not atomic/fsync.

Categorical behavior and effect boundaries have no validated percentage surrogate. Executable Rust approximant unsupported; 1/2/5/10% future caps require a useful metric first.

## invariant

### Invariant and domain

Accepted arrays/refs/UTF8 shape; no sortedness/unique keys/inverse edges/lexfile range/DAG proof. pack accepts malformed public graph; roundtrip requires checked graph. save not atomic/fsync.

#### Limits

Finite synthetic fixtures are not semantic proof. Public invalid indices, allocation failures, huge counters, environment races and live effects outside explicit checks remain unverified.

## cost

### Cost approximation

O(B+N+E+W) codec/checksum with full buffers/copies; u32 count checked_mul4 and take bounds before allocation; no total allocation limit.

Symbolic costs only; no timing or throughput claim.

## composition

### Composition

decode(encode(g))=g for checked representable g. Reader invariant p0=0≤B; n≤B-p ⇒ p'=p+n≤B, so addition cannot overflow. ValidOffsets(o,r,t): |o|=r+1 ∧ o0=0 ∧ o_last=t ∧ monotone(o).

Graph builder→store shape check→Kb; Kb→query→bridge/describe/pron. hs Parsed→hsmap preference/scoring/gates. Core SynMap last-duplicate vs Graph first-duplicate requires unique synset keys for interchangeable lookup. Syntax children below share parent projection; exact byte ranges bind original source semantics.

- reader: Cursor p starts0; take(n) if n<=B-p returns b[p..p+n] and p'=p+n<=B

- shape: For each CSR off require len=rows+1,start0,end=total,monotone; validate UTF8/refs

## models

## behavior_complement

Bad earlier header gate excludes Ok/body decode. n>remaining excludes slice panic under p<=len. Extra bytes/invalid enums exclude accepted graph; semantic defects may pass shape.

## reverse_reconstruction

Checked g has unpack(pack(g))=g; canonical accepted b has pack(unpack(b))=b. Error class admits many inputs; path/metadata unrecoverable.

## header_precedence

magic/38→version→five enum sizes→length→checksum before arrays; version+checksum corruption reports version.

## length_safety

p<=B; require n<=B-p before addition; u64::MAX array length returns Err64bit.

## csr_shape

rows+1/0/total/monotone offsets; UTF8 and refs; inverse CSR agreement not checked.

## effect_roundtrip

Temp-file roundtrip exercised; direct overwrite not tested for disk-full/interruption durability.


## tests

### Actual-code checks

- `store::wave29_tests::enormous_array_length_is_error_not_panic`
- `store::wave29_tests::snapshot_roundtrip_and_precedence`

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective model saved before edits; nine existing sources read fully. Added bounded inline tests; store reader overflow repaired only after compiled old-code failing witness. No live corpus, teacher, model or database effects executed. Source syntax generated twice identically; partial lexical projection only.

### 2026-10-31 · tests

#### Evidence 2026-10-31

Each selected test passed twice debug and twice release after exact mutation restoration (15 named suite checks; all four sorted outcome sets equal, exits0). Compiled source control: old-witness.log (original overflow implementation, compiled assertion failure101). Controls compile successfully and fail an assertion with exit101. Store control is the original unsafe length addition before repair.

Finite graph/token/temp fixtures; public invalid states and unselected branches not exhausted

Whole CLI, live teacher/corpus/model/prior loading, filesystem failure/durability and huge inputs were not executed. No exact wrapper is advertised as a percentage surrogate. Added test blocks rustfmt-formatted independently; unrelated source formatting preserved. Strict broad Clippy not repeated: existing dependency/source warnings are outside this work.
