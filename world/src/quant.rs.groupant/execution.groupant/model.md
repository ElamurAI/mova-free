# world-quant-execution

Command parsing, sentence/number extraction, run, parse_script and check.

## forward

Sentence-prefix validation applies only to numeric literals. References read the current command-order state and carry no sentence provenance. Thus @2 set a.x=19 followed by @1 set b.x=a.x and @1 ask b.x remains green when 19 occurs only in sentence 2. This is a documented current limitation, not complete temporal provenance or a proposed behavior change.

parse_line validates verb, principle and dotted targets; ask bypasses principle validation. run gates literals then evaluates and applies each command. parse_script retains successful commands alongside parse diagnostics; check merges all errors and available answer mismatch.

## invariant

Missing source cannot fund move; existing target required for arithmetic updates. Last successful ask wins. Command anchors need not increase. Missing ask always emits an error.

## cost

Extract each visited sentence once; membership scans current prefix per literal. Rational costs grow with bit lengths; command processing is sequential.

## composition

Parsing feeds ordered execution; literal collection precedes evaluation against state, and check merges parse errors, execution diagnostics and gold inequality. Text extraction is an external exact dependency. Numeric-origin errors do not suppress effects, unlike evaluation and anchor errors; a parent must preserve this distinction. Dataset/teacher adapters do not establish script semantics.

## epoch-memory

Regressions first demonstrated backward-anchor number leakage and destination credit from missing transfer sources. Prefix endpoints now restrict each command to its own sentence prefix while preserving incremental extraction; missing-source transfer skips effects. Diagnostic-only literal errors remain unchanged. No corpus or teacher calls.

## checks

See ../model.md#tests and native quant::wave8_models tests. Models are source-derived partial relations, with finite evidence rather than general proof. Syntax projection excludes macro expansion, name resolution and inferred types. No executable approximant: exact gates and categorical choices have no justified percentage metric here.

## behavior-complement

In transition-result universe, silent update to undefined arithmetic target and credit from missing transfer source are absent. Backward anchors themselves are allowed; excluding them would be a false model.

## reverse-reconstruction

Fixed add by d reconstructs x=x'-d. Multiply by zero with observed zero admits every prior rational. Answer-only fibers also retain errorful histories; observing an answer does not imply green check.

## anchor-history

Sentence-prefix validation applies only to numeric literals. References read the current command-order state and carry no sentence provenance. Thus @2 set a.x=19 followed by @1 set b.x=a.x and @1 ask b.x remains green when 19 occurs only in sentence 2. This is a documented current limitation, not complete temporal provenance or a proposed behavior change.

prefix_ends[j] bounds membership even after larger anchors were scanned. Literal errors are diagnostic-only; backward-anchor test checks the number of diagnostics, not rollback.

## error-atomicity

Missing source, invalid anchor and failed expression skip command effects. Numeric-origin diagnostics do not. Native missing-source fixture preserves destination 7 instead of erroneous 9.

## History

### 2026-10-30 · epoch-memory

2026-10-30: regressions first demonstrated backward-anchor number leakage and destination credit from missing transfer sources. Prefix endpoints now restrict each command to its own sentence prefix while preserving incremental extraction; missing-source transfer skips effects. Diagnostic-only literal errors remain unchanged. No corpus or teacher calls.
