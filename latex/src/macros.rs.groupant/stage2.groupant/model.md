# latex-macros-stage2

## descriptant

### latex-macros-stage2

match_pattern walks pattern. Literal tokens must match next input (skip spaces unless literal Space). Param consumes a normal argument when followed by no literals, otherwise collect_until scans through whole groups until the delimiter suffix appears at depth0. Exactly one outer group is stripped from the captured argument.

## approximant

### Scoped forward relation

match_pattern walks pattern. Literal tokens must match next input (skip spaces unless literal Space). Param consumes a normal argument when followed by no literals, otherwise collect_until scans through whole groups until the delimiter suffix appears at depth0. Exactly one outer group is stripped from the captured argument.

This is an exact source-derived partial model, not an executable approximate replacement.

## invariant

### Scoped invariants

A delimiter nested inside braces cannot terminate capture. Mismatch returns None without rewinding consumed literals. collect_until restores End but not an unmatched depth-zero Close.

At depth0 and out ending with delimiter D, remove |D| tokens then return; comparison uses token kinds, ignoring token positions. First qualifying occurrence wins.

single_group requires first Open and its matching Close at the final token. {a}{b} retains both groups, while {a,b} loses only its outer braces.

## composition

### Scoped interaction

Parser expansion uses returned arguments for #k substitution. Pattern success consumes delimiters as syntax; they are absent from argument payloads unless explicitly in the macro body.

The manifest binds this stage to the unchanged parent syntax projection and exact source byte range. Parent cost remains an upper accounting context; no independent whole-file invocation or execution count is assigned to this fragment.

## models

## behavior_complement

A delimiter nested inside braces cannot terminate capture. Mismatch returns None without rewinding consumed literals. collect_until restores End but not an unmatched depth-zero Close.

## reverse_reconstruction

Captured a,b can arise from unbraced a,b followed by delimiter or from {a,b} followed by delimiter; outer-brace provenance is erased. Failed matching leaves many compatible consumed prefixes. Pattern plus returned args alone cannot reconstruct skipped spaces.

## delimiter_suffix

At depth0 and out ending with delimiter D, remove |D| tokens then return; comparison uses token kinds, ignoring token positions. First qualifying occurrence wins.

## outer_group_quotient

single_group requires first Open and its matching Close at the final token. {a}{b} retains both groups, while {a,b} loses only its outer braces.

## tests

### Evidence

macros::tests::scans_definitions; macros::tests::delimited_def

Selected library filters ast:: lex:: macros:: parse:: print:: table:: and integration gate excluding document_extraction_with_preamble_macros and random_token_soup_never_panics_and_roundtrips. Direct rustc --test latex/build.rs exercises private helpers. All selected tests twice debug/release with identical sorted names. Four compiled mutants at build/AST/parser/printer boundaries fail assertions then exact restoration passes. These do not individually cover every source. Build-main failure effects, allocation failures and universal roundtrip are unproved. Strict Clippy reports existing lints. No corpus/live VMM/API/GPU/full-school/held-out tests.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: replaced stage topic lists with concrete transitions, exclusions and compatible antecedents. Four meaningful typed views replace the previous six-view bookkeeping catalog. Shared syntax and source byte bounds are unchanged.

### 2026-10-31 · tests

Heading `### Evidence — 2026-10-31` stays in tests as `### Evidence`.
