# world-narr

## model

# world-narr

## forward

World::apply clones the world before apply_root. apply_root temporarily takes Story, routes v3 commands to exec_v3 or ordinary commands to the current frame's world, restores Story and returns a Result. register joins newly created events to line/placement/bounds; cause contradictions are soft warnings. relate snapshots the time graph for its own multi-edge rollback. Past/new/branch closure exports events; only Past recreates/merges selected entities.

## invariant

Public refusal preserves the original World through the outer clone. Internal helpers may mutate before error and do not promise rollback. Canonical references claim/know/reveal/time do not create a second event. Frame stack closes only the deepest nonroot frame. New/branch world identities are distinct axes.

## cost

Public apply includes a whole World clone. Routing scans frame parents; event_any scans root then frames and events. Relation insertion clones graph and performs up to two BFS checks. Past projection materializes current entity snapshots; merge snapshots parent per recreation command and checks links. No constant-time public transaction claim.

## composition

Actual join is World::apply -> position -> cloned apply_root -> step -> exec_v3 or frame apply_local -> register -> TimeGraph. Time command calls relate. Reveal may add a cause edge before relate fails: outer rollback, not relate alone, protects it. merge_from performs per-command rollback and may commit other successful recreations with warnings. Unmodeled local event gates and reconstruction command validity remain external.

## checks

Native narr::wave9_models tests exercise the bounded domains described in model.md#evidence at the file root. Ordinary test bodies have partial syntax projections; macro token contents are opaque. The graph oracle uses a dense three-state Floyd closure, not the implementation BFS/bitset algorithm. Typed commands call the real public World::apply boundary.

## epoch-memory

Initial modeling epoch: distinguish admitted graph states from arbitrary public-field edits, and internal mutation from public commit. No production bug was established. In particular narr::relate restores its multi-edge batch, and World::apply discards a cloned failed command even when apply_root has already added a cause edge. Creation ordering has a deliberate 64-round fallback, not an unrestricted topological-sort guarantee. No percentage-error approximant or performance claim.

## behavior-complement

Subject: world-narr, source-bound revision. Domain and observation: the finite typed state/transition family in forward, with dependencies fixed. Operator: Within valid typed World::apply commands, rejected nested close, future knowledge or contradictory reveal cannot commit partial state. Different-world time relations are refused. Role overwrite by reveal is absent when its existing role is a different id. This inner exclusion does not apply to directly calling internal apply_root/register.

Assurance: source-derived partial relation, with finite native evidence where named in checks. Unknown paths remain unknown; no universal coverage or unique inverse is inferred. Complement claims are inner absent-behavior bounds under their stated premises; reverse relations retain all compatible antecedents in their declared family.

## reverse-reconstruction

Subject: world-narr, source-bound revision. Domain and observation: the finite typed state/transition family in forward, with dependencies fixed. Operator: Given public output World, retain all valid command/prestate pairs whose cloned execution commits it. Refused transitions all return unchanged state, so unchanged World alone cannot distinguish a refusal from a successful observational no-op. Claims with different interpretation strings retain one canonical event id; event-count output loses those interpretations.

Assurance: source-derived partial relation, with finite native evidence where named in checks. Unknown paths remain unknown; no universal coverage or unique inverse is inferred. Complement claims are inner absent-behavior bounds under their stated premises; reverse relations retain all compatible antecedents in their declared family.

## transaction-boundary

Subject: world-narr, source-bound revision. Domain and observation: the finite typed state/transition family in forward, with dependencies fixed. Operator: Public apply commits only complete success. Internal apply_root may append a cause edge before a later relation error; the native test proves why the caller clone is part of the model.

Assurance: source-derived partial relation, with finite native evidence where named in checks. Unknown paths remain unknown; no universal coverage or unique inverse is inferred. Complement claims are inner absent-behavior bounds under their stated premises; reverse relations retain all compatible antecedents in their declared family.

## relation-batch

Subject: world-narr, source-bound revision. Domain and observation: the finite typed state/transition family in forward, with dependencies fixed. Operator: During requires S(b)<=S(a) and E(a)<=E(b); Same requires S(a)<=E(b) and S(b)<=E(a). relate restores graph if either insertion fails, including a second-edge failure.

Assurance: source-derived partial relation, with finite native evidence where named in checks. Unknown paths remain unknown; no universal coverage or unique inverse is inferred. Complement claims are inner absent-behavior bounds under their stated premises; reverse relations retain all compatible antecedents in their declared family.

## frame-resumption

Subject: world-narr, source-bound revision. Domain and observation: the finite typed state/transition family in forward, with dependencies fixed. Operator: Existing closed frame resumes only with same parent, narrator and level. Resume reuses frame index, world id and history, adding one span; it does not allocate a fresh axis.

