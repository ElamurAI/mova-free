# syntax-validation

## descriptant

# syntax-validation

## epoch-memory

Wave25. Current callers share the root-termination observation while retaining different implementations: annotation walk/BFS and conversion walk/color. Preserve the earlier transfer epoch as historical evidence; do not infer new timing from old hybrid measurements.

## model

# syntax-validation

## forward

Tagged gate family retains each caller's ordered preflight and diagnostics. Annotation keeps original walks for n<=64 and a 4n walk budget before reverse-head BFS for larger inputs. Conversion keeps original walks for n<=64; larger inputs use functional-graph colors: root is completed (2), unseen nodes become active (1), a path reaching completed nodes is recolored 2, and revisiting active nodes reports the current ordered start. Earlier completed paths leave no active nodes, so the least nonterminating start is preserved, including tails. Reverse-head reachability is the shared mathematical observation, not a runtime BFS dependency of conversion. Local differential tests and restricted cross-gate joins remain separate evidence.

## invariant

For each gate and admitted call, the complete Result is the baseline Result: same acceptance, first error kind, word index and text. Every head entering cycle detection is in range. A token reaches virtual root iff reversed-head reachability marks it; the reported failed token is the minimum unmarked token, which can be a tail entering a cycle. Annotation Count/Form/Head/Registry/Roots precedence and converter parse/range/root-label/root-count precedence remain distinct.

## cost

Let n be token count after preflight. Both small branches use O(n^2) walk work with n<=64 and O(1) cycle-phase auxiliary storage. Large annotation calls use at most 4n walk steps then optional O(n) reversed adjacency/BFS. Large conversion calls use at most 2n head transitions and O(n) color storage, allocated on every n>64 cycle-phase entry; no reverse rows or queue. Conversion separately retains its heads Vec. Diagnostic strings add error-path allocations. Parsing, normalization and conversion edits have their own costs; no whole-module linearity claim follows.

## composition

The production joins are parse/normalize -> annotation gate and selected conversion edits -> converter gate. There is no production call from one gate to the other. Independent harness joins share only a finite head vector with lawful token fields, matching root conventions and matching admitted local constraints; outputs are projected to acceptance or first root-unreachable token. Source-local baseline equality tests and this restricted cross-gate observation are different relations. Algorithm library ownership remains unchanged.

## checks

["Annotation baseline checks and converter legacy/transfer-parent/coloring checks","Small and large thresholds, completed suffix, least failed start and preflight precedence","Independent common-domain gate joins and distinct boundary policies","Current source-local evidence; historical fd540 receipts kept separately"]

## limitations

Claims concern the cycle phase after unchanged preflight. Annotation's walk/BFS policy and conversion's walk/color policy must match the merged sources. Arbitrary edit semantics, normalization, linguistic accuracy, I/O, panics and allocation failures remain outside normal-return guarantees. Current evidence is linked separately from the historical fd540 transfer epoch.

## wave26-parser-and-graph-composition

CoNLL-U bytes -> Doc lexical scan -> per-sentence refresh -> selected conversion edits -> tree_gate is the actual representation/gate boundary. A parse success alone does not imply a tree: raw malformed heads can project to zero while the gate rejects them. On refresh error, prior sent/words remain together while caller raw edits remain. The permissive training reader resets multiword-range state at every blank separator. After gate preflight, successor[0]=0 and successor[i]=head[i] give acceptance iff every token has cycle_id[i]=cycle_id[0]; the first error is the least token outside that basin, including tails. This independent test adapter preserves one-based token coordinates and does not replace the production gate.

Refresh stages an O(R) row map before replacing both derived views. This trades transient storage for consistent failure behavior. Parsing cost is O(B) in input bytes on normal-return inputs.

## wave27-english-lexical

Tag and ptag call dict/hash/gram; ptag uses TnT prior only unknown words. Morph's UD/AGID/rule priority interacts with lexgen src flags and record order: changing a/r/u alters preferred analyses. tok spacing bits are not exact whitespace; consumers cannot infer original byte offsets from detokenize. text order is 1-based while array positions and dictionary indexes are 0-based. udreg validates selected enum features, so upstream Feats::parse unknown-dropping cannot become a complete annotation gate. Existing conllu callers supply Sentence/Token to learning/generation.

## wave27-english-structure

