# 8. Word order and phrase structure

EModE word order is modern in its basic shape (subject – verb – object), but it allows more movement: the verb
can come before the subject after an adverb or a fronted object, an object can stand first, a subject can be
announced and then picked up by a pronoun, and noun phrases carry appositions and postposed modifiers that
modern prose avoids. The biblical register adds patterns calqued from Hebrew (chapter 11). This chapter lists
the orders a parser meets and how UD annotates them.

## 8.1 Subject–verb inversion in declaratives

EModE inherits from Old English a tendency to put the finite verb second after a fronted adverb (a «verb-second»
residue). Abbott 1884, §§420–427 («Transposition») describes the freedom of Elizabethan order, including
transposition after emphatic words (§425) and after relatives (§426).

| Trigger | Example | Count in JPS |
|---|---|---:|
| `there` + verb + subject | «and there went a man of the house of Levi»; «there came two angels» | `there` before an inverted lexical verb: 131 (db) |
| `thus` | «thus saith the LORD» | `thus saith` 15 |
| `then` | «then shall his kinsman … come» (`jps:LEV:758`, LEV 25.25) | `then shall` 33 |
| `therefore` | «therefore shall his days be a hundred and twenty years» (`jps:GEN:151`, GEN 6.3) | `therefore shall` 8 |
| `so` | «so went Abram» | `so went` 2 |
| a fronted prepositional phrase | «with rooms shalt thou make the ark» (`jps:GEN:163`, GEN 6.14); «from Thy face shall I be hid» (`jps:GEN:103`, GEN 4.14) | common |
| a fronted object | «Only the land of the priests bought he not» (`jps:GEN:1563`, GEN 47.22) | rare |
| `neither`/`nor` | «neither shall ye touch it» (`jps:GEN:63`, GEN 3.3) | 5 inverted lexical verbs after `neither` (db) |
| parenthetical speech verb | «By Myself have I sworn, saith the LORD» (`jps:GEN:592`, GEN 22.16) | |

In the db (2026-10-17), the subject follows a lexical VERB head in about 300 declarative cases; the most frequent
word before the verb is `there` (131), then `and` (32), `shall` (31: «shall the man be cut off»), `thus` (14),
`then` (9), `therefore` (5).

**UD.** Inversion changes nothing in the tree: the subject is `nsubj` of the verb, the fronted phrase keeps its
relation (`obl`, `obj`, `advmod`). `there` in `there came` is `expl` only if it is the dummy subject; in «And
there went a man» it is the dummy (`expl`), the real subject `man` is `nsubj`. In «thus saith the LORD»
`thus` is `advmod`.

**Parser risk.** A modern parser expects the first noun phrase to be the subject. «with rooms shalt thou make the
ark» → `rooms` as subject; «Only the land of the priests bought he not» → `land` as subject. The pronoun case
(`thou`, `he` nominative) is the reliable cue, and the archaic pronouns are exactly the forms the SLM mis-tags
(chapter 3). With `thou` tagged PRON|Case=Nom, a rule «a nominative pronoun after the finite verb is its
subject» is safe.

## 8.2 Left dislocation and resumptive pronouns

The biblical register (after Hebrew *casus pendens*) often announces a noun phrase and picks it up with a
pronoun:

- «And Abel, he also brought of the firstlings of his flock» (`jps:GEN:90`, GEN 4.4).
- «the man whom the LORD doth choose, he shall be holy» (`jps:NUM:616`, NUM 16.7).
- «that the LORD, He is God» (`jps:DEU:147`, DEU 4.35).

UD English annotates the dislocated phrase as `dislocated` attached to the head of the clause, and the
resumptive pronoun as `nsubj`. The db analyses «the LORD, He is God» with `LORD` as `vocative` (`jps:DEU:147`),
which is wrong: Moses is not addressing God there. A rule: NP + comma + nominative pronoun agreeing in person and
number + finite verb → NP `dislocated`.

## 8.3 Noun phrases

### Apposition with a possessive

| Pattern | Example | Count |
|---|---|---:|
| the LORD + possessive + God | «the LORD thy God» | 243 (+ `your` 82, `our` 30) |
| name + possessive + kin noun | «Abel thy brother» (`jps:GEN:98`, GEN 4.9); «Isaac his son» | `thy brother` 49, `his son` 161 (all uses) |
| name + `the` + title | «Pharaoh king of Egypt» | |

The db makes `God` the `appos` of `LORD` 231 times out of 233 in «the LORD thy God»: consistent. In «Where is Abel
thy brother?» the db has `brother` as `appos` of `Abel` — right.

### Postposed adjectives and fixed titles

«God Almighty» (6), «the land of Egypt», «the city of Enoch», «a land flowing with milk and honey». The title
type `God Almighty` follows the modern pattern of `flat` or `appos`; pick one.

### Partitive `of`

The biblical register uses `of` + definite noun as a partitive object, with no quantifier: «thou shalt not eat
of it», «he took of the stones of the place», «brought of the firstlings» (`jps:GEN:90`). Counts: `eat of` 34,
`take of` 26, `took of` 7, `drink of` 7.

