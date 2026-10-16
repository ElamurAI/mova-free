# en — Mova's English graph compressor

`en` is Mova's English core. It tokenizes, tags and parses text into full Universal Dependencies (UPOS, XPOS, FEATS, lemma, head, relation) with a deterministic Rust pipeline trained on UD like a model, but without neural nets, and it generates text back from the graph. The rest of Mova reads English through it.

**The original brief (24.09.2026):** take English only; prompts will be run through it. Build a compressor: a random sentence from various domains goes into a graph via a deterministic parser in Rust, and the result comes back out via a deterministic generator as well. The training model is like that of neural networks; the parser and the graph must preserve the structure, and the generator must output it. The compressor must keep its state in the graph; input and output must correspond — not necessarily word for word, but the domain style must be maintained. The idea of "class → class" tables and an automaton comes from an earlier `html_parser.c`.

**Later the same day:**
- Build everything on Rust structures.
- Words can literally be numeric constants in the code, and the rules check integer constants in the L1–L2 cache.
- Hash tables can also be kept cached in the code: word (full expansion of all forms) → class and other features.
- English then turns into a compact, practically deterministic code.

A step-by-step explanation with an example — [`docs/en-explained.md`](../docs/en-explained.md) (command `en explain`).

## Rust structures

- **`gram.rs` — types instead of strings.**
  - A PTB tag (53) and a UD relation (57) are one-byte enums. An unknown relation in a treebank is a read error with a line number.
  - `Case` is an enum with a `Raw` escape.
  - A lemma is a `Sym` number: in the dictionary heap, or, with the high bit set, in the graph's local heap (new words).
- **Graph (`model.rs`).**
  - A `GNode` vertex is 12 bytes without strings: lemma, tag, case, relation, head, space.
  - Sentence bits: "interrogative", "first word capitalized".
- **Tables.**
  - Parser features are numeric keys (template number + word and tag codes).
  - The tagger is a dense 54³ transition array with exact Viterbi.
  - The order generator's keys are structures (`Class`, `SideKey`, `Link`).
  - The hash is deterministic (`hash.rs`), so training is reproducible.

## Dictionary in code (`data/`, `build.rs`, `dict.rs`)

- **`en lexicon <agid infl.txt> <train.conllu>...`** generates two files:
  - `data/lemmas.tsv` — the lemma heap. 110,308 lemmas, number = line, **append-only** (numbers are frozen: a graph written today is always readable). First by UD frequency, then alphabetically.
  - `data/forms.tsv` — full expansion of forms: 242,166 forms, 315,675 entries "form → tag, lemma, UD frequency, source" (UD / main form of an AGID cell / clean for reverse lookup / stem).
- **`build.rs`** compiles both files into static open-addressing hash tables (the same hash as at runtime). Nothing is built or read from disk at runtime.
- **Word constants:** the 5000 most frequent lemmas plus `data/consts.txt` (weather: `w::CELSIUS`, `w::FAHRENHEIT`, `w::RAIN`…). A rule in code is an integer comparison.
- **Morphology (`morph.rs`)** — stateless, on the dictionary in code: UD frequencies → AGID paradigms → suffix rules.
- **Tagger.** Unknown words get a class hint from the full expansion (×100 over the suffix model): +2.2–2.8 points of accuracy on unknown words.

Dictionary sources: AGID 2016.01.19 (Kevin Atkinson, permissive license), UD English EWT + ESLSpok train (CC BY-SA 4.0).

## Layers and gates (training on UD English: EWT + ESLSpok train, 14,400 sentences)