Heads use root0 and1-based word coordinates; Graph truncates heads to u16, while parser/UD use usize. Ud children returns0-based child indices. GNode lemma references global dictionary or local graph heap; Store fingerprint binds global IDs but not arbitrary target serde schema. annotate repairs tree whereas kbest/complete/explain/Model.encode do not; treating them as identical is invalid. Expert consumes conllu Sentence, not Graph directly; callers must preserve tags/features/relations when adapting. Construct callbacks must return enough dependent entries and valid renderable heads.

## wave27-annotation-orchestration

ann.rs is an existing cross-group gate/type dependency, not duplicate ownership. main and batch share Outcome::add_patch_reply/finish; llm direct fixed+variable concatenation differs by two separating newlines from codex message-block join. hints advise only; repair precedes draft gates; optional post-gate FEATS replacement is not re-gated.

## wave28-formula-syntax

Generated enum order must match canonical arrays. Stack reversal preserves expansion order. Printer relax adapters prevent unknown swallowing groups and aligned swallowing brackets; optional rbrack and trailing-row compensation preserve parser structure. Span quotient cannot validate original coordinates.

## catalog

# Typed transfer relations

Source-derived current relations; current evidence is linked separately from retained historical transfer receipts.

## excluded-gate-results

Type: behavior_complement.

In U of tagged lawful calls and Result observations, the candidate inner complement excludes acceptance of a preflight-valid root-disconnected head component, reporting a later unrooted token when an earlier tail enters the same cycle, and a cycle diagnostic masking an earlier Count/Form/Head/Registry or converter root-label error. It also excludes claiming both APIs accept the empty vector: converter accepts it while annotation reports Roots(0). These exclusions describe the relation; negative fixtures only sample it.

## compatible-records

Type: reverse_reconstruction.

For tagged result y, retain every record sequence, root/head vector, registry state and conversion history satisfying that gate's ordered relation. Cycle(k) constrains earlier eligible tokens to terminate and k not to terminate after preflight; it does not identify the cycle vertices or discarded fields. Successful gate observations forget lexical values and edit history. Cross-gate projection further merges different error vocabularies; no unique reconstruction is claimed.

## minimum-unreachable

Type: first_error_projection.

The least root-unreachable token is the mathematical observation. Annotation may compute it from reversed-head BFS after its walk budget; conversion's ordered coloring reports the least nonterminating start directly. For heads [3,0,4,3], token 1 is the failed start although cycle members are 3 and 4. No queue order or cycle-member minimum substitutes for this observation.

## intersection-only

Type: boundary_non_equivalence.

The mutual domain excludes empty input and self-heads: empty converter success differs from annotation Roots(0); a self-head is a converter cycle but annotation rejects Head first. Annotation count/form validation has no universal converter equivalent. The common-domain harness proves no replacement of one complete API by the other.

## walk-budget

Type: resource_fallback.

Both callers retain original walks for n<=64. Large annotation calls spend at most 4n walk steps before optional reverse BFS. Large conversion calls allocate n+1 colors and follow each head at most twice; there is no work-budget fallback or BFS allocation. These policies preserve tagged Result semantics but differ in allocation behavior. Failed allocation remains outside normal-return guarantees.

## local-oracle-join

Type: test_observation.

Each checks.rs calls its own real gate and retained baseline; the independent harness calls both real gates plus a root-reachability observation on compatible fixtures. Passing a finite oracle constrains exact outputs on those rows only. Tests sharing an included implementation are not independent evidence for all conversion and parsing semantics.

## phase-boundary

Type: pipeline_obligation.

Annotation normalization and converter edit application must deliver the same validated head indexing assumed by their gate. Gate correctness alone does not prove the producer's normalization or action semantics. Reused root reachability is applied only after these local obligations; acquisition and other legacy operations keep explicit coverage gaps.

## wave27-english-lexical-dictionary-flags

lexgen UD rows precede AGID; a/ar/r/b flags become dict UD/AGID/REV/BASE bits via build.rs. Morph UD max-count uses later tie while AGID uses first main match; preserving rows and flags is required.

## wave27-english-lexical-coordinate-transport

gram Sym high bit distinguishes graph-local heap from frozen global dictionary. text detok and Morph tree heads use1-based token coordinates; dict record ids and arrays use0-based coordinates; blind interchange invalid.

## wave27-english-lexical-candidate-support

TnT emits only seen positive candidate classes, PTagger masks unseen unless forced or known shortcut. Empty training supports empty tagging but nonempty unconstrained inference is not total; parents cannot assume length-preserving success on unsupported models.

## wave27-english-lexical-annotation-projection

