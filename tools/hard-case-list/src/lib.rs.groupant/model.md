# mova-hard-case-list

## Descriptant

The one shared hard-cases list (mova-hard-case-v2): append-only found/learned/open events per parser (en, gparse) keyed by the first 16 hex of SHA-256 of the trimmed text; reads v1 lines; record, sync_source, recheck, state. Source: `tools/hard-case-list/src/lib.rs`.

## Forward approximant

state(lines) = fold over events: found sets the case's record and opens the event's parser view; learned/open set that view. record(found, parser) appends found for keys not open for the parser (or open without the licence the new report brings). sync_source(source, current) appends found for new keys and learned for that source's open keys missing from current. recheck(check) appends learned for judged-well open cases and open for learned cases judged bad (all=true); unjudged cases (None) unchanged.

## Invariant

Append-only under an exclusive flock with fsync; one key per trimmed text; a parser view changes only by an event naming that parser; other sources' cases are never closed by a source sync; nothing is training-authorized.

## Cost

O(L) per operation for L list lines (the whole file is read); appends are one write.

## Composition

Used by tools/topic-corpus (hardcases.rs), the internal Wikipedia corpus tool (not in the public tree) and tools/hard-cases (wave, judge, main). Parent: hard-cases.

## behavior_complement

Does not delete or compact lines, does not decide training admission, does not judge a sentence itself (callers pass the judge), does not authenticate writers. Counterexample guarded by tests: a topic-corpus sync never learns a Wikipedia case. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

From the list, each case's latest found record (text, source, document pointer, licence, judges) and per-parser history are recoverable; the failed parse is recoverable only when draft_conllu was recorded (Wikipedia). Many event orders map to one state: no unique inverse.

## Tests

hard-case-list unit tests (3): v1 lines read as v2 with per-parser views; record dedupes, licence completion, source-scoped sync and reopening; recheck learns, reopens regressions (all=true), honours the source filter and leaves unjudged cases unchanged (red controls). Finite fixtures and live samples; not a proof.

## History

### 2026-11-04 · Tests

hard-case-list unit tests (2026-11-04, 3): v1 lines read as v2 with per-parser views; record dedupes, licence completion, source-scoped sync and reopening; recheck learns, reopens regressions (all=true), honours the source filter and leaves unjudged cases unchanged (red controls). Finite fixtures and live samples; not a proof.
