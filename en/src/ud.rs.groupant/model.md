# en-ud

## descriptant

### ud

Ud maps hashed backoff keys to UPOS/FEATS counts. View carries form/tag/relation/child flags/agreement. Scores accumulates denominators and exact/base label/content counts.

kids_of scans direct children then propagates subject/aux flags from first conjunction ancestor unless own flags exist. train retains tagged all-UPOS sentences, FEATS only when annotated. upos/feats choose first key with total>=2 and smallest enum/Feats on ties, else X/empty. feats_sentence first predicts all then revisits VERB/AUX left-to-right with agreement unless EN_AGR=0. Scores computes LAS by base relation and MLAS with matching sorted functional children.

Partial source-bound model, not full proof. No useful executable percentage surrogate established for these categorical decisions/effects; budgets1/2/5/10% require a future observation metric. No corpus/API/GPU/teacher, allocation fault injection, adversarial size exhaustion or broad accuracy claim.

## approximant

### Forward relation

kids_of scans direct children then propagates subject/aux flags from first conjunction ancestor unless own flags exist. train retains tagged all-UPOS sentences, FEATS only when annotated. upos/feats choose first key with total>=2 and smallest enum/Feats on ties, else X/empty. feats_sentence first predicts all then revisits VERB/AUX left-to-right with agreement unless EN_AGR=0. Scores computes LAS by base relation and MLAS with matching sorted functional children.

Domain: compatible parallel arrays and valid symbol indices; API-specific omissions/invalid-input behavior in model.md#models. Finite machine arithmetic follows Rust source; no real-number replacement guarantee.

## invariant

### State and cost

Ud maps hashed backoff keys to UPOS/FEATS counts. View carries form/tag/relation/child flags/agreement. Scores accumulates denominators and exact/base label/content counts.

Missing UPOS gold cannot count as UPOS hit. No FEATS layer means no FEATS denominator increments and no MLAS hit. Out-of-range heads are ignored by children; malformed parallel arrays still may panic elsewhere. Unknown table keys cannot manufacture trained features.

kids_of O(n^2) scans plus n bounded ancestor walks; agreement repeated per verb can be O(n^2). Scores functional child collection/sorting may cost O(n^2 log n) across content words. Training count u32 and metric usize arithmetic have no checked overflow guarantee.

## composition

### Scoped composition

Parent includes helpers, public API and test declarations; fragments below focus on distinct mechanisms and share the whole-file partial lexical projection. They are not separately registered sources.

- agreement: bytes[1124,6084) delegates this source's mechanism; forward composition: kids_of scans direct children then propagates subject/aux flags from first conjunction ancestor unless own flags exist. train retains tagged all-UPOS sentences, FEATS only when annotated. upos/feats choose first key with total>=2 and smallest enum/Feats on ties, else X/empty. feats_sentence first predicts all then revisits VERB/AUX left-to-right with agreement unless EN_AGR=0. Scores computes LAS by base relation and MLAS with matching sorted functional children.
- tables: bytes[6084,10999) delegates this source's mechanism; forward composition: kids_of scans direct children then propagates subject/aux flags from first conjunction ancestor unless own flags exist. train retains tagged all-UPOS sentences, FEATS only when annotated. upos/feats choose first key with total>=2 and smallest enum/Feats on ties, else X/empty. feats_sentence first predicts all then revisits VERB/AUX left-to-right with agreement unless EN_AGR=0. Scores computes LAS by base relation and MLAS with matching sorted functional children.
- metrics: bytes[10999,16687) delegates this source's mechanism; forward composition: kids_of scans direct children then propagates subject/aux flags from first conjunction ancestor unless own flags exist. train retains tagged all-UPOS sentences, FEATS only when annotated. upos/feats choose first key with total>=2 and smallest enum/Feats on ties, else X/empty. feats_sentence first predicts all then revisits VERB/AUX left-to-right with agreement unless EN_AGR=0. Scores computes LAS by base relation and MLAS with matching sorted functional children.

## models

## behavior_complement

Missing UPOS gold cannot count as UPOS hit. No FEATS layer means no FEATS denominator increments and no MLAS hit. Out-of-range heads are ignored by children; malformed parallel arrays still may panic elsewhere. Unknown table keys cannot manufacture trained features.

## reverse_reconstruction

Given predicted UPOS/FEATS, compatible antecedents include all Views and tables selecting those values via any sufficient backoff key; output alone cannot recover key level/counts. Scores admit many gold/pred sequences with identical counts; main4 percentages further collapse scale and zero denominators.

## agreement_scope

Subject precedence is own nsubj/csubj then expl; aux/cop use head, conj climbs, relative pronoun may use acl:relcl antecedent. Coordination and first-conj form enter hash.

## feature_ties

Count threshold2 is on total examples, not winning count. UPOS lower index wins equal counts; FEATS lower bitset wins. Hash collisions remain possible.

## metric_units

main4 returns percentages, zero denominators=>0. row uses em dash for absent denominators and content F1 with max1 denominators.

## coordinate_flags

heads1..n/root0; children returns zero-based indices. Subject/aux/passive/fixed bits1/2/4/8; conj propagation uses own snapshot, chain bounded n.

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

Actual source test: ud::wave27::bounded_contract. Command: cargo test -p en --lib wave27 -- --test-threads=1 (debug twice, release twice). External evidence: $MOVA_DATA/runs/octopus-wave27-structure-2026-10-31. Nine focused tests, each in its owned production module; no copied implementation.