Feats parses known pairs to bits and counts unknowns separately; udreg sees only bits. Reconstructing original annotation or validating dropped pairs requires caller to preserve unknown count/raw source.

## wave27-english-structure-head-coordinate-transport

Parser tree index i is token i+1; annotation heads stay usize; GNode head=(parser head as u16); Ud children maps head h to slot h-1. Validity requires heads<=n and graph n<=65535 for lossless coordinate transport.

## wave27-english-structure-annotation-branch-join

repair is between parse and child features only in annotate. complete and kbest recompute child-derived UPOS/FEATS without that transformation; equal raw parser candidates need not imply equal final annotations.

## wave27-english-structure-model-wire-boundary

Annotator save packs before directory/write/rename. Unpack gate order magic/size->version->enum sizes->dictionary->length->checksum->typed body. Accepted envelope does not certify caller-selected target type.

## wave27-english-structure-observability-cost

Explain requests parser trace then independently Model.encode, so semantic presentation includes repeated inference and trace stack copies. Parent runtime is sum of real calls, not max child cost or assumed shared cache.

## wave27-annotation-orchestration-representation-transport

Request chunks index State items and matching drafts; this unchecked alignment is needed before reply custom_id strings can map through main patch gates.

## wave27-annotation-orchestration-gate-order

Model validation before queue effects, snapshot/partial validation before resume calls, and missing/broken/tree gates before accepted patch output. Later finish failures can leave earlier writes.

## wave27-annotation-orchestration-effect-boundary

Fake tests exercise local subprocess/filesystem boundaries with temporary files, not live teachers. finish optional UDPipe/corpus/training paths are unexecuted.

## wave27-annotation-orchestration-cost-join

Source byte and gate costs add to partial lookup P*R and external effects; compatibility USD zero for Codex is not subscription billing or zero compute.

## wave28-formula-syntax-table-transport

Ordered build rows induce discriminants/arrays and lookup Entry joins parser dispatch.

## wave28-formula-syntax-marker-discipline

End markers delimit parser substreams. Stream group_tokens/opt_tokens and macros collect_until explicitly restore End on their failure paths. Macro match_pattern literal matching consumes the next token before comparison and can consume End on mismatch; read_let can likewise consume an invalid End target. No universal scan-failure preservation law is claimed.

## wave28-formula-syntax-canonical-adapters

relax, rbrack and row compensation preserve parsing boundaries.

## wave28-formula-syntax-resource-guards

Depth and expansion thresholds are local; pre-expansion token check does not bound peak substituted vector.

## evidence

# Historical epoch through fd540 — syntax-validation

## Retained historical document

# syntax-validation evidence

## checks (evidence)

See models/mova.groupant/model.md#transfer-evidence, transfer-benchmark.md and transfer-validation.md for the combined receipts. Source revisions are bound by this manifest and paired partial syntax. Finite checks do not establish universal equivalence, floating-point optimality, parser quality or whole-system speed.


## Current wave25 checks

Conversion coloring passed 19 focused checks, and the two cross-gate observations passed, twice in debug and twice in release. The five unchanged annotation-local checks belong to their retained fd540 epoch and are not newly counted here. The compiled first-error mutant failed and was restored. All 126126 source-local head vectors, completed-path/shared-suffix and ordered cycle witnesses are retained. Current timing is in ../../en/src/convert.rs.groupant/model.md#wave25-performance; old BFS results above remain historical.

## Wave26 retained checks

Conversion 19, CoNLL-U four, functional graph joins five and annotation joins two passed twice debug/release. The CoNLL-U regression tests fail the old source and pass the repaired source; two compiled repair mutants confirm the assertions. The graph harness exhausts 3,413 valid-root-label head maps plus representation/boundary fixtures. Its shifted-head adapter mutant fails. No corpus was used.

## wave26-rejected-performance

### In-place certification timing

The candidate is a mixed resource/tiny-chain refinement, not a successful general star-latency repair. Against the current wave25 parent, n32 chain improves 2.306–2.436x and n64 chain 4.510–6.035x. However n128/512/2048 chain ratios are 0.939–0.968x (about 3–6.5% more time), retaining their linear gain versus historical quadratic walks but slightly regressing versus coloring. Star recovery was a failed prediction: n512 star is 0.940–0.941x, n64 star 0.870–0.874x and n64 middle-star 0.696–0.904x. n2048 star is essentially even at 0.996–1.001x. The measured removal of a color allocation does not imply a latency gain. Single/two-node results also include regressions. This evidence does not establish a representative workload win; integration rejected the candidate replacement.

