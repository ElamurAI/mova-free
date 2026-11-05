# Syntax: the clause and the sentence

This chapter describes how Biblical Hebrew builds clauses and sentences: the order of verb, subject and object; the clause with no verb; agreement; the construct chain, apposition and the relative clause as parts of the noun phrase in the sentence; the object marker; the infinitives; coordination with ו and subordination with כִּי and אִם; questions, negation, fronting and dislocation, oaths, comparison and the existential particles. Every point is shown on a verse of the Torah as it stands in Mova's trees (`$MOVA_DATA/corpora/judaism/text/torah-wlc-macula/*.conllu`, converted from MACULA Hebrew by `tools/macula`), with the reading of the classical grammars (Gesenius–Kautzsch–Cowley 1910, Davidson 1901, Driver 1892) beside it, and with a note on where our head rules and the UD Ancient Hebrew treebank (PTNK) read the same construction differently. All counts were made on the trees of 2026-10-17 (5 853 sentences, 125 807 words, GEN–DEU).

How to read the examples. Each example gives the verse and the sentence id in our trees, the pointed text (accents left out for reading), a transliteration, a word-for-word gloss and the JPS 1917 translation. A tree excerpt gives, for each word, `id form → head relation [MACULA gloss] {Type}`. Prefixes such as ו, ה, ב, ל and pronoun suffixes are words of their own in the trees, as in UD.

## Contents

1. The verbal clause and its word order
2. The verbless (nominal) clause
3. Agreement of subject and predicate
4. The construct chain in the sentence
5. Apposition
6. The relative clause with אֲשֶׁר
7. The object: אֵת, definiteness, two objects
8. The infinitive construct
9. The infinitive absolute
10. Coordination: the ubiquitous ו
11. Subordination: כִּי, אִם, causal, final and temporal clauses
12. Conditional sentences
13. Questions
14. Negation and its scope
15. Fronting, casus pendens and left dislocation
16. Oaths
17. Comparison
18. Existence and absence: יֵשׁ and אַיִן
19. In Mova
20. Sources

---

## 1. The verbal clause and its word order

A verbal clause has a finite verb as its predicate (GKC §140, §142). Hebrew marks person, gender and number on the verb, so the subject may be left out altogether ("pro-drop"); an object pronoun is very often a suffix on the verb itself.

**The textbook order is verb – subject – object** (GKC §142f; Davidson 1901 §105). It is the order of every narrative clause opened by a wayyiqtol, because the wayyiqtol form must stand first in its clause (the ו is fused with it, so nothing can come between the conjunction and the verb). The same holds for weqatal, the "perfect with waw consecutive" of procedure and prediction (Driver 1892, ch. VIII).

> **GEN.1.3** · `wlc:GEN:2`
> וַיֹּאמֶר אֱלֹהִים יְהִי אוֹר וַיְהִי־אוֹר
> *wayyōʾmer ʾĕlōhîm yəhî ʾôr wayəhî-ʾôr* — and-said God let-be light and-was light — "And God said: 'Let there be light.' And there was light."

```
1 וַ        → 2 cc
2 יֹּאמֶר   → 0 root      [he.said]   {wayyiqtol}
3 אֱלֹהִים  → 2 nsubj     [God]
4 יְהִי     → 2 ccomp     [let.it.be] {jussive}
5 אוֹר      → 4 nsubj     [light]
6 וַ        → 7 cc
7 יְהִי     → 2 conj      [it.was]    {wayyiqtol}
8 אוֹר      → 7 nsubj     [light]
```

**What the trees show.** Counting, for every verb, whether its `nsubj` and `obj` stand before or after it (all five books, 2026-10-17):

| verb form (MISC `Type`) | subject before | subject after | object before | object after |
|---|---:|---:|---:|---:|
| wayyiqtol | 0 | 1 826 | 0 | 1 569 |
| weqatal | 0 | 543 | 0 | 1 391 |
| qatal | 400 | 774 | 212 | 1 006 |
| yiqtol | 561 | 701 | 525 | 1 264 |
| imperative | 17 | 13 | 30 | 316 |
| jussive | 14 | 58 | 7 | 69 |
| cohortative | 2 | 1 | 0 | 58 |
| participle (active) | 438 | 67 | 21 | 263 |
| participle (passive) | 44 | 46 | – | – |
| infinitive construct | 3 | 414 | 2 | 743 |

Three facts stand out.

- **wayyiqtol and weqatal never have anything before them** in their own clause — not a subject, not an object. This is a property of the forms, not a choice of word order.
- **With qatal and yiqtol the order is free.** The subject comes first in about a third of qatal clauses (400 of 1 174) and in nearly half of yiqtol clauses (561 of 1 262); objects come first in 15–30 %. These preverbal positions are where the language puts a topic, a contrast or a focus (section 15). The "X-qatal" clause, with something before the qatal, is the classic form of background information in narrative (see the discourse chapter, [09-discourse.md](09-discourse.md)).
- **The participle prefers its subject before it** (438 to 67): a participial clause behaves like a verbless clause, whose usual order with a pronoun or noun subject is subject–predicate (section 2; Joosten 2020).

**Is Hebrew VS or SV?** The question has been argued for a century. The traditional view (GKC §142f) takes verb-first as basic, and the counts above, dominated by wayyiqtol, seem to agree. Holmstedt (2011) argues that the counts mislead: wayyiqtol and the verb after a trigger word (כִּי, אֲשֶׁר, a question word, a negative) raise the verb above the subject for reasons of their own, and once those are set aside the basic order of Genesis is subject–verb. Joosten (2025) describes word order as dynamic: the verb-first order of narrative is old and fixed, while order in direct speech and in later books moves toward subject-first. Kaajan (2019) asks whether Hebrew is non-configurational at all, given its discontinuous phrases. Mova does not decide the question; its trees keep the surface order, and the table above is the measurement anyone can repeat.

**A fronted subject with qatal: background.**

> **GEN.4.1** · `wlc:GEN:80`
> וְהָאָדָם יָדַע אֶת־חַוָּה אִשְׁתּוֹ וַתַּהַר וַתֵּלֶד אֶת־קַיִן
> *wəhāʾāḏām yāḏaʿ ʾeṯ-ḥawwâ ʾištô wattahar wattēleḏ ʾeṯ-qayin* — and-the-man knew (ACC) Eve his-wife and-she-conceived and-she-bore (ACC) Cain — "And the man knew Eve his wife; and she conceived and bore Cain."

The subject stands before the qatal יָדַע, so the clause breaks the wayyiqtol chain of chapter 3 and opens a new episode; whether it also means a pluperfect ("had known") is a classic debate (Collins 1995; GKC §106f, §142b). Our tree: `הָאָדָם → nsubj` of the root `יָדַע {qatal}`, and the following wayyiqtols are its `conj`.

**Pro-drop.** In GEN.22.3 (`wlc:GEN:550`) Abraham is named once, and the next six wayyiqtol verbs (he saddled, he took, he split, he arose, he went …) carry no subject word. MACULA's `Frame=A0:…` in MISC still points each verb to its agent: in GEN.22.3 every verb's `A0` is the word id `010220030021`, אַבְרָהָם. So the argument structure survives where the tree has no `nsubj` (section 19).

## 2. The verbless (nominal) clause

A clause can have no verb at all: subject and predicate are simply set side by side, and English supplies "is/are" (GKC §140a, §141; Davidson 1901 §§103–106). The predicate can be a noun, an adjective, a participle, a prepositional phrase or an adverb. Tense is read from context: a verbless clause is timeless in itself.

> **GEN.2.12** · `wlc:GEN:42`
> וּזֲהַב הָאָרֶץ הַהִוא טוֹב שָׁם הַבְּדֹלַח וְאֶבֶן הַשֹּׁהַם
> *ûzăhaḇ hāʾāreṣ hahîʾ ṭôḇ šām habbəḏōlaḥ wəʾeḇen haššōham* — and-gold-of the-land the-that good; there the-bdellium and-stone-of the-onyx — "and the gold of that land is good; there is bdellium and the onyx stone."

