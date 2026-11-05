# 9. The subjunctive, conditions and wishes

Modern English keeps the subjunctive in a few frozen corners (*if I were you*, *God save the King*, *it is
essential that he be present*). EModE used it as a living mood: after `if`, `though`, `lest`, `until`, `ere`,
`whether`, `except`, in wishes and curses, and in commands to a third person. The biblical register uses it all
the time, because the law books are built of conditions: «If a man smite…», «if he be found…», «lest he die».

## 9.1 Forms

The present subjunctive is the **bare stem in every person**: no `-s`, no `-eth`, no `-est`.

| | Indicative | Subjunctive |
|---|---|---|
| 3 sg, lexical | he smiteth / smites | (if) he **smite** |
| 2 sg, lexical | thou smitest | (if) thou **smite** |
| 3 sg, *be* | he is | (if) he **be** |
| 2 sg, *be* | thou art | (if) thou **be** |
| pl, *be* | they are | (if) they **be** |
| past, *be* | he was, thou wast | (if) he **were**, thou **wert** |

For most verbs only the 3rd and 2nd person singular show the difference, so a subjunctive with a plural or a
1st person subject is invisible. *Be* shows it everywhere. Abbott 1884, §§298–302 (Be, Beest, Were), §§361–371
(«Subjunctive, simple form», «Subjunctive auxiliary forms», «The Subjunctive is replaced by the Indicative
after…», «Subjunctive used optatively or imperatively», «Subjunctive in a subordinate sentence», «after verbs
of command and entreaty»), and §363 notes that the indicative was already replacing it in Shakespeare.

## 9.2 Uses in JPS 1917

| Use | Example | Count / note |
|---|---|---|
| condition (`if`) | «If thy brother be waxen poor, and sell some of his possession, then shall his kinsman … come» (`jps:LEV:758`, LEV 25.25) | `if X be` 74 |
| condition, alternation with the indicative | «if thou wilt take the left hand, then I will go to the right; or if thou take the right hand, then I will go to the left» (`jps:GEN:343`, GEN 13.9) | the two moods side by side in one verse |
| condition with `there be` | «if there be a white rising in the skin» (`jps:LEV:320`, LEV 13.10) | |
| negative purpose (`lest`) | «neither shall ye touch it, lest ye die» (`jps:GEN:63`, GEN 3.3) | `lest` 66; `lest X die` 7 |
| temporal limit (`until`, `till`) | «until he come out, and have made atonement» (`jps:LEV:476`, LEV 16.17) | `until X have` 9, `until X be` 8 |
| indirect question (`whether`) | «whether it be …» | `whether X be` 28 |
| purpose (`that`) | «that he die» | `that X die` 22, `that X be` 14 |
| optative (wish) | «So God give thee of the dew of heaven» (`jps:GEN:803`, GEN 27.28) | `cursed be` 16, `blessed be` 9 |
| jussive with *be* | «Be it according to thy word» (`jps:EXO:212`, EXO 8.10) | `be it` 1, `far be it` 2 |
| counterfactual *were* | «as it were», «were it» | `as it were` 5, `were it` 2 |
| wish with *would* | «Would that we had died» (`jps:EXO:448`, EXO 16.3) | 6 |

Pattern count: conjunction (`if`, `though`, `lest`, `until`, `till`, `whether`, `except`, `unless`) + a 2nd or 3rd
person singular subject (*he, she, it, thou, there, a man, any man, a soul, the* + noun) + a bare verb from a list
of 31 frequent verbs: **191** in JPS 1917 (2026-10-17). This undercounts (longer subjects, other verbs), but it
shows the scale: the subjunctive is a routine form of the law books.

### `lest`

`lest` («for fear that», «so that … not») takes the subjunctive or `should`: «lest ye die», «lest thou shouldest
say» (`jps:GEN:375`, GEN 14.23). It is a **negative** subordinator: the clause states what must not happen. For the
pragmatic layer this is a polarity marker; for the parser it is SCONJ `mark`.

The SLM tags `lest` as an adjective superlative with lemma `l` 63 times out of 66 (SLM now, 2026-10-17), because
`lest` looks like *l* + *-est*; it is in **57** Torah sentences. The db has the right analysis (SCONJ, lemma
*lest*) for the 65 tokens it holds, all repaired by the teacher.

## 9.3 Other conditional forms

