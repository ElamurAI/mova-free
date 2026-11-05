# Modern Hebrew (`he`): a reference manual for Mova

An open reference manual of **Modern (Israeli) Hebrew** — עברית *ivrit* — written for Mova, the small transparent language model. It describes the language as it is written and spoken today: the script and its spelling rules, the sounds, the noun and the verb, prepositions and particles, syntax, the lexicon and its registers, pragmatics and politeness, and how the modern language differs from Biblical and Mishnaic Hebrew. It then describes how Hebrew is annotated in Universal Dependencies (UD) and the four UD Hebrew treebanks, and says where Mova stands: Mova does **not** parse Modern Hebrew yet, and the last chapter says what it would need.

The manual is written in our own words. Every non-obvious claim is cited (author, year, title, and an open URL or a path in the research library); short quotations are marked. Corpus counts carry their date. Works we cite but do not hold are marked *not held* and gathered under «wanted» in [sources.md](sources.md).

## Chapters

| # | File | What it covers |
|---|---|---|
| 01 | [01-overview.md](01-overview.md) | names, speakers, status, family, typological profile, varieties and sociolects, the revival in outline |
| 02 | [02-script-and-spelling.md](02-script-and-spelling.md) | the alphabet and final letters, the vowel points (niqqud), full spelling (ktiv male) and the Academy rules, geresh and gershayim, acronyms, letter numerals, punctuation, how ambiguous unpointed text is (measured), Unicode and bidi, romanization standards; **defines the transliteration and the example format of the whole manual** |
| 03 | [03-phonology.md](03-phonology.md) | consonants and vowels of standard Israeli speech, merged historical distinctions, Mizrahi pharyngeals, uvular r, spirantization and its levelling, shva, stress, assimilation, clusters, casual reductions, intonation |
| 04 | [04-nouns-adjectives-pronouns.md](04-nouns-adjectives-pronouns.md) | gender, number, the dual, noun patterns (mishkalim), the construct state (smixut), definiteness, possessive suffixes, adjectives and comparison, pronouns, numerals and their gender polarity |
| 05 | [05-verbs.md](05-verbs.md) | root and pattern, the binyanim and their meanings, valency alternations, new verbs, tense and the participle, imperative and its spoken substitutes, infinitive, verbal nouns, the copula, modals, normative vs spoken forms, weak roots in outline |
| 05b | [05b-verb-paradigms.md](05b-verb-paradigms.md) | full pointed and transliterated paradigms of the strong verb in every binyan, key cells of every weak-root class |
| 06 | [06-prepositions-particles.md](06-prepositions-particles.md) | the two inflection series of prepositions, the seven prefixes and their vowels, the covert article, את, compound prepositions, conjunctions, focus particles |
| 07 | [07-syntax.md](07-syntax.md) | word order and inversion, partial pro-drop, verbless clauses and the copulas, existentials and possession, את and differential object marking, datives, relatives and resumptives, complements without sequence of tenses, questions, negation and negative concord, impersonals, passive, modality, conditionals, comparison, ellipsis, agreement, the noun phrase, information structure |
| 08 | [08-lexicon-register.md](08-lexicon-register.md) | the revival and Ben-Yehuda, the Language Committee and the Academy, how new words are made, loanwords by source, slang, register and norm |
| 09 | [09-pragmatics-politeness.md](09-pragmatics-politeness.md) | *dugri* directness, address without T/V, titles, gender in address, requests, greetings and blessings, religious formulas, thanks and apologies, discourse markers, swearing, irony, conversation, written vs spoken |
| 10 | [10-historical-layers.md](10-historical-layers.md) | Modern Hebrew against Biblical, Mishnaic, Medieval and Haskalah Hebrew: verbs, relatives, possession, negation, pronouns, sounds, word order, European influence |
| 11 | [11-ud-analysis.md](11-ud-analysis.md) | UD for Hebrew: segmentation and multiword tokens, the old HTB and the IAHLT tokenization, tags, features, relations (`compound:smixut`, `case:acc`, `nsubj:cop`…), constructions, the four treebanks with counts, parsing results, pitfalls for a converter |
| 12 | [12-mova.md](12-mova.md) | where Mova stands, what Hebrew asks of a pipeline, licences of the Hebrew data, a plan in steps, open questions |
| — | [sources.md](sources.md) | all sources by kind, with licences and paths; «wanted» |