```
2 זֲהַב  → 7 nsubj           [the.gold]
4 אָרֶץ  → 2 compound:smixut [the.land]
6 הִוא   → 4 det              [that]
7 טוֹב   → 0 root             [[is].good]  {adjective}
8 שָׁם   → 7 conj             [[are].there]
10 בְּדֹלַח → 8 nsubj         [bdellium]
```

The adjective predicate heads the clause and the subject depends on it as `nsubj`; this is the UD analysis of a copula-less clause (the predicate is the head). The second clause is headed by the adverb שָׁם "there".

**Order.** GKC (§141l) gives subject–predicate as the natural order of a noun clause, which describes a state, and the predicate first when it carries the stress. Later work (Andersen 1970, not in our library) refines this into a *classifying* clause, in which an indefinite predicate tells what kind of thing the subject is and tends to come first, and an *identifying* clause, with a definite predicate, in which the subject tends to come first. Counted on our trees (subject before / after a non-verbal predicate, 2026-10-17): adjective predicates 99 / 226 — predicate first wins two to one; noun predicates 467 / 391; proper-name predicates 215 / 86; pronoun predicates 84 / 129. The numbers fit the classifying/identifying contrast loosely; they are raw and include some mis-parsed clauses (below).

**The pronoun as a link: the tripartite clause.** A third-person pronoun often stands between subject and predicate, as in

> **DEU.4.35** · `wlc:DEU:146`
> … כִּי יְהוָה הוּא הָאֱלֹהִים אֵין עוֹד מִלְבַדּוֹ
> *kî YHWH hûʾ hāʾĕlōhîm ʾên ʿôḏ millaḇaddô* — that YHWH he the-God; there-is-not more besides-him — "that the LORD, He is God; there is none else beside Him."

and in GEN.2.11 (`wlc:GEN:41`) "הוּא הַסֹּבֵב אֵת כָּל־אֶרֶץ הַחֲוִילָה" "that is it which compasseth the whole land of Havilah". GKC (§141g–h) and Davidson (1901 §106) call the pronoun a copula or a resumption of the subject; others analyse the first noun as left-dislocated and the pronoun as the true subject (Naudé 2018 tests the two analyses against the accents and finds that prosody separates a dislocation from a tripartite clause; see section 15).

How the three analyses differ in a dependency tree:

```
our tree (DEU.4.35):   יְהוָה ← obj of לָדַעַת;  הָאֱלֹהִים → conj of יְהוָה;  הוּא → nsubj of הָאֱלֹהִים
PTNK convention:       הָאֱלֹהִים root of the clause;  הוּא → cop;  יְהוָה → nsubj (or dislocated)
dislocation reading:   הָאֱלֹהִים root;  הוּא → nsubj;  יְהוָה → dislocated
```

Our head rule has no `cop` relation at all: MACULA's group for this clause makes יְהוָה and the rest siblings, and our coordination rule turns the second into a `conj` of the first. This is wrong for this verse. The UD guidelines for Ancient Hebrew list `cop` among the relations used, and PTNK annotates pronominal copulas and many הָיָה clauses with it (UD Ancient Hebrew PTNK README; Swanson and Tyers 2022). The macula README records the difference: "PTNK also reads הָיָה as a copula in places where we keep it the head".

**הָיָה "be".** When the clause needs a tense, an aspect or a mood, the verb הָיָה supplies it, and the predicate becomes its complement.

> **GEN.3.1** · `wlc:GEN:56`
> וְהַנָּחָשׁ הָיָה עָרוּם מִכֹּל חַיַּת הַשָּׂדֶה
> *wəhannāḥāš hāyâ ʿārûm mikkōl ḥayyaṯ haśśāḏe* — and-the-snake was crafty from-all beast-of the-field — "Now the serpent was more subtle than any beast of the field."

In our tree הָיָה is the root and עָרוּם is its `obj` (MACULA gives the predicate the role `o` here; in GEN.1.2 the predicate תֹהוּ of הָיְתָה is also `obj`). A UD tree would make the predicate an `xcomp` of הָיָה or, with PTNK, make the adjective the head and הָיָה its `cop`. This is a known divergence (section 19).

**A clause with a negative verbless predicate** uses אֵין (section 18); a participial clause (section 1) is formally a verbless clause with a participle as predicate (GKC §116).

## 3. Agreement of subject and predicate

The verb agrees with its subject in person, gender and number; an attributive adjective agrees with its noun in gender, number and definiteness (GKC §145; Davidson 1901 §§112–116). The trees carry the features on both sides (`Gender`, `Number`, `Person` in FEATS), so agreement can be checked mechanically.

**Exceptions that are rules of their own.**

- **A verb before its subject may stay in the masculine singular** even with a feminine or plural subject (GKC §145o; Davidson 1901 §113): the verb is uttered before the speaker "commits" to the subject.
- **Collective nouns** (עָם "people", צֹאן "flock", כָּל־ "all of") take a plural verb as often as a singular one (GKC §145b–e; Davidson 1901 §115).

> **EXO.19.8** · `wlc:EXO:501`
> וַיַּעֲנוּ כָל־הָעָם יַחְדָּו וַיֹּאמְרוּ כֹּל אֲשֶׁר־דִּבֶּר יְהוָה נַעֲשֶׂה
> *wayyaʿănû ḵol-hāʿām yaḥdāw wayyōʾmərû kōl ʾăšer-dibber YHWH naʿăśe* — and-they-answered all-of the-people together and-they-said: all that-spoke YHWH we-will-do — "And all the people answered together, and said: 'All that the LORD hath spoken we will do.'"

  The verb is plural (`Number=Plur`), its subject כָל־ singular in form; the trees keep the mismatch visible.

- **The plural of majesty.** אֱלֹהִים "God" is plural in form (`Number=Plur` in our FEATS) but, referring to the God of Israel, takes a singular verb and singular adjectives (GKC §124g, §132h; Davidson 1901 §116 R. 4): בָּרָא אֱלֹהִים "God created" (GEN.1.1, verb `Number=Sing`). In a few places it takes a plural verb:

> **GEN.20.13** · `wlc:GEN:508`
> וַיְהִי כַּאֲשֶׁר הִתְעוּ אֹתִי אֱלֹהִים מִבֵּית אָבִי
> *wayəhî kaʾăšer hiṯʿû ʾōṯî ʾĕlōhîm mibbêṯ ʾāḇî* — and-it-was when caused-to-wander(PL) me God from-house-of my-father — "And it came to pass, when God caused me to wander from my father's house …"

  and GEN.35.7 (`wlc:GEN:1018`) נִגְלוּ אֵלָיו הָאֱלֹהִים "there God was revealed unto him", with a plural verb. GKC (§145i) discusses these as remnants of a wider sense of the word. For a machine reader the lesson is simple: `Number=Plur` on אֱלֹהִים is morphology, not meaning, and an agreement check must exempt it.

- **Dual subjects** take plural verbs and adjectives (GKC §145n); there is no dual verb.

## 4. The construct chain in the sentence

The form of the chain (construct state, vowel reduction, the article on the last member only) belongs to the noun chapter. Its syntax is: the first noun (the *nomen regens*, in construct) is the head; the second (the *nomen rectum*, in the absolute state) depends on it and fixes its definiteness — the whole chain is definite if and only if its last member is (GKC §127, §128; Davidson 1901 §§23–28).

> **GEN.47.9** · `wlc:GEN:1429`
> יְמֵי שְׁנֵי מְגוּרַי שְׁלֹשִׁים וּמְאַת שָׁנָה
> *yəmê šənê məḡûray šəlōšîm ûməʾaṯ šānâ* — days-of years-of my-sojournings thirty and-hundred year — "The days of the years of my sojournings are a hundred and thirty years."

```
6  יְמֵי     → 10 nsubj
7  שְׁנֵי    → 6  compound:smixut
8  מְגוּר    → 7  compound:smixut
9  ַי        → 8  nmod:poss
10 שְׁלֹשִׁים → 2 ccomp  [[are].thirty]
12 מְאַת     → 10 flat
```

