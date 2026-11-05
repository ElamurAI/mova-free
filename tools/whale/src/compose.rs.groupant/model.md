# mova-whale-compose

## Descriptant

The Whale's description written natively from its reading and retained knowledge: opening (title, size, parts, topics), per part a connecting lead with size and topics, restated clauses in source order, term explanations after first use, knowledge-assisted restatements, the numbers a part reports, a closing; every sentence with its basis; guard and near-duplicate refusal; even per-part share under the word budget; never padded. Source: `tools/whale/src/compose.rs`.

## Forward approximant

Known::of(reading, paper, items): the paper's own acronym definitions as native items; term facts from any paper (paper expansion kept, terminal adds the rest); sentence/section reads_as facts of this paper. write_with: structure sentences (FRAME vocabulary + basis), per readable section lead, clauses (keep filter), explain() at first token match, readings for sentences without a written clause, section readings, numbers (decimals, %, >= 2 digits, not years; >= 2 of them); closing two sentences. push refuses empty, guard-refused (structure: «…» unquoted first), near-duplicate (Jaccard >= threshold over >= 4 content words) and case-insensitive repeats. compose: if over max, share = (max - fixed)/available per section, evenly spaced picks, then drop trailing source sentences until <= max.

## Invariant

Words <= max; nothing added to reach min; every item has kind and basis (sources, knowledge ids or stated counts/titles); no terminal answer text (guard); deterministic (BTreeMap/BTreeSet, source order).

## Cost

O(items^2) near-duplicate comparisons (about 300-450 items per paper) plus one guard check per candidate (O(tokens x occurrences)); two passes when over budget; milliseconds per paper next to the 1 s read.

## Composition

Parent: whale-workers; inputs reader::Reading, knowledge items (knowledge.rs), gates::Guard; output Draft consumed by gates::check and learn.rs.

## behavior_complement

Not done: summarizing or abstracting a part, reordering by argument structure, claims/method/results classification, explaining a term the Whale has no fact for, any text from the terminal, padding. Prospective prediction (before measuring): native round 1 2,500-3,500 words on five open papers; measured 2,964-3,744. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

draft.json keeps every item with kind, sources, knowledge ids and basis: each sentence of description.txt maps back to its source sentences or knowledge item; which clauses were left out for the budget follows from the reading and the share. No unique inverse of the text is claimed.

## History

### 2026-11-04 · Cost

O(items^2) near-duplicate comparisons (about 300-450 items per paper) plus one guard check per candidate (O(tokens x occurrences)); two passes when over budget; milliseconds per paper next to the 1 s read (2026-11-04).

### 2026-11-04 · behavior_complement

Not done: summarizing or abstracting a part, reordering by argument structure, claims/method/results classification, explaining a term the Whale has no fact for, any text from the terminal, padding. Prospective prediction 2026-11-04 (before measuring): native round 1 2,500-3,500 words on five open papers; measured 2,964-3,744. Evidence status: source-derived model with finite checks, not a proof.

### 2026-11-04 · document

#### Epoch 2026-11-04 (writer version)

`WRITER` names the native reader/writer version; a batch retries a paper that stopped unpassed only under a different version, so it must change with every change that can change a description. FRAME gains «asks» for the question frame «The paper asks: …?».

#### Epoch 2026-11-04 (writer-2026-11-04d)

Glosses are keyed by normalized term and hold one sentence per fact relation, each bound to its own knowledge item (own items first, then the pod's); bare-category facts are never written; restated sentences pass the grammar gate before they are written.

### 2026-11-05 · document

#### Epoch 2026-11-05 (epoch writer)

`compose_epoch(r, known, guard, neighbours, max, min_relevance)` = `compose` (unchanged, so `WRITER` stays) followed by neighbour clauses: key terms (top 40 nouns, the paper's acronyms, its unknown terms, normalized); per neighbour text its sentences with at least `min_relevance` key terms, ranked by relevance then position; taken in rounds across neighbours under the word budget; each clause through `Writer::push` (guard, grammar gate now also for kind `world`, near duplicates); output per neighbour after a lead «I also read «title», relation.», in source order; items of kind `world` carry `cites` (new `Item` field, omitted when empty, so existing drafts serialize as before). Version `EPOCH_WRITER`. Measured on five batch papers: 1,161–4,384 neighbourhood words on four papers, 0 on a paper whose reading has no part. Not done: abstraction, explanations for world terms, a share cap between paper and neighbourhood.
