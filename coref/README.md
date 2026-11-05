# coref — SLM coreference: a sieve of rules with a reason for every link

The annotator keeps context and resolves pronouns (`docs/context.md`, "Document memory").

A separate crate that depends on `en` (`en = { path = "../en" }`), not the other way round.

This is the in-house coreference of the small language model (SLM):
- classical and deterministic, without the teacher and at no cost;
- transparent: every link carries the sieve id and its evidence;
- evaluation — GUM from UD 2.18, MUC / B³ / CEAF-e / CoNLL F1 scorer in Rust.

The frame reader has its own pronoun resolver and can call this crate's sieve instead (`PRAG_COREF`; see [how roles and links are found](../docs/pragmatics.md#how-roles-and-links-are-found)).

## How it works

```
UD tree (gold or en annotate) → mentions (mention.rs) → sieves (sieve.rs) → CorefUD Entity= (corefud.rs)
                                                                          → evaluation (score.rs, eval.rs)
```

### Mentions from the tree

**Heads:**
- NOUN, PROPN;
- PRON, except `expl` and "'s" in *let's*;
- DET and NUM in a nominal role: subject, object, oblique, nmod, appos, conjunct…;
- NUM-parataxis — references to sources "[17]"; in GUM these are mentions;
- ADV *home*.

**Not heads:**
- flat, fixed, `nmod:desc`;
- titles (Mr., Dr.…);
- NOUN-compound. A PROPN-compound is a head: a proper name in a modifier is referential, "cell phone" is not;
- non-nominal idioms with a noun: *for example, in fact, a lot of, kind of…*

**Span** — the subtree of the head under CorefUD rules:
- only noun-phrase dependents: det, amod, nummod, compound, flat, nmod, acl, cc/conj of nominal conjuncts;
- without the preposition (`case`) before the head;
- without the clause around a predicative noun (nsubj, cop, aux, mark, obl…);
- without appos: in GUM an apposition is a separate mention;
- punctuation at the edges is stripped, but a bracket with its pair inside is kept.

**Coordination** "John and Mary" gives two mentions with one head: the whole group and the first conjunct. The CRAC scorer tells them apart by span. The group is output only when something refers to it.

**Features** — from UD FEATS (Number, Gender, Person, PronType, Poss, Reflex), hand-made closed lists and the noun lexicon Mova counted from its own parses (below):
- pronoun table;
- titles;
- 60 nouns with gender in the word itself (mother, king…);
- speech verbs;
- idioms.