| Layer | How | Gate (UD test) |
|---|---|---|
| `tok.rs` tokenizer | table automaton: character class × state → command and new state; dictionaries from train | exact: ATIS 100%, PUD 98.0%, GUM 94.1%, EWT 90.0% |
| `ptag.rs` PTB tagger | averaged perceptron: word and neighbours ±2, suffixes and prefixes, word shape, dictionary in code, two previous tags; beam 8. For unknown words, a TnT prior (`tag.rs`: trigram Markov model, suffixes + dictionary) | EWT 94.35%, PUD 94.04%, GUM 93.82% (TnT — 93.12 / 93.12 / 92.84); ~0.12M tok/s |
| `morph.rs` lemma ↔ form | dictionary in code; encoding = lemma + case or escape; position rules (a/an, capitalized first word) | lemma = UD 99.3–99.5%; **lossless 100%** |
| `parse.rs` tree | arc-standard transition automaton; "feature → action weights" tables (type + label), averaged perceptron; **beam 8** with early update (Zhang & Clark 2008), 12 epochs, training ~40 s | EWT UAS/LAS 86.9/83.8 (gold tags), 84.2/79.8 (ours; greedy — 83.4/79.0; greedy with TnT tags — 82.3/77.3); PUD 81.5/76.3, GUM 81.5/76.2; ~58K tok/s |
| `lin.rs` order generator | side of the dependent (table with backoff; "heavy" feature — the dependent has its own complement, so a modifier with a complement goes after the noun) + a Markov chain on each side, dynamic programming over subsets | gold trees: EWT 58.8% exact, 84.8% bigrams |

### How much more data gives (25.09.2026)

Training on a uniform fraction of EWT + ESLSpok (`EN_TRAIN_FRAC`), test — EWT test, TnT tags:

| sentences | tag | UAS | LAS |
|---:|---:|---:|---:|
| 1 800 | 90.21 | 74.39 | 67.67 |
| 3 600 | 91.49 | 77.85 | 71.70 |
| 7 200 | 92.40 | 79.96 | 74.52 |
| 14 400 | 93.12 | 82.27 | 77.27 |

Each doubling of a corpus of the same composition gives ≈ +0.8 tag and +2.5–3 LAS. A different composition gives less:
- **CHILDES** (+35K sentences of child speech) — did not help: EWT LAS −0.2;
- **GUMReddit and LittlePrince** (+1,200 sentences) — ±0.3;
- **GUM** (+11K sentences of various genres) — EWT LAS +0.8, PUD +3.1, GUM +4.2. But its license is CC BY-NC-SA: research only, not for commercial projects.

So Opus silver should be taken from the text the module will work on (for RAG — documents and queries), not just any text.

Jackknifing (the parser learns on tags assigned by a tagger that has not seen those sentences; `EN_JACKKNIFE=10`) gave ±0.2 LAS, so it is off by default.

## Examples are not truth

Design principle (25.09.2026):

> Training must take into account that examples are not the truth: they may contain errors, so the trained algorithm should not be reduced entirely to covering the examples.

- **Tagger** (`ptag.rs`). If the model disagrees with a word's label in every epoch starting from the 3rd, there are no more updates on it. The word goes to a list of suspects — candidates for annotation errors: `EN_SUSPECTS=file en tag-eval …`.
  - Negative control (`EN_NOISE=0.05`, 5% of training tags replaced with random ones). Suspects find corrupted labels with 95.8% precision and 63% recall. Accuracy on the noisy test: 94.00 without robust training, 94.10 with it.
  - On clean EWT — ±0.1. 185 suspects; in a sample of 16, about 4 are real errors in the gold standard (a quote tagged as a comma, "that" as RB), the rest are borderline cases.
- **Parser:** averaged perceptron and beam; epochs are chosen on dev, not on training accuracy.
- **Tables** for UPOS, FEATS and the order generator: a key fires only with ≥ 2 examples (singletons are not considered a rule), otherwise back off to a coarser key.
- **Silver** (`annot`, `corpus/`) weighs less than gold. The yardstick is only independent gold, never our own annotation.

## Goal: the level of top neural networks and beyond

Goal (26.09.2026): train the UD meta-dialect for English to the level of top neural networks and beyond.

**The bar on EWT test** (gold tokens):

| system | LAS | MLAS | source |
|---|---:|---:|---|
| UDPipe 2 (neural, EWT 2.17 model) | 92.37 | 86.31 | our measurement on the whole test, 26.09 (`annot udpipe-eval`) |
| STEPS (XLM-R) | 91.91 | — | paper `2021.iwpt-1.13` |
| **SLM + LLM** (edit mode high) | 91.90 | 84.99 | 300 random sentences of any length; UDPipe on them — 92.11 / 85.90 |
| **SLM alone** (this crate, CPU) | 80.24 | 70.75 | whole test, training EWT + ESLSpok |
| **SLM alone, trained on the meta-dialect** | **81.15** | **71.28** | whole test, training EWT + ESLSpok + GUM in `mova` |

