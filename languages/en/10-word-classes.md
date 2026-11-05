# 10. Word classes

A word class (part of speech, lexical category) is a set of words that share inflection and distribution: they take the same endings and occur in the same positions. English has little inflection, so its classes are defined mostly by **distribution**, and many words belong to more than one class (*round* is a noun, verb, adjective, adverb and preposition). This chapter lists the classes, the tests that tell them apart, their tags in Mova's two tag sets (PTB and UD), the boundaries that are hard to draw, and how Mova's tagger decides.

Reynolds 2026 ch. 3–4 describes English categories in the framework of the *Cambridge Grammar of the English Language* (Huddleston & Pullum 2002): categories (noun, verb, adjective, adverb, preposition, determinative, coordinator, subordinator, interjection) are kept apart from functions (subject, object, modifier, determiner…), and the relation between them is "many-to-many" (§3.2.1). UD and PTB, which Mova uses, follow a more traditional division; the differences are noted below.

## 1. Open and closed classes

| | open (lexical, content) classes | closed (function, grammatical) classes |
|---|---|---|
| members | nouns, verbs, adjectives, adverbs (most), interjections | pronouns, determiners, auxiliaries and modals, prepositions, coordinators, subordinators, particles, numerals (as a system) |
| new members | constantly, by borrowing and word formation | almost never (*they* singular and *xe, ze* are rare examples of change) |
| stress | carry stress | usually unstressed, with weak forms ([04](04-connected-speech.md) §2) |
| frequency | many words, each mostly rare | few words, each very frequent |
| in Mova | lemmas in `en/data/lemmas.tsv` (110 308) | short closed lists in code and seeds (e.g. `en/seeds/verbal/aux-closed-list.md`) |

In UD English EWT + ESLSpok train (204 442 tokens; Mova's `en/data/forms.tsv`, counted 2026-10-17), the most frequent PTB tags are NN 13.2%, IN 10.2%, DT 8.2%, NNP 6.1%, PRP 6.0%, JJ 5.7%, RB 5.2%, VB 4.6%, NNS 4.1%, CC 3.3%; punctuation is about 15%.

## 2. The classes

### 2.1 Nouns

- **Inflection**: plural *-s* (*cats*) and genitive *'s* (*cat's*), with exceptions ([11-nouns.md](11-nouns.md)).
- **Distribution**: head of a noun phrase; after a determiner (*the cat, my book, some water*); after an adjective (*a black cat*); as subject or object.
- **Derivation**: noun suffixes *-ness, -ity, -tion, -ment, -er, -ism* ([17](17-derivation-suffixes.md) §2).
- **Subclasses**: common vs proper; count vs mass ([11](11-nouns.md) §5); concrete vs abstract; collective (*team, family*).

### 2.2 Verbs

- **Inflection**: five forms: base *walk*, third singular *walks*, past *walked*, past participle *walked*, *-ing* form *walking* ([13-verbs.md](13-verbs.md)).
- **Distribution**: head of a clause; takes a subject; after modals and *to* in the base form.
- **Subclasses**: lexical vs auxiliary (§2.6); transitive, intransitive, ditransitive, copular, complex-transitive ([23](23-complementation.md)); stative vs dynamic.

### 2.3 Adjectives

- **Inflection**: comparison *-er, -est* for short adjectives ([15](15-adjectives-adverbs.md)).
- **Distribution**: **attributive** (before a noun: *a happy child*) and **predicative** (after *be, seem, become*: *the child is happy*); modified by *very, too, so* (Reynolds 2026 §§3.3.2–3.3.8 gives these properties as tests, with the ability to take *-ly* to form adverbs).
- **Exceptions**: attributive-only (*the main reason, a mere child, an utter fool*, `en/seeds/nominal/attributive-only.md`), predicative-only (*afraid, asleep, alive, alone, aware*: the *a*-adjectives, `en/seeds/nominal/a-adjectives.md`), postpositive (*something new, the president elect, the people concerned*).

### 2.4 Adverbs

- **Form**: many in *-ly* (*quickly*); others simple (*soon, often, here, very, well*) or flat (*fast, hard, late*) ([15](15-adjectives-adverbs.md) §4).
- **Distribution**: modify verbs, adjectives, other adverbs, clauses; very mobile in the clause.
- The traditional adverb class is a remainder category. Reynolds 2026 §4.2.2 ("Words that are not actually adverbs") argues that many traditional "adverbs" are better analysed as intransitive prepositions (*inside, before, here, away*) or determinatives.

### 2.5 Pronouns and determiners

