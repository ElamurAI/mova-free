# coref — SLM coreference: a sieve of rules with a reason for every link

**Design note:** the annotator must be able to keep context and resolve pronouns (`docs/context.md`, "Document memory").

A separate crate that depends on `en` (`en = { path = "../en" }`), not the other way round.

This is the first in-house coreference of the small language model (SLM):
- classical and deterministic, without Opus, without `claude -p` and at no cost;
- transparent: every link carries the sieve id and its evidence;
- evaluation — GUM from UD 2.18, MUC / B³ / CEAF-e / CoNLL F1 scorer in Rust.

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

On GUM dev with gold trees:
- mentions (by head): recall 92.8%, precision 89.9%;
- exact span — 95.5% among matches.

**Features** — only from UD FEATS (Number, Gender, Person, PronType, Poss, Reflex) and hand-made closed lists:
- pronoun table;
- titles;
- 60 nouns with gender in the word itself (mother, king…);
- speech verbs;
- idioms.

What is not known stays unknown and matches anything, like "unknown" in Lee et al. 2013. **Gender is not guessed from a name** (`docs/context.md`, "Document level"). Instead there are:
- a name from `# speaker` / `# addressee` — evidence of animacy;
- "who" or "which" in a relative clause — animate or not;
- Mr./Mrs. — gender.

Cluster features are the union of the features of its mentions.

### Sieves — from the most precise

The architecture follows Raghunathan et al. 2010 (`papers/r/ra/raghunathan-2010-multi-pass-sieve-coreference-resolution`) and Lee et al. 2013 (`papers/l/le/lee-2013-deterministic-coreference-resolution-based-entity`):
- only a mention that is first in its cluster is linked;
- an indefinite one (a/an, quantified, bare plural) is not linked, except on an exact match.

**Candidate order — in the spirit of Hobbs.** Hobbs 1978 itself is missing from the library; it is taken from the descriptions in Raghunathan 2010 and Lee 2013:
- breadth-first, left-to-right tree traversal: by head depth, then by position;
- first the mention's clause, then the rest of the sentence, then previous sentences;
- in previous sentences, left to right for pronouns (subject first), right to left for nominals.

Heads higher in the tree come first, as in Haghighi & Klein 2009 (`papers/h/ha/haghighi-2009-simple-coreference-resolution-rich-syntactic`).

| id | what it does | default |
|---|---|---|
| `speaker` | "I" of one voice is one person. A voice is a speaker from `# speaker`, a separate quotation or the narrator. "I" in a quotation is its author, i.e. the subject of the speech verb outside the quotes. "we" and "you" of one voice go together. "you" addressed to an addressee is that addressee's earlier "I". Rules follow Lee 2013, §3.3.1 and Baldwin 1997 (`papers/b/ba/baldwin-1997-cogniac-high-precision-coreference-limited`) | yes |
| `exact` | exact text match of nominal mentions | yes |
| `relaxed` | text match up to and including the head. After the head — only relative clauses (acl); a quantified antecedent — no | yes |
| `appos` | `appos` in the tree | yes |
| `pred` | predicative noun: subject — cop — noun, without negation | yes |
| `relpron` | relative pronoun → the noun the relative clause is attached to. Memory only: relatives are not annotated in GUM, so they do not go into the output | yes |
| `acronym` | "AFP" — capital letters of a name | yes |
| `reflexive` | reflexive pronoun → subject of its predicate (principle A) | yes |
| `strict_a` | head is among the cluster heads + cluster words ⊆ + modifiers ⊆ + not i-within-i | yes |
| `strict_b` | the same without modifiers | yes |
| `strict_c` | the same without words | no: 31% on dev |
| `proper_head` | the same proper head, without conflicting proper names and numbers in modifiers | yes |
| `relaxed_head` | the proper head is among the cluster words | no: 24% on dev |
| `pronoun` | 3rd-person pronoun, details below | yes |
| `nearest` | baseline: nearest preceding agreeing mention | baseline only |

**The `pronoun` sieve:**
- **cluster agreement** in number, gender, animacy and person;
- **principle B** at cluster level: a personal pronoun is not bound by a co-argument of its predicate. The test `principle_b_is_what_blocks_who` is the negative control;
- **ban on quantified antecedents** (everyone, every country) — a bound variable;
- **window ≤ 3 sentences** (Lee 2013);
- **animacy preference for he/she**: first a cluster with evidence of animacy, then any agreeing one.

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

