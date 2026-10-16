# qagen — LLM-generated training questions and answers for fairy-tale paragraphs

`qagen` makes training questions: for each paragraph of a public-domain tale, a large model writes question–answer pairs of the FairytaleQA types, which Mova's readers learn from. The large model writes the questions; Mova answers them on its own.

Problem statement: `docs/context.md`, section "Five-year-old test", paragraph "Training questions from the LLM". This is training data for the memory, narrative and motives of the SLM (small language model): it learns to answer from the md graph.

## What

For each fairy-tale paragraph the LLM (Opus) writes 3–6 question-answer pairs. Each pair has:
- **type** from FairytaleQA: `character`, `setting`, `action`, `feeling`, `causal relationship`, `outcome resolution`, `prediction`. The names are verbatim from FairytaleQA, so comparison with the reference needs no mapping;
- **explicitness**: `explicit` — the answer is stated in the text, `implicit` — it has to be inferred. Each paragraph gets at least one implicit pair;
- **short answer** — up to 15 words. Under rule v2 an explicit answer is a **verbatim span of one anchor sentence**, as in FairytaleQA; an implicit one is free text;
- **anchors** — the sentences of this paragraph that support the answer.

The pairs live in the database, in the `qa` table (migrations v3–v4 in `db/schema.sql`). They are loaded by `db qa`; the view with names is `qa_pairs`, current pairs without replaced ones are `qa_current`.

## How

```
qagen select $MOVA_DATA/db/mova.duckdb <dir> [--rule 2] [--exclude <other run dir>/paras.jsonl]
                                                # paragraph selection → paras.jsonl (with batches), pick.json
qagen prompt <dir> 0                            # batch prompt, no call
qagen run    <dir> [--batches 0] [--jobs 5] [--max-calls 20]   # LLM → raw/bNNN.txt, calls.jsonl
qagen gate   <dir>                              # gates → qa.tsv, rejects.tsv, params.json
qagen sample <dir> [--n 20] [--weak 0.5]        # sample for eyeballing → sample.md; explicit pairs with weak support → weak.md
db qa $MOVA_DATA/db/mova.duckdb <dir>/qa.tsv --run <name> --params <dir>/params.json --path <dir>

# fixing the v1 run to rule v2 (src/fix.rs)
qagen fix select <pilot dir> <dir> [--batch 40]   # explicit non-span pairs → items.jsonl, paras.jsonl, select.json, doubt.tsv
qagen fix prompt <dir> 0
qagen fix run    <dir> [--jobs 5] [--max-calls 10]
qagen fix gate   <dir> --replaces <pilot run>    # → qa.tsv, rejects.tsv, changes.md, params.json
db qa $MOVA_DATA/db/mova.duckdb <dir>/qa.tsv --run <name> --replaces <pilot run> --params <dir>/params.json --path <dir>
```

Reused from existing code:
- **`prag::opus`** — calls `claude -p` in an empty directory, without tools. Effort is passed via `--settings '{"env":{"CLAUDE_CODE_EFFORT_LEVEL":"high"}}'`; accounting is one `calls.jsonl` line per call. Model and effort are set by the `PRAG_MODEL` and `PRAG_EFFORT` variables. Since 26.09 calls go through the API key from `~/Mova/local.md`, so `total_cost_usd` is the real price; `MOVA_SUBSCRIPTION=1` switches back to the subscription.
- **`prag::data`** — `duck_query`, `h64`, `has_quote` and the license gate.
- **`en`** — tokenizer, tagger and lemmas (model `$MOVA_DATA/en/models/en.bin`, variable `QAGEN_EN_MODEL`) for the "doubtful implicit" flag.

Database query: `sql/paras.sql`.

### Selection (`src/select.rs`)

- **Which paragraphs:** prose only, i.e. `newpar_block = p`; verse and headings are skipped. A paragraph has 2–16 sentences and 40–250 words. Of the 3,033 fairy-tale paragraphs in the database, 1,440 qualify.
- **Books** — proportional to size in sentences, by the largest remainder method.
- **Dialogue:** within each book, half the paragraphs have dialogue (with quotes) and half don't.
- **Tales** are taken round-robin in a seed-shuffled order: first one paragraph from each tale, then a second one.
- **Context for the LLM** — the last two sentences of the previous paragraph, marked "do not ask about this". If the previous paragraph is directly above in the same batch, there is no context.
- **License:** every document must be in the public domain. In addition, the `en::license::commercial_gate` gate fires with source `gutenberg-tales`.
- **Other:** `--exclude <paras.jsonl>` removes paragraphs of another run; with `--paras` larger than the number of eligible paragraphs, all other eligible paragraphs are taken in text order.

