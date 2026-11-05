# en-expert

## descriptant

### expert

Rule stores ordered match nodes, require clauses (AND of OR), unless clauses (OR), source metadata. Bindings map distinct words to match names.

parse_file collects closed rule-prefix fences; parse_rule checks missing id, first field error, match, require, unless, then references. bindings recursively enumerates distinct word indices, filters local conditions early and resolves forward references only after full binding. check increments fired for every matching binding and emits 1-based indices iff not(all require clauses) and not(any unless clause).

Partial source-bound model, not full proof. No useful executable percentage surrogate established for these categorical decisions/effects; budgets1/2/5/10% require a future observation metric. No corpus/API/GPU/teacher, allocation fault injection, adversarial size exhaustion or broad accuracy claim.

## approximant

### Forward relation

parse_file collects closed rule-prefix fences; parse_rule checks missing id, first field error, match, require, unless, then references. bindings recursively enumerates distinct word indices, filters local conditions early and resolves forward references only after full binding. check increments fired for every matching binding and emits 1-based indices iff not(all require clauses) and not(any unless clause).

Domain: compatible parallel arrays and valid symbol indices; API-specific omissions/invalid-input behavior in model.md#models. Finite machine arithmetic follows Rust source; no real-number replacement guarantee.

## invariant

### State and cost

Rule stores ordered match nodes, require clauses (AND of OR), unless clauses (OR), source metadata. Bindings map distinct words to match names.

No successful validated rule refers to unknown match names. One sentence word cannot bind two match variables. unless suppresses violations but never decrements fired. Unclosed fences do not create rules. Unknown feature presence maps to zero whereas feature equality-to-node rejects unknown keys.

For n tokens and m match nodes, worst enumeration n!/(n-m)! times predicate checks; existential requirements scan n unbound candidates. Recursive depth m; output can be combinatorial. Filesystem loading traverses directories and sorts rules by id; errors are not globally sorted.

## composition

### Scoped composition

Parent includes helpers, public API and test declarations; fragments below focus on distinct mechanisms and share the whole-file partial lexical projection. They are not separately registered sources.

- language: bytes[3760,13874) delegates this source's mechanism; forward composition: parse_file collects closed rule-prefix fences; parse_rule checks missing id, first field error, match, require, unless, then references. bindings recursively enumerates distinct word indices, filters local conditions early and resolves forward references only after full binding. check increments fired for every matching binding and emits 1-based indices iff not(all require clauses) and not(any unless clause).
- binding: bytes[14670,19488) delegates this source's mechanism; forward composition: parse_file collects closed rule-prefix fences; parse_rule checks missing id, first field error, match, require, unless, then references. bindings recursively enumerates distinct word indices, filters local conditions early and resolves forward references only after full binding. check increments fired for every matching binding and emits 1-based indices iff not(all require clauses) and not(any unless clause).

## models

## behavior_complement

No successful validated rule refers to unknown match names. One sentence word cannot bind two match variables. unless suppresses violations but never decrements fired. Unclosed fences do not create rules. Unknown feature presence maps to zero whereas feature equality-to-node rejects unknown keys.

## reverse_reconstruction

For a fixed rule and result (fired,violations), preimage consists of all sentences whose matching injective assignments and require/unless truth values produce that result. Token spellings outside tested predicates, unused features and unattached nodes can vary; check output does not uniquely reconstruct a sentence or rule.

## quantifier_scope

exists/none range over words not already bound. require clauses all must hold, each clause has any alternative; unless only one clause needs to hold.

## missing_features

FeatSame requires nonzero reference bits and exact masked equality. Negated equality is true when positive equality fails, including absent features.

## failure_precedence

Missing rule id wins over bad fields; then match/require syntax, unless syntax, duplicate names and unresolved references. No comprehensive bracket-balance validator.

## file_effects

load silently skips unreadable directories and treats unreadable files as empty, then sorts accepted rules by id. No unique-id enforcement in load itself.

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

Actual source test: expert::wave27::bounded_contract. Command: cargo test -p en --lib wave27 -- --test-threads=1 (debug twice, release twice). External evidence: $MOVA_DATA/runs/octopus-wave27-structure-2026-10-31. Nine focused tests, each in its owned production module; no copied implementation.

### 2026-11-04 · descriptant

#### Epoch notes

- Comments repointed to consolidated docs, 2026-11-04.
