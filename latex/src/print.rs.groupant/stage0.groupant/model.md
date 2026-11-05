# latex-print-stage0

## descriptant

### latex-print-stage0

put inserts space when preceding output ends a control word and next starts alphanumeric, or when numeric/dot fragments would merge. Row spacing uses symbol class. Unknown followed by brace-start emits relax; aligned/gathered initial bracket emits relax. Optional RBrack prints rbrack. A retained trailing empty table row prints an extra separator.

## approximant

### Scoped forward relation

put inserts space when preceding output ends a control word and next starts alphanumeric, or when numeric/dot fragments would merge. Row spacing uses symbol class. Unknown followed by brace-start emits relax; aligned/gathered initial bracket emits relax. Optional RBrack prints rbrack. A retained trailing empty table row prints an extra separator.

This is an exact source-derived partial model, not an executable approximate replacement.

## invariant

### Scoped invariants

Fault=None cannot swap numerator/denominator or drop scripts. Unknown followed by an explicit Group cannot be serialized as an immediately swallowed argument in supported normalized trees. Normal writer does not preserve original whitespace or spans.

SwapFrac swaps every encountered fraction, including nested; DropSup removes sup only; DropLast removes last atom of every Row len>=3. Fraction a/a can be unchanged by swap, so fault activation is not always detectable.

opt_arg saves/restores flag; Group temporarily resets it. A right bracket inside explicit braces can print literally, whereas one exposed in optional content must use rbrack.

## composition

### Scoped interaction

Parser consumes separators/relax without extra AST nodes. This adapter supports parse-produced trees; arbitrary public Num strings, nested Row layouts or Class values need not roundtrip.

The manifest binds this stage to the unchanged parent syntax projection and exact source byte range. Parent cost remains an upper accounting context; no independent whole-file invocation or execution count is assigned to this fragment.

## models

## behavior_complement

Fault=None cannot swap numerator/denominator or drop scripts. Unknown followed by an explicit Group cannot be serialized as an immediately swallowed argument in supported normalized trees. Normal writer does not preserve original whitespace or spans.

## reverse_reconstruction

Canonical LaTeX cannot recover span coordinates or aliases. Row([Num('1'),Num('2')]) prints separated numbers while Num('12') prints joined; thus canonical bytes constrain that distinction. Same canonical symbol has multiple source alias antecedents even though the AST enum is equal.

## fault_support

SwapFrac swaps every encountered fraction, including nested; DropSup removes sup only; DropLast removes last atom of every Row len>=3. Fraction a/a can be unchanged by swap, so fault activation is not always detectable.

## optional_context

opt_arg saves/restores flag; Group temporarily resets it. A right bracket inside explicit braces can print literally, whereas one exposed in optional content must use rbrack.

## tests

### Evidence

print::tests::canonical_latex; print::tests::plain_text; integration golden outputs and corrupted_printer_turns_gate_red

Selected library filters ast:: lex:: macros:: parse:: print:: table:: and integration gate excluding document_extraction_with_preamble_macros and random_token_soup_never_panics_and_roundtrips. Direct rustc --test latex/build.rs exercises private helpers. All selected tests twice debug/release with identical sorted names. Four compiled mutants at build/AST/parser/printer boundaries fail assertions then exact restoration passes. These do not individually cover every source. Build-main failure effects, allocation failures and universal roundtrip are unproved. Strict Clippy reports existing lints. No corpus/live VMM/API/GPU/full-school/held-out tests.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: replaced stage topic lists with concrete transitions, exclusions and compatible antecedents. Four meaningful typed views replace the previous six-view bookkeeping catalog. Shared syntax and source byte bounds are unchanged.

### 2026-10-31 · tests

Heading `### Evidence — 2026-10-31` stays in tests as `### Evidence`.
