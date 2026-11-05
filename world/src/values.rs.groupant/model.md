# value-judgment

## descriptant

# value-judgment

## Forward

Fold commands through names/events/holder/harmed/helped/goals/speech/rivals/rests, emitting judgments when global rules exist.

## Invariant

Lines are 1-based command positions. Harm history persists after relief; helped pairs deduplicate forever. Fields and goal peers iterate lexically. Harm/help eligibility is frozen at command start; mercy requires prior harm by this agent.

## Cost

Parsing O(B+K log(1+F)) for K key/value tokens and F distinct keys, plus string comparison costs; goal updates O(log G), goal scans O(G). Existing history scans can be quadratic in L and goal/rest nesting higher. Other maps retain expected lookup costs; no timing or speedup claim.

## Composition

Fields feeds branches; ownership uses prior events/names; set branch shares achievement and harm/help state. Full ordered observations are modeled under fixed global tables. History owns the nested set-transition equations; its costs overlap with this file fold.

## Epoch memory

## catalog

# value-judgment typed models

## nonbehavior

Type: behavior_complement. Domain, observation and operator: U=finite command traces x projected judgment multisets; R is the deterministic command fold under fixed global tables. Candidate inner bound U minus R includes truth violation from narrator-only speech and causal state judgment without earlier cause event.

Evidence/falsification: Existing four tests and probe. See root model.md#current-evidence for current execution and model.md#evidence for historical observations; other proposed checks remain unexecuted. Status: partial source-derived model with bounded evidence only.

## antecedents

Type: reverse_reconstruction. Domain, observation and operator: For observed y retain all traces with projection(judge(trace))=y. Ignored commands, overwritten events and name aliases preserve ambiguity; because/line observations narrow fibers.

Evidence/falsification: Probe finite speech fibers. See root model.md#current-evidence for current execution and model.md#evidence for historical observations; other proposed checks remain unexecuted. Status: partial source-derived model with bounded evidence only.

## speech-gate

Type: predicate_partition. Domain, observation and operator: Truth requires speaker!=narrator, sincere exactly0 and rule present; why/means/to are not required. Missing speaker defaults empty.

Evidence/falsification: Probe 3x3 matrix. See root model.md#current-evidence for current execution and model.md#evidence for historical observations; other proposed checks remain unexecuted. Status: partial source-derived model with bounded evidence only.

## ownership-rule

Type: state_transition. Domain, observation and operator: Character transfer away from stored owner violates unless event agent is owner and verb class give. Noncharacter destination retains owner; character causal agent is blamed else recipient.

Evidence/falsification: Probe give/drop/transfer. See root model.md#current-evidence for current execution and model.md#evidence for historical observations; other proposed checks remain unexecuted. Status: partial source-derived model with bounded evidence only.

## theft-double

Type: multiplicity. Domain, observation and operator: Theft-concept character cause emits independently from ownership transfer test; both branches can emit justice on one move.

Evidence/falsification: Probe steal double emission. See root model.md#current-evidence for current execution and model.md#evidence for historical observations; other proposed checks remain unexecuted. Status: partial source-derived model with bounded evidence only.

## harm-memory

Type: history_projection. Domain, observation and operator: External character harm cause adds agent once to harmed[target]; relief never removes entry, so same-agent later harm remains suppressed.

Evidence/falsification: Probe harm-relief-harm. See root model.md#current-evidence for current execution and model.md#evidence for historical observations; other proposed checks remain unexecuted. Status: partial source-derived model with bounded evidence only.

## help-memory

Type: deduplication. Domain, observation and operator: Relief needs harmed[target] recorded before this command and absent helped(agent,target); adds pair then optionally mercy from prior agent harm and reciprocity from earlier reverse help. Later harm does not reset suppression.

Evidence/falsification: Probe repeated relief. See root model.md#current-evidence for current execution and model.md#evidence for historical observations; other proposed checks remain unexecuted. Status: partial source-derived model with bounded evidence only.

## reciprocity-time

Type: temporal_precondition. Domain, observation and operator: Reciprocate needs earlier helped(target,agent,line) with line<current. Current appended help cannot satisfy prior-time gate. Each help direction requires target harm history.

Evidence/falsification: Existing lion/mouse. See root model.md#current-evidence for current execution and model.md#evidence for historical observations; other proposed checks remain unexecuted. Status: partial source-derived model with bounded evidence only.

## goal-rivalry

Type: join_constraint. Domain, observation and operator: Self-caused relief status with goal joins other same-goal agents. Rival edges derive endpoints from referenced speech, not rel a/b fields; scorner-target match triggers humility.

