# 4. Verb endings: -est, -st, -eth, -th

EModE verbs agree with their subject in more cells than modern verbs do. Modern English marks only the third
person singular present (`-s`); EModE also marks the **second person singular** in the present and the past, and
it has two competing third-person endings, `-s` and `-eth`. In the biblical register the old endings are kept
everywhere: JPS 1917 never writes `has` or `does` (0 and 0 in the Torah), always `hath` and `doth` (2026-10-17).

## 4.1 The present indicative

| Person | Modern | EModE, biblical | EModE, colloquial (Shakespeare) | Notes |
|---|---|---|---|---|
| 1 sg | I love | I love | I love | |
| 2 sg | you love | thou **lovest** | thou lovest / lov'st | `-est` syllabic after a sibilant or in solemn style; `-st` after a vowel (`thou say'st`, `thou seest`) |
| 3 sg | he loves | he **loveth** | he loves (but `hath`, `doth`) | `-eth` survives longest in `hath` and `doth` (Hejná & Walkden 2022, §4.3.2) |
| 1 pl | we love | we love | we love | |
| 2 pl | you love | ye love | you love | |
| 3 pl | they love | they love | they love / **loves** / **loveth** / **loven** | Abbott §§332–334: Northern `-s`, Midland `-en`, Southern `-eth` plurals all occur |

### History in one paragraph

`-eth` (Old English `-eþ`) was the southern third-person ending; `-s` came from the north. Hejná & Walkden (2022,
§4.3.2) summarise: at 1500 `-th` is the most common form of the third person singular; it declines sharply
during the period, and by 1700 it is rare, surviving longest in `hath` and `doth`; Shakespeare normally uses `-s`
but keeps `hath` and `doth` (after Nevalainen & Raumolin-Brunberg 2016). The King James Bible (1611) is
conservative and uses `-eth` throughout, and every later biblical revision copies it. So `-eth` in a 19th- or
20th-century text is a **style marker**, not a time marker.

### Plural in -s and -th

Abbott §333: «Third person plural in -S. This form is extremely common in the Folio. It is generally altered by
modern editors, so that its commonness has not been duly recognized.» His examples: «The venom clamours of a
jealous woman / Poisons more deadly than a mad dog's tooth» (*Comedy of Errors* V.1.69). §334 gives plural
`-th`: «Where men enforced doth speak anything» (*Merchant of Venice* III.2.33). §335: a singular verb before a
plural subject, especially `There is` + plural, is normal.

For a parser this means: **in Shakespeare, a verb in `-s` or `-th` does not prove a singular subject.** Agreement
cannot be used to pick the subject in EModE drama. In the biblical register it can: JPS 1917 agrees by the
modern rule plus the 2nd person.

### Second person -ts

Abbott §340: in verbs ending in `-t`, the 2nd person `-test` is often written `-ts`: «Thou torments» (*Richard
II* IV.1, Folio), «Thou requests». A trace of the northern `-s` in the 2nd person.

## 4.2 The past

| Person | Weak verb | Strong verb | `be` | `have` | `do` |
|---|---|---|---|---|---|
| 1, 3 sg | loved | came | was | had | did |
| 2 sg | **lovedst**, lovedest | **camest** | **wast**, **wert** | **hadst** | **didst** |
| pl | loved | came | were | had | did |

The 2nd person past adds `-est` or `-st` to the past form: `camest`, `knewest`, `stoodest`, `broughtest`,
`drankest`, `sentest`, `saidst`, `defiledst`, `stretchedst`, `madest`, `gavest`, `tookest`. `Wast` is the
indicative, `wert` the subjunctive (and later a free variant).

JPS 1917 examples:

- «Who told thee that thou wast naked?» (`jps:GEN:71`, GEN 3.11)
- «why didst thou not tell me that she was thy wife?» (`jps:GEN:331`, GEN 12.18)
- «and didst not suffer me to kiss my sons and my daughters?» (`jps:GEN:965`, GEN 31.28)

## 4.3 Spelling rules of the endings

| Stem | -eth | -est | Examples (JPS 1917) |
|---|---|---|---|
| ends in silent `-e` | drop `e`: `make` → `maketh` | `makest` | maketh 4+6, loveth 3+2, moveth 3, causeth 3+7 |
| ends in a single consonant after a short stressed vowel | double: `sit` → `sitteth` | `sittest` | sitteth 7+1, setteth 5, putteth 2+3, biddeth, stirreth, sheddeth |
| ends in consonant + `y` | `-ieth`: `cry` → `crieth` | `criest` | crieth 2, marrieth 1 |
| ends in `-ie` | `die` → `dieth`, `lie` → `lieth` | `diest`, `liest` | dieth 7+2, lieth 13+3 |
| ends in a vowel | `go` → `goeth`, `see` → `seeth`, `do` → `doeth` | `goest`, `seest`, `doest` | goeth 20+4, seeth 4+5, doeth 9+6 |
| sibilant | `pass` → `passeth`, `touch` → `toucheth` | `passest` | toucheth 19+11, passeth 4 |
| contracted (closed set) | `hath`, `doth`, `saith` | `hast`, `dost`, `sayst`, `canst`, `wilt`, `shalt`, `art` | |

(Counts «a+b»: tokens of the form with lemma = form in the db + tokens lemmatized correctly in the db,
2026-10-17; they add up to the frequency.)

**`doeth` and `doth`, `doest` and `dost`.** The full forms are the lexical verb *do* («he that doeth this»),
the contracted forms the auxiliary («God doth know», `jps:GEN:65`, GEN 3.5). The distinction is a
19th-century editorial habit more than a 1611 rule, but JPS keeps it: `doeth` 15, `doest` 8 are always VERB in
the db; `doth` 29, `dost` 7 are AUX 27 and 6 times.

**False endings.** Many ordinary words end in `-eth`, `-est`, `-st` or `-th` and must not be stripped:

| Ending | Not a verb ending | In the Torah |
|---|---|---|
| `-eth` | teeth, Seth, Japheth, Heth, Beth, Daleth, Teth (Hebrew letter names), ordinals *twentieth*, *fortieth* | Heth 16, Beth 32, Japheth 9 in the db — all PROPN, correctly |
| `-est` | priest 283, against 241, beast 72, lest 66, feast 36, rest 36, east 27, harvest 15, youngest 13, west 8, interest 10, nest 3, eldest, best, greatest | superlatives and nouns |
| `-st` | first 210, midst 93, host 40, cast 38, most 32, must 21, dust 19, thrust 10, past 7, just 7, lost 5, locust 5 | |
| `-th` | earth, death, breath, truth, forth, south, month, tenth | |

`morph::archaic` (en/src/morph.rs) handles this with two guards: it acts only on a verb tag (VB, VBD, VBG,
VBN, VBP, VBZ, MD), and it leaves alone any form the built-in dictionary already knows. The second guard is right
for `digest`, `interest`, `harvest` and wrong for a set of AGID entries discussed in 4.6.

## 4.4 Negative and interrogative forms

The endings stay on the lexical verb when there is no `do` (chapter 7):

- «knowest thou not yet that Egypt is destroyed?» (`jps:EXO:277`, EXO 10.7) — inversion, the ending on the verb,
  `not` after the subject.
- «I know not» (`jps:GEN:98`, GEN 4.9).
- «Whither goest thou?» (tales and JPS: `goest` 27 tokens).

## 4.5 How Mova lemmatizes the endings

`en::morph::archaic(lower, tag)` (added 2026-10-17, commit `3e639a08e`) runs first in `Morph::lemmatize`:

1. A list of 19 forms that are never anything but verbs (`shalt`, `hath`, `hast`, `hadst`, `doth`, `dost`, `didst`,
   `wast`, `wert`, `canst`, `couldst`, `shouldst`, `wouldst`, `mayest`, `mayst`, `mightest`, `saith`, `sayest`,
   `spake`) gets its lemma **whatever the tag**: the small model tags `shalt` as a noun.
2. With a verb tag only: 25 irregular forms (the list above plus `wilt`, `art`, `brake`, `gavest`, `camest`,
   `wentest`).
3. With a verb tag, for a form the dictionary does not know: strip `-eth` or `-est`, try the stem, the stem +
   `e`, the stem with a doubled final consonant undoubled, and `-ieth` → `-y`; return the first candidate that is a
   known lemma.

The unit test `archaic_verbs` checks `knoweth`, `maketh`, `crieth`, `saith`, and as negative controls `art` as a
noun, `greatest`, `rest`, `makes`, `digest`, `interest`.

### Measured against the db (2026-10-17)

I ran `archaic` and `Morph::lemmatize` (a small throw-away Rust binary linked to the `en` crate of this branch)
over every VERB or AUX token of the db whose form ends in `-eth`, `-th`, `-est`, `-st` or `-t` and that is not an
ordinary modern form, with the db's XPOS as the tag. 1 484 tokens:

| Outcome | Tokens | Meaning |
|---|---:|---|
| `lemmatize` = db lemma | 972 | handled |
| db lemma = the form itself (`giveth`, `goeth`, `hath`, `saith`) | 400 | **the db is wrong**: of these, `archaic` would give the right lemma for 345 and a wrong one for 55 |
| `lemmatize` ≠ db lemma, db lemma right | 112 | **the lemmatizer is wrong** |

The 400 unlemmatized tokens sit in **307 sentences** with an `-eth` verb plus 21 sentences with a `hath`, `hast`
or `saith` VERB (not AUX) whose lemma is the form (db, 2026-10-17). They did not pass through `archaic` with a
verb tag; chapter 12 traces how (the small model tags them NOUN/ADJ, the graph parser makes them VERB and keeps
the noun's lemma).

The lemmatizer's own errors (types, with tokens):

| Form | db (right) | `archaic` gives | Cause |
|---|---|---|---|
| goest 27, doest 8, seest 7, shouldest 7, liest 3, passest 1, layest 1, livest 1, longest 1, meanest 1, lightest 1 | go, do, see, should, lie, … | nothing (form left as is) | the dictionary knows the form — as an **AGID superlative** (`goest JJS`, `doest JJS`, `seest JJS`, `liest JJS`, `shouldest JJS` in `en/data/forms.tsv`), so guard 3 says «known form, not archaic» |
| cometh 7, comest 5 | come | `com` | stem tried before stem+`e`, and `com` is a known lemma |
| seeth 5 | see | `se` | same |
| hateth 3 | hate | `hat` | same (`hat` is a noun) |
| lieth 3, dieth 2 | lie, die | `li`, `di` | same |
| biteth 1, rideth 1, writeth 1 | bite, ride, write | `bit`, `rid`, `writ` | same |
| putteth 3, puttest 4 | put | `putt` | the undoubled candidate comes after `putt`, which is a (golf) verb |
| marrieth 1 | marry | `marri` | the `-ieth` → `-y` candidate comes last |
| sentest 1 | send | `sente` | past + `-est` not handled |
| saidst 5, broughtest 3, stoodest 2, knewest 1, drankest 1, defiledst 1, stretchedst 1, badest 1 | say, bring, stand, know, drink, defile, stretch, bid | nothing / `bad` | the 2nd person **past** is not handled at all |

All three causes have deterministic fixes (chapter 13, items L1–L3):

1. take a candidate only if it is a **verb** lemma (has a verb paradigm), and among verb candidates prefer the
   most frequent in UD (put over putt, come over com, see over se);
2. ignore AGID-only `JJS` analyses when the tag is a verb tag, or drop AGID superlatives of non-adjectives
   (`go`, `do`, `see`, `lie`, `should`) from the dictionary;
3. for `-est`/`-st`/`-edst` after a **past form** (`stood`, `brought`, `said`, `knew`), look the stem up as a VBD
   form and return that form's lemma.

## 4.6 What the small model does with the endings

The tagger is the bigger problem. SLM now, aligned with the db (2026-10-17):

| Class | Tokens | UPOS wrong | Typical tags |
|---|---:|---:|---|
| `-eth` verbs | 607 | 546 (90.0 %) | NOUN 401, ADJ 101, ADP 40, ADV 14 |
| `-est`/`-st` 2nd person verbs (open class) | 166 | 151 (91.0 %) | ADJ/JJS (`goest` → `go` JJS 28, `camest` → `cam` JJS 9, `liest` → `ly` JJS 3) |
| modern words | 169 654 | 6 123 (3.6 %) | |

The model has never seen `-eth`, so it guesses from the shape: a long unknown word ending in `-th` is a noun
(*earth*, *death*, *breath*). It has seen `-est` thousands of times — as the superlative. With the AGID entries
above, the dictionary confirms the superlative reading, and the lemmatizer then strips `-est` by the superlative
rule: `camest` → `cam`, `comest` → `com`, `liest` → `ly`, `lest` → `l` (63 of 66 `lest`, chapter 9).

Sentence counts, SLM now, out of 6 013 Torah sentences aligned with the db: **454** sentences have a mis-tagged
`-eth` verb, **126** a mis-tagged 2nd person `-est` verb.

The graph parser and the teacher repair most of the UPOS (the db has the right UPOS), but the lemma survives
(4.5) and so do wrong features (chapter 12).

## 4.7 Features

UD has no feature for «archaic ending», but `Style=Arch` is allowed on VERB and AUX by the registry. In the db:

| | with `Style=Arch` | without |
|---|---:|---:|
| `-eth` VERB | 50 | 560 |
| 2nd person `-est` VERB (`Person=2`) | 58 | 71 |

Person and number of the 2nd person verbs are right in the db: `VBP|Mood=Ind|Number=Sing|Person=2|Tense=Pres`
(110), `VBD|…|Person=2|Tense=Past` (19), with 3 stray `Number=Plur|Person=3` copied from the tag's most frequent
features. A deterministic rule can set them all: a verb whose form ends in a 2nd person ending and whose subject
is `thou` (or no subject) gets `Number=Sing|Person=2|Style=Arch`; a verb in `-eth`/`-th` gets
`Number=Sing|Person=3|Style=Arch`.
