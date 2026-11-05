# Discourse: narrative, speech and the shape of a text

Biblical Hebrew prose is built above the sentence as much as within it. A story moves forward on a chain of wayyiqtol verbs; it stops to give background in clauses that break the chain; it hands the floor to characters with fixed formulas of speech; it marks new scenes with וַיְהִי and changes the point of view with וְהִנֵּה. Law moves on a different chain, of yiqtol and weqatal. This chapter describes these devices on the Torah as it stands in Mova's trees (`$MOVA_DATA/corpora/judaism/text/torah-wlc-macula/*.conllu`, from MACULA Hebrew), counts them (2026-10-17), and then takes apart Mova's first reading of the pragmatics of each sentence, `macula prag`: what its rules are, what they get right, where they fail, and what is to be done.

The examples follow the format of the syntax chapter ([08-syntax.md](08-syntax.md)): verse and sentence id, pointed text, transliteration, gloss, JPS 1917. In tree excerpts each word is `id form → head relation [MACULA gloss] {Type}`.

## Contents

1. The mainline of narrative: the wayyiqtol chain
2. Breaking the chain: background and offline clauses
3. The other chain: weqatal in procedure, law and prediction
4. Direct speech and its frames
5. וַיְהִי: setting the scene
6. וְהִנֵּה: the point of view
7. וְעַתָּה: from the facts to the request
8. Paragraphs and section markers
9. Participant reference and tracking
10. Structure of the book: the toledot headings
11. Genre: narrative, list, instruction, poetry
12. Politeness and address
13. The rule-based pragmatics: `macula prag`
14. In Mova
15. Sources

---

## 1. The mainline of narrative: the wayyiqtol chain

The ordinary way to tell a past event in Hebrew is a clause headed by the "imperfect with waw consecutive", the wayyiqtol: וַיֹּאמֶר "and he said", וַיֵּלֶךְ "and he went". Driver (1892, ch. VI §§66–85) describes it as the form that carries a series of events, each new verb linked to the one before it; GKC (§111) calls it the tense of narration. Text-linguistic grammars since the 1970s call the wayyiqtol chain the *mainline* or *foreground* of narrative and everything else background (Longacre 1989, not in our library; Longacre 2006 applies the scheme to Psalm 18). Andrason (2011) gives a dynamic, diachronic account of the form, and Lenzi (2015) describes the relations of taxis — sequence, simultaneity, anteriority — that the chains express.

**A chain in the trees.** Each wayyiqtol clause after the first is a `conj` of the first, so a verse of narrative is a flat fan:

> **GEN.22.3** · `wlc:GEN:550`
> וַיַּשְׁכֵּם אַבְרָהָם בַּבֹּקֶר וַיַּחֲבֹשׁ אֶת־חֲמֹרוֹ וַיִּקַּח אֶת־שְׁנֵי נְעָרָיו אִתּוֹ וְאֵת יִצְחָק בְּנוֹ וַיְבַקַּע עֲצֵי עֹלָה וַיָּקָם וַיֵּלֶךְ אֶל־הַמָּקוֹם אֲשֶׁר־אָמַר־לוֹ הָאֱלֹהִים
> *wayyaškēm ʾaḇrāhām babbōqer wayyaḥăḇōš ʾeṯ-ḥămōrô wayyiqqaḥ ʾeṯ-šənê nəʿārāw ʾittô wəʾēṯ yiṣḥāq bənô wayḇaqqaʿ ʿăṣê ʿōlâ wayyāqom wayyēleḵ ʾel-hammāqôm ʾăšer-ʾāmar-lô hāʾĕlōhîm* — "And Abraham rose early in the morning, and saddled his ass, and took two of his young men with him, and Isaac his son; and he cleaved the wood for the burnt-offering, and rose up, and went unto the place of which God had told him."

Six wayyiqtols in a row (rose early, saddled, took, split, arose, went), then a qatal in a relative clause (אָמַר "had told") that looks back — the one verb off the chain is the one that is not a new event.

**Measured.** In the sequence of all finite verbs and participles of a book, a *run* is a maximal stretch of consecutive wayyiqtols (2026-10-17):

| book | wayyiqtol | runs | mean run | longest run |
|---|---:|---:|---:|---|
| GEN | 2 105 | 830 | 2.54 | 37 (GEN.5.5–5.24) |
| EXO | 888 | 434 | 2.05 | 13 |
| LEV | 189 | 76 | 2.49 | 17 (LEV.9.13–9.20) |
| NUM | 750 | 336 | 2.23 | 49 (NUM.33.15–33.40) |
| DEU | 255 | 141 | 1.81 | 10 |

The longest runs are not the most dramatic stories but the most list-like texts: the genealogy of Genesis 5, where each generation is "and he lived … and he begat … and he died", and the itinerary of Numbers 33, "and they journeyed from … and they pitched in …". A mean run of two to three verbs shows how often the chain is broken by speech, by background and by relative clauses.

> **GEN.5.5** · `wlc:GEN:110`
> וַיִּהְיוּ כָּל־יְמֵי אָדָם אֲשֶׁר־חַי תְּשַׁע מֵאוֹת שָׁנָה וּשְׁלֹשִׁים שָׁנָה וַיָּמֹת
> *wayyihyû kol-yəmê ʾāḏām ʾăšer-ḥay təšaʿ mēʾôṯ šānâ ûšəlōšîm šānâ wayyāmōṯ* — "And all the days that Adam lived were nine hundred and thirty years; and he died."

**The sentence root by book.** Which form heads the sentence is a quick fingerprint of a book's discourse (counts of sentence roots, 2026-10-17):

| book | sentences | top root forms |
|---|---:|---|
| GEN | 1 533 | wayyiqtol 973, qatal 154, yiqtol 72, noun 70, proper name 48, weqatal 32, imperative 31 |
| EXO | 1 213 | wayyiqtol 477, yiqtol 220, weqatal 194, noun 103, qatal 85 |
| LEV | 859 | yiqtol 299, weqatal 213, noun 145, wayyiqtol 96, qatal 28 |
| NUM | 1 289 | wayyiqtol 415, noun 260, yiqtol 162, weqatal 124, qatal 78 |
| DEU | 959 | yiqtol 309, weqatal 185, wayyiqtol 113, qatal 91, noun 84, imperative 44 |

Genesis is narrative; Leviticus is law; Numbers mixes narrative with lists (the many noun roots are the census and the camp lists); Deuteronomy is Moses' speech, with law and exhortation.

