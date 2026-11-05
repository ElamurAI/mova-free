# math-lexicon-build

## descriptant

### Source-bound description

Read lexicon.tsv; skip column-zero comments/blank lines; require3 columns then unique form then known class. Emit match arms in input order and fallback None. frac parses signed i64; zero denominator accepted, mult/times discard denominator. Output written only after all rows processed.

## approximant

### Forward relation

Read lexicon.tsv; skip column-zero comments/blank lines; require3 columns then unique form then known class. Emit match arms in input order and fallback None. frac parses signed i64; zero denominator accepted, mult/times discard denominator. Output written only after all rows processed.

Domain: UTF-8 TSV and writable OUT_DIR; empty TSV emits fallback only.

Executable Rust approximant unsupported: categorical/effect observations have no useful percentage metric yet. Future caps1/2/5/10% only; no automatic substitution.

## invariant

### Invariant and cost

Duplicate or unknown class excludes completed output for that invocation. Zero denominator not excluded; invalid enum identifier can fail later compilation.

Ordering: Read→columns→duplicates→class→OUT_DIR→write. Duplicate with invalid class reports duplicate first.

Cost: O(bytes) plus expected form hash work, O(bytes) storage. Integer parsing rejects out-of-range; filesystem/allocation panics.

## composition

### Composition and epoch memory

Generated source included by lex.rs; compiler there validates enum identifiers.

Fraction helper and ordinary Cargo build exercised; malformed TSV generation pipeline not injected.

Partial syntax and finite tests do not establish all caller assumptions.

## models

## behavior_complement

Duplicate or unknown class excludes completed output for that invocation. Zero denominator not excluded; invalid enum identifier can fail later compilation.

## reverse_reconstruction

Generated Num(n,d) admits alternate integer spellings. mult/times collapse denominators. Comments and blank lines lost; TSV not uniquely recoverable.

## failure_precedence

Read→columns→duplicates→class→OUT_DIR→write. Duplicate with invalid class reports duplicate first.

## representation_transport

Generated source included by lex.rs; compiler there validates enum identifiers.

## resource_boundary

O(bytes) plus expected form hash work, O(bytes) storage. Integer parsing rejects out-of-range; filesystem/allocation panics.

## domain_boundary

UTF-8 TSV and writable OUT_DIR; empty TSV emits fallback only.

## tests

### Evidence

Selected actual-source checks: `rustc --test math/build.rs: wave28::fraction_syntax`. Exact commands, numeric statuses, normalized outcomes, compiled assertion controls and restoration receipts are in the external wave28 mathcontrol run.

Fraction helper and ordinary Cargo build exercised; malformed TSV generation pipeline not injected.

No live teacher/API, trained model, corpus/held-out, GPU, DB or expensive benchmark run. Parent syntax generated twice identically. Scope remains partial.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: prospective model before bounded synthetic tests; production behavior unchanged. Fraction helper and ordinary Cargo build exercised; malformed TSV generation pipeline not injected.

### 2026-10-31 · tests

Heading `### Evidence, 2026-10-31` stays in tests as `### Evidence`.
