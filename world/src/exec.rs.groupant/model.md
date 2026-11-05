# execution-dispatch

## descriptant

# execution-dispatch

## Forward

Initialize mode once; CPU parses through ordered workers; GPU composes cached load, tokenize, nonempty gather, predict, shape guards, complete, repair, lowercase and scatter.

## Invariant

Successful batch cardinality/order equals input. Pure deterministic closures and fixed parser/environment are required for value determinism; arbitrary closure effects are not deterministic.

## Cost

For n jobs, total work is sum of job costs plus O(n+t) aggregation; ideal span is largest chunk cost plus joins. GPU adds loading, model work and O(tokens) intermediate storage; no measured speedup.

## Composition

Workers child supplies the ordered/barrier relation; parser child supplies mode/backend state. Compose through ordered positions and Result boundaries, not by unioning complements. Reverse composition retains all intermediate batches. Parent inference-core must bind parser/world integration.

## Epoch memory

The native correspondence child now distinguishes values, callback visits, owned-output destruction and hidden callback state. It does not inherit parser/GPU claims from the other child. Its finite preimages constrain individual values and preserve cap ambiguity; error strings omit panic locations and payloads.

## catalog

# execution-dispatch: typed catalog

## nonbehavior

Type: behavior_complement. Domain, observation and operator: U consists of (build, environment history, texts, tagged Ok/Error/Exit outcome). R selects the latched mode and executes its pipeline. U minus R includes successful truncated returns after worker panic, successful GPU dispatch in a CPU-only build, and switching mode after a valid latch. Resource aborts and parser quality are outside this bound.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Subprocess mode matrix plus injected worker failures.

## antecedents

Type: reverse_reconstruction. Domain, observation and operator: For Ok word batches y, R^-1(y) contains all texts and parser states mapping to y, with input count equal to output count. Casing, ambiguity and approximate GPU outputs prevent unique text recovery.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Enumerate a noninjective toy map and compare full fibers.

## ordered-map

Type: order_preservation. Domain, observation and operator: For pure f, positive cap and completed jobs, y[i]=f(x[i]); joining and flattening follow source slice order regardless of completion order.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Existing 101-value and empty-map test.

## join-barrier

Type: failure_partition. Domain, observation and operator: All spawned handles are joined before Result collection; any join panic produces Err. Already completed side effects survive. Public par_map converts Err into a panic.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Existing all-worker panic test; mixed outcome counter proposed.

## thread-cap

Type: resource_bound. Domain, observation and operator: Let t=min(available_parallelism or 4,cap)>0 and c=max(ceil(n/t),1). Worker count is ceil(n/c) for n>0 and zero otherwise. cap=0 is outside the helper model.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Enumerate lengths and positive caps.

## mode-latch

Type: state_transition. Domain, observation and operator: Empty OnceLock takes the first accepted environment mode; later environment edits cannot change that stored mode. GPU requires a feature-enabled build.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Isolated process with environment changed after first call.

## mode-failure

Type: failure_preimage. Domain, observation and operator: Before a mode latch, exit(2) corresponds to a Unicode value other than cpu/gpu or gpu without feature. Missing or non-Unicode environment lookup errors choose CPU.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Subprocess exit-status matrix.

## cpu-exact

Type: dependency_equivalence. Domain, observation and operator: With the same annotator and no worker panic, CPU result is the sequence Tree::parse_cased(a,text).w. This says nothing about parse correctness or GPU agreement.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Compare sequential parsing to batching with fixed annotator.

## gpu-shape

Type: cardinality_constraint. Domain, observation and operator: GPU success requires predicted sentence count equal to nonempty-token inputs and per-sentence token count equality; scatter preserves original empty sentence positions.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Stub malformed prediction dimensions.

## gpu-labels

Type: information_loss. Domain, observation and operator: Unknown tags fall back to NN; relations try full label, then pre-colon base, then Dep. Distinct raw labels may therefore yield identical completed outputs.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Enumerate invalid labels and relation subtypes.

## gpu-cache

Type: state_fiber. Domain, observation and operator: GPU OnceLock stores the first Result of model load; failed initialization remains failure even after changing model path. Annotator acquisition precedes cache lookup.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Instrument a failing loader and alter path.

## repair-normalize

