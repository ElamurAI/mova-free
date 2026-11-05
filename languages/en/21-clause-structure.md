# 21. The simple clause

A clause is a predicate with its dependents. This chapter describes the English simple clause in UD terms: what can be a predicate, the clause patterns (valency), the verb chain of auxiliaries, the copula and existential *there*, subjects, objects and obliques, adverbials, the clause types (declarative, interrogative, imperative, exclamative), peripheral elements such as vocatives and interjections, and secondary predicates. Complex sentences — complements, coordination, subordination, relatives — follow in chapters 23–25.

Contents

1. [The predicate is the head](#1-the-predicate-is-the-head)
2. [Clause patterns](#2-clause-patterns)
3. [The verb chain](#3-the-verb-chain)
4. [The copula and non-verbal predicates](#4-the-copula-and-non-verbal-predicates)
5. [Subjects](#5-subjects)
6. [Objects](#6-objects)
7. [Obliques and adverbials](#7-obliques-and-adverbials)
8. [Clause types](#8-clause-types)
9. [Peripheral elements](#9-peripheral-elements)
10. [Secondary predicates](#10-secondary-predicates)
11. [Finite and non-finite clauses at a glance](#11-finite-and-non-finite-clauses-at-a-glance)
12. [How Mova reads a clause](#12-how-mova-reads-a-clause)

---

## 1. The predicate is the head

In UD the head of a clause is its **predicate**: the lexical verb, or, when the verb is the copula *be*, the predicative word (UD guidelines `_u/overview/simple-syntax.md`, https://universaldependencies.org/u/overview/simple-syntax.html). Auxiliaries, the copula, negation and subordinators all depend on that head. The predicate of the main clause is the `root`.

| predicate type | example | head (root) | function words |
|---|---|---|---|
| verbal | *The fox **flattered** the crow.* | *flattered* | — |
| verbal with auxiliaries | *The fox will have been **watching** the crow.* | *watching* | `aux` ×3 |
| adjectival | *Alice was very **tired**.* | *tired* | `cop` *was* |
| nominal | *Mole is a **friend**.* | *friend* | `cop`, `det` |
| prepositional | *The Rabbit is in the **hall**.* | *hall* | `cop`, `case` |
| adverbial | *The meeting is **tomorrow**.* | *tomorrow* | `cop` |
| existential | *There **is** a mouse in the pool.* | *is* (VERB) | `expl` *There* |
| clausal | *The problem is that the door is **locked**.* | *locked* | `cop`, `mark`; `nsubj:outer` *problem* |

The prepositional and nominal predicates show the strongest UD commitment: in *The Rabbit is in the hall* the noun *hall* is the root and *is* and *in* both hang from it. Mova's parser gets this right (1).

(1) «Mova en 0.4, CPU, 2026-10-17» (ID FORM UPOS HEAD DEPREL)

```conllu
1  The     DET    2  det
2  Rabbit  NOUN   6  nsubj
3  is      AUX    6  cop
4  in      ADP    6  case
5  the     DET    6  det
6  hall    NOUN   0  root
7  .       PUNCT  6  punct
```

The traditional grammars describe the same split as complete vs incomplete predication: a "verb of incomplete predication" (Curme 1931, vol. III, ch. III, §6 B, `books/c/cu/curme-1931-grammar-english-language-vol`) needs a predicate complement; UD moves the head to that complement when the verb is *be*, and keeps it on the verb for the other linking verbs (§4.3).

## 2. Clause patterns

### 2.1 The patterns

English clauses fall into a small number of patterns by the complements the verb takes. Onions (1904/1911, *An Advanced English Syntax*, `books/o/on/onions-1911-advanced-english-syntax-based`, card only in the library for licence reasons) gave five "forms of the predicate"; later reference grammars add an obligatory adverbial, giving seven (Quirk et al. 1985, not open — see *Wanted* in the sources file). The UD relations map onto them directly:

| pattern | example | UD relations on the verb | VerbNet-style frame |
|---|---|---|---|
| S V | *The cheese fell.* | `nsubj` | `NP V` |
| S V O | *The fox flattered the crow.* | `nsubj`, `obj` | `NP V NP` |
| S V O O | *The voice tells you who someone is.* | `nsubj`, `iobj`, `obj`/`ccomp` | `NP V NP NP` |
| S V C | *Alice was tired.* / *Alice grew tired.* | `nsubj` + `cop` on the predicate / `nsubj`, `xcomp` | `NP V ADJ` |
| S V O C | *We elected her president.* | `nsubj`, `obj`, `xcomp` | `NP V NP NP-predicate` |
| S V A | *The Rabbit lives in the hall.* | `nsubj`, `obl` | `NP V PP.location` |
| S V O A | *She put the cup on the table.* | `nsubj`, `obj`, `obl` | `NP V NP PP.destination` |

(2) S V O O — `school:english:021-authors-choices-why-this-word-why-this-way.lesson:8` (ID FORM UPOS HEAD DEPREL)

```conllu
1  The      DET    2  det
2  voice    NOUN   3  nsubj
3  tells    VERB   0  root
4  you      PRON   3  iobj
5  who      PRON   3  ccomp
6  someone  PRON   5  nsubj
7  is       AUX    5  cop
```

Here the second object is a clause (*who someone is*), so it is `ccomp`, not `obj`; the wh-word *who* is its predicate (an embedded copular question with *who* as the predicate nominal; see 26).

### 2.2 Arguments and adjuncts

UD 2 does not mark the argument/adjunct boundary for prepositional phrases: *lives **in the hall*** (an argument of *live*) and *slept **in the hall*** (an adjunct) are both `obl`. The universal guidelines call the distinction "subtle, unclear, and frequently argued over" and eliminate it, keeping the core–oblique distinction that typology finds easier to apply, as the Penn Treebank annotators had done (UD `_u-overview/syntax.md`, *Avoiding an argument/adjunct distinction*, https://universaldependencies.org/u/overview/syntax.html; some languages mark selected obliques with the optional subtype `obl:arg`, `_u-dep/obl-arg.md`). Core arguments are only the bare nominals and the clausal complements: `nsubj`, `obj`, `iobj`, `csubj`, `ccomp`, `xcomp`.

Mova recovers the distinction later, in the pragmatics reader: each verb is matched against its VerbNet frames, and a PP that a frame names (a Destination, Location, Recipient…) becomes a role; an unmatched PP is a cheap adjunct ([../../docs/pragmatics.md](../../docs/pragmatics.md), *The linguists' rules: VerbNet and WordNet*; [../../prag/src/vn.rs](../../prag/src/vn.rs)). VerbNet: Kipper Schuler 2005 and later versions (VerbNet 3.0 License), the class lexicon compiled into `prag/data/verbnet.tsv`.

### 2.3 Patterns in the data

Counts of clause patterns on main-clause verbs are not in the treebank, but the relation counts of 20 §5.2 give the proportions: in EWT 2.18 there are 12 254 `obj`, 795 `iobj` and 3 831 `xcomp` against 19 888 `nsubj` and 16 622 roots. On the four school courses used by the frame reader, VerbNet covers 96.1% of verb tokens (2026-10-10, [../../docs/pragmatics.md](../../docs/pragmatics.md)).

## 3. The verb chain

### 3.1 Order and analysis

English auxiliaries come in a fixed order, each choosing the form of the next verb:

| slot | auxiliary | form it requires | feature added (UD registry description) |
|---|---|---|---|
| 1 | modal (*will, can, must…*) | bare infinitive | modality / future |
| 2 | perfect *have* | past participle | perfect aspect |
| 3 | progressive *be* | *-ing* participle | progressive aspect |
| 4 | passive *be* (or *get*) | past participle | passive voice |
| — | lexical verb | — | — |

In UD every auxiliary is a dependent of the lexical verb (`aux`, or `aux:pass` for slot 4); none of them takes dependents of its own except in ellipsis (UD `_en/dep/aux_.md`; Mova seed [../../en/seeds/verbal/aux-flat-leaf.md](../../en/seeds/verbal/aux-flat-leaf.md)). The functions are described per lemma in the UD auxiliary registry: *can/could/may/might/dare* possibility, *must/need/ought/should* necessity, *would* conditional, *will* future, *have* perfect, *be* progressive, passive and copula (registry summary in the research library's `docs/papers/ud/ud-core.md` §6.1). They are not written into FEATS: English modals carry only `VerbForm=Fin` (Mova seed [../../dialects/mova/seeds.md#modal-mood](../../dialects/mova/seeds.md#modal-mood)).

(3) A four-verb chain, «Mova en 0.4, CPU, 2026-10-17» — correct:

```conllu
1  The       DET    2  det
2  fox       NOUN   6  nsubj
3  will      AUX    6  aux
4  have      AUX    6  aux
5  been      AUX    6  aux
6  watching  VERB   0  root
7  the       DET    8  det
8  crow      NOUN   6  obj
9  for       ADP   10  case
10 hours     NOUN   6  obl
```

The flat analysis loses the nesting of tense and aspect that a phrase-structure analysis shows (*will* scopes over *have been watching*). UD keeps the order of the `aux` dependents, so the nesting can be read off the word order; the critics of UD's function-word analysis take this as one of their arguments (Osborne & Gerdes 2019, https://doi.org/10.5334/gjgl.537).

### 3.2 The auxiliary list and the operator

The English AUX class is closed: *be, have, do*, passive *get*, and the modals *can, could, may, might, shall, should, will, would, must, ought, need, dare* (UD `_en/pos/AUX_.md`; seed [../../en/seeds/verbal/aux-closed-list.md](../../en/seeds/verbal/aux-closed-list.md)). *Have to, be going to, used to* are lexical verbs with an infinitive (`xcomp`) (seed [../../en/seeds/verbal/semi-modals.md](../../en/seeds/verbal/semi-modals.md)); *need* and *dare* are auxiliaries only in their modal use (*need not go*) (seed [../../en/seeds/verbal/need-dare.md](../../en/seeds/verbal/need-dare.md)).

The first auxiliary is the **operator**: it is the word that inverts in questions, takes *not*, stands alone in ellipsis and carries emphasis. If there is no auxiliary, *do* is inserted (do-support):

| operation | with an auxiliary | without one: *do*-support |
|---|---|---|
| negation | *He could not name the others.* (EWT 2.18 `weblog-juancole.com_juancole_20040708181175_ENG_20040708_181175-0013`) | *She did not go.* |
| question | *Will Posada be given up?* | *Did the Queen really say that?* |
| ellipsis | *Mary can't leave, but John can.* | *Mary didn't leave, John did.* |
| emphasis | *She CAN swim.* | *She does love her cat.* |

The *do*-periphrasis was rare in Old English, spread from the fourteenth century, was used freely alongside the simple form between about 1500 and 1700, and settled into the present distribution around 1750, leaving fossils such as *I know not*, *if I mistake not* (Curme 1931, vol. III, ch. III, *Uses of do-forms*, `books/c/cu/curme-1931-grammar-english-language-vol`). In UD, *do* is `aux` like any other auxiliary (seed [../../en/seeds/verbal/do-support.md](../../en/seeds/verbal/do-support.md)); in the imperative *Don't go*, both *do* and *go* carry `Mood=Imp` in the Mova dialect ([../../dialects/mova/seeds.md#imperative-do](../../dialects/mova/seeds.md#imperative-do)).

(4) Emphatic and interrogative *do*, «Mova en 0.4, CPU, 2026-10-17» — both correct:

```text
She does love her cat.          aux(love, does)   nsubj(love, She)   obj(love, cat)
Did the Queen really say that?  aux(say, Did)     nsubj(say, Queen)  advmod(say, really)  obj(say, that)
```

But when a wh-word is fronted, the same parser takes *did* for the main verb — see 26 and 20 §10.2.

### 3.3 Contractions

Contracted auxiliaries and negation are separate syntactic words in a multiword token: *Here's* = *Here* + *'s*; *don't* = *do* + *n't*; *he'd* = *he* + *'d* (lemma *have* or *would* by context). The lemma of *'s* is *be* or *have* (EWT 2.18 `email-enronsent38_01-0070`, 20 (2)). Tokenisation is described in the morphology chapters; the syntax is the same as for the full forms.

## 4. The copula and non-verbal predicates

### 4.1 Only *be* is a copula

In UD English the copula is only *be*; it is `cop` and AUX (UD `_en/dep/cop.md`; seeds [../../en/seeds/verbal/copula-be-only.md](../../en/seeds/verbal/copula-be-only.md), [../../en/seeds/verbal/copula-predicate-head.md](../../en/seeds/verbal/copula-predicate-head.md)). In the parse database `cop` has 155 018 tokens (1.92% of words; 2026-10-17).

(5) *What a pig I have been!* — `school:books:042-willows-ch5-part2-mole-end.lesson:20`, the agreed silver tree:

```conllu
1  What  DET    3  det
2  a     DET    3  det
3  pig   NOUN   0  root
4  I     PRON   6  nsubj
5  have  AUX    3  aux
6  been  AUX    3  cop
7  !     PUNCT  3  punct
```

The predicate nominal *pig* is the root and *been* is `cop` — right. But *I* hangs from the copula (`nsubj(been, I)`), which breaks the leaf rule for function words; UD wants `nsubj(pig, I)` ([../../en/seeds/errors/function-words-leaves.md](../../en/seeds/errors/function-words-leaves.md)). Exclamatives with fronted predicates are rare in training data, and the error survived the agreement rules.

### 4.2 Equatives and specificational clauses

When both sides of *be* are noun phrases (*The winner is John* / *John is the winner*), UD English takes the **first** one as the subject and the second as the predicate head (UD `_en/specific-syntax.md`, copula section; seed [../../en/seeds/verbal/copula-equative-order.md](../../en/seeds/verbal/copula-equative-order.md)). This is a convention, not a semantic analysis: the specificational reading (*the winner* = a role, *John* = its value) is left to later layers.

### 4.3 Other linking verbs

*Become, seem, appear, look, feel, sound, remain, stay, get* (in *get rich*), *grow, turn* are lexical verbs: the predicative is their `xcomp` (UD `_en/dep/xcomp.md`; seed [../../en/seeds/lexicon/verb-copula-like.md](../../en/seeds/lexicon/verb-copula-like.md)).

(6) *Running away seemed a good idea.* — UD: `csubj(seemed, Running)`, `xcomp(seemed, idea)`. «Mova en 0.4, CPU, 2026-10-17» gets the clausal subject right but makes *idea* the `obj` of *seemed* — a linking verb with a noun predicate is a frequent confusion.

### 4.4 Existential *there*

In existentials *there* is `expl` and the noun phrase after *be* (the pivot) is `nsubj`; *be* is the root and a VERB, not a copula, because there is no predicate for it to link to (UD `_en/dep/expl.md`; seeds [../../en/seeds/verbal/existential-there.md](../../en/seeds/verbal/existential-there.md), [../../en/seeds/nominal/there-expletive.md](../../en/seeds/nominal/there-expletive.md)).

(7) EWT 2.18 `email-enronsent00_01-0045` *oh god is there an agenda.*

```conllu
1  oh      INTJ   2  discourse
2  god     PROPN  3  discourse
3  is      VERB   0  root
4  there   PRON   3  expl
5  an      DET    6  det
6  agenda  NOUN   3  nsubj
```

Verb agreement follows the pivot in careful usage (*there **are** two mice*) and is often singular in speech (*there's two mice*) (seed [../../en/seeds/verbal/agreement-notional.md](../../en/seeds/verbal/agreement-notional.md)).

Under *let*, the existential needs care. In the JPS 1917 Torah trees the two creation commands are analysed differently:

(8) `jps:GEN:2` *And God said: 'Let there be light.' And there was light.* — `expl(Let, there)`, `ccomp(Let, be)`, `nsubj(be, light)`; then `expl(was, there)`, `nsubj(was, light)`, `conj(said, was)`.

(9) `jps:GEN:15` *…'Let there be lights in the firmament…* — `expl(be, there)`, `xcomp(Let, be)`, `nsubj(be, lights)`.

(9) is the analysis the guidelines support (*let* takes an open complement whose expletive subject is raised; the expletive belongs to *be*); (8) attaches the expletive to *let* and calls the complement `ccomp`. The inconsistency is a parser artefact, not a difference between the verses.

### 4.5 Clausal predicates and the outer subject

When the predicate after *be* is itself a clause, the clause's head is the head of the whole, and the outer subject is `nsubj:outer` (or `csubj:outer`) — the UD 2.10 analysis, adopted by Mova ([../../dialects/mova/seeds.md#clausal-predicate](../../dialects/mova/seeds.md#clausal-predicate); UD `_en/dep/nsubj-outer.md`).

(10) EWT 2.18 `email-enronsent02_01-0023` *Middle Office work being prioritised is as follows*

```conllu
3  work         NOUN  8  nsubj:outer
5  prioritised  VERB  3  acl
6  is           AUX   8  cop
7  as           SCONJ 8  mark
8  follows      VERB  0  root
```

(11) *The problem is that the door is locked.* — UD: `nsubj:outer(locked, problem)`, `cop(locked, is-3)`, `mark(locked, that)`, `nsubj:pass(locked, door)`, `aux:pass(locked, is-7)`. «Mova en 0.4, CPU, 2026-10-17» labels *problem* `nsubj:pass` as well, giving *locked* two passive subjects; the gate `en.verbal.outer-subject-needs-cop` ([../../en/seeds/verbal/outer-subject.md](../../en/seeds/verbal/outer-subject.md)) and the single-subject seed ([../../en/seeds/verbal/single-subject.md](../../en/seeds/verbal/single-subject.md)) flag such trees. The parse database has only 35 `nsubj:outer` in 325 765 sentences (2026-10-17), against 254 in EWT's 16 622.

## 5. Subjects

### 5.1 Kinds of subject

| relation | what | example |
|---|---|---|
| `nsubj` | nominal subject of an active clause or of a non-verbal predicate | ***Alice** was tired* |
| `nsubj:pass` | subject of a passive | ***Posada** be given up* |
| `nsubj:outer` | outer subject of a clausal predicate | §4.5 |
| `csubj` | clausal subject | ***That the Queen was angry** surprised nobody* |
| `csubj:pass` | clausal subject of a passive | ***That he lied** was proven* |
| `csubj:outer` | outer clausal subject | ***What he said** is that…* |
| `expl` | dummy *it* or *there* in subject position | ***It** rains*; ***There** is a mouse* |

A predicate has at most one subject of each kind (seed [../../en/seeds/verbal/single-subject.md](../../en/seeds/verbal/single-subject.md)); a finite verb normally has one (seed [../../en/seeds/verbal/finite-verb-subject.md](../../en/seeds/verbal/finite-verb-subject.md)), with imperatives and coordinated verbs as the main exceptions.

### 5.2 Clausal subjects

Clausal subjects are rare in the canonical position: EWT 2.18 has 336 `csubj` against 19 888 `nsubj` (20 §5.2). The usual alternative is extraposition with anticipatory *it* (§5.3; 29).

(12) EWT 2.18 `email-enronsent11_01-0073` *Must be nice having money!!* — `csubj(nice, having)` with the subject clause after the predicate and no *it*: a colloquial omission of the expletive.

(13) *That the Queen was angry surprised nobody.* — UD: `csubj(surprised, angry)`, `mark(angry, That)`, `nsubj(angry, Queen)`, `cop(angry, was)`, `obj(surprised, nobody)`. «Mova en 0.4, CPU, 2026-10-17» reads *That* as a pronoun subject, *surprised* as an adjective root with *was* as its copula and *angry* as its `amod`, and *nobody* as `obl`: a garden path that the subordinator *That* at the start of a sentence triggers. Seed [../../en/seeds/errors/clausal-subject-csubj.md](../../en/seeds/errors/clausal-subject-csubj.md) describes the expected analysis.

### 5.3 Expletive *it*

English distinguishes several non-referential or weakly referential *it* (Curme 1931, vol. III, §4 II B–C, *Situation and impersonal it*, *Anticipatory it and there*):

| kind | example | UD |
|---|---|---|
| weather / ambient | *It is raining.* | `nsubj` in EWT (*Typically it's cold*, EWT 2.18 `answers-20111108101906AA2JJqq_ans-0002`: `nsubj(cold, it)`); the English `expl` page covers only existential *there* and extraposition |
| situation | *It's my turn.* | `nsubj` (referential to the situation) |
| anticipatory (extraposition) | *It is easy to please John.* / *It is necessary that you exert yourself.* | `expl` + `csubj` (or `ccomp` with passives of saying) |
| cleft | *It was Alice who laughed.* | see 29 |
| object expletive | *I find it odd that…* | `expl` |

(14) *It is easy to please John.* — EWT's analysis is `expl(easy, It)`, `csubj(easy, please)`. «Mova en 0.4, CPU, 2026-10-17» gives `expl` + `xcomp(easy, please)`, mixing the extraposition analysis with the tough-construction one (*John is easy to please*, where the infinitive is `xcomp`; seed [../../en/seeds/errors/tough-vs-extraposition.md](../../en/seeds/errors/tough-vs-extraposition.md); full discussion in 23 and 29).

Mova's frame reader treats a dummy *it* as unresolved — it does not look for an antecedent and matches nothing (`DUMMY_IT` in [../../prag/src/frames.rs](../../prag/src/frames.rs): *it depends*, *it is unkind to …*, *it means* + a clause; an `it` subject of a copular predicate with a clausal or *to*-infinitive dependent). See 35 for reference.

### 5.4 Omitted subjects

A subject may be missing in imperatives (*Sit down*), in coordinated clauses (*own and operate*, 20 (3)), in non-finite clauses (controlled; 23), and in informal writing and diaries (*Must be nice having money*; *Went to the shops*). The last, "diary drop", is the English relative of the topic drop that Schäfer (2025, *The licensing and usage of topic drop in German*, `books/s/sc/schafer-2025-licensing-usage-topic-drop`) studies in German — an omitted pronoun recoverable from the discourse — and Curme already lists *omission of the subject* as a living colloquial pattern (Curme 1931, vol. III, ch. II, *Omission of the subject*). UD has no empty subject node in the basic tree; EUD propagates subjects only in coordination and control (20 §8).

### 5.5 Agreement

The finite verb agrees with its subject in number and person; UD writes Number and Person on finite verbs from the subject (EWT practice; Mova normalisation `en::ud::agreement`, [../../docs/dialect.md](../../docs/dialect.md)), but the form wins where it is unambiguous (*the team **are*** stays plural; [../../dialects/mova/seeds.md#features-context](../../dialects/mova/seeds.md#features-context)). The agreement rules and their exceptions — coordinated subjects, notional agreement with collectives, *each/everyone*, clausal subjects — are seeds: [agreement-third-singular](../../en/seeds/verbal/agreement-third-singular.md), [agreement-compound-subject](../../en/seeds/verbal/agreement-compound-subject.md), [agreement-notional](../../en/seeds/verbal/agreement-notional.md), [agreement-indefinite-pronoun](../../en/seeds/verbal/agreement-indefinite-pronoun.md), [agreement-clausal-subject](../../en/seeds/verbal/agreement-clausal-subject.md), [agreement-non-third](../../en/seeds/verbal/agreement-non-third.md). Curme 1931 vol. III ch. IV is the classical source for the exceptions.

## 6. Objects

### 6.1 Direct and indirect objects

The direct object is `obj`; the first object of a double-object verb is `iobj` (UD `_en/dep/obj.md`, `_en/dep/iobj.md`; seeds [../../en/seeds/verbal/core-objects.md](../../en/seeds/verbal/core-objects.md), [../../en/seeds/verbal/indirect-object.md](../../en/seeds/verbal/indirect-object.md)).

| construction | example | UD |
|---|---|---|
| monotransitive | *flattered the crow* | `obj` |
| ditransitive | *gave the Mole a cake* | `iobj(gave, Mole)`, `obj(gave, cake)` |
| prepositional dative | *gave a cake to the Mole* | `obj`, `obl(gave, Mole)` + `case(Mole, to)` |
| sole indirect object | *remind me*, *ask Bush* | `iobj` alone (Mova dialect) |
| addressee + infinitive | *asked her brother where he'd been*, *told Alice to go* | `iobj` + `ccomp`/`xcomp` |

The *to*-phrase on a verb is `obl` in EWT (1 285 times) and in Mova, not `nmod` as an old English page says ([../../dialects/mova/seeds.md#to-phrase-obl](../../dialects/mova/seeds.md#to-phrase-obl)). The `iobj` alone and with `xcomp` follows the UD 2.12 "sole iobj" amendment; the boundary between *ask/tell/allow/persuade X to* (`iobj`) and *make/let/help X do* (`obj`) is lexical ([../../dialects/mova/seeds.md#iobj-sole](../../dialects/mova/seeds.md#iobj-sole); lexicon [../../en/seeds/lexicon/verb-iobj-licensors.md](../../en/seeds/lexicon/verb-iobj-licensors.md)).

(15) `school:everyday:450-too-many-details-too-few.lesson:0` *Mia asked her brother where he'd been.* — `iobj(asked, brother)`, `ccomp(asked, been)`, `advmod(been, where)`.

Mova's frame reader turns these into roles: an object VerbNet calls a Recipient and that is a person or animal is the **recipient**; the second object of a dative is the **patient** (*gave the Mole a cake*); the person told or asked is the recipient ([../../docs/pragmatics.md](../../docs/pragmatics.md), *Roles applied over the heuristics*).

### 6.2 Particle verbs

A verb particle (*sit **down***, *give **up***, *put **on***) is `compound:prt` of the verb, whatever its tag (ADP or ADV) (UD `_en/dep/compound-prt.md`). The particle may precede or follow a nominal object (*give up the plan* / *give the plan up*) and must follow a pronoun (*give it up*); the order is information-packaging (29).

(16) EWT 2.18 `weblog-blogspot.com_rigorousintuition_20050518101500_ENG_20050518_101500-0022` *But will Posada be given up?* — `compound:prt(given, up)`, `nsubj:pass(given, Posada)`, `aux(given, will)`, `aux:pass(given, be)`.

The frame reader joins the particle to the lemma (`put down` is one verb), with the phrasal verbs taken from the expression base ([../../docs/pragmatics.md](../../docs/pragmatics.md), *Roles, more*). One of the agreed-tree rules moves a misattached `compound` on a particle after its verb to `compound:prt` (rule R4 in [../../tools/treeagree/README.md](../../tools/treeagree/README.md)).

### 6.3 Other kinds of object

- **Cognate and light-verb objects** (*dream a dream*, *take a walk*, *give birth*) are plain `obj`; the light-verb meaning is lexical. In the parse database: *give birth* in `school:religion-ethics:051-life-begins-being-born-and-growing-up.lesson:7` (`obj(give, birth)`).
- **Measure phrases** after verbs (*weighs ten pounds*, *costs five dollars*) are `obj` in EWT when they are the verb's complement; bare-noun adverbials of time and manner are `obl:unmarked` (§7.2).
- **Reflexive objects** (*hide thyself*, `jps:DEU:585`) are `obj` with `Reflex=Yes` on the pronoun (seed [../../en/seeds/nominal/reflexive.md](../../en/seeds/nominal/reflexive.md)).

## 7. Obliques and adverbials

### 7.1 `obl`

A nominal that is not a core argument and attaches to a predicate is `obl`, almost always with a preposition as `case`: *for hours*, *in the hall*, *to the party*. `obl` is the fourth most frequent content relation in the parse database (404 473 tokens, 5.02%) and in EWT (10 817, 4.24%) (20 §5.2). The enhanced graph adds the preposition to the label: `obl:in`, `obl:to` (20 §8.2).

### 7.2 Bare-noun adverbials: `obl:unmarked`

Bare noun phrases used adverbially — mostly times (*last week*, *every day*, *Nov. 28*), also measures and manners (*this way*, *a bit*) — are `obl:unmarked` (UD 2.15 and later; `_en/dep/obl-unmarked.md`).

(17) EWT 2.18 `weblog-juancole.com_juancole_20051126063000_ENG_20051126_063000-0040` *The trial begins again Nov.28.* — `obl:unmarked(begins, 28)`, `nmod:unmarked(28, Nov.)`.

(18) *Mr. Brown, our teacher, met Dr. Jane Smith every day last week.* — «Mova en 0.4, CPU, 2026-10-17»: `obl:unmarked(met, day)`, `obl:unmarked(met, week)` — correct.

### 7.3 Adverbs

Adverbs and adverbial PPs that are not nominal are `advmod`: manner (*quickly*), degree (*very*), time (*again*), sentence adverbs (*really*, *probably*), and negation *not* (`advmod` with `Polarity=Neg`; 27). English adverb position is constrained by class — sentence adverbs initially or before the operator, manner adverbs after the object, not between a verb and its object (*\*She ate quickly the cheese*) (seed [../../en/seeds/errors/adverb-placement.md](../../en/seeds/errors/adverb-placement.md); Curme 1931, vol. III, ch. XVII, word order).

Since the 2.19 change adopted by Mova, adverbs are never `discourse`: *though, so, also, maybe, actually* are `advmod`; only interjections, numbering and emoticons are `discourse` ([../../dialects/mova/seeds.md#discourse-adverbs](../../dialects/mova/seeds.md#discourse-adverbs)).

## 8. Clause types

English marks four clause types by form; their typical uses (speech acts) are the subject of chapter 31.

| type | form | UD signal | prag `form` | count in prag.tsv |
|---|---|---|---|---:|
| declarative | subject before the finite verb | — | `decl` | 257 228 |
| closed interrogative | operator before the subject | `aux` before `nsubj`; `?` | `q` | 4 966 |
| open interrogative | wh-word first, operator before the subject (except subject questions) | `PronType=Int` | `wh` | 6 515 |
| imperative | base form, usually no subject | `Mood=Imp` | `imp` | 14 587 |
| exclamative | *what a* N / *how* ADJ fronted, no inversion | fronted `det`/`advmod` + `!` | `excl` | 2 893 |
| fragment | no finite predicate | root is not a verb or copular predicate | `frag` | 24 505 |

The prag counts are over the 311 101 labelled sentences of `$MOVA_DATA/parsedb/prag.tsv` (2026-10-17); the label set is the closed enum `Form` in [../../prag/src/schema.rs](../../prag/src/schema.rs). The remaining labels are `intj` (184) and `other` (223).

### 8.1 Imperatives

The imperative is the base form with `Mood=Imp|VerbForm=Fin`; the subject is normally absent, sometimes present for contrast or address (*You sit down!*, *Somebody help me!*) (seed [../../en/seeds/verbal/imperative.md](../../en/seeds/verbal/imperative.md)). *Let's* and the *let* + object pattern (*Let him go*, *Let there be light*) are the first-person and third-person periphrases; *let* is a lexical verb, not an auxiliary (seed [../../en/seeds/verbal/aux-closed-list.md](../../en/seeds/verbal/aux-closed-list.md), after Brown 1851, `books/b/br/brown-1851-grammar-english-grammars`).

(19) EWT 2.18 `weblog-typepad.com_ripples_20040407125600_ENG_20040407_125600-0062` *Take care, my friend, Linda* — `root(Take)`, `obj(Take, care)`, `vocative(Take, friend)`, `appos(friend, Linda)`.

(20) *Sit down and listen to me, Alice!* — «Mova en 0.4, CPU, 2026-10-17»: `compound:prt(Sit, down)`, `conj(Sit, listen)`, `obl(listen, me)` correct; *Alice* is `obl(Sit, Alice)` instead of `vocative` — a comma-separated name at the end of an imperative is a vocative in EWT.

Mova's frames give an imperative the agent `you` ([../../docs/pragmatics.md](../../docs/pragmatics.md), *Roles, more*).

Imperatives may be subordinate in meaning: *Do it, you will never regret it* has the force of a condition, *Let him be the greatest villain in the world, I shall never cease…* of a concession (Curme 1931, vol. III, ch. IX, on asyndetic hypotaxis). UD analyses them as `parataxis`; the conditional reading is pragmatic (31, 36).

### 8.2 Interrogatives

Closed (yes/no) questions invert the operator and the subject (*Did the Queen really say that?*); open (wh-) questions front a wh-phrase and invert unless the wh-phrase is the subject (*What did the cook throw?* vs *Who threw the pan?*). Inversion does not change the tree: the operator is still `aux` of the lexical verb, and the fronted wh-word gets its clause-internal relation (`obj`, `obl`, `advmod`…). Full treatment in 26; Mova's seed [../../en/seeds/verbal/questions-inversion.md](../../en/seeds/verbal/questions-inversion.md).

### 8.3 Exclamatives

Exclamatives front *what (a)* + noun or *how* + adjective/adverb, without inversion: *What a pig I have been!* (5); *How small she had grown!* *What* is `det` of the noun, *how* `advmod` of the adjective; the fronted phrase keeps its clause role (predicate, object…). Exclamatives are uncommon: 2 893 of 311 101 labelled sentences (0.9%) in the parse database.

## 9. Peripheral elements

| relation | what | example |
|---|---|---|
| `vocative` | the addressee | *Take care, my **friend*** (19) |
| `discourse` | interjections, fillers, emoticons, numbering | ***oh** god is there an agenda* (7); ***Well**, the Hatter was late again* |
| `dislocated` | a fronted or postposed element doubled by a pronoun | *Your friends, what will **they** say?* |
| `parataxis` | a parenthetical or loosely joined clause | *The Hatter, **I think**, was late* |
| `reparandum` | a self-corrected stretch in speech | *go to the- to the shop* |
| `punct` | punctuation, attached to the head of the phrase it delimits | — |

**Interjection** *well* at the start (*Well, the Hatter was late again*) is `discourse` in both EWT and Mova's parser output (2026-10-17). In EWT 2.18 `discourse` has 1 066 tokens, of them 775 INTJ ([../../dialects/mova/seeds.md#discourse-adverbs](../../dialects/mova/seeds.md#discourse-adverbs)).

**Dislocation** — a subject "springing forth" before the clause and taken up by a pronoun — is old and colloquial: Curme quotes *Your friends, what will they say?* and *The Lord your God, which goes before you, he shall fight for you* (Deut. 1:30) (Curme 1931, vol. III, ch. II). UD marks the fronted phrase `dislocated` (7 tokens in EWT 2.18, 329 in the parse database, which holds more tales and archaic text); see 29.

**Vocatives and discourse markers** carry pragmatic weight — address, politeness, turn-taking — described in 32 and 38.

## 10. Secondary predicates

A secondary predicate says something about a participant beside the main predication.

| kind | example | UD (EWT / Mova dialect) |
|---|---|---|
| object complement (small clause) | *We elected her **president**.* / *I consider him **wise**.* | `obj` + `xcomp` |
| resultative | *She painted the fence **green**.* / *The soldiers painted the roses **red**.* | `obj` + `xcomp` |
| subject depictive | *She entered the room **sad**.* | `advcl` ([../../dialects/mova/seeds.md#depictives](../../dialects/mova/seeds.md#depictives)) |
| absolute | ***The work done**, they left.* | `advcl` (seed [../../en/seeds/verbal/absolute-construction.md](../../en/seeds/verbal/absolute-construction.md)) |
| *with*-absolute | *with the door **open*** | `obl` + `case` on *door*, `acl`/`xcomp` per EWT |

The object-complement and resultative analyses follow UD `_en/dep/xcomp.md` and the Mova seed [../../en/seeds/verbal/objective-complement.md](../../en/seeds/verbal/objective-complement.md). Mova's parser gets some and misses others («Mova en 0.4, CPU, 2026-10-17»):

| sentence | expected | Mova en 0.4 |
|---|---|---|
| *The King of Hearts's soldiers painted the roses red.* | `obj(painted, roses)`, `xcomp(painted, red)` | correct |
| *She painted the fence green.* | `obj(painted, fence)`, `xcomp(painted, green)` | *green* NOUN `obj`, *fence* its `compound` |
| *We elected her president.* | `obj(elected, her)`, `xcomp(elected, president)` | *her* `nmod:poss` of *president*, *president* `obj` |
| *Salt … makes iron rust faster* (`school:chemistry:013-rust-irons-slow-fire.lesson:12`, agreed tree) | `obj(makes, iron)`, `xcomp(makes, rust)` | *rust* NOUN `obj`, *iron* its `compound`, *faster* `xcomp` |

The last row is from the agreed school trees: the witnesses agreed on the wrong noun-compound reading of *iron rust*. The pattern behind all three errors is the same — two nominals after a verb where the second could be a noun — and it is where the absurdity matrix and VerbNet frames have to help (*paint* has a resultative frame `NP V NP ADJ`; *make* has `NP V NP VP`).

## 11. Finite and non-finite clauses at a glance

| clause form | example | typical relations | chapter |
|---|---|---|---|
| finite with *that* | *said **that she left*** | `ccomp`, `csubj`, `acl`, `advcl` | 23, 24 |
| finite with a wh-word | *asked **where he'd been*** | `ccomp`, `acl:relcl` | 23, 25, 26 |
| finite, zero subordinator | *said **she left***; *the book **I read*** | `ccomp`, `acl:relcl` | 23, 25 |
| *to*-infinitive | *wants **to buy*** | `xcomp`, `advcl`, `acl`, `csubj` | 23, 24 |
| bare infinitive | *made it **grow*** | `xcomp` | 23 |
| *-ing* clause | ***Running away** seemed…*; *stopped **talking*** | `csubj`, `xcomp`, `advcl`, `acl` | 23, 24 |
| *-ed* clause | *the work **being prioritised***; ***Attached** please find…* | `acl`, `xcomp`, `advcl` | 25, 28 |
| verbless | *with the door open*; *if necessary* | `advcl`, `obl` | 24 |

The infinitival *to* is `mark` (PART) on the infinitive, not `aux` (seed [../../en/seeds/verbal/to-infinitive-marker.md](../../en/seeds/verbal/to-infinitive-marker.md)).

## 12. How Mova reads a clause

The frame reader turns each predicate of the tree into a frame ([../../prag/src/frames.rs](../../prag/src/frames.rs); [../../docs/pragmatics.md](../../docs/pragmatics.md)):

| clause element | frame field |
|---|---|
| lexical verb (+ particle) | `verb` (`put down`); `be` for a copula |
| `nsubj` | `agent` (also for passives with a *by*-agent missing: the reader keeps the subject; 28) |
| `obj` | `patient`; with a dative, the first object becomes `recipient` |
| `obl` with a VerbNet Destination/Location/Source | `place` (never for persons, animals, times, measures) |
| other `obl` | `obls` with the preposition (`with=queen`) |
| copula + adjective, `xcomp` adjective, manner adverbs | `attrs` |
| `advmod` *not*, `n't` | `neg` |
| inside an *if* clause | `cond` |
| copula with *a/an* noun, *has*, *can*, *lives in*, *made of*, *used for* | `Fact(Link)` — a general statement |

(21) `world frames "The fox flattered the crow because she wanted the cheese. The crow opened her beak and the cheese fell."` ([../../docs/pragmatics.md](../../docs/pragmatics.md), *At a glance*):

```text
#0 Event fox flatter patient=crow | cause→#1 next→#2
     roles by vn lure-59.3 «NP V NP»: agent=Agent crow=Patient
#1 Event fox(«she») want patient=cheese
#2 Event crow open patient=beak | next→#3
#3 Event cheese fall
```

The frame is where the clause pattern of §2 becomes meaning: *NP V NP* matched to VerbNet's `lure-59.3` gives Agent and Patient; the S V pattern of *fall* gives a Theme in the agent slot.

## In Mova

- Parser and annotator: [../../en/src/parse.rs](../../en/src/parse.rs), [../../en/src/annotate.rs](../../en/src/annotate.rs); agreement features from the subject: [../../en/src/ud.rs](../../en/src/ud.rs).
- Seeds of this chapter: [../../en/seeds/verbal/](../../en/seeds/verbal/) (auxiliaries, copula, existential, imperative, agreement, questions, objects), [../../en/seeds/lexicon/verb-copula-like.md](../../en/seeds/lexicon/verb-copula-like.md), [../../en/seeds/lexicon/verb-iobj-licensors.md](../../en/seeds/lexicon/verb-iobj-licensors.md), [../../en/seeds/errors/](../../en/seeds/errors/).
- Dialect decisions: [../../dialects/mova/seeds.md#clausal-predicate](../../dialects/mova/seeds.md#clausal-predicate), [iobj-sole](../../dialects/mova/seeds.md#iobj-sole), [to-phrase-obl](../../dialects/mova/seeds.md#to-phrase-obl), [imperative-do](../../dialects/mova/seeds.md#imperative-do), [depictives](../../dialects/mova/seeds.md#depictives), [discourse-adverbs](../../dialects/mova/seeds.md#discourse-adverbs).
- Clauses to frames: [../../prag/src/frames.rs](../../prag/src/frames.rs), VerbNet matching [../../prag/src/vn.rs](../../prag/src/vn.rs); clause type labels: [../../prag/src/schema.rs](../../prag/src/schema.rs).
- **Not yet**: `nsubj:outer` is rarely predicted (§4.5); vocatives at the end of imperatives are taken as `obl`; object complements and resultatives with a noun-like predicate are taken as compounds (§10); clausal subjects with *That* at the start are misparsed (§5.2); the argument/adjunct split exists only in the frames, not in the tree; Aspect, modality and tense are not frame fields beyond `neg`, `cond` and `Fact(Can)` ([../../docs/pragmatics.md](../../docs/pragmatics.md), *Not yet*).

## Sources

- Universal Dependencies guidelines (snapshot 2026-09-24): `_u/overview/simple-syntax.md`, `_u/dep/obl.md`, `_en/dep/aux_.md`, `_en/dep/cop.md`, `_en/dep/expl.md`, `_en/dep/xcomp.md`, `_en/dep/obj.md`, `_en/dep/iobj.md`, `_en/dep/compound-prt.md`, `_en/dep/obl-unmarked.md`, `_en/dep/nsubj-outer.md`, `_en/pos/AUX_.md`, `_en/specific-syntax.md`. https://universaldependencies.org/en/ ; `books/c/co/contributors-2026-universal-dependencies-guidelines`
- de Marneffe, M.-C. et al. 2021. Universal Dependencies. *Computational Linguistics* 47(2). https://aclanthology.org/2021.cl-2.11
- Curme, G. O. 1931. *A Grammar of the English Language*, vol. III: *Syntax*. Boston: Heath. Ch. II (subject, impersonal and anticipatory *it* and *there*, omission of the subject), ch. III (the predicate, do-forms, incomplete predication), ch. IV (agreement), ch. IX (parataxis and hypotaxis), ch. XVII (word order). `books/c/cu/curme-1931-grammar-english-language-vol`
- Onions, C. T. 1904/1911. *An Advanced English Syntax*. `books/o/on/onions-1911-advanced-english-syntax-based` (card only)
- Brown, G. 1851. *The Grammar of English Grammars*. `books/b/br/brown-1851-grammar-english-grammars`
- Schäfer, L. 2025. *The licensing and usage of topic drop in German*. Language Science Press (a German study, cited for the comparison with English diary drop). `books/s/sc/schafer-2025-licensing-usage-topic-drop`
- Osborne, T., Gerdes, K. 2019. The status of function words in dependency grammar. *Glossa* 4(1). https://doi.org/10.5334/gjgl.537
- Kipper Schuler, K. 2005. *VerbNet: A broad-coverage, comprehensive verb lexicon*. PhD thesis, University of Pennsylvania; VerbNet 3.4 (VerbNet 3.0 License). https://verbs.colorado.edu/verbnet/
- UD English EWT 2.18 (CC BY-SA 4.0): sentences cited by `sent_id`.
- Mova's parse database and pragmatics labels (`$MOVA_DATA/parsedb/current.conllu`, `prag.tsv`), counted 2026-10-17; the Torah JPS 1917 trees (`$MOVA_DATA/corpora/judaism/parsedb/current.conllu`).
- Repo: [../../docs/pragmatics.md](../../docs/pragmatics.md), [../../docs/dialect.md](../../docs/dialect.md), [../../tools/treeagree/README.md](../../tools/treeagree/README.md), the seeds linked above.