### Gates (`src/gate.rs`; negative controls are in the tests)

LLM output is a 6-column TSV: `p type ex anchors question answer`.

**Batch.** A format rejection is the only reason for a retry, and there is exactly one retry. The format is rejected when:
- there is no line with a question;
- a paragraph number is outside 1..N, i.e. alignment is in doubt;
- over 20% of lines have no number;
- over a quarter of paragraphs have no question.

**Pair.** Only this pair is rejected; the first reason goes to the log:
- not 6 columns;
- type outside the closed set, or explicitness not `explicit`/`implicit`;
- anchors are not numbers, missing, repeated or outside the paragraph;
- empty question or answer;
- the pair is implicit, but the answer appears verbatim in one sentence of the paragraph. Consecutive words are compared, ignoring case and punctuation;
- rule v2: the pair is explicit, but the answer is not a span of an anchor sentence (span gate, below);
- the question already occurred in this paragraph;
- it is the seventh or later question of the paragraph.

**Paragraph.** If there are no lines, the paragraph is rejected. If there are fewer than 3 questions, no implicit pair, or all of one type, a note goes to the log and the pairs stay.

**Database (`db qa`).** Checks integrity:
- the document exists in the database;
- every anchor is a sentence of exactly this document and paragraph;
- the question number within a paragraph is not repeated;
- no run with this name exists yet;
- with `--replaces <run>`: every line of the file replaces a line of that run with the same document, paragraph and number; such a line exists and is not yet replaced.

Type, explicitness, source and a non-empty answer are enforced by CHECKs on the `qa` table. Any error, and nothing in the database changes (`tools/db/tests/qa.rs`).

Rejection log: `rejects.tsv`; there are no blind retries. A call error stops the run. A rerun takes only unfinished batches. Ceiling: `--max-calls` calls per run directory (default 20).

### Rule v2: an explicit answer is a verbatim span

Design note: an explicit answer is a verbatim span of an anchor sentence, as in FairytaleQA. That way the SLM learns to extract the answer from the text rather than paraphrase. An implicit answer remains free text. The rule is stored in the run's `pick.json` (`"rule": 2`). The pilot lacks this field, so `qagen gate` on its directory reproduces v1.

**Prompt** (`prompt::RULES_V2`). An explicit answer is a span copied from one sentence: the same words in the same order and form, nothing changed, added or skipped inside. Words at the edges may be dropped, and the shortest span that answers the question should be taken. The sentence the span comes from must be among the anchors. An implicit answer is in one's own words and is not a paraphrase of a single sentence. The prompt includes an example: a three-sentence paragraph and four pairs for it.

**Span gate** (`gate::span_reason`; pair rejection). The answer and the sentence are normalized:
- lowercase;
- quotes removed, while an apostrophe inside a word stays (king's, don't);
- whitespace collapsed to one space;
- punctuation stripped from the edges of the answer.

Then the answer must occur in the anchor sentence as a substring on word boundaries: "the king" in "the king's garden" — yes, "the kin" — no. The rejection reason is one of three:
- "explicit: span of sentence k, which is not an anchor";
- "explicit: consecutive words in sentence k, but different punctuation" — a comma or dash disappeared or appeared inside the span;
- "explicit: not a span of an anchor sentence" — paraphrase, skipped or reordered words, a different word form.

**"Doubtful implicit"** (`lemma::doubt`; a log flag with level `doubt`, the pair stays). If at least 80% of the content words of an implicit answer are covered by lemmas or forms of a single anchor sentence, it is most likely a paraphrase of an explicit answer. The verbatim gate does not catch this: "because he had tasted the king's white snake" vs "…had tasted the white snake". Lemmas and parts of speech come from `en`. Content words are NOUN, PROPN, VERB, ADJ, ADV, NUM, INTJ, X.

Negative controls are in the tests `gate::span_gate_v2`, `lemma::doubt_flags_restated_implicit_answers` (lemma stub), `lemma::doubt_with_en_model` (real model) and `fix::fix_gate_and_negative_controls`.

