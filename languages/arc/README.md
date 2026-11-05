# Aramaic of the Jewish texts — overview and Biblical Aramaic

This folder is two things at once:

1. the **overview of the family**: the five Aramaic varieties of the Jewish texts that Mova distinguishes, the code scheme, the conventions every folder follows, and a table of the features that tell the varieties apart;
2. the **reference manual of Biblical Aramaic** (`arc`): the Aramaic chapters of Daniel and Ezra, a late literary form of Imperial (Official) Aramaic.

Mova has **no Aramaic data parsed yet** (2026-10-17). What it would need is in [mova.md](mova.md); each variety's folder has its own `mova.md` too.

## The five varieties

| code | variety | main texts | date of the language (approx.) | folder |
|---|---|---|---|---|
| `arc` | Biblical Aramaic, a form of Imperial (Official) Aramaic | Dan 2:4b–7:28; Ezra 4:8–6:18, 7:12–26; Jer 10:11; two words in Gen 31:47 | 5th–2nd c. BCE (Ezra's documents Persian-period; Daniel's tales and visions later, the visions 2nd c. BCE) | [`arc/`](.) |
| `arc-x-targum` | Jewish Literary Aramaic of the "official" targums | Targum Onkelos (Torah), Targum Jonathan (Prophets) | formed in Palestine 1st–3rd c. CE, edited in Babylonia 3rd–5th c. | [`../arc-x-targum/`](../arc-x-targum/) |
| `tmr` | Jewish Babylonian Aramaic (JBA) | Babylonian Talmud (Bavli), geonic writings, incantation bowls, Anan's Book of Precepts | 3rd–11th c. CE | [`../tmr/`](../tmr/) |
| `jpa` | Jewish Palestinian (Galilean) Aramaic | Jerusalem Talmud (Yerushalmi), Palestinian midrashim (Genesis Rabbah, Leviticus Rabbah, Pesiqta de-Rav Kahana), Palestinian targums (Neofiti, Fragment targums, Cairo Geniza targums; Pseudo-Jonathan as a late mixed text), synagogue inscriptions | 3rd–8th c. CE | [`../jpa/`](../jpa/) |
| `arc-x-zohar` | Zoharic Aramaic, an artificial literary Aramaic | Zohar, Tiqqunei ha-Zohar, Ra'aya Meheimna, Zohar Ḥadash | written in Castile, late 13th c. CE | [`../arc-x-zohar/`](../arc-x-zohar/) |

## The code scheme

The codes are fixed in the project (the scripture corpus registry `texts.tsv`, column `lang`, and its guide in the Research repository, `docs/papers/scripture.md`, 2026-10-17):

