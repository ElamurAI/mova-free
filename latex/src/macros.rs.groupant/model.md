# latex-macros

## descriptant

### latex-macros

scan visits definition commands in token order, updates hash table on success, counts ignored provides as defined, counts malformed readers as skipped. let clones existing macro or captures CmdRaw. read_def validates body before sequential parameter numbers. match_pattern greedily consumes literal delimiters at brace depth zero.

## approximant

### Forward relation

scan visits definition commands in token order, updates hash table on success, counts ignored provides as defined, counts malformed readers as skipped. let clones existing macro or captures CmdRaw. read_def validates body before sequential parameter numbers. match_pattern greedily consumes literal delimiters at brace depth zero.

No useful approximate executable metric for exact categorical syntax; future caps1/2/5/10% only.

## invariant

### Domain, invariant and cost

scan visits definition commands in token order, updates hash table on success, counts ignored provides as defined, counts malformed readers as skipped. let clones existing macro or captures CmdRaw. read_def validates body before sequential parameter numbers. match_pattern greedily consumes literal delimiters at brace depth zero.

nargs accepted0..9 for newcommand; default read only when nargs>0. Malformed input consumed progressively. Source lifetimes own borrowed names/tokens.

O(B) lex plus definition bodies retained; expected hash lookup. Delimited scan O(T*L), where L delimiter length; cloning captured macros proportional to body/pattern/default sizes.

## composition

### Composition

scan visits definition commands in token order, updates hash table on success, counts ignored provides as defined, counts malformed readers as skipped. let clones existing macro or captures CmdRaw. read_def validates body before sequential parameter numbers. match_pattern greedily consumes literal delimiters at brace depth zero.

Build generates table enums consumed by parser dispatch and printers. Lexer tokens feed macros and parser; parser returns AST whose equality ignores spans. Printers and gate compare that quotient, not original bytes. Shared parent syntax backs scoped stages; stages are byte-bound projections, not independent Rust modules.

## models

## behavior_complement

providecommand cannot overwrite an existing definition. Failed pattern matching does not promise rollback. HashMap names does not promise sorted iteration.

## reverse_reconstruction

Final macro table admits arbitrary overwritten definitions and ignored provides; scan stats constrain success/error counts but not unique source. let copy retains the captured old meaning despite later redefinition.

## overwrite_policy

Later new/renew/def replace; provide only absent, but every successful read increments defined.

## capture_semantics

let existing macro clones value; built-in target saved CmdRaw to avoid future expansion.

## failure_precedence

read_def missing/unclosed body can precede out-of-order parameter diagnosis.

## delimiter_scope

Delimiter matches only at zero brace depth; exactly one group loses braces.

## tests

### Evidence

macros::tests::scans_definitions; macros::tests::delimited_def

Selected library filters ast:: lex:: macros:: parse:: print:: table:: and integration gate excluding document_extraction_with_preamble_macros and random_token_soup_never_panics_and_roundtrips. Direct rustc --test latex/build.rs exercises private helpers. All selected tests twice debug/release with identical sorted names. Four compiled mutants at build/AST/parser/printer boundaries fail assertions then exact restoration passes. These do not individually cover every source. Build-main failure effects, allocation failures and universal roundtrip are unproved. Strict Clippy reports existing lints. No corpus/live VMM/API/GPU/full-school/held-out tests.

## History

### 2026-10-31 · descriptant

Epoch2026-10-31: full source reviewed before edits; prospective models retained externally. No optimization.

### 2026-10-31 · tests

Heading `### Evidence — 2026-10-31` stays in tests as `### Evidence`.
