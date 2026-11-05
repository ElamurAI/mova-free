# world-state-core-commands

Domain: finite in-memory values, successful allocation, parser-valid Cmd shapes and coherent imported registry/state metadata unless a fixture deliberately injects a mismatch. Neural parsing/model loading remains outside the domain; one fixed program exercises the deterministic change-language parser. Model status: source-derived partial relations, finite correspondence only. Unknown behavior is not excluded. Each named section defines its operator/observation; source identity and byte scopes are in manifests.

## forward

exec stages per-entity ordered deltas in a BTreeMap transaction, creates one state per nonempty transaction entry in ID order, with latest+1 version and shared cause/note metadata. Cursor and events append directly to the temporary world. Loc/Obj/Char create attributes and inverse containment links; Cursor inherits omitted at/time. Move updates both containment sides; Have deliberately leaves the old side for post-link rejection; Give requires current holder, distinct recipient and nearness. Set rejects At, wrong keys and silent known-value changes, with v3 repeated-death identity checks. Rel creates/updates endpoints and fact sets, leaving obsolete links for validation. Event/Say capture participant versions and metadata. Plan/Believe maintain owner links and stage transitions; Done/Unset clear known fields with causes. Feel changes a tag collection under continuity/silent-change guards. Detach/Attach rewrite part/whole and containment links with nearness and real-cause checks. Mention only contributes touched IDs. Narrative-only commands return Frame here. Before flushing deltas, a cause's world/branch must match this world.

## behavior_complement

Within parser-valid commands and coherent pre-worlds, exclude silently changing known Set fields without cause, transferring an object between characters through Move, self-Give, or committing staged changes caused by a mismatched world/branch. Other command paths remain source-reviewed rather than exhaustively verified.

## reverse_reconstruction

For a committed state delta bundle, retain prior snapshots, command and cause combinations producing the ordered bundle. Repeated equal Set or same-location Move may leave no state; metadata and mentions distinguish histories that snapshots alone collapse. Recovering a full prior world requires old values unavailable from a forward Set delta.

## transaction_grouping

Tx groups all deltas for one ID into one appended state, preserving operation order inside that state; final entity ordering follows BTreeMap. Atomicity is supplied by outer apply, not by exec. Direct cursor/event writes can precede later errors in a temporary world.

## ownership_transition

A successful object Move from L to C removes L.Objects, sets O.At=C and adds C.Items; historical snapshots remain unchanged. Same destination emits no new delta/state. Have intentionally stages only new holder and object, so old-holder mismatch is rejected by check_links in public apply.

## parser_preconditions

Say assumes a nonempty acts list, Plan assumes create or close shape, entity IDs/types/fields assume parser constraints beyond helper checks. Malformed hand-built Cmd can panic or bypass intended schema. Most tests construct valid commands directly; a fixed five-line change-language program additionally checks real parser-to-root-executor-to-snapshot wiring. Neither establishes general parser acceptance or narrative-root policy.

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
