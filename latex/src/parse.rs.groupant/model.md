# latex-parse

## descriptant

### latex-parse

parse_with initializes local macro state and token stack. list consumes until stop kind, normalizes rows; top tables choose Aligned if any row has multiple cells else Gathered. Local macros shadow external before built-in dispatch, CmdRaw bypasses expansion. Unknown grouped arguments parse or become canonical Text on recoverable failure.

## approximant

### Forward relation

parse_with initializes local macro state and token stack. list consumes until stop kind, normalizes rows; top tables choose Aligned if any row has multiple cells else Gathered. Local macros shadow external before built-in dispatch, CmdRaw bypasses expansion. Unknown grouped arguments parse or become canonical Text on recoverable failure.

No useful approximate executable metric for exact categorical syntax; future caps1/2/5/10% only.

## invariant

### Domain, invariant and cost

parse_with initializes local macro state and token stack. list consumes until stop kind, normalizes rows; top tables choose Aligned if any row has multiple cells else Gathered. Local macros shadow external before built-in dispatch, CmdRaw bypasses expansion. Unknown grouped arguments parse or become canonical Text on recoverable failure.

Depth threshold400; expansions10000; token threshold1000000 checked pre-expansion. primes saturate255; offsets u32 require representable lengths. Argument errors occur before duplicate sup/sub check.

Cost in source bytes B, expanded tokens T, body/argument copies C and tree N: O(B+T+C) plus delimiter matching and repeated nested operations; storage includes tokens, macro clones and tree; recursion bounded only along list depth.

## composition

### Composition

parse_with initializes local macro state and token stack. list consumes until stop kind, normalizes rows; top tables choose Aligned if any row has multiple cells else Gathered. Local macros shadow external before built-in dispatch, CmdRaw bypasses expansion. Unknown grouped arguments parse or become canonical Text on recoverable failure.

Build generates table enums consumed by parser dispatch and printers. Lexer tokens feed macros and parser; parser returns AST whose equality ignores spans. Printers and gate compare that quotient, not original bytes. Shared parent syntax backs scoped stages; stages are byte-bound projections, not independent Rust modules.

## models

## behavior_complement

Accepted unmatched closers and duplicate scripts are excluded, but arbitrary TeX semantic validity is not established. MAX_TOKENS is a check before substitution, not a global peak allocation ceiling.

## reverse_reconstruction

parse preimage includes comments, spacing, aliases, dropped presentation commands and macro programs with the same expanded normalized structure. Error preimage includes sources sharing first reached failure and byte offset; no unique original recoverability.

## failure_precedence

script obtains argument before checking occupied slot; genfrac parses six args before delimiter validation.

## macro_resources

Local definitions persist for formula; expansion increments before threshold check and substitution may exceed token threshold.

## row_normalization

Only one trailing empty row removed; single top cell unwrapped; other tables explicit Env.

## unknown_fallback

Failed unknown argument can become Text after stack truncation; local macro changes are not rolled back.

## tests

### Evidence

parse::tests (7 named tests); integration unbalanced_is_an_error_with_position and macros_newcommand_def_let_declareop

Selected library filters ast:: lex:: macros:: parse:: print:: table:: and integration gate excluding document_extraction_with_preamble_macros and random_token_soup_never_panics_and_roundtrips. Direct rustc --test latex/build.rs exercises private helpers. All selected tests twice debug/release with identical sorted names. Four compiled mutants at build/AST/parser/printer boundaries fail assertions then exact restoration passes. These do not individually cover every source. Build-main failure effects, allocation failures and universal roundtrip are unproved. Strict Clippy reports existing lints. No corpus/live VMM/API/GPU/full-school/held-out tests.

## History

### 2026-10-31 · descriptant

Epoch2026-10-31: full source reviewed before edits; prospective models retained externally. No optimization.

### 2026-10-31 · tests

Heading `### Evidence — 2026-10-31` stays in tests as `### Evidence`.
