# corpus-events-publication

## descriptant

# corpus-events-publication

BookStats, book_stats, top, book_json, run and indexed show.

## Forward

Count events and original sentence postings; summarize roles and symmetric interactions; pool successful books under mutexes then sort final rows after joining.

## Invariant

First-match selection and sentence indices preserve source order. Count ties have canonical key order. Arithmetic assumes no integer overflow; IEEE NaN is filtered only by the source comparison. Counts retain role multiplicity, not unique entities.

## Cost

O(B log D+D log D), output/posting storage O(B); excludes parser and IO.

## Composition

Declared child relations compose through the same body, token arrays, maps and ranked records. Reverse composition existentially retains these intermediate values; complements cannot simply be unioned. Shared work is counted once. Parser and filesystem state remain latent dependencies.

## Epoch memory

## Gaps

Syntax is source-bound and partial: macro expansion, names/types, cfg evaluation and external modules absent. No live IO/parser/corpus/teacher/GPU run. Filesystem traces, thread scheduling, tokenizer/annotator/repair and external sentence splitting beyond bounded fixtures remain unverified dependencies. Models do not certify complete logical semantics.

## catalog

# corpus-events-publication typed models

## nonbehavior

Type: behavior_complement.

U=(BookStats,JSON). Excludes events_total not sum counts, empty actor keys and incorrect tie ordering. Oracle native aggregation.

## antecedents

Type: reverse_reconstruction.

JSON admits maps matching capped top-30 actor/interactions and top-60 rendered events plus sentence/total values. Tail, sentence allocation and show-string collisions remain ambiguous. Oracle proposed truncated-tail changes.

## support-count

Type: document_frequency.

Per-book map increments event documents once; cells first collapse multiple events sharing cell. Bound is successful input instances, not unique stems. Oracle proposed map composition.

## tail-fiber

Type: truncation_relation.

Given top-k L, omitted keys must rank after L by (-count,key); n=0 yields empty. Oracle native tie/truncation; zero cap proposed.

## partial-write

Type: failure_trace.

Per-book failed writes can coexist with successful totals; final write Err can follow earlier successful publications. Files do not uniquely reconstruct completion trace. Oracle source-only.

## indexed-display

Type: lookup_relation.

show uses precisely sentences(book_body(read_utf8)); indices repeated or out of bounds preserve order, missing maps '?'. Read failure propagates. Oracle source-only; no filesystem exercise.

## History

### 2026-10-30 · Epoch memory

2026-10-30 prospective prediction: native fixtures match marker reversal, first-role masking, Unicode byte admission, role multiplicity and degenerate NaN behavior. These are characterized, not policy changes. Only private register helpers were extracted; event production logic is unchanged. No executable approximation/error-cap claim is made.

### 2026-10-30 · document

Subject: source hash in manifest, epoch 2026-10-30. Scope: BookStats, book_stats, top, book_json, run and indexed show.. Domain: terminating Rust calls without integer overflow, narrowed below. Models are partial source-derived relations, not proofs. Complement is an inner bound in the stated input/output universe. Reconstruction retains all compatible antecedents, not a unique inverse. Oracle status is explicit below; ../model.md#tests records executed coverage.
