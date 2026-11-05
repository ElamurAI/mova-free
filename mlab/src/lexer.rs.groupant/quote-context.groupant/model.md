# mlab-lexer-quote-context

## descriptant

### Quoted literal versus transpose

At apostrophe compute prev_value from last token and in_matrix from top delimiter. Quote iff prev_value && !(space_before && in_matrix); otherwise decode single quote string, doubling apostrophes. Double quote decodes known backslash escapes and doubled quotes.

## approximant

### Quoted literal versus transpose forward relation

At apostrophe compute prev_value from last token and in_matrix from top delimiter. Quote iff prev_value && !(space_before && in_matrix); otherwise decode single quote string, doubling apostrophes. Double quote decodes known backslash escapes and doubled quotes.

Domain: UTF-8 input and scanner state at string branch; no evaluation or file effects.

No executable Rust surrogate is supplied: categorical decisions require exact observations; numerical kernels have no measured approximation metric here. Future supported caps are 1%, 2%, 5%, 10% only.

## invariant

### Invariant and cost

Literal error points to opening column; single-quote payload marks false, double-quote true. Context stack influences classification but is not delimiter validation.

Cost: O(literal chars), O(decoded chars) allocation.

Failure ordering: Previous-value discrimination precedes string consumption; doubled quote precedes closing-quote branch; recognized backslash escapes consume two chars.

## models

## behavior_complement

For a previous identifier a, a' excludes Str while [a 'x'] excludes Quote at the spaced opener. Newline/EOF before closing quote excludes successful literal.

## reverse_reconstruction

Decoded single quote x has all escaped source spellings producing x in a non-transpose context; output Quote requires preceding value and absence of spaced matrix condition. Original escape spelling cannot be reconstructed uniquely.

## representation_transport

Literal error points to opening column; single-quote payload marks false, double-quote true. Context stack influences classification but is not delimiter validation.

## failure_precedence

Previous-value discrimination precedes string consumption; doubled quote precedes closing-quote branch; recognized backslash escapes consume two chars.

## resource_domain

UTF-8 input and scanner state at string branch; no evaluation or file effects.

## interaction_relation

Str becomes AST Str, Quote becomes postfix CTranspose. parser matrix spacing must consume the unchanged space_before flag.


## tests

### Bounded evidence

Selected actual-code Rust test: `lexer::wave30::context` with `cargo test --offline --locked -p mlab --lib`. CPU synthetic fixtures only. Two debug and two release runs, sorted named outcomes, numeric statuses, compiled assertion controls and exact source restoration are retained externally under the wave30 expressions receipt directory. The source-to-test map distinguishes runtime semantics from AST representation checks. This fragment shares the parent test: only the exercised branches are observed; its whole prose is not runtime coverage. No corpus, GPU, live filesystem inspection, callbacks, RNG shuffle or wall-clock execution.

Syntax is generated twice from exact source and compared. Lexical projection has no expansion/type/name resolution. No strict Clippy claim; existing dependency/style findings are not re-run merely for repetition.

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective source reading precedes test additions. Retained observations: Str becomes AST Str, Quote becomes postfix CTranspose. parser matrix spacing must consume the unchanged space_before flag.

This is partial source-derived modeling, not proof, full semantic coverage, or a performance result. Public malformed representations, unbounded allocations and exhaustive numerical edge cases remain outside the checked fixtures.