## 2. Breaking the chain: background and offline clauses

Anything that is not a wayyiqtol breaks the chain, and the break tells the reader something (Driver 1892, Appendix I; GKC §156; Davidson 1901 §§137–141). The main offline types:

- **X-qatal**: a subject or other constituent before a qatal. It gives a fact that is not the next event — an earlier event (pluperfect), a contrast, a new topic, or a summary (Collins 1995; Schiller 2020 discusses the term in poetry).

> **GEN.3.1** · `wlc:GEN:56`
> וְהַנָּחָשׁ הָיָה עָרוּם מִכֹּל חַיַּת הַשָּׂדֶה אֲשֶׁר עָשָׂה יְהוָה אֱלֹהִים
> "Now the serpent was more subtle than any beast of the field which the LORD God had made."

  The chapter opens with the snake as a new topic, described before it acts; the first wayyiqtol (וַיֹּאמֶר) comes next.

- **The participle, often with a subject pronoun**: an action going on at the time of the mainline event.

> **GEN.18.1** · `wlc:GEN:425`
> וַיֵּרָא אֵלָיו יְהוָה בְּאֵלֹנֵי מַמְרֵא וְהוּא יֹשֵׁב פֶּתַח־הָאֹהֶל כְּחֹם הַיּוֹם
> *wayyērāʾ ʾēlāw YHWH bəʾēlōnê mamrēʾ wəhûʾ yōšēḇ petaḥ-hāʾōhel kəḥōm hayyôm* — "And the LORD appeared unto him by the terebinths of Mamre, as he sat in the tent door in the heat of the day."

- **The verbless clause**: a state, a description, a name (GEN.2.12 "the gold of that land is good").
- **Negated and yiqtol clauses of a past state**: GEN.2.5 (`wlc:GEN:35`) "No shrub of the field was yet in the earth (טֶרֶם יִהְיֶה) … and there was not a man (וְאָדָם אַיִן) to till the ground" — a whole verse of background before the creation of man.

Background clauses cluster at the beginning of episodes (setting), at their peaks (a pause before the turn) and at their ends (a summary or a comment). Yoo (2008) uses such devices, with וַיְהִי and changes of participant, to find paragraph boundaries in narrative.

**In the trees.** An offline clause joined by ו is, like a wayyiqtol, a `conj` of the first clause: in GEN.18.1 `יֹשֵׁב → conj` of `יֵּרָא`, with `הוּא → nsubj`. The background status is therefore not in the relation but in the verb form (`Type`) and the order (`nsubj` before the verb). The count of section 1 of the syntax chapter (subject before qatal 400 times, before the participle 438 times) is the inventory of these clauses.

## 3. The other chain: weqatal in procedure, law and prediction

The "perfect with waw consecutive" (weqatal) continues a yiqtol, an imperative or a participle that looks forward, and carries a series of future, habitual or prescribed actions (Driver 1892, ch. VIII §§105–129; GKC §112). It is to instruction what wayyiqtol is to narrative. In Leviticus weqatal is the second most common sentence root (213) after yiqtol (299).

> **LEV.1.4** · `wlc:LEV:3`
> וְסָמַךְ יָדוֹ עַל רֹאשׁ הָעֹלָה וְנִרְצָה לוֹ לְכַפֵּר עָלָיו
> *wəsāmaḵ yāḏô ʿal rōʾš hāʿōlâ wənirṣâ lô ləḵappēr ʿālāw* — and-he-shall-lay his-hand on head-of the-burnt-offering and-it-shall-be-accepted for-him to-atone for-him — "And he shall lay his hand upon the head of the burnt-offering; and it shall be accepted for him to make atonement for him."

The ritual of Leviticus 1 is a weqatal chain opened by a כִּי / אִם clause with yiqtol (LEV.1.2–3) and continued step by step: lay the hand, slaughter, sprinkle, flay, cut up, burn.

**From command to procedure.** A command can start with an imperative and continue with yiqtol and weqatal:

> **GEN.6.14** · `wlc:GEN:151`
> עֲשֵׂה לְךָ תֵּבַת עֲצֵי־גֹפֶר קִנִּים תַּעֲשֶׂה אֶת־הַתֵּבָה וְכָפַרְתָּ אֹתָהּ מִבַּיִת וּמִחוּץ בַּכֹּפֶר
> *ʿăśē ləḵā ṯēḇaṯ ʿăṣê-ḡōp̄er qinnîm taʿăśe ʾeṯ-hattēḇâ wəḵāp̄artā ʾōṯāh mibbayiṯ ûmiḥûṣ bakkōp̄er* — "Make thee an ark of gopher wood; with rooms shalt thou make the ark, and shalt pitch it within and without with pitch."

Imperative (עֲשֵׂה), then fronted object + yiqtol (קִנִּים תַּעֲשֶׂה, a detail), then weqatal (וְכָפַרְתָּ, the next step). In our trees all three are in one fan under the imperative root (`conj`).

**weqatal never has anything before it** in its clause (0 preverbal subjects or objects in the Torah, syntax chapter §1), just like wayyiqtol. A preverbal element with a future verb therefore produces X-yiqtol, the offline counterpart in instruction (a condition, a contrast, a detail).

## 4. Direct speech and its frames

Speech is everywhere in the Torah. In the MACULA speaker-quotation data (Clear Bible 2023, CC BY 4.0, `hebgram/discourse/open/clear-speaker-quotations`) the five books hold 972 quotations — Genesis 417, Exodus 190, Numbers 177, Deuteronomy 144, Leviticus 44 — covering about 82 000 word positions (nested quotes counted once per layer), roughly two thirds of the Torah's 125 807 words. The most frequent speakers are God (249 quotations) and Moses (232), then Jacob (61), Joseph (37), Judah (31) and Abraham (30). Fifty-six quotations are embedded two or three levels deep (a quote inside a quote).

**The frame.** A quotation is introduced by a *quotative frame* (Miller-Naudé and Naudé 2026; Miller 1996, not in our library):

- **וַיֹּאמֶר** "and he said" + the speech, with no conjunction: the commonest frame. 807 wayyiqtol forms of אמר in the Torah (2026-10-17).
- **לֵאמֹר** "saying" (the infinitive construct of אמר with לְ), after another verb of speaking or commanding: 306 times. It works as a quotation mark rather than as a separate act of saying (BDB s.v. אָמַר; GKC §114o).
- **וַיְדַבֵּר … לֵאמֹר** "and he spoke … saying": 150 wayyiqtol forms of דבר, 110 of them in a sentence with לֵאמֹר — mostly the formula that opens a divine speech in Exodus–Numbers:

