# 25. Relative clauses

A relative clause modifies a head — usually a noun — that is understood to fill a gap inside the clause: *the book [that I read ___]*. This chapter describes English relative clauses: the relativizers (*who, whom, whose, which, that*, the WH-adverbs and zero), the roles the gap can have, pied-piping and preposition stranding, reduced and infinitival relatives, free relatives, restrictive and non-restrictive uses, clefts and pseudoclefts, and agreement. For each it gives the Universal Dependencies analysis (`acl:relcl`, `advcl:relcl`, `PronType=Rel`, and the enhanced `ref` edge with the antecedent taking the gap's role), where the English guidelines, EWT, Mova's dialect and Mova's parser differ, and how Mova's pragmatics reads relatives into frames.

Contents

1. [Overview](#1-overview)
2. [The basic analysis: `acl:relcl` and the relativizer's role](#2-the-basic-analysis-aclrelcl-and-the-relativizers-role)
3. [Relativizers](#3-relativizers)
4. [Which role is relativized: the accessibility of gaps](#4-which-role-is-relativized-the-accessibility-of-gaps)
5. [Pied-piping and preposition stranding](#5-pied-piping-and-preposition-stranding)
6. [Reduced and infinitival relatives](#6-reduced-and-infinitival-relatives)
7. [Free relatives](#7-free-relatives)
8. [Restrictive and non-restrictive; clefts and pseudoclefts](#8-restrictive-and-non-restrictive-clefts-and-pseudoclefts)
9. [Relatives versus other modifier clauses](#9-relatives-versus-other-modifier-clauses)
10. [The enhanced graph: `ref` and the gap's role](#10-the-enhanced-graph-ref-and-the-gaps-role)
11. [Agreement and case](#11-agreement-and-case)
12. [Mova's parser on relative clauses](#12-movas-parser-on-relative-clauses)
13. [From relative clauses to frames](#13-from-relative-clauses-to-frames)

Cross-references: content clauses on nouns (*the fact that …*) in [23-complementation.md](23-complementation.md) §10; sentential relatives and `advcl:relcl` as clause linking in [24-coordination-subordination.md](24-coordination-subordination.md) §11; wh-questions, which share the wh-words, in [26-questions.md](26-questions.md); clefts as information packaging in [29-ellipsis-word-order-information-structure.md](29-ellipsis-word-order-information-structure.md); relatives and reference in [35-reference-and-coreference.md](35-reference-and-coreference.md).

---

## 1. Overview

| type | example | UD relation of the clause | relativizer |
|---|---|---|---|
| finite, with WH-pronoun | *the man **who** lives next door* | `acl:relcl` | PRON, `PronType=Rel`, in its gap role |
| finite, with *that* | *the book **that** I read* | `acl:relcl` | PRON (WDT), in its gap role |
| finite, zero ("contact clause") | *the man I saw* | `acl:relcl` | none |
| WH-adverb | *the time **when** the pizza exploded* | `acl:relcl` | ADV, `advmod` |
| infinitival | *a bagel to eat*; *a house in which to live* | `acl:relcl` | usually none |
| participial ("reduced") | *people living in glass houses* | `acl` (not a relative in UD) | none |
| free (fused) | *whatever she's having*; *where you were sitting* | the WH-word heads; `acl:relcl` / `advcl:relcl` | fused with the head |
| sentential | *…, which was a bad idea* | `advcl:relcl` | *which* |
| *it*-cleft | *It's John who we want to help* | `advcl:relcl` | any, or zero |

The UD English relative-clause page is a long, recent document marked "under revision", worked out by the UD core group (UD guidelines, `_en/dep/acl-relcl.md`, https://universaldependencies.org/en/dep/acl-relcl.html). It follows *CGEL* (Huddleston and Pullum 2002, *The Cambridge Grammar of the English Language*, ch. 12) on most points, but keeps the traditional analysis of relative *that* as a pronoun where *CGEL* calls it a subordinator.

EWT 2.18 counts (train + dev + test, 2026-10-17): `acl:relcl` 2 341, `advcl:relcl` 141, `acl` 1 831. Every `acl:relcl` stands to the right of its head (2 341 of 2 341; seed [`en/seeds/nominal/relcl-after-head.md`](../../en/seeds/nominal/relcl-after-head.md)).

Traditional grammars give the same distinctions in other words: Curme (1931, *Syntax*, ch. XIV "Attributive Relative Clause", pp. 204–238, `books/c/cu/curme-1931-grammar-english-language-vol`) has sections on *who / what / which*, descriptive and restrictive clauses, personality and form, case and agreement, position, asyndetic (zero) relatives and "abridgment" (reduced relatives); Poutsma (*A Grammar of Late Modern English*, vol. 4, ch. XXXIX, `books/p/po/poutsma-1904-grammar-late-modern-english`, text-4) treats *that* and zero in detail.

---

## 2. The basic analysis: `acl:relcl` and the relativizer's role

The predicate of the relative clause attaches to the head noun as `acl:relcl`. A relative **pronoun** fills the gap's role inside the clause (`nsubj`, `obj`, `obl`, `nmod:poss`); a relative **adverb** is `advmod` inside the clause (UD guidelines `_en/dep/acl-relcl.md`):

```
# text = It has an ideology which claims supremacy over Western culture.
#        (EWT 2.18 weblog-blogspot.com_alaindewitt_20060924104100_ENG_20060924_104100-0221)
ID FORM       UPOS HEAD DEPREL     DEPS
4  ideology   NOUN 2    obj        2:obj|6:nsubj
5  which      PRON 6    nsubj      4:ref
6  claims     VERB 4    acl:relcl  4:acl:relcl
7  supremacy  NOUN 6    obj        6:obj
```

The basic tree keeps *which* inside the clause as `nsubj(claims, which)`; the enhanced graph (§10) moves the subject role onto *ideology* and links *which* to its antecedent by `ref`.

Relativization is unbounded: the gap can be several clauses down.

(1) *I saw the book which you pretended to think was boring.* — `acl:relcl(book, pretended)`, `xcomp(pretended, think)`, `ccomp(think, boring)`, `nsubj(boring, which)` (UD guidelines `_en/dep/acl-relcl.md`, "Notable properties").

Relative clauses can also have their predicate elided: *If we lose (which we probably will), I'm blaming you* — the pronoun *which* stands for the predicate and is promoted to head: `advcl:relcl(lose, which)`, `aux(which, will)` (same page, "Predicate ellipsis").

Mova's dialect follows the guidelines unchanged here; the seeds that check the analysis:

- the relative pronoun is inside the clause, never attached to the antecedent ([`en/seeds/errors/relative-pronoun-in-clause.md`](../../en/seeds/errors/relative-pronoun-in-clause.md));
- a finite `acl:relcl` on a noun has a subject: its own or the relativizer (EWT 2.18: only 2 of 990 lack one, both text glitches; [`en/seeds/nominal/relcl-subject.md`](../../en/seeds/nominal/relcl-subject.md));
- relative *that* is PRON, never `mark` (EWT 2.18: 1 exception; [`en/seeds/nominal/relative-that.md`](../../en/seeds/nominal/relative-that.md)).

---

## 3. Relativizers

### 3.1 The inventory

| relativizer | UPOS / XPOS | features | antecedent | restrictive | non-restrictive | after a preposition |
|---|---|---|---|---|---|---|
| *who* | PRON / WP | `PronType=Rel`; Mova adds `Case=Nom` or `Acc` | persons, personified animals | yes | yes | rare (*to who*), colloquial |
| *whom* | PRON / WP | `PronType=Rel`, `Case=Acc`; lemma *who* in Mova | persons | yes (formal) | yes | yes: *to whom* |
| *whose* | PRON / WP$ | `PronType=Rel`, `Poss=Yes`; Mova `Case=Gen` | persons and things | yes | yes | yes: *from whose crown* |
| *which* | PRON / WDT (DET when it has a noun) | `PronType=Rel` | things, clauses | yes | yes | yes: *in which* |
| *that* | PRON / WDT | `PronType=Rel` | persons and things | yes | archaic | no: *\*in that I live* |
| zero | — | — | persons and things | yes | no | stranded only |
| *where / when / why / how* | ADV / WRB | `PronType=Rel` | places, times, reasons, manners | yes | yes | *from where* |
| *as* (dialectal, *such … as*) | — | | | *such people as I know* | | |
| *what* (dialectal, non-standard) | PRON | | | *the man what I saw* | | |

Sources: UD guidelines `_en/dep/acl-relcl.md` and `_en/feat/PronType.md`; seeds [`who-whom-whose.md`](../../en/seeds/nominal/who-whom-whose.md) and [`relative-that.md`](../../en/seeds/nominal/relative-that.md) (Poutsma vol. 4, ch. XXXIX §§4, 9–12, 16–17, 34; Curme 1931, ch. XIV §23 II 7–8); for dialect *as* and *what*, Wright (1905, *The English Dialect Grammar*, `books/w/wr/wright-1905-english-dialect-grammar-comprising`).

### 3.2 *That*: pronoun or subordinator?

*CGEL* treats relative *that* as a subordinator (it cannot be pied-piped, has no case, and also introduces content clauses). UD keeps it a relative pronoun "roughly interchangeable with *which*", filling the gap's role (UD guidelines `_en/dep/acl-relcl.md`, footnote 1). The practical difference for annotation: content-clause *that* (*the fact that he left*) is SCONJ `mark`, relative *that* (*the fact that he mentioned*) is PRON in a role ([23-complementation.md](23-complementation.md) §10.2).

Relative *that* in EWT 2.18 by role: `nsubj` 379, `obj` 134, `nsubj:pass` 63, `obl` with a stranded preposition (seed `relative-that.md`). A `case` on relative *that* is always to its right (30 of 30): the preposition is stranded, never pied-piped.

### 3.3 Zero relatives

The relativizer can be omitted when the gap is not the subject: an object (*the man [whom] I saw*), the object of a stranded preposition (*the house [that] I live in*), a predicative (*the man [that] he is*) (Curme 1931, ch. XIV §23 II 10 "Asyndetic Relative Construction", p. 233; seed [`relcl-subject.md`](../../en/seeds/nominal/relcl-subject.md)).

(2) *Do you also have a SSD we can use?* — `acl:relcl(SSD, use)`; enhanced `9:obj` on *SSD* (EWT 2.18 email-enronsent03_01-0040).
(3) *Is there anyone else we should tell?* — `acl:relcl(anyone, tell)` (EWT 2.18 email-enronsent40_01-0014).

Subject zero relatives occur in colloquial and dialect speech after *there is / here is / it is*: *There's a man wants to see you* (Poutsma vol. 4, ch. XXXIX §29). They break the subject rule of the seed and are flagged, not corrected.

---

## 4. Which role is relativized: the accessibility of gaps

Any nominal role can be relativized in English, including possessors and objects of prepositions — high on Keenan and Comrie's accessibility hierarchy (subject > direct object > indirect object > oblique > genitive > object of comparison; Keenan and Comrie 1977, "Noun Phrase Accessibility and Universal Grammar", *Linguistic Inquiry* 8(1); not open, wanted).

| gap role | example | relativizer's relation (basic) | enhanced arc on the antecedent |
|---|---|---|---|
| subject | *the man **who** lives next door* | `nsubj(lives, who)` | `nsubj` from *lives* |
| passive subject | *the price **that** is quoted* | `nsubj:pass` | `nsubj:pass` |
| object | *the book **that** I read* | `obj(read, that)` | `obj` |
| indirect object | rare: *the man (that) I gave the book* | `iobj` | `iobj` |
| oblique | *the house **in which** you live* | `obl(live, which)`, `case(which, in)` | `obl` (EWT: without the preposition in the label, see §10) |
| possessor | *the boy **whose** dog barked* | `nmod:poss(dog, whose)` | `nmod:poss` from *dog* |
| object of a nominal's preposition | *a crown, the jewels **of which** were stolen* | `nmod(jewels, which)`, `case(which, of)` | `nmod` |
| adverbial (time, place) | *the time **when** the pizza exploded* | `advmod(exploded, when)` | `obl` |
| predicative | *the man (that) he is* | `nsubj`/root of a copular clause | — |

(4) *3 = Number of 11 September hijackers whose entry visas came through special US-Saudi "Visa Express" programme.* — `nmod:poss(visas, whose)`, enhanced `ref(hijackers, whose)` (EWT 2.18 newsgroup-groups.google.com_alt.animals_0084bdc731bfc8d8_ENG_20040905_212000-0054).
(5) *That would require someone who had access to the strain.* — `nsubj(had, who)`, `ref(someone, who)` (EWT 2.18 weblog-blogspot.com_dakbangla_20050311135387_ENG_20050311_135387-0041).
(6) *He thinks he is «a heaven-born driver» whom nobody can teach.* — `obj(teach, whom)` (`$MOVA_DATA/parsedb/current.conllu`, school:books:038-willows-ch4-part2-toad-news-and-hedgehogs.lesson:17).
(7) *"I am the man on whom many persons fix their hopes.* — `obl(fix, whom)`, `case(whom, on)` (book:pg27200:6716).

---

## 5. Pied-piping and preposition stranding

When the gap is the object of a preposition, the preposition either moves with the relativizer to the front (**pied-piping**, formal) or stays at the gap (**stranding**, informal). Only *which*, *whom*, *whose* and WH-adverbs pied-pipe; *that* and zero only strand (Poutsma vol. 4, ch. XXXIX §34, via [`relative-that.md`](../../en/seeds/nominal/relative-that.md)).

### 5.1 Pied-piping

The preposition is `case` of the relativizer, which is `obl` (or `nmod`) of the clause's predicate (UD guidelines `_en/dep/acl-relcl.md`, "Prepositional relatives"):

```
# text = The unit of measure against which the price is quoted shall be US Dollars.
#        (EWT 2.18 email-enronsent36_02-0081)
ID FORM     UPOS HEAD DEPREL     DEPS
2  unit     NOUN 14   nsubj      10:obl|14:nsubj
5  against  ADP  6    case       6:case
6  which    PRON 10   obl        2:ref
8  price    NOUN 10   nsubj:pass 10:nsubj:pass
10 quoted   VERB 2    acl:relcl  2:acl:relcl
```

The phrase moved can be larger than the relativizer: *the king from whose crown we stole the jewels* — `obl(stole, crown)`, `case(crown, from)`, `nmod:poss(crown, whose)`; *a crown, the jewels of which were stolen* — `nsubj:pass(stolen, jewels)`, `nmod(jewels, which)`.

### 5.2 Stranding with a relativizer

With an overt relativizer the stranded preposition is `case` of the relativizer, even though this makes the arc cross others (non-projective):

```
# text = The ones that I made changes to are in red.
#        (EWT 2.18 email-enronsent25_01-0068)
ID FORM     UPOS HEAD DEPREL     DEPS
2  ones     NOUN 10   nsubj      5:obl|10:nsubj
3  that     PRON 5    obl        2:ref
5  made     VERB 2    acl:relcl  2:acl:relcl
6  changes  NOUN 5    obj        5:obj
7  to       ADP  3    case       3:case
```

### 5.3 Stranding without a relativizer

With zero there is no nominal to carry the preposition, so the preposition is **promoted** to the role, as in ellipsis (UD guidelines `_en/dep/acl-relcl.md`, "Preposition stranding"):

```
# text = Now that's a post I can relate to.
#        (EWT 2.18 weblog-blogspot.com_rigorousintuition_20050518101500_ENG_20050518_101500-0067)
ID FORM    UPOS HEAD DEPREL     DEPS
5  post    NOUN 0    root       0:root|8:obl
8  relate  VERB 5    acl:relcl  5:acl:relcl
9  to      ADP  8    obl        8:obl
```

If the stranded preposition belongs to a copular predicate, it becomes the predicate: *the problem the question is about* — `acl:relcl(problem, about)`, `nsubj(about, question)`, `cop(about, is)`.

The same choice exists in questions (*What are you searching for?*: EWT puts `case(What, for)`, [26-questions.md](26-questions.md) §4).

| | relativizer present | zero |
|---|---|---|
| pied-piped | *the house in which you live*: `case(which, in)`, `obl(live, which)` | impossible |
| stranded | *the house that you live in*: `case(that, in)`, `obl(live, that)` | *the house you live in*: `obl(live, in)` (promoted) |

---

## 6. Reduced and infinitival relatives

### 6.1 Participial modifiers are `acl`, not relatives

*People living in glass houses*, *students given high marks*, *a parakeet named Cookie* look like relatives with *who are* removed ("abridgment", Curme 1931, ch. XIV §23 II 11, p. 236), but UD does not call them relative clauses: they attach as plain `acl`, with no relativizer and no gap to recover in the enhanced graph (UD guidelines `_en/dep/acl-relcl.md`, introduction; `_en/dep/acl.md`; seed [`en/seeds/nominal/reduced-relatives.md`](../../en/seeds/nominal/reduced-relatives.md)).

(8) *There's a new EOL Counterparty listed in the UK.* — `acl(Counterparty, listed)` (EWT 2.18 email-enronsent41_01-0017).
(9) *IAEA Report Lessons learned* — `acl(Lessons, learned)` (EWT 2.18 weblog-typepad.com_ripples_20040407125600_ENG_20040407_125600-0025).

Position decides between `acl` and `amod`: a participle with dependents follows the noun and is `acl` (EWT 2.18: 1 794 of 1 806 `acl` on a noun or pronoun are to its right); a lone participle before the noun is `amod` (*a sleeping child*; VERB with `amod`, 814 times on the left).

### 6.2 Infinitival relatives

An infinitive whose gap corresponds to the head noun is a relative clause, `acl:relcl`; an infinitive without such a gap is plain `acl` (UD guidelines `_en/dep/acl-relcl.md`, "Infinitival relatives"):

| | gap? | UD |
|---|---|---|
| *I found a bagel to eat* | yes: eat the bagel | `acl:relcl(bagel, eat)` |
| *I found a house in which to live* | yes, with a pied-piped relativizer | `acl:relcl(house, live)`, `obl(live, which)` |
| *a house (for my mother) to live in* | yes, stranded | `acl:relcl`, `obl(live, in)` |
| *a simple way to get my discount* | no | `acl(way, get)` |
| *his plan to go* | no (content) | `acl(plan, go)` |

---

## 7. Free relatives

A free (fused) relative is a noun phrase whose head is fused with the relativizer: *whatever she's having* = "the thing that she is having". UD makes the WH-word the head of the phrase, gives it the relation of the whole phrase in the higher clause, and attaches the rest of the clause to it as `acl:relcl` — or `advcl:relcl` when the head is a WH-adverb (UD guidelines `_en/dep/acl-relcl.md`, "Free relatives"; `_en/dep/advcl-relcl.md`):

(10) *I'll have whatever she's having.* — `obj(have, whatever)`, `acl:relcl(whatever, having)`.
(11) *You can go where you want and eat what you want.* — `advmod(go, where)`, `advcl:relcl(where, want)`, `obj(eat, what)`, `acl:relcl(what, want)`.
(12) *What money we have left will go to charity.* — `det(money, What)`, `acl:relcl(money, have)`, `nsubj(go, money)`.
(13) *I don't like how it looks.* — `obj(like, how)`, `advcl:relcl(how, looks)`.

When the wh-word is a determiner of a predicate noun, the head would become its own dependent; then the copula is promoted: *I want to sample whatever dish this is* — `obj(sample, dish)`, `acl:relcl(dish, is)` ("cyclic cases").

**EWT does not always follow this.** In EWT 2.18 the free relative of (14) is annotated with the clause, not the wh-word, as head:

(14) *I CANNOT afford to treat whatever is wrong with him.* — `nsubj(wrong, whatever)`, `ccomp(treat, wrong)` (EWT 2.18 answers-20111108104724AAuBUR7_ans-0016); by the guidelines it should be `obj(treat, whatever)`, `acl:relcl(whatever, wrong)`.
(15) *He is willing to do whatever you need from him without hesitation.* — `obj(do, whatever)` and enhanced `8:obj`: the enhanced graph gives *whatever* both roles (EWT 2.18 reviews-128636-0003).

### 7.1 Free relatives versus embedded questions

Embedded wh-interrogatives (*I wonder what she's having*) are `ccomp` with the wh-word inside the clause. The guidelines give *CGEL*'s tests (pp. 1070–1079): free relatives are always finite, never take *whether*, allow *-ever* (*eat what(ever) you want*), cannot be reduced to the wh-word (*\*he bought what!*), and do not take *else* after a non-*ever* wh-word. With verbs of cognition and communication the interrogative reading is the default (*Alice doesn't know what Kim said*):

(16) *I need to know who you are planning to leave with.* — `ccomp(know, leave)`, `obl(leave, who)`, `case(who, with)` (UD guidelines).
(17) *Whatever it is, I'm against it!* — an interrogative adjunct ("no matter what it is"): `advcl(against, Whatever)`, `cop(Whatever, is)`.

| | free relative | interrogative clause |
|---|---|---|
| *He bought what I was selling* | head *what*, `obj(bought, what)` | — |
| *He wondered what I was selling* | — | `ccomp(wondered, selling)`, `obj(selling, what)` |
| *I asked what he asked* | "the same question he asked" | "what the content of his question was" |
| *-ever* | allowed | not as an interrogative |
| *whether* | never | yes |

---

## 8. Restrictive and non-restrictive; clefts and pseudoclefts

### 8.1 Restrictive and non-restrictive

A restrictive (specifying) relative narrows the set of referents; a non-restrictive (ascriptive, descriptive) one adds a fact about a referent already identified. In writing the non-restrictive one is set off by commas; *that* and zero are restrictive only (Curme 1931, ch. XIV "Descriptive and Restrictive Relative Clauses", p. 223; Poutsma vol. 2, ch. XVI §1; seed [`relcl-after-head.md`](../../en/seeds/nominal/relcl-after-head.md)).

| | restrictive | non-restrictive |
|---|---|---|
| example | *I rented the movie which you bought* (that one, not another) | *I introduced myself to John, who promptly forgot my name* |
| commas | no | yes |
| *that*, zero | yes | no |
| antecedent | a common noun | also proper names, pronouns, whole clauses |
| UD | `acl:relcl` | `acl:relcl` (`advcl:relcl` for a clausal antecedent) |

UD does not mark the distinction (UD guidelines `_en/dep/acl-relcl.md`, "Notable properties"). For meaning it matters: a restrictive relative is part of the description that identifies the referent, a non-restrictive one is a separate assertion — which the pragmatics should read as its own fact, not as part of the referring expression ([35-reference-and-coreference.md](35-reference-and-coreference.md)).

(18) *Do you raid a mosque, which serves as a terrorist ammunition storage?* — non-restrictive: `ref(mosque, which)`, `nsubj(serves, which)` (EWT 2.18 weblog-blogspot.com_alaindewitt_20060924104100_ENG_20060924_104100-0172).

Clausal antecedents (sentential relatives) are `advcl:relcl`: *It's around $100 less, which means you get the same camera but quicker!* — see [24-coordination-subordination.md](24-coordination-subordination.md) §11.

### 8.2 *It*-clefts

An *it*-cleft foregrounds one constituent with expletive *it* + copula and puts the rest in a relative clause: *It's John who we want to help*. *CGEL* (p. 416) describes the relative as a dependent of the main clause, not of a noun phrase, so UD uses `advcl:relcl` from the foregrounded constituent (UD guidelines `_en/dep/acl-relcl.md`, "*It*-clefts"):

(19) *It's John who we want to help.* — root *John*, `expl(John, It)`, `cop(John, 's)`, `advcl:relcl(John, want)`, `xcomp(want, help)`, `obj(help, who)`.
(20) *It was with John that I went to the movies.* — root *John*, `case(John, with)`, `advcl:relcl(John, went)`, `obl(went, that)`: *that* is forced into `obl` although no preposition marks it.
(21) *It's this sort of enlightened self interest that keeps large open source projects alive.* — `expl(sort, It)`, `advcl:relcl(sort, keeps)` (EWT 2.18 weblog-typepad.com_ripples_20050410122300_ENG_20050410_122300-0032).

Earlier versions of the guidelines treated *who we want to help* as a free relative; that was dropped because the cleft also allows *that* and zero (*It's John we want to help*), which free relatives never do (footnote 4 of the page).

**Cleft or extraposition?** In extraposition the clause can replace *it* (*It is clear that we should decline* → *That we should decline is clear*); in a cleft it cannot (*\*That we want to help is John*), and *who(m)* can replace *that* (Calude 2008, *ICAME Journal* 32, http://icame.uib.no/ij32/ij32_7_34.pdf, cited in the guidelines). Extraposition: `csubj` + `expl` ([23-complementation.md](23-complementation.md) §8.2).

### 8.3 Pseudoclefts

A pseudocleft puts a free relative and the foregrounded element on the two sides of *be*. UD analyses it as a copular clause; when the foregrounded element is a clause, the free relative is its `nsubj:outer` ([23-complementation.md](23-complementation.md) §9):

(22) *John is who we want to help.* — root *who*, `nsubj(who, John)`, `cop(who, is)`, `acl:relcl(who, want)`.
(23) *Who we want to help is John.* — root *John*, `nsubj(John, Who)`, `acl:relcl(Who, want)`.
(24) *What you are doing is creating a service.* — `nsubj:outer(creating, What)`, with the enhanced graph giving *What* also `obj` of *doing* (EWT 2.18 newsgroup-groups.google.com_alt.animals.bears_1125853b1f13cff6_ENG_20040126_171100-0041).
(25) *What matters is how well trained he is, and how good your bond it.* — `nsubj:outer(trained, What)` (EWT 2.18 answers-20111108094831AAnOjgr_ans-0007).

---

## 9. Relatives versus other modifier clauses

*When, where, why, how* introduce relative clauses, non-relative adnominal clauses (`acl`) and adverbial clauses (`advcl`). The guidelines' heuristics (`_en/dep/acl-relcl.md`, "RCs versus non-relative modifier clauses" and "Testing whether WH-adverb is a relativizer"):

1. A *where*-clause on a place, situation or arrangement, a *when*-clause on a time, or a WH-clause on *reason / way* that can be paraphrased with *in which / on which*, is a relative clause: *the time when the pizza exploded* — `acl:relcl(time, exploded)`, `advmod(exploded, when)`; *a situation where nobody wins*.
2. A *where*-clause with a locative meaning that does not modify such a noun is a free relative: *Where I had lunch yesterday, it was very windy*.
3. Otherwise a *when*-clause on a noun that locates it in time is `acl`: *The headlines when Nixon resigned were legendary* — `acl(headlines, resigned)`.

| clause | relation | example |
|---|---|---|
| relative with WH-adverb | `acl:relcl` | *the hole where the ground caved in* |
| non-relative adnominal | `acl` | *the headlines when Nixon resigned* |
| content clause | `acl` + `mark` | *the fact that he left* |
| adverbial | `advcl` | *When you leave, be sure to let me know* |
| free relative with WH-adverb | `advcl:relcl` on the WH-word | *I looked where you were sitting* |

The formula *That is why / This is where …* in the school texts is a free relative in a copular clause: *A street is just where your family happens to live* — `advcl:relcl(where, happens)` (`$MOVA_DATA/parsedb/current.conllu`, school:everyday:382-our-street-against-theirs.lesson:11).

---

## 10. The enhanced graph: `ref` and the gap's role

In enhanced UD the relativizer is no longer the argument: an arc from the clause's predicate to the **antecedent** carries the gap's role, and the relativizer attaches to the antecedent by `ref` (UD guidelines `_u-overview/enhanced-syntax.md`, "Relative clauses", https://universaldependencies.org/u/overview/enhanced-syntax.html; Schuster and Manning 2016, https://aclanthology.org/L16-1376). The enhanced graph thus has a cycle: *ideology* → `acl:relcl` → *claims* → `nsubj` → *ideology*.

| | basic | enhanced |
|---|---|---|
| *the boy who lived* | `acl:relcl(boy, lived)`, `nsubj(lived, who)` | `acl:relcl(boy, lived)`, `nsubj(lived, boy)`, `ref(boy, who)` |
| *the man you love* | `acl:relcl(man, love)` | + `obj(love, man)` |
| *the house in which you live* | `obl(live, which)`, `case(which, in)` | `obl(live, house)` (the guidelines: `obl:in`), `ref(house, which)` |
| *the time when it exploded* | `advmod(exploded, when)` | `obl(exploded, time)`, `ref(time, when)` |
| *the boy whose dog barked* | `nmod:poss(dog, whose)` | `nmod:poss(dog, boy)`, `ref(boy, whose)` |

The guidelines show the case-marked label `obl:in` for the pied-piped example; EWT 2.18 writes plain `obl` on the antecedent (examples in §5: *unit* gets `10:obl`, *ones* `5:obl`, *post* `8:obl`). In EWT the case label is kept only when the preposition attaches to the nominal that carries the arc.

A stranded preposition that is promoted to a role (§5.3) does not become `case` of the antecedent in the enhanced graph; otherwise the stranded preposition attaches to the relativized element (guidelines, "Preposition stranding").

**Mova writes no enhanced graph.** The frames reader would need exactly the arc the enhanced graph adds — the antecedent in the gap's role — and does not yet compute it (§13).

---

## 11. Agreement and case

### 11.1 Agreement

When the relativizer is the subject, the relative verb agrees with the antecedent: *people who **live***, *a man who **lives*** ([`en/seeds/errors/relative-clause-agreement.md`](../../en/seeds/errors/relative-clause-agreement.md)). The agreement shows which noun the clause attaches to in *the servant of the actress who was on the balcony* (attachment ambiguity). With *one of the X that/who …* the antecedent is the plural *X*: Fowler calls the singular verb in *one of the best men that has ever lived* a "constant error" (Fowler 1926, *A Dictionary of Modern English Usage*, `books/f/fo/fowler-1926-dictionary-modern-english-usage`, as summarised in the seed). Agreement across a relative clause is among the hardest agreement tests for language models (Marvin and Linzen 2018, "Targeted Syntactic Evaluation of Language Models", EMNLP, https://aclanthology.org/D18-1151).

In UD the features of the relative verb come from its form (VBZ = `Sing|3`), not from the antecedent ([`dialects/mova/seeds.md#features-context`](../../dialects/mova/seeds.md#features-context)). An agreement mismatch therefore stays visible in the tree, and the seed flags it.

### 11.2 Case of *who*

*Who* takes its case from its role in the clause: *who* subject, *whom* object or after a preposition; colloquially *who* is used for both, and *whom* as a subject (*the man whom I think is guilty*) is hypercorrection (Curme 1931, ch. XIV "Case of Relative", p. 230; [`who-whom-whose.md`](../../en/seeds/nominal/who-whom-whose.md)). EWT 2.18 has *whom* as `nsubj` not once.

Mova's dialect writes the case into FEATS ([`dialects/mova/seeds.md#wh-case`](../../dialects/mova/seeds.md#wh-case), accepting the post-2.18 `ud-next` change):

| form | lemma (Mova) | `Case` (Mova) | rule |
|---|---|---|---|
| *who* as `nsubj` | who | `Nom` | by the relation |
| *who* as `obj` / `obl` | who | `Acc` | by the relation (*the man who I saw*) |
| *whom*, *whomever* | who, whoever | `Acc` | always, the form wins |
| *whose* with a noun | whose | `Gen`, `Poss=Yes` | `nmod:poss` |

The reason given in the seed: case shows the role of the gap, one of the most useful clause-level facts for retrieval; personal pronouns already carry case in EWT; GUM, GENTLE, LinES and ParTUT already lemmatize *whom* as *who*. The rules are deterministic both ways, so export to EWT 2.18 is identity (24 lemmas, 460 features changed and restored in the round trip; converter rules `mova.ud-next.who-*` in `dialects/ud-next/convert.md`).

---

## 12. Mova's parser on relative clauses

Fourteen test sentences (Mova en 0.4, CPU, 2026-10-17):

| sentence | Mova en 0.4 | UD | verdict |
|---|---|---|---|
| *The man who lives next door is a doctor.* | `acl:relcl(man, lives)`, `nsubj(lives, who)` | same | right |
| *The book that I read was boring.* | `obj(read, that)` PRON, `acl:relcl(book, read)` | same | right |
| *The house in which you live is old.* | `case(which, in)`, `obl(live, which)`, `acl:relcl(house, live)` | same | right |
| *The house you live in is old.* | `obl(live, in)` (promoted), `acl:relcl(house, live)` | same | right |
| *The boy whose dog barked ran away.* | `nmod:poss(dog, whose)`, `nsubj(barked, dog)`, `acl:relcl(boy, barked)` | same | right |
| *People living in glass houses should not throw stones.* | `acl(People, living)` | same | right |
| *It is John who we want to help.* | **`nsubj(John, It)`**, **`acl:relcl(John, want)`**, `obj(want, who)` | `expl`, `advcl:relcl`, `obj(help, who)` | wrong on three arcs |
| *The man I saw yesterday left.* | **root *saw***, `nsubj(saw, man)`, **`obj(saw, yesterday)`**, **`acl(yesterday, left)`** | `acl:relcl(man, saw)`, root *left* | wrong: the zero relative is read as the main clause |
| *The man whom I trusted lied.* | **root *whom***, `acl:relcl(whom, trusted)`, **`xcomp(trusted, lied)`** | `obj(trusted, whom)`, `acl:relcl(man, trusted)`, root *lied* | wrong |
| *I found a bagel to eat.* | **`acl(bagel, eat)`** | `acl:relcl` | label: the gap is not recognized |
| *I'll have whatever she is having.* | `obj(having, whatever)`, **`ccomp(have, having)`** | `obj(have, whatever)`, `acl:relcl(whatever, having)` | EWT-like, not the guideline (§7) |
| *What John did was to play tennis.* | root **What**, **`acl:relcl(What, was)`** | `nsubj:outer(play, What)`, `cop(play, was)` | wrong |
| *I looked where you were sitting.* | `advmod(sitting, where)`, **`advcl(looked, sitting)`** | `advmod(looked, where)`, `advcl:relcl(where, sitting)` | the older, non-free-relative analysis |
| *the people who lives here* | `nsubj(lives, who)`, `nsubj(lives, people)` | a fragment; agreement error in the text | the parser makes *lives* the root with two subjects |

The parser is reliable on ordinary relatives with an overt relativizer and on stranding. It fails on two patterns that matter for reading stories: the zero object relative followed by the main verb (*The man I saw yesterday left*), which it reads as a main clause plus garden-path remains, and *whom* as a sentence's second word. In the school trees these sentences went through the teacher loop (treeagree witnesses, then the teacher; docs/architecture.md), so `$MOVA_DATA/parsedb/current.conllu` has them mostly right, but the parser trained on them still errs.

The parse database still has wrong *whose* trees: *Whether the owner is rich or poor does not change whose the thing is.* — `nmod:poss(…, whose)` with no possessed noun; *whose* here is the predicate of a copular interrogative clause (school:religion-ethics:042-finders-keepers-and-what-belongs-to-whom.lesson:9).

---

## 13. From relative clauses to frames

The pragmatic reader ([`prag/src/frames.rs`](../../prag/src/frames.rs); docs/pragmatics.md) makes a frame for every predicate, including the predicate of a relative clause, and resolves pronouns in reading order. Relative pronouns are not resolved as pronouns, and the antecedent is not put into the gap's role. A text read on 2026-10-17 (Mova `world frames`, en 0.4, CPU):

```
The cat that the dog chased ran up a tree.

#10 Event dog chase
     roles by vn chase-51.6 «NP V PP.theme»: agent=Agent
#11 Event cat run up patient=tree
     roles by vn run-51.3.2-2-1 «NP V NP»: agent=Theme tree=Trajectory
```

The relative frame #10 has the right agent but no patient: *that* is in the tree as `obj(chased, that)`, but the reader takes entities from nouns, and *that* is not linked to *cat*. The enhanced arc `obj(chased, cat)` (§10) is what the reader needs. With a subject relative (*the fox that flattered the crow*) the agent is missing in the same way, and is sometimes recovered by the learned frame rules ([`prag/src/rules.rs`](../../prag/src/rules.rs): the action "agent from the frame above / from the noun", learned from the frame bench; docs/pragmatics.md, "Rules the reader learns itself").

Reduced relatives (`acl` participles) get frames as well: the frame-rule learner has a site "a participle on a noun that has no frame" with the action "a frame for a participle" (docs/pragmatics.md). A participle's agent or patient (*the cake **eaten** by the Mole*) comes from the noun it modifies; the learner states this as a rule with its evidence when it finds one.

Restrictive relatives are part of the noun's description: the course student's entity is the head lemma (`the Cheshire Cat` = `the Cat` = `cat`, docs/pragmatics.md, "Entities"), so the restricting clause is a separate frame and does not narrow the entity. *The man who lives next door* and *the man who lives in London* are the same entity `man` for the reader.

### Not yet

- **The antecedent in the gap's role** (the enhanced `ref` mechanism): not computed; relative frames miss the relativized argument unless a learned rule recovers it.
- **Restrictive versus non-restrictive**: not distinguished; commas are not used.
- **Restriction of entities**: a restrictive relative does not create a distinct entity.
- **Free relatives**: *whatever she is having* gives no "thing she has" entity; the parser follows the EWT-like analysis.
- **Clefts and pseudoclefts**: no focus information; the parser errs on both (§12).
- **Sentential *which*** as a link: the `advcl:relcl` frame is not linked to the clause it comments on ([24-coordination-subordination.md](24-coordination-subordination.md) §14).
- **Agreement-based attachment** (*the servant of the actress who was …*): not used to choose the antecedent.
- **Parser errors** of §12.

---

## In Mova

- Conventions and checks: [`en/seeds/nominal/relcl-after-head.md`](../../en/seeds/nominal/relcl-after-head.md), [`relcl-subject.md`](../../en/seeds/nominal/relcl-subject.md), [`relative-that.md`](../../en/seeds/nominal/relative-that.md), [`who-whom-whose.md`](../../en/seeds/nominal/who-whom-whose.md), [`reduced-relatives.md`](../../en/seeds/nominal/reduced-relatives.md), [`content-clause-acl.md`](../../en/seeds/nominal/content-clause-acl.md); errors: [`relative-pronoun-in-clause.md`](../../en/seeds/errors/relative-pronoun-in-clause.md), [`relative-clause-agreement.md`](../../en/seeds/errors/relative-clause-agreement.md); verbal: [`wh-adverb-clause.md`](../../en/seeds/verbal/wh-adverb-clause.md), [`outer-subject.md`](../../en/seeds/verbal/outer-subject.md).
- Dialect: [`dialects/mova/seeds.md#wh-case`](../../dialects/mova/seeds.md#wh-case) (case and lemma of *who*), [`clausal-predicate.md`](../../dialects/mova/seeds.md#clausal-predicate) (`:outer` in pseudoclefts).
- Converter engine for the *who* rules: [`en/src/convert.rs`](../../en/src/convert.rs).
- Frames: [`prag/src/frames.rs`](../../prag/src/frames.rs), learned repairs [`prag/src/rules.rs`](../../prag/src/rules.rs), coreference [`coref/src/sieve.rs`](../../coref/src/sieve.rs) (relative pronouns are mentions in the CorefUD scoring).
- Not yet: see §13.

## Sources

- Calude, Andreea S. 2008. "Clefting and extraposition in English." *ICAME Journal* 32. http://icame.uib.no/ij32/ij32_7_34.pdf.
- Curme, George O. 1931. *A Grammar of the English Language*, vol. III: *Syntax*, ch. XIV "Attributive Relative Clause". `books/c/cu/curme-1931-grammar-english-language-vol`.
- Fowler, H. W. 1926. *A Dictionary of Modern English Usage*. `books/f/fo/fowler-1926-dictionary-modern-english-usage`.
- Huddleston, Rodney, and Geoffrey K. Pullum. 2002. *The Cambridge Grammar of the English Language*, ch. 12 (not open; cited through the UD guidelines).
- Keenan, Edward L., and Bernard Comrie. 1977. "Noun Phrase Accessibility and Universal Grammar." *Linguistic Inquiry* 8(1) (not open; wanted).
- Marvin, Rebecca, and Tal Linzen. 2018. "Targeted Syntactic Evaluation of Language Models." EMNLP 2018. https://aclanthology.org/D18-1151.
- Poutsma, Hendrik. *A Grammar of Late Modern English*, vol. 2 ch. XVI, vol. 4 ch. XXXIX. `books/p/po/poutsma-1904-grammar-late-modern-english`.
- Schuster, Sebastian, and Christopher D. Manning. 2016. "Enhanced English Universal Dependencies." LREC 2016. https://aclanthology.org/L16-1376.
- Universal Dependencies guidelines, snapshot 2026-09-24: `_en/dep/acl-relcl.md` (relative clauses and clefts), `advcl-relcl.md`, `acl.md`; `_u-overview/enhanced-syntax.md`; `_en/feat/PronType.md`. https://universaldependencies.org/en/dep/acl-relcl.html.
- UD English EWT 2.18 (Silveira et al. 2014, https://aclanthology.org/L14-1067; CC BY-SA 4.0), examples by `sent_id`.
- Wright, Joseph. 1905. *The English Dialect Grammar*. `books/w/wr/wright-1905-english-dialect-grammar-comprising`.
- Mova: `docs/pragmatics.md`, `docs/architecture.md`, `docs/dialect.md`; `$MOVA_DATA/parsedb/current.conllu`.