Historical comparisons retain the long-chain benefit: n128/512/2048 chains are 8.07–8.97x faster than the transfer parent and 17.14–227.70x faster than legacy. Those historical gains do not erase regressions against the actual current parent. All 165 size/shape/baseline rows below are retained; no confirmation-driven retuning followed the experiment.

CPU Intel Core i5-13500, x86_64; rustc 1.99.0 (b940084d7 2026-09-28), workspace thin-LTO release. Root explicitly granted an exclusive timing slot after all workers paused builds/tests. The prepared runner invoked the existing uninstrumented release test binary serially six times: current, transfer and legacy each twice. All invocations exited 0. Fifty-five cases per invocation cover n=0,1,2,3,8,32,64,65,128,512,2048 and chain, star, forward chain, middle-root star and binary shapes. Degenerate sizes intentionally repeat equivalent head maps. Each case has three warmup pairs then seven alternating-order measured pairs (a 4/3 ordering split), 385 pairs per invocation and 2310 total. Each measured pair has 1000 calls below n128 or 32 calls otherwise. Full Result equality is asserted before each workload, and all timed calls unwrap success. Setup, compilation, IO and allocation instrumentation are outside timed calls.

| Baseline | n | Shape | Median run 1 | Median run 2 | All-pair range |
|---|---:|---|---:|---:|---|
| current | 0 | binary | 1.107x | 1.370x | 1.106–1.375x |
| current | 0 | chain | 1.106x | 1.367x | 0.589–1.372x |
| current | 0 | forward_chain | 1.107x | 1.369x | 1.106–1.389x |
| current | 0 | middle_star | 1.106x | 1.366x | 0.990–1.375x |
| current | 0 | star | 1.109x | 1.359x | 1.107–1.367x |
| current | 1 | binary | 0.941x | 1.000x | 0.938–1.019x |
| current | 1 | chain | 0.942x | 0.996x | 0.834–1.000x |
| current | 1 | forward_chain | 0.939x | 0.997x | 0.837–1.021x |
| current | 1 | middle_star | 0.943x | 0.997x | 0.940–1.004x |
| current | 1 | star | 0.967x | 0.998x | 0.940–1.002x |
| current | 128 | binary | 0.968x | 0.960x | 0.948–0.995x |
| current | 128 | chain | 0.952x | 0.939x | 0.933–0.960x |
| current | 128 | forward_chain | 0.991x | 0.984x | 0.971–1.016x |
| current | 128 | middle_star | 0.973x | 0.997x | 0.959–1.064x |
| current | 128 | star | 0.938x | 0.947x | 0.917–0.984x |
| current | 2 | binary | 1.013x | 0.995x | 0.952–1.045x |
| current | 2 | chain | 0.927x | 0.988x | 0.907–1.100x |
| current | 2 | forward_chain | 0.908x | 1.010x | 0.854–1.034x |
| current | 2 | middle_star | 1.024x | 1.016x | 0.995–3.086x |
| current | 2 | star | 0.910x | 0.984x | 0.905–1.006x |
| current | 2048 | binary | 0.971x | 0.964x | 0.960–0.980x |
| current | 2048 | chain | 0.968x | 0.959x | 0.949–0.971x |
| current | 2048 | forward_chain | 0.977x | 0.972x | 0.963–0.979x |
| current | 2048 | middle_star | 0.965x | 0.958x | 0.935–0.972x |
| current | 2048 | star | 1.001x | 0.996x | 0.945–1.083x |
| current | 3 | binary | 1.006x | 1.000x | 0.991–1.023x |
| current | 3 | chain | 1.041x | 1.015x | 0.996–1.063x |
| current | 3 | forward_chain | 1.006x | 1.015x | 0.959–1.023x |
| current | 3 | middle_star | 1.006x | 0.998x | 0.976–1.023x |
| current | 3 | star | 1.018x | 0.964x | 0.932–1.045x |
| current | 32 | binary | 1.242x | 1.452x | 1.223–1.516x |
| current | 32 | chain | 2.436x | 2.306x | 2.239–2.540x |
| current | 32 | forward_chain | 2.467x | 2.059x | 1.982–2.485x |
| current | 32 | middle_star | 1.116x | 1.082x | 0.947–1.147x |
| current | 32 | star | 1.004x | 1.019x | 0.957–1.037x |
| current | 512 | binary | 0.970x | 0.970x | 0.948–1.001x |
| current | 512 | chain | 0.957x | 0.958x | 0.953–0.966x |
| current | 512 | forward_chain | 0.975x | 0.978x | 0.952–0.981x |
| current | 512 | middle_star | 0.970x | 0.971x | 0.936–0.974x |
| current | 512 | star | 0.941x | 0.940x | 0.929–0.946x |
| current | 64 | binary | 1.250x | 1.234x | 1.204–1.396x |
| current | 64 | chain | 6.035x | 4.510x | 4.192–6.624x |
| current | 64 | forward_chain | 5.409x | 4.857x | 4.432–5.507x |
| current | 64 | middle_star | 0.904x | 0.696x | 0.649–0.914x |
| current | 64 | star | 0.874x | 0.870x | 0.866–1.230x |
| current | 65 | binary | 0.994x | 0.985x | 0.959–1.029x |
| current | 65 | chain | 0.977x | 0.962x | 0.787–0.992x |
| current | 65 | forward_chain | 1.060x | 1.066x | 1.055–1.101x |
| current | 65 | middle_star | 0.985x | 1.024x | 0.973–1.027x |
| current | 65 | star | 0.972x | 0.980x | 0.966–0.987x |
| current | 8 | binary | 1.063x | 1.062x | 1.008–1.119x |
| current | 8 | chain | 1.161x | 1.197x | 1.137–3.657x |
| current | 8 | forward_chain | 1.159x | 1.159x | 1.087–1.361x |
| current | 8 | middle_star | 1.016x | 1.052x | 0.931–1.065x |
| current | 8 | star | 1.037x | 1.029x | 0.945–1.041x |
| legacy | 0 | binary | 0.998x | 0.999x | 0.994–1.003x |
| legacy | 0 | chain | 1.001x | 1.001x | 0.997–1.007x |
| legacy | 0 | forward_chain | 1.000x | 1.002x | 0.997–1.002x |
| legacy | 0 | middle_star | 1.002x | 0.999x | 0.996–1.005x |
| legacy | 0 | star | 0.998x | 0.998x | 0.994–0.999x |
| legacy | 1 | binary | 0.924x | 0.924x | 0.916–0.929x |
| legacy | 1 | chain | 0.925x | 0.925x | 0.919–0.929x |
| legacy | 1 | forward_chain | 0.924x | 0.926x | 0.919–0.929x |
| legacy | 1 | middle_star | 0.923x | 0.926x | 0.915–0.927x |
| legacy | 1 | star | 0.923x | 0.924x | 0.921–0.930x |
| legacy | 128 | binary | 1.455x | 1.418x | 1.391–1.485x |
| legacy | 128 | chain | 17.139x | 17.148x | 16.687–17.586x |
| legacy | 128 | forward_chain | 12.330x | 12.471x | 11.893–13.013x |
| legacy | 128 | middle_star | 0.900x | 0.922x | 0.872–0.978x |
| legacy | 128 | star | 0.893x | 0.877x | 0.855–0.980x |
| legacy | 2 | binary | 0.890x | 0.907x | 0.873–0.914x |
| legacy | 2 | chain | 0.889x | 0.899x | 0.875–0.917x |
| legacy | 2 | forward_chain | 0.880x | 0.919x | 0.860–0.944x |
| legacy | 2 | middle_star | 0.881x | 0.920x | 0.871–0.924x |
| legacy | 2 | star | 0.892x | 0.905x | 0.872–0.908x |
| legacy | 2048 | binary | 1.757x | 1.716x | 1.694–1.760x |
| legacy | 2048 | chain | 227.701x | 220.269x | 217.744–228.226x |
| legacy | 2048 | forward_chain | 170.828x | 154.586x | 152.626–171.260x |
| legacy | 2048 | middle_star | 0.916x | 0.892x | 0.881–0.973x |
| legacy | 2048 | star | 0.865x | 0.845x | 0.841–0.868x |
| legacy | 3 | binary | 0.888x | 0.884x | 0.882–0.898x |
| legacy | 3 | chain | 0.917x | 0.931x | 0.732–0.934x |
| legacy | 3 | forward_chain | 0.937x | 0.968x | 0.925–0.972x |
| legacy | 3 | middle_star | 0.864x | 0.874x | 0.861–0.876x |
| legacy | 3 | star | 0.886x | 0.886x | 0.882–0.896x |
| legacy | 32 | binary | 1.467x | 1.468x | 1.411–1.491x |
| legacy | 32 | chain | 2.590x | 2.795x | 2.445–2.917x |
| legacy | 32 | forward_chain | 2.220x | 2.216x | 2.141–2.241x |
| legacy | 32 | middle_star | 1.087x | 1.068x | 0.956–1.120x |
| legacy | 32 | star | 0.964x | 0.972x | 0.948–1.050x |
| legacy | 512 | binary | 1.387x | 1.391x | 1.382–1.395x |
| legacy | 512 | chain | 60.999x | 61.075x | 60.479–61.289x |
| legacy | 512 | forward_chain | 45.082x | 44.982x | 44.775–45.141x |
| legacy | 512 | middle_star | 0.897x | 0.894x | 0.888–0.919x |
| legacy | 512 | star | 0.873x | 0.871x | 0.842–0.891x |
| legacy | 64 | binary | 1.274x | 1.289x | 1.259–1.300x |
| legacy | 64 | chain | 5.476x | 8.170x | 4.534–8.256x |
| legacy | 64 | forward_chain | 6.606x | 6.771x | 6.151–7.239x |
| legacy | 64 | middle_star | 0.898x | 0.693x | 0.661–0.925x |
| legacy | 64 | star | 0.869x | 0.871x | 0.853–0.878x |
| legacy | 65 | binary | 1.269x | 1.287x | 1.264–1.335x |
| legacy | 65 | chain | 8.108x | 8.057x | 7.320–8.524x |
| legacy | 65 | forward_chain | 7.060x | 6.285x | 6.214–7.428x |
| legacy | 65 | middle_star | 0.884x | 0.913x | 0.875–0.927x |
| legacy | 65 | star | 0.873x | 0.873x | 0.835–0.887x |
| legacy | 8 | binary | 1.006x | 1.011x | 1.004–1.013x |
| legacy | 8 | chain | 1.192x | 1.199x | 1.123–1.200x |
| legacy | 8 | forward_chain | 1.220x | 1.221x | 1.219–1.222x |
| legacy | 8 | middle_star | 0.897x | 0.897x | 0.895–0.931x |
| legacy | 8 | star | 0.909x | 0.914x | 0.908–0.914x |
| transfer | 0 | binary | 1.105x | 1.106x | 1.100–1.111x |
| transfer | 0 | chain | 1.107x | 1.105x | 1.102–1.113x |
| transfer | 0 | forward_chain | 1.108x | 1.106x | 1.101–1.110x |
| transfer | 0 | middle_star | 1.106x | 1.106x | 1.102–1.113x |
| transfer | 0 | star | 1.106x | 1.105x | 1.100–1.110x |
| transfer | 1 | binary | 1.000x | 0.939x | 0.932–1.003x |
| transfer | 1 | chain | 0.940x | 0.940x | 0.937–0.946x |
| transfer | 1 | forward_chain | 0.997x | 0.942x | 0.935–1.008x |
| transfer | 1 | middle_star | 0.994x | 0.942x | 0.936–1.008x |
| transfer | 1 | star | 0.945x | 0.939x | 0.738–3.394x |
| transfer | 128 | binary | 6.084x | 6.228x | 5.972–6.429x |
| transfer | 128 | chain | 8.974x | 8.805x | 8.624–9.149x |
| transfer | 128 | forward_chain | 6.677x | 6.808x | 6.449–7.100x |
| transfer | 128 | middle_star | 0.983x | 0.982x | 0.947–1.013x |
| transfer | 128 | star | 0.964x | 1.002x | 0.936–1.040x |
| transfer | 2 | binary | 0.958x | 0.939x | 0.898–0.983x |
| transfer | 2 | chain | 0.972x | 0.937x | 0.903–1.010x |
| transfer | 2 | forward_chain | 0.958x | 0.930x | 0.913–0.989x |
| transfer | 2 | middle_star | 0.960x | 0.922x | 0.901–1.019x |
| transfer | 2 | star | 0.997x | 0.934x | 0.916–1.026x |
| transfer | 2048 | binary | 4.980x | 4.918x | 4.811–4.989x |
| transfer | 2048 | chain | 8.070x | 8.315x | 7.848–8.345x |
| transfer | 2048 | forward_chain | 6.072x | 6.121x | 6.039–6.259x |
| transfer | 2048 | middle_star | 0.969x | 0.959x | 0.930–1.025x |
| transfer | 2048 | star | 0.927x | 0.926x | 0.921–0.939x |
| transfer | 3 | binary | 0.955x | 0.976x | 0.915–0.986x |
| transfer | 3 | chain | 0.968x | 0.988x | 0.946–1.001x |
| transfer | 3 | forward_chain | 0.990x | 1.029x | 0.963–1.033x |
| transfer | 3 | middle_star | 1.006x | 0.920x | 0.919–1.024x |
| transfer | 3 | star | 0.948x | 0.976x | 0.902–0.978x |
| transfer | 32 | binary | 1.176x | 1.241x | 1.141–1.299x |
| transfer | 32 | chain | 2.752x | 2.168x | 2.152–2.927x |
| transfer | 32 | forward_chain | 2.123x | 2.039x | 1.626–2.166x |
| transfer | 32 | middle_star | 1.019x | 1.087x | 0.962–1.132x |
| transfer | 32 | star | 1.024x | 1.028x | 0.999–1.042x |
| transfer | 512 | binary | 5.045x | 5.068x | 4.975–5.130x |
| transfer | 512 | chain | 8.442x | 8.296x | 8.262–8.502x |
| transfer | 512 | forward_chain | 6.254x | 6.262x | 6.158–6.397x |
| transfer | 512 | middle_star | 0.958x | 0.962x | 0.949–0.967x |
| transfer | 512 | star | 0.927x | 0.928x | 0.925–0.956x |
| transfer | 64 | binary | 1.238x | 1.230x | 1.211–1.250x |
| transfer | 64 | chain | 6.267x | 6.470x | 6.160–6.969x |
| transfer | 64 | forward_chain | 4.939x | 5.221x | 4.740–5.394x |
| transfer | 64 | middle_star | 0.943x | 0.950x | 0.901–0.959x |
| transfer | 64 | star | 0.894x | 0.897x | 0.832–0.997x |
| transfer | 65 | binary | 6.052x | 6.047x | 5.943–6.099x |
| transfer | 65 | chain | 9.362x | 9.287x | 8.922–9.639x |
| transfer | 65 | forward_chain | 8.019x | 7.950x | 7.253–8.138x |
| transfer | 65 | middle_star | 0.958x | 0.954x | 0.938–0.975x |
| transfer | 65 | star | 0.927x | 0.935x | 0.849–0.978x |
| transfer | 8 | binary | 0.969x | 1.075x | 0.964–1.076x |
| transfer | 8 | chain | 1.083x | 1.229x | 1.050–1.259x |
| transfer | 8 | forward_chain | 1.108x | 1.250x | 1.058–1.251x |
| transfer | 8 | middle_star | 0.943x | 1.003x | 0.881–1.005x |
| transfer | 8 | star | 0.974x | 1.009x | 0.941–1.011x |