Assurance: source-derived partial relation, with finite native evidence where named in checks. Unknown paths remain unknown; no universal coverage or unique inverse is inferred. Complement claims are inner absent-behavior bounds under their stated premises; reverse relations retain all compatible antecedents in their declared family.

## canonical-observer

Subject: world-narr, source-bound revision. Domain and observation: the finite typed state/transition family in forward, with dependencies fixed. Operator: Claim/know/reveal reference registered canonical event identity. Distinct claims can change entity/observer records with unchanged event count; observation count is a lossy projection.

Assurance: source-derived partial relation, with finite native evidence where named in checks. Unknown paths remain unknown; no universal coverage or unique inverse is inferred. Complement claims are inner absent-behavior bounds under their stated premises; reverse relations retain all compatible antecedents in their declared family.

## past-projection-fiber

Subject: world-narr, source-bound revision. Domain and observation: the finite typed state/transition family in forward, with dependencies fixed. Operator: Fix retained fields, sets and Id. The past-projection fiber contains histories differing in dynamic fields, mentions and version chain; a chosen projected singleton is not recovery of the actual past.

Assurance: source-derived partial relation, with finite native evidence where named in checks. Unknown paths remain unknown; no universal coverage or unique inverse is inferred. Complement claims are inner absent-behavior bounds under their stated premises; reverse relations retain all compatible antecedents in their declared family.

## merge-partial-commit

Subject: world-narr, source-bound revision. Domain and observation: the finite typed state/transition family in forward, with dependencies fixed. Operator: merge_from imports events before entity recreation; each failed recreation restores only that command's parent snapshot and emits warning. A closed frame may therefore succeed with partial entity enrichment.

Assurance: source-derived partial relation, with finite native evidence where named in checks. Unknown paths remain unknown; no universal coverage or unique inverse is inferred. Complement claims are inner absent-behavior bounds under their stated premises; reverse relations retain all compatible antecedents in their declared family.

## line-splicing

Subject: world-narr, source-bound revision. Domain and observation: the finite typed state/transition family in forward, with dependencies fixed. Operator: When a real main event becomes unreal without forbidden consequences, unchain drops incident Chain/InFrame edges and stitches neighboring main ids; explicit temporal edges persist.

Assurance: source-derived partial relation, with finite native evidence where named in checks. Unknown paths remain unknown; no universal coverage or unique inverse is inferred. Complement claims are inner absent-behavior bounds under their stated premises; reverse relations retain all compatible antecedents in their declared family.

## cause-stamp-loss

Subject: world-narr, source-bound revision. Domain and observation: the finite typed state/transition family in forward, with dependencies fixed. Operator: causal_stamp records an off-line real cause once per frame line using the first line number. register consumes pending causes at the next happened unplaced event; incompatible cause edges warn without failing.

Assurance: source-derived partial relation, with finite native evidence where named in checks. Unknown paths remain unknown; no universal coverage or unique inverse is inferred. Complement claims are inner absent-behavior bounds under their stated premises; reverse relations retain all compatible antecedents in their declared family.

## evidence

### narr bounded evidence

No production code changed: the baseline source is an exact prefix of the final file. The only source additions are native wave9_models test bodies. No bug was inferred from internal partial mutation: World::apply supplies the actual rollback boundary. No runtime repair was warranted.

The file root declares ten typed views and four scoped fragments with three views each: 22 typed views per file, 44 across this pair. Fragments do not claim all branch coverage. Their byte scopes and partial syntax bind the final source.

#### Executed checks

Command family: `cargo test -p world --lib [--release] wave9_models -- --test-threads=1`. Eleven tests pass twice in debug and twice in release, with identical normalized test identities/results. Five story tests and six narr tests run in every profile. The final restored debug run also passes all eleven after both compiled mutation controls. Existing unrelated compiler warnings remain.

The independent three-node Floyd semiring oracle exhausts 729 absent/weak/strict directed-edge codes in fixed insertion order. It checks acceptance/refusal, reach, closure, alive witness continuity, removal of all edges sourced at node 1, and strict/weak self-edge boundaries. It is finite evidence, not a proof for unrestricted graph size. The eight acyclic three-edge inputs produce five compatible strict-reach preimages (masks 3,4,5,6,7).

Narrative relation checks enumerate 36 valid endpoint assignments over 0..2 for each of Before, After, During and Same: 144 comparisons of actual inserted inequalities against independent interval predicates. A separate second-edge During contradiction checks exact graph restoration.

Controlled typed-world checks exercise public frame nesting/refusal/resumption, isolated new-world identity, claim/knowledge canonical references, future-knowledge refusal, past projection erasure, and line splicing with pending-cause dedup/consumption. The reveal rollback fixture first permits a cause edge and then rejects contradictory temporal placement: public World::apply preserves the original graph/reveal/event observations, while direct internal apply_root retains its earlier cause edge. This is a tested boundary distinction, not a proposed public semantic change.

