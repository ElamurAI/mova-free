# The simple clause: agreement and word order

A Greek clause, like a Latin one, is held together by agreement and the cases. The order of its words is free in the sense that it does not decide who did what to whom. It is not random: it shows topic and focus, and it differs from author to author and from period to period. This chapter gives:
- agreement, with the Greek peculiarity that a neuter plural subject takes a singular verb;
- the omitted subject and copula;
- impersonal verbs;
- the order of words, measured in Herodotus, in the classical texts of the Perseus treebank, in the New Testament and in the Septuagint;
- the postpositive words, which fix the second place of a clause.

## Contents

1. Agreement
2. The neuter plural with a singular verb
3. The omitted subject and the omitted copula
4. Impersonal verbs
5. Word order in the clause
6. The second place: postpositives and enclitics
7. Word order in the noun phrase
8. How Mova handles it
9. Sources

---

## 1. Agreement

- **Verb and subject** agree in person and number: ὁ παῖς γράφει, οἱ παῖδες γράφουσι (smyth-1920 §§949–972).
- With two or more subjects the verb is plural, or agrees with the nearest: ἐγὼ καὶ σὺ ταῦτα ἐποιήσαμεν "you and I did this".
- A collective noun may take a plural verb by sense: τὸ πλῆθος ἐψηφίσαντο "the people voted".
- **The dual** subject takes a dual or a plural verb.
- **Adjectives and participles** agree with their noun in gender, number and case, wherever they stand ([chapter 4](04-adjectives.md)).
- **The relative** agrees with its antecedent in gender and number and takes its case from its own clause, unless attraction draws it into the antecedent's ([chapter 6](06-pronouns.md), §7).

## 2. The neuter plural with a singular verb

A neuter plural subject usually takes a **singular** verb in Attic: τὰ ζῷα τρέχει "the animals run", ταῦτα ἐγένετο "these things happened" (smyth-1920 §958). The plural verb is used when the neuter plural stands for persons or for many separate things. Homer and the Koine use either. For a parser this means that a singular verb's subject may be a plural noun: agreement in number cannot be checked between them as in Latin or English.

## 3. The omitted subject and the omitted copula

- **The subject** pronoun is left out unless stressed ([chapter 6](06-pronouns.md), §1). A third person subject is understood from the context; an indefinite "they, people" from the plural (λέγουσι "they say"). Verbs of weather have none: ὕει "it rains", νείφει "it snows".
- **The copula** ἐστί, εἰσί is often left out, especially in proverbs, in questions and after adjectives of necessity: ἀνάγκη ταῦτα ποιεῖν "(it is) necessary to do this"; ἕτοιμος "(I am) ready"; σοφὸς ὁ ἀνήρ "the man (is) wise" (smyth-1920 §944). The position of the article tells the predicate ([chapter 5](05-article.md), §6).

## 4. Impersonal verbs

δεῖ "it is necessary", χρή "one must", ἔξεστι "it is allowed", δοκεῖ "it seems good, it is decided", μέλει "it is a concern", πρέπει "it is fitting", συμβαίνει "it happens". Their subject is an infinitive or a clause, and the person concerned is in the dative (ἔξεστί μοι), the accusative (δεῖ με) or the genitive of the thing (μέλει μοι τούτου "I care about this") (smyth-1920 §§932–935).

The decisions of an assembly are ἔδοξε τῇ βουλῇ καὶ τῷ δήμῳ "it seemed good to the council and the people" = "the council and the people resolved".

## 5. Word order in the clause

There is no fixed place for the verb in classical Greek. The topic tends to come first, the focus just before the verb or at the head of the clause, and the verb follows or ends the clause according to the author and the genre. We counted the order of subject, verb and object in main clauses whose verb has a noun subject and a direct object (UD 2.18 training sets, counted 2026-10-22):

| text | treebank | SOV | SVO | OSV | OVS | VSO | VOS | clauses |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| classical texts, mostly verse (Homer, Hesiod, Sophocles, Aeschylus; Herodotus and Diodorus in prose) | Perseus (NC, counted only) | 21% | 22% | 17% | **30%** | 4% | 6% | 2 110 |
| Herodotus, *Histories* | PROIEL (NC, counted only) | 25% | **37%** | 13% | 12% | 8% | 5% | 581 |
| the New Testament | PROIEL (NC, counted only) | 15% | **47%** | 4% | 7% | 14% | 14% | 517 |
| the Septuagint, Genesis and Ruth | PTNK | 1% | 20% | 0% | 1% | **69%** | 9% | 86 |

