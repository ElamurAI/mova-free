# 15. Adjectives and adverbs

English adjectives do not agree with their noun (*a big dog, big dogs*) and have only one inflection, **comparison**: *big – bigger – biggest*. Adverbs are mostly derived from adjectives by *-ly* and compared with *more, most*; a few have their own forms. This chapter covers degree and comparison (§§1–3), the adverb and its formation (§§4–5), participial and other adjectives that come from verbs (§6), and how Mova handles them (§7). The order, position and complementation of adjectives belong to syntax (chapters [20](20-syntax-overview.md)–38); the adjective-forming suffixes are in [17-derivation-suffixes.md](17-derivation-suffixes.md) §3.

## 1. Degree

| degree | form | PTB | UD |
|---|---|---|---|
| positive | *tall, careful* | JJ, RB | ADJ/ADV `Degree=Pos` |
| comparative | *taller, more careful, less careful* | JJR, RBR (for *more, less* and *-er* forms) | `Degree=Cmp` |
| superlative | *tallest, most careful, least careful* | JJS, RBS | `Degree=Sup` |

The tag carries the degree, so the tag and the feature must agree (seed `en/seeds/morph/adj-degree-tags.md`). In periphrastic comparison the adjective stays positive (JJ, `Degree=Pos`) and *more/most* carry the degree as adverbs (RBR/RBS, relation `advmod`): *a **more** difficult task* (`en/seeds/morph/adj-comparison-periphrastic.md`). In UD EWT train the most frequent comparison forms are *best* JJS (184), *more* JJR (175) and RBR (143), *better* JJR (88), *most* RBS (80) and JJS (65), *least* JJS (62), *later* RBR (54) (counted 2026-10-17).

UD English gives `Degree` to every JJ, JJR, JJS, but to adverbs only for those that are compared (*fast, faster, well, better, soon*); most adverbs have no `Degree` (`docs/_en/feat/Degree.md` in UD; seed `adv-flat-degree.md`).

## 2. Inflected or periphrastic comparison

### 2.1 The general rule

| adjective | comparison | examples |
|---|---|---|
| one syllable | *-er, -est* | *big, tall, fast, nice, cold, young* |
| two syllables ending in *-y* | *-er, -est* (*y → i*) | *happy → happier, easy, busy, pretty, funny, early* |
| two syllables ending in *-ow, -le, -er, -ure* | either | *narrow, simple, gentle, clever, mature* (*narrower / more narrow*) |
| two syllables, stressed on the second | either | *polite, severe, sincere, complete* (*politer / more polite*) |
| other two syllables | *more, most* | *careful, useful, famous, active, modern, foolish, private, recent* |
| three or more syllables | *more, most* | *beautiful, interesting, difficult, expensive* (exception: *un-* + adjective in *-y*: *unhappier, untidier*) |
| participles used as adjectives | *more, most* | *more tired, most interesting, more worn* (exception: *tireder* in casual speech) |

Sweet NEG I §§1038–1039 and Whitney §§199–200 give the rule in terms of length and ending; Reynolds 2026 §§3.3.6–3.3.7 lists "taking *-er* and *-est*" and "being graded with *more* and *less*" among the defining properties of adjectives (seed `en/seeds/morph/adj-comparison-er-est.md`).

The choice is a tendency, not a law. One-syllable adjectives sometimes take *more* (*more real*, *more fun*, *more apt*), especially in predicative position and before *than*; adjectives that are usually compared with *more* may take *-est* in emphatic or literary style (*the beautifullest*, Lewis Carroll's *curiouser and curiouser*, which Alice says because "she was so much surprised, that for the moment she quite forgot how to speak good English" (`book:pg11:74`); Mova's book lesson retells it: *Alice cries "Curiouser and curiouser!", which is not good English (the right form is "more and more curious")…* (`school:books:003-alice-ch2-part1-curiouser-and-curiouser.lesson:5`)). *Fun* compared as *funner, funnest* is informal.

### 2.2 Spelling

*-er, -est* follow the rules of [06-spelling-rules.md](06-spelling-rules.md): doubling after a single stressed vowel (*bigger, hottest, thinner*), *e* + *er* (*nicer, latest*), *y → i* (*happier, driest*; *shyer/shier, dryer/drier*), BrE *crueller*.

### 2.3 Pronunciation

*-er* /ə(r)/, *-est* /ɪst/; *long, strong, young* restore /ɡ/: /ˈlɒŋɡə/ ([04](04-connected-speech.md) §6.4).

### 2.4 Comparatives and superlatives in use

