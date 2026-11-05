# latex-parse-stage1

## descriptant

### latex-parse-stage1

Macro lookup checks local first then document map. expand increments expansions and rejects when>10000 or current stack length>1000000 before collecting args/substituting. Each body Param(k) inserts captured argument if present; other body tokens acquire call position. define commits local successful definitions. unknown parses following groups under End markers and on recoverable failure truncates added stream and stores rendered Text.

## approximant

### Scoped forward relation

Macro lookup checks local first then document map. expand increments expansions and rejects when>10000 or current stack length>1000000 before collecting args/substituting. Each body Param(k) inserts captured argument if present; other body tokens acquire call position. define commits local successful definitions. unknown parses following groups under End markers and on recoverable failure truncates added stream and stores rendered Text.

This is an exact source-derived partial model, not an executable approximate replacement.

## invariant

### Scoped invariants

A document macro cannot override an existing local macro. Threshold does not guarantee peak tokens<=1000000 because one substitution can exceed it. Unknown Text fallback does not roll back local definitions or expansion counter effects. A body parameter without captured argument is dropped, not invented.

The increment happens before rejection. At prior count10000 the next expansion fails; stack exactly1000000 passes this check. Large output may be pushed afterward and only a later macro expansion rechecks size.

unknown records len0 before temporary substream. If failure has consumed below len0 it propagates; otherwise truncate(len0) drops remaining temporary tokens and Text is retained. Parser-local state changes are not truncated with the token stack.

## composition

### Scoped interaction

stage2 command bypasses macro lookup for CmdRaw. The transformed token stream returns to stage0 list parsing; declaration and fallback effects therefore interact with later tokens in the same formula.

The manifest binds this stage to the unchanged parent syntax projection and exact source byte range. Parent cost remains an upper accounting context; no independent whole-file invocation or execution count is assigned to this fragment.

## models

## behavior_complement

A document macro cannot override an existing local macro. Threshold does not guarantee peak tokens<=1000000 because one substitution can exceed it. Unknown Text fallback does not roll back local definitions or expansion counter effects. A body parameter without captured argument is dropped, not invented.

## reverse_reconstruction

Expanded tokens may come from many macro bodies/argument partitions; nonparameter body positions all collapse to call position, while argument token positions survive. Unknown Text can result from different parser errors with the same rendered tokens. Final local map cannot reconstruct overridden definitions.

## pre_expansion_budget

The increment happens before rejection. At prior count10000 the next expansion fails; stack exactly1000000 passes this check. Large output may be pushed afterward and only a later macro expansion rechecks size.

## fallback_state

unknown records len0 before temporary substream. If failure has consumed below len0 it propagates; otherwise truncate(len0) drops remaining temporary tokens and Text is retained. Parser-local state changes are not truncated with the token stack.

## tests

### Evidence

parse::tests (7 named tests); integration unbalanced_is_an_error_with_position and macros_newcommand_def_let_declareop

Selected library filters ast:: lex:: macros:: parse:: print:: table:: and integration gate excluding document_extraction_with_preamble_macros and random_token_soup_never_panics_and_roundtrips. Direct rustc --test latex/build.rs exercises private helpers. All selected tests twice debug/release with identical sorted names. Four compiled mutants at build/AST/parser/printer boundaries fail assertions then exact restoration passes. These do not individually cover every source. Build-main failure effects, allocation failures and universal roundtrip are unproved. Strict Clippy reports existing lints. No corpus/live VMM/API/GPU/full-school/held-out tests.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: replaced stage topic lists with concrete transitions, exclusions and compatible antecedents. Four meaningful typed views replace the previous six-view bookkeeping catalog. Shared syntax and source byte bounds are unchanged.

### 2026-10-31 · tests

Heading `### Evidence — 2026-10-31` stays in tests as `### Evidence`.
