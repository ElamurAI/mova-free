# latex-test-gate-stage0

## descriptant

### latex-test-gate-stage0

SUITE iterates fixed formula literals; ok requires parse success. For each tree, unknowns must be empty, gate::check must return Ok, and to_latex(parse(to_latex(tree))) must equal first print. Failures accumulate then one assert requires an empty list.

## approximant

### Scoped forward relation

SUITE iterates fixed formula literals; ok requires parse success. For each tree, unknowns must be empty, gate::check must return Ok, and to_latex(parse(to_latex(tree))) must equal first print. Failures accumulate then one assert requires an empty list.

This is an exact source-derived partial model, not an executable approximate replacement.

## invariant

### Scoped invariants

The selected test cannot pass if any listed fixture fails parsing, retains unknowns, fails structural gate or changes canonical bytes on the second print. It does not reject a fault confined to unlisted inputs.

gate compares Node equality, which erases Span coordinates. The additional fixed-point assertion constrains printer stability but still does not validate original coordinates.

Success is conjunction over the fixed SUITE, not a random-sampling confidence interval or full TeX completeness theorem. No denominator beyond this suite is justified.

## composition

### Scoped interaction

Fixture test joins parse, unknown traversal, canonical printer and gate. It runs independently of the skipped random-token and document tests.

The manifest binds this stage to the unchanged parent syntax projection and exact source byte range. Parent cost remains an upper accounting context; no independent whole-file invocation or execution count is assigned to this fragment.

## models

## behavior_complement

The selected test cannot pass if any listed fixture fails parsing, retains unknowns, fails structural gate or changes canonical bytes on the second print. It does not reject a fault confined to unlisted inputs.

## reverse_reconstruction

A passing transcript admits all implementations agreeing on this finite suite, including ones with arbitrary differences elsewhere. Span-only changes can pass structural gate; canonical-byte agreement does not reconstruct fixture whitespace/alias choices.

## quotient_oracle

gate compares Node equality, which erases Span coordinates. The additional fixed-point assertion constrains printer stability but still does not validate original coordinates.

## finite_conjunction

Success is conjunction over the fixed SUITE, not a random-sampling confidence interval or full TeX completeness theorem. No denominator beyond this suite is justified.

## tests

### Evidence

7 integration tests via --test gate with document and soup skipped

Selected library filters ast:: lex:: macros:: parse:: print:: table:: and integration gate excluding document_extraction_with_preamble_macros and random_token_soup_never_panics_and_roundtrips. Direct rustc --test latex/build.rs exercises private helpers. All selected tests twice debug/release with identical sorted names. Four compiled mutants at build/AST/parser/printer boundaries fail assertions then exact restoration passes. These do not individually cover every source. Build-main failure effects, allocation failures and universal roundtrip are unproved. Strict Clippy reports existing lints. No corpus/live VMM/API/GPU/full-school/held-out tests.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: replaced stage topic lists with concrete transitions, exclusions and compatible antecedents. Four meaningful typed views replace the previous six-view bookkeeping catalog. Shared syntax and source byte bounds are unchanged.

### 2026-10-31 · tests

Heading `### Evidence — 2026-10-31` stays in tests as `### Evidence`.
