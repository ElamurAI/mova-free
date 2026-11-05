# 11. The biblical style

The English Bible is EModE grammar wrapped around Hebrew syntax. Tyndale and the 1611 translators kept the
Hebrew order and connectives wherever English would bear them, and the result became a register of its own,
imitated for four centuries in sermons, hymns, fairy-tale prose and fantasy. This chapter describes the
discourse features of that register — what makes a sentence «sound biblical» — and how Mova parses them. The
counts are JPS 1917, 6 105 sentences, 2026-10-17.

## 11.1 And-chains (parataxis)

Biblical Hebrew narrative links clauses with *wa-* («and»), and the translations keep it:

> «And God saw the light, that it was good; and God divided the light from the darkness.» (`jps:GEN:3`, GEN 1.4)

- **3 315 of 6 105** sentences of the JPS Torah begin with `And` (54 %); in the db, 3 590 of 6 065 sentences
  start with `And` once the quotation marks are counted as part of the sentence (59 %).
- Inside sentences, `; and` joins clauses that modern prose would make separate sentences.
- Other sentence openers: `Now` (background: «Now the serpent was more subtle», `jps:GEN:60`), `So`, `Then`,
  `And it came to pass`.

**UD.** A sentence-initial `And` is `cc` of the root. Coordinated clauses inside a sentence are `conj`; after a
semicolon UD allows `conj` or `parataxis`. The db mostly uses `conj` for `; and` and `parataxis` for clauses
joined by a colon or semicolon without `and`. Choose one convention and state it; the frame reader must not read
a 15-clause `conj` chain as one event with 15 coordinated verbs sharing a subject.

**For the frame layer**, the and-chain is a sequence: `and` between narrative clauses means «and then». The
order of the clauses is the order of the events, unless a clause is marked as background (`Now …`, a participle,
a verbless clause, `was` + adjective).

## 11.2 «And it came to pass»

The Hebrew *wa-yehi* («and it was») opens a new episode or a time frame. The English formula:

| Pattern | Count |
|---|---:|
| and it came to pass | 103 |
| it came to pass (any) | 106 |
| and it shall come to pass | 23 |
| it shall come to pass (any) | 27 |
| so let it come to pass | rare: «So let it come to pass, that the damsel…» (`jps:GEN:634`, GEN 24.14) |

Typical shape: «And it came to pass, as the camels had done drinking, that the man took a golden ring»
(`jps:GEN:642`, GEN 24.22) — the formula, a temporal clause, then the main event introduced by `that`.

**Meaning.** `it came to pass` has no content of its own: it means «then this happened». The event is the
`that`-clause (or the following conjunct, when there is no `that`).

**UD analysis (proposed).** `came` root; `it` `expl` (it refers to nothing); `to pass` `xcomp` (`to` `mark`);
the temporal clause `advcl` of `came`; the event clause `csubj` of `came` (an extraposed clausal subject, as in
«it happened that…», «it is clear that…»); when the event is a coordinated clause without `that`, `conj`.

**The db (2026-10-17)** has 106 «came to pass» tokens with five different analyses of `it` and `pass`:

| `it` | `pass` | count |
|---|---|---:|
| nsubj | xcomp | 68 |
| expl | xcomp | 23 |
| nsubj | csubj | 7 |
| expl | csubj | 3 |
| nsubj | ccomp | 1 |
| (with `came` as conj or parataxis) | | 4, overlapping the rows above |

and the main event attached to `came` as `ccomp` (23 + 8 with a conj), `csubj` (8 + 10 with a conj), `advcl`
(7), `conj` … — no two large groups agree. The world reader therefore cannot find «what happened» by one rule.
Chapter 13 (S2) proposes a deterministic normaliser that rewrites all of them to the analysis above. Negative
control: «so that it came to pass» inside a quotation, and the literal *come to* + noun («came to the
place»).

## 11.3 Speech framing

| Formula | Count | Example |
|---|---:|---|
| and said | 162 | |
| and he said | 150 | «And he said: 'Here am I, my son.'» (`jps:GEN:582`, GEN 22.7) |
| said unto | 383 | |
| spoke unto | 181 | |
| And the LORD spoke unto Moses, saying: | 73 (99 with other endings) | `jps:EXO:149` (EXO 6.10) |
| answered and said | 15 | |
| saying | 309 | the speech follows after a colon |

`saying` is a participle introducing direct speech: `advcl` of the verb of speaking, with the quotation as its
`ccomp`. When the quotation is in the next sentence (the 73 «spoke unto Moses, saying:» are complete
sentences of four words), `saying` has no complement and the speech act spans two sentences. For the world
layer, the next sentence is the content of the speech; the db cannot express that, and the frame reader must
link them by the colon at the end.

## 11.4 Hebraisms in English dress