> **EXO.14.1** · `wlc:EXO:357`
> וַיְדַבֵּר יְהוָה אֶל־מֹשֶׁה לֵּאמֹר
> *waydabbēr YHWH ʾel-mōše lēʾmōr* — "And the LORD spoke unto Moses, saying:"

  Here the verse ends with לֵּאמֹר and the speech is the next verse — a separate sentence in our trees.

- **וַיַּעַן … וַיֹּאמֶר** "and he answered and said": a frame of reply.

Patty (2026) reads such exchanges with the tools of conversation analysis (turns, adjacency pairs, repair); Savran (2009) studies speech attributed to a group ("and all the people said") as a literary device. Dialogue moves fast: GEN.22.1 has a summons and its answer in one verse.

> **GEN.22.1** · `wlc:GEN:548`
> וַיְהִי אַחַר הַדְּבָרִים הָאֵלֶּה וְהָאֱלֹהִים נִסָּה אֶת־אַבְרָהָם וַיֹּאמֶר אֵלָיו אַבְרָהָם וַיֹּאמֶר הִנֵּנִי
> *wayəhî ʾaḥar haddəḇārîm hāʾēlle wəhāʾĕlōhîm nissâ ʾeṯ-ʾaḇrāhām wayyōʾmer ʾēlāw ʾaḇrāhām wayyōʾmer hinnēnî* — "And it came to pass after these things, that God did prove Abraham, and said unto him: 'Abraham'; and he said: 'Here am I.'"

The verse has every device of this chapter: וַיְהִי + a temporal phrase (section 5); an X-qatal that states the theme of the episode (וְהָאֱלֹהִים נִסָּה "God tested", background, summarising in advance); then two speech turns, the second with no named speaker — the change of speaker is left to the reader.

**Speech in the trees.** Our rules make the first clause after a verb of saying (אמר, דבר, צוה, ענה, שאל, קרא) its `ccomp` when no conjunction stands between them (`SAYING` in `tools/macula/src/main.rs`). Of the 513 sentences whose root is a wayyiqtol of אמר, 480 have a `ccomp` under the root (2026-10-17). But only the *first* clause of the speech is a `ccomp`: later clauses of the same speech become `conj` of the frame verb, as if they were narrated.

> **DEU.1.42** · `wlc:DEU:41`
> וַיֹּאמֶר יְהוָה אֵלַי אֱמֹר לָהֶם לֹא תַעֲלוּ וְלֹא־תִלָּחֲמוּ כִּי אֵינֶנִּי בְּקִרְבְּכֶם וְלֹא תִּנָּגְפוּ לִפְנֵי אֹיְבֵיכֶם
> "And the LORD said unto me: 'Say unto them: Go not up, neither fight; for I am not among you; lest ye be smitten before your enemies.'"

```
2  יֹּאמֶר   → 0  root   {wayyiqtol}     narrator: "the LORD said to me"
6  אֱמֹר     → 2  ccomp  {imperative}    speech 1 (God to Moses): "say to them"
10 תַעֲלוּ   → 6  ccomp  {yiqtol}        speech 2 (Moses to Israel): "do not go up"
13 תִלָּחֲמוּ → 2  conj   {yiqtol}        still speech 2 — but attached to the frame
15 אֵינ      → 2  conj   (אַיִן)          still speech 2, God's "I am not among you"
22 תִּנָּגְפוּ → 2  conj   {yiqtol}        still speech 2
```

Two levels of quotation are nested here (God tells Moses to tell the people), and our tree gets the first two levels right and then flattens the rest of the speech onto the narrator's verb. The README of `tools/macula` counts `parataxis` → `conj` (788 in PTNK train) as the largest difference from PTNK, and much of it is exactly this. The consequence for pragmatics is shown in section 13.

## 5. וַיְהִי: setting the scene

וַיְהִי "and it was / and it came to pass" is the wayyiqtol of הָיָה used not as a verb of existence but as a marker: it opens a new episode or a new step by placing it in time, and the main event follows, usually as a wayyiqtol (Driver 1892 §78; GKC §111f–g). Li (2006) describes it as a discourse marker of episode boundaries in Kings; Westbury (2018) analyses the "wayhî + temporal expression" construction and its subtypes.

**Measured** (2026-10-17): 246 wayyiqtol forms of הָיָה in the Torah, 170 of them heading their sentence. Followed directly by:

| what follows וַיְהִי | count |
|---|---:|
| בְּ / כְּ + infinitive construct ("when …") | 25 |
| כַּאֲשֶׁר ("when, as") | 12 |
| אַחַר / אַחֲרֵי ("after") | 7 |
| בְּיוֹם ("on the day") | 3 |

The rest are mostly the ordinary verb "and there was / and it became" (וַיְהִי־אוֹר "and there was light", GEN.1.3; וַיְהִי־כֵן "and it was so") or a time phrase with another preposition (וַיְהִי בַבֹּקֶר "and it was in the morning").

> **GEN.4.8** · `wlc:GEN:87`
> וַיְהִי בִּהְיוֹתָם בַּשָּׂדֶה וַיָּקָם קַיִן אֶל־הֶבֶל אָחִיו וַיַּהַרְגֵהוּ
> "And it came to pass, when they were in the field, that Cain rose up against Abel his brother, and slew him."

> **GEN.29.25** · `wlc:GEN:820`
> וַיְהִי בַבֹּקֶר וְהִנֵּה־הִוא לֵאָה וַיֹּאמֶר אֶל־לָבָן מַה־זֹּאת עָשִׂיתָ לִּי
> *wayəhî ḇabbōqer wəhinnê-hîʾ lēʾâ wayyōʾmer ʾel-lāḇān ma-zzōʾṯ ʿāśîṯā llî* — "And it came to pass in the morning that, behold, it was Leah; and he said to Laban: 'What is this thou hast done unto me?'"

**In the trees** the וַיְהִי is the root and the main event a `conj` of it, with the time phrase as `obl`/`advcl` of וַיְהִי. That is faithful to the surface, but the semantics is the reverse: the event is the main predicate and the time phrase its adjunct. A rule could re-head these sentences (the main event as root, the time phrase as its `advcl`/`obl`, וַיְהִי as a marker, perhaps `discourse` or `aux`), which would also give `macula prag` the real verb of the sentence. Not done.

