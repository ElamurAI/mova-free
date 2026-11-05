# 5. Auxiliaries and modals

The auxiliaries carry tense, mood and the speech act of the biblical sentence: the commandments are `thou shalt`,
the promises `I will`, the purpose clauses `that thou mayest`. They are also where Mova's pipeline broke first:
on 2026-10-17 the Torah cycle lost **1 087 distinct sentences** to one gate message, «AUX with lemma 'shalt' is not
in the list of auxiliaries» and its siblings (chapter 12). This chapter gives the forms, their meanings, their
UD analysis, and the measured state.

## 5.1 Forms

| Lemma | 1 sg | 2 sg (thou) | 3 sg | Past 2 sg | Notes |
|---|---|---|---|---|---|
| be | am | **art** | is | **wast** (ind.), **wert** (subj.) | `be` also as an indicative plural: «we be twelve brethren» (KJV Gen 42:32); Abbott §§298–300 |
| have | have | **hast** | **hath** (has) | **hadst** | `hath` 417, `has` 0 in JPS Torah |
| do | do | **dost** (aux), **doest** (lexical) | **doth** (aux), **doeth** (lexical) | **didst** | chapter 4.3 |
| shall | shall | **shalt** | shall | **shouldest**, shouldst | `shalt` 908 in JPS |
| will | will | **wilt** | will | **wouldest**, wouldst | |
| can | can | **canst** | can | **couldest**, couldst | |
| may | may | **mayest**, mayst | may | **mightest**, mightst | |
| must | must | must | must | — | invariable |
| dare | dare | **darest**, durst | dare | **durst** (past of *dare*) | |
| need | need | needest | need(s) | | |
| ought | ought | **oughtest** | ought | | past of *owe* in origin |
| wit (know) | **wot** | **wottest** | **wot**, **wotteth** | **wist** (past) | a preterite-present verb like the modals: «I wot not» (KJV Gen 21:26); JPS has none |

Spelling variants in the tales: `may'st`, `could'st`, `would'st`, `should'st`, `hadst`, `canst`, `shan't`,
`'twould`. Source for the paradigm: Abbott 1884, §§298–331 (Be, Were, Do, Can, May, Must, Shall, Will, Should,
Would); Hejná & Walkden 2022, §4.4.1 box «The modals».

## 5.2 Meanings that differ from modern English

### shall / shalt

In the biblical register `shall` is mostly **deontic** (an order, a law) in the 2nd and 3rd person and
**predictive** in the 1st: «Thou shalt not murder» (`jps:EXO:566`, EXO 20.13) is a prohibition, not a forecast.
Counts in JPS 1917 (2026-10-17): `thou shalt` 710, `shalt not` 161, `shalt thou` 160, `shalt surely` 17,
`ye shall` 388, `shall not` 198, `shall be` 717.

Abbott §317: «Shall. You shall. He shall. On the other hand shall, …» — in Shakespeare `shall` with the 2nd and 3rd
persons keeps its old meaning of obligation or necessity; §318: with the 1st person it states the speaker's
future without volition.

**For Mova's pragmatics**, `thou shalt` + verb is a directive speech act (`act=order` or a law), even though the
mood is indicative. `prag/src/feats.rs` takes the modal from the form (`norm_word`), so `shalt` and `shall` are
two different feature values, and `shalt`, `wilt`, `mayest`, `canst`, `shouldest` each start from zero
evidence. Chapter 13 (P2) proposes mapping them to the modern modal.

### will / wilt

`will` keeps its old lexical meaning «want, intend»: «Wilt thou go with this man?» «I will go.»
(`jps:GEN:683`, GEN 24.58) — Rebekah is asked whether she is willing, not what will happen. `I will` 297 and
`will I` 50 in JPS are mostly promises and decisions. `Would` = «wish»: «Would that we had died by the hand of
the LORD» (`jps:EXO:448`, EXO 16.3), `would that` 6. Abbott §329: «Would for will, wish, require»; §330: «Would
often means "liked", "was accustomed"».

### may / mayest / might

`may` is mostly the **purpose** subjunctive substitute: `that X may` 164, `that X mayest` 28, `that X might` 25:
«that thou mayest know that there is none like unto the LORD our God» (`jps:EXO:212`, EXO 8.10 in the numbering of `torah.tsv.verses`). Also
permission and wish («May God give thee…»). Abbott §307: `may` originally meant «to be able».

