# world-state-core-snapshots

Domain: finite in-memory values, successful allocation, parser-valid Cmd shapes and coherent imported registry/state metadata unless a fixture deliberately injects a mismatch. Neural parsing/model loading remains outside the domain; one fixed program exercises the deterministic change-language parser. Model status: source-derived partial relations, finite correspondence only. Unknown behavior is not excluded. Each named section defines its operator/observation; source identity and byte scopes are in manifests.

## forward

Delta, State, Snap and Entity histories. Snap fields are partial maps and sets are ordered unique IDs. Known Set overwrites, Unknown Set removes, Add inserts, Del removes and drops an empty set. Entity.snap(v) folds all deltas in the first min(v,len) states, including v=0 empty and oversized v clamped. state(v) instead returns None outside 1..len. now uses latest; set_by scans backward through the prefix for the last Set regardless of whether its value changed; ver_at scans state insertion order backward for a sentence not greater than the query.

## behavior_complement

Exclude a retained field after its final Unknown Set, duplicate set members, retained empty sets after last deletion, and snapshots depending on suffix states after requested prefix. These are inner forbidden transitions on valid finite histories, not a proof from negative examples.

## reverse_reconstruction

Given snapshot S and prefix v, retain every length-v delta history whose per-field last write and per-ID last set operation yield S. Earlier overwritten values and add/delete pairs are free; metadata and later suffixes are invisible to S. This fiber is nonunique. The finite five-operation alphabet enumerates 625 histories and all five prefixes.

## last_write

For one field f, snapshot value is the last Set(f,value) in the chosen prefix or Unknown. set_by recovers that operation's containing state, including an Unknown write. It does not reconstruct the overwritten value or imply a semantic change.

## membership

For one set/member pair, membership is true iff the latest Add/Del for the pair is Add. Other pairs commute, repeated identical operations are idempotent, but opposite operations on the same pair do not commute. Ordered IDs are a separate representation invariant.

## version_domain

snap(99) can equal now while state(99) is None on short histories. ver_at chooses last inserted qualifying state rather than maximum numerical sentence if imported state metadata is unordered. Versions/count casts require fewer than 2^32 states; metadata coherence is a caller premise.

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