## wave26-rejected-prospective

### Prospective in-place certification

Domain: finite FullSentence rows whose private parsed head vector allocates successfully. Preserve every row parse, range and root-label check in its existing order, then root count, then the smallest nonterminating start. Empty input succeeds. No API or sentence mutation.

For n words set b=n+1. A private head cell contains 0 when known terminating, h in 1..=n when unseen, and h+b while active on this start's path. Starting at k, follow unseen heads after adding b. Stop at a zero cell (certified suffix) or a value greater than n (cycle). In the former case follow the encoded edges again, subtracting b to recover each next node and writing zero. The root cell starts certified. Earlier completed paths leave no active cells, so the first encountered active cell belongs to the current path, and ascending starts return the same first unrooted tail as legacy walks. Encoding retains the original successor until cleanup; overwriting a private vector cannot change sentence rows.

Cost prediction: at most n-1 nonroot cells marked and cleared, two transitions per visited nonroot cell, n outer starts, O(n) certification time and the existing n-usize head vector only. Raw decimal parsing additionally scans B head-text bytes (including arbitrary leading zeros), giving O(n+B) total gate work. Remove the arbitrary n<=64 branch and color vector. Successful Vec<usize> allocation requires n*sizeof(usize)<=isize::MAX, hence 2n+1 fits usize; no overflow-dependent sentinel. No recursion or callback. Normal allocator failure/panic remains outside the Result relation, and diagnostic Strings allocate on errors.

