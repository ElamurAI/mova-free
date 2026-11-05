# Targumic Aramaic in Mova: what it would need

The general plan for all five varieties is in [../arc/mova.md](../arc/mova.md). This file adds what is specific to the targums. **Status (2026-10-17): nothing targumic is parsed.**

## 1. The data

Counted on 2026-10-17 from the Sefaria export in `$RESEARCH_RAW/scripture/1-originals/sefaria/` (registry `texts.tsv`; verses = entries of the text arrays; words = whitespace tokens after stripping HTML).

| text | edition | licence | verses | words | use |
|---|---|---|---:|---:|---|
| Onkelos, Torah (5 books) | "Onkelos Genesis" … | public domain | 5 846 | 82 914 | build and test |
| Onkelos, Torah | Yemenite *Taj* vocalisation | CC BY-SA | 5 books | 80 946 | as little as possible; test and learn only |
| Onkelos, Torah | Metsudah *Sifsei Chachomim* (2009) | CC BY-NC | 5 books | — | tests only |
| Jonathan, Prophets (21 books) | Mikraot Gedolot | public domain | 9 296 | 157 682 | build and test |
| Writings: Psalms, Job, Proverbs, Ruth, Song, Lamentations, Ecclesiastes, Esther | Mikraot Gedolot | public domain | 5 257 | | build and test (mixed language, see README §5) |
| Song of Songs | Melamed 1921, six Yemenite MSS | public domain | 117 | | a second witness |
| Chronicles (1–2) | Wikisource, "Targum Jerusalem" in Sefaria's tree | public domain | 1 765 | | build and test |
| Targum Sheni to Esther | Berlin 1898 | unknown → private | | | not used |
| all PD Writings + Chronicles | | | 7 022 | 104 183 | |

**Every verse has a Hebrew original with the same reference**, and for the Torah the Hebrew already has trees (`tools/macula`, `hbo`, 2026-10-17). That makes the targum the best-placed Aramaic corpus for Mova.

## 2. Alignment and projection

The plan of [../arc/mova.md](../arc/mova.md) §4.2, in detail for Onkelos:

1. **Verse alignment**: by reference. Check that the verse division of the Sefaria Onkelos matches the WLC for every chapter (5 846 Onkelos verses against 5 846 Hebrew verses in the Torah, if they match one to one — to be verified per chapter, since the Masoretic and printed-targum divisions sometimes differ).
2. **Word alignment** inside the verse, in three passes, all deterministic:
   - **anchors**: the pairs of [lexicon.md](lexicon.md) §2.3 (אֶת ↔ יָת, כִּי ↔ אֲרֵי, אֲשֶׁר ↔ דִּי, הִנֵּה ↔ הָא, לֵאמֹר ↔ לְמֵימַר), proper names (identical consonants), יְהוָה ↔ יְיָ;
   - **lexicon**: the Hebrew → Aramaic table of [lexicon.md](lexicon.md) §2 extended from Jastrow (whose entries cite "h. text" and the Hebrew word a targum word renders) and from the regular sound correspondences ([../arc/script-phonology.md](../arc/script-phonology.md) §4);
   - **position**: the remaining words in order, between anchors (monotonic alignment).
3. **Projection**: each Aramaic word takes the head and relation of its Hebrew counterpart; Aramaic words with no counterpart (קֳדָם, מֵימְרָא דַּ־, added explanations) get rules: קֳדָם + noun is an `obl` of the verb; מֵימְרָא is the noun the Hebrew subject's relation goes to, with יְיָ as its `nmod`.
4. **Confidence**: the share of aligned words per verse. Verses below a threshold (poetry, expansions; [translation-syntax.md](translation-syntax.md) §5) are flagged, not projected.

**How to measure.** No Aramaic gold exists. Annotate a sample by hand (for example 100 verses spread over the five books, including Gen 49 and Deut 32–33), then compare. The rule: decisions on half the sample, the other half read once, as with SVAMP. A negative control: projection from a **shuffled** verse (wrong Hebrew) must give a much worse score.

## 3. Tokenisation specifics

- **יְיָ** is a token of its own (lemma = the divine name, the same as Hebrew יהוה); in unpointed text it is the string יי, also inside words: דַּיְיָ = ד־ + יְיָ, וַיְיָ = ו־ + יְיָ, לַייָ.
- The **object marker** יָת takes suffixes (יָתֵהּ, יָתְהוֹן, יָתְכוֹן); split as יָת + pronoun, as UD does for Hebrew אוֹתוֹ.
- **Editorial notes** in parentheses inside the printed text (Gen 28:20 "(נ''י לְחֵם)", Exod 19:4 "(נ״א וְאַטְּלִית)") are removed from the sentence and kept in MISC.
- **ד־ and דִּי**: both occur; the separate דִּי is a word, the prefix is split.

## 4. What the targums can teach the SLM (Mova's small language model)

1. **A bilingual dictionary for free.** Word alignment of Onkelos gives a Hebrew–Aramaic glossary with frequencies, verse examples and an English column from the JPS 1917 translation (already in the corpus).
2. **Paraphrase detection.** The departures of [translation-syntax.md](translation-syntax.md) §3–5 are a labelled set of "translation versus interpretation", decidable by explainable cues (added words, replaced metaphors).
3. **Pragmatics.** The same event told with and without anthropomorphism is a natural test of event structure: who acts (God / the Memra), who perceives (sees / "it is revealed before"), what changes (repents / "turns back in his Memra"). The school's pragmatic frames can be checked against both texts.
4. **A bridge to the Talmud.** The targumic vocabulary is the core of all Jewish Aramaic; the differences from JBA and JPA are few and listed ([lexicon.md](lexicon.md) §5).

## 5. Licences

The Sefaria Onkelos, Jonathan and Writings targums are **public domain** and can be used to build. The Taj edition (CC BY-SA) is the best vocalised text but must stay out of what is built and shipped; it may be used to test and measure. The Metsudah edition (CC BY-NC) is for tests only. Sperber's critical edition (Brill) is in copyright and not on disk. See [../arc/mova.md](../arc/mova.md) §7.

## 6. Open questions

- Do the Sefaria Onkelos verse divisions match the WLC everywhere? (To check before alignment.)
- How much of the Writings targums is JPA-like? The sentence classifier of [../arc/mova.md](../arc/mova.md) §5 should be run on them and the result reported per book.
- Is the Melamed 1921 Song of Songs (six Yemenite MSS) a better witness than the Mikraot Gedolot text for training? Compare the two token by token.

The Elamur team
