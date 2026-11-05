# 14. Sample texts, read word by word

Five short passages, each with what an EModE reader must notice and what Mova does with it. They are worked
examples for the chapters before; the trees are quoted from the db (`parsedb/current.conllu`) and from the small
model (`en-agreed.bin`, «SLM now»), both 2026-10-17.

## 14.1 Genesis 1:1–5, KJV (1769) and JPS 1917 side by side

| v. | KJV (1769 text) | JPS 1917 (`jps:GEN:0`–`jps:GEN:5`; v. 5 is two sentences in `torah.tsv`) |
|---|---|---|
| 1 | In the beginning God created the heaven and the earth. | IN THE beginning God created the heaven and the earth. |
| 2 | And the earth was without form, and void; and darkness [was] upon the face of the deep. And the Spirit of God moved upon the face of the waters. | Now the earth was unformed and void, and darkness was upon the face of the deep; and the spirit of God hovered over the face of the waters. |
| 3 | And God said, Let there be light: and there was light. | And God said: 'Let there be light.' And there was light. |
| 4 | And God saw the light, that [it was] good: and God divided the light from the darkness. | And God saw the light, that it was good; and God divided the light from the darkness. |
| 5 | And God called the light Day, and the darkness he called Night. And the evening and the morning were the first day. | And God called the light Day, and the darkness He called Night. And there was evening and there was morning, one day. |

What to notice:

- **Supplied words.** The KJV brackets (`[was]`, `[it was]`) mark words with no Hebrew counterpart; JPS prints them
  as normal text. A parser must not see the brackets (chapter 2.3).
- **Background `Now`.** JPS opens v. 2 with `Now`, marking a circumstantial clause; KJV keeps the and-chain
  (chapter 11.1).
- **Quotation.** KJV has no quote marks; JPS has a colon and single quotes (chapter 2.3).
- **Prolepsis.** «saw the light, that it was good» = saw that the light was good (chapter 8.7).
- **Object fronting with inversion of nothing:** «the darkness He called Night» — object first, then subject
  and verb; JPS capitalises `He` for God (chapter 2.2).
- **Semantics.** «one day» (JPS, after the Hebrew *yom eḥad*) vs «the first day» (KJV).
- **Modernisation.** JPS: «unformed and void» for «without form, and void», «hovered» for «moved».

## 14.2 Genesis 3:9–13 (JPS 1917) — the second person at work

> 9 «And the LORD God called unto the man, and said unto him: 'Where art thou?'» (`jps:GEN:69`)
> 10 «And he said: 'I heard Thy voice in the garden, and I was afraid, because I was naked; and I hid myself.'» (`jps:GEN:70`)
> 11 «And He said: 'Who told thee that thou wast naked? Hast thou eaten of the tree, whereof I commanded thee that
> thou shouldest not eat?'» (`jps:GEN:71`, `jps:GEN:72`)

Word by word for `jps:GEN:72`, the db against the small model:

| # | Form | db (lemma, UPOS, XPOS) | SLM now | Note |
|---:|---|---|---|---|
| 1 | Hast | have AUX VBP | have **PROPN** NNP | capital at the start + unknown form = name |
| 2 | thou | thou PRON PRP | thou **AUX** VBD | the inverted subject is read as an auxiliary |
| 3 | eaten | eat VERB VBN | eat VERB VBN | |
| 4–6 | of the tree | ADP DET NOUN | same | partitive `of` (chapter 8.3) |
| 8 | whereof | whereof ADV WRB | whereof **ADJ** JJ | pro-adverb opening a relative clause (chapter 10.1) |
| 10 | commanded | command VERB VBD | same | |
| 11 | thee | thou PRON PRP | thee **NOUN** NN | the object pronoun as a noun (chapter 3.9) |
| 12 | that | that SCONJ IN | that PRON WDT | |
| 14 | shouldest | **shall** AUX MD | should **ADJ** JJS | db lemma should be *should*; the SLM reads a superlative (chapter 4.6) |
| 15–16 | not eat | PART, VERB VB | same | |

db tree: `eaten` root; `Hast` aux; `thou` nsubj; `tree` obl; `commanded` acl:relcl of `tree`; `whereof` obl of
`commanded`; `thee` iobj; `eat` ccomp of `commanded`; `shouldest` aux and `not` advmod of `eat`. This is right
(with *should* as the lemma of `shouldest`). In the SLM's draft five of eighteen tokens have the wrong part of
speech, all of them archaic.

## 14.3 Genesis 2:17 — a law and a penalty

> «but of the tree of the knowledge of good and evil, thou shalt not eat of it; for in the day that thou eatest
> thereof thou shalt surely die.» (`jps:GEN:51`)