Full output — `$MOVA_DATA/runs/coref-2026-09-26/explain-dev-GUM_bio_emperor.txt`.

**`coref resolve`** — CoNLL-U with `Entity=` in MISC (CorefUD, `papers/n/ne/nedoluzhko-2022-corefud-1-0-coreference-meets`):
- header `# global.Entity = eid-etype-head-other`;
- `head` — position of the head within the span;
- `other` — id of the sieve that attached the mention: `new` — first in the chain, `sgl` — singleton;
- old `Entity`, `Bridge`, `SplitAnte` are removed.

The resolver does not see the gold standard: test `resolver_ignores_gold_entities`; in addition, every `eval` compares the output from MISC with and without Entity (fail-fast).

## Scorer

**Metrics** (`src/score.rs`):
- MUC — Vilain et al. 1995 (`papers/v/vi/vilain-1995-model-theoretic-coreference-scoring-scheme`);
- B³ — Bagga & Baldwin 1998;
- CEAF-e and CEAF-m — Luo 2005 (`papers/l/lu/luo-2005-coreference-resolution-performance-metrics`). The best entity assignment is Kuhn–Munkres, separately per connected component of the overlap;
- CoNLL F1 — mean of MUC, B³ and CEAF-e.

**Formulas** for predicted mentions without manipulations — Pradhan et al. 2014 (`papers/p/pr/pradhan-2014-scoring-coreference-partitions-predicted-mentions`).

**Mention matching** — by head, as in the main CRAC 2023–2026 metric (`papers/n/no/novak-2025-findings-fourth-shared-task-multilingual`, `papers/n/no/novak-2026-findings-fifth-shared-task-multilingual`):
- the span head as in udapi `corefud.MoveHead` (`papers/p/po/popel-2017-udapi-universal-api-universal-dependencies`): the word whose head is outside the span; among several — the highest, then non-punctuation, then the first;
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
- **`coref score gold output`** (from files) gives the same numbers as the in-memory `eval`. Verified on dev.

## Data and licenses

**GUM** (UD 2.18, `UD_English-GUM`) — CC BY-NC-SA:
- evaluation and research only;
- rules and thresholds were tuned on dev only;
- test was run once, after freezing the code (`runs/coref-2026-09-26/test/RUN-ONCE.txt`);
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

No third-party lexicons. What is missing is listed under "What next".

## Results — GUM test (UD 2.18: 30 documents, 7,955 mentions, 4,330 entities, of which 1,104 non-singleton)

Matching by head. The row "sieve (12)" is the default system. Reports:
- `$MOVA_DATA/runs/coref-2026-09-26/test/en_gum-ud-test.{gold,slm}-trees.md`;
- CoNLL-U output and errors are in the same place.

### CoNLL F1 (MUC / B³ / CEAF-e — F1)

| system | gold trees, with singletons | gold, without singletons | SLM trees, with singletons | SLM, without singletons |
|---|---|---|---|---|
| everything separate | 40.17 (0 / 63.65 / 56.85) | 0.00 | 38.81 | 0.00 |
| exact string match only | 53.07 (29.98 / 67.32 / 61.90) | 27.88 (29.98 / 23.23 / 30.42) | 49.89 | 24.42 |
| pronoun → nearest agreeing | 57.73 (40.39 / 68.32 / 64.46) | 32.67 (40.39 / 31.87 / 25.74) | 55.80 | 32.08 |
| sieve (all 14) | 72.22 | 59.10 | 67.54 | 53.83 |
| **sieve (12)** | **73.04** (69.18 / 75.26 / 74.68) | **60.31** (69.24 / 58.79 / 52.89) | **67.90** (63.36 / 70.68 / 69.65) | **54.03** (63.42 / 52.87 / 45.81) |
| sieve (12), exact span | 68.82 | 58.33 | 57.81 | 49.73 |

**Mention detection (F1, by head):**
- with singletons: 90.7 on gold trees, 87.6 on SLM trees;
- without singletons: 77.6 and 73.7.

**Dev, for comparison** (tuned on it):
- gold trees: 75.18 / 64.30;
- SLM trees: 69.70 / 57.57.

### 3rd-person pronouns by form (test, sieve (12))

