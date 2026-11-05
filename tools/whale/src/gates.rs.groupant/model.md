# mova-whale-gates

## Descriptant

Anti-padding gates of a Whale's own description: word bounds, near-duplicate sentences, verbatim run against the whole paper, n-gram overlap with any terminal answer (unless the paper has it), traceability of every content word to its basis or the frame vocabulary, and the share of structural sentences. Source: `tools/whale/src/gates.rs`.

## Forward approximant

Guard::new(paper, answers, params): token positions of the paper, paper n-grams, answer n-grams (n = answer_ngram). verbatim(s): longest run of s's tokens consecutive in the paper (position DP). answer_overlap(s): n-grams in answers and not in the paper. check(draft, reading, knowledge, guard, min, max): duplicates (exact or Jaccard >= near_duplicate on >= 4 content words), verbatim (structure sentences without «…» quotations), overlap, traceability (source/reading: source sentence texts + cited knowledge; explain: knowledge basis; structure: stated basis; plus FRAME), structure share; pass iff all.

## Invariant

A gate that cannot fail is not a gate: each has a red control in tests; deterministic; defaults verbatim_max 12, answer_ngram 6, near_duplicate 0.8, structure_share_max 0.2 (validated ranges 4..40, 3..20, 0.5..1, 0..1).

## Cost

Guard build O(paper tokens + answer tokens); verbatim O(sentence tokens x occurrences); duplicates O(items^2); milliseconds per draft.

## Composition

Parent: whale-workers; used by compose.rs (admits) while writing and by learn.rs (check) after writing.

## behavior_complement

Not done: semantic paraphrase detection, factual correctness, originality against other papers or the web, judging that a structural sentence is informative. A paper-wide repeated phrase longer than the cap counts against the description even where the paper repeats itself. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

gates.json names every failing item (index, run length, first overlapping n-gram, untraceable text, duplicate pairs) so the failing sentence and the gate are recovered; the passing set is not reconstructible from the report alone.

## History

### 2026-11-04 · Cost

Guard build O(paper tokens + answer tokens); verbatim O(sentence tokens x occurrences); duplicates O(items^2); milliseconds per draft (2026-11-04).

### 2026-11-04 · document

#### Epoch 2026-11-04 (grammar gate)

New gate `grammar`: every source/reading sentence passes `clause::grammatical`; red control on the ma-2018 fragments. Traceability compares normalized forms (a term may be written as «MRFs» for an item keyed «MRF»).

### 2026-11-05 · document

#### Epoch 2026-11-05

Test fixtures construct `compose::Item` with the new empty `cites` field; behaviour unchanged.