- `arc`, `tmr` and `jpa` are **ISO 639-3** codes. In ISO 639-3, `arc` is "Official Aramaic (700–300 BCE); Imperial Aramaic (700–300 BCE)", `tmr` is "Jewish Babylonian Aramaic (ca. 200–1200 CE)" and `jpa` is "Jewish Palestinian Aramaic" (SIL ISO 639-3 code tables, https://iso639-3.sil.org/code_tables/639/data). Biblical Aramaic has no code of its own; it is filed under `arc`.
- `arc-x-targum` and `arc-x-zohar` are **BCP 47 private-use extensions** (`-x-`) of `arc`, because ISO has no codes for the targumic literary language or the Zohar's language. They are Mova's own and mean nothing outside the project.
- Neighbours that Mova does **not** cover now, named so the boundaries are clear: `oar` Old Aramaic (to c. 700 BCE), `sam` Samaritan Aramaic, `syc` Classical Syriac, `myz` Classical Mandaic, `cpa`-type Christian Palestinian Aramaic (ISO `cpa` is not assigned to it; check before use), and the Neo-Aramaic languages (`aii`, `cld`, `trg`, `bjf`, `huy`, `amw` …).
- Mixed texts carry their main language in `lang` and the rest in `mixed`: the Bavli is `tmr` with `hbo-x-mishnaic` (the Mishnah and the Hebrew baraitot), Daniel and Ezra are `hbo` with `arc` in `mixed`, the Yerushalmi is `jpa` with `hbo-x-mishnaic`. The assignment in `texts.tsv` is by Sefaria category, not by sentence: a first approximation, see [mova.md](mova.md).

## The files of this folder

| file | what |
|---|---|
| [README.md](README.md) | this overview: varieties, codes, conventions, glossing |
| [family.md](family.md) | the history and classification of Aramaic, where each Jewish variety sits, and the comparison table of diagnostic features across all five varieties and Hebrew |
| [script-phonology.md](script-phonology.md) | the square script, matres lectionis, the Tiberian vocalisation and other reading traditions, consonants and their Proto-Semitic sources, vowels, stress, spirantisation; the transliteration used in every folder |
| [nouns.md](nouns.md) | the Biblical Aramaic noun: gender, number, the three states, noun patterns, adjectives, numerals |
| [pronouns.md](pronouns.md) | independent pronouns, suffixes on nouns, prepositions and verbs, demonstratives, interrogatives, the relative |
| [verbs.md](verbs.md) | stems (peal, pael, haphel/aphel, shaphel and their passives and reflexives), the perfect, imperfect, imperative, infinitive, participles; weak roots; tense and aspect |
| [particles-syntax.md](particles-syntax.md) | די, the object markers ית and ל־, prepositions, conjunctions, negation, word order, the copula, clause types |
| [lexicon.md](lexicon.md) | the core vocabulary (counted from MACULA), loanwords from Akkadian, Persian, Greek and Hebrew, and Aramaic words in Hebrew |
| [texts.md](texts.md) | annotated passages from Daniel, Ezra, Jeremiah and Genesis, word by word |
| [mova.md](mova.md) | what Mova would need for Aramaic: data, licences, tokenisation, trees, a plan for all five varieties |
| [sources.md](sources.md) | the sources used, with paths and URLs, and the works still **wanted** |

## Conventions for all five folders

### Files

Every variety folder has `README.md` (what it is, what marks it, an index), grammar files, `lexicon.md`, `texts.md` (annotated examples), `mova.md` and `sources.md`. Shared facts (script, the Proto-Semitic background, the basic morphology of Aramaic) are written once here and referred to from the others; each other folder describes **how its variety differs**.

### Transliteration

One scheme for all folders, close to the one in the SBL Handbook of Style and in MACULA's `transliteration` field:

| letter | א | ב | ג | ד | ה | ו | ז | ח | ט | י | כ | ל | מ | נ | ס | ע | פ | צ | ק | ר | שׁ | שׂ | ת |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| translit. | ʾ | b | g | d | h | w | z | ḥ | ṭ | y | k | l | m | n | s | ʿ | p | ṣ | q | r | š | ś | t |

- **Spirantised** begadkefat consonants are written ḇ ḡ ḏ ḵ p̄ ṯ **only** where the text is vocalised in the Tiberian tradition (Biblical Aramaic, vocalised targum editions). For the Babylonian Talmud, the Palestinian texts and the Zohar, which come down to us mostly unvocalised or with late, unreliable vocalisation, the transcription is **broad**: plain b g d k p t, and vowels as the grammars reconstruct them (Margolis 1910 for `tmr`, Dalman 1905 for `jpa`), marked as reconstructions where it matters.
- **Vowels:** a ā e ē i ī o ō u ū; ə for vocal shewa; ă ĕ ŏ for the hatef vowels. A long vowel written with a mater lectionis is still written with a macron (ī for ־ִי).
- **Gemination** by doubling: *qaṭṭel* for קַטֵּל.
- **Proclitics** are joined with a hyphen: *də-malkā* "of the king", *wə-ʾămar* "and he said", *lə-ḇaytā* "to the house".

### Glossing

Examples follow the Leipzig Glossing Rules (Comrie, Haspelmath and Bickel 2008, https://www.eva.mpg.de/lingua/resources/glossing-rules.php) in a five-line block:

```
Aramaic:   מַלְכָּא לְעָלְמִין חֱיִי
Translit.: malkā lə-ʿālmīn ḥĕyī
Gloss:     king.M.SG.DET to-age.M.PL.ABS live.PEAL.IMP.2MS
English:   O king, live for ever!
Source:    Dan 2:4 (WLC, public domain)
```

Abbreviations used in glosses:

| abbr. | meaning | abbr. | meaning |
|---|---|---|---|
| ABS | absolute state | CST | construct state |
| DET | determined (emphatic) state | REL | the relative / genitive particle די, ד־ |
| ACC | object marker (ית, ל־) | GEN | genitive (where ד־ is glossed as such) |
| PEAL, PAEL, HAPH, APH, SHAPH | active stems | PEIL, HITHP, ITHP, ITHPA, ITTAPH | passive / reflexive stems |
| PF | perfect (suffix conjugation) | IPF | imperfect (prefix conjugation) |
| IMP | imperative | INF | infinitive |
| PTCP.ACT, PTCP.PASS | participles | JUSS | jussive (short) imperfect |
| 1, 2, 3 | person | M, F, C | masculine, feminine, common |
| SG, PL, DU | number | NEG | negation |
| Q | interrogative | DEM | demonstrative |
| EXIST | existential (אִיתַי, אִית, איכא) | COP | copula or pronominal copula |
| PROX, DIST | near / far demonstrative | ENCL | enclitic |

The stem names (peal, pael …) stand for the verb's binyan; a gloss like `say.PEAL.PF.3MS` reads "says, peal stem, perfect, third masculine singular".

### Citing

- Every non-obvious claim names its source: author, year, and section or headword (`Rosenthal 1961 §45`, `BDB-Aram s.v. דָּת`, `Jastrow 1903 s.v. אַשְׁכַּח`). Full references, local paths and open URLs are in each folder's `sources.md`.
- Quotes from grammars are short and marked; the explanations are our own words. Nothing copyrighted is copied.
- **The ancient texts themselves** (the Bible, the targums, the Talmuds, the Zohar) are in the public domain. Each quoted line names the digital edition it was taken from and that edition's licence (from `texts.tsv`). Editions under non-commercial terms (for instance the William Davidson / Steinsaltz Talmud on Sefaria, CC BY-NC) are **not quoted**; they may only be used for tests and measurements, as `licence/DATA.md` says. CC BY-SA editions are quoted sparingly, a line at a time, with attribution.
- Numbers carry their date: counts from MACULA and from the corpora were made on 2026-10-17.

## Where to start

- For the family as a whole: [family.md](family.md), the comparison table at its end.
- For Aramaic grammar from zero: [script-phonology.md](script-phonology.md), then [nouns.md](nouns.md), [pronouns.md](pronouns.md) and [verbs.md](verbs.md), with the annotated passages in [texts.md](texts.md) open beside them.
- For one variety: its folder's README, which lists only what differs from Biblical Aramaic.
- For the engineering: [mova.md](mova.md).

The Elamur team
