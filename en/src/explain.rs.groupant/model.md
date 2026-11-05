# en-explain

## descriptant

### explain

Explained owns text, tokenizer trace, top5 class views, parser steps/tree, graph bytes, side decisions, order and decoded strings.

explain tokenizes, tags and parses with steps; separately model.encode repeats tokenizer/tagger/parser. It projects dictionary analyses and graph nodes, explains linearizer decisions, computes order and both decode modes. fsa copies published tokenizer classes/states/commands and transition cells.

Partial source-bound model, not full proof. No useful executable percentage surrogate established for these categorical decisions/effects; budgets1/2/5/10% require a future observation metric. No corpus/API/GPU/teacher, allocation fault injection, adversarial size exhaustion or broad accuracy claim.

## approximant

### Forward relation

explain tokenizes, tags and parses with steps; separately model.encode repeats tokenizer/tagger/parser. It projects dictionary analyses and graph nodes, explains linearizer decisions, computes order and both decode modes. fsa copies published tokenizer classes/states/commands and transition cells.

Domain: compatible parallel arrays and valid symbol indices; API-specific omissions/invalid-input behavior in model.md#models. Finite machine arithmetic follows Rust source; no real-number replacement guarantee.

## invariant

### State and cost

Explained owns text, tokenizer trace, top5 class views, parser steps/tree, graph bytes, side decisions, order and decoded strings.

Displayed analyses do not prove chosen tag probability or linguistic correctness. classes is capped at5 but may contain fewer. explain does not call annotate repair; no equivalence to full UD annotation is promised. No model mutation or external I/O is performed by this file.

Duplicate model.encode and parse_steps make cost approximately two tagging/parsing paths plus graph/order/decode work; copies scale with token/string lengths and graph bytes. Parser trace clones stacks, potentially quadratic output. Analyses enumerate all dictionary records.

## composition

### Scoped composition

Parent includes helpers, public API and test declarations; fragments below focus on distinct mechanisms and share the whole-file partial lexical projection. They are not separately registered sources.


## models

## behavior_complement

Displayed analyses do not prove chosen tag probability or linguistic correctness. classes is capped at5 but may contain fewer. explain does not call annotate repair; no equivalence to full UD annotation is promised. No model mutation or external I/O is performed by this file.

## reverse_reconstruction

Given an Explained record produced by a fixed model, its text field recovers original input text exactly. Without text, graph and trace constrain compatible texts but arbitrary whitespace/case normalization can collapse. Model weights and training history remain many-to-one even with a full trace.

## trace_coordinates

Step stacks and buffer retain parser root-indexed coordinates, arcs expose head/dependent labels; graph bytes are12 per vertex rendered as spaced hex.

## display_projection

src_letters uses fixed order u,a,r,b and ignores other bits; case_name and act_name exhaust enum variants.

## model_join

g.nodes[i] assumes identical tokenizer length in encode and initial tokenize; graph lemma may be local while Word.id is None for local symbols.

## empty_input

Empty text gives empty words/tree/graph/steps and decoded text; FSA trace may include tokenizer-specific terminal events, delegated to tok.

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

Actual source test: explain::wave27::bounded_contract. Command: cargo test -p en --lib wave27 -- --test-threads=1 (debug twice, release twice). External evidence: $MOVA_DATA/runs/octopus-wave27-structure-2026-10-31. Nine focused tests, each in its owned production module; no copied implementation.