**First experiment with training on the meta-dialect** (26.09). GUM was converted to `mova` by a converter (`treebanks/en/gum/convert.md`, 6133 words out of 200K changed) and added to training:

| training | EWT: UPOS | UFeats | UAS | LAS | MLAS | BLEX | GUM: UPOS | UFeats | LAS | MLAS |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| EWT + ESLSpok | 94.74 | 95.66 | 84.27 | 80.24 | 70.75 | 72.61 | 94.26 | 93.50 | 76.87 | 64.02 |
| + GUM in `mova` | 95.02 | 95.46 | 85.14 | 81.15 | 71.28 | 73.72 | 96.57 | 95.48 | 81.53 | 71.32 |

The meta-dialect works:
- EWT: +0.9 LAS;
- GUM: +4.7 LAS, +7.3 MLAS.

UFeats on EWT −0.2: some GUM conventions (*you* with Number, `Degree` on adverbs) still leak through — the next converter fix.

**License caveat.** GUM is licensed CC BY-NC-SA 4.0, so experiment B is research only. The production and open models train only on non-NC treebanks (EWT, ESLSpok, CHILDES, ATIS…) and on our silver (`licence/DATA.md`).

**Plan:**
1. **SLM + LLM — already on par.** Next — cheaper: Opus only on doubtful sentences (two roads to the meta-dialect, rules after Opus). And more accurate on MLAS: features and function words.
2. **SLM alone — from 80 to 90+:**
   - **data:** all English treebanks through converters into the meta-dialect (`docs/dialect.md`) plus LLM silver in large volumes. The SLM is a student of the LLM, not a distillate (`docs/context.md`);
   - **model:** a stronger parser — dynamic oracle, richer features, context features for re-parsing. Then a small neural network on CPU in Rust. GPU only for batch training, when needed.
3. **"And beyond":** one consistent meta-dialect instead of contradictory treebanks, LLM silver and context layers.
4. **Yardstick:** EWT test, other English treebanks through reverse converters, CPU speed.

### Document context in parsing — measurement (26.09; experiment, GUM is NC)

The idea — [`docs/context.md`](../docs/context.md), "Multi-pass parsing". Two questions:
- how much document context gives if it is known perfectly (the ceiling);
- how much a real second pass gives.

**How it was measured.**
- Context features (`ctx.rs`) — four groups:
  - document genre;
  - sentence type (GUM `s_type`) — current and previous;
  - paragraph start and speaker change;
  - token entity — type and new/giv/acc status of the deepest mention containing it.
- The features are minimal, conjoined with existing templates:
  - tagger — class shift, × word shape, × first word;
  - parser — × tag of the stack top and × tag of the buffer front;
  - label — × tags of head and dependent.
- Training — as in the "+ GUM in `mova`" row above: EWT + ESLSpok + GUM, all in `mova`, the same epochs and beam. The `none` baseline reproduces that row to the hundredth: GUM test 81.53 / 71.32. Without the `EN_CTX*` variables the annotation is bit-identical to that before the changes (checked on EWT dev and on this row).
- 3 seeds (`EN_SEED` — shuffle order); ± is the standard deviation across seeds. Difference interval — 95%, paired bootstrap over sentences, 1000 resamples (`en ctx-report`).
- Selection on dev, test only once at the end.

**Ceiling, GUM test** (1464 sentences, 28,397 tokens):

