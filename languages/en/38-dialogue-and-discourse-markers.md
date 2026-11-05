# 38. Dialogue, discourse markers and reported speech

Conversation has its own order: turns, pairs of actions that call for each other, preferred and dispreferred responses, repair, and the steady work of grounding what has been understood. English marks this work with small words — *well, oh, so, you know, I mean, anyway* — with vocatives and with the frames of reported speech that tales are full of. This chapter describes these devices, their UD analysis, how Mova labels and reads dialogue (Say frames, the `voice` field, the family's dialogue layer), and where it still fails. Speech acts and dialogue-act schemes themselves are in [31-speech-acts.md](31-speech-acts.md); politeness in [32-politeness-and-hedging.md](32-politeness-and-hedging.md).

Contents

1. [The organisation of conversation](#1-the-organisation-of-conversation)
2. [Recruitments: getting others to do things](#2-recruitments-getting-others-to-do-things)
3. [Grounding and common ground](#3-grounding-and-common-ground)
4. [Discourse markers](#4-discourse-markers)
5. [Interjections, backchannels and fillers](#5-interjections-backchannels-and-fillers)
6. [Vocatives and address](#6-vocatives-and-address)
7. [Reported speech](#7-reported-speech)
8. [Dialogue in tales](#8-dialogue-in-tales)
9. [Questions and answers in dialogue](#9-questions-and-answers-in-dialogue)
10. [Dialogue acts, briefly](#10-dialogue-acts-briefly)
11. [How Mova reads dialogue](#11-how-mova-reads-dialogue)

---

## 1. The organisation of conversation

Conversation analysis describes talk-in-interaction as an orderly system (Sacks, Schegloff & Jefferson 1974, "A simplest systematics for the organization of turn-taking for conversation", *Language* 50(4); Schegloff 2007, *Sequence Organization in Interaction*; neither is open — Wanted):

| unit | what it is | English example |
|---|---|---|
| turn-constructional unit | a sentence, clause, phrase or word that can complete a turn | *Coffee?* |
| transition-relevance place | the point where another speaker may take over | the end of a TCU |
| adjacency pair | two turns by different speakers, the first making the second relevant | question–answer, greeting–greeting, offer–acceptance/refusal, request–grant/refusal |
| preference | of the possible second parts, one is preferred (given at once, plainly) and one dispreferred (delayed, prefaced, accounted for) | *Sure!* vs *Well, I'd love to, but…* |
| insertion sequence | a pair inside a pair | *Want some cake? — Is it chocolate? — Yes. — Then yes.* |
| repair | fixing trouble in speaking, hearing or understanding; self- or other-initiated | *the red- the blue one*; *Sorry?* |
| pre-sequence | a turn preparing another | *Are you busy tonight?* (pre-invitation) |

Dispreferred responses have a recognisable English shape: a delay, a preface (*well*, *uh*), a hedge or appreciation, an account (*I'd love to, but I have to work*). Levinson (2024, *The Dark Matter of Pragmatics*, `books/l/le/levinson-2024-dark-matter-pragmatics-known`, card only) argues that this interactional machinery — turn-taking timed to fractions of a second, sequence organisation, action recognition — is the largest uncharted territory of pragmatics (30 §2). Laury, Etelämäki & Couper-Kuhlen (eds.) (2017, *Linking Clauses and Actions in Social Interaction*, `books/k/ku/kuhlen-2017-linking-clauses-actions-social`, card only) show how clause combining — the subordination and coordination of chapter 24 — is shaped by the actions turns perform.

## 2. Recruitments: getting others to do things

A **recruitment** is a sequence in which one person gets another to do something practical — pass an object, do a task, change what they are doing — whether by asking, by mentioning a problem, or by a gesture (Floyd, Rossi & Enfield (eds.) 2020, *Getting others to do things: A pragmatic typology of recruitments*, Language Science Press, `books/e/en/enfield-2020-getting-others-things-pragmatic`). The volume compares video-recorded everyday interaction in eight languages, English among them. For English (Kendrick, ch. 4):

| finding | detail |
|---|---|
| bodies matter | about half of the recruiting moves included a relevant visible action (pointing, reaching, holding out an object); recruitment by bodily action alone was rare |
| interrogatives lead | interrogatives (*Can you…?*, *Could you…?*, *Do you have…?*) were the most frequent format and the most varied; imperatives were second and varied little |
| cooperation | recruiting moves overwhelmingly got cooperative responses — the preference for agreement |
| offers and requests together | offering and requesting are two sides of one organisation: both respond to a trouble in someone's course of action |

Italian speakers, by contrast, used imperatives most (the same volume, the Italian chapter). Tales differ from conversation in the other direction: in Mova's parse database, of the 13 545 sentences labelled `request`, 10 833 are imperatives and only 497 yes/no questions (2026-10-17; 30 §7.3) — narrative dialogue favours the direct form that everyday English softens into a question (31, 32).

## 3. Grounding and common ground

Participants do not just send messages; they work to establish that what was said has been understood well enough for current purposes. Clark & Schaefer (1989, "Contributing to Discourse", *Cognitive Science* 13(2), `papers/c/cl/clark-1989-contributing-discourse`) model a contribution as a presentation phase and an acceptance phase; the evidence of acceptance ranges from continued attention, through relevant next turns and acknowledgements (*uh huh, yeah*), to demonstration and verbatim display. Clark & Brennan (1991, "Grounding in Communication", `papers/c/cl/clark-1991-grounding-communication`) add that the medium changes the cost of grounding (face to face, telephone, text), and that participants seek *least collaborative effort*. Clark & Wilkes-Gibbs (1986, "Referring as a collaborative process", *Cognition* 22(1), `papers/c/cl/clark-1986-referring-collaborative-process`) show referring expressions shrinking over repeated mentions as partners agree on them (*the one that looks like a skater* → *the skater*) — the conversational side of 35. Traum (1994, *A Computational Theory of Grounding in Natural Language Conversation*, `papers/t/tr/tra/traum-1994-computational-theory-grounding-natural-language`) turned grounding into acts (initiate, continue, acknowledge, repair, request repair, cancel) for dialogue systems.

| grounding signal | English forms | UD |
|---|---|---|
| continuer / backchannel | *mhm, uh huh, yeah, right* | INTJ `discourse` |
| acknowledgement | *OK, got it, I see* | `discourse` / a clause |
| repetition / display | *Tuesday. — Tuesday, right.* | fragment |
| request for repair | *Sorry? What? Pardon? You mean the red one?* | INTJ / fragment / question |
| self-repair | *the red- the blue one* | `reparandum` |

Disfluencies themselves can be analysed as dialogue moves inside an utterance — a speaker's self-addressed clarification request (Ginzburg, Fernández & Schlangen 2014, `papers/g/gi/gin/ginzburg-2014-disfluencies-as-intra-utterance-dialogue`).

## 4. Discourse markers

### 4.1 What they do

Discourse markers are sequentially dependent words and phrases that bracket units of talk and show how the coming unit relates to what came before or to the speaker's stance (Schiffrin 1987, *Discourse Markers*; Wanted). The main English ones:

| marker | typical functions | example |
|---|---|---|
| *well* | marks a response that is not the simple expected one: dispreferred answers, reorientation, hesitation | *Well, perhaps not* (`book:pg11:129`) |
| *oh* | change of state of knowledge: news receipt, recognition, surprise | *Oh, I see.* |
| *so* | upshot, result, return to the main line, opening a topic | *So, what happened next?* |
| *then* | inference from the prior turn | *Then it ought to be Number One* (`book:pg11:1075`) |
| *now* | topic shift, transition to the next point | *Now, about the tarts…* |
| *anyway* | return after a digression; closing | *Anyway, I got you message* |
| *you know* | appeal to shared knowledge; floor holding | *it was, you know, the usual* |
| *I mean* | self-repair, elaboration, clarification | *I mean, it wasn't bad* |
| *like* | approximation, focus, quotation (*she was like "no way"*) | — |
| *right* / *OK* | acknowledgement, transition, checking | *Right, let's start.* |
| *actually* | contrast with expectation, correction | *Actually, it's Tuesday.* |
| *by the way* | marks a topic shift as incidental | — |

Mauranen (2023, *Reflexively Speaking: Metadiscourse in English as a Lingua Franca*, De Gruyter, `books/m/ma/mauranen-2023-reflexively-speaking-metadiscourse-english`, card only) studies the related category of metadiscourse — talk about the talk (*as I said*, *let me put it this way*) — in English used between non-native speakers. An older guide to the correct use of connectives is Fernald (1904, *Connectives of English Speech*, `books/f/fe/fernald-1904-connectives-english-speech-correct`).

### 4.2 UD analysis

UD analyses discourse markers by their word class, not by their function. In the Mova dialect (after the UD 2.19 change), `discourse` is only for interjections (INTJ), numbering and emoticons; adverbs are `advmod` (*though, so, also, actually, anyway*), PPs are `obl` (*in other words, by the way*) ([../../dialects/mova/seeds.md#discourse-adverbs](../../dialects/mova/seeds.md#discourse-adverbs)). Clausal markers like *you know*, *I mean*, *I think* are clauses with their own predicate; as parentheticals they are `parataxis` in EWT.

| marker | EWT 2.18 analysis | Mova dialect |
|---|---|---|
| *well* (INTJ) | `discourse` | `discourse` |
| *oh, yes, no, ok, wow* | `discourse` | `discourse` |
| *please* | `discourse` (334 tokens, the most frequent `discourse` word) | `discourse` |
| *anyway* | INTJ `discourse` in `email-enronsent15_01-0030` | ADV `advmod` (rule `mova.ud-next.discourse-adv`) |
| *so, though, actually* | mostly `advmod`, a few `discourse` | `advmod` |
| *in other words* | `obl` | `obl` |
| *you know* (parenthetical) | `parataxis` of the clause head (e.g. `email-enronsent35_01-0117`) | `parataxis` |
| *I mean* | `parataxis` 5, `root` 3, `discourse` 3, `conj` 1, `ccomp` 1 (all EWT occurrences, 2026-10-17) | `parataxis` when parenthetical |

The most frequent `discourse` words in EWT 2.18 (2026-10-17): *please* 334, *yes* 68, *well* 59, *:)* 59, *no* 41, *like* 27, *lol* 21, *ok* 17, *hi* 16, *anyway* 16, *thanks* 15, *oh* 15. In Mova's parse database, after the numbering of dictionary senses (*1, 2, 3…*), they are *oh* 1 962, *well* 1 433, *yes* 1 157, *no* 1 102, *please* 930, *ah* 687, *o* 556, *alas* 278 — the vocabulary of tales.

(1) *Well, I mean, you know, it was okay.* «Mova en 0.4, CPU, 2026-10-17»: `discourse(mean, Well)` right; but *mean* becomes the root, *know* its `ccomp` and *okay* the `ccomp` of *know* — the parentheticals are read as the main clause and its complements. In EWT style, *okay* is the root and *mean*, *know* are `parataxis`.

## 5. Interjections, backchannels and fillers

Interjections are INTJ with `discourse`; they take no dependents in the usual case and attach to the head of the clause they belong to. In tales, emotional interjections (*oh, ah, alas, O*) dominate; in conversation, response tokens and fillers (*yeah, mhm, um, uh*).

(2) *Oh, thank you, my dear!* «Mova en 0.4, CPU, 2026-10-17»: `discourse(thank, Oh)`, `obj(thank, you)`; *my dear* is a vocative (UD: `vocative(thank, dear)`), but the parser makes it a `conj` of *thank*.

Fillers (*um, uh*) and tag questions (*isn't it?*) would get their own subtypes (`discourse:filler`, `discourse:tag`) in the spoken-language proposals now discussed in UD; English treebanks do not have them yet, and the Mova dialect waits ([../../dialects/mova/seeds.md#spoken-subtypes](../../dialects/mova/seeds.md#spoken-subtypes)). Laughter is a dialogue move too, with its own semantics and placement in turns (Ginzburg et al. 2020, "Laughter as language", `papers/g/gi/gin/ginzburg-2020-laughter-as-language`).

## 6. Vocatives and address

A vocative names the addressee; UD gives it the relation `vocative` from the head of the clause (UD guidelines `_en/dep/vocative.md`, https://universaldependencies.org/en/dep/vocative.html). Vocatives open and close turns, select the next speaker, and carry stance (endearment, deference, rebuke) — the politeness side is in 32.

| position | function | example |
|---|---|---|
| initial | summons, selecting the addressee | *Alice, come here.* |
| medial | emphasis, attention | *I told you, Alice, not to do that.* |
| final | softening, rapport | *Take care, my friend, Linda* (EWT 2.18 `weblog-typepad.com_ripples_20040407125600_ENG_20040407_125600-0062`) |
| alone | a call | *Alice!* |

In EWT 2.18, vocatives are mostly PROPN (134 of 183) and NOUN (42); emails have the most (89 of 183) — salutations and sign-offs (37 §4). The parse database has 7 899 `vocative` (2026-10-17).

(3) *"Come here, Alice," cried the Badger.* «Mova en 0.4, CPU, 2026-10-17»: `vocative(Come, Alice)` — right inside the quote. But a vocative at the end of an imperative outside quotes is taken as `obl` (*Sit down and listen to me, Alice!*, 21 §8.1), and *my dear* in (2) as a conjunct.

## 7. Reported speech

### 7.1 Forms

| form | example | deixis | UD |
|---|---|---|---|
| direct speech | *"I am late," said the Rabbit.* | the quoted speaker's *I, here, now* | the quote's head is `ccomp` of the speech verb (EWT), or `parataxis` |
| indirect speech | *She said that she was tired.* | the reporter's; tense backshift | `ccomp` with `mark` *that* (23) |
| free indirect speech | *She was tired. Why should she go on?* | the character's thoughts in the narrator's tense and person | no marking: separate sentences |
| quotative *be like*, *go* | *She was like, "no way."* | the quoted speaker's | *be like* + `ccomp` (one of the 7 EWT cases of *be* with `ccomp`) |
| reporting with inversion | *"Shan't," said the cook.*; *"…" cries the Badger* | — | `nsubj` after the verb (29) |
| scare quotes, mention | *the so-called "experts"* | — | no speech frame |

In EWT, when the quote comes first and *said X* follows, the reporting verb is the root and the quote's head is its `ccomp`:

(4) EWT 2.18 `weblog-blogspot.com_aggressivevoicedaily_20060811122000_ENG_20060811_122000-0005` *"We believe this is an ill-advised term …," said Nihad Awad, …* — `ccomp(said, believe)`, `nsubj(said, Awad)`.

### 7.2 Where the parser fails

(5) *"Shan't," said the cook.* «Mova en 0.4, CPU, 2026-10-17»: *Shan't* is split into *Sha* (PROPN) + *n't*, and *Sha* becomes a second `nsubj` of *said* beside *cook*. The tale's original is `book:pg11:1039`; its prag label is `frag disagree`. The frame reader consequently reads the speaker as *sha* (§11.2).

(6) *"Come here, Alice," cried the Badger.* — *cried* becomes a `conj` of *Come* instead of the root with *Come* as its `ccomp`; the speaker *Badger* is found as `nsubj(cried, Badger)`.

The contraction *shan't* is a tokenisation matter (morphology chapters), the attachment of the reporting clause a syntax one (29).

### 7.3 Deixis in quotes

Inside a quote, *I* is the quoted speaker and *you* the addressee; tense and *here/now* are the quoted speaker's. Mova resolves *I* and *you* inside quotes to the speaker and addressee of the Say frame that carries the quote, or the last speaker when no verb of saying does (35 §6).

## 8. Dialogue in tales

The `voice` field of Mova's pragmatics labels says whether a sentence is narration (`narrator`), a character's direct speech (`character`) or neither (`none`) (30 §6). Over all labelled sentences (`$MOVA_DATA/parsedb/prag.tsv`, 2026-10-17): narrator 268 513, character 38 406, none 4 182 of 311 101. By source:

| source | narrator | character | none | character share |
|---|---:|---:|---:|---:|
| tales (`book:`) | 82 549 | 35 688 | 84 | 30.2% |
| dictionary (`dict:`) | 158 156 | 1 868 | 3 677 | 1.1% |
| school lessons (`school:`) | 18 024 | 609 | 22 | 3.3% |
| word problems (`math:`) | 8 235 | 224 | 398 | 2.5% |

Almost a third of the tale sentences are speech. The acts of character speech differ from narration: assert 11 948, request 6 440, ask 5 456, evaluate 3 532, express 2 460, disagree 1 402, suggest 1 342, promise 1 227, warn 1 078, narrate 879, offer 869, agree 817 — requests, questions and disagreements are concentrated in dialogue.

(7) Short exchanges from *Alice's Adventures in Wonderland* (`book:pg11:*`) with their labels (form, act; 2026-10-17):

| id | text | form | act |
|---|---|---|---|
| `book:pg11:1013` | *"I didn't!" the March Hare interrupted in a great hurry.* | decl | disagree |
| `book:pg11:1015` | *"I deny it!" said the March Hare.* | decl | disagree |
| `book:pg11:1039` | *"Shan't," said the cook.* | frag | disagree |
| `book:pg11:1075` | *"Then it ought to be Number One," said Alice.* | decl | agree |
| `book:pg11:1117` | *"No, no!" said the Queen. "Sentence first—verdict afterwards."* | frag | disagree |
| `book:pg11:129` | *"Well, perhaps not," said Alice in a soothing tone: "don't be angry about it.* | decl | disagree (hedge) |

The disagreements show the dispreferred shape of §1: *Well, perhaps not* is hedged and followed by a soothing request; *Shan't* and *I deny it!* are bare refusals, which is part of their comic rudeness.

## 9. Questions and answers in dialogue

A question makes an answer relevant; answers can be direct, partial, indirect (*Is it far? — It's a long walk.*), or refuse the question's presupposition. Indirect answers to yes/no questions are their own research topic (the Circa dataset, cited in [../../docs/context.md](../../docs/context.md) as a test resource). Cordes (ed.) (2022, *Asking and Answering*, Narr Francke Attempto, `books/c/co/cordes-2022-asking-answering`) collects work on the semantics and pragmatics of questions and answers; Trinh, Benz, Goodhue, Yatsushiro & Krifka (eds.) (2025, *Biased questions: Experimental results and theoretical modelling*, Language Science Press, `books/k/kr/krifka-2025-biased-questions-experimental-results`) treats questions that lean toward one answer — negative polar questions (*Isn't it raining?*), rising declaratives (*It's raining?*), tag questions — which in dialogue are often requests for confirmation rather than for information (26).

The response space of a question is larger than its answers: Ginzburg et al. (2022, "Characterizing the response space of questions: data and theory", `papers/g/gi/gin/ginzburg-2022-characterizing-response-space-questions-data`) classify responses into answers, clarification requests, evasions, counter-questions and others.

## 10. Dialogue acts, briefly

Dialogue-act schemes annotate each utterance with its communicative function: DAMSL (Core & Allen 1997, `papers/c/co/cor/core-1997-coding-dialogs-damsl-annotation-scheme`), SWBD-DAMSL on Switchboard (Stolcke et al. 2000, `papers/s/st/sto/stolcke-2000-dialogue-act-modeling-automatic-tagging`), and the ISO 24617-2 standard with its dimensions — Task, Auto- and Allo-Feedback, Turn and Time Management, Discourse Structuring, Own and Partner Communication Management, Social Obligations Management, Contact Management (Bunt et al. 2020, `papers/b/bu/bun/bunt-2020-iso-standard-dialogue-act-annotation`; summary in [../../docs/pragmatics.md](../../docs/pragmatics.md)). The schemes and their relation to Mova's `Act` labels are described in [31-speech-acts.md](31-speech-acts.md); here it is enough that the discourse markers of §4 and the grounding signals of §3 are exactly what fills the feedback, turn-management and discourse-structuring dimensions.

## 11. How Mova reads dialogue

### 11.1 Say frames

The frame reader turns speech and thought into `Say(Act)` frames whose patient is what was said (`quote`), with these rules ([../../docs/pragmatics.md](../../docs/pragmatics.md), *Links* and *Roles, more*; [../../prag/src/frames.rs](../../prag/src/frames.rs)):

| rule | example |
|---|---|
| a `ccomp` of *say/think/know/fear…* and quoted speech → `quote` of the Say frame | *"I am late," said the Rabbit* → `Say rabbit say | says→#0` |
| only a verb of saying or knowing carries a quote; the book's own quoted words in narration are not speech | — |
| verbs all of whose VerbNet classes are communication (37.x: *whisper, mumble, inquire, complain, gossip*… 152 verbs) are speech verbs | — |
| the speaker after a quote | *"…" cries the Badger* |
| the person told or asked is the recipient | *said the Rabbit **to Alice*** → `to=alice` |
| the act of the quote by its *?* / *!* | `Say(ask)`, `Say(express)` |
| *I* / *you* in the quote → the speaker / addressee | 35 §6 |

(8) `world frames "“I can’t help it,” said Alice very meekly. “Shan’t,” said the cook. The Queen asked Alice whether she could play croquet."` «Mova 0.4, CPU, 2026-10-17»:

```text
#0 Fact(can) alice(«i») NOT help patient=it | quoted
#1 Say(assert) alice say attrs=meekly | says→#0 next→#2
#2 Say(assert) sha say | next→#3
#3 Say(ask) queen ask to=alice
#4 Fact(can) queen(«she») play patient=croquet
```

The first quote is right: *I* = Alice, the quote is negated and attached to her Say frame, *meekly* is kept. The second loses the cook to the tokenisation error of (5); and the content of *asked … whether* is not attached to the Say frame as its quote, while *she* is resolved to the Queen (the asker) rather than Alice (the one asked; the sentence is ambiguous, but the expected reading is Alice).

### 11.2 Measurements

Dialogue is the weakest area of the frame reader on the frame bench: speaker F1 .32 and says F1 .38 on dev (2026-10-10), because dialogue often has no verb of saying and the parser splits quotes ([../../docs/pragmatics.md](../../docs/pragmatics.md), *Not yet*).

### 11.3 The family's dialogue layer

`world/src/talk.rs` holds the pragmatics of dialogue between users and Mova's "family" of agents ([../../world/src/talk.rs](../../world/src/talk.rs); [../../docs/pragmatics.md](../../docs/pragmatics.md), *Done*):

- **acts by cues and form** in the dimensions of ISO 24617-2: social obligations (greet, thank, apologize; affection as `express`), feedback on what a member did (`evaluate`, positive or negative), and the task (`ask`, `request`, `assert`); the addressee is the member named at the start;
- **social obligations management**: greeting and goodbye, a politeness question (*How are you?*) answered from the member's recent memories, opening and closing politeness;
- **politeness**: the 20 strategies of Danescu-Niculescu-Mizil et al. (2013) as integer-weighted detectors — a blunt request is still done, but noticed, with the strategies that fired (32);
- **indirect requests**: *Can/Could/Would you (please) X?* is read as the request X only when the speaker wants it, the family is capable, willing now and obligated (after Sarathy et al. 2020, `papers/s/sa/sar/sarathy-2020-reasoning-requirements-indirect-speech-act`, and Brown 1980); otherwise it is answered as a question about ability (31);
- **memory stream**: observations with importance, recalled by importance, recency and word overlap (after Park et al. 2023).

`world/src/mind.rs` lets one talk to the small model's "brain" in English: a told line is read into the story reader's world, a line with *?* is answered from that world with an explanation, or with "I do not know: …" for a gap ([../../world/src/mind.rs](../../world/src/mind.rs)).

### 11.4 Not yet

- No turn structure for tales: adjacency pairs and the turn-taking addressee (*she answered* after the Rabbit spoke to Alice → Alice) are not modelled in the frame reader (35 §6).
- Reporting clauses after quotes and contracted refusals are misparsed (§7.2); speaker attribution without a verb of saying is weak.
- Discourse markers have no function labels (*well* as dispreferred preface, *oh* as change of state); parenthetical *you know*, *I mean* are misread as main clauses.
- Vocatives after imperatives and endearments (*my dear*) are often not `vocative`.
- Free indirect speech is not detected; indirect speech (*asked whether…*) is not always linked as a quote.
- Grounding and repair are not represented; `reparandum` is almost never predicted (12 tokens in the parse database against 49 in EWT, 20 §5.2).

## In Mova

- Say frames, quotes, speakers: [../../prag/src/frames.rs](../../prag/src/frames.rs); labels with `voice`: [../../prag/src/schema.rs](../../prag/src/schema.rs), [../../prag/src/feats.rs](../../prag/src/feats.rs) (quotes tracked across a paragraph).
- Dialogue layer: [../../world/src/talk.rs](../../world/src/talk.rs); the brain dialogue: [../../world/src/mind.rs](../../world/src/mind.rs); QUD in dialogue: [../../world/src/qud.rs](../../world/src/qud.rs).
- Coreference by voice: the `speaker` sieve in [../../coref/src/sieve.rs](../../coref/src/sieve.rs).
- Dialect decisions: [../../dialects/mova/seeds.md#discourse-adverbs](../../dialects/mova/seeds.md#discourse-adverbs), [../../dialects/mova/seeds.md#spoken-subtypes](../../dialects/mova/seeds.md#spoken-subtypes).
- Docs: [../../docs/pragmatics.md](../../docs/pragmatics.md), [../../docs/context.md](../../docs/context.md).

## Sources

- Sacks, H., Schegloff, E. A., Jefferson, G. 1974. A simplest systematics for the organization of turn-taking for conversation. *Language* 50(4). Wanted.
- Schegloff, E. A. 2007. *Sequence Organization in Interaction*. Cambridge University Press. Wanted.
- Schiffrin, D. 1987. *Discourse Markers*. Cambridge University Press. Wanted.
- Levinson, S. C. 2024. *The Dark Matter of Pragmatics: Known Unknowns*. Cambridge University Press. `books/l/le/levinson-2024-dark-matter-pragmatics-known` (card)
- Laury, R., Etelämäki, M., Couper-Kuhlen, E. (eds.) 2017. *Linking Clauses and Actions in Social Interaction*. Finnish Literature Society. `books/k/ku/kuhlen-2017-linking-clauses-actions-social` (card)
- Floyd, S., Rossi, G., Enfield, N. J. (eds.) 2020. *Getting others to do things: A pragmatic typology of recruitments*. Language Science Press. Ch. 4: Kendrick, K. H., Recruitment in English: A quantitative study. `books/e/en/enfield-2020-getting-others-things-pragmatic`
- Clark, H. H., Schaefer, E. F. 1989. Contributing to Discourse. *Cognitive Science* 13(2). `papers/c/cl/clark-1989-contributing-discourse`
- Clark, H. H., Brennan, S. E. 1991. Grounding in Communication. `papers/c/cl/clark-1991-grounding-communication`
- Clark, H. H., Wilkes-Gibbs, D. 1986. Referring as a collaborative process. *Cognition* 22(1). `papers/c/cl/clark-1986-referring-collaborative-process`
- Traum, D. R. 1994. *A Computational Theory of Grounding in Natural Language Conversation*. `papers/t/tr/tra/traum-1994-computational-theory-grounding-natural-language`
- Ginzburg, J., Fernández, R., Schlangen, D. 2014. Disfluencies as intra-utterance dialogue moves. *Semantics and Pragmatics* 7. `papers/g/gi/gin/ginzburg-2014-disfluencies-as-intra-utterance-dialogue`
- Ginzburg, J., Mazzocconi, C., Tian, Y. 2020. Laughter as language. *Glossa*. `papers/g/gi/gin/ginzburg-2020-laughter-as-language`
- Ginzburg, J. et al. 2022. Characterizing the response space of questions: data and theory. `papers/g/gi/gin/ginzburg-2022-characterizing-response-space-questions-data`
- Mauranen, A. 2023. *Reflexively Speaking: Metadiscourse in English as a Lingua Franca*. De Gruyter. `books/m/ma/mauranen-2023-reflexively-speaking-metadiscourse-english` (card)
- Fernald, J. C. 1904. *Connectives of English Speech*. `books/f/fe/fernald-1904-connectives-english-speech-correct`
- Cordes, M. (ed.) 2022. *Asking and Answering*. Narr Francke Attempto. `books/c/co/cordes-2022-asking-answering`
- Trinh, T., Benz, A., Goodhue, D., Yatsushiro, K., Krifka, M. (eds.) 2025. *Biased questions: Experimental results and theoretical modelling*. Language Science Press. `books/k/kr/krifka-2025-biased-questions-experimental-results`
- Core, M., Allen, J. 1997. Coding Dialogs with the DAMSL Annotation Scheme. `papers/c/co/cor/core-1997-coding-dialogs-damsl-annotation-scheme`
- Stolcke, A. et al. 2000. Dialogue Act Modeling for Automatic Tagging and Recognition of Conversational Speech. `papers/s/st/sto/stolcke-2000-dialogue-act-modeling-automatic-tagging`
- Bunt, H. et al. 2020. The ISO Standard for Dialogue Act Annotation, Second Edition. `papers/b/bu/bun/bunt-2020-iso-standard-dialogue-act-annotation`
- Sarathy, V. et al. 2020. Reasoning Requirements for Indirect Speech Act Interpretation. `papers/s/sa/sar/sarathy-2020-reasoning-requirements-indirect-speech-act`
- Danescu-Niculescu-Mizil, C. et al. 2013. A computational approach to politeness with application to social factors. ACL. https://aclanthology.org/P13-1025 (as summarised in [../../docs/pragmatics.md](../../docs/pragmatics.md))
- Universal Dependencies guidelines: `_en/dep/vocative.md`, `_en/dep/discourse.md`, `_en/dep/parataxis.md`. https://universaldependencies.org/en/
- UD English EWT 2.18 (CC BY-SA 4.0), sentences by `sent_id`; counts of 2026-10-17. Tales in the parse database (*Alice's Adventures in Wonderland*, Project Gutenberg #11, public domain) and `$MOVA_DATA/parsedb/prag.tsv`, counted 2026-10-17.
