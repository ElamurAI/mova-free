# Escape alphabet, entity phrase recursion and TAM display

## forward

t prefixes backslash exactly for backslash, backtick, star, underscore, square/angle brackets, pipe and hash, copying other scalars. phrase orders possessor-or-determiner, text-or-id, modifiers, then conjunctions. Recognized possessive pronouns avoid apostrophe-s. ent appends identity only when a resolved pronoun and rendered phrase differ from id ignoring ASCII case. tam_words expands selected dot-separated tags if tam.read.

## behavior-complement

Universe: strings/entities/Frames paired with proposed display strings. Exclude dropping a nonalphabet scalar, failing to escape an alphabet scalar, emitting a determiner alongside a possessor, or emitting TAM while read=false. Newline passthrough is allowed.

## reverse-reconstruction

t is injective on scalar strings and its image admits escape decoding. phrase loses attrs/appos/and fields and can collapse different text/id/prefix splits. ent loses pron values when the annotation is suppressed; empty TAM admits unread or empty readable tags.

## invariant

Character order survives escaping. Possessor wins over determiner. Recursive Vec ordering persists. Entity nesting must be finite and stack-manageable.

## cost

t is linear in bytes. phrase costs sum of recursive subtree byte sizes because ancestors copy descendants; stack depth follows entity nesting. No depth cap is enforced.

## escape-image

Escaped output is not idempotent; t(t(s)) can add escapes. The native family enumerates 169 pairs including Unicode and the full ten-character escape alphabet.

## prefix-precedence

Partition by poss present, else det present, else neither. Changing det with poss fixed cannot alter the prefix. Fixtures cover named/pronoun possessors, modifiers and coordination.

## composition

phrase constructs raw text before ent escapes it; ent separately escapes an identity annotation. table escapes tam_words once. Dependency helper semantics are source-inspected.

## epoch-memory

Wave14 first modeling epoch: preserves rendering and tuning selection semantics. Two public pure tuning seams are now actual production-path calls. Six rendering tests and four tuning tests record field loss, tolerance/duplicate behavior, strict selection thresholds and domain limits. No bug repair or speed claim. Existing semantics retained include newline passthrough, first key/last MId collision, zip truncation and unchecked score arithmetic. Evidence is finite; no parser, teacher, filesystem corpus, journal-backed tune, optimizer training or GPU execution.

## limitations

Partial syntax binds source bytes; macro bodies remain opaque and semantic AST is absent. Native/structural checks are not universal proofs or full-system coverage. No approximation error budget or automatic substitution is claimed. File evidence distinguishes executed pure paths and inspected effects.

## History

### 2026-10-30 · epoch-memory

Wave14 first modeling epoch, 2026-10-30: preserves rendering and tuning selection semantics. Two public pure tuning seams are now actual production-path calls. Six rendering tests and four tuning tests record field loss, tolerance/duplicate behavior, strict selection thresholds and domain limits. No bug repair or speed claim. Existing semantics retained include newline passthrough, first key/last MId collision, zip truncation and unchecked score arithmetic. Evidence is finite; no parser, teacher, filesystem corpus, journal-backed tune, optimizer training or GPU execution.

### Undated · document

Subject: prag/src/frames_md.rs SHA-256 023ca98fa461e9d1545915abd8071ecb542cc61999bbf5a054b712e5edf9b5e3. All entries share this exact revision. Status: partial source-derived relations, finite native evidence for pure seams, source inspection for effects. Domain: finite supplied inputs under the stated resource/arithmetic premises. Exclusions are inner source-derived families, not a complete behavioral complement. Reverse relations retain compatible antecedents but no complete inverse solver is implemented.