### Fixing the pilot (`src/fix.rs`)

Explicit pairs of the v1 run that fail the span gate go to the LLM in batches of 40 paragraphs. The prompt contains the paragraph, question, type, anchors and the old answer. The LLM returns an `item anchors span` line: a verbatim span, or `implicit` if no sentence states it directly. In that case the pair keeps the old answer but becomes implicit. The source of pairs is the pilot's `qa.tsv`, i.e. exactly what is in the database.

Output gate:
- **batch** — the same format rules as in the main gate, with one retry;
- **item** — not 3 columns, wrong anchors, empty span, span not from an anchor (the same three reasons), `implicit` with a verbatim old answer, repeated item;
- **no answer** — the old line stays current.

**Database, migration v4.** Fixed pairs are a separate run (`db qa --replaces <pilot run>`) with the same document, paragraph and question number. The new row carries `replaces` — the qid of the old one. The old row is not deleted; in the same transaction it gets `replaced_by` — the qid of the new one. Current pairs: view `qa_current`.

## Pilot 26.09.2026

Directory `$MOVA_DATA/runs/qagen-2026-09-26/`, run in the database: `qagen-2026-09-26` (run 34).

| | |
|---|---|
| paragraphs | **300** from 137 tales, all with questions; 148 with dialogue |
| pairs | **1,626**, i.e. 5.4 per paragraph |
| explicit / implicit | 1,045 / 581, i.e. **64.3 / 35.7%**; every paragraph has an implicit pair |
| pairs per paragraph | six — 146 paragraphs, five — 135, four — 18, three — 1 |
| answer | 6.7 words on average, longest 14 |
| question | 9.1 words on average |
| anchors per pair | 1.22 |
| rejections | **2 pairs of 1,628 (0.12%)**; batch rejections and paragraph notes — 0 |

Distribution by book:

| book | paragraphs (with dialogue) | pairs |
|---|---|---|
| Grimm, #2591 | 125 (62) | 693 |
| Andersen, #1597 | 81 (40) | 418 |
| Aesop, #21 | 46 (23) | 258 |
| Kipling, #2781 | 46 (23) | 247 |
| Potter, #14838 | 2 (0) | 10 |

### Cost

There were **15 calls**: a pilot batch and a wave of 14 batches, 5 in parallel. A batch has 20 paragraphs. No retries were needed.

| | |
|---|---|
| time | ~7 min wall clock (pilot 104 s + wave 312 s); 1,491 s total across calls, i.e. 5.0 s per paragraph and 0.9 s per pair; longest call 128 s |
| tokens | output 171,807 including thinking (573 per paragraph, 106 per pair); cache: read 126,934, write 349,830; uncached input 30 |
| `total_cost_usd` estimate | $6.26 — subscription, so this is only an estimate |

The batch prompt is ~5k tokens: rules ~0.9k plus 20 paragraphs. The cache write is mostly `claude -p`'s own system prompt.

### Type distribution

| type | pairs | % | explicit | implicit | FairytaleQA, % |
|---|---|---|---|---|---|
| action | 412 | 25.3 | 395 | 17 | 31.6 |
| causal relationship | 333 | 20.5 | 166 | 167 | 27.8 |
| feeling | 228 | 14.0 | 71 | 157 | ~10 |
| character | 222 | 13.7 | 105 | 117 | ~10 |
| outcome resolution | 214 | 13.2 | 186 | 28 | ~10 |
| setting | 130 | 8.0 | 118 | 12 | ~5 |
| prediction | 87 | 5.4 | 4 | 83 | ~5 |

FairytaleQA shares are taken from the dataset description in Xu et al. 2022. There it is **75% explicit and 25% implicit**. We have more implicit pairs (35.7%) because the prompt requires at least one implicit pair per paragraph. Feelings and characters are also more frequent: the paragraph is short, and these are what stands out in it.

### Rejections

1. Aesop, "The North Wind and the Sun": "Who won the contest…? — the Sun", marked `implicit`. Both words of the answer appear verbatim in sentence 1, so by the gate's rule this is an explicit pair.
2. Grimm, "Clever Gretel", paragraph 2: the seventh question of the paragraph, of type `prediction`.

### Quality by eye

