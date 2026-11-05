# The lexicon: word formation, compounds, loans and dictionaries

Greek has a large vocabulary and makes new words easily: by suffixes, by prefixes on verbs, and above all by compounds, which Latin used little and Greek freely. Through Latin and later directly, it gave the European languages most of their words for science, philosophy, medicine and the Church. This chapter gives:
- the main suffixes and the classes of compounds;
- the loanwords Greek took from other languages and gave to them;
- the open dictionaries and morphological data a Greek lexicon for Mova can be built from, with their licences.

## Contents

1. Suffixes: nouns, adjectives, verbs
2. Compounds
3. Verbs with prefixes
4. Words Greek borrowed
5. Greek in Latin and English
6. The open dictionaries and lexical data
7. How Mova handles it
8. Sources

---

## 1. Suffixes: nouns, adjectives, verbs

(smyth-1920 §§838–868)

**Nouns:**
- agent: -τήρ, -τωρ, -της (σωτήρ "saviour", ῥήτωρ "speaker", ποιητής "maker, poet");
- action: -σις (ποίησις "making", κρίσις "judging");
- result: -μα (ποίημα "thing made, poem", πρᾶγμα "deed");
- quality: -ία, -σύνη, -της (σοφία "wisdom", δικαιοσύνη "justice", νεότης "youth");
- place: -τήριον, -εῖον (δικαστήριον "court", μουσεῖον "seat of the Muses");
- diminutive: -ιον, -ίσκος, -ίδιον (παιδίον "little child", νεανίσκος "young man").

**Adjectives:**
- -ικός "of, belonging to, skilled in" (πολιτικός, γραμματικός, μουσικός): the source of English *-ic*;
- -ιος (οὐράνιος "heavenly");
- -ινος of material (λίθινος "of stone");
- -ώδης "like" (θηριώδης "beastly");
- the verbal adjectives -τός, -τέος ([chapter 10](10-nonfinite.md), §9).

**Verbs:**
- denominatives in -έω, -άω, -όω (the contract verbs: [chapter 8](08-contract-verbs.md));
- -εύω (βασιλεύω "be king", παιδεύω "educate");
- -ίζω (ἑλληνίζω "speak Greek", νομίζω "consider", βαπτίζω "dip"): the source of English *-ize*;
- -αίνω, -ύνω (λευκαίνω "whiten", ἡδύνω "sweeten").

## 2. Compounds

Greek compounds join two stems, the first usually with a linking vowel -ο- (smyth-1920 §§869–899):
- **determinative** (the first limits the second): ἀκρό-πολις "upper city", ψευδο-προφήτης "false prophet";
- **verbal governing** (the first or second is a verb governing the other): φιλό-σοφος "loving wisdom", στρατ-ηγός "leading the army", ἱππό-δαμος "horse-taming";
- **possessive** (bahuvrihi: "having …"): ῥοδο-δάκτυλος "rosy-fingered", μεγαλό-ψυχος "great-souled";
- **with the negative prefix ἀ-, ἀν-**: ἄ-θεος "godless", ἀν-άξιος "unworthy", ἀ-θάνατος "immortal";
- **with εὐ- "well" and δυσ- "ill"**: εὐ-δαίμων "happy" (with a good spirit), δυσ-τυχής "unlucky".

The accent of compounds is in [chapter 2](02-accent.md). A compound's head is usually its second member. A lemmatizer meeting an unknown compound can split it at the linking vowel and look up the parts: φιλο-σοφ-ία.

## 3. Verbs with prefixes

Most Greek verbs take one or more prepositional prefixes, often with a spatial or completive sense ([chapter 12](12-prepositions-particles.md), §3):
- ἀπο-θνῄσκω "die";
- κατα-λύω "dissolve, put an end to";
- συν-άγω "gather together";
- ἀνα-γιγνώσκω "read" (recognise again);
- παρα-καλέω "summon, exhort, comfort".

The prefix may change the meaning entirely. The augment stands between it and the verb ([chapter 7](07-verb.md), §4), so that a lemmatizer must strip the prefix, the augment and then find the stem: κατ-έ-λυσε → καταλύω.

## 4. Words Greek borrowed

