# 22. Phrases: noun, adjective, adverb and prepositional phrases

UD has no phrase nodes, but phrases are still there: a phrase is a head with everything that depends on it. This chapter describes the English noun phrase (determiners, possessives, numerals, adjectives, noun modifiers, names, apposition, post-modifiers), quantity expressions and partitives, the adjective phrase with its complements and comparatives, the adverb phrase, and the prepositional phrase, each with its UD analysis, the Mova seeds that check it, and the places where Mova's parser and frame reader stand.

Contents

1. [Phrases in a dependency tree](#1-phrases-in-a-dependency-tree)
2. [The noun phrase](#2-the-noun-phrase)
3. [Quantity: numerals, quantifiers, partitives](#3-quantity-numerals-quantifiers-partitives)
4. [The adjective phrase](#4-the-adjective-phrase)
5. [Comparison](#5-comparison)
6. [The adverb phrase](#6-the-adverb-phrase)
7. [The prepositional phrase](#7-the-prepositional-phrase)
8. [Phrases and ambiguity: where parses go wrong](#8-phrases-and-ambiguity-where-parses-go-wrong)
9. [How Mova reads a noun phrase](#9-how-mova-reads-a-noun-phrase)

---

## 1. Phrases in a dependency tree

A phrase in UD is the **projection** of a word: the word and all its descendants. *All my three old college friends* is the subtree of *friends*. The type of the phrase is the type of its head, and its function is the relation from the head to the outside: `nsubj`, `obj`, `obl`, `nmod`… (UD guidelines `_u-overview/syntax.md`, https://universaldependencies.org/u/overview/syntax.html).

The main word-level relations inside phrases:

| inside a … | relation | dependent | example |
|---|---|---|---|
| noun phrase | `det` | article, demonstrative, quantifier | ***the** pigs*, ***every** day* |
| | `det:predet` | predeterminer | ***all** the pigs* |
| | `nmod:poss` | possessor | ***my** brothers*, ***the man's** beard* |
| | `nummod` | cardinal number | ***three** pigs* |
| | `amod` | adjective, participle | ***little** pigs*, ***gritted** roads* |
| | `compound` | noun modifier | ***college** friends* |
| | `nmod` | PP or noun modifying a noun | *the King **of Hearts*** |
| | `nmod:unmarked` | bare noun modifier | *Dallas, **Texas*** |
| | `nmod:desc` | bare title before a name | ***Aunt** Toni* |
| | `flat` | rest of a headless name | *Jane **Smith*** |
| | `appos` | apposition | *Mr. Brown, **our teacher*** |
| | `acl`, `acl:relcl` | clause on a noun | *sheep **driven away***, *the book **I read*** |
| | `case` | possessive *'s* | *man **'s*** |
| adjective phrase | `advmod` | degree, other adverb | ***very** proud* |
| | `obl` | PP complement | *proud **of her son*** |
| | `ccomp`, `xcomp`, `advcl` | clause complement | *glad **that…***, *eager **to go***, *too tired **to give…*** |
| adverb phrase | `advmod` | degree | ***much** faster* |
| prepositional phrase | `case` | the preposition, on the noun | ***in** the hall* |

## 2. The noun phrase

### 2.1 Order of pre-modifiers

English noun phrases have a fixed order before the head (Mova seed [../../en/seeds/nominal/np-order.md](../../en/seeds/nominal/np-order.md), after Poutsma 1904, *A Grammar of Late Modern English*, `books/p/po/poutsma-1904-grammar-late-modern-english`, and Curme 1931, vol. III, ch. V, `books/c/cu/curme-1931-grammar-english-language-vol`):

| predeterminer | central determiner or possessor | numeral (ordinal before cardinal) | adjectives | noun modifiers | **head** |
|---|---|---|---|---|---|
| *all* | *my* | *three* | *old* | *college* | ***friends*** |
| *both* | *the* | *first two* | *little* | — | ***pigs*** |
| `det:predet` | `det` / `nmod:poss` | `amod` (ordinal) / `nummod` | `amod` | `compound` | — |

A noun has at most one central determiner (*\*the my book*; seed [../../en/seeds/nominal/single-det.md](../../en/seeds/nominal/single-det.md)); determiners precede the noun (seed [../../en/seeds/nominal/det-before-head.md](../../en/seeds/nominal/det-before-head.md)). Adjectives intensified by *so, as, too, how* go before the indefinite article: *so big a house*, *too good a chance* (seed [../../en/seeds/nominal/adj-before-article.md](../../en/seeds/nominal/adj-before-article.md)); *such, what, quite, rather* stand before *a*: *such a day*, *what a pig*.

(1) «Mova en 0.4, CPU, 2026-10-17» — *All the three little pigs* is analysed correctly inside the phrase:

```conllu
1  All     DET   5  det:predet
2  the     DET   5  det
3  three   NUM   5  nummod
4  little  ADJ   5  amod
5  pigs    NOUN  0  root
6  built   VERB  5  acl
7  houses  NOUN  6  obj
```

…but the sentence *All the three little pigs built houses.* is misparsed as a single noun phrase with a reduced relative (*pigs [that] built houses*): *pigs* is the root and *built* its `acl`. A long, fully pre-modified subject followed by a verb that can also be a past participle is the classic noun-phrase/clause ambiguity (§8).

### 2.2 Determiners

English determiners are a closed class (seed [../../en/seeds/nominal/det-prontype.md](../../en/seeds/nominal/det-prontype.md)):

| type (`PronType`) | members | note |
|---|---|---|
| article (`Art`) | *the*, *a/an* | *an* is a form of *a* (one lemma; seed [../../en/seeds/nominal/articles.md](../../en/seeds/nominal/articles.md)) |
| demonstrative (`Dem`) | *this, that, these, those* | the only determiners with number (seed [../../en/seeds/nominal/dem-number.md](../../en/seeds/nominal/dem-number.md)) |
| total (`Tot`) | *all, every, each, both* | *every, each* take a singular noun (seed [../../en/seeds/nominal/distributive-singular.md](../../en/seeds/nominal/distributive-singular.md)) |
| indefinite (`Ind`) | *some, any, another, either, such, many, much, few, several* | *many/few* + plural, *much/little* + mass (seed [../../en/seeds/nominal/quantity-adj-number.md](../../en/seeds/nominal/quantity-adj-number.md)) |
| negative (`Neg`) | *no, neither* | *no* only before a noun; *none* stands alone (seed [../../en/seeds/nominal/no-none.md](../../en/seeds/nominal/no-none.md)) |
| interrogative / relative (`Int`, `Rel`) | *what, which, whatever, whichever* | 25, 26 |

*All, both, half* before another determiner are `det:predet` (seed [../../en/seeds/nominal/predeterminers.md](../../en/seeds/nominal/predeterminers.md)). A floating quantifier is attached to the noun it quantifies, not to the verb next to it: *The five companies … all proposed* → `det(companies, all)` (UD `_en/specific-syntax.md`, *Determiners*).

### 2.3 Possessives

The possessor is `nmod:poss` of the possessed noun; the clitic *'s* is `case` of the possessor (UD `_en/dep/nmod-poss.md`; seeds [../../en/seeds/nominal/nmod-poss.md](../../en/seeds/nominal/nmod-poss.md), [../../en/seeds/nominal/poss-clitic.md](../../en/seeds/nominal/poss-clitic.md)). Possessive pronouns (*my, your, thy*) are also `nmod:poss`; the Mova dialect normalises any `det:poss` to `nmod:poss` ([../../docs/dialect.md](../../docs/dialect.md)).

(2) `jps:DEU:585` *Thou shalt not see thy brother's ox or his sheep driven away…* (ID FORM UPOS HEAD DEPREL):

```conllu
5   thy      PRON  6   nmod:poss
6   brother  NOUN  8   nmod:poss
7   's       PART  6   case
8   ox       NOUN  4   obj
9   or       CCONJ 11  cc
10  his      PRON  11  nmod:poss
11  sheep    NOUN  8   conj
12  driven   VERB  11  acl
```

Possessive structures in detail:

| construction | example | analysis | seed |
|---|---|---|---|
| group genitive | *the King of Hearts's soldiers* | `nmod:poss(soldiers, King)`, `nmod(King, Hearts)`, `case(King, 's)` — *'s* follows the group but attaches to its head | [group-genitive](../../en/seeds/nominal/group-genitive.md) |
| absolute genitive | *at the baker's*; *My car is faster than John's* | the genitive is the head of its phrase | [absolute-genitive](../../en/seeds/nominal/absolute-genitive.md) |
| double genitive | *a friend of my father's*; *a friend of mine* | `nmod(friend, father)` with `case` *of* and *'s* | [double-genitive](../../en/seeds/nominal/double-genitive.md) |
| classifying genitive | *a children's story*, *two weeks' notice* | `nmod:poss`, but the meaning is a kind, like an adjective | [classifying-genitive](../../en/seeds/nominal/classifying-genitive.md) |
| genitive with a gerund | *I appreciate your coming here* | `nmod:poss(coming, your)` | UD `_en/specific-syntax.md` |
| absolute possessive pronoun | *This book is mine.* | *mine* heads its own phrase | [absolute-possessives](../../en/seeds/nominal/absolute-possessives.md) |
| *own* | *my own house* | `amod(house, own)` after a possessor | [own](../../en/seeds/nominal/own.md) |

In (3) Mova's parser builds the group genitive right; the only fault is the lemma of the name *Hearts* (*heart*), which the morphology chapters discuss.

(3) *The King of Hearts's soldiers painted the roses red.* «Mova en 0.4, CPU, 2026-10-17»: `nmod:poss(soldiers, King)`, `nmod(King, Hearts)`, `case(Hearts, of)`, `case(King, 's)`.

The choice between *'s* and *of* follows animacy and weight: the *s*-genitive prevails with living beings, *of* with inanimates (Curme 1931, vol. III, ch. V; seed [../../en/seeds/nominal/nmod-poss.md](../../en/seeds/nominal/nmod-poss.md)).

### 2.4 Adjectival modifiers

Attributive adjectives and participles are `amod`; predicative adjectives are the head of their clause with `cop`, or `xcomp` (seed [../../en/seeds/nominal/predicative-not-amod.md](../../en/seeds/nominal/predicative-not-amod.md)). Position:

| position | when | example | seed |
|---|---|---|---|
| before the noun | a bare adjective (Curme: "adherent") | *a sick man* | [adj-position](../../en/seeds/nominal/adj-position.md) |
| after the noun | with its own complement; coordinated; after *-thing/-body/-one*; some set phrases | *a man proud of his son*; *something new*; *the heir apparent* | [adj-position](../../en/seeds/nominal/adj-position.md), [indefinite-compounds](../../en/seeds/nominal/indefinite-compounds.md) |
| never before the noun | *a-* adjectives | *the child is asleep* / *a sleeping child* | [a-adjectives](../../en/seeds/nominal/a-adjectives.md) |
| only before the noun | *main, mere, sheer, former*, relational adjectives | *a medical school* | [attributive-only](../../en/seeds/nominal/attributive-only.md) |

A post-nominal adjective with a complement is still `amod` in UD English (*a man **proud** of his son*: `amod(man, proud)`, `obl(proud, son)`). Participles before the noun are `amod` (*gritted winter roads*, `school:chemistry:013-rust-irons-slow-fire.lesson:12`: `amod(roads, gritted)`, `compound(roads, winter)`); participles with dependents after the noun are reduced relatives, `acl` (§2.8; 25).

### 2.5 Noun modifiers: compounds

A noun before a noun is `compound` of the head, which is the last noun: *phone book*, *sea spray*, *winter roads* (UD `_en/dep/compound.md`; seed [../../en/seeds/nominal/compound.md](../../en/seeds/nominal/compound.md)). The modifier noun is usually singular even with plural meaning — *toothbrush*, *a five-foot rope*, *a ten-year-old boy* (seed [../../en/seeds/nominal/adnominal-noun-singular.md](../../en/seeds/nominal/adnominal-noun-singular.md)). Compounds stack to the right: in *Middle Office work*, *Office* is `compound` of *work* and *Middle* is `amod` of *Office* (EWT 2.18 `email-enronsent02_01-0023`). Whether a compound is spelled as one word, hyphenated or open is a matter of morphology; the semantic transparency of English compounds is studied by Schäfer (2018, *The semantic transparency of English compound nouns*, `books/s/sc/schafer-2018-semantic-transparency-english-compound`).

### 2.6 Names, titles and descriptors

A personal name has no internal head: the first word is the head and the rest are `flat` (UD `_en/dep/flat.md`; seed [../../en/seeds/nominal/flat-names.md](../../en/seeds/nominal/flat-names.md)). Names whose first word is a type (*Lake Mead*, *Mount Everest*) and numbered identifiers (*page 394*, *Route 66*) are also `flat`; EWT records the kind in MISC as `FlatType` (425 tokens; [../../treebanks/en/ewt/seeds.md#mova-diff](../../treebanks/en/ewt/seeds.md#mova-diff)).

A bare title or role before a name (*President Obama*, *Aunt Toni*, *Dr. Smith*) is `nmod:desc` of the name; with a determiner it is a full apposition (*the poet Burns*, *my friend Nick*) (UD `_en/dep/nmod-desc.md`; seed [../../en/seeds/nominal/nmod-desc.md](../../en/seeds/nominal/nmod-desc.md)).

(4) EWT 2.18 `email-enronsent01_01-0042` *Any news on Aunt Toni?* — `nmod:desc(Toni, Aunt)`, `nmod(news, Toni)`, `case(Toni, on)`.

(5) *Mr. Brown, our teacher, met Dr. Jane Smith every day last week.* «Mova en 0.4, CPU, 2026-10-17»: `nmod:desc(Brown, Mr.)`, `appos(Brown, teacher)`, `nmod:desc(Jane, Dr.)`, `flat(Jane, Smith)` — all as in EWT.

Company suffixes and titles after a name (*Inc.*, *Jr.*) are treated by seed [../../en/seeds/errors/titles-and-company-suffixes.md](../../en/seeds/errors/titles-and-company-suffixes.md); multiword adjectives inside names by [../../en/seeds/errors/adjectives-in-names.md](../../en/seeds/errors/adjectives-in-names.md).

### 2.7 Apposition

An appositive is a second noun phrase that names the same thing as the first: `appos` from the first to the second (UD `_en/dep/appos.md`; seed [../../en/seeds/nominal/appos.md](../../en/seeds/nominal/appos.md)). Poutsma distinguishes identity (*Mr. Lloyd George, the late Prime Minister*), species with genus (*the planet Mars*) and quantity with substance (Poutsma 1904; seed). UD keeps `appos` for the loose, comma-set kind; the close kind with a bare title is `nmod:desc` (§2.6), and abbreviations in brackets are `appos` (*the Australian Broadcasting Corporation (ABC)*).

(6) EWT 2.18 `weblog-typepad.com_ripples_20040407125600_ENG_20040407_125600-0062` *Take care, my friend, Linda* — `vocative(Take, friend)`, `appos(friend, Linda)`.

Apposition is a frequent relation in Mova's parse database (87 220 tokens, 1.08%, against 0.66% in EWT; 20 §5.2): dictionary glosses and school definitions are full of *X, a kind of Y* phrases.

### 2.8 Post-modifiers

| post-modifier | example | UD | chapter |
|---|---|---|---|
| prepositional phrase | *the King **of Hearts***, *the cat **on the mat*** | `nmod` + `case` | §7 |
| relative clause | *a family **that did not give birth to them*** | `acl:relcl` | 25 |
| reduced relative (participle) | *his sheep **driven away***; *work **being prioritised*** | `acl` | 25, 28 |
| infinitive | *a chance **to win***, *food **to eat*** | `acl` | 23, 25 |
| content clause | *the fact **that he left***, *the question **whether…*** | `acl` (no gap; *that* is SCONJ) | 23 |
| post-positive adjective | *something **new*** | `amod` | §2.4 |
| apposition | *Mr. Brown, **our teacher*** | `appos` | §2.7 |
| bare-noun modifier | *$5 **a day***; *Dallas, **Texas*** | `nmod:unmarked` | §2.9 |

Content clauses on nouns are not relative clauses: there is no gap and *that* cannot be replaced by *which* (seed [../../en/seeds/nominal/content-clause-acl.md](../../en/seeds/nominal/content-clause-acl.md); Curme's "attributive substantive clause", Curme 1931, vol. III, ch. XIII).

In (2), *driven away* after *his sheep* is analysed as a reduced relative (`acl`). After a verb of perception (*see X driven away*), the participle is more naturally the object complement of *see* (`xcomp(see, driven)`), parallel to *saw him leave* (23). This is a real attachment ambiguity of English; the tree picks one reading.

### 2.9 Bare-noun modifiers and superlocations

A bare noun phrase modifying a noun is `nmod:unmarked`: measure and rate phrases (*$5 a day*, *two times a week*), and the larger place after a smaller one (*Dallas, Texas*; EWT marks these `Superlocation=Yes`, 170 tokens) (UD `_en/dep/nmod-unmarked.md`; [../../treebanks/en/ewt/seeds.md#mova-diff](../../treebanks/en/ewt/seeds.md#mova-diff)).

(7) EWT 2.18 `answers-20090717131608AAqDfYJ_ans-0005` *you mean miramar florida…* — `nmod:unmarked(miramar, florida)`.

(8) *Both my brothers live in Dallas, Texas.* «Mova en 0.4, CPU, 2026-10-17»: `conj(Dallas, Texas)` with the comma as its `punct` — read as a two-item list. The parse database has only 2 559 `nmod:unmarked` against 87 220 `appos` (2026-10-17), against 1 360 and 1 691 in EWT: the parser seldom predicts the relation (20 §5.2).

### 2.10 Noun phrases without a noun

| construction | example | head of the phrase | seed |
|---|---|---|---|
| prop-word *one* | *the red one*, *the ones I like* | *one* (NOUN) | [prop-word-one](../../en/seeds/nominal/prop-word-one.md) |
| adjective as noun | *the rich*, *the English* | the adjective (ADJ) | [adj-as-noun](../../en/seeds/nominal/adj-as-noun.md) |
| absolute genitive | *at my aunt's* | the genitive | [absolute-genitive](../../en/seeds/nominal/absolute-genitive.md) |
| stranded determiner | *I'll take **this**.*, *Some came.* | the determiner, as PRON | — |
| NP ellipsis | *Alice's cat and the Queen's* | the remnant (promoted) | 29 |

When the head noun is missing, UD promotes the next dependent to head (UD `_u-overview/specific-syntax.md`, *ellipsis*); this is the same rule as for VP ellipsis (29).

### 2.11 Pronouns

Personal pronouns stand for whole noun phrases; their case shows their function (*I* subject, *me* object or after a preposition, also in "detached" positions: *It's me*, *taller than me*) (seeds [../../en/seeds/nominal/pron-case-function.md](../../en/seeds/nominal/pron-case-function.md), [../../en/seeds/nominal/pron-paradigm.md](../../en/seeds/nominal/pron-paradigm.md), [../../en/seeds/nominal/you-it-case.md](../../en/seeds/nominal/you-it-case.md)). Reflexives are objects coreferent with the subject or intensifiers (*The king himself came*) (seeds [../../en/seeds/nominal/reflexive.md](../../en/seeds/nominal/reflexive.md), [../../en/seeds/nominal/reflexive-agreement.md](../../en/seeds/nominal/reflexive-agreement.md)); *each other* and *one another* are reciprocal objects (seed [../../en/seeds/nominal/reciprocal.md](../../en/seeds/nominal/reciprocal.md)). Reference and binding are in 35.

## 3. Quantity: numerals, quantifiers, partitives

### 3.1 Numerals

A cardinal number counting the noun is `nummod` (UD `_en/dep/nummod.md`; seed [../../en/seeds/nominal/nummod.md](../../en/seeds/nominal/nummod.md)). Ordinals are adjectives, `amod` (*the second day*; seed [../../en/seeds/nominal/ordinals.md](../../en/seeds/nominal/ordinals.md)). Inside a complex numeral, the multiplier before *hundred/thousand/million* is `compound`: in EWT 2.18, of 55 dependents of such a NUM head, 52 are `compound` (counted 2026-10-17); *hundred* stays uninflected after a numeral (*two hundred people*) and takes *-s* only as a noun (*hundreds of people*) (seed [../../en/seeds/nominal/hundred-thousand.md](../../en/seeds/nominal/hundred-thousand.md)). A number that is part of a name or identifier (*page 394*) is not `nummod` (seed [../../en/seeds/errors/numbered-entities-nummod.md](../../en/seeds/errors/numbered-entities-nummod.md)).

### 3.2 Modified numerals

*More than*, *less than*, *about*, *over*, *at least* modify the numeral, not the noun; *more than* is a fixed expression (UD `_en/specific-syntax.md`, *Quantifier phrases*, where the v1 label `mwe` is today's `fixed`):

(9) EWT 2.18 `newsgroup-groups.google.com_alt.animals.cat_0382388c4d68ddf5_ENG_20031019_125000-0006` *Now more than 1100 pictures.*

```conllu
2  more      ADJ   4  advmod
3  than      ADP   2  fixed
4  1100      NUM   5  nummod
5  pictures  NOUN  0  root
```

(10) *more than five hundred apples* «Mova en 0.4, CPU, 2026-10-17»: `advmod(hundred, more)`, `fixed(more, than)`, `compound(hundred, five)`, `nummod(apples, hundred)` — the EWT analysis.

Ranges (*2-3 lessons*) treat the dash as a preposition: `nummod(lessons, 2)`, `nmod(2, 3)`, `case(3, -)` (UD `_en/specific-syntax.md`, with v2 labels).

### 3.3 Partitives and quantity nouns

In *a lot of people*, *a piece of advice*, *a number of reasons*, the basic UD tree keeps the syntactic head — the quantity noun — and makes the substance its `nmod`:

(11) EWT 2.18 `reviews-295935-0001` *Very Informative website with alot of good work* — `nmod(website, lot)`, `nmod(lot, work)`, `case(work, of)`.

(12) *A lot of people came to the party.* «Mova en 0.4, CPU, 2026-10-17»: `nsubj(came, lot)`, `nmod(lot, people)` — the EWT analysis.

The semantic head is the substance (*people* came, not a lot), and verb agreement follows it (*a lot of people **are***; seed [../../en/seeds/verbal/agreement-notional.md](../../en/seeds/verbal/agreement-notional.md)). The Enhanced++ representation of Schuster & Manning (2016, https://aclanthology.org/L16-1376) re-heads such phrases with `det:qmod` (*lot* as a quantificational modifier of *people*), but this is not part of the UD standard (20 §8.3). Mass nouns take a portion noun to be counted (*a piece of furniture*; seed [../../en/seeds/nominal/mass-nouns.md](../../en/seeds/nominal/mass-nouns.md)).

Mova's frame reader needs the substance as the participant: whether it re-heads partitives is listed under *Not yet* below.

## 4. The adjective phrase

### 4.1 Structure

| dependent | relation | example |
|---|---|---|
| degree adverb | `advmod` | ***very** proud*, ***too** heavy*, ***so** big* |
| PP complement | `obl` | *proud **of her son***, *afraid **of the dark*** |
| *that*-clause | `ccomp` | *glad **that you came*** |
| *to*-infinitive (controlled) | `xcomp` | *eager **to go***, *able **to swim*** |
| *too / enough* + infinitive | `advcl` | *too tired **to give**…*, *strong enough **to knock out an elephant*** |
| tough-infinitive | `xcomp` | *John is easy **to please*** (23) |
| comparative standard | `obl` / `advcl` | *taller **than me***, *smarter **than I am*** (§5) |

The complement of an adjective is `obl` when it is a PP, because adjectives are predicates (UD v2: dependents of predicates are `obl`; 21 §7.1).

(13) *She was very proud of her son.* «Mova en 0.4, CPU, 2026-10-17»: `advmod(proud, very)`, `obl(proud, son)`, `case(son, of)` — correct.

### 4.2 *Too* and *enough*

Degree constructions with an infinitive attach the infinitive to the adjective as `advcl` (UD `_en/specific-syntax.md`, *Similar constructions*: *This problem was too hard to do* → `advcl(hard, do)`; *strong enough to knock out an elephant* → `advcl(strong, knock)`).

(14) EWT 2.18 `email-enronsent23_04-0020` *i was too tired to give you any action.* — `advmod(tired, too)`, `advcl(tired, give)`, `mark(give, to)`.

(15) *The box is too heavy to carry.* «Mova en 0.4, CPU, 2026-10-17»: `xcomp(heavy, carry)` — Mova's parser takes the tough-construction analysis (*The box is easy to carry*), which differs in meaning: in *too heavy to carry* the infinitive is a degree consequence, not a complement selected by the adjective. *So … that* and *such … that* attach the clause as `advcl` or `acl` respectively (UD `_en/specific-syntax.md`).

## 5. Comparison

Comparatives and superlatives are formed with *-er/-est* or with *more/most* (seeds [../../en/seeds/morph/adj-comparison-er-est.md](../../en/seeds/morph/adj-comparison-er-est.md), [../../en/seeds/morph/adj-comparison-periphrastic.md](../../en/seeds/morph/adj-comparison-periphrastic.md); doubled forms like *more happier* only in old authors and substandard speech, seed [../../en/seeds/nominal/double-comparison.md](../../en/seeds/nominal/double-comparison.md)). The UD English analysis (UD `_en/specific-syntax.md`, *Comparatives*, with v1 labels replaced by v2 ones):

| construction | example | analysis |
|---|---|---|
| periphrastic, phrasal standard | *much more intelligent than me* | `advmod(intelligent, more)`, `advmod(more, much)`, `obl(intelligent, me)`, `case(me, than)` |
| inflected, clausal standard | *much smarter than I am* | `advcl(smarter, am)`, `mark(am, than)`; the elided predicate promotes *am* |
| quantity of a noun | *more brains than me* | `advmod(brains, more)` (not `amod`), `nmod(brains, me)` |
| headless *more* | *have more than I do* | `obj(have, more)`, `acl(more, do)`, `mark(do, than)` |
| predicate vs predicate | *more hurtful than tactful* | `advcl(hurtful, tactful)` |
| measure of the difference | *three times more intelligent* | `obl:unmarked(more, times)` (v1 `npmod`) |
| correlative | *The more, the merrier.* | `mark(angrier, The)`; the clause after the comparative as a relative (UD) |

(16) *He ran much faster than his brother.* «Mova en 0.4, CPU, 2026-10-17»: `advmod(ran, faster)`, `advmod(faster, much)`, `obl(faster, brother)`, `case(brother, than)` — the EWT pattern for a phrasal standard.

Comparative clauses delete material under identity (*taller than John is ___*); the typology and the English facts are studied by Bacskai-Atkari (2018, *Deletion phenomena in comparative constructions*, `books/a/at/atkari-2018-deletion-phenomena-comparative-constructions`); ellipsis is chapter 29.

## 6. The adverb phrase

Adverbs take degree adverbs (`advmod`: *much faster*, *very quickly*), sometimes complements (*independently of the others*: `obl`), and comparative standards like adjectives (§5). Flat adverbs without *-ly* (*fast*, *hard*, *late*) are tagged ADV in their adverbial use (seed [../../en/seeds/morph/adv-flat-degree.md](../../en/seeds/morph/adv-flat-degree.md)); *-ly* formation is in [../../en/seeds/morph/adv-ly.md](../../en/seeds/morph/adv-ly.md). Placement is in 21 §7.3.

Some English "intransitive prepositions" are tagged ADV and analysed as `advmod`: *came **out***, *went **home***, *upstairs*. When they combine with a verb into a phrasal verb they are `compound:prt` (21 §6.2). *Ago* follows its measure phrase: *two years **ago*** → `advmod(came, ago)`, `obl:unmarked(ago, years)` (EWT practice).

## 7. The prepositional phrase

### 7.1 The preposition is `case`

A preposition depends on the noun it introduces, as `case`; the PP attaches through the noun: `obl` on a predicate, `nmod` on a noun (UD `_en/dep/case.md`, `_en/dep/obl.md`, `_en/dep/nmod.md`; Mova's [../../dialects/mova/seeds.md#to-phrase-obl](../../dialects/mova/seeds.md#to-phrase-obl)). The UD 2.18 validator checks the boundary (`obl-should-be-nmod`; seed [../../en/seeds/errors/obl-on-nominal.md](../../en/seeds/errors/obl-on-nominal.md), [../../en/seeds/errors/nmod-on-predicate.md](../../en/seeds/errors/nmod-on-predicate.md)).

| PP attached to | relation | example |
|---|---|---|
| verb / predicate | `obl` | *sat **on the mat*** |
| noun | `nmod` | *the cat **on the mat*** |
| adjective | `obl` | *proud **of her son*** |
| number | `nmod` | *2 **- 3*** |
| clause (a PP-like subordinator + clause) | `advcl` + `mark` | ***after** she left* (24) |

### 7.2 Complex and stacked prepositions

Two prepositions before one noun are both `case` of the noun — a flat analysis parallel to the auxiliary chain: *She ran out of the room* → `case(room, out)`, `case(room, of)` (UD `_en/specific-syntax.md`, *Prepositions*).

(17) *The cat came out from under the bed.* «Mova en 0.4, CPU, 2026-10-17»: `case(bed, from)`, `case(bed, under)`, `obl(came, bed)`, `advmod(came, out)` — consistent with the guideline's flat analysis; *out* here is a separate adverb of direction.

Fixed multiword prepositions (*because of*, *instead of*, *according to*, *in front of*, *as opposed to*) make the second word `fixed` of the first, and the first is `case` (UD `_en/dep/fixed.md`); EWT has 644 `fixed` (20 §5.2). Enhanced labels join them: `obl:because_of`.

### 7.3 Stranded prepositions

When the object of a preposition is fronted (questions, relatives) or passivised, the preposition is left behind. UD English keeps the `case` relation to the fronted wh-word, even though the tree becomes non-projective:

(18) EWT 2.18 `newsgroup-groups.google.com_eHolistic_417b09016150421f_ENG_20051120_175800-0003` *What are you searching for?* — `obl(searching, What)`, `case(What, for)`.

(19) EWT 2.18 `answers-20111107163942AA08rP5_ans-0009` *What country are we talking about?* — `obl(talking, country)`, `det(country, What)`, `case(country, about)`.

In a passive with nothing to attach to (*That matter was talked about*), the stranded preposition is itself the dependent of the verb (UD `_en/specific-syntax.md`, v1 `nmod`, today `obl`).

(20) *Whom were you talking to?* «Mova en 0.4, CPU, 2026-10-17»: `fixed(talking, to)` and `obl(talking, Whom)` — the parser does not produce the long `case` arc and glues the preposition to the verb instead. See 25 (relatives) and 26 (questions) for more stranding.

### 7.4 PPs as predicates

A PP after *be* is a predicate: the noun is the head, with `cop` and `case` (21 §1, *The Rabbit is in the hall*). A PP as the object of a preposition (*from under the bed*) is flattened as in §7.2.

## 8. Phrases and ambiguity: where parses go wrong

The errors of this chapter's examples share a cause: English word order lets one string be read as different phrases.

| ambiguity | example | right reading | Mova en 0.4 |
|---|---|---|---|
| NP with reduced relative vs clause | *All the three little pigs built houses.* | `nsubj(built, pigs)` | `acl(pigs, built)`, root *pigs* (§2.1) |
| compound vs object + complement | *She painted the fence green.* | `obj` + `xcomp` | `compound(green, fence)` (21 §10) |
| possessive vs object + complement | *We elected her president.* | `obj(her)` + `xcomp(president)` | `nmod:poss(president, her)` (21 §10) |
| superlocation vs list | *Dallas, Texas* | `nmod:unmarked` | `conj` (§2.9) |
| degree consequence vs tough | *too heavy to carry* | `advcl` | `xcomp` (§4.2) |
| PP attachment | *the cat on the mat with the hat* | `nmod` on *cat* or *mat* | varies |
| stranded preposition | *Whom were you talking to?* | `case(Whom, to)` | `fixed(talking, to)` (§7.3) |

PP attachment is the best-known case: a PP after an object may modify the object (`nmod`) or the verb (`obl`), and syntax alone cannot decide (Hindle & Rooth 1993, "Structural Ambiguity and Lexical Relations", *Computational Linguistics* 19(1), https://aclanthology.org/J93-1005). Mova attacks these with lexical knowledge after parsing: the absurdity matrix scores who can do what to what and reranks attachments ([../../docs/knowledge.md](../../docs/knowledge.md); `world::rerank`), VerbNet frames decide which PPs a verb selects ([../../prag/src/vn.rs](../../prag/src/vn.rs)), and learned tree-repair rules fix recurring patterns ([../../docs/training.md](../../docs/training.md)).

## 9. How Mova reads a noun phrase

The frame reader turns a noun phrase into an **entity** (`Ent`) keyed by the head lemma: *the Cheshire Cat* = *the Cat* = `cat`; `Ent::same` compares them ([../../docs/pragmatics.md](../../docs/pragmatics.md), *A frame*). Inside the phrase:

| phrase part | what the reader does | code |
|---|---|---|
| head noun | the entity's id (lemma) | [../../prag/src/frames.rs](../../prag/src/frames.rs) `ent_at` |
| coordinated nouns (`conj`) | kept as the entity's *and* list | `frames.rs` (`kids_of(n, Rel::Conj)`) |
| pronoun | resolved to an earlier entity (35) | `frames.rs`, `Ctx` |
| relative pronoun | the noun its clause belongs to (25) | `frames.rs` `ent_at` |
| event noun with an argument | a frame of its own: *the arrival of the Queen* → `Event queen arrive`; *the destruction of the city by the army* → `Event army destroy patient=city`; *Alice's answer* → `Event alice answer` | WordNet nominals, `prag/data/nominals.tsv` |
| possessor (`nmod:poss`) of an event noun | agent, or patient of a transitive verb when a *by*-agent is present or the possessor is a thing (*the city's destruction*) | [../../docs/pragmatics.md](../../docs/pragmatics.md), *Events of nouns* |
| *of*-phrase of an event noun | patient if the verb sense takes an object, else agent | same |

The event-noun rules use 4 951 WordNet nouns that name a verb's event; in the four courses, 1 206 noun tokens with an argument become event frames, most often *name, call, rule, word, amount, light, work* ([../../docs/pragmatics.md](../../docs/pragmatics.md), measured 2026-10-10). Most other phrase-internal structure — adjectives, numerals, compounds — becomes attributes or is not used yet.

## In Mova

- Seeds of this chapter: [../../en/seeds/nominal/](../../en/seeds/nominal/) (order, determiners, possessives, names, apposition, numerals, pronouns, relatives), [../../en/seeds/morph/](../../en/seeds/morph/) (comparison, adverbs), [../../en/seeds/errors/](../../en/seeds/errors/) (`obl-on-nominal`, `nmod-on-predicate`, `possessives-nmod-poss`, `numbered-entities-nummod`, `titles-and-company-suffixes`, `determiner-noun-number`, `much-many-countability`).
- Dialect: `det:poss` → `nmod:poss`, `:tmod`/`:npmod` → `:unmarked` ([../../docs/dialect.md](../../docs/dialect.md)); `Superlocation`, `FlatType`, `TemporalNPAdjunct` kept in MISC ([../../treebanks/en/ewt/seeds.md#mova-diff](../../treebanks/en/ewt/seeds.md#mova-diff)).
- Entities and event nouns: [../../prag/src/frames.rs](../../prag/src/frames.rs), [../../prag/src/vnconv.rs](../../prag/src/vnconv.rs) (`prag lexicon nominals`), `prag/data/nominals.tsv`.
- Attachment repair: the absurdity matrix ([../../docs/knowledge.md](../../docs/knowledge.md)), `world induce` ([../../docs/training.md](../../docs/training.md)).
- **Not yet**: `nmod:unmarked` for superlocations and rates is rarely predicted (§2.9); the long `case` arc of stranded prepositions is not produced (§7.3); *too/enough* + infinitive is taken as `xcomp` (§4.2); partitives are not re-headed in frames, so *a lot of people came* has the agent `lot`; adjectives, numerals and compounds are not systematically turned into entity attributes; no `det:qmod` or other Enhanced++ structure.

## Sources

- Universal Dependencies guidelines (snapshot 2026-09-24): `_u-overview/syntax.md`, `_u-overview/specific-syntax.md`, `_en/specific-syntax.md` (*The nominal domain*, *Quantifier phrases*, *Comparatives*, *Similar constructions*), `_en/dep/nmod-poss.md`, `_en/dep/compound.md`, `_en/dep/flat.md`, `_en/dep/nmod-desc.md`, `_en/dep/appos.md`, `_en/dep/nmod-unmarked.md`, `_en/dep/nummod.md`, `_en/dep/case.md`, `_en/dep/fixed.md`. https://universaldependencies.org/en/ ; `books/c/co/contributors-2026-universal-dependencies-guidelines`
- Poutsma, H. 1904–1926. *A Grammar of Late Modern English*. `books/p/po/poutsma-1904-grammar-late-modern-english`
- Curme, G. O. 1931. *A Grammar of the English Language*, vol. III: *Syntax*. Ch. V (modifiers of the noun: adjective, genitive, apposition), ch. XIII (attributive substantive clauses). `books/c/cu/curme-1931-grammar-english-language-vol`
- Schuster, S., Manning, C. D. 2016. Enhanced English Universal Dependencies. LREC. https://aclanthology.org/L16-1376
- Schäfer, M. 2018. *The semantic transparency of English compound nouns*. Language Science Press. `books/s/sc/schafer-2018-semantic-transparency-english-compound`
- Bacskai-Atkari, J. 2018. *Deletion phenomena in comparative constructions: English comparatives in a cross-linguistic perspective*. Language Science Press. `books/a/at/atkari-2018-deletion-phenomena-comparative-constructions`
- Hindle, D., Rooth, M. 1993. Structural Ambiguity and Lexical Relations. *Computational Linguistics* 19(1). https://aclanthology.org/J93-1005
- UD English EWT 2.18 (CC BY-SA 4.0): sentences cited by `sent_id`; counts of 2026-10-17.
- Mova's parse database (`$MOVA_DATA/parsedb/current.conllu`) and the Torah JPS 1917 trees (`$MOVA_DATA/corpora/judaism/parsedb/current.conllu`).
- Repo: [../../docs/pragmatics.md](../../docs/pragmatics.md), [../../docs/dialect.md](../../docs/dialect.md), [../../docs/knowledge.md](../../docs/knowledge.md), the seeds linked above.
