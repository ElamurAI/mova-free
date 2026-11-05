# 30. Pragmatics and discourse: overview

Pragmatics is the study of meaning in use: what a speaker does with a sentence, what they mean beyond what they say, what they take for granted, and how sentences hang together into a text or a conversation. This chapter sets out the layers of meaning that the following chapters describe for English, the context they depend on, and how Mova represents them — frames for who did what, a closed set of sentence-level labels, small rule-based inference modules, and a world that keeps the discourse. It closes with the label counts of Mova's parse database as of 2026-10-17.

Contents

1. [The chapters of the pragmatics part](#1-the-chapters-of-the-pragmatics-part)
2. [Layers of meaning](#2-layers-of-meaning)
3. [Context](#3-context)
4. [From syntax to pragmatics](#4-from-syntax-to-pragmatics)
5. [Mova's pragmatics: the design](#5-movas-pragmatics-the-design)
6. [The sentence-level labels](#6-the-sentence-level-labels)
7. [The labels in the parse database](#7-the-labels-in-the-parse-database)
8. [How labels are made and checked](#8-how-labels-are-made-and-checked)
9. [Measurements](#9-measurements)
10. [What is not there yet](#10-what-is-not-there-yet)

---

## 1. The chapters of the pragmatics part

| file | what it covers |
|---|---|
| 30-pragmatics-overview.md | this chapter: layers of meaning, context, Mova's design, the label counts |
| [31-speech-acts.md](31-speech-acts.md) | speech acts, sentence types and acts, indirect speech acts, dialogue act schemes |
| [32-politeness-and-hedging.md](32-politeness-and-hedging.md) | face and politeness, impoliteness, hedges and boosters, epistemic stance |
| [33-implicature-and-presupposition.md](33-implicature-and-presupposition.md) | the cooperative principle, scalar and other implicatures, presupposition triggers, RSA |
| [34-irony-and-figurative-language.md](34-irony-and-figurative-language.md) | irony, sarcasm, hyperbole, understatement, rhetorical questions, metaphor, idioms, humour |
| [35-reference-and-coreference.md](35-reference-and-coreference.md) | referring expressions, givenness, anaphora, bridging, coreference resolution |
| [36-discourse-relations.md](36-discourse-relations.md) | coherence relations (RST, PDTB, SDRT), connectives, narrative and temporal structure, QUD |
| [37-register-and-variation.md](37-register-and-variation.md) | register, genre, style, spoken and written, web language, dialects, archaic and biblical English |
| [38-dialogue-and-discourse-markers.md](38-dialogue-and-discourse-markers.md) | conversation structure, grounding, discourse markers, vocatives, reported speech |

The syntax chapters (20–29) are the base: every pragmatic reading in Mova starts from a UD tree.

## 2. Layers of meaning

A sentence used on an occasion carries several kinds of meaning at once. The classical division goes back to Grice's distinction between what is said and what is implicated (Grice 1975, "Logic and Conversation" — not open, see the *Wanted* list in [sources.md](sources.md#syntax-pragmatics); an open survey is Davis, "Implicature", *Stanford Encyclopedia of Philosophy*, https://plato.stanford.edu/entries/implicature/) and to Austin's and Searle's speech act theory (open survey: Green, "Speech Acts", *SEP*, https://plato.stanford.edu/entries/speech-acts/).

| layer | question it answers | English example | chapter |
|---|---|---|---|
| sentence meaning | what do the words and the structure mean? | *Can you open the window?* = a question about ability | 20–29 |
| what is said | what proposition is expressed here, with references fixed? | *the hearer is able to open that window now* | 35 |
| illocutionary force | what is the speaker doing? | a request | 31 |
| presupposition | what is taken for granted? | there is a (salient) window; it is closed | 33 |
| conversational implicature | what is meant beyond what is said, inferred from the assumption that the speaker is cooperative? | *Some of the guests left* → not all of them | 33 |
| conventional implicature | what do certain words add without being part of the truth conditions? | *She is poor **but** honest* → a contrast | 33, 36 |
| social meaning | what does the form say about the relation and the setting? | *could you possibly…* → deference | 32, 37 |
| figurative meaning | is it meant literally at all? | *Great, another Monday.* → the opposite | 34 |
| discourse meaning | how does this sentence relate to the others? | *because*, *then*, an answer to an open question | 36, 38 |

Two other organising ideas run through the chapters. First, **pragmatics is about inference under uncertainty**: the hearer chooses the most plausible reading given the words, the context and what speakers usually do — the view behind Grice's maxims, relevance theory and the probabilistic Rational Speech Acts model (33). Second, **much of pragmatics is still unmapped**: Levinson (2024, *The Dark Matter of Pragmatics: Known Unknowns*, Cambridge Elements, `books/l/le/levinson-2024-dark-matter-pragmatics-known`, card only in the library) argues that the coordination of action in interaction — turn-taking, sequence organisation, the recognition of what an utterance is *for* — is a large body of regular behaviour that linguistic theory has hardly described (38).

## 3. Context

Pragmatic meaning is relative to a context. The parts of context the following chapters use:

| part of context | what it holds | English expressions that depend on it | in Mova |
|---|---|---|---|
| participants | speaker, addressee, audience; their roles | *I, you, we*; vocatives; honorifics | `voice` label (narrator, character); `I`/`you` inside quotes resolved to the speaker and addressee of the Say frame |
| time and place | the moment and location of speaking | *now, here, today, yesterday, this*; tense | a time anchor is planned ([../../docs/context.md](../../docs/context.md), *What we infer from context*) |
| the preceding text | entities, events, open questions | pronouns, definite NPs, ellipsis, *again*, *too* | `prag::frames::Ctx` (one discourse per text), presupposition licensing in a 50-token window, `world::qud` |
| common ground | what participants mutually take for granted | presuppositions, definites, *of course* | the world of the text (`world`) and the compiled knowledge (`global`) |
| the question under discussion | what the conversation is trying to settle | focus, answers, relevance | `world/src/qud.rs` |
| the activity type and register | lesson, tale, dialogue, law, chat | register markers, politeness | the course and source of a sentence; register lexicons |

Grounding — how participants add to the common ground — is the subject of Clark & Schaefer (1989, "Contributing to Discourse", *Cognitive Science* 13, `papers/c/cl/clark-1989-contributing-discourse`) and Clark & Brennan (1991, "Grounding in Communication", `papers/c/cl/clark-1991-grounding-communication`); see 38. The question under discussion (Roberts 1996/2012, "Information structure in discourse", *Semantics and Pragmatics* 5, https://doi.org/10.3765/sp.5.6) organises focus and relevance; Mova's QUD stack is described in 29 and 36.

## 4. From syntax to pragmatics

Pragmatic categories are signalled by grammatical form, but not one-to-one. The cues Mova reads from the tree:

| pragmatic category | syntactic and lexical cues | syntax chapter |
|---|---|---|
| sentence form → default act | root type, inversion, wh-word, imperative mood, `!`/`?` | 21 §8, 26 |
| indirect request | *can/could/would you* + VP; *I wonder if*; *would you mind* | 26, 31 |
| hedge | *maybe, perhaps, I think, sort of, kind of, seem, might* | 21 §3, 32 |
| polarity and evaluation | evaluative adjectives and verbs; negation flips | 27, 34 |
| presupposition | factive verbs (*know, regret*), change of state (*stop, continue*), iteratives (*again*), additives (*too, also*), definite NPs, clefts | 23, 29, 33 |
| scalar implicature | scale items (*some/most/all*, *warm/hot*, *possible/certain*) not under negation | 22 §3, 27, 33 |
| coherence relation | subordinators and connectives (*because, so, then, although*); clause order | 24, 36 |
| reported speech | verbs of saying with `ccomp` or a quote; inverted quotative (*cries the Badger*) | 23, 29, 38 |
| reference | pronoun form, definiteness, demonstratives, `expl` | 22, 35 |

The tree is necessary but not sufficient: *Can you swim?* and *Can you pass the salt?* have the same structure, and only the context (is ability in question? is the hearer in a position to act?) decides the act (31).

## 5. Mova's pragmatics: the design

### 5.1 Principles

- **No bag of words.** Every pragmatic reader works on the structure — frames built from UD trees — never on the raw words of whole lines ([../../docs/pragmatics.md](../../docs/pragmatics.md); [../../docs/architecture.md](../../docs/architecture.md)).
- **Transparent and deterministic.** Each decision comes with its reason: the VerbNet frame that gave the roles, the rule that fired, the features that tipped a classifier, the RSA table. Ordered maps and fixed tie-breaking make runs repeatable ([../../prag/src/rsa.rs](../../prag/src/rsa.rs); [../../prag/src/model.rs](../../prag/src/model.rs)).
- **A large model is only a teacher.** It writes silver labels and checks samples; it never answers at test time ([../../docs/architecture.md](../../docs/architecture.md), *The teacher loop*).
- **Licences tracked separately.** Pragmatics lives in its own crate (`prag`) so that the licences of its data can be kept apart from the parser's ([../../docs/context.md](../../docs/context.md), *Pragmatics: what was meant*).

### 5.2 The parts

| part | what it does | code |
|---|---|---|
| frames | one frame per event or fact: verb, agent, patient, recipient, place, attributes, obliques; `neg`, `cond`; links `cause`, `next`, `quote`; kinds `Event`, `Fact`, `Say(Act)`, `Bare` | [../../prag/src/frames.rs](../../prag/src/frames.rs), [../../prag/src/vn.rs](../../prag/src/vn.rs), [../../prag/src/roles.rs](../../prag/src/roles.rs) |
| sentence labels | `form`, `act`, `indirect`, `hedge`, `polarity`, `voice`, `means` (§6) | [../../prag/src/schema.rs](../../prag/src/schema.rs) |
| labellers | averaged perceptron, IGTree, k-NN behind one `Classifier` interface, over a fixed feature set from the tree and the words | [../../prag/src/model.rs](../../prag/src/model.rs), [../../prag/src/feats.rs](../../prag/src/feats.rs) |
| agreer | the labeller's labels stand unless written rules, on the evidence of agreeing open models, say otherwise | [../../prag/src/agree.rs](../../prag/src/agree.rs) |
| silver | batch prompts to the teacher with gates and a rejects log | [../../prag/src/silver.rs](../../prag/src/silver.rs), [../../prag/src/opus.rs](../../prag/src/opus.rs) |
| inference | scalar implicatures, presuppositions with licensing, Quality and Manner maxims | [../../prag/src/infer.rs](../../prag/src/infer.rs) |
| irony | sarcasm candidates by incongruity, with the incongruent pair as the explanation | [../../prag/src/irony.rs](../../prag/src/irony.rs) |
| RSA | Rational Speech Acts in exact fractions over small closed sets | [../../prag/src/rsa.rs](../../prag/src/rsa.rs) |
| lens | idioms, proverbs, phrasal verbs and register labels with their real meanings | [../../prag/src/lens.rs](../../prag/src/lens.rs) |
| rules | transformations of frames learned from errors, each written as a sentence | [../../prag/src/rules.rs](../../prag/src/rules.rs) |
| coreference | a multi-pass sieve with a reason for every link | [../../coref/src/sieve.rs](../../coref/src/sieve.rs) |
| discourse and dialogue | QUD stack; social talk, politeness strategies, indirect requests; register lexicons; humour trigger | [../../world/src/qud.rs](../../world/src/qud.rs), [../../world/src/talk.rs](../../world/src/talk.rs), [../../world/src/register.rs](../../world/src/register.rs), [../../world/src/domains.rs](../../world/src/domains.rs) |

The design note behind all this, written on 2026-09-26, describes the target: per utterance, the act and its form, what was meant, what is presupposed and the speaker's stance — added to the literal text so that, for example, a review saying *I wouldn't buy it again* is found by a search for negative reviews ([../../docs/context.md](../../docs/context.md), *Pragmatics: what was meant*). The algorithms chosen from the literature, and why each fits a deterministic Rust model, are listed in [../../docs/pragmatics.md](../../docs/pragmatics.md).

### 5.3 A frame with its pragmatics

(1) `world frames "The fox flattered the crow because she wanted the cheese. The crow opened her beak and the cheese fell."` ([../../docs/pragmatics.md](../../docs/pragmatics.md)):

```text
#0 Event fox flatter patient=crow | cause→#1 next→#2
#1 Event fox(«she») want patient=cheese
#2 Event crow open patient=beak | next→#3
#3 Event cheese fall
```

Three pragmatic operations are already visible: reference (*she* → fox, 35), a coherence relation from a connective (*because* → `cause`, 36), and temporal order from *and* and sentence order (`next`; the Manner maxim "be orderly", 33).

## 6. The sentence-level labels

The first cut of the schema (2026-09-26) labels each sentence with seven fields; the value sets are closed enums, and a value outside the set is a parse error ([../../prag/src/schema.rs](../../prag/src/schema.rs); [../../docs/context.md](../../docs/context.md)):

| field | values | meaning |
|---|---|---|
| `form` | `decl`, `q`, `wh`, `imp`, `excl`, `frag`, `intj`, `other` | the sentence type by form (21 §8) |
| `act` | `assert`, `narrate`, `ask`, `request`, `offer`, `suggest`, `promise`, `thank`, `apologize`, `greet`, `agree`, `disagree`, `evaluate`, `express`, `warn`, `other` | the speech act (31) |
| `indirect` | 0 / 1 | the form is not the usual one for the act (*Can you open the window?*: `q` + `request`) |
| `hedge` | 0 / 1 | the speaker weakens the commitment (*maybe, I think, sort of*) (32) |
| `polarity` | `pos`, `neg`, `none` | the speaker's evaluation |
| `voice` | `narrator`, `character`, `none` | who speaks in the text: the narrator or a character in direct speech (38) |
| `means` | free text, up to ~15 words | what was meant, beyond the literal; written only by the teacher; the labeller does not learn it yet |

When a sentence contains direct speech, the labels describe the speech, not the narrator's frame around it: *"Can you hop on your hind legs?" asked the furry rabbit* is labelled as the rabbit's question ([../../prag/src/feats.rs](../../prag/src/feats.rs), header comment). Quotes are tracked across the sentences of a paragraph.

## 7. The labels in the parse database

The parse database keeps one pragmatics row per sentence next to its tree: `$MOVA_DATA/parsedb/prag.tsv` (`sent_id form act indirect hedge polarity voice teacher`; the newest row wins; written by `tools/cycle`, [../../tools/cycle/src/db.rs](../../tools/cycle/src/db.rs)). All counts below were taken on 2026-10-17 over 311 101 rows.

### 7.1 Sources

| prefix | what | rows |
|---|---|---:|
| `dict:` | dictionary definitions and examples | 163 701 |
| `book:` | public-domain tales (Project Gutenberg and others; e.g. `pg11` *Alice's Adventures in Wonderland*, `pg11339` *Aesop's Fables*, `pg11027` *Grimm's Fairy Stories*, `pg27200` *Fairy Tales of Hans Christian Andersen*) | 118 321 |
| `school:` | Mova's school lessons | 18 655 |
| `math:` | word problems | 8 857 |
| `myst:` | Mova's own texts on mysticism | 1 567 |

### 7.2 Distributions

| field | value | rows | % |
|---|---|---:|---:|
| form | `decl` | 257 228 | 82.7 |
| | `frag` | 24 505 | 7.9 |
| | `imp` | 14 587 | 4.7 |
| | `wh` | 6 515 | 2.1 |
| | `q` | 4 966 | 1.6 |
| | `excl` | 2 893 | 0.9 |
| | `other` / `intj` | 223 / 184 | 0.1 |
| act | `assert` | 192 183 | 61.8 |
| | `narrate` | 68 078 | 21.9 |
| | `request` | 13 545 | 4.4 |
| | `ask` | 9 128 | 2.9 |
| | `evaluate` | 7 302 | 2.3 |
| | `other` | 4 936 | 1.6 |
| | `warn` | 3 899 | 1.3 |
| | `express` | 3 415 | 1.1 |
| | `suggest` | 2 884 | 0.9 |
| | `promise` | 1 618 | 0.5 |
| | `disagree` | 1 573 | 0.5 |
| | `offer` | 985 | 0.3 |
| | `agree` | 843 | 0.3 |
| | `greet` | 387 | 0.1 |
| | `thank` | 222 | 0.1 |
| | `apologize` | 103 | 0.03 |
| indirect | 1 | 3 536 | 1.1 |
| hedge | 1 | 2 718 | 0.9 |
| polarity | `neg` / `pos` / `none` | 18 051 / 14 375 / 278 675 | 5.8 / 4.6 / 89.6 |
| voice | `narrator` / `character` / `none` | 268 513 / 38 406 / 4 182 | 86.3 / 12.3 / 1.3 |

### 7.3 Form and act

The cross-table of form and act (cells of 100 or more rows) shows how far form and act diverge:

| form ↓ act → | assert | narrate | request | ask | evaluate | warn | suggest | express | promise | other acts |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `decl` | 174 926 | 64 386 | 1 906 | — | 5 556 | 2 180 | 1 411 | 1 649 | 1 575 | 3 639 |
| `frag` | 16 037 | 2 988 | 215 | 181 | 379 | — | — | 290 | — | 4 245 |
| `imp` | 298 | — | 10 833 | — | — | 1 604 | 1 217 | — | — | 282 |
| `wh` | 105 | — | — | 5 737 | 294 | — | — | 137 | — | — |
| `q` | 285 | 124 | 497 | 3 098 | 298 | — | 167 | 148 | — | 210 |
| `excl` | 489 | 460 | — | — | 730 | — | — | 1 009 | — | — |

Read along the rows: an imperative is a request three times in four (10 833 of 14 587), but also a warning (1 604) or a suggestion (1 217); a yes/no question is a question only in 62% of cases (3 098 of 4 966), and a request, an evaluation, an assertion or an offer otherwise; declaratives carry every act. This is the territory of indirect speech acts (31).

### 7.4 What the teacher changed

The `teacher` column lists the fields the teacher changed, with their new values (e.g. `act=assert,indirect=1`). The teacher changed at least one field in 95 243 rows (30.6%):

| field | rows changed |
|---|---:|
| `act` | 53 023 |
| `form` | 32 493 |
| `polarity` | 22 496 |
| `voice` | 8 252 |
| `indirect` | 2 980 |
| `hedge` | 1 800 |

The act is the hardest field and the one the teacher corrects most; `form` corrections are mostly fragments and direct speech, where the sentence type of the quote differs from that of the frame around it.

## 8. How labels are made and checked

The pipeline follows the one for trees ([../../docs/architecture.md](../../docs/architecture.md)):

1. **Silver.** The teacher labels open texts — tales as paragraphs with one or two preceding sentences, single sentences from CC0 sources — in batches; outputs pass gates (closed values, `means` length), and rejections go to a log without blind retries ([../../prag/src/silver.rs](../../prag/src/silver.rs)).
2. **Labeller.** Three memory-light models learn from the silver on the same features: an averaged perceptron, IGTree and k-NN (Daelemans, van den Bosch & Weijters 1997; Aha et al. 1991 — `papers/aha-1991-instance-based-learning-algorithms.md` in the repo). Each explains its choice: the features that tipped the balance, the path through the tree, the most similar remembered sentences ([../../prag/src/model.rs](../../prag/src/model.rs)).
3. **Agreement.** The labeller's labels stand unless written rules, on the evidence of open models that agree with each other, say otherwise. Witnesses: two zero-shot NLI models for act, polarity and uncertainty, a certainty regressor; a politeness and an irony model are reported only. Rules in force: **P0** the form comes from the tree (*?* → `q`/`wh`; a root base-form verb without a subject at the start → `imp`; *!* → `excl`; else `decl`), except for direct speech; **P1** an imperative whose act is narrate/assert/evaluate/other becomes `request` (or `warn` if both NLI witnesses say so); **P2** outside the books course, `narrate` becomes `assert` when both witnesses agree. Rules for evaluation, polarity and hedging were switched off after a spot check: the witnesses agreed with each other on wrong answers ([../../prag/src/agree.rs](../../prag/src/agree.rs)).
4. **Teacher in tiers.** The cheap teacher checks every sentence, a stronger one a tenth of those, the strongest a tenth of that; their changes are recorded in the `teacher` column (§7.4).

The tools for comparing with other approaches — RST and PDTB parsers, dialogue-act taggers, politeness and sarcasm models — are collected, with their licences, in [../../docs/pragmatics.md](../../docs/pragmatics.md); most are for tests only.

## 9. Measurements

| what | measure | value | date | source |
|---|---|---|---|---|
| frame reader, dev | micro F1 over roles and links | .579 → .633 | 2026-10-10 | [../../docs/pragmatics.md](../../docs/pragmatics.md) |
| frame reader, held-out (run once) | micro F1 | .618 → .667 | 2026-10-10 | same |
| frame reader, negative control | first-noun baseline | ~.17 | 2026-10-10 | same |
| weakest roles (dev) | F1 of `next`, speaker, says, place, coref, agent | .23, .32, .38, .39, .42, .59 | 2026-10-10 | same, *Not yet* |
| course pupil (CPU) | test questions right, four options | 11 730 of 26 992 (43.5%) | 2026-10-15 | [../../docs/architecture.md](../../docs/architecture.md) |
| teacher's changes to prag labels | rows with a change | 30.6% | 2026-10-17 | §7.4 |

The frame bench is described in [../../docs/pragmatics.md](../../docs/pragmatics.md) (*The yardstick*): 240 lesson lines whose frames the strongest teacher wrote once; P/R/F1 per role; every reader step can be switched off for ablation (`PRAG_OFF`).

## 10. What is not there yet

- **No speaker meaning in the student.** `means` is written by the teacher but not learned; the student answers from frames and their links.
- **Acts are labelled, not reasoned.** The indirect-request reasoning with preconditions exists only in the dialogue layer ([../../world/src/talk.rs](../../world/src/talk.rs)), not in the sentence labeller (31).
- **No implicit discourse relations and no RST tree** (36).
- **Tense, aspect and modality** are not frame fields beyond `neg`, `cond` and `Fact(Can)`.
- **Prosody** is absent: Mova reads text only, so intonation cues to focus, irony and question bias are not available (29, 34).
- **Hedge and polarity agreement rules** are switched off until better witnesses exist (§8).

## In Mova

- Frames: [../../prag/src/frames.rs](../../prag/src/frames.rs); labels: [../../prag/src/schema.rs](../../prag/src/schema.rs), [../../prag/src/feats.rs](../../prag/src/feats.rs), [../../prag/src/model.rs](../../prag/src/model.rs), [../../prag/src/agree.rs](../../prag/src/agree.rs), [../../prag/src/silver.rs](../../prag/src/silver.rs); inference: [../../prag/src/infer.rs](../../prag/src/infer.rs), [../../prag/src/irony.rs](../../prag/src/irony.rs), [../../prag/src/rsa.rs](../../prag/src/rsa.rs), [../../prag/src/lens.rs](../../prag/src/lens.rs); learned rules: [../../prag/src/rules.rs](../../prag/src/rules.rs).
- Discourse: [../../coref/](../../coref/), [../../world/src/qud.rs](../../world/src/qud.rs), [../../world/src/talk.rs](../../world/src/talk.rs), [../../world/src/register.rs](../../world/src/register.rs).
- Docs: [../../docs/pragmatics.md](../../docs/pragmatics.md), [../../docs/context.md](../../docs/context.md).

## Sources

- Grice, H. P. 1975. Logic and Conversation. In Cole & Morgan (eds.), *Syntax and Semantics 3*. Not open — see *Wanted*.
- Davis, W. "Implicature". *Stanford Encyclopedia of Philosophy*. https://plato.stanford.edu/entries/implicature/
- Green, M. "Speech Acts". *Stanford Encyclopedia of Philosophy*. https://plato.stanford.edu/entries/speech-acts/
- Korta, K., Perry, J. "Pragmatics". *Stanford Encyclopedia of Philosophy*. https://plato.stanford.edu/entries/pragmatics/
- Levinson, S. C. 2024. *The Dark Matter of Pragmatics: Known Unknowns*. Cambridge University Press (Elements in Pragmatics). `books/l/le/levinson-2024-dark-matter-pragmatics-known` (card)
- Clark, H. H., Schaefer, E. F. 1989. Contributing to Discourse. *Cognitive Science* 13. `papers/c/cl/clark-1989-contributing-discourse`
- Clark, H. H., Brennan, S. E. 1991. Grounding in Communication. `papers/c/cl/clark-1991-grounding-communication`
- Roberts, C. 1996/2012. Information structure in discourse: Towards an integrated formal theory of pragmatics. *Semantics and Pragmatics* 5. https://doi.org/10.3765/sp.5.6
- Daelemans, W., van den Bosch, A., Weijters, T. 1997. IGTree: Using Trees for Compression and Classification in Lazy Learning Algorithms. *Artificial Intelligence Review* 11.
- Aha, D. W., Kibler, D., Albert, M. K. 1991. Instance-based learning algorithms. *Machine Learning* 6. Repo summary: [../../papers/aha-1991-instance-based-learning-algorithms.md](https://research.elamur.ai/free/en/papers/aha-1991-instance-based-learning-algorithms/)
- Mova's parse database: `$MOVA_DATA/parsedb/prag.tsv`, counted 2026-10-17; book titles from the tales registry (`$MOVA_DATA/corpora/tales/free/text/books.tsv`).
- Repo docs listed under *In Mova*.
