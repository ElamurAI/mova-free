# latex-table-stage2

## descriptant

### latex-table-stage2

font_letter maps ℂ,ℍ,ℕ,ℙ,ℚ,ℝ,ℤ to (Bb,C/H/N/P/Q/R/Z), otherwise None. Included tests sample generated aliases, canonical forms and delimiter classes, and enumerate successful negation output classes.

## approximant

### Scoped forward relation

font_letter maps ℂ,ℍ,ℕ,ℙ,ℚ,ℝ,ℤ to (Bb,C/H/N/P/Q/R/Z), otherwise None. Included tests sample generated aliases, canonical forms and delimiter classes, and enumerate successful negation output classes.

This is an exact source-derived partial model, not an executable approximate replacement.

## invariant

### Scoped invariants

Unlisted mathematical Unicode letters cannot be normalized by font_letter, even if print::font_text can emit them. This is not a full inverse of Unicode mathematical-font rendering.

print::font_text(Bb,'A') emits mathematical double-struck A, but font_letter does not map that scalar back. The mapping's domain is seven compatibility capitals, not the entire font block.

table_lookups checks selected aliases and absent nosuchcommand; every_negation_is_a_relation_or_ord quantifies over generated SYM_ALL. Neither test exhausts every generated lookup alias.

## composition

### Scoped interaction

char_atom tries lookup_char before font_letter, so an explicit character table entry would take precedence over this fallback.

The manifest binds this stage to the unchanged parent syntax projection and exact source byte range. Parent cost remains an upper accounting context; no independent whole-file invocation or execution count is assigned to this fragment.

## models

## behavior_complement

Unlisted mathematical Unicode letters cannot be normalized by font_letter, even if print::font_text can emit them. This is not a full inverse of Unicode mathematical-font rendering.

## reverse_reconstruction

Each Some(Bb,letter) among the seven has exactly one scalar preimage here; None has all other Unicode scalars. A parser Font(Bb,Char(R)) can also originate from an explicit mathbb command, so parser-level recovery is broader.

## font_asymmetry

print::font_text(Bb,'A') emits mathematical double-struck A, but font_letter does not map that scalar back. The mapping's domain is seven compatibility capitals, not the entire font block.

## table_witness_scope

table_lookups checks selected aliases and absent nosuchcommand; every_negation_is_a_relation_or_ord quantifies over generated SYM_ALL. Neither test exhausts every generated lookup alias.

## tests

### Evidence

table::tests::table_lookups; table::tests::every_negation_is_a_relation_or_ord

Selected library filters ast:: lex:: macros:: parse:: print:: table:: and integration gate excluding document_extraction_with_preamble_macros and random_token_soup_never_panics_and_roundtrips. Direct rustc --test latex/build.rs exercises private helpers. All selected tests twice debug/release with identical sorted names. Four compiled mutants at build/AST/parser/printer boundaries fail assertions then exact restoration passes. These do not individually cover every source. Build-main failure effects, allocation failures and universal roundtrip are unproved. Strict Clippy reports existing lints. No corpus/live VMM/API/GPU/full-school/held-out tests.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: replaced stage topic lists with concrete transitions, exclusions and compatible antecedents. Four meaningful typed views replace the previous six-view bookkeeping catalog. Shared syntax and source byte bounds are unchanged.

### 2026-10-31 · tests

Heading `### Evidence — 2026-10-31` stays in tests as `### Evidence`.
