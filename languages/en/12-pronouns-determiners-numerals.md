# 12. Pronouns, determiners and numerals

Pronouns, determiners and numerals are small closed classes with the richest inflection left in English: the personal pronouns keep case (*I/me/my/mine*), person, number and gender. This chapter gives the full paradigms and the main points of use; the syntax of reference, agreement and relative clauses is in the syntax chapters (chapters [20](20-syntax-overview.md)–38).

The UD features named here are those of UD English EWT 2.18, with token counts from its train set (counted 2026-10-17). Mova's seeds `en/seeds/morph/pron-*.md`, `det-articles.md`, `num-ordinal-mult.md` and `en/seeds/nominal/*.md` hold the rules and detailed references.

## 1. Overview

| class | members | PTB | UPOS and main FEATS |
|---|---|---|---|
| personal pronouns | *I, you, he, she, it, we, they* and their case forms | PRP | PRON, `PronType=Prs`, `Case`, `Person`, `Number`, `Gender` |
| possessive determiners | *my, your, his, her, its, our, their* | PRP$ | PRON, `Case=Gen`, `Poss=Yes` |
| possessive pronouns | *mine, yours, his, hers, ours, theirs* | PRP | PRON, `Poss=Yes` (no `Case`) |
| reflexive / intensive | *myself, yourself, himself, herself, itself, ourselves, yourselves, themselves, oneself* | PRP | PRON, `Reflex=Yes`, `Case=Acc` |
| reciprocal | *each other, one another* | DT + JJ / CD + DT | PRON-like, `PronType=Rcp` |
| demonstratives | *this, that, these, those* | DT | DET (before a noun), PRON (alone), `PronType=Dem`, `Number` |
| interrogative / relative | *who, whom, whose, what, which, that* (rel.), *whoever…* | WP, WP$, WDT | PRON or DET, `PronType=Int` or `Rel` |
| indefinite pronouns | *someone, anybody, everything, nothing, none, one* | NN | PRON, `PronType=Ind, Tot, Neg` |
| articles | *the, a, an* | DT | DET, `PronType=Art`, `Definite=Def/Ind` |
| quantifiers | *some, any, no, every, each, all, both, either, neither, many, much, few, little, several, enough, another* | DT, JJ, PDT | DET, `PronType=Ind, Tot, Neg` |
| numerals | *one, two, … first, second, … once, twice* | CD; JJ; RB | NUM (`NumType=Card`); ADJ (`NumType=Ord`); ADV (`NumType=Mult`) |

## 2. Personal pronouns

### 2.1 The paradigm

| person, number, gender | subject (nominative) | object (accusative) | possessive determiner | possessive pronoun | reflexive |
|---|---|---|---|---|---|
| 1 sg | *I* | *me* | *my* | *mine* | *myself* |
| 2 sg | *you* | *you* | *your* | *yours* | *yourself* |
| 3 sg masc | *he* | *him* | *his* | *his* | *himself* |
| 3 sg fem | *she* | *her* | *her* | *hers* | *herself* |
| 3 sg neut | *it* | *it* | *its* | (*its*, rare) | *itself* |
| 1 pl | *we* | *us* | *our* | *ours* | *ourselves* |
| 2 pl | *you* | *you* | *your* | *yours* | *yourselves* |
| 3 pl | *they* | *them* | *their* | *theirs* | *themselves* (sg. *themself*) |
| generic | *one* | *one* | *one's* | — | *oneself* |
| archaic 2 sg | *thou* | *thee* | *thy, thine* | *thine* | *thyself* ([09](09-early-modern-and-archaic.md) §2) |

Sweet NEG I §§1076–1114 gives the paradigm and its history; *its* spread only at the end of the Early Modern period, replacing *his* for neuter possessors (§1101). UD English (EWT) features: *I* `Case=Nom|Number=Sing|Person=1|PronType=Prs` (3 377 tokens), *me* `Case=Acc` (597), *my* `Case=Gen|Poss=Yes` (895), *mine* `Poss=Yes` without `Case` (18), *myself* `Case=Acc|Reflex=Yes` (16). **_You_ has no `Number`** in EWT (1 639 tokens of *you* with `Case=Nom|Person=2|PronType=Prs`), since the form does not show it; *it* has `Gender=Neut`.

### 2.2 Case

