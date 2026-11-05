# world-bookans

## model

# world-bookans

## forward

Normalize alphanumeric/apostrophe words; compare hashed seven-word windows; independently flag matching quote pairs with >=4 words. Retrieval sums twice index term occurrences plus distinct lexical hits, ranks (-score,index), takes30, expands +/-1, returns up to90 ascending indices. Invalid index positions are skipped before scoring. Teacher/rewrite/log path is read-only effect modeling, not executed correspondence.

## invariant

Supplied worlds require valid one-based event sentence anchors and consistent entity/state references. Book retrieval returns unique increasing in-range zero-based indices. Answer helpers do not mutate worlds. Copy gate acceptance establishes neither factual correctness nor a general copyright guarantee. Partial syntax is not semantic analysis.

## cost

B normalized book bytes and A normalized answer bytes: expected fixed-width-window hash construction O(B), membership scans O(A), plus emitted bytes. Retrieval reads index/sentence bytes, builds ordered sets/maps and sorts K scored indices O(K log K).

## composition

Normalization feeds hashed membership and independent quote scanning. Retrieval combines index/lexical evidence before ranking and expansion. run feeds notes to prompt and at most two teacher calls, gates responses, then logs after printing. Query annotation, sentence splitting, teacher transport, environment mutation and log errors remain unmodeled semantic/effect dependencies. Local evidence does not certify that path.

## epoch-memory

Initial wave8 source review and native checks. Preserve ordering, ambiguity and external dependency gaps. No useful approximation metric is supplied for categorical dispatch, so executable Rust approximants remain absent. Tests are not fast surrogates. See root model.md#evidence for repeated profiles and compiled controls.

## checks

cargo test -p world --lib wave8_models -- --test-threads=1; repeat with --release before -p. Only named native helpers exercised; no live parser/teacher. Fragments use whole-file partial syntax projections plus source byte envelopes, not inferred types, expanded macros or semantic AST.

## behavior-complement

Universe U is admissible supplied inputs paired with categorical observations of scoped relation R below. The source-derived inner exclusion hypothesis rejects observations violating local priorities, thresholds or constructor outputs. Unmodeled branches and dependencies remain unknown; U minus a partial model is not a certified complement of the entire Rust file.

## reverse-reconstruction

B(y)={x in the declared domain:R(x,y)} retains all compatible latent tokens, scores, ties and states. Equal text or score does not identify a unique input/history. This is a relational specification, not an inverse solver. Native finite fibers give scoped evidence, not completeness outside their domain.

## hash-fiber

Only u64 hashes retained: collisions and distinct books with same window set are compatible. Adjacent matching hashes can originate from different book positions, so emitted run need not be one contiguous source passage.

## quote-boundary

Straight/curly/guillemet scans are independent. Three words pass, four fail; unmatched opening ignored. Book membership unnecessary for quote rejection.

## ranking-cut

Top30 precedes expansion; output is source order, not score order. On 100 equally scoring sentences top0..29 expands to0..30.

## normalization-fiber

Case and boundary punctuation variants yield identical normalized words. Interior apostrophe retained; edge apostrophes trimmed. Different from ans toks.

## retry-effects

At most two teacher responses. Success prints before log append; logging error may follow visible output. Teacher errors propagate. Word limit exists only in prompt.

## invalid-index

IDs>=sentence count ignored before ranking/arithmetic, including usize::MAX and all IDs for empty corpus. File-read errors propagate. Valid rows retain weights. Unbounded occurrence-count overflow outside fixtures.

## evidence

### Book helper evidence

Gate checks cover prefix lengths 0..8, normalization fibers, one extended copy run, three quote styles, three/four-word boundary and unmatched quotes. Retrieval checks enumerate all 64 hit masks across six sentences against an independent neighbour-distance oracle. Equal-score 100-sentence fixture predicts indices 0..30 after top30 plus expansion. Empty/no-term domains and prompt note order/IDs are explicit fixtures.

Prediction from source review: out-of-range event-index IDs can overflow neighbour arithmetic, consume ranking slots or leak final context. Regression with usize::MAX, index equal to count, valid index zero and empty sentences failed on old production code (`bug-before.log`: add overflow). Local fix skips IDs >= sentence count before score insertion; regression now returns [0,1] for the valid row and empty for no sentences. Valid-row weighting is unchanged. Temporary TSV is process-specific and removed on successful regression execution.

Compiled independent production control changes COPY_RUN from 7 to 8; `gate_threshold_and_normalization_fibers` fails its named threshold assertion (`bookans-mutant.log`). Source restored and final eleven tests pass.

No teacher/annotation calls. Gate hashes may collide; consecutive matching answer windows need not come from one contiguous source passage. Gate success does not prove factual grounding, word-limit compliance or general copyright compliance. Recommended parent boundary: book-body/sentence segmentation -> annotation terms -> indexed retrieval -> prompt -> teacher -> gate/retry -> print/log. The print-before-log failure ordering remains an explicit unexecuted effect model.

#### Ranking-crowding refinement

Compiled control moves the range check from before score insertion to after top-thirty selection. Earlier overflow/empty-domain observations still hold, but the new crowding assertion fails: selected [] instead of [0,1,2]. `crowding-control.log` verifies named failure and exit 101; `crowding-mutant.log` retains raw diagnostics. Source restored; all six focused tests pass again (`crowding-restored.log`). Production behavior is unchanged in this refinement. Current source/syntax bindings and all fragment hashes are refreshed; syntax repeats identically and isolated structural validation passes.

## History

### 2026-10-30 · document

Partial source model, 2026-10-30. Each typed view shares this source revision, scope, assumptions and checks unless narrowed below. Observation is one finite helper call, not full conversation. Source-derived relations remain hypotheses supported by bounded correspondence; no general semantic proof.

### 2026-10-30 · evidence

Heading `### Book helper evidence — 2026-10-30` stays in evidence as `### Book helper evidence`.

Six bookans native tests and five ans tests pass twice debug and twice release with filter `wave8_models`, single thread. Restored debug also passes eleven. Normalized successful test identities/results SHA-256: `cf53c00ccc34b8a9cceb369b05c4253f2f9efc486fe7d2f69b5a7c15ee1fe67c`. External run label `octopus-wave8-answers-2026-10-30` retains raw profile/control logs. No durations are promoted to throughput claims.

Current source SHA-256: `c070e111a9244f3f3066d4bf2652f207b912282b82a62c64a17363a8d4c380ab`. Current syntax SHA-256: `dcfa9c8c9f3668228987686576b04cfe79dff995e347bd401f8af89063743a3a`. The initial eleven-test evidence above predates the additional crowding fixture; its source hash was `9505299221edb77a81d460b918212a3707951be7c06dafce151a2b84dc9a3c3e`. Two syntax generations match. Root eight typed views plus three four-view fragments total twenty. Isolated exact-source groupant validation reports zero diagnostics; parent handles full registry/ancestor integration.

The existing invalid-index test now supplies forty invalid rows scoring six each and a valid row scoring two. It asserts indices [0,1,2], so invalid rows cannot consume the top-thirty budget. Test count stays six. Twice-debug and twice-release focused runs under `bookans::wave8_models` pass with identical normalized identities/results: SHA-256 `0c4050ad5293e5714c8c2fe8e4b35a2e3eab5e07c7fb53ab7f83bbd44b361efe`. Logs use `crowding-debug1.log`, `crowding-debug2.log`, `crowding-release1.log`, `crowding-release2.log` in the same external run.
