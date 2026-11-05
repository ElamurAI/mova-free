# Pronouns

Greek pronouns fall into the same classes as the Latin ones: personal, reflexive, possessive, demonstrative, interrogative and indefinite, relative, and the correlative series. Two words do much of the work. **αὐτός** is "self" in one position, "the same" in another, and in the oblique cases the ordinary third-person pronoun "him, her, it". The **demonstratives** οὗτος, ὅδε, ἐκεῖνος distinguish "this (just said)", "this (here, coming)" and "that". This chapter gives the forms in brief and the uses, the ways the three UD treebanks tag them, and what the Septuagint does differently.

## Contents

1. The personal pronouns
2. αὐτός: self, same, him
3. The reflexive and reciprocal pronouns
4. The possessives
5. The demonstratives
6. Interrogative and indefinite: τίς and τις
7. The relative pronouns: ὅς and ὅστις
8. The correlative series
9. ἄλλος, ἕτερος and the other pronominal adjectives
10. The pronouns of the Septuagint
11. How Mova handles it
12. Sources

---

## 1. The personal pronouns

| | 1st sg. | 2nd sg. | 1st pl. | 2nd pl. |
|---|---|---|---|---|
| N | ἐγώ | σύ | ἡμεῖς | ὑμεῖς |
| G | ἐμοῦ, μου | σοῦ, σου | ἡμῶν | ὑμῶν |
| D | ἐμοί, μοι | σοί, σοι | ἡμῖν | ὑμῖν |
| A | ἐμέ, με | σέ, σε | ἡμᾶς | ὑμᾶς |

The dual (νώ "we two", σφώ "you two") is rare in prose (smyth-1920 §325).

- **Enclitic and stressed forms.** The short forms μου, μοι, με, σου, σοι, σε are enclitics ([chapter 2](02-accent.md), §6). The long forms are used for emphasis and after prepositions: πρὸς ἐμέ, but δός μοι "give me".
- **The nominative is left out** unless it is stressed or contrasted, as in Latin: ἐγὼ μὲν … σὺ δέ "I … but you" (smyth-1920 §1190). The verb's ending carries the person ([chapter 7](07-verb.md)).
- **The third person.** The old third-person pronoun (οὗ, οἷ, ἕ; σφεῖς) survives in Attic only as an indirect reflexive, "him" referring to the subject of the main verb in a reported clause, and mostly in Plato and Xenophon. Its place is taken by the oblique cases of αὐτός (§2) and by the demonstratives (§5).

## 2. αὐτός: self, same, him

αὐτός, αὐτή, αὐτό declines like ἀγαθός, with neuter αὐτό (smyth-1920 §§327–328). It has three uses, told apart by its position and its case (smyth-1920 §§1204–1217):

