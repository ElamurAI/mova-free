# Absurdity matrix — the central common-sense tool

A graded table of how plausible an event is in the everyday world:
`0` normal, `1` slightly odd, `2` odd, `3` very strange, `4` absurd.
Scores come from Opus through the Message Batches API (half price),
built slowly, wave by wave, and compiled into level-1 knowledge (`global`).

## Why it is central

One table, many consumers:

| consumer | use |
|---|---|
| parse error search | `world sense-check`: an absurd tree is most likely a wrong tree |
| reranking | of several candidate parses / readings / answers, prefer the least absurd |
| irony and jokes | absurdity that the text itself marks (dialogue, "laughed", fairy-tale frame) is a genre signal, not an error |
| fairy-tale lens | talking animals are expected in tales; the lens shifts the scale instead of hiding it |
| gap journal | events the reader cannot place get a plausibility, not just "unknown" |
| QA / generation | reject absurd generated questions and distractors |

## Two modes

- **Debug mode** (parse and reader development): absurdity is the error finder — an absurd tree is first of
  all a suspected parse error (`world absurd-check`, `absurd-repair`). Here the matrix is gold.
- **Working mode** (reading real text): the parse is trusted, and absurdity that survives the domain layer
  (a talking fox in a tale is not absurd) is a signal that the text may be joking or ironic — it switches on
  the humor/irony module (a domain/lens), not a correction.

The humor/irony domain needs no absurdity layer of its own: absurdity is its trigger, not its knowledge.

## Domain scales

Absurdity depends on the domain. A talking fox scores 3 in the real world and 0 in a fairy tale;
a dragon eating a knight is absurd in news and normal in a legend. So a cell is `(event, domain) → score`:

- `real` — the everyday world (the base layer, rated first);
- `tale` — fairy tales and fables (talking animals, magic objects, giants);
- later: `news`, `science`, `myth`, `children`, `technical`, … as consumers need them.

Domain layers are rated as **deltas** over `real`: only cells with a real score ≥ 2 are sent again
with a domain prompt, which keeps the cost small. The domain of a text comes from the lenses
(fairy-tale frame, genre metadata); an event absurd even in its own domain is the strong signal
(error, joke or irony), an event absurd only in `real` is the genre signal.

## Growth, step by step

Each step is a separate run, measured before the next one starts.

