# latex-print-stage1

## descriptant

### latex-print-stage1

to_sexpr recursively writes tagged forms: Group, fraction kind, scripts with optional sub/sup/primes, environment row lists, unknown command/environment names with start offset. Text ignores TextStyle; Env serialization omits arg column metadata; XArrow prints above before optional below.

## approximant

### Scoped forward relation

to_sexpr recursively writes tagged forms: Group, fraction kind, scripts with optional sub/sup/primes, environment row lists, unknown command/environment names with start offset. Text ignores TextStyle; Env serialization omits arg column metadata; XArrow prints above before optional below.

This is an exact source-derived partial model, not an executable approximate replacement.

## invariant

### Scoped invariants

Debug output cannot recover full AST equality generally: different TextStyle variants and Env args can serialize identically. It is not parsed by the formula parser as a guaranteed inverse format.

Unknown start is printed with @; end is omitted. Therefore this view distinguishes some values that Node equality treats equal while collapsing other structural fields.

XArrow stored below/above prints above then below; Stack prints annotation then base through its tagged parts. Consumers must use these explicit orders rather than assume field declaration order.

## composition

### Scoped interaction

This debugging view is separate from canonical LaTeX and plain text. Neither gate equality nor canonical idempotence uses it as a serialization contract.

The manifest binds this stage to the unchanged parent syntax projection and exact source byte range. Parent cost remains an upper accounting context; no independent whole-file invocation or execution count is assigned to this fragment.

## models

## behavior_complement

Debug output cannot recover full AST equality generally: different TextStyle variants and Env args can serialize identically. It is not parsed by the formula parser as a guaranteed inverse format.

## reverse_reconstruction

Text(Plain,'a') and Text(Bf,'a') share the text debug form. Two Env nodes differing only array column arg share output. Unknown spans with equal start but different end also share output.

## debug_coordinate

Unknown start is printed with @; end is omitted. Therefore this view distinguishes some values that Node equality treats equal while collapsing other structural fields.

## label_order

XArrow stored below/above prints above then below; Stack prints annotation then base through its tagged parts. Consumers must use these explicit orders rather than assume field declaration order.

## tests

### Evidence

print::tests::canonical_latex; print::tests::plain_text; integration golden outputs and corrupted_printer_turns_gate_red

Selected library filters ast:: lex:: macros:: parse:: print:: table:: and integration gate excluding document_extraction_with_preamble_macros and random_token_soup_never_panics_and_roundtrips. Direct rustc --test latex/build.rs exercises private helpers. All selected tests twice debug/release with identical sorted names. Four compiled mutants at build/AST/parser/printer boundaries fail assertions then exact restoration passes. These do not individually cover every source. Build-main failure effects, allocation failures and universal roundtrip are unproved. Strict Clippy reports existing lints. No corpus/live VMM/API/GPU/full-school/held-out tests.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: replaced stage topic lists with concrete transitions, exclusions and compatible antecedents. Four meaningful typed views replace the previous six-view bookkeeping catalog. Shared syntax and source byte bounds are unchanged.

### 2026-10-31 · tests

Heading `### Evidence — 2026-10-31` stays in tests as `### Evidence`.