| use | where | example |
|---|---|---|
| **intensive** "self" | in the predicate position, or alone in the nominative | αὐτὸς ὁ ἀνήρ, ὁ ἀνὴρ αὐτός "the man himself"; αὐτὸς ἔφη "he himself said" (Pythagoras' αὐτὸς ἔφα "the master said it") |
| **identity** "the same" | in the attributive position, after the article | ὁ αὐτὸς ἀνήρ "the same man"; ταὐτά (= τὰ αὐτά) "the same things" |
| **third-person pronoun** "him, her, it, them" | the oblique cases, unstressed, not at the head of the clause | εἶδον αὐτόν "I saw him"; ὁ πατὴρ αὐτοῦ "his father" |

The nominative αὐτός never means simply "he". With the article the identity sense is often written by crasis: αὑτός "the same (man)", ταὐτό "the same (thing)". Do not confuse it with the reflexive αὑτοῦ (§3).

## 3. The reflexive and reciprocal pronouns

**Reflexives** refer to the subject of their clause and have no nominative (smyth-1920 §§329, 1218–1237):

| | 1st | 2nd | 3rd |
|---|---|---|---|
| G sg. | ἐμαυτοῦ, -ῆς | σεαυτοῦ (σαυτοῦ) | ἑαυτοῦ (αὑτοῦ), -ῆς, -οῦ |
| pl. | ἡμῶν αὐτῶν | ὑμῶν αὐτῶν | ἑαυτῶν (αὑτῶν), σφῶν αὐτῶν |

The reflexive is **direct** when it points to the subject of its own clause: γνῶθι σεαυτόν "know thyself". It is **indirect** when, in a dependent clause, it points to the subject of the main clause: ἐκέλευσε τοὺς στρατιώτας ἐπὶ ἑαυτὸν ἐλθεῖν "he ordered the soldiers to come to him (the speaker)". This is the Latin *sē* of the accusative and infinitive ([la chapter 10](../la/10-subordination.md), §3).

**The reciprocal** ἀλλήλων "one another" has only the oblique cases of the plural and dual: φιλοῦσιν ἀλλήλους "they love each other" (smyth-1920 §§331, 1277). The middle voice often does the same work: διαλέγονται "they converse".

## 4. The possessives

The possessive adjectives ἐμός, σός, ἡμέτερος, ὑμέτερος decline like adjectives of the first and second declension (smyth-1920 §§330, 1196–1203). They stand in the attributive position with the article ([chapter 5](05-article.md), §6): ὁ ἐμὸς πατήρ "my father". The third person has no common possessive adjective in Attic prose (the poetic ὅς and σφέτερος aside). Its owner is expressed:
- by the genitive of αὐτός when it is not the subject: ὁ πατὴρ αὐτοῦ "his (someone else's) father";
- by the reflexive when it is the subject: ὁ ἑαυτοῦ πατήρ "his own father";
- by the article alone when the owner is plain: ὁ πατήρ "his father".

The unstressed genitive of the personal pronoun (ὁ πατήρ μου) spreads in later Greek and is the norm in the Koine (§10).

## 5. The demonstratives

| | near, just mentioned | near, to come; here | far |
|---|---|---|---|
| pronoun | οὗτος, αὕτη, τοῦτο | ὅδε, ἥδε, τόδε | ἐκεῖνος, ἐκείνη, ἐκεῖνο |
| meaning | "this, the aforesaid" | "this here, the following" | "that, the well-known" |

οὗτος has two stems: the τ- forms (τοῦτον, ταῦτα) and the forms without τ in the nominative masculine and feminine (οὗτος, αὕτη, οὗτοι, αὗται), like the article. Its vowel follows the ending: ου before an *o*-sound, αυ before an *a*-sound (τούτου but ταύτης) (smyth-1920 §333).

**Use** (smyth-1920 §§1238–1261):
- With a noun, a demonstrative stands in the predicate position, and the noun has the article: οὗτος ὁ ἀνήρ, ὁ ἀνὴρ οὗτος "this man" ([chapter 5](05-article.md), §6).
- οὗτος usually points back and ὅδε forward: ταῦτα ἔλεξεν "he said this (as given above)"; ἔλεξε τάδε "he spoke as follows". In Attic prose the speech after τάδε follows directly.
- ἐκεῖνος marks the remote, the absent, the famous: ἐκεῖνος ὁ Σωκράτης "the great Socrates".
- The deictic **-ί** strengthens a demonstrative in comedy and in oratory: οὑτοσί "this man here", ὁδί, τουτί.

## 6. Interrogative and indefinite: τίς and τις

The interrogative **τίς, τί** "who? what?" and the indefinite **τις, τι** "someone, something, a certain" have the same forms and differ in accent (smyth-1920 §§334, 1262–1270):
- τίς always has the acute on its first syllable, and never changes it to the grave: τίς; τίνος; τί;
- τις is an enclitic: ἀνήρ τις "a certain man", λέγει τι "he says something".

τί also means "why?". The indefinite τις serves as the indefinite article where one is needed: ἀνήρ τις "a (certain) man". The short forms του, τῳ (for τινός, τινί) and τοῦ, τῷ (for τίνος, τίνι) are common in Attic.

## 7. The relative pronouns: ὅς and ὅστις

**ὅς, ἥ, ὅ** "who, which" declines like the article without the τ-: ὅς, οὗ, ᾧ, ὅν; ἥ, ἧς; ὅ (smyth-1920 §338). It always has an accent, which tells it from the proclitic article (ὁ, ἡ, οἱ, αἱ). It agrees with its antecedent in gender and number and takes its case from its own clause (smyth-1920 §2501).

**Attraction.** A relative that should be accusative is drawn into the genitive or dative of its antecedent: ἐκ τῶν πόλεων ὧν ἔχει "from the cities which he holds" (for ἅς ἔχει) (smyth-1920 §§2522–2528). The antecedent is often left out: ὧν ἔχει "(of those) which he holds". Attraction is common in Attic and the Koine. It means that the case of a relative pronoun does not show its role in its own clause.

**ὅστις, ἥτις, ὅ τι** "whoever, anyone who" is the general or indefinite relative (smyth-1920 §339). Both parts decline (οὗτινος, ᾧτινι). It also introduces indirect questions: ἐρωτᾷ ὅστις ἐστίν "he asks who he is". ὅ τι is written apart to tell it from the conjunction ὅτι "that, because".

## 8. The correlative series

Pronouns and adverbs come in parallel series, interrogative, indefinite, demonstrative and relative (smyth-1920 §340):

| | interrogative | indefinite (enclitic) | demonstrative | relative | indefinite relative |
|---|---|---|---|---|---|
| who | τίς | τις | ὅδε, οὗτος | ὅς | ὅστις |
| how much, how many | πόσος | ποσός | τοσόσδε, τοσοῦτος | ὅσος | ὁπόσος |
| of what kind | ποῖος | ποιός | τοιόσδε, τοιοῦτος | οἷος | ὁποῖος |
| where | ποῦ | που | ἐνθάδε, ἐκεῖ | οὗ | ὅπου |
| when | πότε | ποτέ | τότε | ὅτε | ὁπότε |
| how | πῶς | πως | ὧδε, οὕτως | ὡς | ὅπως |

The ὁ- forms (ὁπόσος, ὅπου, ὅπως) serve as indirect interrogatives: οὐκ οἶδα ὅπου ἐστίν "I don't know where he is".

## 9. ἄλλος, ἕτερος and the other pronominal adjectives

- ἄλλος "other" has the neuter ἄλλο, like αὐτός (smyth-1920 §335).
- ἕτερος "the other of two, another, different" (smyth-1920 §§337, 1271–1276).
- The article changes the sense ([chapter 5](05-article.md), §7): οἱ ἄλλοι "the rest", ὁ ἕτερος "the other one".
- Repeated ἄλλος makes a compressed statement: ἄλλος ἄλλα λέγει "one says one thing, another another".
- **δεῖνα** "so-and-so" always takes the article: ὁ δεῖνα (smyth-1920 §336).

## 10. The pronouns of the Septuagint

The Greek of the Septuagint renders the Hebrew pronoun suffixes word for word, and so uses the unstressed personal pronoun far more than Attic does:
- **possession by the genitive of αὐτός and of the personal pronoun**, after the noun: ὁ θεὸς αὐτοῦ "his God", ἡ γυνή σου "your wife". In the PTNK training set (Genesis and Ruth, UD 2.18, counted 2026-10-22), αὐτός is the most frequent pronoun, 915 tokens. In 414 of them it is a possessor (`nmod:poss`), against 153 objects and 50 subjects;
- **the resumptive pronoun in relative clauses**, from the Hebrew *ʾăšer … lô*: ἡ γῆ ἐφ᾽ ἧς σὺ καθεύδεις ἐπ᾽ αὐτῆς "the land on which you lie, on it" (Gen. 28:13);
- **ὅδε dies out:** 6 tokens against 166 of οὗτος in the same set.

These are traits of translation Greek, treated in the Koine manual (`grc-x-koine`).

## 11. How Mova handles it

**In the UD trees.**
- The personal, reflexive, interrogative, indefinite and relative pronouns are `PRON`.
- αὐτός is `PRON` or `ADJ` according to the treebank and the use, and its intensive and identity uses are not told apart by the tag (PTNK has 915 tokens of αὐτός as `PRON` and 1 as `ADJ`).
- The demonstratives are `DET`, `PRON` or `ADJ` by treebank ([chapter 5](05-article.md), §10).
- In PTNK the enclitic and stressed forms are separate lemmas for σύ (σύ and σέ: 216 and 53 tokens).

**In Mova.** Greek goes through Mova's English parser after `en xpos-fill` (PTNK dev LAS 74.41 with its own tags, 78.29 with gold tags; 2026-10-22). What a Greek frames reader needs from this chapter:
1. **αὐτός by position**: "self" in the predicate position or in the nominative, "same" after the article, "him" elsewhere. A rule on the tree (is there an article before it in its noun phrase? is it nominative?), deterministic and checkable on the treebanks.
2. **The reflexive's antecedent**: the subject of its clause, or, for an indirect reflexive, of the main clause. This is the same cross-clause link as the Latin *sē* and the English *he said that he*.
3. **The relative's role in its own clause**, which attraction (§7) hides: read from the tree (the relative's relation), not from its case.
4. **τις and τίς** told apart by the accent: one is an indefinite determiner, the other a question.
5. **The Septuagint's possessive αὐτοῦ** read as possession, like the English *his*.

## 12. Sources

- Smyth, H. W. 1920 (smyth-1920): §§325–340 (the forms), §§1190–1278 (the uses), §§2488–2573 (relative clauses), paraphrased from the Perseus text (CC BY-SA).
- Goodwin, W. W. 1892 (goodwin-1892): the pronouns, public domain.
- UD Ancient Greek PTNK (CC BY-SA 4.0): counts and the Septuagint example.
