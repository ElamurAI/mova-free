# latex-parse-stage0

## descriptant

### latex-parse-stage0

top parses a table; one row/one cell unwraps, otherwise any row with>1 cells yields Aligned, else Gathered. table removes exactly one trailing row consisting of one empty cell. list increments depth and rejects>400 before processing; Close succeeds only for Brace/TableBrace, & only for table-enabled stop. script parses its argument before checking existing sub/sup slot.

## approximant

### Scoped forward relation

top parses a table; one row/one cell unwraps, otherwise any row with>1 cells yields Aligned, else Gathered. table removes exactly one trailing row consisting of one empty cell. list increments depth and rejects>400 before processing; Close succeeds only for Brace/TableBrace, & only for table-enabled stop. script parses its argument before checking existing sub/sup slot.

This is an exact source-derived partial model, not an executable approximate replacement.

## invariant

### Scoped invariants

Unmatched Close cannot be silently ignored. A duplicate superscript with missing argument reports MissingArg before DoubleSup. Rows with multiple empty cells are not removed by the trailing-empty rule. Decimal dot merges only once and only when followed by a digit.

End under Marker terminates; otherwise it is pushed back and mapped through eof(stop), e.g Brace->UnclosedGroup, Right->MissingRight. Explicit Close under Right yields ExtraClose rather than MissingRight.

Without a prior atom, script base is Group(empty()). Existing Scripts receives the missing slot; duplicate slot errors after argument parsing. Prime runs saturate u8 at255, so counts>=255 lose multiplicity.

## composition

### Scoped interaction

Table/argument routines feed stage1 macro checks and stage2 command dispatch, then normalize atom vectors using ast::norm. This scope also converts Unicode font letters and handles environment arguments before body parsing.

The manifest binds this stage to the unchanged parent syntax projection and exact source byte range. Parent cost remains an upper accounting context; no independent whole-file invocation or execution count is assigned to this fragment.

## models

## behavior_complement

Unmatched Close cannot be silently ignored. A duplicate superscript with missing argument reports MissingArg before DoubleSup. Rows with multiple empty cells are not removed by the trailing-empty rule. Decimal dot merges only once and only when followed by a digit.

## reverse_reconstruction

Top output Char(x) can come from one-cell token sequences with discarded whitespace/presentation commands. Env rows lose at most one trailing empty row separator; extra separators can encode retained empty rows. Num('12') preimages include macro substitution digits as well as literal12, but a Space between digits prevents merging.

## stop_failure_order

End under Marker terminates; otherwise it is pushed back and mapped through eof(stop), e.g Brace->UnclosedGroup, Right->MissingRight. Explicit Close under Right yields ExtraClose rather than MissingRight.

## script_attachment

Without a prior atom, script base is Group(empty()). Existing Scripts receives the missing slot; duplicate slot errors after argument parsing. Prime runs saturate u8 at255, so counts>=255 lose multiplicity.

## tests

### Evidence

parse::tests (7 named tests); integration unbalanced_is_an_error_with_position and macros_newcommand_def_let_declareop

Selected library filters ast:: lex:: macros:: parse:: print:: table:: and integration gate excluding document_extraction_with_preamble_macros and random_token_soup_never_panics_and_roundtrips. Direct rustc --test latex/build.rs exercises private helpers. All selected tests twice debug/release with identical sorted names. Four compiled mutants at build/AST/parser/printer boundaries fail assertions then exact restoration passes. These do not individually cover every source. Build-main failure effects, allocation failures and universal roundtrip are unproved. Strict Clippy reports existing lints. No corpus/live VMM/API/GPU/full-school/held-out tests.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: replaced stage topic lists with concrete transitions, exclusions and compatible antecedents. Four meaningful typed views replace the previous six-view bookkeeping catalog. Shared syntax and source byte bounds are unchanged.

### 2026-10-31 · tests

Heading `### Evidence — 2026-10-31` stays in tests as `### Evidence`.