- From the languages of the Aegean before Greek: words in -ινθος, -σσος (λαβύρινθος, Παρνασσός, ἀσάμινθος "bathtub").
- From the Semitic languages, early: χιτών "tunic", χρυσός "gold", οἶνος "wine" (shared), and the names of the letters themselves (ἄλφα, βῆτα from *ʾālep̄, bêt*).
- From Persian: παράδεισος "park, paradise", σατράπης.
- In the Koine, from Latin: κεντυρίων, λεγεών, δηνάριον, κῆνσος.
- From Hebrew, through the Septuagint: σάββατον, πάσχα, ἀμήν, ἀλληλουϊά, σατανᾶς.

## 5. Greek in Latin and English

Greek words came into Latin from the earliest contacts: everyday words (*oleum*), the learned words of philosophy and rhetoric, and the vocabulary of the Church ([la chapter 11](../la/11-numerals-lexicon.md), §7). Through Latin and French, and from the Renaissance directly, they make much of the learned vocabulary of English:
- philosophy, democracy, theatre, school (σχολή "leisure"), church (κυριακόν "the Lord's"), bishop (ἐπίσκοπος), priest (πρεσβύτερος);
- the prefixes and stems of science, combined as Greek never did: tele-phone, micro-scope, bio-logy.

## 6. The open dictionaries and lexical data

What the Scientist holds for Greek (`$RESEARCH_RAW/grc-lexres/`), with each item's licence. The folder `sa` holds share-alike data, `nc` non-commercial:

| resource | covers | licence | use in Mova |
|---|---|---|---|
| Abbott-Smith, *Manual Greek Lexicon of the New Testament* (1922), TEI edition | the New Testament, with the Septuagint and Hebrew equivalents | text public domain; the edition's licence to check | the first candidate for a Koine lexicon |
| Dodson, *Greek Lexicon* | the New Testament, short glosses | CC0 | can be built in |
| Strong's Greek dictionary (Open Scriptures) | New Testament words by Strong's number | public domain | the link to the Bible tools and to STEPBible |
| STEPBible data | morphology, lexicon and alignment of the Greek Bible | CC BY 4.0 | morphology and lemmas of the New Testament and the Septuagint |
| MorphGNT (SBLGNT morphology) | every word of the New Testament parsed | CC BY-SA | measure and learn only |
| Morpheus (Perseus) | a morphological analyser of classical Greek | share-alike | measure and learn only |
| Middle Liddell, Autenrieth (Alpheios) | classical; Homer | share-alike | measure and learn only |
| LSJ (Liddell–Scott–Jones), Unicode edition | the whole of classical Greek | in `nc` | tests only |

So for the Koine there is an open path: Abbott-Smith, Dodson, Strong's and STEPBible give lemmas, glosses and morphology under free licences. For classical Greek the main tools (LSJ, Morpheus) are share-alike or non-commercial. Mova may measure and learn with them, but not build them into a release ([licence/DATA.md](../../licence/DATA.md)).

## 7. How Mova handles it

Mova has no Greek lexicon yet. Its Greek lemmas come only from the treebank it parses, and a word not in the treebank has none. The plan, in order of what is open:
1. **The Koine lexicon** from Dodson (CC0), Strong's (public domain) and STEPBible (CC BY), linked by Strong's number to the English Bibles Mova already reads. This would put Greek words into the meta-world of the parallel Bibles.
2. **A lemmatizer by rule:**
   - the contraction table ([chapter 8](08-contract-verbs.md));
   - augment and reduplication stripping ([chapter 7](07-verb.md));
   - the prefix list (§3) and the dialect alternatives ([chapter 16](16-dialects.md));
   - the -μι verbs and the suppletive verbs as a closed list ([chapter 9](09-mi-verbs.md)).

   Every rule is deterministic, and each can be tested against the treebank lemmas, error by error.
3. **Compound splitting** at the linking vowel for unknown words (§2), as the English lexicon guesses from suffixes.

## 8. Sources

- Smyth, H. W. 1920 (smyth-1920): §§838–899 (formation of words), paraphrased from the Perseus text (CC BY-SA).
- The resources of §6, each under its own licence, as listed in the Scientist's `grc-lexres/resources.tsv`.
