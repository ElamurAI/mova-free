# prag-model-perceptron

## domain

Aligned finite supplied x/y/keys, SLOTS-width valid dictionary coordinates (UNSEEN allowed at prediction), nonempty names of at most 65536 classes, valid class indices, representable offsets/counts and finite weights. Lower-level gain and distance helpers accept their explicitly stated dimensions. Fixed target arithmetic and successful allocation are premises. Macro bodies and semantic AST analysis are absent.

## forward

A row activates one indicator per known in-range value plus bias. Scores are class-wise sums of weights over active indices. Training orders examples by h64(index,seed xor epoch), chooses largest score then smaller class, and adds +1 to gold and -1 to guessed class indicators on errors. Timestamp accumulator U records step*delta; final weights are W-U/(T+1). Prediction ranks scores, emits winning margin and up to five strictly positive winner-minus-runner contributions sorted descending with stable original feature order.

## invariant

UNSEEN and out-of-vocabulary values do not activate lexical indicators; bias remains. Zero epochs yields zero scores, class 0, margin 0 and empty why. Positive explanations are a selected projection: their sum need not equal margin, since negatives and truncation are omitted. Single-class prediction has zero margin.

## cost

F=sum dictionary sizes+1, C classes, N rows, S slots, E epochs. Dense state O(CF+NS); training O(ECNS+EN log N+CF), prediction O(CS+C log C+S log S). Hashing string indices adds bounded index-digit work. Representable feature offsets and finite weights are premises.

## behavior-complement

For a fixed finite model and all-UNSEEN row, observation candidates assigning a lexical feature to why are absent: active set is bias alone. For a two-class single-example gold-1 update starting at zero, margin other than S+1 on that row or other than 1 on an all-UNSEEN row is excluded by averaging over the initial and updated states. At zero epochs, class 1 or positive margin is absent. These explicit parameterized families are an inner complement on valid constructed models.

## reverse-reconstruction

Pred(class,margin,why) cannot recover inactive weights, omitted negative contributions, truncated explanations, original dictionary row or training order. On an all-UNSEEN row it constrains only bias differences; infinitely many finite inactive weights remain compatible. Zero-epoch output admits all dictionaries/training labels with the same nonempty class count. A reconstructed compatible weight matrix is not the actual learning history.

## specialization

Unknown-projection intervention holds trained weights fixed and replaces all coordinates by UNSEEN; the observation becomes bias-only. A single supplied training row with gold 1 gives independent literal weights +/-0.5, hence literal margins S+1 and 1. Zero epochs is a separate abstention/tie boundary.

## composition

Shared gain ratios determine tree order and kNN weights. Perceptron reads the same Dict coordinates independently. Caller eval::fit_set binds one label/name vector per Field to common rows/keys; predict_seq overwrites prev-slot from its prior predicted act. That caller path is inspected here; this leaf executes Dict::build/encode -> fit -> predict on three supplied rows only. Child fragments partition production mechanics; test module remains at the file root. No parser/teacher/corpus/held-out/training-job execution.

## checks

Five native tests in model::wave13_models check canonical histogram arithmetic, literal one-update averaging, tree pruning/fallback, 80 kNN shell-label cases and repeated public fit observations. Exact names and evidence hashes are in root ../model.md#evidence. Two debug and two release runs must have equal normalized named outcomes. Negative production control changes shell admission and must fail a named assertion. No automatic inverse solver, general proof, calibrated probability claim or percentage-error approximant.

## epoch-memory

Initial scoped model. Prospective prediction identified randomized floating reduction in gain_ratio. Old implementation produced nine f64 values on 64 identical histogram calls. BTreeMap now fixes ascending category order; the expected grouping cost increases to O(N log V), with no timing claim. Other reviewed semantics were retained: perceptron averages initial state, tree pruning can lower confidence, and kNN explanations can extend beyond voters. Malformed state, huge counters, long-run floating accumulation, serialization durability and semantic accuracy remain unverified. All typed records bind the exact source hash in the manifest; they are partial source models supported by finite native checks, not universal proofs.

## History

### 2026-10-30 · epoch-memory

2026-10-30 initial scoped model. Prospective prediction identified randomized floating reduction in gain_ratio. Old implementation produced nine f64 values on 64 identical histogram calls. BTreeMap now fixes ascending category order; the expected grouping cost increases to O(N log V), with no timing claim. Other reviewed semantics were retained: perceptron averages initial state, tree pruning can lower confidence, and kNN explanations can extend beyond voters. Malformed state, huge counters, long-run floating accumulation, serialization durability and semantic accuracy remain unverified. All typed records bind the exact source hash in the manifest; they are partial source models supported by finite native checks, not universal proofs.
