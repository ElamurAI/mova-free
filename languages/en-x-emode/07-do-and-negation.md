# 7. `Do`, its absence, and negation

The single deepest syntactic difference between EModE and modern English is the status of the lexical verb.
Modern English needs a dummy `do` to negate or question a lexical verb (*She does not know*, *Does she know?*).
EModE could put the lexical verb itself where the auxiliary goes (*She knows not*, *Knows she?*), and could also
use `do` where modern English cannot (*She did know* with no emphasis). The biblical register keeps both old
patterns. This chapter shows the patterns, how often they occur in JPS 1917, and where Mova's trees go wrong.

## 7.1 The change in brief

Kulick, Ryant & Santorini (2022, «Parsing Early Modern English for Linguistic Search», SCiL) parse the
1.7-million-word PPCEME (1500–1712) to track the rise of auxiliary `do` in six sentence types. Their summary of
the two endpoints:

| Type | Dominant in 1500 | Dominant by 1700 |
|---|---|---|
| negative declarative | *They drank not the ale* | *They did not drink the ale* |
| negative imperative | *Drink not the ale* | *Do not drink the ale* |
| direct question | *Drank they the ale?* | *Did they drink the ale?* |

Hejná & Walkden (2022, §4.4.1) explain the change as the loss of verb movement to the inflection position (V-to-I),
which stretched from about 1400 to 1800, and the rise of `do` to fill that position. Their examples:

