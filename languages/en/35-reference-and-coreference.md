# 35. Reference and coreference

Speakers pick out people, things, events and ideas with referring expressions — names, descriptions, pronouns, demonstratives, nothing at all — and the form they choose tells the hearer how easy the referent should be to find. This chapter describes the English system of referring expressions, the theories of givenness and salience that explain their distribution, the kinds of anaphora (pronominal, nominal, bridging, abstract, cataphora, generic), binding, the annotation of coreference in UD-based corpora, the classical algorithms, and the two resolvers Mova has: the `coref` sieve and the pronoun resolver inside the frame reader.

Contents

1. [Referring expressions in English](#1-referring-expressions-in-english)
2. [Givenness, accessibility and salience](#2-givenness-accessibility-and-salience)
3. [Kinds of anaphora](#3-kinds-of-anaphora)
4. [Non-referential and generic uses](#4-non-referential-and-generic-uses)
5. [Binding: reflexives and pronouns in the clause](#5-binding-reflexives-and-pronouns-in-the-clause)
6. [Reference in quoted speech and dialogue](#6-reference-in-quoted-speech-and-dialogue)
7. [Annotating coreference](#7-annotating-coreference)
8. [Algorithms](#8-algorithms)
9. [Mova's `coref` crate](#9-movas-coref-crate)
10. [The resolver inside the frame reader](#10-the-resolver-inside-the-frame-reader)
11. [Generating referring expressions](#11-generating-referring-expressions)

---

## 1. Referring expressions in English

| form | example | UD shape | typical use |
|---|---|---|---|
| proper name | *Alice*, *the March Hare* | PROPN head, `flat` for multiword names (22 §2.6) | introduce or re-identify a known individual |
| definite NP | *the cheese*, *the door of the house* | `det` *the* + NOUN | a referent the hearer can identify: mentioned, visible, unique, or inferable |
| indefinite NP | *a crow*, *some cheese* | `det` *a/some* + NOUN | a new referent (or a non-specific one) |
| demonstrative NP | *this crow*, *that idea* | `det` *this/that* (`PronType=Dem`) | a referent pointed at in the situation or the text; contrast |
| demonstrative pronoun | *this*, *that* | PRON `PronType=Dem` | often an event, fact or stretch of text (§3.4) |
| personal pronoun | *she, it, they* | PRON `PronType=Prs` | the referent in focus |
| possessive | *her beak*, *the fox's tail* | `nmod:poss` | anchors a new referent to a known one |
| reflexive / reciprocal | *herself*, *each other* | PRON `Reflex=Yes`; `fixed` *each other* | a co-argument of the same predicate (§5) |
| zero | *[she] opened her beak and [ ] dropped the cheese*; *Sit down!* | no node; shared subject in EUD (20 §8) | the referent continues as topic |
| *one*-anaphora | *the red one* | NOUN *one* (22 §2.10) | a new member of a mentioned type |

English marks definiteness by articles, not by case or word order (Curme 1931, vol. III, chs. XXV–XXVI, `books/c/cu/curme-1931-grammar-english-language-vol`; seeds [../../en/seeds/nominal/articles.md](../../en/seeds/nominal/articles.md), [../../en/seeds/nominal/det-prontype.md](../../en/seeds/nominal/det-prontype.md)). Pronouns carry person, number and, in the third singular, gender; *it* and *you* do not distinguish nominative from accusative (seeds [../../en/seeds/nominal/pron-paradigm.md](../../en/seeds/nominal/pron-paradigm.md), [../../en/seeds/nominal/you-it-case.md](../../en/seeds/nominal/you-it-case.md)). Singular *they* for a person of unknown or non-binary gender is old and now widespread; a resolver must not treat *they* as plural-only.

(1) *The fox flattered the crow because **she** wanted the cheese. The crow opened **her** beak and the cheese fell.* ([../../docs/pragmatics.md](../../docs/pragmatics.md), *At a glance*) — *she* is the fox, *her* the crow, *the cheese* the one just mentioned; the reader needs all three.

## 2. Givenness, accessibility and salience

### 2.1 The givenness hierarchy

Gundel, Hedberg & Zacharski (1993, "Cognitive status and the form of referring expressions in discourse", *Language* 69(2); not in the library — see *Wanted*) rank six cognitive statuses; each English form signals that the referent has at least a given status:

| status | the hearer… | English form that signals it |
|---|---|---|
| in focus | has it at the centre of attention | *it* (unstressed pronouns) |
| activated | has it in short-term memory | *that*, *this*, *this N* |
| familiar | already knows it | *that N* |
| uniquely identifiable | can identify it from the description | *the N* |
| referential | will be told which one | indefinite *this N* (*this guy I met*) |
| type identifiable | knows the type | *a N* |

Because the statuses are nested, a speaker who uses a weaker form than possible implicates that the stronger status does not hold — a Quantity implicature (33). The earlier paper on demonstratives by the same author is in the library (Gundel 1988, "On the generation and interpretation of demonstrative expressions", `papers/g/gu/gun/gundel-1988-generation-interpretation-demonstrative-expressions`).

### 2.2 Accessibility

Ariel's accessibility theory (Ariel 1990, *Accessing Noun-Phrase Antecedents*; Wanted) makes a similar scale continuous: the more accessible a referent, the shorter and less informative the expression — zero < reflexive < unstressed pronoun < stressed pronoun < demonstrative < short definite description < full name < full name with modifier. Distance, competition, salience and unity of the discourse segment raise or lower accessibility.

### 2.3 Centering

Centering theory models local coherence between adjacent utterances (Grosz, Joshi & Weinstein 1995, "Centering: A Framework for Modeling the Local Coherence of Discourse", *Computational Linguistics* 21(2), https://aclanthology.org/J95-2003, `papers/g/gr/gro/grosz-1995-centering-framework-modeling-local-coherence`). Each utterance U has forward-looking centres Cf(U), ranked (in English by grammatical role: subject > object > other), and one backward-looking centre Cb(U), the highest-ranked element of the previous Cf realised in U.

| transition | Cb(Uₙ) = Cb(Uₙ₋₁)? | Cb(Uₙ) = Cp(Uₙ) (the top of Cf)? | example after *The fox saw the crow.* |
|---|---|---|---|
| Continue | yes | yes | ***She** flattered the crow.* |
| Retain | yes | no | *The crow ignored **her**.* |
| Shift | no | — | *The crow flew away.* |

Rule 1: if any element of Cf(Uₙ₋₁) is a pronoun in Uₙ, then Cb(Uₙ) is too. Rule 2: Continue is preferred to Retain, Retain to Shift. Brennan, Friedman & Pollard (1987) turned the rules into a pronoun resolution algorithm, and Kehler (1997, "Current Theories of Centering for Pronoun Interpretation: A Critical Evaluation", `papers/k/ke/keh/kehler-1997-current-theories-centering-pronoun-interpretation`) showed its limits for incremental interpretation. Strube's functional centering ranks hearer-old before hearer-new entities instead of by grammatical role (`papers/s/st/str/strube-1998-never-look-back-alternative-centering`).

### 2.4 Salience in practice

Algorithms turn these ideas into numbers. Lappin & Leass (1994, "An Algorithm for Pronominal Anaphora Resolution", *Computational Linguistics* 20(4), https://aclanthology.org/J94-4002, `papers/l/la/lap/lappin-1994-algorithm-pronominal-anaphora-resolution`; repo summary [../../papers/lappin-1994-algorithm-pronominal-anaphora-resolution.md](https://research.elamur.ai/free/en/papers/lappin-1994-algorithm-pronominal-anaphora-resolution/)) weigh: recency 100, subject 80, existential 70, direct object 50, indirect object and oblique 40, head noun 80, non-adverbial 50; the weights halve with each sentence; parallel roles add 35 and cataphora subtracts 175. Mova's frame reader uses a simpler version of the same idea (§10).

## 3. Kinds of anaphora

### 3.1 Pronominal and nominal

| kind | example | relation to the antecedent |
|---|---|---|
| pronominal | *The fox … she* | identity |
| repeated name / description | *Alice … Alice*; *the Cheshire Cat … the Cat* | identity; the head lemma is enough in tales |
| epithet | *Norton … the Emperor* | identity, new description |
| hypernym | *a sparrow … the bird* | identity via the type hierarchy |
| *one*-anaphora | *a red apple and a green one* | same type, new token |
| VP / clausal ellipsis | *Mary left and John did too* | the predicate is recovered (29) |

### 3.2 Bridging

A definite NP may refer to something not mentioned but inferable from what was: *I walked into the room. **The chandeliers** sparkled brightly.* Clark (1975, "Bridging", *TINLAP*, https://aclanthology.org/T75-2034, `papers/c/cl/clark-1975-bridging`) calls this indirect reference and classifies the inferences the hearer must build:

| Clark's class | sub-kinds | English example |
|---|---|---|
| direct reference | identity, pronoun, epithet, set membership | *a house … the house*; *three boys … one of them* |
| indirect reference by association | necessary, probable and inducible parts | *a room … the ceiling* (necessary), *… the windows* (probable), *… the chandelier* (inducible) |
| indirect reference by characterisation | necessary roles, optional roles | *a murder … the murderer* (necessary), *… the knife* (optional) |
| reasons, causes, consequences, concurrences | — | *John fell. **What he wanted** was to annoy Mary.* |

Bridging is the most frequent kind of non-identity anaphora in narrative: *the Mole opened **the door** of the house … **the handle** was cold* (the handle of that door). It needs world knowledge of parts and roles; Hou et al. resolve it with rules and learned relations (Hou, Markert & Strube 2018, "Unrestricted Bridging Resolution", `papers/h/ho/hou/hou-2018-unrestricted-bridging-resolution`). Typologically, "bridging constructions" in the sense of Guérin (ed.) (2019, *Bridging constructions*, Language Science Press, `books/g/gu/guerin-2019-bridging-constructions`) are something else — clause-linking recapitulations like *He went home. Having gone home, he slept.* — common in narrative across languages; English uses them in instructions and chronicles.

In the CorefUD version of GUM, bridging is marked with `Bridge=` in MISC; Mova's `coref` removes the gold `Bridge` and `SplitAnte` before output and does not resolve bridging yet ([../../coref/README.md](../../coref/README.md)). The design note lists bridging (*the house … the door*) as one of the things to infer from context ([../../docs/context.md](../../docs/context.md), *What we infer from context*).

### 3.3 Cataphora

A pronoun can precede its antecedent, mostly when it sits in a fronted subordinate clause: ***When he** woke up, **the Mole** was hungry.* A pronoun cannot c-command its antecedent (*\*He said the Mole was hungry*, with *he* = the Mole). Lappin & Leass penalise cataphora heavily (−175). Mova's frame reader resolves pronouns in word order only, so the cataphoric *he* stays unresolved:

(2) `world frames "When he woke up, the Mole was hungry. He opened the door of the house. The handle was cold."` «Mova 0.4, CPU, 2026-10-17»:

```text
#0 Event he wake up
#1 Fact(is) mole be attrs=hungry
#2 Event he open patient=door
#3 Fact(is) handle be attrs=cold
```

Neither *he* is linked to the Mole: the first is cataphoric, and the second, though anaphoric, also stays *he* — the reader's gender evidence for *mole* is missing (a common noun without a gendered antecedent; §10). *The handle* is not bridged to the door.

### 3.4 Abstract anaphora

*This*, *that* and *it* often refer to an event, a fact, a proposition or a stretch of discourse rather than an NP: *It's way too hot here. **That**'s why I'm moving to Alaska* (example from Kolhatkar et al. 2018, "Anaphora With Non-nominal Antecedents in Computational Linguistics: a Survey", *Computational Linguistics* 44(3), https://aclanthology.org/J18-3007, `papers/k/ko/kol/kolhatkar-2018-survey-anaphora-non-nominal-antecedents`). Byron (2002, "Resolving Pronominal Reference to Abstract Entities", `papers/b/by/byron-2002-resolving-pronominal-reference-abstract-entities`) resolves them with semantic constraints from the verb that takes the pronoun.

(3) EWT 2.18 `answers-20111108105629AAiZUDY_ans-0048` *That's why it's good to have a python!* — *That* (`nsubj` of the predicate *why*) points to the reason given earlier in the answer.

Shell nouns — *this issue*, *the fact*, *the problem* — do the same with a noun (Kolhatkar & Hirst, `papers/k/ko/kol/kolhatkar-2014-resolving-shell-nouns`). In GUM, discourse deixis is linked with `link=disc` (113 on dev); Mova's `coref` builds no event mentions, which it names as the main cause of its errors on *it* ([../../coref/README.md](../../coref/README.md), *Three main error types*).

### 3.5 Split antecedents and plurals

*Alice met the Hatter. **They** argued.* — *they* = Alice + the Hatter. CorefUD marks split antecedents with `SplitAnte=`. Mova's frame reader keeps coordinated nouns as one entity with an *and* list, so a plural pronoun after a coordination resolves to the group:

(4) `world frames "Alice and the Hatter argued. They were both angry. Alice told the Hatter that he was rude."` «Mova 0.4, CPU, 2026-10-17»:

```text
#0 Say(disagree) alice+hatter argue
#1 Fact(is) alice+hatter(«they») be attrs=angry,both
#2 Say(assert) alice tell to=hatter | says→#3
#3 Fact(is) hatter(«he») be attrs=rude
```

Split antecedents across sentences (*Alice … the Hatter … they*) are not built.

## 4. Non-referential and generic uses

| use | example | UD | does it refer? |
|---|---|---|---|
| expletive *there* | ***There** is a mouse in the pool.* | `expl` | no |
| extraposition *it* | ***It** is easy to please John.* | `expl` (21 §5.3) | no |
| weather / ambient *it* | ***It** was raining.* | `nsubj` in EWT (21 §5.3) | no (situation) |
| idiomatic *it* | *make **it**, call **it** a day* | `obj` | no |
| generic *you* | ***you** never know where you might get stuck* (EWT 2.18 `weblog-blogspot.com_healingiraq_20040409053012_ENG_20040409_053012-0002`) | `nsubj` | people in general |
| generic / institutional *they* | ***they** say …*; *they've closed the road* | `nsubj` | unspecified people |
| generic definite / bare plural | ***The tiger** is a predator.*; ***Tigers** hunt.* | `nsubj` | the kind |
| predicative NP | *She is **a teacher**.* | root with `cop` | a property, not a new referent |
| quantified NP | ***every** child* | `det` | no fixed referent; can bind a pronoun |

Mova handles three of these explicitly. The frame reader marks a **dummy *it*** — *it depends*, *it means* + a clause, an *it* subject of a copular predicate with a clausal or *to*-infinitive dependent — as unresolved, and it matches nothing ([../../docs/pragmatics.md](../../docs/pragmatics.md); `DUMMY_IT` in [../../prag/src/frames.rs](../../prag/src/frames.rs)). In (5) the last sentence gets the empty agent `_`:

(5) `world frames "The fox met the crow. The crow had cheese. The fox wanted it. It is easy to see why."` «Mova 0.4, CPU, 2026-10-17»:

```text
#0 Event fox meet patient=crow
#1 Fact(has) crow have patient=cheese
#2 Event fox want patient=crow(«it»)
#3 Fact(is) _ be attrs=easy | cause→#4
#4 Event _ see
```

The extraposed *It* is correctly left alone. But *it* in *The fox wanted it* resolves to *crow*, not *cheese*: the reader allows *it* for an animal, and the crow, as the subject of the last sentence, outranks the object *cheese*. A preference for objects of *have* or a selectional check (*want* + edible thing; the absurdity matrix, [../../docs/knowledge.md](../../docs/knowledge.md)) would fix it.

The **generic *you*** is no evidence for the pragmatic student ([../../docs/pragmatics.md](../../docs/pragmatics.md), *Clients*), and the `coref` sieve bans **quantified antecedents** (*everyone*, *every country*) for pronouns: those are bound variables, not referents ([../../coref/README.md](../../coref/README.md)).

## 5. Binding: reflexives and pronouns in the clause

English binding follows the classic pattern:

| principle | statement | example |
|---|---|---|
| A | a reflexive is bound by a co-argument, usually the subject of its own clause | *The king hurt **himself**.* / *\*Himself hurt the king.* |
| B | a personal pronoun is not bound by a co-argument | *The king hurt **him*** (≠ the king) |
| C | a name or description is not bound by a c-commanding pronoun | *\***He** said **the king** was tired* (same person) |

Reflexives agree with their antecedent in person, number and gender (seed [../../en/seeds/nominal/reflexive-agreement.md](../../en/seeds/nominal/reflexive-agreement.md)) and have an intensive use with no binding at all (*The king **himself** came*; seed [../../en/seeds/nominal/reflexive.md](../../en/seeds/nominal/reflexive.md)). Picture NPs and locative PPs allow both forms (*She saw a snake near her/herself*). Older English also used the plain object pronoun reflexively: Shakespeare has *I fear me* and *haste thee*, and many verbs now intransitive were then reflexive (Abbott 1869, *A Shakespearian Grammar*, §296, `books/a/ab/abbott-1869-shakespearian-grammar-attempt-illustrate`); the JPS Torah has the reflexive form in *hide thyself from them* (`jps:DEU:585`).

Mova's `coref` implements A and B as sieves: `reflexive` links a reflexive to the subject of its predicate, and the `pronoun` sieve applies principle B at cluster level — its test `principle_b_is_what_blocks_who` is the negative control ([../../coref/src/sieve.rs](../../coref/src/sieve.rs)). The frame reader encodes B as "a plain object pronoun is never its own clause's subject" ([../../docs/pragmatics.md](../../docs/pragmatics.md)).

## 6. Reference in quoted speech and dialogue

Inside a quotation the deictic centre moves: *I* is the speaker of the quote and *you* its addressee, not the narrator and the reader. Tales mix narration and direct speech constantly — 12.3% of the labelled sentences in the parse database are a character's speech (38 406 of 311 101; `$MOVA_DATA/parsedb/prag.tsv`, 2026-10-17; 30 §7).

(6) `world frames "\"I am late,\" said the Rabbit to Alice. \"You must hurry,\" she answered."` «Mova 0.4, CPU, 2026-10-17»:

```text
#0 Fact(is) rabbit(«i») be attrs=late | quoted
#1 Say(assert) rabbit say to=alice | says→#0
#2 Event you hurry | says→#3 quoted
#3 Say(assert) rabbit(«she») answer | says→#2
```

*I* in the first quote is the Rabbit, right. In the reply, *she* should be Alice — the addressee of the previous turn takes the next turn — but the reader picks the most salient earlier subject, the Rabbit; and *you* in the second quote stays unresolved because its Say frame comes after it. The `coref` crate's `speaker` sieve handles the turn logic better: *I* of one voice is one person, *you* addressed to someone links to that addressee's earlier *I*, and clusters with *I* of different voices never merge (after Lee et al. 2013 §3.3.1 and Baldwin 1997). It is its most precise sieve (§9.3). See 38 for reported speech.

## 7. Annotating coreference

| scheme / corpus | what it marks | English data | licence |
|---|---|---|---|
| MUC-6/7 | identity chains in news | news | LDC |
| OntoNotes 5.0 | identity coreference, no singletons, events by verbs; the CoNLL-2011/2012 shared task data | news, broadcast, web, Bible, telephone | LDC (Hovy et al. 2006, `papers/h/ho/hov/hovy-2006-ontonotes-90-solution`; Pradhan et al. 2012, `papers/p/pr/pra/pradhan-2012-conll-2012-shared-task-modeling`) |
| ARRAU | identity, bridging, discourse deixis, non-referring | news, dialogue, fiction | LDC (Poesio et al. 2018/2024) |
| LitBank | entities and coreference in 100 novels | fiction | CC BY 4.0 (Bamman, Lewke & Mansoor 2020, `papers/b/ba/bam/bamman-2020-annotated-dataset-coreference-english-literature`) |
| GUM (CorefUD) | identity with singletons, bridging, split antecedents, discourse deixis, information status | 15 spoken and written genres | CC BY-NC-SA 4.0 — tests only |
| CorefUD | many corpora in one format over UD trees | GUM, GENTLE, and others | per corpus |

CorefUD (Nedoluzhko et al. 2022, "CorefUD 1.0: Coreference Meets Universal Dependencies", LREC, https://aclanthology.org/2022.lrec-1.520, `papers/n/ne/ned/nedoluzhko-2022-corefud-1-0-coreference-meets`; repo summary [../../papers/nedoluzhko-2022-corefud-1-0-coreference-meets.md](https://research.elamur.ai/free/en/papers/nedoluzhko-2022-corefud-1-0-coreference-meets/)) puts coreference into the MISC column with bracket notation: a mention opens with `Entity=(e3-person-1-new` on its first word and closes with `e3)` on its last; a `# global.Entity = eid-etype-head-other` header names the fields. The trees stay valid UD. The Universal Anaphora initiative extends the format to bridging and discourse deixis (Poesio et al. 2024, "Universal Anaphora: The First Three Years", `papers/p/po/poe/poesio-2024-universal-anaphora-first-three-years`).

Metrics: MUC (Vilain et al. 1995), B³ (Bagga & Baldwin 1998), CEAF (Luo 2005), and their mean, CoNLL F1; for predicted mentions the formulas of Pradhan et al. 2014 (`papers/p/pr/pra/pradhan-2014-scoring-coreference-partitions-predicted-mentions`). The CRAC shared tasks on CorefUD match mentions by their head (Novák et al. 2025, `papers/n/no/nov/novak-2025-findings-fourth-shared-task-multilingual`).

## 8. Algorithms

| approach | idea | source |
|---|---|---|
| Hobbs's algorithm | walk the parse tree from the pronoun, breadth-first, left to right; syntactic constraints only | Hobbs 1978, *Lingua* 44 (repo summary [../../papers/hobbs-1978-resolving-pronoun-references.md](https://research.elamur.ai/free/en/papers/hobbs-1978-resolving-pronoun-references/)) |
| centering-based (BFP) | choose the antecedent that gives the preferred transition | Brennan, Friedman & Pollard 1987 |
| salience weighting (RAP) | weighted factors, halved per sentence (§2.4) | Lappin & Leass 1994 |
| high-precision rules (CogNIAC) | resolve only when a confident rule fires, otherwise abstain | Baldwin 1995/1997, `papers/b/ba/bal/bald/baldwin-1997-cogniac-high-precision-coreference-limited`; repo summaries [../../papers/baldwin-1995-cogniac-discourse-processing-engine.md](https://research.elamur.ai/free/en/papers/baldwin-1995-cogniac-discourse-processing-engine/), [../../papers/baldwin-1997-cogniac-high-precision-coreference-limited.md](https://research.elamur.ai/free/en/papers/baldwin-1997-cogniac-high-precision-coreference-limited/) |
| syntactic and semantic compatibility | heads higher in the tree first; strong syntactic constraints beat learning | Haghighi & Klein 2009, `papers/h/ha/hag/haghighi-2009-simple-coreference-resolution-rich-syntactic` |
| multi-pass sieve | deterministic sieves from most to least precise; entity-centric features | Raghunathan et al. 2010, `papers/r/ra/rag/raghunathan-2010-multi-pass-sieve-coreference-resolution`; Lee et al. 2011, `papers/l/le/lee/lee2011/lee-2011-stanford-s-multi-pass-sieve`; Lee et al. 2013, https://aclanthology.org/J13-4004, `papers/l/le/lee/lee2013/lee-2013-deterministic-coreference-resolution-based-entity` |
| entity-level learning | mention ranking with cluster features | Clark & Manning 2016, `papers/c/cl/clark-2016-improving-coreference-resolution-learning-entity` |
| end-to-end neural | score all spans as mentions and antecedents jointly | Lee et al. 2017, `papers/l/le/lee/lee2017/lee-2017-end-end-neural-coreference-resolution` |

The Stanford sieve won the CoNLL-2011 shared task; end-to-end neural models have dominated since 2017, and on GUM the best CRAC 2025 system reaches CoNLL F1 76.1 (head match, without singletons) against a 61.7 baseline ([../../coref/README.md](../../coref/README.md), *CRAC on GUM*). The hardest pronoun cases need world knowledge — the Winograd schemas (*The trophy doesn't fit in the suitcase because **it** is too big*; Levesque et al. 2012, `papers/l/le/lev/levesque-2012-winograd-schema-challenge`).

## 9. Mova's `coref` crate

### 9.1 Design

A deterministic multi-pass sieve over UD trees, "classical and deterministic, without a large model and at no cost", in which every link carries the sieve id and its evidence ([../../coref/README.md](../../coref/README.md); [../../coref/src/sieve.rs](../../coref/src/sieve.rs)). Pipeline:

```text
UD tree (gold or en annotate) → mentions (mention.rs) → sieves (sieve.rs) → CorefUD Entity= (corefud.rs)
                                                                          → evaluation (score.rs, eval.rs)
```

**Mentions** ([../../coref/src/mention.rs](../../coref/src/mention.rs)) are NOUN and PROPN heads, pronouns except `expl`, DET and NUM in nominal roles; not `flat`, `fixed`, `nmod:desc`, titles, or common-noun compounds (*cell phone*), but proper-noun compounds yes. The span is the head's subtree restricted to nominal dependents, without the preposition and without an apposition (GUM treats the apposition as a separate mention). A coordination gives the group and the first conjunct. On GUM dev with gold trees: mention recall 92.8%, precision 89.9% by head; exact span 95.5% among matches.

**Features** come only from UD FEATS and small hand-made lists (pronouns, titles, 60 nouns with gender in the word — *mother, king* —, speech verbs, idioms). Unknown values match anything. **Gender is never guessed from a name**; a name from `# speaker`, *who* vs *which* in a relative clause, and *Mr./Mrs.* are the evidence.

**Candidate order** follows Hobbs's spirit: breadth-first, left to right, by head depth then position; the mention's clause first, then the rest of the sentence, then earlier sentences (left to right for pronouns, so subjects come first).

### 9.2 The sieves

| sieve | what it links | on by default |
|---|---|---|
| `speaker` | *I/we/you* by voice: a quotation's *I* is the subject of the speech verb outside it | yes |
| `exact` | exact string match of nominal mentions | yes |
| `relaxed` | match up to the head, plus relative clauses after it | yes |
| `appos` | apposition in the tree | yes |
| `pred` | subject — copula — predicate noun, without negation | yes |
| `relpron` | relative pronoun → the noun of its clause (memory only; GUM does not annotate relatives) | yes |
| `acronym` | *AFP* ~ capital letters of a name | yes |
| `reflexive` | reflexive → subject of its predicate (principle A) | yes |
| `strict_a`, `strict_b` | head in the cluster, words and modifiers included | yes |
| `strict_c`, `relaxed_head` | looser head matches | no (precision 31% and 24% on dev) |
| `proper_head` | same proper head, no conflicting names or numbers | yes |
| `pronoun` | third-person pronoun: cluster agreement in number, gender, animacy and person; principle B; no quantified antecedents; window ≤ 3 sentences; for *he/she* an animate cluster first | yes |

### 9.3 Results

GUM test, UD 2.18 (30 documents, 7 955 mentions), the default 12-sieve system, run once after freezing the code; measured 2026-09-26 ([../../coref/README.md](../../coref/README.md)):

| trees | CoNLL F1 with singletons | without singletons |
|---|---:|---:|
| gold trees | 73.04 | 60.31 |
| Mova's own trees | 67.90 | 54.03 |
| baseline: pronoun → nearest agreeing (gold) | 57.73 | 32.67 |
| baseline: exact string match (gold) | 53.07 | 27.88 |

Third-person pronoun precision (share of linked pronouns whose antecedent is in the right gold chain), gold trees: *his* 91.0%, *he* 81.6%, *their* 68.6%, *them* 67.3%, *she* 54.1%, *they* 50.5%, *its* 47.2%, *it* 41.2%; all 743 pronouns 62.4% (Mova's trees 57.7%, nearest-agreeing baseline 40.4%). On dev, the Hobbs order beat pure recency: pronoun precision 64.2% vs 55.4%. The `speaker` sieve made 771 links at 90.7% precision and added +21.88 CoNLL F1 without singletons — the largest single gain.

The README names three main error types: *it/its* to the wrong noun (events and propositions have no mentions, and nouns have no semantic class); the same text for different entities (*some politicians* ~ *some politicians*, two different Bushes); and parser errors on Mova's trees (reference numbers attached as `appos`). The whole test runs in 13 s on one core.

### 9.4 Transparency

`coref explain` prints every chain with the sieve and its evidence, e.g. for a pronoun: «candidate #1 in Hobbs order, 1 sent. back; cluster Number=Sing Gender=Masc Animacy=Anim; animacy preference», and the rejected candidates with the reason («Gender Masc≠Fem») ([../../coref/README.md](../../coref/README.md), *Transparency*). `coref resolve` writes CorefUD `Entity=` with the sieve id in the `other` field.

## 10. The resolver inside the frame reader

The frame reader has its own resolver, tuned for tales and lessons ([../../docs/pragmatics.md](../../docs/pragmatics.md), *A frame*; [../../prag/src/frames.rs](../../prag/src/frames.rs)):

| rule | detail |
|---|---|
| word order | a pronoun sees only what came before it (no cataphora) |
| agreement | number; *he/she* → a person or animate being (global's level-1 categories) of the right gender (gendered nouns, an earlier *he/she*); never a thing already called *it* |
| *it* | never a person; numbers and times are never antecedents |
| salience | the best role of each sentence (subject > object > other, not stacking), **halved per sentence**, a bonus for the pronoun's own role |
| binding | a plain object pronoun is never its own clause's subject |
| dummy *it* | *it depends*, *it is unkind to …*, *it means* + clause: unresolved |
| quotes | *I* = the speaker of the Say frame carrying the quote (or the last speaker), *you* = its addressee |
| relative pronouns | the noun the relative clause belongs to (25) |

The switch `PRAG_COREF` chooses the resolver: 3 (default) this resolver; 1, 2, 4 the `coref` sieve alone, over the discourse, or first; 0 the old salience. On the frame bench the resolver beat the sieve: coref F1 .380 against .345 ([../../docs/pragmatics.md](../../docs/pragmatics.md), measured 2026-10-10). Coreference remains one of the weakest parts of the frame reader (dev F1 .42), mostly for lack of world knowledge of who can do what and of names' gender (30 §9).

The QUD module resolves pronouns more crudely, to the last subject entity (recency only, [../../world/src/qud.rs](../../world/src/qud.rs)) — which is why, in 36 (7), *she* after *the hill was steep* becomes the hill.

## 11. Generating referring expressions

Choosing a referring expression is the mirror task: given a referent and a context, choose name, description, pronoun or zero. Same (2024, *Referring expression generation in context: Combining linguistic and computational approaches*, Language Science Press, `books/s/sa/same-2024-referring-expression-generation-context`) compares linguistically informed feature models with neural ones and finds that a small set of discourse features — grammatical role, recency, whether the referent was the previous subject, competing referents — carries most of the choice between pronoun and description. Laury (ed.) (2005, *Minimal reference: The use of pronouns in Finnish and Estonian discourse*, `books/l/la/laury-2005-minimal-reference-use-pronouns`, card only) studies the same choice for zero and minimal forms in two languages with richer verb agreement — useful as a contrast: English rarely allows zero subjects outside coordination, imperatives and diary style (21 §5.4). Demonstratives in discourse — tracking, contrast, discourse deixis — are compared across languages in Næss, Margetts & Treis (eds.) (2020, *Demonstratives in discourse*, Language Science Press, `books/m/ma/margetts-2020-demonstratives-discourse`). In the Rational Speech Acts model, the choice of the most informative description is a reference game, which Mova's `prag rsa reference` solves exactly over small sets (33; [../../prag/src/rsa.rs](../../prag/src/rsa.rs)).

## In Mova

- The `coref` crate: [../../coref/README.md](../../coref/README.md), [../../coref/src/mention.rs](../../coref/src/mention.rs), [../../coref/src/sieve.rs](../../coref/src/sieve.rs), [../../coref/src/corefud.rs](../../coref/src/corefud.rs), [../../coref/src/score.rs](../../coref/src/score.rs), [../../coref/src/eval.rs](../../coref/src/eval.rs).
- The frame resolver: [../../prag/src/frames.rs](../../prag/src/frames.rs) (`Ctx`, `ent_at`, `DUMMY_IT`, `PRAG_COREF`); QUD pronouns: [../../world/src/qud.rs](../../world/src/qud.rs); RSA reference games: [../../prag/src/rsa.rs](../../prag/src/rsa.rs).
- Seeds: [../../en/seeds/nominal/reflexive.md](../../en/seeds/nominal/reflexive.md), [reflexive-agreement](../../en/seeds/nominal/reflexive-agreement.md), [reciprocal](../../en/seeds/nominal/reciprocal.md), [pron-paradigm](../../en/seeds/nominal/pron-paradigm.md), [there-expletive](../../en/seeds/nominal/there-expletive.md), [../../en/seeds/verbal/expletive-it.md](../../en/seeds/verbal/expletive-it.md).
- **Not yet**: no cataphora (§3.3); no bridging (§3.2); no event or proposition mentions, so abstract *this/that/it* are not resolved (§3.4); no split antecedents across sentences (§3.5); turn-taking addressee logic is in the `coref` sieve but not in the frame reader, which is the default (§6); no noun animacy and gender lexicon with a commercial licence and no semantic classes of mentions; Lappin & Leass weights not tried; no second parsing pass with the clusters as memory ([../../coref/README.md](../../coref/README.md), *What next*).

## Sources

- Gundel, J. K., Hedberg, N., Zacharski, R. 1993. Cognitive status and the form of referring expressions in discourse. *Language* 69(2). Not in the library — Wanted.
- Gundel, J. K. 1988. On the generation and interpretation of demonstrative expressions. `papers/g/gu/gun/gundel-1988-generation-interpretation-demonstrative-expressions`
- Ariel, M. 1990. *Accessing Noun-Phrase Antecedents*. Routledge. Wanted.
- Grosz, B. J., Joshi, A. K., Weinstein, S. 1995. Centering: A Framework for Modeling the Local Coherence of Discourse. *Computational Linguistics* 21(2). https://aclanthology.org/J95-2003 ; `papers/g/gr/gro/grosz-1995-centering-framework-modeling-local-coherence`
- Kehler, A. 1997. Current Theories of Centering for Pronoun Interpretation: A Critical Evaluation. `papers/k/ke/keh/kehler-1997-current-theories-centering-pronoun-interpretation`
- Strube, M. 1998. Never Look Back: An Alternative to Centering. `papers/s/st/str/strube-1998-never-look-back-alternative-centering`
- Clark, H. H. 1975. Bridging. TINLAP. https://aclanthology.org/T75-2034 ; `papers/c/cl/clark-1975-bridging`
- Hou, Y., Markert, K., Strube, M. 2018. Unrestricted Bridging Resolution. *Computational Linguistics* 44(2). `papers/h/ho/hou/hou-2018-unrestricted-bridging-resolution`
- Guérin, V. (ed.) 2019. *Bridging constructions*. Language Science Press. `books/g/gu/guerin-2019-bridging-constructions`
- Kolhatkar, V., Roussel, A., Dipper, S., Zinsmeister, H. 2018. Anaphora With Non-nominal Antecedents in Computational Linguistics: a Survey. https://aclanthology.org/J18-3007 ; `papers/k/ko/kol/kolhatkar-2018-survey-anaphora-non-nominal-antecedents`
- Byron, D. 2002. Resolving Pronominal Reference to Abstract Entities. `papers/b/by/byron-2002-resolving-pronominal-reference-abstract-entities`
- Hobbs, J. R. 1978. Resolving Pronoun References. *Lingua* 44. Repo summary [../../papers/hobbs-1978-resolving-pronoun-references.md](https://research.elamur.ai/free/en/papers/hobbs-1978-resolving-pronoun-references/)
- Lappin, S., Leass, H. J. 1994. An Algorithm for Pronominal Anaphora Resolution. https://aclanthology.org/J94-4002 ; `papers/l/la/lap/lappin-1994-algorithm-pronominal-anaphora-resolution`
- Baldwin, B. 1995, 1997. CogNIAC. `papers/b/ba/bal/bald/baldwin-1997-cogniac-high-precision-coreference-limited`; repo summaries in [../../papers/](../../papers/)
- Haghighi, A., Klein, D. 2009. Simple Coreference Resolution with Rich Syntactic and Semantic Features. `papers/h/ha/hag/haghighi-2009-simple-coreference-resolution-rich-syntactic`
- Raghunathan, K. et al. 2010. A Multi-Pass Sieve for Coreference Resolution. `papers/r/ra/rag/raghunathan-2010-multi-pass-sieve-coreference-resolution`
- Lee, H. et al. 2011. Stanford's Multi-Pass Sieve Coreference Resolution System at the CoNLL-2011 Shared Task. `papers/l/le/lee/lee2011/lee-2011-stanford-s-multi-pass-sieve`
- Lee, H. et al. 2013. Deterministic Coreference Resolution Based on Entity-Centric, Precision-Ranked Rules. https://aclanthology.org/J13-4004 ; `papers/l/le/lee/lee2013/lee-2013-deterministic-coreference-resolution-based-entity`
- Clark, K., Manning, C. D. 2016. Improving Coreference Resolution by Learning Entity-Level Distributed Representations. `papers/c/cl/clark-2016-improving-coreference-resolution-learning-entity`
- Lee, K., He, L., Lewis, M., Zettlemoyer, L. 2017. End-to-end Neural Coreference Resolution. `papers/l/le/lee/lee2017/lee-2017-end-end-neural-coreference-resolution`
- Levesque, H., Davis, E., Morgenstern, L. 2012. The Winograd Schema Challenge. `papers/l/le/lev/levesque-2012-winograd-schema-challenge`
- Hovy, E. et al. 2006. OntoNotes: The 90% Solution. `papers/h/ho/hov/hovy-2006-ontonotes-90-solution`
- Pradhan, S. et al. 2012. CoNLL-2012 Shared Task: Modeling Multilingual Unrestricted Coreference in OntoNotes. `papers/p/pr/pra/pradhan-2012-conll-2012-shared-task-modeling`
- Pradhan, S. et al. 2014. Scoring Coreference Partitions of Predicted Mentions. `papers/p/pr/pra/pradhan-2014-scoring-coreference-partitions-predicted-mentions`
- Bamman, D., Lewke, O., Mansoor, A. 2020. An Annotated Dataset of Coreference in English Literature. `papers/b/ba/bam/bamman-2020-annotated-dataset-coreference-english-literature`
- Nedoluzhko, A. et al. 2022. CorefUD 1.0: Coreference Meets Universal Dependencies. https://aclanthology.org/2022.lrec-1.520 ; `papers/n/ne/ned/nedoluzhko-2022-corefud-1-0-coreference-meets`
- Poesio, M. et al. 2024. Universal Anaphora: The First Three Years. `papers/p/po/poe/poesio-2024-universal-anaphora-first-three-years`
- Novák, M. et al. 2025. Findings of the Fourth Shared Task on Multilingual Coreference Resolution. `papers/n/no/nov/novak-2025-findings-fourth-shared-task-multilingual`
- Same, F. 2024. *Referring expression generation in context*. Language Science Press. `books/s/sa/same-2024-referring-expression-generation-context`
- Laury, R. (ed.) 2005. *Minimal reference: The use of pronouns in Finnish and Estonian discourse*. Finnish Literature Society. `books/l/la/laury-2005-minimal-reference-use-pronouns` (card)
- Næss, Å., Margetts, A., Treis, Y. (eds.) 2020. *Demonstratives in discourse*. Language Science Press. `books/m/ma/margetts-2020-demonstratives-discourse`
- Curme, G. O. 1931. *A Grammar of the English Language*, vol. III: *Syntax*. `books/c/cu/curme-1931-grammar-english-language-vol`
- Abbott, E. A. 1869. *A Shakespearian Grammar*. `books/a/ab/abbott-1869-shakespearian-grammar-attempt-illustrate`
- UD English EWT 2.18 (CC BY-SA 4.0), sentences by `sent_id`; GUM 2.18 (CC BY-NC-SA 4.0) only as the evaluation set of `coref`.
- Repo: [../../coref/README.md](../../coref/README.md), [../../docs/pragmatics.md](../../docs/pragmatics.md), [../../docs/context.md](../../docs/context.md).