The chain is a ladder: each member depends on the one before it. Later in the same verse the chain grows to four nouns: אֶת־יְמֵי שְׁנֵי חַיֵּי אֲבֹתַי "the days of the years of the life of my fathers".

**UD and our trees.** The UD guidelines for Hebrew use the subtype `compound:smixut` (*smixut* is the Modern Hebrew name of the construct state) for the dependent of a construct noun; our first head rules used plain `nmod`, and switching to `compound:smixut` was one of the changes that raised our agreement with PTNK from LAS 73.79 to 82.23 on test (macula README, 2026-10-17). We have 7 421 `compound:smixut` arcs in the Torah. A pronoun suffix on a noun is `nmod:poss` (6 596 arcs); a genitive expressed with a preposition (לְ "of", GKC §129) is `nmod` with `case`.

**Where the rule needs care.** A coordinated rectum (GEN.1.1 does not have one, but EXO.19.8's דִּבְרֵי הָעָם does) can make it unclear whether the second noun depends on the first in construct or is coordinated with it; the README lists `conj` → `nmod` (385 cases in train) among the remaining differences from PTNK. The construct state itself is not in our FEATS (MACULA marks it, but `tools/macula` does not yet carry it over), so a reader of the trees learns that a noun is in construct only from the `compound:smixut` arc below it.

## 5. Apposition

Two noun phrases naming the same thing stand side by side, the second explaining the first (GKC §131; Davidson 1901 §29).

> **GEN.22.2** · `wlc:GEN:549`
> קַח־נָא אֶת־בִּנְךָ אֶת־יְחִידְךָ אֲשֶׁר־אָהַבְתָּ אֶת־יִצְחָק
> *qaḥ-nāʾ ʾeṯ-binḵā ʾeṯ-yəḥîḏəḵā ʾăšer-ʾāhaḇtā ʾeṯ-yiṣḥāq* — take please (ACC) your-son (ACC) your-only-one whom you-love (ACC) Isaac — "Take now thy son, thine only son, whom thou lovest, even Isaac."

```
3  קַח      → 2 ccomp  {imperative}
4  נָא      → 3 advmod {exhortation}
6  בִּנ      → 3 obj
9  יְחִיד   → 6 appos
12 אָהַבְתָּ → 9 acl:relcl
14 יִצְחָק  → 6 appos
```

Each apposed member repeats the object marker; the verse narrows step by step from "your son" to the name, a famous rhetorical climax. Other common types: a name with a title or origin (לְאַבְרָם הָעִבְרִי "to Abram the Hebrew", GEN.14.13, `appos`), a divine name with its epithet (יְהוָה אֱלֹהִים, which our trees parse as `appos` throughout GEN.2–3), and a measure after a noun. We have 1 966 `appos` arcs.

## 6. The relative clause with אֲשֶׁר

אֲשֶׁר is not a pronoun but an undeclinable marker that introduces a clause (GKC §138, §155; Davidson 1901 §9, §142). The role of the head noun inside the relative clause is shown, if at all, by a **resumptive** pronoun in the clause. In our trees אֲשֶׁר is `mark` (SCONJ, `Type=relative`, 1 908 times) and the relative clause is `acl:relcl` of its head noun (1 475 arcs).

**Resumptive pronouns.**

> **GEN.28.13** · `wlc:GEN:786`
> הָאָרֶץ אֲשֶׁר אַתָּה שֹׁכֵב עָלֶיהָ לְךָ אֶתְּנֶנָּה וּלְזַרְעֶךָ
> *hāʾāreṣ ʾăšer ʾattâ šōḵēḇ ʿālêhā ləḵā ʾettənennâ ûləzarʿeḵā* — the-land which you lying on-it, to-you I-will-give-it and-to-your-seed — "The land whereon thou liest, to thee will I give it, and to thy seed."

עָלֶיהָ "on it" resumes הָאָרֶץ inside the relative clause; and the whole phrase is resumed once more by the suffix of אֶתְּנֶנָּה "I will give it" — a relative clause inside a dislocation (section 15).

A relative clause can have no verb: in GEN.1.11 (`wlc:GEN:10`) עֵץ פְּרִי … אֲשֶׁר זַרְעוֹ־בוֹ "fruit-tree … wherein is the seed thereof", the clause is "its seed (is) in it". Our tree makes the preposition's pronoun בוֹ the head of the clause (`acl:relcl`) — the head rule for verbless clauses picks the predicate, here the prepositional phrase.

**Without resumption.** When the head noun is the subject or object of the relative verb, the pronoun is usually not expressed: כֹּל אֲשֶׁר־דִּבֶּר יְהוָה "all that the LORD has spoken" (EXO.19.8), the object gap after דִּבֶּר.

**Headless relatives.** אֲשֶׁר with no head noun means "that which, the one who", often after the object marker:

> **GEN.9.24** · `wlc:GEN:229`
> וַיֵּדַע אֵת אֲשֶׁר־עָשָׂה־לוֹ בְּנוֹ הַקָּטָן
> *wayyēḏaʿ ʾēṯ ʾăšer-ʿāśâ-lô bənô haqqāṭān* — and-he-knew (ACC) that-which did to-him his-son the-small — "and knew what his youngest son had done unto him."

Here the relative verb itself is the `obj` of וַיֵּדַע, with אֵת as its `case` — the head rule promotes the clause to the object slot. PTNK annotates headless relatives in a similar way (a clause in a nominal slot); the UD guidelines for Hebrew allow the clause itself to fill the core relation.

**Other relatives.** The article on a participle works as a relative ("the one who …", GKC §138i), e.g. הַסֹּבֵב "which goes around" in GEN.2.11; the old relative זוּ / שֶׁ belongs to poetry and later Hebrew (שֶׁ becomes the normal relative in Mishnaic Hebrew; see the Mishnaic chapter).

## 7. The object: אֵת, definiteness, two objects

**The object marker אֵת / אֶת־** stands before a *definite* direct object: a noun with the article, a proper name, a noun with a suffix, a construct chain ending in a definite noun, or a pronoun (אֹתִי, אֹתוֹ …) (GKC §117a–e; Davidson 1901 §66 ff.). It is optional: poetry and some prose omit it. Bunis (2025) and Boulet (2019) discuss its history and its status as a case marker.

In our trees אֵת is ADP with `case`, `Type=direct_object_marker` (4 153 tokens), attached to the object noun. Counting the objects (`obj` arcs whose head is a noun, name or pronoun), by how the object is marked definite (2026-10-17):

| object | with אֵת | without |
|---|---:|---:|
| noun with the article | 676 | 88 |
| proper name | 339 | 107 |
| noun with a suffix | 633 | 458 |
| head of a construct chain | 430 | 456 |
| independent pronoun or suffix | 762 | 1 248 |
| bare noun (no article, suffix or rectum) | 222 | 1 334 |

Read with care: "without" for pronouns is mostly the suffix on the verb itself (וַיַּלְבִּשֵׁם "and he clothed them", GEN.3.21), which takes no marker; a "bare" noun with אֵת is mostly כֹּל "all" heading a definite phrase (אֵת כָּל־אֶרֶץ הַחֲוִילָה, GEN.2.11) or a name the rules did not mark PROPN. With those caveats the rule is plain: an article-marked object takes אֵת nine times in ten, an indefinite one almost never.

**Two objects.** Verbs of making, clothing, filling, naming and giving can take two accusatives (GKC §117cc–ii; Davidson 1901 §§76–78):

> **GEN.2.7** · `wlc:GEN:37`
> וַיִּיצֶר יְהוָה אֱלֹהִים אֶת־הָאָדָם עָפָר מִן־הָאֲדָמָה
> *wayyîṣer YHWH ʾĕlōhîm ʾeṯ-hāʾāḏām ʿāp̄ār min-hāʾăḏāmâ* — and-formed YHWH God (ACC) the-man dust from-the-ground — "Then the LORD God formed man of the dust of the ground."

Our tree makes הָאָדָם the `obj` and עָפָר an `advmod` — the material is a second accusative of a looser kind (GKC §117hh). MACULA has a role `o2` for a second object, which our rules map to `obj` as well; UD would prefer `obj` + `obl` or `xcomp`.

**An indirect object.** The recipient is usually a prepositional phrase with לְ or אֶל (`obl` in our trees); MACULA's `io` role becomes `iobj`, which PTNK added only in UD 2.17.

## 8. The infinitive construct

The infinitive construct is a verbal noun: it can take a subject (as a suffix or a following noun, in a genitive relation) and an object, and it can follow a preposition (GKC §114–115; Davidson 1901 §§89–96; Doron 2020). In our trees it is mostly `advcl` (1 466 of 1 739), because our rules make an infinitive after a preposition an adverbial clause — one of the UD conventions that raised LAS against PTNK.

**Purpose with לְ.**

> **GEN.2.15** · `wlc:GEN:45`
> וַיַּנִּחֵהוּ בְגַן־עֵדֶן לְעָבְדָהּ וּלְשָׁמְרָהּ
> *wayyanniḥēhû ḇəḡan-ʿēḏen ləʿāḇəḏāh ûləšāmərāh* — and-he-put-him in-garden-of Eden to-work-it and-to-keep-it — "and put him into the garden of Eden to dress it and to keep it."

```
9  יַּנִּח   → 2  conj   {wayyiqtol}
15 עָבְד   → 9  advcl  [till]  {infinitive_construct}
14 לְ       → 15 case
16 ָהּ      → 15 obj
19 שָׁמְר   → 15 conj   {infinitive_construct}
```

**Temporal with בְּ / כְּ.** "When …" is most often an infinitive after בְּ (at the time of) or כְּ (just as), with the subject as a suffix or noun (GKC §114e, §164g):

> **GEN.4.8** · `wlc:GEN:87`
> וַיְהִי בִּהְיוֹתָם בַּשָּׂדֶה וַיָּקָם קַיִן אֶל־הֶבֶל אָחִיו
> *wayəhî bihyôṯām baśśāḏe wayyāqām qayin ʾel-heḇel ʾāḥîw* — and-it-was in-their-being in-the-field and-rose Cain against Abel his-brother — "And it came to pass, when they were in the field, that Cain rose up against Abel his brother."

`הְיוֹת → advcl` of וַיְהִי, its suffix `ָם → nsubj`. GEN.2.4 has בְּהִבָּרְאָם "when they were created" and בְּיוֹם עֲשׂוֹת יְהוָה "in the day the LORD made", where the infinitive is the `acl` of יוֹם — the construct "day of making".

**Complement and the quotative לֵאמֹר.** After verbs of beginning, being able, refusing, and in לֵאמֹר "saying" (306 times in the Torah), the infinitive with לְ completes the verb; לֵאמֹר is treated in the discourse chapter. **Negation** of the infinitive uses לְבִלְתִּי or בִּלְתִּי, not לֹא (GEN.3.11 לְבִלְתִּי אֲכָל־מִמֶּנּוּ "that thou shouldest not eat of it"; Ehrensvärd 2022).

## 9. The infinitive absolute

The infinitive absolute has no suffixes and no prepositions. Its main uses (GKC §113; Davidson 1901 §§84–88):

**Paronomastic: before or after a finite verb of the same root, to strengthen or make certain.** This is the overwhelming use in the Torah: of 255 infinitives absolute, 191 depend (`advmod`) on a verb of the same lemma (2026-10-17).

> **GEN.2.17** · `wlc:GEN:47`
> כִּי בְּיוֹם אֲכָלְךָ מִמֶּנּוּ מוֹת תָּמוּת
> *kî bəyôm ʾăḵālḵā mimmennû môṯ tāmûṯ* — for in-day-of your-eating from-it dying you-will-die — "for in the day that thou eatest thereof thou shalt surely die."

```
20 מוֹת   → 21 advmod [surely.(die)] {infinitive_absolute}
21 תָּמוּת → 3 conj    [you.will.die] {yiqtol}
```

**In place of an imperative or a finite verb** (GKC §113bb; "imperatival infinitive"), most famously in the Decalogue:

> **EXO.20.8** · `wlc:EXO:526`
> זָכוֹר אֶת־יוֹם הַשַּׁבָּת לְקַדְּשׁוֹ
> *zāḵôr ʾeṯ-yôm haššabbāṯ ləqaddəšô* — remembering (ACC) day-of the-sabbath to-sanctify-it — "Remember the sabbath day, to keep it holy."

Here the infinitive absolute is the root of the sentence (12 such roots in the Torah). Deuteronomy's version has שָׁמוֹר "keep" (DEU.5.12).

**Adverbial**: הַרְבֵּה "much", הֵיטֵב "well" are infinitives absolute used as adverbs (GKC §113k).

**In Mova.** MACULA types the form (`Type=infinitive_absolute`), and our tree hangs it on its verb as `advmod`. UD would accept `advmod` or `obl`; PTNK's choice differs in places (`obl` → `advmod` is one of the README's listed differences). The macula README lists the infinitive absolute among the constructions the general head rules do not treat specially; the paronomastic pair is nevertheless rendered consistently because MACULA groups the two words together.

