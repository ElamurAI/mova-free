# Release 0.3 — notes (published 2026-10-03)

## What was new in 0.3 (from its README)

- **A self-retraining SLM with states** ([docs/selfplay.md](selfplay.md)) — a frozen Rust skeleton and a hot
  layer the model writes itself. States form a tree of deltas with rollback (to the last working state, then to the
  original); every state has its own *brain* (one memory cell in plain English, base + change log, crash recovery)
  and receives frozen *decision letters*; a request runs in a state given by its hash (or the last stable one) and
  every answer names both. `world brain serve` / `say` talk to the brain in English.
- **The family** — the training loop as a chat on a Unix socket: Mova Dev decides, Mova Current speaks for the
  current state, Mova State members explore from their own states, Random commenters read the history, the journal
  and the letters. Ideas go into a pool; Dev builds a child state from the best ones; a guard on untouched data
  keeps it or rolls it back. Training runs in its own thread and the channel keeps answering: `status`, `stop`,
  `focus`, `reject`, `pool`, `try`, `propose` (your own rule change), `negative`.
- **Learning on big texts** — samples of 300,000 sentences from the tree store, judged without gold by the
  absurdity matrix, guarded by UD grammar constraints (two subjects, case-marked subjects, vocatives) and by a
  teacher check (a large model judges 20 relabelled words). Rules that proved to be nonsense go to a negative list
  and are not tried again (a rare review after 30 days and more). Honest result: the matrix alone accepted harmful
  changes; the checks reject them; the first verified gain — `nsubj/obj → obl` when the word has a preposition —
  raised both test LAS numbers and is in the shipped rules.
- **Absurdity matrix** ([docs/absurdity.md](absurdity.md)) — a level-1 table of how plausible an event is:
  405 verbs × 601 nouns = 243 346 cells scored 0–4 for the doer and the object, plus a fairy-tale layer. In debug
  mode it finds parse errors (score 4 marks an error in 86–92% of cases on tales); in working mode absurdity that
  survives the domain layer switches on humor and irony.
- **Repairs and rules the model induces itself** ([docs/induce.md](induce.md)) — transformation-based learning
  of tree-repair rules with level-1 features; hidden rules come back (12 of 12) from unlabelled books.
- **StepGame, corrected and contrast** — the reader of new phrasings (direction words, the reference agent by cue
  words) next to the learned perceptron: 100% on the corrected StepGame and on a contrast set written before it.
- **Trees by logical derivation** ([docs/derive.md](derive.md)), **expressions with their real meanings**
  ([docs/pragmatics-expressions.md](pragmatics-expressions.md)), **tree store** (`world store`: trees per model
  version, delta versions, feature indexes, named sets, shuffled chapter-sized blocks).

## Metrics 0.2 → 0.3 (same commands as in the 0.2 README; 2026-10-02)

| measure | 0.2 | 0.3 |
|---|---|---|
| bAbI, 20 tasks | 99.8% | 99.8% |
| StepGame | 96.3% | 96.3% |
| SpartQA-Human YN | 60.1% | 75.5% |
| SpartQA-Human FB | 67.1% | 79.7% |
| SpartQA-Human CO | 58.2% | 75.9% |
| SpartQA-Human FR | 50.6% | 63.6% |
| SpartQA-Human total | — | 74.1% (280 of 378) |
| SVAMP (`MATH_NOLEX=1 mathsolve steps --set svamp --exact --qread --epochs 20 --beam 32 --rerank 5`) | 59.7% (179 of 300) | 64.0% (192 of 300) |
| LAS, tales test (induced repairs, tale domain) | 76.85 | 77.33 |
| LAS, EWT test | 79.82 | 79.84 |

## Against other systems (numbers from the papers)

| measure | Mova 0.3 | others | note |
|---|---|---|---|
| LAS, EWT test | 79.84 | UDify 88.5 (gold tokens); UDPipe 2.0 91.5; a treebank-tuned LLM 94.1; prompt-only LLMs 55–76 | no neural network; Stanza-level is the realistic target |
| SpartQA-Human YN | 75.5 | BERT+Q-Chain 59.44; Llama-3-8B CoT 67.83; PistaQ 75.52; GPT-4 77.62 | ours keeps DK in gold (stricter than their two-class) |
| SpartQA-Human FR | 63.6 | BERT with SpaRTUN 50.64 | exact set match |
| SVAMP | 64.0 | GTS 41.0; Graph2Tree+RoBERTa 43.8; DeductReasoner 47.3; Plan-and-Solve (GPT-3) 75.7; PAL 79.4; PoT 85.2 | first three are trained solvers, the rest LLM prompting with code |
| bAbI, 20 tasks | 99.8 | 10k training: STM 99.6, UT+ACT 99.8, EntNet 99.5 (0 failed); 1k: UT+ACT 95.5, MemN2N 87.6; no QA training: GPT-3 CoT 86.2, GPT-3 + ASP 99.99 | our setting is closest to the no-QA-training row |
| StepGame, corrected (Li, Hogg & Cohn 2024), clean / noise | **100.0 / 100.0** at every k=1..10 (macro mean 100.0; 66,517 / 46,824 questions) | Map+ASP 100 at all k (same corrected set) | reader trained on the corrected qa1_train only; negative control: test labels shifted by one class → 0.0%; paraphrases: a contrast set of new phrasings (test set written before the lexical reader, zero overlap with training: 29 phrasings) 100% at every k, perceptron alone 17.1%; caveat: Li's LLM rows use 100–1,000 samples, ours the full test |
| StepGame (original labels) | 96.3 | TP-MANN mean 53.0 (k=1..5); BERT+SpaRTUN 98.6 (k=1) → 45.7 (k=10); GPT-3 + ASP 92.6 → 88.3 | the original has ~10.7% wrong labels (Yang; Li, Hogg & Cohn 2024: 14 of 214 templates); on the corrected set Map+ASP is 100 at all k — we must measure on the corrected set |

