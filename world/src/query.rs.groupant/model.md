# world-query

## descriptant

# world-query

## Forward

Immutable queries project snapshots, histories, labels, resolution and attention/context. state_at/holder_at delegate replay to Entity. qa.rs calls context_at/where_at/holder_at; md.rs renders context. Text-to-frame extraction is not performed here.

## Invariant

Query methods borrow immutable worlds; the frame reader mutates caller-owned discourse. Reverse models preserve compatible hidden inputs, never assume unique recovery. Ordering and failure assumptions in catalogs are part of the contract.

## Cost

For D replayed deltas, M mentions, E events, C cursors, K characters and L output bytes: state/history O(D+L), attention O(M+K log K), context adds O(E+C) and O(E) references. diff adds sorting and quadratic vector membership for set deltas. Resolve replays relevant entities and compares words/alternatives. Label recursion/ancestry cost depends on structure; cycles excluded.

## Composition

Rendering/resolution, history/location and attention/context fragments share immutable World. where_at->ver_at->where_ver->snapshot/label/place_chain; context_at->attention plus independent event/cursor projections. Current labels/ancestry are not historical snapshots. Inference composition must include intervening world construction; frames are not direct query inputs.

## AST gap

Source-bound syn-2 partial syntax is present. Macro expansion, name resolution, type inference, cfg evaluation, external-module loading and semantic AST remain absent. Child envelopes may overlap and need not cover the file.

## Epoch memory

## catalog

# Typed models: world-query

Observations are single terminating calls unless a recording trace is named. Preconditions: finite well-formed inputs, stable dependencies, no allocation failure or integer overflow.

## nonbehavior

Type: behavior_complement. Operator/domain/observation: U is (finite well-formed W, query q, typed return y) for terminating immutable calls; R is y=Q(W,q). Inner bound of U minus R: state_at returning a version absent from the entity, attention containing zero-count/non-character IDs, diff returning Some for zero/out-of-range endpoints.

Approximation: source-derived conditional relation; unknown behavior outside its declared domain remains unknown. Assumptions: shared preconditions above and descriptant dependency boundaries. Evidence: inspected source and callers; model.md#tests distinguishes executed checks. Falsifiable oracle: construct admissible inputs, execute the named operation and compare this predicate; contrary output refutes it.

## antecedents

Type: reverse_reconstruction. Operator/domain/observation: Q^-1(y)={(W,q):Q(W,q)=y}; retain every compatible unobserved state, mention and event. Strings lose provenance; bounded-world enumeration is a proposed oracle, not unique recovery.

Approximation: source-derived conditional relation; unknown behavior outside its declared domain remains unknown. Assumptions: shared preconditions above and descriptant dependency boundaries. Evidence: inspected source and callers; model.md#tests distinguishes executed checks. Falsifiable oracle: construct admissible inputs, execute the named operation and compare this predicate; contrary output refutes it.

## label-priority

Type: priority_partition. Operator/domain/observation: For current snapshot label chooses Name then Class then Type among Text/Words/Tag values. Events prefer speech acts to verb; relations recursively label A/B. Missing identity returns bare ID. Cyclic relation references lie outside terminating domain.

Approximation: source-derived conditional relation; unknown behavior outside its declared domain remains unknown. Assumptions: shared preconditions above and descriptant dependency boundaries. Evidence: inspected source and callers; model.md#tests distinguishes executed checks. Falsifiable oracle: construct admissible inputs, execute the named operation and compare this predicate; contrary output refutes it.

## historical-state

Type: temporal_preimage. Operator/domain/observation: state_at(id,s)=Some(v,S) requires v be last stored state with pos.sent<=s and S replay first v states. Later states and overwritten deltas remain compatible; selection is storage order, not timestamp sorting.

Approximation: source-derived conditional relation; unknown behavior outside its declared domain remains unknown. Assumptions: shared preconditions above and descriptant dependency boundaries. Evidence: inspected source and callers; model.md#tests distinguishes executed checks. Falsifiable oracle: construct admissible inputs, execute the named operation and compare this predicate; contrary output refutes it.