## 6. וְהִנֵּה: the point of view

הִנֵּה "behold, look" (256 times; 126 of them as וְהִנֵּה, 2026-10-17) points at something. In speech it presents a fact to the hearer ("here is …") or takes a pronoun suffix for "here I am" (הִנֵּנִי, GEN.22.1). In narrative, וְהִנֵּה after a verb of seeing — or a dream, or a waking up — shifts to the character's point of view: the reader sees what the character sees, often with surprise (GKC §105b, §147b).

> **GEN.37.29** · `wlc:GEN:1112`
> וַיָּשָׁב רְאוּבֵן אֶל־הַבּוֹר וְהִנֵּה אֵין־יוֹסֵף בַּבּוֹר וַיִּקְרַע אֶת־בְּגָדָיו
> "And Reuben returned unto the pit; and, behold, Joseph was not in the pit; and he rent his clothes."

Reuben's discovery is told through his eyes, as a verbless clause; his reaction is the next wayyiqtol. In GEN.29.25 above the same device gives the shock of the morning after.

**In the trees.** הִנֵּה is INTJ (230 times) or ADV/PRON, MACULA `Type=demonstrative`. Our rules attach the clause after it as a `conj` of הִנֵּה, and הִנֵּה itself becomes `conj` of the preceding wayyiqtol (GEN.37.29), or even the root of a sentence (GEN.28.13, `wlc:GEN:786`, "וְהִנֵּה יְהוָה נִצָּב עָלָיו", "and, behold, the LORD stood beside him"). A UD tree would make הִנֵּה a `discourse` dependent of the clause it introduces; the relation `discourse` occurs only 5 times in our trees. Not yet fixed.

## 7. וְעַתָּה: from the facts to the request

וְעַתָּה "and now" (54 of the 84 עַתָּה, 2026-10-17) rarely means "at this moment". In speeches and letters it marks the turn from the grounds — what has happened, what is the case — to the consequence: a request, a command, a decision (BDB s.v. עַתָּה; Warren-Rothlin 2007 treats it among politeness and persuasion strategies).

> **GEN.27.8** · `wlc:GEN:735`
> וְעַתָּה בְנִי שְׁמַע בְּקֹלִי לַאֲשֶׁר אֲנִי מְצַוָּה אֹתָךְ
> *wəʿattâ ḇənî šəmaʿ bəqōlî laʾăšer ʾănî məṣawwâ ʾōṯāḵ* — and-now my-son listen to-my-voice to-what I command you — "Now therefore, my son, hearken to my voice according to that which I command thee."

Rebekah has told Jacob what she overheard (GEN.27.6–7); וְעַתָּה turns the report into a plan. Also GEN.3.22 וְעַתָּה פֶּן־יִשְׁלַח יָדוֹ "and now, lest he put forth his hand", and GEN.45.5 in Joseph's speech to his brothers.

**In the trees** עַתָּה is ADV and here the root of the sentence; the vocative בְנִי and the imperative שְׁמַע hang on it as `conj`. The imperative, the real speech act, is below the root, which matters for `macula prag` (section 13).

## 8. Paragraphs and section markers

The Masoretic text divides the Torah into open (*petuchah*, marked פ) and closed (*setumah*, ס) sections — paragraphing older than chapters and verses (Ginsburg 1897 describes the system; `hebgram/pd/ginsburg-introduction-masoretic-1897`). The sections often but not always match the episodes found by linguistic signals (וַיְהִי, a change of participants or place, a break in the wayyiqtol chain; Yoo 2008).

**In our data the markers are absent.** MACULA's lowfat trees do not carry them, and the `# text` lines of our trees contain no פ or ס section signs (0 of 5 853 sentences, 2026-10-17). The weekly portions (*parashot*) are known to the Torah course of the school through a separate table (`macula lessons` takes a `parashot.tsv`), but the petuchah/setumah division is not yet joined to the trees. The Westminster Leningrad Codex as published by OpenScriptures marks them, so they can be added by verse. Not done.

## 9. Participant reference and tracking

A story must keep its participants apart. Hebrew narrative does this with a small scale of forms: a full noun phrase (often a name with an epithet: "Abraham", "Abel his brother"), an independent pronoun, a pronoun suffix, and zero — nothing but the person, gender and number of the verb (Runge 2007; Levinsohn 2018; Winther-Nielsen 2016). The rule of thumb: zero or a pronoun for the participant who stays the subject, a full noun when the subject changes, when a participant is re-introduced, or at a peak or the start of a new unit. A full noun for a participant who is already active is "marked" and signals a boundary or salience (Levinsohn 2018; Runge 2007).

> **GEN.4.8** (again)
> וַיָּקָם קַיִן אֶל־הֶבֶל אָחִיו וַיַּהַרְגֵהוּ
> "that **Cain** rose up against **Abel his brother**, and slew **him**."

Cain is renamed although he is the speaker of the clause before (4.8a), and Abel gets the full "Abel his brother" — at the peak of the story, kinship is named exactly where it is violated. Then the killing has a zero subject and a suffix object.

**The `Frame` links in MISC.** MACULA gives most verbs a `Frame=A0:…;A1:…;` field: the ids of the words that fill its arguments (A0 roughly the agent, A1 the patient or theme), even when they are not in the clause. In GEN.22.3 (section 1) every one of the six verbs has `A0:010220030021` — אַבְרָהָם, named once at the start. Over the Torah (2026-10-17) 17 087 verbs carry a Frame, with 27 701 argument links; 7 241 of the links (26 %) point to a word outside the verb's own verse — an antecedent earlier in the text. These links are a ready-made participant chain: every zero subject is resolved to a word.

**Speaker data.** The quotation data of section 4 name the speaker of each quotation (God, Moses, Jacob …) and the words that refer to them (`ClearSubjectReferents`, MACULA word ids), and the ETCBC participant work (Erwich, in `hebgram/discourse/sa/erwich-participant-analysis`, CC BY-SA, for study only) gives another layer.

**In Mova** the trees carry `Frame` in MISC untouched; nothing reads it yet. The obvious first use is coreference: resolve each zero `nsubj` to its `A0` word, count the forms of reference per participant, and test the rule of thumb above. Not done.

## 10. Structure of the book: the toledot headings

