# 28. Voice and the passive

English has one productive grammatical voice opposition: the active (*The dog chased the cat*) and the passive (*The cat was chased by the dog*). The passive is built analytically — *be* or *get* plus the past participle — and it demotes the doer to an optional *by*-phrase while promoting an object to subject. Around it lie constructions that look passive but are not (adjectival participles, middles, unaccusatives, causatives) and passives without an auxiliary (reduced relatives, headlines, absolutes). This chapter describes them in UD terms, gives counts from EWT and from Mova's parse database, and shows how Mova's frame reader turns a passive back into "who did what to whom".

Contents:

1. The be-passive
2. Voice=Pass: which participles are passive
3. The agent: *by*-phrases and `obl:agent`
4. The get-passive
5. Passives of ditransitives and retained objects
6. Prepositional passives
7. Clausal subjects and impersonal passives with *it*
8. Passives of raising and ECM verbs, and of naming verbs
9. Verbs without a passive
10. Reduced passives: participles in noun phrases, absolutes, headlines
11. Adjectival vs verbal participles
12. Look-alikes: middles, unaccusatives, reflexives, causatives
13. Why the passive is used
14. Frequencies
15. Quick reference
16. In Mova
17. Sources

Related chapters: clause structure and subjects in [21-clause-structure.md](21-clause-structure.md); `xcomp` and control in [23-complementation.md](23-complementation.md); relative and participial modifiers in [25-relative-clauses.md](25-relative-clauses.md); the passive as information packaging in [29-ellipsis-word-order-information-structure.md](29-ellipsis-word-order-information-structure.md) §10.

---

## 1. The be-passive

### 1.1 Form

The passive consists of a form of *be* followed by the past participle (VBN) of a transitive verb. The object of the active becomes the subject of the passive; the active subject, if expressed, follows *by* (Jespersen 1914, MEG III, ch. XV "Subject of Passive Verb", `books/j/je/jespersen-1914-modern-english-grammar-historical`; Reed & Kellogg 1877, *Higher Lessons*, lesson 129, `books/r/re/reed-1877-higher-lessons-english`).

(1) a. *The dog chased the cat.* → *The cat was chased (by the dog).*
    b. *Moths have eaten the painting.* → *The painting has been eaten by moths.*

*Be* can itself appear in any form of the verb chain, so the passive combines with every tense, aspect and modal:

| form | example |
|---|---|
| present | *is chased* |
| past | *was chased* |
| perfect | *has been chased* |
| progressive | *is being chased* |
| modal | *will / can / must be chased* |
| perfect progressive (rare) | *has been being chased* |
| infinitive | *to be chased* |
| perfect infinitive | *to have been chased* |
| gerund | *being chased* |
| perfect gerund | *having been chased* |

Poutsma's study of the English verb treats the passive infinitive and gerund in detail (Poutsma 1921, *The Characters of the English Verb*, `books/p/po/poutsma-1921-characters-english-verb`).

### 1.2 UD analysis

UD English marks the passive in three places (UD guidelines, `_en/dep/nsubj-pass.md`, `_en/dep/aux-pass.md`, `_en/dep/obl-agent.md`, `_en/feat/Voice.md`):

| element | UD |
|---|---|
| the participle | VERB, XPOS `VBN`, `Tense=Past|VerbForm=Part|Voice=Pass`; it is the head of the clause |
| *be* (or *get*) | AUX, relation `aux:pass` |
| other auxiliaries | `aux` (*has*, *will*, *being* is `aux:pass` too when it is the passive *be*) |
| the subject | `nsubj:pass` (nominal) or `csubj:pass` (clausal) |
| the *by*-phrase | `obl:agent`, with *by* as `case` |

(2) *The window was broken by the boy.* (Mova en 0.4, CPU, 2026-10-17; matches UD)

```
1  The     the     DET   2  det
2  window  window  NOUN  4  nsubj:pass
3  was     be      AUX   4  aux:pass
4  broken  break   VERB  0  root       Voice=Pass
5  by      by      ADP   7  case
6  the     the     DET   7  det
7  boy     boy     NOUN  4  obl:agent
```

(3) *Kennedy has been killed.* — `aux(killed, has)`, `aux:pass(killed, been)`, `nsubj:pass(killed, Kennedy)` (UD guidelines, `_en/dep/aux-pass.md`).

Note that `obl:agent` belongs to the **basic** tree in English (EWT 2.18 has 376 basic `obl:agent`); the enhanced graph keeps it as `obl:agent` too (419 DEPS edges, including those propagated to conjuncts). The UD universal guidelines leave the subtype optional; English uses it.

### 1.3 Constraints in the tree