## 10. Coordination: the ubiquitous ו

ו "and" is the commonest word of the Torah (13 768 times in the dictionary `torah-en.tsv`, key `lemma:וְ`). It joins words, phrases and clauses, and it opens most sentences of narrative. BDB (1906, s.v. ו) notes that it was originally not a mere "and" but had a demonstrative force, "so, then", which survives in its many uses: copulative, adversative ("but"), explicative ("namely"), consecutive ("so that"), and introducing an apodosis ("then", the *waw apodosis*, GKC §143d; Zulker 2025).

**Clauses joined by ו** (GKC §154; Davidson 1901 §136). Hebrew prefers to chain clauses with ו where English subordinates: "and it came to pass … and Cain rose" is English "when … Cain rose". A reader must recover the relation — temporal, causal, circumstantial, contrastive — from the verb forms and the word order:

- ו + wayyiqtol: the next event (the narrative chain; see the discourse chapter);
- ו + subject + qatal / participle / verbless: a circumstance or a contrast, "while", "now", "but" (GKC §156; Driver 1892, Appendix I). GEN.18.1 וְהוּא יֹשֵׁב פֶּתַח־הָאֹהֶל "as he sat in the tent door" (`wlc:GEN:425`) is a participial circumstantial clause;
- ו + weqatal: the next step of a procedure or a prediction.

**In the trees.** Every ו is `cc` (CCONJ, 14 183 arcs) of the member after it, and the later members are `conj` of the first (13 460 arcs). A ו at the head of a clause hangs on the clause's head; a conjunction never hangs on another conjunction — a bug of our first version fixed on 2026-10-17 (macula README). The UD convention, and PTNK, distinguish clause chains from coordination more finely: the README lists `parataxis` → `conj` (788 cases in train) as our single largest difference. In a long narrative verse every wayyiqtol becomes `conj` of the first (GEN.4.1 above: וַתַּהַר, וַתֵּלֶד, וַתֹּאמֶר all `conj` of יָדַע), and that flat structure is exactly what the discourse layer reads (see 09-discourse.md).