Genesis is divided by the formula אֵלֶּה תּוֹלְדֹת X "these are the generations (descendants, story) of X". The noun תּוֹלֵדוֹת occurs in the Torah's trees in GEN.2.4, 5.1 (זֶה סֵפֶר תּוֹלְדֹת אָדָם "this is the book of the generations of Adam"), 6.9, 10.1, 10.32, 11.10, 11.27, 25.12, 25.13, 25.19, 36.1, 36.9, 37.2, then in EXO.6.16, 6.19, 28.10 and the census of NUM.1 and NUM.3.1. As headings, the eleven of GEN.2.4, 5.1, 6.9, 10.1, 11.10, 11.27, 25.12, 25.19, 36.1, 36.9 and 37.2 open the book's sections; the others are inside lists.

> **GEN.2.4** · `wlc:GEN:34`
> אֵלֶּה תוֹלְדוֹת הַשָּׁמַיִם וְהָאָרֶץ בְּהִבָּרְאָם בְּיוֹם עֲשׂוֹת יְהוָה אֱלֹהִים אֶרֶץ וְשָׁמָיִם
> "These are the generations of the heaven and of the earth when they were created, in the day that the LORD God made earth and heaven."

The heading is a verbless clause (`תוֹלְדוֹת` root, `אֵלֶּה` nsubj). Whether it closes the first creation account or opens the second is an old question; the formula elsewhere always opens. In Mova a section map of Genesis could be made from the root lemma of the sentence alone. Not done.

## 11. Genre: narrative, list, instruction, poetry

The ETCBC "Syntactic Variation" project labels every verse of the Hebrew Bible as prose, list, instruction or poetry (`hebgram/discourse/open/etcbc-genre-synvar/genre_ranges.csv`, MIT licence; Eep Talstra Centre 2020). In the Torah: poetry is Jacob's blessing (GEN.49.2–27), the Song of the Sea (EXO.15.2–18), the priestly blessing (NUM.6.24–26), the Balaam oracles (NUM.23–24) and other short pieces in NUM.21, and the Song and Blessing of Moses (DEU.32.1–43, 33.3–29); lists are the genealogies (GEN.5, 10, 11, 25, 36, 46), censuses and camp orders (NUM.1–4), the itinerary (NUM.33); instruction covers most of Leviticus, the tabernacle (EXO.25–31, 35–40), the Covenant Code (EXO.20–23) and DEU.12–25.

Joining these labels to `macula prag` (section 13), verse by verse (2026-10-17):

| genre | sentences | act = narrate | act = assert | act = request | act = ask | voice = character | polarity = neg |
|---|---:|---:|---:|---:|---:|---:|---:|
| prose | 3 117 | 1 667 | 1 082 | 330 | 38 | 809 | 162 |
| instruction | 2 128 | 197 | 1 356 | 571 | 4 | 824 | 249 |
| list | 470 | 137 | 321 | 12 | 0 | 23 | 2 |
| poetry | 138 | 9 | 110 | 14 | 5 | 48 | 14 |

The profile is as expected — narration in prose, requests in law, assertions in lists — and it also shows the weakness of the rules: two thirds of the law's sentences are "assert", because a law in the third person ("he shall lay his hand", LEV.1.4) is not recognised as a command (section 13).

**Poetry** differs from prose in almost every respect that the rules use: few wayyiqtols, few articles and object markers, parallel lines, the vocative, rhetorical questions in the first person plural or with no clear person. The Song of Moses opens:

> **DEU.32.1** · `wlc:DEU:866`
> הַאֲזִינוּ הַשָּׁמַיִם וַאֲדַבֵּרָה וְתִשְׁמַע הָאָרֶץ אִמְרֵי־פִי
> *haʾăzînû haššāmayim waʾăḏabbērâ wəṯišmaʿ hāʾāreṣ ʾimrê-p̄î* — give-ear O-heavens and-let-me-speak, and-let-hear the-earth the-words-of my-mouth — "Give ear, ye heavens, and I will speak; and let the earth hear the words of my mouth."

Imperative, cohortative, jussive in one line; the heavens and the earth as addressees. Our tree makes הַשָּׁמַיִם a `conj` of the imperative instead of a `vocative`. `macula prag` gets the sentence right (imp, request, character) because its root is the imperative. A dozen lines later, in DEU.32.30 (`wlc:DEU:895`), "How should one chase a thousand …?", the rules give "q, ask" with voice "narrator", because the speaker of the song is nowhere marked in the sentence.

## 12. Politeness and address

Speakers in the Torah show deference by a fixed set of devices (Warren-Rothlin 2007; Kim 2023, *Hebrew Forms of Address*, not yet read for this chapter; Hare 2018 on gendered speech):

- **נָא** after an imperative, a jussive or a cohortative, or after הִנֵּה / אִם — traditionally "please, I pray", sometimes a mere softener (110 times as INTJ, 2026-10-17; MACULA types 95 of them `exhortation`).
- **Third-person self-reference**: עַבְדְּךָ "your servant" for "I" (62 forms of עֶבֶד with a second-person suffix), and **אֲדֹנִי** "my lord" for "you" (62 forms of אָדוֹן with a first-person suffix).
- **The conditional of favour**: אִם־נָא מָצָאתִי חֵן בְּעֵינֶיךָ "if now I have found favour in your eyes".
- **בִּי אֲדֹנִי** "(on) me, my lord!", an opening of a plea.

> **GEN.18.3** · `wlc:GEN:427`
> אֲדֹנָי אִם־נָא מָצָאתִי חֵן בְּעֵינֶיךָ אַל־נָא תַעֲבֹר מֵעַל עַבְדֶּךָ
> *ʾăḏōnāy ʾim-nāʾ māṣāʾṯî ḥēn bəʿênêḵā ʾal-nāʾ ṯaʿăḇōr mēʿal ʿaḇdeḵā* — my-lord, if-please I-found favour in-your-eyes, not-please pass from-upon your-servant — "My lord, if now I have found favour in thy sight, pass not away, I pray thee, from thy servant."

> **GEN.44.18** · `wlc:GEN:1342`
> בִּי אֲדֹנִי יְדַבֶּר־נָא עַבְדְּךָ דָבָר בְּאָזְנֵי אֲדֹנִי וְאַל־יִחַר אַפְּךָ בְּעַבְדֶּךָ
> *bî ʾăḏōnî yəḏabber-nāʾ ʿaḇdəḵā ḏāḇār bəʾozənê ʾăḏōnî wəʾal-yiḥar ʾappəḵā bəʿaḇdeḵā* — "Oh my lord, let thy servant, I pray thee, speak a word in my lord's ears, and let not thine anger burn against thy servant."