| context | UPOS | UFeats | LAS | MLAS | ΔLAS [95%] | ΔMLAS [95%] |
|---|---:|---:|---:|---:|---:|---:|
| none (baseline) | 96.57 | 95.49 | 81.70 ± 0.15 | 71.61 ± 0.27 | — | — |
| **gold, all groups** | 96.79 | 95.60 | 82.47 ± 0.12 | 72.79 ± 0.18 | **+0.77** [+0.37; +1.18] | **+1.18** [+0.68; +1.65] |
| all without genre | 96.86 | 95.67 | 82.67 ± 0.05 | 73.12 ± 0.09 | +0.97 [+0.58; +1.35] | +1.51 [+1.04; +1.95] |
| entities only | 96.75 | 95.59 | 82.58 ± 0.06 | 72.90 ± 0.14 | +0.88 [+0.55; +1.22] | +1.29 [+0.89; +1.74] |
| — only "token in a mention" | 96.80 | 95.61 | 82.45 ± 0.13 | 72.70 ± 0.08 | +0.75 [+0.37; +1.12] | +1.09 [+0.67; +1.49] |
| sentence type only | 96.68 | 95.54 | 81.67 ± 0.15 | 71.77 ± 0.07 | −0.03 [−0.38; +0.30] | +0.16 [−0.22; +0.53] |
| paragraph and speaker only | 96.64 | 95.50 | 81.57 ± 0.05 | 71.64 ± 0.23 | −0.13 [−0.43; +0.16] | +0.03 [−0.32; +0.37] |
| genre only | 96.62 | 95.53 | 81.61 ± 0.14 | 71.69 ± 0.11 | −0.09 [−0.38; +0.23] | +0.08 [−0.28; +0.46] |
| without entities | 96.69 | 95.53 | 81.64 ± 0.13 | 71.69 ± 0.16 | −0.06 [−0.39; +0.26] | +0.08 [−0.33; +0.48] |
| control: gold-trained model, another sentence's context | 95.20 | 94.11 | 77.23 ± 0.36 | 65.77 ± 0.39 | −4.46 [−4.90; −4.01] | −5.84 [−6.38; −5.32] |
| control: trained and measured on shuffled | 96.56 | 95.40 | 80.81 ± 0.27 | 70.62 ± 0.23 | −0.89 [−1.20; −0.61] | −0.99 [−1.33; −0.62] |

**Where the ceiling comes from.**
- **It is carried by mention boundaries.** The gain comes from GUM entities, and within them the bare mark "token in a mention", without type or status, already gives +0.75 LAS. Type only or status only give the same (+0.75 / +0.91). 59% of tokens are in mentions, so these are essentially gold noun phrase boundaries.
- **Where the gain is.** Most of all — nmod (+2.6 LAS), root (+1.8), acl (+3.7), advcl (+2.8), expl (+16). Tagger: PROPN 91.3 → 94.6. By genre — where noun phrases are long: voyage +2.4, whow +2.2, textbook +1.8, academic +1.6. In conversation, court and speech there is no gain.
- **Genre, sentence type, paragraph** do not move LAS. Sentence type gives only UPOS +0.10 — significant, but small.
- **Genre hurt on dev** (−0.29). In GUM there are ~13 training documents per genre, so the feature learns documents, not genre. On EWT (5 genres from the `sent_id` prefix, `EN_CTX_EWT=1`, training EWT + ESLSpok, as for the production model) genre is also neutral: test LAS 80.24 → 80.19, Δ −0.05 [−0.35; +0.24], MLAS −0.18 [−0.52; +0.17]. This number is for the production model: EWT is CC BY-SA, clean for commercial use.
- **Negative controls do not beat the baseline.** The model trained with gold context loses 4.5 LAS on someone else's context — so it really does use context. The model on random features loses 0.9.

**Stage 2: a real second pass.**
- The first pass is the `none` baseline.
- Context is derived from the first pass (`ctx::derived`):
  - genre, paragraph and speaker — from the document passport;
  - sentence type — by a rule over the tree (`ctx::stype_rule`). Accuracy: 89.5% on gold dev trees, 84–85% on the first-pass parse (test). In GUM `sub` is modality (would, could, should, might, may, must);
  - entities — only noun status: the lemma has already occurred in the document → giv.
- The second-pass model trains on the same kind of context. First-pass annotation for training sentences comes from jackknifing: 5 parts by document (`en folds`).
- Without jackknifing (rules over gold) the model learns to copy the "noun" mark, i.e. gold UPOS. At measurement it carries no new information, and with someone else's context such a model loses 14.5 LAS.

