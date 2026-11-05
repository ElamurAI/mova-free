# prag-model-gain

## domain

Aligned finite supplied x/y/keys, SLOTS-width valid dictionary coordinates (UNSEEN allowed at prediction), nonempty names of at most 65536 classes, valid class indices, representable offsets/counts and finite weights. Lower-level gain and distance helpers accept their explicitly stated dimensions. Fixed target arithmetic and successful allocation are premises. Macro bodies and semantic AST analysis are absent.

## forward

For aligned rows/labels, construct per-value class histogram h[v,c] and totals a[c]. H(a)=-sum p log2 p with zero terms omitted. In ascending numeric value order compute Q=sum_v n_v/N H(h_v), S=-sum_v n_v/N log2(n_v/N). Return 0 if S<=1e-12, otherwise max(0,(H(a)-Q)/S). Empty input returns zero through the no-partition branch. weights applies this independently at each slot. freq and argmax_tie select maximum (count,prior,-class).

## invariant

Fixed histograms and target floating implementation yield the same sum order and bits. Counters and their sums fit u32; labels index classes. Mathematical gain ratio lies in [0,1], but no hard floating upper clamp is asserted. Empty class sets are outside training domain.

## cost

N rows, V distinct values and C classes: O(N log V + VC) time and O(VC) temporary counts per slot. Previously expected O(N+VC) grouping used randomized hashing. S slots repeats this work. No measured timing improvement is claimed.

## behavior-complement

Observation universe is a histogram plus a scalar f64 result under fixed floating arithmetic. Proven-by-formula absent families: a single constant slot paired with a nonzero result; two equally frequent perfectly class-separating values paired with a result other than 1; the same fixed category histograms producing two output bit patterns. The last exclusion requires canonical traversal, which the old code violated. Counter sums and valid classes are premises; this is a source-derived inner complement, not a claim over malformed rows.

## reverse-reconstruction

A zero result admits constant slots, empty input and nonconstant slots statistically independent of labels. One admits arbitrary duplicated balanced perfectly separating rows; result alone loses N, value IDs, row order and key identity. Retain every histogram whose computed residual equals the observed scalar, and every row sequence realizing it. Permuting categories can alter floating accumulation, so equality is target-bit specific rather than ideal-real equality.

## specialization

Histogram-preserving row permutations leave integer counts and canonical floating reduction identical, while permuting value IDs may reorder reduction. The 17-category oracle constructs counts directly without map grouping and compares 64 calls; analytic zero/one and empty endpoints are separate observations.

## composition

Shared gain ratios determine tree order and kNN weights. Perceptron reads the same Dict coordinates independently. Caller eval::fit_set binds one label/name vector per Field to common rows/keys; predict_seq overwrites prev-slot from its prior predicted act. That caller path is inspected here; this leaf executes Dict::build/encode -> fit -> predict on three supplied rows only. Child fragments partition production mechanics; test module remains at the file root. No parser/teacher/corpus/held-out/training-job execution.

## checks

Five native tests in model::wave13_models check canonical histogram arithmetic, literal one-update averaging, tree pruning/fallback, 80 kNN shell-label cases and repeated public fit observations. Exact names and evidence hashes are in root ../model.md#evidence. Two debug and two release runs must have equal normalized named outcomes. Negative production control changes shell admission and must fail a named assertion. No automatic inverse solver, general proof, calibrated probability claim or percentage-error approximant.

## epoch-memory

Initial scoped model. Prospective prediction identified randomized floating reduction in gain_ratio. Old implementation produced nine f64 values on 64 identical histogram calls. BTreeMap now fixes ascending category order; the expected grouping cost increases to O(N log V), with no timing claim. Other reviewed semantics were retained: perceptron averages initial state, tree pruning can lower confidence, and kNN explanations can extend beyond voters. Malformed state, huge counters, long-run floating accumulation, serialization durability and semantic accuracy remain unverified. All typed records bind the exact source hash in the manifest; they are partial source models supported by finite native checks, not universal proofs.

## History

### 2026-10-30 · epoch-memory

2026-10-30 initial scoped model. Prospective prediction identified randomized floating reduction in gain_ratio. Old implementation produced nine f64 values on 64 identical histogram calls. BTreeMap now fixes ascending category order; the expected grouping cost increases to O(N log V), with no timing claim. Other reviewed semantics were retained: perceptron averages initial state, tree pruning can lower confidence, and kNN explanations can extend beyond voters. Malformed state, huge counters, long-run floating accumulation, serialization durability and semantic accuracy remain unverified. All typed records bind the exact source hash in the manifest; they are partial source models supported by finite native checks, not universal proofs.
