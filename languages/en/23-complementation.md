# 23. Complementation

A predicate chooses its complements: how many, of which shape (a noun phrase, a finite clause, an infinitive, an *-ing* clause, a bare predicate), and who the understood subject of a non-finite complement is. This chapter describes the complement patterns of English verbs, adjectives and nouns, how Universal Dependencies (UD) annotates each of them (`obj`, `iobj`, `ccomp`, `xcomp`, `csubj`, `acl`, the `:outer` subjects and the enhanced `nsubj:xsubj`), where the English UD guidelines, the EWT data and Mova's dialect and parser differ, and how Mova's pragmatics reads complements into frames.

Contents

1. [Overview: arguments and the shapes of complements](#1-overview-arguments-and-the-shapes-of-complements)
2. [Nominal objects: `obj` and `iobj`](#2-nominal-objects-obj-and-iobj)
3. [Finite clausal complements: `ccomp`](#3-finite-clausal-complements-ccomp)
4. [Open complements: `xcomp`](#4-open-complements-xcomp)
5. [Control and raising](#5-control-and-raising)
6. [Infinitive, bare infinitive and *-ing* complements](#6-infinitive-bare-infinitive-and--ing-complements)
7. [Small clauses, objective complements, resultatives and depictives](#7-small-clauses-objective-complements-resultatives-and-depictives)
8. [Clausal subjects: `csubj`, extraposition and *tough* constructions](#8-clausal-subjects-csubj-extraposition-and-tough-constructions)
9. [Predicate clauses: `nsubj:outer` and `csubj:outer`](#9-predicate-clauses-nsubjouter-and-csubjouter)
10. [Complements of adjectives and nouns](#10-complements-of-adjectives-and-nouns)
11. [Speech, quotation and reported speech](#11-speech-quotation-and-reported-speech)
12. [The enhanced graph: `nsubj:xsubj`](#12-the-enhanced-graph-nsubjxsubj)
13. [Mova's parser on complements: a test](#13-movas-parser-on-complements-a-test)
14. [From complements to frames](#14-from-complements-to-frames)

Cross-references: the inventory of relations and the enhanced layer as a whole are in [20-syntax-overview.md](20-syntax-overview.md); subjects, objects and the copula in [21-clause-structure.md](21-clause-structure.md); adverbial clauses (purpose, result) in [24-coordination-subordination.md](24-coordination-subordination.md); embedded questions in [26-questions.md](26-questions.md); passives of complement-taking verbs in [28-voice-and-passive.md](28-voice-and-passive.md).

---

## 1. Overview: arguments and the shapes of complements

UD separates **core arguments** (subjects, objects, clausal complements) from **oblique** dependents (prepositional phrases and adverbials). In English, prepositional phrases are never core: *she gave it **to me*** has `obl`, not `iobj` (UD guidelines, `_en/dep/iobj.md`, https://universaldependencies.org/en/dep/iobj.html; Mova's decision on the label in [`dialects/mova/seeds.md#to-phrase-obl`](../../dialects/mova/seeds.md#to-phrase-obl)).

The core labels split along two axes (de Marneffe et al. 2021, "Universal Dependencies", *Computational Linguistics* 47(2), https://aclanthology.org/2021.cl-2.11):

| | nominal | clausal, own subject possible | clausal, subject controlled |
|---|---|---|---|
| subject | `nsubj`, `nsubj:pass`, `nsubj:outer` | `csubj`, `csubj:pass`, `csubj:outer` | — |
| object | `obj`, `iobj` | `ccomp` | `xcomp` |

The distinction between `ccomp` and `xcomp` is **not** finite versus non-finite, although in English it nearly coincides with it. The test is control: if the understood subject of the complement *must* be an argument of the higher clause, the complement is `xcomp`; if it is a matter of anaphora or context, it is `ccomp` (UD guidelines, `_u-overview/complex-syntax.md`, "Clausal Complements", https://universaldependencies.org/u/overview/complex-syntax.html).

(1) *The boss said **to start digging**.* — `ccomp(said, start)`: who starts is not fixed by the grammar.
(2) *We started **digging**.* — `xcomp(started, digging)`: the diggers are necessarily *we*.

The shapes of complements that English verbs select, with the labels they get:

| shape | example | label of the complement head |
|---|---|---|
| NP | *She read **the letter*** | `obj` |
| NP NP | *She gave **him** **a book*** | `iobj` + `obj` |
| *that*-clause / bare finite clause | *He says (**that**) you like it* | `ccomp` |
| *whether / if / wh*-clause | *I wonder **whether** it works* | `ccomp` |
| *to*-infinitive, controlled | *I want **to go*** | `xcomp` |
| NP + *to*-infinitive | *I asked **him** **to stay*** | `iobj`/`obj` + `xcomp` |
| NP + bare infinitive | *I saw **him** **leave*** | `obj` + `xcomp` |
| *-ing*, controlled | *I enjoy **swimming*** | `xcomp` |
| NP + predicative AdjP/NP | *They elected **her** **president*** | `obj` + `xcomp` |
| quotation | *"**Come here**," said the Badger* | `ccomp` (or `parataxis`, §11) |

Traditional grammars draw the same map with other names. Curme (1931, *A Grammar of the English Language*, vol. III, *Syntax*, ch. XV "Object Clause", pp. 239–258, `books/c/cu/curme-1931-grammar-english-language-vol`) sorts object clauses by the case an equivalent noun would have (genitive, dative, accusative, prepositional clauses); Jespersen (MEG V, ch. XII "Infinitive as Object" and ch. XVIII "Subject + Infinitive as Object of Main Verb", `books/j/je/jespersen-1914-modern-english-grammar-historical`, text-1) sorts infinitive complements by what supplies their "subject". Reed and Kellogg (1877, *Higher Lessons in English*, Lesson 31, `books/r/re/reed-1877-higher-lessons-english`) call the predicative of an object its "objective complement".

EWT 2.18 counts (train + dev + test, 254 820 words; counted 2026-10-17):

| relation | count | | relation | count |
|---|---:|---|---|---:|
| `xcomp` | 3 831 | | `csubj` | 336 |
| `ccomp` | 2 540 | | `csubj:pass` | 8 |
| `acl` | 1 831 | | `csubj:outer` | 3 |
| `iobj` | 795 | | `nsubj:outer` | 254 |
| `expl` | 756 | | | |

---

## 2. Nominal objects: `obj` and `iobj`

### 2.1 The direct object

The direct object is the second core nominal of a transitive verb: *She read **the letter***. In English it has no case marking except in pronouns (*her*, *them*), and it follows the verb without a preposition. A verb has at most one `obj` in UD ([`en/seeds/verbal/core-objects.md`](../../en/seeds/verbal/core-objects.md)); a second bare nominal is either `iobj` (§2.2), a predicative `xcomp` (§7) or a bare-NP adjunct `obl:unmarked` (*stayed **a week***).

### 2.2 The indirect object

English ditransitives put a recipient, addressee or beneficiary before the theme: *She gave **me** a raise*. UD calls that nominal `iobj`.

(3) *Give **him** a call tommorow.* — `iobj(Give, him)`, `obj(Give, call)` (EWT 2.18 email-enronsent08_02-0059).
(4) *When she goes to the zoo the elephants throw **her** peanuts.* — `iobj(throw, her)` (EWT 2.18 email-enronsent08_02-0015).

Verb classes that license the double-object frame (from VerbNet's double-object classes and EWT, collected in [`en/seeds/lexicon/verb-iobj-licensors.md`](../../en/seeds/lexicon/verb-iobj-licensors.md)):

| class | verbs |
|---|---|
| transfer | give, hand, lend, owe, sell, send, pay |
| communication | tell, show, teach, email, write, ask |
| benefactive | buy, build, cook, fix, find, get, make |
| cost and billing | cost, charge, bill, fine |
| promising | promise, offer, guarantee |
| speech and urging (with a clause) | inform, notify, remind, warn, advise, assure, convince, persuade, urge, ask, allow, cause |

### 2.3 The sole `iobj` and `iobj` with `xcomp` (UD 2.12, Mova)

The English page `_en/dep/iobj.md` still says that `iobj` "occurs only when there is a `obj` or `ccomp` in the clause". That sentence is a remnant: the UD 2.12 amendment "Sole iobj" allows an `iobj` without a direct object, and EWT follows the amendment (UD `docs/changes.md`, 2.12; the digest of the guidelines, research library `docs/papers/ud/ud-core.md`). Mova adopts the amendment ([`dialects/mova/seeds.md#iobj-sole`](../../dialects/mova/seeds.md#iobj-sole)):

- `iobj` can be the only internal argument: *remind **me***, *ask **Bush***;
- `iobj` can stand with an `xcomp` when the verb takes an addressee: *ask / tell / allow / convince / urge / persuade / cause / trust / teach / remind / warn **X** to do*;
- with *let, make, get, have, keep, find, want, help* + `xcomp` the nominal is `obj`;
- the boundary is set by the lexicon, not by examples.

(5) *Hiller asked **Bush** to name the leaders of Chechnya, Taiwan, India and Pakistan.* — `iobj(asked, Bush)`, `xcomp(asked, name)` (EWT 2.18 weblog-juancole.com_juancole_20040708181175_ENG_20040708_181175-0011).
(6) *He did not allow **Palestinian radicals** to launch operations against Israel from Iraq.* — `iobj(allow, radicals)`, `xcomp(allow, launch)` (EWT 2.18 weblog-juancole.com_juancole_20040604210986_ENG_20040604_210986-0018).
(7) *They don't let **just anybody** do it.* — `obj(let, anybody)`, `xcomp(let, do)` (EWT 2.18 weblog-blogspot.com_alaindewitt_20040929103700_ENG_20040929_103700-0012).

EWT 2.18 has 795 `iobj`, of which 243 have no `obj` or `ccomp` and 93 stand with an `xcomp`; 39 verbs of the same class have `obj` + `xcomp` instead (*encourage, enable, order, invite, force, require*), so the class boundary in EWT is not crisp (counts from the seed above, 2026-09-25). The gain of keeping `iobj` is information: it separates the addressee of *ask X to* from the causee of *make X do*.

(8) *You informed **me** that you would discuss this with your attorney.* — `iobj(informed, me)`, `ccomp(informed, discuss)` (EWT 2.18 email-enronsent12_01-0035).

In the Mova school trees the sole `iobj` is common in plain explanatory prose:

(9) *The voice tells **you** who someone is.* — `iobj(tells, you)` + `ccomp` (`$MOVA_DATA/parsedb/current.conllu`, school:english:021-authors-choices-why-this-word-why-this-way.lesson:8).
(10) *Mia didn't think anyone would ask **her** that fast.* — `iobj(ask, her)` with no other object (school:everyday:461-how-best-friends-start.lesson:17).

### 2.4 Prepositional datives

The prepositional variant of the dative alternation is `obl`, with the preposition as `case`; in the enhanced graph the label carries the preposition (`obl:to`):

| | basic | enhanced |
|---|---|---|
| *She gave me a raise* | `iobj(gave, me)` | `iobj` |
| *She gave a raise to me* | `obl(gave, me)`, `case(me, to)` | `obl:to` |

Mova's frames do not depend on which variant the text uses: VerbNet frames name the recipient role in both (`Agent V Theme {to} Recipient`), and `prag::vn` maps either the `iobj` or the `to`-oblique onto the frame's `recipient` (docs/pragmatics.md, "The linguists' rules"; [`prag/src/vn.rs`](../../prag/src/vn.rs)).

---

## 3. Finite clausal complements: `ccomp`

### 3.1 Declarative content clauses

A clause with its own subject that fills the object slot is `ccomp`. The complementizer *that* is `mark` (SCONJ) and may be absent (UD guidelines, `_en/dep/ccomp.md`, https://universaldependencies.org/en/dep/ccomp.html):

(11) *He says **that** you like to swim.* — `ccomp(says, like)`, `mark(like, that)`.
(12) *He says you like to swim.* — `ccomp(says, like)`.
(13) *A little birdie told me that you were checking into tickets for Seoul.* — `iobj(told, me)`, `ccomp(told, checking)` (EWT 2.18 email-enronsent34_01-0024).

A `ccomp` occupies the object position, so a head with `ccomp` has no `obj` ([`en/seeds/verbal/clausal-complement.md`](../../en/seeds/verbal/clausal-complement.md), rule `en.verbal.ccomp-no-obj`).

Verbs with `that`-complements group by meaning (after Curme 1931, ch. XV "Accusative Clause", pp. 241–253):

| class | verbs | notes |
|---|---|---|
| saying | say, tell, report, claim, announce, explain, reply | addressee as `iobj` (tell) or `obl:to` (say to) |
| thinking and believing | think, believe, suppose, guess, assume, expect | *that* very often absent |
| knowing (factive) | know, realize, notice, regret, forget | complement presupposed true ([30-pragmatics-overview.md](30-pragmatics-overview.md), presupposition) |
| deciding and demanding | insist, suggest, demand, recommend, require | the present subjunctive in the complement, `Mood=Sub` ([`dialects/mova/seeds.md#subjunctive`](../../dialects/mova/seeds.md#subjunctive)) |
| emotion | hope, fear, wish, be glad, be afraid | *wish* takes a past-tense "unreal" complement |

### 3.2 Interrogative content clauses

Embedded yes/no questions take *whether* or *if* (`mark`), embedded wh-questions start with the wh-word in its clause-internal role; the clause is `ccomp` of the verb or adjective. They are treated in detail in [26-questions.md](26-questions.md) §5.

(14) *Please consider **whether** I might fit in your group.* — `mark(fit, whether)`, `ccomp(consider, fit)` (EWT 2.18 email-enronsent14_01-0015).
(15) *Could you tell me what deal tickets you are seeing?* — `iobj(tell, me)`, `ccomp(tell, seeing)`, `det(tickets, what)` (EWT 2.18 email-enronsent11_01-0004).

Free relatives look like wh-complements but fill a nominal slot (*I'll have whatever she's having*); UD separates them by tests from Huddleston and Pullum (2002, *The Cambridge Grammar of the English Language*, pp. 1070–1079, cited in the guidelines `_en/dep/acl-relcl.md`, "Free relatives versus interrogative complement clauses"): free relatives are always finite, never introduced by *whether*, allow *-ever* forms and never take *else* after a non-*ever* head. With verbs of cognition and communication the guidelines take the interrogative reading as the default. See [25-relative-clauses.md](25-relative-clauses.md) §7.

### 3.3 Non-finite `ccomp`

A *to*-infinitive is `ccomp`, not `xcomp`, when its subject is not fixed by the higher clause. This is rare and comes mostly after verbs of saying:

(16) *The boss said **to** start digging.* — `ccomp(said, start)`, `mark(start, to)` (UD guidelines, `_en/dep/ccomp.md`).

Mova's school trees almost never contain this pattern; Mova's parser gives `xcomp` for it, which the frames then read as control by the subject (a wrong reading for (16): the boss is not the digger).

---

## 4. Open complements: `xcomp`

An open complement is a predicate without its own subject; its subject is "borrowed" from the higher clause, normally the object if there is one, else the subject. It is always non-finite in English (UD guidelines, `_en/dep/xcomp.md`, https://universaldependencies.org/en/dep/xcomp.html; the label comes from Lexical-Functional Grammar). It covers:

| construction | example | controller |
|---|---|---|
| subject control | *I tried **to finish** it* | subject |
| object control | *Sue asked George **to respond*** | the `iobj`/`obj` |
| raising to subject | *The cat seems **to be in pain*** | subject |
| raising to object (ECM) | *I expect him **to win*** | the `obj` |
| copula-like verbs | *She looks **very beautiful***, *I became very **upset*** | subject |
| objective complements | *I consider him **honest***, *They elected her **president*** | the `obj` |
| resultatives | *He painted the barn **red***, *The pond froze **solid*** | object or subject |
| obligatory depictives | *Keep your powder **dry*** | the `obj` |

An `xcomp` never has its own `nsubj` (rule `en.verbal.xcomp-no-subject` in [`en/seeds/verbal/open-complement-xcomp.md`](../../en/seeds/verbal/open-complement-xcomp.md)); the one exception is raising with *there*: *There seems to be **a problem*** — *problem* attaches to *be*.

The basic tree does not say who the controller is: the same `xcomp` arc stands for subject control (*promise*), object control (*persuade*) and raising (*seem*). The difference lives in the lexicon and in the enhanced graph (§12).

---

## 5. Control and raising

### 5.1 The four classes

Control and raising verbs differ in whether the "lent" argument gets a semantic role from the higher verb (Jespersen MEG V, ch. XVIII; the summary in [`en/seeds/lexicon/verb-raising-control.md`](../../en/seeds/lexicon/verb-raising-control.md)):

| class | the higher verb's own argument? | test | verbs |
|---|---|---|---|
| subject control | yes, the subject is an experiencer or agent of *want/try* | no expletive subject: *\*There tries to be a riot* | want, try, hope, decide, promise, refuse, manage, attempt |
| object control | yes, the object is an addressee or causee | passive keeps the meaning: *George was asked to respond* | ask, tell, persuade, convince, order, urge, allow, force, make, let |
| raising to subject | no, *seem* assigns no role to its subject | expletives raise: *There seems to be a problem*; idiom chunks keep their meaning: *The cat seems to be out of the bag* | seem, appear, tend, happen, be likely, be sure, begin (one sense) |
| raising to object (ECM) | no, the object belongs semantically to the infinitive | the infinitive can be passive with the same meaning: *I expect the doctor to examine her* ≈ *I expect her to be examined by the doctor* | believe, expect, consider, find, want (in this frame), like |

The *promise* exception: *She promised me **to come** early* is subject control although there is an object (the classic exception to "Visser's generalization", named in the UD guidelines `_u-overview/complex-syntax.md`: the controller is usually the object when one is present).

(17) *Did you happen **to sign** your employment agreement?* — raising: `nsubj(happen, you)`, `xcomp(happen, sign)` (EWT 2.18 email-enronsent30_01-0019).
(18) *Colombo, too, seems **to be in an intractable position**.* — `xcomp(seems, position)` with `cop(position, be)` (EWT 2.18 weblog-blogspot.com_dakbangla_20041119231111_ENG_20041119_231111-0019).
(19) *Now the Electoral Commission is refusing **to punish** people for mere past Baath Party membership.* — subject control; enhanced `nsubj:xsubj(punish, Commission)` (EWT 2.18 weblog-juancole.com_juancole_20051126063000_ENG_20051126_063000-0034).

### 5.2 UD's single analysis

UD does not distinguish "small clause" or "exceptional case marking" structures: *Sue wanted Fred to accept the job* has `obj(wanted, Fred)` and `xcomp(wanted, accept)`, the same shape as object control (UD guidelines, `_u-overview/complex-syntax.md`, which calls this an "object with infinitive" or "raising to object" analysis). The raising/control difference is therefore invisible in the basic tree. For a reader that builds frames this matters: under raising, the higher verb has no agent of its own (*seem* has no seemer), and its frame should not be read as an event done by the subject.

### 5.3 Controlled gerunds and the *for*-subject

Control also reaches *-ing* complements (*I enjoy swimming*; *Keep it going*) and adjectival complements (*eager to please*). A complement with an overt *for*-subject has its own subject and is `ccomp`/`csubj`/`advcl`, never `xcomp`: *It was helpful for John to solve this problem* — `mark(solve, for)`, `nsubj(solve, John)` (UD guidelines, `_en/specific-syntax.md`, "Tough-constructions").

### 5.4 In the lexicon

The repo keeps the verb lists as seeds: [`verb-raising-control.md`](../../en/seeds/lexicon/verb-raising-control.md), [`verb-iobj-licensors.md`](../../en/seeds/lexicon/verb-iobj-licensors.md), [`verb-no-passive.md`](../../en/seeds/lexicon/verb-no-passive.md) (raising verbs and *have*, *resemble* have no passive), [`verb-copula-like.md`](../../en/seeds/lexicon/verb-copula-like.md). The pragmatics reader has its own compiled lists: `CONTROL` and `OBJ_CONTROL` in [`prag/src/frames.rs`](../../prag/src/frames.rs) (34 object-control verbs: *make, let, help, have, see, watch, hear, feel, ask, tell, want, allow, force, teach, get, cause, order, persuade …*).

---

## 6. Infinitive, bare infinitive and *-ing* complements

### 6.1 The *to*-infinitive

The marker *to* is `mark` of the infinitive (PART, XPOS TO). Verbs that take only an infinitive, never a gerund, are verbs of wanting, intending, deciding, attempting and managing: *want, hope, decide, refuse, manage, pretend, offer, agree, promise, seem* ([`en/seeds/lexicon/verb-infinitive-object.md`](../../en/seeds/lexicon/verb-infinitive-object.md), after Poutsma, *A Grammar of Late Modern English*, Part II, ch. XIX §§19–20, `books/p/po/poutsma-1904-grammar-late-modern-english`). Poutsma observes that the infinitive keeps a sense of futurity relative to the main verb (Poutsma 1923, *The Gerund*, §44, cited in the seed).

### 6.2 The bare infinitive

After verbs of causing and permitting (*make, let, have, bid*) and of perception (*see, hear, feel, watch, notice*) the infinitive has no *to* in the active; *to* returns in the passive (Brown 1851, *The Grammar of English Grammars*, Rule XIX and Obs. 5, `books/b/br/brown-1851-grammar-english-grammars`; seed [`verb-bare-infinitive.md`](../../en/seeds/lexicon/verb-bare-infinitive.md)):

| active | passive |
|---|---|
| *They made him **wait*** | *He was made **to** wait* |
| *I saw him **leave*** | *He was seen **to** leave* |
| *Let **heads** roll!* | — (*let* has no passive) |

(20) *(Newsweek reported that Bush commanded his cabinet, "Let heads roll!")* — `obj(Let, heads)`, `xcomp(Let, roll)`; the quotation is `ccomp` of *commanded*, the cabinet `iobj` (EWT 2.18 weblog-juancole.com_juancole_20040823064025_ENG_20040823_064025-0020).
(21) *In fact, you were helping them **solve** their problem.* — *help* allows both bare and *to*: `obj(helping, them)`, `xcomp(helping, solve)` (EWT 2.18 weblog-blogspot.com_alaindewitt_20040929103700_ENG_20040929_103700-0054).

The hortative *let's* is the same construction: *Let's just remember a seminal Bush moment in 1999* — `obj(Let, 's)`, `xcomp(Let, remember)` (EWT 2.18 weblog-juancole.com_juancole_20040708181175_ENG_20040708_181175-0003).

### 6.3 The *-ing* complement

A gerund-participle complement of a verb is `xcomp` when its subject is controlled: *I enjoy reading this blog*, *Keep it going*. Verbs that take only the gerund: *admit, deny, avoid, escape, finish, give up, quit, stop* ("cease"), *postpone, delay, mind, miss, risk, resist, imagine, consider, suggest, practise, enjoy, appreciate, keep* ([`en/seeds/lexicon/verb-gerund-object.md`](../../en/seeds/lexicon/verb-gerund-object.md), after Poutsma, Part II, ch. XIX §§17–18). *Stop to smoke* is a different construction: purpose `advcl`.

(22) *You really got me **thinking**, I enjoy **reading** this blog.* — `obj(got, me)`, `xcomp(got, thinking)`; `xcomp(enjoy, reading)` (EWT 2.18 weblog-blogspot.com_rigorousintuition_20050518101500_ENG_20050518_101500-0068).

UD sets `VerbForm=Ger` on an *-ing* form in a nominal position without auxiliaries and `VerbForm=Part` elsewhere (UD guidelines `_en/feat/VerbForm.md`, rules refined in v2.14; seed [`en/seeds/verbal/gerund-participle.md`](../../en/seeds/verbal/gerund-participle.md)). The difference between complement gerunds and subject gerunds is the relation (`xcomp` vs `csubj`), not the feature.

### 6.4 Verbs with two complement types and a change of meaning

| verb | with *to*-infinitive | with *-ing* |
|---|---|---|
| remember / forget | the act is still to be done: *remember to lock it* | the act is past: *remember locking it* |
| stop | purpose adjunct: *stopped to talk* (`advcl`) | cessation: *stopped talking* (`xcomp`) |
| try | attempt: *try to open it* | experiment: *try opening it* |
| regret | announcing: *regret to say* | the past act: *regret saying* |
| go on | a new act: *went on to win* | continuation: *went on talking* |

The UD tree does not distinguish these readings except for *stop*, where the label changes. Mova's frames read *to* after a non-control verb as purpose (§14).

---

## 7. Small clauses, objective complements, resultatives and depictives

### 7.1 Objective complements

After *make, call, name, elect, appoint, consider, think, find, keep, leave, paint* the object receives a predicate: *They made Victoria **queen***; *I consider him **a fool***; *I consider him **honest***. UD calls the predicate `xcomp` of the verb (UD guidelines `_en/dep/xcomp.md`; seed [`en/seeds/verbal/objective-complement.md`](../../en/seeds/verbal/objective-complement.md)). With *as* (*regard X as Y*) the predicate is an `obl` with `case(as)`.

(23) *I'll keep you **posted**.* — `obj(keep, you)`, `xcomp(keep, posted)` (EWT 2.18 email-enronsent44_01-0013).
(24) *Keep Your Powder **Dry**!* — `obj(Keep, Powder)`, `xcomp(Keep, Dry)` (EWT 2.18 email-enronsent11_01-0066).
(25) *Please find **attached** the Enron Master Agreement.* — `xcomp(find, attached)`, `obj(find, Agreement)` (EWT 2.18 email-enronsent13_01-0022).

Example (25) shows a disagreement inside UD: the English overview `_en/specific-syntax.md` ("Depictives", marked "may be subject to change") gives *Find attached resume* with `advmod(Find, attached)`, while the EWT 2.18 data (email-enronsent08_02-0068, email-enronsent13_01-0022) give `xcomp`. Mova follows the data and the priority rule of the guidelines digest: `changes.md` first, then label pages, then the language overview.

### 7.2 Resultatives

A resultative says what state the object (or, with an unaccusative, the subject) ends up in; it is an argument, `xcomp` (UD guidelines `_en/specific-syntax.md`, "Resultatives"):

(26) *He painted the barn **red**.* — `obj(painted, barn)`, `xcomp(painted, red)`.
(27) *The pond froze **solid**.* — `xcomp(froze, solid)`.
(28) *Guerrillas near Hawijah launched an attack that left 6 **dead**, including 4 Iraqi soldiers.* — `obj(left, 6)`, `xcomp(left, dead)`; enhanced `nsubj:xsubj(dead, 6)` (EWT 2.18 weblog-juancole.com_juancole_20051126063000_ENG_20051126_063000-0022).

### 7.3 Optional depictives: `advcl` in Mova

A depictive describes a participant at the time of the event without being required by the verb: *She entered the room **sad***. The UD 2.10 amendment and the English label page `_en/dep/advcl.md` make the optional depictive an `advcl` of the verb; `_en/specific-syntax.md` still says `advmod` in one place and `acl` in another. Mova accepts `advcl` ([`dialects/mova/seeds.md#depictives`](../../dialects/mova/seeds.md#depictives)): it is one answer instead of three, it matches EWT 2.18 (22 depictives with `advcl`, none with `acl` or `advmod`), and it marks secondary predication rather than manner. The predicand, if visible, is linked only in the enhanced graph.

(29) *John Lennon entered stardom being a murderer.* — `advcl(entered, murderer)` (EWT 2.18 newsgroup-groups.google.com_alt.animals_434fe80fb3577e8e_ENG_20031011_200300-0009).

| | obligatory (argument) | optional (adjunct) |
|---|---|---|
| resultative | `xcomp`: *painted the barn red* | — (all resultatives are `xcomp`) |
| depictive | `xcomp`: *keep it dry*, *elect her president* | `advcl`: *entered the room sad*, *came back dead* |

### 7.4 Copula-like verbs

*Become, remain, look, seem, sound, feel, get, grow, turn* take a predicative `xcomp`: *I became very **upset***, *So his name will remain **unspoken*** (EWT 2.18 weblog-blogspot.com_rigorousintuition_20050518101500_ENG_20050518_101500-0032). Only *be* is a copula (`cop`) in English UD ([`en/seeds/verbal/copula-be-only.md`](../../en/seeds/verbal/copula-be-only.md)); the others stay heads.

---

## 8. Clausal subjects: `csubj`, extraposition and *tough* constructions

### 8.1 The clausal subject

A clause in subject position is `csubj` of the predicate (UD guidelines `_en/dep/csubj.md`, https://universaldependencies.org/en/dep/csubj.html). With a copular predicate the head is the predicate:

(30) *Whether he was actually AWOL during this stint is unclear.* — `csubj(unclear, AWOL)` (EWT 2.18 weblog-juancole.com_juancole_20040823064025_ENG_20040823_064025-0032).
(31) *Saying goodnight to your family shows that you care.* — `csubj(shows, Saying)` (`$MOVA_DATA/parsedb/current.conllu`, school:everyday:119-bedroom-living-room-and-manners.lesson:13).
(32) *Mocking someone's religion or clothes, like a headscarf, is bullying.* — `csubj(bullying, Mocking)` (school:everyday:524-wheres-your-dad-questions-and-cruel-jokes.lesson:52).

A `nsubj` on a VERB is almost always an error: either the clause is a `csubj` or the "verb" is a noun ([`en/seeds/errors/clausal-subject-csubj.md`](../../en/seeds/errors/clausal-subject-csubj.md); EWT 2.18: 4 such tokens, GUM 8). A verbal noun (*The swimming was fun*, NN) is `nsubj`. A clausal subject of a passive is `csubj:pass` (EWT 2.18: 8).

### 8.2 Extraposition

A heavy clausal subject is usually postponed, with expletive *it* in the subject position. The clause keeps the `csubj` relation; *it* is `expl` (UD guidelines `_en/dep/expl.md`):

(33) *It was good to hear from you!* — `expl(good, It)`, `csubj(good, hear)` (EWT 2.18 email-enronsent15_01-0001).
(34) *It is very important that your students respect you.* — `expl(important, It)`, `csubj(important, respect)` (UD guidelines `_en/dep/csubj-outer.md`).
(35) *If you want to be sure, it is best to do both.* — `expl(best, it)`, `csubj(best, do)` (EWT 2.18 weblog-blogspot.com_alaindewitt_20060924104100_ENG_20060924_104100-0207).

The test that separates extraposition from an *it*-cleft: in extraposition the clause can replace *it* (*That we should decline is clear*); in a cleft it cannot (*\*That we want to help is John*) (Calude 2008, cited by `_en/dep/acl-relcl.md`, "It-clefts versus extraposition"; see [25-relative-clauses.md](25-relative-clauses.md) §8).

### 8.3 *Tough* constructions

In *This book is easy to read* the subject *book* is the understood object of *read*; the infinitive complements the adjective. Three analyses are on record:

| source | *X is easy to V* | *It is easy to V X* |
|---|---|---|
| UD overview `_en/specific-syntax.md`, "Tough-constructions" | `nsubj(hard, problem)`, `xcomp(hard, solve)` | `expl`, `csubj` |
| EWT 2.18 data | `ccomp` (sometimes `advcl`) | `expl`, `csubj` (4 exceptions of 107) |
| Mova en 0.4 parser (§13) | `xcomp` | `expl`, `xcomp` (wrong) |

(36) *His books are always hard to put down.* — `nsubj(hard, books)`, `ccomp(hard, put)` (EWT 2.18 email-enronsent11_01-0064).
(37) *no pet is really easy to take care of.* — `ccomp(easy, take)` (EWT 2.18 answers-20111108102204AAIivYN_ans-0004).

The seed [`en/seeds/errors/tough-vs-extraposition.md`](../../en/seeds/errors/tough-vs-extraposition.md) makes the two checkable: with `expl` the infinitive must be `csubj` (rule `en.errors.extraposed-infinitive-csubj`), with a nominal subject it must not be (`en.errors.tough-infinitive-not-csubj`). The BLiMP benchmark has separate paradigms for *tough* versus raising, and language models confuse them (Warstadt et al. 2020, "BLiMP: The Benchmark of Linguistic Minimal Pairs for English", *TACL* 8, https://aclanthology.org/2020.tacl-1.25).

The *for*-phrase with a *tough* adjective has two readings: an experiencer (*hard for me*, `nmod`/`obl` of the adjective) or the subject of the infinitive (*helpful for John to solve*, `mark(for)` + `nsubj`) (UD guidelines `_en/specific-syntax.md`).

---

## 9. Predicate clauses: `nsubj:outer` and `csubj:outer`

When the predicate of a copular sentence is itself a clause, the clause's predicate heads the sentence, *be* is its `cop`, and the subject of the outer clause gets the subtype `:outer`, to keep it apart from the clause's own subject (UD 2.10 amendment; `_en/dep/nsubj-outer.md`, https://universaldependencies.org/en/dep/nsubj-outer.html; `_en/dep/csubj-outer.md`):

(38) *The problem is that nobody listened.* — `nsubj:outer(listened, problem)`, `cop(listened, is)`, `mark(listened, that)`, `nsubj(listened, nobody)`.
(39) *I seem to recall the object was to smuggle explosives.* — `nsubj:outer(smuggle, object)` (EWT 2.18 weblog-blogspot.com_rigorousintuition_20050518101500_ENG_20050518_101500-0035).
(40) *The least we can do is pass this news along.* — `nsubj:outer(pass, least)` (EWT 2.18 email-enronsent40_01-0013).
(41) *But the difficulty was to place me on it.* — `nsubj:outer(place, difficulty)` (`$MOVA_DATA/parsedb/current.conllu`, book:pg503:3850).
(42) *To hike in the mountains is to experience the best of nature.* — `csubj:outer(experience, hike)` (UD guidelines `_en/dep/csubj-outer.md`).

The older overview `_en/specific-syntax.md` (lines 46–51 in the 2026-09-24 snapshot) still shows the copula as head with a `ccomp`; EWT 2.18 has 257 `:outer` subjects and no instance of the old analysis. Mova accepts the amendment ([`dialects/mova/seeds.md#clausal-predicate`](../../dialects/mova/seeds.md#clausal-predicate), rule `en.verbal.outer-subject-needs-cop` in [`en/seeds/verbal/outer-subject.md`](../../en/seeds/verbal/outer-subject.md)): the copula stays `cop` everywhere, and the old tree can be derived from `:outer` + `cop` if a target needs it. Pseudoclefts (*What John did was to play tennis*) use the same `:outer` subject ([25-relative-clauses.md](25-relative-clauses.md) §8).

---

## 10. Complements of adjectives and nouns

### 10.1 Adjectives

Adjectives take finite and non-finite complements like verbs: `ccomp` for a clause with its own subject, `xcomp` for a controlled infinitive.

| pattern | example | label |
|---|---|---|
| ADJ + *that*-clause | *I am certain **that** he did it* | `ccomp(certain, did)` |
| ADJ + *to*-infinitive (control) | *She is eager **to please*** | `xcomp(eager, please)` |
| ADJ + *whether*-clause | *I'm not sure **whether** it works* | `ccomp` |
| ADJ + PP | *proud **of** her* | `obl(proud, her)` |
| *tough* ADJ | *easy to read* | see §8.3 |

### 10.2 Nouns: content clauses are `acl`

Nouns never take core clausal arguments in UD (`_en/specific-syntax.md`, "Clausal core arguments"). A clause that spells out the content of *fact, idea, hope, belief, news, question, fear, doubt, rumor* is `acl`, with *that* as `mark` (SCONJ) — not `acl:relcl`, since there is no gap and *that* cannot be replaced by *which* (UD guidelines `_en/dep/acl.md`, which follows Huddleston and Pullum's "content clauses"; Curme 1931 calls them "attributive substantive clauses", ch. XIII, p. 199; seed [`en/seeds/nominal/content-clause-acl.md`](../../en/seeds/nominal/content-clause-acl.md)):

(43) *I admire the fact **that** you are honest.* — `acl(fact, honest)`, `mark(honest, that)`.
(44) *We have no useful information on whether users are at risk.* — `acl(information, risk)` with two marks, `mark(risk, on)` and `mark(risk, whether)` (UD guidelines `_en/dep/mark.md`).
(45) *I just want a simple way to get my discount.* — `acl(way, get)`: an infinitive without a gap corresponding to the noun is `acl`; *a bagel to eat* (with a gap) is `acl:relcl` ([25-relative-clauses.md](25-relative-clauses.md) §6).

In the enhanced graph EWT writes the marker into the label: `acl:that` 104 times, `acl:to` 621, `acl:of` 117, `acl:for` 48 (EWT 2.18, 2026-10-17).

| noun | content clause | relative clause |
|---|---|---|
| *the fact* | *the fact that he left* (`acl`, *that* = SCONJ `mark`) | *the fact that he mentioned* (`acl:relcl`, *that* = PRON `obj`) |
| *the question* | *the question whether we can do it* | *the question which he asked* |

---

## 11. Speech, quotation and reported speech

### 11.1 Indirect speech

Reported speech is a `ccomp` of the verb of saying (§3.1). Tense and person shift to the reporter's point of view (*He said he was tired*), and the addressee is `iobj` (*told me*) or `obl:to` (*said to me*).

### 11.2 Direct speech

A quotation whose verb of saying follows it is analysed in EWT and in Mova's trees as `ccomp` of the verb of saying when it fills the object role; a parenthetical reporting clause inside or after the quote can also be `parataxis` (UD guidelines `_en/dep/parataxis.md`: *The guy, John said, left early in the morning* — `parataxis(left, said)`). Mova's parser output for a tale sentence (Mova en 0.4, CPU, 2026-10-17):

```
# text = "Come here," said the Badger.
2  Come    come    VERB   6  ccomp
3  here    here    ADV    2  advmod
6  said    say     VERB   0  root
8  Badger  Badger  PROPN  6  nsubj
```

Subject-verb inversion after a quote (*said the Badger*) is ordinary in narrative English and leaves the relations unchanged; the frames reader uses it to find the speaker (docs/pragmatics.md, "Roles, more": "the speaker after a quote").

### 11.3 Quotative *be like*

In spoken and informal English, *be like* introduces quoted speech or thought: *and I'm like, no way*. EWT annotates the 7 cases of *be* with `ccomp` as quotative *be like* and one *there are hints … that* ([`dialects/mova/seeds.md#clausal-predicate`](../../dialects/mova/seeds.md#clausal-predicate)). Quotative *go* and *be all* are rarer.

### 11.4 In the Torah English

The JPS 1917 translation introduces speech with *said* + colon + quote, which Mova's trees analyse as `ccomp`:

(46) *And God said: 'Let there be light.' And there was light.* — `ccomp(said, Let)`, then `conj(said, was)` (`$MOVA_DATA/corpora/judaism/parsedb/current.conllu`, jps:GEN:2). Inside the quote the tree has `ccomp(Let, be)` with `expl(Let, there)`; by the UD analysis of *let* (§6.2) the complement should be `xcomp(Let, be)` with *there* as the raised expletive.

---

## 12. The enhanced graph: `nsubj:xsubj`

The basic tree lacks the subject arc of a controlled or raised predicate. Enhanced UD adds it, from the `xcomp` to the controller, with the subtype `:xsubj` (UD guidelines `_u-overview/enhanced-syntax.md`, "Controlled/raised subjects", https://universaldependencies.org/u/overview/enhanced-syntax.html; origin in Schuster and Manning 2016, "Enhanced English Universal Dependencies: An Improved Representation for Natural Language Understanding Tasks", LREC, https://aclanthology.org/L16-1376):

```
# Mary wants to buy a book .         (UD guidelines, enhanced-syntax.md)
ID FORM   HEAD DEPREL DEPS
1  Mary   2    nsubj  2:nsubj|4:nsubj:xsubj
2  wants  0    root   0:root
3  to     4    mark   4:mark
4  buy    2    xcomp  2:xcomp
5  a      6    det    6:det
6  book   4    obj    4:obj
```

With object control the arc starts at the object: *Mary made me buy the house* — `3:obj` on *me* gets `4:nsubj:xsubj`. With a passive infinitive the label is `nsubj:pass:xsubj`. Raising and control get the same arc; the enhanced graph, like the basic tree, does not say whether the higher verb also assigns a role.

From EWT 2.18:

```
# text = Now the Electoral Commission is refusing to punish people for mere past Baath Party membership.
#        (EWT 2.18 weblog-juancole.com_juancole_20051126063000_ENG_20051126_063000-0034)
4  Commission  6  nsubj  6:nsubj|8:nsubj:xsubj
6  refusing    0  root   0:root
8  punish      6  xcomp  6:xcomp
```

Chains propagate: in *I seem to recall …* the subject *I* gets `nsubj:xsubj` on *recall*; a controlled predicate inside another controlled predicate gets the arc from the top controller. The arc also covers objective complements and resultatives (*left 6 dead*: `nsubj:xsubj(dead, 6)`, example 28).

Known limits of the automatic enhancement: quantifiers with control (*Everybody wants to leave*) produce a wrong meaning if the arc is read as coreference; coordination of verb phrases with different objects is not propagated (Schuster and Manning 2016, discussed in the digest `docs/papers/ud/ud-core.md`, §4).

**Mova does not write enhanced dependencies yet**: the `en` parser fills the DEPS column with `_` (see the output in §13), and the dialect has no rule that builds DEPS (`docs/dialect.md`: "There are no actions for DEPS"). The pragmatics reader computes the same information directly from the basic tree (§14).

---

## 13. Mova's parser on complements: a test

Seventeen test sentences, parsed by the `en` annotator (Mova en 0.4, CPU, 2026-10-17; `en annotate $MOVA_DATA/.../en.bin`). The parser is the averaged-perceptron arc-standard parser described in [`docs/architecture.md`](../../docs/architecture.md). Right where it agrees with the UD guidelines and EWT; the errors are listed with the expected analysis.

| sentence | Mova en 0.4 | UD / EWT | verdict |
|---|---|---|---|
| *Sue asked George to respond to her offer.* | `iobj(asked, George)`, `xcomp(asked, respond)` | same | right |
| *I expect him to win.* | `obj(expect, him)`, `xcomp(expect, win)` | same | right |
| *She promised me to come early.* | `iobj(promised, me)`, `xcomp(promised, come)` | same | right (control by the subject is not in the tree) |
| *The cat seems to be in pain.* | `xcomp(seems, pain)`, `cop(pain, be)` | same | right |
| *I saw him leave.* | `obj(saw, him)`, `xcomp(saw, leave)` | same | right |
| *I enjoy swimming in the lake.* | `xcomp(enjoy, swimming)` | same | right |
| *I wonder whether it works.* | `ccomp(wonder, works)`, `mark(works, whether)` | same | right |
| *He painted the barn red.* | `xcomp(painted, red)` | same | right |
| *The problem is that nobody listened.* | `nsubj:outer(listened, problem)` | same | right |
| *"Come here," said the Badger.* | `ccomp(said, Come)` | same | right |
| *I consider him a fool.* | `obj(consider, him)`, **`obj(consider, fool)`** | `xcomp(consider, fool)` | wrong: two objects |
| *They elected her president.* | **`nmod:poss(president, her)`**, `obj(elected, president)` | `obj(elected, her)`, `xcomp(elected, president)` | wrong: *her* read as a possessive |
| *She entered the room sad.* | **`xcomp(entered, sad)`** | `advcl` (Mova seed) | wrong label: depictive as argument |
| *This book is easy to read.* | `xcomp(easy, read)` | `xcomp` (overview) / `ccomp` (EWT) | follows the overview, not EWT |
| *It is easy to please John.* | `expl(easy, It)`, **`xcomp(easy, please)`** | `csubj(easy, please)` | wrong |
| *That he lied is obvious.* | **`nsubj(obvious, That)`**, **`acl:relcl(That, lied)`** | `csubj(obvious, lied)`, `mark(lied, That)` | wrong: *that* read as a demonstrative pronoun |
| *the fact that he left surprised us.* | root *left*, **`xcomp(left, surprised)`** | `acl(fact, left)`, root *surprised* | wrong: garden path |

Four of the seven errors involve a second predicative nominal or a clause where the parser expected a noun phrase. The seed rules already detect some of them as gate violations on silver data (`en.verbal.xcomp-no-subject`, `en.errors.extraposed-infinitive-csubj`, `en.verbal.ccomp-no-obj`), but the parser does not consult them at parse time. The GPU parser (`gparse`) was not run for this chapter.

---

## 14. From complements to frames

Mova's pragmatics turns each predicate into a frame (`prag::frames`, [`prag/src/frames.rs`](../../prag/src/frames.rs); docs/pragmatics.md). Complements decide who does what in the lower frame and how the frames link:

| tree | frame reading | code |
|---|---|---|
| `xcomp` of a control verb, no own subject | the lower frame's agent = the higher frame's agent | `frames.rs`, the agent-inheritance branch for `Conj`, `Xcomp`, `Advcl`, `Ccomp` |
| `xcomp` of an object-control verb (`OBJ_CONTROL`) | the lower frame's agent = the higher frame's patient: *make the mould grow*, *tell Alice to go* | same branch, `OBJ_CONTROL` (34 verbs) |
| `xcomp`/`advcl`/`acl` with *to* after a non-control verb | purpose: the higher frame's `cause` is the lower frame | the `m("to")` branch; switches `purpose-wide`, `purpose-acl` |
| `ccomp`/`csubj` of a verb of saying or knowing | the lower frame is quoted: `quote` of a `Say(act)` frame | the `is_say` / `CONTENT` branch |
| a quote without a verb of saying | carried by the nearest verb of saying or knowing | "quoted frames not yet carried" |
| VerbNet frame `NP V NP S_INF` etc. | roles of the matrix arguments: `alice=Recipient clause=Topic` | [`prag/src/vn.rs`](../../prag/src/vn.rs) |

The reader prints these decisions. For a short text (Mova `world frames`, model en 0.4, CPU, 2026-10-17):

```
#6 Say(assert) queen tell to=alice | next→#8
     roles by vn transfer_mesg-37.1.1-1-1 «NP V NP S_INF»: agent=Agent alice=Recipient clause=Topic
#7 Event queen run
#8 Say(assert) bill say | says→#9 next→#11
     roles by vn say-37.7-1-1 «NP V that S»: agent=Agent
#9 Fact(is) house be attrs=empty
```

The input was *The Queen told Alice to run. Bill said that the house was empty.* Frame #8 and #9 are right: *say* is a Say frame carrying the `ccomp` as its quote. Frame #7 is wrong: *Alice* should run. VerbNet made *Alice* the Recipient (an `iobj`-like role, rightly), and the object-control rule reads the controller from the higher frame's *patient*, which is now empty, so it falls back to the agent. This is a case where the `iobj` + `xcomp` analysis of §2.3 needs a matching rule in the reader: the controller of *tell / ask / persuade X to* is the recipient.

### Not yet

- **Enhanced dependencies** (`nsubj:xsubj` and the rest): not produced by the `en` parser, not built by the dialect converters; the frames reader recomputes control from the basic tree.
- **Raising versus control**: the reader does not know that *seem / appear / tend* assign no role to their subject; *The cat seems to be in pain* yields a frame for *seem* with *cat* as its agent. The lexicon seed exists ([`verb-raising-control.md`](../../en/seeds/lexicon/verb-raising-control.md)), the reader does not use it.
- **Recipient control** (*tell X to*): §14 above.
- **Promise-type subject control** with an object: the reader treats *promise* as object control if it is in `OBJ_CONTROL` (it is not, so subject control is applied — right by accident of the list).
- **Objective complements and depictives** as attributes of the object or subject: the reader writes a copula's adjective into `attrs`, but an `xcomp` adjective of *keep / find / paint* is not yet attached to the object's frame as a resulting state.
- **Non-finite `ccomp`** (*said to start digging*): read as purpose or control, not as an open instruction.
- **Parser errors of §13**: second predicative nominals, *it*-extraposition with `csubj`, clause-initial *that*, garden-path content clauses.

---

## In Mova

- Annotation conventions and checks: [`en/seeds/verbal/open-complement-xcomp.md`](../../en/seeds/verbal/open-complement-xcomp.md), [`clausal-complement.md`](../../en/seeds/verbal/clausal-complement.md), [`objective-complement.md`](../../en/seeds/verbal/objective-complement.md), [`outer-subject.md`](../../en/seeds/verbal/outer-subject.md), [`gerund-participle.md`](../../en/seeds/verbal/gerund-participle.md), [`purpose-clause.md`](../../en/seeds/verbal/purpose-clause.md), [`core-objects.md`](../../en/seeds/verbal/core-objects.md), [`indirect-object.md`](../../en/seeds/verbal/indirect-object.md), [`expletive-it.md`](../../en/seeds/verbal/expletive-it.md); errors: [`tough-vs-extraposition.md`](../../en/seeds/errors/tough-vs-extraposition.md), [`clausal-subject-csubj.md`](../../en/seeds/errors/clausal-subject-csubj.md), [`xcomp-no-subject.md`](../../en/seeds/errors/xcomp-no-subject.md), [`iobj-verb-classes.md`](../../en/seeds/errors/iobj-verb-classes.md); nominal: [`content-clause-acl.md`](../../en/seeds/nominal/content-clause-acl.md).
- Lexicon seeds: [`verb-raising-control.md`](../../en/seeds/lexicon/verb-raising-control.md), [`verb-iobj-licensors.md`](../../en/seeds/lexicon/verb-iobj-licensors.md), [`verb-bare-infinitive.md`](../../en/seeds/lexicon/verb-bare-infinitive.md), [`verb-gerund-object.md`](../../en/seeds/lexicon/verb-gerund-object.md), [`verb-infinitive-object.md`](../../en/seeds/lexicon/verb-infinitive-object.md), [`verb-copula-like.md`](../../en/seeds/lexicon/verb-copula-like.md), [`verb-no-passive.md`](../../en/seeds/lexicon/verb-no-passive.md).
- Dialect decisions: [`dialects/mova/seeds.md#iobj-sole`](../../dialects/mova/seeds.md#iobj-sole), [`depictives.md`](../../dialects/mova/seeds.md#depictives), [`clausal-predicate.md`](../../dialects/mova/seeds.md#clausal-predicate), [`to-phrase-obl.md`](../../dialects/mova/seeds.md#to-phrase-obl), [`subjunctive.md`](../../dialects/mova/seeds.md#subjunctive).
- Parser: [`en/src/parse.rs`](../../en/src/parse.rs) (arc-standard transitions), [`en/src/expert.rs`](../../en/src/expert.rs) (the seed rule engine that checks the conventions above).
- Frames: [`prag/src/frames.rs`](../../prag/src/frames.rs) (control, purpose, quotes), [`prag/src/vn.rs`](../../prag/src/vn.rs) (VerbNet frames with `S_INF`, `that S`, `NP.recipient`), [`prag/src/roles.rs`](../../prag/src/roles.rs).
- Not yet: see the list at the end of §14.

## Sources

- Brown, Goold. 1851. *The Grammar of English Grammars*. `books/b/br/brown-1851-grammar-english-grammars`.
- Calude, Andreea S. 2008. "Clefting and extraposition in English." *ICAME Journal* 32. http://icame.uib.no/ij32/ij32_7_34.pdf (cited via the UD guidelines).
- Curme, George O. 1931. *A Grammar of the English Language*, vol. III: *Syntax*. Boston: D. C. Heath. `books/c/cu/curme-1931-grammar-english-language-vol` (ch. XIII, XV).
- de Marneffe, Marie-Catherine, Christopher D. Manning, Joakim Nivre, Daniel Zeman. 2021. "Universal Dependencies." *Computational Linguistics* 47(2). https://aclanthology.org/2021.cl-2.11.
- Huddleston, Rodney, and Geoffrey K. Pullum. 2002. *The Cambridge Grammar of the English Language*. Cambridge UP (not open; cited through the UD guidelines).
- Jespersen, Otto. 1909–1949. *A Modern English Grammar on Historical Principles*, Part V (ch. XII, XVIII). `books/j/je/jespersen-1914-modern-english-grammar-historical`, text-1.
- Poutsma, Hendrik. 1904–1929. *A Grammar of Late Modern English*. `books/p/po/poutsma-1904-grammar-late-modern-english`.
- Reed, Alonzo, and Brainerd Kellogg. 1877. *Higher Lessons in English*. `books/r/re/reed-1877-higher-lessons-english`.
- Schuster, Sebastian, and Christopher D. Manning. 2016. "Enhanced English Universal Dependencies: An Improved Representation for Natural Language Understanding Tasks." LREC 2016. https://aclanthology.org/L16-1376.
- Universal Dependencies guidelines, snapshot 2026-09-24: `_en/dep/xcomp.md`, `ccomp.md`, `csubj.md`, `csubj-outer.md`, `nsubj-outer.md`, `iobj.md`, `acl.md`, `advcl.md`, `mark.md`, `expl.md`, `parataxis.md`, `acl-relcl.md`; `_en/specific-syntax.md`; `_u-overview/complex-syntax.md`, `enhanced-syntax.md`; `changes.md`. https://universaldependencies.org/en/ and https://universaldependencies.org/u/overview/.
- UD English EWT 2.18 (Silveira et al. 2014; CC BY-SA 4.0), examples cited by `sent_id`. https://github.com/UniversalDependencies/UD_English-EWT.
- Warstadt, Alex, et al. 2020. "BLiMP: The Benchmark of Linguistic Minimal Pairs for English." *TACL* 8. https://aclanthology.org/2020.tacl-1.25.
- Research library digest: `docs/papers/ud/ud-core.md` (UD core, enhanced UD, English registry).
- Mova: `docs/pragmatics.md`, `docs/dialect.md`, `docs/architecture.md`; parse database `$MOVA_DATA/parsedb/current.conllu`; Torah English trees `$MOVA_DATA/corpora/judaism/parsedb/current.conllu`.