## field-history

Type: event_projection. Operator/domain/observation: history concatenates every Delta::Set of requested field in state/delta order, including repeated sets in one state. Non-Set deltas and other fields are erased; missing entity yields empty.

Approximation: source-derived conditional relation; unknown behavior outside its declared domain remains unknown. Assumptions: shared preconditions above and descriptant dependency boundaries. Evidence: inspected source and callers; model.md#tests distinguishes executed checks. Falsifiable oracle: construct admissible inputs, execute the named operation and compare this predicate; contrary output refutes it.

## location-anchor

Type: temporal_coupling. Operator/domain/observation: where_ver reads At and last Set at v, but current labels/ancestry. A character holder is located at target state v sentence, not original query cutoff; missing state or missing Id-valued At yields None.

Approximation: source-derived conditional relation; unknown behavior outside its declared domain remains unknown. Assumptions: shared preconditions above and descriptant dependency boundaries. Evidence: inspected source and callers; model.md#tests distinguishes executed checks. Falsifiable oracle: construct admissible inputs, execute the named operation and compare this predicate; contrary output refutes it.

## holder-fiber

Type: reverse_reconstruction. Operator/domain/observation: holder_at(o,s)=Some(h,v,st) requires last admitted version v, snapshot At=h and st=last Set(At) up to v. Unrelated intervening deltas vary freely; h does not determine a transfer event.

Approximation: source-derived conditional relation; unknown behavior outside its declared domain remains unknown. Assumptions: shared preconditions above and descriptant dependency boundaries. Evidence: inspected source and callers; model.md#tests distinguishes executed checks. Falsifiable oracle: construct admissible inputs, execute the named operation and compare this predicate; contrary output refutes it.

## diff-direction

Type: orientation_relation. Operator/domain/observation: Valid diff a to b directs field and set changes; provenance steps always use min(a,b)<ver<=max(a,b). Reversing endpoints reverses add/remove but retains chronology interval.

Approximation: source-derived conditional relation; unknown behavior outside its declared domain remains unknown. Assumptions: shared preconditions above and descriptant dependency boundaries. Evidence: inspected source and callers; model.md#tests distinguishes executed checks. Falsifiable oracle: construct admissible inputs, execute the named operation and compare this predicate; contrary output refutes it.

## resolver-fiber

Type: lexical_preimage. Operator/domain/observation: Parsable ID resolves iff entity/event exists; otherwise char/obj/loc searches current Class/Name/Type words. Matching lowercases and allows one trailing s on either side; alternatives not trimmed; each entity emitted once.

Approximation: source-derived conditional relation; unknown behavior outside its declared domain remains unknown. Assumptions: shared preconditions above and descriptant dependency boundaries. Evidence: inspected source and callers; model.md#tests distinguishes executed checks. Falsifiable oracle: construct admissible inputs, execute the named operation and compare this predicate; contrary output refutes it.

## attention-order

Type: order_statistic. Operator/domain/observation: c_i(s)=character mention count at sentences<=s. attention keeps c_i>0 and sorts descending count then ascending Id. Same-sentence repeated mentions count independently.

Approximation: source-derived conditional relation; unknown behavior outside its declared domain remains unknown. Assumptions: shared preconditions above and descriptant dependency boundaries. Evidence: inspected source and callers; model.md#tests distinguishes executed checks. Falsifiable oracle: construct admissible inputs, execute the named operation and compare this predicate; contrary output refutes it.

## context-projections

Type: selection_relation. Operator/domain/observation: main iff 2*c>=max count. what=last stored admitted event; why=last with why; speaker/addressee=last non-narrator speech even if missing Agent/To; cursor=last stored admitted entry. Assume no overflow.