SVAMP note:
~77% of model errors on the original SVAMP come from benchmark flaws; on the cleaned SVAMP-Platinum (273
questions) o1 and Gemini Thinking make 0 errors; full SVAMP: Self-Agreement (gpt-3.5) 86.0, ECHO 85.4.

SVAMP is reproducible only with the whole command above: without `--exact` (gold trees from the dataset's own
equations) the same code gives 148 of 300 for 0.3 and 153 for 0.2; without `--qread`, 169 (0.3) and 166 (0.2).
Re-measured 2026-10-05 on the code of both releases: 179 and 192 exactly.

Tales test (tales-odd) stays the 0.3 number from `world induce` (77.33). The family's later states were guarded
on tales-odd-b and explored with tales-odd-a held-out — both halves of the tales test — so their tales-test score
is not a clean number. Fixed 2026-10-03: the family now explores on tales-even-a (held-out tales-even-b) and is
guarded on tales-even-g, refuses any test set (negative control: `FAMILY_EVAL=…tales-odd-b` → refusal), and was
retrained from the original: its best clean idea (`parataxis → conj`) lowered the guard (78.00 → 77.95), so the
original stays stable — the earlier "gains" did not survive a clean split.

## Self-training in 0.3: what the family found, verified (2026-10-03)

The architecture ships: states as deltas with brains and letters, the family channel with live control (status,
stop, focus, reject, try, propose), exploring on gold splits and on big store samples (300k sentences, judged by
the absurdity matrix), guards on untouched data, a drift bound against the original. What it found so far:

| finding | evidence the family used | check by hand / on benchmarks | verdict |
|---|---|---|---|
| test leak: the family selected on halves of the tales test | — | found while writing the comparison table | fixed: tale splits of tales-even only, test sets refused (negative control) |
| scope bug: children dropped the general rules outside the tales | — | found in the code | fixed: rules keep their scope |
| `parataxis → conj` (tales) | dev +7, held +5 | clean guard 78.00 → 77.95 | rejected by the family |
| `obl:tmod → obl:unmarked` (ESL train) | dev +54, held +1 | EWT dev guard ±0 — a label-convention difference between treebank versions | rejected by the family |
| s464843d9: relax `obj → nsubj` with speech verbs (big tales) | guard sample: absurd −22, sensible +31 | 25 changed words read by hand: almost all wrong — vocatives ("say, March") and "what" become subjects, "the multitude shouted" loses its subject | **rejected by the check** |
| s1464e05c: relax `obj → nsubj IF head finite …` (big novels) | guard sample: absurd −516, sensible +781 | 40 of 207 changed words read by hand: about 6 right (inverted subjects: "Then arrive a crowd", "Long live the King", "cried out Father John"), the rest wrong (objects, vocatives, predicatives) | **rejected by the check** |
| s894bcb40: `nsubj/obj → obl IF the word has a preposition` (my proposal through `propose`, new atom) | gold guard EWT dev 78.80 → 78.83 | by hand 24 of 30 right (possessive 's fixed in the atom); teacher 18 vs 2; grammar violations −1,776 on 60k sentences; tests: tales 77.32 → 77.38, EWT 79.84 → 79.86; bAbI, StepGame, SpartQA unchanged | **kept — the first verified gain** |
| both children on benchmarks | — | bAbI 99.8, StepGame 96.3, SpartQA 74.1 — identical to the original; tales test 77.32 → 77.30, EWT test 79.84 | no gain |

Lesson: the matrix alone is a weak judge of label changes — its "sensible" count rewards vocatives and wh-words as
subjects; a big-text idea needs a precision check of its changed words (`world big-diff <state> <tale|real>`)
before it stays. 0.3 ships the original rules plus the verified case rule (s894bcb40); the family's machinery ships as the
self-training tool, with these results reported as they are. Rules that proved to be nonsense go to the family's
negative list and are not tried again (a rare review after 30 days and more).

## Corpus 0.3 (separate tar.gz on the project site)

Only what carries our work: question sets, summaries, checked treebanks (silver), scripts, contrast sets — with
the texts of the sentences they are attached to. Not the tree store of millions of automatically parsed sentences:
anyone rebuilds it with mova-free (`world store build`, deterministic), so it adds no work of ours; raw
paragraphs of public books are not published either.

Released 2026-10-03: mova-free v0.3 (commit 7ad04ca), corpus 0.3 on research.elamur.ai.
