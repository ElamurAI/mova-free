# 37. Register and variation

The same English grammar is used differently in a court ruling, a tale, a text message and a schoolyard: some constructions cluster in some situations, and some forms belong to one region, one generation or one century. This chapter describes register (variation by situation of use), genre and style, the syntax of spoken and written, formal and informal English, computer-mediated English, regional and social dialect syntax, and the archaic and biblical register of the tales and the Torah translation Mova reads — with counts from EWT, GUM and Mova's corpora and the register tools Mova has.

Contents

1. [Register, genre, style, dialect](#1-register-genre-style-dialect)
2. [Register as co-occurring features](#2-register-as-co-occurring-features)
3. [Spoken and written English](#3-spoken-and-written-english)
4. [Formal and informal syntax](#4-formal-and-informal-syntax)
5. [Web and computer-mediated English](#5-web-and-computer-mediated-english)
6. [Regional and social dialect syntax](#6-regional-and-social-dialect-syntax)
7. [Learner English](#7-learner-english)
8. [Archaic and biblical English](#8-archaic-and-biblical-english)
9. [Slang and generational registers in Mova's school](#9-slang-and-generational-registers-in-movas-school)
10. [Style guides as register norms](#10-style-guides-as-register-norms)
11. [How Mova measures and uses register](#11-how-mova-measures-and-uses-register)

---

## 1. Register, genre, style, dialect

| term | varies with | examples |
|---|---|---|
| register | the situation of use: medium (spoken/written), purpose, relation of the participants, planning time | conversation, news, academic prose, legal drafting, chat |
| genre | the conventional text type and its structure | fable, recipe, court ruling, review, sermon |
| style | the individual or aesthetic choices within a register | terse, ornate, ironic, mock-formal |
| dialect | the user: region, social group, generation | Northern English, African American English, Scots |
| period | time | Early Modern English (Shakespeare, King James), Victorian tales |

The distinction register vs dialect goes back to Halliday's work in the 1960s; the corpus-based account of English registers is Biber's multidimensional analysis (Biber 1988, *Variation across Speech and Writing*; Biber et al. 1999, *Longman Grammar of Spoken and Written English*; neither is open — Wanted). Its method — count many linguistic features per text, find the factors along which they co-vary, and read each factor as a functional dimension (involved vs informational, narrative vs non-narrative, explicit vs situation-dependent reference, overt persuasion, abstract style) — has been applied to chat and other computer-mediated registers by Jonsson (2015, *Conversational Writing: A Multidimensional Study of Synchronous and Supersynchronous Computer-Mediated Communication*, Peter Lang, `books/j/jo/jonsson-2015-conversational-writing-multidimensional-study`, card only).

Mova's design note asks for the register to be part of the document's "passport": domain, genre and speech style, formal or informal, spoken or written, narrator, purpose and mode — because the same word is read differently by register (*like* is a discourse marker in conversation, *shall* an obligation in law) ([../../docs/context.md](../../docs/context.md), *Document level*).

## 2. Register as co-occurring features

GUM, a UD treebank of 15 genres, shows the multidimensional picture in miniature. Rates per 1 000 words, counted on all of UD English GUM 2.18 on 2026-10-17 (GUM is CC BY-NC-SA: counts only):

| genre | words | `discourse` | `reparandum` | `nsubj:pass` | *I / you / we* |
|---|---:|---:|---:|---:|---:|
| vlog | 16 848 | 32.0 | 4.27 | 2.4 | 74.7 |
| conversation | 17 928 | 30.0 | 20.86 | 2.5 | 58.9 |
| podcast | 16 168 | 26.1 | 11.26 | 3.8 | 47.6 |
| court | 17 089 | 12.8 | 9.19 | 9.2 | 27.7 |
| interview | 18 172 | 3.0 | 0.72 | 6.0 | 36.2 |
| fiction | 17 501 | 2.2 | 0.06 | 3.8 | 34.0 |
| letter | 16 348 | 1.3 | 0.18 | 7.9 | 33.4 |
| speech | 16 717 | 0.5 | 1.14 | 5.3 | 27.8 |
| how-to (whow) | 17 075 | 1.0 | 0.12 | 4.5 | 22.5 |
| essay | 17 160 | 0.6 | 0.00 | 6.1 | 12.8 |
| textbook | 16 690 | 1.6 | 0.00 | 8.4 | 11.1 |
| voyage | 16 503 | 0.8 | 0.06 | 9.7 | 6.7 |
| academic | 17 164 | 1.2 | 0.00 | 11.0 | 5.0 |
| news | 17 182 | 0.2 | 0.06 | 11.2 | 4.1 |
| bio | 18 194 | 0.2 | 0.00 | 9.6 | 1.2 |

The features move together: unplanned spoken genres have many interjections and self-repairs and many first- and second-person pronouns (Biber's "involved" pole); news, biography and academic prose have passives and almost no *I/you/we* ("informational"). Court transcripts are spoken but formal and sit in the middle. Prepared speech is written to be spoken: low on repairs and interjections, mid on pronouns.

## 3. Spoken and written English

| feature | spoken (unplanned) | written (edited) | UD |
|---|---|---|---|
| fragments and non-sentences | frequent: *Yeah. Me too. In the kitchen.* | rare | root is not a predicate; prag form `frag` |
| interjections, fillers | *um, uh, oh, well, like* | rare | `discourse` (INTJ only in the Mova dialect) |
| self-repair | *go to the- to the shop* | edited out | `reparandum` |
| backchannels | *mhm, right, yeah* | absent | `discourse` |
| vocatives | frequent | letters, dialogue | `vocative` |
| dislocation | *That guy, I don't trust him.* | rare | `dislocated` |
| clause linking | *and, so, then* chains; parataxis | subordination, participial clauses | `conj`, `parataxis` vs `advcl`, `acl` |
| noun phrases | short, pronouns | long, nominalised, stacked modifiers | `compound`, `amod`, `nmod` |
| ellipsis | situational: *Want some?* | textual | 29 |

UD gives the spoken constructions their own relations (`reparandum`, `discourse`, `vocative`, `dislocated`); subtypes for fillers, tag questions and reformulations (`discourse:filler`, `discourse:tag`, `conj:reform`) are documented for French but not yet for English, and the Mova dialect waits for a universal decision ([../../dialects/mova/seeds.md#spoken-subtypes](../../dialects/mova/seeds.md#spoken-subtypes)). Disfluencies are not noise but part of how speakers manage turns (Ginzburg et al. 2014, "Disfluencies as intra-utterance dialogue moves", *Semantics and Pragmatics* 7, `papers/g/gi/gin/ginzburg-2014-disfluencies-as-intra-utterance-dialogue`; 38).

Spoken English in UD: ESLSpok (spoken learner English, CC BY-SA 4.0, no lemmas, [../../treebanks/INDEX.md#en-eslspok](../../treebanks/INDEX.md#en-eslspok)), CHILDES (child and child-directed speech), and the spoken genres of GUM. Mova's parser was trained on EWT and ESLSpok plus the school trees ([../../docs/architecture.md](../../docs/architecture.md)).

## 4. Formal and informal syntax

| construction | formal | informal | chapter |
|---|---|---|---|
| contraction | *do not, it is* | *don't, it's, gonna, wanna* | 21 §3.3 |
| relative pronoun | *the man whom I saw*, *with whom* | *the man I saw*, *who … with* | 25 |
| preposition | pied-piped: *to whom* | stranded: *who … to* | 22 §7.3, 25 |
| case after *be* / *than* | *It is I*, *taller than I* | *It's me*, *taller than me* | 22 §2.11 |
| passive | frequent; agentless | less frequent; *get*-passive | 28 |
| nominalisation | *the distribution of crayons* | *giving out crayons* | 22 §9 |
| subjunctive | *if I were*, *insist that he go* | *if I was*, *insist that he goes* | 21 §3 |
| negation | *I have no idea* | *I haven't got a clue*, *I don't know nothing* (dialect) | 27 |
| address | *Madam*, title + surname | first name, *mate*, *dude* | 32, 38 |
| connectives | *however, therefore, moreover* | *but, so, plus* | 36 |
| sentence length | long, embedded | short, coordinated | — |

In EWT 2.18, genres differ along these lines (counted 2026-10-17 over all splits, per 1 000 words): *n't* is most frequent in answers (6.0) and reviews (5.1) and least in weblogs (2.0) and newsgroups (1.6); passive subjects run the other way, from 7.6 in weblogs to 4.3 in answers; interjections (`discourse`) are densest in answers (7.1) and emails (6.7) and rarest in weblogs (1.0).

| EWT genre | sentences | words | `discourse` /1k | *n't* /1k | `nsubj:pass` /1k | `vocative` |
|---|---:|---:|---:|---:|---:|---:|
| answers | 3 488 | 53 999 | 7.09 | 6.04 | 4.33 | 15 |
| email | 4 899 | 57 806 | 6.71 | 2.61 | 6.19 | 89 |
| reviews | 3 814 | 55 588 | 2.88 | 5.14 | 4.48 | 45 |
| newsgroup | 2 391 | 43 045 | 2.14 | 1.63 | 6.20 | 23 |
| weblog | 2 030 | 44 382 | 0.97 | 1.98 | 7.59 | 11 |

Emails have the most vocatives (89): salutations (*Hi John*) and sign-offs.

## 5. Web and computer-mediated English

Web English mixes written medium with conversational involvement (Yus 2011, *Cyberpragmatics: Internet-mediated communication in context*, John Benjamins, `books/y/yu/yus-2011-cyberpragmatics-internet-mediated-communication`, card only; Jonsson 2015). Features that matter for parsing:

| feature | example | UD treatment in EWT |
|---|---|---|
| emoticons, emoji | *:)*, *:(* | SYM, `discourse` (59 *:)* in EWT) |
| laughter, acronyms | *lol*, *btw*, *imo* | INTJ `discourse` / ADV `advmod` |
| non-standard spelling | *u, ur, thx, alot* | lemma normalised; `Typo=Yes` or `Style=` |
| missing punctuation and capitals | *you mean miramar florida theyy have good seafood* | `parataxis` between run-on clauses |
| email headers, signatures, lists | *"Les Spahnn" <spahnn@…>* | `flat`, `appos`, `list` |
| URLs, file names | — | `FlatType` in MISC for numbered and file-name `flat` |

EWT marks deliberate non-standard forms with a `Style` feature and true errors with `Typo=Yes`. In EWT 2.18 (2026-10-17): `Style=Expr` 49 (expressive spellings), `Vrnc` 19 (vernacular), `Slng` 12, `Arch` 9, `Coll` 4. The Mova seed [../../en/seeds/errors/vernacular-style.md](../../en/seeds/errors/vernacular-style.md) explains why the difference matters: *should of took* is *have* with `Style=Vrnc`, not a preposition; *them boys* is a determiner; *ain't* has the lemma *be*; *cos, coz* are *because* with `Abbr=Yes`. An annotator that "corrects" such forms loses the register; one that does not see *of* = *have* breaks the tree.

(1) EWT 2.18, a weblog sentence: *"Another generation of heavy metal has taken over, and -- sorry -- it ain't just about strippers and dope.* — *ai* (lemma *be*) with `Style=Vrnc`; *sorry* is an interjected apology, `discourse`.

## 6. Regional and social dialect syntax

### 6.1 Features

| feature | example | where | analysis |
|---|---|---|---|
| negative concord | *I can't go no lower* | widespread non-standard; older English | each negative is `advmod`/`det` with `Polarity=Neg`; one logical negation (27) |
| *ain't* | *it ain't* | widespread | *ai* AUX lemma *be/have* + *n't* |
| *as / at / what* as relatives | *the man as/what lives here* | England, Scotland | relative pronoun, `nsubj` etc. in the relative clause (25) |
| zero subject relatives | *I know a man will do it for you* | dialects; colloquial | `acl:relcl` without a relativizer |
| Northern Subject Rule | *birds sings*, but *they sing* | Northern England, Scotland | agreement with a noun subject, not with an adjacent pronoun |
| *them* as demonstrative | *them boys* | widespread | DET `det`, `Style=Vrnc` |
| double modals | *might could*, *used to could* | US South, Scotland, N. England | two AUX `aux` |
| habitual / invariant *be* | *she be working* | African American English, some Irish English | AUX *be* with a habitual meaning |
| *a*-prefixing | *a-going, a-hunting* | Appalachia, older English | a prefix of the participle (morphology) |
| *done* perfective | *I done told you* | US South, AAE | *done* AUX-like |
| *for to* infinitive | *I went for to see* | older and regional | `mark` *for* + `mark` *to* |
| *was* levelling | *we was* | widespread non-standard | agreement; the form wins for features |
| *youse, y'all, you guys* | second-person plurals | regional | PRON `Number=Plur` |

The classic survey of dialects of England, Scotland and Ireland is Wright (1905, *The English Dialect Grammar*, Oxford, `books/w/wr/wright-1905-english-dialect-grammar-comprising`). Two of its observations are relevant to syntax: the relative pronoun is "gen. expressed by *as, at, that,* or *what* … when the antecedent is expressed", *whom* "is never used in the dialects", and relatives are often omitted even in the subject position (*I know a man will do for you*) (Wright 1905, §423); and in Scotland, Ireland, the North and most of the north Midlands every person of the present takes the *-s* ending when the subject is a noun or a relative, or is separated from the verb, but not when the verb is next to its own pronoun (Wright 1905, §435) — the pattern later called the Northern Subject Rule.

(2) Negative concord in a tale — `book:pg11:1028` (*Alice's Adventures in Wonderland*): *"I can't go no lower," said the Hatter: "I'm on the floor, as it is."* The Hatter's two negatives make one negation; Carroll gives him a non-standard voice. The prag label is `decl disagree` (2026-10-17).

### 6.2 How Mova treats dialect forms

The parser annotates **what is written**, not what the speaker should have said — the "literal reading" principle of the learner treebank TLE (Berzak et al. 2016, repo summary [../../papers/berzak-2016-universal-dependencies-learner-english.md](https://research.elamur.ai/free/en/papers/berzak-2016-universal-dependencies-learner-english/); seed [../../en/seeds/errors/literal-annotation-learner.md](../../en/seeds/errors/literal-annotation-learner.md)). Dialect words still fool the witness parsers: in the agreed school trees, the spot check of rule R5 found dialect words like *aboot* among the cases where the witnesses agreed on a wrong head ([../../tools/treeagree/README.md](../../tools/treeagree/README.md)). The Mova dialect of UD is a different thing — a variant of the *annotation scheme*, with converters to other treebanks' conventions ([../../docs/dialect.md](../../docs/dialect.md)) — but its converters are how ESLSpok, CHILDES and older-UD data with their own conventions join the training data.

## 7. Learner English

Learner English (ESL) differs from dialect: the variation is not a community norm but an interlanguage. ESLSpok in UD 2.18 is spoken learner English without lemmas or features; TLE (Berzak et al. 2016) pairs each learner sentence with a corrected one. Mova's seeds catalogue typical learner errors as checks — article choice, missing copula, agreement, *much/many*, prepositions, over-regularised verbs ([../../en/seeds/errors/](../../en/seeds/errors/): `missing-article`, `missing-copula`, `singular-subject-plural-verb`, `much-many-countability`, `preposition-errors`, `overregularized-verbs`, and the ERRANT taxonomy in `errant-taxonomy.md`).

## 8. Archaic and biblical English

### 8.1 Features

| feature | example (JPS 1917 Torah) | id | UD |
|---|---|---|---|
| *thou/thee/thy/thine/ye* | *The thing which **thou hast** spoken is good for us to do.* | `jps:DEU:13` | PRON with Person=2, Number=Sing for *thou* |
| *-eth* third singular | *in the morning he **devoureth** the prey* | `jps:GEN:1626` | VERB `VBZ` |
| *-est* second singular | *what **hast thou** found* | `jps:GEN:977` | AUX/VERB with *thou* |
| inverted questions without *do* | *what **hast thou** found* | `jps:GEN:977` | `aux` before `nsubj` |
| quotative inversion | *Thus **saith the LORD*** | `jps:EXO:304` | `nsubj` after the verb (29) |
| fronted adverbial + inversion | *About midnight **will I** go out* | `jps:EXO:304` | `obl` first, `aux` before `nsubj` |
| episode opener | ***And it came to pass** at the end of two full years, that Pharaoh dreamed* | `jps:GEN:1297` | see (3) |
| *and*-chains | *and, behold, he stood by the river* | `jps:GEN:1297` | `cc` + `conj`/`parataxis` |
| *unto* | *said unto Aaron* | `jps:EXO:966` | ADP `case` |
| subjunctive and optative | ***Cursed be** every one that curseth thee* | `jps:GEN:806` | `Mood=Sub`, predicate first |
| *that* as relative with any antecedent | *a wolf **that** raveneth* | `jps:GEN:1626` | PRON `nsubj` in `acl:relcl` |

In the JPS 1917 trees (`$MOVA_DATA/corpora/judaism/parsedb/current.conllu`, 6 065 sentences, 241 216 words, counted 2026-10-17): *thou/thee/thy/thine/ye* 4 794 tokens (2.0% of words); verbs in *-eth* 617, verb and auxiliary forms in *-est* 230; 3 264 sentences begin with *And* (53.8%); *And it came to pass* opens 99 sentences.

### 8.2 Where the parser stumbles

(3) `jps:GEN:1297` — *it* `nsubj(came, it)`, *pass* `xcomp`, *dreamed* `ccomp(came, dreamed)`. *It* is not a referent here: the construction is an extraposition with a clausal subject (*that Pharaoh dreamed* came to pass), and the UD analysis would make *it* `expl` (21 §5.3; 29).

(4) `jps:EXO:304` *Thus saith the LORD* — `ccomp(said, saith)`, `nsubj(saith, LORD)`: the inverted subject is found, but the lemma of *saith* is *saith*, not *say*. Archaic verb forms are a morphology task (seed [../../en/seeds/morph/verb-archaic-endings.md](../../en/seeds/morph/verb-archaic-endings.md)); the frame reader matches *saith* to no VerbNet class until the lemma is right.

The tales use a milder version of the same register — *thou* in fairy-tale dialogue, *-eth* in proverbs, *said he* inversions — and the register lexicons find it: in Mova's tale lexicon the second most marked word against the reference books is *thou* (§11.1). Abbott (1869, *A Shakespearian Grammar*, `books/a/ab/abbott-1869-shakespearian-grammar-attempt-illustrate`) and Hejná & Walkden (2022, *A history of English*, Language Science Press, `books/h/he/hejna-2022-history-english`) describe the Early Modern forms; Emerson (1894, *The History of the English Language*, `books/e/em/emerson-1894-history-english-language`) is an older history.

## 9. Slang and generational registers in Mova's school

Mova's school courses deliberately mix registers in their conversational lines. The writing guide asks for real speech — clipped, with wrong grammar, slips and self-corrections, gossip, euphemisms, rough everyday swearing without slurs — and specifies: "Half the conversational lines the everyday mix (light general slang, sometimes dry, sometimes formal), half hard slang and dialect of the speaker's generation and place — US generations, and the words of every English-speaking country, Britain village by village"; the exam, in contrast, is "so formal the pupil panics" ([../../docs/school.md](../../docs/school.md), *The voice*). The reason given is saturation: hundreds of examples of popular words in every register a child meets, accepting worse scores at first.

This makes register a measurable variable for the student: the same fact is taught in slang and examined in formal language, and the course pupil's exam score (formal) can be compared with its test score (mixed) (30 §9).

## 10. Style guides as register norms

Style guides codify the written formal register and are evidence of what was considered correct at a time. The library holds:

| guide | year | what it norms | library path |
|---|---|---|---|
| Fowler & Fowler, *The King's English* | 1906 | British educated prose; errors of journalese | `books/f/fo/fowler-1906-kings-english` |
| Fowler, *A Dictionary of Modern English Usage* | 1926 | British usage, word by word | `books/f/fo/fowler-1926-dictionary-modern-english-usage` |
| Strunk, *The Elements of Style* | 1918 | concise American prose | `books/s/st/strunk-1918-elements-style` |
| University of Chicago Press, *Manual of Style* | 1906 | typography and editing | `books/p/pr/press-1906-manual-style-being-compilation` |
| U.S. GPO, *Style Manual* | 2016 | US government writing | `books/o/of/office-2016-gpo-style-manual` |
| European Commission, *English Style Guide* | 1982 onwards | EU institutional English | `books/c/co/commission-1982-english-style-guide-handbook` |
| Vizetelly, *A Desk-Book of Errors in English* | 1906 | American usage errors | `books/v/vi/vizetelly-1906-desk-book-errors-english` |

Their prescriptions (no split infinitives, *whom* in object position, no stranded prepositions, no sentence-initial *and* or *but*) describe the formal pole of §4; the corpora show how often English speakers ignore them in other registers. Mova uses such guides as sources for seeds, never as the definition of grammaticality: the parser annotates what is written (§6.2).

## 11. How Mova measures and uses register

### 11.1 Register lexicons from corpora

`world register-stats` measures which words a register uses far more than a reference corpus: log-odds with an informative Dirichlet prior (Monroe, Colaresi & Quinn 2008, "Fightin' Words"), z-scored ([../../world/src/register.rs](../../world/src/register.rs)). The reason given in the code: Wiktionary labels few formal words (*pursuant, notwithstanding, whereas* have no label), so the register is measured, not listed. Three lexicons are compiled into `global` (headers of the files, 2026-10-17):

| lexicon | register corpus | reference | entries | most marked words |
|---|---|---|---:|---|
| [`register-legal.tsv`](../../global/data/register-legal.tsv) | 113 public-domain legal and official books, 7.3 M tokens | 2 738 books, 174 M tokens | ~5 500 | *law, states, of* |
| [`register-tale.tsv`](../../global/data/register-tale.tsv) | 1 488 books, 102.5 M tokens | 1 363 books, 78.8 M tokens | ~11 500 | *king, thou, of* |
| [`register-children.tsv`](../../global/data/register-children.tsv) | 1 200 books, 67.0 M tokens | 1 651 books, 114.2 M tokens | ~9 150 | *you, we, had* |

The counts derive from public-domain texts and are part of the open release ([../../licence/DATA.md](../../licence/DATA.md)). `global::register_z(name, word)` returns a word's z ([../../global/src/lib.rs](../../global/src/lib.rs)).

### 11.2 Domains: formal, slang, humour, hidden meaning

`world domains` switches on reading domains with reasons ([../../world/src/domains.rs](../../world/src/domains.rs); [../../docs/pragmatics.md](../../docs/pragmatics.md)): register labels on the main sense of a word or expression (from the expression base), idioms and proverbs, the legal register measured on corpora, and a **register clash** — formal language about nursery things is read as mock-formal humour, with the homely words taken from the children's lexicon.

(5) `world domains "The toddler is hereby charged with the distribution of crayons pursuant to the agreement."` «Mova 0.4, CPU, 2026-10-17»:

```text
domain formal:
  because 4 words of the legal/official register (corpus log-odds z ≥ 20): hereby 32, charged 28, distribution 27, agreement 47
```

The formal register is found; the clash with *toddler* and *crayons* did not fire on this sentence at the current thresholds.

(6) `world domains "Grandpa kicked the bucket, the poor old geezer."` — hidden-meaning, humour and slang from the labels of *kicked the bucket* (idiomatic, euphemistic, humorous, colloquial: to die), *Grandpa* and *geezer* (informal).

The label-to-domain map is closed ([../../global/src/lib.rs](../../global/src/lib.rs), `register_domain`): *slang, internet slang, informal, colloquial, vulgar, derogatory, offensive, nonstandard* → slang; *formal, literary, archaic, legal, law, officialese, bureaucratese, poetic* → formal; *humorous, jocular, ironic, sarcastic* → humour; *euphemistic, figurative, idiomatic* → hidden meaning. Measured on contrast sets of 200 sentences written by the teacher (test set, 2026-10-02): formal precision 67.7% / recall 70.0%, slang 59.3% / 47.1%, hidden meaning 69.2% / 46.2%, humour 28.9% / 17.7%; literal sentences with a false alarm 23.1% ([../../docs/pragmatics.md](../../docs/pragmatics.md)). Modern slang (*lowkey, no cap, ghosting*) is mostly a secondary dictionary sense of common words and needs time-sliced corpus statistics, not the dictionary.

The expression base behind the labels comes from the English Wiktionary (CC BY-SA) and is kept out of the free release ([../../licence/DATA.md](../../licence/DATA.md)): in the free release the lens and the label-based domains have no data; the corpus-measured lexicons of §11.1 remain.

### 11.3 Not yet

- No document passport yet (domain, genre, style, narrator, purpose, mode as `# mova::` comments; [../../docs/context.md](../../docs/context.md)).
- No register-conditioned parsing: the same model parses tales, lessons and dictionary entries.
- No dialect feature detection (negative concord, Northern Subject Rule, double modals) and no normalisation layer for the frame reader.
- Archaic morphology (*saith*, *-eth*, *-est* forms) is not fully lemmatised, so frames on the Torah and old tales lose verbs.
- `Style` features (`Vrnc`, `Coll`, `Slng`, `Arch`) are not predicted.
- Humour from register clash has low recall.

## In Mova

- Register lexicons: [../../world/src/register.rs](../../world/src/register.rs), [../../global/data/register-legal.tsv](../../global/data/register-legal.tsv), [register-tale](../../global/data/register-tale.tsv), [register-children](../../global/data/register-children.tsv), `global::register_z` in [../../global/src/lib.rs](../../global/src/lib.rs).
- Domains and the lens: [../../world/src/domains.rs](../../world/src/domains.rs), [../../prag/src/lens.rs](../../prag/src/lens.rs), `global::register_domain`; docs [../../docs/pragmatics.md](../../docs/pragmatics.md).
- Seeds: [../../en/seeds/errors/vernacular-style.md](../../en/seeds/errors/vernacular-style.md), [literal-annotation-learner](../../en/seeds/errors/literal-annotation-learner.md), [double-negation](../../en/seeds/errors/double-negation.md), [../../en/seeds/morph/verb-archaic-endings.md](../../en/seeds/morph/verb-archaic-endings.md); spoken subtypes [../../dialects/mova/seeds.md#spoken-subtypes](../../dialects/mova/seeds.md#spoken-subtypes).
- Treebank cards: [../../treebanks/INDEX.md#en-eslspok](../../treebanks/INDEX.md#en-eslspok), [../../treebanks/INDEX.md#en-ewt](../../treebanks/INDEX.md#en-ewt), [../../treebanks/INDEX.md#en-gum](../../treebanks/INDEX.md#en-gum).
- The school's voice: [../../docs/school.md](../../docs/school.md).

## Sources

- Biber, D. 1988. *Variation across Speech and Writing*. Cambridge University Press. Wanted.
- Biber, D., Johansson, S., Leech, G., Conrad, S., Finegan, E. 1999. *Longman Grammar of Spoken and Written English*. Wanted.
- Jonsson, E. 2015. *Conversational Writing: A Multidimensional Study of Synchronous and Supersynchronous Computer-Mediated Communication*. Peter Lang. `books/j/jo/jonsson-2015-conversational-writing-multidimensional-study` (card)
- Yus, F. 2011. *Cyberpragmatics: Internet-mediated communication in context*. John Benjamins. `books/y/yu/yus-2011-cyberpragmatics-internet-mediated-communication` (card)
- Ginzburg, J., Fernández, R., Schlangen, D. 2014. Disfluencies as intra-utterance dialogue moves. *Semantics and Pragmatics* 7. `papers/g/gi/gin/ginzburg-2014-disfluencies-as-intra-utterance-dialogue`
- Wright, J. 1905. *The English Dialect Grammar*. Oxford. §423 (relatives), §435 (present-tense endings). `books/w/wr/wright-1905-english-dialect-grammar-comprising`
- Berzak, Y. et al. 2016. Universal Dependencies for Learner English. ACL. Repo summary [../../papers/berzak-2016-universal-dependencies-learner-english.md](https://research.elamur.ai/free/en/papers/berzak-2016-universal-dependencies-learner-english/)
- Abbott, E. A. 1869. *A Shakespearian Grammar*. `books/a/ab/abbott-1869-shakespearian-grammar-attempt-illustrate`
- Hejná, M., Walkden, G. 2022. *A history of English*. Language Science Press. `books/h/he/hejna-2022-history-english`
- Emerson, O. F. 1894. *The History of the English Language*. `books/e/em/emerson-1894-history-english-language`
- Fowler, H. W., Fowler, F. G. 1906. *The King's English*. `books/f/fo/fowler-1906-kings-english`
- Fowler, H. W. 1926. *A Dictionary of Modern English Usage*. `books/f/fo/fowler-1926-dictionary-modern-english-usage`
- Strunk, W. 1918. *The Elements of Style*. `books/s/st/strunk-1918-elements-style`
- University of Chicago Press. 1906. *Manual of Style*. `books/p/pr/press-1906-manual-style-being-compilation`
- U.S. Government Publishing Office. 2016. *GPO Style Manual*. `books/o/of/office-2016-gpo-style-manual`
- European Commission. 1982–. *English Style Guide*. `books/c/co/commission-1982-english-style-guide-handbook`
- Vizetelly, F. H. 1906. *A Desk-Book of Errors in English*. `books/v/vi/vizetelly-1906-desk-book-errors-english`
- Monroe, B. L., Colaresi, M. P., Quinn, K. M. 2008. Fightin' Words: Lexical Feature Selection and Evaluation for Identifying the Content of Political Conflict. *Political Analysis* 16(4). (As cited in [../../world/src/register.rs](../../world/src/register.rs).)
- UD English EWT 2.18 (CC BY-SA 4.0) and GUM 2.18 (CC BY-NC-SA 4.0, counts only), counted 2026-10-17.
- The tales and Torah trees in Mova's parse databases (`$MOVA_DATA/parsedb/current.conllu`, `$MOVA_DATA/corpora/judaism/parsedb/current.conllu`), counted 2026-10-17.
- Repo: [../../docs/context.md](../../docs/context.md), [../../docs/school.md](../../docs/school.md), [../../docs/pragmatics.md](../../docs/pragmatics.md), [../../docs/dialect.md](../../docs/dialect.md), [../../licence/DATA.md](../../licence/DATA.md).
