# world-state-core-indexes

Domain: finite in-memory values, successful allocation, parser-valid Cmd shapes and coherent imported registry/state metadata unless a fixture deliberately injects a mismatch. Neural parsing/model loading remains outside the domain; one fixed program exercises the deterministic change-language parser. Model status: source-derived partial relations, finite correspondence only. Unknown behavior is not excluded. Each named section defines its operator/observation; source identity and byte scopes are in manifests.

## forward

next_free returns fixed registry order L,O,C,R,E,P,B with maximum stored number plus one; no search for internal holes. fresh rejects IDs already in either registry. place_n returns a location ID directly without existence validation; character/object PartOf takes priority over At, otherwise character holders recurse. Depth>16 gives None. ancestors follows Parent and stops before a repeated parent or after 65 entries. near accepts unknown places and accepts known equality or ancestor containment, not arbitrary siblings.

## behavior_complement

Exclude choosing a hole below an existing maximum as next_free, treating all sibling locations as near, or claiming an unknown place is a contradiction. Bounds and termination are predicates of the implemented lookup relation, not physical geography.

## reverse_reconstruction

Given resolved place L, retain all character/object holding or PartOf chains of admitted depth reaching L. Ownership identities and intermediate chain length are lost. None includes absent entities, absent At, unsupported registries and depth-limited cycles; no unique missing location is recoverable.

## priority

PartOf dominates At even if both exist in an injected inconsistent snapshot. Real executor link checks can reject such coexistence, but place alone does not validate it. A direct Reg::L ID resolves without a stored entity, as tested with L99.

## depth_budget

place permits recursive levels 0..16 and rejects entry at 17. ancestors can return at most 65 parents because its guard is out.len()>64 before push, and uses seen-parent membership to stop cycles. These are resource caps, not proofs of valid containment trees.

## registry_order

next_free exposes maximum+1 for each fixed registry; event IDs are scanned separately from entities. Requires all maxima below u32::MAX to avoid arithmetic overflow. Parser-bounded IDs satisfy this in ordinary workflows, but public Id construction can violate it; the model excludes that case explicitly.

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
