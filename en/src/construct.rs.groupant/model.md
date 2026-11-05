# en-construct

## descriptant

### construct

Item is construction name, token strings and target (head,dependent,Rel) arcs; report consumes ordered count/floor maps.

load reads UTF-8, skips blank and column-zero # lines, requires exactly3 tab fields, splits nonempty space-delimited tokens/arcs, resolves relation then head then dependent. node rsplit_once('-') supports hyphenated forms; Root-0 is special. recall accumulates exact (head,relation) matches. pct=round(10000*ok/max(n,1))/100; report flags pct<floor.

Partial source-bound model, not full proof. No useful executable percentage surrogate established for these categorical decisions/effects; budgets1/2/5/10% require a future observation metric. No corpus/API/GPU/teacher, allocation fault injection, adversarial size exhaustion or broad accuracy claim.

## approximant

### Forward relation

load reads UTF-8, skips blank and column-zero # lines, requires exactly3 tab fields, splits nonempty space-delimited tokens/arcs, resolves relation then head then dependent. node rsplit_once('-') supports hyphenated forms; Root-0 is special. recall accumulates exact (head,relation) matches. pct=round(10000*ok/max(n,1))/100; report flags pct<floor.

Domain: compatible parallel arrays and valid symbol indices; API-specific omissions/invalid-input behavior in model.md#models. Finite machine arithmetic follows Rust source; no real-number replacement guarantee.

## invariant

### State and cost

Item is construction name, token strings and target (head,dependent,Rel) arcs; report consumes ordered count/floor maps.

A known floor equal to rounded recall is not red. Missing floors produce no rejection. Unknown relations or mismatched token spelling cannot produce a successfully loaded Item. Loader does not exclude root-0 as dependent; downstream recall is not safe for that item.

Read O(B) bytes plus token/arc parsing and map O(log C) per construction. recall builds token references in O(total token count), calls the supplied parser once per Item, performs O(A) arc indexing, and pays construction-key cloning/map lookup costs. misses likewise traverses tokens and invokes the callback once per Item, then repeats sentence joining/rendering for each missed arc, proportional to emitted bytes. report O(C+output bytes). usize totals can overflow; percentages cast to f64 and accept nonfinite floors.

## composition

### Scoped composition

Parent includes helpers, public API and test declarations; fragments below focus on distinct mechanisms and share the whole-file partial lexical projection. They are not separately registered sources.


## models

## behavior_complement

A known floor equal to rounded recall is not red. Missing floors produce no rejection. Unknown relations or mismatched token spelling cannot produce a successfully loaded Item. Loader does not exclude root-0 as dependent; downstream recall is not safe for that item.

## reverse_reconstruction

Given per-construction counts, compatible inputs are all Item/tree pairs with identical aggregate exact arc successes and totals; token identities and failure placement are unrecoverable. Rounded percentage admits intervals of underlying ratios. Report cannot recover omitted floor entries or parse trees.

## failure_order

I/O and UTF-8 precede field count; arc delimiters precede Rel parse, then head and dependent lookup, then nonempty arcs.

## coordinate_domain

node accepts root0; recall/misses require 1<=dependent<=tree.len and valid predicted heads for rendering. Public Item construction can violate this.

## floor_quantization

Floor comparison uses rounded two-decimal percentages. n=0 is divided by1; NaN floor comparisons never mark red; duplicate floor keys keep the last value.

## callback_effects

recall/misses invoke an arbitrary callback exactly once per item and cannot promise purity or parser accuracy.

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

Actual source test: construct::wave27::bounded_contract. Command: cargo test -p en --lib wave27 -- --test-threads=1 (debug twice, release twice). External evidence: $MOVA_DATA/runs/octopus-wave27-structure-2026-10-31. Nine focused tests, each in its owned production module; no copied implementation.