- **Pronouns** stand for a whole noun phrase: personal (*I, you, she*), reflexive (*herself*), possessive (*mine*), demonstrative (*this* used alone), interrogative and relative (*who, which*), indefinite (*someone, nothing*) ([12](12-pronouns-determiners-numerals.md)).
- **Determiners** (CGEL: *determinatives*) introduce a noun: articles *the, a/an*; demonstratives *this, that, these, those*; quantifiers *some, any, no, every, each, all, both, many, much, few*; possessive determiners *my, your, her* (UD: PRON with `Poss=Yes`); interrogative *which, what, whose*.
- Words like *this, some, all, either* are determiners before a noun and stand alone as pronouns. UD English EWT splits them: standalone **demonstratives** are PRON (*I like this*: *this* PRON 427 times as subject, object or oblique in EWT train, *that* 288, *those* 48, *these* 29), while standalone **quantifiers** stay DET (*all* 149, *some* 65) (counted 2026-10-17; see `en/seeds/nominal/det-prontype.md`).

### 2.6 Auxiliaries and modals

A closed class of verbs that carry tense, aspect, mood, voice, negation and questions: *be, have, do* (as auxiliaries) and the **modals** *can, could, may, might, must, shall, should, will, would* (plus *ought, need, dare* in some uses). Their special syntax (the NICE properties: **N**egation with *not*, **I**nversion, **C**ode (ellipsis: *Yes, I can*), **E**mphasis) is described by Reynolds 2026 §5.3 ("The NICER properties of auxiliary verbs"). UD tags them **AUX**; PTB gives modals **MD** and the others the ordinary verb tags. The morphology is in [13-verbs.md](13-verbs.md) §§4–6.

### 2.7 Prepositions

- **Form**: invariable; mostly short and native (*at, by, for, from, in, of, on, to, with*), some complex (*because of, in front of, according to*).
- **Distribution**: before a noun phrase (*in the house*), a gerund (*before leaving*), or nothing (*come in, the book I was looking for*).
- Traditional grammar distinguishes prepositions (with a complement), adverbs (without: *come in*), particles in phrasal verbs (*give up*), and subordinating conjunctions (with a clause: *before he left*). CGEL and Reynolds 2026 §4.3 treat all of these as one class, prepositions, which may take an NP, a clause or no complement. UD splits them: ADP (*in the house*), ADV (*come in* in some uses, but EWT uses ADP with `compound:prt` for particles), SCONJ (*before he left*); PTB: IN for both prepositions and subordinators, RP for particles, RB for adverbs.

### 2.8 Conjunctions

- **Coordinators**: *and, or, but, nor, so, yet, for* (CC; UD CCONJ); correlative pairs *both … and, either … or, neither … nor, not only … but also*.
- **Subordinators**: *that, if, whether, because, although, while, when, since, as, until, unless* (PTB IN, WRB for *when, where*; UD SCONJ). They are a closed list (`en/seeds/verbal/subordinators-mark.md`).

### 2.9 Numerals

Cardinals (*one, two, 3, 1,000*: CD; UD NUM), ordinals (*first, 3rd*: JJ; UD ADJ with `NumType=Ord`), multiplicatives (*twice*: RB), fractions ([12](12-pronouns-determiners-numerals.md) §10).

### 2.10 Interjections and discourse words

*oh, ah, hey, wow, ouch, yes, no, hello, thanks, please, um, uh, lol*: UH; UD INTJ. In EWT 695 tokens of UH (0.34%), many from e-mail and reviews.

### 2.11 Other tags

| UD | PTB | what |
|---|---|---|
| PART | TO, POS, RB (*not*) | the infinitive marker *to* (TO, always PART: 3 285 tokens in EWT train; prepositional *to* is tagged IN, 1 861), the genitive *'s* (POS, 685), negation *not/n't* (RB, 1 692) |
| PROPN | NNP, NNPS | proper nouns |
| SYM | SYM, $, # | symbols, currency |
| PUNCT | `.` `,` `:` and others | punctuation ([07](07-punctuation-capitals-numbers.md) §1) |
| X | FW, GW, ADD, AFX, XX | foreign words, split pieces of a word (GW), addresses, affixes written apart, unknown |
| PRON | EX | existential *there*: PRON with `PronType=Dem` and relation `expl` (355 in EWT train) |
| — | PDT | predeterminers *all, both, half, such* before a determiner (UD DET) |

## 3. The tag sets side by side

The full mapping between Mova's PTB tags (`TAGS` in `en/build.rs`, 54 tags) and UD UPOS with the most important features:

| PTB | meaning | UPOS | FEATS (typical) | example |
|---|---|---|---|---|
| NN | common noun, singular or mass | NOUN | `Number=Sing` | *cat, water* |
| NNS | common noun, plural | NOUN | `Number=Plur` | *cats* |
| NNP | proper noun, singular | PROPN | `Number=Sing` | *London* |
| NNPS | proper noun, plural | PROPN | `Number=Plur` | *the Alps* |
| PRP | personal pronoun | PRON | `Case, Number, Person, PronType=Prs` | *I, him, it* |
| PRP$ | possessive pronoun (determiner) | PRON | `Case=Gen, Poss=Yes, PronType=Prs` + person, number, gender | *my, their* |
| WP | wh-pronoun | PRON | `PronType=Int,Rel` | *who, what* |
| WP$ | possessive wh-pronoun | PRON | `Poss=Yes` | *whose* |
| DT | determiner | DET | `PronType, Definite` | *the, a, this, some* |
| PDT | predeterminer | DET | | *all the, both the* (DET 173 times in EWT train) |
| WDT | wh-determiner | PRON (relative *that* 556, *which* 323) or DET (*what* 57, *which* 21 before a noun) | `PronType=Int,Rel` | *which, that* (relative) |
| EX | existential *there* | PRON | | *there is* |
| VB | verb, base form | VERB/AUX | `VerbForm=Inf` (or imperative, subjunctive) | *go, be* |
| VBP | verb, non-3rd singular present | VERB/AUX | `Mood=Ind, Tense=Pres, VerbForm=Fin` | *go, are* |
| VBZ | verb, 3rd singular present | VERB/AUX | + `Number=Sing, Person=3` | *goes, is* |
| VBD | verb, past | VERB/AUX | `Tense=Past, VerbForm=Fin` | *went, was* |
| VBN | past participle | VERB/AUX | `Tense=Past, VerbForm=Part` | *gone, been* |
| VBG | gerund or present participle | VERB/AUX | `VerbForm=Ger` or `Part` | *going* |
| MD | modal | AUX | `VerbForm=Fin` | *can, would* |
| JJ, JJR, JJS | adjective; comparative; superlative | ADJ | `Degree=Pos, Cmp, Sup` | *big, bigger, biggest* |
| RB, RBR, RBS | adverb; comparative; superlative | ADV | `Degree` | *fast, faster, fastest* |
| RP | particle | ADP (758 in EWT train), relation `compound:prt` | | *give up* |
| IN | preposition or subordinator | ADP or SCONJ | | *in, because* |
| TO | infinitival *to* | PART | | *to go* (prepositional *to* is IN, ADP) |
| CC | coordinator | CCONJ | | *and* |
| CD | cardinal number | NUM | `NumType=Card` | *three, 3* |
| UH | interjection | INTJ | | *oh* |
| POS | genitive ending | PART | | *'s* (685 in EWT train) |
| FW | foreign word | NOUN (58), X (29), ADV (15) by function | `Foreign=Yes` | *et al* |
| SYM, $, # | symbols | SYM | | *$, %* |
| LS | list item marker | NUM (117) | | *1.* |
| ADD | address | PROPN (249) or X (46) | | e-mail, URL |
| AFX | affix written as a separate token | X (35), ADV, NOUN | | *pre-* in *pre- and post-war* |
| GW | "goes with": part of a word split by a space | X (200) or the class of the whole word | | *e mail* |
| NFP | non-final punctuation, emoticons | PUNCT (234) or SYM (103) | | *:)* |
| HYPH | hyphen | PUNCT | | *-* |