| Form | db | SLM now | Point |
|---|---|---|---|
| shalt (×2) | shall AUX MD | shall **NOUN** NN | deontic `shall`: prohibition, then sentence (chapter 5.2) |
| eatest | eat VERB VBP | eat **ADJ** JJS | 2nd person `-est` (chapter 4) |
| thereof | thereof ADV RB | thereof **ADJ** JJ | «of it» (chapter 10.1) |
| surely | surely ADV | surely ADV | renders the Hebrew infinitive absolute: certainty, not manner (chapter 11.4) |
| die | die VERB VB | die VERB **VBP** | |
| that (in *the day that*) | that PRON, `obl:unmarked` of `eatest` | | a relative `that` standing for *on which* |

The db tree: `eat` root, `die` conj of `eat`, `eatest` acl:relcl of `day`. For the frame reader: a prohibition
(`shalt not eat`), a condition in time (`in the day that thou eatest`), a consequence with modal certainty
(`shalt surely die`).

## 14.4 A fairy tale in the old voice (*Grimm's Fairy Stories*, PG 11027)

> «But what wilt thou give me if I succeed in fetching thy plaything up again?» (`book:pg11027:1293`)
>
> «Dost thou know that when I first saw the portrait of thy beauteous face I fell down in a swoon before it?»
> (`book:pg11027:927`)
>
> «They called to her and asked, "Who art thou?" But she gave no answer.» (`book:pg11027:1167`)
>
> «"Falada, Falada, there thou art hanging!"» (`book:pg11027:45`)

What to notice:

- `wilt thou give` — `will` as the future of a question, and the speech of a frog to a princess: the old
  pronoun is a voice marker, not a number marker (chapter 3.2).
- `Dost thou know` — `do`-support in a question, in the archaic form; the 19th-century translator uses the modern
  question syntax with the old morphology.
- `there thou art hanging` — the progressive with `art`: a modern construction in old forms.
- The narrator («They called to her and asked») speaks modern English; only the characters' speech is archaic.
  A document-level register detector would mark the **quotations**, not the whole text.

From the *Russian Fairy Tales* (PG 22373): «She was terribly frightened, but answered, "I am." "Art thou sitting?"
"I sit." "Dost thou spin?" "I spin."» (`book:pg22373:5543`) — the questions are archaic, the answers modern (`I
sit`, not `I do sit`): exactly the EModE alternation of chapter 7.

From the *Cossack Fairy Tales* (PG 29672): «An thou knowest him again, he is thine; an thou knowest him not, he is
mine.» (`book:pg29672:104`) — conditional `an` = *if* twice (chapter 9.3), postverbal `not` (chapter 7),
predicative `thine`/`mine` (chapter 3.5).

## 14.5 Shakespeare, from the grammars

Short quotations, as given by Abbott (1884) and Hejná & Walkden (2022):

| Line | Source | Point |
|---|---|---|
| «Looks it not like the king?» | *Hamlet* I.1, in Hejná & Walkden §4.4.1 | question by inversion of the lexical verb; `not` after the subject |
| «I know not by what power I am made bold» | *A Midsummer Night's Dream* I.1, ibid. | postverbal `not` |
| «I not doubt» | *Tempest* II.1.121, Abbott §305 | preverbal `not` without `do` |
| «It not belongs to you» | *2 Henry IV* IV.1.98, Abbott §305 | same |
| «if thou thou'st him some thrice, it shall not be amiss» | *Twelfth Night* III.2, Hejná & Walkden §4.3.1 | `thou` used as a verb: to insult with *thou* |
| «you are over boots in love» … «it boots thee not» | *Two Gentlemen of Verona* I.1.25, 28, Abbott §231 | switch between `you` (jest) and `thee` |
| «The venom clamours of a jealous woman / Poisons more deadly than a mad dog's tooth» | *Comedy of Errors* V.1.69, Abbott §333 | plural subject with a verb in `-s` |
| «Where men enforced doth speak anything» | *Merchant of Venice* III.2.33, Abbott §334 | plural subject with `doth` |
| «Thou torments» | *Richard II* IV.1, Folio, Abbott §340 | 2nd person `-ts` for `-test` |
| «The innocent milk in it most innocent mouth» | *Winter's Tale* III.2, Abbott §228 | `it` as a genitive (= *its*) |

None of these is in Mova's corpora today. They are the hard end of EModE: agreement does not identify the subject
(§333–334), `not` can precede the verb (§305), `thou` can be a verb. If Mova ever reads the plays, chapter 4.1's
warning applies: agreement cannot be used as a parsing cue.
