# 34. Irony and figurative language

English speakers often mean something other than, or more than, the literal content of their words: *Great, another Monday* means the opposite, *I've told you a million times* exaggerates, *Do you think I can listen all day to such stuff?* is not a question, and *kicked the bucket* is about dying, not kicking. This chapter describes verbal irony and sarcasm, hyperbole and understatement, rhetorical questions, metaphor and metonymy, idioms and humour — the theories, the English forms, the computational work, and what Mova implements: a sarcasm detector by incongruity, a lens over idioms, and a humour trigger from the absurdity matrix.

Contents

1. [Figurative meaning: an overview](#1-figurative-meaning-an-overview)
2. [Verbal irony and sarcasm](#2-verbal-irony-and-sarcasm)
3. [Theories of irony](#3-theories-of-irony)
4. [Cues of irony in English](#4-cues-of-irony-in-english)
5. [Hyperbole and understatement](#5-hyperbole-and-understatement)
6. [Rhetorical questions](#6-rhetorical-questions)
7. [Metaphor and metonymy](#7-metaphor-and-metonymy)
8. [Idioms and the expression lens](#8-idioms-and-the-expression-lens)
9. [Computational irony and sarcasm detection](#9-computational-irony-and-sarcasm-detection)
10. [Mova's irony detector](#10-movas-irony-detector)
11. [Humour and the absurdity trigger](#11-humour-and-the-absurdity-trigger)
12. [Figurative language in the tales](#12-figurative-language-in-the-tales)

---

## 1. Figurative meaning: an overview

| figure | what departs from the literal | English example | maxim flouted (33) | chapter section |
|---|---|---|---|---|
| verbal irony | the attitude: the speaker dissociates from what is said | *Lovely weather!* in a storm | Quality | §2–4 |
| sarcasm | irony aimed at a target, usually critical | *Oh, you're a genius.* | Quality | §2 |
| hyperbole | the degree: more than is meant | *I've told you a million times.* | Quality | §5 |
| understatement (meiosis), litotes | the degree: less than is meant | *It's a bit chilly* (blizzard); *not bad* | Quantity / Manner | §5; 33 §12 |
| rhetorical question | the act: an assertion in question form | *Who cares?* | Quality (questions) | §6 |
| metaphor | the concept: one domain described in terms of another | *Time is running out.* | Quality | §7 |
| metonymy | the referent: something associated stands for it | *The White House said…* | — | §7 |
| idiom | the whole phrase has a conventional meaning | *spill the beans* | — (conventional) | §8 |
| parody, echo | the voice: someone else's words, mocked | Alice's *How doth the little crocodile* | Quality | §12 |

Figures differ in how conventional they are. Idioms and dead metaphors are stored with their meaning; creative metaphor, irony and hyperbole are inferred in context. A system can look up the first and must reason about the second — the division of labour in Mova between the lens (§8) and the detectors (§10–11).

## 2. Verbal irony and sarcasm

**Verbal irony** is saying something while showing that one does not endorse it, typically meaning something opposite or critical. **Sarcasm** is usually treated as a sub-type: irony with a victim, critical or mocking in intent. The two terms are used loosely in everyday English and in annotated corpora (hashtags *#irony* and *#sarcasm* label overlapping material; Van Hee, Lefever & Hoste 2018, §9).

| type | example | what is meant |
|---|---|---|
| ironic praise (blame by praise) | *Great, my tooth hurts.* | it is bad |
| ironic blame (praise by blame) | *Oh, you terrible person, you brought me flowers.* | it is good (rarer) |
| ironic understatement | *Well, that went slightly wrong* (after a disaster) | it went very wrong |
| ironic question | *Could you be any slower?* | you are too slow |
| ironic agreement | *Yeah right.* | no |
| situational irony | a fire station burns down | (not a speech act; an ironic situation) |

Riloff et al. (2013, "Sarcasm as Contrast between a Positive Sentiment and Negative Situation", EMNLP, https://aclanthology.org/D13-1066, `papers/r/ri/ril/riloff-2013-sarcasm-as-contrast-between-positive`) identified one very frequent English pattern on Twitter: a **positive sentiment** expression followed by a **negative situation** — *I love waiting in line for hours*, *Oh how I love being ignored* — and bootstrapped lists of both from tweets. This pattern is the core of Mova's detector (§10).

## 3. Theories of irony

| theory | idea | what it predicts | source |
|---|---|---|---|
| Gricean | irony flouts Quality; the hearer infers the opposite | meaning = the opposite of what is said | Grice 1975 (*Wanted*); Davis, *SEP*, https://plato.stanford.edu/entries/implicature/ |
| echoic mention | the speaker echoes a thought or utterance (someone's, or a norm) and expresses a dissociative attitude to it | explains why irony is not always "the opposite", and why it needs a voice to echo | Sperber & Wilson 1981; Wilson & Sperber 2012 (*Wanted*) |
| pretence | the speaker pretends to be an injudicious person speaking to an uninitiated audience; the real audience sees through it | explains the "ironic tone" and the role of a target | Clark & Gerrig 1984, "On the pretense theory of irony" (*Wanted*) |
| allusional pretence, implicit display | irony alludes to a failed expectation and violates a pragmatic principle | unifies echo and pretence | Kumon-Nakamura, Glucksberg & Brown 1995 (*Wanted*) |
| incongruity (computational) | a contrast between polarities, or between expectation and reality, signals irony | the detectable signature in text | Riloff et al. 2013; Joshi et al. 2015 |

Rhetoric has discussed irony since antiquity. Aristotle notes that meeting someone's earnestness with irony provokes anger, "for irony implies disdain", and ranks irony above buffoonery because the ironical man jokes for his own amusement rather than to amuse others (*Rhetoric* II.2 and III.18, transl. Jebb 1909, `books/a/ar/aristotle-1909-rhetoric-transl-r-c`, public domain). An eighteenth-century defence of ridicule and irony as legitimate means of argument is Collins (1729, *A Discourse Concerning Ridicule and Irony in Writing*, `books/c/co/collins-1729-discourse-concerning-ridicule-irony`, card only in the library).

The echoic view explains a case the Gricean view handles badly. When Alice recites *How doth the little crocodile / Improve his shining tail…* (*Alice's Adventures in Wonderland*, ch. 2) the poem echoes Isaac Watts's moral verse *How doth the little busy bee*, and the irony lies in the echo — a predator praised in the words of a hymn to industry — not in any single false proposition. The parse database labels the second stanza (`book:pg11:108`, *How cheerfully he seems to grin, … With gently smiling jaws!*) as `excl`, `evaluate`, polarity `neg`, voice `character`: the teacher read the evaluation as negative behind the positive words.

## 4. Cues of irony in English

Irony is inferred from context, but English has cues that raise its likelihood (Kreuz & Caucci 2007, "Lexical Influences on the Perception of Sarcasm", https://aclanthology.org/W07-0101, `papers/k/kr/kre/kreuz-2007-lexical-influences-perception-sarcasm`, found interjections and adjectives/adverbs to increase readers' perception of sarcasm in literary excerpts):

| cue | example | in Mova's detector |
|---|---|---|
| positive word about a negative situation | *I love being ignored.* | explicit + implicit incongruity |
| interjection + positive evaluation | *Oh great*, *Wow, brilliant* | phrase *oh great* |
| fixed ironic phrases | *Yeah right*, *Just what I needed*, *Thanks a lot* | *yeah right*, *just what I needed* |
| exaggerated intensifiers | *Sooo much fun*, *absolutely thrilled* | — |
| punctuation | *!!*, *?!*, quotation marks around a word (*"help"*) | *!!*, *?!* |
| capitals | *GREAT job* | ALL CAPS word of ≥ 3 letters |
| emoticons, emoji, laughter | *:)*, *🙄*, *lol* | emoticons, laughter |
| hashtags | *#not*, *#sarcasm* (on Twitter) | — |
| prosody | slow, flat or exaggerated intonation; "ironic tone of voice" | — (text only) |
| incongruity with known facts | *What a sunny day* during rain | only lexical (situation list) |

None of these is decisive: *lol that was fun* is usually sincere, and *!!* marks emphasis more often than irony. Mova's detector therefore never fires on cues alone (§10).

## 5. Hyperbole and understatement

**Hyperbole** overstates for emphasis or evaluation: *a million times*, *I'm starving*, *it weighs a ton*, *the best day ever*, *forever*. It is extremely common in conversation and children's tales, where it is often conventional (*starving* for hungry). Its UD analysis is the ordinary one — the numbers are `nummod`, the degree words `advmod` — and its pragmatics is a flouting of Quality toward the upper end of a scale (33 §4).

**Understatement** says less: *it's a bit chilly*, *I was not entirely pleased*. Its negative variant, **litotes** (*not bad*, *not unknown*), is discussed with negation and implicature in 27 §11 and 33 §12. Understatement is a frequent carrier of irony (*a slight problem* about a disaster).

| device | scale effect | example | Mova |
|---|---|---|---|
| hyperbole | above the true value | *I've told you a million times.* | not detected |
| understatement | below the true value | *It's a bit chilly.* (blizzard) | not detected; *a bit* is a down-toner (32 §7) |
| litotes | negated opposite | *not bad*, *not uncommon* | the tree shows the negation; no inference |

Mova does not detect hyperbole or understatement; both require knowing the true value (a world model of temperatures, frequencies, weights), which `global` has only in part (measures and the absurdity matrix).

## 6. Rhetorical questions

A rhetorical question has the form of a question but the force of an assertion, often the assertion of the opposite polarity: *Who cares?* (= nobody cares), *Do you think I can listen all day to such stuff?* (= I cannot). Oraby, Harrison, Misra, Riloff & Walker (2017, "Are you serious?: Rhetorical Questions and Sarcasm in Social Media Dialog", SIGDIAL, https://aclanthology.org/W17-5537, `papers/o/or/oraby-2017-are-you-serious-rhetorical-questions`) built a corpus of 10 270 rhetorical questions from debate forums and Twitter, distinguished them from sincere questions at 0.76 F1, and found that rhetorical questions are used both sarcastically and not — the non-sarcastic ones are mostly argumentative in forums and persuasive in tweets.

| type | example | implied assertion |
|---|---|---|
| negative polarity | *Who cares?*; *What's the use?* | nobody cares; it is no use |
| positive polarity (from negative form) | *Isn't it obvious?* | it is obvious |
| exasperated | *Do you think I can listen all day to such stuff?* (`book:pg11:348`) | I cannot |
| answered by the speaker | *And why? Because…* | discourse structuring (36) |

The parse database labels `book:pg11:348` as `q`, `request`, not indirect, voice `character`: the act chosen is the one behind the assertion (stop talking), which is what the speaker wants. The form/act table (30 §7.3) shows 285 `q` sentences labelled `assert` and 298 `evaluate` — the place where most rhetorical questions end up. Mova's Quality rule for questions (33 §5) still says "the speaker does not know whether «you think I can listen all day to such stuff»", which is exactly what a rhetorical question does not mean: the rule has no rhetorical-question exception.

Rhetorical and other biased questions are treated syntactically in 26; the experimental work on question bias is collected in Trinh, Benz, Goodhue, Yatsushiro & Krifka (eds.) 2025, `books/k/kr/krifka-2025-biased-questions-experimental-results`.

## 7. Metaphor and metonymy

### 7.1 Metaphor

A metaphor describes one domain in terms of another: *time is running out*, *a heated debate*, *she shot down my argument*. Conceptual metaphor theory (Lakoff & Johnson 1980, *Metaphors We Live By* — *Wanted*) holds that such expressions reflect systematic mappings (ARGUMENT IS WAR, TIME IS A RESOURCE); corpus work identifies metaphor word by word. The VU Amsterdam Metaphor Corpus, annotated with the MIPVU procedure, underlies the 2018 and 2020 metaphor detection shared tasks (Leong et al. 2020, "A Report on the 2020 VUA and TOEFL Metaphor Detection Shared Task", https://aclanthology.org/2020.figlang-1.3, `papers/l/le/leo/leong-2020-report-2020-vua-toefl-metaphor`).

| kind | example | note |
|---|---|---|
| conventional (dead) | *grasp an idea*, *the leg of a table* | stored as a word sense |
| creative | *Juliet is the sun* | inferred in context |
| verb metaphor | *prices soared*, *time flies* | the verb's selectional restrictions are violated |
| noun metaphor | *a mountain of paperwork* | `nmod` of a measure-like noun |
| adjective metaphor | *a bright student*, *a cold reception* | `amod` |

The syntactic signature of verb metaphor — a subject or object that the verb does not normally take — is exactly what Mova's absurdity matrix scores: how plausible a noun is as the doer or object of a verb ([../../docs/knowledge.md](../../docs/knowledge.md)). A high absurdity score that survives the genre (§11) is evidence for a non-literal reading, metaphorical or humorous; the matrix does not yet tell the two apart.

### 7.2 Metonymy

Metonymy refers to something by an associated entity: *The White House said* (the administration), *read Dickens* (his works), *the kettle is boiling* (the water), *a new face* (a person). Markert & Nissim's SemEval-2007 task on metonymy resolution for location and organisation names (https://aclanthology.org/S07-1007, `papers/m/ma/mar/mark/markert-2007-semeval-2007-task-08-metonymy`) made the most common English patterns — place for people, place for event, organisation for its members — a classification task. In UD the metonymic noun is analysed as what it is syntactically (*White House* `nsubj` of *said*); the shift is a reference problem (35). Mova's entities use head lemmas and do not resolve metonymy; VerbNet's animacy restrictions (*say* wants an animate Agent) are where it could be detected ([../../prag/src/vn.rs](../../prag/src/vn.rs)).

## 8. Idioms and the expression lens

Idioms are multiword expressions whose meaning is not composed from their parts: *kick the bucket* (die), *spill the beans* (reveal a secret), *give up* (stop trying). They vary in flexibility — *spill the beans* passivises (*the beans were spilled*), *kick the bucket* does not — and many have register labels (euphemistic, humorous, slang, archaic) (37). In UD, idioms are analysed by their literal syntax (*bucket* is `obj` of *kicked*); only grammaticalised multiword function words are `fixed`, and phrasal-verb particles are `compound:prt` (21 §6.2; 22 §7.2).

Mova reads idioms through the **pragmatics lens** ([../../prag/src/lens.rs](../../prag/src/lens.rs); [../../docs/pragmatics.md](../../docs/pragmatics.md)): an expression base of idioms, proverbs, phrases and verb phrases with their real meanings and register labels is matched on lemmas and forms, longest first, and the matches give "what was meant" and switch on domains (slang, formal, humour, hidden meaning). The expression base is closed: it is kept out of the free release, so the lens is empty in a free checkout.

(1) The example of [../../docs/pragmatics.md](../../docs/pragmatics.md): *Grandpa kicked the bucket, the poor old geezer.* → *kicked the bucket* = to die (euphemistic, humorous); *geezer* (informal, derogatory) → domains hidden-meaning, humour, slang.

Two lessons from running it (`prag lens`, «Mova 0.4, CPU, 2026-10-17», with the closed base available): matching finds the lemma-based idiom *kicked the bucket* but does not always find form-based ones, and a word's labelled senses can mislead — *gave up smoking* was read with the "surrender; inform on" sense of *give up*, and *party* in a birthday sentence with a legal sense. The lens was tried as a feature of the act classifier and gave no clear gain (cross-validated macro-F1 for act 33.3 → 35.7, within noise; 2026-10-02), so it feeds "what was meant" and the domain switch instead ([../../docs/pragmatics.md](../../docs/pragmatics.md)).

## 9. Computational irony and sarcasm detection

| work | data | method | result |
|---|---|---|---|
| Riloff et al. 2013, https://aclanthology.org/D13-1066 | tweets | bootstrapped positive-sentiment phrases and negative-situation phrases; contrast of the two | the "positive about negative situation" pattern |
| Ptáček, Habernal & Hong 2014, "Sarcasm Detection on Czech and English Twitter", COLING, https://aclanthology.org/C14-1022 (`papers/p/pt/ptacek-2014-sarcasm-detection-czech-english-twitter`) | Czech and English tweets | supervised classifiers over n-grams, patterns, punctuation, emoticons | the first Czech sarcasm corpus; English with hashtag labels |
| Joshi, Sharma & Bhattacharyya 2015, "Harnessing Context Incongruity for Sarcasm Detection", ACL, https://aclanthology.org/P15-2124 (`papers/j/jo/jos/joshi-2015-harnessing-context-incongruity-sarcasm-detection`) | tweets, discussion forums | explicit incongruity (polarity flips, longest same-polarity run, counts) + implicit incongruity (Riloff's phrases) + lexical and pragmatic features | tweets F .842 (lexical) → .888 (all features) |
| Oraby et al. 2016, "Creating and Characterizing a Diverse Corpus of Sarcasm in Dialogue", SIGDIAL, https://aclanthology.org/W16-3604 (`papers/o/or/oraby-2016-creating-characterizing-diverse-corpus-sarcasm`) | Internet Argument Corpus | a corpus with three sarcasm types: general, rhetorical questions, hyperbole | the types differ in their lexical cues |
| Joshi 2017, "Detecting Sarcasm Using Different Forms of Incongruity", WASSA, https://aclanthology.org/W17-5201 (`papers/j/jo/jos/joshi-2017-detecting-sarcasm-different-forms-incongruity`) | — | survey of incongruity-based methods | — |
| Van Hee, Lefever & Hoste 2018, "SemEval-2018 Task 3: Irony Detection in English Tweets", https://aclanthology.org/S18-1005 (`papers/h/he/hee/hee-2018-semeval-2018-task-3-irony`) | 3 834 training and 784 test tweets, collected by *#irony*, *#sarcasm*, *#not*, hashtags removed, manually annotated | shared task: binary (A) and irony type (B) | best F1 0.71 (A), 0.51 (B); 43 and 31 teams |
| Cignarella, Sanguinetti, Bosco & Rosso 2020, "Marking Irony Activators in a Universal Dependencies Treebank", LREC, https://aclanthology.org/2020.lrec-1.627 (`papers/c/ci/cignarella-2020-marking-irony-activators-universal-dependencies`) | TWITTIRÒ-UD (Italian tweets in UD) | annotates the tokens that activate irony, with their morpho-syntactic features | a syntax-aware layer for irony |

The SemEval numbers set the bar: even with large training sets and neural models, binary irony detection in tweets reached F1 0.71 in 2018. Cignarella et al.'s idea of marking **irony activators** on a UD tree is the closest to Mova's approach, where the incongruent pair of tree nodes is the explanation (§10).

## 10. Mova's irony detector

### 10.1 What it does

[../../prag/src/irony.rs](../../prag/src/irony.rs) finds sarcasm and irony **candidates** by incongruity, deterministically and with an explanation; it follows Joshi, Sharma & Bhattacharyya (2015) for the features and Riloff et al. (2013) for negative situations ([../../docs/pragmatics.md](../../docs/pragmatics.md)).

| component | content (as in the code, 2026-10-17) | score |
|---|---|---|
| positive lexicon `POSITIVE` | 65 content-word lemmas (*love, great, wonderful, perfect, lucky, adorable*…) | — |
| negative lexicon `NEGATIVE` | 69 lemmas (*hate, ignore, broken, late, waste, insult*…); a negated polar word flips its sign | — |
| explicit incongruity | at least one polarity flip, and a positive word governs or precedes a negative one, with no contrastive connector between them (`CONTRAST`: *but, although, though, however, yet*) | +2 |
| implicit incongruity | a positive verb (`POSITIVE_VERBS`: *love, enjoy, adore, like, appreciate*, and *can't wait*) whose dependency subtree after it holds a negative situation | +3 |
| negative situations `SITUATIONS` | 45 seed lemma sequences (*wait in line, be on hold, get ignored, work late, traffic, homework, monday, rain, delay, toothache, taxes, meeting, dentist, deadline, flat tire, jury duty, alarm clock*…); a situation that is ended or denied (`ENDED`: *stop, end, finish, cease, over, no, without*) does not count | — |
| pragmatic cues | emoticons (*:) ;) :D :P* and five emoji), laughter (*lol, haha, hehe, lmao, rofl*), *!!*, *?!*, an ALL-CAPS word of three or more letters, *yeah right*, *oh great*, *just what I needed* | +1 per kind |
| threshold `THRESHOLD` | a candidate needs at least one incongruity and a score ≥ 2 | — |
| explanation | the incongruent pair (positive part, negative part), the counts (positive, negative, flips, longest same-polarity run, overall polarity), the features that fired | — |

The design keeps cues from deciding alone: *lol that was fun* is not irony, cues are "only emphasis".

### 10.2 Examples

(2) «Mova 0.4, CPU, 2026-10-17», `prag infer`:

```text
I love waiting in line for hours.
  irony candidate (score 5) [implicit incongruity, explicit incongruity]: «love» about «waiting in line» —
  the positive verb «love» takes the negative situation «waiting in line» as its complement (implicit
  incongruity, +3); the positive «love» governs the negative «waiting in line» (1 positive, 1 negative,
  1 flip(s), longest run 1; explicit incongruity, +2) — score 5 ≥ 2
Great, my tooth hurts!!
  irony candidate (score 3) [explicit incongruity, !!]: «Great» about «hurts» — … pragmatic cues: !! (+1)
Oh great, my flight is delayed again.
  irony candidate (score 3) [explicit incongruity, oh great]: «great» about «delayed» …
I can't wait to do my taxes.   (in one sentence with «What a wonderful day, it rained all morning :)»)
  irony candidate (score 6) [implicit incongruity, explicit incongruity, emoticon]: «can't wait» about «taxes» …
```

(3) Negative controls, same run — no candidate:

| sentence | why not |
|---|---|
| *I love my mother.* | no negative word or situation |
| *I don't like being ignored.* | the positive verb is negated: no flip toward irony |
| *The rain stopped and I love it.* | the situation *rain* is ended by *stopped* |
| *lol that was fun.* | cues alone |
| *Yeah right, like that will ever work.* | a cue (*yeah right*) without incongruity — a real sarcastic sentence missed |
| *The food was good but the service was bad.* | *but* between the pair: a sincere mixed opinion |

(4) A false positive, same run: *The meeting was great, but the coffee was awful.* → candidate (score 2), «great» about «meeting». *Meeting* is in the seed list of negative situations, and the pair *great … meeting* stands before the *but*, so the contrastive-connector rule does not apply. A seed list of situations that are "usually bad" (meetings, Mondays, rain) is a stereotype; Riloff et al. learned theirs from tweets where the pattern actually signalled sarcasm.

### 10.3 On a tale

On 400 sentences of *Alice's Adventures in Wonderland* (`book:pg11:*`, 2026-10-17; 33 §11) the detector gave 6 candidates, each at the minimum score 2 by explicit incongruity alone. Reading them, none is ironic: *to Alice's great surprise, the Duchess's voice died away* (*great* as a degree adjective, *die away* as fading), *this seemed to Alice a good opportunity for making her escape; so she … ran till she was quite tired* (*good* and *tired* in different clauses), *I think you might do something better with the time than waste it* (a sincere reproach). Narrative prose is full of positive and negative words side by side; the explicit-incongruity rule, designed for one-line tweets, has no notion of clause boundaries or of degree uses. Precision on this sample is therefore 0 of 6 — a measure of the domain gap, not of the tweet-style cases the rule was built for.

## 11. Humour and the absurdity trigger

Humour has no single linguistic marker. Incongruity theories — a joke sets up an expectation and resolves it unexpectedly — are the closest to something computable; failed humour, when the hearer does not get or does not accept the joke, is a regular part of interaction (Bell 2015, *We Are Not Amused: Failed Humor in Interaction*, De Gruyter Mouton, `books/b/be/bell-2015-we-are-not-amused`, card only).

Mova switches on a **humour/irony domain** from two kinds of evidence ([../../world/src/domains.rs](../../world/src/domains.rs); [../../docs/knowledge.md](../../docs/knowledge.md); [../../docs/pragmatics.md](../../docs/pragmatics.md)):

| evidence | rule | example |
|---|---|---|
| absurdity that survives the genre | a doer or object scored absurd (score ≥ 3) by the absurdity matrix, after the genre's domain layer (talking animals are normal in a tale) | *The fox said good morning to the judge.* |
| register clash | words of the children's or everyday register (log-odds z above a threshold) in a formal sentence: mock-formal humour | *the distribution of crayons* in legal prose |
| labelled senses | a main sense labelled humorous, jocular, ironic or sarcastic in the expression base | *kick the bucket* (humorous, euphemistic) |

(5) `world domains`, «Mova 0.4, CPU, 2026-10-17»:

```text
$ world domains "The fox said good morning to the judge."
domain humor:
  because absurd in the real domain: fox as the doer of said (score 3)

$ world domains "The fox said good morning to the judge." --genre tale
domains: - (literal reading)
```

The same sentence is humorous or odd in the real world and literal in a tale — the genre decides what counts as absurd. The absurdity matrix is the trigger, not the knowledge of humour: "absurdity is its trigger, not its knowledge" ([../../docs/knowledge.md](../../docs/knowledge.md)). The irony detector of §10 meets this trigger from the other side: absurdity catches odd doers and objects, incongruity catches praise of the bad; both switch on the hidden-meaning reading ([../../prag/src/irony.rs](../../prag/src/irony.rs), header).

Measured on contrast sets written by the teacher (dev 200 and test 200 sentences, 65 literal controls each; 2026-10-02), the humour domain is the weakest: precision 30.0% / recall 14.3% on dev, 28.9% / 17.7% on test, against 67.7% / 70.0% for the formal domain on test; literal sentences raised a false alarm in 21.5–23.1% of cases ([../../docs/pragmatics.md](../../docs/pragmatics.md)). Jokes rest on world knowledge and context that word statistics do not see.

## 12. Figurative language in the tales

The tales of Mova's corpus (public domain) are rich in figures, and the parse database's labels show where the teacher read beyond the literal:

| sent_id | text | labels | figure |
|---|---|---|---|
| `book:pg11:108` | *How cheerfully he seems to grin, How neatly spread his claws, And welcome little fishes in With gently smiling jaws!* | `excl evaluate neg` | parody, ironic praise |
| `book:pg11:348` | *Do you think I can listen all day to such stuff?* | `q request` | rhetorical question |
| `book:pg11:754` | *"Very true," said the Duchess: "flamingoes and mustard both bite.* | `decl assert` | absurd reasoning (pun on *bite*) |
| `book:pg11339:1145` | *"Would you kindly bring me some water from the stream close by?* | `q request indirect=1` | politeness as a cover (32 §3.3) |
| `book:pg11:201` | *"Hold your tongue, Ma!" said the young Crab, a little snappishly. "You're enough to try the patience of an oyster!"* | `imp request` | idiom (*hold your tongue*), comic hyperbole |

The Duchess's line is typical of *Alice*: the humour comes from treating a polysemous word (*bite* of mustard's taste and of a bird's beak) as one sense — an incongruity of meaning that neither the lexical irony detector nor the absurdity matrix (which scores roles, not senses) can see.

## In Mova

- Irony and sarcasm candidates: [../../prag/src/irony.rs](../../prag/src/irony.rs) (`POSITIVE`, `NEGATIVE`, `POSITIVE_VERBS`, `SITUATIONS`, `EMOTICONS`, `LAUGHTER`, `PHRASES`, `CONTRAST`, `ENDED`, `THRESHOLD`); printed by `prag infer`.
- Idioms and register through the lens: [../../prag/src/lens.rs](../../prag/src/lens.rs) (`prag lens`); the expression base is closed and not in the free release.
- Domains (humour, hidden meaning, slang, formal) and the absurdity trigger: [../../world/src/domains.rs](../../world/src/domains.rs) (`world domains`), [../../docs/knowledge.md](../../docs/knowledge.md), [../../docs/pragmatics.md](../../docs/pragmatics.md).
- The agreer runs an irony model as a witness and only reports it ([../../prag/src/agree.rs](../../prag/src/agree.rs)).
- **Not yet**:
  - the irony detector works on lemma lists with no clause boundaries or degree uses (§10.3) and is not used by the sentence labels or the student;
  - no hyperbole, understatement or rhetorical-question detection; the Quality rule for questions has no rhetorical exception (§6);
  - metaphor and metonymy are not detected; the absurdity matrix flags odd roles but does not separate metaphor from humour;
  - the negative-situation list is a hand-written seed, not learned from data;
  - no prosody: the "ironic tone" is out of reach in text.

## Sources

- Grice, H. P. 1975. Logic and Conversation. Not open — *Wanted*.
- Sperber, D., Wilson, D. 1981. Irony and the use–mention distinction; Wilson, D., Sperber, D. 2012. *Meaning and Relevance*. Not open — *Wanted*.
- Clark, H. H., Gerrig, R. J. 1984. On the pretense theory of irony. *Journal of Experimental Psychology: General* 113. Not open — *Wanted*.
- Kumon-Nakamura, S., Glucksberg, S., Brown, M. 1995. How about another piece of pie: The allusional pretense theory of discourse irony. Not open — *Wanted*.
- Lakoff, G., Johnson, M. 1980. *Metaphors We Live By*. Not open — *Wanted*.
- Davis, W. "Implicature". *Stanford Encyclopedia of Philosophy*. https://plato.stanford.edu/entries/implicature/
- Riloff, E., Qadir, A., Surve, P., De Silva, L., Gilbert, N., Huang, R. 2013. Sarcasm as Contrast between a Positive Sentiment and Negative Situation. EMNLP. https://aclanthology.org/D13-1066; `papers/r/ri/ril/riloff-2013-sarcasm-as-contrast-between-positive`
- Ptáček, T., Habernal, I., Hong, J. 2014. Sarcasm Detection on Czech and English Twitter. COLING. https://aclanthology.org/C14-1022; `papers/p/pt/ptacek-2014-sarcasm-detection-czech-english-twitter`
- Joshi, A., Sharma, V., Bhattacharyya, P. 2015. Harnessing Context Incongruity for Sarcasm Detection. ACL. https://aclanthology.org/P15-2124; `papers/j/jo/jos/joshi-2015-harnessing-context-incongruity-sarcasm-detection`
- Joshi, A. 2017. Detecting Sarcasm Using Different Forms of Incongruity. WASSA. https://aclanthology.org/W17-5201; `papers/j/jo/jos/joshi-2017-detecting-sarcasm-different-forms-incongruity`
- Oraby, S., Harrison, V., Reed, L., Hernandez, E., Riloff, E., Walker, M. 2016. Creating and Characterizing a Diverse Corpus of Sarcasm in Dialogue. SIGDIAL. https://aclanthology.org/W16-3604; `papers/o/or/oraby-2016-creating-characterizing-diverse-corpus-sarcasm`
- Oraby, S., Harrison, V., Misra, A., Riloff, E., Walker, M. 2017. Are you serious?: Rhetorical Questions and Sarcasm in Social Media Dialog. SIGDIAL. https://aclanthology.org/W17-5537; `papers/o/or/oraby-2017-are-you-serious-rhetorical-questions`
- Van Hee, C., Lefever, E., Hoste, V. 2018. SemEval-2018 Task 3: Irony Detection in English Tweets. https://aclanthology.org/S18-1005; `papers/h/he/hee/hee-2018-semeval-2018-task-3-irony`
- Cignarella, A. T., Sanguinetti, M., Bosco, C., Rosso, P. 2020. Marking Irony Activators in a Universal Dependencies Treebank: The Case of an Italian Twitter Corpus. LREC. https://aclanthology.org/2020.lrec-1.627; `papers/c/ci/cignarella-2020-marking-irony-activators-universal-dependencies`
- Kreuz, R., Caucci, G. 2007. Lexical Influences on the Perception of Sarcasm. https://aclanthology.org/W07-0101; `papers/k/kr/kre/kreuz-2007-lexical-influences-perception-sarcasm`
- Leong, C. W. et al. 2020. A Report on the 2020 VUA and TOEFL Metaphor Detection Shared Task. https://aclanthology.org/2020.figlang-1.3; `papers/l/le/leo/leong-2020-report-2020-vua-toefl-metaphor`
- Markert, K., Nissim, M. 2007. SemEval-2007 Task 08: Metonymy Resolution at SemEval-2007. https://aclanthology.org/S07-1007; `papers/m/ma/mar/mark/markert-2007-semeval-2007-task-08-metonymy`
- Bell, N. 2015. *We Are Not Amused: Failed Humor in Interaction*. De Gruyter Mouton. `books/b/be/bell-2015-we-are-not-amused` (card)
- Collins, A. 1729. *A Discourse Concerning Ridicule and Irony in Writing*. `books/c/co/collins-1729-discourse-concerning-ridicule-irony` (card)
- Aristotle. *The Rhetoric*, transl. R. C. Jebb, ed. J. E. Sandys. Cambridge University Press, 1909. `books/a/ar/aristotle-1909-rhetoric-transl-r-c`
- Trinh, T., Benz, A., Goodhue, D., Yatsushiro, K., Krifka, M. (eds.) 2025. *Biased questions*. Language Science Press. `books/k/kr/krifka-2025-biased-questions-experimental-results`
- Mova's parse database (`$MOVA_DATA/parsedb/current.conllu`, `prag.tsv`): *Alice's Adventures in Wonderland* (`pg11`), *Aesop's Fables* (`pg11339`); runs of `prag infer`, `prag lens` and `world domains` with the release 0.4 model, 2026-10-17.
- Repo: [../../docs/pragmatics.md](../../docs/pragmatics.md), [../../docs/knowledge.md](../../docs/knowledge.md).