### can / canst

Ability; also «know how to»: «whereof thou canst not be healed» (`jps:DEU:764`, DEU 28.27). `can` 17, `canst` 4,
`could` 19 in JPS.

### should / shouldest

Obligation and the conditional after `lest`: «lest thou shouldest say: I have made Abram rich» (`jps:GEN:375`,
GEN 14.23); a hypothetical question: «shouldest thou therefore serve me for nought?» (`jps:GEN:866`, GEN 29.15).
Abbott §§322–326.

### must, ought, need, dare

`must` 21, `ought` 2 in JPS («deeds unto me that ought not to be done», `jps:GEN:531`, GEN 20.9). `need` and
`dare` behave as modals without `-s` and without `do`: «he need not», «durst not».

### do

`do` in the biblical register is either the lexical verb («Noah did according unto all», `jps:GEN:176`) or an
auxiliary that is often **not required by grammar**: «God doth know» (`jps:GEN:65`, GEN 3.5), «did eat their
portion» (`jps:GEN:1563`, GEN 47.22), «God did prove Abraham» (`jps:GEN:576`, GEN 22.1). Chapter 7 treats `do`
in full.

### be as a perfect auxiliary

With verbs of motion and change, EModE forms the perfect with `be`, not `have`:

| Pattern | JPS count | Example |
|---|---:|---|
| are come | 17 | «and they are come» |
| is come / was come / were come / am come / art come | 7 / 4 / 5 / 4 / 3 | |
| was gone / is gone / were gone | 5 / 3 / 2 | |
| is become | 5 | |
| be waxen / am waxed | 7 / 1 | «If thy brother be waxen poor» (`jps:LEV:758`, LEV 25.25); «After I am waxed old» (`jps:GEN:452`, GEN 18.12) |
| was returned | 2 | |

Abbott §295: «With some few intransitive verbs, mostly of motion, be is used» for the perfect. These are **not
passives**: `come` is intransitive. In UD the `be` is `aux` and the participle has `Tense=Past|VerbForm=Part`
without `Voice=Pass`. The db is mixed: of the patterns found, `are come` is `aux` + no voice 10 times but
`aux` + `Voice=Pass` 5 times; `was come`/`were come` are `aux:pass` + `Voice=Pass` 2 + 3 times; `be waxen` is a
copula with an adjective (4) — acceptable for `waxen` (= grown), but then its lemma should be `wax`.
A rule: `be` + past participle of an intransitive verb of motion or change (`come`, `go`, `rise`, `wax`,
`become`, `depart`, `enter`, `return`, `fall`, `grow`) → `aux`, no `Voice=Pass`.

### have as a full verb and `had rather`

`have` + infinitive with `to` = must («I have to tell thee») is rare in JPS. `had rather`, `had liefer`,
`were better` (preference) occur in Shakespeare and the tales.

## 5.3 Word order with auxiliaries

- **Inversion in questions**: «Hast thou eaten of the tree?» (`jps:GEN:72`, GEN 3.11); «Art thou my very son
  Esau?» (`jps:GEN:798`, GEN 27.24); «Wilt thou go with this man?» (`jps:GEN:683`).
- **Inversion after a fronted adverbial or object**: «with rooms shalt thou make the ark» (`jps:GEN:163`, GEN 6.14);
  «therefore shall his days be a hundred and twenty years» (`jps:GEN:151`, GEN 6.3).
- **Negation after the auxiliary or after the subject**: «thou shalt not eat of it» (`jps:GEN:51`); «hast thou
  not withheld» type; with inversion `not` follows the subject: «knowest thou not» (`jps:EXO:277`).
- **Stranded auxiliary**: «'Wilt thou go with this man?' And she said: 'I will go.'» — modern ellipsis would
  answer «I will»; JPS repeats the verb.

## 5.4 UD analysis

