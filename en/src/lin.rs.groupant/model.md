# en-lin

## descriptant

### lin

Linearizer stores side counts and interpolated bigram/context counts. PunctTable stores ordered signatures and a process-global first-set OnceLock.

Train tagged sentences into five side key levels and three Markov levels. left chooses first key with count>=3 and left>right; otherwise right. order groups heads clamped to n, recursively emits left subtree/head/right subtree. sort_side maximizes interpolated log-probability with subset DP for k<=10, greedy next-link for k>10. punctuate uses learned majority when known else structural rules.

Partial source-bound model, not full proof. No useful executable percentage surrogate established for these categorical decisions/effects; budgets1/2/5/10% require a future observation metric. No corpus/API/GPU/teacher, allocation fault injection, adversarial size exhaustion or broad accuracy claim.

## approximant

### Forward relation

Train tagged sentences into five side key levels and three Markov levels. left chooses first key with count>=3 and left>right; otherwise right. order groups heads clamped to n, recursively emits left subtree/head/right subtree. sort_side maximizes interpolated log-probability with subset DP for k<=10, greedy next-link for k>10. punctuate uses learned majority when known else structural rules.

Domain: compatible parallel arrays and valid symbol indices; API-specific omissions/invalid-input behavior in model.md#models. Finite machine arithmetic follows Rust source; no real-number replacement guarantee.

## invariant

### State and cost

Linearizer stores side counts and interpolated bigram/context counts. PunctTable stores ordered signatures and a process-global first-set OnceLock.

Tied side counts cannot choose left. PunctTable total<3 or best*2<=total cannot override rules. OnceLock cannot be replaced by later set calls. order does not guarantee permutation for invalid cyclic/disconnected inputs or depth>200.

Child grouping O(n); repeated construction/heavy scans can be quadratic at high degree. Each side degree k<=10 costs O(2^k*k^2) time/O(2^k*k) memory; k>10 greedy O(k^2), not exact optimum. Emission depth bounded201. u32 count arithmetic is unchecked.

## composition

### Scoped composition

Parent includes helpers, public API and test declarations; fragments below focus on distinct mechanisms and share the whole-file partial lexical projection. They are not separately registered sources.

- ordering: bytes[4903,12628) delegates this source's mechanism; forward composition: Train tagged sentences into five side key levels and three Markov levels. left chooses first key with count>=3 and left>right; otherwise right. order groups heads clamped to n, recursively emits left subtree/head/right subtree. sort_side maximizes interpolated log-probability with subset DP for k<=10, greedy next-link for k>10. punctuate uses learned majority when known else structural rules.
- punctuation: bytes[12628,18602) delegates this source's mechanism; forward composition: Train tagged sentences into five side key levels and three Markov levels. left chooses first key with count>=3 and left>right; otherwise right. order groups heads clamped to n, recursively emits left subtree/head/right subtree. sort_side maximizes interpolated log-probability with subset DP for k<=10, greedy next-link for k>10. punctuate uses learned majority when known else structural rules.

## models

## behavior_complement

Tied side counts cannot choose left. PunctTable total<3 or best*2<=total cannot override rules. OnceLock cannot be replaced by later set calls. order does not guarantee permutation for invalid cyclic/disconnected inputs or depth>200.

## reverse_reconstruction

An output ordering admits all tree/side/chain table states whose local choices emit it; many weights preserve the same order. PunctTable write/read recovers represented map state but malformed lines are skipped and duplicate keys overwrite. Order alone cannot recover original sentence order or counts.

## side_ties

MIN3 chooses highest-specificity sufficient key, equality goes right. explain reports None/(0,0) on complete miss, levels4..0 otherwise.

## chain_ties

DP updates only on strict score increase, final max_by prefers later equal endpoint; greedy max_by prefers later equal candidate. Uniform floor0.05/64 keeps log finite for valid nonnegative finite counts.

## punctuation_majority

Some(empty) is a learned no-mark override, distinct from None fallback. Predict requires strict majority and at least3 observations.

## tree_domain

Heads are1-based root0; invalid high heads clamp to n for order/learn, but punctuation key directly indexes head tag and may panic. Depth cutoff emits head without descendants.

## tests

### Evidence

Compiled source mutants cover construct rounded-floor boundary, parse root legality and store checksum gate. Other sources have no compiled source mutation claim. Preflight empty-model nonempty tagging failed in ptag; synthetic fixtures adjusted without production changes. Existing corpus/seeds tests not selected.

Partial source-bound model, not full proof. No useful executable percentage surrogate established for these categorical decisions/effects; budgets1/2/5/10% require a future observation metric. No corpus/API/GPU/teacher, allocation fault injection, adversarial size exhaustion or broad accuracy claim.

## prospective

### Prospective structure model — before edits

Read all nine sources. Annotation composes tagging, parse, repair, child flags, UPOS, two-pass features and morphology; complete and kbest omit repair. Save packs before mkdir/write/rename; no fsync or concurrent-writer guarantee. Graph encoding casts heads to u16 and local symbols deduplicate by linear search; lossless preserves token spacing flags, not arbitrary original whitespace. Explanation separately runs tag/parse and encode: duplicated work and no repair equivalence claim.

Parser maintains root-indexed stack/buffer, attachment and saturating child counts. Legal root attachment waits for exhausted buffer; 2n transitions; greedy ties favor later ACTS, beam ties stable with gold lower. Width zero clamps to one at step. Constraints require compatible sizes/heads; no input validation. Sparse table restores weights, not averaging accumulators. Linearizer uses MIN=3 side backoff, right on ties, subset DP through degree10 then greedy; depth>200 truncates traversal. Invalid forests/cycles may omit nodes. Punctuation first OnceLock wins, majority strictly >half.

UD child flags scan quadratically and propagate conjunction flags with bounded chains. UPOS/FEATS need count2, lower enum tie; sentence features refine verbs left-to-right unless EN_AGR=0. Scores compare base labels, content F1, and sorted function children. Expert binds distinct nodes, checks all require clauses then any unless; missing references rejected, unclosed blocks omitted. Construct loader checks exact token spelling, but allows root-0 dependent: recall requires d>=1 and sufficient tree length; pct zero denominator clamps to1, report compares rounded percentage strictly below floor. Store validates magic/size, version, enum shape, dictionary, length, checksum, then typed body; varint decoding is bounded but not canonical and high bits in tenth byte are not independently rejected.

Tests will use synthetic in-memory fixtures only: nine focused contracts, no corpus, GPU, API or live training. Model::train_with(empty,1) is bounded zero-data initialization. Compiled mutations target construct threshold, parser root legality and store checksum rejection; failures and restored positives retained. Expected costs: expert O(n falling-factorial m times predicate/require work), parser beam O(n*b*(features*labels+n)+n*b log b) due cloned configs; linearizer sum O(2^k*k^2) for k<=10, quadratic greedy otherwise; store linear bytes plus typed allocation; graph local intern worst quadratic strings. No executable percent surrogate is justified for these categorical/effectful APIs; keep explicit gap. Models and syntax remain partial.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: source read before tests/models. Synthetic checks characterize a small boundary, not whole NLP quality. Preflight empty-trained tagger panicked on nonempty input; model/explain fixtures now use two synthetic one-token sentences. This is an unmodified dependency/domain gap.

### 2026-10-31 · tests

Actual source test: lin::wave27::bounded_contract. Command: cargo test -p en --lib wave27 -- --test-threads=1 (debug twice, release twice). External evidence: $MOVA_DATA/runs/octopus-wave27-structure-2026-10-31. Nine focused tests, each in its owned production module; no copied implementation.
