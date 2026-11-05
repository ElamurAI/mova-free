# latex-lex-stage2

## descriptant

### latex-lex-stage2

render maps Cmd/CmdRaw(n) to backslash+n, adding a space when an ASCII-letter command is followed by Char(ASCII letter). Other punctuation tokens print literally, Param(k) prints # plus digit, Space prints one space, End prints nothing.

## approximant

### Scoped forward relation

render maps Cmd/CmdRaw(n) to backslash+n, adding a space when an ASCII-letter command is followed by Char(ASCII letter). Other punctuation tokens print literally, Param(k) prints # plus digit, Space prints one space, End prints nothing.

This is an exact source-derived partial model, not an executable approximate replacement.

## invariant

### Scoped invariants

render cannot retain CmdRaw versus Cmd or End markers. It does not universally preserve arbitrary token identity under lex: an explicitly stored Space after a control word can be swallowed on relex.

render(CmdRaw(n))=render(Cmd(n)); macro capture semantics must remain in tokens rather than relying on rendered text to preserve raw status.

Cmd(alpha),Char(x) renders with a separating space so relex keeps two tokens; Cmd(alpha),Char('1') needs no separator because digits cannot extend the control word.

## composition

### Scoped interaction

Text/column/environment arguments use render's canonical bytes, not a raw-source recovery. Included tests constrain stable render(relex(render(...))) on named fixtures.

The manifest binds this stage to the unchanged parent syntax projection and exact source byte range. Parent cost remains an upper accounting context; no independent whole-file invocation or execution count is assigned to this fragment.

## models

## behavior_complement

render cannot retain CmdRaw versus Cmd or End markers. It does not universally preserve arbitrary token identity under lex: an explicitly stored Space after a control word can be swallowed on relex.

## reverse_reconstruction

Cmd(alpha) and CmdRaw(alpha) have identical rendered bytes; insertion of End anywhere also leaves bytes unchanged. Rendered control-word separator spaces can originate from generated separation or input Space, so original token vectors are not uniquely recoverable.

## raw_flag_erasure

render(CmdRaw(n))=render(Cmd(n)); macro capture semantics must remain in tokens rather than relying on rendered text to preserve raw status.

## separator_adapter

Cmd(alpha),Char(x) renders with a separating space so relex keeps two tokens; Cmd(alpha),Char('1') needs no separator because digits cannot extend the control word.

## tests

### Evidence

lex::tests::lexes_commands_and_spaces; lex::tests::positions_are_bytes; lex::tests::render_is_stable

Selected library filters ast:: lex:: macros:: parse:: print:: table:: and integration gate excluding document_extraction_with_preamble_macros and random_token_soup_never_panics_and_roundtrips. Direct rustc --test latex/build.rs exercises private helpers. All selected tests twice debug/release with identical sorted names. Four compiled mutants at build/AST/parser/printer boundaries fail assertions then exact restoration passes. These do not individually cover every source. Build-main failure effects, allocation failures and universal roundtrip are unproved. Strict Clippy reports existing lints. No corpus/live VMM/API/GPU/full-school/held-out tests.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: replaced stage topic lists with concrete transitions, exclusions and compatible antecedents. Four meaningful typed views replace the previous six-view bookkeeping catalog. Shared syntax and source byte bounds are unchanged.

### 2026-10-31 · tests

Heading `### Evidence — 2026-10-31` stays in tests as `### Evidence`.
