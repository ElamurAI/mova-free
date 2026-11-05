# latex-table-stage1

## descriptant

### latex-table-stage1

negated(s) returns the explicitly listed negated counterpart, e.g Eq->Ne, In->Notin, Le/Leqslant->Nleq; all unmatched symbols return None.

## approximant

### Scoped forward relation

negated(s) returns the explicitly listed negated counterpart, e.g Eq->Ne, In->Notin, Le/Leqslant->Nleq; all unmatched symbols return None.

This is an exact source-derived partial model, not an executable approximate replacement.

## invariant

### Scoped invariants

negation cannot be assumed involutive: Ne has no reverse arm to Eq. Unsupported symbols cannot receive synthetic negations from this function.

Only Some outputs normalize parser Not(Sym) into one Sym; None tells parser to retain Node::Not around its operand.

Existing test checks every Some result's class is Rel or Ord; this finite table property does not assert logical complementation for every mathematical interpretation.

## composition

### Scoped interaction

Parser Entry::Not uses this mapping after parsing its argument; arbitrary nonsymbol arguments bypass it and keep Not structure.

The manifest binds this stage to the unchanged parent syntax projection and exact source byte range. Parent cost remains an upper accounting context; no independent whole-file invocation or execution count is assigned to this fragment.

## models

## behavior_complement

negation cannot be assumed involutive: Ne has no reverse arm to Eq. Unsupported symbols cannot receive synthetic negations from this function.

## reverse_reconstruction

Preimage(Nleq) includes Le and Leqslant; Preimage(Ngeq) includes Ge and Geqslant. An observed None admits every unlisted symbol, including already-negated variants. Thus a negated result does not generally recover the original spelling or symbol.

## partial_map

Only Some outputs normalize parser Not(Sym) into one Sym; None tells parser to retain Node::Not around its operand.

## relation_class

Existing test checks every Some result's class is Rel or Ord; this finite table property does not assert logical complementation for every mathematical interpretation.

## tests

### Evidence

table::tests::table_lookups; table::tests::every_negation_is_a_relation_or_ord

Selected library filters ast:: lex:: macros:: parse:: print:: table:: and integration gate excluding document_extraction_with_preamble_macros and random_token_soup_never_panics_and_roundtrips. Direct rustc --test latex/build.rs exercises private helpers. All selected tests twice debug/release with identical sorted names. Four compiled mutants at build/AST/parser/printer boundaries fail assertions then exact restoration passes. These do not individually cover every source. Build-main failure effects, allocation failures and universal roundtrip are unproved. Strict Clippy reports existing lints. No corpus/live VMM/API/GPU/full-school/held-out tests.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: replaced stage topic lists with concrete transitions, exclusions and compatible antecedents. Four meaningful typed views replace the previous six-view bookkeeping catalog. Shared syntax and source byte bounds are unchanged.

### 2026-10-31 · tests

Heading `### Evidence — 2026-10-31` stays in tests as `### Evidence`.
