# prag-model

## model

# prag-model

## domain

Aligned finite supplied x/y/keys, SLOTS-width valid dictionary coordinates (UNSEEN allowed at prediction), nonempty names of at most 65536 classes, valid class indices, representable offsets/counts and finite weights. Lower-level gain and distance helpers accept their explicitly stated dimensions. Fixed target arithmetic and successful allocation are premises. Macro bodies and semantic AST analysis are absent.

## forward

fit dispatches a shared Train to one of three learners: perceptron with 12 epochs, IGTree, or kNN. A common encoded row then produces Pred{class,conf,why}. The child gain function supplies both tree slot ordering and kNN overlap weights. Class indices mean exactly the supplied names order; Dict slot/value coordinates and sentence key order must remain aligned across fitting, prediction and explanation.

## invariant

All class indices stay within nonempty names for valid models. Repeated same supplied input/seed on a fixed target must preserve prediction observations. conf is a margin for perceptron, local support for IGTree and vote share for kNN, so confidence is not a shared calibrated probability. bytes serializes full perceptron but selected core tuples for IGTree/kNN; it is not full resident-memory usage.

## cost

Dispatch cost equals selected child training plus allocation, not sum of all learners. Prediction chooses one child. Shared gain work is charged once within that learner. Serialization allocates bytes; info formats metadata. No training-throughput or accuracy measurement is claimed.

## behavior-complement

For any two-class zero-weight perceptron and equal-scoring row, class 1 is absent; the tie selects 0. For the supplied three-row tree, absent first-slot value cannot produce the minority class 1. For positive k and fixed four-row kNN memory with two exact matches, a prediction counting only one exact match as its vote denominator is absent. These algorithm-tagged transition families define a concrete source-derived inner behavior complement in the universe (Kind,valid state,row,Pred); malformed state and untested arithmetic extremes remain unknown.

## reverse-reconstruction

An observed class 0 may come from zero perceptron ties, tree majority fallback or kNN tie resolution. Even Kind plus conf/why leaves inactive dimensions and training histories ambiguous. Candidate reconstruction must retain shared names order, dictionary coordinates, row/key alignment, seed and child constraints. Without these correlations an invented tree default or nearest key is not a compatible antecedent.

## specialization

Confidence-observation projection intentionally erases kind and explanation. Equal numeric confidence then has incompatible meanings across learners (margin versus support versus vote fraction); it cannot support direct probability calibration. The public tiny fit test retains kind and compares class/conf/why and byte counts across two independent fits; actual eval::fit_set and predict_seq use common dictionary coordinates and per-field names.

## composition

Shared gain ratios determine tree order and kNN weights. Perceptron reads the same Dict coordinates independently. Caller eval::fit_set binds one label/name vector per Field to common rows/keys; predict_seq overwrites prev-slot from its prior predicted act. That caller path is inspected here; this leaf executes Dict::build/encode -> fit -> predict on three supplied rows only. Child fragments partition production mechanics; test module remains at the file root. No parser/teacher/corpus/held-out/training-job execution.

## checks

Five native tests in model::wave13_models check canonical histogram arithmetic, literal one-update averaging, tree pruning/fallback, 80 kNN shell-label cases and repeated public fit observations. Exact names and evidence hashes are in root model.md#evidence. Two debug and two release runs must have equal normalized named outcomes. Negative production control changes shell admission and must fail a named assertion. No automatic inverse solver, general proof, calibrated probability claim or percentage-error approximant.

## epoch-memory

Initial scoped model. Prospective prediction identified randomized floating reduction in gain_ratio. Old implementation produced nine f64 values on 64 identical histogram calls. BTreeMap now fixes ascending category order; the expected grouping cost increases to O(N log V), with no timing claim. Other reviewed semantics were retained: perceptron averages initial state, tree pruning can lower confidence, and kNN explanations can extend beyond voters. Malformed state, huge counters, long-run floating accumulation, serialization durability and semantic accuracy remain unverified. All typed records bind the exact source hash in the manifest; they are partial source models supported by finite native checks, not universal proofs.

## evidence

### Native evidence

Old production failed gain_ratio_canonical_histogram_oracle: 64 identical calls produced nine different f64 bit patterns. Ascending category accumulation repairs this target-local nondeterminism. The earlier canonical-value assertion also failed by one low bit; both logs are retained. No semantic prediction-change or speedup claim follows from these bit differences.

The compiled production mutation distinct > k to distinct >= k fails knn_distance_shells_and_frequency_ties at its mask=0, k=1 assertion. Source bytes were restored exactly; all five positive tests pass again.

The independent histogram test has 17 category histograms and analytic empty/zero/one endpoints. Perceptron uses one example and one update with literal expected margins; tree uses three supplied rows and literal fallback/pruning behavior. kNN enumerates 16 labelings x five shell counts = 80 vote checks, plus skip/UNSEEN and prior ties. The public fit join trains only three supplied rows twice per kind. Two syntax projections are byte-identical. Four fragments plus root declare 15 typed views; this count is not proof coverage.

