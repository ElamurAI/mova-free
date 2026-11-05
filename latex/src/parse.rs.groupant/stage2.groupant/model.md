# latex-parse-stage2

## descriptant

### latex-parse-stage2

genfrac reads left,right,thickness,style,numerator,denominator before validating delimiter forms. Zero-thickness predicate trims leading0/dots then requires remaining chars ASCII letters; matching parentheses choose binomial, otherwise Atop or frac styles. root collects to literal Cmd(of), parses marked index then body. command first expands nonraw macros, then dispatches Entry; unknown names delegate to stage1.

## approximant

### Scoped forward relation

genfrac reads left,right,thickness,style,numerator,denominator before validating delimiter forms. Zero-thickness predicate trims leading0/dots then requires remaining chars ASCII letters; matching parentheses choose binomial, otherwise Atop or frac styles. root collects to literal Cmd(of), parses marked index then body. command first expands nonraw macros, then dispatches Entry; unknown names delegate to stage1.

This is an exact source-derived partial model, not an executable approximate replacement.

## invariant

### Scoped invariants

Bad genfrac delimiters cannot outrank a prior missing denominator. Unicode escapes for controls or reserved TeX syntax characters cannot yield atoms. Space/Ignore commands yield no atom; Phantom discards its raw argument rather than preserving hidden structure.

Space/Ignore drop token, SpaceArg/IgnoreArg consume raw arg then drop, TextColor drops color and splices content, Eqno consumes until boundary. Their discarded arguments have unconstrained preimages.

Numeric code parsing accepts decimal or x/X hex; successful char must be noncontrol and outside the reserved set. Table lookup/font normalization can map the admitted scalar to a symbol/font node rather than Char.

## composition

### Scoped interaction

Entry dispatch builds AST values consumed by printers; parser unit tests in this scope witness only selected branches, not full TeX validation.

The manifest binds this stage to the unchanged parent syntax projection and exact source byte range. Parent cost remains an upper accounting context; no independent whole-file invocation or execution count is assigned to this fragment.

## models

## behavior_complement

Bad genfrac delimiters cannot outrank a prior missing denominator. Unicode escapes for controls or reserved TeX syntax characters cannot yield atoms. Space/Ignore commands yield no atom; Phantom discards its raw argument rather than preserving hidden structure.

## reverse_reconstruction

frac12 and frac{1}{2} yield equal fractions. genfrac style outside0/1 falls back to plain, so original style text is lost. Thickness strings accepted by the zero predicate—including letter-only strings—share zero behavior; exact TeX dimension semantics cannot be reconstructed.

## command_erasure

Space/Ignore drop token, SpaceArg/IgnoreArg consume raw arg then drop, TextColor drops color and splices content, Eqno consumes until boundary. Their discarded arguments have unconstrained preimages.

## unicode_admission

Numeric code parsing accepts decimal or x/X hex; successful char must be noncontrol and outside the reserved set. Table lookup/font normalization can map the admitted scalar to a symbol/font node rather than Char.

## tests

### Evidence

parse::tests (7 named tests); integration unbalanced_is_an_error_with_position and macros_newcommand_def_let_declareop

Selected library filters ast:: lex:: macros:: parse:: print:: table:: and integration gate excluding document_extraction_with_preamble_macros and random_token_soup_never_panics_and_roundtrips. Direct rustc --test latex/build.rs exercises private helpers. All selected tests twice debug/release with identical sorted names. Four compiled mutants at build/AST/parser/printer boundaries fail assertions then exact restoration passes. These do not individually cover every source. Build-main failure effects, allocation failures and universal roundtrip are unproved. Strict Clippy reports existing lints. No corpus/live VMM/API/GPU/full-school/held-out tests.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: replaced stage topic lists with concrete transitions, exclusions and compatible antecedents. Four meaningful typed views replace the previous six-view bookkeeping catalog. Shared syntax and source byte bounds are unchanged.

### 2026-10-31 · tests

Heading `### Evidence — 2026-10-31` stays in tests as `### Evidence`.
