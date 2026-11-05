# 11. Modern Hebrew in Universal Dependencies

How Modern Hebrew is cut into words and annotated in Universal Dependencies (UD): the segmentation of prefixes and suffixes into syntactic words, multiword tokens, the tags and features, the relations specific to Hebrew (`compound:smixut`, `case:acc`, `case:gen`, `nsubj:cop`, `mark:q`, `compound:affix`), and the four UD Hebrew treebanks — HTB, IAHLTwiki, IAHLTknesset and PostRab — with what each one does differently. The counts in this chapter were made on the UD 2.18 release files on 2026-10-17 (see [§ 11.9](#119-how-the-counts-were-made)).

[← README](README.md) · [← 10 Historical layers](10-historical-layers.md) · [12 Mova and Hebrew →](12-mova.md)

Related chapters: the orthographic facts that make segmentation hard are in [02 Script and spelling](02-script-and-spelling.md); the construct state in [04 Nouns](04-nouns-adjectives-pronouns.md); the inflected prepositions in [06 Prepositions](06-prepositions-particles.md); the copula and existentials in [07 Syntax](07-syntax.md).

---

## Contents

- [11.1 The problem: one space-delimited token, several words](#111-the-problem-one-space-delimited-token-several-words)
- [11.2 Tokens, words and multiword tokens](#112-tokens-words-and-multiword-tokens)
- [11.3 Two tokenization schemes: old HTB and IAHLT](#113-two-tokenization-schemes-old-htb-and-iahlt)
- [11.4 Parts of speech](#114-parts-of-speech)
- [11.5 Features](#115-features)
- [11.6 Relations](#116-relations)
- [11.7 Constructions, one by one](#117-constructions-one-by-one)
- [11.8 The treebanks](#118-the-treebanks)
- [11.9 How the counts were made](#119-how-the-counts-were-made)
- [11.10 Parsing Hebrew: the state of the art](#1110-parsing-hebrew-the-state-of-the-art)
- [11.11 Pitfalls for a converter](#1111-pitfalls-for-a-converter)
- [References](#references)

---

## 11.1 The problem: one space-delimited token, several words

Hebrew writes a closed set of function words as prefixes on the next word, and pronouns as suffixes on prepositions, nouns and (in formal style) verbs. The prefixes are the seven letters remembered by the mnemonic **מש״ה וכל״ב** *Moshe ve-Khalev* "Moses and Caleb": מ *mi-* "from", ש *she-* "that", ה *ha-* "the", ו *ve-* "and", כ *ke-* "as", ל *le-* "to", ב *be-* "in" (UD Hebrew guidelines, Tokenization; the same mnemonic is used in Hebrew school grammar). A single orthographic word can therefore hold a conjunction, a subordinator, a preposition, the article, a content word and a pronoun:

```
וכשבביתו
ve-khshe-ba-béyt-o
and-when-in.DEF?-house.CS-3MS.POSS
'and when in his house'
```

The real difficulty is not the length of the chain but its **ambiguity**. Unpointed text does not show vowels, so the same letters can be cut in different places. The textbook case from Adler and Elhadad (2006), repeated by Zeldes et al. (2022: 4331), is the four letters בצלם:

| Cut | Reading | Gloss |
|---|---|---|
| ב־צל־ם | *be-tsil-am* | in-shadow-their |
| ב־צלם | *be-tsélem* / *ba-tsélem* | in-(a/the) image |
| ב־צלם | *be-tsalam* / *ba-tsalam* | in-(a/the) photographer |
| בצל־ם | *betsal-am* | onion-their |
| בצלם | *Betsélem* | B'Tselem (an organization) |

Two further facts make Hebrew segmentation **non-concatenative** in the sense of the older treebank:

1. **The covert article.** After ב, ל, כ the article ה is not written; its vowel fuses into the preposition: בבית is *be-váyit* "in a house" or *ba-báyit* "in the house". The letters do not tell which (UD Hebrew guidelines; see [02](02-script-and-spelling.md) and [06](06-prepositions-particles.md)).
2. **Fused pronouns.** בית + הוא is written ביתו *beyt-o* "his house"; אותו *ot-o* "him" is the accusative marker את + הוא; לו *l-o* "to him" is ל + הוא. The surface letters of the pronoun (ו, ה, ם, ן, נו, כם…) are not the letters of the free pronoun.

Tsarfaty (2019: "What's wrong with Hebrew NLP") argues that this is why pipelines built for English fail on Hebrew: a tagger cannot tag what has not been segmented, and segmentation cannot be decided without syntax. The Hebrew NLP tradition therefore built **joint** morphological-syntactic models (Tsarfaty 2006; Cohen and Smith 2007; Goldberg and Tsarfaty 2008; Goldberg and Elhadad 2011; More et al. 2019) and lattice-based analysis where every possible segmentation is kept until the parser decides.

## 11.2 Tokens, words and multiword tokens

UD separates the **token** (what stands between spaces and punctuation) from the **syntactic word** (what gets a POS tag and a node in the tree). A token made of several words is a **multiword token** (MWT); in CoNLL-U it is a range line `3-4` followed by the word lines `3` and `4`.

| Treebank (UD 2.18) | Tokens | Words | MWTs | Words per token |
|---|---:|---:|---:|---:|
| HTB | 114 648 | 160 195 | 36 783 | 1.40 |
| IAHLTwiki | 103 395 | 140 961 | 34 692 | 1.36 |
| IAHLTknesset | 50 499 | 67 007 | 15 377 | 1.33 |
| PostRab (test only) | 5 288 | 8 029 | 2 438 | 1.52 |

Source: the treebank cards in `treebanks/cards.tsv` (Mova repo, generated from the UD 2.18 `stats.xml`) and our own count. About **one token in three** in contemporary Hebrew is a multiword token. Spoken Knesset Hebrew has fewer than Wikipedia (more pronouns as free words, fewer bound possessives), and the older, more literary PostRab texts have the most.

The shapes of the MWTs in IAHLTwiki train, by the UPOS of their parts (count 2026-10-17):

| Parts | Count | Example |
|---|---:|---|
| ADP NOUN | 7 451 | בבית *ba-báyit* "in the house" |
| DET NOUN | 6 815 | הבית *ha-báyit* "the house" |
| DET ADJ | 2 918 | הגדול *ha-gadol* "the big" |
| SCONJ VERB | 1 768 | שהיה *she-haya* "that was" |
| ADP PROPN | 1 307 | בירושלים *bi-yerushaláyim* "in Jerusalem" |
| DET PROPN | 1 098 | הכנסת *ha-kenéset* "the Knesset" |
| ADP PRON | 1 054 | לו *lo* "to him" |
| NOUN PRON | 970 | ביתו *beyt-o* "his house" |
| CCONJ VERB | 641 | והיה *ve-haya* "and was" |
| CCONJ NOUN | 628 | ובית *u-váyit* "and a house" |
| ADP NOUN PRON | 586 | בביתו *be-veyt-o* "in his house" |
| CCONJ ADV | 346 | וגם *ve-gam* "and also" |
| CCONJ ADP NOUN | 288 | ובבית *u-va-báyit* "and in the house" |
| CCONJ DET NOUN | 249 | והבית *ve-ha-báyit* "and the house" |
| ADP DET NOUN | 231 | מהבית *me-ha-báyit* "from the house" |
| SCONJ ADP PRON | 141 | שבו *she-bo* "in which" |

Note on the last row: שבו *she-bo* (lit. "that in-it") is the relative marker with a resumptive pronoun on a preposition. The possessive של "of" is one word in IAHLT, so שלו "his" is ADP + PRON ([07 Syntax](07-syntax.md), relative clauses).

Note that ADP DET NOUN (231) is rare beside ADP NOUN (7 451). This is the covert article: in IAHLT only מ "from" keeps a written ה (מהבית), while ב/ל/כ swallow it and the definiteness moves to a feature (§ 11.3).

## 11.3 Two tokenization schemes: old HTB and IAHLT

UD Hebrew has two schemes in its releases, and a converter has to know which one a file follows.

### The old HTB scheme (UD_Hebrew-HTB as released in UD 2.18)

The HTB was built by MILA from 1990–91 issues of the daily *Ha'aretz* as a constituency treebank (Sima'an et al. 2001), turned into dependencies by Goldberg and Elhadad (2009), brought into early UD by Tsarfaty (2013) and into UD v2 by Sade et al. (2018) (Zeldes et al. 2022: § 2). Its tokenization **restores** elements that are not written:

- the covert article becomes a word of its own, with an underscore on the side where it fused: ב + **ה_** + בית;
- a fused possessive becomes noun + **_של_** + pronoun (*house _of_ him*);
- a fused object pronoun becomes verb + **את_** + **_הוא**;
- the pronoun keeps the lemma of the free pronoun and the form gets an underscore: **_הוא** for the ו of ביתו.

In the UD 2.18 HTB train file we count **2 547** explicit `ה_` article words. The guideline page for Hebrew still describes this scheme ("Tokenization and Word Segmentation"; UD Hebrew docs, `_he/index.md`).

An example of the shape (HTB is CC BY-NC-SA, so we describe and cite only a fragment of one tree, `he_htb-ud-train.conllu`):

```
14-16  בידיים
14     ב       ADP   case
15     ה_      DET   det      PronType=Art
16     ידיים   NOUN  obl      Gender=Fem|Number=Plur
```

*ba-yadáyim* "with the hands": three words for a token of two written morphemes.

### The IAHLT scheme (IAHLTwiki, IAHLTknesset, and the revised HTB on GitHub)

Zeldes, Howell, Ordan and Ben Moshe (2022) changed the scheme so that **an MWT is always the concatenation of its parts**: nothing is inserted. Their three examples (2022: 4333):

| Old HTB | IAHLT | Token |
|---|---|---|
| בית + _של_ + _הוא | בית + ו | ביתו "his house" |
| ראיתי + _את_ + _היא | ראיתי + ה | ראיתיה "I saw her" |
| ב + _ה_ + בית | ב + בית | בבית "in the house" |

The covert article is the only change that loses information, so it is kept as a **feature on the preposition**: `Definite=Def|PronType=Art` on ב/ל/כ. We count **2 241** such prepositions in IAHLTwiki train. The pronoun suffix keeps the lemma of the free pronoun (ו → הוא) and carries `Poss=Yes|Case=Gen` for a possessive.

The IAHLTknesset example from its README (CC BY-SA 4.0; `# sent_id = 13_ptm_532410-8`) shows both points: בעולם is split into `ב` (ADP, `Definite=Def|PronType=Art`) + `עולם`, and לו into `ל` + `ו` (lemma הוא).

The authors give three reasons for the change: the new scheme is simpler, it matches UD Arabic and the other Semitic treebanks, and it allows character-level segmenters that only decide "does a new word start here?" (Zeldes 2018; Zeldes et al. 2022: § 4). Converting between the two is deterministic in both directions (2022: 4334) — a fact a Mova converter can use (§ 11.11).

The IAHLT project also released a **revised HTB** in the new scheme (https://github.com/IAHLT/UD_Hebrew), valid for UD 2.10. The official `UD_Hebrew-HTB` in UD 2.18 has **not** taken it: our count of its files still finds `ה_`, `case:acc` (1 946) and `case:gen` (4 812). Whoever trains on "HTB" must say which HTB.

## 11.4 Parts of speech

UD Hebrew uses all universal tags except PART (guidelines). The points where Hebrew practice is not obvious:

| Item | UPOS | Note |
|---|---|---|
| ה "the" | DET | `PronType=Art`, `Definite=Def` |
| כל "every, all", הרבה "many", אילו "which" | DET | quantifiers and interrogative determiners |
| זה, זאת, אלה "this, these" | PRON | demonstratives are PRON, not DET (guidelines), with `PronType=Dem` |
| של "of" | ADP | `Case=Gen` |
| את (object marker) | ADP | `Case=Acc` (HTB always; IAHLT on most tokens) |
| ש "that, which", כש "when" | SCONJ | כש has `Case=Tem` in HTB |
| יש / אין "there is / there is not" | VERB | existentials; HTB adds `HebExistential=Yes` |
| היה "be" in a copular or periphrastic use | AUX | `VerbType=Cop`; as an existential it is VERB |
| יכול, צריך, חייב, אמור, עלול… | AUX | modals, `VerbType=Mod`; the guidelines note that their class is debated |
| הוא, היא, הם as a present copula | PRON | relation `cop`; see § 11.7.2 |
| participles (כותב "writes / writer") | VERB / NOUN / ADJ | VERB when it has complements, NOUN or ADJ otherwise (guidelines) |
| אנטי, מולטי, בלתי, אי as prefixes | ADV | `Prefix=Yes`, relation `compound:affix` |

UPOS frequencies (all splits, 2026-10-17):

| UPOS | HTB | IAHLTwiki | IAHLTknesset | PostRab |
|---|---:|---:|---:|---:|
| NOUN | 38 046 | 34 627 | 12 992 | 1 819 |
| ADP | 26 586 | 23 009 | 9 066 | 1 197 |
| PUNCT | 18 099 | 16 616 | 8 071 | — |
| DET | 17 281 | 14 177 | 6 476 | 636 |
| VERB | 14 288 | 10 645 | 7 367 | 1 157 |
| ADJ | 8 416 | 8 711 | 3 090 | 256 |
| PROPN | 7 912 | 10 447 | 2 345 | 142 |
| PRON | 7 706 | 5 633 | 6 362 | 938 |
| ADV | 6 551 | 3 528 | 4 292 | 435 |
| SCONJ | 5 520 | 4 319 | 2 935 | 546 |
| CCONJ | 5 121 | 4 707 | 2 467 | 657 |
| NUM | 3 288 | 3 127 | 695 | 144 |
| AUX | 1 239 | 961 | 713 | 98 |
| X | 139 | 304 | 60 | — |
| SYM | — | 146 | 40 | — |
| INTJ | 3 | 4 | 36 | 4 |

The genre shows plainly: spoken Knesset Hebrew has more than twice the share of PRON, VERB and ADV of Wikipedia, and nine times as many INTJ; Wikipedia has the most PROPN. PostRab has no punctuation in the text.

The PROPN column is not fully comparable: Zeldes et al. (2022: § 5) found that HTB tags transparent place names such as תל אביב *tel aviv* or כפר סבא *kfar saba* as NOUN, while IAHLT tags them PROPN; likewise דומה *dome* "similar" and שונה *shone* "different" are participles (VERB) in HTB but ADJ in IAHLT.

## 11.5 Features

The universal features used, and the Hebrew-specific ones. Frequencies are for the **train** split (2026-10-17).

| Feature | Values | HTB train | IAHLTwiki train | Note |
|---|---|---:|---:|---|
| Gender | Masc, Fem, Fem,Masc | 35 264 / 18 977 / 2 710 | 32 002 / 18 797 / 159 | Fem,Masc for forms unmarked for gender (1st person verbs, some nouns) |
| Number | Sing, Plur, Dual, Dual,Plur, Plur,Sing | 41 344 / 15 375 / 78 / 5 / 142 | 36 885 / 13 755 / 23 / — / 6 | Dual for שנתיים "two years"; Dual,Plur for pluralia like שוליים "margins" (guidelines) |
| Person | 1, 2, 3, 1,2,3 | 778 / 171 / 12 523 / 3 657 | 196 / 112 / 12 367 / — | 1,2,3 for present participles (HTB only) |
| Definite | Def, Cons | 2 246 / 9 125 | 16 074 / 9 264 | `Cons` marks the first member of a construct; IAHLT also puts `Def` on the article and on prepositions with a covert article |
| Case | Acc, Gen, Tem | 1 781 / 4 173 / 130 | 727 / 3 892 / — | on את, של, כש |
| Tense | Past, Fut, Pres | 5 323 / 1 220 / — | 5 551 / 350 / 2 145 | HTB leaves the present unmarked; IAHLT writes `Tense=Pres` on participles in verbal use |
| VerbForm | Part, Inf | 4 019 / 2 202 | 2 238 / 1 301 | |
| Mood | Imp | 30 | 32 | imperatives are rare in written text |
| Voice | Act, Pass, Mid | 7 537 / 861 / 1 257 | 5 878 / 1 574 / 1 208 | Mid = unaccusative / middle (nif'al, hitpa'el readings) |
| HebBinyan | PAAL, NIFAL, PIEL, PUAL, HIFIL, HUFAL, HITPAEL, NITPAEL | see below | see below | the verb's pattern ([05 Verbs](05-verbs.md)) |
| HebExistential | Yes | 462 | — | HTB only |
| Polarity | Pos, Neg | 1 187 / 1 147 | 813 / 594 | on copulas and existentials: יש Pos, אין Neg |
| VerbType | Cop, Mod | 978 / — | 240 / 121 | |
| PronType | Art, Prs, Dem, Int, Ind, Emp, Tot, Neg | 13 967 / 4 992 / 1 010 / 362 / 26 / 39 / — / — | 14 131 / 3 693 / 919 / 19 / 16 / 42 / 221 / 4 | IAHLT adds Tot (כל) and Neg |
| NumType | Card, Ord | — | 666 / 343 | IAHLT only |
| Poss | Yes | — | 1 788 | IAHLT: on possessive suffix pronouns |
| Reflex | Yes | 147 | 58 | עצמו "himself" |
| Abbr | Yes | 876 | 361 | ח״כ, ארה״ב, ע״י |
| Prefix | Yes | 196 | 103 | bound loan prefixes |
| ExtPos | ADP, SCONJ, ADV, CCONJ | — | 804 / 223 / 165 / 7 | IAHLT: on the head of a `fixed` expression, e.g. על ידי "by" = ADP |
| Aspect | Prog | — | 42 | IAHLT only, rare |
| Foreign, Typo | Yes | — | 95, 56 | IAHLT only |

HebBinyan counts over all splits (2026-10-17):

| Binyan | HTB | IAHLTwiki | IAHLTknesset |
|---|---:|---:|---:|
| PAAL | 4 152 | 3 924 | 3 571 |
| HIFIL | 2 544 | 1 853 | 1 332 |
| PIEL | 2 068 | 1 641 | 1 125 |
| NIFAL | 1 470 | 1 538 | 469 |
| HITPAEL | 1 083 | 717 | 433 |
| HUFAL | 559 | 600 | 76 |
| PUAL | 506 | 563 | 128 |
| NITPAEL | — | 17 | 4 |

The passive binyanim huf'al and pu'al are about seven times rarer in parliamentary speech than in Wikipedia — a register fact ([05 Verbs](05-verbs.md), [08 Register](08-lexicon-register.md)). IAHLT adds NITPAEL for the literary nitpa'el (e.g. נתקבל *nitkabel* "was received"), a Mishnaic form ([10](10-historical-layers.md)).

The UD guideline page for HebBinyan (`_he/feat/HebBinyan.md`) lists the seven classic binyanim with an example of each and their typical meanings; the IAHLT data shows that the inventory had to be extended.

## 11.6 Relations

### Hebrew subtypes

From the guidelines (`_he/index.md`, `_he/dep/*.md`) and the data:

| Relation | Meaning | HTB | IAHLTwiki | IAHLTknesset | PostRab |
|---|---|---:|---:|---:|---:|
| compound:smixut | dependent of a construct noun | 8 624 | — (plain `compound` 10 381) | — (`compound` 3 137) | 391 |
| case:acc | the object marker את | 1 946 | — | — | 3 |
| case:gen | the genitive של | 4 812 | — | — | 4 |
| nsubj:cop | subject of a copular clause | 360 | — | — | 138 |
| mark:q | yes/no question marker האם | 41 | — | — | — |
| compound:affix | bound loan prefix + its host | 235 | 211 | 67 | — |
| flat:name | second part of a name | 2 683 | — (plain `flat` 1 870) | — (`flat` 307) | — (`flat` 33) |
| nmod:poss | possessor (suffix or של) | 4 071 | 4 459 | 1 611 | 259 |
| nmod:unmarked, obl:unmarked | bare nominal modifiers (time, measure) | — | 402, 134 | 46, 170 | 2, 27 |
| nsubj:outer, csubj:outer | outer subject in a nested predication | 50 | 92, 2 | 74, 1 | 10 |
| compound:svc, compound:redup | serial verbs, reduplication | — | — | — | 8, 2 |

The **IAHLT scheme drops the subtypes that are predictable from the words** they join — `case:acc` (always את), `case:gen` (always של), `mark:q` (always האם) — and writes plain `compound` for the construct (Zeldes et al. 2022: 4334). The information is still in the features: `Case=Acc` on את, `Case=Gen` on של, `Definite=Cons` on the construct head. A converter between the schemes is therefore a rewrite by lemma and feature (§ 11.11).

Relations not used in Hebrew at all, per the guidelines: `clf` and `expl` (we still count 1 and 4 `expl` in IAHLT, 4 `iobj` in HTB and 7 in IAHLTknesset — leftovers or edge cases).

### Most frequent relations

| Rank | HTB | IAHLTwiki | IAHLTknesset |
|---:|---|---|---|
| 1 | case 19 484 | case 22 535 | case 8 887 |
| 2 | punct 18 097 | punct 16 616 | punct 8 070 |
| 3 | det 17 693 | det 14 816 | det 6 962 |
| 4 | obl 12 025 | obl 10 857 | nsubj 4 522 |
| 5 | compound:smixut 8 624 | compound 10 381 | obl 4 418 |
| 6 | nsubj 8 479 | amod 7 443 | advmod 3 857 |
| 7 | nmod 7 137 | nmod 6 216 | compound 3 137 |
| 8 | amod 6 955 | nsubj 5 894 | conj 2 979 |
| 9 | root 6 143 | conj 5 753 | mark 2 895 |
| 10 | conj 5 405 | root 5 039 | root 2 883 |

Two things stand out. First, the construct state is the fifth most frequent relation in written Hebrew: about one word in every 14–16. Second, `det` is third everywhere because the article ה is a separate word in UD. The spoken corpus also has 228 `vocative` (forms of address to the chair: *adoni ha-yoshev rosh* "Mr. Chairman") against 1 in Wikipedia.

## 11.7 Constructions, one by one

### 11.7.1 The construct state (smixut)

The first noun of a construct is in a bound form and carries `Definite=Cons`; the second noun depends on it with `compound:smixut` (HTB, PostRab) or `compound` (IAHLT). Definiteness is marked once, on the last member, and spreads to the whole chain ([04](04-nouns-adjectives-pronouns.md)).

The UD Hebrew page `compound:smixut` (`_he/dep/compound-smixut.md`) defines it as the relation for "a noun modifying another noun in a construct state". The choice of `compound` (and not `nmod`) is a convention of UD Hebrew: the construct behaves like a single noun phrase head, cannot be interrupted by an adjective, and often lexicalises (בית ספר *beyt séfer* "school", lit. "house of book"). A construct whose second member is a pronoun suffix (ביתו) is instead `nmod:poss`.

Annotated example (IAHLTwiki, CC BY-SA 4.0, `iahltwiki_penicillin-88`):

```
# text = בנזילפניצילין הוא טיפול הבחירה לעגבת בכל שלביה.
1      בנזילפניצילין  NOUN  3  nsubj
2      הוא           PRON  3  cop        Polarity=Pos|PronType=Prs
3      טיפול         NOUN  0  root
4-5    הבחירה
4      ה             DET   5  det
5      בחירה         NOUN  3  compound
6-7    לעגבת
6      ל             ADP   7  case
7      עגבת          NOUN  3  nmod
8-9    בכל
8      ב             ADP   10 case
9      כל            DET   10 det        Definite=Cons|PronType=Tot
10-11  שלביה
10     שלבי          NOUN  3  nmod
11     ה             PRON  10 nmod:poss  Case=Gen|Poss=Yes|Person=3|Gender=Fem
12     .             PUNCT 3  punct
```

*benzilpenitsilin hu tipul ha-bkhira le-agévet be-khol shlav-éha* "Benzylpenicillin is the treatment of choice for syphilis in all its stages." Note three things: טיפול הבחירה "the treatment of choice" is a construct with the article on the second member (`compound`); כל "all" is itself in the construct (`Definite=Cons`) before its noun; and שלביה "its stages" is the construct plural שלבי + the feminine pronoun suffix (antecedent עגבת, feminine).

In the HTB and IAHLTknesset examples on the treebank card, the same pattern appears with ראש עיר *rosh ir* "mayor" (lit. "head of city": ראש `Definite=Cons`, עיר `compound`) and מדינת ישראל *medinat yisra'el* "the State of Israel" (מדינת is the construct of מדינה).

**Counted:** `Definite=Cons` is on 10 801 words of HTB, 11 014 of IAHLTwiki and 3 789 of IAHLTknesset — 6.7%, 7.8% and 5.7% of all words. Spoken Hebrew uses the construct less and the analytic של more ([04](04-nouns-adjectives-pronouns.md)).

### 11.7.2 Copular clauses

Present-tense Hebrew needs no verb "to be": הבית גדול *ha-báyit gadol* "the house [is] big" is a full clause with the adjective as root and הבית as `nsubj`. When a copula appears, UD follows its general rule: the **predicate** is the head and the copula depends on it with `cop`.

Hebrew has several copulas (guidelines, *Non-verbal clauses*):

| Copula | Tense | UPOS (IAHLT) | `cop` count, both IAHLT |
|---|---|---|---:|
| היה and its forms | past, future, infinitive | AUX | 768 |
| הוא / היא / הם / הן (pronominal) | present | PRON | 570 |
| זה (demonstrative, does not agree) | present, colloquial | PRON | 69 |
| אינו / איננו (negative) | present, formal | AUX | 57 |
| הינו / הנו (positive, formal) | present | AUX | (rare) |

Both schemes tag the pronominal copula PRON (HTB: 380 הוא/PRON as `cop`, beside 588 היה, 289 אינו and 5 הינו as AUX). They differ in the subject: HTB labels it `nsubj:cop` (360), IAHLT plain `nsubj`; IAHLT also adds `Polarity=Pos` to the pronoun. The guideline page remarks that זה as a copula ("chocolate זה tasty") is probably modelled on Slavic *to/èto* — a claim we report, not verify.

Example (IAHLTwiki, `iahltwiki_mammography-58`):

```
# text = השיטה הדיגיטלית היא המקובלת כיום.
2   שיטה      NOUN  7  nsubj
4   דיגיטלית  ADJ   2  amod
5   היא       PRON  7  cop    Gender=Fem|Number=Sing|Person=3|Polarity=Pos
7   מקובלת    ADJ   0  root
8   כיום      ADV   7  advmod
```

*ha-shitá ha-digitálit hi ha-mekubélet kayom* "The digital method is the accepted one today." The copula היא agrees with the subject (feminine singular). It is near-obligatory here because both sides are definite: without it, השיטה הדיגיטלית המקובלת would read as one noun phrase "the accepted digital method" ([07 Syntax](07-syntax.md)).

### 11.7.3 Existentials and possession: יש / אין

יש *yesh* "there is" and אין *eyn* "there is not" are tagged VERB (not AUX) and head their clause; the existing thing is `nsubj` in IAHLT. In the past and future the existential is a form of היה (היה, היתה/הייתה, יהיה), also VERB. HTB marks all of them `HebExistential=Yes` (462 in train); IAHLT marks `Polarity` and adds a construction label in MISC (§ 11.7.7).

Possession is the existential with ל: יש לי ספר *yesh li séfer* "I have a book" (lit. "there is to-me a book"). The possessor is `obl` (ל + pronoun). The guideline page notes that the possessed is grammatically a subject in formal Hebrew but behaves like an object in the colloquial יש לי **את** הספר "I have the book" ([07 Syntax](07-syntax.md)).

The existential sentence often has the order V-obl-S ("there-is to-him the-skill…"; guidelines, *Word order*).

Example (IAHLTwiki, `iahltwiki_glasgow-coma-scale-13`):

```
# text = ישנן ארבע רמות:
1  ישנן   יש    VERB  0  root    Gender=Fem|Number=Plur|Person=3|Tense=Pres   Cxn=Existential-ExistPred-VblPart
2  ארבע   ארבע  NUM   3  nummod
3  רמות   רמה   NOUN  1  nsubj
```

*yeshnan arba ramot* "There are four levels:". ישנן is the formal inflected יש (lemma יש, feminine plural to agree with רמות) — a form that exists for יש but not in colloquial speech ([07](07-syntax.md)).

The IAHLTknesset card sentence `13_ptm_532410-8` shows אין as root with נבחר ציבור "elected official" as `nsubj`, and a relative clause שיש לו סמכויות "who has powers" with יש as `acl:relcl`.

### 11.7.4 Objects and את

The object marker את precedes **definite** direct objects ([07](07-syntax.md), differential object marking). It is ADP with `Case=Acc` and depends on the object noun: `case:acc` in HTB (1 946 in all splits, plus 41 plain `case` in train), plain `case` in IAHLT. With a pronoun it fuses: אותי *oti* "me", אותו *oto* "him" — split as את + pronoun (HTB `את_` + `_הוא`).

A peculiarity of IAHLTwiki train: 568 standalone את carry `Case=Acc`, but 622 carry no feature at all. The feature is therefore not a reliable cue in that treebank; the lemma is.

Inflected verbs with an object suffix (ראיתיה "I saw her", הרגו "he killed him") are literary. IAHLT splits them into verb + pronoun with no inserted את.

### 11.7.5 Relative clauses with ש

The relative marker ש is SCONJ, a prefix on the first word of the clause, and depends on the clause's predicate with `mark`; the clause attaches to its noun with `acl:relcl` (2 969 HTB, 3 065 IAHLTwiki, 1 403 IAHLTknesset). The formal אשר is a separate word with the same analysis. A resumptive pronoun inside the clause (שבו "in which", lit. "that in-it") gets its normal relation (`obl` with its `case`). Free relatives (מי ש "whoever", מה ש "what") are headed by מי/מה.

The guideline example for a participial subject — המגדל נחש בביתו "whoever raises a snake at home" (literally "the raising a snake in his house") — shows the article ה used as a relative marker on a participle, a construction from earlier Hebrew still used in formal writing ([10](10-historical-layers.md)).

### 11.7.6 Questions and negation

- **Yes/no questions** in speech are marked only by intonation; formal writing adds האם. HTB labels האם `mark:q` (41); IAHLT gives it plain `mark`. Embedded questions use the same האם ("whether").
- **Wh-words** are PRON (מי, מה), ADV (איפה, מתי, למה, איך) or DET (איזה, כמה) with `PronType=Int`.
- **Negation**: לא is ADV with `advmod`; אין as an existential is VERB; אינו as a negative copula is AUX (`cop`) and as a negator of a participle (אינו יודע "does not know") is AUX (`aux`, 141 in IAHLT). The prohibitive אל + future (אל תדאג "don't worry") is ADV `advmod`.

### 11.7.7 Construction annotation (Cxn) in IAHLT

IAHLT adds labels from the UD construction layer (UCxn) in MISC: `Cxn=` on the head and `CxnElt=` on the parts. Counts over both IAHLT treebanks (2026-10-17):

| Cxn | Count |
|---|---:|
| Conditional-NeutralEpistemic | 174 |
| Existential-ExistPred-VblPart | 102 |
| Interrogative-WHInfo-Direct | 98 |
| Interrogative-Polar-Direct | 86 |
| Interrogative-WHInfo-Indirect | 59 |
| Existential-CopPred | 44 |
| Existential-ExistPred-FullVerb | 37 |
| Existential-NotExistPred-VblPart | 34 |
| NPN | 15 |

`VblPart` means the existential is the particle יש/אין; `FullVerb` means a form of היה or a verb like קיים "exist"; `CopPred` means the existential is expressed by a copula. NPN covers patterns like יום יום "day by day" or פנים אל פנים "face to face". These labels are exactly the kind of enrichment the Mova dialect keeps in MISC (`docs/dialect.md`).

### 11.7.8 Named entities

Both IAHLT treebanks carry nested named entities in MISC (`Entity=(PER…PER)`) with twelve types: ANG (language), DUC (product), EVE, FAC, GPE, LOC, ORG, PER, TIMEX, TTL (title), WOA (work of art), MISC (treebank READMEs). The IAHLTwiki README example (`iahltwiki_piskat-hitgabrut-27`) nests an ORG בית המשפט העליון "the Supreme Court" inside a TTL נשיא בית המשפט העליון "President of the Supreme Court".

## 11.8 The treebanks

Cards with the full metadata are in the Mova repo at [`treebanks/INDEX.md`](../../treebanks/INDEX.md).

| | HTB | IAHLTwiki | IAHLTknesset | PostRab |
|---|---|---|---|---|
| Text | *Ha'aretz* news, 1990–91 | Hebrew Wikipedia, 39 articles in 7 domains | Knesset proceedings (transcribed speech), 100 discussions | post-Rabbinic texts: Rashi, Shulhan Arukh, Sefer ha-Hinukh, Maharil, Igeret Orhot Olam, misc. |
| Sentences / words | 6 143 / 160 195 | 5 039 / 140 961 | 2 883 / 67 007 | 296 / 8 029 |
| Splits | train/dev/test | train/dev/test | train/dev/test | test only (in UD 2.18) |
| Annotation | converted from constituency (MILA) | native UD, manual | native UD, manual | native UD, manual |
| Tokenization | old (inserted ה_, של_, את_) | IAHLT (concatenative) | IAHLT | close to IAHLT |
| Construct | compound:smixut | compound | compound | compound:smixut |
| Extras | XPOS (= UPOS) | Entity, Cxn | Entity, Cxn, speaker metadata | source metadata |
| In UD since | v1.1 | v2.10 | v2.15 | v2.18 |
| Licence | **CC BY-NC-SA 4.0** | CC BY-SA 4.0 | CC BY-SA 4.0 | CC BY-SA 4.0 |
| Paper | Sima'an et al. 2001; Tsarfaty 2013; Sade et al. 2018 | Zeldes et al. 2022 | Zeldes et al. 2022; Goldin et al. 2024 | Tal et al. 2025 |

**HTB.** The historical benchmark. Its weaknesses are age (pre-internet: Zeldes et al. 2022 count about a dozen mentions of "computer" and none of mobile phones), a single newspaper's house style (היתה, never the common spelling הייתה "was"), and conversion errors flagged in MISC as `HebSource=ConvUncertainHead|ConvUncertainLabel` (guidelines). For Mova it is **NC**: tests and measurements only, never in what is built (`licence/DATA.md`).

**IAHLTwiki.** The first Hebrew treebank annotated natively in UD. Inter-annotator agreement on ~440 double-annotated sentences: word segmentation 99.1%, Cohen's κ lemma 98.5, UPOS 94.8, FEATS 90.9, head 95.8, deprel 95.7 (Zeldes et al. 2022: Table 3). Annotated in Arborator-Grew and continuously checked by *grewv*, a validator of Grew patterns written as Hebrew-specific rules.

**IAHLTknesset.** Spoken, parliamentary Hebrew: speaker turns with `# speaker`, `# turnnumber`, `# is_chairman`. Drawn from the Knesset Corpus (Goldin et al. 2024), which itself holds tens of millions of sentences of proceedings since 1992 with automatic morphology. It is the only UD Hebrew corpus with conversational features: vocatives, interjections, code-switching (the card's example has English *balance* as NOUN in a Hebrew sentence).

**PostRab.** Not Modern Hebrew: legal, exegetical and other texts from the medieval and early-modern periods, with adaptations "to account for differences in orthography, morphology, and syntax relative to modern Hebrew" (treebank README). The paper reports over 11 800 words (Tal et al. 2025); UD 2.18 holds 8 029 words as a test set. It is relevant here for two reasons: it shows where Modern Hebrew parsers break on older Hebrew ([10](10-historical-layers.md)), and it uses `compound:svc` and `compound:redup` which the modern treebanks do not.

**Not in UD.** The Ancient Hebrew treebank PTNK (`hbo`, Biblical Hebrew) is a separate language in UD; see the Ancient Hebrew manual (`../hbo/`, if present) and `tools/macula` in the Mova repo. Earlier Hebrew dependency resources (Goldberg and Elhadad 2009's scheme; the SPMRL 2013/2014 shared-task Hebrew data) are not UD.

## 11.9 How the counts were made

All counts above were made on 2026-10-17 from the UD 2.18 CoNLL-U files of the four treebanks (`UD_Hebrew-HTB`, `-IAHLTwiki`, `-IAHLTknesset`, `-PostRab`), by streaming the files and counting word lines (integer ids), range lines (MWTs), and the UPOS, FEATS and DEPREL columns. "All splits" means train + dev + test; "train" means the train file only. Token and sentence totals come from the treebank cards, which check their counts against each release's `stats.xml`. The counts are simple, deterministic tallies; a recount on the same files gives the same numbers.

## 11.10 Parsing Hebrew: the state of the art

End-to-end scores from plain text (gold sentence splits), from Zeldes et al. (2022: Table 4); MWT F1 is segmentation, the others are aligned accuracy:

| Train → test | System | MWT F1 | UPOS | LAS |
|---|---|---:|---:|---:|
| HTB → HTB | Stanza | 93.97 | 96.99 | 82.67 |
| HTB → HTB | Trankit (XLM-R) | 97.27 | 97.46 | 82.82 |
| HTB → HTB | HebPipe (AlephBERT) | 98.81 | 97.34 | 89.42 |
| Wiki → Wiki | Stanza | 92.87 | 95.76 | 80.22 |
| Wiki → Wiki | Trankit | 94.64 | 96.98 | 87.65 |
| Wiki → Wiki | HebPipe | 98.78 | 97.27 | 90.01 |

Out of domain (HebPipe), LAS drops by roughly 3 to 4.5 points; a joint model trained on both corpora scores 88.29 LAS on HTB and 90.36 on Wiki (Table 6). The most confused labels across corpora are NOUN/PROPN (the name convention above) and obl/nmod (prepositional attachment) (Table 7).

The segmentation leap comes from combining a character-level classifier (RFTokenizer, Zeldes 2018: each character either begins a new word or not) with a word-level transformer that predicts whether a word has prefixes and suffixes. A further cue is the position of a word inside its MWT (B/I/E/S): the letter ו is "and" at the start of an MWT and "his" at its end (Zeldes et al. 2022: § 4).

Pretrained Hebrew encoders: AlephBERT (Seker et al. 2022), DictaBERT (Shmidman et al. 2023), and the generative DictaLM (Shmidman et al. 2024). Dicta also built Nakdan, the diacritizer (Shmidman et al. 2020). The rule-based and lexicon-based tradition before them: the MILA morphological analyser and lexicon (Itai and Wintner 2008; Itai et al. 2006), Hspell's analyser (Har'El and Kenigsberg, Hspell), the HMM disambiguator of Adler and Elhadad (2006), and the YAP joint parser (More et al. 2019).

## 11.11 Pitfalls for a converter

A list for whoever writes a Hebrew reader or converter for the Mova dialect:

1. **Know the scheme.** Old-HTB files have inserted `ה_`, `_של_`, `את_` and underscored pronoun forms; IAHLT files never insert. Detect by the presence of underscores in FORM or `case:acc` / `case:gen`.
2. **Old → new is deterministic** (Zeldes et al. 2022): drop an inserted `ה_` and put `Definite=Def|PronType=Art` on the preposition; drop an inserted `_של_` or `את_` and keep the pronoun as the dependent (`nmod:poss` / `obj`); strip the underscores.
3. **New → old** must re-insert the article from the feature, and the של/את from the relation of the suffix pronoun.
4. **Subtypes are predictable**: `case:acc` ⇔ lemma את; `case:gen` ⇔ lemma של; `mark:q` ⇔ lemma האם; `compound:smixut` ⇔ `compound` whose head has `Definite=Cons`. `flat:name` ⇔ `flat` between PROPNs.
5. **The copular subject** is `nsubj:cop` in HTB (only some: 360 of over 1 400 copulas) and `nsubj` in IAHLT; the pronominal copula is PRON in both.
6. **Present tense** is unmarked in HTB, `Tense=Pres` in IAHLT.
7. **Names**: HTB tags transparent place names NOUN; IAHLT PROPN. A converter cannot fix that by rule: it needs a gazetteer.
8. **Spelling**: HTB follows *Ha'aretz* house spelling (closer to ktiv haser, e.g. היתה), IAHLT has free web spelling (הייתה). A lexicon keyed on the spelling must hold both ([02](02-script-and-spelling.md)).
9. **Right-to-left text in a left-to-right file**: CoNLL-U stores Hebrew in logical order; viewers show it reversed or mixed with digits and Latin. Never reverse strings when reading; compare by code points. Punctuation like the gershayim ״ is often typed as ASCII `"` (צה"ל), which collides with quotation marks in TSV and JSON.
10. **Licences**: HTB is NC (tests only); IAHLT and PostRab are CC BY-SA (quote, test and learn from; do not carry into a release as modified data).

---

## References

- Adler, Meni and Michael Elhadad. 2006. An Unsupervised Morpheme-Based HMM for Hebrew Morphological Disambiguation. ACL 2006. https://aclanthology.org/P06-1084/ — local: `$RESEARCH_RAW/aclbulk-2006-adler-unsupervised-morpheme-based-hmm-hebrew/`.
- Cohen, Shay B. and Noah A. Smith. 2007. Joint Morphological and Syntactic Disambiguation. EMNLP-CoNLL 2007. https://aclanthology.org/D07-1022/ — local: `$RESEARCH_RAW/aclbulk-2007-cohen-joint-morphological-syntactic-disambiguation/`.
- Goldberg, Yoav and Michael Elhadad. 2009. Hebrew Dependency Parsing: Initial Results. IWPT 2009. https://aclanthology.org/W09-3819/ — local: `$RESEARCH_RAW/lingbulk-2009-goldberg-hebrew-dependency-parsing-initial-results/`.
- Goldberg, Yoav and Michael Elhadad. 2011. Joint Hebrew Segmentation and Parsing using a PCFGLA Lattice Parser. ACL 2011. https://aclanthology.org/P11-2124/ — local: `$RESEARCH_RAW/aclbulk-2011-goldberg-joint-hebrew-segmentation-parsing-pcfgla/`.
- Goldberg, Yoav and Reut Tsarfaty. 2008. A Single Generative Model for Joint Morphological Segmentation and Syntactic Parsing. ACL 2008. https://aclanthology.org/P08-1043/ — local: `$RESEARCH_RAW/aclbulk-2008-goldberg-single-generative-model-joint-morphological/`.
- Goldin, Gili, Nick Howell, Noam Ordan, Ella Rabinovich and Shuly Wintner. 2024. The Knesset Corpus: An Annotated Corpus of Hebrew Parliamentary Proceedings. arXiv:2405.18115. https://arxiv.org/abs/2405.18115 — local: `$RESEARCH_RAW/arxivrecent-2024-goldin-knesset-corpus-annotated-corpus-hebrew/`.
- Itai, Alon and Shuly Wintner. 2008. Language Resources for Hebrew. *Language Resources and Evaluation* 42: 75–98. (Not held; the MILA resources are described in Wintner 2003/2004, local: `$RESEARCH_RAW/lingbulk-2003-wintner-resources-processing-israeli-hebrew/`.)
- Itai, Alon, Shuly Wintner and Shlomo Yona. 2006. A Computational Lexicon of Contemporary Hebrew. LREC 2006. Local: `$RESEARCH_RAW/lingbulk-2006-itai-computational-lexicon-contemporary-hebrew/`.
- More, Amir, Amit Seker, Victoria Basmova and Reut Tsarfaty. 2019. Joint Transition-Based Models for Morpho-Syntactic Parsing: Parsing Strategies for MRLs and a Case Study from Modern Hebrew. *TACL* 7: 33–48. https://aclanthology.org/Q19-1003/ (not held locally).
- Sade, Shoval, Amit Seker and Reut Tsarfaty. 2018. The Hebrew Universal Dependency Treebank: Past, Present and Future. UDW 2018. https://aclanthology.org/W18-6016/ (not held locally).
- Seker, Amit, Elron Bandel, Dan Bareket, Idan Brusilovsky, Refael Shaked Greenfeld and Reut Tsarfaty. 2022. AlephBERT: Language Model Pre-training and Evaluation from Sub-Word to Sentence Level. ACL 2022. https://aclanthology.org/2022.acl-long.4/ — local: `$RESEARCH_RAW/acl-2022-seker-alephbert-language-model-pre-training/`.
- Shmidman, Avi, Shaltiel Shmidman, Moshe Koppel and Yoav Goldberg. 2020. Nakdan: Professional Hebrew Diacritizer. ACL 2020 demos. https://aclanthology.org/2020.acl-demos.23/ — local: `$RESEARCH_RAW/aclbulk-2020-shmidman-nakdan-professional-hebrew-diacritizer/`.
- Shmidman, Shaltiel, Avi Shmidman and Moshe Koppel. 2023. DictaBERT: A State-of-the-Art BERT Suite for Modern Hebrew. arXiv:2308.16687 — local: `$RESEARCH_RAW/arxivrecent-2023-shmidman-dictabert-state-art-bert-suite/`.
- Shmidman, Shaltiel et al. 2024. Adapting LLMs to Hebrew: Unveiling DictaLM 2.0. arXiv:2407.07080 — local: `$RESEARCH_RAW/arxivrecent-2024-shmidman-adapting-llms-hebrew-unveiling-dictalm/`.
- Sima'an, Khalil, Alon Itai, Yoad Winter, Alon Altman and Noa Nativ. 2001. Building a Tree-Bank of Modern Hebrew Text. *Traitement Automatique des Langues* 42(2). (Not held.)
- Tal, Rachel, Shlomit Fuchs, Orly Albeck, Elisheva Brauner, Avi Shmidman, Yitzchak Lindenbaum and Ephraim Meiri. 2025. A New Hebrew Universal Dependency Treebank: The First Treebank of Post-Rabbinic Historical Hebrew. TLT 2025. https://aclanthology.org/2025.tlt-1.11/
- Tsarfaty, Reut. 2006. Integrated Morphological and Syntactic Disambiguation for Modern Hebrew. COLING-ACL 2006 Student Research Workshop. https://aclanthology.org/P06-3009/ — local: `$RESEARCH_RAW/aclbulk-2006-tsarfaty-integrated-morphological-syntactic-disambiguation-modern/`.
- Tsarfaty, Reut. 2013. A Unified Morpho-Syntactic Scheme of Stanford Dependencies. ACL 2013. https://aclanthology.org/P13-2103/
- Tsarfaty, Reut, Shoval Sadde, Stav Klein and Amit Seker. 2019. What's Wrong with Hebrew NLP? And How to Make it Right. EMNLP-IJCNLP 2019 demos. https://aclanthology.org/D19-3044/ — local: `$RESEARCH_RAW/aclbulk-2019-tsarfaty-what-s-wrong-hebrew-nlp/`.
- Tsarfaty, Reut, Djamé Seddah, Sandra Kübler and Joakim Nivre. 2013. Parsing Morphologically Rich Languages: Introduction to the Special Issue. *Computational Linguistics* 39(1). https://aclanthology.org/J13-1003/ — local: `$RESEARCH_RAW/cl-2013-tsarfaty-parsing-morphologically-rich-languages-introduction/`.
- UD Hebrew documentation: language page and the pages for `compound:smixut`, `case:acc`, `case:gen`, `nsubj:cop`, `mark:q`, `compound:affix`, `HebBinyan`, `HebExistential`, `Prefix`. https://universaldependencies.org/he/ — local snapshot (commit 3ae7161, 2026-10-17): `$RESEARCH_RAW/hebgram/sa/ud-docs-he-hbo/_he/`.
- UD 2.18 treebanks UD_Hebrew-HTB, -IAHLTwiki, -IAHLTknesset, -PostRab: https://github.com/UniversalDependencies/ — local: `$RESEARCH_RAW/lexres/treebanks/ud-2.18/` (HTB under `nc/`); Mova cards: `treebanks/cards.tsv`.
- Zeldes, Amir. 2018. A Characterwise Windowed Approach to Hebrew Morphological Segmentation. SIGMORPHON 2018. https://aclanthology.org/W18-5811/ — local: `$RESEARCH_RAW/aclbulk-2018-zeldes-characterwise-windowed-approach-hebrew-morphological/`.
- Zeldes, Amir, Nick Howell, Noam Ordan and Yifat Ben Moshe. 2022. A Second Wave of UD Hebrew Treebanking and Cross-Domain Parsing. EMNLP 2022, 4331–4344. https://aclanthology.org/2022.emnlp-main.292/
