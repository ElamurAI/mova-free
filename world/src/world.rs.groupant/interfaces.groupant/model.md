# world-state-core-interfaces

Domain: finite in-memory values, successful allocation, parser-valid Cmd shapes and coherent imported registry/state metadata unless a fixture deliberately injects a mismatch. Neural parsing/model loading remains outside the domain; one fixed program exercises the deterministic change-language parser. Model status: source-derived partial relations, finite correspondence only. Unknown behavior is not excluded. Each named section defines its operator/observation; source identity and byte scopes are in manifests.

## forward

Speech keeps main act then additional acts without deduplication; Event.role returns first matching role, all retains duplicate roles in order, has projects participant IDs, happened requires not negated and branch real. Event state references are frozen values. Cursor/Text/Gate formatting is deterministic. World construction starts empty with document world 0, real branch, chapter 1 and a Story. Entity map access follows BTree key order; event lookup takes first matching ID. Public apply validates position, clones the full world, dispatches apply_root when story is Some and apply_local otherwise, then commits only on success. apply_local directly executes and validates links, adds strictly increasing sorted anchored mentions for touched existing entities, and returns new event IDs.

## behavior_complement

For public apply, any returned Err excludes mutation of the original world, assuming ordinary Rust execution and pure cloned state dependencies. For role projections, exclude role reordering or removal of duplicate participants from all. These are scoped inner exclusions; private apply_local is not transactional.

## reverse_reconstruction

An observed role list admits all events with that role subsequence, arbitrary other roles and irrelevant event fields. A failed public apply admits exactly input worlds/commands with the same pre-state and a failing gate along the staged path; failure message alone does not uniquely recover which world.

## atomic_commit

Position validation runs before clone. Errors from exec or post-exec check_links leave the caller unchanged because mutation occurs only in the clone. The native Have fixture reaches the post-link error after staging an inconsistent ownership update and compares the entire Debug projection before/after.

## frozen_roles

Creating an event copies current SRef values. Later appended entity states cannot change those participant versions. Duplicated agent roles remain duplicated; first-role and all-role projections have different fibers.

## mention_order

Sort all supplied anchors by TextPos, then append only positions strictly after the last mention. Repeated touched IDs and repeated anchors cannot duplicate stored mentions. A no-op Move may add a later mention without adding a state. Narrative root dispatch remains an external dependency beyond the local fixture.

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