The **nominative** is used for subjects of finite verbs; the **accusative** for objects, after prepositions, and in most other positions in informal English:

| context | formal / prescriptive | usual informal | note |
|---|---|---|---|
| after *be* | *It is I. This is she.* | *It's me. That's her.* | the accusative is standard in speech |
| after *than, as* | *taller than I (am)* | *taller than me* | both standard; the nominative implies a clause |
| coordinated objects | *between you and me* | *between you and I* (hypercorrect) | |
| coordinated subjects | *My friend and I went* | *Me and my friend went* | informal, stigmatized in writing |
| alone | *Who did it? – I.* (formal) | *Me.* | |
| before a gerund | *I dislike his smoking* | *I dislike him smoking* | both used |

Mova's seeds `en/seeds/nominal/pron-case-function.md`, `you-it-case.md` (the forms *you, it* that do not show case get it from their function) and `en/seeds/morph/pron-personal-case.md` state the annotation rules; `en/seeds/errors/subject-pronoun-case.md` and `pronoun-after-preposition.md` cover learner errors.

### 2.3 Gender and singular *they*

*He* and *she* are used for people (and pets) by sex or gender identity, *it* for things, animals, and sometimes babies. For a person whose gender is unknown or irrelevant, or who uses it as their pronoun, **singular *they*** is used: *Someone left their umbrella; they'll be back for it.* It takes plural verb agreement (*they are*) even with singular meaning; the reflexive is *themselves* or *themself*. Singular *they* after indefinite antecedents (*everyone, someone, a person*) goes back to Middle English; its use for a specific known person is recent. EWT annotates *they* `Number=Plur` in all uses.

### 2.4 Second person plural

Standard English does not distinguish singular and plural *you*. Varieties fill the gap: *you guys* (general American, increasingly British), *y'all* (US South, African American English), *youse / yous* (Ireland, Scotland, Liverpool, Australia, Philadelphia), *ye* (Ireland), *you lot* (BrE), *yinz* (Pittsburgh), *you all*.

### 2.5 Generic and impersonal pronouns

- ***you*** (informal): *You never know.*
- ***one*** (formal): *One must be careful.* The possessive is *one's*, reflexive *oneself*. AmE formal style may continue with *he* or *his* (*One must do his best*), now dated.
- ***they*** (people in general, authorities): *They say it will rain. They're building a new road.*
- ***we*** (inclusive, authorial): *As we saw in chapter 2…*

### 2.6 *It*

Besides reference, *it* is a dummy (expletive) subject: weather and time (*It's raining, it's late*), extraposition (*It is clear that…*), clefts (*It was John who called*). UD marks expletive *it* with the relation `expl` (`en/seeds/verbal/expletive-it.md`); the morphology is the same.

## 3. Possessives

The possessive **determiner** precedes a noun (*my book*); the possessive **pronoun** stands alone (*the book is mine*; *mine is red*; *a friend of mine*). Only *his* and *its* have the same form in both functions; *its* as a standalone pronoun is rare. None of them takes an apostrophe (*its, hers, ours, yours, theirs*): *it's* is *it is/has* ([07](07-punctuation-capitals-numbers.md) §3). *Own* reinforces a possessive (*my own room, a room of my own*; `en/seeds/nominal/own.md`). Dialectal *hisn, hern, ourn, yourn, theirn* (Midlands, southern England, Appalachia) are analogical forms after *mine, thine*.

## 4. Reflexive and intensive pronouns

The *-self/-selves* forms have two uses:

- **Reflexive** (the object is the subject): *She hurt herself. They introduced themselves.* Some verbs are inherently reflexive or nearly so: *pride oneself on, avail oneself of, behave (oneself), absent oneself*.
- **Intensive (emphatic)**: *The president himself called. I did it myself.* UD gives these `PronType=Emp` beside `Reflex=Yes` in some treebanks; EWT keeps `Reflex=Yes`.

Their stems mix cases: *my-, your-, our-* are possessive, *him-, them-, it-* objective; non-standard *hisself, theirselves* regularize the system on the possessive pattern (Sweet NEG I §§1109–1114; seed `en/seeds/morph/pron-reflexive.md`). After prepositions of place the plain pronoun is usual: *She looked around her* (not \**herself*); *He put the box beside him*.

## 5. Reciprocals

