# latex-print-stage2

## descriptant

### latex-print-stage2

to_text recursively renders then trims. Groups/classes lose wrappers; Text drops dollar characters/style. Scripts remove spaces before all-or-none Unicode conversion, otherwise use _/^ and parentheses when length!=1. Frac wraps complex operands; font_text maps selected families and exceptions; env joins cells/rows by kind.

## approximant

### Scoped forward relation

to_text recursively renders then trims. Groups/classes lose wrappers; Text drops dollar characters/style. Scripts remove spaces before all-or-none Unicode conversion, otherwise use _/^ and parentheses when length!=1. Frac wraps complex operands; font_text maps selected families and exceptions; env joins cells/rows by kind.

This is an exact source-derived partial model, not an executable approximate replacement.

## invariant

### Scoped invariants

Plain text cannot preserve all font/text styles, Group boundaries or array column specifications. A partially mappable script cannot mix Unicode and fallback per character: mapping returns None for the whole string.

Sup 'n+1' maps fully to superscripts after spaces removed; an unsupported letter forces the entire script fallback. Empty script maps to empty string, losing its explicit presence.

Aligned/Gathered/Split collapse each row whitespace then join newline; cases join cells comma and rows semicolon; unknown env names disappear into the array-style join. Distinct env metadata can produce identical text.

## composition

### Scoped interaction

Plain text serves display/bridge use, not the structural roundtrip gate. Temporary plain subtree strings and nested wrappers can amplify copying with depth.

The manifest binds this stage to the unchanged parent syntax projection and exact source byte range. Parent cost remains an upper accounting context; no independent whole-file invocation or execution count is assigned to this fragment.

## models

## behavior_complement

Plain text cannot preserve all font/text styles, Group boundaries or array column specifications. A partially mappable script cannot mix Unicode and fallback per character: mapping returns None for the whole string.

## reverse_reconstruction

Group(Char(x)), Class(Ord,Char(x)) and Char(x) all render x. Font Cal and Scr share mappings; Text styles share output, and removed dollar markers add antecedents. Output x does not uniquely determine tree shape or type.

## script_projection

Sup 'n+1' maps fully to superscripts after spaces removed; an unsupported letter forces the entire script fallback. Empty script maps to empty string, losing its explicit presence.

## matrix_projection

Aligned/Gathered/Split collapse each row whitespace then join newline; cases join cells comma and rows semicolon; unknown env names disappear into the array-style join. Distinct env metadata can produce identical text.

## tests

### Evidence

print::tests::canonical_latex; print::tests::plain_text; integration golden outputs and corrupted_printer_turns_gate_red

Selected library filters ast:: lex:: macros:: parse:: print:: table:: and integration gate excluding document_extraction_with_preamble_macros and random_token_soup_never_panics_and_roundtrips. Direct rustc --test latex/build.rs exercises private helpers. All selected tests twice debug/release with identical sorted names. Four compiled mutants at build/AST/parser/printer boundaries fail assertions then exact restoration passes. These do not individually cover every source. Build-main failure effects, allocation failures and universal roundtrip are unproved. Strict Clippy reports existing lints. No corpus/live VMM/API/GPU/full-school/held-out tests.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: replaced stage topic lists with concrete transitions, exclusions and compatible antecedents. Four meaningful typed views replace the previous six-view bookkeeping catalog. Shared syntax and source byte bounds are unchanged.

### 2026-10-31 · tests

Heading `### Evidence — 2026-10-31` stays in tests as `### Evidence`.
