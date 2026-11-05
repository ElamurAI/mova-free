# latex-test-gate-stage2

## descriptant

### latex-test-gate-stage2

Macro test scans fixed definitions and requires (defined,skipped)=(7,0), compares parse_with expansion against explicit expected trees, checks local definition use and two loop errors. Document test and deterministic 30000-case soup are defined later but excluded from selected runs.

## approximant

### Scoped forward relation

Macro test scans fixed definitions and requires (defined,skipped)=(7,0), compares parse_with expansion against explicit expected trees, checks local definition use and two loop errors. Document test and deterministic 30000-case soup are defined later but excluded from selected runs.

This is an exact source-derived partial model, not an executable approximate replacement.

## invariant

### Scoped invariants

A local macro loop cannot hang in these selected witnesses: the call must return MacroLoop. Passing macro equalities does not validate external document extraction or the skipped PRNG population.

oldsqrt is captured before sqrt is redefined; comparing both parses witnesses raw built-in capture preventing recursion for this fixed setup.

Only macros_newcommand_def_let_declareop runs from this fragment. Document and soup test bodies remain source-inspected evidence, with zero runtime coverage claimed in this epoch.

## composition

### Scoped interaction

This selected macro test composes scanning, let capture, token expansion, parser normalization and structural equality. Equality omits spans and cannot recover the preamble.

The manifest binds this stage to the unchanged parent syntax projection and exact source byte range. Parent cost remains an upper accounting context; no independent whole-file invocation or execution count is assigned to this fragment.

## models

## behavior_complement

A local macro loop cannot hang in these selected witnesses: the call must return MacroLoop. Passing macro equalities does not validate external document extraction or the skipped PRNG population.

## reverse_reconstruction

Equal expanded trees admit different macro bodies, spelling and source positions. let oldsqrt captures prior meaning, so later sqrt redefinition plus oldsqrt invocation is compatible with the same result as built-in sqrt without definitions.

## capture_witness

oldsqrt is captured before sqrt is redefined; comparing both parses witnesses raw built-in capture preventing recursion for this fixed setup.

## execution_selection

Only macros_newcommand_def_let_declareop runs from this fragment. Document and soup test bodies remain source-inspected evidence, with zero runtime coverage claimed in this epoch.

## tests

### Evidence

7 integration tests via --test gate with document and soup skipped

Selected library filters ast:: lex:: macros:: parse:: print:: table:: and integration gate excluding document_extraction_with_preamble_macros and random_token_soup_never_panics_and_roundtrips. Direct rustc --test latex/build.rs exercises private helpers. All selected tests twice debug/release with identical sorted names. Four compiled mutants at build/AST/parser/printer boundaries fail assertions then exact restoration passes. These do not individually cover every source. Build-main failure effects, allocation failures and universal roundtrip are unproved. Strict Clippy reports existing lints. No corpus/live VMM/API/GPU/full-school/held-out tests.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: replaced stage topic lists with concrete transitions, exclusions and compatible antecedents. Four meaningful typed views replace the previous six-view bookkeeping catalog. Shared syntax and source byte bounds are unchanged.

### 2026-10-31 · tests

Heading `### Evidence — 2026-10-31` stays in tests as `### Evidence`.
