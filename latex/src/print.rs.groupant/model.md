# latex-print

## descriptant

### latex-print

to_latex walks AST using command-word and digit separators; optional right brackets escape, unknown followed by brace gets relax, aligned leading bracket gets relax. Scripts print primes/sub/sup, trailing empty table row compensated. to_text projects style/group distinctions to Unicode with all-or-none sub/sup conversion; to_sexpr exposes selected structure and unknown start positions.

## approximant

### Forward relation

to_latex walks AST using command-word and digit separators; optional right brackets escape, unknown followed by brace gets relax, aligned leading bracket gets relax. Scripts print primes/sub/sup, trailing empty table row compensated. to_text projects style/group distinctions to Unicode with all-or-none sub/sup conversion; to_sexpr exposes selected structure and unknown start positions.

No useful approximate executable metric for exact categorical syntax; future caps1/2/5/10% only.

## invariant

### Domain, invariant and cost

to_latex walks AST using command-word and digit separators; optional right brackets escape, unknown followed by brace gets relax, aligned leading bracket gets relax. Scripts print primes/sub/sup, trailing empty table row compensated. to_text projects style/group distinctions to Unicode with all-or-none sub/sup conversion; to_sexpr exposes selected structure and unknown start positions.

Arbitrary publicly constructed ASTs need not round trip; empty string prints empty Row, numeric boundary spaces preserve separate Num nodes, prime count bounded by u8 representation.

LaTeX O(N+output) with recursive stack O(D); text creates subtree strings and can repeat copying across nested wrappers, conservative O(D*output+N). Allocation failure unmodeled.

## composition

### Composition

to_latex walks AST using command-word and digit separators; optional right brackets escape, unknown followed by brace gets relax, aligned leading bracket gets relax. Scripts print primes/sub/sup, trailing empty table row compensated. to_text projects style/group distinctions to Unicode with all-or-none sub/sup conversion; to_sexpr exposes selected structure and unknown start positions.

Build generates table enums consumed by parser dispatch and printers. Lexer tokens feed macros and parser; parser returns AST whose equality ignores spans. Printers and gate compare that quotient, not original bytes. Shared parent syntax backs scoped stages; stages are byte-bound projections, not independent Rust modules.

## models

## behavior_complement

Normal printer cannot use fault substitutions; intentional SwapFrac/DropSup/DropLast change only applicable structures. Plain text is not an injective encoding of the AST.

## reverse_reconstruction

Canonical string preimage includes ASTs differing only spans and potentially nonnormalized structures; normalized parser trees are the intended round-trip domain. Text preimage additionally varies styles, groups, presentation variants and column metadata.

## token_separation

Control word before alphanumeric and adjacent numeric fragments get separating space.

## optional_scope

Right bracket prints rbrack in optional context; explicit Group resets optional flag inside.

## fault_support

SwapFrac affects fractions, DropSup scripts, DropLast rows len>=3; unaffected trees can remain green.

## text_loss

Text trims ends, drops dollar markers/styles, font families may collapse; no inverse guarantee.

## tests

### Evidence

print::tests::canonical_latex; print::tests::plain_text; integration golden outputs and corrupted_printer_turns_gate_red

Selected library filters ast:: lex:: macros:: parse:: print:: table:: and integration gate excluding document_extraction_with_preamble_macros and random_token_soup_never_panics_and_roundtrips. Direct rustc --test latex/build.rs exercises private helpers. All selected tests twice debug/release with identical sorted names. Four compiled mutants at build/AST/parser/printer boundaries fail assertions then exact restoration passes. These do not individually cover every source. Build-main failure effects, allocation failures and universal roundtrip are unproved. Strict Clippy reports existing lints. No corpus/live VMM/API/GPU/full-school/held-out tests.

## History

### 2026-10-31 · descriptant

Epoch2026-10-31: full source reviewed before edits; prospective models retained externally. No optimization.

### 2026-10-31 · tests

Heading `### Evidence — 2026-10-31` stays in tests as `### Evidence`.
