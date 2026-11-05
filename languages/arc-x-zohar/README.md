# The Aramaic of the Zohar (`arc-x-zohar`)

The **Zohar** (סֵפֶר הַזֹּהַר, "the Book of Radiance") is the central work of Jewish Kabbalah: a large body of mystical commentary on the Torah, the Song of Songs and Ruth, written for the most part in **Aramaic**. It appeared in Castile in the last decades of the 13th century. Its Aramaic is not a spoken dialect of any period but a **literary language created by its authors** out of the Aramaic they read — Onkelos and the Babylonian Talmud above all — and shaped, often visibly, by their own medieval Hebrew.

The code `arc-x-zohar` is Mova's private-use extension of `arc` ([../arc/README.md](../arc/README.md)). This folder describes the Zohar as a **language and a text**, from the outside: what its grammar and vocabulary are, where they come from, how to read them, and what Mova would need to process them. It does not present or interpret the Zohar's teachings.

Conventions: [../arc/README.md](../arc/README.md). Transcription is broad; vowels follow the vocalised public-domain edition of the Tiqqunei ha-Zohar on Sefaria where quoted from it, and are otherwise our reconstruction.

## 1. What the Zohar is

- **The corpus.** The printed Zohar (Mantua 1558–1560, Cremona 1559–1560; the standard pagination is that of Mantua, in three volumes, cited as Zohar 1:1a etc.) contains several distinct works (Scholem 1941, lecture 5; Tishby 1989, introduction; Huss 2016):
  - the **main Zohar**, homilies on the weekly Torah portions, framed as conversations of Rabbi Shimon bar Yoḥai and his companions (חַבְרַיָּא) as they walk through Galilee;
  - the **Idrot** (אִדְרָא רַבָּא, אִדְרָא זוּטָא), gatherings in which the divine "countenances" are described;
  - **Sifra di-Tseniʿuta** ("the Book of Concealment"), a short, very dense text;
  - **Midrash ha-Neʿelam** ("the Hidden Midrash"), largely in **Hebrew**;
  - **Raʿaya Meheimna** ("the Faithful Shepherd") and **Tiqqunei ha-Zohar** (seventy "rectifications" of the word בְּרֵאשִׁית), a later layer with a distinct style;
  - **Zohar Ḥadash** ("the New Zohar"), material collected after the first printing.
- **Authorship.** The work presents itself as the teaching of R. Shimon bar Yoḥai (2nd c. CE). Since Gershom Scholem's studies (Scholem 1941, lectures 5–6), most scholars have attributed the main body to **Moses de León** (c. 1240–1305) in Castile; later research (Liebes 1993; Meroz; Huss 2016) speaks of a **circle** of authors and of composition over several decades, with Tiqqunei ha-Zohar and Raʿaya Meheimna as a later layer by another hand. The traditional view of ancient authorship is still held by many readers. The linguistic facts in this folder (medieval Hebrew calques, Castilian vocabulary, artificial grammar) are among the main arguments of the scholarly view (Scholem 1941, lecture 5, § "The language of the Zohar").
- **Status.** From the 16th century the Zohar became a canonical text of Judaism; passages from it entered the liturgy (*Berikh shemeh*, the *Patah Eliyahu* passage from the Tiqqunim, the hymns of Isaac Luria built on its language).

## 2. What marks its Aramaic

Against **all the classical varieties**:

| feature | Zohar | count (PD Zohar texts on disk) | elsewhere |
|---|---|---:|---|
| **אִיהוּ / אִיהִי** as the general copula and pronoun ("is", "he/it") | very frequent: אִיהוּ 2 285, דְּאִיהוּ 1 744, וְאִיהוּ 494, אִיהִי 923 | per 100 000 tokens: ~670 (אִיהוּ) | Bavli: 314 (17 per 100 000); Yerushalmi and Onkelos: 0 |
| **דָּא** "this" as a pronoun and copula-like topic marker | 4 225 | | Onkelos 67; Yerushalmi 212; Bavli 50 |
| **רָזָא** "secret, mystery" | 541; רָזָא דָא 256 | | Bavli 1; Yerushalmi 1 |
| **כְּגַוְונָא** "like, in the manner of" | 488; כְּגַוְונָא דָא 281 | | Bavli 0; Yerushalmi 0 |
| **בְּגִין** "because, for the sake of" (also בְּגִין כָּךְ, וּבְגִין דָּא) | 1 098 | | Yerushalmi 250; Bavli 2 |
| **כְּמָה דְּאַתְּ אָמַר** "as you say" (verse quotation) | 270 | | Yerushalmi 72 |
| **תָּא חֲזֵי** "come and see" | 280 (abbr. ת"ח in other editions) | | Bavli: the formula is תָּא שְׁמַע |
| **סִטְרָא** "side" (סִטְרָא אַחֲרָא "the other side") | 186; 62 | | |
| **קוּדְשָׁא בְּרִיךְ הוּא** (God) | 1 251 | | Bavli (abbr. קוב"ה) |
| **עִלָּאָה / תַּתָּאָה** "upper / lower" | 610 / 348 | | |
| **סְפִירָן**, **סְפִירָה** (Hebrew noun, Aramaic plural) | 192; 62 | | none |
| **mixed dialect forms** | both אִית and אִיכָּא (514 / 6), both הַשְׁתָּא and כְּעַן (74 / 43), JPA אוּף (91) beside JBA נַמִי (38), JPA לוֹן beside JBA forms | | each Talmud uses one set |

(Counts: unpointed whitespace tokens in the two **public-domain** Zohar files on disk — the vocalised Tiqqunei ha-Zohar and Zohar Ḥadash on Sefaria, 344 133 words — 2026-10-17; editorial parentheses removed. The main Zohar on disk is in editions of unknown licence and was not used for counting. Bavli, Yerushalmi and Onkelos counts as in the other folders.)

The mixture is the point: the Zohar takes its vocabulary from **Onkelos** (the literary standard), its idiom and many particles from the **Bavli**, some formulas from **Palestinian** sources (כְּמָה דְּאַתְּ אָמַר, אוּף, בְּגִין, לוֹן), and its syntax and many meanings from **medieval Hebrew** (Scholem 1941; Liebes 1976).

## 3. The files of this folder

| file | what |
|---|---|
| [README.md](README.md) | this overview |
| [grammar.md](grammar.md) | spelling, nouns and states, pronouns (the copula אִיהוּ), the verb and its stems, syntax; the "errors" and innovations, with examples |
| [style-formulas.md](style-formulas.md) | the formulas that structure a Zohar homily: about 60, glossed, with function and counts |
| [lexicon.md](lexicon.md) | the technical vocabulary (names of the sefirot and of God, cosmological terms), neologisms, Hebrew words in Aramaic dress, loanwords |
| [texts.md](texts.md) | annotated passages: the opening of the Zohar (1:1a), "at the head of the king's authority" (1:15a), *Patah Eliyahu* (Tiqqunei ha-Zohar, introduction), *Berikh shemeh* (2:206a), the start of the Idra Rabba (3:127b) |
| [mova.md](mova.md) | what Mova would need |
| [sources.md](sources.md) | sources and wanted works |

The Elamur team