What is not known stays unknown and matches anything, like "unknown" in Lee et al. 2013. **Gender is not guessed from a name** ([context](../docs/context.md#document-memory)). Instead there are:
- a name from `# speaker` / `# addressee` — evidence of animacy;
- "who" or "which" in a relative clause — animate or not;
- Mr./Mrs. — gender.

Cluster features are the union of the features of its mentions.

### The noun lexicon (`src/lexicon.rs`)

When the tree, the closed lists and titles say nothing, a common noun takes gender and animacy from a lexicon that
Mova counted from its **own parses** (after Bergsma & Lin 2006, `bergsma-2006-bootstrapping-path-based-pronoun-resolution`;
Lee et al. 2013 use their table the same way):

- **paths** — a noun and a third-person pronoun joined by a dependency path that rarely joins two entities, each with a
  test: `refl` («the farmer hurt himself», «the king himself»), `poss` («the hen sat on her eggs»), `say` («the miller
  said he was poor», outside quotes), `pred` («she was a widow», no negation, no preposition), `appos`, and `relpron`
  (a relative clause on the noun starting with who/whom/whose or which);
- **counts** — classes he, she, it, they, who, which per noun lemma; only NOUN (never a name: gender is not guessed
  from a name), not in a compound, a flat name or a coordination; a plural noun with a singular pronoun is dropped;
- **decision** — smoothed shares p = (n + α·prior) / (N + α), α = 2: animate if he+she+who ≥ 0.8, inanimate if
  it+which ≥ 0.8, gender he / she / it / he-or-she at 0.8, at least 3 counts; mixed nouns (cat, horse, ship, sun, family)
  stay unknown;
- **transparency** — every row keeps its sources, paths and one example sentence id; a link the lexicon helped carries
  its counts in `explain` («lexicon «farmer»: he 702 she 18 it 13 …»).

```
en annotate $MOVA_DATA/en/models/en.bin pool.tsv | coref lexicon count tales - > part.tsv   # one part per corpus chunk
coref lexicon build $MOVA_DATA/coref/gender.tsv part-*.tsv --label "<provenance>"
coref lexicon paths <gold.conllu> [--trees f]      # precision of each path against gold coreference
coref lexicon rotate <table> <out> <k>             # negative control: every row moved k lemmas on
coref lexicon show <table> <lemma>…  |  coref lexicon compare <a> <b>
```

The table is data, outside the repository: `$MOVA_DATA/coref/gender.tsv`, or `COREF_GENDER=<path>`. `COREF_GENDER=off`
turns it off everywhere, `COREF_X=no-gender-table` in the sieve only; `COREF_GENDER_TAU`, `COREF_GENDER_MIN`,
`COREF_GENDER_ALPHA` change the decision. An internal-only table (counted from Wikipedia: a `# source wikipedia` line, `# use: internal`, or a licence line
that says internal) is refused unless `COREF_GENDER_INTERNAL=1`; CC BY-SA alone is not internal — the released
table is a public CC BY-SA file. Without a table the sieve is byte for byte the sieve without the lexicon (GUM dev, both
tree sources, 2026-11-04).

**The table of 2026-11-04**: 13.87 million sentences parsed by `en.bin` — 5 110 public-domain (EU/UA) Project Gutenberg
books of `corpora/tales/free` (13.51 million) and the school's lessons (0.36 million); 23 666 lemmas, 6 222 decided;
pairs: poss 336 169, pred 257 958, refl 62 016, say 33 005, appos 2 360, who 187 237, which 302 652. Parsing took
45 minutes on 6 CPU processes. Two counts of the school part and two builds were byte-identical. The internal
comparison table from the Wikipedia school's 127 096 sentences (CC BY-SA, never shipped) decides 199 lemmas; of 140
decided in both, animacy agrees on 124 (88.6%).

**Measured** (2026-11-04, dev only; the switch off = main):

| measure | without | with the table | rotated table (control) |
|---|---|---|---|
| GUM dev, gold trees: CoNLL F1 with / without singletons | 75.25 / 64.36 | 75.22 / 64.29 | 74.86 / 63.74 |
| GUM dev, gold trees: 3rd-person pronouns right | 498 of 741 | 500 | 481 |
| GUM dev, SLM trees: CoNLL F1 | 70.60 / 58.81 | 70.69 / 58.87 | 70.29 / 58.33 |
| GUM dev, SLM trees: 3rd-person pronouns right | 470 of 743 | 478 | 454 |

Precision of the paths on GUM dev gold (`coref lexicon paths`): poss 20 of 25, pred 22 of 25. τ from 0.7 to 0.9 and
the minimum from 3 to 30 move CoNLL F1 by at most 0.1. The lexicon fixes pronouns whose antecedent is a person noun
(«his», «him», «he»); most «it» errors on GUM are events, statements and inanimate rivals, which gender cannot tell
apart.

The frame reader (`prag`) can use the same table (`PRAG_VN=gender-table`, opt-in). Measured 2026-11-04 on the school
(CPU, train and cv courses only):

| measure | without | with the table | rotated table (control) |
|---|---|---|---|
| frame bench dev (`world frames bench dev prons`): overall F1 / coref F1 | 0.738 / 0.471 (89 right) | 0.741 / 0.489 (92) | 0.737 / 0.466 (88) |
| 24 primary courses: tests right (odd / even), exam | 12 306 (6 162 / 6 144), 3 564 | 12 305 (6 156 / 6 149), 3 562 | 12 285, 3 560 |
| the books' course: tests right (odd / even), exam | 7 339 (3 721 / 3 618), 2 195 | 7 339 (3 704 / 3 635), 2 179 | 7 283, 2 191 |
| Wikipedia school: train / cv-a / cv-b | 15 672 / 7 654 / 8 014 | 15 671 / 7 653 / 8 013 | 15 672 / 7 650 / 8 018 |

All course controls stayed red. The rotated table loses 21 and 56 tests, the real table neither gains nor loses: the
pupil reads a few pronouns better (the bench) but its answers rarely depend on them; 557 tests of the 24 courses
flip, 280 one way and 277 the other, as often with a pronoun in the question as without. So the reader keeps the
lexicon opt-in. The Wikipedia school's lead «It» goes to the nearest noun («It sits on …» → «state») for another
reason: the frame reader never resolves «it» to a name (a name is a person to it), and the lexicon has no names.
Two runs with the table gave byte-identical traces (24 courses and the Wikipedia courses); the runs without it were
byte-identical across two builds.

### Sieves — from the most precise

The architecture follows Raghunathan et al. 2010 ([raghunathan-2010-multi-pass-sieve-coreference-resolution](https://research.elamur.ai/free/en/papers/raghunathan-2010-multi-pass-sieve-coreference-resolution/)) and Lee et al. 2013 ([lee-2013-deterministic-coreference-resolution-based-entity](https://research.elamur.ai/free/en/papers/lee-2013-deterministic-coreference-resolution-based-entity/)):
- only a mention that is first in its cluster is linked;
- an indefinite one (a/an, quantified, bare plural) is not linked, except on an exact match.

**Candidate order — in the spirit of Hobbs.** Hobbs 1978 itself is missing from the library; it is taken from the descriptions in Raghunathan 2010 and Lee 2013:
- breadth-first, left-to-right tree traversal: by head depth, then by position;
- first the mention's clause, then the rest of the sentence, then previous sentences;
- in previous sentences, left to right for pronouns (subject first), right to left for nominals.

Heads higher in the tree come first, as in Haghighi & Klein 2009 ([haghighi-2009-simple-coreference-resolution-rich-syntactic](https://research.elamur.ai/free/en/papers/haghighi-2009-simple-coreference-resolution-rich-syntactic/)).

| id | what it does | default |
|---|---|---|
| `speaker` | "I" of one voice is one person. A voice is a speaker from `# speaker`, a separate quotation or the narrator. "I" in a quotation is its author, i.e. the subject of the speech verb outside the quotes. "we" and "you" of one voice go together. "you" addressed to an addressee is that addressee's earlier "I". Rules follow Lee 2013, §3.3.1 and Baldwin 1997 ([baldwin-1997-cogniac-high-precision-coreference-limited](https://research.elamur.ai/free/en/papers/baldwin-1997-cogniac-high-precision-coreference-limited/)) | yes |
| `exact` | exact text match of nominal mentions | yes |
| `relaxed` | text match up to and including the head. After the head — only relative clauses (acl); a quantified antecedent — no | yes |
| `appos` | `appos` in the tree | yes |
| `pred` | predicative noun: subject — cop — noun, without negation | yes |
| `relpron` | relative pronoun → the noun the relative clause is attached to. Memory only: relatives are not annotated in GUM, so they do not go into the output | yes |
| `acronym` | "AFP" — capital letters of a name | yes |
| `reflexive` | reflexive pronoun → subject of its predicate (principle A) | yes |
| `strict_a` | head is among the cluster heads + cluster words ⊆ + modifiers ⊆ + not i-within-i | yes |
| `strict_b` | the same without modifiers | yes |
| `strict_c` | the same without words | no |
| `proper_head` | the same proper head, without conflicting proper names and numbers in modifiers | yes |
| `relaxed_head` | the proper head is among the cluster words | no |
| `deixis` | an event «it» and a bare «this/that» → the clause before it, added as a mention (details below) | no |
| `pronoun` | 3rd-person pronoun, details below | yes |
| `nearest` | baseline: nearest preceding agreeing mention | baseline only |

**What «it» is** (`src/it.rs`, before the sieves; `PRAG_OFF=it-class` turns it off, `it-pleonastic` and `it-event` each half). Tests on the predicate, after Loáiciga et al. 2017 and Eckert and Strube's context test in Kolhatkar et al. 2018:
- **pleonastic**, refers to nothing: weather verbs («it rained»), a copula with a time or weather word («it was late»), a copula, a raising verb or a passive with a clause («it is clear that…», «it seems that…», «it takes years to…»), an object before a small clause («made it clear that…»). Such an «it» is not output: in GUM it is not a mention;
- **event**, refers to a clause: the subject of *happen, occur, mean, matter*, of a copula with *true, false, right, wrong, possible, a pity, the problem, the reason, a mistake…*, or of «it is because…». Such an «it» is kept from nominal antecedents;
- **anaphoric**: the rest, for the `pronoun` sieve.

The SLM's frame reader does not use this test.

**The `deixis` sieve** (off): an event «it» or a bare «this/that» (not before a noun) is linked to the clause before it — the clause to the left in the same sentence, else the main clause of the previous sentence (Yu et al. 2022's baseline, Kolhatkar et al. 2018's search order). The clause is added as a mention with the predicate as its head. The right clause is rarely the nearest one, so the sieve is not in the default.

**The `pronoun` sieve:**
- **cluster agreement** in number, gender, animacy and person;
- **principle B** at cluster level: a personal pronoun is not bound by a co-argument of its predicate. The test `principle_b_is_what_blocks_who` is the negative control;
- **ban on quantified antecedents** (everyone, every country) — a bound variable;
- **window ≤ 3 sentences** (Lee 2013);
- **animacy preference for he/she**: first a cluster with evidence of animacy, then any agreeing one;
- **notional «they»** (Zeldes 2018, `PRAG_OFF=coref-notional`): only when no candidate agrees strictly, *they / them / their* may take a singular collective noun (*family, government, team, company, police…*, a hand-made list). The SLM's frame reader does not use this fallback: in the everyday lessons «they» is mostly people in general.

The speaker constraint (Lee 2013) lives in `compatible`: clusters with "I" of different voices do not merge.

### Transparency

**`coref explain`** — the chains of a document: each mention with its sentence and bounds, the sieve id and evidence. Example from GUM dev (gold trees):

```
## chain 2 (53 mentions): Number=Sing Gender=Masc Animacy=Anim Person=3
  - GUM_bio_emperor-2:1-17 «Joshua Abraham Norton (c. 1818 – January 8, 1880), known as Emperor Norton» — first mention
  - GUM_bio_emperor-2:20-42 «a citizen of San Francisco, California, who … "Norton I, Emperor of the United States"» ← [pred] «Joshua Abraham Norton …» (GUM_bio_emperor-2:1) —cop «was»— «a citizen …» (GUM_bio_emperor-2:21)
  - GUM_bio_emperor-2:28-28 «who» [memory only] ← [relpron] relative «who» in the relative clause of «a citizen …» (GUM_bio_emperor-2:21)
  - GUM_bio_emperor-2:32-32 «himself» ← [reflexive] «himself» → subject «who» (GUM_bio_emperor-2:28) of predicate «proclaimed»
  - GUM_bio_emperor-3:1-1 «He» ← [pronoun] «he» → «a citizen …» (GUM_bio_emperor-2:21): candidate #1 in Hobbs order, 1 sent. back; cluster Number=Sing Gender=Masc Animacy=Anim; animacy preference
  - GUM_bio_emperor-5:10-10 «his» ← [pronoun] «his» → «his» (GUM_bio_emperor-5:5): candidate #4 in Hobbs order, 0 sent. back; …; rejected: «his mother» (GUM_bio_emperor-5:6): Gender Masc≠Fem
  - GUM_bio_emperor-6:1-1 «Norton» ← [strict_a] head «norton» is in the cluster; words ⊆ ["born", "england", "norton"]; modifiers [] ⊆ []; …
```

**`coref resolve`** — CoNLL-U with `Entity=` in MISC (CorefUD, [nedoluzhko-2022-corefud-1-0-coreference-meets](https://research.elamur.ai/free/en/papers/nedoluzhko-2022-corefud-1-0-coreference-meets/)):
- header `# global.Entity = eid-etype-head-other`;
- `head` — position of the head within the span;
- `other` — id of the sieve that attached the mention: `new` — first in the chain, `sgl` — singleton;
- old `Entity`, `Bridge`, `SplitAnte` are removed.

The resolver does not see the gold standard: test `resolver_ignores_gold_entities`; in addition, every `eval` compares the output from MISC with and without Entity (fail-fast).

## Scorer

**Metrics** (`src/score.rs`):
- MUC — Vilain et al. 1995 ([vilain-1995-model-theoretic-coreference-scoring-scheme](https://research.elamur.ai/free/en/papers/vilain-1995-model-theoretic-coreference-scoring-scheme/));
- B³ — Bagga & Baldwin 1998;
- CEAF-e and CEAF-m — Luo 2005 ([luo-2005-coreference-resolution-performance-metrics](https://research.elamur.ai/free/en/papers/luo-2005-coreference-resolution-performance-metrics/)). The best entity assignment is Kuhn–Munkres, separately per connected component of the overlap;
- CoNLL F1 — mean of MUC, B³ and CEAF-e.

**Formulas** for predicted mentions without manipulations — Pradhan et al. 2014 ([pradhan-2014-scoring-coreference-partitions-predicted-mentions](https://research.elamur.ai/free/en/papers/pradhan-2014-scoring-coreference-partitions-predicted-mentions/)).

**Mention matching** — by head, as in the main CRAC 2023–2026 metric ([novak-2025-findings-fourth-shared-task-multilingual](https://research.elamur.ai/free/en/papers/novak-2025-findings-fourth-shared-task-multilingual/), [novak-2026-findings-fifth-shared-task-multilingual](https://research.elamur.ai/free/en/papers/novak-2026-findings-fifth-shared-task-multilingual/)):
- the span head as in udapi `corefud.MoveHead` ([popel-2017-udapi-universal-api-universal-dependencies](https://research.elamur.ai/free/en/papers/popel-2017-udapi-universal-api-universal-dependencies/)): the word whose head is outside the span; among several — the highest, then non-punctuation, then the first;
- several mentions with one head are told apart by span: first exact, then the largest overlap;
- exact span is also available for reference.

**Singletons:** with and without them. Without them, singleton entities are removed on both sides before matching. The total over documents is the sum of numerators and denominators.

**Checks that can show red:**
- **the example from Pradhan et al. 2014, §4.** K = {a,b,c}{d,e,f,g}, R = {a,b}{c,d}{f,g,h,i}:
  - MUC 0.40 / 0.40;
  - B³ 35/84 and 0.50;
  - CEAF-e 0.65 and 1.3/3;
  - CEAF-m 4/7 and 0.50.
  In the paper B³ F1 = "0.46", computed from the rounded 0.42 and 0.50; the exact value is 0.4545;
- **negative controls:**
  - one giant cluster: MUC recall 1, but CoNLL < 0.7;
  - everything separate: MUC 0, B³ precision 1, CoNLL < 0.6;
- **Kuhn–Munkres versus brute force** over permutations;
- **before every evaluation — a self-check:** the Pradhan example and "gold against itself = 100" (fail-fast);
- **`coref score gold output`** (from files) gives the same numbers as the in-memory `eval`.

## Data and licenses

**GUM** (UD 2.18, `UD_English-GUM`) — CC BY-NC-SA:
- evaluation and research only;
- rules and thresholds were tuned on dev only;
- test is run once, after freezing the code;
- no lexicons from GUM.

**SLM trees** — `en annotate $MOVA_DATA/en/models/en.bin` on gold tokens (EWT + ESLSpok, CC BY-SA 4.0). Trees are supplied with `--trees`:
- comments and Entity are taken from the gold file;
- LEMMA, UPOS, XPOS, FEATS, HEAD, DEPREL — from the trees;
- a mismatch of sentences or forms is an error.

**Hand-made lists** in `src/` are our own work, row `coref-hand-lists` in `en/data/train-licenses.tsv`:
- pronouns;
- titles;
- gender of 60 nouns;
- speech verbs;
- idioms.

**The noun lexicon** (`$MOVA_DATA/coref/gender.tsv`, not in git) is counted by Mova from its own parses of public-domain books and its own lessons; never from GUM, and the Wikipedia table only for an internal comparison.

No third-party lexicons. What is missing is listed under "Limits".

## How to run

```
cd coref && cargo build --release -j 2
G=$MOVA_DATA/en/ud/ud-treebanks-v2.18/UD_English-GUM
.../target/release/en annotate $MOVA_DATA/en/models/en.bin $G/en_gum-ud-test.conllu > test.slm-trees.conllu
../target/release/coref eval $G/en_gum-ud-test.conllu --out <dir>                               # gold trees
../target/release/coref eval $G/en_gum-ud-test.conllu --trees test.slm-trees.conllu --out <dir>  # SLM trees
../target/release/coref resolve <in.conllu> [--trees f] > out.conllu    # CorefUD
../target/release/coref explain <in.conllu> [doc-id] [--trees f]        # chains with reasons
../target/release/coref score <gold.conllu> <output.conllu> [--exact]   # scorer
../target/release/coref mentions <gold.conllu> [--trees f]              # mention detection against gold
```

`coref eval … --label <name>` names the input in the report header (default «gold trees» or «SLM trees»).

**Dev experiment switches** — environment variables; the defaults are what dev selected:
- `COREF_MD` (mentions): `compound`, `nopcompound`, `appos-in`, `advmod`, `desc`, `adv`, `noidiom`;
- `COREF_X` (sieves): `pron-recency`, `win=N`, `itwin=N`, `no-anim-pref`, `exact-noindef`, `pred-noit`, `relaxed-any`, `quant-ante`, `indef-ante`, `coord-sgl`;
- `PRAG_OFF` (steps that are on, the same switch as the frame reader's): `it-class`, `it-pleonastic`, `it-event`, `it-ev-verb`, `it-ev-pred`, `it-ev-because`, `dem-deixis`, `coref-notional`.

## Limits

- **The noun lexicon is counted, not checked.** Parser errors become counts: an adjective tagged NOUN after a copula («he was content») gets person counts, and a relative clause attached to the wrong noun gives a wrong who/which. Animals and ships stay mixed and unknown. It cannot help «it» that refers to an event or a statement, the commonest «it» error, and a demonstrative without gender can still win in Hobbs order.
- **The lexicon's licence.** The released lexicon, counted from public-domain tales and the school's lessons, is distributed separately under CC BY-SA 4.0 (in the model archive, not in the repository); the Wikipedia-derived comparison table stays internal and is not shipped. See `licence/DATA.md`.
- **Same text or head, different entities.** Generic and repeated groups («some politicians» ~ «some politicians»), different persons with one head («[George W.] Bush» ~ «[George H. W.] Bush») and part versus whole («Senegal» ~ «Southern Senegal») are linked by `exact` and `strict_a`.
- **Parser errors carry over on SLM trees.** The parser can attach reference numbers "[1–3]" as appos to nouns, which misleads `appos` and `pronoun`.
- **No semantic classes of mentions** (person, place, organization, time…) for "it" and for `relaxed_head` (in Lee — the NER type).
- **Discourse deixis is weak.** The `deixis` sieve exists but is off: the right clause is rarely the nearest one.
- **No demonyms** (Israel ~ Israeli): Lee's `demonym` sieve is not implemented, and a list with a commercial licence is missing.
- **Rules, not a trained model.** Moving to candidate ranking needs cleanly licensed data: GUM is NC, OntoNotes is LDC; LitBank, FantasyCoref and GAP are candidates with licences to check. The scorer and `link_stats` are ready for such training.
- **Salience weights of Lappin & Leass 1994** ([lappin-1994-algorithm-pronominal-anaphora-resolution](https://research.elamur.ai/free/en/papers/lappin-1994-algorithm-pronominal-anaphora-resolution/)) are not tried as a replacement for or addition to Hobbs order. Parameters from the paper:
  - weights: recency 100, subject 80, existential 70, object 50, indirect object and oblique 40, head 80, non-adverbial 50;
  - halving per sentence;
  - parallelism +35, cataphora −175.
- **Demonstrative pronoun sieves** (`demonstratives` in `Config`) are disabled; no cataphora.
- **Preposed acl in the span** ("Born in England, Norton"): in GUM the mention is only "Norton".
- **No second parsing pass with memory** (`docs/context.md`, "Multi-pass parsing"): the clusters exist, but the tree is not re-parsed.