| Form | UPOS | XPOS | Lemma | FEATS | Relation |
|---|---|---|---|---|---|
| shalt | AUX | MD | shall | `Number=Sing|Person=2|VerbForm=Fin|Style=Arch` | aux |
| wilt | AUX | MD | will | same | aux |
| canst, mayest, mightest, shouldest, wouldest, couldst | AUX | MD | can, may, might, should, would, could | same | aux |
| hast | AUX | VBP | have | `Mood=Ind|Number=Sing|Person=2|Tense=Pres|VerbForm=Fin|Style=Arch` | aux |
| hath | AUX / VERB | VBZ | have | `Mood=Ind|Number=Sing|Person=3|Tense=Pres|VerbForm=Fin|Style=Arch` | aux or (main verb) root etc. |
| hadst | AUX / VERB | VBD | have | `…|Person=2|Tense=Past…` | |
| doth, dost, didst | AUX | VBZ, VBP, VBD | do | person, tense, `Style=Arch` | aux |
| art | AUX | VBP | be | `Mood=Ind|Number=Sing|Person=2|Tense=Pres|VerbForm=Fin|Style=Arch` | cop / aux / aux:pass |
| wast | AUX | VBD | be | `…|Person=2|Tense=Past…` | cop / aux |
| wert | AUX | VBD | be | `Mood=Sub|…` or `Mood=Ind` | cop / aux |

The registry's list of auxiliary lemmas (`en/data/ud-registry-en.tsv`) is: be, can, could, dare, do, get, have,
may, might, must, need, ought, shall, should, will, would. Every archaic form above lemmatizes to one of these,
so **no change to the registry is needed** — only the lemma has to be right before the gate. `wot`/`wit` is a
lexical verb (VERB), not AUX.

UD English EWT 2.18 has one auxiliary `art` in its training set, annotated `be|AUX|…|Person=2|Style=Arch|…`
(`en_ewt-ud-train.conllu`); the convention above follows it.

### Feature consistency in the db (2026-10-17)

| Form | db FEATS | Comment |
|---|---|---|
| shalt (890 tokens, all AUX) | `VerbForm=Fin` 796, `Mood=Ind|Tense=Pres|VerbForm=Fin` 24, `_` 17, with `Person=2` 34, other 19: ten different feature sets | no person, no `Style=Arch`; modern `shall` in EWT has `VerbForm=Fin` only, so this is defensible but loses the person agreement |
| hath (AUX/VERB) | `…|Person=3|…` 344, with `Style=Arch` 48, without person 15 | mostly right |
| art | `be|AUX` cop 32, aux 6, aux:pass 4, `art|NOUN` 2 (correct nouns: «the art of the perfumer») | right |

## 5.5 What the small model does

SLM now (2026-10-17), UPOS of the archaic auxiliaries in the 6 105 Torah sentences:

| Form | Tokens | Right (AUX/VERB) | Wrong |
|---|---:|---:|---|
| shalt | 908 | 46 AUX | NOUN 857, X 4, PROPN 1 |
| hath | 417 | 56 AUX + 56 VERB | NOUN 214, ADV 53 |
| hast | 136 | 0 | NOUN 118, ADV 13, PROPN 5 |
| art | 58 | 0 | NOUN 56, PROPN 2 |
| didst | 44 | 0 | NOUN 44 |
| doth | 29 | 2 VERB | ADV 11, ADJ 10, NOUN 2, other 4 |
| dost | 7 | 1 VERB | NOUN 6 |
| wilt | 36 | 36 AUX | — |
| mayest | 50 | 0 | ADJ 42 (`may` JJS), ADV 8 |

`wilt` is right because it is in EWT as the verb («wilt» = droop) and the modal reading follows from the
context `thou wilt` + verb; everything else is a noun or an adjective to the model. **1 129** of the 6 013 aligned
Torah sentences have a mis-tagged archaic auxiliary (SLM now). The lemma is now right thanks to the
`ONLY_VERBS` list (`shall`, `have`, `do` whatever the tag), and since 2026-10-17 21:11 `annot` repairs the lemma
of an AUX whose lemma is still the form (`annot/src/main.rs`, function `drafts_from`). After both fixes the
refusals at the gate dropped from 28 % of the drafts (round 5: 1 050 of 3 791) to 0.4 % (round 9: 1 of 244) — chapter 12.

The remaining problem is that a **noun** `shalt` does not become an AUX with the right features by any
deterministic step; the graph parser re-tags it and copies the features of the most frequent `MD` token. A
closed-class table in front of the tagger (chapter 13, T1) removes the problem at its source.