- «Looks it not like the king?» (*Hamlet*, c. 1600)
- «Came this out of Cobham's Quiver?» (trial of Walter Raleigh, 1600)
- «I know not by what power I am made bold» (*A Midsummer Night's Dream*, 1595–6)
- «But I rendered not to him according to the benefit received» (Anne Bradstreet, c. 1656)

Following Ecay (2015) and Ellegård (1953), they note that `do` in negative declaratives rose until about 1575 and
then levelled out or dipped; Warner (2005) links the dip to a stylistic dispreference for `do` in high-register
texts after 1600. The King James Bible (1611) is exactly such a text: it prefers the old patterns.

Jespersen (1917, *Negation in English and Other Languages*, p. 14 of the 1917 edition) adds that some verbs kept
the old negative long after the others: besides the auxiliaries, «we must here mention *know*, which now takes
*do*, but was long used in the form *know not*, thus pretty regularly in the seventeenth and often in the
eighteenth and even in the first part of the nineteenth century». He notes that later prose uses of `know not`,
`understood not` imitate «Biblical usage».

Abbott (1884, §305) has the Shakespearean picture: «They, however, did not always observe the modern rule of using
the auxiliary whenever *not* precedes the verb», with a third pattern unknown today, `not` **before** the
lexical verb without `do`: «I not doubt» (*Tempest* II.1.121), «It not belongs to you» (*2 Henry IV* IV.1.98),
«Neat Terence, witty Plautus, now not please» (Jonson on Shakespeare).

## 7.2 The patterns in JPS 1917

Counts in the 6 105 sentences (whole-phrase, case-insensitive, 2026-10-17):

| Pattern | Example | Count |
|---|---|---:|
| lexical verb + `not` | «I know not» (`jps:GEN:98`, GEN 4.9); «Therefore the children of Israel eat not the sinew» (`jps:GEN:1032`, GEN 32.32) | `know not` 8, `knew not` 13, `hearkened not` 11, `saw not` 2, `believed not` 1 … (see 7.4) |
| negative imperative without `do` | «Fear not, Abram» (`jps:GEN:377`, GEN 15.1); «Draw not nigh hither» (`jps:EXO:57`, EXO 3.5); «Go not down unto Egypt» (`jps:GEN:735`, GEN 26.2) | `fear not` 12, `be not` 40, `go not` 6, `eat not` 2, `let not` 11 |
| negative imperative with `do` | «do not» | 8 |
| question with lexical verb inverted | «knowest thou not yet that Egypt is destroyed?» (`jps:EXO:277`, EXO 10.7) | `knowest thou` 1 |
| question with auxiliary inverted | «Hast thou eaten of the tree?» (`jps:GEN:72`) | common |
| affirmative unstressed `do` | «and did eat their portion» (`jps:GEN:1563`, GEN 47.22); «God did prove Abraham» (`jps:GEN:576`, GEN 22.1); «God doth know» (`jps:GEN:65`, GEN 3.5) | `did eat` 27, `doth know` 1, `did prove` 1 |
| negative with `do` | «didst not tell me» (`jps:GEN:331`, GEN 12.18) | `did not` 11, `didst not` 9, `doth not` 3 |
| OVS with postposed subject and `not` | «Only the land of the priests bought he not» (`jps:GEN:1563`, GEN 47.22) | rare |

So JPS 1917 has both systems side by side, as 17th-century prose did. The modern pattern is in the minority.

**Affirmative `do`.** «did eat» is not emphatic: it is the plain past («ate»). In the db, 26 of the 27 `did eat`
are `did` AUX `aux` → `eat` VERB, which is right. The pragmatic layer must not read it as emphasis or contrast.

## 7.3 UD analysis

| Pattern | Head | `not` | Subject |
|---|---|---|---|
| *I know not* | `know` (root/ccomp) | `advmod` of `know`, `Polarity=Neg` optional | `nsubj` of `know` |
| *Fear not, Abram* | `Fear` (imperative, `Mood=Imp`) | `advmod` of `Fear` | none; `Abram` is `vocative` |
| *knowest thou not* | `knowest` | `advmod` of `knowest` | `thou` `nsubj`, after the verb |
| *bought he not* | `bought` | `advmod` of `bought` | `he` `nsubj`, after the verb; object fronted |
| *did eat* | `eat` | — | `nsubj` of `eat`; `did` `aux` |
| *I not doubt* (Shakespeare) | `doubt` | `advmod` of `doubt`, before it | |

The rule is simple: **`not` attaches to the verb it negates, wherever it stands.** In EModE it may stand after the
verb, after the subject of an inverted verb, or after the object («he knew it not»).

## 7.4 Where Mova's trees put `not` (db, 2026-10-17)

For each `not` in the db, I took the word before it (skipping one pronoun such as *thou*, *ye*, *it*, *him*), and
counted where `not` is attached when that word is a lexical VERB:

| Verb tag before `not` | `not` attached to that verb | attached elsewhere |
|---|---:|---:|
| VBD | 74 | 27 |
| VB (imperative, infinitive) | 58 | 21 |
| VBP | 52 | 15 |
| VBZ | 9 | 4 |
| VBN, VBG | 2 | 5 |
| total | 195 | **72** |

(For comparison: `not` after an AUX, 782 cases.)

The 72 «elsewhere» cases are mostly errors. Examples from the db:

| Sentence | `not` attached to | Should be |
|---|---|---|
| «We know not who put our money in our sacks» (`jps:GEN:1423`) | *put* (inside the object clause) | *know* |
| «I know not the day of my death» (`jps:GEN:774`) | *day* (the object) | *know* |
| «Therefore the children of Israel eat not the sinew» (`jps:GEN:1032`) | *sinew* (the object) | *eat* |
| «Go not down unto Egypt» (`jps:GEN:735`) | *down* (the particle) | *Go* |
| «he knew not when she lay down» (`jps:GEN:513`) | *lay* (inside the clause) | *knew* |
| «See that ye fall not out by the way» (`jps:GEN:1498`) | *out* (the particle) | *fall* |
| «fear not to go down into Egypt» (`jps:GEN:1505`) | *go* (the infinitive) | *fear* |
| «Only the land of the priests bought he not» (`jps:GEN:1563`) | *Pharaoh* (in another clause) | *bought* |
| «there remained not so much as one» (`jps:EXO:413`) | *much* | arguably *remained* |

Some cases are defensible (`let not the Lord be angry` → `not` on *angry* negates the lower clause), but the
pattern is clear: the parser has learnt from modern English that `not` before a word negates **that** word
(«not the sinew but the bone», «not to go»), and applies the rule to EModE, where `not` after the verb negates the
verb.

**Why it matters.** `prag/src/feats.rs` decides polarity by looking for a negation word among the root's
children (`neg = "root"`) or anywhere (`neg = "other"`). A `not` hanging on the object makes «I know not the day of
my death» a sentence with a negated object rather than a negated root: the frame «Isaac knows the day of his
death» comes out positive with a negated argument. For a reader of the Torah the polarity of commandments and of
statements like «he knew not» is the core of the meaning.

**Fix** (chapter 13, S1): a deterministic post-parse rule. If `not` (or `never`) immediately follows a finite or
imperative lexical verb, or follows the subject pronoun of an inverted lexical verb, and is attached to a word to
the right of it, re-attach it to the verb as `advmod`. Negative control: «not the sinew but the bone» has no verb
before `not`; «I will not eat» has an AUX before `not` and is untouched.

## 7.5 Other negators

| Form | Meaning | JPS count | UD |
|---|---|---:|---|
| nor | and not; also after a positive clause | 139 | CCONJ `cc`; in «neither … nor» both `cc` (`neither` `cc:preconj`) |
| neither | nor, not either; clause-initial «neither shall ye touch it» (`jps:GEN:63`) | 139 | CCONJ `cc` before a clause; ADV/DET otherwise. The SLM tags it DET 33 and ADV 32 against the db's CCONJ |
| none | no one, not any | 46 | PRON `PronType=Neg` |
| nothing / nought / naught | nothing | 19 / 8 / 0 | PRON |
| no man | nobody | 18 | DET + NOUN |
| never | not ever, often just «not» | 4 | ADV |
| nay | no (answer); also «nay, but…» | 9 | INTJ |
| in no wise | not at all | 1 | ADP + DET + NOUN, `obl`, with `fixed` possible |
| lest | so that … not (chapter 9) | 66 | SCONJ |

**Multiple negation.** EModE allowed negative concord («I cannot go no further», *As You Like It* II.4, often
quoted; Jespersen 1917 ch. on double negation), but the biblical register does not use it. `prag/src/feats.rs`
treats *not, never, nothing, nobody, none, neither, nor, no* as negators; *nought*, *naught*, *nay* and *lest* are
missing (chapter 13, P1).

## 7.6 Questions

Inversion of the lexical verb: «Knowest thou not?», «Whither goest thou?», «Saw ye him whom my soul loveth?»
(KJV Song 3:3). Inversion of an auxiliary: «Hast thou eaten?», «Art thou my very son Esau?» (`jps:GEN:798`).
`Do`-questions: «Dost thou know…?» (`book:pg11027:927`, *Grimm's Fairy Stories*), «Didst thou not…?».

In UD, the subject after the verb is still `nsubj`; nothing else changes. The SLM gets the relations right
in the examples checked (`jps:EXO:277`: `knowest` parataxis, `thou` nsubj, `not` advmod), once the verb is tagged
as a verb. With `knowest` tagged ADJ/JJS (the SLM tags `knowest` → `know` ADJ 12 times, chapter 4), the
question becomes an adjective phrase.

## 7.7 Summary for the parser

1. `not` after a lexical verb negates the verb. Re-attach it.
2. `did` + base verb in affirmative clauses is `aux`, with no emphasis.
3. A verb before its subject is still the head; the subject is `nsubj`.
4. Negative imperatives have no subject: «Fear not» = `Fear` root, `Mood=Imp`, `not` advmod.
5. Add `nought`, `naught`, `nay`, `lest` to the negation lexicon of the pragmatic layer, with `lest` as a negative
   purpose marker rather than a plain negator.