Reading paths:

- **For a linguist**: 01 → 02 → 03 → 04 → 05 → 06 → 07 → 10.
- **For an engineer building a Hebrew pipeline**: 02 (§ ambiguity, Unicode) → 06 (prefixes, covert article) → 11 → 12, with 05b as the morphology reference.
- **For pragmatics and a frame reader**: 07 → 09 → 12 § 12.4 step 3.

## Conventions

- **Examples** are four-line blocks: Hebrew as normally written (unpointed unless points matter), a broad transliteration of Israeli speech, a Leipzig-style gloss, and a translation ([02 § 2.12](02-script-and-spelling.md#2122-example-layout)).
- **Transliteration**: *kh* for [χ] (both ח and כ), *ts* for צ, *sh* for ש, *r* for the uvular [ʁ], *'* only for a glottal stop actually pronounced; stress marked with an acute accent only when it is not on the last syllable (*yéled*, *báyit*, but *tapuakh*); prefixes split off with a hyphen (*ha-yéled*, *be-váyit*) ([02 § 2.12.1](02-script-and-spelling.md#2121-the-transliteration-used-in-this-manual)).
- **Corpus examples** from UD IAHLTwiki and IAHLTknesset (CC BY-SA 4.0) are quoted sparingly with their `sent_id`. HTB (CC BY-NC-SA) is only described and counted, never used to build anything.
- **Counts** were made on the UD 2.18 release files on 2026-10-17; each is a deterministic tally whose method is given where it appears.

## Key facts at a glance

| | |
|---|---|
| Family | Afro-Asiatic › Semitic › West Semitic › Northwest Semitic › Canaanite |
| Script | Hebrew square script, 22 letters (5 with final forms), right to left; vowels mostly unwritten |
| Word order | SVO; VS after fronted adverbials and with unaccusatives; prepositions; noun before adjective and genitive |
| Morphology | root and pattern (three or four consonants in a template); prefixed function words; pronominal suffixes on nouns, prepositions and (formal) verbs |
| Nouns | two genders, singular / plural / dual, construct state, definite article ה |
| Verbs | seven binyanim; past (suffixes), future (prefixes), present = participle (gender and number only); imperative, infinitive |
| Copula | none in the present tense; pronoun copula (הוא, זה) or היה in past and future |
| Possession | יש לי "there is to me" = "I have" |
| Pro-drop | partial: 1st and 2nd person past and future |
| Negation | לא; אין for existence; negative concord |
| UD treebanks | HTB (news, NC), IAHLTwiki (Wikipedia), IAHLTknesset (parliament speech), PostRab (post-Rabbinic texts) — about 376 000 words in all |
| Mova | does not parse Modern Hebrew; Biblical Hebrew via `tools/macula` |

## Licence of this manual

The text of this manual is the Mova project's own work under the Elamur Cosmic License ([`LICENSE.md`](../../LICENSE.md)). Short quotations and the few treebank sentences keep their own licences, given where they appear; see [`licence/DATA.md`](../../licence/DATA.md).

## Related

- Treebank cards: [`treebanks/INDEX.md`](../../treebanks/INDEX.md).
- Ancient (Biblical) Hebrew: the Ancient Hebrew manual at [`../hbo/README.md`](../hbo/README.md), if present in this checkout, and [`tools/macula`](../../tools/macula/README.md).
- The Mova UD dialect: [`docs/dialect.md`](../../docs/dialect.md).

*The Elamur team, 2026-10-17.*
