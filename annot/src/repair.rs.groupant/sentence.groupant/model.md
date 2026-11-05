# annot-repair-sentence

## descriptant

### annot-repair-sentence

Shape gate precedes ordered lexical changes, postverbal-not repair and selected-row publication.

## approximant

### Scoped forward relation

Shape gate precedes ordered lexical changes, postverbal-not repair and selected-row publication.

Parent composition and symbolic resource costs remain in ../model.md#approximant. No executable percentage surrogate is supplied for this categorical/effect scope.

## invariant

### Scoped fragment: sentence

Local scope: Shape gate precedes ordered lexical changes, postverbal-not repair and selected-row publication.

The following parent relations supply composition context; claims apply only where this byte scope implements the stated operation. Other operations remain sibling/parent obligations.

### Invariants and limits

Malformed selected row causes no mutation/count increment for the entire sentence. Nonselected range/empty-node lines remain untouched by sentence. Lexical branches do not change FORM, UPOS or XPOS. Counts are additive; each lexical row contributes to at most one branch plus possible not repair.

- Existing broad repair test returns early if dictionary is empty; new shape/lowercase test does not depend on dictionary presence.
- No universal idempotence, morphology correctness or malformed-head preservation claim.

## models

# Scoped fragment: sentence

Local scope: Shape gate precedes ordered lexical changes, postverbal-not repair and selected-row publication.

The following parent relations supply composition context; claims apply only where this byte scope implements the stated operation. Other operations remain sibling/parent obligations.

## behavior_complement

A non-ten-column selected row excludes all lexical and not repairs for that sentence. PROPN/NOUN/ADJ/NUM/X/SYM exclude lowercase-only branch; lemma I is preserved. Range and decimal ids are excluded from selection; empty id accidentally satisfies all-digit predicate.

## reverse_reconstruction

Repaired rows admit original rows differing in corrected lemma/features/head/relation, subject to ordered rule predicates. Counts restrict the number of changed rule applications but do not identify old values. text output admits LF or CRLF input and final-newline variants; original bytes cannot be uniquely reconstructed.

## atomic_shape_gate

All selected column widths are checked before any per-word mutation. Invalid tag/upos/rel skips lexical branch, while postverbal_not sees fallback enum/head values.

## rule_precedence

emode::fix with same upos wins, tagged distinguishes table vs ending; then eligible archaic fallback; only afterward capitalized lemma lowering. This is not an unordered set of substitutions.

## count_units

Counters count changed words per lexical rule and n returned by postverbal_not, not bytes or sentences; repeated application on tested normalized fixtures does not increase counts.

## newline_projection

text uses str::lines, outputs newline after each retained nonblank line and each blank separator; whitespace-only blanks normalize to empty and final newline may be added. Comments retain content but not original line terminators.


## tests

### Scoped fragment: sentence

Local scope: Shape gate precedes ordered lexical changes, postverbal-not repair and selected-row publication.

The following parent relations supply composition context; claims apply only where this byte scope implements the stated operation. Other operations remain sibling/parent obligations.

### Bounded source evidence

Selected actual-code tests (twice debug, twice release):

- repair::wave27_tests::malformed_word_aborts_before_repair_and_ranges_survive
- repair::tests::repairs_with_negative_controls

Run command: cargo test -p annot --bin annot --test codex_batches [--release] -- --test-threads=1 --skip ann:: --skip bfs_transfer_checks. The fake executable fixture uses temporary files, no live teacher/API. Paired named outcomes must match. Six compiled production mutations target batch last-wins, llm closed-set, repair shape gate, hints severity, main seed alias and queue resume. Each compilation exits0, test exits101, exact source restored and focused positive rerun exits0. The fixture itself has no separate mutation. Existing repair test may early-return without dictionary; independent added repair test always runs.

Unexecuted limits:
- Existing broad repair test returns early if dictionary is empty; new shape/lowercase test does not depend on dictionary presence.
- No universal idempotence, morphology correctness or malformed-head preservation claim.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: scoped source-derived model. Byte scope [1310,4740); AST is shared partial projection, not a separate semantic proof.