Behavior complement excludes cycle acceptance, false cycle on completed suffix, and wrong least-tail diagnostic. Reverse reconstruction is many-valued: success admits every rooted tree and arbitrary lexical fields; Cycle(k) admits every preflight-valid head function whose first nonterminating vertex is k, including different cycle lengths and tails. Internal zeroed scratch is not invertible without saved heads. Typed refinements are failure precedence, representation equivalence, resource refinement and termination preimage. Exact categorical semantics have no supported percentage approximation profile: 1/2/5/10% surrogates remain a gap.

Validation plan: retain historical references, add independent head-vector reachability closure oracle over all 126126 maps through n=6, exercise shared suffixes/root positions/cycles/row-error precedence across former threshold, add compiled false-cycle and false-success mutations, run focused debug twice and release twice, and regenerate source-bound partial syntax twice identically. Allocation instrumentation is separate from timing. All finite evidence is scoped, not a universal runtime guarantee.

## History

### 2026-10-30 · epoch-memory

Wave25, 2026-10-30. Current callers share the root-termination observation while retaining different implementations: annotation walk/BFS and conversion walk/color. Preserve the earlier transfer epoch as historical evidence; do not infer new timing from old hybrid measurements.

### 2026-10-30 · checks (evidence)

2026-10-30: 25 focused checks passed twice in debug and twice in release with identical normalized observations. Five annotation checks, 18 converter checks, two independent cross-gate observations. Each source-local enumeration compares 126,126 head vectors. Active-BFS reverse-edge controls and the root adapter control failed as intended and were restored.