| GUM test | UPOS | UFeats | LAS | MLAS | ΔLAS [95%] | ΔMLAS [95%] |
|---|---:|---:|---:|---:|---:|---:|
| first pass (none) | 96.57 | 95.49 | 81.70 ± 0.15 | 71.61 ± 0.27 | — | — |
| second pass, all groups | 96.75 | 95.61 | 81.64 ± 0.21 | 71.70 ± 0.17 | −0.06 [−0.37; +0.27] | +0.09 [−0.26; +0.45] |
| second pass without genre | 96.70 | 95.54 | 81.74 ± 0.12 | 71.81 ± 0.08 | +0.05 [−0.22; +0.33] | +0.20 [−0.14; +0.54] |
| control: second pass, another context | 95.91 | 94.82 | 79.69 ± 0.05 | 68.93 ± 0.08 | −2.01 [−2.39; −1.63] | −2.68 [−3.11; −2.23] |
| control: trained on shuffled | 96.57 | 95.40 | 81.18 ± 0.16 | 71.10 ± 0.17 | −0.52 [−0.83; −0.21] | −0.51 [−0.87; −0.14] |

**Conclusion.**
- A second pass on these features does not raise LAS or MLAS. Only UPOS +0.18 and UFeats +0.12 are significant.
- The reason lies in the ceiling itself: it is carried by mention boundaries, and those cannot be derived from the first pass. The rule "the noun has already occurred" repeats what the first pass already knew.
- A candidate for real gain is mention boundaries from another source: the LLM or a separate mention detector on open data. The ceiling promises up to +0.75…+0.9 LAS if the boundaries are accurate.
- Experiment commands:
  - `EN_CTX=gold|shuffle` — training context;
  - `EN_CTX_EVAL=gold,shuffle,pred` — measurement contexts, each a separate row;
  - `EN_CTX_GROUPS=genre,stype,doc,entity` — feature groups;
  - `EN_CTX_ENT=span|type|info` — part of the entity;
  - `EN_CTX_SRC=rule` together with `EN_PASS1_DIR` and `EN_CTX_TRAIN_PASS1` — stage 2;
  - `EN_PRED_DIR` — test annotation for `en ctx-report`.
- Runs, reports and worklog — `$MOVA_DATA/runs/context-parse-2026-09-26/`, `$MOVA_DATA/worklog/2026-09-26/context-parse.md`. Models are not saved.

## Full UD annotation (`en ud-eval`)

Design decision (25.09.2026): do full UD right away, and use the other English treebanks as tests.

**The output is full, as in CoNLL-U:**
- UPOS;
- XPOS (PTB);
- FEATS;
- lemma;
- head and relation with subtype.

UPOS and FEATS (`ud.rs`) are backoff tables over what the graph already knows: PTB tag, form, relation, child flags (subject, aux, passive, fixed). They are not written into the graph, since they are derived from it, so the vertex stays 12 bytes.

**Agreement (26.09).** EWT 2.18 writes Number and Person on VBZ, VBP and VBD according to the subject, and only VerbForm=Fin on MD. The subject "you" gives Number=Sing|Person=2. So verb FEATS are computed in a second pass (`Ud::feats_sentence`). `agreement` finds the subject:
- its own nsubj or csubj, failing those — expl;
- for aux and cop — the head's subject;
- for conj — the subject of the first conjunct;
- for a relative pronoun — the antecedent.

The table key gets the subject's class, its Number and Person, and a coordination feature. Two more changes:
- a conjunct without its own subject or aux takes them from the first conjunct: "has seen and heard";
- "10.2" is a separate number form (NumType=Frac).

Results:
- EWT test: UFeats 94.04 → 95.66, MLAS 68.63 → 70.75.
- Rust features on top of Opus annotation (`annot` calibration, 300 sentences): UFeats 95.68 → 97.90.
- The cause was found by `annot feats-diff`: half of the wrong features were in agreement.
- ATIS dropped by −2 UFeats. There the imperative is VerbForm=Inf, and agreement is not written on VBP. The target is EWT conventions.

**How we measure.** Train on EWT + ESLSpok train, test on all English UD 2.18 treebanks. Metrics — CoNLL 2018 on gold tokens:
- LAS is computed without subtypes; LAS with subtypes is a separate column;
- CLAS, MLAS and BLEX are F1 on content words.

