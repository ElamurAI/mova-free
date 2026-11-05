# en-model

## descriptant

### model

Model combines Lexicon,PTagger,Morph,Parser,Linearizer and retained base sentences. Graph has ordered GNodes, local string heap, question and positional capitalization bits.

train_with filters tagged sentences and trains layers; beam>1 chooses12 beam epochs, else10 greedy. encode tokenizes/tags/parses, morphology-encodes each node, interns symbols, repairs article representation and extracts positional capitalization/mood. decode uses original indices or linearizer order, regenerates articles based on next emitted form, applies first capitalization, then detokenizes. adapt re-trains only lin on base plus weight repeated pseudo sentences.

Partial source-bound model, not full proof. No useful executable percentage surrogate established for these categorical decisions/effects; budgets1/2/5/10% require a future observation metric. No corpus/API/GPU/teacher, allocation fault injection, adversarial size exhaustion or broad accuracy claim.

## approximant

### Forward relation

train_with filters tagged sentences and trains layers; beam>1 chooses12 beam epochs, else10 greedy. encode tokenizes/tags/parses, morphology-encodes each node, interns symbols, repairs article representation and extracts positional capitalization/mood. decode uses original indices or linearizer order, regenerates articles based on next emitted form, applies first capitalization, then detokenizes. adapt re-trains only lin on base plus weight repeated pseudo sentences.

Domain: compatible parallel arrays and valid symbol indices; API-specific omissions/invalid-input behavior in model.md#models. Finite machine arithmetic follows Rust source; no real-number replacement guarantee.

## invariant

### State and cost

Model combines Lexicon,PTagger,Morph,Parser,Linearizer and retained base sentences. Graph has ordered GNodes, local string heap, question and positional capitalization bits.

Raw article escapes cannot undergo a/an regeneration. Graph local symbols cannot reconstruct omitted arbitrary whitespace; lossless means stored token/order/space information. Encode head cast to u16 can truncate beyond65535. adapt leaves parser/tagger/base unchanged but may replace lin even at weight0.

encode includes delegated tokenizer/tagger/parser plus O(n*L) local symbol linear search (L local heap length), string work. decode adds linearizer cost only when generative. adapt allocates base+weight*pseudo tokens, then linearizer training. u16 head cast and usize allocations lack checked bounds.

## composition

### Scoped composition

Parent includes helpers, public API and test declarations; fragments below focus on distinct mechanisms and share the whole-file partial lexical projection. They are not separately registered sources.

- graph: bytes[540,2607) delegates this source's mechanism; forward composition: train_with filters tagged sentences and trains layers; beam>1 chooses12 beam epochs, else10 greedy. encode tokenizes/tags/parses, morphology-encodes each node, interns symbols, repairs article representation and extracts positional capitalization/mood. decode uses original indices or linearizer order, regenerates articles based on next emitted form, applies first capitalization, then detokenizes. adapt re-trains only lin on base plus weight repeated pseudo sentences.
- pipeline: bytes[2819,9850) delegates this source's mechanism; forward composition: train_with filters tagged sentences and trains layers; beam>1 chooses12 beam epochs, else10 greedy. encode tokenizes/tags/parses, morphology-encodes each node, interns symbols, repairs article representation and extracts positional capitalization/mood. decode uses original indices or linearizer order, regenerates articles based on next emitted form, applies first capitalization, then detokenizes. adapt re-trains only lin on base plus weight repeated pseudo sentences.

## models

## behavior_complement

Raw article escapes cannot undergo a/an regeneration. Graph local symbols cannot reconstruct omitted arbitrary whitespace; lossless means stored token/order/space information. Encode head cast to u16 can truncate beyond65535. adapt leaves parser/tagger/base unchanged but may replace lin even at weight0.

## reverse_reconstruction

Given Graph g and model M, compatible source texts are those encoding to g; local heap and raw cases constrain strings but whitespace normalization can yield multiple antecedents. decode(g,true) is a canonical reconstruction, not a universal byte-inverse. Serialized Model state cannot identify training order/history uniquely.

## node_wire

GNode.bytes writes lemma4,tag1,case1,rel1,space1,head2 little endian and two zero reserved bytes; layout check is separate from language semantics.

## article_escape

Only DT a/an in lowercase or title case can normalize when vowel-letter rule reproduces input. Exceptions and other case patterns use Raw.

## capital_position

First Title node becomes AsIs plus cap_first unless proper noun, I, or inherently uppercase lemma; decode transfers title to first emitted node.

## adaptation_effect

Pseudo labels are own predictions; weight0 retrains from base, empty domain supplies no extra evidence. No external teacher or persistence occurs.

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

Actual source test: model::wave27::bounded_contract. Command: cargo test -p en --lib wave27 -- --test-threads=1 (debug twice, release twice). External evidence: $MOVA_DATA/runs/octopus-wave27-structure-2026-10-31. Nine focused tests, each in its owned production module; no copied implementation.
