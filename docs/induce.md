# The SLM derives its own tree rules

`world induce <train.conllu>... --dev <dev.conllu> --test <test.conllu>... --out <dir> [--iters N] [--min N]`
(world/src/induce.rs) — transformation-based, error-driven learning (after Brill 1995) of label rules on top of
the parser. Features are level-1 knowledge, not words: part of speech of the word and its head, finiteness,
position, whether the head has a subject or another object, matrix intransitivity, speech/motion verb class, light
verbs, the absurdity matrix scores (OBJ, SUBJ in the real or tale layer), idiom kind, measure/time nouns, archaic
adverbs, animacy, punctuation around. Candidates are conjunctions of 1–3 atoms built from the parser's label
errors; the greedy loop takes the best `fixed − broken` with precision ≥ 0.85, applies it and repeats. Every rule
is printed as a sentence with its counts.

Rules are scoped like the matrix layers: **domain rules** — everything induced on the training domain;
**general rules** — those that do not hurt a dev set of another domain.

## Runs (2026-10-02)

Train on the parser's errors where it has not seen the text (training on EWT train finds only 2 rules: the
parser has memorised it, LAS 93.65).

| run | train | rules | tales test LAS | EWT test LAS |
|---|---|---|---|---|
| v1 | EWT train + tales even | 2 | 76.85 → 77.03 (p < 0.001) | 79.82 → 79.82 |
| v3, domain rules | tales even (1 044 sentences) | 12 | 76.85 → 77.32 (p < 0.001) | 79.82 → 79.79 |
| v3, general rules (not hurting EWT dev) | same | 8 | 76.85 → 77.17 (p < 0.001) | 79.82 → 79.84 |
| hand-written repairs v4 (`world absurd-repair`) | — | 4 | 76.85 → 76.98 | 79.82 → 79.82 |

The first rule found in every run is the inverted subject — "obj → nsubj IF the head is a speech/motion verb, not
an idiom, no comma or quote before" — rediscovered without being told, and better than the hand-written version
(56 fixed / 2 broken on the tales test). Others: nsubj:pass → nsubj when the head has another object; parataxis →
conj; obj → obl:unmarked for measure/time nouns the matrix finds absurd as objects; nsubj → obl:unmarked for a
measure/time noun when the verb already has a subject. Rules: `data/runs/induce-v3/rules.tsv`.

Next: induce on more held-out domains (EWT folds parsed by a model that did not see them), structural rules (head
changes), and wire the domain/general rule sets into the reader in place of the hand-written repairs.

## Hidden rules come back (2026-10-02)

`world rediscover <book-list> [--sentences N] [--iters K]` — leave-one-out over the shipped rules: the teacher
parses book sentences with all rules, the student without one hidden rule; the inducer turns the student into the
teacher with no gold. On 8 000 sentences from public-domain books: **12 of 12 hidden rules rediscovered, each
as the same rule** (e.g. obj → nsubj for speech/motion verbs: 182 fixed, 0 broken). Caveat: the hidden rules were
induced in the same feature space, so this checks that the inducer is stable and finds them from unlabelled
books; the harder test is hiding the hand-written repairs (other formulations) or a piece of knowledge (a matrix
layer, the idiom base) and seeing what the inducer finds instead.

## What each piece of knowledge gives the inducer (ablation, 2026-10-02)

`INDUCE_WITHOUT=<group> world induce …` hides one feature group; same train (tales even), same tests.

| hidden | rules | tales test, domain rules | tales test, general rules | EWT test, general rules |
|---|---|---|---|---|
| nothing | 12 | 77.32 | 77.17 | 79.84 |
| syntax (finiteness, position, subject/object of the head) | 6 | 77.11 | 76.89 | 79.82 |
| matrix (OBJ, SUBJ, intransitivity) | 8 | 77.25 | 77.01 | 79.82 |
| verb classes (speech/motion, light verbs) | 10 | 77.23 | 77.21 | 79.86 |
| noun lists (measure/time, archaic, animacy) | 10 | 77.29 | 77.14 | 79.83 |
| punctuation around | 10 | 77.30 | 77.14 | 79.86 |
| idioms | 12 | 77.32 | 77.18 | 79.86 |

(no repairs: tales 76.85, EWT 79.82.) Syntax and the matrix carry the most: without them the inducer finds half or
two thirds of the rules and loses a third to a half of the gain. Verb classes partly substitute for each other
(the general set even gains without them). Idioms give nothing to label repair — they matter for the absurdity
alarms and the meanings, not for labels. Differences between rows are small (tens of words); the "nothing" row
against no repairs is significant (p < 0.001).
