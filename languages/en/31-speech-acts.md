# 31. Speech acts

To say something is to do something: to assert, ask, request, promise, thank, warn. This chapter describes how English marks what a speaker is doing — explicit performative verbs, sentence types, modals, *please*, tags and punctuation — how often form and act part company in Mova's data, how indirect speech acts such as *Can you open the window?* are recognised, and how dialogue-act schemes (DAMSL, SWBD-DAMSL, ISO 24617-2) classify acts in conversation. It ends with what Mova's labeller, agreer, frames and dialogue layer do with acts.

Contents

1. [Austin and Searle in brief](#1-austin-and-searle-in-brief)
2. [Explicit performatives in English](#2-explicit-performatives-in-english)
3. [Illocutionary force indicators](#3-illocutionary-force-indicators)
4. [Sentence types and acts in the data](#4-sentence-types-and-acts-in-the-data)
5. [Indirect speech acts](#5-indirect-speech-acts)
6. [Recruitments: requests in interaction](#6-recruitments-requests-in-interaction)
7. [Dialogue act schemes](#7-dialogue-act-schemes)
8. [Speech acts inside narrative](#8-speech-acts-inside-narrative)
9. [How Mova labels acts](#9-how-mova-labels-acts)
10. [Not yet](#10-not-yet)

---

## 1. Austin and Searle in brief

Austin (1962, *How to Do Things with Words*) observed that some utterances do not describe but perform: *I name this ship…*, *I promise to come*. He split an utterance into three acts — the **locutionary** act of saying something meaningful, the **illocutionary** act performed *in* saying it (a warning, a promise), and the **perlocutionary** act performed *by* saying it (frightening, persuading) — and described the **felicity conditions** an act needs to succeed. Searle (1969, *Speech Acts*; 1975, "Indirect speech acts"; 1976, "A classification of illocutionary acts", *Language in Society* 5) made the conditions systematic and proposed five classes of illocutionary acts. None of these works is openly licensed; they are listed under *Wanted* in [sources.md](sources.md#syntax-pragmatics). Open accounts: Green, "Speech Acts", *Stanford Encyclopedia of Philosophy* (https://plato.stanford.edu/entries/speech-acts/), and the background chapter of Floyd, Rossi & Enfield (eds.) (2020, *Getting others to do things: A pragmatic typology of recruitments*, Language Science Press, ch. 1, `books/e/en/enfield-2020-getting-others-things-pragmatic`), which summarises Searle's conditions for a request — the utterance refers to a future act of the hearer, and the speaker believes the hearer can do it, among others (citing Searle 1969: 66).

Searle's five classes, with Mova's `Act` values (the closed enum in [../../prag/src/schema.rs](../../prag/src/schema.rs)):

| Searle's class | point | direction of fit | English verbs | Mova `Act` |
|---|---|---|---|---|
| assertives (representatives) | commit the speaker to the truth of a proposition | words to world | *state, claim, tell, report, insist* | `assert`, `narrate`, `evaluate`, `agree`, `disagree` |
| directives | get the hearer to do something | world to words | *ask, request, order, beg, advise, suggest, warn* | `request`, `ask`, `suggest`, `warn` |
| commissives | commit the speaker to a future action | world to words | *promise, offer, vow, threaten, guarantee* | `promise`, `offer` |
| expressives | express a psychological state about a situation | none | *thank, apologise, congratulate, welcome, deplore* | `thank`, `apologize`, `greet`, `express` |
| declarations | change the world by declaring it changed | both | *pronounce, name, resign, sentence, fire* | — (`other`) |

Mova's set splits Searle's classes along lines that matter for reading tales and lessons: `narrate` (telling what happened in a story) is kept apart from `assert` (stating a general fact), `evaluate` (praise or blame) from both, and `ask` (a request for information) from `request` (a request for action). *Warn* is a directive in Searle's terms but is kept as its own act because its consequences differ: a warning predicts harm. Declarations are rare in the data and fall under `other`.

## 2. Explicit performatives in English

### 2.1 Form

An explicit performative names its own act with a verb in the first person, simple present, active, indicative, often with *hereby*: *I promise…*, *I thank you…*, *I beg you…*, *I apologise…*, *I warn you…*. The *hereby* test separates performative uses (*I hereby promise*) from descriptions (*\*I hereby promised*, *\*She hereby promises*).

| performative | example (tale, parse database) | Mova `act` | UD shape |
|---|---|---|---|
| *I thank you* | *"Master, I thank you, and wish you all prosperity.* — `book:pg128:1865` (*The Arabian Nights Entertainments*) | `thank` | `nsubj(thank, I)`, `obj(thank, you)` |
| *I thank your Majesty for…* | *"Sire," said Saouy, "I thank your Majesty for the justice you do me.* — `book:pg128:3073` | `thank` | `obl(thank, justice)` + `case` *for* |
| *I beg you* | *Then the King cried: "Oh! my most trusty John, pardon! pardon!* — `book:pg503:3704` (*The Blue Fairy Book*; elliptical) | `apologize` | fragment |
| *I vote…* | *I vote the young lady tells us a story.* — `book:pg11:590` (*Alice*) | `suggest`, indirect | `ccomp(vote, tells)` |
| *I wish you would…* | *I wish you would have this cat removed!* — `book:pg11:718` | `request`, indirect | `ccomp(wish, have)` |
| *I am sorry* | *"I'm so sorry," said Wendy Moira Angela.* — `book:pg16:344` (*Peter Pan*) | `apologize` | copular *sorry* |

Performative verbs take the content of the act as a clausal complement (`ccomp`: *I promise that I will come*; `xcomp`: *I promise to come*) or as an object; the syntax is that of chapter 23. The performative verb is the root, but the act is carried by the whole construction: the frame reader makes such a clause a `Say` frame with the act of its verb (§8).

### 2.2 Hedged and embedded performatives

English often embeds the performative under a modal or a verb of wanting, which softens it and makes it formally indirect: *I would like to thank…*, *I must ask you…*, *There is one which I should like to present to your majesty* (`book:pg128:487`, labelled `offer`, indirect), *I should like to do something to show how grateful I am* (`book:pg3027:1170`, labelled `thank`, indirect). These are conventional politeness forms (32).

### 2.3 Performative verbs in the data

Mova maps verbs of speaking to acts directly when they introduce speech (`act_of` in [../../prag/src/frames.rs](../../prag/src/frames.rs)):

| verb lemma(s) | act |
|---|---|
| *ask, wonder* | `ask` |
| *beg, demand, plead, order* | `request` |
| *promise* | `promise` |
| *warn* | `warn` |
| *thank* | `thank` |
| *apologize/apologise* | `apologize` |
| *greet* | `greet` |
| *agree* | `agree` |
| *correct, protest, argue* | `disagree` |
| *suggest* | `suggest` |
| *shout, cry, scream, exclaim, sing, yell* | `express` |
| *say, tell, answer, reply, explain, add, remark, mutter, whisper, call* | `assert` |
| any other | `other` |

In addition, all verbs whose VerbNet classes are all communication classes (37.x: *whisper, mumble, inquire, complain, gossip…*, 152 verbs) count as verbs of speech ([../../docs/pragmatics.md](../../docs/pragmatics.md), *Speech*).

## 3. Illocutionary force indicators

Searle called the forms that signal force "illocutionary force indicating devices". In written English they are:

| device | signals | example | UD / feature |
|---|---|---|---|
| sentence type | the default act (§4) | *Sit down.* / *Did she sit?* / *She sat.* | `Mood=Imp`; inversion; wh-word (21 §8, 26) |
| performative verb | the act named | *I promise…* | root verb, 1st person, present |
| modal verbs | ability, permission, obligation, volition → request, offer, suggestion, warning | *Can you…?*, *May I…?*, *You must…*, *Shall I…?*, *You'd better…* | `aux` (21 §3) |
| *please* | request | *Please sit.*, *Can you please…?* | `discourse` (INTJ) or `advmod` |
| *let's* / *let us* | suggestion including the speaker | *Let's just imagine this is Rhodes* (`book:pg11339:693`) | `Let` root + `obj` *'s* + `xcomp` |
| *why don't you* / *how about* | suggestion | *Why don't you ask her?* | wh-question form |
| tags | confirmation request, softened assertion | *You know the song, perhaps?* (`book:pg11:574`) | `parataxis` / `advmod` (26) |
| vocatives | address; often with requests and greetings | *Take care, my friend* | `vocative` (21 §9) |
| punctuation | question, exclamation | *?*, *!* | `punct`; Mova's form rule P0 |
| intonation | question vs statement, irony, focus | rising declarative *You're leaving?* | not in text |

Mova's agreer takes the sentence form from the tree with these devices (rule P0, `tree_form` in [../../prag/src/agree.rs](../../prag/src/agree.rs)): a final *?* gives `q`, or `wh` when the first word is a wh-word (Penn tags WDT, WP, WP$, WRB); a root base-form verb (VB) with no subject and no auxiliary before it, opening the sentence after at most two softeners (*please, just, never, always, do, don't, not, now, then, so, and, but, first, also*), gives `imp`; a final *!* gives `excl`; anything else is `decl`. For direct speech the form of the speech itself is used.

## 4. Sentence types and acts in the data

### 4.1 The default and the rest

Each sentence type has a default act: declaratives assert, interrogatives ask, imperatives request. The parse database shows how often the default holds (counts of `$MOVA_DATA/parsedb/prag.tsv`, 311 101 rows, 2026-10-17; the full cross-table is in 30 §7.3):

| form | rows | default act | share with the default | the next most frequent acts |
|---|---:|---|---:|---|
| `decl` | 257 228 | `assert` / `narrate` | 93.0% | `evaluate` 5 556, `warn` 2 180, `request` 1 906, `express` 1 649, `promise` 1 575, `suggest` 1 411 |
| `imp` | 14 587 | `request` | 74.3% | `warn` 1 604, `suggest` 1 217, `assert` 298 |
| `wh` | 6 515 | `ask` | 88.1% | `evaluate` 294, `express` 137, `assert` 105 |
| `q` | 4 966 | `ask` | 62.4% | `request` 497, `evaluate` 298, `assert` 285, `offer` 210, `suggest` 167 |
| `excl` | 2 893 | `express` / `evaluate` | 60.1% | `assert` 489, `narrate` 460 |
| `frag` | 24 505 | — | — | `assert` 16 037, `other` 3 527, `narrate` 2 988 |

Yes/no questions are the least faithful to their default: more than a third of them do something other than ask.

### 4.2 Examples by cell

All from the tale part of the parse database (public-domain texts); the act is the agreed label, *ind.* marks `indirect=1`.

| form → act | example | id |
|---|---|---|
| `imp` → `request` | *"Get along with you!" answered the merchant.* | `book:pg22373:3813` |
| `imp` → `warn` | *Give a wide berth to those who can do damage at a distance.* | `book:pg11339:1373` (*Aesop's Fables*) |
| `imp` → `warn` | *Betray a friend, and you'll often find you have ruined yourself.* | `book:pg11339:495` |
| `imp` → `suggest` | *Look and see which way the wind blows before you commit yourself.* | `book:pg11339:434` |
| `imp` → `offer` | *"Have some wine," the March Hare said in an encouraging tone.* | `book:pg11:522` (*Alice*) |
| `imp` → `offer`, ind. | *Let me put them on.* | `book:pg27200:13519` (*Andersen*) |
| `imp` → `thank`, ind. | *Allow me to embrace you as a sign of my eternal gratitude.* | `book:pg500:2739` (*Pinocchio*) |
| `imp` → `agree`, ind. | *"Let it be so," she replied.* | `book:pg2892:1379` (*Irish Fairy Tales*) |
| `imp` → `evaluate`, ind. | *"Serve you right!"* | `book:pg4018:2471` (*Japanese Fairy Tales*) |
| `q` → `request`, ind. | *"Can you not protect me, then?" inquired the sister.* | `book:pg11027:1158` (*Grimm's Fairy Stories*) |
| `q` → `request`, ind. | *…"Would you kindly bring me some water from the stream close by?* | `book:pg11339:1145` |
| `q` → `assert`, ind. | *Can you doubt any longer?"* | `book:pg128:2328` |
| `q` → `assert`, ind. | *But who knows that?* | `book:pg11027:933` |
| `q` → `apologize`, ind. | *O Chinese Venus! how shall I excuse myself for giving you so much trouble?* | `book:pg3282:247` (*The Brown Fairy Book*) |
| `decl` → `request`, ind. | *I wish you would have this cat removed!* | `book:pg11:718` |
| `decl` → `suggest`, ind. | *I vote the young lady tells us a story.* | `book:pg11:590` |
| `decl` → `ask`, ind. | *"I want to know if Catherine is indoors!" said she.* | `book:pg11027:562` |
| `decl` → `offer`, ind. | *"Well," says she, "I wouldn't mind looking after thee myself."* | `book:pg14241:1352` (*More English Fairy Tales*) |
| `decl` → `warn`, ind. | *You better run 'long.* | `book:pg2306:255` (*Uncle Remus*) |
| `decl` → `greet`, ind. | *I hope you are quite well?'* | `book:pg3027:576` (*The Orange Fairy Book*) |
| `decl` → `promise` | *"I will run and get someone else," said Rose-Red.* | `book:pg11027:1455` |
| `decl` → `disagree`, ind. | *"I should like it very much," said Alice, "but I haven't been invited yet."* | `book:pg11:502` |
| `frag` → `agree`, ind. | *"Why not?" she replied.* | `book:pg55:821` (*The Wonderful Wizard of Oz*) |
| `frag` → `request` | *Off with his whiskers!"* | `book:pg11:1046` |
| `frag` → `request`, ind. | *"If you will hold the horse."* | `book:pg7439:497` (*English Fairy Tales*) |
| `excl` → `evaluate` | *"What a fellow you are!" said he to himself, astonished at his own bravery.* | `book:pg11027:585` |
| `excl` → `promise` | *I won't never do so no mo'!* | `book:pg2306:1301` |

Several rows show grammar that later chapters take up: the conditional fragment *If you will hold the horse* used as a request (an insubordinate *if*-clause, 24); the rhetorical questions *Can you doubt any longer?* and *But who knows that?* as assertions (26, 34); the negative concord of *I won't never do so no mo'* in dialect (27, 37).

## 5. Indirect speech acts

### 5.1 What makes an act indirect

An act is **indirect** when it is performed by means of another: the literal act (a question about ability) is not the point, and the hearer infers the intended act (a request). In Mova's schema, `indirect=1` means "the form is not the usual one for the act" ([../../docs/context.md](../../docs/context.md)). In the parse database 3 536 rows (1.1%) are indirect (2026-10-17); the most frequent cells:

| form / act | indirect rows |
|---|---:|
| `decl` / `request` | 726 |
| `q` / `request` | 401 |
| `q` / `assert` | 217 |
| `q` / `evaluate` | 188 |
| `wh` / `evaluate` | 183 |
| `q` / `offer` | 172 |
| `q` / `ask` | 171 |
| `wh` / `ask` | 157 |
| `decl` / `suggest` | 156 |
| `q` / `suggest` | 124 |

The `q` / `ask` and `wh` / `ask` cells are indirect questions in a different sense: a question that asks something other than what it literally asks (*Do you know it is a whole year's journey from here?* — `book:pg128:671`, which asks whether the hearer realises the distance, not whether he knows it).

### 5.2 Conventional indirect requests in English

Requests in English are mostly made indirectly, by questioning or asserting one of the conditions a request needs. Gordon & Lakoff (1971, "Conversational postulates", CLS 7 — Wanted) and Searle (1975) observed that a speaker can request by asking about the hearer's ability or willingness, or by stating the speaker's want:

| condition questioned or stated | English forms | example |
|---|---|---|
| hearer's ability | *Can/Could you…?*, *Are you able to…?* | *Can you pass the salt?* |
| hearer's willingness | *Will/Would you…?*, *Would you mind…?*, *Would you be willing to…?* | *Would you kindly bring me some water…?* (`book:pg11339:1145`) |
| speaker's want | *I want/would like you to…*, *I wish you would…*, *I'd appreciate…* | *I wish you would have this cat removed!* (`book:pg11:718`) |
| reason for the act | *Why don't you…?*, *You'd better…*, *You should…* | *You better run 'long.* (`book:pg2306:255`) |
| the act itself, embedded | *I must ask you to…*, *Let me ask you…* | *But one thing we would ask of you.* (`book:pg128:1328`) |
| availability of an object | *Do you have…?*, *Is there any…?* | *Do you happen to have a watch?* |

The forms are **conventional**: *Can you…?* works as a request while its paraphrase *Are you able to…?* does not, and *please* fits *Can you please…?* but not the paraphrase. Hinkelman & Allen (1989, "Two Constraints on Speech Act Ambiguity", https://aclanthology.org/P89-1026, `papers/h/hi/hin/hinkelman-1989-two-constraints-speech-act-ambiguity`) argue that such conventions must be combined with plan-based reasoning, because pure idiom lists ignore context and pure inference cannot explain the conventions.

### 5.3 Computational accounts

| approach | idea | source |
|---|---|---|
| plan-based inference | hearers reconstruct the speaker's plan: beliefs, wants, actions with preconditions and effects; a question about a precondition of an action the hearer can do signals a request for it | Allen 1979, https://aclanthology.org/P79-1021 (`papers/a/al/all/allen-1979-plans-inference-indirect-speech-acts`); Perrault & Allen 1980, https://aclanthology.org/J80-3003 (`papers/p/pe/per/perrault-1980-plan-based-analysis-indirect-speech`) |
| conditions as rules | a request is recognised when the speaker wants the act, the hearer is capable, willing and obligated | Brown 1980, "Characterizing Indirect Speech Acts", https://aclanthology.org/J80-3002 (`papers/b/br/bro/brown-1980-characterizing-indirect-speech-acts`) |
| schemas with context | an utterance, its literal meaning, contexts and intended meanings; reasoning in Answer Set Programming with closed-world defaults; human agreement on hard cases only 44% | Sarathy et al. 2020, https://aclanthology.org/2020.coling-main.433 (`papers/s/sa/sar/sarathy-2020-reasoning-requirements-indirect-speech-act`); Roque et al. 2020, https://aclanthology.org/2020.lrec-1.28 (`papers/r/ro/roq/roque-2020-developing-corpus-indirect-speech-act`) |
| indirect answers | polar questions answered without *yes/no* (*"I'm starving"* to *Are you hungry?*); the Circa corpus; fine-tuned classifiers reach about 88% on four labels and 85% on six, and fall when a scenario is unseen | Louis, Roth & Radlinski 2020, https://aclanthology.org/2020.emnlp-main.601 (`papers/l/lo/lou/louis-2020-i-d-rather-just-go`) — data for tests only |

The 44% human agreement figure is from the summary of Sarathy et al. in [../../docs/pragmatics.md](../../docs/pragmatics.md).

### 5.4 Mova's indirect requests

Mova reasons about indirect requests in its dialogue layer ([../../world/src/talk.rs](../../world/src/talk.rs), `indirect_request`), following Brown's conditions as used by Sarathy et al.:

1. The sentence starts with *can/could/would/will you (please)* or *would you mind*; the rest is the action.
2. **Wanted**: a modal question (or *please*) shows the speaker wants it.
3. **Capable**: the action is one the system can do (a fixed list of commands).
4. **Willing**: the action is not blocked right now (state-changing actions wait while training runs).
5. **Obligated**: the speaker is one the system works for.

If all hold, the question is read as a request and the reply says why ("you want it, I can do it, I am willing now, and I work for you"); if the action is not one it can do, the direct reading wins and the reply answers the ability question, listing what it can do. Tests: `tests03::indirect_requests_need_their_preconditions` (2026-10-03, [../../docs/pragmatics.md](../../docs/pragmatics.md)). This layer works on cue words in the sentence, not on the tree; the sentence labeller (§9) learns `indirect` from silver labels and has no precondition reasoning.

## 6. Recruitments: requests in interaction

Conversation analysis treats requests as one way among many to **recruit** others: Floyd, Rossi & Enfield (eds.) (2020) define a recruitment sequence as one where a person gets another to do something practical, verbally or not, and compare eight languages with one coding scheme; they find cross-linguistic diversity in this domain "relatively low" (ch. 1, `books/e/en/enfield-2020-getting-others-things-pragmatic`). In the English chapter, Kendrick (2020, "Recruitment in English: A quantitative study", ch. 4 of the same volume) finds that most English recruitment sequences in his video data are minimal — a single recruiting move — in 65.9% of cases (n = 139), organised as adjacency pairs where an imperative (*move over a little*) or an interrogative request makes compliance the expected response.

The point for a text reader is that a request is a sequence, not a sentence: it may be pre-empted, granted by an action, or refused with a reason. The tales show this: *"Can you not protect me, then?" inquired the sister* (`book:pg11027:1158`) is followed by the brother's response, and the frame reader links the two through `Say` frames and `next` (38).

## 7. Dialogue act schemes

### 7.1 DAMSL and SWBD-DAMSL

Dialogue-act schemes classify utterances in conversation by their function, including functions Searle's classes do not cover: backchannels (*uh-huh*), acknowledgements, answers, turn management. **SWBD-DAMSL** adapted the DAMSL scheme to the Switchboard telephone corpus: Stolcke et al. (2000, "Dialogue act modeling for automatic tagging and recognition of conversational speech", *Computational Linguistics* 26(3), https://aclanthology.org/J00-3003, `papers/s/st/sto/stolcke-2000-dialogue-act-modeling-automatic-tagging`) annotated 1 155 conversations (about 198 000 utterances) with 42 labels; the distribution is very skewed — statements about 36%, backchannel/acknowledge 19%, opinion 13%. Their HMM tagger reached 71% accuracy on reference transcripts and 65% on recognised speech with prosody, against a 35% baseline and 84% human agreement. Jurafsky et al. (1998, "Lexical, Prosodic, and Syntactic Cues for Dialog Acts", https://aclanthology.org/W98-0319, `papers/j/ju/jur/jurafsky-1998-lexical-prosodic-syntactic-cues-dialog`) show how short feedback words (*yeah, uh-huh, okay*) split into continuers, agreements, yes-answers and assessments by their words, prosody and the grammar of assessments.

### 7.2 ISO 24617-2

The ISO standard for dialogue act annotation (Bunt et al. 2010, https://aclanthology.org/L10-1385, `papers/b/bu/bun/bunt-2010-iso-standard-dialogue-act-annotation`; revised in Bunt, Petukhova & Fang 2017, https://aclanthology.org/W17-7404; second edition Bunt et al. 2020, https://aclanthology.org/2020.lrec-1.69, `papers/b/bu/bun/bunt-2020-iso-standard-dialogue-act-annotation`) is multidimensional: one segment can carry several functions in different dimensions. Summary after [../../docs/pragmatics.md](../../docs/pragmatics.md):

| dimension | what it covers | example functions |
|---|---|---|
| Task | the business of the dialogue | Inform, Agreement, Disagreement, Correction, Answer, Confirm, Disconfirm, Set-/Propositional-/Check-/Choice-Question, Offer, Promise, Request, Instruct, Suggest, Accept/Decline Request/Suggest |
| Auto-Feedback, Allo-Feedback | understanding of one's own / the partner's previous turn | positive or negative, on five levels: attention, perception, interpretation, evaluation, execution |
| Turn Management | who speaks | Turn Take, Turn Keep, Turn Release |
| Time Management | stalling, pausing | Stalling, Pausing |
| Discourse Structuring | opening and shifting topics | Interaction Structuring, Opening |
| Own / Partner Communication Management | self-correction, completion | Self-Correction, Completion |
| Social Obligations Management | rituals | Init/Return Greeting, Self-Introduction, Apology, Accept Apology, Thanking, Accept Thanking, Init/Return Goodbye |
| Contact Management | checking the channel | Contact Check |

Qualifiers add conditionality (*Can you tell me…* as a conditional request), certainty and sentiment; an answer depends on its question, and feedback on a segment.

### 7.3 Mova's acts against the schemes

| Mova `Act` | ISO 24617-2 | SWBD-DAMSL (tag family) |
|---|---|---|
| `assert`, `narrate` | Inform | statement-non-opinion |
| `evaluate` | Inform (+ sentiment qualifier) | statement-opinion, appreciation |
| `ask` | Set-/Propositional-/Check-/Choice-Question | yes-no / wh / declarative / tag question |
| `request` | Request, Instruct | action-directive |
| `suggest` | Suggest | action-directive (open option) |
| `offer` | Offer | offer, commit |
| `promise` | Promise | commit |
| `warn` | Inform / Instruct (no own function) | — |
| `agree`, `disagree` | Agreement, Disagreement; Accept/Decline | agree/accept, reject |
| `thank`, `apologize`, `greet` | Social Obligations Management | thanking, apology, conventional opening/closing |
| `express` | Inform + sentiment | exclamation |
| `other` | — | other, non-verbal, uninterpretable |

Mova does not annotate feedback, turn or time management, because its texts are written narratives and lessons, where those functions are mostly absent; the dialogue layer handles greetings, thanks, apologies and goodbyes as social obligations (38).

## 8. Speech acts inside narrative

In tales most acts are performed by characters inside quotations. Mova's frame reader builds a `Say` frame for a verb of saying or thinking and puts what was said into its `quote` ([../../prag/src/frames.rs](../../prag/src/frames.rs); [../../docs/pragmatics.md](../../docs/pragmatics.md)):

- the act of a `Say` frame comes from its verb (§2.3);
- for the plain verbs of saying (*say, answer, reply, add, call, call out, whisper, mutter, remark*) the act is refined by what is said: a quote with *?* makes it `ask`, a quote with *!* makes it `express` (switch `act-by-quote`);
- *say* + *sorry* becomes `apologize` (switch `say-sorry`);
- the speaker after a quote (*"…" cries the Badger*), the person told or asked as the recipient, and inside the quote *I* = the speaker, *you* = the addressee;
- a quote with no verb of saying is given to the last speaker of the discourse.

(1) `world frames` on the whole sentence of *Aesop's Fables* `book:pg11339:1145` (Mova 0.4, CPU, 2026-10-17; the frames of the narration before the quote abridged):

```text
By and by he began to revive, and, feeling very hungry, called out to a passing Sheep and said,
"Would you kindly bring me some water from the stream close by?"

#3 Say(assert) _ call out to=sheep
     roles by vn manner_speaking-37.3 «NP V PP.recipient»: agent=Agent to sheep=Recipient
#5 Say(ask) _ say | says→#6 cause→#0
     roles by vn say-37.7-1-1 «NP V that S»: agent=Agent
#6 Fact(does) you bring patient=water to=i place=stream attrs=kindly | quoted
     roles by vn bring-11.3-1 «NP V NP NP»: agent=Agent i=Destination water=Theme
```

The `Say` frame of *said* gets the literal act `ask` from the *?* of its quote, and *call out to a passing Sheep* makes the sheep a recipient of speech. Three things are missing or wrong, and they are typical: the speaker is unresolved (`_`, the wolf of the earlier sentences is out of the one-sentence context given here), the sheep is not carried over as the addressee of *said*, and the quoted request is a general `Fact(does)` rather than an event the hearer is asked to perform. The intended act (`request`) is in the sentence label (`q` / `request` / `indirect=1`), not in the frame. Bringing the two together is listed under *Not yet*.

Weakest on the frame bench are exactly these speech parts: speaker F1 .32 and says F1 .38 on dev (2026-10-10), because dialogue often has no verb of saying and the parser splits quotes ([../../docs/pragmatics.md](../../docs/pragmatics.md), *Not yet*).

## 9. How Mova labels acts

The act field is labelled per sentence by the pipeline of 30 §8: teacher silver → three transparent classifiers on the same features → the agreer → the tiered teacher. Specific to acts:

- **Features** ([../../prag/src/feats.rs](../../prag/src/feats.rs)): the first words and the last word of the utterance, end punctuation, *?* and *!*, length; from the tree the root's UPOS, form and lemma, tense, the first auxiliary, inversion, a wh-word, a modal, negation, the subject and its person, *you*, first person, a vocative; the act of the previous sentence and whether the sentence is direct speech.
- **Agreer rules** ([../../prag/src/agree.rs](../../prag/src/agree.rs)): **P1** an imperative whose act is narrate/assert/evaluate/other becomes `request` (or `warn` when both NLI witnesses say warn); **P2** outside the books course, `narrate` becomes `assert` when both witnesses agree — the labeller learned on tales and called lesson facts narration.
- **Teacher changes** (`teacher` column, 2026-10-17): the act was changed in 53 023 of 311 101 rows (17.0%). The new values, most often: `assert` 23 371, `narrate` 13 316, `warn` 3 474, `other` 3 399, `evaluate` 1 910, `request` 1 844, `suggest` 1 387, `express` 1 171, `ask` 1 051. The assert/narrate border (a general statement vs telling what happened) accounts for most changes; among the rarer acts, warnings are the ones the labeller misses most.

## 10. Not yet

- **Indirectness is labelled, not inferred.** The precondition reasoning of §5.4 runs in the dialogue layer only, on cue words; the sentence labeller predicts `indirect` from features.
- **Frames keep the literal act.** A `Say` frame of *"Would you kindly bring me some water?"* is `ask`; the request it performs is not propagated into the frame.
- **No multidimensional acts.** One act per sentence; ISO 24617-2's feedback, turn and time dimensions are not annotated.
- **Declarations** have no act of their own (`other`).
- **Prosody** is absent: rising declaratives and intonation-marked acts are invisible in text.
- **Evaluation is weakly checked**: the agreer's evaluation rule (P3) was switched off after the witnesses agreed on wrong answers ([../../prag/src/agree.rs](../../prag/src/agree.rs)).

## In Mova

- The act enum and labels: [../../prag/src/schema.rs](../../prag/src/schema.rs); features: [../../prag/src/feats.rs](../../prag/src/feats.rs); classifiers: [../../prag/src/model.rs](../../prag/src/model.rs); agreer (P0 form from the tree, P1, P2): [../../prag/src/agree.rs](../../prag/src/agree.rs); silver: [../../prag/src/silver.rs](../../prag/src/silver.rs).
- Speech in frames (`act_of`, `Say` frames, act by the quote's punctuation): [../../prag/src/frames.rs](../../prag/src/frames.rs).
- Dialogue acts, social obligations and indirect requests with preconditions: [../../world/src/talk.rs](../../world/src/talk.rs).
- Docs: [../../docs/context.md](../../docs/context.md) (*Pragmatics: what was meant*), [../../docs/pragmatics.md](../../docs/pragmatics.md) (*Speech acts and dialogue acts*), [../../docs/pragmatics.md](../../docs/pragmatics.md).

## Sources

- Austin, J. L. 1962. *How to Do Things with Words*. Oxford: Clarendon. Not open — *Wanted*.
- Searle, J. R. 1969. *Speech Acts*. 1975. Indirect speech acts. 1976. A classification of illocutionary acts, *Language in Society* 5. Not open — *Wanted*.
- Gordon, D., Lakoff, G. 1971. Conversational postulates. CLS 7. Not open — *Wanted*.
- Green, M. "Speech Acts". *Stanford Encyclopedia of Philosophy*. https://plato.stanford.edu/entries/speech-acts/
- Floyd, S., Rossi, G., Enfield, N. J. (eds.) 2020. *Getting others to do things: A pragmatic typology of recruitments*. Language Science Press. Ch. 1 (introduction), ch. 4 (Kendrick, K. H., Recruitment in English: A quantitative study). `books/e/en/enfield-2020-getting-others-things-pragmatic`
- Allen, J. F. 1979. Plans, Inference, and Indirect Speech Acts. ACL. https://aclanthology.org/P79-1021; `papers/a/al/all/allen-1979-plans-inference-indirect-speech-acts`
- Perrault, C. R., Allen, J. F. 1980. A Plan-Based Analysis of Indirect Speech Acts. *AJCL* 6(3–4). https://aclanthology.org/J80-3003; `papers/p/pe/per/perrault-1980-plan-based-analysis-indirect-speech`
- Brown, G. P. 1980. Characterizing Indirect Speech Acts. *AJCL* 6(3–4). https://aclanthology.org/J80-3002; `papers/b/br/bro/brown-1980-characterizing-indirect-speech-acts`
- Hinkelman, E. A., Allen, J. F. 1989. Two Constraints on Speech Act Ambiguity. ACL. https://aclanthology.org/P89-1026; `papers/h/hi/hin/hinkelman-1989-two-constraints-speech-act-ambiguity`
- Sarathy, V., Tsuetaki, A., Roque, A., Scheutz, M. 2020. Reasoning Requirements for Indirect Speech Act Interpretation. COLING. https://aclanthology.org/2020.coling-main.433; `papers/s/sa/sar/sarathy-2020-reasoning-requirements-indirect-speech-act`
- Roque, A., Tsuetaki, A., Sarathy, V., Scheutz, M. 2020. Developing a Corpus of Indirect Speech Act Schemas. LREC. https://aclanthology.org/2020.lrec-1.28; `papers/r/ro/roq/roque-2020-developing-corpus-indirect-speech-act`
- Louis, A., Roth, D., Radlinski, F. 2020. "I'd rather just go to bed": Understanding Indirect Answers. EMNLP. https://aclanthology.org/2020.emnlp-main.601; `papers/l/lo/lou/louis-2020-i-d-rather-just-go`
- Stolcke, A. et al. 2000. Dialogue act modeling for automatic tagging and recognition of conversational speech. *Computational Linguistics* 26(3). https://aclanthology.org/J00-3003; `papers/s/st/sto/stolcke-2000-dialogue-act-modeling-automatic-tagging`
- Jurafsky, D., Shriberg, E., Fox, B., Curl, T. 1998. Lexical, Prosodic, and Syntactic Cues for Dialog Acts. https://aclanthology.org/W98-0319; `papers/j/ju/jur/jurafsky-1998-lexical-prosodic-syntactic-cues-dialog`
- Bunt, H. et al. 2010. Towards an ISO Standard for Dialogue Act Annotation. LREC. https://aclanthology.org/L10-1385; `papers/b/bu/bun/bunt-2010-iso-standard-dialogue-act-annotation`
- Bunt, H., Petukhova, V., Fang, A. C. 2017. Revisiting the ISO standard for dialogue act annotation. ISA-13. https://aclanthology.org/W17-7404; `papers/b/bu/bun/bunt-2017-revisiting-iso-standard-dialogue-act`
- Bunt, H. et al. 2020. The ISO Standard for Dialogue Act Annotation, Second Edition. LREC. https://aclanthology.org/2020.lrec-1.69; `papers/b/bu/bun/bunt-2020-iso-standard-dialogue-act-annotation`
- Mova's parse database `$MOVA_DATA/parsedb/prag.tsv` and trees, counted 2026-10-17; tales identified by Project Gutenberg number (titles from the tales registry).
- Repo docs listed under *In Mova*.