"—" means the treebank does not annotate this layer: ESLSpok — FEATS and lemmas, CHILDES — FEATS, ATIS, CTeTex, LinES and ParTUT — PTB tags.

| test | tokens | UPOS | XPOS | UFeats | Feats | AllTags | Lemmas | UAS | LAS | LAS with subtypes | CLAS | MLAS | BLEX |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| en_atis | 6580 | 74.50 | — | 74.39 | 72.02 | — | 79.35 | 68.59 | 63.01 | 61.49 | 54.51 | 22.53 | 43.90 |
| en_childes | 57400 | 93.38 | 93.48 | — | — | — | 96.63 | 85.87 | 80.01 | 79.73 | 75.32 | — | 72.48 |
| en_ctetex | 9273 | 87.03 | — | 58.07 | 57.89 | — | 91.02 | 71.01 | 61.49 | 60.48 | 50.27 | 13.26 | 45.06 |
| en_eslspok | 2266 | 95.23 | 95.45 | — | — | — | — | 90.25 | 86.23 | 85.88 | 84.21 | — | — |
| en_ewt | 25094 | 94.74 | 94.40 | 95.66 | 95.25 | 92.47 | 96.78 | 84.27 | 80.24 | 79.82 | 75.09 | 70.75 | 72.61 |
| en_gentle | 17799 | 88.47 | 86.76 | 87.30 | 86.72 | 80.53 | 94.33 | 74.72 | 67.94 | 67.43 | 59.73 | 48.18 | 56.02 |
| en_gum | 28397 | 94.26 | 93.80 | 90.84 | 90.52 | 87.46 | 96.64 | 81.46 | 76.85 | 76.25 | 70.62 | 60.08 | 67.65 |
| en_gumreddit | — | text removed from the treebank (source license) |
| en_lines | 19984 | 93.13 | — | 83.44 | 82.64 | — | 96.12 | 81.42 | 76.05 | 75.16 | 70.15 | 52.12 | 66.74 |
| en_littleprince | 6852 | 96.29 | 95.81 | 85.90 | 83.42 | 83.71 | 97.85 | 84.82 | 79.80 | 79.14 | 77.60 | 59.02 | 74.31 |
| en_partut | 3403 | 92.12 | — | 78.90 | 76.79 | — | 95.59 | 82.69 | 77.67 | 76.67 | 70.31 | 44.45 | 67.04 |
| en_pronouns | 1705 | 95.72 | 92.90 | 69.56 | 68.68 | 68.39 | 99.35 | 75.95 | 70.50 | 69.15 | 67.40 | 33.39 | 67.40 |
| en_pud | 21180 | 94.42 | 94.07 | 87.55 | 87.39 | 84.28 | 96.67 | 81.46 | 77.02 | 76.30 | 73.01 | 58.04 | 69.97 |

**What the tests show:**
- **EWT and close genres** (PUD, GUM, LittlePrince, ESLSpok, CHILDES) — UPOS 93–96, LAS 76–86.
- **ATIS** — lowercase queries without punctuation ("flights from dallas to baltimore"): UPOS 74, LAS 63. Users send such queries too, so this is a separate target for silver.
- **GENTLE and CTeTex** — rare genres (legal, poetic, technical texts): LAS 62–68.
- **LinES and ParTUT** have their own annotation conventions, so some of the discrepancies are not errors.
- **Pronouns** — a special pronoun set: UFeats 70.
- **GUMReddit** is distributed without text: Reddit words are replaced with "_".
- **GUM, GENTLE, ParTUT** are NC-licensed — tests only, not training.

### Beam: width for the task (`en beam-curve`)

Design note: for real-time answers the beam can be narrower, and for batches wider. There is one model (the parser is trained with beam 8); only the width at parse time changes. Tag / LAS with subtypes; speed is tokens per second on one core, tagger and parser together:

