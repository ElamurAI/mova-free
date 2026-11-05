# latex-macros-stage0

## descriptant

### latex-macros-stage0

scan lexes input and dispatches recognized definition commands in order. Successful overwrite definitions replace the map; provide inserts only absent names but increments defined either way. Reader error increments skipped after its consumed prefix. let_meaning clones an existing target macro or saves CmdRaw(target) when absent.

## approximant

### Scoped forward relation

scan lexes input and dispatches recognized definition commands in order. Successful overwrite definitions replace the map; provide inserts only absent names but increments defined either way. Reader error increments skipped after its consumed prefix. let_meaning clones an existing target macro or saves CmdRaw(target) when absent.

This is an exact source-derived partial model, not an executable approximate replacement.

## invariant

### Scoped invariants

provide cannot replace an existing mapping; malformed definitions cannot roll back earlier successful definitions. names() cannot promise lexical order because it exposes HashMap keys.

let(a,b) copies b's current Macro value, including nargs/default/pattern/body. Missing b becomes a zero-argument CmdRaw body, which bypasses later macro expansion.

defined counts successful reads, not map growth; providing an existing name adds1 with zero size delta. skipped counts reader failures, not all unknown commands.

## composition

### Scoped interaction

The definition readers in stage1 consume Stream. scan does not reset the stream on error; parser local definitions reuse the readers but convert errors to BadDefinition instead of skipped statistics.

The manifest binds this stage to the unchanged parent syntax projection and exact source byte range. Parent cost remains an upper accounting context; no independent whole-file invocation or execution count is assigned to this fragment.

## models

## behavior_complement

provide cannot replace an existing mapping; malformed definitions cannot roll back earlier successful definitions. names() cannot promise lexical order because it exposes HashMap keys.

## reverse_reconstruction

Final map loses overwritten definitions and ignored provides. A map with R->body can result from one definition or many redefinitions ending in body; ScanStats constrains counts but not those bodies. Captured let meaning does not recover whether its source target was later changed.

## captured_meaning

let(a,b) copies b's current Macro value, including nargs/default/pattern/body. Missing b becomes a zero-argument CmdRaw body, which bypasses later macro expansion.

## scan_accounting

defined counts successful reads, not map growth; providing an existing name adds1 with zero size delta. skipped counts reader failures, not all unknown commands.

## tests

### Evidence

macros::tests::scans_definitions; macros::tests::delimited_def

Selected library filters ast:: lex:: macros:: parse:: print:: table:: and integration gate excluding document_extraction_with_preamble_macros and random_token_soup_never_panics_and_roundtrips. Direct rustc --test latex/build.rs exercises private helpers. All selected tests twice debug/release with identical sorted names. Four compiled mutants at build/AST/parser/printer boundaries fail assertions then exact restoration passes. These do not individually cover every source. Build-main failure effects, allocation failures and universal roundtrip are unproved. Strict Clippy reports existing lints. No corpus/live VMM/API/GPU/full-school/held-out tests.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: replaced stage topic lists with concrete transitions, exclusions and compatible antecedents. Four meaningful typed views replace the previous six-view bookkeeping catalog. Shared syntax and source byte bounds are unchanged.

### 2026-10-31 · tests

Heading `### Evidence — 2026-10-31` stays in tests as `### Evidence`.
