# 26. Questions

English marks questions by word order (subject–auxiliary inversion, the fronted wh-phrase), by the dummy auxiliary *do*, by intonation, and by special forms (tags, *or not*, *whether*). This chapter describes yes/no, wh-, alternative, embedded, echo and tag questions, biased and rhetorical questions and the related exclamatives; it shows how Universal Dependencies annotates each of them (the dependencies are those of the statement, only the order changes), where the English guidelines, EWT, Mova's dialect and Mova's parser differ, and how Mova's pragmatics labels the form of a sentence (`wh`, `q`) and reads a question into a frame with an asked slot — after putting do-support questions back into statement order, because the parser misreads them.

Contents

1. [Overview: kinds of questions](#1-overview-kinds-of-questions)
2. [Yes/no questions: inversion and do-support](#2-yesno-questions-inversion-and-do-support)
3. [Wh-questions](#3-wh-questions)
4. [Preposition stranding and pied-piping in questions](#4-preposition-stranding-and-pied-piping-in-questions)
5. [Embedded questions](#5-embedded-questions)
6. [Alternative questions](#6-alternative-questions)
7. [Tag questions](#7-tag-questions)
8. [Declarative, echo and in-situ questions](#8-declarative-echo-and-in-situ-questions)
9. [Biased, negative and rhetorical questions](#9-biased-negative-and-rhetorical-questions)
10. [Exclamatives](#10-exclamatives)
11. [Archaic and biblical questions](#11-archaic-and-biblical-questions)
12. [Mova's parser on questions](#12-movas-parser-on-questions)
13. [Questions in Mova's pragmatics: form, act and the asked slot](#13-questions-in-movas-pragmatics-form-act-and-the-asked-slot)

Cross-references: the auxiliaries and *do* in [21-clause-structure.md](21-clause-structure.md); negation in questions in [27-negation.md](27-negation.md); free relatives versus embedded questions in [25-relative-clauses.md](25-relative-clauses.md) §7; indirect speech acts (questions used as requests, offers, assertions) in [31-speech-acts.md](31-speech-acts.md); questions under discussion in [36-discourse-relations.md](36-discourse-relations.md); VP ellipsis in tags and short answers in [29-ellipsis-word-order-information-structure.md](29-ellipsis-word-order-information-structure.md).

---

## 1. Overview: kinds of questions

Jespersen divides questions into **nexus-questions**, which put the truth of a whole subject–predicate link in question (yes/no), and **x-questions**, which ask for an unknown "quantity" named by an interrogative word (Jespersen 1924, *The Philosophy of Grammar*, p. 303, `books/j/je/jespersen-1924-philosophy-grammar`; MEG V, ch. XXV "Questions", pp. 480 ff., `books/j/je/jespersen-1914-modern-english-grammar-historical`, text-1). Modern semantics treats a question as the set of its possible answers: *Is it raining?* denotes {it is raining, it is not raining} (Hamblin 1973, Groenendijk and Stokhof 1984, as summarised by Krifka and Trinh 2025, "Introduction", in Trinh et al. (eds.), *Biased Questions: Experimental Results and Theoretical Modelling*, Language Science Press, `books/k/kr/krifka-2025-biased-questions-experimental-results`; CC BY 4.0).

| kind | form | example | answer |
|---|---|---|---|
| yes/no (polar) | inversion or *do*-support, rising intonation | *Did you see the fox?* | *yes / no* |
| wh- (constituent, x-question) | wh-phrase first; inversion unless the wh-phrase is the subject | *Where do you live?* | a constituent |
| alternative | polar form with *or* | *Do you want tea or coffee?* | one of the alternatives |
| embedded | *whether / if* or a wh-phrase, no inversion | *I wonder where he lives* | — (part of a statement) |
| tag | statement + reduced polar question | *It is cold, isn't it?* | confirmation |
| declarative | statement order, rising intonation | *You're leaving?* | *yes / no* |
| echo / in situ | the wh-word in place | *You saw what?* | a repeat of the constituent |
| rhetorical | any question form | *Who cares?* | none expected |
| exclamative | *what a* / *how* + statement order | *What a hidden gem!* | none |

UD annotates all of these with the relations of the corresponding statement. Inversion changes the order of `aux`/`cop` and `nsubj`, a fronted wh-phrase keeps its clause-internal relation, and nothing in the basic tree says "question" except the order and the final `?` (seed [`en/seeds/verbal/questions-inversion.md`](../../en/seeds/verbal/questions-inversion.md); UD guidelines `_en/dep/cop.md`, https://universaldependencies.org/en/dep/cop.html). The interrogative wh-words carry `PronType=Int` (relative ones `PronType=Rel`), and the sentence's mood is not a feature: `Mood=Ind` on the finite auxiliary is the same in *Did you see?* and *You did see.*

Frequencies in Mova's parse database (pragmatic labels of `$MOVA_DATA/parsedb/prag.tsv`, 311 101 sentences of school lessons, tales, dictionary entries and maths, counted 2026-10-17): form `wh` 6 515, `q` 4 966, `decl` 257 228, `imp` 14 587, `excl` 2 893. In the Torah English (JPS 1917, 6 065 verses' sentences): `q` 125, `wh` 79.

---

## 2. Yes/no questions: inversion and do-support

### 2.1 Subject–auxiliary inversion

A polar question puts the first auxiliary (or copular *be*) before the subject (Reed and Kellogg 1877, *Higher Lessons in English*, Lesson 55, `books/r/re/reed-1877-higher-lessons-english`; Brown 1851, Part II, ch. VI, "V. Form of Question", `books/b/br/brown-1851-grammar-english-grammars`):

| statement | question | UD (both) |
|---|---|---|
| *You have sent this.* | *Have you sent this to congress yet?* | `aux(sent, Have)`, `nsubj(sent, you)` |
| *The distinction is accurate.* | *Is the distinction accurate and valid?* | `cop(accurate, Is)`, `nsubj(accurate, distinction)` |
| *We are going to attend.* | *Are we going to attend?* | `aux(going, Are)` |
| *He can swim.* | *Can he swim?* | `aux(swim, Can)` |

(1) *Have you sent this to congress yet?* — `aux(sent, Have)` (EWT 2.18 weblog-blogspot.com_rigorousintuition_20060511134300_ENG_20060511_134300-0054).
(2) *Is the distinction accurate and valid?* — `cop(accurate, Is)` (EWT 2.18 weblog-blogspot.com_rigorousintuition_20060511134300_ENG_20060511_134300-0223).
(3) *Is the solution a democratic Arab world?* — `cop(world, Is)`, `nsubj(world, solution)`: in an equative copular question the predicate is the second nominal, as in the statement (EWT 2.18 weblog-blogspot.com_alaindewitt_20060924104100_ENG_20060924_104100-0240).

Only the first auxiliary inverts: *Has he been working?* — *Has* before the subject, *been* after it.

### 2.2 Do-support

When the statement has no auxiliary, *do* is inserted and carries tense and agreement; the main verb becomes a bare infinitive (`VB`, `VerbForm=Inf`) (seed [`en/seeds/verbal/do-support.md`](../../en/seeds/verbal/do-support.md), rule `en.verbal.do-support-base`: after do-support the main verb is not `VBZ/VBP/VBD/VBG`). *Do* never combines with modals or perfect *have* (*\*Does he can swim?*).

```
# text = Do we expect to have ONE EPC contract, or several?     (EWT 2.18 email-enronsent26_01-0027)
ID FORM    LEMMA  UPOS HEAD DEPREL
1  Do      do     AUX  3    aux
2  we      we     PRON 3    nsubj
3  expect  expect VERB 0    root
```

*Do* as a main verb (*What do you do?*) is VERB with an object; the auxiliary *do* is AUX with `aux` ([`en/seeds/verbal/aux-no-object.md`](../../en/seeds/verbal/aux-no-object.md)).

### 2.3 Archaic main-verb inversion

Before do-support became obligatory (Early Modern English), lexical verbs inverted: *Have you any thoughts?*, *What say ye?*, *Knowest thou …?* British English keeps *Have you (got) …?* with possessive *have*. The seed `questions-inversion.md` notes these as a regular pattern of archaic and poetic text; §11 shows how Mova's trees handle them in the Torah English.

---

## 3. Wh-questions

### 3.1 Subject and non-subject wh-phrases

The wh-phrase stands first and keeps its role. If it is the subject (or inside the subject), there is no inversion and no do-support; otherwise the auxiliary inverts and *do* appears when needed (seed `questions-inversion.md`; Jespersen MEG V, ch. XXV §25.6–25.8 on *do* in questions):

| role of the wh-phrase | example | UD | inversion |
|---|---|---|---|
| subject | *Who has the 2000 tax return file for 969?* | `nsubj(has, Who)` | no |
| subject determiner | *Which rat cage is better?* | `det(cage, Which)`, `nsubj(better, cage)` | no |
| object | *What can we do about it?* | `obj(do, What)` | yes |
| object with do | *What do you do?* | `obj(do, What)`, `aux(do, do)` | yes, do-support |
| adverbial | *Where do we vote?* | `advmod(vote, Where)` | yes |
| oblique | *To whom do you want it faxed?* | `obl(faxed, whom)`, `case(whom, To)` | yes |
| predicate of *be* | *What is long term viability?* | root *What*, `cop(What, is)`, `nsubj(What, viability)` | (copula before subject) |
| degree | *How much better does it get?* | `advmod(much, How)` | yes |
| reason | *Why did the crow open her beak?* | `advmod(open, Why)` | yes |

Examples from EWT 2.18:

(4) *Who has the 2000 tax return file for 969?* — `nsubj(has, Who)` (email-enronsent09_01-0002).
(5) *What can we do about it?* — `obj(do, What)`, `aux(do, can)` (weblog-blogspot.com_alaindewitt_20060924104100_ENG_20060924_104100-0197).
(6) *Where in the states could I go?* — `advmod(go, Where)` with its own `obl` *in the states* (answers-20111108102445AA5mfrq_ans-0003).
(7) *Which do you prefer?* — `obj(prefer, Which)` (email-enronsent24_01-0020).
(8) *What is the best way of doing it?* — root *What* with `cop(What, is)` (answers-20111107212131AACQ65F_ans-0015).
(9) *What about 10:30 my time?* — a verbless wh-question: root *What*, `nmod(What, 10:30)` (email-enronsent17_01-0052).

The copular *What is X?* puts the wh-word at the root: UD makes the predicate of a copular clause its head, and in a question the predicate is what is asked (UD guidelines `_en/dep/cop.md`: *What is that?* — `cop(What, is)`, `nsubj(What, that)`).

### 3.2 Long-distance wh-questions

A wh-phrase can be extracted from an embedded clause: *What did you say that he bought?* — `obj(bought, What)`, `ccomp(say, bought)`. As with relatives the dependency is unbounded ([25-relative-clauses.md](25-relative-clauses.md) §2). Subject extraction across *that* is ungrammatical in standard English (*\*Who did you say that left?*, the "*that*-trace" effect), but fine without *that*.

### 3.3 Multiple wh-questions

*Who bought what?* fronts only one wh-phrase; the other stays in place: `nsubj(bought, Who)`, `obj(bought, what)`. Coordinated wh-phrases are possible: *Where and to whom do you want it faxed?* — `advmod(faxed, Where)`, `conj(Where, whom)` with enhanced `obl:to` from *faxed* (EWT 2.18 email-enronsent24_01-0064).

### 3.4 Wh-word inventory

| word | UPOS | features | asks for |
|---|---|---|---|
| *who* | PRON | `PronType=Int`; Mova `Case=Nom/Acc` | a person (subject or object) |
| *whom* | PRON | `PronType=Int`, `Case=Acc` (lemma *who* in Mova) | a person (object, after a preposition) |
| *whose* | PRON (DET-like) | `Poss=Yes`; Mova `Case=Gen` | a possessor |
| *what* | PRON / DET | `PronType=Int` | a thing, a property, an action (*what … do*) |
| *which* | PRON / DET | `PronType=Int` | a choice from a known set |
| *where, when, why, how* | ADV (WRB) | `PronType=Int` | place, time, reason, manner or degree |
| *how many / how much* | ADV + ADJ | | a quantity |

Mova's case rules for *who* apply to questions as to relatives ([`dialects/mova/seeds.md#wh-case`](../../dialects/mova/seeds.md#wh-case)): *Who called you?* (`Nom`), *Who did you call?* (`Acc`).

---

## 4. Preposition stranding and pied-piping in questions

In questions, as in relatives, a preposition either moves with the wh-word (formal: *To whom are you talking?*) or stays at the gap (usual: *Who are you talking to?*). EWT attaches a stranded preposition as `case` of the fronted wh-word, so the arc goes back across the clause:

```
# text = What are you searching for?
#        (EWT 2.18 newsgroup-groups.google.com_eHolistic_417b09016150421f_ENG_20051120_175800-0003)
ID FORM       UPOS HEAD DEPREL DEPS
1  What       PRON 4    obl    4:obl:for
2  are        AUX  4    aux    4:aux
3  you        PRON 4    nsubj  4:nsubj
4  searching  VERB 0    root   0:root
5  for        ADP  1    case   1:case
```

The same in EWT 2.18: *what are you having problems with?* (answers-20111108103957AAcF3iZ_ans-0023), *what is a kimberwick bit used for?* (answers-20111108110437AA5JNkr_ans-0001), and in embedded questions *Can you find out who I need to send it to in HR?* (`case(who, to)`, email-enronsent25_01-0002).

Mova's parser does not follow this convention for every preposition: for *Whom were you talking to?* it gives `fixed(talking, to)` (Mova en 0.4, CPU, 2026-10-17) — the preposition treated as part of a fixed expression on the verb rather than the `case` of *whom*.

---

## 5. Embedded questions

An embedded (indirect) question is a complement clause: `ccomp` of a verb or adjective (*wonder, ask, know, tell, be sure*), `acl` of a noun (*the question whether …*), or `csubj` (*Whether he lied is beside the point*). It has statement order and no do-support; a yes/no question is introduced by *whether* or *if* (`mark`), a wh-question by its wh-phrase in its role ([23-complementation.md](23-complementation.md) §3.2; UD guidelines `_en/dep/acl-relcl.md`, "Free relatives versus interrogative complement clauses").

(10) *I wonder if they were going down a hill!* — `ccomp(wonder, going)`, `mark(going, if)` (EWT 2.18 reviews-135800-0005).
(11) *Now, people wonder if Google can even survive.* — the same (EWT 2.18 weblog-blogspot.com_marketview_20040611132900_ENG_20040611_132900-0004).
(12) *Could you tell me what deal tickets you are seeing?* — `ccomp(tell, seeing)`, `det(tickets, what)`, `obj(seeing, tickets)` (EWT 2.18 email-enronsent11_01-0004).
(13) *I don't know where he lives, who he is, how old he is, …, whether he has kids, or why he is here.* — a coordination of embedded questions under one `ccomp` (UD guidelines, `_en/dep/acl-relcl.md`).

*Whether* and *if* differ in bias: Bolinger observed that a report of *Will you marry me?* reads more naturally with *if* than with *whether*, and that *whether* goes with alternative questions — the speaker has weighed both answers (Bolinger 1978, as discussed in Krifka and Trinh 2025, pp. vi–vii, `books/k/kr/krifka-2025-biased-questions-experimental-results`). Only *whether* takes *or not* directly (*whether or not*, `fixed` in EWT) and introduces subject clauses and infinitives (*whether to go*).

Embedded questions with inversion appear in informal and Irish/Indian English (*I asked him where was he going*); EWT has a few, and the tree is the same as with statement order.

---

## 6. Alternative questions

An alternative question offers a closed set of answers joined by *or*; intonation rises on the first alternative and falls on the last. The same string can be a polar question about the disjunction (*Do you want tea or coffee?* — "yes, either") (Krifka 2022, "Modelling Questions in Commitment Spaces", in Cordes (ed.), *Asking and Answering*, Narr Francke Attempto, `books/c/co/cordes-2022-asking-answering`; CC BY-SA 4.0). UD does not separate the two readings: both have `conj` + `cc(or)`.

(14) *Do we expect to have ONE EPC contract, or several?* — `conj(contract, several)`, `cc(several, or)`; the enhanced graph gives *several* the `obj` role too (EWT 2.18 email-enronsent26_01-0027).
(15) *Do we intend to reference a particular manufacturer, or should this be more generic?* — coordination of two full questions: `conj(intend, generic)` (EWT 2.18).

*Or not* questions (*Will you help me or not?*) are alternative questions with a "cornering" effect: the hearer is pressed to choose (Biezma 2009, as summarised by Krifka and Trinh 2025, p. vii). They are odd as conversation starters (*Nice to meet you! #Do you like to play golf or not?*).

---

## 7. Tag questions

A tag question is a reduced polar question attached to a statement (the "anchor"): an auxiliary or copula copied from the anchor, usually of opposite polarity, and a pronoun copy of the subject: *It is cold, isn't it?*, *You don't mind, do you?* The tag shows the speaker's bias toward the anchor; Bill and Koev (2025, "Bias in tag questions", in Trinh et al. (eds.), *Biased Questions*, pp. 141–174, `books/k/kr/krifka-2025-biased-questions-experimental-results`) separate weak from strong and optional from obligatory biases of tags. Invariant tags (*right?*, *eh?*, *innit?*, *yeah?*) have no copying.

| tag | anchor | example | meaning |
|---|---|---|---|
| reversed polarity, falling | positive | *It's raining, isn't it?* ↘ | asks for agreement |
| reversed polarity, rising | positive | *It's raining, isn't it?* ↗ | a real question with a bias |
| reversed polarity | negative | *It's not raining, is it?* | expects *no* |
| constant polarity | positive | *So you're the new boss, are you?* | surprise, sarcasm |
| invariant | any | *Bryan, you're in, right?* | asks for confirmation |

UD has no relation for tags. EWT annotates a copied tag as `parataxis` of the anchor's predicate, with the auxiliary as the head of the tag (the predicate is elided), and an invariant tag as `discourse`:

```
# text = They really are milking the incompetence angle though, aren't they?
#        (EWT 2.18 weblog-blogspot.com_rigorousintuition_20060511134300_ENG_20060511_134300-0101)
ID FORM     UPOS  HEAD DEPREL
4  milking  VERB  0    root
10 are      AUX   4    parataxis
11 n't      PART  10   advmod
12 they     PRON  10   nsubj

# text = Bryan, you're in, right?       (EWT 2.18 email-enronsent23_13-0003)
5  in       ADV   0    root
7  right    INTJ  5    discourse
```

A French-led proposal adds `discourse:tag` and related subtypes for spoken treebanks, and the UniDive working group is discussing parataxis, tag questions and backchannels for all languages (UD issues #1290, #1280, #1273, #1289). Mova **waits** on these subtypes: they would be more informative (a tag is not ordinary parataxis), but there is no universal decision yet and no English gold to train on ([`dialects/mova/seeds.md#spoken-subtypes`](../../dialects/mova/seeds.md#spoken-subtypes)).

The tag's own predicate is elided (VP ellipsis); the auxiliary is promoted to head, as in other VP-ellipsis cases ([`en/seeds/verbal/vp-ellipsis.md`](../../en/seeds/verbal/vp-ellipsis.md); [29-ellipsis-word-order-information-structure.md](29-ellipsis-word-order-information-structure.md)).

---

## 8. Declarative, echo and in-situ questions

### 8.1 Declarative questions

A statement with rising intonation asks for confirmation: *You're leaving?* In writing only the question mark shows it. Krifka (2022, §2.10 "Declarative Questions", in Cordes (ed.), *Asking and Answering*) models them as a speaker's commitment offered for the addressee's confirmation, unlike a neutral polar question; they presuppose evidence for the proposition (*It's raining?* after seeing someone come in wet). UD gives them the statement tree. Mova's pragmatic form labeller relies on the `?` and the absence of inversion: such sentences are form `q` with no inverted auxiliary (features `qm`, `inv`, `aux1` in [`prag/src/feats.rs`](../../prag/src/feats.rs)).

### 8.2 Echo and in-situ questions

An echo question repeats a previous utterance with the unclear part replaced by a stressed wh-word: *You saw WHAT?* Quiz-show and exam questions also keep the wh-word in place (*The capital of France is what?*). The tree is the statement's: `obj(saw, what)`. Mova en 0.4 parses *You saw what?* right (`obj(saw, what)`, Mova en 0.4, CPU, 2026-10-17). The frames reader's statement-order rewriting (§13) produces exactly this in-situ form from a do-support question.

### 8.3 Fragment questions

Short questions are common in dialogue: *Really?*, *Why?*, *And you?*, *What about 10:30 my time?* The fragment's head is the root, and the missing clause is understood from context; Mova's pragmatics labels many of them form `frag` with act `ask` (example: `book:pg11339:1054`, `frag` / `ask` / indirect / hedged in `prag.tsv`).

---

## 9. Biased, negative and rhetorical questions

### 9.1 Bias

A polar question can be **biased**: the speaker expects one answer. The bias comes from the speaker's prior belief (epistemic bias) or from evidence in the context (evidential bias). The volume edited by Trinh, Benz, Goodhue, Yatsushiro and Krifka (2025, `books/k/kr/krifka-2025-biased-questions-experimental-results`) lists the forms that interact with bias (its Table 1): positive, low-negation (*Is it not raining?*), high-negation (*Isn't it raining?*), *really*-questions, focused (*IS it raining?*), declarative (*It is raining?*), incredulity questions, *or not* and antonym alternative questions, and polar questions with polarity items (*Do you have any potatoes?*).

| form | example | bias (summary) |
|---|---|---|
| positive polar | *Is it raining?* | neutral or mildly positive; impossible with negative evidence (Büring and Gunlogson 2000) |
| low negation | *Is it not raining?* | checks the negative proposition |
| high negation, outer reading | *Isn't Jane coming too?* | the speaker believes Jane is coming and wants confirmation (Ladd 1981) |
| high negation, inner reading | *Isn't Jane coming either?* | the speaker had believed she is coming; now evidence says no |
| *really* | *Is Jane really coming?* | doubt: the speaker leans to *no* |
| *or not* | *Will you marry me or not?* | cornering: demands an answer |
| tag | *It's raining, isn't it?* | toward the anchor (§7) |
| NPI *any / ever* | *Did anyone call?* | neutral; with minimizers (*lift a finger*) negative |

(The table follows Krifka and Trinh 2025, pp. ix–xi, who discuss Ladd 1981, Romero and Han 2004, Büring and Gunlogson 2000, Krifka 2015.) In UD high negation is an `advmod` *n't* on the inverted auxiliary's head, low negation an `advmod` *not* after the subject: the tree records the position, and the reading depends on it ([27-negation.md](27-negation.md)).

### 9.2 Rhetorical questions

A rhetorical question has question form but asserts: *Who cares?* ("nobody cares"), *Who for such dainties would not stoop?* ("everybody would"). Kiss, Lo and Leung (2025, in the same volume) distinguish rhetorical questions whose implied answer is the empty set (*Who cares?*) from those with a non-empty answer, and show that Cantonese marks the difference prosodically. Oraby et al. (2017, "Are you serious?: Rhetorical Questions and Sarcasm in Social Media Dialog", SIGDIAL, `papers/o/or/oraby-2017-are-you-serious-rhetorical-questions`) collect rhetorical questions in social media and show that they are frequent carriers of sarcasm ([34-irony-and-figurative-language.md](34-irony-and-figurative-language.md)).

Mova's parse database labels questions with a speech act and an indirectness flag (silver labels from the teacher loop, `$MOVA_DATA/parsedb/prag.tsv`, counted 2026-10-17). Most questions ask (`wh ask` 5 580, `q ask` 2 927), but a sizable minority are indirect acts:

| form, act, indirect | count | example (sent_id, `prag.tsv` label) |
|---|---:|---|
| `q request 1` | 401 | *"Can you not protect me, then?" inquired the sister.* (book:pg11027:1158) |
| `q assert 1` | 217 | *Can you doubt any longer?"* (book:pg128:2328) |
| `q evaluate 1` | 188 | |
| `wh evaluate 1` | 183 | *If you are my friend, why do you bite me?* (book:pg11339:1117) |
| `q offer 1` | 172 | *“Would you like to see a little of it?” said the Mock Turtle.* (book:pg11:870) |
| `q suggest 1` | 124 | *Will you, won’t you, will you, won’t you, won’t you join the dance?* (book:pg11:880) |
| `wh assert 1` | 76 | *Who for such dainties would not stoop?* (book:pg11:953); *How could a handful of men hope to stand against so great a host?* (book:pg18442:1405) |

Questions as requests (*Can you …?*), offers (*Would you like …?*), suggestions (*Shall we …?*) and assertions are the classic indirect speech acts; [31-speech-acts.md](31-speech-acts.md) treats them.

---

## 10. Exclamatives

Exclamatives begin with *what* (+ *a*) + noun or *how* + adjective/adverb, keep statement order, and express that a degree is remarkable. They share their first word with questions but are not questions: they cannot be answered and are presupposed true (*\*I wonder what a jerk he is*; *I know what a jerk he is*). UD gives *what* the relation `det:predet` before *a*, or `det`; *how* is `advmod` of the adjective (UD guidelines `_en/dep/acl-relcl.md`, "Exclamative clauses"):

(16) *What a hidden gem!* — root *gem*, `det:predet(gem, What)`, `det(gem, a)` (EWT 2.18 reviews-064146-0002).
(17) *I know what a jerk he is!* — `ccomp(know, jerk)`, `det:predet(jerk, what)`, `cop(jerk, is)`.
(18) *How big it is!* — root *big*, `advmod(big, How)`, `cop(big, is)` (Mova en 0.4, CPU, 2026-10-17; matches UD).

Jespersen notes that mass nouns take *what* and count nouns *what a* in exclamatory "quasi-questions" (*What nonsense!* / *What a lovely sight!*; MEG II §5.213, `books/j/je/jespersen-1914-modern-english-grammar-historical`, text-2).

Mova's form labeller tags a sentence `excl` when it ends with `!` and starts with *how*, or with *what a/an* ([`prag/src/feats.rs`](../../prag/src/feats.rs), the `excl` feature). *How much better does it get?!* (EWT 2.18) shows the limit: an exclamatory rhetorical question, inverted like a question, with both `?` and `!`.

---

## 11. Archaic and biblical questions

The JPS 1917 Torah keeps Early Modern patterns: main-verb inversion, *thou/ye*, and *art/hast/doth*. Mova's trees (`$MOVA_DATA/corpora/judaism/parsedb/current.conllu`) handle the copular ones and misread lexical inversion:

(19) *And the LORD God called unto the man, and said unto him: 'Where art thou?'* — `ccomp(said, Where)`, `cop(Where, art)`, `nsubj(Where, thou)`: the predicate *Where* heads the copular clause, as UD requires (jps:GEN:69).
(20) *When thy son asketh thee in time to come, saying: 'What mean the testimonies, and the statutes, and the ordinances, which the LORD our God hath commanded you?'* — Mova's tree has **`nsubj(mean, What)`** and **`obj(mean, testimonies)`**; the right analysis is `obj(mean, What)`, `nsubj(mean, testimonies)` (main-verb inversion: "what do the testimonies mean"). The same tree also leaves *asketh* unlemmatized and makes *saying* the root (jps:DEU:215).

The Torah English labels: 125 `q` and 79 `wh` sentences in `prag.tsv` of the Judaism corpus (2026-10-17). Archaic morphology (*-eth*, *-est*, *thou*) is described in the morphology chapters of this manual.

---

## 12. Mova's parser on questions

Fifteen test questions (Mova en 0.4, CPU, 2026-10-17):

| question | Mova en 0.4 | UD | verdict |
|---|---|---|---|
| *Who called you?* | `nsubj(called, Who)`, `obj(called, you)` | same | right |
| *Did you see the fox?* | `aux(see, Did)`, root *see* | same | right |
| *Are you going home?* | `aux(going, Are)` | same | right |
| *Where do you live?* | `advmod(live, Where)`, `aux(live, do)` | same | right |
| *I wonder where he lives.* | `ccomp(wonder, lives)`, `advmod(lives, where)` | same | right |
| *You saw what?* | `obj(saw, what)` | same | right |
| *Do you want tea or coffee?* | `conj(tea, coffee)` | same | right |
| *What a hidden gem!* | `det:predet(gem, What)` | same | right |
| *How big it is!* | `advmod(big, How)`, `cop(big, is)` | same | right |
| *Who cares?* | `nsubj(cares, Who)` | same | right |
| *What should you do if you get lost?* | `obj(do, What)`, `advcl(do, lost)`; **`aux:pass(lost, get)`** | `aux:pass` is right for the *get*-passive, but *get lost* is an idiom (intransitive) | right by UD |
| *What did the cook throw at the baby?* | **root *did* (VERB)**, **`nsubj(did, What)`**, **`ccomp(did, throw)`**; *throw* tagged **VBP** | root *throw*, `obj(throw, What)`, `aux(throw, did)`, *throw* VB | wrong |
| *Which book did she buy?* | **`nsubj(buy, book)`** and `nsubj(buy, she)` | `obj(buy, book)` | wrong: two subjects |
| *Why did the crow open her beak?* | root **did** (VERB), **`obj(did, crow)`**, *open* **ADJ** `xcomp` | root *open*, `aux(open, did)`, `nsubj(open, crow)` | wrong |
| *It is cold, isn't it?* | **`advcl(cold, it)`** with `cop(it, is)` | `parataxis(cold, is)` with `nsubj(is, it)` | wrong |

The failure is systematic for **non-subject wh-questions with do-support**: the parser, trained mostly on statements, reads *did* after a wh-word as a main verb and the real verb as a complement or as a non-verb. The misparse of the first example in full:

```
# text = What did the cook throw at the baby?          (Mova en 0.4, CPU, 2026-10-17)
ID FORM   LEMMA UPOS XPOS HEAD DEPREL
1  What   what  PRON WP   2    nsubj
2  did    do    VERB VBD  0    root
3  the    the   DET  DT   4    det
4  cook   cook  NOUN NN   5    nsubj
5  throw  throw VERB VBP  2    ccomp
6  at     at    ADP  IN   8    case
7  the    the   DET  DT   8    det
8  baby   baby  NOUN NN   5    obl
9  ?      ?     PUNCT .   2    punct
```

The seed rule `en.verbal.do-support-base` would flag the VBP after *do* if *did* were `aux`; here the parser made *did* a VERB, so no rule fires. Yes/no questions with do-support (*Did you see the fox?*) are parsed right: the auxiliary is sentence-initial, a pattern common enough in the training data. Questions in the EWT and school training data are few compared to statements (`wh` + `q` are about 3.7% of the labelled sentences of the parse database), which explains the gap.

---

## 13. Questions in Mova's pragmatics: form, act and the asked slot

### 13.1 The sentence form

Mova's pragmatics labels every sentence with a **form** from a closed set (`prag::schema::Form`: `decl`, `q`, `wh`, `imp`, `excl`, `frag`, `intj`, `other`; [`prag/src/schema.rs`](../../prag/src/schema.rs)) and a **speech act** (`assert, narrate, ask, request, offer, suggest, promise, thank, apologize, greet, agree, disagree, evaluate, express, warn, other`), plus an `indirect` flag (docs/pragmatics.md; [30-pragmatics-overview.md](30-pragmatics-overview.md)). The form classifier is a transparent model over tree and word features, among them the question mark of the utterance (not of the narrator's frame around a quote), the first auxiliary (`aux1`), inversion (`inv`), the first wh-word among the first three content words (`wh`), and the exclamative pattern ([`prag/src/feats.rs`](../../prag/src/feats.rs); the model in [`prag/src/model.rs`](../../prag/src/model.rs)). For quoted speech the features come from the quote: *"Can you hop on your hind legs?" asked the furry rabbit.* is `q` / `request` / indirect (book:pg11757:98).

| form | what it means | key cues |
|---|---|---|
| `q` | a polar, alternative, tag or declarative question | `?`, inverted auxiliary or none |
| `wh` | a wh-question | `?`, a wh-word with `PronType=Int` near the start |
| `excl` | an exclamative | `!`, *how* or *what a/an* first |
| `frag` | a verbless fragment (many short questions) | no finite verb |

### 13.2 The asked slot

When the reader is told the text is a question (`question = true`), the first wh-word gives an `Asked { frame, what }` (docs/pragmatics.md, "Questions"; [`prag/src/frames.rs`](../../prag/src/frames.rs), `enum Ask`):

| question word | `Ask` | example |
|---|---|---|
| *why* | `Cause` — follow the frame's `cause` link | *Why did the crow open her beak?* |
| *what happened next / after*, *what happens when / would happen if X* | `Next` on X | *What happened after the cheese fell?* |
| *who* | `Role(Agent)` (by its relation: subject → agent) | *Who called you?* |
| *what* (object) | `Role(Patient)` | *What did the cook throw?* |
| *what … do* | `Role(Verb)` | *What did Alice do?* |
| *where* | `Role(Place)` | *Where does the Mole live?* |
| *how* | `Role(Attr)`; *how many / much* → `Role(Patient)` | *How did she feel?* |
| *which N* | N's role | *Which book did she buy?* |
| *when*, yes/no, *put him right* | `Frame` — the frame itself | *Did the fox flatter the crow?* |

The pragmatic student ([`world/src/knowledge.rs`](../../world/src/knowledge.rs)) answers by matching the question frame's known roles against the frames of the lesson and following the asked slot (docs/pragmatics.md, "Clients").

### 13.3 Statement order before parsing

Because of the misparse of §12, the reader rewrites a do-support or modal wh-question into statement order **before parsing** (`prag::frames::statement_order`): *What did the cook throw at the baby?* → *the cook threw What at the baby?*; *What should you do if …* → *you should do What if …*; *Why did the crow open her beak?* → *Why the crow opened her beak?* The object wh-word goes after the verb (in situ, §8.2); *why / how / where / when* stay in front; the verb is re-inflected from the tense of *do* (`threw` from *did* + *throw*) with the compiled lexicon. The rewritten sentence is what the parser has seen most often, and it is parsed right. Frames for the four questions of the frame examples (Mova `world frames`, en 0.4, CPU, 2026-10-17):

```
== What did the cook throw at the baby?
the cook threw What at the baby?
1:the/DET/det>2 2:cook/NOUN/nsubj>3 3:threw/VERB/root>0 4:what/PRON/obj>3 5:at/ADP/case>7 6:the/DET/det>7 7:baby/NOUN/obl>3 8:?/PUNCT/punct>3
#0 Event cook throw place=baby
     roles by vn throw-17.1-1-1 «NP V NP PP.destinations»: agent=Agent what=Theme at baby=Destination
asked: «what» in #0: the patient of the frame

== Why did the crow open her beak?
Why the crow opened her beak?
#0 Event crow open patient=beak
asked: «why» in #0: why → the cause

== What happened after the cheese fell?
#0 Event _ happen
#1 Event cheese fall
asked: «what» in #1: what next → the next frame

== Did the fox flatter the crow?
1:did/AUX/aux>4 2:the/DET/det>3 3:fox/NOUN/nsubj>4 4:flatter/VERB/root>0 5:the/DET/det>6 6:crow/NOUN/obj>4
#0 Event fox flat patient=crow
asked: «do» in #0: the frame itself
```

The tree of the yes/no question is right, but the frame's verb is *flat*: the lemmatizer took *flatter* for the comparative of *flat* after *did*. This is the kind of error the frame bench and the question-answering students expose; it is not yet fixed (2026-10-17).

### 13.4 Questions in the absurdity and teacher loops

The teacher never answers the students' questions; it checks frames of sampled lines and logs its verdicts (`world knowledge teacher-check`, docs/pragmatics.md). The course students' tests are four-option questions read by the same reader, so every question form above is exercised daily by the course runs (docs/architecture.md, "The students").

### Not yet

- **Parser**: non-subject wh-questions with do-support, *which*-N object questions, tag questions; archaic main-verb inversion (§11–12). The statement-order rewrite covers do-support and modal wh-questions only in the reader, not in the parse database's own trees.
- **Embedded questions as asked slots**: *Tell me where he lives* is not read as a question.
- **Bias and rhetorical readings**: the pragmatics labels indirect acts (§9.2), but the frames reader does not turn a rhetorical question into its implied assertion (*Who cares?* → nobody cares), and high versus low negation is not distinguished.
- **Alternative questions**: no `Ask` kind for a choice among the coordinated alternatives.
- **Tag questions**: no subtype; the anchor's bias is not recorded.
- **Echo and declarative questions**: labelled only by `?`; no link to the utterance echoed.
- **Lemmas after *do***: *flatter* → *flat* (§13.3).

---

## In Mova

- Conventions and checks: [`en/seeds/verbal/questions-inversion.md`](../../en/seeds/verbal/questions-inversion.md), [`do-support.md`](../../en/seeds/verbal/do-support.md), [`aux-no-object.md`](../../en/seeds/verbal/aux-no-object.md), [`vp-ellipsis.md`](../../en/seeds/verbal/vp-ellipsis.md), [`wh-adverb-clause.md`](../../en/seeds/verbal/wh-adverb-clause.md); [`en/seeds/morph/pron-wh.md`](../../en/seeds/morph/pron-wh.md); [`en/seeds/nominal/who-whom-whose.md`](../../en/seeds/nominal/who-whom-whose.md).
- Dialect: [`dialects/mova/seeds.md#wh-case`](../../dialects/mova/seeds.md#wh-case) (case of *who*), [`spoken-subtypes.md`](../../dialects/mova/seeds.md#spoken-subtypes) (`discourse:tag` — wait), [`discourse-adverbs.md`](../../dialects/mova/seeds.md#discourse-adverbs) (*right* as INTJ `discourse`).
- Pragmatics: the form and act schema [`prag/src/schema.rs`](../../prag/src/schema.rs), the features [`prag/src/feats.rs`](../../prag/src/feats.rs), the classifiers [`prag/src/model.rs`](../../prag/src/model.rs); question reading and `statement_order` in [`prag/src/frames.rs`](../../prag/src/frames.rs); implicatures of questions (the speaker does not know the questioned part) in [`prag/src/infer.rs`](../../prag/src/infer.rs).
- Students: [`world/src/knowledge.rs`](../../world/src/knowledge.rs) (the pragmatic student), [`world/src/qframe.rs`](../../world/src/qframe.rs) and [`world/src/qa.rs`](../../world/src/qa.rs) (question readers of the tale benchmarks).
- Not yet: see §13.

## Sources

- Bill, Cory, and Todor Koev. 2025. "Bias in tag questions." In Trinh et al. (eds.), *Biased Questions*, 141–174. Language Science Press. `books/k/kr/krifka-2025-biased-questions-experimental-results`.
- Brown, Goold. 1851. *The Grammar of English Grammars*, Part II, ch. VI. `books/b/br/brown-1851-grammar-english-grammars`.
- Cordes, Moritz (ed.). 2022. *Asking and Answering*. Narr Francke Attempto, CC BY-SA 4.0. `books/c/co/cordes-2022-asking-answering`; https://library.oapen.org/handle/20.500.12657/57955. Chapters used: Ciardelli, "Why We Need a Question Semantics"; Krifka, "Modelling Questions in Commitment Spaces"; Wiśniewski, "An Essay on Inferential Erotetic Logic".
- Jespersen, Otto. 1924. *The Philosophy of Grammar*. `books/j/je/jespersen-1924-philosophy-grammar`.
- Jespersen, Otto. *A Modern English Grammar on Historical Principles*, Part II (§5.213) and Part V (ch. XXV "Questions"). `books/j/je/jespersen-1914-modern-english-grammar-historical`.
- Kiss, Angelika, Roger Yu-Hsiang Lo, and Justin R. Leung. 2025. "What can Cantonese sentence-final particles tell us about rhetorical questions?" In Trinh et al. (eds.), *Biased Questions*.
- Krifka, Manfred, and Tue Trinh. 2025. "Introduction." In Trinh et al. (eds.), *Biased Questions*, v–xxiv. DOI 10.5281/zenodo.17158174.
- Oraby, Shereen, Vrindavan Harrison, Amita Misra, Ellen Riloff, Marilyn Walker. 2017. "Are you serious?: Rhetorical Questions and Sarcasm in Social Media Dialog." SIGDIAL 2017. `papers/o/or/oraby-2017-are-you-serious-rhetorical-questions`.
- Reed, Alonzo, and Brainerd Kellogg. 1877. *Higher Lessons in English*, Lesson 55. `books/r/re/reed-1877-higher-lessons-english`.
- Trinh, Tue, Anton Benz, Daniel Goodhue, Kazuko Yatsushiro, Manfred Krifka (eds.). 2025. *Biased Questions: Experimental Results and Theoretical Modelling*. Language Science Press, CC BY 4.0. https://zenodo.org/records/17084327; `books/k/kr/krifka-2025-biased-questions-experimental-results`.
- Universal Dependencies guidelines, snapshot 2026-09-24: `_en/dep/cop.md`, `_en/dep/acl-relcl.md` (interrogative clauses, exclamatives), `_en/dep/parataxis.md`, `_en/dep/discourse.md`, `_en/feat/PronType.md`; UD issues #1273, #1280, #1289, #1290 (spoken-language subtypes). https://universaldependencies.org/en/.
- UD English EWT 2.18 (Silveira et al. 2014, https://aclanthology.org/L14-1067; CC BY-SA 4.0), examples by `sent_id`.
- Works cited through Krifka and Trinh 2025 (not in the library): Bolinger 1978; Büring and Gunlogson 2000; Ladd 1981; Romero and Han 2004; Biezma 2009; Hamblin 1973; Groenendijk and Stokhof 1984.
- Mova: `docs/pragmatics.md`, `docs/architecture.md`; `$MOVA_DATA/parsedb/current.conllu` and `prag.tsv`; `$MOVA_DATA/corpora/judaism/parsedb/current.conllu` and `prag.tsv`.