- Comparative + *than*: *taller than me*; double comparison *more taller* is non-standard (seed `en/seeds/nominal/double-comparison.md`, error seed `en/seeds/errors/double-comparative.md`).
- Superlative with *the*: *the tallest*; the article may drop in predicative use (*This is best*).
- *Most* without *the* means "very": *a most interesting talk* (intensifier, not a superlative).
- *Less, least* make the downward comparison for all adjectives: *less expensive, least likely*.
- The comparative of two (*the taller of the two*) and the superlative of more (*the tallest of the three*) are the formal norm; *the tallest of the two* is common in speech.

## 3. Irregular comparison

### 3.1 Suppletive forms

| positive | comparative | superlative | note |
|---|---|---|---|
| *good, well* | *better* | *best* | *well* "healthy" too |
| *bad, badly, ill* | *worse* | *worst* | |
| *much, many* | *more* | *most* | determiners/quantifiers ([12](12-pronouns-determiners-numerals.md) §9) |
| *little* (amount) | *less* | *least* | *little* "small" usually *smaller, smallest*; *littler, littlest* informal |
| *few* | *fewer* | *fewest* | regular in form |

### 3.2 Double forms

| positive | comparatives | superlatives | difference |
|---|---|---|---|
| *far* | *farther, further* | *farthest, furthest* | both for distance; *further* also "additional, more" (*further details*) and as a verb |
| *old* | *older, elder* | *oldest, eldest* | *elder, eldest* only for family members and attributively (*my elder brother*), and in *elder statesman* |
| *late* | *later, latter* | *latest, last* | *latter* "the second of two"; *last* "final", *latest* "most recent" |
| *near* | *nearer* | *nearest, next* | *next* "following, adjacent" |
| *fore* (obsolete) | *former* | *first, foremost* | |

Sweet NEG I §§1044–1052 and Whitney §202 list these; seed `en/seeds/morph/adj-comparison-suppletive.md`.

### 3.3 False comparatives

Many words end in *-er* or *-est* but are not comparative or superlative and are tagged JJ: *other, proper, clever, bitter, eager, tender, sober, slender*; *honest, modest, earnest*. The Latin comparatives *superior, inferior, senior, junior, major, minor, interior, exterior, prior, ulterior* are positive adjectives in English (*superior to*, not *than*). Old comparatives and superlatives fossilized as plain adjectives: *inner, outer, upper, utter, former, latter; innermost, outermost, uppermost, utmost, foremost, topmost* (forms in *-most*) (Sweet NEG I §§1038, 1043, 1746; Whitney §§202, 211; seed `en/seeds/morph/adj-false-comparatives.md`).

### 3.4 Non-gradable adjectives

Adjectives with absolute meanings are not normally compared: *dead, unique, perfect, square, wooden, nuclear, annual, chief*. In usage they often are (*more perfect, very unique*), and the prescriptive objection is old. Classifying adjectives (*a nuclear physicist, a medical student*) cannot take *very* or comparison in their classifying sense.

## 4. Adverbs

### 4.1 Kinds by form

| kind | examples |
|---|---|
| derived with *-ly* from adjectives | *quickly, carefully, happily, basically, truly* (§5) |
| flat adverbs, identical with the adjective | *fast, hard, late, early, high, low, long, straight, wrong, right, deep, near, far, daily, weekly, kindly* (some also with *-ly* in another sense: §4.2) |
| simple, not derived | *soon, often, never, always, here, there, now, then, very, quite, too, still, just, yet, even, almost, again* |
| with *-wise, -ward(s), -ways* | *clockwise, otherwise, likewise, backward(s), forward(s), homeward(s), sideways, always, longways* |
| with *a-* | *away, abroad, ahead, asleep, aloud, apart, around, ago* |
| old genitive *-s* | *always, besides, nowadays, sometimes, towards, once, twice, unawares, needs* (*must needs*); Jespersen MEG VI 18.1 ("the endings *-s* and *-st* in particles") |
| compounds | *somewhere, anyhow, therefore, nevertheless, meanwhile, however, indeed, maybe* |

In UD EWT train 20.7% of the 10 172 ADV tokens end in *-ly* (2 101; counted 2026-10-17); the most frequent are *only* (213), *really* (150), *actually* (68), *probably* (61), *highly* (52), *especially* (40), *recently, finally* (36). Most adverb tokens are simple words like *not, so, just, also, very, now, then, here*.

### 4.2 Flat adverbs vs *-ly* adverbs

Where both exist, they often differ in meaning (Whitney §§99, 313d; Sweet NEG I §§1498–1499; seed `en/seeds/morph/adv-flat-degree.md`):