Approximation: source-derived conditional relation; unknown behavior outside its declared domain remains unknown. Assumptions: shared preconditions above and descriptant dependency boundaries. Evidence: inspected source and callers; model.md#tests distinguishes executed checks. Falsifiable oracle: construct admissible inputs, execute the named operation and compare this predicate; contrary output refutes it.

## roles-observation

Type: information_loss. Operator/domain/observation: roles_str preserves roles vector order and SRef version, but uses current labels stripped of leading ID. Empty roles yield narrator only for narrator speech, otherwise em dash. Equal rendered strings need not identify equal events.

Approximation: source-derived conditional relation; unknown behavior outside its declared domain remains unknown. Assumptions: shared preconditions above and descriptant dependency boundaries. Evidence: inspected source and callers; model.md#tests distinguishes executed checks. Falsifiable oracle: construct admissible inputs, execute the named operation and compare this predicate; contrary output refutes it.


## tests

### Checks

The current native fixtures are inline in query.rs under cfg(test). The original
finite-tests.rs.txt is retained as the owner-epoch evidence snapshot.
Run cargo test -p world --lib --offline query_finite, and repeat with --release.
Four native tests compare 397 input rows to independent finite oracles and
compare full output-to-input maps: 64 label triples, 77 label/query pairs,
64 repeated-delta words, 192 count/cutoff assignments.
The model.md#finite-models domains and model.md#finite-evidence distinguish these checks
from the still-prospective general catalog claims.

Negative control: in an isolated checkout replace the production expression
x.1 * 2 >= top with x.1 * 2 > top, compile, and run
query_finite_attention_context_fibers. Require assertion failure, then restore
the source. Compilation failure does not satisfy the control.
No live teacher, API, bus, GPU, external parser, whole-world clippy or complete
school validation is claimed. Integration also repeats twelve neighboring exec/index tests and the new state/span tests; the full world suite remains outside this check.

## evidence

### Query evidence

Baseline: 4279da541. Runtime sources are unchanged. Root world-query has 12 typed models; render, history and context fragments have four each. All nodes begin with behavior_complement and reverse_reconstruction. The six selected-method envelopes across both files bind actual source-bound syntax node IDs; they are not complete semantic coverage.

Read world/src/query.rs, Entity replay/version/last-set methods in world/src/world.rs, qa.rs query dispatch and md.rs callers. Storage-order temporal selection, current-state labels and the holder-at-object-state-sentence behavior are explicit model assumptions. The latter is a potential historical-query defect: a holder moving after the object's last state can render an earlier location even when the move precedes the requested cutoff. Runtime behavior was not changed; no historical consistency guarantee is claimed.

The std-only builder probe.rs.txt extracts the exact current attention function and frames known/cover_line functions into bounded harnesses. Dependency stubs provide entity mentions and lexical membership; these are function-body checks, not complete World or real lexicon/parser integration. Run from the repository root:

```sh
rustc --edition=2024 --crate-name query_probe world/src/query.rs.groupant/probe.rs.txt -o "$MOVA_DATA/query-probe"
"$MOVA_DATA/query-probe" "$MOVA_DATA/query-probe-run"
```

Executed twice with separate external directories; summary bytes identical. Attention passes 256 count assignments, cutoff exclusion and non-character exclusion. Independent bucket enumeration checks count/tie ordering; reversed count-order mutation is rejected. Coverage passes 64 frequency maps, three exact report preimages and three literal-space head boundaries; replacing token weighting with type weighting is rejected. The initial harness had a missing stub delimiter; fixed before successful runs. No production code fix was needed.

Both partial syntax projections regenerate byte-identically. Unchanged-registry groupants validation passes twice with identical reports: 20 file roots, 6 registered groups, 512 missing file sidecars, no diagnostics. These totals describe the existing checkout plus these two new roots, not complete repository coverage. changed against baseline reports changed=[] and missing=[] because no Rust source changed; it does not validate catalog semantics. registry-node.json is a proposed integration descriptor; this branch does not change the registry.