| Hebraism | English | Count | Example |
|---|---|---:|---|
| infinitive absolute (emphasis) | `surely` + verb; KJV also «dying thou shalt die» (margin) | `surely` 97 | «thou shalt surely die» (`jps:GEN:51`, GEN 2.17) |
| cognate object | «die the death», «dream a dream» | 1 | |
| construct chain | X of Y of Z | very common | «the tree of the knowledge of good and evil» |
| «son of N years» | N years old | `years old` 67 | |
| «lift up the eyes and see» | lifted up his eyes, and saw | `lifted up` 31 | «Abraham lifted up his eyes, and saw the place afar off» (`jps:GEN:579`, GEN 22.4) |
| «rise early in the morning» | rose up early | 6 (+ `betimes`) | `jps:GEN:767` |
| «find favour in the eyes of» | found grace in the eyes of | 7 | «But Noah found grace in the eyes of the LORD» (`jps:GEN:156`, GEN 6.8) |
| «in the eyes of» = in the opinion of | in the eyes of / in the sight of | 11 / 35 | «Let it not be grievous in thy sight» (`jps:GEN:553`) |
| «to the face of» | upon the face of the earth | `the face of the` 26 | |
| «by the hand of» = through, by means of | by the hand of | 18 | |
| «unto this day» | to this day | 15 | `jps:GEN:1032` |
| «know» = have intercourse | knew his wife | 2 | `jps:GEN:106` |
| «flowing with milk and honey» | | 12 | |
| «with a mighty hand and an outstretched arm» | | 5 | |
| «these are the generations of» | genealogy heading | 11 | |
| «in the selfsame day» | on that very day | 6 | |
| «throughout your generations» | for all time | 25 | |
| «a statute for ever» | | 15 | |
| «be cut off from (his people)» | die / be excommunicated | 21 | |
| «I am the LORD» | formula closing a law | 75 | |
| «behold» (*hinneh*) | look, here is | 237 | chapter 10.2 |
| «unto» | to | 2 372 | chapter 8.4 |
| pleonastic pronoun | «And Abel, he also brought» | | chapter 8.2 |

These are **not ungrammatical** in English; they are frequent and formulaic. For the parser they are ordinary
trees. For the world layer they are idioms whose meaning is not the sum of the words:

- «found grace in the eyes of X» = X favoured him;
- «lifted up his eyes, and saw» = looked up and saw (no lifting event);
- «knew his wife» = had intercourse (chapter 10);
- «be cut off from his people» = a punishment, not a physical cut;
- «surely die» = will certainly die (modal certainty, not manner).

Mova's pragmatic layer has an expression base for idioms (`docs/pragmatics.md`); these formulas
belong there, with the biblical register as their domain.

## 11.5 Numbers, dates, measures

- Numbers in words, in the old order: «nine hundred and thirty years» (`jps:GEN:121`, GEN 5.5), «an hundred and twenty» (KJV)
  vs «a hundred and twenty» (JPS, `jps:GEN:151`); «threescore and ten», «fourscore» (score = 20). JPS keeps
  *threescore* (24; KJV 27) and *fourscore* (6; KJV 6) beside *seventy* (27) and *sixty* (11).
- Dates by month number and day: «in the second month, on the seventeenth day of the month» (2 such dates with the seventeenth day).
- Measures: cubits, shekels, ephahs, hins (chapter 10.5).

`threescore and ten` is a compound numeral (70): `nummod` with `flat` or `compound` inside; the value must be
computed (3 × 20 + 10). The math part of Mova (`math/`) handles number words; *score* must be in its table.

## 11.6 The tales imitate the register

The 0.3 tales use the biblical register for solemnity, magic and old speech:

- «Falada, Falada, there thou art hanging!» (`book:pg11027:45`, *Grimm's Fairy Stories*)
- «"Listen, O Dog! thou wert a Dog (Sobaka), a clean beast; through all Paradise the most holy didst thou roam."»
  (`book:pg22373:6435`, *Russian Fairy Tales*)
- «These speeches then their brother spake / To this sick couple there» (`book:pg14241:1251`, a ballad in *More
  English Fairy Tales*)

## 11.7 What the register gives Mova for free

The biblical register is a gift for a deterministic system:

- its vocabulary of archaic forms is **closed** (chapters 3–6 list nearly all of it);
- its formulas recur hundreds of times (`thou shalt` 710, `unto` 2 372, `and it came to pass` 103), so one rule
  fixes hundreds of trees;
- every verse exists in Hebrew (MACULA, `tools/macula`) and in the KJV, so a tree can be checked against an
  aligned second reading (`macula prag`, `prag-compare` in the Hebrew pragmatics baseline of commit `ed0701a37`).

The register is also a trap for a learnt model: one word in 32 is a form the training data has never seen, and
the forms are frequent enough to teach the model wrong habits if the drafts it learns from are wrong.
