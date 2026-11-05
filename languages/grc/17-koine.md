# From Attic to the Koine

After Alexander (died 323 BC), Greek became the common language, ἡ κοινὴ διάλεκτος, of the eastern Mediterranean and the Near East. Its base was Attic, softened by Ionic and spoken by millions for whom it was a second language. It is the Greek of:
- the Septuagint (3rd–1st century BC);
- the New Testament (1st century AD);
- Polybius, the papyri and the inscriptions;
- the Church Fathers.

Its spoken form leads, through Byzantine Greek, to Modern Greek. The Koine has its own manual, `grc-x-koine`, built from the Septuagint and the New Testament as the Ancient Hebrew one is built from its corpus. This chapter is the bridge: what changed from the Attic of this manual, with the measurements made in the chapters before.

## Contents

1. Sounds and spelling
2. Nouns and pronouns
3. The verb
4. Syntax
5. Translation Greek: the Septuagint
6. What was measured
7. How Mova handles it
8. Sources

---

## 1. Sounds and spelling

- **Itacism:** η, ι, υ, ει, οι, υι merge towards /i/, so that the papyri confuse them.
- **αι** becomes /e/.
- **The rough breathing** is lost in speech.
- **Vowel length** is lost, and the pitch accent becomes a stress accent ([chapter 2](02-accent.md); [chapter 15](15-verse.md), §7).
- **Attic -ττ-** gives way to the general -σσ- (θάλασσα), and Attic -ρρ- to -ρσ-.
- **The final ν** is added almost everywhere ([chapter 7](07-verb.md), §7).

## 2. Nouns and pronouns

- **The dual is gone.**
- **The dative** retreats, replaced by εἰς + accusative and by the genitive.
- **Possession** is expressed more and more by the unstressed genitive of the pronoun after the noun: ὁ πατήρ μου ([chapter 6](06-pronouns.md), §§4, 10).
- **ὅδε** almost disappears: 6 tokens against 166 of οὗτος in PTNK Genesis and Ruth ([chapter 6](06-pronouns.md), §10).
- **The superlative** in -τατος gives way to the comparative.

## 3. The verb

- **The optative** almost disappears: 3.9% of finite verbs in Herodotus, 0.3% in the New Testament ([chapter 14](14-subordination.md), §9).
- **The -μι verbs** go over to the thematic type: δεικνύω, ἱστάνω ([chapter 9](09-mi-verbs.md), §9).
- **Second-aorist forms** take first-aorist endings: εἶπα, ἦλθαν.
- **The 3rd plural** in -οσαν spreads: εἴποσαν, ἤλθοσαν ([chapter 8](08-contract-verbs.md), §5).
- **The perfect and the aorist** begin to merge in meaning.
- **The future** is often replaced by periphrases (θέλω + infinitive, ἔχω + infinitive), the start of the Modern Greek future θα.

## 4. Syntax

- **ὅτι and ἵνα** with finite verbs replace the infinitive: the accusative and infinitive outnumbers ὅτι eight to one in Herodotus, and ὅτι outnumbers it two to one in the New Testament ([chapter 14](14-subordination.md), §9). ἵνα spreads to every kind of complement and becomes the Modern Greek να.
- **The order moves to SVO:** 47% of the clauses with a noun subject and an object in the New Testament ([chapter 13](13-simple-clause.md), §5).
- **Prepositions widen:** εἰς for ἐν "in", ἀπό for ἐκ and ὑπό ([chapter 12](12-prepositions-particles.md), §1).
- **The genitive absolute** loosens its classical rule ([chapter 10](10-nonfinite.md), §6).

## 5. Translation Greek: the Septuagint

The Septuagint is Koine translated word for word from Hebrew, and much of its strangeness to an Attic reader comes from the source:
- **verb first** in narrative: VSO in 69% of the counted clauses ([chapter 13](13-simple-clause.md), §5);
- **καί** joining every clause: 76 a thousand words; μέν almost absent ([chapter 12](12-prepositions-particles.md), §9);
- **ἐγένετο** "it came to pass" opening a narrative (Hebrew *wayəhî*);
- **the genitive of the construct chain**: υἱοὶ Ἰσραήλ, and the first noun without the article ([chapter 5](05-article.md), §5);
- **the possessive αὐτοῦ, σου** after every noun that has a suffix in Hebrew ([chapter 6](06-pronouns.md), §10);
- **the resumptive pronoun** in relative clauses ([chapter 6](06-pronouns.md), §10);
- **τοῦ + infinitive** of purpose for *lə-* + infinitive, ἐν τῷ + infinitive for *bə-* ([chapter 10](10-nonfinite.md), §3);
- **ἐναντίον, ἐνώπιον** "before the face of" ([chapter 12](12-prepositions-particles.md), §3);
- **the infinitive absolute** rendered by a cognate dative or participle: θανάτῳ ἀποθανεῖσθε "you shall surely die" (Gen. 2:17), as the Vulgate's *morte morieris* ([la chapter 13](../la/13-late-latin.md), §6).

The Latin Vulgate shows the same features, and Mova's parallel Bibles put the Hebrew, the Greek, the Latin and the English of one verse side by side ([Holy Wars](https://mova.elamur.ai/free/en/holy-wars/)).

## 6. What was measured

All on UD 2.18 training sets, counted 2026-10-22; PROIEL and Perseus are NC and counted only:

| measure | classical | Koine | chapter |
|---|---|---|---|
| optative, share of finite verbs | Herodotus 3.9% | New Testament 0.3%; Septuagint 2.0% | [14](14-subordination.md) |
| accusative + infinitive : ὅτι clauses | Herodotus 583 : 69 | New Testament 398 : 875; Septuagint 23 : 93 | [14](14-subordination.md) |
| the commonest order | classical verse OVS 30%; Herodotus SVO 37% | New Testament SVO 47%; Septuagint VSO 69% | [13](13-simple-clause.md) |
| καί per 1 000 words | — | Septuagint 76 | [12](12-prepositions-particles.md) |
| aorist share of indicatives | — | Septuagint 1 264 of 1 942 | [7](07-verb.md) |

## 7. How Mova handles it

Mova's Greek measure is PTNK, which is Koine (the Septuagint): LAS 74.41 with Mova's own tags and 78.29 with gold tags (dev, 2026-10-22). Two things follow:
- **the Koine is the right first target** for a Greek reader in Mova: its order is closer to English, its syntax has finite clauses where Attic has infinitives and participles, and its largest texts (the Septuagint, the New Testament) are parallel to the Hebrew, the Latin and the English Mova already reads;
- **the measures of §6 are a stylometric signal**, computable on any parsed Greek text, that places a text between Herodotus and the Septuagint without a model.

The Koine manual, `grc-x-koine`, will be written from the corpus, as the Ancient Hebrew one was: its grammar from the Septuagint and the New Testament, its examples aligned verse by verse with the Hebrew and the Latin.

## 8. Sources

- Thackeray, H. St J. 1909, *A Grammar of the Old Testament in Greek according to the Septuagint* (thackeray-1909-grammar-old-testament-greek), public domain.
- Blass, F. 1905, *Grammar of New Testament Greek*, trans. Thackeray (blass-1905-grammar-new-testament-greek), public domain.
- Conybeare, F. C. & Stock, St G. 1905, *Selections from the Septuagint* (conybeare-1905-selections-from-septuagint-according), public domain: its grammar of Septuagint Greek.
- Burton, E. D. 1898 (burton-1898), public domain.
- UD Ancient Greek PTNK (CC BY-SA 4.0); UD Ancient Greek PROIEL (CC BY-NC-SA, counts only).
