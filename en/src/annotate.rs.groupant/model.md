# en-annotate

## descriptant

### annotate

Annotator holds Model and Ud; Word holds form, lemma, UPOS, tag, FEATS, head and relation.

annotate = tags -> parser.parse -> repair -> kids_of -> upos -> feats_sentence -> words_of. EModE forced tags precede parsing; EModE word fixes follow morphology. complete uses supplied tags/tree without repair. annotate_kbest caches lemmas and recomputes UPOS/FEATS per tree, also without repair. train filters tagged sentences before Ud and Model training.

Partial source-bound model, not full proof. No useful executable percentage surrogate established for these categorical decisions/effects; budgets1/2/5/10% require a future observation metric. No corpus/API/GPU/teacher, allocation fault injection, adversarial size exhaustion or broad accuracy claim.

## approximant

### Forward relation

annotate = tags -> parser.parse -> repair -> kids_of -> upos -> feats_sentence -> words_of. EModE forced tags precede parsing; EModE word fixes follow morphology. complete uses supplied tags/tree without repair. annotate_kbest caches lemmas and recomputes UPOS/FEATS per tree, also without repair. train filters tagged sentences before Ud and Model training.

Domain: compatible parallel arrays and valid symbol indices; API-specific omissions/invalid-input behavior in model.md#models. Finite machine arithmetic follows Rust source; no real-number replacement guarantee.

## invariant

### State and cost

Annotator holds Model and Ud; Word holds form, lemma, UPOS, tag, FEATS, head and relation.

For aligned valid inputs, completion cannot change token forms or token count. It cannot guarantee repair equivalence with annotate: complete and kbest omit repair. A failed pack prevents save from creating directories; a failed rename may leave a .part file.

For n words, add tagging/parsing costs to O(n^2) child scans and bounded agreement walks; kbest repeats annotation for each retained tree. Save/load cost serialized byte size plus allocation/I/O. No length or allocation cap.

## composition

### Scoped composition

Parent includes helpers, public API and test declarations; fragments below focus on distinct mechanisms and share the whole-file partial lexical projection. They are not separately registered sources.

- persistence: bytes[1226,4212) delegates this source's mechanism; forward composition: annotate = tags -> parser.parse -> repair -> kids_of -> upos -> feats_sentence -> words_of. EModE forced tags precede parsing; EModE word fixes follow morphology. complete uses supplied tags/tree without repair. annotate_kbest caches lemmas and recomputes UPOS/FEATS per tree, also without repair. train filters tagged sentences before Ud and Model training.
- pipeline: bytes[4212,8752) delegates this source's mechanism; forward composition: annotate = tags -> parser.parse -> repair -> kids_of -> upos -> feats_sentence -> words_of. EModE forced tags precede parsing; EModE word fixes follow morphology. complete uses supplied tags/tree without repair. annotate_kbest caches lemmas and recomputes UPOS/FEATS per tree, also without repair. train filters tagged sentences before Ud and Model training.

## models

## behavior_complement

For aligned valid inputs, completion cannot change token forms or token count. It cannot guarantee repair equivalence with annotate: complete and kbest omit repair. A failed pack prevents save from creating directories; a failed rename may leave a .part file.

## reverse_reconstruction

Given output Word sequence y and fixed model/ud, compatible antecedents include exactly form sequences equal to y.form plus tag/head/relation arrays whose completion yields y. Morphology/EModE/table backoff are many-to-one; outputs cannot recover training sentences. Successfully loaded bytes identify compatible serialized model states, not a unique byte encoding because map order may differ.

## layer_order

Repair uses pre-repair UPOS, then child flags and UPOS are recomputed; words_of performs EModE fixes last.

## array_domain

complete indexes tags/heads/rels/feature vectors by form index; unequal lengths can panic. Root uses head0 and words use 1-based heads.

## save_effects

pack first; mkdir parent if nonempty; write path.with_extension(part); rename. No fsync, concurrent writer isolation or rollback cleanup.

## empty_rows

Empty forms complete to empty; rows defaults missing space_after entries to true and emits ten columns with DEPS underscore.

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

Actual source test: annotate::wave27::bounded_contract. Command: cargo test -p en --lib wave27 -- --test-threads=1 (debug twice, release twice). External evidence: $MOVA_DATA/runs/octopus-wave27-structure-2026-10-31. Nine focused tests, each in its owned production module; no copied implementation.

### 2026-11-04 · descriptant

2026-11-04 epoch (en-morph): tags() merges the EModE forced tags with the Modern English context rules (tagrule::force), EModE first. Source 4a126331234c; earlier receipts and evidence refer to the previous source hash and are kept as history.

### 2026-11-04 · approximant

2026-11-04: forced = emode::force ⊕ tagrule::force (first Some wins); the tagger runs forced when any is Some.