Counts in the table are from UD English EWT 2.18 train (2026-10-17). The PTB definitions are Santorini 1990; the UD definitions are the UD 2 guidelines (https://universaldependencies.org/u/pos/ and https://universaldependencies.org/en/pos/). Mova's tagger outputs PTB tags, and UPOS and FEATS are derived from them by tables (`en/README.md`, "Tables for UPOS, FEATS").

## 4. Tests for the hard boundaries

| question | test | examples |
|---|---|---|
| noun or adjective (as a modifier)? | can it be compared or take *very*? can it be predicative? | *a stone wall* (*\*very stone*, *\*the wall is stone*? marginal): noun; *a golden ring*: adjective |
| adjective or past participle? | *very* (adj), *by*-phrase (participle), *seem* (adj), *un-* + verb impossible | *a very interested reader* (ADJ), *the window was broken by the boy* (VERB), *she seemed worried* (ADJ); `en/seeds/verbal/participle-or-adjective.md` |
| gerund or participle or noun? | determiner and adjective (noun) vs object and adverb (verb) | *the singing of birds* (NOUN), *singing songs loudly* (VERB, VBG); `en/seeds/verbal/gerund-participle.md` |
| preposition or particle? | the particle can move after the object | *look up the word / look the word up* (RP); *look up the chimney / \*look the chimney up* (IN) |
| preposition or subordinator? | NP vs clause after it | *after lunch* (ADP) vs *after we ate* (SCONJ) |
| *that*: determiner, pronoun, relative, subordinator? | replaceable by *this* (DT), by *which/who* (WDT relative), omissible before a content clause (IN) | *that book* (DT), *the book that I read* (WDT), *I know that he left* (IN) |
| *to*: infinitive marker or preposition? | followed by a verb base vs NP | *to go* (TO, PART), *to London* (IN, ADP) |
| *there*: existential or locative? | unstressed, takes agreement from the following NP | *There are cats* (EX) vs *Put it there* (RB) |
| adverb or adjective (flat forms)? | modifies a verb vs a noun | *drive fast* (RB) vs *a fast car* (JJ) |
| *like*: verb, preposition, conjunction, discourse marker? | | *I like it* (VB), *like a fish* (IN), *like I said* (IN, SCONJ), *it was, like, huge* (UH) |

## 5. Words in several classes

Conversion ([18-compounding-conversion-minor.md](18-compounding-conversion-minor.md) §3) makes multiple class membership normal. Of the frequent words in EWT train, *to* is TO 3 266 times and IN 1 861; *that* is IN 994, WDT 556, DT 471; *in* IN 2 956, RB 106, RP 37; *as* IN 759, RB 194; *on* IN 1 174, RB 47, RP 41; *'s* POS 550, VBZ (*is*) 329, VBZ (*has*) 15, PRP (*us*) 13; *do* VBP 517, VB 331 (all from `en/data/forms.tsv`, 2026-10-17).

Classic sets:

| word | classes |
|---|---|
| *round* | noun (*a round of drinks*), verb (*round the corner*), adjective (*a round table*), adverb (*turn round*), preposition (*round the world*) |
| *down* | preposition, adverb/particle, adjective (*feeling down*), noun (*goose down*; *first down*), verb (*down a drink*) |
| *back* | noun, verb, adjective, adverb |
| *light* | noun, verb, adjective (two: "not heavy", "not dark") |
| *still* | adverb, adjective, noun, verb |
| *fast* | adjective, adverb, verb, noun |
| *that, this* | determiner, pronoun; *that* also relative and subordinator |
| *before, after, since, until* | preposition, subordinator, adverb |
| *-ing* words | verb, noun (*a building*), adjective (*interesting*) |

## 6. How Mova assigns word classes

- **Tagger** (`en/src/ptag.rs`): an averaged perceptron over the word, its neighbours ±2, suffixes, prefixes, word shape, the dictionary in code and the two previous tags, with beam 8; unknown words get a TnT-style prior (`en/src/tag.rs`: a trigram Markov model with suffixes and the dictionary) and a class hint from the full AGID form expansion, worth +2.2–2.8 points on unknown words. Accuracy on UD test sets as documented in `en/README.md`: EWT 94.35%, PUD 94.04%, GUM 93.82% (PTB tags); with GUM converted to Mova's dialect and added to training, UPOS on EWT 95.02% and on GUM 96.57%.
- **UPOS and FEATS** are derived from PTB tags and context by tables.
- **Closed classes** are fixed lists in the seeds: auxiliaries (`en/seeds/verbal/aux-closed-list.md`), subordinators, determiners' `PronType` (`en/seeds/nominal/det-prontype.md`), and rules check their tags (`en expert-check`).
- **Known weak spots** (from the seeds of `en/seeds/errors/`): adjective vs participle, `compound` noun modifiers vs adjectives, particles vs prepositions, *that*, and the *-ing* form; `docs/school.md` tracks where the model fails.

## Sources for this chapter

- Reynolds 2026, ch. 3 (§§3.2–3.4: categories and functions; adjectives; pronouns and determinatives), ch. 4 (§4.2 adverbs, §4.3 prepositions), §5.3 (auxiliaries). https://zenodo.org/records/20612113
- Huddleston, Rodney & Geoffrey K. Pullum. 2002. *The Cambridge Grammar of the English Language.* Cambridge University Press. Not in the library (copyrighted); its analyses are reflected in Reynolds 2026 and in the CGELBank treebank (Reynolds et al., CC BY 4.0; raw copy `$MOVA_DATA/raw/en-gram-cgelbank`). Listed under "wanted".
- Santorini 1990, PTB tagging guidelines. https://repository.upenn.edu/cis_reports/570/
- Universal Dependencies: universal POS tags https://universaldependencies.org/u/pos/ ; English-specific notes https://universaldependencies.org/en/pos/
- Mova: `en/src/ptag.rs`, `en/src/tag.rs`, `en/build.rs`, `en/README.md`; seeds in `en/seeds/nominal/`, `en/seeds/verbal/`, `en/seeds/errors/`; counts from `en/data/forms.tsv` (2026-10-17).
