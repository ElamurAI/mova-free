# world-domains-judgment-boundary

## forward

Word-to-Node projection preserves position, form, head and relation, lowercases lemma and wraps UPOS. Matrix::global_in(genre) and judge return indices into that vector; inclusive score>=threshold creates humor reasons using original Word forms. Final domains sort by name. score_set compares exactly four memberships; unknown gold names are ignored.

## invariant

Projection/judgment/reason indices share cardinality and order. Supplied annotation stays unchanged. domains_with forwards genre verbatim, whereas load_set/eval normalize to real/tale. Labels do not prove humor or impossibility.

## cost

O(N) projection plus O(VN) judge scans/lookups. Four-label membership scoring follows classification. Parser/repair and evaluation I/O costs remain unexecuted dependencies.

## composition

Actual callers: domains supplies default_cfg; score_set classifies supplied Labelled rows; run/load_set/eval annotate and repair first. Native tests execute domains_with -> Matrix::global_in -> judge -> sense::arguments -> original-form reasons. tune/family build full default maps and alter explicit values; absent-map fallback is a deliberate public-API semantic repair.

## epoch-memory

Initial source-bound epoch. Old native witnesses exposed role-index panic and spurious formal activation with empty overrides. Guards/default resolution repair these; explicit finite values and sparse-cell gap semantics remain. No optimization, surrogate or percentage bound is claimed.

## checks

cargo test -p world --lib domains::wave11_models -- --test-threads=1. Root ../model.md#evidence records repeated profiles and mutants. Partial syntax leaves macro bodies opaque; no semantic AST, parser, teacher or corpus execution.

## behavior-complement

Universe: terminating scoped calls and their outputs. Inner absent-behavior model excludes outputs contradicting: Projection/judgment/reason indices share cardinality and order. Supplied annotation stays unchanged. domains_with forwards genre verbatim, whereas load_set/eval normalize to real/tale. Labels do not prove humor or impossibility. Unknown external behavior remains outside this exclusion.

## reverse-reconstruction

A humor reason constrains displayed forms, genre and chosen score but leaves other nodes, lexicon/config and annotation alternatives ambiguous. A 0..4 score reveals agreement count, not which labels agree.

## coupling

Fixed-judgment threshold fibers retain s>=t. Six thresholds in two genres natively check projection and exact reason text; threshold five removes rated 0..4 reasons. These are join checks, not parser validation.

All typed entries bind this scope and manifest source revision. Domain and observation/operator are stated above. Status: partial source-derived relations with finite evidence named in checks, assuming stable dependencies and bounded successful execution. Reverse relations preserve ambiguity, with no complete-search or causal claim.

## History

### 2026-10-30 · epoch-memory

2026-10-30 initial source-bound epoch. Old native witnesses exposed role-index panic and spurious formal activation with empty overrides. Guards/default resolution repair these; explicit finite values and sparse-cell gap semantics remain. No optimization, surrogate or percentage bound is claimed.