Evidence/falsification: Existing tortoise/hare; contradictory endpoints proposed. See root model.md#current-evidence for current execution and model.md#evidence for historical observations; other proposed checks remain unexecuted. Status: partial source-derived model with bounded evidence only.

## rest-window

Type: interval_constraint. Domain, observation and operator: Both agents' rest windows use achiever goal time since, not peer goal time: since<rest<achievement. No achiever rest plus some peer rest triggers paired perseverance judgments.

Evidence/falsification: Existing tortoise/hare; asymmetric timestamps proposed. See root model.md#current-evidence for current execution and model.md#evidence for historical observations; other proposed checks remain unexecuted. Status: partial source-derived model with bounded evidence only.

## gap-observation

Type: knowledge_gap. Domain, observation and operator: gaps reports unknown nonempty event verbs and unknown values only on feeling/freedom/status/health/sleep, deduplicating exact strings. This is catalog coverage, not truth.

Evidence/falsification: Probe duplicate unknown verb. See root model.md#current-evidence for current execution and model.md#evidence for historical observations; other proposed checks remain unexecuted. Status: partial source-derived model with bounded evidence only.

## canonical-command-order

Type: deterministic_projection. Domain, observation and operator: Field and goal-peer iteration is lexical. Harm/help eligibility uses command-start history; first qualifying lexical fields select explanations. Equal final field maps yield equal ordered observations under fixed prior state and tables.

Evidence/falsification: Current finite snapshot oracle and historical mixed-field counterexample. See root model.md#current-evidence for current execution and model.md#evidence for historical observations; other proposed checks remain unexecuted. Status: partial source-derived model with bounded evidence only.


## tests

### Checks

model.md#current-evidence records the current direct-source oracle, source tests, controls and regenerated syntax. Historical model.md#evidence preserves earlier observations. The mixed-state-probe.rs.txt two-projection assertion requires values.rs from baseline 1e63aaf0a; it is not a test of current behavior.

The current oracle checks all Judgment fields in order across 1,280 finite cases. Catalog experiments beyond executed scopes remain proposed. Structural checks validate identities and syntax envelopes, not semantic correctness. No GPU, teacher, full application run or general proof.

Current policy details and the 1,280-case oracle are in model.md#current-evidence; benchmark.rs.txt records a separate bounded CPU workload. Historical mixed-state assertions require the earlier checkout. The integration also rejected three isolated source mutants and passed the fresh world crate value tests.

## evidence

### Executed evidence

Seven tests pass twice: four existing source tests plus speech_fibers, ownership_and_history and gap_dedup. Nine speech inputs have two positives (C1 and empty speaker with sincere=0); duplicate-key case checks last-write. Four ownership fixtures cover absent cause, lawful give, ground transit and double justice. Repeated harm/relief yields one harm, one help, one mercy. False per-episode harm reset predicts two harm judgments and fails, exit101. Duplicate unknown event gaps collapse.

HashMap traversal makes ordering unconstrained; mixed harm/relief fields can alter behavior, not just order. Probes use one state field; no general multiset determinism claim. Persistent harm/help and double justice are current implementation facts, not normative claims. Goal timestamp edges, event overwrite and additional mixed-field adversarial checks remain proposed; the bounded integration witness below is executed.

#### Reproduction and provenance

Compile probe.txt as Rust using rustc --edition=2024 --test --crate-name values_groupant_probe world/src/values.rs.groupant/probe.txt with --extern global=RLIB and -L dependency=DEPS from a default-CPU build. Run --nocapture --test-threads=1. Set FALSE_FORMULA=1 and select ownership_and_history --exact --nocapture to reject the false formula, exit101.

The probe directly compiles current values.rs and its tests through #[path]. Global dependency rlib reuses octopus-groupants-anchored build at baseline5794fd531; world fingerprint features=[]. Git diff through4279da541 excluding sidecars found unchanged en/global/Cargo.lock and the two source files. Generated global tables remain historical build data; not rebuilt or proven against live data. No world rlib reused.

#### Structural scope

Twelve root models plus three four-model fragments:24 entries/four nodes per file;48 entries/eight nodes across numbers/values. First entries target U minus R; second preserve ambiguous reconstruction. Partial syntax binds fragment envelopes; macro expansion, type/name resolution and universal correspondence absent.

Structural inventory passes unchanged registry with20 file sidecars,six groups,512 missing sidecars,no diagnostics. Parent registers new roots using registry-node.json. changed reports no production Rust changes. Structural validity is not semantic proof. No GPU, training, live teacher or active-data mutation.

#### Mixed-field schedule witness