**Disjunction** uses אוֹ "or" (GKC §162): קַן־צִפּוֹר … בְּכָל־עֵץ אוֹ עַל־הָאָרֶץ "a bird's nest … in any tree or on the ground" (DEU.22.6, in the dictionary's examples).

## 11. Subordination: כִּי, אִם, causal, final and temporal clauses

Hebrew has few subordinating words, and each does many jobs. Our rules make these `mark` (SCONJ): כִּי, אִם, לְמַעַן, פֶּן, טֶרֶם, יַעַן, עֵקֶב, לְבִלְתִּי (the list `SUBORDINATORS` in `tools/macula/src/main.rs`), plus אֲשֶׁר (`Type=relative`). Counts of `mark`: אֲשֶׁר 1 908, כִּי 1 015, אִם 327, לְמַעַן 75, פֶּן 62.

**כִּי** introduces an object clause ("that"), a causal clause ("because"), a temporal or conditional clause ("when, if"), an adversative after a negative ("but rather"), and an asseverative "indeed" (GKC §157–159; BDB s.v. כִּי). Staps's annotated set of כִּי clauses (2024, CC BY 4.0, in `hebgram/discourse/open/staps-ki`, 857 rows of which 290 are in the Torah) labels them causal 433, complementizer 200, resultative 41, standalone 32, temporal 30, exceptive 22, causal-adversative 21, adversative 14, conditional 6 — causal and complement uses make up three quarters. Benigni (2022) reads כִּי from a discourse point of view.

> **GEN.1.4** · `wlc:GEN:3`
> וַיַּרְא אֱלֹהִים אֶת־הָאוֹר כִּי־טוֹב
> *wayyarʾ ʾĕlōhîm ʾeṯ-hāʾôr kî-ṭôḇ* — and-saw God (ACC) the-light that-good — "And God saw the light, that it was good."

The famous "prolepsis": the subject of the object clause (הָאוֹר) is drawn up into the main clause as the object of "saw" (compare GKC §117h on verbs of perceiving with a second, predicative object). In our tree כִּי is `mark` of טוֹב, which is a clause depending on the verb.

**Causal** with כִּי (most common), יַעַן, יַעַן אֲשֶׁר, עֵקֶב, תַּחַת אֲשֶׁר (GKC §158):

> **GEN.3.14** · `wlc:GEN:69`
> כִּי עָשִׂיתָ זֹּאת אָרוּר אַתָּה מִכָּל־הַבְּהֵמָה
> *kî ʿāśîṯā zōʾṯ ʾārûr ʾattâ mikkol-habbəhēmâ* — because you-did this cursed you from-all the-cattle — "Because thou hast done this, cursed art thou from among all cattle."

The causal clause comes first and the main clause follows without a conjunction.

**Final (purpose)** with לְמַעַן "in order that", or with ו + cohortative/jussive after a volitive; **negative purpose** with פֶּן "lest" (GKC §165; GEN.3.3 פֶּן־תְּמֻתוּן "lest ye die").

**Temporal** with כִּי, כַּאֲשֶׁר "when, as", טֶרֶם "before, not yet", עַד "until", אַחֲרֵי "after", or with בְּ/כְּ + infinitive (section 8) (GKC §164). טֶרֶם with a yiqtol even in past narrative (GEN.2.5 טֶרֶם יִהְיֶה "was not yet", `wlc:GEN:35`) is a known peculiarity (GKC §107c).

## 12. Conditional sentences

A conditional sentence has a protasis ("if …") and an apodosis ("then …") (GKC §159; Davidson 1901 §§129–132; Driver 1892, ch. XI). **אִם** marks a condition the speaker presents as open or hypothetical; **כִּי** marks a case that is expected to occur ("when", the typical opening of a case law); **לוּ / לוּלֵא** mark an unreal condition ("if only, had not"). The apodosis is often introduced by ו + weqatal.

> **GEN.18.26** · `wlc:GEN:450`
> אִם־אֶמְצָא בִסְדֹם חֲמִשִּׁים צַדִּיקִם בְּתוֹךְ הָעִיר וְנָשָׂאתִי לְכָל־הַמָּקוֹם בַּעֲבוּרָם
> *ʾim-ʾemṣāʾ ḇisḏōm ḥămiššîm ṣaddîqim bəṯôḵ hāʿîr wənāśāʾṯî ləḵol-hammāqôm baʿăḇûrām* — if I-find in-Sodom fifty righteous in-midst-of the-city and-I-will-forgive to-all the-place for-their-sake — "If I find in Sodom fifty righteous within the city, then I will forgive all the place for their sake."

**Case law with כִּי** — the protasis states the case, the apodosis the ruling:

> **EXO.21.2** · `wlc:EXO:546`
> כִּי תִקְנֶה עֶבֶד עִבְרִי שֵׁשׁ שָׁנִים יַעֲבֹד וּבַשְּׁבִעִת יֵצֵא לַחָפְשִׁי חִנָּם
> *kî ṯiqne ʿeḇeḏ ʿiḇrî šēš šānîm yaʿăḇōḏ ûḇaššəḇiʿiṯ yēṣēʾ laḥop̄šî ḥinnām* — when you-buy servant Hebrew, six years he-shall-serve, and-in-the-seventh he-shall-go-out to-freedom for-nothing — "If thou buy a Hebrew servant, six years he shall serve; and in the seventh he shall go out free for nothing."

**How our trees render the pair.** In GEN.18.26 the protasis verb אֶמְצָא is the `ccomp` of וַיֹּאמֶר (speech), and the apodosis weqatal וְנָשָׂאתִי is its `conj` — the dependency of the apodosis on the condition is flattened into coordination. In EXO.21.2 the protasis verb תִקְנֶה is the *root*, with the apodosis יַעֲבֹד as its `conj`. A UD tree would make the apodosis the main clause and the protasis an `advcl` of it. This inversion is systematic in our trees (MACULA groups the protasis as the first clause of the sentence, and our clause rule takes the first clause as head when there is no verb role) and is part of the `advcl` → `conj` difference (247 cases in train) in the README. A reader that wants "if A then B" must currently reconstruct it from `mark=אִם/כִּי` on the first member of a `conj` pair.

## 13. Questions

**Yes/no questions** are marked by the prefix הֲ (GKC §150; Davidson 1901 §§121–126), or by intonation alone (then only context shows a question). **Content questions** use מִי "who", מָה "what", אֵי / אַיֵּה "where", אֵיךְ / אֵיכָה "how", לָמָּה / מַדּוּעַ "why", מָתַי "when". Rhetorical questions with הֲלוֹא "is it not …?" assert the opposite of their form.

> **GEN.4.9** · `wlc:GEN:88`
> אֵי הֶבֶל אָחִיךָ … לֹא יָדַעְתִּי הֲשֹׁמֵר אָחִי אָנֹכִי
> *ʾê heḇel ʾāḥîḵā … lōʾ yāḏaʿtî hăšōmēr ʾāḥî ʾānōḵî* — where Abel your-brother … not I-know; Q-keeper-of my-brother I? — "'Where is Abel thy brother?' And he said: 'I know not; am I my brother's keeper?'"

```
6  אֵי     → 2  ccomp  [where?]  {interrogative}
7  הֶבֶל   → 6  nsubj
14 הֲ      → 15 advmod {interrogative}
15 שֹׁמֵר  → 2  conj   [keeper?]  {participle_active}
18 אָנֹכִי → 15 nsubj
```

In the trees the interrogative words carry `Type=interrogative` (342 tokens): the question words are PRON or PART and the prefix הֲ is PART (`advmod`). The question word heads a verbless question (אֵי is the predicate of "where is Abel"). In 2026-10-17's trees הֲ appears 115 times as an `advmod`. **Disjunctive questions** ("A or B?") use הֲ … אִם (GKC §150c, §162). Note in GEN.18.24 (`wlc:GEN:448`) the interrogative הַ (pointed with patach before a guttural-like context) on הַאַף תִּסְפֶּה "wilt Thou indeed sweep away …?", glossed `?` by MACULA and typed interrogative — distinct from the article, which is DET.

**In the trees, a question inside speech is attached to the frame.** The second question of GEN.3.11 (הֲמִן־הָעֵץ … אָכָלְתָּ "hast thou eaten of the tree …?") becomes `conj` of וַיֹּאמֶר, not of the first question, because our `ccomp` rule fires only for the clause directly after the verb of saying. The pragmatics layer (09-discourse.md) inherits this.

## 14. Negation and its scope

- **לֹא** negates a verbal clause in the indicative, and a permanent prohibition with yiqtol (GKC §152; §107o): לֹא תִּרְצָח "thou shalt not murder" (EXO.20.13, `wlc:EXO:531`). 1 302 times as `advmod` in the Torah.
- **אַל** negates a jussive or a cohortative: an immediate, specific prohibition or a wish (GKC §109c, §152f):

> **GEN.22.12** · `wlc:GEN:559`
> אַל־תִּשְׁלַח יָדְךָ אֶל־הַנַּעַר וְאַל־תַּעַשׂ לוֹ מְאוּמָּה
> *ʾal-tišlaḥ yāḏəḵā ʾel-hannaʿar wəʾal-taʿaś lô məʾûmmâ* — not-stretch your-hand to the-lad and-not-do to-him anything — "Lay not thy hand upon the lad, neither do thou any thing unto him."

- **אֵין / אַיִן** negates existence and the verbless or participial clause (section 18).
- **בִּלְתִּי / לְבִלְתִּי** negates the infinitive (section 8) and some nouns; **טֶרֶם** "not yet"; **בַּל** and **בְּלִי** in poetry.

**Scope.** The negative stands immediately before the word it negates; it negates the clause when it stands before the verb, a constituent when it stands before that constituent (contrastive negation, GKC §152d). Miller-Naudé and Naudé (2019) use the position of negatives to tell apart left dislocation, topicalisation and extraposition (section 15). In our trees the negative is `advmod` of the word after it, so scope is read from the attachment: on the verb, clause negation; on a noun or phrase, constituent negation. The `macula prag` layer counts a sentence as negative only if a negative stands in the root's own clause (see 09-discourse.md, where we show that this can leak through flat clause chains).

## 15. Fronting, casus pendens and left dislocation

Constituents can come before the verb for reasons of information structure. Three constructions are distinguished in recent work (Westbury 2014, 2016; Miller-Naudé and Naudé 2019; Van der Merwe 2018; Floor 2004):

- **Fronting (topicalisation, focus).** A constituent of the clause is placed before the verb with no resumption. It marks a contrastive topic or a focus, often an exhaustive one ("this and nothing else", Van der Merwe 2018).

> **GEN.3.14** · `wlc:GEN:69`
> עַל־גְּחֹנְךָ תֵלֵךְ וְעָפָר תֹּאכַל כָּל־יְמֵי חַיֶּיךָ
> *ʿal-gəḥōnḵā ṯēlēḵ wəʿāp̄ār tōʾḵal kol-yəmê ḥayyêḵā* — on-your-belly you-will-go and-dust you-will-eat all days-of your-life — "upon thy belly shalt thou go, and dust shalt thou eat all the days of thy life."

  Both the place and the object are fronted before their yiqtols: the curse is about *what* and *where*, not about going and eating.

- **Left dislocation / casus pendens.** A noun phrase stands outside the clause, at its left edge, and is taken up again inside it by a pronoun (GKC §143; Davidson 1901 §106; Driver 1892, Appendix V.1). It sets up or re-activates a topic.

> **GEN.3.12** · `wlc:GEN:67`
> הָאִשָּׁה אֲשֶׁר נָתַתָּה עִמָּדִי הִוא נָתְנָה־לִּי מִן־הָעֵץ וָאֹכֵל
> *hāʾiššâ ʾăšer nāṯattâ ʿimmāḏî hîʾ nāṯənâ-llî min-hāʿēṣ wāʾōḵēl* — the-woman whom you-gave with-me, she gave to-me from the-tree, and-I-ate — "The woman whom Thou gavest to be with me, she gave me of the tree, and I did eat."

  The man's speech starts with the woman and her relation to God's act — the blame shifts before the clause begins. The pronoun הִוא resumes her.

```
our tree:  6 אִשָּׁה → 2 ccomp ;  12 נָתְנָה → 6 conj ;  11 הִוא → 12 nsubj
UD/PTNK:   12 נָתְנָה → 2 ccomp ;  6 אִשָּׁה → 12 dislocated ;  11 הִוא → 12 nsubj
```

  Our trees have no `dislocated` relation: the dislocated phrase becomes the head of the speech and the real clause its `conj`. GEN.28.13 (section 6) gets the same treatment: הָאָרֶץ … is `conj` of the root הִנֵּה. The UD guidelines for Ancient Hebrew list `dislocated` and `nsubj:outer`; the macula README names casus pendens as one of the "special constructions" the general head rules do not yet know.

- **Extraposition / a heavy phrase on the right edge**, resumed earlier by a pronoun, is rarer (Miller-Naudé and Naudé 2019).

**The prosodic test.** Naudé (2018) shows that the Masoretic accents usually put a disjunctive accent after a dislocated phrase but join the subject to the pronoun in a tripartite verbless clause — an argument for reading the accents as data, which the trees keep in FORM (see the cantillation chapter).

**A fronted object or PP as the tree's root.** When a long fronted phrase opens a verse, our rules can make the phrase itself the root: in GEN.2.17 (`wlc:GEN:47`) the root of the sentence is the noun עֵץ of וּמֵעֵץ הַדַּעַת … לֹא תֹאכַל מִמֶּנּוּ "but of the tree of the knowledge … thou shalt not eat of it" — a fronted PP resumed by מִמֶּנּוּ. The verb תֹאכַל is a `conj` of it. A UD tree would make תֹאכַל the root and the fronted phrase an `obl` or `dislocated`.

## 16. Oaths

An oath is a conditional sentence whose apodosis — a self-curse — is left unsaid (GKC §149; Davidson 1901 §§119–120). So **אִם** "if" means "surely not", and **אִם־לֹא** "if not" means "surely". The formula is introduced by חַי־יְהוָה "as the LORD lives", בִּי נִשְׁבַּעְתִּי "by myself I swear", or a verb of swearing.

> **NUM.14.23** · `wlc:NUM:526`
> אִם־יִרְאוּ אֶת־הָאָרֶץ אֲשֶׁר נִשְׁבַּעְתִּי לַאֲבֹתָם
> *ʾim-yirʾû ʾeṯ-hāʾāreṣ ʾăšer nišbaʿtî laʾăḇōṯām* — if they-will-see (ACC) the-land which I-swore to-their-fathers — "surely they shall not see the land which I swore unto their fathers."

> **GEN.14.23** · `wlc:GEN:359`
> אִם־מִחוּט וְעַד שְׂרוֹךְ־נַעַל וְאִם־אֶקַּח מִכָּל־אֲשֶׁר־לָךְ
> *ʾim-miḥûṭ wəʿaḏ śərôḵ-naʿal wəʾim-ʾeqqaḥ mikkol-ʾăšer-lāḵ* — if from-a-thread and-to a-sandal-thong and-if I-take from-all that-to-you — "that I will not take a thread nor a shoe-latchet nor aught that is thine."

**In Mova.** The trees mark אִם as `mark` (SCONJ) like any condition; nothing yet tells the oath from the condition, and the `macula prag` polarity of NUM.14.23 is "none", though the verse means "not". The macula README lists oaths among the constructions with the generic reading. A rule could key on a preceding verb of swearing (נשבע), on חַי, or on an אִם clause with no apodosis; it is listed under "next" in section 19, not done.

## 17. Comparison

Hebrew has no comparative form of the adjective. "More than" is expressed by the preposition מִן "from" on the standard of comparison (GKC §133; Davidson 1901 §§33–34):

> **GEN.3.1** · `wlc:GEN:56`
> וְהַנָּחָשׁ הָיָה עָרוּם מִכֹּל חַיַּת הַשָּׂדֶה
> "Now the serpent was more subtle than any beast of the field."

> **GEN.29.30** · `wlc:GEN:825`
> וַיֶּאֱהַב גַּם־אֶת־רָחֵל מִלֵּאָה
> *wayyeʾĕhaḇ gam-ʾeṯ-rāḥēl millēʾâ* — and-he-loved also (ACC) Rachel from-Leah — "and he loved Rachel more than Leah."

In our trees the מִן phrase is `nmod` of the adjective (GEN.3.1) or `obl` of the verb (GEN.29.30); MACULA glosses the preposition "more" / "than", but nothing in the relation marks comparison. מִן can also mean "too … for" (GEN.4.13 גָּדוֹל עֲוֹנִי מִנְּשֹׂא "my punishment is greater than I can bear") and "away from, out of" — the dictionary (`torah-en.tsv`, key 4480) lists the senses; note the known gap that מִן is matched there to the neighbouring BDB entry מְנִי (macula README).

**The superlative** is expressed by the article ("the small one" = the youngest: בְּנוֹ הַקָּטָן, GEN.9.24), by a construct with a plural of the same word (קֹדֶשׁ קָדָשִׁים "most holy", EXO.29.37), or by a divine name as intensifier (GKC §133g–i).

## 18. Existence and absence: יֵשׁ and אַיִן

**יֵשׁ** "there is" (30 times in the Torah) and **אַיִן / אֵין** "there is not" (132 times) are particles, originally nouns ("existence", "nothingness"), that take suffixes for their subject: אֵינֶנּוּ "he is not", יֶשְׁךָ "you are" (GKC §100o, §141k, §152i). With לְ they express possession: יֵשׁ לִי "I have".

> **GEN.28.16** · `wlc:GEN:789`
> אָכֵן יֵשׁ יְהוָה בַּמָּקוֹם הַזֶּה וְאָנֹכִי לֹא יָדָעְתִּי
> *ʾāḵēn yēš YHWH bammāqôm hazze wəʾānōḵî lōʾ yāḏāʿtî* — surely there-is YHWH in-the-place the-this and-I not I-knew — "Surely the LORD is in this place; and I knew it not."

> **GEN.37.29** · `wlc:GEN:1112`
> וְהִנֵּה אֵין־יוֹסֵף בַּבּוֹר
> *wəhinnê ʾên-yôsēp̄ babbôr* — and-behold there-is-not Joseph in-the-pit — "and, behold, Joseph was not in the pit."

In the trees both are ADV heading their clause, with the existing (or absent) thing as `nsubj` — the predicate-as-head analysis of a verbless clause. MACULA types יֵשׁ as `demonstrative`(sic) and אֵין as `negative`. PTNK treats them similarly as predicates; some UD treebanks of Hebrew use `VERB` or `AUX` for יש, a known cross-treebank variation.

---

## 19. In Mova

**Where the syntax lives.** `tools/macula` reads MACULA Hebrew's lowfat trees (word groups with clause roles) and writes dependency trees in CoNLL-U: FORM is the pointed, accented text; LEMMA MACULA's pointed lemma; UPOS from MACULA's class; FEATS `Gender`, `Number`, `Person`, `HebBinyan`; MISC `Gloss`, `Ref`, `MId`, `Strong`, `Frame` (MACULA's argument links A0/A1…) and `Type` (the verb form or the word's subtype: wayyiqtol, qatal, infinitive_absolute, direct_object_marker, relative, interrogative, negative, …). Every tree is checked: one head per word, one root, no cycle; a negative control in the tests shows red.

**The head rules** (`head_child` and `relation` in `tools/macula/src/main.rs`): the verb heads a clause, else the predicate, else the first clause; the noun heads a PP (the preposition is `case`); the first content word heads a noun phrase; coordination is first-member-headed; MACULA's roles map `s`→`nsubj`, `o`/`o2`→`obj`, `io`→`iobj`, `pp`→`obl` (or `advcl` for an infinitive), `adv`→`advmod`/`obl`/`advcl`, `p`→`xcomp` beside a verb, else `nsubj`. UD conventions for Hebrew added on 2026-10-17: `compound:smixut`, SCONJ `mark` for the subordinators, `det` for demonstratives after a noun, `advcl` for an infinitive after a preposition, `flat` for compound numbers, `ccomp` for speech directly after a verb of saying (אמר, דבר, צוה, ענה, שאל, קרא), and a clause-initial ו on its clause's head.

**Against UD PTNK** (CC BY-NC; used only to measure; its words carry MACULA ids, so they align word for word): test UAS 88.38 / LAS 82.23, dev 86.61 / 79.98 (2026-10-17, macula README). The remaining differences, in the README's own list, map onto this chapter:

| difference (PTNK → ours, train) | count | where in this chapter |
|---|---:|---|
| `parataxis` → `conj` | 788 | §10, clause chains; §13, speech |
| `conj` → `nmod` | 385 | §4, coordinated construct members |
| `obl` → `advmod` | 278 | §7 second objects; §9 infinitive absolute |
| `det` → `amod` | 263 | demonstratives and adjectives with the article |
| `mark` → `det` | 250 | the article vs relative/subordinator readings |
| `advcl` → `conj` | 247 | §12 protasis/apodosis; §15 dislocation |
| הָיָה as copula | – | §2 |

**Constructions the rules do not yet know** (all read generically, as the README warns): copular pronouns and הָיָה (no `cop`); left dislocation and casus pendens (no `dislocated`, the phrase can become the root or a `conj`); the conditional's apodosis (coordinated with the protasis instead of heading it); oaths (no polarity reversal); the vocative (הַשָּׁמַיִם in DEU.32.1 is `conj`, not `vocative`); and the boundary between a frame verb and its quoted speech beyond the first clause. Two data points to keep in mind when reading counts: the construct state is not carried into FEATS; and one MACULA word can be duplicated (GEN.35.7, `wlc:GEN:1018`, has בֵּית־אֵל twice as two tokens with the same text — a split of the place name אֵל בֵּית־אֵל).

**What the rest of Mova reads from the syntax.**

- `macula prag` reads the root's verb form and the root's own clause (the root and its direct dependents) for the speech act, polarity and voice (09-discourse.md).
- The dictionary builder builds the Torah dictionary from the trees' lemmas and glosses (`torah-en.tsv`, 3 486 keys; examples in `.examples`, the two shortest one-verse sentences per word with Hebrew, MACULA's word-for-word gloss and JPS 1917).
- The `Frame` argument links survive pro-drop: of 27 701 argument links on 17 087 verbs, 7 241 point to a word outside the verb's own verse (2026-10-17) — an antecedent named earlier. That is the raw material for participant tracking (09-discourse.md).

