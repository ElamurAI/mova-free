# prag-model-tree

## domain

Aligned finite supplied x/y/keys, SLOTS-width valid dictionary coordinates (UNSEEN allowed at prediction), nonempty names of at most 65536 classes, valid class indices, representable offsets/counts and finite weights. Lower-level gain and distance helpers accept their explicitly stated dimensions. Fixed target arithmetic and successful allocation are premises. Macro bodies and semantic AST analysis are absent.

## forward

Sort slot indices by descending gain ratio then ascending index. Each node records the maximum (local count,global prior,-class), local support N and winning hits. Partition recursively by ordered slot values, stop on purity or exhausted slots, and drop a leaf matching its parent's default. Prediction follows matching edges in slot order; on the first missing edge it returns that node default, support fraction and visited path. Leaf-pruning preserves the label at that branch, not its original confidence.

## invariant

Children sort by value and refer to retained nodes. Unknown edge does not synthesize a child; missing-edge prediction reports the current node share. Pruning a pure same-label leaf can replace confidence 1 with parent share. Public malformed deserialized graphs and out-of-dictionary training values remain outside this invariant.

## cost

Gain costs plus O(S log S) ordering; at most O(NS) node visits over training rows with O(NS log N) partition upper bound and O(CNS) conservative histogram setup. Model space O(NS+dictionary text). Prediction O(S log N) edge searches plus explanation text allocation; nodes and retained edges may be much smaller after pruning.

## behavior-complement

Fix the three rows (a,0),(a,0),(b,1), all other slots constant. Output class 1 for absent first-slot value is excluded; the root fallback is class 0 at 2/3. A prediction for a with confidence 1 is also excluded by same-label leaf pruning. More generally a trace continuing beyond its first absent edge, or a reported class different from the terminal node default, belongs to an inner absent-behavior model.

## reverse-reconstruction

Observed root fallback (0,2/3) admits both a pruned known value and unseen values; even with the model fixed it does not identify which query was supplied. Path observations retain visited values only, losing all unused coordinates. Multiple training multisets and dictionaries can yield the same retained tree after pruning; no recovery of pruned counts or original examples is unique.

## specialization

Pruning counterfactual retains same-default leaves instead of dropping them. Labels agree in the tiny fixture, but known-a confidence changes from 2/3 to 1 and explanation length from 1 to 2. The native oracle explicitly asserts current compressed confidence, while missing values still use root support.

## composition

Shared gain ratios determine tree order and kNN weights. Perceptron reads the same Dict coordinates independently. Caller eval::fit_set binds one label/name vector per Field to common rows/keys; predict_seq overwrites prev-slot from its prior predicted act. That caller path is inspected here; this leaf executes Dict::build/encode -> fit -> predict on three supplied rows only. Child fragments partition production mechanics; test module remains at the file root. No parser/teacher/corpus/held-out/training-job execution.

## checks

Five native tests in model::wave13_models check canonical histogram arithmetic, literal one-update averaging, tree pruning/fallback, 80 kNN shell-label cases and repeated public fit observations. Exact names and evidence hashes are in root ../model.md#evidence. Two debug and two release runs must have equal normalized named outcomes. Negative production control changes shell admission and must fail a named assertion. No automatic inverse solver, general proof, calibrated probability claim or percentage-error approximant.

## epoch-memory

Initial scoped model. Prospective prediction identified randomized floating reduction in gain_ratio. Old implementation produced nine f64 values on 64 identical histogram calls. BTreeMap now fixes ascending category order; the expected grouping cost increases to O(N log V), with no timing claim. Other reviewed semantics were retained: perceptron averages initial state, tree pruning can lower confidence, and kNN explanations can extend beyond voters. Malformed state, huge counters, long-run floating accumulation, serialization durability and semantic accuracy remain unverified. All typed records bind the exact source hash in the manifest; they are partial source models supported by finite native checks, not universal proofs.

## History

### 2026-10-30 · epoch-memory

2026-10-30 initial scoped model. Prospective prediction identified randomized floating reduction in gain_ratio. Old implementation produced nine f64 values on 64 identical histogram calls. BTreeMap now fixes ascending category order; the expected grouping cost increases to O(N log V), with no timing claim. Other reviewed semantics were retained: perceptron averages initial state, tree pruning can lower confidence, and kNN explanations can extend beyond voters. Malformed state, huge counters, long-run floating accumulation, serialization durability and semantic accuracy remain unverified. All typed records bind the exact source hash in the manifest; they are partial source models supported by finite native checks, not universal proofs.