| Form | Meaning | Example | UD |
|---|---|---|---|
| `an`, `and`, `an if`, `and if` | if | «An thou knowest him again, he is thine; an thou knowest him not, he is mine.» (`book:pg29672:104`, *Cossack Fairy Tales*) | SCONJ `mark`, lemma *an*; Abbott §§101–105 (*An*, *And if*) |
| inversion with *had*, *were*, *should* | if | «had I known», «were it so», «should he come» | the auxiliary is `aux`/`cop` of the clause; the clause `advcl` |
| `except` | unless | «except thou bless me» (KJV Gen 32:26) | SCONJ `mark` |
| `so` | provided that | «so thou wilt» | SCONJ |
| `but` | unless, except that | Abbott §§118–128 | SCONJ or CCONJ by use |
| `save` | except | «save only that which the young men have eaten» (`jps:GEN:376`, GEN 14.24) | ADP / SCONJ; db `save` ADP 8, VERB 9 (the VERB ones may be *save* = rescue) |

The conditional `an` is the main parser trap in the tales: «An thou knowest» looks like the article `an` before a
noun. Abbott §103: «Latterly the subjunctive, falling into disuse, was not considered sufficient to express a
condition», and `and if`/`an if` took over. A rule: sentence-initial or clause-initial `an`/`and` immediately
followed by a nominative pronoun (*I, thou, he, she, we, ye, they*) and a finite verb is SCONJ `mark` = *if*.

## 9.4 The third-person imperative (jussive) with *let*

«Let there be light» (`jps:GEN:2`, GEN 1.3), «Let us make man» (`jps:GEN:27`, GEN 1.26), «let them have dominion»,
«let not the Lord be angry» (`jps:GEN:472`, GEN 18.30). Counts: `let us` 37, `let him` 27, `let them` 31, `let there
be` 5, `let not` 11. These are modern in form; the archaism is only in their frequency. UD: `Let` root
(`Mood=Imp`), the subject of the lower clause `obj` of *let* (or `nsubj` of the lower verb: follow the db's
practice), the lower verb `xcomp` or `ccomp`.

## 9.5 How UD marks it, and how the db does

UD English EWT 2.18 marks a present subjunctive as `VB` with
`Mood=Sub|Number=Sing|Person=3|Tense=Pres|VerbForm=Fin` («it is crucial that the forces on the offensive be
united»), and subjunctive *were* as `VBD` with `Mood=Sub|…|Tense=Past|VerbForm=Fin` (`en_ewt-ud-train.conllu`: 6
and 6 tokens of *be*, plus 2 *go*, 2 *come*, 1 *take*, 1 *use* … with `Mood=Sub`). EWT is small on this point
because modern English rarely needs it.

In the db (2026-10-17) the subjunctive is marked inconsistently:

| Case | Tagged `VerbForm=Inf` (no mood) | `Mood=Sub` | `Mood=Ind` |
|---|---:|---:|---:|
| `be` two words after *if / though / lest* | 35 | 15 | 5 |
| any verb right after *if / lest / though / until / till / whether / except* + *he / she / it / thou / there* | 53 | 24 | — |

`Mood=Sub` appears on 79 `be` AUX tokens, 18 `die`, 14 `have` VERB, 14 `come`, 12 `have` AUX … in the db: the
teacher knows the category but does not apply it every time. `VerbForm=Inf` on a finite verb with its own
subject is simply wrong, and it changes the clause type for any reader of the tree (an infinitive clause has no
tense and no subject agreement).

**Rule** (chapter 13, S3): a verb tagged VB that has an `nsubj` (or `expl` *there*) and a `mark` among
{*if, though, although, lest, until, till, ere, whether, except, unless, that*} — or is the root of an optative
clause («God give thee», «cursed be») — gets `Mood=Sub|Tense=Pres|VerbForm=Fin` with the subject's person and
number. Negative control: «that he may go» (modal + infinitive: `go` stays `VerbForm=Inf`), «if ye will do
this» (indicative `will`).

## 9.6 The tales and Shakespeare

The tales keep the subjunctive in formulaic speech: «if it please your majesty», «though he slay me», «God
save you», «Heaven forbid». Shakespeare uses it freely after `if`, `though`, `till`, `ere`, `lest`, and in
wishes («The devil damn thee black», *Macbeth* V.3), and often mixes it with the indicative in one sentence
(Abbott §363). Optative subjunctives at the start of a sentence («Long live the king») look like imperatives; the
subject before the verb tells them apart: an imperative has no subject (or *thou*/*you* after it), an optative
has a third-person subject before the verb.

## 9.7 Effect on Mova's frames

A conditional sentence is a rule, not a fact: «If a man smite his servant … he shall be punished» states a law,
not an event. The frame reader needs:

1. the `mark` of the clause (`if`, `lest`, `until`) to decide **modality** — condition, negative purpose, limit;
2. `Mood=Sub` to confirm that the clause is not asserted;
3. `lest` as a negative purpose (what must not happen), not as an adjective (SLM now) and not as a plain negator.

Today the first point works (the `mark` is right in the db), the second is unreliable (see 9.5), and the third
depends on the teacher having repaired the SLM's `l` JJS.