UD has no partitive case in English; the phrase is an oblique (`obl` with `case` = `of`). The db does this
consistently: `eat of` → `obl` 32 times, `took of` → `obl` 6; `take of` → `obl` 20, `obj` 5 (the 5 are the
inconsistent ones). For the **world** of the text, though, «eat of the tree» means «eat (some fruit) of the
tree»: the tree is the source, the eaten thing is implicit. The frame reader needs a rule that turns
`obl:of` after *eat*, *take*, *drink*, *give*, *offer*, *bring* into a partitive object.

### Possessive `of` and construct chains

«the children of Israel» 371, «the face of the deep», «the tree of the knowledge of good and evil». These are
ordinary `nmod` chains, longer than modern ones because they render Hebrew construct chains. They make
sentences long and deep; they do not make them hard.

## 8.4 Double prepositions and adverbial particles

| Form | Example | Count |
|---|---|---:|
| from off | «put off thy shoes from off thy feet» (`jps:EXO:57`, EXO 3.5) | 22 |
| from before | «from before the LORD» | 34 |
| from among | | 38 |
| from under, from above, from within | | |
| unto (= to) | «said unto him» | 2 372 |
| according unto | «Noah did according unto all» (`jps:GEN:176`, GEN 7.5) | 15 |

`from off`: the first preposition is `case` of the noun, the second also `case` (UD allows two `case` dependents)
or `fixed`. The db marks `according unto` as `fixed` 15 times — consistent with modern `according to`.

`unto` is a plain preposition (ADP, `case`): 2 339 times in the db, plus 16 `fixed`, 3 `obl`, 1 `iobj` (the last
four are errors). `unto` before a dative object («gave unto him») is still `case` of an `obl`, not an `iobj`: UD
English reserves `iobj` for bare noun phrases.

## 8.5 Pronoun objects and ethical datives

- **Benefactive object pronouns after imperatives**: «Make thee an ark» (`jps:GEN:163`), «get thee into the land
  of Moriah» (`jps:GEN:577`), «Take Me a heifer» (`jps:GEN:385`). Counts: `get thee` 13, `make thee` 21,
  `take thee` 7.
- **`I pray thee`** as a parenthetical politeness formula: «Say, I pray thee, thou art my sister» (`jps:GEN:326`,
  GEN 12.13): 52 times. UD: `parataxis` of the main verb (or `discourse` if treated as a fixed formula, like
  modern *please*).
- **Reflexive without `-self`**: «he lay him down», «get thee hence»: the simple pronoun is the reflexive object
  (`obj`), and its lemma is the ordinary pronoun.

## 8.6 Phrasal verbs and separated particles

«Go not down unto Egypt» (`jps:GEN:735`), «rose up early» (6), «lifted up his eyes» (9), «put off thy shoes». The
particle is `compound:prt`. With `not` between the verb and the particle, the parser tends to attach `not` to the
particle (chapter 7.4).

## 8.7 Clause-level patterns

| Pattern | Example | UD analysis |
|---|---|---|
| predicate first | «Here am I» (`jps:GEN:576`, GEN 22.1), 9 times | `Here` is the predicate (root), `am` `cop`, `I` `nsubj` (UD English treats locative *here* with copula as the predicate) |
| verb of saying + colon + quotation | «And he said: 'I will go.'» | quotation `ccomp` of *said* (db practice; parataxis also possible: choose one) |
| `saying` before speech | «And the LORD spoke unto Moses, saying:» (`jps:EXO:149`) | `saying` `advcl` of *spoke*, the speech `ccomp` of *saying* (or the speech is in the next sentence: then *saying* has no complement in its own sentence) |
| `answered and said` | 15 | two conjoined verbs, the speech `ccomp` of the second |
| `that` as a general subordinator | «And God saw the light, that it was good» (`jps:GEN:3`, GEN 1.4) — prolepsis: the object of the clause is raised into the main clause | `light` `obj` of *saw*, the `that`-clause `ccomp` or `acl` of *light*; the meaning is «saw that the light was good» |
| `for that` | «for that he also is flesh» (`jps:GEN:151`, GEN 6.3) | `for that` = because; `fixed` |
| `as` + participle | | |
| verbless clauses | «And it was so.» | normal |

## 8.8 Order inside the verb group

«thou shalt surely die» (`jps:GEN:51`, GEN 2.17): adverb between the auxiliary and the verb, as today.
«ye shall not surely die» (GEN 3.4) puts `not` before `surely`. «I will surely visit you». `surely` renders a
Hebrew infinitive absolute (chapter 11) and is `advmod`.

## 8.9 What the parser needs

1. Use the case of the archaic pronoun (`thou`, `ye`, `he`) to find a postposed subject.
2. Attach `there` in `there went/came/was` as `expl`.
3. Re-attach dislocated noun phrases (`dislocated`) when a resumptive pronoun follows.
4. Keep `unto` as `case` and never `iobj`.
5. Map partitive `of` after verbs of eating, taking and giving to a partitive role in the frame layer.
