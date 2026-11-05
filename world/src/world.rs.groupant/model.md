# world-state-core

## model

# world-state-core

Domain: finite in-memory values, successful allocation, parser-valid Cmd shapes and coherent imported registry/state metadata unless a fixture deliberately injects a mismatch. Neural parsing/model loading remains outside the domain; one fixed program exercises the deterministic change-language parser. Model status: source-derived partial relations, finite correspondence only. Unknown behavior is not excluded. Each named section defines its operator/observation; source identity and byte scopes are in manifests.

## forward

The source implements delta snapshots and entity history; event/cursor interfaces; anchored transactional application; registry/spatial lookups; local command staging; inverse-link/reference validation; and multi-command reporting. The children below specify these actual source regions. Public apply composes validation, copied state mutation and atomic commit. Only scoped local projections have new finite tests; command-level linguistic truth and narrative Story semantics remain external.

## behavior_complement

For public apply on parser-valid commands and admitted coherent finite worlds, returned Err excludes a changed caller state. Successful local dispatch excludes any mismatched checked inverse link, and accepted snapshots cannot depend on future deltas. This is an inner forbidden-transition model; external Story policy and unchecked numeric exhaustion are explicit gaps.

## reverse_reconstruction

Observed snapshot/cursor/event projections admit every prior world and valid command trace consistent with last-write folding, frozen references, mention ordering and gates. Overwritten scalar values, superseded holders and omitted event metadata prevent unique recovery. Snapshot fibers are bounded-enumerated; general reconstruction is relational, not an executable inverse.

## snapshot_projection

Entity snapshots discard cause/line/note and all overwritten values. State and set_by retain some provenance, but snap(v) clamps oversized v while state(v) rejects it. Cross-checking these APIs requires their different version-domain contracts.

## atomic_boundary

Public apply validates anchors, clones, dispatches, then commits. Private exec/apply_local mutate directly and can fail after writes, so callers must not generalize the rollback law to them. A failing post-link Have fixture demonstrates the complete local transactional path.

## ownership_composition

Move's three entity updates and subsequent inverse-link checks share one temporary world and one committed position. History remains immutable while current object/holder snapshots change together; no-op Move can still append a mention.

## cursor_and_coverage

Position uses maximum valid anchor and text paragraph; mentions sort and strictly append. run_mode counts attempted anchors even for rejected commands. Coverage is text participation, not accepted semantic understanding.

## resource_domain

Assume finite allocations, existing and appended entity history lengths below 2^32, next-free maxima below u32::MAX and at most u16::MAX sentences for run coverage. place and ancestors impose separate traversal caps. No claim of checked arithmetic for all publicly constructible worlds, nor measured speedup.

## dependency_boundary

types supplies closed tags/key policy, lang supplies schema-valid commands, prag supplies Act names, and story::apply_root supplies narrative dispatch/time/reality semantics. These remain fixed external relations. New tests invoke the deterministic change-language parser on one fixed five-command program, then actual root run and historical/current snapshots. They load no neural parser/model and use no files, environment writes, teacher or GPU. This tiny join observes accepted moves and a rejected missing-location move, not all parser or Story behavior.

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

## evidence

### World core evidence

Actual bounded wiring join: lang::parse_all on five commands creates L1/L2, creates C1 at L1, moves C1 to L2, then attempts missing L9. world::run accepts four and rejects the fifth with Ref; C1 retains two states, historical At=L1 and current At=L2; all links pass, committed position remains sentence 2, and attempted rejected sentence 3 is covered. This is deterministic change-language parsing, not neural parsing or a general narrative correctness result. No live I/O, environment mutation, teacher, neural model or GPU is used by the tests.

## History

### 2026-10-30 · epoch-memory

2026-10-30 initial epoch. Prospective expectations: last-operation oracle agrees with replay; temporary-world rejection leaves the original unchanged; no-op moves affect mentions but not versions. Review records clamped snapshot versions, parser-dependent Say/Plan invariants, missing-target default snapshots, nonvalidating location lookup, and coverage of rejected lines. No production semantics were changed. Preserve these boundaries in later modeling epochs; a new registry entry alone is not verification.

### 2026-10-30 · evidence

Heading `### World core evidence, 2026-10-30` stays in evidence as `### World core evidence`.

Source SHA-256: `ce8f9760a986966e234aebdd5e4709ac82002515fcb126b2f95d1b63cbdf91c4`. Production prefix is byte-identical to the parent revision; only six inline native tests were appended. No production fix or broad formatting change.

Six tests passed twice each in debug and release after the final parser join was added. All four sorted test-name/outcome records were byte-identical. Normalized outcome SHA-256: `0b4110decf08fe2a735b7a73a8ab766b03233646d4030511cca27854b23d5dd9`. The snapshot oracle checks 625 histories x five prefixes = 3,125 cases using the final relevant operation, plus 25 independently classified anchor pairs. Additional fixtures test staged post-link rollback, successful/no-op move and immutable history, cursor inheritance and mention ordering, spatial/registry projections, frozen participant versions and rejected-command coverage.

Negative control: compiled actual production mutation replaces Unknown Set removal with a read-only lookup. The independent delta-prefix test fails a real equality assertion after compilation. Source restored byte-for-byte and all six tests passed again. Mutation-log SHA-256: `38eae2d34ff3a1e1a2af5500929f8ed2ac08c1293ad4466e242a26395a948578`.

Syntax generated twice with mova-modelcheck and compared byte-for-byte; syntax SHA-256: `885e4032e594d8d65bdbff04d4c7ec555e4ed0ed3599c57c17bf306d20ac5809`. An isolated byte-identical source/sidecar snapshot passed groupant structural checking for the file root and six scoped fragments; this does not validate integration with ancestor registry entries. The new test block alone passes rustfmt check; legacy file formatting is preserved. Structural output: `{"diagnostics":[],"group_sidecars":0,"missing_group_sidecars":0,"missing_sidecars":0,"sidecars":1,"valid":true,"verification_inferred":false}`.

External run directory basename: octopus-wave7-world-2026-10-30. Logs debug1-final, debug2-final, release1-final, release2-final, compiled-mutant, restored-debug and leaf-structure remain there. Models contain eight root and five per-child typed relations (38 total), behavior complement first and reverse reconstruction second. Broader command policies, all Story branches, allocation failure and counter exhaustion remain explicit gaps; registration is not proof.
