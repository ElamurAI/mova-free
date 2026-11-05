# latex-test-gate-stage1

## descriptant

### latex-test-gate-stage1

Golden cases compare exact LaTeX or text. Unknown test checks ordered names and source slices for two command spans, then roundtrip and canonical output. Malformed cases compare expected ErrorKind/position plus Unicode column/context. Fault test requires some of SwapFrac/DropSup/DropLast to turn each affected fixture red, then verifies an unaffected swap stays green.

## approximant

### Scoped forward relation

Golden cases compare exact LaTeX or text. Unknown test checks ordered names and source slices for two command spans, then roundtrip and canonical output. Malformed cases compare expected ErrorKind/position plus Unicode column/context. Fault test requires some of SwapFrac/DropSup/DropLast to turn each affected fixture red, then verifies an unaffected swap stays green.

This is an exact source-derived partial model, not an executable approximate replacement.

## invariant

### Scoped invariants

A corrupt printer that alters no listed affected observation may escape this test; red is not required for every fault/fixture pair. Unknown order or checked source coordinates cannot change while those assertions still pass.

For each affected fixture exists a fault producing non-Ok; this is not for every fixture every fault. Symmetric fraction arguments can also hide a swap in general.

Unlike structural gate, source slicing explicitly validates stored start/end for norm and R commands; Unicode column/context asserts conversion from byte positions for one malformed formula.

## composition

### Scoped interaction

These tests constrain three distinct observations: canonical bytes, lossy text and structural/coordinate behavior. Their results cannot be substituted for each other.

The manifest binds this stage to the unchanged parent syntax projection and exact source byte range. Parent cost remains an upper accounting context; no independent whole-file invocation or execution count is assigned to this fragment.

## models

## behavior_complement

A corrupt printer that alters no listed affected observation may escape this test; red is not required for every fault/fixture pair. Unknown order or checked source coordinates cannot change while those assertions still pass.

## reverse_reconstruction

A green faulty-gate result on x+y with SwapFrac is compatible with a no-op because no fraction exists; red on a/b does not identify whether numerator/denominator or another structural component changed. Expected byte errors constrain those literals but not all equal-kind inputs.

## fault_quantifier

For each affected fixture exists a fault producing non-Ok; this is not for every fixture every fault. Symmetric fraction arguments can also hide a swap in general.

## coordinate_oracle

Unlike structural gate, source slicing explicitly validates stored start/end for norm and R commands; Unicode column/context asserts conversion from byte positions for one malformed formula.

## tests

### Evidence

7 integration tests via --test gate with document and soup skipped

Selected library filters ast:: lex:: macros:: parse:: print:: table:: and integration gate excluding document_extraction_with_preamble_macros and random_token_soup_never_panics_and_roundtrips. Direct rustc --test latex/build.rs exercises private helpers. All selected tests twice debug/release with identical sorted names. Four compiled mutants at build/AST/parser/printer boundaries fail assertions then exact restoration passes. These do not individually cover every source. Build-main failure effects, allocation failures and universal roundtrip are unproved. Strict Clippy reports existing lints. No corpus/live VMM/API/GPU/full-school/held-out tests.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: replaced stage topic lists with concrete transitions, exclusions and compatible antecedents. Four meaningful typed views replace the previous six-view bookkeeping catalog. Shared syntax and source byte bounds are unchanged.

### 2026-10-31 · tests

Heading `### Evidence — 2026-10-31` stays in tests as `### Evidence`.