20 random pairs checked (`sample.md`, seed 7):
- **Answers** correct in 20 of 20.
- **Anchors** correct in 20 of 20.
- **Genuinely about comprehension** — 17 of 20: who did what and why, what came of what, what someone felt, what kind of character. The other three test a detail. Examples: "What was the goldsmith making? — a gold chain", "What did Chanticleer and his wife do with the egg?". For FairytaleQA these are legitimate `action` questions, but of little value.
- **Explicitness doubtful** in 1 of 20. "Why could the servant understand the sparrows? — because he had tasted the king's white snake" is marked `implicit`. Yet sentence 6 says it directly: "Eating the snake had given him power of understanding the language of animals". The answer is paraphrased, so the verbatim gate does not catch it.

Answer support by anchors can be estimated automatically: it is the share of answer words that occur in the anchor sentences.
- For explicit pairs, full support in 607 of 1,045.
- Support below 50% in 28 pairs (2.7%); they are collected in `weak.md`.
- 4 such pairs reviewed: all correct, the difference is only morphology and paraphrase ("to their surprise" → *surprised*, "much amusement" → *amused*, "ran" → *running*).

So a word-based measure without lemmas often raises false alarms.

### Example: "Hansel and Gretel", paragraph 1

| type | explicitness | anchors | question | answer |
|---|---|---|---|---|
| character | explicit | 1, 2 | Who lived by the great forest? | a poor wood-cutter, his wife, and children Hansel and Gretel |
| causal relationship | explicit | 3 | Why could the wood-cutter no longer get daily bread? | a great dearth fell on the land |
| action | explicit | 6 | What was the wife's plan for the children? | leave them alone where the forest is thickest |
| causal relationship | implicit | 9 | What made the wood-cutter finally agree to the wife's plan? | she kept nagging him, saying they would all starve |
| character | implicit | 6, 7, 9 | What kind of person was the wood-cutter's wife? | cruel and selfish |
| feeling | explicit | 10 | How did the wood-cutter feel about the plan? | very sorry for the poor children |

More examples of implicit pairs:
- "The Woman and Her Hen", feeling: "How did the woman probably feel when the hen stopped laying? — disappointed and sorry".
- "The Shadow", character: "What kind of character is the shadow, based on how it talks? — boastful, acting like a know-it-all".
- "The Happy Family", prediction: "What will the snails probably do next? — ask the ants to find a wife for the little snail".

## Rule v2, 26.09.2026: fixing the pilot and the remaining paragraphs

Directory: `$MOVA_DATA/runs/qagen-v2-2026-09-26/`: the main run in the root, the pilot fix in `fix/`. Calls went through the API, so the price below is real.

### Fixing the pilot

Run in the database: `qagen-v2-fix-2026-09-26` (run 35), replacing run `qagen-2026-09-26` (run 34).

| | |
|---|---|
| explicit pilot pairs | 1,045 |
| passed the span gate as is | **379 (36.3%)** |
| to fix | 666 from 288 paragraphs: 644 "not a span", 21 "different punctuation", 1 "not an anchor" |
| became spans | **657** |
| became implicit | **9** — the answer is assembled from two or three sentences, e.g. "Who lived by the great forest? — a poor wood-cutter, his wife, and children Hansel and Gretel" (anchors 1, 2) |
| gate rejections | 0; retries 0; unfixed 0 |
| calls | 8 batches of 40 paragraphs (17–105 items), 5 in parallel, ~2.5 min wall clock; 424 s total |
| tokens | output 51,316; cache: read 60,462, write 157,468 |
| price | **$1.83** |

In the database, 666 new rows carry `replaces`, and 666 pilot rows got `replaced_by`. Document, paragraph, number and question match in every pair. Current pairs: 1,626, as before — 960 pilot rows and 666 fixed ones.

Spans are longer than the old answers: 10.4 words on average vs 6.7 for explicit pilot pairs, longest 43. By length, the 666 answers are distributed as follows:

| words | 1–5 | 6–10 | 11–15 | 16–20 | 21+ |
|---|---|---|---|---|---|
| pairs | 114 | 278 | 185 | 54 | 35 |

Long spans are mostly lists that the old answer compressed: "What did the girl take when she ran away?" now has a whole 43-word list as its answer.

