# coref-lexicon

## descriptant

### coref-lexicon

A gender, number and animacy lexicon of common nouns, counted from Mova's own parses (`en annotate` with `en.bin`)
over high-precision noun–pronoun dependency paths after Bergsma and Lin 2006, smoothed into per-lemma shares; the
mention features it gives the coreference sieve (`mention.rs`) and the frame reader (`prag::frames::lex_table`).

## approximant

### Forward model

For a sentence s, `hits(s)` is the sorted list of (lemma, class, path) for every pronoun or relative word that a
path joins to a countable noun. For a table T with counts c and corpus prior π over masc/fem/neut,
p_k = (c_k + α π_k) / (c_masc + c_fem + c_neut + α) and p_anim = (c_masc + c_fem + c_who + α π_anim) /
(c_masc + c_fem + c_neut + c_who + c_which + α). `feats` returns ANIM when p_anim ≥ τ, INAN when 1 − p_anim ≥ τ
(each with at least `min_n` counts), gender MASC/FEM/NEUT when that share ≥ τ, MASC|FEM when the personal share is;
contradictory or empty results are None. Defaults τ = 0.8, min_n = 3, α = 2.

## invariant

### Invariant and cost

Counting is a pure function of the trees: block boundaries do not change counts (a block is a whole number of
sentences and documents do not carry state). The table text is a function of the merged counts (BTreeMap order).
`feats` never gives NEUT together with ANIM nor INAN together with MASC or FEM. A missing, `off` or refused table
gives the features of the build without the lexicon.

hits O(n²) per sentence; counting linear in words plus block parsing; build O(K log K); lookup O(log L).

## models

## behavior_complement

The lexicon never reads a proper noun (no gender from names), a noun in a compound, flat name or coordination, a
pronoun in another quote than the noun, a negated or prepositional predicate, or a plural noun with a singular
pronoun. It never decides a lemma seen fewer than `min_n` times, and never overrides gender or animacy that the tree,
the closed lists or a title already gave (mention.rs uses it only when gender is unknown). An internal-only table
(Wikipedia-derived: `# source wikipedia`, `# use: internal`, or a licence line saying internal) is refused unless
`COREF_GENDER_INTERNAL=1`; CC BY-SA alone is not internal (the released table is public CC BY-SA); the unit tests never read the data directory.

## reverse_reconstruction

From a row one recovers the per-class totals and, from the `paths`, `sources` and `example` columns, which paths and
sources contributed and one smallest example sentence id; not the other sentences. From a Feats value one recovers
only the set of rows clearing the same thresholds. From the rotated control table no lemma's own counts are
recoverable without the rotation offset.

## path_precision

Precision of each path against gold coreference of GUM dev (gold trees, measured `coref lexicon paths`):
poss 20 of 25, pred 22 of 25, refl 0 of 1, say 0 of 2; relpron is not measurable (GUM does not annotate relative
pronouns). Pairs over the corpus (13.87 million sentences): poss 336 169, pred 257 958, refl 62 016, say 33 005,
appos 2 360, relpron who 187 237 and which 302 652; the table header splits them by class.

## decision

The decision is a threshold on smoothed shares; between τ = 0.7 and 0.9 and min_n 3 to 30, GUM dev CoNLL F1 stays
within 0.1 (gold trees 75.22–75.29, SLM trees 70.61–70.69 with singletons).

## licence_scope

Sources: 5 110 public-domain (EU/UA) Project Gutenberg books of `corpora/tales/free` and the school's lessons (ours).
Parsed by `en.bin` (CC BY-SA 4.0 model). The table lives outside the repository and ships separately in the release's model archive under CC BY-SA 4.0;
the Wikipedia-derived table is internal comparison only and never shipped.

## tests

### Evidence

`cargo test --release -p coref`: 25 library tests pass, including the four lexicon tests. Negative controls inside
them: negation, prepositional predicate, a name, a plural noun with «himself» and a possessive inside the subject give
no pair; `lexicon_feature_keeps_it_from_a_person_noun` shows «it» → farmer without the table and → stone with it.

## History

### 2026-11-04 · descriptant

Epoch 2026-11-04: first model, written with the code; the predictions below were stated before the corpus runs.

### 2026-11-04 · approximant

Prediction (2026-11-04, before the runs): CoNLL F1 on GUM dev within ±0.5 (GUM is mostly names and abstract nouns),
a gain on pronouns with person-noun antecedents; the rotated table lower than both.

### 2026-11-04 · path_precision

Precision of each path against gold coreference of GUM dev (gold trees, measured 2026-11-04, `coref lexicon paths`):
poss 20 of 25, pred 22 of 25, refl 0 of 1, say 0 of 2; relpron is not measurable (GUM does not annotate relative
pronouns). Pairs over the corpus (13.87 million sentences): poss 336 169, pred 257 958, refl 62 016, say 33 005,
appos 2 360, relpron who 187 237 and which 302 652; the table header splits them by class.

### 2026-11-04 · tests

Heading `### Evidence — 2026-11-04` stays in tests as `### Evidence`.

Measured 2026-11-04 (dev only, CPU; runs in `$MOVA_DATA/runs/coref-gender-2026-11-04`). The prediction held: GUM dev
CoNLL F1 without singletons 64.36 → 64.29 (gold trees) and 58.81 → 58.87 (SLM trees); third-person pronouns right
498 → 500 and 470 → 478; rotated table 63.74 / 58.33 and 481 / 454 (red). Frame bench dev coref F1 0.471 → 0.489,
rotated 0.466. School with the reader's opt-in: 24 courses 12 306 → 12 305, books 7 339 → 7 339, rotated 12 285 and
7 283; so the reader keeps it opt-in. Switch off = main byte for byte (GUM eval, frame bench, four courses with the
opt-in build); two runs with the table identical; `lexicon_feature_keeps_it_from_a_person_noun` goes red with
`COREF_X=no-gender-table`.

### Undated · descriptant

State: one tree at a time while counting; `Counts` maps keyed by (lemma, source, path, class); a process-wide table
loaded once from `COREF_GENDER`, `$MOVA_DATA/coref/gender.tsv` or `~/mova-data/coref/gender.tsv`; thresholds from
`COREF_GENDER_TAU`, `COREF_GENDER_MIN`, `COREF_GENDER_ALPHA`.