Full Cargo tests, complete query/world correspondence, live parser, courses, teacher, GPU and semantic AST checks were not run. Existing world/src/tests.rs tests remain useful follow-up oracles, not evidence claimed as executed here.

## finite-evidence

### Owner-snapshot native evidence

The four query_finite tests pass in two debug and two release executions,
each bounded by 600 seconds, offline, with one test thread and an isolated
target directory. They cover 397 enumerated native fixture rows and compare
complete finite output fibers. Three extra missing-identity/empty-context
conditions are assertions within those tests, not separately counted cases.
There are 162 existing world tests outside the filter, which were not executed.

#### Negative control

A real compiled production mutation changes context_at's half-threshold
comparison from >= to >. The native test fails an equality assertion at
input 5: effective counts [2,1,1] should put all three characters in main,
while the mutant moves the two count-one characters to secondary.
The log contains successful compilation followed by one failed test.
Source was restored before both second-profile checks. No compile-error
control is counted and no mutant remains committed.

#### Structural checks

The full registry was copied to an external candidate and its world-query
node replaced with the adjacent registry-node.json. The changed gate passes
for query.rs with no missing model. The groupants report has exactly two
diagnostics: stale query.rs bindings in models/inference-core.groupant and
models/mova.groupant. Leaf/fragment declarations have no diagnostics.
Parent binding refresh belongs to integration; no shared registry or parent
groupants were changed in this branch. An earlier one-node registry probe
was unsuitable for this repository-wide scanner and is not counted as passing.

Four specialized root models were added: 12 -> 16. The three fragments retain
four models each, so the complete local family is 24 -> 28 across four nodes.
Required behavior_complement and reverse_reconstruction remain first/second.
Finite observations are conditional; no full-world verification is inferred.

#### Limits and retained observations

Runtime query methods were not changed. The earlier extracted/stub probe
remains historical evidence. Command validation restricts relation endpoints
to characters; forged relation cycles remain outside the terminating domain.
No live teacher, API, bus, GPU, full-world clippy or school run was performed.
The tests construct query states directly and do not establish parser or
command-validation correctness.

## finite-models

# Finite native query relations

For each domain X below, the oracle O is specified independently of the production
query Q. The native checks assert Q(x)=O(x) for every x, then compare the entire
maps y -> ordered input indices. This retains every compatible input, including
collisions, rather than recovering a selected antecedent. Unreachable strings
have empty preimages. Finite evidence supports these exact domains only.
For U=X times String, the complement is {(x,y):y != O(x)}. Thus the finite
relation supplies both required baseline types; rejection alone is not the model.

## label-value-priority-fiber

Type: priority_preimage. Hold identity C1 and one state fixed. X is all 64
triples of Name/Class/Type values chosen from Unknown, Text("fox"),
Words(["red","fox"]), and Id(L1). The first eligible Text/Words slot gives
"C1 fox" or "C1 red,fox"; no eligible slot gives "C1". Id-valued fields
do not shadow a later text-compatible field. Full inverse is every triple
with that first eligible observation. Invariant: fixture identity survives.
Oracle: literal output table indexed by the first digit in {1,2}.
Cost: three slot checks plus output bytes; fixture snapshot replay is bounded.
Falsifier: query_finite_label_priority_fibers. Missing-identity label also checked.
No events, relations, tags, arbitrary Unicode or cyclic states are quantified.

## lexical-membership-fiber

Type: lexical_preimage. Hold C1, one Name Text field and an otherwise empty
world fixed. X consists of seven labels crossed with eleven queries (77 pairs).
The exact labels, queries and literal membership masks are in the native test;
they distinguish case folding, hyphen tokenization, underscore preservation,
single-s tolerance, untrimmed alternatives, register mismatch and direct IDs.
The output is either [] or [C1]; the inverse contains all pairs whose mask bit
matches the observed output. Invariant: no duplicate identity.
Cost: bounded label/query bytes and one entity snapshot.
Falsifier: query_finite_resolver_fibers against a literal matrix, without
reimplementing the production tokenizer or plural predicate.
No semantic stemming or natural-language accuracy is claimed.

