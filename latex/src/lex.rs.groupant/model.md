# latex-lex

## descriptant

### latex-lex

lex appends tokens, borrows command substrings and reports base+byte offset. ASCII command words eat following ASCII whitespace; comments consume until LF; spaces and tilde collapse. Stream reverses vectors, pushes front in reverse, and preserves End on failed substream scans. render serializes tokens with control-word separators.

## approximant

### Forward relation

lex appends tokens, borrows command substrings and reports base+byte offset. ASCII command words eat following ASCII whitespace; comments consume until LF; spaces and tilde collapse. Stream reverses vectors, pushes front in reverse, and preserves End on failed substream scans. render serializes tokens with control-word separators.

No useful approximate executable metric for exact categorical syntax; future caps1/2/5/10% only.

## invariant

### Domain, invariant and cost

lex appends tokens, borrows command substrings and reports base+byte offset. ASCII command words eat following ASCII whitespace; comments consume until LF; spaces and tilde collapse. Stream reverses vectors, pushes front in reverse, and preserves End on failed substream scans. render serializes tokens with control-word separators.

Empty appends nothing; existing trailing Space affects append coalescing. UTF-8 input valid; base+byte offset must fit u32 for nonwrapping semantics.

O(B+T) lex/storage, push O(k), group scanning O(k), render O(output); u32 addition overflow and oversized input cast excluded by representable-offset domain.

## composition

### Composition

lex appends tokens, borrows command substrings and reports base+byte offset. ASCII command words eat following ASCII whitespace; comments consume until LF; spaces and tilde collapse. Stream reverses vectors, pushes front in reverse, and preserves End on failed substream scans. render serializes tokens with control-word separators.

Build generates table enums consumed by parser dispatch and printers. Lexer tokens feed macros and parser; parser returns AST whose equality ignores spans. Printers and gate compare that quotient, not original bytes. Shared parent syntax backs scoped stages; stages are byte-bound projections, not independent Rust modules.

## models

## behavior_complement

No parser errors are returned by lex; trailing backslash and malformed parameter syntax remain characters. End may not disappear through group_tokens failure.

## reverse_reconstruction

Sources producing a token stream differ by comments and whitespace and command-word spacing. render removes End and maps CmdRaw to Cmd; exact reverse is a compatible-source set with those distinctions forgotten.

## byte_coordinates

Greek alpha occupies2 bytes so subsequent token offsets differ from scalar columns.

## substream_boundary

Group and optional collectors restore End, consume preceding tokens, and return failure.

## stack_order

next(push_front([a,b],s))=a then b then old s.

## render_loss

End absent and raw flag lost; separator required only control word before ASCII letter in render.

## tests

### Evidence

lex::tests::lexes_commands_and_spaces; lex::tests::positions_are_bytes; lex::tests::render_is_stable

Selected library filters ast:: lex:: macros:: parse:: print:: table:: and integration gate excluding document_extraction_with_preamble_macros and random_token_soup_never_panics_and_roundtrips. Direct rustc --test latex/build.rs exercises private helpers. All selected tests twice debug/release with identical sorted names. Four compiled mutants at build/AST/parser/printer boundaries fail assertions then exact restoration passes. These do not individually cover every source. Build-main failure effects, allocation failures and universal roundtrip are unproved. Strict Clippy reports existing lints. No corpus/live VMM/API/GPU/full-school/held-out tests.

## History

### 2026-10-31 · descriptant

Epoch2026-10-31: full source reviewed before edits; prospective models retained externally. No optimization.

### 2026-10-31 · tests

Heading `### Evidence — 2026-10-31` stays in tests as `### Evidence`.
