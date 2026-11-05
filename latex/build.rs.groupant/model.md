# latex-build

## descriptant

### latex-build

En::canon/alt retain first form; variants retain first-seen order. main validates row arity, identifier, symbol uniqueness and class before forms; explicit characters claim slots before automatic text fallback. Fixed enum emission order avoids HashMap iteration nondeterminism; writes OUT_DIR/table.rs only after validation.

## approximant

### Forward relation

En::canon/alt retain first form; variants retain first-seen order. main validates row arity, identifier, symbol uniqueness and class before forms; explicit characters claim slots before automatic text fallback. Fixed enum emission order avoids HashMap iteration nondeterminism; writes OUT_DIR/table.rs only after validation.

No useful approximate executable metric for exact categorical syntax; future caps1/2/5/10% only.

## invariant

### Domain, invariant and cost

En::canon/alt retain first form; variants retain first-seen order. main validates row arity, identifier, symbol uniqueness and class before forms; explicit characters claim slots before automatic text fallback. Fixed enum emission order avoids HashMap iteration nondeterminism; writes OUT_DIR/table.rs only after validation.

A, A0 accepted; empty, lowercase initial, underscore and Unicode variant names rejected. Empty table can emit unusable empty enums; no whole-table semantic completeness guarantee.

Table bytes B, variants V, forms F: parsing O(B+V²) worst-case linear En::add membership plus expected hash operations; emitted bytes E retained O(B+F+E).

## composition

### Composition

En::canon/alt retain first form; variants retain first-seen order. main validates row arity, identifier, symbol uniqueness and class before forms; explicit characters claim slots before automatic text fallback. Fixed enum emission order avoids HashMap iteration nondeterminism; writes OUT_DIR/table.rs only after validation.

Build generates table enums consumed by parser dispatch and printers. Lexer tokens feed macros and parser; parser returns AST whose equality ignores spans. Printers and gate compare that quotient, not original bytes. Shared parent syntax backs scoped stages; stages are byte-bound projections, not independent Rust modules.

## models

## behavior_complement

Duplicate form acceptance and overwrite of the first canonical form are excluded for validated successful compilation.

## reverse_reconstruction

Compatible inputs are tables with identical ordered variants, canonical first forms and lookup maps; comments, blank lines and redundant alternate canonical declarations can differ.

## canonical_selection

First form is retained separately for canonical and infix/plain forms.

## duplicate_precedence

Symbol identity and class checked before form duplication; references after whole pass.

## generated_transport

SYM arrays share row order and enum discriminants; include in table.rs consumes matching arrays.

## effect_boundary

Read table then validate and construct output, finally require OUT_DIR and write; errors panic without transaction guarantee.

## tests

### Evidence

wave28_tests::canonical_first_form_and_variant_order; wave28_tests::variant_identifier_boundary

Selected library filters ast:: lex:: macros:: parse:: print:: table:: and integration gate excluding document_extraction_with_preamble_macros and random_token_soup_never_panics_and_roundtrips. Direct rustc --test latex/build.rs exercises private helpers. All selected tests twice debug/release with identical sorted names. Four compiled mutants at build/AST/parser/printer boundaries fail assertions then exact restoration passes. These do not individually cover every source. Build-main failure effects, allocation failures and universal roundtrip are unproved. Strict Clippy reports existing lints. No corpus/live VMM/API/GPU/full-school/held-out tests.

## History

### 2026-10-31 · descriptant

Epoch2026-10-31: full source reviewed before edits; prospective models retained externally. No optimization.

### 2026-10-31 · tests

Heading `### Evidence — 2026-10-31` stays in tests as `### Evidence`.
