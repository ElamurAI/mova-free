# Jewish Palestinian Aramaic in Mova: what it would need

General plan: [../arc/mova.md](../arc/mova.md). **Status (2026-10-17): no JPA text is parsed or tagged.**

## 1. The data

Counted 2026-10-17 in the Sefaria export (registry `texts.tsv`); words are whitespace tokens after stripping HTML; Hebrew and Aramaic together.

| text | edition | licence | files | words | use |
|---|---|---|---:|---:|---|
| Jerusalem Talmud (39 tractates) | Guggenheimer (De Gruyter 1999–2015), vocalised | **CC BY** | 39 | 811 526 | build and test, with attribution |
| Jerusalem Talmud | Venice 1523; Mechon-Mamre | unknown → private | | | not used |
| Pseudo-Jonathan (Torah) | Sefaria "Targum Jonathan on Genesis" … | public domain | 5 | | build and test (mixed language) |
| Fragment targum | Sefaria "Targum Jerusalem on Torah" | public domain | 1 | | build and test |
| Pseudo-Jonathan + Fragment targum together | | public domain | 6 | 122 656 | |
| Targum Neofiti | Vatican MS (Sefaria file) | unknown → private; the file holds only 137 words | 1 | 137 | not used; a full PD text is wanted |
| Palestinian midrashim (Genesis Rabbah etc.) | various Sefaria versions | some PD, some unknown | | | mostly Hebrew; Aramaic stories inside |

**The Guggenheimer Yerushalmi is the most valuable open Aramaic text Mova has**: 39 tractates, vocalised, CC BY (attribution only). It is the one large Talmudic corpus that can be used to build.

## 2. What is special about JPA for a model

1. **Very little parsed or annotated material exists**, and none of it is open. Sokoloff's dictionary and Fassberg's grammar are in copyright.
2. **The text is mixed with Hebrew** more than the Bavli; Aramaic sentences are often short links between Hebrew statements ([lexicon.md](lexicon.md) §4).
3. **The printed Yerushalmi was Babylonianised by copyists**; a model trained on it will see some JBA forms (לימא 3×, האי 6×). These are not errors of the model; they should be counted and reported.

## 3. Telling JPA from JBA: a measured cue set

The counts in [README.md](README.md) §2 and [lexicon.md](lexicon.md) §1 give a **deterministic classifier** with explainable cues:

| JPA cue | Y | B | JBA cue | Y | B |
|---|---:|---:|---|---:|---:|
| כְּדוֹן | 419 | 0 | הַשְׁתָּא | 1 | 974 |
| לוֹן | 731 | 0 | לְהוּ | 9 | 4 024 |
| הָדָא | 2 117 | 23 | הַאי | 6 | 2 246 |
| הֵיךְ | 218 | 0 | הֵיכִי | — | 1 399 |
| אוּף | 188 | 0 | נַמִי | — | 6 858 |
| בְּגִין | 250 | 2 | קָא | 0 | 2 828 |
| סַגִּין | 61 | 0 | טוּבָא | 0 | 191 |
| אִין תֵּימַר | 382 | — | אִיכָּא | 6 | 2 488 |

(Y = Guggenheimer Yerushalmi, CC BY; B = Wikisource Bavli, CC BY-SA, measurement only; unpointed tokens; 2026-10-17.)

**Proposed test** (not yet run): score each paragraph by (JPA cues − JBA cues); measure on held-out paragraphs of both Talmuds, and on the hard cases — Palestinian traditions quoted in the Bavli, and Babylonian forms in the Yerushalmi. The negative control: the same classifier on **Hebrew** paragraphs (Mishnah) must say "neither", not guess.

The same cue table extends to the targums: Onkelos has neither set (it has כְּעַן, אֲרֵי, יָת), Pseudo-Jonathan has the JPA set partly ([../arc-x-targum/lexicon.md](../arc-x-targum/lexicon.md) §5).

## 4. Tokenisation and trees

- Proclitics and suffixes as in [../arc/mova.md](../arc/mova.md) §3.
- **לוֹן** = לְ + ־הוֹן contracted: one token in writing, split as preposition + pronoun.
- **The ethical dative** (נִיזוּל וְנִיחוּת לִי, נָחַת לֵיהּ) — a ל־ phrase co-referent with the subject — needs one convention (proposal: `obl:ethical` or `expl:dat`), applied everywhere ([verbs.md](verbs.md) §6).
- **חַד / חֲדָא** as an indefinite article: `det` (with `PronType=Ind` or `NumType=Card` kept in MISC), not `nummod`, when it introduces a new referent.
- **לֵית + pronoun + participle**: the existential is the negation (`advmod` with `Polarity=Neg`), the pronoun the subject, the participle the head.
- **Vocalisation**: the Guggenheimer pointing is modern and consistent; keep it in FORM, match by consonants.

## 5. What JPA can teach the SLM

- **Short stories with clear events** (y. Bava Metzia 2:5, y. Berakhot 2:8): agent, goal, purchase, return, moral — material for the event frames of the school, from an open (CC BY) source.
- **Dialect identification as an explainable task**: the cue table above is a model that can say why it decided.
- **Midrashic expansion in the Palestinian targums** (Gen 4:8): a natural contrast between a literal (Onkelos) and an interpretive (Palestinian) rendering of the same verse.

The Elamur team
