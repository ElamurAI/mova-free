# latex-ast

## descriptant

### latex-ast

norm collapses exactly one atom; empty is Row([]). walk is preorder and preserves row/cell order; Scripts visits base, sub, sup, optional roots visit index before body. unknowns collects names and actual stored spans; has_unknown traverses all nodes.

## approximant

### Forward relation

norm collapses exactly one atom; empty is Row([]). walk is preorder and preserves row/cell order; Scripts visits base, sub, sup, optional roots visit index before body. unknowns collects names and actual stored spans; has_unknown traverses all nodes.

No useful approximate executable metric for exact categorical syntax; future caps1/2/5/10% only.

## invariant

### Domain, invariant and cost

norm collapses exactly one atom; empty is Row([]). walk is preorder and preserves row/cell order; Scripts visits base, sub, sup, optional roots visit index before body. unknowns collects names and actual stored spans; has_unknown traverses all nodes.

Public Node permits non-normal forms; no validation enforces singleton Row absence. Span u32 represents byte coordinates but equality deliberately ignores them.

walk/size/has_unknown O(N), O(D) recursion; unknowns adds name bytes and K outputs; usize size overflow not separately guarded.

## composition

### Composition

norm collapses exactly one atom; empty is Row([]). walk is preorder and preserves row/cell order; Scripts visits base, sub, sup, optional roots visit index before body. unknowns collects names and actual stored spans; has_unknown traverses all nodes.

Build generates table enums consumed by parser dispatch and printers. Lexer tokens feed macros and parser; parser returns AST whose equality ignores spans. Printers and gate compare that quotient, not original bytes. Shared parent syntax backs scoped stages; stages are byte-bound projections, not independent Rust modules.

## models

## behavior_complement

Span equality cannot distinguish coordinates; traversal cannot silently omit Unknown arguments or script children.

## reverse_reconstruction

A structural equality class admits arbitrary span assignments. norm inverse of atom n includes [n], and when n is Row(v), v subject to normalization; original grouping/positions cannot be uniquely inferred.

## span_quotient

Eq(Span a,Span b)=true independently of start/end.

## normalization

norm([])=Row([]), norm([a])=a, norm(v)=Row(v) when len differs from1.

## traversal_order

Root precedes children; scripts base/sub/sup and env row-major.

## unknown_projection

unknowns preserves traversal order and stored offsets; has_unknown agrees with nonempty unknowns for finite valid trees.

## tests

### Evidence

ast::wave28_tests::normalization_and_span_quotient; ast::wave28_tests::traversal_preserves_script_and_unknown_order

Selected library filters ast:: lex:: macros:: parse:: print:: table:: and integration gate excluding document_extraction_with_preamble_macros and random_token_soup_never_panics_and_roundtrips. Direct rustc --test latex/build.rs exercises private helpers. All selected tests twice debug/release with identical sorted names. Four compiled mutants at build/AST/parser/printer boundaries fail assertions then exact restoration passes. These do not individually cover every source. Build-main failure effects, allocation failures and universal roundtrip are unproved. Strict Clippy reports existing lints. No corpus/live VMM/API/GPU/full-school/held-out tests.

## History

### 2026-10-31 · descriptant

Epoch2026-10-31: full source reviewed before edits; prospective models retained externally. No optimization.

### 2026-10-31 · tests

Heading `### Evidence — 2026-10-31` stays in tests as `### Evidence`.