*each other* and *one another*, now interchangeable (the old rule "*each other* for two, *one another* for more" is not observed). They have a genitive: *each other's houses*. UD treats them as a fixed expression (`fixed`) with `PronType=Rcp` (seed `en/seeds/nominal/reciprocal.md`, after Poutsma GLME IV ch. XL §§37–39).

## 6. Demonstratives

| | near | far |
|---|---|---|
| singular | *this* | *that* |
| plural | *these* | *those* |

They agree in number with the noun (*this book / these books*; a frequent check in Mova's seeds: `en/seeds/nominal/dem-number.md`, after the BLiMP agreement pairs). Before a noun they are DET, alone they are PRON in EWT (*this* PRON 427 times as subject, object or oblique in train; DET 711 times with `Number=Sing|PronType=Dem`). Informal *this* introduces new referents in narratives (*There was this guy…*); *them* as a demonstrative (*them books*) is non-standard ([08](08-national-varieties.md) §12); *yon, yonder* are archaic or dialectal.

*That* is also a relative pronoun and a subordinator (§7, [10-word-classes.md](10-word-classes.md) §4); *so* and *such* act as pro-forms (*I think so; such people*).

## 7. Interrogative and relative pronouns

| | person | thing | choice from a set | possessive |
|---|---|---|---|---|
| interrogative | *who* (subj.), *whom* (obj., formal) | *what* | *which* | *whose* |
| relative | *who, whom* | *which* | — | *whose* (persons and things) |
| relative (any) | *that*, zero | *that*, zero | | |
| free relative / indefinite | *whoever, whomever* | *whatever* | *whichever* | |

Notes:

- ***Whom*** survives in formal writing and directly after a preposition (*to whom, with whom*); elsewhere *who* is usual (*Who did you see?*). Sweet NEG I §1086 already noted that *whom* "is disappearing from speech". EWT has *whom* 13 times against *who* 311 (`PronType=Rel` for both in their most frequent use).
- ***Which*** vs ***what*** as determiners: *which* for a known, limited set (*Which colour do you want, red or blue?*), *what* for an open set (*What colour is it?*).
- ***Whose*** is used for things as well as persons: *a house whose roof leaks*.
- **Relative *that*** is a pronoun in UD (`PronType=Rel`, WDT; 556 tokens in EWT train) though many grammars (CGEL, Reynolds 2026) treat it as a subordinator; it cannot follow a preposition (\**the house in that I live*) and is avoided in non-restrictive clauses.
- The **zero relative** (*the man I saw*) is possible when the relative is not the subject.

UD features: `PronType=Int` for interrogatives, `PronType=Rel` for relatives; seed `en/seeds/morph/pron-wh.md` (UD PRON documentation, Santorini 1990 §2 and §4.1, Sweet NEG I §§211, 216, 1086); `en/seeds/nominal/who-whom-whose.md`, `relative-that.md`.

## 8. Articles

### 8.1 Forms

| article | form | before | examples |
|---|---|---|---|
| definite | *the* /ðə/ | consonant sound | *the cat, the university* /ðə ˌjuː…/ |
| | *the* /ði/ | vowel sound; emphatic /ðiː/ | *the apple, the hour*; *THE place to be* |
| indefinite | *a* /ə/ (strong /eɪ/) | consonant sound | *a cat, a university, a one-off, a European* |
| | *an* /ən/ (strong /æn/) | vowel sound | *an apple, an hour, an MP, an X-ray, an honest man* |

EWT: *the* `Definite=Def|PronType=Art` (9 064 tokens in train), *a* `Definite=Ind|PronType=Art` (3 740), *an* (496).

### 8.2 *A* or *an*

The choice depends on the **sound** of the next word, not its letter: *a unit* /j/, *a one-way street* /w/, *an hour, an heir, an honest, an honour* (silent *h*), *an MBA, an FBI agent, an SOS* (letter names beginning with a vowel sound: *M* /em/, *F* /ef/, *S* /es/), *a UN report* (*U* /juː/). Before an unstressed syllable beginning with /h/ older British usage had *an historic(al), an hotel, an hypothesis*; today *a historic* prevails, and *an historic* persists in formal British usage. Sweet NEG I §1137, Whitney §§220–221 and Kruisinga II.2 §§1305–1307 record the state of their time (*an hotel, an historical*; survivals like *an unit, an useless*); seeds `en/seeds/morph/det-articles.md`, `en/seeds/nominal/a-an-sound.md`.

### 8.3 Use in outline

The article system (definite, indefinite, zero) belongs to syntax and semantics and is treated in [22](22-phrases.md) and [35](35-reference-and-coreference.md). In outline: *the* marks a referent the hearer can identify; *a/an* introduces a new countable referent or classifies (*She is a doctor*); the **zero article** is used with plural and mass nouns in a generic or indefinite sense (*Cats like milk*), with most proper names, and in many fixed phrases (*by bus, at school, in bed, go to church, at night*). The choice is the most frequent single error of learners whose languages have no articles (seed `en/seeds/errors/missing-article.md`).

## 9. Quantifiers and other determiners

| word | count sg | count pl | mass | as pronoun | UD PronType |
|---|---|---|---|---|---|
| *some* (/səm/ weak, /sʌm/ "a certain") | *some guy* (informal) | *some books* | *some water* | *some* | Ind |
| *any* | *any book* | *any books* | *any water* | *any* | Ind |
| *no* / *none* | *no book* | *no books* | *no water* | *none* | Neg |
| *every* | *every book* | — | — | — (*everyone*) | Tot |
| *each* | *each book* | — | — | *each* | Tot |
| *all* | (*all day*) | *all books* | *all water* | *all* | Tot |
| *both* | — | *both books* (two) | — | *both* | Tot |
| *either, neither* | *either book* (of two) | — | — | *either* | Ind, Neg |
| *many / much* | — | *many books* | *much water* | yes | Ind (JJ in PTB) |
| *few / little* | — | *(a) few books* | *(a) little water* | yes | Ind |
| *several* | — | *several books* | — | yes | Ind |
| *enough* | — | *enough books* | *enough water* | yes | Ind |
| *another* | *another book* | — | — | *another* | Ind |
| *such* | *such a book* | *such books* | *such water* | | Ind (PDT before *a*) |

Notes:

- ***Some*** vs ***any***: *some* in affirmative statements and in offers and requests (*Would you like some tea?*); *any* in negatives, questions and conditionals (*I don't have any*), and in the free-choice sense "it doesn't matter which" (*Take any card*). The same split holds in *someone/anyone, somewhere/anywhere*.
- ***No*** is a determiner (*no money*), ***none*** a pronoun (*none of them, I have none*); *none* takes a singular or plural verb (*none of them is/are*) (seed `en/seeds/nominal/no-none.md`).
- **Predeterminers** precede another determiner: *all the books, both my parents, half the time, double the price, such a day, what a mess, many a man* (PTB PDT; UD DET with relation `det:predet`; EWT has PDT 173 times; seed `en/seeds/nominal/predeterminers.md`).
- ***A few, a little*** (some) vs ***few, little*** (not many, not much): *a few friends* (positive) vs *few friends* (negative).
- **Comparison of quantifiers**: *many/much → more → most*, *few → fewer → fewest*, *little → less → least* ([15](15-adjectives-adverbs.md) §3). Prescriptively *fewer* with count and *less* with mass nouns; *less* with plurals (*less people*) is common in speech.

## 10. Numerals

### 10.1 Cardinals

*zero (nought, BrE), one, two, three, four, five, six, seven, eight, nine, ten, eleven, twelve, thirteen … nineteen, twenty, twenty-one … thirty, forty* (not *fourty*), *fifty, sixty, seventy, eighty, ninety, a/one hundred, a/one thousand, a/one million, a/one billion*.

- Tens and units are hyphenated: *forty-two*. BrE puts *and* after hundreds: *three hundred and five*; AmE often omits it.
- ***Hundred, thousand, million, dozen*** take no *-s* after a numeral or *several, a few*: *three hundred people, several thousand years*; they take *-s* as nouns of indefinite quantity: *hundreds of people, thousands of years, dozens of times* (seed `en/seeds/nominal/hundred-thousand.md`; Poutsma GLME IV ch. XLII §§1–2).
- ***One*** is also a pro-form for a count noun: *the red one, a big one, these ones* (BrE and informal AmE; `en/seeds/nominal/prop-word-one.md`).
- PTB tags cardinals CD; UD NUM with `NumType=Card`; in EWT the most frequent CD form is *one* (counted in `en/data/forms.tsv`).

### 10.2 Ordinals

*first, second, third* (suppletive), then the cardinal + *-th*: *fourth, fifth* (*v* → *f*), *sixth, seventh, eighth* (one *t*), *ninth* (no *e*), *tenth, eleventh, twelfth* (*v* → *f*), *thirteenth … twentieth* (*-y* → *-ieth*), *twenty-first* (only the last element becomes ordinal), *hundredth, thousandth, millionth*. Written with figures: *1st, 2nd, 3rd, 4th, 21st, 22nd, 23rd, 101st, 111th*. Sweet NEG I §§1170–1176 and Whitney §§215–218; Jespersen MEG VI 24.4 discusses *-th* in ordinals. PTB tags ordinals JJ; UD ADJ with `NumType=Ord` (seed `en/seeds/morph/num-ordinal-mult.md`; `en/seeds/nominal/ordinals.md`).

### 10.3 Fractions, multiplicatives, collectives

- Fractions use ordinals: *a third, two-thirds, three-fifths*; but *a half, a quarter* (BrE; AmE also *a fourth*).
- Multiplicatives: *once, twice*, archaic *thrice*, then *three times*; *single, double, triple, quadruple*; *twofold, threefold* (UD `NumType=Mult`).
- Collective numerals: *a pair, a couple, a dozen, a score* (20, archaic), *a gross* (144), *a brace* (of game birds).
- Distributives: *one by one, two at a time, each*.

## 11. How Mova handles these words

- **Closed lists.** Pronouns, determiners and numerals are among the most frequent lemmas and are compiled as word constants (`en/build.rs` writes constants for the 5 000 most frequent lemmas; *I, you, the, a* among them), so rules in code compare integers.
- **Lemmas.** UD English EWT lemmatizes the object forms of personal pronouns to the nominative (*me → I* 597 times in train, *them → they* 347, *us → we* 169), keeps the possessive determiners and reflexives as their own lemmas (*my → my, his → his, her → her* when possessive, *themselves → themselves*), and maps possessive pronouns to the determiner (*mine → my* 18, *yours → your* 4); Mova follows EWT because UD counts take precedence in `Morph::lemmatize`. The clitic *'s* in *let's* is lemmatized *we* (13 tokens in EWT train).
- **Features.** Case, person, number, gender and `PronType` come from the FEATS tables learned from EWT and are checked by rules in the seeds (`pron-personal-paradigm.md`, `pron-possessive.md`, `pron-reflexive.md`, `det-prontype.md`).
- **a / an.** Not checked (no pronunciation component; [01-consonants.md](01-consonants.md) §6).
- **Numbers.** Cardinal words and figures are CD/NUM; their numeric value is read by the `math` crate for word problems, not by `en`.

## Sources for this chapter

- Sweet 1891, NEG I §§1076–1176 (pronouns, articles, numerals), §§211, 216 (interrogative and relative words). https://archive.org/details/newenglishgramma01sweeuoft
- Whitney 1877, §§155–160, 215–221. https://archive.org/details/essentialsofengl01whit
- Jespersen 1942, MEG VI 24.4 (ordinals). https://archive.org/details/modernenglishgra06jesp
- Kruisinga 1931–1932, *Handbook* II.2 §§1305–1307 (*a/an*). https://archive.org/details/handbookofpresen22krui
- Poutsma 1914–1929, GLME vols. I, III, IV (predeterminers, *no/none*, *one*, reciprocals, *who/whom*). https://archive.org/details/grammaroflatemod01poutuoft
- Reynolds 2026, §3.4 (pronouns, determinatives, definiteness). https://zenodo.org/records/20612113
- Universal Dependencies, English pronouns and determiners: https://universaldependencies.org/en/pos/PRON.html , https://universaldependencies.org/en/pos/DET.html ; features `PronType`, `Case`, `Poss`, `Reflex`, `NumType`: https://universaldependencies.org/u/feat/
- Santorini 1990, PTB tagging guidelines. https://repository.upenn.edu/cis_reports/570/
- Mova: seeds `en/seeds/morph/pron-*.md`, `det-articles.md`, `num-ordinal-mult.md`, `en/seeds/nominal/*.md`; counts from UD English EWT 2.18 train (2026-10-17).