Precision is the share of linked pronouns whose antecedent is in the same gold chain.

| form | linked | precision % (gold trees) | precision % (SLM trees) | baseline "nearest", % (gold) |
|---|---|---|---|---|
| it | 194 | 41.2 | 30.1 | 29.9 |
| he | 125 | 81.6 | 80.8 | 37.8 |
| they | 93 | 50.5 | 47.9 | 52.1 |
| his | 89 | 91.0 | 89.9 | 42.7 |
| them | 55 | 67.3 | 64.3 | 51.8 |
| their | 51 | 68.6 | 71.2 | 62.7 |
| she | 37 | 54.1 | 59.0 | 33.3 |
| its | 36 | 47.2 | 33.3 | 30.6 |
| her | 23 | 65.2 | 75.0 | 33.3 |
| him | 24 | 62.5 | 73.9 | 25.0 |
| himself / themselves / itself | 6 / 6 / 4 | 100 / 100 / 75 | 100 / 67 / 25 | 50 / 100 / 50 |
| **total** | 743 | **62.4** | **57.7** | **40.4** |

### Precision and contribution of each sieve (test, gold trees; contribution is cumulative, CoNLL F1 without singletons)

| sieve | links | precision % | Δ CoNLL without singletons (gold / SLM) |
|---|---|---|---|
| speaker | 771 | 90.7 | +21.88 / +21.99 |
| exact | 964 | 69.6 | +20.94 / +18.07 |
| relaxed | 24 | 54.2 | +0.26 / +0.28 |
| appos | 89 | 75.3 (SLM: 16.7) | +2.33 / +0.18 |
| pred | 157 | 58.0 | +2.33 / +2.03 |
| relpron | 150 | n/a (not annotated in GUM) | +0.14 / −0.00 |
| acronym | 2 | 100 | +0.03 / +0.01 |
| reflexive | 15 | 73.3 | +0.15 / +0.07 |
| strict_a | 391 | 54.7 | +5.18 / +4.55 |
| strict_b | 1 | 100 | +0.05 / +0.11 |
| strict_c (disabled) | 105 | 21.9 | −0.50 / +0.31 |
| proper_head | 41 | 61.0 | +0.54 / +0.30 |
| relaxed_head (disabled) | 54 | 3.7 | −0.62 / −0.43 |
| pronoun | 758 | 59.5 | +6.38 / +6.36 |

Sieve precision is the share of links where both mentions are in the gold standard (by head) and in the same chain. It is computed on the full sieve (all 14), so that the disabled ones are visible too.

### What dev decided (gold trees, CoNLL F1 with / without singletons)

**Hobbs order versus pure recency:**
- pronoun precision 64.2% versus 55.4%;
- CoNLL 73.43 / 63.34 versus 72.23 / 61.33.

**Animacy preference for he/she:** +0.8 / +1.0. **Pronoun window** of 1–4 sentences — the same (±0.1); kept 3 (Lee 2013).

**Mentions:**
- **spans:** a whitelist of nominal dependents, `case` outside the span, appos separate — exact span 59.6% → 95.3%;
- **compound:** PROPN only gives +0.6 / +0.9; all NOUN-compounds would add another +0.1 / +0.3, but `exact` precision drops 79.7% → 73.6%;
- **a singleton coordination group** is not output: +1.1 with singletons, 0 without.

**Sieves:**
- exact without indefinites — worse (−0.4 / −0.9); kept as in Lee;
- pred without it/this subjects — worse (−0.5 / −0.9); kept;
- relaxed with only relative clauses after the head — +0.14 / +0.14, precision 45% → 53%;
- strict_c and relaxed_head disabled: precision 31% and 24%, contribution ≤ 0.

### CRAC on GUM — for reference

**CRAC 2025**, table 8 in `papers/n/no/novak-2025-findings-fourth-shared-task-multilingual/text.md` — en_gum, CoNLL F1, by head, without singletons:

| system | CoNLL F1 |
|---|---|
| CorPipeEnsemble | 76.1 |
| CorPipeSingle | 75.2 |
| LLM-UWB | 73.6 |
| LLM-NUST-FewShot | 69.8 |
| Stanza | 69.0 |
| BASELINE (mBERT end-to-end) | 61.7 |

