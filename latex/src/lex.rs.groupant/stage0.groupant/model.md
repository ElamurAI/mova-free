# latex-lex-stage0

## descriptant

### latex-lex-stage0

lex scans UTF-8 bytes and appends tokens. Backslash+ASCII letters forms one borrowed Cmd and consumes following ASCII whitespace. Backslash+nonletter consumes one scalar as command; final backslash becomes Char. Percent skips through bytes before LF. Whitespace/tilde emits Space only if output's last token is not Space. Stream::new reverses the resulting vector.

## approximant

### Scoped forward relation

lex scans UTF-8 bytes and appends tokens. Backslash+ASCII letters forms one borrowed Cmd and consumes following ASCII whitespace. Backslash+nonletter consumes one scalar as command; final backslash becomes Char. Percent skips through bytes before LF. Whitespace/tilde emits Space only if output's last token is not Space. Stream::new reverses the resulting vector.

This is an exact source-derived partial model, not an executable approximate replacement.

## invariant

### Scoped invariants

Unicode command letters cannot extend an ASCII control word. No error Result or file effect is produced. A comment's terminating LF is not consumed by the comment branch and may yield Space; appending to an existing Space cannot emit a duplicate adjacent Space.

pos=base+i as u32; valid nonoverflowing coordinate domain requires offsets and additions to fit. α occupies2 bytes, so scalar index cannot replace i.

Control words consume only ASCII letters; control symbols consume one UTF-8 scalar. #1..#9 map to Param, while #0 and #x retain Char('#') plus following character.

## composition

### Scoped interaction

Stream reverse storage makes the last vector element the next token; later push-front operations rely on the same reversal convention.

The manifest binds this stage to the unchanged parent syntax projection and exact source byte range. Parent cost remains an upper accounting context; no independent whole-file invocation or execution count is assigned to this fragment.

## models

## behavior_complement

Unicode command letters cannot extend an ASCII control word. No error Result or file effect is produced. A comment's terminating LF is not consumed by the comment branch and may yield Space; appending to an existing Space cannot emit a duplicate adjacent Space.

## reverse_reconstruction

Token preimages lose comments and eaten control-word whitespace: backslash-alpha followed by one or several ASCII spaces before x yields the same kinds. Position observations additionally constrain byte lengths before each token but do not recover comment text. Initial output state changes whitespace coalescing.

## byte_offset_transport

pos=base+i as u32; valid nonoverflowing coordinate domain requires offsets and additions to fit. α occupies2 bytes, so scalar index cannot replace i.

## word_symbol_boundary

Control words consume only ASCII letters; control symbols consume one UTF-8 scalar. #1..#9 map to Param, while #0 and #x retain Char('#') plus following character.

## tests

### Evidence

lex::tests::lexes_commands_and_spaces; lex::tests::positions_are_bytes; lex::tests::render_is_stable

Selected library filters ast:: lex:: macros:: parse:: print:: table:: and integration gate excluding document_extraction_with_preamble_macros and random_token_soup_never_panics_and_roundtrips. Direct rustc --test latex/build.rs exercises private helpers. All selected tests twice debug/release with identical sorted names. Four compiled mutants at build/AST/parser/printer boundaries fail assertions then exact restoration passes. These do not individually cover every source. Build-main failure effects, allocation failures and universal roundtrip are unproved. Strict Clippy reports existing lints. No corpus/live VMM/API/GPU/full-school/held-out tests.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: replaced stage topic lists with concrete transitions, exclusions and compatible antecedents. Four meaningful typed views replace the previous six-view bookkeeping catalog. Shared syntax and source byte bounds are unchanged.

### 2026-10-31 · tests

Heading `### Evidence — 2026-10-31` stays in tests as `### Evidence`.
