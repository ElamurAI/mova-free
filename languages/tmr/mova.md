# Jewish Babylonian Aramaic in Mova: what it would need

The general plan is in [../arc/mova.md](../arc/mova.md). **Status (2026-10-17): no JBA text is parsed or tagged.** This file lists what is specific to the Bavli.

## 1. The data

| edition | licence (texts.tsv) | tractates | tokens | use |
|---|---|---:|---:|---|
| Wikisource Talmud Bavli (in the Sefaria export) | CC BY-SA | 37 | 1 858 059 (81 793 lines) | **as little as possible**: measure, test, learn; never carried or modified into a release |
| William Davidson Edition, Aramaic (Steinsaltz) | CC BY-NC | 37 | 1 854 708 | tests only |
| William Davidson Edition, vocalised Aramaic | CC BY-NC (some files "unknown") | | | tests only (useful as a vocalisation reference in measurement) |
| Rodkinson, *New Edition of the Babylonian Talmud* (1896–1903), English | public domain (incomplete on Wikisource) | partial | | an English reference for some tractates |
| Ein Yaakov (aggadah of the Bavli), Hebrew/Aramaic and Glick's English (1916–1921) | public domain | | | build and test (aggadic passages) |
| Rishonim and Aḥaronim commentaries | various, many PD | | | quote the Bavli; Hebrew with Aramaic |

Counts on 2026-10-17 (whitespace tokens after stripping HTML; Hebrew and Aramaic together).

**The key gap is licensing, not size.** The Vilna text itself is in the public domain, but the open digital editions on disk are CC BY-SA or CC BY-NC. To **build** anything shipped from the Bavli, Mova needs a public-domain digitisation (for example a transcription of the Vilna print released as PD, or older public-domain digital texts with a clear statement) — see [sources.md](sources.md) "wanted". Until then, the Bavli is a **measurement** corpus.

## 2. Splitting Hebrew from Aramaic

The Bavli mixes Hebrew and JBA at every level ([particles-syntax.md](particles-syntax.md) §11). The split must be made per **clause** and, in places, per **word** (Shabbat 31a: בָּא לִפְנֵי הִלֵּל גַּיְּירֵיהּ). A deterministic, explainable splitter can use:

1. **Structure**: the Mishnah is marked (מתני׳ … גמ׳); quotations open with תְּנַן, תַּנְיָא, תָּנוּ רַבָּנַן, דִּכְתִיב, שֶׁנֶּאֱמַר and close at the next Aramaic discourse term ([discourse-terms.md](discourse-terms.md)).
2. **Cue words**: the JBA function words (מַאי, הַאי, הָנֵי, אִיכָּא, לֵיכָּא, קָא, נַמִי, הָכִי, לָאו, אֶלָּא in Aramaic syntax, אֲמַר לֵיהּ) against Hebrew ones (אֶת, אֲשֶׁר, שֶׁל, שֶׁ־, זֶה, אֵין, הַ־ article, plural ־ִים / ־וֹת).
3. **Morphology**: the l-/n- imperfect, the enclitic pronouns, ־ֵיהּ suffixes, plural ־ֵי, determined ־ָא.

The output is a language tag per clause and per word in MISC (`Lang=hbo-x-mishnaic` inside a `tmr` sentence). **Measure** on hand-labelled pages (for example the five pages of [texts.md](texts.md)), and keep a negative control: a page with its Hebrew and Aramaic sentences swapped between two tractates must still be split correctly, and a splitter that labels everything `tmr` must score visibly worse.

## 3. Tokenisation

| problem | example | rule |
|---|---|---|
| proclitics | דְּקָאָמַר = ד + קָא + אָמַר; וְלִיתְנֵי = ו + לִיתְנֵי; אַקְּרָא = אַ (עַל) + קְרָא | split ו, ד, ל, ב, מ, כ; split קָא when fused; expand אַ־ = עַל |
| enclitic pronouns on participles | אָמֵינָא = אָמֵר + אֲנָא; אָמְרִינַן = אָמְרִין + אֲנַן; סָמְכַתְּ = סָמֵךְ + אַתְּ | split into verb + `nsubj` pronoun; keep the surface form in MISC |
| object suffixes | גַּיְּירֵיהּ, שַׁמְעִינְהוּ | split verb + pronoun (`obj`) |
| copulas | נִיהוּ, נִינְהוּ | one token, `cop` (or `nsubj` + `cop` per the UD Hebrew convention) |
| contractions | אִיכָּא, לֵיכָּא, הַשְׁתָּא, אַמַּאי | keep as one token, lemma = the contraction |
| abbreviations | א"ל, ת"ש, מ"ט, ש"מ, קמ"ל, קוב"ה | expand by a table before parsing ([phonology-orthography.md](phonology-orthography.md) §1.3); keep the abbreviation in MISC |
| numerals as letters | ד' מאות | read as numbers |
| editorial brackets | האי [גברא] עלמא | a bracketed word is an editor's addition: keep it, mark it in MISC |

## 4. Trees and features

- **Definiteness.** The ־ָא ending is not a definiteness marker in `tmr` ([nouns.md](nouns.md) §1). Do not set `Definite=Def` from it; set definiteness only from demonstratives, suffixes and ד־ genitives, or leave it unset.
- **Prolepsis** (אַשְׁכְּחֵיהּ … לְאֵלִיָּהוּ). Decide one convention for the doubled object (suffix pronoun as `expl` or as an agreement feature on the verb, the ל־ noun as `obj` with `case`) and keep it everywhere.
- **The l-/n- imperfect** gets `Mood` from use (לֵימָא as a proposal is a question / jussive), `Tense=Fut` only where it is a plain future.
- **קָא** is `advmod` with a feature (`Aspect=Prog` on the verb, or a MISC tag); it should not be split off as a separate clause.
- **Discourse terms** are multi-word expressions; tag them as such (MISC `Cxn=sugya:objection` etc.) so the argument structure is recoverable ([discourse-terms.md](discourse-terms.md)).

No JBA treebank exists anywhere that we know of (checked: UD 2.18 has none). The first trees will be a hand-annotated sample plus a parser trained on Biblical and targumic Aramaic, with the drop measured on the sample.

## 5. What the Bavli can teach the SLM

1. **Argument structure.** The discourse terms make a sugya a labelled graph: source → question → objection → answer → conclusion. This is the clearest natural corpus of **explicit reasoning moves** in any language, and it is deterministic to segment by the term list.
2. **Counterfactuals and modality.** הֲוָה אָמֵינָא "I would have thought", אִי לָאו … הֲוָה, לֵימָא "shall we say?", צְרִיכָא "both are needed": the Bavli names its hypotheses and rejects them. These are labelled examples of supposition and refutation.
3. **Code-switching.** Hebrew quoted, Aramaic commenting: a natural corpus for language identification per word.
4. **Stories.** The aggadah (BM 59b, Taanit 23a) is short narrative with events, participants and a moral (often a proverb): material for the event frames of the school, once a PD text is available.

## 6. Measures to report when work starts

- the share of tokens tagged Hebrew vs Aramaic per tractate (with the splitter's measured accuracy);
- the coverage of the JBA word types by Jastrow headwords (after normalising spelling and expanding abbreviations);
- the number of sugya moves found by the discourse-term tagger per page, against a hand-segmented sample.

None of these exist yet (2026-10-17).

The Elamur team
