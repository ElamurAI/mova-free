# 32. Politeness and hedging

Speakers rarely say exactly what they want with full force. They soften requests, cushion criticism, show deference or, at times, deliberately attack the hearer's face; and they adjust how strongly they commit to what they assert. This chapter describes politeness and impoliteness in English grammar and vocabulary, the recruitment of others to act, hedging and boosting, and epistemic stance and evidentiality — with the UD analysis of the forms involved, counts from Mova's parse database, and what Mova's code detects.

Contents

1. [Face and politeness: the frameworks](#1-face-and-politeness-the-frameworks)
2. [Politeness in English grammar](#2-politeness-in-english-grammar)
3. [Requests and recruitments](#3-requests-and-recruitments)
4. [Address: vocatives, titles, *thou* and *you*](#4-address-vocatives-titles-thou-and-you)
5. [Impoliteness](#5-impoliteness)
6. [Computational politeness](#6-computational-politeness)
7. [Hedging and boosting](#7-hedging-and-boosting)
8. [Epistemic modality and evidentiality](#8-epistemic-modality-and-evidentiality)
9. [The `hedge` label in the parse database](#9-the-hedge-label-in-the-parse-database)
10. [Computational hedge detection](#10-computational-hedge-detection)
11. [Politeness across cultures and registers](#11-politeness-across-cultures-and-registers)

---

## 1. Face and politeness: the frameworks

### 1.1 Face

The dominant account of linguistic politeness is Brown & Levinson's (1978/1987, *Politeness: Some Universals in Language Usage* — not open; see *Wanted* in [sources.md](sources.md#syntax-pragmatics)). It builds on Goffman's notion of **face**, "people's public self-image", and on Grice and Searle; the theory began from parallel strategies in three unrelated languages, Tamil, Tzeltal and English. This summary of its genealogy follows Kendrick (2020, "Recruitment in English: A quantitative study", in Floyd, Rossi & Enfield (eds.), *Getting others to do things: A pragmatic typology of recruitments*, Language Science Press, `books/e/en/enfield-2020-getting-others-things-pragmatic`, §1).

| notion | meaning | English example |
|---|---|---|
| positive face | the wish to be liked and approved of | praise, agreement, in-group address (*mate*, *dear*) |
| negative face | the wish to be unimpeded | apologies for imposing, indirectness, *if you don't mind* |
| face-threatening act (FTA) | an act that threatens either face | requests, orders, criticism, disagreement, refusal |
| weight of an FTA | social distance + power of the hearer + ranking of the imposition | *Could I possibly borrow your car?* vs *Pass the salt.* |

### 1.2 The strategies

Brown & Levinson order the speaker's choices by how much they redress the threat:

| strategy | idea | English realisation |
|---|---|---|
| bald on record | no redress | *Hold your tongue!* (`book:pg11:1119`) |
| positive politeness | attend to the hearer's wants, claim common ground | *Nice work, mate — could you…*; *Let's…*; jokes, compliments |
| negative politeness | minimise the imposition, give options, be indirect | *Could you possibly…*; *I was wondering if…*; *just*, *a bit*; apologies |
| off record | hint, leave the act deniable | *It's cold in here* (→ shut the window) |
| don't do the FTA | stay silent | — |

### 1.3 Other frameworks

- **Lakoff** (1973, "The logic of politeness", CLS 9) — rules "don't impose, give options, make the hearer feel good". Not open; *Wanted*.
- **Leech** (1983, *Principles of Pragmatics*; 2014, *The Pragmatics of Politeness*) — a Politeness Principle with maxims of tact, generosity, approbation, modesty, agreement, sympathy, parallel to Grice's Cooperative Principle (33). Not open; *Wanted*.
- **Discursive and relational approaches** treat politeness as evaluated by participants in context rather than computed from forms; a survey of the corpus side is Landert, Dayter, Messerli & Locher (2023, *Corpus Pragmatics*, Cambridge Elements, `books/l/lo/locher-2023-corpus-pragmatics`, card only).

Mova's position is pragmatic in the everyday sense: it detects the **forms** that the literature associates with politeness (§6) and records the act they serve (31), but does not claim to compute how polite an utterance *is* for its participants.

## 2. Politeness in English grammar

Politeness in English is carried mostly by grammar that softens: past tense and conditional modals, interrogative form, embedding under verbs of wishing and wondering, negation, minimisers, and the particle *please*.

| device | example | UD analysis | what it does |
|---|---|---|---|
| modal past / conditional | *Could you…*, *Would you…* | `aux(VERB, could)` | distances the request from the present and real (counterfactual) |
| interrogative form | *Can you open the window?* | `aux` before `nsubj`, `?` | gives the hearer the option to refuse (31) |
| negative question | *Can you not protect me, then?* (`book:pg11027:1158`) | `advmod(protect, not)` | pleading, or pressure; often expects *yes* |
| embedding | *I was wondering if you could give me some references…* (EWT 2.18 `email-enronsent18_01-0015`) | `ccomp(wondering, give)`, `mark(give, if)` | turns a request into a report of the speaker's thoughts |
| past progressive of the frame | *I was wondering*, *I was hoping* | `aux(wondering, was)` | distances in time and aspect |
| *please* | *Could you please let us know…* (EWT 2.18 `email-enronsent22_01-0038`) | `discourse(let, please)`, INTJ | marks a request as such; politeness marker |
| *if you please*, *kindly* | *Would you kindly bring me some water…?* (`book:pg11339:1145`) | `advmod(bring, kindly)` | archaic or formal softening |
| minimisers | *just*, *a bit*, *a little*, *a moment* | `advmod`; `obl:unmarked` | shrinks the imposition: *Just wanted to confirm our meeting* (EWT 2.18 `email-enronsent20_02-0014`) |
| conditional clauses | *if you don't mind*, *if it's not too much trouble* | `advcl` + `mark(if)` | conditions the act on the hearer's consent |
| apology | *Sorry to bother you, but…* | `xcomp`/`advcl` | pays for the imposition in advance |
| impersonalisation | *Passengers are requested to…*; *It would be appreciated if…* | passive, `expl` + `csubj` (28, 21 §5.3) | removes speaker and hearer from the act |
| plural *we* | *Shall we start?* | `nsubj(start, we)` | inclusive: the hearer as partner |
| tag questions | *Pass me that, would you?* | `parataxis` (26) | softens an imperative into a request |

(1) EWT 2.18 `email-enronsent22_01-0038` (ID FORM UPOS HEAD DEPREL):

```conllu
1  Could   AUX    4  aux
2  you     PRON   4  nsubj
3  please  INTJ   4  discourse
4  let     VERB   0  root
5  us      PRON   4  obj
6  know    VERB   4  xcomp
```

The request is the root's lexical verb (*let … know*); *could*, the inversion and *please* are all dependents. The form is a yes/no question (prag `form=q`), the act a request (`act=request`, `indirect=1`) — the configuration that the agreer and the teacher label as indirect (30 §7.3: 497 `q` sentences are requests).

(2) EWT 2.18 `email-enronsent18_01-0015` — the request embedded twice, under *wondering* and in an *if*-clause with *could*:

```conllu
1  I          PRON   3  nsubj
2  was        AUX    3  aux
3  wondering  VERB   0  root
4  if         SCONJ  7  mark
5  you        PRON   7  nsubj
6  could      AUX    7  aux
7  give       VERB   3  ccomp
8  me         PRON   7  iobj
```

Stoffel's study of English "down-toners" already describes the apologetic function of degree adverbs: *rather* "is frequently employed to take off the edge of what might otherwise produce an unpleasant impression on the hearer" (*You are rather late*), and *slightly, somewhat, pretty, a little, a bit, a trifle* do the same with a weaker "apologetic note" (Stoffel 1901, *Intensives and Down-toners: A Study in English Adverbs*, Part II ch. I, `books/s/st/stoffel-1894-intensives-down-toners-study`, public domain). The same words are hedges when they weaken a claim (§7); the difference lies in the act they modify.

## 3. Requests and recruitments

### 3.1 From speech act to recruitment

Requests are the central face-threatening act. Speech act theory gives them felicity conditions — the act is a future act of the hearer, the speaker believes the hearer can do it (Searle 1969; 31). Psycholinguists added that a request is formulated to address "the greatest potential obstacle" the speaker can anticipate: inability (*Can you tell me what time it is?*) or the availability of an object (*Do you happen to have a watch?*) (Gibbs 1985; Francik & Clark 1985; summarised in Kendrick 2020, §1).

Conversation analysts widened the object of study to **recruitment**: "a basic cooperative phenomenon in interaction consisting of a sequence of two moves" — one participant's move that leads another to do something practical, and the other's response — including offers occasioned by someone's trouble and requests made by gesture alone (Floyd, Rossi & Enfield 2020, ch. 1, §4, `books/e/en/enfield-2020-getting-others-things-pragmatic`, Language Science Press, CC BY 4.0).

### 3.2 English recruitments in numbers

Kendrick's chapter on English (in Floyd, Rossi & Enfield 2020) gives a quantitative picture from video-recorded everyday interaction:

| finding | value |
|---|---|
| recruitment sequences with a single recruiting move | 65.9% (n = 139) |
| outcome: a service (a practical action for the recruiter) | 49.8% (n = 105) |
| outcome: transfer of an object | 38.8% (n = 82) |
| visible bodily actions in recruiting moves that involve the hands | 64.9% (n = 63) |
| imperatives that are simple verbal predicates | 83.6% (n = 61) |
| imperatives with *don't* | 8.2% (n = 6) |
| imperatives with *let me* / *let's* | 8.2% (n = 6) |

Kendrick argues that *can/could you* and *will/would you* are not interchangeable: *will/would you* indexes greater contingency, and *can/could you* is chosen when the speaker finds fault with the recipient's inaction; imperatives are used when the contingency of asking is unnecessary — inside an agreed joint activity, or when the situation calls for immediate action (Kendrick 2020, §4.2). So the "most polite" form is not always the most indirect one: politeness is fitted to the sequence and the relationship.

### 3.3 Requests in Mova's data

The agreed pragmatic labels show how English realises requests (counts of 2026-10-17 over `$MOVA_DATA/parsedb/prag.tsv`, 30 §7.3):

| form of a `request` | rows | example |
|---|---:|---|
| imperative (`imp`) | 10 833 | *"Off with her head!" the Queen shouted* (`book:pg11:1121`) |
| declarative (`decl`) | 1 906 | *"Oh, if you please," said the Queen, "I want a baby…"* (`book:pg23661:1050`, `indirect=1`) |
| yes/no question (`q`) | 497 | *"Would you kindly bring me some water from the stream close by?"* (`book:pg11339:1145`, `indirect=1`) |
| fragment (`frag`) | 215 | *Off with his whiskers!* (`book:pg11:1046`) |

The Aesop example is instructive: in the fable the Wolf asks the Sheep politely for water, and the Sheep answers that if it brought the water the Wolf would make a meal of it — the politeness is a cover. Mova's labels capture the form and the act (`q`, `request`, `indirect=1`), not the deception; that lies in the story's world (`world`) and its frames.

In the dialogue layer, Mova reads *Can/Could/Would you (please) X?* as a request only when its preconditions hold — the speaker wants X (a modal question), the hearer is capable (X is one of its commands), willing now and obligated — and otherwise answers the literal ability question; the reply says which reading it took and why ([../../world/src/talk.rs](../../world/src/talk.rs) `indirect_request`, after Sarathy et al. 2020 and Brown 1980; corpus: Roque, Tsuetaki, Sarathy & Scheutz 2020, "Developing a Corpus of Indirect Speech Act Schemas", https://aclanthology.org/2020.lrec-1.28, `papers/r/ro/roq/roque-2020-developing-corpus-indirect-speech-act`). See 31 for indirect speech acts in general.

## 4. Address: vocatives, titles, *thou* and *you*

### 4.1 Vocatives

Address terms are `vocative` in UD (21 §9): *Take care, my friend, Linda* (EWT 2.18 `weblog-typepad.com_ripples_20040407125600_ENG_20040407_125600-0062`). They do politeness work — affection (*dear*, *my friend*), deference (*sir*, *my lord King*), insult (*you simpleton*) — and interactional work: securing the attention of a particular recipient before a request (Kendrick 2020, §4; 38).

| vocative | relation signalled | example |
|---|---|---|
| title alone | deference | *"Well, I should like to be a little larger, **sir**, if you wouldn't mind," said Alice* (`book:pg11:359`) |
| title + name | formal respect | *Mr. Brown*, *Dr. Smith* (22 §2.6) |
| kin term | intimacy or respect | *Ma*, *Grandmother* |
| endearment | positive face | *my dear*, *darling* |
| epithet | insult | *You stupid goose* (`book:pg11027:214`) |
| *my lord*, *thy servant* | ritual deference | §4.2 |

The Alice example combines four negative-politeness devices in one sentence: the conditional *should like*, the minimiser *a little*, the title *sir* and the conditional clause *if you wouldn't mind*.

### 4.2 *Thou* and *you*

Early Modern English kept the old singular *thou/thee/thy/thine* beside *you/ye*, and the choice could signal intimacy, condescension or address to God; Modern English generalised *you* (Abbott 1869, *A Shakespearian Grammar*, `books/a/ab/abbott-1869-shakespearian-grammar-attempt-illustrate`; the morphology chapters give the paradigm). The JPS 1917 Torah translation keeps the archaic forms throughout. Counts over its English trees (`$MOVA_DATA/corpora/judaism/parsedb/current.conllu`, 2026-10-17):

| form | tokens | | form | tokens |
|---|---:|---|---|---:|
| *thou* | 1 512 | | *you* | 559 |
| *thee* | 967 | | *your* | 522 |
| *thy* | 1 321 | | *yours* | 3 |
| *thine* | 115 | | | |
| *ye* | 879 | | | |

In this text *thou* is the number-neutral singular of address (to God, to a person) and *ye/you* the plural; the T/V politeness contrast of the Early Modern stage is not what the choice encodes here. Deference is expressed lexically and by formula instead: *I pray thee* (39 + 11 with *Thee*), *I pray you* (10), *pray thee* (4), *my lord* (37 + 5 *My lord*), *thy servant* (43 + 12 *Thy servant*), *I beseech* (1).

(3) `jps:GEN:637` *And the servant ran to meet her, and said: 'Give me to drink, I pray thee, a little water of thy pitcher.'* — the request is an imperative (`ccomp(said, Give)`), and the parenthetical *I pray thee* is `parataxis(Give, pray)` with *thee* as its `obj`. The pragmatic row: `imp request`, not indirect, voice `character`.

## 5. Impoliteness

Impoliteness is not just the absence of politeness: speakers have conventional means to attack face. Research on impoliteness (Culpeper and others) has catalogued insults, dismissals, threats, silencers and the like; *The Grammar of Impoliteness* (Van Olmen, Andersson, Culpeper & Giomi (eds.) 2025, De Gruyter, `books/g/gi/giomi-2025-grammar-impoliteness`, card only) collects work on the grammatical constructions that conventionally carry it. On contested readings — when a hearer and a speaker disagree about whether an utterance was offensive — see Bourgeois & Bousfield (2026, *Pragmatics in Contested Interpretation*, Springer, `books/b/bo/bousfield-2026-pragmatics-contested-interpretation`, card only).

| impolite format | example from the tales | UD | prag row |
|---|---|---|---|
| bald imperative of silencing | *"Hold your tongue!" said the Queen, turning purple.* (`book:pg11:1119`) | root imperative | `imp request` |
| *you* + epithet | *"You stupid goose," said she* (`book:pg11027:214`) | `vocative` or root NP | `decl disagree neg` |
| exclamative insult | *You stupid pale-faced creatures!* (`book:pg11027:1452`) | root NP + `!` | `excl evaluate neg` |
| verbless command | *Off with her head!* (`book:pg11:1121`) | fragment (21 §8) | `imp request` |
| sarcastic praise | *Great, another Monday.* | — | irony candidate (34) |

The *you* + epithet frame (*you idiot*, *you simpleton*) is a well-known conventionalised insult; its analysis as `vocative` or as an appositional root depends on whether it addresses or describes. A computational study of the same frame in literary dialogue is Holgate & Erk (2021, "'Politeness, you simpleton!' retorted [MASK]", https://aclanthology.org/2021.iwcs-1.19, `papers/h/ho/hol/holgate-2021-politeness-you-simpleton-retorted-mask`). The agreer of Mova's labels runs a politeness model as a witness but only reports it: one witness alone never changes a label ([../../prag/src/agree.rs](../../prag/src/agree.rs)).

## 6. Computational politeness

### 6.1 The Stanford politeness strategies

Danescu-Niculescu-Mizil, Sudhof, Jurafsky, Leskovec & Potts (2013, "A computational approach to politeness with application to social factors", ACL, https://aclanthology.org/P13-1025, `papers/m/mi/miz/mizil-2013-computational-approach-politeness-application-social`) annotated requests from Wikipedia talk pages and Stack Exchange for politeness and derived 20 strategies, detected by regular expressions over dependency parses and lexicons, each with its mean politeness score. An SVM over these features reached 83.8% on Wikipedia (human 86.7%) and 78.2% on Stack Exchange, 67.5–75.4% across domains (as summarised in [../../docs/pragmatics.md](../../docs/pragmatics.md); the data are CC BY-NC-SA, so only the strategy list is used, as facts).

### 6.2 Mova's detectors

[../../world/src/talk.rs](../../world/src/talk.rs) (`politeness`) implements 18 of the strategies as surface detectors with the published mean scores × 100 as integer weights; the score is something a member of the dialogue layer notices and mentions — a blunt request is still carried out ([../../docs/pragmatics.md](../../docs/pragmatics.md), *Done (2026-10-03)*):

| strategy | cue in `talk.rs` | weight |
|---|---|---:|
| gratitude | *thank*, *appreciate*, *grateful* | +87 |
| deference | *nice work*, *good job*, *great job*, *well done*, *good work* | +78 |
| indirect (*by the way*) | *by the way*, *btw* | +63 |
| *please* (not first) | *please* inside the sentence | +49 |
| counterfactual modal | *could you*, *would you* | +47 |
| greeting | first word *hi*, *hello*, *hey* | +43 |
| apologising | *sorry*, *apologize*, *excuse me* | +36 |
| hedges | *I suggest*, *I think*, *maybe*, *perhaps*, *I guess* | +14 |
| positive lexicon | a praise word list | +12 |
| first person start | first word *I*, *my*, *mine*, *myself* | +12 |
| indicative modal | *can you*, *will you* | +9 |
| first person plural | *we*, *our*, *us* | +8 |
| negative lexicon | a blame word list | −13 |
| direct question | wh-word first and *?* | −27 |
| *please* first | sentence begins with *please* | −30 |
| second person start | first word *you*, *your*, *yours* | −30 |
| factuality | *in fact*, *actually*, *the truth is*, *the point is* | −38 |
| direct start | first word *so*, *then*, *and*, *but* | −43 |

The asymmetry between *please* inside a request (+49) and *please* at its start (−30) is one of the most quoted findings: an initial *please* reads as an impatient order. The detectors work on the lowercased string, not on the tree; moving them onto the UD tree (e.g. *please* as `discourse` of an imperative root vs of an interrogative one) is listed under *Not yet*.

The social-talk functions of ISO 24617-2 (greeting, goodbye, politeness question *How are you?*, opening and closing politeness *Nice to meet you*, *Have a good day*) are recognised by the same module (`som_kind`) and answered in kind (31, 38).

## 7. Hedging and boosting

### 7.1 What a hedge is

The term goes back to Lakoff (1972, "Hedges: A study in meaning criteria and the logic of fuzzy concepts", CLS 8 — not open), who used it for words "whose job is to make things fuzzier or less fuzzy" (*sort of*, *technically*, *loosely speaking*). In pragmatics and discourse analysis it came to mean devices by which a speaker **weakens commitment** to a proposition or **softens** an act; Hyland's work on academic writing (1998, *Hedging in Scientific Research Articles* — not open) made it central to the study of stance. Boosters (*certainly*, *clearly*, *in fact*) do the opposite. The two functions — weakening the content (approximators: *about*, *sort of*) and weakening the speaker's commitment (shields: *I think*, *perhaps*) — are often distinguished.

### 7.2 English hedging devices

| class | examples | typical UD relation |
|---|---|---|
| epistemic modals | *may, might, could, would* | `aux` |
| epistemic adverbs | *perhaps, maybe, possibly, probably, apparently* | `advmod` |
| epistemic adjectives | *likely, possible, unclear* + extraposed clause | root with `expl` + `csubj` (21 §5.3) |
| parenthetical verbs of thinking | *I think, I suppose, I believe, I guess, I dare say, I fancy* | root of the main clause with the claim as `ccomp`, or `parataxis` when parenthetical |
| verbs of seeming | *seem, appear, tend* | `xcomp` (23) |
| approximators | *about, around, roughly, sort of, kind of, somewhat* | `advmod`; `fixed` (*sort of*) |
| down-toners | *rather, pretty, a bit, slightly, fairly* | `advmod`, `obl:unmarked` |
| conditionals and concessions | *if I'm not mistaken*, *as far as I know* | `advcl` |
| evidential frames | *apparently*, *I hear that*, *they say* | `advmod`; `ccomp` (§8) |
| polite disclaimers | *I'm afraid…* | root *afraid* + `ccomp` |

Two syntactic notes. First, **parenthetical** *I think* (*The Hatter, I think, was late*) is `parataxis` attached to the main predicate, while initial *I think that…* makes *think* the root and the claim a `ccomp` — the tree's head is then the hedge, not the claim; a reader that wants the claim must look one level down (Mova's frames do: a ccomp of *think* is the `quote` of a Say frame, 30 §5.2). Second, *sort of* and *kind of* are hedges only when they modify a predicate or adjective (*sort of tired*); as nouns with an *of*-phrase (*a kind of therapist*) they classify — which is why their counts in §9 are dominated by non-hedges.

(4) *I think that will be the best plan.* (`book:pg11:297`, `decl evaluate`, hedge 0) — the teacher did not mark this one as hedged: in context Alice is proposing, not doubting. Hedge labels depend on function, not on the presence of *I think*.

### 7.3 Boosters and intensives

Intensifiers (*very, quite, so, awfully, terribly, absolutely*) and certainty markers (*certainly, of course, in fact, clearly*) boost. Stoffel's monograph (1901, Part I) traces the English intensives *full, pure, very, right, quite, so, as* and their "fashionable, colloquial and vulgar" successors — the turnover of intensifiers is one of the fastest lexical changes in English. In Mova's politeness detectors, factuality markers (*in fact*, *actually*) carry a negative weight (−38, §6.2): boosting a claim against the hearer is face-threatening.

## 8. Epistemic modality and evidentiality

**Epistemic modality** evaluates how likely a proposition is; **evidentiality** marks the source of the information (seen, heard, inferred, reported). The two are often expressed by the same markers and are grouped together as *epistemicity*: evidentials give the justification for a claim, epistemic modals the degree of support (Grzech, in Bergqvist & Kittilä (eds.) 2020, *Evidentiality, egophoricity and engagement*, Language Science Press, `books/k/ki/kittila-2020-evidentiality-egophoricity-engagement`, after Boye 2012; see also Grzech & Bergqvist (eds.) 2025, *Expanding the Boundaries of Epistemicity*, De Gruyter, `books/b/be/bergqvist-2025-expanding-boundaries-epistemicity`, card only).

English has no grammatical evidentials; it uses lexical and constructional means:

| source of information | English means | example |
|---|---|---|
| direct perception | perception verbs with complements | *I saw him leave*; *I can hear the birds singing* |
| inference | *must*, *seem*, *apparently*, *evidently* | *She must be home — the light is on* |
| assumption | *probably*, *I suppose*, *I guess* | *"I suppose you are real?" said the Rabbit* (`book:pg11757:38`) |
| hearsay / report | *they say*, *reportedly*, *apparently*, *I hear* | *It is said that…* (28) |
| general knowledge | generic present | *Flamingoes and mustard both bite* (`book:pg11:754`) |

Bergqvist & Kittilä (2020, introduction) point out that cross-linguistically indirect evidentials work as "a disclaimer of epistemic authority": the speaker declines responsibility. English hedges such as *apparently* and *I hear* have the same effect, and that is why hedging is also a politeness device (§2): it leaves the hearer room to disagree.

**Egophoricity** — marking that treats first-person statements and second-person questions alike, because the knower is the speaker or the addressee — is not grammaticalised in English, but its pragmatic counterpart is visible: English speakers hedge statements about other people's minds (*he seems upset*) more than about their own (*I'm upset*).

Mova's frames do not yet carry modality or evidential source as fields; only `neg`, `cond` and `Fact(Can)` exist ([../../docs/pragmatics.md](../../docs/pragmatics.md), *Not yet*). The UD tree keeps the modal lemma on `aux`, and the Mova dialect keeps modals without a `Mood` feature ([../../dialects/mova/seeds.md#modal-mood](../../dialects/mova/seeds.md#modal-mood); 21 §3.1).

## 9. The `hedge` label in the parse database

### 9.1 Counts

The pragmatics schema has a binary `hedge` field: "the speaker weakens the commitment (*maybe, I think, sort of, perhaps*)" ([../../prag/src/schema.rs](../../prag/src/schema.rs); 30 §6). In `$MOVA_DATA/parsedb/prag.tsv` (2026-10-17) 2 718 of 311 101 rows (0.9%) have `hedge=1`; the teacher changed the field in 1 800 rows. By source:

| source | rows | hedged | % | indirect | % |
|---|---:|---:|---:|---:|---:|
| `book:` (tales) | 118 321 | 1 743 | 1.47 | 3 330 | 2.81 |
| `dict:` (dictionary) | 163 701 | 789 | 0.48 | 138 | 0.08 |
| `school:` (lessons) | 18 655 | 133 | 0.71 | 62 | 0.33 |
| `myst:` (essays) | 1 567 | 53 | 3.38 | 4 | 0.26 |
| `math:` (word problems) | 8 857 | 0 | 0.00 | 2 | 0.02 |

Speculative essays hedge most (*Perhaps life appears because life is wanted*, `myst:04-the-biological-intelligence-hypothesis:149`), tales next — mostly in characters' speech — and word problems never.

### 9.2 Which cues go with the label

For each candidate cue, the number of sentences containing it with `hedge=1` and with `hedge=0` (substring match on the lowercased text, rows with text available, 2026-10-17):

| cue | hedged | not hedged | share hedged |
|---|---:|---:|---:|
| *I suppose* | 105 | 35 | 75.0% |
| *I dare say* | 13 | 5 | 72.2% |
| *I think* | 258 | 101 | 71.9% |
| *maybe* | 81 | 60 | 57.4% |
| *perhaps* | 385 | 306 | 55.7% |
| *I fancy* | 10 | 8 | 55.6% |
| *I believe* | 37 | 35 | 51.4% |
| *I'm afraid* | 6 | 12 | 33.3% |
| *I guess* | 3 | 7 | 30.0% |
| *may* | 1 025 | 3 184 | 24.4% |
| *probably* | 43 | 165 | 20.7% |
| *possibly* | 34 | 140 | 19.5% |
| *seem* | 388 | 1 733 | 18.3% |
| *might* | 277 | 2 371 | 10.5% |
| *rather* | 57 | 772 | 6.9% |
| *sort of* | 9 | 242 | 3.6% |
| *kind of* | 27 | 909 | 2.9% |

The first-person parenthetical verbs are the most reliable hedges; the modals and *seem* hedge in a fifth or less of their uses, because in lessons and dictionary entries *may* states possibility as a fact (*In school councils a meeting may be adjourned*, `dict:adjourn:3`) rather than the speaker's doubt; *kind of* and *sort of* are mostly classifiers (§7.2). A word list alone would therefore overcount hedges several times; the label depends on whether the device weakens the **speaker's** commitment.

### 9.3 Examples

| sent_id | form, act | text | teacher change |
|---|---|---|---|
| `book:pg11027:343` | `decl assert` | *I am as strong as you, and perhaps stronger.* | `act=assert,hedge=1,voice=character` |
| `book:pg11339:1576` | `decl evaluate` | *"Your conduct," said they, "is very strange and, we think, very unfair.* | `polarity=neg,hedge=1` |
| `book:pg11757:38` | `q ask` | *"I suppose you are real?" said the Rabbit.* | — |
| `book:pg11:129` | `decl disagree` | *"Well, perhaps not," said Alice in a soothing tone: "don't be angry about it.* | — |
| `book:pg11027:697` | `decl assert` | *I believe you don't hear how sweetly the birds are singing.* | — |
| `myst:04-the-biological-intelligence-hypothesis:45` | `decl assert` | *Our causality could be its update rule.* | `polarity=none,hedge=1` |
| `dict:afraid:5` | `decl assert` | *Polite speech: «I'm afraid I can't come» softens bad news.* | — |

The Aesop row shows hedging inside a criticism: the parenthetical *we think* softens *very unfair* — negative evaluation with negative-face redress. *Well, perhaps not* (Alice) hedges a disagreement in a "soothing tone" — the hedge serves politeness, not doubt.

### 9.4 How the label is made

The labeller predicts `hedge` from features of the tree and words ([../../prag/src/feats.rs](../../prag/src/feats.rs)); the teacher checks it. The agreer had a rule **P5 hedge** — set `hedge=1` when both zero-shot NLI witnesses say "uncertain" with p ≥ 0.6 and the certainty regressor is below 4.3 — but it was switched off after a spot check: the witnesses agreed with each other on wrong answers, labelling plain statements as uncertain ([../../prag/src/agree.rs](../../prag/src/agree.rs); `PRAG_AGREE_ALL=1` turns it on for measuring). This is the negative lesson of the hedge field: agreement between two models of the same kind is not evidence.

## 10. Computational hedge detection

| work | data | task | note |
|---|---|---|---|
| Light, Qiu & Srinivasan 2004, https://aclanthology.org/W04-3103 (`papers/l/li/lig/light-2004-language-bioscience-facts-speculations-statements`) | MEDLINE abstracts | speculative vs definite sentences | early annotation study |
| Medlock & Briscoe 2007, "Weakly supervised learning for hedge classification in scientific literature" (`papers/m/me/med/medlock-2007-weakly-supervised-learning-hedge-classification`) | biomedical papers | hedge sentences | weak supervision |
| Vincze et al. 2008, "The BioScope corpus", *BMC Bioinformatics* 9(Suppl 11):S9 | biomedical texts | hedge and negation cues with scopes | open corpus |
| Morante & Daelemans 2009, https://aclanthology.org/W09-1304 (`papers/m/mo/mor/mora/morante-2009-learning-scope-hedge-cues-biomedical`) | BioScope | scope of hedge cues | memory-based learning |
| Farkas et al. 2010, "The CoNLL-2010 Shared Task: Learning to Detect Hedges and their Scope in Natural Language Text", https://aclanthology.org/W10-3001 | Wikipedia "weasel words", BioScope | uncertain sentences; cue scopes | the benchmark task |
| Ulinski, Benjamin & Hirschberg 2018 (`papers/u/ul/ulinski-2018-hedge-detection-improve-committed-belief`) | dialogue and news | hedges for committed belief | dictionary + disambiguation |

Two lessons carry over to Mova. First, cues are ambiguous (*may*, *might*, *sort of*, §9.2), so scope and function matter; the CoNLL-2010 task scored both sentence-level detection and in-sentence scope. Second, the memory-based approach of Morante & Daelemans is the same family as Mova's IGTree and k-NN labellers ([../../prag/src/model.rs](../../prag/src/model.rs)), which can show the remembered sentences behind a decision.

## 11. Politeness across cultures and registers

Politeness norms differ between cultures and between registers of one language. Schauer (2024, *Intercultural Competence and Pragmatics*, Springer, `books/s/sc/schauer-2024-intercultural-competence-pragmatics`) discusses how learners and intercultural speakers acquire the pragmatic norms of English — which request strategies, how much mitigation, when directness is acceptable. The recruitment study (§3) found that across eight languages "cross-linguistic diversity in this pragmatic domain is relatively low" (Floyd, Rossi & Enfield 2020, ch. 1): the basic formats — imperatives, interrogatives, statements of need, gestures — recur, while their proportions and the social meanings attached to them vary.

Within English, register sets the level (37):

| register | typical request | typical hedge |
|---|---|---|
| business email (EWT `email-*`) | *Could you please let us know…*; *Just wanted to confirm…* | *I was wondering if…* |
| academic writing | impersonal passives | *may*, *suggest*, *appear* |
| tales, dialogue | imperatives, *pray*, *if you please* | *I dare say*, *I suppose*, *perhaps* |
| biblical English (JPS 1917) | imperative + *I pray thee* | rare |
| legal English | *shall* (obligation) | none — commitment is the point |
| web forums | direct questions, imperatives | *IMO*, *IIRC*, *kinda* |

## In Mova

- Politeness strategies, social talk and indirect requests with preconditions: [../../world/src/talk.rs](../../world/src/talk.rs) (`politeness`, `som_kind`, `indirect_request`); design and sources: [../../docs/pragmatics.md](../../docs/pragmatics.md).
- Sentence labels `hedge`, `indirect`, `act`: [../../prag/src/schema.rs](../../prag/src/schema.rs), features [../../prag/src/feats.rs](../../prag/src/feats.rs), labellers [../../prag/src/model.rs](../../prag/src/model.rs), agreer with the switched-off P5 [../../prag/src/agree.rs](../../prag/src/agree.rs); the labels in `$MOVA_DATA/parsedb/prag.tsv`.
- Register labels and domains (formal, slang): [../../prag/src/lens.rs](../../prag/src/lens.rs), [../../world/src/domains.rs](../../world/src/domains.rs), [../../world/src/register.rs](../../world/src/register.rs).
- **Not yet**:
  - the politeness detectors read the lowercased string, not the tree, and are used only in the dialogue layer, not in the sentence labels or the parse database;
  - no politeness score or face-threat label in `prag.tsv`; impoliteness is visible only as `polarity=neg` with an act;
  - `hedge` is binary — no scope, no distinction between approximators and shields, no boosters;
  - modality and evidential source are not frame fields;
  - the P5 hedge rule is off until a witness of a different kind exists;
  - *thou/thee* keep their own lemmas in the Torah trees (`thee` → *thee*), so pronoun-based features see them as different words from *you*.

## Sources

- Brown, P., Levinson, S. C. 1987. *Politeness: Some Universals in Language Usage*. Cambridge University Press. Not open — *Wanted*.
- Lakoff, R. 1973. The logic of politeness; or, minding your p's and q's. CLS 9. Not open — *Wanted*.
- Lakoff, G. 1972. Hedges: A study in meaning criteria and the logic of fuzzy concepts. CLS 8. Not open — *Wanted*.
- Leech, G. 1983/2014. *Principles of Pragmatics*; *The Pragmatics of Politeness*. Not open — *Wanted*.
- Hyland, K. 1998. *Hedging in Scientific Research Articles*. John Benjamins. Not open — *Wanted*.
- Floyd, S., Rossi, G., Enfield, N. J. (eds.) 2020. *Getting others to do things: A pragmatic typology of recruitments*. Language Science Press. Ch. 1 and Kendrick, K. H., "Recruitment in English: A quantitative study". `books/e/en/enfield-2020-getting-others-things-pragmatic`
- Van Olmen, D., Andersson, M., Culpeper, J., Giomi, R. (eds.) 2025. *The Grammar of Impoliteness*. De Gruyter. `books/g/gi/giomi-2025-grammar-impoliteness` (card)
- Bourgeois, S., Bousfield, D. 2026. *Pragmatics in Contested Interpretation*. Springer. `books/b/bo/bousfield-2026-pragmatics-contested-interpretation` (card)
- Landert, D., Dayter, D., Messerli, T. C., Locher, M. A. 2023. *Corpus Pragmatics*. Cambridge Elements. `books/l/lo/locher-2023-corpus-pragmatics` (card)
- Schauer, G. A. 2024. *Intercultural Competence and Pragmatics*. Springer. `books/s/sc/schauer-2024-intercultural-competence-pragmatics`
- Stoffel, C. 1901. *Intensives and Down-toners: A Study in English Adverbs*. Heidelberg: Winter. `books/s/st/stoffel-1894-intensives-down-toners-study`
- Abbott, E. A. 1869. *A Shakespearian Grammar*. `books/a/ab/abbott-1869-shakespearian-grammar-attempt-illustrate`
- Bergqvist, H., Kittilä, S. (eds.) 2020. *Evidentiality, egophoricity and engagement*. Language Science Press. `books/k/ki/kittila-2020-evidentiality-egophoricity-engagement`
- Grzech, K., Bergqvist, H. (eds.) 2025. *Expanding the Boundaries of Epistemicity*. De Gruyter. `books/b/be/bergqvist-2025-expanding-boundaries-epistemicity` (card)
- Danescu-Niculescu-Mizil, C., Sudhof, M., Jurafsky, D., Leskovec, J., Potts, C. 2013. A computational approach to politeness with application to social factors. ACL. https://aclanthology.org/P13-1025; `papers/m/mi/miz/mizil-2013-computational-approach-politeness-application-social`
- Roque, A., Tsuetaki, A., Sarathy, V., Scheutz, M. 2020. Developing a Corpus of Indirect Speech Act Schemas. LREC. https://aclanthology.org/2020.lrec-1.28; `papers/r/ro/roq/roque-2020-developing-corpus-indirect-speech-act`
- Holgate, E., Erk, K. 2021. "Politeness, you simpleton!" retorted [MASK]: Masked prediction of literary characters. IWCS. https://aclanthology.org/2021.iwcs-1.19; `papers/h/ho/hol/holgate-2021-politeness-you-simpleton-retorted-mask`
- Light, M., Qiu, X. Y., Srinivasan, P. 2004. The Language of Bioscience: Facts, Speculations, and Statements in Between. https://aclanthology.org/W04-3103; `papers/l/li/lig/light-2004-language-bioscience-facts-speculations-statements`
- Medlock, B., Briscoe, T. 2007. Weakly supervised learning for hedge classification in scientific literature. ACL. `papers/m/me/med/medlock-2007-weakly-supervised-learning-hedge-classification`
- Vincze, V., Szarvas, G., Farkas, R., Móra, G., Csirik, J. 2008. The BioScope corpus: biomedical texts annotated for uncertainty, negation and their scopes. *BMC Bioinformatics* 9(Suppl 11):S9. https://doi.org/10.1186/1471-2105-9-S11-S9
- Morante, R., Daelemans, W. 2009. Learning the scope of hedge cues in biomedical texts. BioNLP. https://aclanthology.org/W09-1304; `papers/m/mo/mor/mora/morante-2009-learning-scope-hedge-cues-biomedical`
- Farkas, R., Vincze, V., Móra, G., Csirik, J., Szarvas, G. 2010. The CoNLL-2010 Shared Task: Learning to Detect Hedges and their Scope in Natural Language Text. https://aclanthology.org/W10-3001
- Ulinski, M., Benjamin, S., Hirschberg, J. 2018. Using Hedge Detection to Improve Committed Belief Tagging. `papers/u/ul/ulinski-2018-hedge-detection-improve-committed-belief`
- UD English EWT 2.18 (CC BY-SA 4.0), sentences cited by `sent_id`.
- Mova's parse database (`$MOVA_DATA/parsedb/prag.tsv`, `current.conllu`) and the Torah JPS 1917 trees, counted 2026-10-17; book titles from the tales registry: `pg11` *Alice's Adventures in Wonderland*, `pg11339` *Aesop's Fables* (Jones 1912), `pg11027` *Grimm's Fairy Stories*, `pg11757` *The Velveteen Rabbit*, `pg23661` *The Book of Dragons*, `pg27200` *Fairy Tales of Hans Christian Andersen*.
- Repo: [../../docs/pragmatics.md](../../docs/pragmatics.md), [../../docs/context.md](../../docs/context.md).