| flat | *-ly* |
|---|---|
| *work hard* (with effort) | *hardly* (scarcely) |
| *arrive late* | *lately* (recently) |
| *fly high* | *highly* (very: *highly unlikely*) |
| *come near* | *nearly* (almost) |
| *go direct* (BrE) | *directly* (straight away, BrE; in a direct way) |
| *sharp* (*at 5 o'clock sharp*) | *sharply* |
| *pretty* (fairly: *pretty good*) | *prettily* (in a pretty way) |
| *most* (*most interesting*) | *mostly* (mainly) |
| *free* (without paying) | *freely* (without restriction) |
| *wrong* (*go wrong*) | *wrongly* (*wrongly accused*) |
| *right* (*turn right*) | *rightly* (correctly, justly) |
| *dead* (*dead tired*, informal) | *deadly* (adj.: *deadly serious*) |

Informal American English uses flat forms more widely: *real good, drive safe, go slow, talk quiet*. *Slow, quick, loud, clear, safe, tight* have long been flat adverbs in set phrases (*go slow, come quick, read loud and clear, hold tight, sleep tight*).

### 4.3 Comparison of adverbs

- Flat adverbs and a few others take *-er, -est*: *faster, harder, later, sooner, earlier, longer, nearer, higher, oftener* (now usually *more often*).
- Irregular: *well – better – best; badly – worse – worst; far – farther/further – farthest/furthest; little – less – least; much – more – most*.
- *-ly* adverbs take *more, most*: *more carefully, most easily* (*earlier* is from *early*, which is both adjective and flat adverb).

## 5. The suffix *-ly*

### 5.1 Adjective → adverb

*-ly* turns almost any adjective into an adverb of manner, degree or viewpoint: *quick → quickly, careful → carefully, recent → recently, obvious → obviously, honest → honestly, frank → frankly*. It applies to participles too (*willingly, surprisingly, reportedly, allegedly, markedly*: the last two with /ɪd/). Sweet NEG I §1500 calls it fully productive.

Spelling: *happy → happily* (*y → i*), *true → truly, due → duly, whole → wholly* (*e* dropped), *simple → simply, gentle → gently, able → ably* (*-le → -ly*), *basic → basically* (*-ic → -ically*; except *publicly*), *full → fully, dull → dully* (*ll* + *y*), *shy → shyly, sly → slyly, dry → drily/dryly, gay → gaily* ([06](06-spelling-rules.md) §§3, 5, 6).

Restrictions:

- Adjectives that already end in *-ly* (*friendly, lovely, lonely, ugly, silly, likely, lively*) do not usually add another *-ly* (*friendlily* is rare); a phrase is used instead: *in a friendly way* (Sweet NEG I §1500).
- Some adjectives have no *-ly* adverb in common use: *big, small, little, old, young, tall, fat, wet* (\**bigly* is a joke).
- Some *-ly* adverbs exist without an adjective: *accordingly, exceedingly, seemingly, purportedly, supposedly*.

### 5.2 Noun → adjective

The same suffix on a noun makes an **adjective**: *friend → friendly, man → manly, cost → costly, love → lovely, coward → cowardly, time → timely, order → orderly, scholar → scholarly*; on time words it makes adjectives that also serve as flat adverbs: *daily, weekly, monthly, yearly, hourly* (*a daily paper; it is published daily*). So *-ly* is not a reliable mark of an adverb: in EWT train *friendly* (53), *early* (35), *likely* (27), *daily* (18), *weekly* (12), *lovely* (11) occur as ADJ (seed `en/seeds/morph/adv-ly.md`; Sweet NEG I §341; Whitney §193).

### 5.3 History

*-ly* comes from Old English *-līce*, the adverbial form of the suffix *-līc* "like, having the body of" (the adjective suffix that became *-ly* in *manly*). Middle English also formed adverbs with a plain *-e*, which was lost and left the flat adverbs (Jespersen MEG VI 22.8–22.9; Sweet NEG I §1500).

## 6. Adjectives from verbs

### 6.1 Participial adjectives

Both participles serve as adjectives:

| type | meaning | examples |
|---|---|---|
| *-ing* (active) | causing the feeling, doing the action | *an interesting book, a boring film, a frightening noise, running water, a sleeping child* |
| *-ed/-en* (passive, resultative) | having undergone it, feeling it | *an interested reader, bored students, a frightened child, a broken window, a fallen tree, the retired teacher* |

Learners confuse the pairs (*I am boring* for *I am bored*). Whether a participle is an adjective or a verb form is decided by tests: *very*, *seem*, *un-*, comparison and the absence of an agent point to an adjective ([10-word-classes.md](10-word-classes.md) §4; seeds `en/seeds/verbal/participle-or-adjective.md`, `en/seeds/morph/wf-ed-adjectives.md`).

### 6.2 *-ed* adjectives from nouns

*-ed* added to a noun makes an adjective "having X": *talented, bearded, walled, gifted, skilled, aged*; most often in compounds: *blue-eyed, long-legged, kind-hearted, left-handed, three-cornered, old-fashioned*. These are not participles of any verb (\**to talent*) ([17](17-derivation-suffixes.md) §3.2; Jespersen MEG VI 24.1).

### 6.3 Old participles

*drunken, sunken, molten, swollen, laden, rotten, wrought* and others survive as adjectives beside the newer participles ([14](14-irregular-verbs.md) §7; seed `en/seeds/morph/verb-participle-adjectives.md`).

## 7. Adjectives used as nouns, and nouns used as adjectives

- *the rich, the poor, the young, the old, the English, the French, the unknown, the impossible*: adjectives heading a noun phrase without *-s*, referring to people in general or to an abstract quality. They stay ADJ in UD (`Degree=Pos`, no `Number`). Full conversion to a noun takes *-s* and *a*: *a native, natives, a criminal, the ancients, valuables, greens* (seed `en/seeds/morph/adj-as-noun.md`, `en/seeds/nominal/adj-as-noun.md`; Sweet NEG I §§106–107, 1035; Jespersen MEG II 11.3–11.5).
- Nouns before nouns (*a stone wall*) remain nouns (UD `compound`), unless they have become adjectives (*a gold watch* is NOUN, *a golden opportunity* ADJ) ([11](11-nouns.md) §9).

## 8. How Mova handles adjectives and adverbs

- **Tags and lemmas.** The tagger assigns JJ/JJR/JJS and RB/RBR/RBS; the lemma of a comparative or superlative is the positive (`Slot::Comp`, `Slot::Super` in `en/src/morph.rs`): *bigger → big, happiest → happy, better → good* (JJR) or *well* (RBR, by UD counts), *worse → bad*. The unit test `dictionary` checks *better* JJR ↔ *good*.
- **Fallback rules** for unknown adjectives: `suffix` makes *-er/-est*, with *y → ier/iest* and *e + r/st*, but **no consonant doubling** (an unknown *glum* would come out *glumer*); `unsuffix` tries *-ier → -y*, *-er*, *-r* (and the same for *-est*) but not undoubling, so an unseen *glummer* lemmatizes as *glumm*, not *glum*. Known adjectives are in the lexicon with AGID's forms, so the gap only matters for unknown words.
- **Choice of inflected vs periphrastic comparison** is not generated by rule; the generator does not create comparatives.
- **False comparatives** (*other, proper, clever, honest*) are JJ because their UD counts say so; the seed `adj-false-comparatives.md` has rules that flag JJR/JJS on them.
- **-ly words** are tagged by context, not by suffix: *friendly, daily* can be JJ, *only* RB or JJ.
- **Not yet**: no lexical feature for gradability or for "takes *-er*"; no link between an adjective and its *-ly* adverb in the lexicon (*quick* and *quickly* are separate lemmas, as in UD).

## Sources for this chapter

- Sweet 1891, NEG I §§1035–1052 (comparison), §§1498–1527 (adverbs), §§341–342, §1746. https://archive.org/details/newenglishgramma01sweeuoft
- Whitney 1877, §§94–99, 193, 199–211, 313–316. https://archive.org/details/essentialsofengl01whit
- Jespersen 1942, MEG VI 18.1 (*-s* in particles), 22.7–22.9 (*-ly*), 24.1 (*-ed* adjectives), 5.5–5.7 (old participles). https://archive.org/details/modernenglishgra06jesp
- Kruisinga 1932, *Handbook* II.3 §§1723–1729 (comparison). https://archive.org/details/handbookofpresen32krui
- Reynolds 2026, §§3.3.1–3.3.8 (adjectives), §4.2 (adverbs). https://zenodo.org/records/20612113
- Carroll, Lewis. 1865. *Alice's Adventures in Wonderland*, ch. 2 (public domain; PG #11; `book:pg11` in Mova's parse database).
- Universal Dependencies, `Degree`: https://universaldependencies.org/en/feat/Degree.html
- Santorini 1990 (JJ, JJR, JJS, RB, RBR, RBS). https://repository.upenn.edu/cis_reports/570/
- Mova: `en/src/morph.rs`; seeds `en/seeds/morph/adj-*.md`, `adv-*.md`, `wf-ed-adjectives.md`, `verb-participle-adjectives.md`; counts from UD English EWT 2.18 train (2026-10-17).
