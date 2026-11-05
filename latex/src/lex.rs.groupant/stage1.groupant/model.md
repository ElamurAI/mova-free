# latex-lex-stage1

## descriptant

### latex-lex-stage1

next pops stack; peek reads last and peek2 next-to-last. push_front(t1..tk) extends reversed tokens so subsequent next yields t1..tk then the old stream. group_tokens tracks brace depth, returning at depth-zero Close; End is restored and yields failure. opt_tokens requires initial [, ignores ] within braces, restores depth-zero Close/End on failure.

## approximant

### Scoped forward relation

next pops stack; peek reads last and peek2 next-to-last. push_front(t1..tk) extends reversed tokens so subsequent next yields t1..tk then the old stream. group_tokens tracks brace depth, returning at depth-zero Close; End is restored and yields failure. opt_tokens requires initial [, ignores ] within braces, restores depth-zero Close/End on failure.

This is an exact source-derived partial model, not an executable approximate replacement.

## invariant

### Scoped invariants

Failed collectors do not restore already consumed body tokens. End cannot be consumed permanently by group_tokens or opt_tokens; arg_tokens similarly restores Close/End and returns no argument. truncate never grows the stream.

End and an unmatched outer Close bound optional scans; group scan preserves only End, while exhausting the stack returns None. These differing failure states matter to parser fallback.

For [{]}], the bracket inside braces is data. Only a depth-zero ] closes the option; leading spaces are consumed even when no option follows.

## composition

### Scoped interaction

Parser markers and macro argument readers call these mutators. Stack residual length is used by unknown-argument fallback, so collector failure is a state transition rather than a pure predicate.

The manifest binds this stage to the unchanged parent syntax projection and exact source byte range. Parent cost remains an upper accounting context; no independent whole-file invocation or execution count is assigned to this fragment.

## models

## behavior_complement

Failed collectors do not restore already consumed body tokens. End cannot be consumed permanently by group_tokens or opt_tokens; arg_tokens similarly restores Close/End and returns no argument. truncate never grows the stream.

## reverse_reconstruction

After successful group_tokens, output tokens plus remaining stream reconstruct the consumed interior and a closing brace, but not the caller-consumed opening token. After failure, lost consumed tokens are not recoverable from residual stack. push_front has unique undo only if the pushed prefix and its exact length are known.

## boundary_preservation

End and an unmatched outer Close bound optional scans; group scan preserves only End, while exhausting the stack returns None. These differing failure states matter to parser fallback.

## optional_scope

For [{]}], the bracket inside braces is data. Only a depth-zero ] closes the option; leading spaces are consumed even when no option follows.

## tests

### Evidence

lex::tests::lexes_commands_and_spaces; lex::tests::positions_are_bytes; lex::tests::render_is_stable

Selected library filters ast:: lex:: macros:: parse:: print:: table:: and integration gate excluding document_extraction_with_preamble_macros and random_token_soup_never_panics_and_roundtrips. Direct rustc --test latex/build.rs exercises private helpers. All selected tests twice debug/release with identical sorted names. Four compiled mutants at build/AST/parser/printer boundaries fail assertions then exact restoration passes. These do not individually cover every source. Build-main failure effects, allocation failures and universal roundtrip are unproved. Strict Clippy reports existing lints. No corpus/live VMM/API/GPU/full-school/held-out tests.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: replaced stage topic lists with concrete transitions, exclusions and compatible antecedents. Four meaningful typed views replace the previous six-view bookkeeping catalog. Shared syntax and source byte bounds are unchanged.

### 2026-10-31 · tests

Heading `### Evidence — 2026-10-31` stays in tests as `### Evidence`.