## repeated-delta-history-fiber

Type: trace_projection_preimage. Hold one C1 state at version 1 fixed.
X is all 64 length-three delta words over Set(Name,"fox"), Set(Name,Unknown),
Set(Type,"animal"), Set(Class,Unknown). O erases the latter two symbols
and maps each remaining symbol to (1,value), preserving repetitions/order.
The inverse retains every erased-symbol placement and choice; a final
snapshot alone cannot reconstruct this history. Empty and repeated traces
are reachable. Invariant: every surviving item keeps its version and value.
Cost: three delta inspections and up to three output items.
Falsifier: query_finite_history_projection_fibers; missing identity is checked.
Multiple states, Add/Del and historical location strings remain outside this domain.

## attention-threshold-fiber

Type: rank_threshold_preimage. Hold three characters and one object fixed.
X is [0,3]^3 times cutoffs {0,2,4} (192 inputs). Character i has n_i mentions
at sentence 2 plus one at sentence 4; the object has nine mentions at 0.
Effective counts are zero, n_i, or n_i+1 at the three cutoffs. O emits positive
counts by descending count buckets then ascending ID. For maximum count
0/1/2/3/4, the main cutoff table is 0/1/1/2/2. The observation is the joint
(attention,main,secondary) triple; inverse fibers preserve cutoff/count
ambiguity. Empty event/cursor projections must remain None.
Invariant: all positive-count characters appear exactly once in the partition;
the object is absent even when its count dominates.
Cost: bounded mentions plus four count buckets; production remains O(M+K log K).
Falsifier: query_finite_attention_context_fibers. A compiled production mutant
changing >= to > at the half threshold must fail an assertion.
No event selection, unbounded usize counts or overflow claim is made.

## Dependency and termination boundary

World command validation requires relation endpoints to be characters:
lang.rs parses rel a/b as Reg::C; world.rs Cmd::Rel calls need(...,[Reg::C])
for creation and updates. Generic attribute names do not expose A/B.
This source inspection supports excluding relation cycles for that construction
path. Forged crate-private relation states and other construction routes are
not proved safe here; no recursion fallback or output policy is introduced.

## Epoch memory (finite-models)

## History

### 2026-10-29 · document

Revision 2026-10-29; SHA-256 27dd12e09019d22b057ab892136aec5be542da170e529573638c631b44ddd37f.

### 2026-10-29 · Epoch memory

2026-10-29: source-derived first epoch. Historical snapshots do not freeze current labels; direct read_with callers bypass the adapter. Recording errors can retain global state. Preserve these observations when strengthening models. Runtime source is unchanged.

### 2026-10-29 · document

Subject: ../query.rs; SHA-256 27dd12e09019d22b057ab892136aec5be542da170e529573638c631b44ddd37f. Revision 2026-10-29.

### 2026-10-29 · tests

The 2026-10-29 model.md#evidence retains the extracted attention/coverage probe
with stub dependencies. It is not the native evidence below.

### 2026-10-29 · evidence

Heading `### Query evidence — 2026-10-29` stays in evidence as `### Query evidence`.

Source SHA-256: b85ff6f386281082c3d600cafa8114d7a78d1436c8ddc84c0a42c9575e474f3d.

Syntax artifact SHA-256: 8de24ad5bb7630e58fb725cc1d0e6ec7b34b65290eb5dd603815bb7977e39602.

### 2026-10-29 · Epoch memory (finite-models)

2026-10-29 remains the source-derived catalog epoch and extracted/stub probe.
2026-10-30 prediction: all 397 native rows agree with the four finite oracles;
replacing inclusive half threshold with strict threshold exposes a counterexample.
The native evidence strengthens selected projections only. It supersedes neither
the earlier scoped observations nor the explicit semantic AST and integration gaps.