Actual caller join: eval::fit_set shares x/dict/keys across fields, then predict_seq overwrites prev-slot by prior predicted act. The leaf executes Dict::build/encode -> fit -> predict; broader per-field and previous-sentence joins belong to integration. No filesystem durability, parser, teacher, corpus, GPU, held-out or calibrated probability evidence. Existing unrelated compiler warnings remain.

## predictions

### Prospective model review

Domain: supplied in-memory aligned x/y/keys; rows have SLOTS.len() entries; positive class count <=65536, valid labels and Dict coordinates; representable counters/features; finite weights. No parser, teacher, corpus, GPU or training job.

Forward: gain ratio aggregates label histograms per slot value then computes entropy reduction divided by split entropy. Perceptron uses active known indicators plus bias, seed-hashed example order and timestamp averaging. IGTree sorts slots by gain ratio and index, retains plurality defaults (prior then smaller class breaks ties), prunes same-default leaves. kNN sorts weighted-overlap distances then memory index, includes all examples in first k distinct distance shells, and chooses k by leave-one-out accuracy with smaller candidate on ties.

Suspected witnessed bug: gain_ratio sums floating contributions over randomized HashMap iteration. Equal data can produce different low bits, potentially changing tied slot order or weighted distance boundaries. Predict deterministic ascending slot-value accumulation fixes repeated bits; no signature change. Cost changes expected grouping O(N) to O(N log V); traversal still O(V*C), memory O(V*C). No speedup claim.

Invariants: unknown values are inactive in perceptron; UNSEEN mismatches even itself in kNN; deterministic smaller-class tie; IGTree missing edges use current node default; k counts distances, not rows. Prediction confidence differs by model and is not comparable probability calibration.

Tests will use analytic entropy endpoints, an independent sorted-value histogram oracle, literal single-step averaged weights, literal tree defaults/pruning, exhaustive small kNN shell histograms and tiny public fit/dictionary joins. Mutation must break a named assertion. Empty classes, malformed lengths/serialized state and overflow-sized training remain outside valid-input model. Actual eval::fit_set passes common x/dict/keys and one label vector per Field; predict_seq supplies encoded rows and previous predicted act. Only tiny in-memory API joins will execute.

## History

### 2026-10-30 · epoch-memory

2026-10-30 initial scoped model. Prospective prediction identified randomized floating reduction in gain_ratio. Old implementation produced nine f64 values on 64 identical histogram calls. BTreeMap now fixes ascending category order; the expected grouping cost increases to O(N log V), with no timing claim. Other reviewed semantics were retained: perceptron averages initial state, tree pruning can lower confidence, and kNN explanations can extend beyond voters. Malformed state, huge counters, long-run floating accumulation, serialization durability and semantic accuracy remain unverified. All typed records bind the exact source hash in the manifest; they are partial source models supported by finite native checks, not universal proofs.

### 2026-10-30 · evidence

Heading `### Native evidence, 2026-10-30` stays in evidence as `### Native evidence`.

Five tests in model::wave13_models pass twice in debug and twice in release with equal normalized named outcomes (5/5 each, no ignored cases). Raw logs live under $MOVA_DATA/runs/octopus-wave13-model-2026-10-30/. These are tiny supplied-input checks, not the legacy 200-row training test, a corpus evaluation or live training job.

#### Bindings

Final source SHA-256: e18c587fd1f87b18c7e0118e0009b16fb469e26ba9cd0edf26b097226130ee25

- predictions.md: b06357400771b84108bc1b5b572433bd117118c78c05b11f322ec25b7a7cec99
- old-code.log: 95fef9db8366108136d5a2f42c36968817e83235c77fbd715173f09e6fbc1928
- old-nondeterminism.log: eca4cb464efa87b1c82794507d18b6ccceca1f4027fa4f97a4db58175b3fe9e1
- debug1.log: 0f6a8a64366821793e69e63e16a5f9906b138acd368c97a9070e5341c4891ff1
- debug2.log: 5bea463837ce5aaa92fef411d824f1e5e1d5dd94719a0b75e8434e0e5cec1d30
- release1.log: 3a10dab388cbbfa889e682893c13edfd1404c1165cc8f67feece939609281052
- release2.log: 2dbd7f8d7caf930b3077ab5395a12645ce159ad6eeabb32027003441a56a0e05
- mutant-shell.log: 8a1a91e2b812f342ce657d3ac26f01ab9849bbecf235eff0f6cfbed5182cb34c
- restored.log: 783d61c3ee7704aa44b5e1173c485d1bd6762256952535ad83f7769b728857c0
- normalized.txt: 70bd9773789e1f09a75441c4e5f1a7c554994418c32c4df5af4cbe6505a44d88
- syntax1.json: d1e176089771c44de040b35da944811ab1c38e54290d99e0adcfe3978d86662c
- syntax2.json: d1e176089771c44de040b35da944811ab1c38e54290d99e0adcfe3978d86662c

### 2026-10-30 · predictions

Heading `### Prospective model review, 2026-10-30` stays in predictions as `### Prospective model review`.