Judah's plea before Joseph uses every device at once: בִּי, the title twice, the third-person jussive "let your servant speak" instead of "let me speak", נָא, and "your servant" again for himself.

**In `macula prag`** both verses come out as "decl, narrate, narrator" with hedge 0: the root is the narrator's וַיֹּאמֶר, and the rules look only at the root's own clause. The 110 נָא of the Torah yield only 17 sentences with hedge = 1 (section 13).

## 13. The rule-based pragmatics: `macula prag`

`macula prag <trees-dir> <out.tsv>` gives every Hebrew sentence a pragmatic reading in the same columns that Mova's English pragmatics uses, so that the two can be compared verse by verse (`macula prag-compare`). The output for the Torah is `$MOVA_DATA/corpora/judaism/hbo-prag.tsv` (columns `sent_id form act indirect hedge polarity voice teacher`, teacher always `rules`).

### 13.1 The rules, exactly

The function `hebrew_prag` in `tools/macula/src/main.rs` reads one sentence:

1. It finds the **root** and its verb form `rk` (MISC `Type`), and the root's **own clause**: the root and the words whose head is the root (not deeper).
2. **Frame**: the root is a wayyiqtol or a qatal, and the root is not in the first or second person. A framed sentence is the narrator's, whatever it quotes: *form* `decl`, *act* `narrate` for a wayyiqtol and `assert` for a qatal, *voice* `narrator`.
3. **Volitive**: the root is an imperative, jussive or cohortative → *form* `imp`, *act* `request`.
4. **Command of the law**: the root is a yiqtol or weqatal in the second person → *act* `request`, *indirect* `1` (a statement used as a command: "thou shalt …").
5. **Questions**: a word of the own clause has `Type=interrogative` and its lemma is one of מִי, מָה, מַדּוּעַ, אֵיךְ, מָתַי, לָמָּה, אַיֵּה → *form* `wh`; another interrogative (in practice הֲ) → *form* `q`. Either gives *act* `ask`.
6. **Hedge**: נָא in the own clause → *hedge* `1`.
7. **Voice**: `character` if the root is in the first or second person, or is a volitive, or the own clause has נָא or a word in the first or second person; else `narrator`.
8. **Polarity**: `neg` if the own clause has לֹא, אַל or אַיִן.

The precedence is: frame first (it overrides everything), then volitive, then wh, then q, else `decl`; for the act, frame, then volitive or command, then question, else `assert`.

### 13.2 The distributions

On the Torah (5 853 sentences, 2026-10-17):

| column | values |
|---|---|
| form | decl 5 591, imp 211, wh 34, q 17 |
| act | assert 2 869, narrate 2 010, request 927, ask 47 |
| indirect | 1: 716; 0: 5 137 |
| hedge | 1: 17 |
| polarity | neg 427, none 5 426 |
| voice | narrator 4 149, character 1 704 |

### 13.3 Worked examples, one per category

| sentence | verse | reading | right? |
|---|---|---|---|
| `wlc:GEN:2` | GEN.1.3 "And God said: Let there be light. And there was light." | decl, narrate, narrator | yes for the frame; the jussive יְהִי inside is invisible |
| `wlc:GEN:151` | GEN.6.14 "Make thee an ark …" | imp, request, character | yes |
| `wlc:EXO:531` | EXO.20.13 "Thou shalt not murder." | decl, request, indirect 1, neg, character | yes: a command by a yiqtol, negated |
| `wlc:DEU:6` | DEU.1.7 "turn you, and take your journey …" | imp, request, character | yes |
| `wlc:GEN:465` | GEN.19.8 "Behold now, I have two daughters …" | decl, assert, hedge 1, character | partly: the root is הִנֵּה, so the cohortative "let me bring them out" and the imperative are missed |
| `wlc:DEU:11` | DEU.1.12 "How can I myself alone bear your cumbrance …?" | q, ask, character | the act is right; the form should be wh — אֵיכָה is not in the list of wh-words |
| `wlc:GEN:86` | GEN.4.7 "If thou doest well, shall it not be lifted up? …" | q, request, indirect 1, neg, character | mixed: הֲלוֹא is a rhetorical question; the second-person yiqtol root makes it a "command" |
| `wlc:DEU:895` | DEU.32.30 "How should one chase a thousand …?" | q, ask, narrator | the voice is wrong: the speaker of the Song is not marked in the sentence |
| `wlc:GEN:64` | GEN.3.9 "And … said unto him: Where art thou?" | decl, narrate, narrator | the frame is right; the question is lost |
| `wlc:GEN:427` | GEN.18.3 "My lord, if now I have found favour …, pass not away, I pray thee" | decl, narrate, narrator | the frame is right; the polite request is lost |
| `wlc:GEN:735` | GEN.27.8 "Now therefore, my son, hearken to my voice" | decl, assert, narrator | wrong: the root is עַתָּה, and the imperative below it is not seen |
| `wlc:LEV:3` | LEV.1.4 "And he shall lay his hand upon the head …" | decl, assert, narrator | wrong for a law: a third-person weqatal prescription is a command |
| `wlc:DEU:41` | DEU.1.42 "And the LORD said unto me: Say unto them: Go not up …" | decl, narrate, neg, narrator | the polarity leaks: the speech's אֵינֶנִּי hangs on the frame verb (section 4) |

### 13.4 Agreement with the English reading

`macula prag-compare` takes, for every verse that is one Hebrew sentence, the English pragmatics of the JPS 1917 sentences of the same verse (request if any English sentence is a request, then ask, else the first; voice character if any; polarity neg if any) and counts agreement. The first baseline (2026-10-17, commit ed0701a37): **form 89 %, voice 66–74 %, polarity 75–82 %, act about 50 %.** Form agrees because both sides are mostly declarative; act is the weak column.

### 13.5 Why act agrees only half the time

