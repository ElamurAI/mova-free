# latex-build-stage0

## descriptant

### latex-build-stage0

is_ident(s) = first(s).is_ascii_uppercase() AND every remaining scalar is_ascii_alphanumeric(). Empty has no first scalar and returns false.

## approximant

### Scoped forward relation

is_ident(s) = first(s).is_ascii_uppercase() AND every remaining scalar is_ascii_alphanumeric(). Empty has no first scalar and returns false.

This is an exact source-derived partial model, not an executable approximate replacement.

## invariant

### Scoped invariants

Outputs true for empty, lowercase-leading, non-ASCII or underscore-containing names are excluded. This predicate has no file writes or table mutation.

The accepted language excludes ordinary Rust identifiers such as Alpha_beta; it is the generator's stricter variant convention.

main calls this predicate before symbol uniqueness/class/form validation. Its false result triggers that caller's identifier assertion; the predicate itself does not panic on empty input.

## composition

### Scoped interaction

Build validation consumes this boolean for sym and other enum row IDs; lexical acceptance does not prove a generated enum is complete.

The manifest binds this stage to the unchanged parent syntax projection and exact source byte range. Parent cost remains an upper accounting context; no independent whole-file invocation or execution count is assigned to this fragment.

## models

## behavior_complement

Outputs true for empty, lowercase-leading, non-ASCII or underscore-containing names are excluded. This predicate has no file writes or table mutation.

## reverse_reconstruction

Preimage(true) is [A-Z][A-Za-z0-9]*; preimage(false) is every other UTF-8 string. A and Z9 share true, so the bool cannot reconstruct spelling.

## identifier_language

The accepted language excludes ordinary Rust identifiers such as Alpha_beta; it is the generator's stricter variant convention.

## validation_order

main calls this predicate before symbol uniqueness/class/form validation. Its false result triggers that caller's identifier assertion; the predicate itself does not panic on empty input.

## tests

### Evidence

wave28_tests::canonical_first_form_and_variant_order; wave28_tests::variant_identifier_boundary

Selected library filters ast:: lex:: macros:: parse:: print:: table:: and integration gate excluding document_extraction_with_preamble_macros and random_token_soup_never_panics_and_roundtrips. Direct rustc --test latex/build.rs exercises private helpers. All selected tests twice debug/release with identical sorted names. Four compiled mutants at build/AST/parser/printer boundaries fail assertions then exact restoration passes. These do not individually cover every source. Build-main failure effects, allocation failures and universal roundtrip are unproved. Strict Clippy reports existing lints. No corpus/live VMM/API/GPU/full-school/held-out tests.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: replaced stage topic lists with concrete transitions, exclusions and compatible antecedents. Four meaningful typed views replace the previous six-view bookkeeping catalog. Shared syntax and source byte bounds are unchanged.

### 2026-10-31 · tests

Heading `### Evidence — 2026-10-31` stays in tests as `### Evidence`.
