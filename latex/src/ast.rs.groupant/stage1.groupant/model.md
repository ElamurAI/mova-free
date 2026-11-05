# latex-ast-stage1

## descriptant

### latex-ast-stage1

items(Row(v)) borrows v; items(other) borrows a one-element slice. walk visits self then children: binary a,b; optional index then body; Scripts base,sub,sup; Env row-major cells; Unknown args in vector order.

## approximant

### Scoped forward relation

items(Row(v)) borrows v; items(other) borrows a one-element slice. walk visits self then children: binary a,b; optional index then body; Scripts base,sub,sup; Env row-major cells; Unknown args in vector order.

This is an exact source-derived partial model, not an executable approximate replacement.

## invariant

### Scoped invariants

walk cannot omit the root or reorder script sub before base. Num/Char/Sym/Middle/Text are traversal leaves even if their strings contain syntax-like characters. items does not recursively flatten rows.

Visit(n)= [n] concatenated with Visit(c1)..Visit(ck) for the explicit child order; Env name/column spec and Scripts primes are metadata, not child nodes.

items and walk expose references into the original tree and cannot mutate it through their signatures; callbacks may mutate their own state, so traversal alone does not promise pure observer effects.

## composition

### Scoped interaction

unknowns/has_unknown/size depend on this complete traversal. Root-before-children gives unknown commands before their argument unknowns.

The manifest binds this stage to the unchanged parent syntax projection and exact source byte range. Parent cost remains an upper accounting context; no independent whole-file invocation or execution count is assigned to this fragment.

## models

## behavior_complement

walk cannot omit the root or reorder script sub before base. Num/Char/Sym/Middle/Text are traversal leaves even if their strings contain syntax-like characters. items does not recursively flatten rows.

## reverse_reconstruction

An items observation [a] can arise from Row([a]) or node a when a is not a Row. A callback retaining only Char values loses wrappers and numeric/symbol leaves; e.g Group(Char(x)) and Char(x) both yield the Char sequence [x], although full node visits differ.

## preorder_recurrence

Visit(n)= [n] concatenated with Visit(c1)..Visit(ck) for the explicit child order; Env name/column spec and Scripts primes are metadata, not child nodes.

## borrowed_projection

items and walk expose references into the original tree and cannot mutate it through their signatures; callbacks may mutate their own state, so traversal alone does not promise pure observer effects.

## tests

### Evidence

ast::wave28_tests::normalization_and_span_quotient; ast::wave28_tests::traversal_preserves_script_and_unknown_order

Selected library filters ast:: lex:: macros:: parse:: print:: table:: and integration gate excluding document_extraction_with_preamble_macros and random_token_soup_never_panics_and_roundtrips. Direct rustc --test latex/build.rs exercises private helpers. All selected tests twice debug/release with identical sorted names. Four compiled mutants at build/AST/parser/printer boundaries fail assertions then exact restoration passes. These do not individually cover every source. Build-main failure effects, allocation failures and universal roundtrip are unproved. Strict Clippy reports existing lints. No corpus/live VMM/API/GPU/full-school/held-out tests.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: replaced stage topic lists with concrete transitions, exclusions and compatible antecedents. Four meaningful typed views replace the previous six-view bookkeeping catalog. Shared syntax and source byte bounds are unchanged.

### 2026-10-31 · tests

Heading `### Evidence — 2026-10-31` stays in tests as `### Evidence`.