- **narrate vs assert.** The English side has no "narrate"; it calls a narrative sentence an assertion. The 2 010 Hebrew `narrate` readings can never agree. This is a mismatch of label sets, not an error of either side; it should be resolved by mapping, not by changing the Hebrew rules.
- **The frame swallows the speech.** A verse like GEN.3.9 is one Hebrew sentence (frame + question) but in English its request or question is visible to the verse-level reading ("Where art thou?" makes the English verse an `ask`). The Hebrew rule, by design, reads the frame. Of 513 framed sentences with אמר, 480 contain speech as `ccomp` — almost all of them are candidates for this disagreement.
- **Commands of the law in the third person.** "He shall lay his hand" (weqatal 3rd person) and "the priest shall burn" are prescriptions, but rule 4 recognises only the second person. The English reading sees "shall" and often calls it a request; the Hebrew rules call it an assertion.
- **The real verb is not the root.** וַיְהִי (section 5), הִנֵּה (section 6), וְעַתָּה (section 7) and a fronted or dislocated phrase (syntax chapter §15) make a non-verb the root, and then the volitive below it is not seen.
- **Questions not in the lists.** אֵיכָה, אֵי and הֲלוֹא are typed interrogative by MACULA but not in the wh-list; הֲלוֹא is rhetorical, an assertion in the form of a question.

### 13.6 What to do next (not done)