**By eye**, 12 changes (every 60th in `fix/changes.md`):
- 11 good. Examples: "only one" → "one only"; "The Bitch would not let the shepherd approach." → "would not permit the shepherd to approach"; "he had had nothing to eat since morning" → "I've had nothing to eat since morning". Outside the sample there is also "at a pond" → "a pond".
- 1 too long. "Where did the Mouse and the Frog each live?" got a 21-word span with an unneeded middle ("…by an unlucky chance formed an intimate acquaintance with a Frog…"). The answer is spread over two places in the sentence, and a span must be contiguous.

Implicit pilot pairs covered by the lemmas of one anchor ("doubtful implicit"): **27 of 581 (4.6%)**, list in `fix/doubt.tsv`. Example: "Why did the little match girl still have so many matches? — No one had bought any from her all day." These pairs were not changed.

### Run on the remaining paragraphs

Run in the database: `qagen-v2-2026-09-26` (run 36). Paragraphs: all eligible ones except the pilot's: `qagen select … --paras 5000 --exclude <pilot>/paras.jsonl`, batches of 20 in text order.

| | |
|---|---|
| paragraphs | **1,140** from 336 tales, all with questions; 878 with dialogue |
| pairs | **6,217**, i.e. 5.5 per paragraph |
| explicit / implicit | 4,172 / 2,045, i.e. **67.1 / 32.9%**; an implicit pair exists in 1,137 of 1,140 paragraphs |
| span gate | 4,172 explicit proposed, **4,172 (100%)** passed on the first try |
| pairs per paragraph | six — 582 paragraphs, five — 493, four — 65 |
| answer | 6.3 words on average. Explicit — 6.0: longest 16 words, three longer than 15. Implicit — 6.8, longest 13 |
| question | 9.2 words on average |
| anchors per pair | 1.26 |
| rejections | **15 pairs of 6,232 (0.24%)**: 13 — seventh question of the paragraph, 2 — implicit answer verbatim in a sentence. "No implicit" notes — 3, batch rejections — 0 |
| "doubtful implicit" | **78 of 2,045 (3.8%)** |

For comparison: of the explicit pilot pairs, written under rule v1, only 36.3% passed the span gate as is.

Distribution by book:

| book | paragraphs (with dialogue) | pairs |
|---|---|---|
| Grimm, #2591 | 382 (312) | 2,150 |
| Andersen, #1597 | 358 (237) | 1,878 |
| Aesop, #21 | 251 (210) | 1,398 |
| Kipling, #2781 | 146 (119) | 774 |
| Potter, #14838 | 3 (0) | 17 |

Type distribution is almost the same as in the pilot:

| type | pairs | % | explicit | implicit | pilot, % |
|---|---|---|---|---|---|
| action | 1,508 | 24.3 | 1,463 | 45 | 25.3 |
| causal relationship | 1,339 | 21.5 | 800 | 539 | 20.5 |
| character | 906 | 14.6 | 467 | 439 | 13.7 |
| feeling | 898 | 14.4 | 294 | 604 | 14.0 |
| outcome resolution | 681 | 11.0 | 582 | 99 | 13.2 |
| setting | 543 | 8.7 | 516 | 27 | 8.0 |
| prediction | 342 | 5.5 | 50 | 292 | 5.4 |

"Doubtful implicit" pairs are mostly near-verbatim paraphrases: "Which way did people look when they heard the bell? — toward the forest" (lemmas of sentence 2 cover 100%), "What will the princess probably do next? — work as a kitchen-maid in the king's palace". The list is the `doubt` lines in `rejects.tsv`.

### Cost

| | |
|---|---|
| calls | **57**: a trial batch and a wave of 56 batches, 5 in parallel; no retries |
| time | ~21.5 min wall clock for the wave, trial 130 s; 6,332 s total across calls, i.e. 5.6 s per paragraph; longest call 146 s |
| tokens | output 757,875 including thinking (665 per paragraph, 122 per pair); cache: read 479,999, write 709,579; uncached input 114 |
| price | **$18.80** |

Together with the pilot fix: **65 calls of the allowed 70, $20.63**, 809,191 output tokens, ~26 min wall clock.

### Quality by eye: 20 pairs

20 random pairs checked (`sample.md`, seed 7): 14 explicit and 6 implicit.