- **The classical texts**, Homer above all, use every order and put the object first in nearly half the clauses (OVS and OSV: 47%): the object, often the topic, comes first.
- **Herodotus** in prose leans to SVO and SOV.
- **The New Testament** is SVO in nearly half its clauses, like the Latin Vulgate (44%: [la chapter 9](../la/09-simple-clause.md), §6) and the later Greek that leads to the modern language.
- **The Septuagint** puts the verb first in nearly four clauses in five (VSO and VOS: 78%), the order of Biblical Hebrew narrative (*wayyiqtol* + subject + object), carried over word for word: καὶ ἐποίησεν ὁ θεὸς τὸ στερέωμα "and God made the firmament" (Gen. 1:7).

The same Hebrew verse, read in its Greek, Latin and English translations, thus gives three word orders: VSO in the Septuagint, and SVO more and more in the later Latin and the English.

## 6. The second place: postpositives and enclitics

Greek fixes one thing in the order of the clause: the **second place**. The postpositive particles (δέ, γάρ, οὖν, μέν, ἄρα, δή, γε, τοι) and the enclitic pronouns (μου, μοι, με, σου, σοι, σε, τις, του) stand after the first word or phrase of their clause (Wackernagel's law, smyth-1920 §§2769, 181–187):

> Κῦρος **δὲ** μεταπέμπεται … "and Cyrus summons …"
> ἐγὼ **μέν σοι** λέγω "I tell you"

In the Septuagint, δέ still takes second place, but καί at the head of the clause is far commoner ([chapter 12](12-prepositions-particles.md), §9).

## 7. Word order in the noun phrase

- **The article group** holds its attributes between the article and the noun or after a repeated article: ὁ σοφὸς ἀνήρ, ὁ ἀνὴρ ὁ σοφός ([chapter 5](05-article.md), §6). An adjective outside the group is a predicate. This is the one place where order changes the meaning in a way the cases do not.
- **The genitive** stands inside the article group (ἡ τοῦ πατρὸς οἰκία) or after the noun with a repeated article (ἡ οἰκία ἡ τοῦ πατρός), or after the noun without one (ἡ οἰκία τοῦ πατρός), as in the Septuagint and the New Testament.
- **Hyperbaton**: a word separated from its group for emphasis, frequent in poetry and in Plato and Demosthenes: τὴν ἀρχὴν ἔχειν τῆς Ἑλλάδος "to hold the rule of Greece".

## 8. How Mova handles it

**In the UD trees.**
- Agreement is in the `FEATS`.
- The relations do not depend on the order.
- An omitted subject has no node.
- A neuter plural subject with a singular verb is an ordinary `nsubj`.

**In Mova.** The parser Mova uses for Greek is its English one after `en xpos-fill`, and it leans on order. The counts of §5 explain why its Greek score depends on the text:
- PTNK, the Septuagint, has a fixed Hebrew-like order and formulaic narrative: LAS 74.41 with Mova's own tags, 78.29 with gold tags (dev, 2026-10-22);
- the classical texts, with their object-first orders and hyperbaton, would fall further; they are not measured, since the Perseus treebank is NC and its open alternative is still to be found.

A Greek frames reader needs:
1. **the case** to find subject and object in a free order, the need of Latin too ([chapter 11](11-cases.md), §8);
2. **agreement in number with a neuter plural exception** (§2);
3. **the omitted subject and copula** supplied from the verb's person and the article's position;
4. **the impersonal verbs** read as events with no agent and the dative as the experiencer.

The order counts of §5 are a stylometric signal that Mova can compute for any parsed Greek text. It tells translation Greek (verb first) from classical prose and from the Koine.

## 9. Sources

- Smyth, H. W. 1920 (smyth-1920): §§932–972 (subject, predicate, agreement), §958 (the neuter plural), §§181–187 (enclitics), §2769 (postpositives), paraphrased from the Perseus text (CC BY-SA).
- UD Ancient Greek PTNK (CC BY-SA 4.0): counts and the Genesis example; UD Ancient Greek Perseus and PROIEL (CC BY-NC-SA): counts only, never quoted.
