# world-state-core-anchors

Domain: finite in-memory values, successful allocation, parser-valid Cmd shapes and coherent imported registry/state metadata unless a fixture deliberately injects a mismatch. Neural parsing/model loading remains outside the domain; one fixed program exercises the deterministic change-language parser. Model status: source-derived partial relations, finite correspondence only. Unknown behavior is not excluded. Each named section defines its operator/observation; source identity and byte scopes are in manifests.

## forward

Position checks each anchor is 1..sentence_count, rejects no anchors, uses maximum anchor, rejects backward maximum relative to current sentence, derives paragraph from text, and validates optional Cursor sentence/paragraph/chapter. need checks registry kind before existence; need_event checks event presence. need_cause excludes absent cause with Silent but does not itself reject negation; real_cause rejects negated events. need_any chooses event versus entity lookup; not_part excludes an object with known PartOf.

## behavior_complement

Exclude successful position for zero/out-of-range/no anchors, a backward maximal anchor, or cursor coordinates disagreeing with text. Exclude need accepting wrong registry even when that identity exists. Local unchecked command execution without position validation is outside this exclusion.

## reverse_reconstruction

Given valid TextPos p and fixed text/current chapter, compatible anchor lists have maximum p.sent, all members in range, and satisfy cursor coordinate constraints. Duplicate/permuted anchors may share p; para is constrained by the selected sentence. A failure payload limits the earliest failing condition but is noninjective.

## anchor_priority

Range scan precedes empty/max logic; backward sentence precedes Cursor coordinate checks. Current paragraph need not be monotone independently: the input Text supplies paragraph labels. The 25-pair fixture separates accepted anchors by a direct Boolean condition.

## cause_partition

need_cause(Some(e)) proves existence only; real_cause checks x.neg only. World and reality-branch compatibility is checked later for staged deltas. Do not equate any one helper with complete causal validation.

## coordinate_fiber

Under a fixed maximum sentence s, every anchor permutation has the same position, though invalid-anchor error choice can change. Cursor None inherits current chapter; explicit chapter must be at least that chapter. Neither condition authenticates factual text content.

## invariant

Fields follow last writes; sets are ordered unique IDs with empty sets removed after deletion. Versions are appended under the stated counter bounds. Public apply's Err preserves the caller; local mutations require that outer boundary. Inverse containment links hold only after successful checking, not for arbitrary injected worlds. All preimages retain ambiguity and shared intermediate state.

## composition

Immediate children share the same Entity/Snap/World/Line values and source snapshot. Compose position -> clone -> local/narrative dispatch -> link checks -> mentions -> commit, retaining failure order and intermediate registry identity. Do not combine independent projection fibers as a Cartesian product. The file root additionally covers native test helpers and full-file glue. Story dispatch is a dependency, not a child claimed verified here.

## cost

Let H be total stored deltas, P their payload bytes, N entities, E events and K inspected link entries. Snapshot replay costs its prefix deltas plus payload cloning and ordered-map operations. apply clones all retained world text/history/story state before dispatch; that cost cannot be omitted. check_links may repeatedly replay referenced histories, so a conservative bound counts O((N+K)*(H+P+log(N+1))) work plus ordered-set operations, not a linear cached-snapshot claim. need_event is O(E); next_free scans IDs. place has 17 admitted recursion levels; ancestors holds at most 65 IDs. run sums executed calls, cloning both accepted and subsequently rejected attempts. External Story work, allocation latency and formatting costs are unmeasured; no wall-clock or asymptotic bound for opaque dependencies is asserted.

## checks

Six native wave7_world tests: 625 delta histories times five prefixes (3,125 snapshot/last-write/set-membership checks), 25 anchor pairs, staged ownership rollback and successful/no-op move, ordered mentions/cursor inheritance, spatial and registry projections, frozen event references and rejected-anchor coverage; a five-line parser/root-run join creates two locations and a character, moves it, rejects a missing location, then checks both historical and latest snapshots. Oracles use final-operation lookup and direct Boolean conditions rather than replaying the implementation. Broader command branches remain source-reviewed, not exhaustively tested. Evidence records repeated debug/release runs, a compiled production mutation and syntax identity.

## epoch-memory

Initial epoch. Prospective expectations: last-operation oracle agrees with replay; temporary-world rejection leaves the original unchanged; no-op moves affect mentions but not versions. Review records clamped snapshot versions, parser-dependent Say/Plan invariants, missing-target default snapshots, nonvalidating location lookup, and coverage of rejected lines. No production semantics were changed. Preserve these boundaries in later modeling epochs; a new registry entry alone is not verification.

## History

### 2026-10-30 · epoch-memory

2026-10-30 initial epoch. Prospective expectations: last-operation oracle agrees with replay; temporary-world rejection leaves the original unchanged; no-op moves affect mentions but not versions. Review records clamped snapshot versions, parser-dependent Say/Plan invariants, missing-target default snapshots, nonvalidating location lookup, and coverage of rejected lines. No production semantics were changed. Preserve these boundaries in later modeling epochs; a new registry entry alone is not verification.
