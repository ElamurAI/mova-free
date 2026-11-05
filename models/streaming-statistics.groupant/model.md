# streaming-statistics

## model

# streaming-statistics

## forward

Summarize one accepted stream using bounded heavy-hitter counters, histogram rank/value intervals and online or merged moments.

## invariant

Accepted prefix counts agree under shared legal updates; counter, bucket and moment statistics retain different conservation and error relations.

## cost

Misra-Gries and Space-Saving O(k) worst update and O(k) state with deterministic BTreeMap order; histogram O(1) update/O(b) query/state, b<=10^6; moments O(1) update/merge/state.

## composition

Only shared successfully accepted inputs yield common N. There is no transaction across separate components. Exact counts, sorted ranks and direct finite mean/variance serve as independent fixture observations. The harness tests MG=SS jointly on 800 updates; histogram and moments use separate fixtures. A shared accepted-prefix execution across all four is not exercised.

## epoch-memory

Wave19. Models retain real-arithmetic assumptions separately from native finite checks. Kernels are library tools, not automatically adopted into training or inference.

## checks

["merged_moments_join_compensated_centered_sums","frequency_summaries_join_literal_histogram_certificates","histogram_intervals_join_sorted_rank_and_exact_counts"]

## limitations

Frequency error/N, quantile value/rank intervals and floating variance are different metrics. Moment arithmetic has no universal relative-error certificate. Map allocation failure lies outside the Result contract; histogram allocation is fallible.

## catalog

# Typed family relations

Each relation keeps its own domain and assurance.

## behavior-complement

Type: behavior_complement.

Exclude frequency intervals omitting the exact accepted-stream count, rank/value rectangles omitting the sorted requested rank, mismatched accepted-prefix N and successful nonfinite moment output. A rejected individual update must not mutate that component; no cross-component rollback is promised.

## reverse-reconstruction

Type: reverse_reconstruction.

Compatible streams lie in the intersection of frequency intervals, histogram bucket occupancies and moment constraints with execution consistency. Permutations and unrecorded identities/within-bucket values remain ambiguous; rounded moments may distinguish some permutations.

## shared-count

Type: accepted_prefix_transport.

Under legal updates for every component, MG.len=SS.len=histogram.len=moments.count=N. Rejection in one component invalidates this common-prefix premise unless caller coordinates updates.

## conservation

Type: stream_conservation.

MG counter sum+(k+1)*cancellations=N; SS estimate sum=N; histogram bucket sum=N; moments count=N. These equalities concern accepted updates and do not convert all output errors into one percentage.

## summary-loss

Type: information_loss_relation.

Frequency summaries omit most identities; histogram retains only bucket order/value intervals; moments retain low-order statistics. Equal low-order moments do not establish equal distributions or equal quantiles.

## wave19-evidence

### Family evidence

This family binds eight implementation/test sources through its manifest; views are partial source-derived models. Selected named joins are ["merged_moments_join_compensated_centered_sums","frequency_summaries_join_literal_histogram_certificates","histogram_intervals_join_sorted_rank_and_exact_counts"] in algorithms/tests/composition_wave19.rs. Those checks pass four times as part of the 616-check suite. Leaf oracles and compiled production controls remain separately bound in adjacent evidence.

[Integrated evidence](../algorithm-library.groupant/model.md#wave19-evidence) records receipts; conditional laws outside these fixture scopes are not promoted to executed checks. No joint universal percentage bound follows from family membership.

## History

### 2026-10-30 · epoch-memory

Wave19, 2026-10-30. Models retain real-arithmetic assumptions separately from native finite checks. Kernels are library tools, not automatically adopted into training or inference.

### 2026-10-30 · wave19-evidence

Heading `### Family evidence — 2026-10-30` stays in wave19-evidence as `### Family evidence`.
