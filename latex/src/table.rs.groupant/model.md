# latex-table

## descriptant

### latex-table

Class::latex merges Ord/Fence and operator-family classes; include consumes build output. negated matches a fixed partial symbol map; Le and Leqslant both become Nleq. font_letter recognizes only seven double-struck capitals and returns Latin base.

## approximant

### Forward relation

Class::latex merges Ord/Fence and operator-family classes; include consumes build output. negated matches a fixed partial symbol map; Le and Leqslant both become Nleq. font_letter recognizes only seven double-struck capitals and returns Latin base.

No useful approximate executable metric for exact categorical syntax; future caps1/2/5/10% only.

## invariant

### Domain, invariant and cost

Class::latex merges Ord/Fence and operator-family classes; include consumes build output. negated matches a fixed partial symbol map; Le and Leqslant both become Nleq. font_letter recognizes only seven double-struck capitals and returns Latin base.

Enum validity comes from compiler; unknown command/env/character returns None. No numerical approximate metric appropriate.

Fixed dispatch costs per lookup; generated arrays constant index; code size proportional to compiled command table. No runtime table file reads.

## composition

### Composition

Class::latex merges Ord/Fence and operator-family classes; include consumes build output. negated matches a fixed partial symbol map; Le and Leqslant both become Nleq. font_letter recognizes only seven double-struck capitals and returns Latin base.

Build generates table enums consumed by parser dispatch and printers. Lexer tokens feed macros and parser; parser returns AST whose equality ignores spans. Printers and gate compare that quotient, not original bytes. Shared parent syntax backs scoped stages; stages are byte-bound projections, not independent Rust modules.

## models

## behavior_complement

Unsupported negations and nonlisted font letters return None rather than invented symbol mappings; negation is not involutive.

## reverse_reconstruction

Preimage of Nleq contains Le and Leqslant; preimage of mathop contains Op, OpNoLim, Fn, FnLim. Generated alias lookups admit multiple spellings for one enum.

## class_projection

Class::latex deliberately merges operator families and Fence with Ord.

## negation_preimage

Distinct source symbols can map to same negation; absent mapping is None.

## font_subset

Only C,H,N,P,Q,R,Z double-struck capitals map to Font::Bb.

## generated_binding

include OUT_DIR/table.rs binds variants, canonical arrays and lookup matches from one build.

## tests

### Evidence

table::tests::table_lookups; table::tests::every_negation_is_a_relation_or_ord

Selected library filters ast:: lex:: macros:: parse:: print:: table:: and integration gate excluding document_extraction_with_preamble_macros and random_token_soup_never_panics_and_roundtrips. Direct rustc --test latex/build.rs exercises private helpers. All selected tests twice debug/release with identical sorted names. Four compiled mutants at build/AST/parser/printer boundaries fail assertions then exact restoration passes. These do not individually cover every source. Build-main failure effects, allocation failures and universal roundtrip are unproved. Strict Clippy reports existing lints. No corpus/live VMM/API/GPU/full-school/held-out tests.

## History

### 2026-10-31 · descriptant

Epoch2026-10-31: full source reviewed before edits; prospective models retained externally. No optimization.

### 2026-10-31 · tests

Heading `### Evidence — 2026-10-31` stays in tests as `### Evidence`.
