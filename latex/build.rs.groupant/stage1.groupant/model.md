# latex-build-stage1

## descriptant

### latex-build-stage1

main reads commands.txt, skips trimmed empty/comment rows, and processes remaining rows in order. sym requires>=5 columns, valid unique ID and known class before registering each form. Other enum rows require>=3 and a valid ID; font switches defer primary-variant validation. Explicit character registrations are completed before unclaimed Unicode text fallbacks are inserted.

## approximant

### Scoped forward relation

main reads commands.txt, skips trimmed empty/comment rows, and processes remaining rows in order. sym requires>=5 columns, valid unique ID and known class before registering each form. Other enum rows require>=3 and a valid ID; font switches defer primary-variant validation. Explicit character registrations are completed before unclaimed Unicode text fallbacks are inserted.

This is an exact source-derived partial model, not an executable approximate replacement.

## invariant

### Scoped invariants

Duplicate explicit commands, symbols, characters or environment spellings cannot silently succeed. Auto fallback cannot replace an explicit character. Invalid input before emission cannot reach the final table.rs write; cargo rerun directives have already been printed.

En canon/alt use or_insert; variant list appends only absent IDs. Command/environment aliases all remain lookup entries, even though only the first canonical name prints.

sym checks arity then identifier then duplicate ID then class then forms. Deferred font references fail after the line pass. Input read failure precedes all row checks; OUT_DIR lookup is deferred to the emission phase.

## composition

### Scoped interaction

This stage passes ordered syms/enums/cmds/chars/envs to emission. Hash maps provide membership and canonical lookup; vectors, not map iteration, determine generated order.

The manifest binds this stage to the unchanged parent syntax projection and exact source byte range. Parent cost remains an upper accounting context; no independent whole-file invocation or execution count is assigned to this fragment.

## models

## behavior_complement

Duplicate explicit commands, symbols, characters or environment spellings cannot silently succeed. Auto fallback cannot replace an explicit character. Invalid input before emission cannot reach the final table.rs write; cargo rerun directives have already been printed.

## reverse_reconstruction

Compatible validated states arise from tables differing in comments/blank lines or duplicate canonical declarations with new aliases. The first canonical form and first variant occurrence constrain row order; their stored maps do not recover ignored comments. Same fallback character is claimed by the earliest auto candidate only when no explicit claim exists.

## first_claim_policy

En canon/alt use or_insert; variant list appends only absent IDs. Command/environment aliases all remain lookup entries, even though only the first canonical name prints.

## failure_precedence

sym checks arity then identifier then duplicate ID then class then forms. Deferred font references fail after the line pass. Input read failure precedes all row checks; OUT_DIR lookup is deferred to the emission phase.

## tests

### Evidence

wave28_tests::canonical_first_form_and_variant_order; wave28_tests::variant_identifier_boundary

Selected library filters ast:: lex:: macros:: parse:: print:: table:: and integration gate excluding document_extraction_with_preamble_macros and random_token_soup_never_panics_and_roundtrips. Direct rustc --test latex/build.rs exercises private helpers. All selected tests twice debug/release with identical sorted names. Four compiled mutants at build/AST/parser/printer boundaries fail assertions then exact restoration passes. These do not individually cover every source. Build-main failure effects, allocation failures and universal roundtrip are unproved. Strict Clippy reports existing lints. No corpus/live VMM/API/GPU/full-school/held-out tests.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: replaced stage topic lists with concrete transitions, exclusions and compatible antecedents. Four meaningful typed views replace the previous six-view bookkeeping catalog. Shared syntax and source byte bounds are unchanged.

### 2026-10-31 · tests

Heading `### Evidence — 2026-10-31` stays in tests as `### Evidence`.
