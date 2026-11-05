# 36. Discourse relations and coherence

A text is coherent when its parts are related: one sentence gives the cause of another, contrasts with it, elaborates it, follows it in time, or answers a question it raised. This chapter describes how English signals such relations — connectives, clause order, tense and aspect, and the questions a text answers — and the main frameworks that annotate them: Rhetorical Structure Theory, the Penn Discourse Treebank, SDRT and the question under discussion. It ends with what Mova's frame reader and QUD stack do with discourse structure, and what they do not do yet.

Contents

1. [Cohesion and coherence](#1-cohesion-and-coherence)
2. [Coherence relations: the families](#2-coherence-relations-the-families)
3. [Rhetorical Structure Theory](#3-rhetorical-structure-theory)
4. [The Penn Discourse Treebank](#4-the-penn-discourse-treebank)
5. [SDRT and other frameworks](#5-sdrt-and-other-frameworks)
6. [English connectives and their syntax](#6-english-connectives-and-their-syntax)
7. [Implicit relations](#7-implicit-relations)
8. [Time and narrative](#8-time-and-narrative)
9. [Questions under discussion](#9-questions-under-discussion)
10. [Discourse segmentation and parsing](#10-discourse-segmentation-and-parsing)
11. [How Mova reads discourse relations](#11-how-mova-reads-discourse-relations)

---

## 1. Cohesion and coherence

**Cohesion** is the set of surface ties between sentences: reference (*she*, *the cheese*, 35), substitution and ellipsis (*one*, *did too*, 29), conjunction (*and, but, so, then*), and lexical repetition and collocation. The classic inventory is Halliday & Hasan (1976, *Cohesion in English*; not open — Wanted). **Coherence** is the relation the reader infers between the meanings: a text can be cohesive and incoherent (*I bought a Ford. A car in which the President rode was black.*), or coherent with almost no cohesive ties (*Don't go out. It's raining.*).

Grisot (2018, *Cohesion, Coherence and Temporal Reference from an Experimental Corpus Pragmatics Perspective*, Springer, `books/g/gr/grisot-2018-cohesion-coherence-temporal-reference`) studies how tense contributes to both: verbal tenses are procedural cues that guide the hearer to temporal relations, and their interpretation interacts with connectives and with the hearer's world knowledge (§8).

## 2. Coherence relations: the families

Different frameworks cut the relations differently, but four or five families recur — Hobbs's coherence relations (Hobbs 1979, "Coherence and Coreference", *Cognitive Science* 3; Wanted), Kehler's three classes (Kehler 2002, *Coherence, Reference, and the Theory of Grammar*; Wanted), RST, PDTB and SDRT:

| family | relations | English signals | example |
|---|---|---|---|
| causal (Kehler: cause–effect) | cause, result, reason, purpose, condition, explanation | *because, so, therefore, if, in order to* | *The cheese fell **because** the crow sang.* |
| temporal (contiguity) | sequence, simultaneity, precedence, succession | *then, after, before, while, when* | *The fox ate the cheese, **then** he ran away.* |
| comparison / contrast (resemblance) | contrast, concession, similarity | *but, although, however, while, similarly* | ***Although** the crow was angry, she said nothing.* |
| expansion / elaboration (resemblance) | elaboration, instantiation, restatement, list, alternative, exception | *for example, in other words, and, or, except* | *She was proud. **In fact**, she was vain.* |
| attribution and evaluation | attribution, evaluation, background | *say, think*; evaluative words | *"I am late," said the Rabbit.* |

Kehler groups the relations by the kind of inference that establishes them — resemblance (parallel, contrast, exemplification, elaboration), cause–effect (result, explanation, violated expectation, denial of preventer) and contiguity (occasion) — and shows that the class of relation conditions pronoun interpretation and ellipsis; one of his early papers on the effect is in the library (Kehler 1993, "The Effect of Establishing Coherence in Ellipsis and Anaphora Resolution", `papers/k/ke/keh/kehler-1993-effect-establishing-coherence-ellipsis-anaphora`).

## 3. Rhetorical Structure Theory

RST (Mann & Thompson 1988, "Rhetorical Structure Theory: Toward a functional theory of text organization", *Text* 8(3); Wanted) analyses a text as a tree. The leaves are elementary discourse units (EDUs), roughly clauses; adjacent spans are joined by a relation, either **nucleus–satellite** (the satellite supports the more central nucleus: Evidence, Elaboration, Background, Concession, Condition…) or **multinuclear** (equal parts: Sequence, Joint, List, Contrast).

(1) A three-EDU analysis:

```text
[The crow sang]₁ [so the cheese fell.]₂ [Then the fox ran away with it.]₃

          Sequence (multinuclear)
         /                     \
   Result (N ← S)              [3]
   /           \
 [1] cause   [2] result   (N = nucleus: the result is what the writer wants to convey)
```

**RST-DT** (Carlson, Marcu & Okurowski 2001, "Building a Discourse-Tagged Corpus in the Framework of Rhetorical Structure Theory", SIGdial, https://aclanthology.org/W01-1605, `papers/c/ca/car/carl/carlson-2001-building-discourse-tagged-corpus-framework`) annotated 385 Wall Street Journal documents with 78 relations grouped into 16 classes: Attribution, Background, Cause, Comparison, Condition, Contrast, Elaboration, Enablement, Evaluation, Explanation, Joint, Manner-Means, Topic-Comment, Summary, Temporal, Topic-Change; plus the structural *span*, *same-unit* and *textual-organization*. RST-DT is distributed by the LDC.

**eRST** (Zeldes et al. 2025, "eRST: A Signaled Graph Theory of Discourse Relations and Organization", *Computational Linguistics*, https://aclanthology.org/2025.cl-1.3, `papers/z/ze/zel/zeldes-2025-erst-signaled-graph-theory-discourse`) extends RST to a graph: tree-breaking, non-projective and concurrent relations are allowed, and every relation is linked to the signals that mark it — connectives, but also lexical, syntactic, graphical and referential signals. Its corpus is GUM (12 genres at the time of the paper, over 200 000 tokens), where the relations appear in the CoNLL-U MISC column as `Discourse=`. GUM is CC BY-NC-SA: Mova uses it for measurement only.

## 4. The Penn Discourse Treebank

### 4.1 The approach

The PDTB (Prasad et al. 2008, "The Penn Discourse TreeBank 2.0", LREC, https://aclanthology.org/L08-1093, `papers/p/pr/pra/prasad-2008-penn-discourse-treebank-2-0`; Prasad, Webber & Joshi 2014, "Reflections on the Penn Discourse TreeBank, Comparable Corpora, and Complementary Annotation", *Computational Linguistics* 40(4), https://aclanthology.org/J14-4007, `papers/p/pr/pra/prasad-2014-reflections-penn-discourse-treebank-comparable`) is **lexically grounded and theory-neutral about higher structure**: it annotates each connective with its two arguments, Arg1 and Arg2 (Arg2 is the one the connective is syntactically bound to), and a sense, without building a tree over the whole text. Between adjacent sentences without a connective, annotators insert the connective that best expresses the inferred relation (an *implicit* connective).

PDTB-2.0 over the 1-million-word WSJ corpus (Prasad et al. 2008, table 2):

| relation type | what | tokens |
|---|---|---:|
| Explicit | a connective in the text | 18 459 |
| Implicit | no connective; annotators insert one | 16 224 |
| AltLex | an alternative lexicalisation (*That's why…*, *The reason is…*) | 624 |
| EntRel | only an entity relation between the sentences | 5 210 |
| NoRel | no relation | 254 |
| total | | 40 600 |

Implicit relations are almost as frequent as explicit ones: a reader of English infers half of the relations without a word that marks them.

### 4.2 The sense hierarchy

PDTB 3.0 (Webber, Prasad, Lee & Joshi 2019, *The Penn Discourse Treebank 3.0 Annotation Manual*, https://catalog.ldc.upenn.edu/docs/LDC2019T05/PDTB3-Annotation-Manual-Final.pdf) revised the hierarchy; levels 1 and 2, with typical English connectives:

| level 1 | level 2 | English connectives (explicit) |
|---|---|---|
| Temporal | Synchronous | *while, when, as, meanwhile* |
| | Asynchronous (Precedence / Succession) | *then, before, after, until, since, once* |
| Contingency | Cause (Reason / Result) | *because, since, as, so, therefore, thus, as a result* |
| | Cause+Belief, Cause+SpeechAct | *because* (*She's home, because the light is on*) |
| | Condition (+SpeechAct) | *if, unless, as long as, provided that* |
| | Negative-condition | *unless* |
| | Purpose | *so that, in order to, to* |
| Comparison | Concession (+SpeechAct) | *although, though, even though, yet, nevertheless, still* |
| | Contrast | *but, however, while, whereas, by contrast* |
| | Similarity | *similarly, likewise, also* |
| Expansion | Conjunction | *and, also, in addition, moreover* |
| | Disjunction | *or, either … or* |
| | Equivalence | *in other words, that is* |
| | Exception | *except, other than* |
| | Instantiation | *for example, for instance* |
| | Level-of-detail (Arg1-as-detail / Arg2-as-detail) | *specifically, in particular, in short, overall* |
| | Manner | *by, thereby, as if* |
| | Substitution | *instead, rather than* |

Many connectives are ambiguous: *since* is temporal or causal, *while* temporal or contrastive, *as* temporal, causal or manner. Webber et al. (2019, "Ambiguity in Explicit Discourse Connectives", IWCS, `papers/w/we/web/webber-2019-ambiguity-explicit-discourse-connectives`) discuss four further kinds of ambiguity beyond the usage and sense ambiguity already known — connectives are ambiguous like any other expression. A lexicon of 149 English connectives with their syntactic categories, PDTB senses and non-connective uses is DiMLex-Eng (Das et al. 2018, "Constructing a Lexicon of English Discourse Connectives", SIGdial, https://aclanthology.org/W18-5042, `papers/d/da/das/das-2018-constructing-lexicon-english-discourse-connectives`).

## 5. SDRT and other frameworks

**SDRT** (Segmented Discourse Representation Theory; Asher & Lascarides 2003, *Logics of Conversation*; Wanted) builds a graph of labelled discourse units connected by relations such as Narration, Elaboration, Explanation, Result, Background, Contrast, Parallel, Question-Answer-Pair, and uses defeasible reasoning with world knowledge to infer them. Its **right frontier** constraint says that new material attaches only to the last unit or to units that dominate it — which also limits which antecedents a pronoun can take (Afantenos & Asher 2010, "Testing SDRT's Right Frontier", `papers/a/af/afantenos-2010-testing-sdrt-s-right-frontier`). Lascarides & Asher (1993, "Temporal Interpretation, Discourse Relations and Commonsense Entailment"; the library has their related papers `papers/l/la/las/lascarides-1992-inferring-discourse-relations-context`, `papers/l/la/las/lascarides-1993-temporal-connectives-discourse-context`) explain why *Max fell. John pushed him.* is read with the pushing first: the relation Explanation, inferred from causal knowledge, overrides narrative order. Their game-theoretic account of non-cooperative conversation is in Asher & Lascarides (2013, "Strategic conversation", *Semantics and Pragmatics* 6, https://semprag.org/index.php/sp/article/view/sp.6.2, `papers/a/as/asher-2013-strategic-conversation`; see 33), and a multi-party chat corpus with SDRT structure in Asher et al. (2016, `papers/a/as/asher-2016-discourse-structure-dialogue-acts-multiparty`).

| framework | structure | units | relation inventory | English corpora |
|---|---|---|---|---|
| RST | tree (eRST: graph) | EDUs, roughly clauses | ~20–80, nucleus/satellite | RST-DT (LDC), GUM (NC) |
| PDTB | none above pairs | connective + 2 arguments | 4 classes, ~22 level-2 senses | PDTB 2/3 (LDC) |
| SDRT | graph with right frontier | labelled units, complex segments | ~15–30 | STAC (chat) |
| QUD | tree of questions | assertions and their foci | question–answer, sub-question | QUD-annotated samples (§9) |
| Discourse dependencies | dependency tree over EDUs | EDUs | RST-derived | converted RST corpora (DISRPT) |

Riester (2021, "Combined discourse representations: Coherence relations and questions under discussion", `papers/r/ri/rie/riester-2021-combined-discourse-representations-coherence-relations`) shows how a QUD tree and coherence relations describe the same text from two sides.

## 6. English connectives and their syntax

Discourse connectives are a functional class, not a part of speech: in UD they are subordinators, coordinators, adverbs, prepositional phrases or fixed expressions, and their relation follows their syntax, not their discourse function (UD guidelines; the Mova dialect keeps adverbs out of `discourse`, [../../dialects/mova/seeds.md#discourse-adverbs](../../dialects/mova/seeds.md#discourse-adverbs)).

| syntactic type | UPOS | UD relation | examples | discourse arguments |
|---|---|---|---|---|
| subordinator | SCONJ | `mark` on the subordinate predicate, which is `advcl` | *because, although, if, unless, while, since, until, before, after* | Arg2 = the subordinate clause |
| coordinator | CCONJ | `cc` on the second conjunct | *and, but, or, so, yet, nor* | Arg2 = the second conjunct |
| adverbial | ADV | `advmod` | *however, therefore, then, also, instead, meanwhile, thus* | Arg2 = the clause it modifies; Arg1 anywhere before |
| prepositional phrase | ADP + NOUN | `obl` (+ `case`) | *in other words, as a result, for example, in addition, by contrast* | as adverbials |
| fixed multiword | — | `fixed` inside | *as well as, even though, so that, as long as, in order to, rather than* | as the head word |
| wh-adverb | ADV | `advmod` of the clause (a temporal *when* clause is still `advcl`) | *when, where, whenever* | Arg2 = its clause |

Counts in EWT 2.18 (all splits, 2026-10-17) show how a connective's syntax varies:

| word | as `mark` | as `advmod` | as `cc` |
|---|---:|---:|---:|
| *if* | 899 | — | — |
| *but* | 4 | — | 902 |
| *so* | 84 | 531 | — |
| *when* | — | 439 | — |
| *also* | — | 392 | — |
| *then* | 2 | 247 | — |
| *because* | 202 | — | — |
| *since* | 109 | 5 | — |
| *however* | — | 107 | — |
| *while* | 104 | — | — |
| *before* | 108 | 41 | — |
| *after* | 102 | 5 | — |
| *though* | 45 | 42 | — |
| *yet* | — | 52 | 20 |
| *although* | 50 | 1 | — |

*So* is mostly an adverb (`advmod`, *…, so the cheese fell*) and sometimes a subordinator (*so that*); *though* is a subordinator before a clause and an adverb at the end (*It was late, though.*). In EWT the wh-adverb *when* is `advmod` of its own clause, and the clause is `advcl` of the main predicate (seed [../../en/seeds/verbal/wh-adverb-clause.md](../../en/seeds/verbal/wh-adverb-clause.md)).

(2) EWT 2.18 `weblog-blogspot.com_alaindewitt_20060827093500_ENG_20060827_093500-0008` *In other words, those Americans were there by their own choice…* — `obl(choice … , words)`: the connective PP is an oblique of the main predicate; its discourse function (Expansion: Equivalence) is not visible in the tree.

(3) EWT 2.18 `email-enronsent15_01-0030` *Anyway, I got you message…* — EWT tags *Anyway* INTJ and `discourse`; under the Mova dialect's rule, an adverbial connective like *anyway* is `advmod` (UD 2.19 change).

The syntax of connectives and coordination is in 24; the old usage book on them is Fernald (1904, *Connectives of English Speech*, `books/f/fe/fernald-1904-connectives-english-speech-correct`).

## 7. Implicit relations

In PDTB-2.0, 16 224 of 40 600 relations are implicit (§4.1), and EntRel adds another 5 210. In narrative, most relations are implicit: sentence order alone gives sequence (*The crow opened her beak. The cheese fell.* → Narration, and Result by world knowledge).

| cue | relation it suggests | example |
|---|---|---|
| sentence order of two events | Narration / Precedence | *She opened the door. She walked in.* |
| a state after an event | Background / Result | *He fell. The floor was wet.* (Explanation by world knowledge) |
| an event followed by its plausible cause | Explanation | *Max fell. John pushed him.* (Lascarides & Asher) |
| a general statement then a specific one | Instantiation / Elaboration | *Foxes are clever. This one fooled a crow.* |
| a question then an assertion | Question–Answer | *Why did she sit down? The hill was steep.* |
| repetition with a contrastive pair | Contrast | *Alice laughed. The Queen didn't.* |
| a colon | Elaboration / Explanation | *There was one problem: the door was locked.* |

Implicit relation classification is the hardest discourse task: in the DISRPT benchmark, relation classification across frameworks remains far from solved, while segmentation is near its plateau (Braud et al. 2024, "DISRPT: A Multilingual, Multi-domain, Cross-framework Benchmark for Discourse Processing", LREC-COLING, https://aclanthology.org/2024.lrec-main.447, `papers/b/br/bra/braud-2024-disrpt-multilingual-multi-domain-cross`).

## 8. Time and narrative

### 8.1 Temporal relations

English expresses the order of events by connectives (*before, after, then, while, until*), by tense and aspect (the past perfect for an earlier event, the progressive for an ongoing background), by temporal adverbials (*yesterday*, *later*), and by default by the order of telling. Tense is a discourse device: the past perfect in *She opened the door. He had left.* moves the reader back; the progressive gives a frame (*It was raining. She went out.*). Egetenmeyer, Dessì Schmid & Becker (eds.) (2024, *Tense, aspect and discourse structure*, De Gruyter, `books/b/be/becker-2024-tense-aspect-discourse-structure`, card only) collects studies of these functions, mostly for Romance languages; Grisot (2018) measures how readers use English and French tenses and connectives to recover temporal order.

The Manner maxim ("be orderly") makes two coordinated events with *and* read in the told order (*She opened the door and walked in*), unless a word reorders them or a verb is stative (Tatu & Moldovan 2012; implemented in [../../prag/src/infer.rs](../../prag/src/infer.rs), §11.3; 33).

### 8.2 Narrative structure

Narratives have levels above relations between sentences: episodes, a narrator and a narrated world, story time against discourse time (flashbacks, prolepses), framed stories within stories, and scripts of typical event sequences (Schank & Abelson 1977; Chambers & Jurafsky 2009 narrative schemas — both in the research library's topic guide `docs/papers/narrative.md`). Demske & Bloom (eds.) (2025, *Discourse structure and narration: A diachronic view from Germanic*, Language Science Press, `books/b/bl/bloom-2025-discourse-structure-narration-diachronic`) trace how Germanic languages, English among them, mark foreground and background, episode boundaries and perspective, and how these markers changed over time — the *and*-chains of older narrative are one of them.

The biblical narrative style shows the *and*-chain at its extreme: in the JPS 1917 Torah trees, 3 264 of 6 065 sentences begin with *And* (53.8%, counted 2026-10-17 in `$MOVA_DATA/corpora/judaism/parsedb/current.conllu`), and the episode opener *And it came to pass* occurs 99 times (37 §8).

(4) `jps:GEN:1297` *And it came to pass at the end of two full years, that Pharaoh dreamed: and, behold, he stood by the river.* — the opener sets a time frame (Background), *behold* shifts to the character's perception, and *and* strings the events.

## 9. Questions under discussion

The QUD view (Roberts 1996/2012, "Information structure in discourse: Towards an integrated formal theory of pragmatics", *Semantics and Pragmatics* 5, https://doi.org/10.3765/sp.5.6, `papers/r/ro/rob/roberts-2012-information-structure-discourse-towards-integrated`) treats a discourse as an inquiry: participants keep a stack of open questions, every assertion should (partially) answer the current question or a sub-question of a strategy for it, and focus marks what answers the question (29). Riester, Brunetti & De Kuthy (2018, *Annotation guidelines for Questions under Discussion and information structure*, `papers/r/ri/rie/riester-2018-annotation-guidelines-questions-under`; exact wording in the research library's `docs/papers/qud-constraints.md`) give three constraints for implicit questions:

| constraint | wording (Riester et al. 2018, ms. pp. 10–13) |
|---|---|
| Q-A-Congruence | "QUDs must be answerable by the assertion(s) that they immediately dominate." |
| Maximize-Q-Anaphoricity | "Implicit QUDs should contain as much given material as possible." |
| Q-Givenness | "Implicit QUDs can only consist of given (or, at least, highly salient) material." |

At the start of a text the question is the "Big Question" *What is the way things are?* / *What happened?*. QUDeval (Wu et al. 2023, `papers/w/wu/wu2023/wu-2023-qudeval-evaluation-questions-under-discussion`) checks generated QUDs for answer compatibility, givenness and anchor relevance.

## 10. Discourse segmentation and parsing

| task | input | output | status |
|---|---|---|---|
| segmentation | text | EDU boundaries | near plateau in DISRPT |
| connective identification | text | connective spans (PDTB-style) | room for improvement |
| relation classification | two units | relation label | hardest; implicit relations worst |
| full discourse parsing | text | RST tree / SDRT graph | separate parsers per framework |

The DISRPT shared tasks (Zeldes et al. 2019, `papers/z/ze/zel/zeldes-2019-disrpt-2019-shared-task-elementary`; 2021, `papers/z/ze/zel/zeldes-2021-disrpt-2021-shared-task-elementary`; Braud et al. 2023, `papers/b/br/bra/braud-2023-disrpt-2023-shared-task-elementary`) unify RST, SDRT, PDTB and discourse-dependency corpora — by 2024, 24 corpora in 13 languages, about 4 million tokens and 250 000 relation instances (Braud et al. 2024). Open RST and PDTB parsers, with their licences (most for tests only), are collected in [../../docs/pragmatics.md](../../docs/pragmatics.md): for example isanlp_rst, DPLP and StageDP (MIT) for RST, discopy (MIT) for PDTB-style shallow parsing; DMRST and the DISRPT repositories have no licence file and are tests only.

## 11. How Mova reads discourse relations

### 11.1 Links between frames

Mova keeps discourse relations as links between frames, not as a separate tree ([../../docs/pragmatics.md](../../docs/pragmatics.md), *Links*; [../../prag/src/frames.rs](../../prag/src/frames.rs), `set_cause`, `set_next`):

| cue | link | PDTB sense it approximates |
|---|---|---|
| *because, since, as* (+ colloquial *cos, cause, cuz*) on an `advcl` | `cause` of the main frame → the subordinate frame | Contingency.Cause.Reason |
| purpose *to* + infinitive (not a control complement), also on a noun; *by V-ing* | `cause` | Contingency.Purpose; Expansion.Manner |
| *so, therefore, thus* (not *so that*) | the clause right before is the cause | Contingency.Cause.Result |
| *so … that* | `cause` of the result | Contingency.Cause.Result |
| *and, then, after, before, when, until*; consecutive sentences | `next` (only between actions and speech; never from or to a state, a general fact or a condition) | Temporal.Asynchronous / Expansion.Conjunction |
| a `ccomp` of *say/think/know/fear*…; quoted speech | `quote` of the Say frame | Attribution |
| *if* clause | `cond` flag on the frame | Contingency.Condition (no link yet) |

(5) `world frames "The crow sang, so the cheese fell. After the fox ate the cheese, he ran away. Although the crow was angry, she said nothing. If the wolf comes, the kid will hide."` «Mova 0.4, CPU, 2026-10-17»:

```text
#0 Say(express) crow sing | next→#1
#1 Event cheese fall | cause→#0
#2 Event fox eat patient=cheese | next→#3
#3 Event fox(«he») run away attrs=away | next→#5
#4 Fact(is) crow be attrs=angry
#5 Say(assert) crow(«she») say patient=nothing | next→#7
#6 Event wolf come | if
#7 Event kid hide
```

What it gets: *so* → Result (`cause` of #1 is #0); *after* orders the eating before the running (`next`); *if* marks the wolf's coming as a condition. What it misses: the concession of *although* (#4 has no link: states are excluded from `next`, and Concession has no field); the condition #6 is flagged but not linked to its consequence #7; and *sing* is read as a speech act (*sing* is in a VerbNet class of manner of speaking).

### 11.2 Where the links matter

The pragmatic student answers *why* questions by following `cause` and *what happened next* questions by following `next` ([../../docs/pragmatics.md](../../docs/pragmatics.md), *Questions*; *Clients*). `next` is the weakest role on the frame bench (dev F1 .23, 2026-10-10): the gold standard's sense of "happened right after" is narrower than *and* and sentence order. A learned-rules experiment showed the tension: rules that cut `next` links raised the bench score but cost the student questions in the courses that needed them ([../../docs/pragmatics.md](../../docs/pragmatics.md), *Rules the reader learns itself*).

### 11.3 Inference: "be orderly"

`prag infer` adds the Manner inference that two event verbs coordinated by *and* happened in the told order — not with a stative verb (*be, have, know, like, love, own, believe, want, need, seem, belong, contain*) and not when a reordering word is present (*after, before, while, meanwhile, simultaneously, afterwards, beforehand, previously, earlier*; *then* anywhere but on the second verb); the explanation names the rule ([../../prag/src/infer.rs](../../prag/src/infer.rs), `maxims`, `STATIVE`, `REORDER`).

### 11.4 Story time and narrative frames

The world's narration layer keeps the **story time axis** apart from the order of telling: each event has a start and an end, BEFORE/AFTER/DURING/SAME are edges in a partial order (the tractable point-algebra fragment of Allen's interval algebra, as in TimeML), and a cycle with a strict edge is rejected; narration frames form a stack (who tells, to whom, with what world), so a recollection or a story within a story has its own cursor and time line ([../../world/src/story.rs](../../world/src/story.rs), [../../world/src/narr.rs](../../world/src/narr.rs)). Event statistics for books — who did what to whom, counts only — come from [../../world/src/events.rs](../../world/src/events.rs).

### 11.5 The QUD stack

`world qud` keeps a stack of questions for stories and dialogues ([../../world/src/qud.rs](../../world/src/qud.rs); [../../docs/pragmatics.md](../../docs/pragmatics.md)): explicit questions are pushed with their wh-type and content lemmas; an assertion attaches to the deepest open question with a shared content lemma and an answer of the right type (*why*: *because/since/so* or an `advcl`; *who*: an animate subject or object; …); a complete answer pops the question; otherwise an implicit QUD is accommodated from given lemmas only, and a candidate needing a new lemma is reported as answer leakage (Q-Givenness).

(6) `world qud "Alice was tired. Why did she sit down? Because the hill was steep. Then she fell asleep. Who woke her? The Rabbit woke her."` «Mova 0.4, CPU, 2026-10-17» (abridged):

```text
 1  Alice was tired.            implicit "What happened?" · stack []
                                givenness: "Who is Alice?" rejected: "alice" occurs only in the answer
 2  Why did she sit down?       push Q2 · stack [Q2]   ("she" → alice)
 3  Because the hill was steep. implicit "What happened next?" · stack [Q2]
                                why: Q2: no shared lemma; subject "hill" new
 4  Then she fell asleep.       implicit "What did the hill do?"   ("she" → hill)
 5  Who woke her?               push Q5 · stack [Q2 Q5]
 6  The Rabbit woke her.        answers Q5 (complete, popped) · stack [Q2]
```

(7) The same run shows the limits. The fragment *Because the hill was steep* is the answer to Q2, but the congruence test requires a shared content lemma, and a *because* fragment shares none; Q2 stays open to the end. The QUD module's own pronoun rule (recency of subjects) then makes *she* in line 4 the hill — the frame reader's resolver, with its animacy check, would have said Alice (35 §10).

### 11.6 Not yet

- No implicit relations beyond sentence order (`next`) and no Explanation, Contrast, Concession, Elaboration or Instantiation fields.
- No discourse tree or graph (RST, SDRT); no EDU segmentation.
- Conditions are flagged, not linked to their consequences.
- The QUD congruence test does not accept a *because* fragment as an answer to *why*.
- Connective senses are not disambiguated (*since*, *while*, *as*); only the readings in §11.1 are used.
- eRST relations in GUM are not used even for measurement yet.

## In Mova

- Links: [../../prag/src/frames.rs](../../prag/src/frames.rs) (`set_cause`, `set_next`, `quote`); docs [../../docs/pragmatics.md](../../docs/pragmatics.md) (*Links*, *The yardstick*).
- Inference: [../../prag/src/infer.rs](../../prag/src/infer.rs) (`maxims`).
- Narrative and time: [../../world/src/story.rs](../../world/src/story.rs), [../../world/src/narr.rs](../../world/src/narr.rs), [../../world/src/events.rs](../../world/src/events.rs).
- QUD: [../../world/src/qud.rs](../../world/src/qud.rs); algorithms: [../../docs/pragmatics.md](../../docs/pragmatics.md).
- Tools for comparison: [../../docs/pragmatics.md](../../docs/pragmatics.md).
- Seeds: [../../en/seeds/verbal/subordinators-mark.md](../../en/seeds/verbal/subordinators-mark.md), [../../en/seeds/verbal/wh-adverb-clause.md](../../en/seeds/verbal/wh-adverb-clause.md), [../../en/seeds/verbal/purpose-clause.md](../../en/seeds/verbal/purpose-clause.md); [../../dialects/mova/seeds.md#discourse-adverbs](../../dialects/mova/seeds.md#discourse-adverbs).

## Sources

- Halliday, M. A. K., Hasan, R. 1976. *Cohesion in English*. Longman. Wanted.
- Hobbs, J. R. 1979. Coherence and Coreference. *Cognitive Science* 3. Wanted.
- Kehler, A. 2002. *Coherence, Reference, and the Theory of Grammar*. CSLI. Wanted.
- Kehler, A. 1993. The Effect of Establishing Coherence in Ellipsis and Anaphora Resolution. `papers/k/ke/keh/kehler-1993-effect-establishing-coherence-ellipsis-anaphora`
- Mann, W. C., Thompson, S. A. 1988. Rhetorical Structure Theory: Toward a functional theory of text organization. *Text* 8(3). Wanted.
- Carlson, L., Marcu, D., Okurowski, M. E. 2001. Building a Discourse-Tagged Corpus in the Framework of Rhetorical Structure Theory. https://aclanthology.org/W01-1605 ; `papers/c/ca/car/carl/carlson-2001-building-discourse-tagged-corpus-framework`
- Zeldes, A. et al. 2025. eRST: A Signaled Graph Theory of Discourse Relations and Organization. *Computational Linguistics*. https://aclanthology.org/2025.cl-1.3 ; `papers/z/ze/zel/zeldes-2025-erst-signaled-graph-theory-discourse`
- Prasad, R. et al. 2008. The Penn Discourse TreeBank 2.0. LREC. https://aclanthology.org/L08-1093 ; `papers/p/pr/pra/prasad-2008-penn-discourse-treebank-2-0`
- Prasad, R., Webber, B., Joshi, A. 2014. Reflections on the Penn Discourse TreeBank, Comparable Corpora, and Complementary Annotation. https://aclanthology.org/J14-4007 ; `papers/p/pr/pra/prasad-2014-reflections-penn-discourse-treebank-comparable`
- Webber, B., Prasad, R., Lee, A., Joshi, A. 2019. *The Penn Discourse Treebank 3.0 Annotation Manual*. https://catalog.ldc.upenn.edu/docs/LDC2019T05/PDTB3-Annotation-Manual-Final.pdf
- Webber, B. et al. 2019. Ambiguity in Explicit Discourse Connectives. IWCS. `papers/w/we/web/webber-2019-ambiguity-explicit-discourse-connectives`
- Das, D., Scheffler, T., Bourgonje, P., Stede, M. 2018. Constructing a Lexicon of English Discourse Connectives. SIGdial. https://aclanthology.org/W18-5042 ; `papers/d/da/das/das-2018-constructing-lexicon-english-discourse-connectives`
- Asher, N., Lascarides, A. 2003. *Logics of Conversation*. Cambridge University Press. Wanted.
- Asher, N., Lascarides, A. 2013. Strategic conversation. *Semantics and Pragmatics* 6. `papers/a/as/asher-2013-strategic-conversation`
- Asher, N. et al. 2016. Discourse Structure and Dialogue Acts in Multiparty Dialogue: the STAC Corpus. `papers/a/as/asher-2016-discourse-structure-dialogue-acts-multiparty`
- Afantenos, S., Asher, N. 2010. Testing SDRT's Right Frontier. `papers/a/af/afantenos-2010-testing-sdrt-s-right-frontier`
- Lascarides, A., Asher, N. 1992, 1993. `papers/l/la/las/lascarides-1992-inferring-discourse-relations-context`, `papers/l/la/las/lascarides-1993-temporal-connectives-discourse-context`
- Riester, A. 2021. Combined discourse representations: Coherence relations and questions under discussion. `papers/r/ri/rie/riester-2021-combined-discourse-representations-coherence-relations`
- Riester, A., Brunetti, L., De Kuthy, K. 2018. Annotation guidelines for Questions under Discussion and information structure. `papers/r/ri/rie/riester-2018-annotation-guidelines-questions-under`; research library `docs/papers/qud-constraints.md`
- Roberts, C. 2012. Information structure in discourse. *Semantics and Pragmatics* 5. https://doi.org/10.3765/sp.5.6 ; `papers/r/ro/rob/roberts-2012-information-structure-discourse-towards-integrated`
- Wu, Y. et al. 2023. QUDeval: The Evaluation of Questions Under Discussion Discourse Parsing. `papers/w/wu/wu2023/wu-2023-qudeval-evaluation-questions-under-discussion`
- Zeldes, A. et al. 2019, 2021; Braud, C. et al. 2023, 2024. DISRPT shared tasks and benchmark. `papers/z/ze/zel/zeldes-2019-disrpt-2019-shared-task-elementary`, `papers/z/ze/zel/zeldes-2021-disrpt-2021-shared-task-elementary`, `papers/b/br/bra/braud-2023-disrpt-2023-shared-task-elementary`, https://aclanthology.org/2024.lrec-main.447 (`papers/b/br/bra/braud-2024-disrpt-multilingual-multi-domain-cross`)
- Grisot, C. 2018. *Cohesion, Coherence and Temporal Reference from an Experimental Corpus Pragmatics Perspective*. Springer. `books/g/gr/grisot-2018-cohesion-coherence-temporal-reference`
- Egetenmeyer, J., Dessì Schmid, S., Becker, M. G. (eds.) 2024. *Tense, aspect and discourse structure*. De Gruyter. `books/b/be/becker-2024-tense-aspect-discourse-structure` (card)
- Demske, U., Bloom, B. (eds.) 2025. *Discourse structure and narration: A diachronic view from Germanic*. Language Science Press. `books/b/bl/bloom-2025-discourse-structure-narration-diachronic`
- Fernald, J. C. 1904. *Connectives of English Speech*. `books/f/fe/fernald-1904-connectives-english-speech-correct`
- Tatu, M., Moldovan, D. 2012. LREC paper on rule-based Gricean implicatures with maxim axioms (title not checked here; as summarised in [../../docs/pragmatics.md](../../docs/pragmatics.md)).
- Research library topic guide `docs/papers/narrative.md` (scripts, narrative schemas, temporal relations).
- UD English EWT 2.18 (CC BY-SA 4.0), sentences by `sent_id`; counts of 2026-10-17. The Torah JPS 1917 trees (`$MOVA_DATA/corpora/judaism/parsedb/current.conllu`).
