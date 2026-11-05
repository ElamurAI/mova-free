# prag-model-knn

## domain

Aligned finite supplied x/y/keys, SLOTS-width valid dictionary coordinates (UNSEEN allowed at prediction), nonempty names of at most 65536 classes, valid class indices, representable offsets/counts and finite weights. Lower-level gain and distance helpers accept their explicitly stated dimensions. Fixed target arithmetic and successful allocation are premises. Macro bodies and semantic AST analysis are absent.

## forward

Distance is sum of slot weight for unequal values OR either equal UNSEEN pair; known equal pairs contribute zero. Sort by distance then memory index. Vote uses every row in the first k distinct distance values, choosing (vote count,training prior,-class). Training selects k from [1,3,5,7,11] by leave-one-out hit count, first candidate on ties. Prediction confidence is winning votes/max(1,selected rows); explanation takes the nearest three winning-class memories from the entire sorted memory, not necessarily only voters.

## invariant

Equal distance ties include all examples, so k is not neighbor count. skip excludes the exact index, not duplicate rows/keys. Empty vote yields prior winner with zero confidence. Unknown matching is deliberately penalized. Explanations preserve keys and distances but can include nonvoting examples. Class storage u16 and counts u32 impose declared representability bounds.

## cost

Training gain costs plus N distance lists: O(N^2 S+N^2 log N+5N^2+5NC); model O(NS+N+C+key bytes). Prediction O(NS+N log N+C), retaining O(N+C) scratch. k limits voting distance shells but does not avoid full distance sorting.

## behavior-complement

Fix weights (1,2), query (0,0) and memory rows (0,0),(0,0),(1,0),(0,1). The candidate transition selecting only one voter at k=1 is absent for every binary labeling: exactly the two distance-zero rows vote. A distance zero for (UNSEEN,0) versus itself is absent under these weights. With equal class votes and priors, larger-index winner is absent. These 16 labelings and five shell limits form a concrete inner complement with independent integer histogram checks.

## reverse-reconstruction

Vote tuple (class,hits,n) retains all label assignments within admitted distance shells satisfying the plurality and prior tie rule; labels outside shells are unconstrained by votes. With two tied-prior zero-distance rows, (0,1,2) admits both [0,1] and [1,0]. Distances lose which mismatch slots contributed when weights coincide. Explanation keys cannot recover original sentence content or unselected examples.

## specialization

Shell-boundary intervention changes strict distinct>k to distinct>=k, excluding the kth shell. The compiled mutation must fail the independent 16-labeling histogram oracle. Skip-index and UNSEEN self-distance checks distinguish identity and uncertainty from feature equality. k tuning uses training-only leave-one-out; no held-out evidence or accuracy gain is inferred.

## composition

Shared gain ratios determine tree order and kNN weights. Perceptron reads the same Dict coordinates independently. Caller eval::fit_set binds one label/name vector per Field to common rows/keys; predict_seq overwrites prev-slot from its prior predicted act. That caller path is inspected here; this leaf executes Dict::build/encode -> fit -> predict on three supplied rows only. Child fragments partition production mechanics; test module remains at the file root. No parser/teacher/corpus/held-out/training-job execution.

## checks

Five native tests in model::wave13_models check canonical histogram arithmetic, literal one-update averaging, tree pruning/fallback, 80 kNN shell-label cases and repeated public fit observations. Exact names and evidence hashes are in root ../model.md#evidence. Two debug and two release runs must have equal normalized named outcomes. Negative production control changes shell admission and must fail a named assertion. No automatic inverse solver, general proof, calibrated probability claim or percentage-error approximant.

## epoch-memory

Initial scoped model. Prospective prediction identified randomized floating reduction in gain_ratio. Old implementation produced nine f64 values on 64 identical histogram calls. BTreeMap now fixes ascending category order; the expected grouping cost increases to O(N log V), with no timing claim. Other reviewed semantics were retained: perceptron averages initial state, tree pruning can lower confidence, and kNN explanations can extend beyond voters. Malformed state, huge counters, long-run floating accumulation, serialization durability and semantic accuracy remain unverified. All typed records bind the exact source hash in the manifest; they are partial source models supported by finite native checks, not universal proofs.

## History

### 2026-10-30 · epoch-memory

2026-10-30 initial scoped model. Prospective prediction identified randomized floating reduction in gain_ratio. Old implementation produced nine f64 values on 64 identical histogram calls. BTreeMap now fixes ascending category order; the expected grouping cost increases to O(N log V), with no timing claim. Other reviewed semantics were retained: perceptron averages initial state, tree pruning can lower confidence, and kNN explanations can extend beyond voters. Malformed state, huge counters, long-run floating accumulation, serialization durability and semantic accuracy remain unverified. All typed records bind the exact source hash in the manifest; they are partial source models supported by finite native checks, not universal proofs.