### 2026-10-30 · Epoch memory

2026-10-30: selected native finite epoch. Four independent oracles and complete
finite output fibers now cover 397 rows; see model.md#finite-models and
model.md#finite-evidence. Runtime methods remain unchanged. The half-threshold
strictness mutant must compile and fail an assertion. Label relation cycles
are excluded by the inspected command path's character-only endpoint gates;
forged states remain outside the terminating-domain claim. Existing source
inspection, extracted-probe evidence and unmodeled dependency gaps remain.

### 2026-10-30 · finite-evidence

Heading `### Owner-snapshot native evidence, 2026-10-30` stays in finite-evidence as `### Owner-snapshot native evidence`.

Run: $MOVA_DATA/runs/octopus-query-wave6-2026-10-30.

Base: 48f537bb250b47e571aaf2758a39c82ab6bddadd.

Source SHA-256: d11cdf447b04c6239c445ea1873c78c7174936f1149a569a242f4beb213a5ff4.

Native fixture SHA-256: 1a48bc6af1a4baf908cfc72dff2b2c9414dc372796869f2a0d2b757edda40e53.

Finite model SHA-256: f141550654c4c995e4f31129643c48d5784f7b407fcee31dab747ca937eb5023.

Normalization retains the ordered test query:: lines only, removing compiler
warnings, build timing, harness timing and target paths. Both profiles and
both repetitions yield the same four passing names. The common normalized
SHA-256 is efb730feee45a067984b316431a0582cd9fe264f4c81ae338bc0af03029bf1b7.
This is a deterministic correctness observation, not a throughput measurement.

Partial syntax and all three fragment source scopes were refreshed twice.
Both syntax artifacts have SHA-256
eae81d60dd919d8e42db35146f2e30d89bcbfec7a6925eeded71fd3f169ca524.
The new cfg(test) include is an opaque macro in this projection; its fixture
bytes are separately bound above. There is no expanded semantic AST.

#### Raw artifact hashes

| Artifact | SHA-256 |
| --- | --- |
| debug-1.log | a8b751cc4801ceacbd672f33e05c620f2664608afa193c91f1e0345232590b4c |
| debug-2.log | 7b489500c89c2cd67b2c1e8c0c27c69f4241e7fdd4cccc00971565cc11ff1446 |
| release-1.log | 8bb26df13d682683cb04327a32cf6bc68f29580e0a10fb8a2bc8cd353fc197df |
| release-2.log | e050b5f6a64d259baebc22ceffaae8d0cec352713681c622f3593a2567eb37bb |
| mutant.log | ccdc512258efab4d5fbecb65e81bc9beeac203a23f194d58250b432bc4e4bb44 |
| groupants-full.json | 9b09c2103b913d63984ab4a8c1e2e219bc167031967d1c8fd5a61b6ad201825e |
| changed.json | b686eabc6d56bc2a6e3ec6f16e52974260af46b9739f8933bfede0c1e8ea2259 |

#### Integrated source binding

The owner snapshot above used an include macro. Integration moved the identical test body inline into query.rs so the source hash and syntax projection also bind test contents. The archived fixture remains historical evidence. Final source SHA is 27dd12e09019d22b057ab892136aec5be542da170e529573638c631b44ddd37f. The new syntax projection was generated twice identically; existing production fragment scopes were preserved. All four native tests pass twice in each profile after relocation, with the same names and outcomes. Current evidence: $MOVA_DATA/runs/octopus-models-wave6-2026-10-30/final-tests/.

### 2026-10-30 · document

Revision 2026-10-30. Subject: query.rs with finite-tests.rs.txt included only under cfg(test).
These are conditional exact finite relations, not whole-world verification. Fixtures use
real World, Entity, State and query methods; direct construction bypasses command gates.
The per-file catalog's broad claims remain partial. No runtime policy is changed.