| tagger | parser | EWT | PUD | GUM | tok/s |
|---:|---:|---:|---:|---:|---:|
| 1 | 1 | 94.34 / 75.98 | 94.03 / 71.90 | 93.72 / 71.80 | 231K |
| 1 | 2 | 94.34 / 78.54 | 94.03 / 75.22 | 93.72 / 74.73 | 127K |
| 1 | 4 | 94.34 / 79.40 | 94.03 / 76.18 | 93.72 / 75.58 | 87K |
| 2 | 8 | 94.38 / 79.78 | 94.09 / 76.38 | 93.77 / 76.16 | 48K |
| 8 | 16 | 94.35 / 79.88 | 94.04 / 76.44 | 93.82 / 76.25 | 24K |
| 8 | 32 | 94.35 / 79.79 | 94.04 / 76.21 | 93.82 / 76.22 | 14K |

- **The tagger needs only beam 2** (now the default): a wider one gives nothing.
- **A parser trained with a beam drops without one** to 76.0. A greedy parser trained greedily gives 79.0. So the fastest parsing needs a separate greedy model, not a narrow beam.
- **Real time.** A query sentence is 10–20 words, so even beam 16 processes it in ~1 ms per core. Here the widest useful beam, 8–16, can be used.
- **Batches.** Throughput matters: beam 4 gives 99.5% of the quality twice as fast as 8, and the greedy model gives ~230K tok/s with LAS 79.0.

## Full cycle `en rt-eval` (text → graph → text)

| | EWT test | PUD test |
|---|---|---|
| lossless (the graph keeps vertex order) | **100%** exact | **100%** |
| generative (order from the tree) | 56.6% exact, chrF 92.8 (TnT and greedy parser — 54.6%, 92.4) | 34.0%, chrF 93.0 (was 30.9%, 92.2) |
| speed | ~2,600 sentences/s (TnT and greedy parser — ~10,500) | ~1,500 sentences/s (was ~5,900) |

The switch to Rust structures (verified: on the same tables all metrics are identical, graphs of 1177 forecasts — 0 discrepancies) sped things up:

| | before | after |
|---|---|---|
| tagger | 62K tok/s | 0.95M tok/s |
| parser | 170K tok/s | 550K tok/s |
| full cycle | 2.5K sentences/s | 10.5K sentences/s |
| model training | 13 s | 4.5 s |

chrF dropped by 0.4–0.5 points because of the first-word capitalization rule. It writes correctly ("I should bring…" instead of "I Should bring…"), but chrF is case-sensitive, so with a wrong order the correct spelling matches the original less.

## Commands

```
en train <train.conllu>... -- <model> [<check.conllu>...]                full UD annotator → model file (~40 s; with check files — load back and the same annotation)
en annotate <model> <in.conllu|in.txt>...                               CoNLL-U from a model file (load ~0.13 s): .conllu on gold tokens, anything else as text
en tok-eval|tag-eval|morph-eval|parse-eval|lin-eval|rt-eval|ud-eval|beam-curve <train.conllu>... -- <test.conllu>...
en rt-text <train.conllu>... -- <dom-train.txt> <dom-test.txt> [weight] general model vs one fine-tuned on the domain
en explain <train.conllu>... -- "sentence"...                           every step, human-readable
en graph <train.conllu>... -- <file.txt>...                             line graphs: lemma/tag/head/relation (version comparison)
en lexicon <agid infl.txt> <train.conllu>... [-- <data-dir>]            dictionary into code (data/)
en ctx-report <gold.conllu> <base>=<f1>,<f2>… <system>=<…>… [--focus <name>] [--boot N]
                                                                        context experiment: metrics by seed, paired bootstrap, breakdown, examples
en folds <in.conllu> <k> <dir>                                          documents into k parts (first-pass jackknife)
```

The library (`lib.rs`) is for domain crates. The first such crate is [`../weather`](../weather): forecasts and queries, two variants (US °F / international °C).

## Next

- Question inversion in the generator ("Should I…", not "I should…").
- Prepositional phrase attachment ("fog after 7am").
- Perceptron tagger (expected 95%+), more parser features / dynamic oracle.

**Density — not now** (design note, 24.09.2026: nothing needs compressing yet — the long-term plan is to compress trillions of statements, while here we are talking about thousands). The format for scale is already in place: id heaps, frozen numbers, 12-byte vertices.

## Licence

Elamur Cosmic License ([LICENSE.md](../LICENSE.md)). Third-party data keeps its own terms, as noted where it is used.
