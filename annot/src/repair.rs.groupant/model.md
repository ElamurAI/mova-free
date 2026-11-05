# annot-repair

## descriptant

### annot-repair

Deterministic Early Modern English lexical, feature and postverbal-not repairs on selected ten-column word rows, with per-rule changed-word counts. text is a line-oriented normalization wrapper, not a byte-preserving serializer.

## approximant

### Forward approximant

sentence selects all-digit first fields, copies their columns, returns unchanged if any selected row lacks exactly10 columns. It parses tag/upos/rel, applies matching emode table/ending before archaic verb lemma before eligible lowercase lemma, counts only changed rows, then postverbal_not updates heads/relations and writes selected rows back. text flushes sentence blocks and preserves comment content while normalizing newlines.

Cost: B row bytes and N selected words: O(B) projection/serialization plus delegated morphology/table lookup and postverbal_not cost. Temporary columns/forms O(B+N). Counters usize and string allocation have no fallible resource boundary or overflow certificate.

Executable approximation gap: categorical parsing, effects and validation lack a useful measured percent metric here. No executable surrogate or 1/2/5/10% profile is attached; an exact test is not an approximant.

## invariant

### Invariants and limits

Malformed selected row causes no mutation/count increment for the entire sentence. Nonselected range/empty-node lines remain untouched by sentence. Lexical branches do not change FORM, UPOS or XPOS. Counts are additive; each lexical row contributes to at most one branch plus possible not repair.

- Existing broad repair test returns early if dictionary is empty; new shape/lowercase test does not depend on dictionary presence.
- No universal idempotence, morphology correctness or malformed-head preservation claim.

## composition

### Source composition

Nested scopes use UTF-8 byte offsets [start,end) against the exact source SHA-256. They share the parent lexical projection; no nested fragment becomes a second registry source. Source outside these scopes, including helper types/tests, remains parent-modeled.

- sentence [1310,4740): Shape gate precedes ordered lexical changes, postverbal-not repair and selected-row publication.
- text [4829,5898): Block flush separates comments from word rows, applies sentence and rebuilds normalized newline text.

sentence selects all-digit first fields, copies their columns, returns unchanged if any selected row lacks exactly10 columns. It parses tag/upos/rel, applies matching emode table/ending before archaic verb lemma before eligible lowercase lemma, counts only changed rows, then postverbal_not updates heads/relations and writes selected rows back. text flushes sentence blocks and preserves comment content while normalizing newlines.

Cross-file join: ann.rs supplies Ann/Item, parse/check/write; main supplies Outcome and finish; batch and llm prompts feed codex_batch; hints and repair influence draft content before gates. These joins do not establish remote correctness or crash durability.

## models

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

### Bounded source evidence

Selected actual-code tests (twice debug, twice release):

- repair::wave27_tests::malformed_word_aborts_before_repair_and_ranges_survive
- repair::tests::repairs_with_negative_controls

Run command: cargo test -p annot --bin annot --test codex_batches [--release] -- --test-threads=1 --skip ann:: --skip bfs_transfer_checks. The fake executable fixture uses temporary files, no live teacher/API. Paired named outcomes must match. Six compiled production mutations target batch last-wins, llm closed-set, repair shape gate, hints severity, main seed alias and queue resume. Each compilation exits0, test exits101, exact source restored and focused positive rerun exits0. The fixture itself has no separate mutation. Existing repair test may early-return without dictionary; independent added repair test always runs.

Unexecuted limits:
- Existing broad repair test returns early if dictionary is empty; new shape/lowercase test does not depend on dictionary presence.
- No universal idempotence, morphology correctness or malformed-head preservation claim.

Validation limits: strict scoped Clippy exited101 on six pre-existing lints (three collapsible_if, one needless_range_loop, one type_complexity, one redundant_locals). Re-running with those four known lint categories allowed exited0. changed with an external augmented registry exited0 and found no missing changed-source models. Local groupants inspection reports only expected ancestor child/source-binding updates owned by integration; no owned leaf/fragment diagnostics.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: source read before tests/models. Prospective notes and numeric receipts retained externally under the annotation wave27 run. Four new inline tests supplement existing bounded tests. Preserve distinctions: compatibility cost is not subscription cost; mock process tests do not establish live teacher behavior; registry presence is partial modeling.
