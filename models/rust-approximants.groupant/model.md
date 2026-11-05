# rust-approximants

## model

# Executable approximation group

## Forward

The shared budget selects a maximum error from {0.01,0.02,0.05,0.10}. Each kernel implements a scoped relation R(x,y,b): scalar relative error for roots, byte edits and range minima; rank displacement divided by input length; objective loss for feasible knapsack values and valid LIS witnesses; or positive-cell L0/S1 probability relative error with exact zero cells. Two runners bind these implementations to independent reference observations and report error separately from timing.

## Invariant

Budgets are upper bounds. Zero error is permitted in every profile. No result may trade an error budget for broken input validation, infeasible capacity, invalid probability support or silent effects. Exact fallback is explicit in the selection, knapsack, byte-edit and LIS results and the range model state. RSA rejects its unsupported domain. These surrogates do not replace production APIs.

## Cost

Source-derived predictions: reduced-root constant work replaces integer binary search; rank sampling still needs a full certification scan and may pay for exact selection; certified knapsack costs O(n log n), otherwise adds O(n*C); bounded L0 replaces rational reductions with integer sums and one reciprocal per supported row. The runner includes allocation, validation and certification, excludes compilation, alternates timing order and records repeated samples. Fallback overhead can reverse a gain.

## nonbehavior

Universe U consists of typed input/budget/output/evidence tuples for the eight observations. The complement of the accepted relation contains unsupported budgets, invalid domains, invalid metric values and outputs exceeding their declared bound. This is a scoped relation, not an assertion that arbitrary unmodeled behavior has been excluded. A rank estimate paired with a partition claim is outside the modeled relation even if its rank error is zero.

## compatible-evidence

Given output y, budget b and measurement e, compatible inputs are the union of each kernel's metric-specific preimage. Quantized roots admit intervals, rank values admit many input multisets, knapsack values admit many feasible subsets and L0 rows admit common-scale prior families. Finite timing/error aggregates cannot reconstruct a unique input, implementation or proof.

## metric-domain

Error percentages have different denominators and cannot be substituted across kernels. A zero scalar reference allows only an exact zero approximation in the shared relative metric. Zero optimum implies zero objective loss. Zero probability cells must remain zero. Rank normalization is by n on the nonempty valid-index domain; it does not measure numerical value displacement.

## composition

The runner is a product of separately checked observations, not a fused Mova computation. It asserts no joint 10% system bound. For future numerical compositions with a declared downstream Lipschitz constant L, absolute error propagates as L*e+d; this needs a valid invariant domain and does not directly apply to relative error, branching, recursive RSA or discrete language answers.

## dispatch-cost

For acceptance probability a on a named fixture population, expected work is T_certificate + a*T_fast + (1-a)*T_fallback. This is a cost planning model, not a universal distribution. Timing records must expose fallback frequency. A worse worst-case cost remains possible even with a faster certified path.

## source-evidence

Every executable attachment binds the exact source, Rust surrogate and evidence bytes. The structural checker detects stale bindings and invalid declared tiers; executable comparisons detect tested numerical failures. Neither gate alone establishes source equivalence, universal bounds or a full-system speedup. The ordinary algorithm branch remains the authority for exact production algorithms.

## tests

### Executable checks

Run `cargo test -p mova-approx --example groupants` for the attached implementations and their direct reference comparisons; `cargo test -p mova-approx` covers the shared budget/metric contract. Run the same commands with `--release` for optimized execution. `cargo run --release -p mova-approx --example groupants` emits separate deterministic check records and timing records. Two independent runs must agree on the check records; durations are observations, not deterministic outputs.

The existing `cargo test -p mova-algorithms` suite checks the exact library. Modelcheck tests include rejected attachment hashes, unsupported caps, invalid fields and stale evidence. `mova-modelcheck groupants REPO models/contracts.json NEW_JSON` validates the final tree and source bindings. None of these finite checks is a universal proof.

Each surrogate's adjacent evidence records its domain controls and mathematical argument. The integrated runner additionally compares actual RSA `Game::l0`, rejects corrupted probability outputs, verifies rank displacement independently, and exercises certified and exact-fallback paths. Compiled zero-root, bypassed rank certificate, bypassed knapsack certificate and reversed shared-budget mutants were rejected by their focused tests during development.

Integration measurements are recorded in the public executable-approximant documentation; raw outputs stay in an external run directory.

## History

### 2026-10-29 · document

#### Epoch memory — 2026-10-29

The first four kernels deliberately use different useful metrics. The literal-listener approximant is more accurate than every requested cap; introducing noise to fill the wider bands would waste accuracy. Standard-library exact root and selection baselines are included in the integrated runner so gains over a simple library implementation are not confused with gains over every exact algorithm.

### 2026-10-30 · document

#### Classification contract — 2026-10-30

Every executable profile has a metric_kind, method_kind and assurance_kind. The class determines which error observation is being discussed; it is not proof of that observation. Analytical declarations use analytical or per_input_certificate assurance; empirical declarations use finite_observation. Supported caps are exactly 1%, 2%, 5%, 10%; larger values, missing classes and incompatible assurance labels fail structural validation. The Rust certificate rejects values above the selected cap, including the f64 immediately above 10%. New measurements and explicit fallback regressions are recorded in docs/mathematical-modeling.md.

#### Extended interval and speaker models — 2026-10-30

Four additional observations cover byte edit distance, strict subsequence length with a valid witness, scalar range minima, and bounded pragmatic S1 probabilities. The first three use interval_certificate with per_input_certificate assurance. They derive conservative intervals for the exact objective and either certify a result or invoke the reference. S1 evaluates its formula directly within a checked finite domain; no recursion-wide RSA error bound is inferred.

The edit-distance preimage permits strings and alphabets with compatible histogram bounds and residual edits. LIS preimages preserve valid strict index/value order while permitting many equal-length witnesses. Range preimages permit different arrays sharing min/max/length and branch choice; the returned scalar need not be a member of the requested interval. S1 preimages remain ambiguous under prior scaling and indistinguishable truth/cost structures. These are separate reverse relations, not one invertible system.

Cost predictions and repeated Rust measurements are in docs/rust-approximants-next.md. Compact-range queries reuse a built state; preprocessing and queries are reported separately. Fallback overhead is included. New runner checks compose original references, scoped metrics and invariant gates without asserting a joint whole-system 10% bound.

### 2026-10-30 · tests

2026-10-30 extension: `cargo test -p mova-approx --all-targets` runs 71 approximation tests across the shared contract and both runners. `groupants_next` supplies 34 of these, including embedded reference tests. Two release measurements produced identical eight CHECK and 16 CLASSIFICATION records and 40 timing pairs each. The next-kernel documentation records favorable paths, fallback regressions and the checked S1 domain. The extended manifest admits only the declared interval-certificate classification; an over-cap profile, stale Rust hash and incompatible assurance were all rejected in integrated controls.
