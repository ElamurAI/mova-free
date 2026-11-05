# Contract verbs

Verbs whose stem ends in α, ε or ο contract that vowel with the thematic vowel of the ending in the present and imperfect: τιμά-ω → τιμῶ "honour", ποιέ-ω → ποιῶ "make, do", δηλό-ω → δηλῶ "show". They are the largest class of Greek verbs, because most verbs made from nouns and adjectives belong to it (τιμή → τιμάω, φίλος → φιλέω, δοῦλος → δουλόω). Outside the present system they lengthen the stem vowel and behave like λύω: τιμήσω, ἐτίμησα, τετίμηκα. This chapter gives the rules of contraction, the three paradigms, the exceptions, and the forms that do not contract in other dialects and in the Koine.

## Contents

1. The rules of contraction
2. The present and imperfect of the three types
3. Outside the present system: the long vowel
4. Exceptions and short-vowel verbs
5. Uncontracted forms in Ionic and Homer; the Koine
6. How Mova handles it
7. Sources

---

## 1. The rules of contraction

Two vowels meeting in a word combine into a long vowel or a diphthong ([chapter 1](01-script-sounds.md)). For the contract verbs the outcomes are (smyth-1920 §§48–59, 385):

| stem vowel + ending vowel | α | ε | ο |
|---|---|---|---|
| + ω, ο, ου | ω (τιμῶμεν < τιμά-ομεν) | ου / ω (ποιοῦμεν < ποιέ-ομεν; ποιῶ < ποιέ-ω) | ου / ω (δηλοῦμεν < δηλό-ομεν; δηλῶ < δηλό-ω) |
| + ε, η | ᾱ (τιμᾶτε < τιμά-ετε) | ει / η (ποιεῖτε < ποιέ-ετε) | ου / ω (δηλοῦτε < δηλό-ετε) |
| + ει, ῃ | ᾳ (τιμᾷ < τιμά-ει) | ει / ῃ (ποιεῖ < ποιέ-ει) | οι (δηλοῖ < δηλό-ει) |
| + οι | ῳ (τιμῷμεν, optative) | οι (ποιοῖμεν) | οι (δηλοῖμεν) |

**The accent.** If either of the two vowels was accented, the contracted vowel is accented with a circumflex if the accent stood on the first, an acute if on the second ([chapter 2](02-accent.md)): τιμά-ω → τιμῶ, τιμα-όμεθα → τιμώμεθα. The circumflex of a contract form is the commonest sign that a verb is contracted.

## 2. The present and imperfect of the three types

**Present indicative active:**

| | τιμάω "honour" | ποιέω "make" | δηλόω "show" |
|---|---|---|---|
| 1 sg. | τιμῶ | ποιῶ | δηλῶ |
| 2 sg. | τιμᾷς | ποιεῖς | δηλοῖς |
| 3 sg. | τιμᾷ | ποιεῖ | δηλοῖ |
| 1 pl. | τιμῶμεν | ποιοῦμεν | δηλοῦμεν |
| 2 pl. | τιμᾶτε | ποιεῖτε | δηλοῦτε |
| 3 pl. | τιμῶσι(ν) | ποιοῦσι(ν) | δηλοῦσι(ν) |
| infinitive | τιμᾶν | ποιεῖν | δηλοῦν |
| participle | τιμῶν, -ῶσα, -ῶν | ποιῶν, -οῦσα, -οῦν | δηλῶν, -οῦσα, -οῦν |

**Imperfect:** ἐτίμων, ἐτίμας, ἐτίμα; ἐποίουν, ἐποίεις, ἐποίει; ἐδήλουν, ἐδήλους, ἐδήλου.

**Middle:** τιμῶμαι, τιμᾷ, τιμᾶται; ποιοῦμαι, ποιῇ (ποιεῖ), ποιεῖται; δηλοῦμαι, δηλοῖ, δηλοῦται.

Note the **ambiguities** the contraction makes:
- τιμᾷ is present indicative active 3 sg., subjunctive active 3 sg., or middle 2 sg.;
- τιμῶ is indicative or subjunctive;
- the -α- verbs have the same forms in the indicative and the subjunctive through much of the paradigm.