#### Compiled mutation controls

- story: change strict-cycle guard `s || strict` to `s && strict`. `story::wave9_models::finite_time_oracle_and_kill` fails assertion at code 11 (accepted=true versus oracle=false).
- narr: replace `st.time = saved` in relate with discarding saved. `narr::wave9_models::relation_endpoint_oracle_and_atomic_pair` fails with an extra first inequality after second-edge refusal.

Each mutant compiled successfully, failed a named assertion, and was restored byte for byte before the final positive run. Compile failures are not counted as controls.

#### Actual joins and limits

Source inspection: World::apply -> position -> clone -> apply_root -> step -> exec_v3 or frame apply_local -> register -> TimeGraph. Time commands call relate; reveal does cause insertion before relation, requiring the outer clone for whole-command rollback. Past closing calls merge_from -> creation_order/recreate -> local exec/check_links with per-command rollback and warning accumulation. qa3::knows reads known_since (caller name/line may change independently); vmm uses world_frame for current frame world. These call paths are not all exhaustively executed.

Valid parent trees and sufficient u8 levels are premises. Public mutable frame/graph fields can construct invalid cycles or indexes; those malformed-state APIs have no universal safety guarantee. creation_order's 64-pass fallback is bounded but not an unrestricted topological-order guarantee. No full merge branch coverage, live parser, corpus, filesystem persistence, teacher, GPU, throughput or approximation-error claim.

#### Structural evidence

Two fresh syntax exports per source are byte-identical. groupants-final.json reports valid=true and no diagnostics using an external registry that wraps the unchanged baseline tree plus these two leaves. The external validation parent intentionally has no repository sidecar (missing_group_sidecars=1); the integration agent owns permanent ancestor/registry updates. changed.json reports only story.rs and narr.rs changed, missing=[]. Structural validity does not verify semantics.

## History

### 2026-10-30 · document

Partial source-bound model, 2026-10-30. The reference is the exact sibling Rust source. The domain is valid finite typed values reachable through checked World::apply, except explicitly constructed graph/helper fixtures. Public mutable graph/frame fields are not defended against arbitrary corruption. No live parser, teacher, corpus, GPU or persistence is exercised.

### 2026-10-30 · evidence

2026-10-30. Source SHA-256: `bb5c39260b0854bfae2cfe3f6140f8ddb65b356c3da36356c2d55b2ae4302b80`.

Raw evidence resides under `$MOVA_DATA/runs/octopus-wave9-story-2026-10-30/`. SHA-256 bindings:

| Artifact | SHA-256 |
|---|---|
| debug1.log | `4a5ae0ad9a645189574c8869249014b60dc10d548b09c748c171e63c285646ed` |
| debug2.log | `597dd5322b098415c0efced4592ca4edcab74d20e87536f4b064337b07dab685` |
| release1.log | `8632f33f1a6ce3bba5ce5300d91340f1f15a27e21de3484239445ec2d8b68127` |
| release2.log | `18b654d2ce3e5e7ddbe7fcfee9cbd18d8927b4e0c427b92e25eaee76c1034b08` |
| mutant-story.log | `bfddeae7d2c6b5c264c0342c8a8f2f82f31138dbe48ba432b83c01594e1acd94` |
| mutant-narr.log | `5c604a72f84ca9dcff5c067452eb65e3e64bbf862e453e7256f896e9d0e81829` |
| restored.log | `312856721b3639774d3714626c18e70fe19b64dbc6e7018377f4069f631622b6` |
| normalized.txt | `a126d1c58ee365428170a357360ef3abb9a7b07643cc8193d07fd8c2d758cf30` |
| validation.log | `d220d39db7a8b4decc9876fb43eeec73cc1b1f36e4dddb5f852345a2a26e14ec` |
| story-final-a.json | `4b82b3f4c0f05eb308cc29fb0e825df8d432d04ec8612f942017354248e01227` |
| story-final-b.json | `4b82b3f4c0f05eb308cc29fb0e825df8d432d04ec8612f942017354248e01227` |
| narr-final-a.json | `81c6f6c662760b8e5dd029575cd09852464ca0ac96b5e1623b80d9eedd88e26c` |
| narr-final-b.json | `81c6f6c662760b8e5dd029575cd09852464ca0ac96b5e1623b80d9eedd88e26c` |
| groupants-final.json | `af066b29359f1aef6a5aac6880ebba017f8e63392dab4d67beaef9c7de6016a8` |
| changed.json | `533bae8ae43062891c32162ebb2dcb9756d7d138199b70a4373087f7070a5ac0` |
