# qagen — LLM-generated training questions and answers for fairy-tale paragraphs

`qagen` makes training questions: for each paragraph of a public-domain tale, a large model writes question–answer pairs of the FairytaleQA types (Xu et al., ACL 2022, `2022.acl-long.34`; [card](https://research.elamur.ai/free/en/papers/xu-2022-fantastic-questions-where-find-them/)), which Mova's readers learn from. The large model writes the questions; Mova answers them on its own.

Problem statement: `docs/context.md`, section "Five-year-old test", paragraph "Training questions from the LLM". This is training data for the memory, narrative and motives of the SLM (small language model): it learns to answer from the md graph.

## What

For each fairy-tale paragraph the LLM writes 3–6 question-answer pairs. Each pair has:
- **type** from FairytaleQA: `character`, `setting`, `action`, `feeling`, `causal relationship`, `outcome resolution`, `prediction`. The names are verbatim from FairytaleQA, so comparison with the reference needs no mapping;
- **explicitness**: `explicit` — the answer is stated in the text, `implicit` — it has to be inferred. Each paragraph gets at least one implicit pair;
- **short answer** — up to 15 words. Under the span rule an explicit answer is a **verbatim span of one anchor sentence**, as in FairytaleQA; an implicit one is free text;
- **anchors** — the sentences of this paragraph that support the answer.

The pairs live in the database, in the `qa` table (migrations v3–v4 in `tools/db/schema.sql`). They are loaded by `db qa`; the view with names is `qa_pairs`, current pairs without replaced ones are `qa_current`.

## How

```
qagen select $MOVA_DATA/db/mova.duckdb <dir> [--rule 2] [--exclude <other run dir>/paras.jsonl]
                                                # paragraph selection → paras.jsonl (with batches), pick.json
qagen prompt <dir> 0                            # batch prompt, no call
qagen run    <dir> [--batches 0] [--jobs 5] [--max-calls 20]   # LLM → raw/bNNN.txt, calls.jsonl
qagen gate   <dir>                              # gates → qa.tsv, rejects.tsv, params.json
qagen sample <dir> [--n 20] [--weak 0.5]        # sample for eyeballing → sample.md; explicit pairs with weak support → weak.md
db qa $MOVA_DATA/db/mova.duckdb <dir>/qa.tsv --run <name> --params <dir>/params.json --path <dir>

# bringing a run made without the span rule to it (src/fix.rs)
qagen fix select <pilot dir> <dir> [--batch 40]   # explicit non-span pairs → items.jsonl, paras.jsonl, select.json, doubt.tsv
qagen fix prompt <dir> 0
qagen fix run    <dir> [--jobs 5] [--max-calls 10]
qagen fix gate   <dir> --replaces <pilot run>    # → qa.tsv, rejects.tsv, changes.md, params.json
db qa $MOVA_DATA/db/mova.duckdb <dir>/qa.tsv --run <name> --replaces <pilot run> --params <dir>/params.json --path <dir>
```

Reused from existing code:
- **`prag::opus`** — calls the shared Codex teacher ([`mova-teacher`](../teacher)) in an empty directory, without tools; accounting is one `calls.jsonl` line per call. The model comes from `MOVA_TEACHER_MODEL` or `PRAG_MODEL` (default `gpt-6.1-terra`), the effort from `PRAG_EFFORT` (medium).
- **`prag::data`** — `duck_query`, `h64`, `has_quote` and the license gate.
- **`en`** — tokenizer, tagger and lemmas (model `$MOVA_DATA/en/models/en.bin`, variable `QAGEN_EN_MODEL`) for the "doubtful implicit" flag.

Database query: `sql/paras.sql`.

### Selection (`src/select.rs`)

- **Which paragraphs:** prose only, i.e. `newpar_block = p`; verse and headings are skipped. A paragraph has 2–16 sentences and 40–250 words.
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
- span rule: the pair is explicit, but the answer is not a span of an anchor sentence (span gate, below);
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

### The span rule: an explicit answer is a verbatim span

An explicit answer is a verbatim span of an anchor sentence, as in FairytaleQA. That way the SLM learns to extract the answer from the text rather than paraphrase. An implicit answer remains free text. The rule is stored in the run's `pick.json` (`"rule": 2`). A run without this field is gated without the span gate, so `qagen gate` on its directory reproduces it.

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

### Fixing a run (`src/fix.rs`)

Explicit pairs of a run without the span rule that fail the span gate go to the LLM in batches of 40 paragraphs. The prompt contains the paragraph, question, type, anchors and the old answer. The LLM returns an `item anchors span` line: a verbatim span, or `implicit` if no sentence states it directly. In that case the pair keeps the old answer but becomes implicit. The source of pairs is that run's `qa.tsv`, i.e. exactly what is in the database.

Output gate:
- **batch** — the same format rules as in the main gate, with one retry;
- **item** — not 3 columns, wrong anchors, empty span, span not from an anchor (the same three reasons), `implicit` with a verbatim old answer, repeated item;
- **no answer** — the old line stays current.

**Database, migration v4.** Fixed pairs are a separate run (`db qa --replaces <pilot run>`) with the same document, paragraph and question number. The new row carries `replaces` — the qid of the old one. The old row is not deleted; in the same transaction it gets `replaced_by` — the qid of the new one. Current pairs: view `qa_current`.

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

This example comes from a run without the span rule; under the span rule the explicit answers are verbatim spans of an anchor sentence.

### Database

Runs that replace pairs of another run need migration v4 (`db migrate`).

## Run files

| file | contents |
|---|---|
| `paras.jsonl` | selected paragraphs with sentences (`sent_id`), context and batch number |
| `pick.json` | selection parameters and rule (`rule`: 2 is the span rule; absent — no span gate) |
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

## Limits

- **No comparison with FairytaleQA yet**: its data license is not checked. Its tales are also from Gutenberg and overlap with our Grimm and Andersen paragraphs; since our explicit answers are spans, exact match could be computed.
- **Long spans.** A fixed answer can be longer than 15 words, mostly a list, because a span must be contiguous. Length filtering for training is SQL.
- **"Doubtful implicit" pairs** stay unchanged; `qagen fix` could turn them into spans. Paraphrase with synonyms ("entangled by his horns" → "horns got stuck") is not caught by lemmas.
- **Paragraphs outside the filter** (verse, headings, too short or too long) have no pairs.

## Licence

Elamur Cosmic License ([LICENSE.md](../LICENSE.md)). Third-party data keeps its own terms, as noted where it is used.