The integrated mixed-state-probe.rs.txt directly includes unchanged values.rs with the same historical global dependency artifact described above. A single set command combines freedom=captive and status=free after a causal event. Two independent runs of 256 fresh-map calls each observed two distinct sorted judgment projections; the summaries were byte-identical. Thus sorting does not repair this witnessed membership difference. The --false-determinism control asserts only one projection and is rejected with exit 101.

## current-evidence

### Current command-start policy evidence

#### Finite correspondence

The direct-source set-transition-probe.rs.txt enumerates 10 reachable prior-history settings: absent harm, another agent's harm, or current agent's harm; current-agent prior help is allowed only with prior target harm; earlier reverse help is independently absent/present. Each combines 16 assignments to freedom/status from absent, unclassified, harm and relief, four gates (missing causal event, noncharacter agent, self-cause, distinct characters), and two field-token orders: 1,280 cases, eight observed eligibility/output bit patterns.

Expected outputs independently use closed-form history predicates and literal lexical field representatives. Every emitted Judgment field is compared in vector order, including explanation and source line. Prefix judgments are excluded by the final command's line. Both same-kind keys exercise first-explanation selection; mixed keys exercise snapshot eligibility and output-bundle order. Seven included source tests additionally cover 144 state-field permutations, 120 gap permutations, lexical goal peers, duplicate final writes and lifetime suppression. This records bounded correspondence, not a general proof or implemented reverse executor.

#### Build and dependency scope

Compile with rustc --edition=2024 --test --crate-name value_snapshot_probe world/src/values.rs.groupant/set-transition-probe.rs.txt --extern global=RLIB -L dependency=DEPS. This run reuses the historical default-CPU global artifact described in model.md#evidence, from octopus-groupants-anchored. Actual values.rs is compiled directly; no world rlib is reused. The probe asserts its captive/free/unclassified state classifications and requires named principle rules. Generated global tables remain an external dependency not bound by this source hash. The parent separately ran fresh Cargo world values tests.

#### Controls and structural limits

FALSE_SNAPSHOT=1 adds the false expectation that mixed harm/relief on an unharmed target permits help. This control must fail at a designated mixed-field case. It tests model discrimination; it is not a production mutation. Current full observations must agree across both token orders and both test runs. The historical randomized two-projection assertion must be run against the historical source checkout.

All five local manifests are reanchored to regenerated syntax, including the new nested child. Scope selection follows current function/set/guard boundaries, with node IDs and envelopes recomputed; hashes alone were not refreshed. Syntax remains partial, excludes macro expansion/type resolution and does not prove source semantics. The file now declares 32 models over five nodes; the eight child models add explicit questions, not eight proofs.

#### Executed results

Two positive runs each passed eight tests, including all 1,280 finite cases; observations matched excluding elapsed-time summaries. The false-snapshot expectation failed with exit 101. Two isolated source mutants independently replaced frozen help eligibility and frozen mercy eligibility with live history reads; the finite oracle rejected both with exit 101. Production source was never changed for these controls. Three syntax generations (including the freshly built current modelcheck) were byte-identical.

Current modelcheck validates this file and all four fragments with no local diagnostics. The full inventory in this isolated worktree reports only the three expected stale ancestor bindings (lexical-semantics, inference-core and mova); their refresh belongs to parent integration. No standalone full-inventory pass is claimed here. A discarded one-leaf-registry check also reported unrelated missing ownership declarations, so the meaningful local result comes from the full-registry run.

Model review corrected an overstrong idempotence claim: initially unharmed mixed commands can emit harm on their first application and help on their second; only after that do repeated identical commands stabilize. The child catalog now retains that two-step boundary.

#### Integration tests, controls and bounded timing

Before editing, prospective.md predicted ordered-map comparison costs and explicitly made no speedup prediction. benchmark.rs.txt preserves the bounded CPU fixture: two ordinary single-state-field commands with 0/4/16/64 extra neutral fields, 32 warmups and 2,000 judge calls per timing. The comparison compiled baseline1e63aaf0a and current source with rustc opt-level=3 against the same historical global dependency artifact. Three rounds alternated before/after order. Full Debug-output fingerprints were equal across all fixtures/runs (a regression observation, not a collision-free equivalence proof).

| Extra neutral fields per set | Before median ms | After median ms | Change |
|---:|---:|---:|---:|
| 0 | 18.394 | 17.952 | -2.4% |
| 4 | 54.772 | 52.060 | -5.0% |
| 16 | 165.037 | 160.481 | -2.8% |
| 64 | 607.628 | 588.662 | -3.1% |

## History

