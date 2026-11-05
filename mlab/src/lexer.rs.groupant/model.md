# mlab-lexer

## descriptant

### Context-sensitive tokenization

lex scans Unicode chars, maintains one-based line/char-column, whitespace-before flag and delimiter stack. Numbers normalize D exponent for parsing and imaginary suffix j/J/I to i spelling. Apostrophe after a value is Quote except whitespace in matrix/cell context, where it starts Str. Newline is emitted; ... consumes remainder and newline. EOF token always appended.

## approximant

### Context-sensitive tokenization forward relation

lex scans Unicode chars, maintains one-based line/char-column, whitespace-before flag and delimiter stack. Numbers normalize D exponent for parsing and imaginary suffix j/J/I to i spelling. Apostrophe after a value is Quote except whitespace in matrix/cell context, where it starts Str. Newline is emitted; ... consumes remainder and newline. EOF token always appended.

Domain: UTF-8 strings, finite memory; no external effects. Mixed delimiter stack pops are permissive. Block marker recognition requires trimmed whole line at line start.

No executable Rust surrogate is supplied: categorical decisions require exact observations; numerical kernels have no measured approximation metric here. Future supported caps are 1%, 2%, 5%, 10% only.

## invariant

### Invariant and cost

Coordinates count chars, not UTF-8 bytes. EOF exists even for empty input. Newlines inside parentheses remain tokens despite the file header's stale whitespace description. Numeric overflow may produce nonfinite f64; no finite-value guarantee.

Cost: O(C+token text) ordinary scan and O(C+T+depth) memory; line_rest allocates trimmed lines for block comments. No source-size cap.

Failure ordering: Whitespace/comment/continuation processing precedes token detection; numeric detection precedes identifier detection; contextual apostrophe precedes generic operators. First encountered literal/invalid-character error returns its opening/current position.

## composition

### Parent composition

parser consumes Newline, space_before and token tags, so dropping spacing changes [1 -2] and [1 - 2]. AST Num/Imag carry original/normalized spelling; coordinate changes alter diagnostics.

Scoped fragments are projections over exact byte intervals in this source; they share one parent syntax projection. Uncovered declarations and embedded checks remain parent modeled. Whitespace/comment/continuation processing precedes token detection; numeric detection precedes identifier detection; contextual apostrophe precedes generic operators. First encountered literal/invalid-character error returns its opening/current position.

The relation is source-derived and finite checks cannot establish complete correctness or a composition-wide numerical error bound.

## models

## behavior_complement

Accepted token sequences exclude treating [a 'x']'s spaced quote as transpose, or a' as a string opener. Unterminated strings exclude Ok; invalid characters exclude tokens past the error. Scanner does not reject unmatched delimiters or unterminated block-comment EOF; these are not lexer validation guarantees.

## reverse_reconstruction

For tokens T, preimage is all Unicode strings whose contextual scan yields T including retained locations/space flags. Comments, quote escapes and normalized imaginary suffixes can collapse. Removing coordinate observations enlarges fibers further; token text is not an exact inverse source serializer.

## representation_transport

Coordinates count chars, not UTF-8 bytes. EOF exists even for empty input. Newlines inside parentheses remain tokens despite the file header's stale whitespace description. Numeric overflow may produce nonfinite f64; no finite-value guarantee.

## failure_precedence

Whitespace/comment/continuation processing precedes token detection; numeric detection precedes identifier detection; contextual apostrophe precedes generic operators. First encountered literal/invalid-character error returns its opening/current position.

## resource_domain

UTF-8 strings, finite memory; no external effects. Mixed delimiter stack pops are permissive. Block marker recognition requires trimmed whole line at line start.

## interaction_relation

parser consumes Newline, space_before and token tags, so dropping spacing changes [1 -2] and [1 - 2]. AST Num/Imag carry original/normalized spelling; coordinate changes alter diagnostics.


## tests

### Source-specific bounded evidence

`lexer::wave30::context`: matrix quote versus transpose, normalized imaginary suffix, continuation coordinates, emitted parenthesized newline, unterminated string, EOF.

Unexecuted here: No exhaustive Unicode, block-comment or numeric scan coverage. No corpus, GPU, live filesystem, timer or callback run. No broad Clippy claim; dependency/style checks were not repeated. Only appended test blocks were rustfmt-formatted; original production bytes are unchanged.

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective source reading precedes test additions. Retained observations: parser consumes Newline, space_before and token tags, so dropping spacing changes [1 -2] and [1 - 2]. AST Num/Imag carry original/normalized spelling; coordinate changes alter diagnostics.

This is partial source-derived modeling, not proof, full semantic coverage, or a performance result. Public malformed representations, unbounded allocations and exhaustive numerical edge cases remain outside the checked fixtures.

### 2026-10-31 · tests

Seven named tests each passed twice debug and twice release on 2026-10-31, with actual named log lines sorted and identical across all four runs (28 selected successful executions). Each source has a compiled changed-assertion negative control; six runtime sources additionally have compiled production-body mutants that fail those unchanged tests. AST has representation evidence only. Every mutant is byte-restored and its positive test rerun. See `evidence.json` for this source's exact hash, mutation and exit statuses. External receipts retain full logs, command metadata, control records and prospective notes.