- **Answers** correct — 20 of 20.
- **Anchors** correct — 20 of 20.
- **Span boundaries.** 13 of 14 explicit ones read as an answer. One is awkward: "What happened to the miser after he confessed? — left the miser to take his place at the gallows". The span starts with a verb of another subject (the musician); a better span is "to take his place at the gallows".
- **Explicitness doubtful** — in 1 of 20. "Why was the Lion able to catch the Stag? — The Stag's horns got stuck in the wood, so he couldn't run." is marked `implicit`, while sentence 5 says it directly: "he became entangled by his horns". This is a paraphrase with synonyms: only 2 of 6 content words come from the anchor's lemmas, so the "doubtful implicit" flag does not catch it.
- **Genuinely about comprehension** — 18 of 20. Two pairs are about a detail: "What was on the door the brothers came to? — three locks", "What did the swallows know about where the trees had been taken? — did not know anything about it".

Good examples from the sample:
- causal, explicit: "Why did the rogues keep sixteen lights burning? — everyone might see how anxious they were to finish the Emperor's new suit";
- feeling, implicit: "How did the student feel while he waited? — impatient" (the text says "the time hung heavy upon him");
- outcome, implicit: "What happens to the Flea when men pat it? — It gets crushed and killed."

Automatic answer support by anchors is now almost always 100%. The exception is 2 pairs with a possessive: "his wife" in "his wife's name" (`weak.md`, `--weak 1.0`). The span gate treats the apostrophe as a word boundary, while the word-based measure does not.

### Database

Before the changes: a copy `$MOVA_DATA/db/mova.duckdb.bak-before-qa-v2-2026-09-26`. Then `db migrate` (v4), and `db qa` for the fix and for the new run. After the changes `db verify` reported 73 files byte-for-byte identical, counters 73, size as in the card for 29 of 29.

Current pairs (`qa_current`): **7,843 across all 1,440 eligible paragraphs**: 5,208 explicit (66.4%), 2,635 implicit. Every current explicit answer is a span of an anchor sentence: 379 pilot pairs passed the gate as is, 657 were fixed, 4,172 come from the new run.

## Run files

| file | contents |
|---|---|
| `paras.jsonl` | selected paragraphs with sentences (`sent_id`), context and batch number |
| `pick.json` | selection parameters and rule (`rule`; absent — v1) |
| `raw/bNNN.prompt.txt`, `raw/bNNN.txt` | prompt and raw LLM output; the retry, if any, is `raw/bNNN.retry.txt` |
| `calls.jsonl` | accounting: time, tokens and price per call |
| `qa.tsv` | accepted pairs for `db qa`; anchors are comma-separated `sent_id`s |
| `rejects.tsv` | log of rejections and flags: level `batch`, `para`, `pair`, `note` or `doubt` ("doubtful implicit") |
| `params.json` | accounting and distributions; stored in the database's `runs.params` |
| `sample.md`, `weak.md` | samples for eyeballing |
| `run.log` | execution log: one line per batch |

The fix (`fix/`) has the same `paras.jsonl`, `raw/`, `calls.jsonl`, `qa.tsv`, `rejects.tsv`, `params.json` and `run.log`, plus:

| file | contents |
|---|---|
| `items.jsonl` | explicit pairs to fix, with the span gate's rejection reason and batch number |
| `select.json` | selection summary: how many passed as is, reasons, "doubtful implicit" pairs of the old run |
| `doubt.tsv` | implicit pairs of the old run flagged "doubtful implicit" |
| `changes.md` | each change: question, before → after, anchors, explicitness |

## Next steps

- **Comparison with FairytaleQA.** First the data license must be checked. Their tales are also from Gutenberg, and our paragraphs overlap with Grimm and Andersen, so this gives a reference for the same texts. Our explicit answers are now spans, like theirs, so exact match can be computed too.
- **Long spans from the fix.** 89 of the 666 fixed answers are longer than 15 words, mostly lists. The new run has only 3 such. Two options: split such questions into several, or use spans of up to 15 words in training (length filtering is SQL).
- **"Doubtful implicit" — 106 pairs**: 27 from the pilot, 1 from the fix, 78 from the new run. They can be passed through the same `qagen fix` and turned into spans; that is 1–2 calls. Paraphrase with synonyms ("entangled by his horns" → "horns got stuck") is not caught by lemmas — that needs a different measure. Only on explicit assignment.
- **Remaining fairy-tale paragraphs.** 1,593 of the 3,033 paragraphs fail the filter: verse, headings, too short or too long paragraphs. Only on explicit assignment.

## Licence

Elamur Cosmic License ([LICENSE.md](../LICENSE.md)). Third-party data keeps its own terms, as noted where it is used.
