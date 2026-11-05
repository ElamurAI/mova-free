# latex-table-stage0

## descriptant

### latex-table-stage0

Class::latex maps Ord/Fence->mathord; Op/OpNoLim/Fn/FnLim->mathop; other listed classes have their dedicated math commands. Entry carries typed parser actions. include inserts the generated enums, canonical arrays and lookup functions.

## approximant

### Scoped forward relation

Class::latex maps Ord/Fence->mathord; Op/OpNoLim/Fn/FnLim->mathop; other listed classes have their dedicated math commands. Entry carries typed parser actions. include inserts the generated enums, canonical arrays and lookup functions.

This is an exact source-derived partial model, not an executable approximate replacement.

## invariant

### Scoped invariants

Class projection cannot distinguish operator-family subclasses or Fence from Ord. This source performs no runtime command-table file read; include is compile-time.

Class print is many-to-one, so manually constructed Node::Class(Fence,...) need not survive parse(print(...)) with the same Class.

Sym metadata and Entry variant payloads must come from the same generated table version; successful compilation checks variant references but not a semantic proof of canonicalization.

## composition

### Scoped interaction

Parser dispatch matches Entry; printers use Class/Sym metadata. Array indexing semantics originate in build.rs, not from runtime reflection.

The manifest binds this stage to the unchanged parent syntax projection and exact source byte range. Parent cost remains an upper accounting context; no independent whole-file invocation or execution count is assigned to this fragment.

## models

## behavior_complement

Class projection cannot distinguish operator-family subclasses or Fence from Ord. This source performs no runtime command-table file read; include is compile-time.

## reverse_reconstruction

Preimage(mathop)={Op,OpNoLim,Fn,FnLim}; preimage(mathord)={Ord,Fence}. Each other emitted class command identifies one listed Class. Entry representation alone does not recover original command aliases.

## class_quotient

Class print is many-to-one, so manually constructed Node::Class(Fence,...) need not survive parse(print(...)) with the same Class.

## generated_join

Sym metadata and Entry variant payloads must come from the same generated table version; successful compilation checks variant references but not a semantic proof of canonicalization.

## tests

### Evidence

table::tests::table_lookups; table::tests::every_negation_is_a_relation_or_ord

Selected library filters ast:: lex:: macros:: parse:: print:: table:: and integration gate excluding document_extraction_with_preamble_macros and random_token_soup_never_panics_and_roundtrips. Direct rustc --test latex/build.rs exercises private helpers. All selected tests twice debug/release with identical sorted names. Four compiled mutants at build/AST/parser/printer boundaries fail assertions then exact restoration passes. These do not individually cover every source. Build-main failure effects, allocation failures and universal roundtrip are unproved. Strict Clippy reports existing lints. No corpus/live VMM/API/GPU/full-school/held-out tests.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: replaced stage topic lists with concrete transitions, exclusions and compatible antecedents. Four meaningful typed views replace the previous six-view bookkeeping catalog. Shared syntax and source byte bounds are unchanged.

### 2026-10-31 · tests

Heading `### Evidence — 2026-10-31` stays in tests as `### Evidence`.