### 2026-10-29 · Epoch memory

2026-10-29: source-derived initial model, source hash f450843ba1c92f35884eb71667d432f36a34f6316c9b8a252811e0dac87ddbe6. Scope envelope [0,18574) is anchored to selected partial syntax nodes, not semantic proof or full coverage. Runtime source unchanged. AST omits macros expansion, name resolution, inferred types and cfg evaluation. Generated dictionary/global tables are fixed dependencies not bound by this file hash. Catalog false-formula controls and boundary observations refine claims without modifying production.

2026-10-29 current policy epoch: source b885d25364939e38e130ff598a24a46efd21f500ba51e27cd4465ceddb56f064. Regenerated syntax and reselected source-node envelopes replace historical anchors above. Historical unchanged-source claims refer only to their epoch. Current correspondence and dependency limits appear in the file-root model.md#current-evidence.

### 2026-10-29 · document

Epoch 2026-10-29; subject values.rs, SHA-256 b885d25364939e38e130ff598a24a46efd21f500ba51e27cd4465ceddb56f064. Domain is the bounded calls/traces stated below; observation horizon is one invocation unless explicitly a trace. Fixed compiled global/dictionary tables, successful allocation, unpoisoned locks and nonoverflowing arithmetic are assumptions. Every relation is a partial source-derived specification; Rust correspondence beyond recorded checks remains open. Behavior complements target U minus R and restrictions are candidate inner bounds, not proven complements. Reverse relations preserve all compatible antecedents. Fields and goal peers use lexical order; harm eligibility uses command-start history. Historical traversal evidence is retained separately.

### 2026-10-29 · evidence

Epoch 2026-10-29; baseline 4279da541. Production source unchanged.

Logs: $MOVA_DATA/runs/octopus-next-lexical-2026-10-29/values-positive.log, values-repeat.log, values-negative-confirmed.log, dependency-diff.txt, verification.txt, values-syntax-repeat.json, groupants.json, changed.json. Test observations repeat identically excluding elapsed-time summary. Syntax repeats byte-for-byte. No timing gain claimed.

This bounded observation depends on randomized HashMap traversal; it makes no probability or exhaustive schedule claim. If a future run does not observe both schedules, it stops for inspection rather than silently retrying. It is not a stable universal determinism test. Source-derived schedule reasoning and the observed witness remain distinct. No production fix is included. Build with rustc --edition=2024 --crate-name mixed_state_probe and the same --extern global / dependency directory as probe.txt, then run normally or with --false-determinism. Logs are under $MOVA_DATA/runs/octopus-next-groupants-2026-10-29/mixed-{1,2}.json, mixed-control.json and mixed-false.log.

### 2026-10-29 · current-evidence

Epoch 2026-10-29. Runtime changes are the two source commits 4e6767c33aafc9ad0364ee115249522371f32b07 and 13bfe15cbd61be38000f7eebafd91b5bd5a49cd2; manifest.json binds the final source hash. The older model.md#evidence and mixed-state-probe.rs.txt describe the historical source, not current behavior.

The run directory is $MOVA_DATA/runs/octopus-value-models-2026-10-29. It contains two positive logs, a false-snapshot rejection, deterministic regenerated syntax and structural reports. No GPU, teacher, training, task-score improvement or performance gain is claimed.

Fresh CPU `cargo test -p world --lib values::tests --locked -- --test-threads=1` passed all seven source tests. An independent baseline build with the new regression rejected the old source for same-command help. Three isolated current-source mutants (live help eligibility, live mercy eligibility, reversed goal order) each failed their designated test. Logs are in $MOVA_DATA/runs/octopus-value-policy-2026-10-29. The source and tests were not mutated in the repository for controls.

These 2026-10-29 measurements include allocation and use a shared host; the first baseline zero-extra round was 30.463 ms. Three medians do not establish a broad speedup or statistically calibrated uncertainty. Generated dependencies were reused, not release-rebuilt; this is not school/course or GPU throughput. The symbolic cost model remains uncalibrated outside these fixtures. Raw TSVs, builder, templates and summary are in the integration run directory. The retained benchmark can be compiled in each checkout with its explicit --extern global and dependency directory.

### Undated · evidence

#### Historical evidence boundary

Historical evidence below applies to source f450843ba1c92f35884eb71667d432f36a34f6316c9b8a252811e0dac87ddbe6 (available at baseline 1e63aaf0a). Reproduce mixed-state-probe.rs.txt in that historical checkout; it includes adjacent values.rs and its two-projection assertion is not a current regression test. The current fix and deterministic checks are recorded in model.md#current-evidence.