Type: normalization_fiber. Domain, observation and operator: GPU finish repairs when WORLD_NO_REPAIR lookup errors, then lowercases form and lemma. Compatible input fibers include alternative case and pre-repair structures; switch is read per finish.

Approximation: source-derived conditional specification; behavioral complements are candidate inner bounds of absence under the explicit assumptions. Reverse relations retain every compatible antecedent in the stated domain and do not imply unique recovery. Specialized observations omit unmentioned state and do not establish full semantic correspondence.

Falsifiable check: Controlled case and repair variants.


## tests

### Checks

Existing focused check: cargo test -p world --lib exec::tests -- --nocapture. Named tests: batches_preserve_order_and_cardinality (101 mapped values and empty input); worker_panics_are_errors_instead_of_partial_results (four panicking jobs). The file-root model.md#evidence records actual execution. Mode caching, exit behavior, mixed worker outcomes, GPU shape guards and GPU/CPU agreement remain untested here.

No training, live service or GPU execution; no speedup or broad semantic correctness claim.

## evidence

### Executed evidence

CPU-only command: `cargo test -p world --lib exec::tests -- --nocapture`, with a separate Cargo target directory. Result: 2 passed, 0 failed, 131 filtered out. The panic test deliberately produces four worker panic messages, then passes after the helper returns an error. This tests ordered mapping on 101 integers, empty mapping and all-worker failure; it does not establish mixed-worker completion, arbitrary closure determinism or parser accuracy.

The unchanged-registry structural command `mova-modelcheck groupants . models/contracts.json NEW_JSON` passes with no diagnostics and verification_inferred=false. It discovers these sidecars without registering their Rust sources yet. The complete registry-node.json is supplied for the integrating parent to attach to inference-core. Shared registry and group manifests are intentionally owned by the integrating parent.

Tree: execution-dispatch has workers and parser children. Twelve root models plus four per child = twenty typed models. Baseline complement precedes reconstruction at every node. No AST projection, mode subprocess test, GPU run, external service call or training was performed. Catalog falsifiers beyond the two named existing tests remain prospective.

## worker-laws

# Native worker laws

## Forward (worker-laws)

For positive cap k, nonpanicking extensional f and input vector x, the returned vector is map(f,x) in original index order. The host worker count affects partitioning, not this value equation. The existing helper intentionally models only positive caps. No production policy change is proposed.

Prospective prediction: bounded map/fiber fixtures, mixed panic frontiers and dropped-output accounting match these laws. Pure callback values are deterministic; callback effects and arbitrary stateful outputs need separate observations. Tests use closures only, never the parser or GPU.

## product-fibers

Type: pointwise_preimage_product. Fix D={0,1,2,3}, f(x)=x mod 2, and output y of length n<=3. The complete preimage in D^n is the Cartesian product of {y_i,y_i+2} when every y_i is 0 or 1; an out-of-image coordinate has empty preimage. Its size is 2^n for reachable outputs and zero otherwise. Input order remains visible but each value's upper bit and thread cap do not. Native calls enumerate the whole finite input domain for positive caps one and three; an independent bit-choice constructor supplies expected fibers. This is finite correspondence, not a general inverse executor.

## callback-frontiers

Type: per_worker_effect_prefix. Fix available_parallelism (fallback four), positive cap, n and chunk size c=max(ceil(n/t),1). Under ordinary unwind without spawn failure, each chunk invokes callbacks in index order until its first injected panic, inclusive. Other workers retain their own prefixes. All joins complete before Err returns. Err hides successful outputs, panic indices and payloads; executed effects survive. Distinct failed chunks can share an error string. Tests sample first/last chunk failures and check every visit; arbitrary external callback effects and aborting panics are outside this domain.

## discarded-output-accounting

Type: owned_output_failure_frame. Every successful instrumented callback creates one owned output with a nonpanicking Drop counter. On worker failure, all constructed outputs drop before Err returns, including values produced in other completed chunks. Created equals dropped, but zero returned outputs does not imply zero callback work. This counts ownership effects, not allocator bytes, leaks in arbitrary callbacks or destructor panics.

## stateful-observation-boundary

