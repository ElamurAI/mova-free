# Zoharic Aramaic in Mova: what it would need

General plan: [../arc/mova.md](../arc/mova.md). **Status (2026-10-17): nothing of the Zohar is parsed or tagged.**

## 1. The data

Counted 2026-10-17 in the Sefaria export (`$RESEARCH_RAW/scripture/2-rabbinic/sefaria/`, registry `texts.tsv`); whitespace tokens after stripping HTML; including editorial notes.

| text | edition | licence | words | use |
|---|---|---|---:|---|
| Tiqqunei ha-Zohar | "Tikkunei Zohar - Vocalized" | public domain | 148 282 | build and test |
| Zohar Ḥadash | "Zohar Chadash" | public domain | 214 033 | build and test |
| Tiqqunei ha-Zohar | Constantinople 1740 | CC BY-NC | | tests only |
| main Zohar | "Vocalized Zohar, Israel 2013" | unknown → private | 903 606 | not used (counts only, to size the task) |
| main Zohar | Sulam edition, Jerusalem 1945 (with Hebrew translation and commentary) | unknown → private | 820 323 | not used |
| main Zohar, English | Sperling and Simon, *The Zohar*, London: Soncino 1933 | public domain (per texts.tsv) | | a parallel English text for the parts it covers (the Soncino translation omits some sections) |
| main Zohar, English | Sefaria Community Translation | CC0 | | parallel English |
| commentaries (Ketem Paz, Mikdash Melekh, Or ha-Ḥammah, Beur ha-Gra …) | various | many public domain | | Hebrew with Aramaic quotations |

**The main gap**: no public-domain digital text of the **main Zohar** in Aramaic is on disk. The ancient text is public domain; what is needed is a transcription with a clear licence (for example of the Mantua or Vilna printing, from public-domain scans) — see [sources.md](sources.md), wanted. Until then, Mova can build only from the Tiqqunim and Zohar Ḥadash, which lean towards the later stratum ([grammar.md](grammar.md) §6).

## 2. Why the Zohar is a stress test

A model trained on Onkelos (targumic) and, later, on the Bavli (JBA) will meet in the Zohar:

1. **mixed dialect forms** in one sentence (הַנֵּי … לוֹן, [texts.md](texts.md) §7): a dialect classifier must allow a fifth, "mixed" profile rather than force a choice between `tmr` and `jpa`;
2. **Hebrew grammar in Aramaic words**: nominal sentences with אִיהוּ, Hebrew nouns with Aramaic endings (סְפִירָן), Hebrew function words (מִכָּאן, כְּאִלּוּ, כְּלָל);
3. **broken agreement** (state, gender), which a parser must tolerate;
4. **neologisms** absent from every dictionary (קַרְדִינוּתָא);
5. **dense editorial apparatus** inside the printed text (variants in parentheses, cross-references).

Measuring a parser's drop from Onkelos to the Zohar is therefore a good test of robustness — provided the test sentences are hand-annotated, not projected.

## 3. Tokenisation

| problem | rule |
|---|---|
| editorial parentheses: (נ"א …), (ס"א …), (פירוש …), page and cross-references | remove from the sentence; keep in MISC as `Variant=`, `Gloss=`, `Ref=` |
| verse quotations (Hebrew), often marked with a reference in parentheses | tag as Hebrew (`Lang=hbo`), keep as one quoted span |
| abbreviations: ת"ח, קב"ה, ר"ש, ע"ד, כד"א, הה"ד, א"ר | expand by a table ([style-formulas.md](style-formulas.md)); keep the abbreviation in MISC |
| divine names written with quotation marks: הוי"ה, יאהדונה"י, אֵי"ן סוֹ"ף | one token each; lemma = the name |
| proclitics and suffixes | as in [../arc/mova.md](../arc/mova.md) §3 |
| copula אִיהוּ / דְּאִיהוּ | split ד־; אִיהוּ is `cop` when it links subject and predicate, a pronoun otherwise |

## 4. A Zohar dictionary by linking

Most Zoharic words are words of other varieties ([lexicon.md](lexicon.md) §6). A Zohar dictionary can be built as:

1. **links** from each word to its entry in the general Aramaic dictionary (Jastrow), with the variety it comes from (targum, JBA, JPA, Hebrew);
2. a separate **technical layer**: the names of God and the sefirot, structural terms (סִטְרָא, רָזָא, עִלָּאָה, תַּתָּאָה), with the Zohar's own usage, described neutrally ("in the Zohar, X refers to …");
3. a **neologism list**, each with the passages where it occurs and the scholarly opinions, with sources.

## 5. What the Zohar can teach the SLM

- **Language mixing as a measurable property**: the share of targumic, Babylonian, Palestinian and Hebrew forms per passage, from the cue tables of the other folders, gives a profile that can be reported and explained.
- **Homiletic structure**: the formulas of [style-formulas.md](style-formulas.md) segment a homily into question, identification, analogy and proof, deterministically.
- **Symbolic identification as a relation**: the Zohar's "X is Y" statements (מָאן שׁוֹשַׁנָּה? דָּא כְּנֶסֶת יִשְׂרָאֵל) are explicit, extractable pairs; a world of the text can store them as "the text identifies X with Y", without taking a position on the doctrine.

## 6. Licences, again

Build only from the public-domain Tiqqunim and Zohar Ḥadash (and, when found, a PD main Zohar); use the Soncino 1933 English (PD per the registry) and the Sefaria Community Translation (CC0) as parallels. The Sulam and the Israel 2013 vocalised edition have no known licence and are not used. Modern translations (Matt's *Pritzker Edition*, Tishby's anthology, Scholem's selections) are in copyright and are cited, not used ([../arc/mova.md](../arc/mova.md) §7).

The Elamur team