**CRAC 2026** (`novak-2026-findings-fifth-shared-task-multilingual`, table 7): the row with CorPipeEnsemble 76.1, Stanza 73.0 and baselines 62.4 is most likely en_gum. The row label is lost in the extracted text, so the attribution is not certain.

**The settings differ — compare only roughly:**
- they use CorefUD 1.3 / 1.4 (GUM v11) and a reduced mini-test; we use the full UD 2.18 test;
- they use UDPipe or their own trees; we use gold trees (ceiling) or SLM trees;
- they use trained neural networks; we use deterministic rules without training.

Our 60.3 on gold trees and 54.0 on SLM trees are below the CRAC baseline (61.7). But the system is transparent, runs the whole test in 13 s on one core and was not trained on GUM.

## Three main error types (test)

1. **"it" / "its" → wrong noun** (precision 41%, on SLM trees 30%). Among the causes:
   - "it" often refers to an event or a statement, and we do not build event mentions (VERB head);
   - a semantic class of nouns is missing.

   Examples:
   - `GUM_academic_eegimaa-2`: «its» → «This section»;
   - `GUM_bio_dvorak-11`: «it» → «1874»;
   - `GUM_conversation_lambada-21`: «it» → «this person» — without a lexicon "person" is not known to be animate.

2. **Same text or head, different entities** (`exact` 69.6%, `strict_a` 54.7%). These are generic and repeated groups, different persons with one head, part versus whole:
   - «some politicians» ~ «some politicians», «those days» ~ «those days»;
   - «[George W.] Bush» ~ «[George H. W.] Bush»;
   - «Senegal» ~ «Southern Senegal»;
   - «the first shuttle mission» ~ «the first space shuttle mission».

3. **Wrong relations in SLM trees.** The parser attaches reference numbers "[1–3]" as appos to nouns: `GUM_academic_discrimination-4`, «1 - 3» ~ «much social science research». Because of this:
   - `appos` drops from 75% to 17%;
   - `pronoun` — from 59.5% to 54.6%;
   - CoNLL without singletons loses 6.3.

   This also covers name gender without a lexicon: «she» → «that» (`GUM_conversation_lambada-57`) — a demonstrative pronoun without gender wins in Hobbs order.

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

**Dev experiment switches** — environment variables; the defaults are what dev selected:
- `COREF_MD` (mentions): `compound`, `nopcompound`, `appos-in`, `advmod`, `desc`, `adv`, `noidiom`;
- `COREF_X` (sieves): `pron-recency`, `win=N`, `itwin=N`, `no-anim-pref`, `exact-noindef`, `pred-noit`, `relaxed-any`, `quant-ante`, `indef-ante`, `coord-sgl`.

**Speed:** the whole test (30 documents, 28k words) — all systems and tables in 13 s on one core.

## What next (needed)

- **A noun gender and animacy lexicon** with a commercial license: person, customer, doctor — animate; section, table — not. Without it "it" / "she" make the most errors.
- **Semantic classes of mentions** (person, place, organization, time…) — for "it" and for `relaxed_head` (in Lee — the NER type).
- **Event mentions and discourse deixis** (VERB head; "this" / "it" → clause). In GUM `link=disc` — 113 on dev.
- **Demonyms** (Israel ~ Israeli) — a list with a commercial license. Lee's `demonym` sieve was not implemented.
- **Cleanly licensed data for a trained model**, to move from rules to candidate ranking:
  - GUM — NC;
  - OntoNotes — LDC;
  - candidates — LitBank, FantasyCoref, GAP; licenses to be checked.

  The scorer and `link_stats` are already ready for such training.
- **Salience weights of Lappin & Leass 1994** (`papers/l/la/lappin-1994-algorithm-pronominal-anaphora-resolution`) as a replacement for or addition to Hobbs order. Parameters from the paper:
  - weights: recency 100, subject 80, existential 70, object 50, indirect object and oblique 40, head 80, non-adverbial 50;
  - halving per sentence;
  - parallelism +35, cataphora −175.

  Not tried.
- **Demonstrative pronoun sieves** (`demonstratives` in `Config`, disabled) and cataphora.
- **Preposed acl in the span** ("Born in England, Norton") — in GUM the mention is only "Norton".
- **A second parsing pass with memory** (`docs/context.md`, "Multi-pass parsing"): the clusters exist already, but the tree is not re-parsed yet.