1. **Roles** (pilot done: 5 verbs × 302 nouns,: verb × noun → SUBJ and OBJ scores.
   Now: 100 frequent tale verbs × 302 nouns, 4 waves of 25 verbs, 20 minutes apart (`data/runs/absurd-roles`).
2. **Compile**: scores → `global` table (build.rs, integer ids, source noted per cell);
   `sense-check` uses graded scores; measure precision/recall against gold trees.
3. **Reranking A/B**: parser k-best → least-absurd choice; paired test on LAS.
4. **Tale layer**: re-rate the cells with a real score ≥ 2 on the fairy-tale scale; sense-check on the tale treebank with the domain lens.
5. **Pairs**: who does what to whom (subject × verb × object) for key verbs among animate nouns.
6. **Width**: more nouns (corpus frequency, Wikidata classes), more verbs, prepositions (in/on/with + place/instrument), adjectives (noun × property).
7. **Classes**: generalize cells into noun classes so unseen words get a score through their class; Opus only for disagreements.
8. **Humor / irony** (working mode): absurdity that survives the domain layer switches on the humor/irony module; a small contrast set of jokes vs literal sentences measures the trigger. No humor layer in the matrix.

The matrix grows to colossal size only through steps that each pay off.

## First measurement (2026-10-02, 53 of 100 verbs)

`world absurd-check <scores.json> <in.conllu> [<gold.conllu>] --out <dir>` logs every judgment (plausible and
absurd, with the cell used) to `judgments.jsonl` and what the matrix does not know to `gaps.jsonl`.
Parse-error rate at the judged noun by score (real scale):

| score | EWT test parse vs gold | Rust tales2000 vs silver |
|---|---|---|
| 0 | 6/60 = 10.0% | 55/415 = 13.3% |
| 1 | 3/15 = 20.0% | 9/57 = 15.8% |
| 2 | 1/3 | 10/42 = 23.8% |
| 3 | — | 20/49 = 40.8% |
| 4 | 4/6 | 50/67 = 74.6% |

The rate rises monotonically with the score, so the score is usable for error search and reranking.
Controls on gold trees: EWT test 3 absurd of 74 judgments; the silver tales 74 absurd of 719 — mostly talking
animals ("said the dog"), the case for the `tale` layer. Coverage is the limit: the verb is in the matrix for
27–45% of verbs with a noun argument; top gaps are have/be/do (excluded on purpose), use, need, throw, open.
Deterministic (two runs, identical logs).

## Roles complete (2026-10-02, 105 verbs × 302 nouns, 31 708 cells)

| score | EWT test parse vs gold | Rust tales2000 vs silver |
|---|---|---|
| 0 | 10/75 = 13.3% | 61/535 = 11.4% |
| 1 | 3/16 = 18.8% | 11/79 = 13.9% |
| 2 | 3/5 | 16/63 = 25.4% |
| 3 | — | 21/59 = 35.6% |
| 4 | 4/6 | 64/85 = 75.3% |

Coverage of verbs with a noun argument: tales 44.9% → 57.3%, EWT 31% → 37%. Still monotone; score 4 marks a
parse error three times out of four on the tales.
Runs: `data/runs/absurd-check-105/`.

## Reranking and repair (2026-10-02)

`world absurd-rerank <dev> <test> [--k 8]` — the parser's k best trees (`en` `parse_kbest`, `annotate_kbest`),
`score − λ·penalty`, λ on dev. **No gain**: tales LAS 77.23 → 77.17 at λ = ∞ (25 sentences changed), EWT
unchanged; k = 32 is worse still. The beam's alternatives differ in structure, but the absurd errors are labels:
of 93 absurd parse errors on the tales, 44 are obj for a gold nsubj (inverted subject "said the fox"), 9 obj for
obl:unmarked, 6 nsubj:pass for nsubj — candidates the beam never proposes (one best label per arc).

`world absurd-repair <dev> <test>` — label repairs driven by the matrix, chosen on dev:

| rule | tales dev | tales test | EWT dev | EWT test |
|---|---|---|---|---|
| inverted-subject: obj after the verb, OBJ ≥ 3, SUBJ ≤ 1, no subject → nsubj | 10 fixed / 2 broken | 7 / 0 | 1 / 0 | 1 / 0 |
| bare-oblique: OBJ ≥ 3, SUBJ ≥ 2 → obl:unmarked (rejected on dev) | 4 / 12 | — | — | — |

Tales test LAS 76.85 → 76.88 (paired bootstrap p = 0.002): small, precise, significant. The main value is
cheaper development — the matrix points at the errors and says which fix fits — rather than a large LAS gain.

## Growth on 2026-10-02 (rated through a batch API)

| step | cells |
|---|---|
| fairy-tale layer, 105 verbs | 27 589 |
| roles, 300 new verbs × 302 nouns | 90 585 |
| roles, 405 verbs × 299 new nouns | 121 053 |

Level 1 now: 405 verbs × 601 nouns = 243 346 real-world cells + the tale layer for the first 105 verbs.

| measure | 105 × 302 | 405 × 601 |
|---|---|---|
| verb in the matrix (EWT / tales) | 37% / 57% | 65% / 76% |
| judgments, EWT test parse / Rust tales2000 | 102 / 825 | 317 / 1306 |
| score 4 → parse error, tales (tale domain) | 75% (real) → 92% (tale) | 86% |
| repair inverted-subject, tales test | 7 fixed / 0 broken | 14 / 1 (LAS +0.05, p < 0.001) |

The tale layer on the silver tales: false alarms 89 → 39. The new verbs have no tale layer yet.

## Fixes found with the matrix (debug mode, 2026-10-02)

`world absurd-events <events.tsv> [--domain tale]` ranks corpus events (617 public-domain books) by count among
those the matrix finds absurd; frequent ones are systematic errors. Found and fixed:

- tokenizer: "cannot" stayed one token and became a verb ("cannot help") — now `can|not` as in UD (`en::tok`);
- parser labels, repaired after parsing (`world::rerank::repairs`, wired into reader trees and `world events`):
  - inverted subject — "'…,' said the cat", "then came a knight", "there lived a king": obj → nsubj when the
    finite verb takes no object (matrix intransitivity, `global::absurd_intransitive`, ≥ 80% OBJ = 4) or the cell
    is absurd as an object, the noun is a normal doer in the real or tale layer, no subject (an expletive does not
    block), no xcomp/ccomp after the noun;
  - bare oblique — "went a long way", "waited a while": obj → obl:unmarked for `measure_time` nouns after an
    intransitive verb.

| | tales dev | tales test | EWT dev | EWT test |
|---|---|---|---|---|
| inverted subject, fixed / broken | 17 / 0 | 20 / 0 | 1 / 0 | 0 / 0 |
| bare oblique, fixed / broken | 4 / 1 | 4 / 1 | 0 / 1 | 2 / 1 |
| archaic adverb (whence, hither… read as nouns → ADV advmod), fixed / broken | 0 / 0 | 3 / 0 | — | — |
| LAS | 77.23 → 77.31 | 76.85 → 76.95 (p < 0.001) | 79.39 → 79.39 | 79.82 → 79.82 |

Absurd share of corpus events: 11.0% → 8.2% after the first version of the fixes. Remaining top items point at
the next fixes: archaic adverbs tagged as nouns ("whence came" — fixed: `archaic_adverbs` seed, two repair
passes so that "Whence came this stranger?" ends with stranger as the subject), names ("Diamond said").
Bench (quick): svamp and ftqa unchanged with and without the repairs.

### Second pass over 2 638 books (5.78 M sentences, ~5 250 sentences/s on 10 threads)

Absurd share of event uses (tale domain): 9.7% → 9.1% after repairs v4 — names after speech verbs ("cried
Tom"), "there" read as a subject → expl, the rest of a quotation (ccomp) no longer blocks, speech swap ("'Good
morning,' said the fox" with morning as the subject), measure/time nouns after verbs that do not take them
("stopped a moment"), "quoth" as a speech verb. Tales test LAS 76.85 → 76.98; EWT neutral; bench unchanged.
Rejected on dev: vocative after a speech verb ("I say, young man") — 0 fixed, 1 broken.

What remains on top is mostly not label errors: idioms and strict cells ("make no matter", "open fire", "fight
his way", "slay the dragon" — slay has no tale layer yet), attachment errors that a label repair cannot fix
("strong-looking men", "living men", "a man called Robert"), sentence-split errors. Next: the tale layer for the
300 new verbs, idiom cells, and attachment fixes in the parser itself (training data), not in repairs.
`world events-show <book.txt> <n>...` prints the sentences behind index ids for this kind of debugging.

## Idioms, light verbs, phrasal verbs (2026-10-02)

Idioms are a level-1 base, not a hand list. `tools/wikt-idioms` reads the English Wiktionary dump (342 045
multiword entries: 9 085 idioms, 61 684 verb phrases, 3 381 phrases, 1 519 proverbs); `world idioms <cells.tsv>
<events.tsv> <wikt.tsv> --out <dir>` joins them with corpus statistics — PMI of verb and object over dependency
arcs in 5.78 M sentences (syntax-aware PMI after Bogdanova 2026, PARSEME 2.0; association over frequency, Dunn
2019) — and the matrix (literal reading odd or absurd). Result `global/data/idioms.tsv` (CC BY-SA, Wiktionary):
8 057 pairs — idioms (make one's way, take part, pay attention), light verbs (take place, take care, make an
effort), collocations (tell a story, shake one's head), 564 phrasal verbs (give up, carry out); corpus-only pairs
are `candidate` and never suppress an alarm (the negative control caught "eat mouse" as a corpus "idiom").

In the matrix a dictionary-confirmed idiom, light verb or collocation is not literally absurd as an object, and
with an idiomatic object the doer is not judged ("the marriage took place"). Corpus absurd share 9.1% → 8.9%.
Asked the research library for MAGPIE, PARSEME 1.3 English, EPIE and the classics (Sag et al. 2002, Church & Hanks
1990, Fazly et al. 2009, Constant et al. 2017) and LLM-and-idiom studies.
