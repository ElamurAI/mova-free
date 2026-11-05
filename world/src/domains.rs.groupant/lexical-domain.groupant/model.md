# world-domains-lexical-domain

## forward

Expression cues pass POS/content and main-gloss filters. Legal-register counts can activate formal; formal with children-register terms activates humor. push deduplicates exact reason strings. Two distinct slang reasons enable secondary content readings outside expression spans; repeated identical reasons do not count twice.

## invariant

Word order and forms are preserved; supplied annotations are immutable. Domain labels are dictionary/register judgments. Secondary readings require content POS and uncovered position; no actual speaker intent is inferred.

## cost

O(N) case normalization plus external expression/lens searches; Vec span membership adds O(NS) worst-case for S covered positions. Reason dedup can be quadratic in reasons; external lexical costs remain opaque.

## composition

Actual callers: domains supplies default_cfg; score_set classifies supplied Labelled rows; run/load_set/eval annotate and repair first. Native tests execute domains_with -> Matrix::global_in -> judge -> sense::arguments -> original-form reasons. tune/family build full default maps and alter explicit values; absent-map fallback is a deliberate public-API semantic repair.

## epoch-memory

Initial source-bound epoch. Old native witnesses exposed role-index panic and spurious formal activation with empty overrides. Guards/default resolution repair these; explicit finite values and sparse-cell gap semantics remain. No optimization, surrogate or percentage bound is claimed.

## checks

cargo test -p world --lib domains::wave11_models -- --test-threads=1. Root ../model.md#evidence records repeated profiles and mutants. Partial syntax leaves macro bodies opaque; no semantic AST, parser, teacher or corpus execution.

## behavior-complement

Universe: terminating scoped calls and their outputs. Inner absent-behavior model excludes outputs contradicting: Word order and forms are preserved; supplied annotations are immutable. Domain labels are dictionary/register judgments. Secondary readings require content POS and uncovered position; no actual speaker intent is inferred. Unknown external behavior remains outside this exclusion.

## reverse-reconstruction

A reason set retains all annotated sequences/configurations and stable lexical snapshots producing it. Duplicate reason occurrences collapse; occurrence counts and original order are not uniquely recoverable.

## coupling

Reason identity intervention inserts an exact duplicate reason without changing cue count; distinct reasons increase it. Native checks cover dedup and all sixteen four-domain gold-subset scores on empty annotations.

All typed entries bind this scope and manifest source revision. Domain and observation/operator are stated above. Status: partial source-derived relations with finite evidence named in checks, assuming stable dependencies and bounded successful execution. Reverse relations preserve ambiguity, with no complete-search or causal claim.

## History

### 2026-10-30 · epoch-memory

2026-10-30 initial source-bound epoch. Old native witnesses exposed role-index panic and spurious formal activation with empty overrides. Guards/default resolution repair these; explicit finite values and sparse-cell gap semantics remain. No optimization, surrogate or percentage bound is claimed.