The `en` seeds check the passive as a structure (seeds [`passive-subject.md`](../../en/seeds/verbal/passive-subject.md), [`passive-aux.md`](../../en/seeds/verbal/passive-aux.md), [`passive-agent.md`](../../en/seeds/verbal/passive-agent.md), [`errors/passive-structure.md`](../../en/seeds/errors/passive-structure.md)):

| rule | statement | severity |
|---|---|---|
| `en.verbal.passive-subject-voice` | `nsubj:pass` and `csubj:pass` only on a `Voice=Pass` head | error |
| `en.verbal.passive-aux-lemma` | `aux:pass` only with lemma *be* or *get* | error |
| `en.verbal.agent-passive-by` | `obl:agent` only on a `Voice=Pass` head, with *by* as `case` | error |
| (Voice.md) | a `Voice=Pass` head never has plain `nsubj` | error |
| `en.errors.by-agent` | a *by*-phrase with an article under a passive left as plain `obl` | warn |

## 2. Voice=Pass: which participles are passive

Until UD 2.13, English put `Voice=Pass` only on participles that had an `aux:pass` dependent. Since 2.13 every past participle **interpreted as passive** gets it, whatever its relation — including attributive (*a ruined sandwich*), reduced relatives (*paintings eaten by moths*) and parentheticals (*Beethoven (born 1770, died 1827)*) (UD guidelines, `_en/feat/Voice.md`, history note; EWT issue 290, https://github.com/UniversalDependencies/UD_English-EWT/issues/290).

The participle of the active perfect is **not** passive: *Moths have eaten the painting* — *eaten* has no `Voice`. Words attached as `aux`, `aux:pass` or `cop` are never eligible.

EWT 2.18 has 3 261 `Voice=Pass` participles (2026-10-17). Their relations:

| relation of the participle | count | typical example |
|---|---:|---|
| root | 757 | *Kennedy was killed.* |
| amod | 585 | *a ruined sandwich* |
| acl | 558 | *paintings eaten by moths* |
| conj | 317 | *… was arrested and charged* |
| advcl | 308 | *Founded in 1990, the firm …* |
| xcomp | 223 | *He got arrested; I want it done* |
| acl:relcl | 184 | *the man who was arrested* |
| ccomp | 162 | *I heard it was sold* |

So only about a quarter of passive participles head a main clause; the rest sit in modifiers and complements. Any reader that looks for passives by `aux:pass` alone misses three quarters of them.

## 3. The agent: *by*-phrases and `obl:agent`

### 3.1 Agent vs other *by*-phrases

Only a *by*-phrase that names the doer is `obl:agent`. Means, time limits and places with *by* are plain `obl` (seed [`errors/passive-agent.md`](../../en/seeds/errors/passive-agent.md)):

(4) a. *The cat was chased **by the dog**.* — `obl:agent`
    b. *Arms are transported **by land**.* — `obl` (means)
    c. *It must be finished **by Friday**.* — `obl` (time limit)
    d. *The house was built **by the river**.* — `obl` (place)

Reed & Kellogg (1877, lesson 129) give a test that also separates passive from adjectival participles: if *by* + doer can be added without changing the sense, the form is passive (*The coat was worn by Joseph*); otherwise it is an adjective after a copula (*The coat was badly worn*).

### 3.2 Agentless passives

Most passives have no *by*-phrase. In EWT 2.18, 376 `obl:agent` stand against 1 643 `aux:pass` (23 %); in Mova's parse database (2026-10-17) 5 920 against 50 180 (12 %). The agent is left out when it is unknown (*My bike was stolen*), obvious (*He was elected in 2004*), generic (*English is spoken here*), or deliberately concealed (*Mistakes were made*). See §13.

### 3.3 Mova's parser

Mova en 0.4 labels the agent `obl:agent` in a simple passive (2) but not in a ditransitive one (§5): «Mary was given a book by her aunt.» gets plain `obl(given, aunt)` (CPU, 2026-10-17). The frame reader compensates (§16.2).

## 4. The get-passive

### 4.1 Form and meaning

Colloquial English forms a passive with *get*: *He got shot*, *The book got stolen*, *I got put on hold twice*. It is more dynamic than the *be*-passive (it reports an event, never a state), often implies that the subject is affected adversely or beneficially, and is avoided in formal writing (Curme 1931, *Syntax*, on the passive with *get* and *become*, `books/c/cu/curme-1931-grammar-english-language-vol`).

(5) *The next 5 companies were my responsibility while in EBS, and did not get assigned during the reorg.* — EWT 2.18 `email-enronsent09_01-0037`

### 4.2 UD analysis

In a get-passive *get* is `AUX` with `aux:pass` and the participle has `Voice=Pass` (UD guidelines, `_en/dep/aux-pass.md`: *Kennedy got killed*; `_en/dep/aux_.md`). EWT 2.18 has 44 `aux:pass` *get* against 1 599 *be*. Other uses of *get* stay `VERB` (seed [`get-passive.md`](../../en/seeds/verbal/get-passive.md)):

| sentence | *get* | analysis |
|---|---|---|
| *He got shot.* | AUX | `aux:pass(shot, got)`, `nsubj:pass(shot, He)` |
| *Bill got rich.* | VERB | `xcomp(got, rich)` — 'became' |
| *I got it fixed.* | VERB | `obj(got, it)`, `xcomp(got, fixed)` — causative (§12.4) |
| *I got a letter.* | VERB | `obj` — 'received' |

The rule `en.verbal.get-aux-passive-only` (error) allows AUX *get* only as `aux:pass` (or as the head of an elided clause).

### 4.3 Mova's parser

Mova en 0.4 does not produce get-passives: «The window got broken.» is parsed as `nsubj(got, window)`, `xcomp(got, broken)` with *got* as VERB root (CPU, 2026-10-17) — the analysis UD reserves for *got rich*. The frame reader then makes two frames, `window get` and `window break` with the window as the agent of *break*; the second is the useful one, by luck of the `xcomp` subject rule.

## 5. Passives of ditransitives and retained objects

With verbs of giving and telling, either object can in principle become the subject. In present-day English the recipient (the first object of the active) is the usual passive subject; the theme stays as a **retained object** (Jespersen 1914, MEG III, §§15.1–15.2):

(6) a. *Her aunt gave Mary a book.*
    b. *Mary was given a book (by her aunt).* — recipient passive, retained object *a book*
    c. *A book was given (to) Mary (by her aunt).* — theme passive; *to* is usual in American English

(7) *Then, at his request, he was given permission to go.* — EWT 2.18 `weblog-blogspot.com_alaindewitt_20040929103700_ENG_20040929_103700-0059`

In UD the retained object is `obj` of the passive participle; the subject is `nsubj:pass`. The seed [`lexicon/verb-passive-retained-object.md`](../../en/seeds/lexicon/verb-passive-retained-object.md) lists the verbs that allow `obj` under `Voice=Pass` (*give, send, tell, show, offer, bring, pay, ask, teach, lend, hand, owe, promise, grant, deny, allow, award, charge …*, plus naming verbs and *take care of*); any other `obj` under a passive is flagged as suspect.

Mova en 0.4 on (6b) gives `nsubj(given, Mary)` — plain `nsubj`, not `nsubj:pass` — together with `aux:pass(given, was)`, `obj(given, book)` and `obl(given, aunt)` (CPU, 2026-10-17). That violates the structural rule of §1.3 (a passive head with `aux:pass` must have a passive subject); the teacher loop's gates exist to catch exactly this.

## 6. Prepositional passives

English, unusually among European languages, lets the object of a preposition become a passive subject, leaving the preposition **stranded** after the verb (Jespersen 1914, MEG III, ch. XV; Curme 1931):

(8) a. *This bed has been slept in.*
    b. *That matter was talked about in detail already.* (UD guidelines, `_en/specific-syntax.md`, VP ellipsis section)
    c. *She was laughed at.*
    d. *The problem has been dealt with.*
    e. *He was taken care of.* (idiom with a retained noun)

The condition is that verb and preposition form a semantic unit and the subject is affected: *The bed has been slept in* (it is now unmade) is fine, *\*The city was slept in* is odd.

**UD analysis.** (Stranded prepositions in general: [22-phrases.md](22-phrases.md) §7.3.) A stranded preposition has no nominal to attach to as `case`, so it takes the nominal relation itself. The English specific-syntax page (written in UD v1 terms) gives `nmod(talked, about)`; in UD v2 a dependent of a verb is `obl`, so the current form is `obl(talked, about)`. Phrasal verbs whose particle is an adverb (*look up*, *give up*) are different: there the particle is `compound:prt` both in the active and in the passive (*The word was looked up*).

Mova en 0.4 parses (8a) with `compound:prt(slept, in)` (CPU, 2026-10-17): the stranded preposition is treated as a particle. Both analyses keep *in* on the verb; the frame reader reads it as part of a phrasal verb *sleep in*, which loses the relation 'the bed is where someone slept'.

## 7. Clausal subjects and impersonal passives with *it*

A clause can be the subject of a passive verb; it is `csubj:pass` (UD guidelines, `_en/dep/csubj-pass.md`):

(9) *That she lied was suspected by everyone.* — `csubj:pass(suspected, lied)`

Usually the clause is extraposed and *it* fills the subject position as an expletive:

(10) *It is often said that politicians, diplomats and perhaps also lawyers and business people must sometimes lie …* — EWT 2.18 `weblog-blogspot.com_alaindewitt_20060924104100_ENG_20060924_104100-0107`

```
1   It    it    PRON   4   expl
2   is    be    AUX    4   aux:pass
3   often often ADV    4   advmod
4   said  say   VERB   0   root
5   that  that  SCONJ  18  mark
18  lie   lie   VERB   4   csubj:pass
```

EWT 2.18 has 8 `csubj:pass`; most are of this extraposed kind (*it should be noted that …*, *it must be said that …*). The parse database has 381 (2026-10-17).

Mova en 0.4 parses «It is said that the king is ill.» with `nsubj(said, It)` and the *that*-clause as `advcl` (CPU, 2026-10-17): two errors — *it* should be `expl` and the clause `csubj:pass`. A reader of this tree takes *it* for an entity and loses the content clause as what was said.

The alternative "raising" form *The king is said to be ill* is treated in §8.

## 8. Passives of raising and ECM verbs, and of naming verbs

### 8.1 *Be said / believed / reported to*

Verbs of saying and believing that take an object plus infinitive in the active (*They believe him to be guilty*) passivize with the infinitive as `xcomp`:

(11) a. *He is believed to be guilty.* — `nsubj:pass(believed, He)`, `xcomp(believed, guilty)`; enhanced: `nsubj:xsubj(guilty, He)`
     b. *Over 300 Iraqis are reported dead and 500 wounded in Fallujah alone.* — EWT 2.18 `weblog-blogspot.com_healingiraq_20040409053012_ENG_20040409_053012-0022`

(11b) also shows gapping with an empty node in the enhanced graph (see chapter 29 §2.2):

```
3    Iraqis    PROPN  5  nsubj:pass  5:nsubj:pass|6:nsubj:xsubj|8:nsubj:pass
5    reported  VERB   0  root        0:root
6    dead      ADJ    5  xcomp       5:xcomp
7    and       CCONJ  8  cc          8:cc|8.1:cc
8    500       NUM    5  conj        5:conj:and|8.1:nsubj:pass|9:nsubj:xsubj
8.1  reported  VERB   _  _           5:conj:and
9    wounded   ADJ    8  orphan      8.1:xcomp
```

(The DEPS edge `8:nsubj:pass` on *Iraqis* is an oddity of the gold file: it makes *Iraqis* the subject of the number *500*.)

### 8.2 Naming and electing verbs

*Call, name, elect, appoint, make, consider* take a predicative complement that stays in the passive as `xcomp`, not `obj` (seed [`lexicon/verb-passive-retained-object.md`](../../en/seeds/lexicon/verb-passive-retained-object.md)):

(12) a. *He was elected president.* — `xcomp(elected, president)`
     b. *She is called "martyr" by several Arab leaders and "activist" by the European press.* — EWT 2.18 `weblog-blogspot.com_alaindewitt_20060924104100_ENG_20060924_104100-0130` (also a gapping example with an empty node)
     c. *Neither shall thy name any more be called Abram …* — Torah (JPS 1917) `jps:GEN:418`: `nsubj:pass(called, name)`, `aux:pass(called, be)`, `xcomp(called, Abram)` in the parse database — right.

## 9. Verbs without a passive

Only verbs with a real object of action passivize. Three groups do not (Jespersen 1914, MEG III, §15.12 and ch. XVI "Transitivity"; Brown 1851, Part II, ch. VI; seed [`lexicon/verb-no-passive.md`](../../en/seeds/lexicon/verb-no-passive.md)):

| group | examples | why |
|---|---|---|
| intransitive (no object) | *happen, occur, exist, arrive, arise, emerge, seem, appear, remain* | *\*The accident was happened* |
| measure and relation verbs with an object-like complement | *cost, weigh, last, resemble, fit, suit, have* (possession), *lack* | the "object" is a measure or a relation, not affected: *\*Two shillings are cost by it*; *\*His father was resembled by him* (Jespersen) |
| reflexive and reciprocal objects | *behave oneself, each other* | the object is bound to the subject |
| some symmetric and stative uses | *marry* (in 'X married Y' the passive is fine; *\*Y was married by X* in the sense of the wedding officiant only) | lexical |

The rule `en.lexicon.no-passive-verb` (warn) flags `Voice=Pass` on these lemmas. PropBank's first rolesets of these verbs have no proto-agent ARG0, which the seed cites as a formal sign (PropBank is used for tests only, not in the build; `docs/pragmatics.md`).

Unaccusative verbs (*arrive, fall, die*) never passivize: they already have their undergoer as subject (§12.2).

## 10. Reduced passives: participles in noun phrases, absolutes, headlines

### 10.1 Participial modifiers of nouns

A passive participle can modify a noun directly. If it comes after the noun with its own dependents, it is `acl`; if it is a bare prenominal word, it is `amod`. In both cases since UD 2.13 it has `Voice=Pass` (§2):

(13) a. *paintings **eaten** by moths* — `acl(paintings, eaten)`, `obl:agent(eaten, moths)` (UD guidelines, `_en/feat/Voice.md`)
     b. *a **ruined** sandwich* — `amod(sandwich, ruined)`
     c. *The letter **written** by the queen arrived.*

These are not relative clauses in the UD English analysis: past-participial and gerund-participial clauses are "never considered relative clauses" (UD guidelines, `_en/dep/acl-relcl.md`); see [25-relative-clauses.md](25-relative-clauses.md).

Mova en 0.4 parses (13c) with `acl(letter, written)` — right — but the *by*-phrase as plain `obl(written, queen)`, not `obl:agent` (CPU, 2026-10-17); «The broken vase lay on the floor.» gets `amod(vase, broken)` with *broken* VERB — right.

### 10.2 Absolutes and adverbial participles

(14) a. *Founded in 1990, the firm grew fast.* — `advcl(grew, Founded)`
     b. *His master being absent, the business was neglected.* (Reed & Kellogg 1877, lesson 44) — absolute; seed [`absolute-construction.md`](../../en/seeds/verbal/absolute-construction.md)
     c. *Armed militias, many staffed by former soldiers, …* — `nsubj:pass` without `aux:pass` (seed [`passive-subject.md`](../../en/seeds/verbal/passive-subject.md))

### 10.3 Headlines and notices

Headlines drop the auxiliary: *Man arrested*, *Key suspect detained*, *Passport needed*. The participle is the root with `Voice=Pass` and the noun is `nsubj:pass`; there is no `aux:pass`. EWT 2.18 has about 40 such passive subjects without an auxiliary (seed [`errors/passive-structure.md`](../../en/seeds/errors/passive-structure.md)).

Business e-mail in EWT has its own reduced passive, fronted *attached* with inversion: *Attached is the file*, *Attached please find …*. *Attach* is the most frequent passive root lemma in EWT (38 roots of 757, 2026-10-17).

## 11. Adjectival vs verbal participles

The *-ed/-en* form stands between verb and adjective. The distinction matters for UD (VERB + `aux:pass` vs ADJ + `cop`) and for meaning (an event vs a state). Signs of each (Poutsma 1921, *Participles*, §§7–13, 33–42; seed [`participle-or-adjective.md`](../../en/seeds/verbal/participle-or-adjective.md)):

| verbal (passive) | adjectival |
|---|---|
| a *by*-agent: *was interested **by the offer*** | degree words: ***very** interested*, *more surprised* |
| progressive *be being*: *is being built* | after *seem, become, look, remain*: *seems tired* |
| an object (retained): *was given a book* | *un-* with no verb behind it: *unexpected, uninteresting* |
| time adverbials of the event: *was closed at 5 pm* | coordination with an adjective: *tired and hungry* |
| the active is natural: *someone closed the shop* | *-ed* adjectives from nouns: *talented, gifted, kind-hearted* (not participles at all) |

(15) a. *The shop was closed at five by the owner.* — event: VERB, `aux:pass`, `Voice=Pass`
     b. *The shop was closed all day.* — state: often ADJ, `cop`; EWT is not fully consistent here
     c. *I am very interested.* — ADJ (`JJ`), `cop`
     d. *He was interested by the offer.* — VERB (`VBN`), passive

The rule `en.verbal.very-participle` warns when *very* modifies a VERB participle (but *very much appreciated* is fine: *very* modifies *much*).

## 12. Look-alikes: middles, unaccusatives, reflexives, causatives

### 12.1 Middles

A middle has an active form, the patient as subject, no expressible agent, and usually a generic meaning with a manner adverb:

(16) a. *These shirts wash well.*
     b. *This book reads easily.*
     c. *The car handles like a dream.*

UD treats them as ordinary active clauses: `nsubj(wash, shirts)`, `advmod(wash, well)` — exactly what Mova en 0.4 gives (CPU, 2026-10-17). The role of the subject (patient, not agent) is not in the tree.

### 12.2 Unaccusatives and the causative alternation

Many change-of-state verbs alternate between a transitive causative and an intransitive "anticausative" (ergative, unaccusative) use (Levin 1993, *English Verb Classes and Alternations*, wanted; VerbNet classes built on it):

(17) a. *The boy opened the door.* / *The door opened.*
     b. *The sun melted the ice.* / *The ice melted.*
     c. *She broke the window.* / *The window broke.*

The intransitive is **not** a passive: no `aux:pass`, no `Voice`, plain `nsubj`. Marking it `nsubj:pass` is a known annotation error (seed [`errors/passive-structure.md`](../../en/seeds/errors/passive-structure.md)). Mova en 0.4 parses «The door opened.» as `nsubj(opened, door)` — right. VerbNet tells the reader that the subject is a Patient; the frame reader keeps the subject in the agent slot anyway (`docs/pragmatics.md`, "The subject stays the agent whatever its role"), so *The door opened* gives `door open` with *door* as the frame's agent.

### 12.3 Reflexives

A reflexive object keeps the clause active: *She dressed herself*, *He hurt himself*. Some verbs drop the reflexive with the same meaning (*She dressed*, *He shaved*). UD: `obj(dressed, herself)`, with *herself* `PRON Reflex=Yes`. English has no reflexive passive (unlike the Romance *se*-passive or Slavic *-sja*).

### 12.4 Causatives: *have/get something done*

(18) a. *She had her hair cut.*
     b. *I usually wait until I go home to get my hair cut.* — EWT 2.18 `reviews-156953-0003`

```
16  get   get   VERB  10  advcl   10:advcl:to
17  my    my    PRON  18  nmod:poss
18  hair  hair  NOUN  16  obj     16:obj|19:nsubj:xsubj
19  cut   cut   VERB  16  xcomp   16:xcomp
```

The causer is the subject of *have/get*; the thing affected is its `obj`; the participle is `xcomp`, and in the enhanced graph the object is its subject (`nsubj:xsubj`). The participle is passive in meaning but its subject is not `nsubj:pass` in basic UD. The same structure serves the "experiential" *have*: *He had his car stolen* (he suffered it). Curme (1931, *Syntax*) separates the two by stress: stressed *had/got* in *I had a new suit made* presents the subject as planning the action; lightly stressed *had/got* in *I had my right leg hurt in the accident* presents the subject as suffering it. He treats both as a colloquial "passive form" with the passive force in the participle. The tree does not show the difference. Mova en 0.4 parses (18a) as `obj(had, hair)`, `xcomp(had, cut)` — the EWT analysis (CPU, 2026-10-17).

### 12.5 Summary

| construction | example | voice in UD | subject relation |
|---|---|---|---|
| be-passive | *The window was broken.* | `Voice=Pass` | `nsubj:pass` |
| get-passive | *The window got broken.* | `Voice=Pass` | `nsubj:pass` |
| adjectival | *The window was broken (for weeks).* | ADJ, no Voice | `nsubj` + `cop` |
| unaccusative | *The window broke.* | active | `nsubj` |
| middle | *This glass breaks easily.* | active | `nsubj` |
| causative | *I had the window repaired.* | participle `Voice=Pass`, `xcomp` | `obj` of *have* |
| reflexive | *He cut himself.* | active | `nsubj` |

## 13. Why the passive is used

The passive changes **information packaging**, not propositional content (Jespersen 1924, *The Philosophy of Grammar*, ch. XII "Active and Passive", `books/j/je/jespersen-1924-philosophy-grammar`; Curme 1931). Its uses:

1. **Agent unknown or irrelevant**: *My bike was stolen.* *English is spoken here.*
2. **Topic continuity**: keep the current topic as subject. *The fox saw the crow. **She** was holding a piece of cheese …* vs *… **It** was seen by the fox …* — the passive lets the given referent stay first (chapter 29 §§9–10).
3. **End weight and end focus**: put a long or new agent at the end. *The play was written **by a young woman from Leeds who had never seen a theatre**.*
4. **Impersonal and scientific style**: *The solution was heated to 80 °C.* — common in the school lessons' science courses; the most frequent passive lemmas in Mova's parse database are *use* (5 506), *make* (4 021) and *call* (3 689) (2026-10-17): *X is used for Y*, *X is made of Y*, *X is called Y*.
5. **Concealment or politeness**: *Mistakes were made.* *You are requested to …* (see [32-politeness-and-hedging.md](32-politeness-and-hedging.md)).

Fowler (1926, *A Dictionary of Modern English Usage*, `books/f/fo/fowler-1926-dictionary-modern-english-usage`) and Strunk (1918, *The Elements of Style*, `books/s/st/strunk-1918-elements-style`) advise the active voice for vigour; usage guides warn against "double passives" (*It was attempted to be done*).

## 14. Frequencies

Counted on 2026-10-17 with `awk` over the CoNLL-U files.

| measure | EWT 2.18 (254 820 words) | parse database (8.06 M words) |
|---|---:|---:|
| `aux:pass` | 1 643 (be 1 599, get 44) | 50 180 |
| `nsubj:pass` | 1 445 | 42 380 |
| `csubj:pass` | 8 | 381 |
| `obl:agent` | 376 | 5 920 |
| `Voice=Pass` participles | 3 261 | 103 386 |
| `Voice=Pass` per 1 000 words | 12.8 | 12.8 |
| agent share (`obl:agent` / `aux:pass`) | 23 % | 12 % |

The two corpora use passive participles at the same rate; the lessons and tales express the agent half as often. The parse database is silver (agreed parser drafts checked by the teacher, `docs/architecture.md`), so its counts carry the parsers' biases, such as the missing `obl:agent` in ditransitives (§3.3).

## 15. Quick reference

| construction | example | analysis |
|---|---|---|
| be-passive | *was chased by the dog* | `aux:pass(chased, was)`, `nsubj:pass`, `obl:agent(chased, dog)`, `case(dog, by)`; `Voice=Pass` |
| perfect passive | *has been killed* | `aux(killed, has)`, `aux:pass(killed, been)` |
| get-passive | *got killed* | `aux:pass(killed, got)`, *got* AUX |
| retained object | *was given a book* | `nsubj:pass(given, Mary)`, `obj(given, book)` |
| prepositional passive | *was talked about* | `obl(talked, about)` (v1 page: `nmod`) |
| clausal subject | *That she lied was suspected* | `csubj:pass(suspected, lied)` |
| extraposed | *It is said that …* | `expl(said, It)`, `csubj:pass(said, …)` |
| raising passive | *is believed to be guilty* | `xcomp(believed, guilty)`; EUD `nsubj:xsubj` |
| naming | *was elected president* | `xcomp(elected, president)` |
| reduced relative | *paintings eaten by moths* | `acl(paintings, eaten)`, `Voice=Pass` |
| attributive | *a ruined sandwich* | `amod`, `Voice=Pass` |
| headline | *Man arrested* | root participle, `nsubj:pass`, no aux |
| causative | *had her hair cut* | `obj(had, hair)`, `xcomp(had, cut)`; EUD `nsubj:xsubj(cut, hair)` |
| middle / unaccusative | *These shirts wash well / The door opened* | active, `nsubj` |

---

## 16. In Mova

### 16.1 Parsing

The `en` parser predicts `aux:pass`, `nsubj:pass`, `csubj:pass` and `obl:agent` as in EWT. Measured (Mova en 0.4, CPU, 2026-10-17):

| sentence | Mova en 0.4 | UD | verdict |
|---|---|---|---|
| *The window was broken by the boy.* | nsubj:pass, aux:pass, obl:agent | same | right |
| *The window got broken.* | *got* VERB root, xcomp(got, broken) | aux:pass(broken, got) | wrong |
| *This bed has been slept in.* | compound:prt(slept, in) | obl(slept, in) | wrong label |
| *Mary was given a book by her aunt.* | **nsubj**(given, Mary), obj(book), obl(aunt) | nsubj:pass, obj, obl:agent | two errors |
| *It is said that the king is ill.* | nsubj(said, It), advcl(said, ill) | expl, csubj:pass | two errors |
| *That he lied was proven.* | *That* PRON nsubj:pass, *lied* acl:relcl of *That* | csubj:pass(proven, lied), mark(lied, That) | wrong |
| *The letter written by the queen arrived.* | acl(letter, written), obl(written, queen) | acl, obl:agent | label |
| *The broken vase lay on the floor.* | amod(vase, broken) | same | right |
| *She had her hair cut.* | obj(had, hair), xcomp(had, cut) | same | right |
| *The door opened. / These shirts wash well.* | nsubj | same | right |

The gates of the teacher loop ([`tools/treeagree`](../../tools/treeagree), seeds in [`en/seeds/verbal/`](../../en/seeds/verbal/)) reject the structural violations (a plain `nsubj` under `aux:pass`); the parse database is cleaner than the bare parser for that reason.

### 16.2 The frame reader

[`prag/src/frames.rs`](../../prag/src/frames.rs) turns the passive back into roles:

- **Detection**: a predicate is passive when it has an `aux:pass` or `nsubj:pass` child (`let passive = …`). Reduced passives (`acl`, `amod`) and the misparsed get-passive are therefore read as active.
- **Subject**: the subject of a passive becomes the **patient**; of an active, the agent.
- **Agent**: an `obl:agent` child becomes the agent. If the parser left the *by*-phrase as plain `obl`, the switch `vn-by` (on by default) takes a *by*-oblique that can act — a person, animal or pronoun, not a means such as *by hand* — as the agent (`fn lexicon_roles`; `PRAG_OFF=vn-by` turns it off).
- **VerbNet** ([`prag/src/vn.rs`](../../prag/src/vn.rs)): in matching a passive clause against VerbNet frames, the subject stands where the first object stands, and the *by*-phrase where the subject stands.
- **Facts**: a passive *make … of* gives `Fact(made of)`, a passive *use … for* gives `Fact(used for)` — the two most common lesson passives.

Measured (`world frames`, CPU, 2026-10-17):

| text | frame |
|---|---|
| *Mary was given a book by her aunt.* | `Event aunt give patient=mary` (vn give-13.1-1: subject=Recipient, book=Theme) |
| *The window got broken.* | `Event window get` + `Event window break` |
| *Bread is made of flour.* | `Fact(made of) _ make patient=bread of=flour` |

The first line shows the rescue of the agent by `vn-by` and also its limit: VerbNet knows that Mary is the Recipient, yet the frame puts her in the patient slot and the book is lost — the rule "the subject of a passive is the patient" wins over VerbNet's role for a recipient passive.

### 16.3 Not yet

- Recipient passives: the subject should be the recipient and the retained object the patient (§5).
- Reduced passives (`acl`/`amod` participles with `Voice=Pass`) make no Event frame with the head noun as patient; *paintings eaten by moths* gives no `moth eat painting`.
- Get-passives are parsed as `xcomp` and read only by accident.
- Middles and unaccusatives keep the undergoer in the agent slot.
- `Voice=Pass` itself is not read by the frame reader: it looks only at the relations.
- Prepositional passives lose the role of the subject (*the bed* as place of *sleep in*).

## 17. Sources

- Brown, Goold. 1851. *The Grammar of English Grammars.* Part II, ch. VI, "Form of Passive Verbs". `books/b/br/brown-1851-grammar-english-grammars`.
- Curme, George O. 1931. *A Grammar of the English Language, vol. III: Syntax.* `books/c/cu/curme-1931-grammar-english-language-vol`.
- Fowler, H. W. 1926. *A Dictionary of Modern English Usage.* `books/f/fo/fowler-1926-dictionary-modern-english-usage`.
- Jespersen, Otto. 1914. *A Modern English Grammar on Historical Principles*, part III, ch. XV "Subject of Passive Verb", ch. XVI "Transitivity". `books/j/je/jespersen-1914-modern-english-grammar-historical`.
- Jespersen, Otto. 1924. *The Philosophy of Grammar*, ch. XII "Active and Passive" (pp. 164 ff.). `books/j/je/jespersen-1924-philosophy-grammar`.
- Poutsma, H. 1921. *The Characters of the English Verb and the Expanded Form*; and Poutsma's treatment of participles and the infinitive (as cited in the `en` seeds). `books/p/po/poutsma-1921-characters-english-verb`.
- Reed, Alonzo and Brainerd Kellogg. 1877. *Higher Lessons in English*, lessons 44 and 129. `books/r/re/reed-1877-higher-lessons-english`.
- Strunk, William. 1918. *The Elements of Style.* `books/s/st/strunk-1918-elements-style`.
- Universal Dependencies. English guidelines, snapshot 2026-09-24: `_en/feat/Voice.md` (https://universaldependencies.org/en/feat/Voice.html), `_en/dep/nsubj-pass.md`, `_en/dep/csubj-pass.md`, `_en/dep/aux-pass.md`, `_en/dep/obl-agent.md` (https://universaldependencies.org/en/dep/obl-agent.html), `_en/dep/aux_.md`, `_en/dep/acl-relcl.md`, `_en/specific-syntax.md`. EWT issue 290 (Voice=Pass broadened in 2.13): https://github.com/UniversalDependencies/UD_English-EWT/issues/290.
- UD English EWT 2.18 (Silveira et al. 2014, https://aclanthology.org/L14-1067; CC BY-SA 4.0). Sentences cited by `sent_id`.
- VerbNet 3.4 (Kipper Schuler et al.; VerbNet 3.0 License), as compiled in `prag/data/verbnet.tsv` (`docs/pragmatics.md`).
- Mova: seeds [`passive-subject.md`](../../en/seeds/verbal/passive-subject.md), [`passive-aux.md`](../../en/seeds/verbal/passive-aux.md), [`passive-agent.md`](../../en/seeds/verbal/passive-agent.md), [`get-passive.md`](../../en/seeds/verbal/get-passive.md), [`participle-or-adjective.md`](../../en/seeds/verbal/participle-or-adjective.md), [`absolute-construction.md`](../../en/seeds/verbal/absolute-construction.md), [`lexicon/verb-no-passive.md`](../../en/seeds/lexicon/verb-no-passive.md), [`lexicon/verb-passive-retained-object.md`](../../en/seeds/lexicon/verb-passive-retained-object.md), [`errors/passive-structure.md`](../../en/seeds/errors/passive-structure.md), [`errors/passive-agent.md`](../../en/seeds/errors/passive-agent.md); code [`prag/src/frames.rs`](../../prag/src/frames.rs), [`prag/src/vn.rs`](../../prag/src/vn.rs); [`docs/pragmatics.md`](../../docs/pragmatics.md).
- Wanted: Levin 1993 *English Verb Classes and Alternations*; Huddleston & Pullum 2002, ch. 16 (information packaging, passive); Siewierska 1984 *The Passive*; Keenan & Dryer 2007 "Passive in the world's languages".
