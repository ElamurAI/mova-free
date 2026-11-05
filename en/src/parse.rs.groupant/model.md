# en-parse

## descriptant

### parse

Parser holds dense perceptron type/label tables, labels in encounter order, sorted known-word ids, training clock and beam. Config holds root0 stack, next buffer, heads/labels, extrema and saturating child counts.

Legal arc-standard transitions shift next buffer, Left attaches second-top under top, Right attaches top under second-top. Root is never Left dependent; Right to root waits for buffer exhaustion. Greedy maximizes valid type scores then label scores. Beam expands valid actions, sorts path scores, truncates max(width,1), labels greedily except gold/forced arcs. Constrained search returns None when beam dies. kbest deduplicates trees.

Partial source-bound model, not full proof. No useful executable percentage surrogate established for these categorical decisions/effects; budgets1/2/5/10% require a future observation metric. No corpus/API/GPU/teacher, allocation fault injection, adversarial size exhaustion or broad accuracy claim.

## approximant

### Forward relation

Legal arc-standard transitions shift next buffer, Left attaches second-top under top, Right attaches top under second-top. Root is never Left dependent; Right to root waits for buffer exhaustion. Greedy maximizes valid type scores then label scores. Beam expands valid actions, sorts path scores, truncates max(width,1), labels greedily except gold/forced arcs. Constrained search returns None when beam dies. kbest deduplicates trees.

Domain: compatible parallel arrays and valid symbol indices; API-specific omissions/invalid-input behavior in model.md#models. Finite machine arithmetic follows Rust source; no real-number replacement guarantee.

## invariant

### State and cost

Parser holds dense perceptron type/label tables, labels in encounter order, sorted known-word ids, training clock and beam. Config holds root0 stack, next buffer, heads/labels, extrema and saturating child counts.

A legal nonempty completed parse has n arcs and exactly one root attachment. No transition attaches root as dependent. Greedy label ties choose latest encountered label, not lexical order. Constraints do not guarantee existence with finite beam; nonprojective training may stop/skip. Tables do not validate all resource dimensions.

2n legal transitions; greedy scores O(n*F*(3+R)). Beam adds candidate sorting O(n*b log b) and clones O(n) configuration/path per surviving step, giving O(b*n^2) clone work. Constraints scan n for child readiness. Dense deserialization allocates rows*k with unchecked arithmetic.

## composition

### Scoped composition

Parent includes helpers, public API and test declarations; fragments below focus on distinct mechanisms and share the whole-file partial lexical projection. They are not separately registered sources.

- weights: bytes[1153,5917) delegates this source's mechanism; forward composition: Legal arc-standard transitions shift next buffer, Left attaches second-top under top, Right attaches top under second-top. Root is never Left dependent; Right to root waits for buffer exhaustion. Greedy maximizes valid type scores then label scores. Beam expands valid actions, sorts path scores, truncates max(width,1), labels greedily except gold/forced arcs. Constrained search returns None when beam dies. kbest deduplicates trees.
- configuration: bytes[5917,10059) delegates this source's mechanism; forward composition: Legal arc-standard transitions shift next buffer, Left attaches second-top under top, Right attaches top under second-top. Root is never Left dependent; Right to root waits for buffer exhaustion. Greedy maximizes valid type scores then label scores. Beam expands valid actions, sorts path scores, truncates max(width,1), labels greedily except gold/forced arcs. Constrained search returns None when beam dies. kbest deduplicates trees.
- decoding: bytes[31344,39385) delegates this source's mechanism; forward composition: Legal arc-standard transitions shift next buffer, Left attaches second-top under top, Right attaches top under second-top. Root is never Left dependent; Right to root waits for buffer exhaustion. Greedy maximizes valid type scores then label scores. Beam expands valid actions, sorts path scores, truncates max(width,1), labels greedily except gold/forced arcs. Constrained search returns None when beam dies. kbest deduplicates trees.

## models

## behavior_complement

A legal nonempty completed parse has n arcs and exactly one root attachment. No transition attaches root as dependent. Greedy label ties choose latest encountered label, not lexical order. Constraints do not guarantee existence with finite beam; nonprojective training may stop/skip. Tables do not validate all resource dimensions.

## reverse_reconstruction

For fixed words/tags and resulting tree, compatible parser states are all weight tables/label orders whose selected transition path builds that tree. Multiple action paths can induce one tree and kbest collapses them; tree cannot recover scores or training history. Sparse table decoding reconstructs weights exactly but loses averaging accumulators.

## root_legality

Shift requires b<=n; Left requires stack>=2 and second-top!=0; Right requires stack>=2 and either second-top!=0 or b>n.

## tie_and_zero

Greedy total_cmp favors later ACTS; beam stable equal scores preserve enumeration except gold sorts below nongold. width0 clamps1; empty kbest returns one zero-score empty tree.

## sparse_transport

Checks length consistency, duplicate feature keys, then class bounds; duplicate class entries overwrite. Signed zero is serialized because to_bits()!=0; u accumulator is empty after load.

## training_boundary

Context count asserted before filtering; tagged-only inputs retained. Word frequency>=2 sorted for ids. EN_PX cached once; training seed is delegated to ctx. Invalid heads/lengths can panic.

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

Actual source test: parse::wave27::bounded_contract. Command: cargo test -p en --lib wave27 -- --test-threads=1 (debug twice, release twice). External evidence: $MOVA_DATA/runs/octopus-wave27-structure-2026-10-31. Nine focused tests, each in its owned production module; no copied implementation.