A treebank records these as feature sets (`Mood=Ind,Sub`: [chapter 7](07-verb.md), §10).

**The infinitives** τιμᾶν and δηλοῦν have no iota subscript (from -άεεν, -όεεν), against the subscript of the 3rd singular τιμᾷ, δηλοῖ.

## 3. Outside the present system: the long vowel

In the other tenses the stem vowel is lengthened before the tense suffix, α (after ε, ι, ρ: ᾱ; otherwise η), ε → η, ο → ω (smyth-1920 §487):

| present | future | aorist | perfect | aorist passive |
|---|---|---|---|---|
| τιμάω | τιμήσω | ἐτίμησα | τετίμηκα | ἐτιμήθην |
| ποιέω | ποιήσω | ἐποίησα | πεποίηκα | ἐποιήθην |
| δηλόω | δηλώσω | ἐδήλωσα | δεδήλωκα | ἐδηλώθην |
| ἐάω "allow" | ἐάσω | εἴασα | εἴακα | εἰάθην |
| θηράω "hunt" | θηράσω | ἐθήρασα | τεθήρακα | ἐθηράθην |

## 4. Exceptions and short-vowel verbs

- **The disyllabic -έω verbs** (πλέω "sail", πνέω "blow", θέω "run", δέω "need", ῥέω "flow") contract only ε + ε and ε + ει: πλεῖ, πλεῖτε, but πλέω, πλέομεν, πλέουσι (smyth-1920 §397).
- **ζάω "live"**, πεινάω "be hungry", διψάω "be thirsty", χράομαι "use" contract to η where the other -άω verbs have ᾱ: ζῇς, ζῇ, ζῆν; χρῇ, χρῆται, χρῆσθαι (smyth-1920 §394).
- **Verbs that keep the short vowel** in the other tenses: γελάω, γελάσομαι, ἐγέλασα; τελέω, τελῶ (Attic future), ἐτέλεσα; καλέω, καλῶ, ἐκάλεσα (smyth-1920 §488).
- **The Attic future** of verbs in -ίζω and of some others is itself a contract future: νομίζω, νομιῶ, νομιεῖς; καλέω, καλῶ (the same form as the present).

## 5. Uncontracted forms in Ionic and Homer; the Koine

**Ionic** (Herodotus) and **Homer** leave many of these vowels open: ποιέει, ποιέουσι, φιλέω. Homer also has the "distracted" forms that restore a long vowel for the metre: ὁρόω, ὁράᾳ (for ὁρῶ, ὁρᾷ) ([chapter 16](16-dialects.md)).

**In the Koine** the -άω and -έω verbs begin to mix: ἐρωτάω and ἐρωτέω, ἠρώτουν beside ἠρώτων. The -όω verbs keep their type. In the Septuagint the 3rd plural of the imperfect and the second aorist takes the ending -οσαν (εἴποσαν, ἤλθοσαν, ἐλάβοσαν in PTNK Genesis and Ruth; UD 2.18, 2026-10-22), and contract verbs follow it in the imperfect (ἐποιοῦσαν) (`grc-x-koine`).

## 6. How Mova handles it

**In the UD trees.** A contract form has the uncontracted lemma: ποιεῖ, ποιοῦσι, ἐποίει all have the lemma ποιέω, τιμᾷ the lemma τιμάω. Where the form is ambiguous, PTNK keeps the set of moods (`Mood=Ind,Sub`) and PROIEL and Perseus choose one.

**In Mova.** A Greek lexicon in Mova would need a **contraction rule** to find ποιέω from ποιεῖ, ποιοῦμεν, ἐποίουν. The table of §1 is small and exact, and can be run backwards to give the candidates of any contracted ending, then checked against the lemma list. It is deterministic, and its errors can be shown one by one against the treebanks' lemmas. None of this exists yet. Today Greek lemmas come only from the treebank, and a word not in it has none.

## 7. Sources

- Smyth, H. W. 1920 (smyth-1920): §§48–59 (contraction), §§385, 394–397 (the contract verbs), §§487–488 (the vowel outside the present), paraphrased from the Perseus text (CC BY-SA).
- Goodwin, W. W. 1892 (goodwin-1892): the contract verbs, public domain.
