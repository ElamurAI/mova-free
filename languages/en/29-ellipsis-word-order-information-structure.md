# 29. Ellipsis, word order and information structure

English leaves out what the context supplies (ellipsis) and arranges what it says so that known things come first and new or heavy things come last (information structure). Both bend the canonical subject–verb–object clause: ellipsis removes heads that a dependency tree needs, and the information-packaging constructions — fronting, inversion, dislocation, extraposition, clefts, existentials, the passive, the dative and particle alternations — move material away from its usual place. This chapter describes the constructions, their analysis in UD (including the enhanced layer's empty nodes), what Mova's parser does with them, and how information structure is modelled: given/new, topic/comment, focus, and the question under discussion.

Contents:

1. Overview: what is missing, what is moved
2. Gapping and stripping (`orphan`, empty nodes)
3. VP ellipsis
4. Sluicing
5. Noun-phrase ellipsis
6. Right-node raising
7. Comparative deletion and comparative ellipsis
8. Fragments, answers and non-sentential utterances
9. Canonical word order and its variants
   9.1 SVO and the order of complements · 9.2 subject–auxiliary inversion · 9.3 negative and restrictive inversion · 9.4 locative and directional inversion · 9.5 quotative inversion · 9.6 fronting (topicalization) · 9.7 dislocation · 9.8 extraposition and *tough*-constructions · 9.9 existential *there* · 9.10 heavy-NP shift · 9.11 particle placement · 9.12 adverb placement
10. Information-packaging alternations: clefts, pseudo-clefts, passive, dative
11. Information structure: given/new, topic, focus
12. The question under discussion (QUD)
13. Prosody
14. Quick reference
15. In Mova
16. Sources

Related chapters: coordination in [24-coordination-subordination.md](24-coordination-subordination.md); questions and wh-movement in [26-questions.md](26-questions.md); negation and negative inversion in [27-negation.md](27-negation.md); the passive in [28-voice-and-passive.md](28-voice-and-passive.md); reference and salience in [35-reference-and-coreference.md](35-reference-and-coreference.md); discourse relations in [36-discourse-relations.md](36-discourse-relations.md).

---

## 1. Overview: what is missing, what is moved

| phenomenon | example | what is absent or displaced | UD device |
|---|---|---|---|
| gapping | *Marie went to Paris and Miriam to Prague.* | the verb of the second clause | promotion + `orphan`; EUD empty node |
| stripping | *Mary left, but not John.* | everything but one remnant (+ polarity word) | promotion (+ `orphan`) |
| VP ellipsis | *Mary will win and John will too.* | the VP after an auxiliary | the auxiliary is the head |
| sluicing | *Someone called, but I don't know who.* | the clause after a wh-word | the wh-word stands in for the clause |
| NP ellipsis | *I bought three apples and Tom bought two.* | the head noun | promotion of a dependent (`nummod` → `obj`) |
| right-node raising | *John bought, and Mary sold, the old car.* | the shared right element in the first conjunct | shared element on the first verb |
| comparative deletion | *Mary is taller than Peter is.* | the degree phrase | no special relation |
| fragment | *Not yet.* / *To Paris.* | the whole clause | the remnant is the root |
| fronting | *This book I have never read.* | the object moved left | relations unchanged, order changed |
| dislocation | *My brother, he lives in Leeds.* | a peripheral nominal doubled by a pronoun | `dislocated` |
| extraposition | *It is easy to please John.* | the clausal subject moved right | `expl` + `csubj` |
| cleft | *It was the cat that ate the fish.* | one constituent foregrounded | `expl` + `cop` + `advcl:relcl` |

The basic UD tree has no empty nodes: when a head is missing, a dependent is **promoted** to its place, following an obliqueness hierarchy (UD guidelines, `_u-overview/specific-syntax.md`, Ellipsis; https://universaldependencies.org/u/overview/specific-syntax.html#ellipsis). The enhanced representation can add an **empty node** (ID like `8.1`) for the missing predicate and attach the remnants to it with their real relations (UD guidelines, `_u-overview/enhanced-syntax.md`; Schuster & Manning 2016, https://aclanthology.org/L16-1376).

## 2. Gapping and stripping (`orphan`, empty nodes)

### 2.1 Gapping

In **gapping**, coordinated clauses share a verb that is pronounced only in the first; the second keeps two or more contrasting remnants:

(1) *Marie went to Paris and Miriam to Prague.* (UD guidelines, `_en/dep/orphan.md`)

```
nsubj(went, Marie)
obl(went, Paris)
conj(went, Miriam)      Miriam is promoted to the place of the missing "went"
cc(Miriam, and)
orphan(Miriam, Prague)  the other remnant attaches to the promoted one
case(Prague, to)
```

The promoted remnant is the one highest on the hierarchy nsubj > obj > iobj > obl > advmod > csubj > xcomp > ccomp > advcl (UD guidelines, `_u-overview/specific-syntax.md`). `orphan` links only remnants of a missing predicate; it is not used when any verbal material remains (VP ellipsis, §3; right-node raising, §6) (UD guidelines, `_en/dep/orphan.md`; seeds [`verbal/gapping-orphan.md`](../../en/seeds/verbal/gapping-orphan.md), [`errors/gapping-orphan.md`](../../en/seeds/errors/gapping-orphan.md)).

### 2.2 The enhanced layer: empty nodes

(2) *I actually loss 4 pounds after my 1st treatment and 2 pounds after my 2nd treatment.* — EWT 2.18 `reviews-247097-0005`

```
ID    FORM       HEAD DEPREL  DEPS
3     loss       0    root    0:root
5     pounds     3    obj     3:obj
9     treatment  3    obl     3:obl:after
10    and        12   cc      10.1:cc|12:cc
10.1  loss       _    _       3:conj:and
12    pounds     3    conj    3:conj:and|10.1:obj
16    treatment  12   orphan  10.1:obl:after
```

In the basic tree *pounds* (12) is promoted to `conj` and *treatment* (16) is its `orphan`; in DEPS the empty node 10.1 is a copy of *loss* and the remnants hang from it as `obj` and `obl:after`, exactly as in the first clause. EWT 2.18 has 29 `orphan` relations and 43 empty nodes (2026-10-17). The sentence also shows a gold oddity: *I* (1) has a DEPS edge `12:nsubj`, making it a subject of the promoted noun as well as of the empty node.

Gapping can apply in passives and with non-verbal predicates:

(3) *Over 300 Iraqis are reported dead and 500 wounded in Fallujah alone.* — EWT 2.18 `weblog-blogspot.com_healingiraq_20040409053012_ENG_20040409_053012-0022`: `conj(reported, 500)`, `orphan(500, wounded)`; DEPS: empty node 8.1 *reported* with `8.1:nsubj:pass` on *500* and `8.1:xcomp` on *wounded*.

(4) *She is called "martyr" by several Arab leaders and "activist" by the European press.* — EWT 2.18 `weblog-blogspot.com_alaindewitt_20060924104100_ENG_20060924_104100-0130`

### 2.3 Stripping

In **stripping** (bare argument ellipsis) only one remnant is left, usually with a polarity or focus word (*not, too, also, only, just*):

(5) a. *Mary left, but not John.*
    b. *He's not against gays in the bedroom, just at the altar.* (UD guidelines, `_en/specific-syntax.md`, Gapping / Stripping, written in v1 labels: `remnant` = today's `orphan`)
    c. *GCP procedure is to create Counterparty names that reflect either the Tradename / Division name or Parent Name -- not both simultaneously.* — EWT 2.18 `email-enronsent41_01-0091`

In (5c) *both* is promoted to `conj` of the verb *reflect*, *not* is its `advmod`, and *simultaneously* is `orphan`; DEPS has an empty node 20.1 *reflect* with *both* as `obj`.

### 2.4 Mova's parser on gapping

Mova en 0.4 never produces `orphan` in these test sentences, and the parse database has only 22 `orphan` relations in 8.06 M words (against 29 in EWT's 0.25 M; 2026-10-17). Gapped clauses are forced into ordinary structures:

(6) *John likes tea and Mary coffee.* (Mova en 0.4, CPU, 2026-10-17)

```
1  John    PROPN  2  nsubj
2  likes   VERB   0  root
3  tea     NOUN   2  obj
4  and     CCONJ  6  cc
5  Mary    PROPN  6  compound    UD: conj(likes, Mary)
6  coffee  NOUN   3  conj        UD: orphan(Mary, coffee)
```

*Mary* becomes a modifier of *coffee* and *coffee* a second object of *likes*. The frame reader then says `Event john like patient=tea+coffee` and Mary disappears. In the parse database the same failure shows in the tales:

(7) *Into the castle he ran, and the dogs after him, …* — `book:pg3454:3491`: *dogs* is a second `nsubj` of *ran* and *after him* an `nmod` of *dogs*; UD would make *dogs* `conj` of *ran* with *him* as its `orphan`.

**Not yet**: the `en` parser has no empty nodes and no DEPS output; gapping is learned only from the few `orphan` arcs in its training data.

## 3. VP ellipsis

### 3.1 Form

In **VP ellipsis** a verb phrase is left out after an auxiliary, the infinitival *to*, or the copula; the auxiliary stays (seed [`verbal/vp-ellipsis.md`](../../en/seeds/verbal/vp-ellipsis.md); Bacskai-Atkari 2018, ch. 6.2.1 "VP-ellipsis revisited", `books/a/at/atkari-2018-deletion-phenomena-comparative-constructions`):

(8) a. *Mary didn't leave, John did.*
    b. *I can't, but she can.*
    c. *So please update whatever you need to.* (UD guidelines, `_en/specific-syntax.md`)
    d. *She has been to Rome and I have too.*
    e. *The grocer laughed, and so did the student.* — `book:pg5615:135` (tale, parse database)

### 3.2 UD analysis: the auxiliary is promoted

UD normally makes auxiliaries leaves, but when the verb is elided "the auxiliary inherits the head-status", and so does *to* (UD guidelines, `_en/specific-syntax.md`, VP ellipsis). The auxiliary keeps UPOS `AUX` (or `PART` for *to*) and takes the relation of the missing verb:

(9) *John will win gold and Mary will too.* (UD guidelines, `_en/dep/orphan.md`)

```
nsubj(win, John)
aux(win, will-2)
obj(win, gold)
conj(win, will-7)
cc(will-7, and)
nsubj(will-7, Mary)
advmod(will-7, too)
```

No `orphan` is used: the auxiliary is verbal material. The error to avoid is attaching the second auxiliary as `aux` of the first verb, which makes a long backward `aux` arc (seed [`errors/vp-ellipsis-aux-head.md`](../../en/seeds/errors/vp-ellipsis-aux-head.md); EWT 2.18 has 2 `aux` arcs after a VERB head, both inversions such as *Attached is the file*).

In (8e) the parse database has `conj(laughed, did)`, `nsubj(did, student)`, `advmod(did, so)` — right. In a history lesson the same pattern (*and so did ideas*, `school:history:054-china-the-silk-road-and-the-tang.lesson:4`) tags *did* as VERB instead of AUX: the structure is right, the part of speech is not.

### 3.3 Pseudogapping and predicate ellipsis in relatives

Two rarer types (UD guidelines, `_en/dep/acl-relcl.md`, Predicate Ellipsis in the Relative Clause):

(10) a. *She reads more novels than she does poems.* — pseudogapping: auxiliary + object remain
     b. *If we lose (which we probably will), I'm blaming you.* — the relative pronoun stands for a predicate: `advcl:relcl(lose, which)`, `aux(which, will)`

### 3.4 Mova on VP ellipsis

(11) *Mary can swim and John can too.* (Mova en 0.4, CPU, 2026-10-17): `conj(swim, can)`, `nsubj(can, John)` — right — but `advmod(swim, too)` instead of `advmod(can, too)`.

The frame reader makes `Fact(can) mary swim` and **no frame for John**: the promoted auxiliary is not a predicate it reads, and nothing copies *swim* from the antecedent (`world frames`, 2026-10-17). **Not yet**: resolution of VP ellipsis by copying the antecedent frame with the new subject.

## 4. Sluicing

In **sluicing** a wh-question clause is reduced to its wh-word (Ross 1969, wanted):

(12) a. *Someone called, but I don't know who.*
     b. *They came in under a lot of other quotes and now I know why!!!* — EWT 2.18 `reviews-043020-0018`
     c. *She is hiding something, but I can't tell what.*

UD has no dedicated analysis. EWT treats the wh-remnant as a dependent of the embedding verb with its own function: in (12b) `advmod(know, why)`. Mova en 0.4 gives (12a) `obj(know, who)` (CPU, 2026-10-17), consistent with that practice. The meaning — *who called* — is not recoverable from the tree; the frame reader takes *who* as an entity in the patient slot of *know*.

## 5. Noun-phrase ellipsis

The head noun is omitted after a determiner, numeral, possessive or adjective that stays:

(13) a. *I bought three apples and Tom bought two.*
     b. *Your idea is better than mine.* (possessive pronoun, no ellipsis in UD: *mine* is a PRON)
     c. *I like the red shirt more than the blue one.* (*one* is a pro-form, not ellipsis)
     d. *The rich get richer.* (adjective as head)

UD promotes the remaining dependent to the noun's relation: in (13a) *two* is `obj(bought, two)` with UPOS NUM. Mova en 0.4 parses (13a) exactly so (CPU, 2026-10-17). The seed [`nominal/adj-as-noun.md`](../../en/seeds/nominal/adj-as-noun.md) and the morphology seed [`morph/adj-as-noun.md`](../../en/seeds/morph/adj-as-noun.md) treat (13d).

## 6. Right-node raising

In **right-node raising** (RNR) coordinated clauses share a final element pronounced only in the last conjunct:

(14) a. *John bought and ate an apple.* (UD guidelines, `_en/dep/orphan.md`)
     b. *John bought, and Mary sold, the old car.*
     c. *I have never been and I will never be a Government official …* — EWT 2.18 `weblog-blogspot.com_alaindewitt_20060924104100_ENG_20060924_104100-0007`

The UD guideline attaches the shared element to the **first** verb: `obj(bought, apple)`, `conj(bought, ate)` (UD guidelines, `_en/dep/orphan.md`). The enhanced layer adds the second edge (`obj(ate, apple)`) by propagation of shared dependents. In (14c) the left conjunct lacks its predicate (*a Government official*), so its copula *been* is promoted to root, and *official* is `conj` of it (the English page calls this "promotion by head elision", `_en/specific-syntax.md`, Right-node raising).

Mova en 0.4 attaches the shared object in (14b) to the second verb, `obj(sold, car)` (CPU, 2026-10-17) — the nearer head, not the UD choice; only *Mary* sold the car in the frames.

## 7. Comparative deletion and comparative ellipsis

The clause after *than/as* in a comparison loses material in two ways (Bacskai-Atkari 2018, chs. 3–4 and 6; Bresnan 1973, wanted):

(15) a. *Mary is taller than Peter is.* — **comparative deletion**: the degree expression (*x tall*) is obligatorily absent; *\*than Peter is tall*.
     b. *Mary has more books than Peter has magazines.* — **comparative subdeletion**: only the degree operator is missing.
     c. *Mary is taller than Peter.* — further reduction: *than* + a nominal.
     d. *Gracee is more excited to see her than she is to see me!!!!* — EWT 2.18 `reviews-071278-0003`

Bacskai-Atkari (2018) argues that comparative deletion is triggered by a covert operator, while overt operators (as in non-standard *than what Peter is*) do not trigger it, and treats the variation inside and across languages with features rather than language-wide parameters (2018, ch. 1). The UD analysis of comparatives is in `_en/specific-syntax.md`, Comparatives, and in [22-phrases.md](22-phrases.md) §5: the *than*-clause or phrase is `advcl` or `obl` of the degree word, with *than* as `mark` or `case`; see [24-coordination-subordination.md](24-coordination-subordination.md). Deletion itself leaves no trace in the tree.

## 8. Fragments, answers and non-sentential utterances

Much of conversation consists of **fragments**: answers, confirmations, follow-up questions, exclamations (Ginzburg 2012, wanted; Schäfer 2025, on German topic drop, `books/s/sc/schafer-2025-licensing-usage-topic-drop`):

(16) a. A: *Where are you going?* B: *To Paris.*
     b. *Not yet.* — EWT 2.18 `email-enronsent30_01-0018` (`advmod(yet, Not)`, *yet* root)
     c. *Down, down, down.* — `book:pg11:27` (Alice's fall)
     d. *Why not wait for him …?* — EWT 2.18 `answers-20111108071348AAWu2FU_ans-0009`
     e. *Into the pot you go!* — `book:pg11339:722`

In UD the head of the fragment is the root; when several remnants of one missing predicate remain, `orphan` links them (`_en/specific-syntax.md`, Marginal disfluencies). **Topic drop** — leaving out a given subject or object at the start of an utterance (*Can't complain.* *Sounds good.* *Told you so.*) — is common in informal English and diaries; Schäfer (2025) studies German topic drop — a comparison, not English — separating its grammatical licensing from its usage, and gives an information-theoretic account of the usage (ch. 6; her experiments found no effect of verb surprisal); and Lemke, Schäfer & Reich (eds. 2024, *Information structure and information theory*, `books/r/re/reich-2024-information-structure-information-theory`) collect work that explains omissions by uniform information density: speakers shorten what is predictable (contraction, pronouns, ellipsis) and keep the information rate even.

In Mova's pragmatic labels (`$MOVA_DATA/parsedb/prag.tsv`, 2026-10-17) 24 505 of 311 101 sentences have form `frag` (7.9 %); the frame reader turns a fragment without a predicate into a `Bare` frame (e.g. an answer option «A frying-pan»).

## 9. Canonical word order and its variants

### 9.1 SVO and the order of complements

English is rigidly **subject–verb–object** in declarative main clauses; word order, not case, signals the grammatical relations (Sweet 1891, *A New English Grammar*, §§ on word order, `books/s/sw/sweet-1891-new-english-grammar-logical`; Jespersen 1924, *The Philosophy of Grammar*, `books/j/je/jespersen-1924-philosophy-grammar`). Weil (1844, *The Order of Words in the Ancient Languages Compared with That of the Modern Languages*, `books/w/we/weil-1844-order-words-ancient-languages`) contrasted languages where order follows the "march of ideas" from the known starting point to the new goal with languages, like English and French, where grammar fixes it — the first statement of the given-to-new principle (§11).

The order inside the clause:

| position | content | example |
|---|---|---|
| 1 | (fronted element, §9.6) | *Yesterday* |
| 2 | subject | *the fox* |
| 3 | auxiliaries, with *not* after the first; frequency adverbs | *had never* |
| 4 | lexical verb | *given* |
| 5 | indirect object | *the crow* |
| 6 | direct object | *a compliment* |
| 7 | particle (or before the object, §9.11) | |
| 8 | predicative / `xcomp` | |
| 9 | obliques: manner, place, time (typically in that order) | *in the forest that morning* |
| 10 | extraposed clauses, heavy NPs (§§9.8, 9.10) | |

UD relations do not depend on order: a fronted object is still `obj`, an inverted subject still `nsubj`. Order rules that assume canonical order (e.g. "`iobj` before `obj`") give false alarms in questions and fronting (seed [`verbal/questions-inversion.md`](../../en/seeds/verbal/questions-inversion.md)).

### 9.2 Subject–auxiliary inversion

The first auxiliary moves before the subject in (Jespersen 1914, MEG; Curme 1931, *Syntax*, `books/c/cu/curme-1931-grammar-english-language-vol`):

| context | example |
|---|---|
| yes/no and non-subject wh-questions | *Did you see it? What did you see?* ([26-questions.md](26-questions.md)) |
| fronted negative or restrictive phrase (§9.3) | *Never have I seen such a thing.* |
| conditionals without *if* | *Had I known, …; Were I you, …; Should you need help, …* |
| *so/neither/nor* + auxiliary | *so did the student; nor did he* |
| exclamatives (literary) | *Isn't it lovely! How happy were we then!* |
| wishes | *May you live long!* |
| after fronted *so/such* | *So great was his anger that …* |

The tree is unchanged: `aux(seen, have)`, `nsubj(seen, I)` with the auxiliary simply to the left of the subject. Mova en 0.4 parses «Never have I seen such a thing.» right: `advmod(seen, Never)`, `aux(seen, have)`, `nsubj(seen, I)` (CPU, 2026-10-17). Questions with do-support are harder: the tagger, trained mostly on statements, takes *did* for the main verb, so the frame reader puts questions into statement order before parsing (`docs/pragmatics.md`, Questions).

### 9.3 Negative and restrictive inversion

A fronted negative or restrictive phrase with clausal scope forces inversion; one with constituent scope does not ([27-negation.md](27-negation.md) §6):

(17) a. *Not only did Bush slack off …* — EWT 2.18 `weblog-juancole.com_juancole_20040823064025_ENG_20040823_064025-0033`
     b. *Neither shall thy name any more be called Abram …* — Torah (JPS 1917) `jps:GEN:418`: `advmod(called, Neither)`, `aux(called, shall)`, `nsubj:pass(called, name)`
     c. *Only then did she understand.*
     d. *Not long ago she left.* (constituent negation, no inversion)

### 9.4 Locative and directional inversion

With a fronted locative or directional phrase and an intransitive verb of motion or position, the subject can follow the **lexical** verb (no auxiliary needed):

(18) a. *From the AP comes this story:* — EWT 2.18 `weblog-blogspot.com_nominations_20041117172713_ENG_20041117_172713-0001`: `obl(comes, AP)`, `nsubj(comes, story)` — right.
     b. *Then came Amalek, and fought with Israel in Rephidim.* — Torah (JPS 1917) `jps:EXO:492`: `advmod(came, Then)`, `nsubj(came, Amalek)` — right in the parse database.
     c. *Into the room ran a small dog.*
     d. *Down the hill rolled the ball.*

The inverted subject is typically new and heavy; the fronted phrase links to the preceding context (Birner & Ward 1998, wanted). Mova en 0.4 parses (18c) with `obj(ran, dog)` (CPU, 2026-10-17) — the post-verbal noun is taken for an object, and the frame reader would find no agent for *ran*. When the subject is a pronoun there is no inversion: *Into the castle he ran* (`book:pg3454:3491`).

### 9.5 Quotative inversion

After a quotation the verb of saying may precede a nominal subject (not usually a pronoun): *"…," said the Hatter* vs *"…," he said*.

(19) *"Then you shouldn't talk," said the Hatter.* — `book:pg11:632`

```
6   talk    VERB   9  ccomp
9   said    VERB   0  root
11  Hatter  PROPN  9  nsubj
```

This is the EWT analysis: the verb of saying is the root, the quote its `ccomp`, the speaker its `nsubj` (cf. EWT 2.18 `email-enronsent07_01-0029`, *"I don't know how these events could have taken place …," said Dunn.*). Mova en 0.4 alone does worse on a quote that is itself an imperative: «"Go away," cried the Badger.» makes *Go* the root and *cried* its `conj` (CPU, 2026-10-17). The frame reader has a special rule for the speaker after a quote («"…" cries the Badger») so that the quote becomes what the Badger said (`docs/pragmatics.md`, Roles, more).

### 9.6 Fronting (topicalization)

An object or complement can be moved to the front for contrast or to link to the preceding discourse:

(20) a. *This book I have never read.*
     b. *Into the pot you go!* — `book:pg11339:722`
     c. *… for the father of a multitude of nations have I made thee.* — Torah (JPS 1917) `jps:GEN:418` (fronted predicative with inversion, an archaic pattern)
     d. *Silk, paper, porcelain and tea went west; horses, glass, grapes and cotton came east …* — `school:history:054-china-the-silk-road-and-the-tang.lesson:4` (no fronting; a parallel structure, cf. §12)

In UD a fronted object stays `obj`. Mova en 0.4 parses (20a) with *book* as `nsubj:pass` of *read* (CPU, 2026-10-17): a sentence-initial noun before a subject and a participle-like *read* looks like a passive subject to it. In (20c) the parse database makes *father* `obj` of *made* and *thee* a second `obj`; *the father …* is a predicative (`xcomp`) of *made* (cf. *He made them martyrs*, `_en/specific-syntax.md`, Resultatives).

### 9.7 Dislocation

A **dislocated** nominal stands at the periphery, outside the clause's core relations, usually resumed by a pronoun (left dislocation) or anticipated by one (right dislocation) (UD guidelines, `_en/dep/dislocated.md`):

(21) a. *The new Pakistani general, he's just been elected.* (UD guidelines, `_en/dep/dislocated.md`) — `dislocated(elected, general)`, `nsubj:pass(elected, he)`
     b. *anyways, the mezza luna: you should try it.* — EWT 2.18 `reviews-018548-0010`: `dislocated(try, luna)`, `obj(try, it)`
     c. *This is our office, me and Sam.* (right dislocation; UD guidelines) — `dislocated(office, me)`
     d. *My brother, he lives in Leeds.*

EWT 2.18 has 7 `dislocated`; the parse database 329 (2026-10-17), mostly from dialogue in tales. Mova en 0.4 gives (21d) two subjects, `nsubj(lives, brother)` and `nsubj(lives, he)` (CPU, 2026-10-17) — a violation of the one-subject rule (seed [`verbal/single-subject.md`](../../en/seeds/verbal/single-subject.md)) that the teacher loop's gates catch.

Dislocation differs from fronting (§9.6): the fronted element has a role in the clause and no resumptive pronoun; the dislocated one is doubled by a pronoun that carries the role.

### 9.8 Extraposition and *tough*-constructions

A clausal subject is usually moved to the end and replaced by expletive *it* (see also [21-clause-structure.md](21-clause-structure.md) §5.3; UD guidelines, `_en/specific-syntax.md`, *Tough*-constructions; seeds [`verbal/expletive-it.md`](../../en/seeds/verbal/expletive-it.md), [`errors/tough-vs-extraposition.md`](../../en/seeds/errors/tough-vs-extraposition.md)):

(22) a. *It is important to take the idea, not just the intellectual analysis.* — EWT 2.18 `newsgroup-groups.google.com_Meditation20052_06390a5f75b2e1f2_ENG_20050316_091700-0055`: `expl(important, It)`, `cop(important, is)`, `csubj(important, take)`
     b. *It is often said that politicians … must sometimes lie.* — `expl` + `csubj:pass` ([28-voice-and-passive.md](28-voice-and-passive.md) §7)
     c. *That he lied is obvious.* → *It is obvious that he lied.*

The ***tough*-construction** looks similar but the subject is the logical object of the infinitive:

(23) *The door is easy to use and it keeps the cold out during the winter.* — EWT 2.18 `reviews-389298-0002`: `nsubj(easy, door)`, `cop(easy, is)`, **`ccomp(easy, use)`**

Here the guidelines and the data disagree. The English specific-syntax page says that when the object is fronted, the clause "is no longer a `csubj`, being instead analyzed as an `xcomp`" (*This problem was hard for me to solve*: `xcomp(hard, solve)`); EWT 2.18 annotates it as `ccomp` (sometimes `advcl`), as the seed [`errors/tough-vs-extraposition.md`](../../en/seeds/errors/tough-vs-extraposition.md) records. `csubj` is impossible there because *door* is already the subject.

| sentence | UD guideline | EWT 2.18 | Mova en 0.4 (CPU, 2026-10-17) |
|---|---|---|---|
| *It is easy to please John.* | `expl` + `csubj(easy, please)` | `expl` + `csubj` | `expl` + **`xcomp`(easy, please)** |
| *John is easy to please.* | `nsubj` + `xcomp` | `nsubj` + `ccomp` | — |
| *It's hard to make money in this economy.* | `expl` + `csubj` | `expl` + `csubj` | — |

Mova's parser keeps *it* as `expl` but labels the extraposed clause `xcomp`, mixing the two constructions — the error that the seed's rule `en.verbal.expl-it-has-clause` is written to catch ("with an adjective with `expl` *it*, the infinitive is `csubj`, not `xcomp`"). BLiMP has minimal pairs for *tough* vs raising, on which language models also err (Warstadt et al. 2020, https://aclanthology.org/2020.tacl-1.25).

### 9.9 Existential *there*

*There* + *be* introduces a new entity, postponing the notional subject (clause structure in [21-clause-structure.md](21-clause-structure.md) §4.4; seed [`verbal/existential-there.md`](../../en/seeds/verbal/existential-there.md); UD guidelines, `_en/dep/expl.md`):

(24) a. *There is a ghost in the room.* — `expl(is, There)`; *is* is VERB (not AUX, not `cop`); `nsubj(is, ghost)`
     b. *There is a mouse in the kitchen.* — Mova en 0.4, CPU, 2026-10-17: `expl(is, There)`, *is* VERB root, `nsubj(is, mouse)`, `obl(is, kitchen)` — right
     c. *There seems to be a problem.* — `xcomp(seems, be)`, the notional subject on *be*
     d. *And there was evening and there was morning, one day.* — Torah (JPS 1917) `jps:GEN:898`

The verb agrees with the notional subject (*There **are** magicians*), though colloquial *there's* + plural is common. Existential *there* is the main device for **presentational** sentences: it lets a new, indefinite referent avoid the subject position, which English reserves for given material (§11).

The Torah trees show an inconsistency for the jussive *let there be*: in `jps:GEN:2` (*Let there be light*) *there* is `expl` of *Let* and *be* is `ccomp` of *Let*, while in `jps:GEN:15` (*Let there be lights in the firmament …*) *there* is `expl` of *be* and *be* is `xcomp` of *Let*. The second is the EWT-style analysis (*let* + bare infinitive as `xcomp`, the expletive on the existential verb).

EWT 2.18 has 458 `expl` *there* and 289 `expl` *it* (2026-10-17).

### 9.10 Heavy-NP shift

A long ("heavy") object can follow other complements, by the principle of **end weight** (Behaghel's law; Quirk et al. 1985, wanted):

(25) a. *She gave to the museum **all the letters her grandmother had written during the war**.*
     b. *I understand better **the risks that we face**.* (seed [`errors/adverb-placement.md`](../../en/seeds/errors/adverb-placement.md))

With a short object the same order is an error: *✗ I like very much football* (a typical learner error, WO in the ERRANT taxonomy; the seed reports 1.6 % of edits in W&I). UD relations are unaffected; only the linear order changes, which can create non-projective or long arcs that transition parsers find harder.

### 9.11 Particle placement

A phrasal verb's particle can precede or follow a nominal object, but must follow a pronoun:

(26) a. *He picked up the book.* / *He picked the book up.*
     b. *He picked it up.* / *✗ He picked up it.*
     c. *He picked up the book that had fallen behind the sofa.* (heavy object: particle first)

The particle is `compound:prt` in both orders. Mova en 0.4 parses both orders of (26a) right (CPU, 2026-10-17). The choice is information-structural: the object goes last when it is new or heavy, before the particle when it is given (Gries 2003, wanted).

### 9.12 Adverb placement

| adverb type | usual position | example |
|---|---|---|
| frequency, degree (*always, never, often, almost*) | before the lexical verb, after the first auxiliary | *She has **always** liked it.* |
| manner | after the object, or before the verb | *She read the letter **carefully**.* |
| place, time | end, or front for scene-setting | ***Yesterday** we went …* |
| sentence adverbs (*probably, fortunately*) | front, or after the first auxiliary | ***Fortunately**, nobody was hurt.* |
| focusing (*only, even, also*) | before the focused constituent | *She **only** drank tea.* |

An adverb between the verb and its object is ungrammatical except with heavy objects (§9.10). In UD all are `advmod`; *only/even* attach to the word they focus.

## 10. Information-packaging alternations: clefts, pseudo-clefts, passive, dative

### 10.1 *It*-clefts

An *it*-cleft splits a clause into a foregrounded constituent and a relative-like remainder: *It was **the cat** that ate the fish* (focus: *the cat*; presupposed: *something ate the fish*) (UD guidelines, `_en/dep/acl-relcl.md`, Clefts; Huddleston & Pullum 2002, ch. 16, wanted).

UD analysis: the foregrounded element is the root, *it* is `expl`, *be* is `cop`, and the remainder is an **adverbial** relative clause, `advcl:relcl` — not `acl:relcl`, because it does not modify the noun but the copular predication (UD guidelines, `_en/dep/advcl-relcl.md`, following CGEL p. 416):

(27) *It was Joseph Goebbels who said that if you repeat a lie often enough, people will believe it.* — EWT 2.18 `weblog-blogspot.com_alaindewitt_20060924104100_ENG_20060924_104100-0136`

```
1  It        PRON   3  expl         3:expl
2  was       AUX    3  cop          3:cop
3  Joseph    PROPN  0  root         0:root|6:nsubj
4  Goebbels  PROPN  3  flat         3:flat
5  who       PRON   6  nsubj        3:ref
6  said      VERB   3  advcl:relcl  3:advcl:relcl
```

The enhanced graph makes *Joseph* the subject of *said* and *who* a `ref` of it. The clefted element can be a PP or adverbial: *It was with John that I went to the movies* (`case(John, with)`, `advcl:relcl(John, went)`, `obl(went, that)`).

Mova en 0.4 parses «It was the cat that ate the fish.» with `nsubj(cat, It)` and `acl:relcl(cat, ate)` (CPU, 2026-10-17): the older analysis, which reads *it* as a referring subject. In the parse database, a long cleft from a tale — *It was the middle of winter, when the broad flakes of snow were falling around, that the queen … sat working at her window* (`book:pg2591:1254`) — also has *It* as `nsubj`. EWT 2.18 has 141 `advcl:relcl`, the parse database 473 (2026-10-17).

### 10.2 Pseudo-clefts

A pseudo-cleft puts a free relative (*what …*) in subject or predicate position (UD guidelines, `_en/dep/acl-relcl.md`, Pseudoclefts):

(28) a. *What follows is the text of an e-mail sent to me by a friend of mine.* — EWT 2.18 `weblog-blogspot.com_alaindewitt_20040929103700_ENG_20040929_103700-0015`: `nsubj(text, What)`, `acl:relcl(What, follows)`, `cop(text, is)`, *text* root
     b. *What she wanted was a cup of tea.* — Mova en 0.4 (CPU, 2026-10-17): `nsubj(cup, What)`, `acl:relcl(What, wanted)`, `cop(cup, was)` — right
     c. *What John did was to play tennis.* — when the foregrounded part is a clause, its predicate is the root and *What* is `nsubj:outer` (UD guidelines; `dialects/mova/seeds.md#clausal-predicate`)
     d. *John is who we want to help.* — reversed pseudo-cleft: `root(who)`, `nsubj(who, John)`

### 10.3 The passive and the dative alternation as packaging

The passive ([28-voice-and-passive.md](28-voice-and-passive.md) §13) and the dative alternation let the speaker choose which participant comes first and which last:

(29) a. *She gave **the dog** a bone.* — recipient given, theme new
     b. *She gave a bone **to the dog**.* — theme given, recipient new or heavy

Corpus studies show that the double-object construction is preferred when the recipient is given, pronominal, short and animate, and the prepositional dative when the theme is (Bresnan et al. 2007, wanted). UD marks the first as `iobj` + `obj`, the second as `obj` + `obl`. Mova en 0.4 parses (29a) with **two** `obj` (`obj(gave, dog)`, `obj(gave, bone)`; CPU, 2026-10-17) — the recipient should be `iobj` (seed [`verbal/indirect-object.md`](../../en/seeds/verbal/indirect-object.md)); the frame reader recovers the recipient through VerbNet ("the second object of a dative is the patient", `docs/pragmatics.md`).

| alternation | variant A | variant B | what moves to the end |
|---|---|---|---|
| active / passive | *The fox ate the cheese.* | *The cheese was eaten by the fox.* | the agent |
| dative | *gave the dog a bone* | *gave a bone to the dog* | recipient or theme |
| particle | *picked up the book* | *picked the book up* | particle or object |
| extraposition | *That he lied is obvious.* | *It is obvious that he lied.* | the clausal subject |
| existential | *A mouse is in the kitchen.* | *There is a mouse in the kitchen.* | the new subject |
| cleft | *The cat ate the fish.* | *It was the cat that ate the fish.* | the presupposed part |
| locative inversion | *A small dog ran into the room.* | *Into the room ran a small dog.* | the subject |

## 11. Information structure: given/new, topic, focus

### 11.1 The notions

**Information structure** is the packaging of a sentence's content for the hearer's current state of knowledge (Chafe 1976; Lambrecht 1994; Krifka 2008 — all wanted; overview in Song 2017, ch. 2–3, `books/s/so/song-2017-modeling-information-structure-cross`). The main dimensions:

| dimension | opposition | test / marker in English |
|---|---|---|
| **information status** (of referents) | given (active, accessible) vs new | pronouns and definites for given, indefinites for new |
| **topic** | what the sentence is about vs the comment on it | *As for X, …*; first position; dislocation |
| **focus** | the part that answers the current question vs the background | the nuclear pitch accent; clefts; *only, even* |
| **contrast** | focus or topic that evokes explicit alternatives | contrastive stress, *not X but Y*, clefts |
| **theme / rheme** (Prague School) | starting point vs goal of the message | linear order |

The idea that a sentence moves from a known starting point to a new goal goes back to Weil (1844), who distinguished the "point of departure" of the utterance, common to speaker and hearer, from its "goal" (`books/w/we/weil-1844-order-words-ancient-languages`).

### 11.2 Focus types

Following Gundel's distinction as presented by Song (2017, ch. 3), **semantic** (information) focus is the most prominent part, assigned to what answers the question, and affects truth conditions with focus-sensitive particles; **contrastive** focus always brings its alternatives into attention and is marked prosodically or by construction. Song (2017) argues, against Krifka (2008), for keeping contrast as a component of its own in a cross-linguistic representation, and models focus, topic and contrast as features in Minimal Recursion Semantics within the HPSG Grammar Matrix.

(30) a. Q: *What happened?* A: *[The DOG ate the cheese]F.* — broad (all-new, thetic) focus
     b. Q: *What did the dog eat?* A: *The dog ate [the CHEESE]F.* — narrow focus on the object
     c. Q: *Who ate the cheese?* A: *[The DOG]F ate it.* / *It was the DOG that ate it.* — narrow focus on the subject
     d. *The DOG ate the cheese, not the CAT.* — contrastive focus

### 11.3 English devices

English marks information structure mostly by **prosody** and **construction**, much less by word order than languages with free order:

| device | packages | chapter section |
|---|---|---|
| nuclear accent position | focus | §13 |
| subject position | topic, given | §9.1 |
| pronoun vs full NP | given vs new | [35-reference-and-coreference.md](35-reference-and-coreference.md) |
| definite vs indefinite article | identifiable vs new | 1x chapters (determiners) |
| existential *there*, locative inversion | new entity, presentational | §§9.4, 9.9 |
| fronting, left dislocation | topic shift, contrastive topic | §§9.6, 9.7 |
| *it*-cleft, pseudo-cleft | narrow or contrastive focus | §10 |
| passive, dative, particle alternations | given before new; end weight | §10.3 |
| *only, even, also, too* | focus-sensitive particles | §9.12 |
| *as for, regarding, speaking of* | topic introduction | — |

### 11.4 Given before new, end weight, end focus

Three principles converge on the same order (Quirk et al. 1985, wanted; Behaghel 1909/1932, wanted):

- **Given before new**: referents already in the discourse come first (Weil 1844; the Prague School's communicative dynamism).
- **End weight**: long constituents come last (§9.10).
- **End focus**: the default nuclear accent falls on the last content word, so the end of the clause is the default focus position.

Since given material is usually short (pronouns) and new material long, the principles mostly agree. When they conflict, English uses a construction — passive, cleft, *there* — rather than free reordering. Portele & Bader (in Lemke, Schäfer & Reich 2024) compare an accessibility account with uniform information density for the choice and order of referring expressions in German and find that speakers pronominalize the previous topic rather than the most expected referent — evidence for accessibility.

## 12. The question under discussion (QUD)

### 12.1 The model

A discourse can be seen as a sequence of answers to (mostly implicit) questions: the **question under discussion** (Roberts 1996/2012, wanted; von Stutterheim & Klein 1989). The focus of a sentence is the part that answers the current QUD; the rest is background. Riester, Brunetti & De Kuthy (2018) turned this into annotation guidelines with three constraints for implicit QUDs (quoted from the authors' manuscript, ms. pp. 10–13; citation sheet in the research library, `docs/papers/qud-constraints.md`; DOI https://doi.org/10.1075/slcs.199.14rie):

| constraint | gist |
|---|---|
| **Q-A-Congruence** | "QUDs must be answerable by the assertion(s) that they immediately dominate." |
| **Maximize-Q-Anaphoricity** | "Implicit QUDs should contain as much given material as possible." |
| **Q-Givenness** | "Implicit QUDs can only consist of given (or, at least, highly salient) material." |

At the start of a text, or when nothing given can form a question, the QUD is a "Big Question" such as *What happened?* — the all-focus (thetic) case. Parallel assertions share a QUD whose constant part recurs and whose varying part is the wh-phrase; with two varying positions, a superquestion splits into subquestions (contrastive topic + focus):

(31) *Silk, paper, porcelain and tea went west; horses, glass, grapes and cotton came east, and so did ideas, like Buddhism from India.* — `school:history:054-china-the-silk-road-and-the-tang.lesson:4`
     Superquestion: {What went in which direction?} > {What went west?} *[Silk, paper, porcelain and tea]F went west* > {What came east?} *[horses, glass, grapes and cotton]F came east*; *west/east* are contrastive topics.

Non-at-issue content — parentheticals, non-restrictive modifiers, evidentials — is optional with respect to the QUD (Riester et al. 2018, §6): *like Buddhism from India* in (31) does not answer it.

### 12.2 QUD and syntax

The QUD connects information structure to the constructions above: an *it*-cleft answers a *who/what*-question and presupposes its background; a fronted contrastive topic signals a subquestion strategy; existential *there* answers *What is there?*; ellipsis is licensed when the elided material is given by the QUD (sluicing: *Someone called, but I don't know who [called]*).

## 13. Prosody

Spoken English marks focus mainly by the placement of the **nuclear pitch accent**; givenness by deaccenting (Ladd 2008, wanted). Written text has no direct marking except typography (capitals, italics) and the constructions of §§9–10.

(32) a. *I didn't take your BOOK.* (I took something else)
     b. *I didn't take YOUR book.* (I took someone else's)
     c. *I DIDN'T take your book.* (denial: verum focus)

Samek-Lodovici (2015, *The Interaction of Focus, Givenness, and Prosody: A Study of Italian Clause Structure*, OUP; card only, `books/l/lo/lodovici-2015-interaction-focus-givenness-prosody`) argues, for Italian, that word-order effects attributed to focus follow from prosodic requirements on givenness and stress. Fliessbach (2023, *The intonation of expectations*, Language Science Press, CC BY 4.0, `books/f/fl/fliessbach-2023-intonation-expectations-marked-declaratives`) shows for Spanish that intonation of declaratives encodes the speaker's expectations about the hearer's beliefs — the territory of English rising declaratives (*You're leaving?*) and of biased questions ([26-questions.md](26-questions.md)). Mova reads text only; prosody enters as punctuation (*?!*, capitals) — the irony module counts ALL-CAPS words as a pragmatic cue ([`prag/src/irony.rs`](../../prag/src/irony.rs)).

## 14. Quick reference

| construction | example | UD (basic) | EUD |
|---|---|---|---|
| gapping | *A went to P and B to Q* | `conj(went, B)`, `orphan(B, Q)` | empty node `went` with `nsubj`, `obl:to` |
| stripping | *A left, but not B* | `conj(left, B)`, `advmod(B, not)` | empty node |
| VP ellipsis | *A will win and B will too* | `conj(win, will)`, `nsubj(will, B)` | — |
| sluicing | *I don't know who* | wh-word as dependent of the verb | — |
| NP ellipsis | *Tom bought two* | `obj(bought, two)` | — |
| RNR | *A bought and ate an apple* | `obj(bought, apple)` | + `obj(ate, apple)` |
| fragment | *Not yet.* | remnant is root | — |
| subject–aux inversion | *Never have I seen* | relations as in the statement | — |
| locative inversion | *From the AP comes this story* | `obl` + `nsubj` after the verb | — |
| quotative inversion | *"…," said the Hatter* | `ccomp(said, quote)`, `nsubj(said, Hatter)` | — |
| fronting | *This book I never read* | `obj(read, book)` | — |
| dislocation | *The general, he's been elected* | `dislocated(elected, general)` | — |
| extraposition | *It is easy to please John* | `expl`, `csubj(easy, please)` | — |
| *tough* | *The door is easy to use* | `nsubj`, `xcomp` (guideline) / `ccomp` (EWT) | — |
| existential | *There is a ghost* | `expl(is, There)`, *is* VERB, `nsubj(is, ghost)` | — |
| *it*-cleft | *It was J. who said* | `expl`, `cop`, `advcl:relcl(J., said)` | `nsubj(said, J.)`, `ref(J., who)` |
| pseudo-cleft | *What she wanted was tea* | `nsubj(tea, What)`, `acl:relcl(What, wanted)` | — |

---

## 15. In Mova

### 15.1 Parsing (Mova en 0.4, CPU, 2026-10-17)

| sentence | Mova en 0.4 | UD | verdict |
|---|---|---|---|
| *John likes tea and Mary coffee.* | compound(coffee, Mary), conj(tea, coffee) | conj(likes, Mary), orphan(Mary, coffee) | wrong |
| *Mary can swim and John can too.* | conj(swim, can), advmod(swim, too) | advmod(can, too) | half right |
| *Someone called, but I don't know who.* | obj(know, who) | EWT-like | acceptable |
| *I bought three apples and Tom bought two.* | obj(bought, two) | same | right |
| *John bought, and Mary sold, the old car.* | obj(sold, car) | obj(bought, car) | wrong head |
| *Into the room ran a small dog.* | obj(ran, dog) | nsubj(ran, dog) | wrong |
| *"Go away," cried the Badger.* | Go root, conj(Go, cried) | ccomp(cried, Go), cried root | wrong |
| *This book I have never read.* | nsubj:pass(read, book) | obj(read, book) | wrong |
| *My brother, he lives in Leeds.* | two nsubj | dislocated(lives, brother) | wrong |
| *It is easy to please John.* | expl + xcomp | expl + csubj | wrong label |
| *It was the cat that ate the fish.* | nsubj(cat, It), acl:relcl | expl, advcl:relcl | wrong |
| *What she wanted was a cup of tea.* | nsubj(cup, What), acl:relcl(What, wanted) | same | right |
| *There is a mouse in the kitchen.* | expl, *is* VERB, nsubj | same | right |
| *He picked up the book. He picked the book up.* | compound:prt both | same | right |
| *She gave the dog a bone.* | obj + obj | iobj + obj | wrong label |
| *Never have I seen such a thing.* | advmod, aux, nsubj | same | right |

The pattern is clear: the parser handles the constructions that keep canonical relations in non-canonical order when they are frequent in its training data (inversion after negatives, particles, existentials, pseudo-clefts), and fails on those that need a relation it rarely sees (`orphan`, `dislocated`, `advcl:relcl`, `csubj` with `expl`) or a subject after the verb (locative and quotative inversion with a quote as head). The seeds that check these in the teacher loop: [`verbal/gapping-orphan.md`](../../en/seeds/verbal/gapping-orphan.md), [`verbal/vp-ellipsis.md`](../../en/seeds/verbal/vp-ellipsis.md), [`errors/vp-ellipsis-aux-head.md`](../../en/seeds/errors/vp-ellipsis-aux-head.md), [`errors/gapping-orphan.md`](../../en/seeds/errors/gapping-orphan.md), [`verbal/expletive-it.md`](../../en/seeds/verbal/expletive-it.md), [`errors/tough-vs-extraposition.md`](../../en/seeds/errors/tough-vs-extraposition.md), [`verbal/existential-there.md`](../../en/seeds/verbal/existential-there.md), [`verbal/questions-inversion.md`](../../en/seeds/verbal/questions-inversion.md), [`errors/adverb-placement.md`](../../en/seeds/errors/adverb-placement.md), [`verbal/single-subject.md`](../../en/seeds/verbal/single-subject.md).

### 15.2 Frames and discourse

- **Ellipsis in frames** ([`prag/src/frames.rs`](../../prag/src/frames.rs)): gapped and VP-elided clauses produce no frame of their own (`world frames`, 2026-10-17: «Mary can swim and John can too.» → only `Fact(can) mary swim`; «John likes tea and Mary coffee.» → `Event john like patient=tea+coffee`). Bare fragments become `Bare` frames.
- **Word order in the reader**: «because / so» give `cause`, «and / then» and sentence order give `next`; consecutive sentences are linked in reading order, so order of mention stands for order of events (`docs/pragmatics.md`, Links). The implicature module draws the "be orderly" inference — two coordinated event verbs happened in the order told — unless a reordering word (*after, before, while*) or a stative verb is present ([`prag/src/infer.rs`](../../prag/src/infer.rs)).
- **Quotative inversion**: the speaker after a quote («"…" cries the Badger») is taken as the speaker of the Say frame.
- **Questions**: do-support and modal questions are put into statement order before parsing (`docs/pragmatics.md`, Questions).
- **Salience and givenness**: pronouns are resolved in word order by salience — subject > object > other, halved per sentence — so the topic of the previous sentence is the preferred antecedent (`prag::frames::Ctx`; `docs/pragmatics.md`; [35-reference-and-coreference.md](35-reference-and-coreference.md)). Information status is otherwise not represented.
- **QUD** ([`world/src/qud.rs`](../../world/src/qud.rs)): a stack of questions for stories and dialogues after Roberts (2012). Explicit questions are pushed; an assertion attaches to the most recent open question with a shared content lemma and an answer of the right type (who → an animate subject or object, where → a place oblique, when → a time word, why → *because* or an `advcl`, …); a complete answer pops the question. With no congruent question an **implicit QUD** is accommodated from given lemmas and the predicate's shape (*What did the fox do?*, *What happened next?*), after Riester et al. (2021). Givenness is checked as in QUDeval (Wu et al. 2023): a question may use only lemmas from earlier sentences, and a lemma new in the answer is reported as answer leakage. The stack and the reasons are the explanation; the module is deterministic.

### 15.3 Not yet

- No empty nodes or DEPS; no `orphan`; no reconstruction of gapped or VP-elided frames from their antecedents.
- No focus/background or topic marking in frames; no use of clefts, fronting or *there* as information-structure signals.
- Dislocated nominals and fronted objects are misparsed as subjects, so frames get wrong agents.
- *Tough* vs extraposition is not distinguished (both end up `xcomp`).
- The QUD module works on its own pre-annotated tokens and is not yet wired into the pragmatic student.

## 16. Sources

- Bacskai-Atkari, Julia. 2018. *Deletion Phenomena in Comparative Constructions: English Comparatives in a Cross-Linguistic Perspective.* Language Science Press, CC BY 4.0. https://zenodo.org/records/1211896; `books/a/at/atkari-2018-deletion-phenomena-comparative-constructions`.
- Curme, George O. 1931. *A Grammar of the English Language, vol. III: Syntax.* `books/c/cu/curme-1931-grammar-english-language-vol`.
- Fliessbach, Jan. 2023. *The Intonation of Expectations: On Marked Declaratives, Exclamatives, and Discourse Particles in Castilian Spanish.* Language Science Press, CC BY 4.0. `books/f/fl/fliessbach-2023-intonation-expectations-marked-declaratives`.
- Jespersen, Otto. 1914. *A Modern English Grammar on Historical Principles*, part III. `books/j/je/jespersen-1914-modern-english-grammar-historical`.
- Jespersen, Otto. 1924. *The Philosophy of Grammar.* `books/j/je/jespersen-1924-philosophy-grammar`.
- Lemke, Robin, Lisa Schäfer and Ingo Reich (eds.). 2024. *Information Structure and Information Theory.* Language Science Press (Topics at the Grammar-Discourse Interface), CC BY 4.0. https://zenodo.org/records/12784266; `books/r/re/reich-2024-information-structure-information-theory`. (Introduction; Portele & Bader, "Choosing referential expressions and their order".)
- Samek-Lodovici, Vieri. 2015. *The Interaction of Focus, Givenness, and Prosody: A Study of Italian Clause Structure.* Oxford University Press. `books/l/lo/lodovici-2015-interaction-focus-givenness-prosody` (card only).
- Riester, Arndt, Lisa Brunetti and Kordula De Kuthy. 2018. Annotation guidelines for Questions under Discussion and information structure. In E. Adamou, K. Haude, M. Vanhove (eds.), *Information Structure in Lesser-Described Languages*, John Benjamins, 403–443. https://doi.org/10.1075/slcs.199.14rie; manuscript hal-01794160v2; library card `papers/r/ri/rie/riester-2018-annotation-guidelines-questions-under`; citation sheet `docs/papers/qud-constraints.md` in the research library.
- Schäfer, Lisa. 2025. *The Licensing and Usage of Topic Drop in German.* Language Science Press (Open Germanic Linguistics), CC BY 4.0. `books/s/sc/schafer-2025-licensing-usage-topic-drop`.
- Schuster, Sebastian and Christopher D. Manning. 2016. Enhanced English Universal Dependencies: An improved representation for natural language understanding tasks. LREC. https://aclanthology.org/L16-1376.
- Song, Sanghoun. 2017. *Modeling Information Structure in a Cross-Linguistic Perspective.* Language Science Press (Topics at the Grammar-Discourse Interface), CC BY 4.0. https://zenodo.org/records/818365; `books/s/so/song-2017-modeling-information-structure-cross`.
- Sweet, Henry. 1891. *A New English Grammar, Logical and Historical.* `books/s/sw/sweet-1891-new-english-grammar-logical`.
- Universal Dependencies. Guidelines, snapshot 2026-09-24: `_en/dep/orphan.md` (https://universaldependencies.org/en/dep/orphan.html), `_en/dep/dislocated.md` (https://universaldependencies.org/en/dep/dislocated.html), `_en/dep/expl.md`, `_en/dep/acl-relcl.md` (clefts, pseudoclefts, predicate ellipsis), `_en/dep/advcl-relcl.md`, `_en/specific-syntax.md` (VP ellipsis, gapping/stripping, right-node raising, marginal disfluencies, *tough*-constructions, comparatives), `_u-overview/specific-syntax.md` (ellipsis, promotion), `_u-overview/enhanced-syntax.md` (empty nodes).
- UD English EWT 2.18 (Silveira et al. 2014, https://aclanthology.org/L14-1067; CC BY-SA 4.0). Sentences cited by `sent_id`.
- Warstadt, A. et al. 2020. BLiMP: The Benchmark of Linguistic Minimal Pairs for English. TACL 8. https://aclanthology.org/2020.tacl-1.25.
- Weil, Henri. 1844. *The Order of Words in the Ancient Languages Compared with That of the Modern Languages.* Transl. C. W. Super, Boston: Ginn, 1887. Public domain. https://archive.org/details/orderofwordsinan00weiluoft; `books/w/we/weil-1844-order-words-ancient-languages`.
- Roberts 2012 and Riester et al. 2021 (QUD), Wu et al. 2023 (QUDeval): as cited in [`world/src/qud.rs`](../../world/src/qud.rs).
- Mova: seeds listed in §15.1; [`dialects/mova/seeds.md#clausal-predicate`](../../dialects/mova/seeds.md#clausal-predicate); code [`prag/src/frames.rs`](../../prag/src/frames.rs), [`prag/src/infer.rs`](../../prag/src/infer.rs), [`prag/src/irony.rs`](../../prag/src/irony.rs), [`world/src/qud.rs`](../../world/src/qud.rs); [`docs/pragmatics.md`](../../docs/pragmatics.md).
- Wanted: Ross 1969 (sluicing); Bresnan 1973 (comparative deletion); Merchant 2001 *The Syntax of Silence*; Ginzburg 2012 *The Interactive Stance*; Birner & Ward 1998 *Information Status and Noncanonical Word Order in English*; Lambrecht 1994 *Information Structure and Sentence Form*; Chafe 1976; Krifka 2008 "Basic notions of information structure"; Roberts 1996/2012 "Information structure in discourse"; Quirk et al. 1985 *A Comprehensive Grammar of the English Language*; Huddleston & Pullum 2002 (ch. 16); Bresnan, Cueni, Nikitina & Baayen 2007 "Predicting the dative alternation"; Gries 2003 *Multifactorial Analysis in Corpus Linguistics: particle placement*; Ladd 2008 *Intonational Phonology*; Behaghel 1932 *Deutsche Syntax IV* (law of increasing constituents).
