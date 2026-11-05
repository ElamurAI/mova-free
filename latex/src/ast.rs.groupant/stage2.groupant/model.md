# latex-ast-stage2

## descriptant

### latex-ast-stage2

unknowns runs walk and emits (backslash+command,stored span) or (begin{name},stored span) for unknown environments. has_unknown ORs the same node predicates without short-circuiting walk. size counts each node visit, including root and empty Row.

## approximant

### Scoped forward relation

unknowns runs walk and emits (backslash+command,stored span) or (begin{name},stored span) for unknown environments. has_unknown ORs the same node predicates without short-circuiting walk. size counts each node visit, including root and empty Row.

This is an exact source-derived partial model, not an executable approximate replacement.

## invariant

### Scoped invariants

An unknown-looking string inside Text cannot itself appear in unknowns. has_unknown cannot be false when unknowns is nonempty for a finite tree. Span-insensitive Node equality cannot establish coordinate equality.

has_unknown(n) iff !unknowns(n).is_empty(), under terminating traversal. Names preserve the spelling stored in Unknown/EnvName; spans are copied rather than recomputed.

size(n)=1+sum(size(child)); empty Row contributes1. Replacing a known leaf by another preserves size; adding a Group increases size by1.

## composition

### Scoped interaction

These observers use the previous traversal stage. Their output coordinates remain meaningful even though Span::eq returns true for arbitrary coordinates; the appended tests explicitly distinguish those observations.

The manifest binds this stage to the unchanged parent syntax projection and exact source byte range. Parent cost remains an upper accounting context; no independent whole-file invocation or execution count is assigned to this fragment.

## models

## behavior_complement

An unknown-looking string inside Text cannot itself appear in unknowns. has_unknown cannot be false when unknowns is nonempty for a finite tree. Span-insensitive Node equality cannot establish coordinate equality.

## reverse_reconstruction

unknowns=[] is compatible with every tree containing no Unknown node or unknown Env, including empty Row and large known trees. A listed unknown's name/span does not determine its children or surrounding known nodes. size=1 is compatible with any leaf and also empty Row.

## unknown_presence

has_unknown(n) iff !unknowns(n).is_empty(), under terminating traversal. Names preserve the spelling stored in Unknown/EnvName; spans are copied rather than recomputed.

## node_count

size(n)=1+sum(size(child)); empty Row contributes1. Replacing a known leaf by another preserves size; adding a Group increases size by1.

## tests

### Evidence

ast::wave28_tests::normalization_and_span_quotient; ast::wave28_tests::traversal_preserves_script_and_unknown_order

Selected library filters ast:: lex:: macros:: parse:: print:: table:: and integration gate excluding document_extraction_with_preamble_macros and random_token_soup_never_panics_and_roundtrips. Direct rustc --test latex/build.rs exercises private helpers. All selected tests twice debug/release with identical sorted names. Four compiled mutants at build/AST/parser/printer boundaries fail assertions then exact restoration passes. These do not individually cover every source. Build-main failure effects, allocation failures and universal roundtrip are unproved. Strict Clippy reports existing lints. No corpus/live VMM/API/GPU/full-school/held-out tests.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: replaced stage topic lists with concrete transitions, exclusions and compatible antecedents. Four meaningful typed views replace the previous six-view bookkeeping catalog. Shared syntax and source byte bounds are unchanged.

### 2026-10-31 · tests

Heading `### Evidence — 2026-10-31` stays in tests as `### Evidence`.
