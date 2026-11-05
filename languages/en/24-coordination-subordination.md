# 24. Coordination and subordination

Clauses and phrases combine in two ways: as equals (coordination, with or without a conjunction, and the looser joining of parataxis and lists) or as head and dependent (subordination: adverbial clauses marked by a subordinator or by non-finite form). This chapter describes both in English, their analysis in Universal Dependencies (`conj`, `cc`, `cc:preconj`, `parataxis`, `list`, `appos`, `advcl`, `mark`, `advcl:relcl`, and the enhanced labels `conj:and`, `advcl:if` with the propagation of shared dependents), how the UD guidelines, EWT and Mova's parser differ, and how Mova's pragmatics turns conjunctions and subordinators into links between frames: cause, what came next, and condition.

Contents

1. [Overview](#1-overview)
2. [Coordination in UD: the first conjunct is the head](#2-coordination-in-ud-the-first-conjunct-is-the-head)
3. [Coordinators and their meanings](#3-coordinators-and-their-meanings)
4. [Correlatives: `cc:preconj`](#4-correlatives-ccpreconj)
5. [Shared dependents and the enhanced graph](#5-shared-dependents-and-the-enhanced-graph)
6. [Coordination of unlikes, and what coordination is not](#6-coordination-of-unlikes-and-what-coordination-is-not)
7. [Loose joining: parataxis, comma splices, lists, asyndeton](#7-loose-joining-parataxis-comma-splices-lists-asyndeton)
8. [Subordination: adverbial clauses (`advcl`)](#8-subordination-adverbial-clauses-advcl)
9. [Subordinators: inventory, multiword and fixed forms](#9-subordinators-inventory-multiword-and-fixed-forms)
10. [Non-finite and verbless adverbial clauses; absolutes](#10-non-finite-and-verbless-adverbial-clauses-absolutes)
11. [Adverbial relatives: `advcl:relcl` and sentential relatives](#11-adverbial-relatives-advclrelcl-and-sentential-relatives)
12. [Comparative clauses](#12-comparative-clauses)
13. [Mova's parser on coordination and subordination](#13-movas-parser-on-coordination-and-subordination)
14. [From conjunctions to frame links](#14-from-conjunctions-to-frame-links)

Cross-references: the relation inventory and the enhanced layer in [20-syntax-overview.md](20-syntax-overview.md); complement clauses in [23-complementation.md](23-complementation.md); relative clauses in [25-relative-clauses.md](25-relative-clauses.md); gapping and other ellipsis in coordination in [29-ellipsis-word-order-information-structure.md](29-ellipsis-word-order-information-structure.md); discourse relations above the sentence in [36-discourse-relations.md](36-discourse-relations.md).

---

## 1. Overview

| | coordination | subordination |
|---|---|---|
| status of the parts | equal; any conjunct could stand alone | one clause depends on another |
| linking word | coordinator *and, or, but, nor, yet, so, for* (CCONJ) | subordinator *because, if, although, when …* (SCONJ or ADV) |
| UD relation of the second part | `conj` (to the first conjunct) | `advcl` (to the head of the main clause) |
| UD relation of the linking word | `cc` (to the following conjunct) | `mark` (to the subordinate predicate) or `advmod` (wh-adverbs) |
| order | fixed: *\*And I saw, I came* | the subordinate clause moves: *If it rains, we stay* / *We stay if it rains* |
| enhanced label | `conj:and`, `conj:or`, `conj:but` | `advcl:if`, `advcl:because`, `advcl:when` |

Curme divides compound sentences into copulative, disjunctive, adversative and causal coordination (1931, *Syntax*, ch. IX "Classes of Sentences: The Compound Sentence", pp. 161–173, `books/c/cu/curme-1931-grammar-english-language-vol`) and adverbial clauses into place, time, manner, degree, cause, condition, concession, purpose and means (ch. XVI "Adverbial Clause", pp. 259–346). The tables below follow those classes.

EWT 2.18 counts (train + dev + test; 2026-10-17):

| relation | count | relation | count |
|---|---:|---|---:|
| `conj` | 9 395 | `parataxis` | 2 031 |
| `cc` | 8 292 | `appos` | 1 691 |
| `mark` | 8 633 | `list` | 1 061 |
| `advcl` | 4 527 | `advcl:relcl` | 141 |
| `cc:preconj` | 98 | | |

---

## 2. Coordination in UD: the first conjunct is the head

UD treats coordination asymmetrically: the first conjunct is the head of the coordinate structure, every later conjunct attaches to it as `conj`, and each coordinator attaches as `cc` to the conjunct that **follows** it (UD guidelines `_en/dep/conj.md`, `_en/dep/cc.md`, https://universaldependencies.org/en/dep/conj.html). The coordinate structure as a whole takes its relation (subject, object …) on the first conjunct.

(1) *Bill is big and honest.* — `conj(big, honest)`, `cc(honest, and)`.
(2) *He came, saw and conquered.* — `conj(came, saw)`, `conj(came, conquered)`, `cc(conquered, and)`: flat, not nested.

```
# text = Now you can tour Chernobyl and write your own story.
#        (EWT 2.18 weblog-typepad.com_ripples_20040407125600_ENG_20040407_125600-0004)
ID FORM       HEAD DEPREL DEPS
2  you        4    nsubj  4:nsubj|7:nsubj
4  tour       0    root   0:root
5  Chernobyl  4    obj    4:obj
6  and        7    cc     7:cc
7  write      4    conj   4:conj:and
10 story      7    obj    7:obj
```

A sentence-initial coordinator is `cc` of the root: *And then we left* — `cc(left, And)`. This is common in narrative and in the Torah English, where nearly every verse begins with *And*:

(3) *And God said: 'Let there be light.' And there was light.* — `cc(said, And)`, `conj(said, was)`, `cc(was, And)` (`jps:GEN:2`).

**History.** In Stanford Dependencies and UD v1 the coordinator hung on the first conjunct; UD v2 moved it to the following conjunct (de Marneffe et al. 2021, https://aclanthology.org/2021.cl-2.11; the digest `docs/papers/ud/english-banks.md`, §1.3). The move is the main source of non-projective arcs in some converted treebanks (cited in [`en/seeds/errors/coordination-structure.md`](../../en/seeds/errors/coordination-structure.md): W18-4918, https://aclanthology.org/W18-4918). In EWT 2.18 all 9 395 `conj` go left to right and 8 292 of 8 295 `cc` stand before their head (the same seed, counted 2026-09-25).

**Criticism.** The asymmetric head is a convention, not a finding. Przepiórkowski and Woźniak (2023, "Conjunct Lengths in English, Dependency Length Minimization, and Dependency Structure of Coordination", ACL, https://aclanthology.org/2023.acl-long.864) show that the tendency of shorter conjuncts to come first in the Penn Treebank is explained by symmetric (or coordinator-headed) structures, not by the first-conjunct-headed one. Other dependency traditions make the coordinator the head (Prague) or chain the conjuncts (Mel'čuk's Meaning-Text Theory); the Surface-Syntactic UD keeps the UD conjunct head but differs on function words (Gerdes et al. 2018, https://aclanthology.org/W18-6008; Osborne and Song 2025 on head-initial versus head-final coordinate structures, https://aclanthology.org/2025.depling-1.6, `papers/o/os/osborne-2025-head-initial-head-final-coordinate`).

---

## 3. Coordinators and their meanings

The coordinators of English, with their EWT 2.18 enhanced labels (the lemma of the `cc` written into `conj:<lemma>`, counted 2026-10-17):

| coordinator | relation of the conjuncts (Curme's class) | EWT `conj:` label | notes |
|---|---|---:|---|
| *and* | copulative: addition, sequence, result | `conj:and` 6 699 | in narrative, sequence by default (§14) |
| *or* | disjunctive: alternatives | `conj:or` 898 | inclusive or exclusive by context |
| *but* | adversative: contrast, denial of expectation | `conj:but` 730 | |
| *nor* | negative copulative | `conj:nor` 20 | after a negative clause; subject-auxiliary inversion: *nor did he* |
| *yet* | adversative | `conj:yet` 15 | |
| *and/or*, */* | | `conj:and_or` 34, `conj:slash` 37 | the slash is `cc` |
| *as well as* | additive, multiword | `conj:as_well_as` 24 | *as* `cc` + `fixed` *well*, *as* |
| *rather than* | substitutive, multiword | `conj:rather_than` 17 | |
| no coordinator | asyndetic | `conj:` (empty) 884 | the enhanced label has no lemma |
| *so*, *for* | result, cause | (rare as `cc`) | *for* is archaic as a coordinator; *so* is usually `advmod` in EWT |

*So* and *for* sit between coordinators and adverbs or subordinators. UD English tags clause-linking *so* as ADV/`advmod` in most cases; Mova's dialect makes this a rule: an adverb is never `discourse`, it is `advmod` ([`dialects/mova/seeds.md#discourse-adverbs`](../../dialects/mova/seeds.md#discourse-adverbs); in GUM *so* is `discourse` 57 times and `advmod` 622).

(4) *Or they weren't eyewitnesses and are just lying.* — `cc(weren't…, Or)` sentence-initially, and `conj` of a verb phrase with the shared subject *they* (EWT 2.18 weblog-juancole.com_juancole_20040823064025_ENG_20040823_064025-0004; enhanced `nsubj` from both predicates).

---

## 4. Correlatives: `cc:preconj`

The first part of a correlative pair *both … and*, *either … or*, *neither … nor* is `cc:preconj`, attached to the first conjunct (UD guidelines `_en/dep/cc-preconj.md`). They are not determiners here: *Both the cat and the dog are here* does not entail *\*Both the cat is here*.

(5) *Both India and Afghanistan deny the claim.* — `cc:preconj(India, Both)` (EWT 2.18 weblog-blogspot.com_dakbangla_20041016151002_ENG_20041016_151002-0005).
(6) *Either fax or someone else works fine.* — `cc:preconj(fax, Either)` (EWT 2.18 email-enronsent24_01-0068).
(7) *Work-life balance means having time both for work and for family.* — `cc:preconj(work, both)` (`$MOVA_DATA/parsedb/current.conllu`, school:money:029-jobs-and-what-people-earn.lesson:13).

Other correlatives are not `cc:preconj`: *not only … but also* (`advmod` *only*, `cc` *but*, `advmod` *also*), *whether … or* (the first is `mark`), *the more … the more* (comparative correlatives, §12).

A Mova error worth knowing: in *At least one: 14 + 12 - 5 = 21, neither: 30 - 21 = 9* the school tree has *neither* as `cc:preconj` although it is a pronoun-like label in a list (school:lessons:175-venn-neither.lesson:4).

---

## 5. Shared dependents and the enhanced graph

### 5.1 What the basic tree cannot show

In the basic tree a dependent shared by all conjuncts attaches to the first one only. *The store buys and sells cameras* has `nsubj(buys, store)` and `obj(sells, cameras)`: the tree says neither that the store sells nor that it buys cameras. Two conventions decide where a shared dependent goes in the basic tree:

| shared dependent | basic attachment | example |
|---|---|---|
| shared subject (on the left) | first conjunct | *you* in (EWT) *you can tour … and write* |
| shared object (on the right) | the nearest conjunct (the last) | *buys and sells **cameras*** |
| a modifier of the whole coordination | first conjunct | *the old men and women* (ambiguous) |

### 5.2 Propagation in enhanced UD

Enhanced UD adds the missing arcs (UD guidelines `_u-overview/enhanced-syntax.md`, "Propagation of incoming dependencies to conjuncts" and "Propagation of outgoing dependencies from conjuncts"; Schuster and Manning 2016, https://aclanthology.org/L16-1376):

1. **Incoming**: every conjunct gets the relation of the first: in *Both India and Afghanistan deny the claim* both *India* and *Afghanistan* are `nsubj` of *deny*.
2. **Outgoing**: a dependent shared by coordinated predicates attaches to each: the subject *you* above has `4:nsubj|7:nsubj`; auxiliaries too (*can tour and write*: *can* is `aux` of both only if the enhancement is applied to auxiliaries, as in the UD examples).
3. **Labels**: `conj` carries the coordinator's lemma: `conj:and`, `conj:or`.

```
# text = Do we expect to have ONE EPC contract, or several?
#        (EWT 2.18 email-enronsent26_01-0027)
ID FORM      HEAD DEPREL DEPS
5  have      3    xcomp  3:xcomp
8  contract  5    obj    5:obj
10 or        11   cc     11:cc
11 several   8    conj   5:obj|8:conj:or
```

### 5.3 Limits

- **Object propagation** in coordinated verb phrases is not done when ambiguous: in *she was reading or watching a movie*, is the movie read? (Schuster and Manning 2016; the digest `docs/papers/ud/ud-core.md`, §4).
- **Distributive versus collective** readings are not distinguished: *John and Mary met* (collective) and *John and Mary left* (distributive) get the same arcs (UD issue #876, open).
- **Converter errors** cluster on coordinations whose conjuncts differ in tense or mood (Grünewald et al. 2021, a study of 1 417 EWT sentences, https://aclanthology.org/2021.eacl-main.67, as summarised in [`coordination-structure.md`](../../en/seeds/errors/coordination-structure.md)).

Mova's parser writes no DEPS. Its frames reader propagates the subject itself: a `conj` predicate without its own subject takes the agent of the predicate it is coordinated with ([`prag/src/frames.rs`](../../prag/src/frames.rs), the agent inheritance for `Conj`), so *Alice opened the door and walked in* gives two frames with the agent *alice* (§14).

### 5.4 Agreement with coordinated subjects

Two singular subjects joined by *and* take a plural verb (*the cat and the dog are*); with *or / nor* the verb usually agrees with the nearest conjunct (*either the cat or the dogs are*); a coordination naming one thing is singular (*bread and butter is …*). In UD the features of the verb follow the form (VBP is never `Sing|3`), whatever the meaning ([`dialects/mova/seeds.md#features-context`](../../dialects/mova/seeds.md#features-context); [`en/seeds/verbal/agreement-compound-subject.md`](../../en/seeds/verbal/agreement-compound-subject.md); [`en/seeds/errors/coordinated-subject-agreement.md`](../../en/seeds/errors/coordinated-subject-agreement.md)).

---

## 6. Coordination of unlikes, and what coordination is not

### 6.1 Unlike categories

English coordinates phrases of different categories that share a function: *He is a fool and proud of it* (NP and AdjP as predicates), *slowly and with great care* (AdvP and PP), *Your name and where you live* (NP and clause). UD needs no special device: the first conjunct heads, and its category decides the relation of the whole. In a copular coordination of unlikes the copula attaches to the first conjunct only; the enhanced graph propagates the subject but not the copula.

### 6.2 Apposition is not coordination

An appositive renames its head without a coordinator: *My friend, a doctor, helped me* — `appos(friend, doctor)`. Apposition is between nominals, the second is not linked by a coordinator, and only the first takes the external relation (UD guidelines `_en/dep/appos.md`). EWT has 1 691 `appos`, many in emails (*Take care, my friend, Linda* — `appos(friend, Linda)`, EWT 2.18 weblog-typepad.com_ripples_20040407125600_ENG_20040407_125600-0062). Appositions are a rich source of definitional facts (*Phuket, an island in Thailand*) and of coreference ([35-reference-and-coreference.md](35-reference-and-coreference.md)).

| | coordination | apposition |
|---|---|---|
| referents | two or more | one |
| linker | coordinator (or asyndetic comma) | comma, dash, parentheses |
| verb agreement with two singulars | plural | singular |
| UD | `conj` + `cc` | `appos` |

### 6.3 Comitative *with*, and *as well as*

*The President, with his advisers, was …* is not coordination: the verb is singular and *with his advisers* is `nmod`/`obl`. *As well as* behaves as a coordinator in EWT (`conj:as_well_as`, 24 times) but as an adverb when final (*Please deliver a copy to me **as well*** — `fixed(as, well)`, `advmod`, EWT 2.18 email-enronsent13_01-0006).

### 6.4 Coordinated infinitives and subordinate clauses

Coordinated *to*-infinitives may repeat *to* or not (*to buy and (to) sell*); the second conjunct's *to*, if present, is its own `mark` ([`en/seeds/verbal/infinitive-coordination.md`](../../en/seeds/verbal/infinitive-coordination.md)). Coordinated subordinate clauses share the subordinator only in the enhanced graph: *if it rains and the road floods* — `mark(rains, if)`; *floods* is `conj` of *rains* and inherits `advcl:if` in DEPS.

---

## 7. Loose joining: parataxis, comma splices, lists, asyndeton

### 7.1 Parataxis

`parataxis` links the main predicate to a clause placed beside it without coordination or subordination: a parenthetical, a clause after a colon or semicolon, a run-on (UD guidelines `_en/dep/parataxis.md`, https://universaldependencies.org/en/dep/parataxis.html):

(8) *Let's face it we're annoyed.* — `parataxis(Let, annoyed)`.
(9) *You know, nature hates a void. :)* — `parataxis(hates, know)`: the parenthetical points backward to the main clause on its right (EWT 2.18 weblog-blogspot.com_rigorousintuition_20060511134300_ENG_20060511_134300-0146).
(10) *The guy, John said, left early in the morning.* — `parataxis(left, said)`.
(11) *They really are milking the incompetence angle though, aren't they?* — the tag question is `parataxis(milking, are)` (EWT 2.18 weblog-blogspot.com_rigorousintuition_20060511134300_ENG_20060511_134300-0101; see [26-questions.md](26-questions.md) §7).

Several paratactic clauses in one sentence are flat: all attach to the first, even when they tell a sequence. EWT 2.18 has 2 031 `parataxis`.

### 7.2 Comma splices and run-ons

Informal writing joins full clauses with a comma alone (*just go here, its simply amazing*). UD has two options: `parataxis` (EWT's usual choice: `parataxis(go, amazing)`, EWT 2.18 weblog-blogspot.com_rigorousintuition_20060511134300_ENG_20060511_134300-0076) or asyndetic `conj` when the clauses form a list of like events (*I came, I saw, I conquered*). The guidelines leave the choice to meaning: a list of parallel items is coordination; a comment, explanation or afterthought is parataxis. Mova's school trees use `parataxis` for a colon followed by an explanation:

(12) *Some children are adopted: they join a family that did not give birth to them, and they are just as much part of it.* — `parataxis(adopted, join)`, `conj(join, are)` (school:religion-ethics:051-life-begins-being-born-and-growing-up.lesson:7).

### 7.3 Lists

`list` joins items of a list that are not coordinated sentence parts, typical of addresses, signatures and tables: *Maria Valdes superior $62,500* — `list(Maria, superior)`, `list(Maria, $)` (EWT 2.18 email-enronsent30_01-0050). EWT has 1 061 `list`, almost all in emails and headings.

### 7.4 Asyndetic coordination

A comma alone can coordinate when the items are parallel: *red, white, blue*; the last item usually gets *and*. Without any coordinator the enhanced label is bare `conj:` (884 cases in EWT 2.18).

| loose joining | UD | test |
|---|---|---|
| parallel items, coordinator absent | `conj` | an *and* could be inserted without changing the meaning |
| comment, explanation, parenthetical | `parataxis` | *and* would be odd; a colon or dash fits |
| items of an address, signature, table row | `list` | not a sentence |
| speech reporting inside a quote | `parataxis` | *X, he said, Y* |

---

## 8. Subordination: adverbial clauses (`advcl`)

An adverbial clause modifies a predicate as an adjunct, not as a complement: time, cause, condition, concession, purpose, result, manner, comparison. Its predicate attaches to the main predicate as `advcl`; its subordinator is `mark` on the subordinate predicate (UD guidelines `_en/dep/advcl.md`, https://universaldependencies.org/en/dep/advcl.html; `_en/dep/mark.md`):

(13) *If you know who did it, you should tell the teacher.* — `advcl(tell, know)`, `mark(know, If)`.
(14) *He was upset when I talked to him.* — `advcl(upset, talked)`, `advmod(talked, when)`.
(15) *Do you return fire, if you are attacked from a hospital?* — `advcl(return, attacked)` (EWT 2.18 weblog-blogspot.com_alaindewitt_20060924104100_ENG_20060924_104100-0173).
(16) *1584 Ivan the Terrible dies while starting a game of chess.* — `advcl(dies, starting)`, `mark(starting, while)` (EWT 2.18 newsgroup-groups.google.com_humanities.lit.authors.shakespeare_0018a7697318f71f_ENG_20031006_163200-0077).

The semantic classes with their subordinators (after Curme 1931, ch. XVI) and the EWT 2.18 enhanced label counts (`advcl:<marker>`, 2026-10-17):

| class | subordinators | EWT `advcl:` counts | example |
|---|---|---|---|
| purpose | *to*, *in order to*, *so that*, *so as to*, *lest* | to 915, so_that 32 | *He left early so that he could catch the train* |
| condition | *if*, *unless*, *provided (that)*, *in case*, *as long as*, *once* | if 787, unless 32, once 33 | *If it rains, we will stay home* |
| time | *when*, *while*, *before*, *after*, *until*, *since*, *as*, *once*, *as soon as*, *whenever* | when 416, before 105, while 102, after 99, until 51 | *Forces engaged in fighting after insurgents attacked* |
| cause | *because*, *since*, *as*, *for*, *now that*, *seeing that* | because 201, since 107, as 389 (all senses) | *He ran away because he was afraid* |
| concession | *although*, *though*, *even though*, *even if*, *while*, *whereas* | although 50, though 40 | *Although she was tired, she kept working* |
| result | *so that*, *so … that*, *such … that* | (in so_that, that 25) | *They were so hungry that they ate the rats* |
| manner and comparison | *as*, *as if*, *as though*, *like*, *than* | like 87, than 54 | *She sings better than she dances* |
| place | *where*, *wherever* | where 14 | *Where I had lunch yesterday, it was very windy* (a free relative, §11) |
| means | *by* + *-ing*, *without* + *-ing* | by 73, without 27 | *She won by training every day* |
| preposition + clause | *about*, *of*, *on*, *for*, *in*, *with* + *-ing* | in 113, with 85, for 77, on 58, of 55, about 54 | *They heard about you missing classes* |

Because one subordinator may serve several classes (*as*, *since*, *while*, *once*), the enhanced label records the word, not the meaning. The class is read by the pragmatics reader from the word and the clause (§14).

**WH-adverbs are not `mark`.** *When, where, why, how* introduce adverbial clauses but also play a role inside them, so they are ADV with `advmod` (never `mark`), and a clause they introduce on its own is either `advcl` (with *when* as `advmod`) or a free relative with the WH-word as head (`advcl:relcl`) ([`en/seeds/verbal/wh-adverb-clause.md`](../../en/seeds/verbal/wh-adverb-clause.md), rule `en.verbal.wh-adverb-not-mark`; UD guidelines `_en/dep/acl-relcl.md`, "RCs versus Non-relative Modifier Clauses").

**Purpose versus complement.** An infinitive of purpose is `advcl` (it can take *in order to* and can be added to almost any action); an infinitive required by the verb is `xcomp` ([`en/seeds/verbal/purpose-clause.md`](../../en/seeds/verbal/purpose-clause.md); [23-complementation.md](23-complementation.md) §6): *I came **to see** you* (`advcl`) versus *I want **to see** you* (`xcomp`). EWT 2.18:

(17) *<John can come over to eat at my house anytime.* — `advcl(come, eat)` (EWT 2.18 newsgroup-groups.google.com_INTPunderground_b2c62e87877e4a22_ENG_20050906_165900-0034).
(18) *He talked to him in order to secure the account.* — `advcl(talked, secure)`, `mark(secure, in)`, `fixed(in, order)`, `mark(secure, to)` (UD guidelines `_en/dep/advcl.md`).

**Order and punctuation.** A preposed adverbial clause is usually followed by a comma (*If it rains, …*); a postposed one has no comma when restrictive (*We stay if it rains*). Postposed *because*-clauses after a comma often give the speaker's reason for saying something rather than the cause of the event (*He's home, because his car is outside*): Sweetser's epistemic and speech-act readings (Sweetser 1990, *From Etymology to Semantics*, Cambridge UP; not open, listed as wanted). UD does not mark the difference.

---

## 9. Subordinators: inventory, multiword and fixed forms

### 9.1 The `mark` inventory

The most frequent `mark` words in EWT 2.18 (all clause types, 2026-10-17): *to* 4 021, *that* 1 133, *if* 899, *as* 423, *because* 202, *for* 190, *of* 153, *in* 120, *since* 109, *before* 108, *with* 105, *like* 105, *while* 104, *after* 102, *so* 84, *on* 77, *about* 71, *by* 68, *than* 57, *whether* 54, *although* 50, *though* 45, *until* 44, *from* 40, *unless* 36, *once* 33, *without* 25.

`mark` is a function word: never NOUN, PRON, ADJ or AUX; its children can only be `advmod` (*just because*, *right after*), `fixed`, `conj`, `cc`, `punct` ([`en/seeds/verbal/subordinators-mark.md`](../../en/seeds/verbal/subordinators-mark.md), rule `en.verbal.mark-leaf`, following the UD validator). A preposition that introduces a clause is `mark`, not `case`: *on whether users are at risk* has two marks (UD guidelines `_en/dep/mark.md`). *Because of the rain* has *because* as `case` (a preposition), since its dependent is a nominal.

### 9.2 Multiword subordinators: `fixed`

A grammaticalized multiword expression gets a flat `fixed` structure headed by its first word, and the whole acts as one `mark` or `cc` or `case` (UD guidelines `_u/dep/fixed.md`). The commonest in EWT 2.18 (the first word + `fixed` word, 2026-10-17):

| expression | count | function |
|---|---:|---|
| *as well (as)* | 92 | adverb or coordinator |
| *due to* | 55 | preposition |
| *because of* | 48 | preposition |
| *according to* | 46 | preposition |
| *such as* | 33 | preposition |
| *as … as* (the second *as*) | 30 | comparison |
| *so that* | 29 | subordinator |
| *instead of* | 24 | preposition |
| *rather than* | 18 | coordinator or preposition |
| *in order (to)* | 17 | subordinator |
| *prior to* | 16 | preposition |
| *whether or (not)* | 10 + 10 | subordinator |
| *in case* | 10 | subordinator |
| *as to*, *as for*, *as of* | 20, 5, 8 | prepositions |
| *let alone* | 4 | coordinator |

*Even though* and *even if* are not `fixed`: *even* is an `advmod` of the subordinator, since *even* modifies other subordinators freely (*even when*, *even after*). *As if* and *as though* are `fixed` in some treebanks and two marks in others.

Mova parses *so that* as `mark` + `fixed` (§13) and uses the pair in the frames reader (*so … that* result versus *so that* purpose, §14).

---

## 10. Non-finite and verbless adverbial clauses; absolutes

Adverbial clauses need not be finite:

| form | example | UD |
|---|---|---|
| *to*-infinitive (purpose) | *I came to see you* | `advcl` + `mark(to)` |
| *-ing* participle | *Walking home, she met a friend* | `advcl` |
| past participle | *Asked about it, he refused* | `advcl` (`Voice=Pass`) |
| subordinator + participle | *while starting a game of chess* | `advcl` + `mark(while)` |
| subordinator + adjective | *She entered the room while sad*; *If mistaken, …* | `advcl` of the adjective |
| absolute (own subject) | *The work done, they went home* | `advcl(went, done)`, `nsubj:pass(done, work)` |
| *with*-absolute | *With the kids in school, I have plenty of free time* | `advcl(have, school)`, `mark(school, With)`, `nsubj(school, kids)` |

(19) *If mistaken, how did those reports first come about?* — `advcl(come, mistaken)` (EWT 2.18 weblog-blogspot.com_dakbangla_20050311135387_ENG_20050311_135387-0111).
(20) *With Colin Powell around, the U.S. has no need of an Osama bin Laden* — `mark(around, With)`, `nsubj(around, Powell)` (EWT 2.18 weblog-blogspot.com_dakbangla_20041028153019_ENG_20041028_153019-0036).

The **absolute construction** has its own subject and no grammatical link to the main clause (Reed and Kellogg 1877, Lesson 44, `books/r/re/reed-1877-higher-lessons-english`; Brown 1851, Rule VIII "Nom. Absolute", `books/b/br/brown-1851-grammar-english-grammars`; Jespersen MEG V, ch. VI on "nexus-tertiaries" and absolute participles, `books/j/je/jespersen-1914-modern-english-grammar-historical`, text-1 — Jespersen notes that *He stood, hat in hand* is literary, *with his hat in his hand* colloquial). In UD the head of the absolute (participle, adjective, noun or preposition-headed predicate) is `advcl` of the main predicate; its subject is `nsubj` or, for a passive participle, `nsubj:pass` without `aux:pass` ([`en/seeds/verbal/absolute-construction.md`](../../en/seeds/verbal/absolute-construction.md)).

**Dangling participles** (*Walking home, the rain started*) have the same tree as well-formed ones; the reader that inherits the participle's agent from the main subject will get the wrong agent. Mova's frames reader does exactly that inheritance for `advcl` without a subject (§14).

---

## 11. Adverbial relatives: `advcl:relcl` and sentential relatives

A relative clause whose antecedent is a clause, not a noun, is `advcl:relcl` (UD guidelines `_en/dep/advcl-relcl.md`, https://universaldependencies.org/en/dep/advcl-relcl.html):

(21) *I tried to explain myself – which was a bad idea.* — `advcl:relcl(tried, idea)`, `nsubj(idea, which)`.
(22) *Yes obviously the USB is region free which means you can use it anywhere.* — `advcl:relcl(free, means)`, `nsubj(means, which)`; enhanced `ref(free, which)` (EWT 2.18 answers-20111108104015AAGxHNm_ans-0004).
(23) *It's around $100 less, which means you get the same camera but quicker!* — `advcl:relcl(less, means)` (EWT 2.18 answers-20111108082505AAtbdh9_ans-0012).
(24) *Which I then realized I missed.* — a sentential relative written as its own sentence; EWT annotates *Which* as `obj` of *missed* in a root clause (EWT 2.18 email-enronsent37_01-0093).

The same label serves free relatives headed by a WH-adverb (*I looked where you were sitting* — `advmod(looked, where)`, `advcl:relcl(where, sitting)`) and *it*-clefts (*It's John who we want to help* — `advcl:relcl(John, want)`); both are in [25-relative-clauses.md](25-relative-clauses.md) §§7–8.

The common school-text formula *That is why …* is an `advcl:relcl` on the demonstrative:

(25) *That is why you always wait behind the yellow line.* — `advcl:relcl(That, wait)` (`$MOVA_DATA/parsedb/current.conllu`, school:technology:021-trains-and-the-underground.lesson:5).
(26) *That is why Egypt was called the gift of the Nile.* — `advcl:relcl(That, called)` (school:history:005-ancient-egypt-the-nile-and-the-pharaohs.lesson:3).

EWT 2.18 analyses the same formula alike: *AND THAT IS WHY ALL AMERICANS SHOULD OPPOSE THE NOMINATION OF SAMUEL ALITO.* — `advcl:relcl(THAT, OPPOSE)` (newsgroup-groups.google.com_civicamerican_22c54c292026bfd2_ENG_20060128_072400-0037). For the pragmatics this is a causal link: the previous sentence is the reason for the fact after *why* ([36-discourse-relations.md](36-discourse-relations.md)).

---

## 12. Comparative clauses

Comparative clauses (*than she dances*, *as tall as his father is*) are `advcl` of the comparative word, with *than* / *as* as `mark`, and they are often elliptical (*taller than me*: `obl` with `case(than)`). The UD English overview treats comparatives under "dependency-introducing adverbs": *more* / *-er* / *as* introduce the dependency that the *than*/*as* clause fills (`_en/specific-syntax.md`, "Comparatives", "The more, the merrier"). Comparative deletion and subdeletion (*more cats than there are dogs*) are described by Bacskai-Atkari (2018, *Deletion Phenomena in Comparative Constructions: English Comparatives in a Cross-Linguistic Perspective*, Language Science Press, `books/a/at/atkari-2018-deletion-phenomena-comparative-constructions`).

(27) *Gracee is more excited to see her than she is to see me!!!!* — the second clause has an elided predicate; EWT annotates the remnant with `orphan` and adds an empty node `10.1` in the enhanced graph (EWT 2.18 reviews-071278-0003; ellipsis in [29-ellipsis-word-order-information-structure.md](29-ellipsis-word-order-information-structure.md)).

| construction | example | UD |
|---|---|---|
| clausal comparative | *She sings better than she dances* | `advcl(better, dances)`, `mark(dances, than)` |
| phrasal comparative | *taller than me* | `obl(taller, me)`, `case(me, than)` |
| equative | *as tall as his father* | `advmod(tall, as)`, `obl/advcl` with `case/mark(as)` |
| comparative correlative | *The more, the merrier* | special analysis in `_en/specific-syntax.md` |

---

## 13. Mova's parser on coordination and subordination

Fourteen test sentences (Mova en 0.4, CPU, 2026-10-17):

| sentence | Mova en 0.4 | UD / EWT | verdict |
|---|---|---|---|
| *The store buys and sells cameras.* | `conj(buys, sells)`, `cc(sells, and)`, `obj(sells, cameras)` | same | right |
| *Both the cat and the dog are here.* | `cc:preconj(cat, Both)`, `conj(cat, dog)` | same | right |
| *Either you leave or I call the police.* | `cc:preconj(leave, Either)`, `conj(leave, call)` | same | right |
| *He is rich but unhappy.* | `conj(rich, unhappy)`, `cc(unhappy, but)` | same | right |
| *If it rains, we will stay home.* | `advcl(stay, rains)`, `mark(rains, If)` | same | right |
| *He left early so that he could catch the train.* | `mark(catch, so)`, `fixed(so, that)`, `advcl(left, catch)` | same | right |
| *He ran away because he was afraid.* | `advcl(ran, afraid)`, `mark(afraid, because)` | same | right |
| *I came, I saw, I conquered.* | `conj(came, saw)`, **`ccomp(saw, conquered)`** | `conj(came, conquered)` (or `parataxis`) | wrong |
| *My friend, a doctor, helped me.* | **`conj(friend, doctor)`** | `appos(friend, doctor)` | wrong |
| *Although she was tired, she kept working.* | root **tired**, **`conj(tired, kept)`** | root *kept*, `advcl(kept, tired)` | wrong: the concessive clause made the root |
| *The work done, they went home.* | **`acl(work, done)`**, `nsubj(went, work)` | `advcl(went, done)`, `nsubj:pass(done, work)` | wrong: absolute read as reduced relative |
| *She sings better than she dances.* | **`xcomp(sings, better)`** (ADJ), `advcl(better, dances)` | `advmod(sings, better)` (ADV), `advcl(better, dances)` | half right |
| *I tried to explain myself, which was a bad idea.* | **`obl(explain, idea)`** | `advcl:relcl(tried, idea)` | wrong |
| *John likes tea and Mary coffee.* | **`compound(coffee, Mary)`**, `conj(tea, coffee)` | `conj(likes, Mary)`, `orphan(Mary, coffee)` | wrong: gapping (see [29](29-ellipsis-word-order-information-structure.md)) |

The parser handles canonical coordination and preposed or postposed finite adverbial clauses well. It fails where the construction is rare in its training data (absolutes, sentential relatives, gapping) and where a preposed subordinate clause is long enough to look like a main clause (*Although …*). The last kind of error matters most for the frames: the main event becomes a `conj` of a state, so the concession turns into a sequence.

---

## 14. From conjunctions to frame links

Mova's pragmatics reads clause links into two fields of every frame, `cause` and `next`, and a flag `cond` (docs/pragmatics.md, "Links"; [`prag/src/frames.rs`](../../prag/src/frames.rs)). The rules, as implemented on 2026-10-17:

| in the tree | link | direction |
|---|---|---|
| `mark` *because / since / 'cos / cause / cuz*; *as* on an `advcl` | `cause` | the main frame's cause = the subordinate frame |
| *so / therefore / thus* (`mark` or `advmod`), not *so that* | `cause` + `next` | the nearest preceding predicate is the cause of this one; and it comes before it |
| *so / such … that* (a result clause) | `cause` | the result's cause = the clause with *so / such* |
| `mark` *if / unless* | `cond` | the subordinate frame is a condition (printed `if`) |
| purpose *to*: `advcl`, `acl`, or `xcomp` of a non-control verb | `cause` | the main frame's cause (its purpose) = the infinitive's frame |
| `advcl` with *by* + *-ing* | `cause` | the means is why it works: *re-zip them by preening* |
| `mark` *after / once / when / whenever* | `next` | the subordinate event comes first, then the main one |
| `mark` *before / until / till* | `next` | the main event comes first, then the subordinate one |
| `ccomp` / `csubj` of a verb of saying or knowing | `quote` | the subordinate frame is what was said or thought |
| `conj` / `parataxis`, or *then* | `next` | the previous conjunct comes first (the maxim of manner) |
| consecutive sentences | `next` | in reading order |

`next` links are kept only between actions and speech, never from or to a state, a general fact or a condition. The ordering of *and*-coordinated events is Grice's sub-maxim "be orderly" (Grice 1975, "Logic and Conversation", in *Syntax and Semantics* 3; not open, wanted); [`prag/src/infer.rs`](../../prag/src/infer.rs) states the same inference as an implicature with its exceptions (no ordering with stative verbs, or when *after, before, while, meanwhile, simultaneously* or a non-final *then* is present), following the maxim axioms of Tatu and Moldovan (2012), as cited in the module header.

Frames for a short text (Mova `world frames`, en 0.4, CPU, 2026-10-17):

```
The fox flattered the crow because she wanted the cheese. If the crow sings, the cheese falls.
Alice opened the door and walked in. ... They were so hungry that they ate the rats.

#0 Event fox flatter patient=crow | cause→#1 next→#3
#1 Event fox(«she») want patient=cheese
#2 Say(express) crow sing | if
#3 Event cheese fall | next→#4
#4 Event alice open patient=door | next→#5
#5 Event alice walk in
#12 Fact(is) they be attrs=hungry
#13 Event they eat patient=rat | cause→#12
```

- *because* gives `cause→#1`: the flattery's cause is the wanting (right).
- *If the crow sings* gives a conditional frame `if`, and the `next` chain skips it (#0 → #3), as the rule says. *sing* is read as a Say frame because VerbNet puts *sing* in the manner-of-speaking class 37.3 — a lexical error for this text.
- *and* gives `next→#5` with the subject propagated to *walk* (right).
- *so … that* gives `cause→#12` from eating to hunger (right).

The frame bench measures these links against teacher-written gold: on 2026-10-10 `next` scored F1 .23 and `cause` was among the stronger links (docs/pragmatics.md, "Not yet": the gold's sense of "happened right after" is narrow).

### Not yet

- **Concession and contrast**: *although / though / but / yet / however* produce no link; the frame of the concessive clause is neither a cause nor a sequence, and there is no field for "despite". A reader that answers *why* questions may wrongly follow a *but*-coordination as `next`.
- **Senses of polysemous subordinators**: *since* is always causal and *as* on `advcl` always causal, though both are often temporal (*since 2004*, *as the night was falling*); *while* (time or contrast) and *once* (time or condition) are not distinguished.
- **Epistemic and speech-act *because***: not distinguished from causal *because*.
- **Enhanced dependencies** (`conj:and`, `advcl:if`, propagated subjects and objects): not produced; the reader propagates subjects itself and does not propagate shared objects (*buys and sells cameras*: the *buy* frame has no patient).
- **Distributive versus collective** readings of coordinated subjects: not represented.
- **Parser errors of §13**: absolutes, preposed concessives, sentential relatives, gapping, apposition.

---

## In Mova

- Conventions and checks: [`en/seeds/errors/coordination-structure.md`](../../en/seeds/errors/coordination-structure.md), [`en/seeds/verbal/subordinators-mark.md`](../../en/seeds/verbal/subordinators-mark.md), [`wh-adverb-clause.md`](../../en/seeds/verbal/wh-adverb-clause.md), [`purpose-clause.md`](../../en/seeds/verbal/purpose-clause.md), [`absolute-construction.md`](../../en/seeds/verbal/absolute-construction.md), [`infinitive-coordination.md`](../../en/seeds/verbal/infinitive-coordination.md), [`agreement-compound-subject.md`](../../en/seeds/verbal/agreement-compound-subject.md), [`en/seeds/errors/coordinated-subject-agreement.md`](../../en/seeds/errors/coordinated-subject-agreement.md), [`en/seeds/nominal/appos.md`](../../en/seeds/nominal/appos.md), [`en/seeds/errors/gapping-orphan.md`](../../en/seeds/errors/gapping-orphan.md).
- Dialect: [`dialects/mova/seeds.md#discourse-adverbs`](../../dialects/mova/seeds.md#discourse-adverbs) (*so*, *though* as `advmod`), [`depictives.md`](../../dialects/mova/seeds.md#depictives) (depictive `advcl`), [`spoken-subtypes.md`](../../dialects/mova/seeds.md#spoken-subtypes) (`conj:reform` waits).
- Frames and links: [`prag/src/frames.rs`](../../prag/src/frames.rs) (`cause`, `next`, `cond`, `quote`; switches `so-nearest`, `so-that`, `purpose-wide`, `purpose-acl`, `by-means`, measurable with `PRAG_OFF`), [`prag/src/infer.rs`](../../prag/src/infer.rs) (the "be orderly" implicature of *and*), [`prag/src/fbench.rs`](../../prag/src/fbench.rs) (cause and next F1).
- Students that follow the links: `world knowledge` (the pragmatic student answers *why* by `cause` and *what happened next* by `next`, [`world/src/knowledge.rs`](../../world/src/knowledge.rs)).
- Not yet: see §14.

## Sources

- Bacskai-Atkari, Julia. 2018. *Deletion Phenomena in Comparative Constructions: English Comparatives in a Cross-Linguistic Perspective*. Language Science Press. `books/a/at/atkari-2018-deletion-phenomena-comparative-constructions`.
- Brown, Goold. 1851. *The Grammar of English Grammars*. `books/b/br/brown-1851-grammar-english-grammars`.
- Curme, George O. 1931. *A Grammar of the English Language*, vol. III: *Syntax* (ch. IX, XVI). `books/c/cu/curme-1931-grammar-english-language-vol`.
- de Marneffe, Marie-Catherine, et al. 2021. "Universal Dependencies." *Computational Linguistics* 47(2). https://aclanthology.org/2021.cl-2.11.
- Gerdes, Kim, Bruno Guillaume, Sylvain Kahane, Guy Perrier. 2018. "SUD or Surface-Syntactic Universal Dependencies." UDW 2018. https://aclanthology.org/W18-6008.
- Grünewald, Stefan, et al. 2021. On enhanced UD coordination in EWT (as cited in `en/seeds/errors/coordination-structure.md`). https://aclanthology.org/2021.eacl-main.67.
- Jespersen, Otto. *A Modern English Grammar on Historical Principles*, Part V. `books/j/je/jespersen-1914-modern-english-grammar-historical`, text-1.
- Osborne, Timothy John, and Chenchen Song. 2025. "Head-initial and Head-Final coordinate structures in two annotation schemes of dependency grammar." Depling 2025. https://aclanthology.org/2025.depling-1.6; `papers/o/os/osborne-2025-head-initial-head-final-coordinate`.
- Przepiórkowski, Adam, and Michał Woźniak. 2023. "Conjunct Lengths in English, Dependency Length Minimization, and Dependency Structure of Coordination." ACL 2023. https://aclanthology.org/2023.acl-long.864.
- Reed, Alonzo, and Brainerd Kellogg. 1877. *Higher Lessons in English*. `books/r/re/reed-1877-higher-lessons-english`.
- Schuster, Sebastian, and Christopher D. Manning. 2016. "Enhanced English Universal Dependencies." LREC 2016. https://aclanthology.org/L16-1376.
- Universal Dependencies guidelines, snapshot 2026-09-24: `_en/dep/conj.md`, `cc.md`, `cc-preconj.md`, `parataxis.md`, `appos.md`, `advcl.md`, `advcl-relcl.md`, `mark.md`, `acl-relcl.md`; `_u/dep/fixed.md`; `_en/specific-syntax.md` (coordination, comparatives); `_u-overview/enhanced-syntax.md`. https://universaldependencies.org/en/.
- UD English EWT 2.18 (Silveira et al. 2014, https://aclanthology.org/L14-1067; CC BY-SA 4.0), examples by `sent_id`.
- Research library digests: `docs/papers/ud/ud-core.md`, `docs/papers/ud/english-banks.md`.
- Mova: `docs/pragmatics.md` (links, the frame bench), `docs/dialect.md`; `$MOVA_DATA/parsedb/current.conllu`; `$MOVA_DATA/corpora/judaism/parsedb/current.conllu`.
- Wanted (not open): Grice 1975, "Logic and Conversation"; Sweetser 1990, *From Etymology to Semantics*; Huddleston and Pullum 2002, *CGEL*.