Type: callback_state_projection. Fn+Sync does not imply referential transparency. An atomic ticket callback can expose hidden state while output positions follow input order. Different initial counters yield disjoint ticket ranges for identical inputs; returned (input index,ticket) retains its input index and the ticket multiset is contiguous. No within-range ticket order or schedule is asserted. Ordered output does not establish deterministic semantics for every permitted callback.

## Cost and invariant

Each finite call pays callback work plus O(n) buffering and joins; enumeration repeats this for every antecedent/cap. Failure tests predict callback counts from contiguous slices independently of completion order. Tests use positive caps, ordinary unwind, nonpanicking atomics/destructors and stable available_parallelism during each measurement. Source-bound partial syntax is not a concurrency proof.

## History

### 2026-10-29 · Epoch memory

2026-10-29: first source-bound instance. Source SHA-256 180f88005a4cd3852f649059e420951c1fe9ad4a699c9d175358169f25e43b4d; bytes [0,12143). Production source is unchanged. Ranges are snapshot anchors, not proof of coverage. AST gap: no projection is materialized; macro expansion, type inference, semantic dependency analysis and generated proof remain absent. Catalog oracles are prospective unless explicitly recorded as executed. Counts are viewpoints, not correctness evidence.

### 2026-10-29 · document

Epoch: 2026-10-29. Subject: exec.rs, SHA-256 180f88005a4cd3852f649059e420951c1fe9ad4a699c9d175358169f25e43b4d, bytes [0,12143). Every entry is a partial source-derived specification within its stated observation domain. Fixed dependencies and successful ordinary allocation are assumptions. Transition horizon is one invocation unless a history is explicitly named. Unsampled behavior remains unknown; no semantic proof is claimed. Checks below are prospective unless model.md#evidence records execution.

### 2026-10-29 · evidence

2026-10-29, base commit 5794fd531c61d5b66e53eb222abe3c75d082fc50.

Source SHA-256: 180f88005a4cd3852f649059e420951c1fe9ad4a699c9d175358169f25e43b4d.

### 2026-10-30 · tests

#### Native worker correspondence, 2026-10-30

Six exec tests pass twice in debug and twice in release with identical normalized named outcomes. Four new tests enumerate 170 modulo-map calls and 80 complete fiber comparisons, mixed panic-prefix visits, constructed/dropped output counts and stateful callback tickets. Pure callbacks preserve pointwise values across scheduling; Fn+Sync by itself does not guarantee purity. Positive caps and stable available_parallelism are explicit premises.

A standalone harness extracts the actual two worker functions and the same four tests from the current source. Its reference passes; reversing iteration inside each chunk compiles and causes the selected fiber test to fail with exit 101. This control has no parser/GPU dependencies; native world tests separately execute the real module. CPU/GPU parsing, mode latch and model loading are not tested by these fixtures. Execution mechanics are unchanged; only the public comment was scoped to pure callbacks.

The source hash and existing fragment byte ranges were refreshed after that comment changed; transformed fragment bytes were checked against their original slices. Root partial syntax was generated twice identically. A new correspondence fragment binds the test module separately. Raw evidence: $MOVA_DATA/runs/octopus-algorithms-wave5-2026-10-30/.

### 2026-10-30 · evidence

#### Native worker correspondence, 2026-10-30

Six exec tests pass twice in debug and twice in release with identical normalized named outcomes. Four new tests enumerate 170 modulo-map calls and 80 complete fiber comparisons, mixed panic-prefix visits, constructed/dropped output counts and stateful callback tickets. Pure callbacks preserve pointwise values across scheduling; Fn+Sync by itself does not guarantee purity. Positive caps and stable available_parallelism are explicit premises.

A standalone harness extracts the actual two worker functions and the same four tests from the current source. Its reference passes; reversing iteration inside each chunk compiles and causes the selected fiber test to fail with exit 101. This control has no parser/GPU dependencies; native world tests separately execute the real module. CPU/GPU parsing, mode latch and model loading are not tested by these fixtures. Execution mechanics are unchanged; only the public comment was scoped to pure callbacks.

The source hash and existing fragment byte ranges were refreshed after that comment changed; transformed fragment bytes were checked against their original slices. Root partial syntax was generated twice identically. A new correspondence fragment binds the test module separately. Raw evidence: $MOVA_DATA/runs/octopus-algorithms-wave5-2026-10-30/.
