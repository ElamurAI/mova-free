# world-state-core-validation

Domain: finite in-memory values, successful allocation, parser-valid Cmd shapes and coherent imported registry/state metadata unless a fixture deliberately injects a mismatch. Neural parsing/model loading remains outside the domain; one fixed program exercises the deterministic change-language parser. Model status: source-derived partial relations, finite correspondence only. Unknown behavior is not excluded. Each named section defines its operator/observation; source identity and byte scopes are in manifests.

## forward

check_ev2 rejects self/event-missing references, enforces same-world explicit placement, and validates about. check_value validates Mother/Father IDs as existing distinct characters. check_links folds latest snapshots and checks bidirectional character-location, object-holder, relation-endpoint, location-parent, plan/belief-owner and part/whole links in ordered entity traversal; missing target snapshots default empty. It does not validate arbitrary semantic facts or prove acyclicity. check_parts enforces the reverse PartOf pointer. Test-only inject bypasses gates. run_mode creates a fresh root world, attempts every Line, counts successful commands, records rejected Lines/errors, and marks all attempted anchors covered even when rejected; uncovered uses sentence indices cast to u16.

## behavior_complement

Exclude check_links success when a modeled inverse pair disagrees, e.g. L.Objects contains O but O.At is C. Exclude run aborting after its first returned gate error, or treating a rejected line's anchor as uncovered. These are scoped checks, not complete schema validation.

## reverse_reconstruction

Given a Link error, retain snapshots violating its first encountered inverse relation while earlier traversed checks passed. Given accepted/rejected counts and uncovered set, many Line sequences remain possible; coverage reconstructs only the union of attempted in-range anchors, not which ones succeeded.

## bidirectional_join

For admitted coherent registries, C.At=L iff C in L.Chars, O.At=holder iff O in the matching Objects/Items set, and O.PartOf=X iff O in X.Parts. Link checker relies on snapshot folding and ordered IDs; separately correct snapshots do not prove transaction wiring.

## coverage_semantics

run_mode extends covered before calling apply. Therefore an entirely rejected command can still remove a sentence from uncovered. Sentence count<=u16::MAX is a modeling premise; larger texts truncate the range cast and are not claimed correct.

## reference_limits

check_links uses missing snapshots as empty, and does not universally validate registry type or parent/part acyclicity; command guards and caller coherence supply those premises. next_free/version counters, and aggregate collection allocation, have no recoverable overflow/resource error protocol here.

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
