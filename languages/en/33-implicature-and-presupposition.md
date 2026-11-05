# 33. Implicature and presupposition

Much of what an English sentence conveys is neither said outright nor entailed: *Some of the guests left* suggests that not all did, *She stopped crying* takes for granted that she had been crying, and *She opened the door and walked in* is understood in that order. This chapter describes conversational and conventional implicature, scales, cancellation, presupposition triggers and their projection, accommodation, the probabilistic Rational Speech Acts account — and exactly what Mova's inference module computes, with its output on English examples and on a stretch of *Alice's Adventures in Wonderland*.

Contents

1. [What is said, what is implicated, what is presupposed](#1-what-is-said-what-is-implicated-what-is-presupposed)
2. [The cooperative principle and the maxims](#2-the-cooperative-principle-and-the-maxims)
3. [Kinds of implicature](#3-kinds-of-implicature)
4. [Scalar implicatures](#4-scalar-implicatures)
5. [Other Quantity, Quality and Manner inferences](#5-other-quantity-quality-and-manner-inferences)
6. [Presupposition](#6-presupposition)
7. [Projection and accommodation](#7-projection-and-accommodation)
8. [Relevance theory and neo-Gricean views](#8-relevance-theory-and-neo-gricean-views)
9. [Rational Speech Acts](#9-rational-speech-acts)
10. [Mova's inference module](#10-movas-inference-module)
11. [Running it on a tale](#11-running-it-on-a-tale)
12. [Litotes, understatement and negation](#12-litotes-understatement-and-negation)
13. [Datasets](#13-datasets)

---

## 1. What is said, what is implicated, what is presupposed

| layer | test | example: *Even John finally stopped smoking.* |
|---|---|---|
| entailment (said) | cannot be denied without contradiction | John does not smoke now |
| presupposition | survives negation and questioning (*Did John stop smoking?*) | John smoked before |
| conventional implicature | carried by a word, not part of truth conditions | *even*: John was among the least likely; *finally*: it took long |
| conversational implicature | derived from the assumption of cooperation; cancellable | (in context) others had stopped earlier |

The distinctions go back to Frege and Strawson for presupposition and to Grice for implicature (Grice 1975, "Logic and Conversation", not open — *Wanted*). Open surveys: Davis, "Implicature" (*Stanford Encyclopedia of Philosophy*, https://plato.stanford.edu/entries/implicature/) and Beaver, Geurts & Denlinger, "Presupposition" (*SEP*, https://plato.stanford.edu/entries/presupposition/). The overview of meaning layers is in 30 §2.

## 2. The cooperative principle and the maxims

Grice proposed that conversation is governed by a **Cooperative Principle** — make your contribution such as is required, at the stage at which it occurs, by the accepted purpose of the exchange — spelled out in four maxims (Grice 1975, as summarised in Davis, *SEP*):

| maxim | content | typical inference when observed | example |
|---|---|---|---|
| Quantity | be as informative as required, not more | a weaker statement implies the stronger does not hold | *Some of the guests left* → not all |
| Quality | say only what you believe true and have evidence for | the speaker believes it; a question means the speaker does not know | *Who ate the tarts?* → the speaker does not know who |
| Relation | be relevant | the utterance bears on the current question | *Hungry? — I'm starving.* → yes |
| Manner | be clear, brief and orderly | events are told in order; a marked form signals a marked meaning | *She opened the door and walked in* → first opened |

When a maxim is **flouted** — blatantly broken — the hearer infers a meaning that restores cooperation: tautologies (*Boys will be boys*), irony (Quality, 34), hyperbole, deliberately vague answers.

## 3. Kinds of implicature

| kind | definition | English example |
|---|---|---|
| generalised conversational | arises by default, without special context | *some* → not all; *or* → not both; *a woman* → not the speaker's wife |
| particularised conversational | depends on a specific context | *It's cold in here* → shut the window |
| conventional | lexically triggered, not cancellable, not at-issue | *but* (contrast), *even*, *still*, *therefore*, *yet* |
| clausal | a disjunction or conditional implicates that the speaker does not know which | *Alice or the Queen took it* → the speaker does not know which |

Conversational implicatures are **cancellable** (*Some, in fact all, of the tarts were stolen*), **non-detachable** (another wording with the same content keeps them), and **calculable** (a reasoning path from the maxims leads to them). Conventional implicatures are not cancellable: *She is poor but honest — and there's no contrast* is odd (Davis, *SEP*). The meaning of *but* is treated as a discourse relation (Comparison/Contrast) in 36.

## 4. Scalar implicatures

### 4.1 Horn scales

Many English words form scales ordered by informational strength (Horn 1972/1989, not open — *Wanted*): if the stronger term holds, the weaker one does too. Using the weaker term implicates that the speaker is not in a position to use the stronger one.

Mova's inference module ([../../prag/src/infer.rs](../../prag/src/infer.rs), `SCALES`) contains exactly these 17 scales, weakest first, matched on lemmas:

| # | scale | weaker item implicates |
|---|---|---|
| 1 | *some < many < most < all* | *some* → not many, most, all |
| 2 | *some < most < all* | *some* → not most, all |
| 3 | *or < and* | *A or B* → not both |
| 4 | *possible < likely < certain* | *possible* → not likely, certain |
| 5 | *may < must* | *may* → not must |
| 6 | *might < must* | *might* → not must |
| 7 | *possibly < probably < certainly* | *possibly* → not probably, certainly |
| 8 | *sometimes < often < always* | *sometimes* → not often, always |
| 9 | *occasionally < usually < always* | *occasionally* → not usually, always |
| 10 | *warm < hot* | *warm* → not hot |
| 11 | *cool < cold* | *cool* → not cold |
| 12 | *good < excellent* | *good* → not excellent |
| 13 | *like < love* | *like* → not love |
| 14 | *try < manage* | *try* → not manage |
| 15 | *start < finish* | *start* → not finish |
| 16 | *believe < know* | *believe* → not know |
| 17 | *few < none* | *few* → not none |

### 4.2 When the implicature does not arise

The module blocks a scalar implicature when (all from [../../prag/src/infer.rs](../../prag/src/infer.rs)):

| block | reason | example |
|---|---|---|
| a stronger item of the same scale is in the sentence | the speaker already said the stronger | *The King ate some or all of the tarts* → no "not all" (but *or* still fires, see §11) |
| the item is negated (`not`, `n't`, `never`, `no` on the word, on its head, or right before it) | negation reverses scales: *not all* does not implicate *not some* | *He did not eat some of the tarts* → nothing |
| a cancellation phrase is in the sentence | the speaker cancels it | *if not all*, *in fact all*, *or even*, *if not*, *indeed all*, *and maybe all* |
| *or* that is not a coordinator (`cc`) | not a disjunction | — |

(1) «Mova 0.4, CPU, 2026-10-17», `prag infer`:

```text
Some of the guests left.
  scalar implicature [Some]: not «all» (nor «many», «most») — «Some» is weaker than «all» on the scale
  some < many < most < all: a speaker who could say the stronger would have (Quantity)
It was warm in the hall.
  scalar implicature [warm]: not «hot» — «warm» is weaker than «hot» on the scale warm < hot …
It might rain.
  scalar implicature [might]: not «must» — «might» is weaker than «must» on the scale might < must …
He did not eat some of the tarts.
  (nothing: negated)
Some, in fact all, of the tarts were stolen.
  (nothing: cancelled by «in fact all»)
Most of the cakes were eaten, if not all.
  (nothing: cancelled by «if not all»)
```

### 4.3 Embedded implicatures

Whether scalar implicatures arise inside embedded positions (*Every student read some of the books*) is debated between "grammatical" and "pragmatic" theories; Potts, Lassiter, Levy & Frank (2016, "Embedded implicatures as pragmatic inferences under compositional lexical uncertainty", *Journal of Semantics* 33(4), `papers/p/po/pot/potts-2016-embedded-implicatures-pragmatic-inferences-under`) model them within RSA (§9). Mova computes implicatures per sentence and does not distinguish embedded from root occurrences, except under negation.

## 5. Other Quantity, Quality and Manner inferences

Tatu & Moldovan (2012, "A Tool for Extracting Conversational Implicatures", LREC, https://aclanthology.org/L12-1039, `papers/t/ta/tat/tatu-2012-tool-extracting-conversational-implicatures`) implemented Grice's maxims as 63 hand-written axioms over logical forms with an abductive reasoner, reaching P 75.8 / R 65.8 / F 70.4 on Twitter conversations ([../../docs/pragmatics.md](../../docs/pragmatics.md)). Mova takes four of their axioms as rules ([../../prag/src/infer.rs](../../prag/src/infer.rs), `maxims`):

| maxim | rule | condition | example and output |
|---|---|---|---|
| Quantity | weaker scale item → not the stronger | §4 | *Some of the guests left.* → not all |
| Quality (questions) | a question → the speaker does not know the questioned part | ends in `?`; wh-word first → what it stands for; inverted auxiliary first → whether the rest holds; otherwise a confirmation question | *Who ate the tarts?* → "the speaker does not know who ate the tarts" |
| Quality (exclamations) | an exclamation → a strong feeling | ends in `!` | *Great, my tooth hurts!!* → "the speaker feels strongly about it" |
| Manner ("be orderly") | *e₁ and e₂* with two event verbs → e₁ before e₂ | `conj` VERB with `cc` *and* on a VERB head; neither verb stative; no reordering word | *The fox ate the grapes and ran away.* → «ate» before «ran» |

The Manner rule's exceptions are listed in the code: **stative verbs** (`STATIVE`: *be, have, know, like, love, own, believe, want, need, seem, belong, contain* — states do not follow each other) and **reordering words** (`REORDER`: *after, before, while, meanwhile, simultaneously, afterwards, beforehand, previously, earlier*; *then* blocks the rule unless it sits on the second verb).

(2) «Mova 0.4, CPU, 2026-10-17»:

```text
The fox ate the grapes and ran away.
  manner implicature [and]: «ate» happened before «ran» — two events «ate» and «ran» joined by «and» are told
  in the order they happened (Manner: be orderly); no stative verb, no reordering word
She walked in after she opened the door.
  (nothing: «after» reorders)
Did the crow sing again?
  quality implicature [?]: the speaker does not know whether «the crow sing again» — a yes/no question with
  the inverted «did»: a truthful speaker asks only what he does not know (Quality)
```

The same "be orderly" reasoning gives the `next` links of frames: *and*, *then* and sentence order link event frames in told order (36; [../../docs/pragmatics.md](../../docs/pragmatics.md), *Links*). The Quality rule for questions is also what the dialogue layer uses: a member asked a question answers what it knows and says what it does not ([../../docs/pragmatics.md](../../docs/pragmatics.md), *What goes first*).

**Relation** (relevance) inferences — *Hungry? — I'm starving.* — are not computed; indirect answers are the subject of the Circa dataset (§13).

## 6. Presupposition

### 6.1 Triggers

A presupposition is information a sentence takes for granted; it survives when the sentence is negated, questioned or made conditional. English presupposition triggers (Beaver, Geurts & Denlinger, *SEP*; Levinson 1983, *Pragmatics*, ch. 4 — not open, *Wanted*):

| trigger class | English triggers | presupposition | in Mova |
|---|---|---|---|
| iterative adverbs | *again, still* | the event or state held before | yes |
| additive particles | *also, too, either* | someone or something else did the same | *also, too* |
| change of state | *stop, quit, continue, keep, start, begin* | the earlier state | *stop, quit, continue, keep* |
| factive verbs | *know, realize, regret, forget that, discover, be aware* | the complement is true | *know, realize/realise, regret, forget* |
| definite descriptions | *the King of Hearts*, *her sister* | the referent exists and is unique in context | no (35: reference) |
| possessives | *Alice's cat* | Alice has a cat | no |
| clefts | *It was the Knave who stole the tarts* | someone stole the tarts | no (29) |
| wh-questions | *Who stole the tarts?* | someone stole them | no (the Quality inference covers the speaker's ignorance, not the existential) |
| temporal clauses | *before/after/since she came* | she came | no |
| implicatives | *manage to, forget to* | trying / intending | no (*try < manage* is a scale) |
| *even, only* | *Even John came*; *Only John came* | others were expected / John came | no |

The adverb triggers *too, again, also, still, yet* were studied as a prediction task by Cianflone, Feng, Kabbara & Cheung (2018, "Let's do it 'again': A First Computational Approach to Detecting Adverbial Presupposition Triggers", ACL, https://aclanthology.org/P18-1256, `papers/c/ci/cianflone-2018-let-s-do-it-again`): the cue for an adverb is a repeated event or entity in the preceding context; a bigram logistic regression is near chance except for *too* and *yet*, and an attention model reaches 76% on their Penn Treebank corpus ([../../docs/pragmatics.md](../../docs/pragmatics.md)).

### 6.2 What Mova computes

For each trigger, [../../prag/src/infer.rs](../../prag/src/infer.rs) (`presuppositions`) builds the content from the tree:

| trigger | the predicate it reads | content |
|---|---|---|
| *again* | its head | «PRED» happened before |
| *still* | its head | «PRED» held before |
| *also*, *too* | its head (not *too* on an adjective or adverb: the excess *too heavy*) | someone or something else «PRED» too |
| *stop, quit, continue, keep* (VERB) | its `xcomp`, `ccomp` or `obj` | «X» was going on before |
| *know, realize, realise, regret, forget* (VERB) | its `ccomp`, or an `xcomp` with its own subject or *that* (a parser slip) | it is true that «the clause» |

### 6.3 Licensing

A presupposition is **licensed** when the earlier text — up to **50 tokens back** — contains the same predicate lemma; otherwise it is new information that the hearer must accommodate, and the output says so. The window and the same-predicate test follow Cianflone et al.'s observation that the cue is a repeated event in the context ([../../docs/pragmatics.md](../../docs/pragmatics.md)).

(3) «Mova 0.4, CPU, 2026-10-17»:

```text
The crow sang a song.
Then the crow sang again.
  presupposition [again]: «sing» happened before — «again» needs it, and the earlier text has «sing»
Did the crow sing again?          (as a first sentence)
  presupposition [again]: «sing» happened before (new) — «again» needs it, but the earlier text has no
  «sing»: the hearer must accommodate it as new
The fox came too.
  presupposition [too]: someone or something else «come» too (new) — …
The box is too heavy.
  (nothing: the excess «too» on an adjective is not additive)
She stopped crying.
  presupposition [stopped]: «cry» was going on before (new) — …
Alice knew that the door was locked.
  presupposition [knew]: it is true that «the door was locked» (new) — …
```

## 7. Projection and accommodation

### 7.1 Projection

Presuppositions **project** from embedded positions where entailments do not: *Alice did not know that the door was locked*, *Did Alice know…?*, *If Alice knew…, she…* all still presuppose that the door was locked. Karttunen (1973, "Presuppositions of compound sentences", *Linguistic Inquiry* 4 — *Wanted*) classified embedding contexts as **holes** (let presuppositions through: negation, questions, modals), **plugs** (block them: verbs of saying, *believe*) and **filters** (block them under conditions: *if A then B* filters B's presupposition when A entails it — *If Alice has a cat, her cat is black*). The SEP survey describes the later dynamic and DRT accounts (Beaver, Geurts & Denlinger, *SEP*).

Mova's module has no projection logic of its own: a trigger produces its presupposition wherever it occurs, which is right for holes (negation and questions) and wrong for plugs and filters. In practice the parser decides first. In (4) the trigger is in a hole, but the parser attaches the complement clause as `advcl`, so no presupposition is produced at all:

(4) «Mova en 0.4, CPU, 2026-10-17»:

```text
Alice did not know that the Queen was angry.   advcl(know, angry)   → no presupposition
Did Alice regret that she had come?            advcl(regret, come)  → no presupposition
Alice knew that the door was locked.           ccomp(knew, locked)  → «the door was locked» is true
```

The UD analysis of all three is `ccomp` (23). This is a parser error with a pragmatic cost — a reminder that every pragmatic reading in Mova starts from the tree.

### 7.2 Accommodation

When a presupposition is not in the common ground, cooperative hearers usually add it silently — *I have to pick up my sister* is fine to someone who did not know about the sister (Lewis 1979, "Scorekeeping in a language game" — *Wanted*; Beaver, Geurts & Denlinger, *SEP*). Mova does not refuse unlicensed presuppositions; it marks them `(new)` with the explanation "the hearer must accommodate it as new". In a story reader this is useful: an unlicensed *again* or *still* points to an event the text has not told, a gap in the world of the story.

## 8. Relevance theory and neo-Gricean views

**Relevance theory** (Sperber & Wilson 1986/1995, *Relevance: Communication and Cognition* — not open, *Wanted*) replaces the maxims with one cognitive principle: an utterance creates an expectation of optimal relevance — enough effect to be worth the effort of processing. Implicatures are derived by mutual adjustment of explicit content, context and conclusions; much of what Grice called implicature (enrichment of *and* to *and then*, the resolution of *some*) is treated as part of the explicit content, the *explicature*. The **neo-Gricean** line (Horn's Q- and R-principles; Levinson 2000, *Presumptive Meanings* — *Wanted*) reduces the maxims to two or three heuristics and stresses generalised, default implicatures. Mova's rules follow the neo-Gricean, default style (§5): they fire without looking at the wider context, and context can only block them (negation, cancellation, reordering words).

## 9. Rational Speech Acts

### 9.1 The model

The Rational Speech Acts (RSA) framework models Gricean reasoning as recursive probabilistic inference between a speaker and a listener (Frank & Goodman 2012, "Predicting Pragmatic Reasoning in Language Games", *Science* 336(6084):998, https://doi.org/10.1126/science.1218633, `papers/f/fr/fra/frank-2012-predicting-pragmatic-reasoning-language-games`; Goodman & Frank 2016, "Pragmatic Language Interpretation as Probabilistic Inference", *Trends in Cognitive Sciences* 20(11), https://doi.org/10.1016/j.tics.2016.08.005, `papers/g/go/goo/goodman-2016-pragmatic-language-interpretation-probabilistic-inference`):

| layer | definition | meaning |
|---|---|---|
| literal listener L0 | L0(w \| u) ∝ ⟦u⟧(w) · P(w) | the worlds where the utterance is true, by prior |
| pragmatic speaker S1 | S1(u \| w) ∝ L0(w \| u)^α · e^(−α·C(u)) | chooses utterances that make the listener guess right, at a cost |
| pragmatic listener L1 | L1(w \| u) ∝ S1(u \| w) · P(w) | reasons about which world made the speaker say this |

Monroe & Potts (2015, "Learning in the Rational Speech Acts Model", arXiv:1510.06807, `papers/m/mo/mon/monroe-2015-learning-rational-speech-acts-model`) learn the speaker's features from data; on the TUNA referring-expression corpus a learned RSA beats plain RSA (accuracy 25.3% vs 2.2%, Dice .777 vs .386, as summarised in [../../docs/pragmatics.md](../../docs/pragmatics.md)).

### 9.2 Mova's exact RSA

[../../prag/src/rsa.rs](../../prag/src/rsa.rs) computes RSA over small closed sets **exactly**: α is a small positive integer, costs are whole bits, priors positive integers, so S1 ∝ L0^α · 2^(−α·C) and every value is a reduced fraction (u128 with gcd reduction; overflow fails loudly instead of rounding). Worlds and utterances keep their given order and ties go to the lowest index. The L0/S1/L1 rows are printed as the explanation.

(5) The scalar game, `prag rsa scalar`, «Mova 0.4, CPU, 2026-10-17» (worlds *none*, *some-not-all*, *all*; utterances *none*, *some*, *all*; uniform priors, α = 1, no costs):

```text
"some" (cost 0, alpha 1), true of: some-not-all, all
  none             prior 1    L0 0 (0.0%)      S1 0 (0.0%)        L1 0 (0.0%)
  some-not-all     prior 1    L0 1/2 (50.0%)   S1 1 (100.0%)      L1 3/4 (75.0%)
  all              prior 1    L0 1/2 (50.0%)   S1 1/3 (33.3%)     L1 1/4 (25.0%)
  → some-not-all (L1 3/4; literal L0 1/2)
```

The literal listener cannot choose between *some-not-all* and *all* (1/2 each); the pragmatic listener reasons that a speaker in the *all* world would more likely have said *all*, and reaches 3/4 for *not all* — the scalar implicature as a probability. With a cost of 2 bits on *all* the implicature weakens to 3/5, and with α = 2 then to 5/9 ([../../docs/pragmatics.md](../../docs/pragmatics.md)).

(6) The reference game of Frank & Goodman, `prag rsa reference`: a blue square, a blue circle and a green square; L1(*blue*) gives the blue square 3/5 and the blue circle 2/5 — *blue* points to the square because a speaker meaning the circle could have said *circle*; L1(*square*) gives 3/5 to the blue square. Negative control: utterances true of every world return the prior unchanged ([../../prag/src/rsa.rs](../../prag/src/rsa.rs) tests).

RSA is not yet connected to the sentence reader: it runs on hand-specified games. Its role in Mova is the explained, exact reference model against which the rule-based implicatures can be checked.

## 10. Mova's inference module

### 10.1 What it is

`prag infer <en model> "<text>"` parses the text with the `en` model and prints, per sentence, the scalar implicatures, presuppositions with their licensing, the Quality and Manner inferences, and irony candidates (34). Each inference has a kind, the trigger word, the content, a reason, and (for presuppositions) whether it was licensed ([../../prag/src/infer.rs](../../prag/src/infer.rs), struct `Inference`). It is deterministic; its tests include negative controls (a stronger item, negation, cancellation, stative verbs, reordering words) ([../../docs/pragmatics.md](../../docs/pragmatics.md), *Done (2026-10-03)*).

### 10.2 What it gets wrong

Matching lemmas without the word's function overgenerates. On sentences from *Alice* («Mova 0.4, CPU, 2026-10-17»):

| sentence | output | why it is wrong |
|---|---|---|
| *The rabbit-hole went straight on like a tunnel for some way.* | *like* → not «love» | *like* is a preposition here, not the verb of liking |
| *I should like to be a little larger.* | *like* → not «love» | *should like* is a polite formula of wanting (32), not a scale point |
| *That depends a good deal on where you want to get to.* | *good* → not «excellent» | *a good deal* is an idiom of quantity |
| *That's the most important piece of evidence we've heard yet.* | *most* → not «all» | superlative *most*, not the quantifier |
| *She heard a voice outside, and stopped to listen.* | «listen» was going on before | *stop to V* is *stop (doing something) in order to V*: the infinitive is a purpose, not the stopped activity |
| *Tea or coffee?* | *or* → not «and» | an alternative question presupposes exactly one, so the inference is right in content but comes from the question, not from Quantity |

Each error has a structural fix that the tree supports: require the scale item's UPOS (*like* as VERB, *most* as DET or ADV of quantity, not `advmod` of an adjective), exclude fixed expressions (*a good deal*, *should like*) through the expression base ([../../prag/src/lens.rs](../../prag/src/lens.rs)), and read *stop* + `xcomp` with *to* as purpose ([../../prag/src/frames.rs](../../prag/src/frames.rs) already treats *to*-infinitives that are not control complements as purposes).

## 11. Running it on a tale

`prag infer` was run on 400 sentences of *Alice's Adventures in Wonderland* (`book:pg11:*` in the parse database, in database order — not reading order — so licensing counts are a lower bound) on 2026-10-17:

| inference | count | most frequent triggers |
|---|---:|---|
| Quality (questions and exclamations) | 226 | `?`, `!` |
| scalar implicatures | 127 | *like* 22, *some* 20, *might* 12, *tried* 11, *or* 10, *good* 10, *trying* 7, *may* 6, *few* 6 |
| Manner (told order) | 90 | *and* |
| presuppositions | 69 | *again* 41, *know* 7, *still* 5, *kept* 4, *continued* 3 |
| — of them unlicensed (new) | 64 | |
| irony candidates | 6 | (34 §9) |

Two observations. First, *again* dominates the presuppositions: in a story, *again* links an event to an earlier one, which is exactly the `next` structure the story reader keeps (36). Second, *like* is the most frequent scalar trigger only because of the preposition and the *should like* formula (§10.2) — the count is a measure of the errors as much as of the phenomenon.

## 12. Litotes, understatement and negation

**Litotes** affirms by denying the opposite: *This is not unknown to New Delhi* (EWT 2.18 `weblog-blogspot.com_dakbangla_20050210141134_ENG_20050210_141134-0021`) means it is known, with a hedge or an emphasis that a plain *known* lacks; *not bad* is usually weaker praise than *good*, while *not uncommon* is "fairly common". The tree shows one negation and one negative affix; the meaning combines them (27, §11). Horn's account (*A Natural History of Negation*, 1989 — *Wanted*) explains the weakening as an R-based implicature: the speaker chose the longer, marked form, so a marked meaning is intended (Manner).

**Understatement** (meiosis) says less than is meant: *It's a bit chilly* in a snowstorm; *I was not entirely pleased*. It flouts Quantity in the direction opposite to hyperbole and is a common vehicle of British irony (34).

**Negative questions** carry a bias: *Can you not understand that?* (`book:pg1597:1669`) expects *yes* and reproaches; *Isn't it lovely?* invites agreement. The experimental and theoretical work on such biased questions is collected in Trinh, Benz, Goodhue, Yatsushiro & Krifka (eds.) 2025, *Biased questions: Experimental results and theoretical modelling*, Language Science Press, `books/k/kr/krifka-2025-biased-questions-experimental-results` (26).

In Mova, scalar implicatures are blocked under negation (§4.2); litotes and negative-question bias are not computed. The `polarity` label of `prag.tsv` is the speaker's evaluation, not grammatical negation (30 §6; 27).

## 13. Datasets

All of the following are for tests and measurements only, under their own licences (`licence/DATA.md` in the repo; [../../docs/pragmatics.md](../../docs/pragmatics.md)):

| dataset | what it tests | size | source |
|---|---|---|---|
| IMPPRES | scalar implicatures and presupposition triggers as NLI pairs | ~32 000 semi-automatically generated pairs | Jeretic, Warstadt, Bhooshan & Williams 2020, https://aclanthology.org/2020.acl-main.768, `papers/j/je/jeretic-2020-are-natural-language-inference-models` |
| Circa | indirect answers to polar questions (*Hungry? — I'm starving.*) | 34 268 question–answer pairs | Louis, Roth & Radlinski 2020, "'I'd rather just go to bed': Understanding Indirect Answers", https://aclanthology.org/2020.emnlp-main.601, `papers/l/lo/lou/louis-2020-i-d-rather-just-go` |
| GRICE | implicature recovery and conversational reasoning in generated dialogues | grammar-generated | Zheng, Qiu, Fan, Zhu & Zhu 2021, https://aclanthology.org/2021.findings-acl.182, `papers/z/zh/zhe/zhen/zheng/zheng2021/zheng-2021-grice-grammar-based-dataset-recovering` |
| adverbial presupposition triggers | predicting *too, again, also, still, yet* from context | PTB- and Gigaword-derived | Cianflone et al. 2018 (§6.1) |
| TUNA (via RSA) | referring expressions | — | Monroe & Potts 2015 (§9) |

Jeretic et al. report that BERT trained on MultiNLI reliably treats the scalar implicature of *some* as an entailment and recognises some presupposition triggers such as *only*, though MultiNLI contains very few such pairs — useful as a point of comparison for Mova's rule-based output, which states its inference and its reason instead of a label.

## In Mova

- Implicatures, presuppositions, Quality and Manner: [../../prag/src/infer.rs](../../prag/src/infer.rs) (`SCALES`, `implicatures`, `presuppositions`, `maxims`, `STATIVE`, `REORDER`); CLI `prag infer <en model> "<text>"`.
- Exact RSA: [../../prag/src/rsa.rs](../../prag/src/rsa.rs); CLI `prag rsa scalar|reference`.
- Told order as frame links (`next`): [../../prag/src/frames.rs](../../prag/src/frames.rs); design: [../../docs/pragmatics.md](../../docs/pragmatics.md).
- **Not yet**:
  - no projection logic (holes, plugs, filters) — a trigger fires wherever the parser shows it with the right relation, and not at all when the parser mislabels the complement (§7.1);
  - scale items are matched by lemma without their part of speech or function (§10.2);
  - Relation inferences (indirect answers), clausal implicatures, conventional implicatures of *but*, *even*, *only*, and presuppositions of definites, clefts, possessives, wh-questions and temporal clauses are not computed;
  - the inferences are printed, not stored: they do not enter frames, the world or the student's answers;
  - RSA runs on hand-specified games only.

## Sources

- Grice, H. P. 1975. Logic and Conversation. Not open — *Wanted*.
- Horn, L. R. 1972. *On the Semantic Properties of Logical Operators in English*; 1989. *A Natural History of Negation*. Not open — *Wanted*.
- Karttunen, L. 1973. Presuppositions of compound sentences. *Linguistic Inquiry* 4. Not open — *Wanted*.
- Lewis, D. 1979. Scorekeeping in a language game. *Journal of Philosophical Logic* 8. Not open — *Wanted*.
- Levinson, S. C. 1983. *Pragmatics*; 2000. *Presumptive Meanings*. Not open — *Wanted*.
- Sperber, D., Wilson, D. 1986/1995. *Relevance: Communication and Cognition*. Not open — *Wanted*.
- Davis, W. "Implicature". *Stanford Encyclopedia of Philosophy*. https://plato.stanford.edu/entries/implicature/
- Beaver, D. I., Geurts, B., Denlinger, K. "Presupposition". *Stanford Encyclopedia of Philosophy*. https://plato.stanford.edu/entries/presupposition/
- Tatu, M., Moldovan, D. 2012. A Tool for Extracting Conversational Implicatures. LREC. https://aclanthology.org/L12-1039; `papers/t/ta/tat/tatu-2012-tool-extracting-conversational-implicatures`
- Cianflone, A., Feng, Y., Kabbara, J., Cheung, J. C. K. 2018. Let's do it "again": A First Computational Approach to Detecting Adverbial Presupposition Triggers. ACL. https://aclanthology.org/P18-1256; `papers/c/ci/cianflone-2018-let-s-do-it-again`
- Frank, M. C., Goodman, N. D. 2012. Predicting Pragmatic Reasoning in Language Games. *Science* 336(6084):998. https://doi.org/10.1126/science.1218633; `papers/f/fr/fra/frank-2012-predicting-pragmatic-reasoning-language-games`
- Goodman, N. D., Frank, M. C. 2016. Pragmatic Language Interpretation as Probabilistic Inference. *Trends in Cognitive Sciences* 20(11). https://doi.org/10.1016/j.tics.2016.08.005; `papers/g/go/goo/goodman-2016-pragmatic-language-interpretation-probabilistic-inference`
- Monroe, W., Potts, C. 2015. Learning in the Rational Speech Acts Model. arXiv:1510.06807. `papers/m/mo/mon/monroe-2015-learning-rational-speech-acts-model`
- Potts, C., Lassiter, D., Levy, R., Frank, M. C. 2016. Embedded implicatures as pragmatic inferences under compositional lexical uncertainty. *Journal of Semantics* 33(4). `papers/p/po/pot/potts-2016-embedded-implicatures-pragmatic-inferences-under`
- Jeretic, P., Warstadt, A., Bhooshan, S., Williams, A. 2020. Are Natural Language Inference Models IMPPRESsive? Learning IMPlicature and PRESupposition. ACL. https://aclanthology.org/2020.acl-main.768; `papers/j/je/jeretic-2020-are-natural-language-inference-models`
- Louis, A., Roth, D., Radlinski, F. 2020. "I'd rather just go to bed": Understanding Indirect Answers. EMNLP. https://aclanthology.org/2020.emnlp-main.601; `papers/l/lo/lou/louis-2020-i-d-rather-just-go`
- Zheng, Z., Qiu, S., Fan, L., Zhu, Y., Zhu, S.-C. 2021. GRICE: A Grammar-based Dataset for Recovering Implicature and Conversational rEasoning. Findings of ACL. https://aclanthology.org/2021.findings-acl.182
- Trinh, T., Benz, A., Goodhue, D., Yatsushiro, K., Krifka, M. (eds.) 2025. *Biased questions: Experimental results and theoretical modelling*. Language Science Press. `books/k/kr/krifka-2025-biased-questions-experimental-results`
- UD English EWT 2.18 (CC BY-SA 4.0), sentences cited by `sent_id`.
- Mova's parse database (`$MOVA_DATA/parsedb/current.conllu`): *Alice's Adventures in Wonderland* (`pg11`), *Andersen's Fairy Tales* (`pg1597`); runs of `prag infer` and `prag rsa` with the release 0.4 model, 2026-10-17.
- Repo: [../../docs/pragmatics.md](../../docs/pragmatics.md).
