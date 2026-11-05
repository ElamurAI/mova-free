# The dialects and Homer

Ancient Greek was never one language with one norm. Every region had its dialect, and every literary genre took its own: epic Homer's mixed Ionic, history and medicine Ionic (Herodotus, Hippocrates), choral lyric a literary Doric, love lyric Aeolic (Sappho, Alcaeus), drama and prose Attic. Attic became the base of the Koine and so of every later Greek ([chapter 17](17-koine.md)). This manual describes Attic; this chapter gives the main dialects and how they differ from it, and the language of Homer, which is the largest part of the classical treebank.

## Contents

1. The dialect groups
2. The marks that tell them apart
3. Ionic
4. Doric
5. Aeolic
6. The language of Homer
7. How Mova handles it
8. Sources

---

## 1. The dialect groups

(smyth-1920, Introduction: the Greek language and its dialects)

| group | where | literature |
|---|---|---|
| **Attic** | Athens | drama, Thucydides, Plato, the orators, Xenophon |
| **Ionic** | the Ionian coast of Asia Minor, the Cyclades, Euboea | Herodotus, Hippocrates; elegy and iambus (Archilochus, Anacreon); the base of Homer |
| **Doric** (West Greek) | the Peloponnese (Sparta, Corinth, Argos), Crete, Rhodes, Sicily, southern Italy | choral lyric (Pindar, Alcman), Theocritus, Archimedes; the choruses of tragedy |
| **Aeolic** | Lesbos, Thessaly, Boeotia | Sappho, Alcaeus; Homer's Aeolic layer |
| **Arcado-Cypriot** | Arcadia, Cyprus | inscriptions; the nearest to the Mycenaean Greek of the Linear B tablets |

## 2. The marks that tell them apart

| feature | Attic | Ionic | Doric | Aeolic |
|---|---|---|---|---|
| old long ᾱ | η, but ᾱ after ε, ι, ρ (χώρᾱ) | η everywhere (χώρη) | ᾱ (μᾱ́τηρ, ἁμέρα) | ᾱ |
| -ττ- / -σσ- | θάλαττα | θάλασσα | θάλασσα | θάλασσα |
| -ρρ- / -ρσ- | θάρρος | θάρσος | θάρσος | θέρσος |
| contraction | full | often open (ποιέει) | ε + ε → η (Severer Doric) | open |
| the digamma ϝ (w) | lost | lost | kept in some (ϝοῖκος) | kept in some |
| 3rd pl. -ουσι | λέγουσι | λέγουσι | λέγοντι | λέγοισι |
| infinitive | -ειν, -ναι | -ειν, -ναι | -εν, -μεν | -μεναι, -ην |
| "if" | εἰ, ἐάν | εἰ, ἤν | αἰ | αἰ |
| article plural | οἱ, αἱ | οἱ, αἱ | τοί, ταί | οἱ, αἱ |
| psilosis (no rough breathing) | no | yes (East Ionic) | no | yes |

(smyth-1920 §30 for Attic η and ᾱ, and the dialect notes «D» throughout the morphology)

## 3. Ionic

Herodotus' Ionic:
- has η for every Attic ᾱ: ἱστορίη, χώρη;
- leaves many vowels uncontracted: ποιέειν, νόος;
- has κ for π in the pronouns: κῶς, κοῦ, κότε for πῶς, ποῦ, πότε;
- has no rough breathing (psilosis): ἀπίκετο for ἀφίκετο;
- uses the τ- forms of the article as relatives: τόν "whom";
- has the dative plural -οισι, -ῃσι.

PROIEL's Greek includes the *Histories* of Herodotus (NC, counted only). Its word order and moods are counted in [chapter 13](13-simple-clause.md), §5 and [chapter 14](14-subordination.md), §9.

## 4. Doric

Doric keeps the old ᾱ (Δᾶμος for δῆμος, ἁ for ἡ), has τοί, ταί in the article, -μες for -μεν in the 1st plural (λέγομες), the future in -σέω (contract) and the infinitive in -εν. Its literary form colours the choral odes of Attic tragedy: a tragic chorus says ἁ μάτηρ where the actors say ἡ μήτηρ.

## 5. Aeolic

Lesbian Aeolic, the language of Sappho and Alcaeus:
- has no rough breathing;
- throws the accent back as far as it can (recessive accent everywhere: βάσιλευς);
- has -οισα for -ουσα in participles, ζ for δ in some words;
- conjugates contract verbs as -μι verbs (φίλημι for φιλέω).

## 6. The language of Homer

Homer's language was never spoken anywhere. It is an artificial epic language grown over centuries of oral poetry: an Ionic base, with an older Aeolic layer and a few Arcado-Cypriot words. It keeps alternative forms side by side because the metre needs one shape here and another there (smyth-1920, the «D» notes; Monro 1891):
- **genitive singular** -οιο beside -ου, -ᾱο and -εω for the first declension: θεοῖο, Ἀτρείδᾱο;
- **dative plural** -οισι, -ῃσι, -εσσι (πόδεσσι);
- **infinitives** -μεν, -μεναι beside -ειν: ἔμμεναι, ἔμεν for εἶναι;
- **the augment** often absent: βῆ for ἔβη;
- **the article** still a demonstrative or relative ([chapter 5](05-article.md), §2);
- **the subjunctive** with a short vowel: ἴομεν "let us go";
- **κε(ν)** beside ἄν;
- **the suffix -φι(ν)** for an instrumental or locative: βίηφι "by force";
- **tmesis** ([chapter 15](15-verse.md), §6);
- **uncontracted and "distracted" forms**: ὁρόωντες for ὁρῶντες;
- **the digamma**, lost in the script but felt in the metre: the hiatus in τὸν δ᾽ ἠμείβετ᾽ ἔπειτα ἄναξ (ϝάναξ).

**The Homeric treebank.** Homer is the largest part of the Perseus training set: 6,003 sentences from the *Iliad* and the *Odyssey* (tlg0012), against 2,360 of Sophocles and 1,092 of Herodotus (UD 2.18, counted 2026-10-22). Perseus is NC, so Mova counts on it and never quotes it.

## 7. How Mova handles it

**In the UD trees.**
- A dialect form has its Attic lemma: θεοῖο → θεός, ἔμμεναι → εἰμί, Ἀτρείδᾱο → Ἀτρείδης.
- Homeric forms are tagged with their ordinary features.
- PTNK (the Septuagint) has no dialect forms beyond the Koine's own.

**In Mova.** A Greek morphology for Mova should be built on Attic, with dialect rules as **alternatives**, never as a second lexicon:
- the ᾱ/η exchange;
- -σσ- for -ττ-;
- the open vowels for the contracted;
- the Homeric endings of §6;
- the psilosis of Ionic and Aeolic.

Each rule maps a dialect form to its Attic counterpart, which the lexicon looks up. The rules are short, deterministic and testable form by form against the lemmas of the treebanks: a rule that maps θεοῖο to θεός can be checked on every Homeric genitive in Perseus. The same method serves the Latin spellings of the charters ([la chapter 14](../la/14-medieval-latin.md), §9).

## 8. Sources

- Smyth, H. W. 1920 (smyth-1920): the Introduction (the dialects), §30, and the dialect notes «D» throughout, paraphrased from the Perseus text (CC BY-SA).
- Monro, D. B. 1891, *A Grammar of the Homeric Dialect* (monro-1891-grammar-homeric-dialect), public domain.
- Buck, C. D. 1910, *Introduction to the Study of the Greek Dialects*, public domain; not yet in our library (wanted).
- UD Ancient Greek Perseus and PROIEL (CC BY-NC-SA): counts only.