**Next (not done).** A `cop` rule for the third-person pronoun between a subject and a definite predicate, checked against the accents (Naudé 2018); `dislocated` for a fronted noun phrase resumed by a pronoun in the next clause; the apodosis as head of a conditional with `advcl` for the protasis; an oath flag (polarity reversal) after נשבע / חַי; the `vocative` for a definite noun after an imperative; `State=Cons` from MACULA into FEATS. Each should be measured against PTNK train/dev before and after, with the test split read once per change, as the README requires.

## 20. Sources

- **GKC**: W. Gesenius, ed. E. Kautzsch, tr. A. E. Cowley, *Gesenius' Hebrew Grammar*, 2nd English ed., Oxford 1910 (public domain). §§106–167 for syntax: §113 infinitive absolute, §114–115 infinitive construct, §116 participle, §117 accusative and אֵת, §124g plural of majesty, §127–130 construct, §131 apposition, §133 comparison, §138 relative, §140–143 noun clause, verbal clause, compound sentence and casus pendens, §145 agreement, §149 oaths, §150 questions, §152 negation, §154 ו, §155 relative clauses, §156 circumstantial clauses, §157–166 subordinate clauses. Wikisource text: `$RESEARCH_RAW/hebgram/open/gesenius-kautzsch-cowley-1910-wikisource/gesenius-kautzsch-cowley-1910-wikisource.txt`; https://en.wikisource.org/wiki/Gesenius%27_Hebrew_Grammar
- A. B. Davidson, *Hebrew Syntax*, 3rd ed., Edinburgh 1901 (public domain): §9 relative pronoun, §§19–22 article, 23–28 construct, 29 apposition, 33–34 comparison, 66–78 government of the verb (accusative), 84–96 infinitives, 103–107 noun clause and casus pendens, 112–116 agreement, 119–120 oath, 121–128 questions and negatives, 129–132 conditionals, 136 conjunctive sentences, 137–141 circumstantial clauses, 142 relative clauses, 145 temporal, 146 object clauses, 147–151 causal, final, comparative clauses. OCR: `$RESEARCH_RAW/hebgram/pd/davidson-hebrew-syntax-1901/cu31924060183385/cu31924060183385_djvu.txt`; https://archive.org/details/cu31924060183385
- S. R. Driver, *A Treatise on the Use of the Tenses in Hebrew*, 3rd ed., Oxford 1892 (public domain): ch. VI wayyiqtol, ch. VIII weqatal, ch. XI hypotheticals, Appendix I circumstantial clause, Appendix V casus pendens, infinitive with lamed, order of words. `$RESEARCH_RAW/hebgram/pd/driver-tenses-1892/treatiseonuseof00driv/`; https://archive.org/details/treatiseonuseof00driv
- F. Brown, S. R. Driver, C. A. Briggs, *A Hebrew and English Lexicon of the Old Testament*, Oxford 1906 (public domain), s.v. ו, כִּי, אִם, מִן — via Sefaria `$RESEARCH_RAW/lexres/dictionaries/sefaria/BDB_Dictionary.jsonl` and in our `torah-en.tsv`.
- W. R. Harper, *Elements of Hebrew Syntax*, New York 1908 (public domain), `$RESEARCH_RAW/hebgram/pd/harper-elements-of-hebrew-syntax-1908/` — consulted for the order of topics.
- R. D. Holmstedt, "The Typological Classification of the Hebrew of Genesis: Subject-Verb or Verb-Subject?", *Journal of Hebrew Scriptures* 11 (2011), CC BY 2.5. `$RESEARCH_RAW/hbo-2011-holmstedt-typological-classification-hebrew-genesis-subject/`
- J. Joosten, "Some Dynamics of Word Order in Biblical Hebrew" (2025), CC BY 4.0. `$RESEARCH_RAW/hbo-2025-joosten-some-dynamics-word-order-biblical/`
- J. Joosten, "The participle as a component of the verbal system in Biblical Hebrew" (2020), CC BY 4.0. `$RESEARCH_RAW/hbo-2020-joosten-participle-component-verbal-system-biblical/`
- Kaajan, "Is Biblical Hebrew a non-configurational language? Reconsidering the evidence from discontinuous phrases", *HIPHIL Novum* (2019). `$RESEARCH_RAW/hbo-2019-kaajan-biblical-hebrew-non-configurational-language/`
- C. J. Collins, "The Wayyiqtol as 'Pluperfect': When and Why" (1995). `$RESEARCH_RAW/hbo-1995-collins-wayyiqtol-pluperfect-why/`
- J. A. Westbury, *Left Dislocation in Biblical Hebrew: A Cognitive Linguistic Account*, PhD thesis, Stellenbosch 2014, https://scholar.sun.ac.za/handle/10019.1/95852; and "A Functional Profile of Left Dislocation in Biblical Hebrew" (2016), CC BY 4.0. `$RESEARCH_RAW/hbo-2014-westbury-left-dislocation-biblical-hebrew-cognitive/`, `$RESEARCH_RAW/hbo-2016-westbury-functional-profile-left-dislocation-biblical/`
- C. L. Miller-Naudé and J. A. Naudé, "Differentiating dislocations, topicalisation, and extraposition in Biblical Hebrew: Evidence from negation" (2019), CC BY 4.0. `$RESEARCH_RAW/hbo-2019-miller-naude-differentiating-dislocations-topicalisation-extraposition-biblical/`
- J. A. Naudé (with C. L. Miller-Naudé), "At the interface of syntax and prosody: differentiating Left Dislocated and Tripartite Verbless Clauses in Biblical Hebrew" (2018), CC BY 4.0. `$RESEARCH_RAW/hbo-2018-naude-interface-syntax-prosody-differentiating-left/`
- C. H. J. van der Merwe, "Fronting and exhaustive exclusion in Biblical Hebrew" (2018), CC BY 4.0. `$RESEARCH_RAW/hbo-2018-merwe-fronting-exhaustive-exclusion-biblical-hebrew/`
- S. J. Floor, *From Information Structure, Topic and Focus, to Theme in Biblical Hebrew Narrative*, PhD thesis, Stellenbosch 2004, https://scholar.sun.ac.za/handle/10019.1/1270
- A. Andrason, "To resume or not to resume: some remarks on 'resumption' in left dislocation constructions in Polish, and its relevance for Biblical Hebrew" (2016), CC BY 4.0. `$RESEARCH_RAW/hbo-2016-andrason-resume-resume-some-remarks-resumption/`
- M. Naaijer, "Syntactic variation in Masoretic Hebrew: the object clause reconsidered" (2017), CC BY 4.0, and *Clause Structure Variation in Biblical Hebrew: A Quantitative Approach*, VU Amsterdam 2020. `$RESEARCH_RAW/hbo-2017-naaijer-syntactic-variation-masoretic-hebrew-object/`, `$RESEARCH_RAW/hbo-2020-naaijer-clause-structure-variation-biblical-hebrew/`
- E. Doron, "The Biblical Hebrew infinitive" (2020), https://zenodo.org/records/4749002 — `$RESEARCH_RAW/hbo-2020-doron-biblical-hebrew-infinitive/`
- M. Ehrensvärd, "Negating the infinitive in biblical Hebrew", *ZAH* (2022). `$RESEARCH_RAW/hbo-2022-ehrensvard-negating-infinitive-biblical-hebrew/`
- Bunis, "The Direct Object Marker /t-/ in Bar-Kosiba Hebrew" (2025), CC BY 4.0; Boulet, *A Linguistic Reappraisal of the Biblical Hebrew Accusative*, Toronto 2019, http://hdl.handle.net/1807/97347. `$RESEARCH_RAW/hbo-2025-bunis-direct-object-marker-t-bar/`, `$RESEARCH_RAW/hbo-2019-boulet-linguistic-reappraisal-biblical-hebrew-accusative/`
- Benigni, "The biblical Hebrew particle ky from a discourse analysis perspective", *ZAH* (2022). `$RESEARCH_RAW/hbo-2022-benigni-biblical-hebrew-particle-ky-discourse/`
- Staps, annotated כִּי clauses (2024), https://zenodo.org/records/14535294, CC BY 4.0. `$RESEARCH_RAW/hebgram/discourse/open/staps-ki/ki.csv`
- Zulker, *The Semitic \*wa and \*pa of Apodosis: Pragmatics, Syntax, and Diachrony of Two Non-Coordinating 'And's*, Chicago 2025. `$RESEARCH_RAW/hbo-2025-zulker-semitic-wa-pa-apodosis-pragmatics/`
- D. Swanson and F. Tyers, "A Universal Dependencies Treebank of Ancient Hebrew", LREC 2022, https://aclanthology.org/2022.lrec-1.252 ; UD Ancient Hebrew PTNK README, UD 2.18 (data CC BY-NC 4.0, used only to measure): `$RESEARCH_RAW/lexres/treebanks/ud-2.18/nc/UD_Ancient_Hebrew-PTNK/README.md`
- UD guidelines for Ancient Hebrew (hbo): tokenisation, features and relations (CC BY-SA; described, not copied). `$RESEARCH_RAW/hebgram/sa/ud-docs-he-hbo/`; https://universaldependencies.org/hbo/
- MACULA Hebrew Linguistic Datasets, Biblica / Clear Bible / the Groves Center / OpenScriptures, CC BY 4.0, https://github.com/Clear-Bible/macula-hebrew/ — the source of our trees, glosses, `Type` and `Frame`.
- Mova: `tools/macula/README.md`, `tools/macula/src/main.rs`; the trees `$MOVA_DATA/corpora/judaism/text/torah-wlc-macula/`; the dictionary `$MOVA_DATA/corpora/judaism/dict/torah-en.tsv`; JPS 1917 from eBible.org (public domain), `$MOVA_DATA/corpora/judaism/text/torah-jps1917/`.
- Wanted (not in our library): F. I. Andersen, *The Hebrew Verbless Clause in the Pentateuch* (1970); B. K. Waltke and M. O'Connor, *An Introduction to Biblical Hebrew Syntax* (1990); P. Joüon and T. Muraoka, *A Grammar of Biblical Hebrew* (2nd ed. 2006; the French Joüon 1923 is in `hebgram/pd/jouon-grammaire-1923`); C. H. J. van der Merwe, J. A. Naudé and J. H. Kroeze, *A Biblical Hebrew Reference Grammar* (2nd ed. 2017); W. Gross, *Die Satzteilfolge im Verbalsatz alttestamentlicher Prosa* (1996).