### 2026-10-30 · wave26-rejected-performance

Heading `### In-place certification timing — 2026-10-30` stays in wave26-rejected-performance as `### In-place certification timing`.

Status: **rejected by root for production adoption in wave26**. The star-recovery prediction failed, n64 middle-star reached 0.696x (about 44% greater latency), and most large shapes regressed approximately 3–6%. Tiny-chain gains do not establish a representative overall win. The candidate remains on its isolated experiment branch; integration retains conversion source and sidecars from base 5c1a2b9823231a2854bc52cfc04f48983c32cb22. No tuning or further timing follows this rejection. Candidate implementation/evidence commit: 2ada8b8be929438c1c4083e6684f534d75a4e16c.

Ratios are baseline time divided by candidate time; above one favors the candidate. Each run median is the median of seven paired ratios, not a ratio of medians. All-pair ranges combine fourteen samples and are not confidence intervals. Very short timings, changing CPU clocks, cache and compiled caller layout limit precision; no universal throughput/latency, corpus, training, GPU or end-to-end school claim follows. Current baseline is exact function body from 5c1a2b9823231a2854bc52cfc04f48983c32cb22; transfer is fd540f37badaad97b53d56ba81684bbbc2ca4b9b. Legacy is the retained original ordered-walk test reference. These functions are compiled test-only. Source/binary hashes before and after all timings match exactly.