1. **Read the speech, not only the frame.** Emit a second reading for the `ccomp` speech of a framed sentence (speaker = the frame's `nsubj` or its `Frame A0`), and compare speech with speech. The Clear speaker-quotation data (CC BY 4.0, word ids aligned to MACULA) give gold spans and speakers for all 972 Torah quotations, so the speech boundary can be measured — not only guessed from `ccomp` — and the `conj` leak of section 4 can be counted.
2. **Map narrate → assert** for the comparison with English, keeping `narrate` in the Hebrew output.
3. **Third-person prescriptions**: in genre `instruction` (ETCBC labels), a yiqtol or weqatal root in the third person is a command (`request`, indirect 1). Measure on Leviticus before and after.
4. **Re-head marker roots**: when the root is וַיְהִי, הִנֵּה, עַתָּה or a non-verb with a verb `conj`, take the first finite verb as the pragmatic root.
5. **Complete the question lists** (אֵיכָה, אֵי, אָן, הֲלוֹא as rhetorical) and add a vocative cue (a definite noun with no relation to an imperative's arguments).
6. **Politeness**: count נָא, עַבְדְּךָ, אֲדֹנִי in the speech span, not the own clause.
7. **Poetry**: the speaker of a song is given by its frame verse (DEU.31.30, EXO.15.1); carry it over the whole span.

Each change must be measured with `prag-compare` on the whole Torah and reported with its date; the English side is itself rule-based, so agreement is a measure of consistency, not of truth (the Hebrew reading of DEU.32.30 as a question is right whatever the English does).

## 14. In Mova

- **Trees.** `tools/macula` writes `$MOVA_DATA/corpora/judaism/text/torah-wlc-macula/{GEN,EXO,LEV,NUM,DEU}.conllu` with `# sent_id = wlc:<BOOK>:<n>` and `# verse = <BOOK>.<ch>.<v>`, plus `verses.tsv`. MISC `Type` carries the verb form (wayyiqtol 4 187, yiqtol 3 776, qatal 2 694, weqatal 2 434, imperative 782, jussive 192, cohortative 129, …) — everything in sections 1–3 is read from it. MISC `Frame` carries the argument links of section 9.
- **Clause chains** are flat: every clause joined by ו is `conj` of the first clause of the sentence (syntax chapter §10). That is the right input for counting chains and the wrong one for nested speech (section 4).
- **Speech** is `ccomp` for the first clause after a verb of saying; later clauses of the speech attach to the frame as `conj`.
- **Discourse markers** have no relation of their own: וַיְהִי is a root or `conj` verb, הִנֵּה an INTJ heading a `conj`, עַתָּה an ADV, לֵאמֹר an `advcl` infinitive (EXO.14.1). The `discourse` relation is used 5 times.
- **Pragmatics** is `macula prag` (section 13) → `hbo-prag.tsv`, compared with English by `macula prag-compare`. A copy of the same table travels with the Hebrew parse database (`$MOVA_DATA/corpora/judaism/parsedb-hbo/prag.tsv`).
- **The dictionary** (the dictionary builder → `torah-en.tsv`, 3 486 keys) gives each discourse particle its BDB entry and MACULA glosses: וְ (`lemma:וְ`, 13 768 times), הִנֵּה, עַתָּה, נָא, אָמַר; its `.examples` give two short verses per word with Hebrew, MACULA's word-for-word gloss and JPS 1917, and an empty "idea" column for a commentary to come.
- **Not yet in the data:** petuchah/setumah sections, speaker spans, genre labels. All three exist in open sources (OSHB/WLC; Clear speaker quotations, CC BY 4.0; ETCBC genre ranges, MIT) and align by verse or by MACULA word id.

## 15. Sources

- S. R. Driver, *A Treatise on the Use of the Tenses in Hebrew and Some Other Syntactical Questions*, 3rd ed., Oxford 1892 (public domain): ch. VI (§§66–85) wayyiqtol, §78 on וַיְהִי, ch. VIII (§§105–129) weqatal, ch. X participle, Appendix I the circumstantial clause, Appendix V casus pendens. `$RESEARCH_RAW/hebgram/pd/driver-tenses-1892/treatiseonuseof00driv/`; https://archive.org/details/treatiseonuseof00driv
- GKC: Gesenius–Kautzsch–Cowley, *Gesenius' Hebrew Grammar*, Oxford 1910 (public domain): §105b and §147b הִנֵּה; §111 wayyiqtol (§111f–g וַיְהִי); §112 weqatal; §114o לֵאמֹר; §156 circumstantial clauses. `$RESEARCH_RAW/hebgram/open/gesenius-kautzsch-cowley-1910-wikisource/`; https://en.wikisource.org/wiki/Gesenius%27_Hebrew_Grammar
- A. B. Davidson, *Hebrew Syntax*, 3rd ed., 1901 (public domain): §§137–141 circumstantial clause. `$RESEARCH_RAW/hebgram/pd/davidson-hebrew-syntax-1901/`
- C. D. Ginsburg, *Introduction to the Massoretico-Critical Edition of the Hebrew Bible*, London 1897 (public domain): the open and closed sections. `$RESEARCH_RAW/hebgram/pd/ginsburg-introduction-masoretic-1897/`
- F. Brown, S. R. Driver, C. A. Briggs, *A Hebrew and English Lexicon of the Old Testament*, 1906 (public domain), s.v. ו, אָמַר, הִנֵּה, עַתָּה, נָא — in `torah-en.tsv` and `$RESEARCH_RAW/lexres/dictionaries/sefaria/BDB_Dictionary.jsonl`.
- R. E. Longacre, "Discourse Structure, Verb Forms, and Archaism in Psalm 18", *Journal of Translation* (2006), https://doi.org/10.54395/jot-je6cc — `$RESEARCH_RAW/hbo-2006-longacre-discourse-structure-verb-forms-archaism/`
- A. Andrason, "Biblical Hebrew Wayyiqtol: A Dynamic Definition", *JHS* 11 (2011), CC BY 2.5. `$RESEARCH_RAW/hbo-2011-andrason-biblical-hebrew-wayyiqtol-dynamic-definition/`
- G. Lenzi, "Sequences of Verbal Forms and Taxis in Biblical Hebrew", *JHS* (2015), https://jhsonline.org/index.php/jhs/article/view/29348 — `$RESEARCH_RAW/hbo-2015-lenzi-sequences-verbal-forms-taxis-biblical/`
- C. J. Collins, "The Wayyiqtol as 'Pluperfect': When and Why" (1995). `$RESEARCH_RAW/hbo-1995-collins-wayyiqtol-pluperfect-why/`
- Schiller, "From x-qatal to information structure – and back again: Remarks on the analysis of Ancient Hebrew sentences in poetry", *Protokolle zur Bibel* (2020). `$RESEARCH_RAW/hbo-2020-schiller-x-qatal-information-structure-back/`
- T. Li, "Va'yehi as a Discourse Marker in Kings", *AUSS* 44/2 (2006), https://digitalcommons.andrews.edu/auss/vol44/iss2/4 — `$RESEARCH_RAW/hbo-2006-li-va-yehi-discourse-marker-kings/`
- J. A. Westbury, "Towards a complex analysis of Wayihî + T constructions in Biblical Hebrew" (2018), https://scholar.sun.ac.za/handle/10019.1/106379 — `$RESEARCH_RAW/hbo-2018-westbury-towards-complex-analysis-wayihi-t/`
- Yoo, *Paragraphs as episodes: distinguishing paragraphs in Biblical Hebrew narrative text on the basis of linguistic devices*, Stellenbosch 2008, https://scholar.sun.ac.za/handle/10019.1/2818 — `$RESEARCH_RAW/hbo-2008-yoo-paragraphs-episodes-distinguishing-paragraphs-biblical/`
- S. E. Runge, *A Discourse-Functional Description of Participant Reference in Biblical Hebrew Narrative*, Stellenbosch 2007, http://hdl.handle.net/10019.1/1212 — `$RESEARCH_RAW/hbo-2007-runge-discourse-functional-description-participant-reference/`
- S. H. Levinsohn, "NP References to Active Participants and Story Development in Ancient Hebrew", *SIL Working Papers* 44 (2018), https://doi.org/10.31356/silwp.vol44.01 — `$RESEARCH_RAW/hbo-2018-levinsohn-np-references-active-participants-story/`
- N. Winther-Nielsen, "Top Notch Participant Tracking in the Hebrew Bible", *HIPHIL Novum* (2016). `$RESEARCH_RAW/hbo-2016-winther-nielsen-top-notch-participant-tracking-hebrew/`
- C. L. Miller-Naudé and J. A. Naudé, "The Translation of Quotative Frames in the Hebrew Bible" (2026), CC BY 4.0. `$RESEARCH_RAW/hbo-2026-miller-naude-translation-quotative-frames-hebrew-bible/`
- Patty, *Interaction in Biblical Hebrew Narrative: A Conversation-Analytic Approach*, Cambridge 2026, https://www.repository.cam.ac.uk/items/aac07ba1-2170-401f-a043-eb6ad844f7ba — `$RESEARCH_RAW/hbo-2026-patty-interaction-biblical-hebrew-narrative-conversation/`
- G. Savran, "Multivocality in Group Speech in Biblical Narrative" (2009), CC BY 2.5. `$RESEARCH_RAW/hbo-2009-savran-multivocality-group-speech-biblical-narrative/`
- A. Warren-Rothlin, "Politeness Strategies in Biblical Hebrew and West African Languages", *Journal of Translation* (2007), https://doi.org/10.54395/jot-t933r — `$RESEARCH_RAW/hbo-2007-warren-rothlin-politeness-strategies-biblical-hebrew-west/`
- Hare, "Gendered speech: a sociolinguistic study of conversations" (2018). `$RESEARCH_RAW/hbo-2018-hare-gendered-speech-sociolinguistic-study-conversations/`
- Kim, *Hebrew Forms of Address* (2023). `$RESEARCH_RAW/hbobook-2023-kim-hebrew-forms-address/` (not yet read for this chapter)
- Clear Bible, *MACULA Quotation and Speaker Data* (2023), CC BY 4.0, with Glyssen character data (SIL, MIT), https://github.com/Clear-Bible/speaker-quotations/ — `$RESEARCH_RAW/hebgram/discourse/open/clear-speaker-quotations/`
- Eep Talstra Centre for Bible and Computer, *Genre Labels from the Syntactic Variation Project* (2020), MIT, https://doi.org/10.5281/zenodo.7883679 — `$RESEARCH_RAW/hebgram/discourse/open/etcbc-genre-synvar/genre_ranges.csv`
- Staps, annotated כִּי clauses (2024), CC BY 4.0, https://zenodo.org/records/14535294 — `$RESEARCH_RAW/hebgram/discourse/open/staps-ki/`
- MACULA Hebrew Linguistic Datasets (Biblica, Clear Bible, the Groves Center, OpenScriptures), CC BY 4.0, https://github.com/Clear-Bible/macula-hebrew/ — trees, `Type`, `Frame`.
- Mova: `tools/macula/README.md`; `tools/macula/src/main.rs` (`hebrew_prag`, `prag`, `prag_compare`, `SAYING`, `SUBORDINATORS`); `$MOVA_DATA/corpora/judaism/hbo-prag.tsv`; commit ed0701a37 (the first prag-compare baseline); JPS 1917 (public domain) in `$MOVA_DATA/corpora/judaism/text/torah-jps1917/`.
- Wanted (not in our library): R. E. Longacre, *Joseph: A Story of Divine Providence* (1989; 2nd ed. 2003); A. Niccacci, *The Syntax of the Verb in Classical Hebrew Prose* (1990); C. L. Miller, *The Representation of Speech in Biblical Hebrew Narrative* (1996); S. E. Runge and J. R. Westbury, *The Lexham Discourse Hebrew Bible* (2012, data); E. Talstra, "Text Grammar and Hebrew Bible" (I–II, 1978, 1982); R. Buth on information structure in Hebrew narrative.
