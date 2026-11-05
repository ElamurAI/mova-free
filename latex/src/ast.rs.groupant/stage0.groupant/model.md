# latex-ast-stage0

## descriptant

### latex-ast-stage0

empty()=Row([]). norm(v)=v[0] when len(v)=1, otherwise Row(v); singleton extraction consumes the vector by pop.

## approximant

### Scoped forward relation

empty()=Row([]). norm(v)=v[0] when len(v)=1, otherwise Row(v); singleton extraction consumes the vector by pop.

This is an exact source-derived partial model, not an executable approximate replacement.

## invariant

### Scoped invariants

norm([Char(x)]) cannot yield Row([Char(x)]); norm([]) cannot invent an atom. Existing nested Row nodes are not recursively flattened.

norm([Row([a,b])]) and norm([a,b]) both produce Row([a,b]); this is an outer-list quotient, not recursive semantic simplification.

empty and norm([]) are structurally identical, but norm([Group(empty())]) preserves Group(empty()) rather than equating explicit braces to an empty row.

## composition

### Scoped interaction

Parser list/table routines use norm after collecting atoms. These constructors do not enforce invariants on manually constructed Node::Row values.

The manifest binds this stage to the unchanged parent syntax projection and exact source byte range. Parent cost remains an upper accounting context; no independent whole-file invocation or execution count is assigned to this fragment.

## models

## behavior_complement

norm([Char(x)]) cannot yield Row([Char(x)]); norm([]) cannot invent an atom. Existing nested Row nodes are not recursively flattened.

## reverse_reconstruction

For non-Row atom a, norm inverse is exactly {[a]}. For Row(v), compatible lists include v when len(v)!=1 and [Row(v)] always. Thus Row([]) can come from [] or [Row([])]; normalization loses that outer-list provenance.

## singleton_quotient

norm([Row([a,b])]) and norm([a,b]) both produce Row([a,b]); this is an outer-list quotient, not recursive semantic simplification.

## empty_identity

empty and norm([]) are structurally identical, but norm([Group(empty())]) preserves Group(empty()) rather than equating explicit braces to an empty row.

## tests

### Evidence

ast::wave28_tests::normalization_and_span_quotient; ast::wave28_tests::traversal_preserves_script_and_unknown_order

Selected library filters ast:: lex:: macros:: parse:: print:: table:: and integration gate excluding document_extraction_with_preamble_macros and random_token_soup_never_panics_and_roundtrips. Direct rustc --test latex/build.rs exercises private helpers. All selected tests twice debug/release with identical sorted names. Four compiled mutants at build/AST/parser/printer boundaries fail assertions then exact restoration passes. These do not individually cover every source. Build-main failure effects, allocation failures and universal roundtrip are unproved. Strict Clippy reports existing lints. No corpus/live VMM/API/GPU/full-school/held-out tests.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: replaced stage topic lists with concrete transitions, exclusions and compatible antecedents. Four meaningful typed views replace the previous six-view bookkeeping catalog. Shared syntax and source byte bounds are unchanged.

### 2026-10-31 · tests

Heading `### Evidence — 2026-10-31` stays in tests as `### Evidence`.
