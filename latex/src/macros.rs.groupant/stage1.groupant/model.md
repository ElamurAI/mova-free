# latex-macros-stage1

## descriptant

### latex-macros-stage1

read_newcommand consumes optional star/name, optional count0..9, optional default only if nargs>0, then one token/group body. read_def consumes name and parameter text to Open, reads balanced body, then requires parameter numbers1,2,... in encounter order. read_let consumes optional equals and one target; declareop builds CmdRaw(operatorname), optional star, braces and text.

## approximant

### Scoped forward relation

read_newcommand consumes optional star/name, optional count0..9, optional default only if nargs>0, then one token/group body. read_def consumes name and parameter text to Open, reads balanced body, then requires parameter numbers1,2,... in encounter order. read_let consumes optional equals and one target; declareop builds CmdRaw(operatorname), optional star, braces and text.

This is an exact source-derived partial model, not an executable approximate replacement.

## invariant

### Scoped invariants

Newcommand count10 or nonnumber cannot succeed. A default is not read when count0. read_def out-of-order diagnosis cannot precede an unclosed-body error because body is read first. read_let failure may consume its invalid Close/End target.

newcommand missing name precedes count validation, which precedes body availability; def missing/unclosed body precedes parameter order checks. Body parameter references are not checked against nargs here.

DeclareMathOperator emits a raw operatorname token so a macro of that name cannot intercept the generated operator. Its output braces and star are manufactured at the supplied command position.

## composition

### Scoped interaction

Successful readers return borrowed-name/Macro pairs for scan or parser-local commit. No reader itself inserts a macro table entry.

The manifest binds this stage to the unchanged parent syntax projection and exact source byte range. Parent cost remains an upper accounting context; no independent whole-file invocation or execution count is assigned to this fragment.

## models

## behavior_complement

Newcommand count10 or nonnumber cannot succeed. A default is not read when count0. read_def out-of-order diagnosis cannot precede an unclosed-body error because body is read first. read_let failure may consume its invalid Close/End target.

## reverse_reconstruction

Simple def and newcommand may construct equal zero-argument bodies from different spellings. Definition outputs omit name braces/star syntax except declareop's semantic star. Error tuples constrain the first failing read and position but do not recover consumed prefixes.

## definition_precedence

newcommand missing name precedes count validation, which precedes body availability; def missing/unclosed body precedes parameter order checks. Body parameter references are not checked against nargs here.

## raw_operator_encoding

DeclareMathOperator emits a raw operatorname token so a macro of that name cannot intercept the generated operator. Its output braces and star are manufactured at the supplied command position.

## tests

### Evidence

macros::tests::scans_definitions; macros::tests::delimited_def

Selected library filters ast:: lex:: macros:: parse:: print:: table:: and integration gate excluding document_extraction_with_preamble_macros and random_token_soup_never_panics_and_roundtrips. Direct rustc --test latex/build.rs exercises private helpers. All selected tests twice debug/release with identical sorted names. Four compiled mutants at build/AST/parser/printer boundaries fail assertions then exact restoration passes. These do not individually cover every source. Build-main failure effects, allocation failures and universal roundtrip are unproved. Strict Clippy reports existing lints. No corpus/live VMM/API/GPU/full-school/held-out tests.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: replaced stage topic lists with concrete transitions, exclusions and compatible antecedents. Four meaningful typed views replace the previous six-view bookkeeping catalog. Shared syntax and source byte bounds are unchanged.

### 2026-10-31 · tests

Heading `### Evidence — 2026-10-31` stays in tests as `### Evidence`.