Receipts: `$MOVA_DATA/runs/octopus-wave26-mova-2026-10-30/timing-{current,transfer,legacy}-{1,2}.log`, timing-status.log, timing-before.sha256, timing-after.sha256, timing-summary.md and timing.rs. Allocation observations in wave26-evidence.md are separately scoped and do not explain every measured instruction-level effect.

### 2026-10-30 · wave26-rejected-prospective

Heading `### Prospective in-place certification — 2026-10-30` stays in wave26-rejected-prospective as `### Prospective in-place certification`.

Expected measured gain: remove large-star extra allocation and improve tiny chains that previously repeated suffixes; retain linear long-chain performance. Small-star instruction cost may still regress. Predictions are provisional until paired measurements. Compare actual current parent 5c1a2b9823231a2854bc52cfc04f48983c32cb22, retained transfer parent fd540f37badaad97b53d56ba81684bbbc2ca4b9b and legacy. No timing during other agents' builds and no threshold retuning on confirmation data.

### Undated · document

The retained text below describes the transfer-integration revision contained in fd540f37badaad97b53d56ba81684bbbc2ca4b9b. Its original baseline 34811b38001b47df99fcba3e6a98967adccb4b72 and all measured counts, controls, hashes and timings remain historical. They do not validate later source bytes. Wave25 compares against fd540f37badaad97b53d56ba81684bbbc2ca4b9b; see ../mova.groupant/model.md#transfer-wave25. No old receipt is relabeled as coloring or lazy-index evidence.

### Undated · Wave26 retained checks

The in-place conversion candidate is rejected on measured regressions; current coloring and its sidecars stay at 5c1a2b9. See [candidate timings](model.md#wave26-rejected-performance) and [prospective model](model.md#wave26-rejected-prospective). Candidate implementation 2ada8b8be929438c1c4083e6684f534d75a4e16c; source SHA256 2eb27f331cd7bee9a1b7e5426915ac0a22d89ba473ca0966a5efdf9e6e2f7be5. No candidate performance claim applies to retained production.
