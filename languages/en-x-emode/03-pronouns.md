# 3. Pronouns

The pronoun system is the most visible difference between EModE and modern English, and in the biblical register
it is also the most frequent one: in JPS 1917 the forms `thou`, `thee`, `thy`, `thine`, `thyself` and `ye` make up
4 895 of 156 603 words — one word in 32 (2026-10-17). A parser that gets them wrong loses person, number and case
in every third sentence.

## 3.1 The paradigm

| | Subject | Object | Possessive determiner | Possessive pronoun | Reflexive |
|---|---|---|---|---|---|
| 1 sg | I | me | my / **mine** (before a vowel) | mine | myself |
| 2 sg | **thou** | **thee** | **thy** / **thine** (before a vowel) | **thine** | **thyself**, thee |
| 3 sg m | he | him | his | his | himself, him |
| 3 sg f | she | her | her | hers | herself, her |
| 3 sg n | it | it | **his** (to ~1600), **it** (dialect), **its** (from ~1600) | — | itself |
| 1 pl | we | us | our | ours | ourselves |
| 2 pl | **ye** | you | your | yours | yourselves |
| 3 pl | they | them | their | theirs | themselves |

Bold marks what differs from modern English. Sources: Abbott 1884 §§ 212–213 (`thee` for `thou`), 217–218
(`his`), 228 (`its`), 231–235 (`thou` and `you`), 236 (`ye`), 237 (`mine`/`thine`); Hejná & Walkden 2022,
§4.3.1.

### Person and number in UD terms

| Form | UPOS | XPOS (PTB) | Lemma (proposed) | FEATS |
|---|---|---|---|---|
| thou | PRON | PRP | thou | `Case=Nom|Number=Sing|Person=2|PronType=Prs|Style=Arch` |
| thee | PRON | PRP | thou | `Case=Acc|Number=Sing|Person=2|PronType=Prs|Style=Arch` |
| thy | PRON | PRP$ | thy | `Case=Gen|Number=Sing|Person=2|Poss=Yes|PronType=Prs|Style=Arch` |
| thine (determiner) | PRON | PRP$ | thy (as `mine` → `my`) | `Case=Gen|Number=Sing|Person=2|Poss=Yes|PronType=Prs|Style=Arch` |
| thine (predicate) | PRON | PRP | thine (as predicative `mine`) | `Number=Sing|Person=2|Poss=Yes|PronType=Prs|Style=Arch` |
| thyself | PRON | PRP | thyself | `Case=Acc|Number=Sing|Person=2|PronType=Prs|Reflex=Yes|Style=Arch` |
| ye | PRON | PRP | ye (or *you*: a project decision, see 3.8) | `Case=Nom|Number=Plur|Person=2|PronType=Prs|Style=Arch` |

The proposed lemmas follow the convention the db already uses for the modern forms: `me` → `I`, `him` → `he`,
`thee` → `thou`, while possessive determiners keep their own lemma (`my` → `my`, `his` → `his`, `thy` → `thy`).
The registry (`en/data/ud-registry-en.tsv`) allows `Style=Arch` on PRON, so all of these features pass the gate.

## 3.2 Thou and you: grammar

In Old and Middle English `thou` was simply the singular and `ye`/`you` the plural. Under French influence the
plural became a polite singular (the T/V distinction: Brown & Gilman 1960, cited in Hejná & Walkden 2022, §4.3.1).
By Shakespeare's time:

> «*Thou*, in Shakespeare's time was, very much like "du" now among the Germans, the pronoun of (1) affection
> towards friends (2) good-humoured superiority to servants, and (3) contempt or anger to strangers. It had,
> however, already fallen somewhat into disuse, and, being regarded as archaic, was naturally adopted (4) in the
> higher poetic style and in the language of solemn prayer.» — Abbott 1884, §231.

Abbott's further observations (§§231–235):

- fathers almost always address sons with `thou`, sons fathers with `you`;
- a master uses `thou` to a servant, but not always (§232);
- `thou` to a stranger who is not an inferior is an insult (§233); Sir Toby's advice in *Twelfth Night* III.2,
  «if thou thou'st him some thrice, it shall not be amiss», uses `thou` as a verb meaning «to insult by saying
  *thou*» (Hejná & Walkden 2022, §4.3.1);
- speakers switch inside one scene with their mood: Valentine and Proteus use `thou` in earnest and `you` when
  jesting (*Two Gentlemen of Verona* I.1; Abbott §231).

Wales (1983, cited by Hejná & Walkden) argues that by the late 16th century `thou` was already rare and that a
switch to it was usually expressive. The Quakers then made a point of `thee` and `thou` to everyone; George Fox
called `you` to one person an «evill custome» (Hejná & Walkden 2022, box «Pronoun iconoclasts»).

### In the biblical register

The Bible keeps the **old number distinction**, not the social one: `thou` is singular, `ye`/`you` plural,
whoever the addressee. God says `thou` to Adam and Adam says `thou` to God:

- «Where art thou?» (`jps:GEN:69`, GEN 3.9) — God to the man.
- «wilt Thou indeed sweep away and not forgive the place» (`jps:GEN:466`, GEN 18.24) — Abraham to God, with
  the capital that JPS gives divine pronouns.
- «Ye shall not eat of any tree of the garden?» (`jps:GEN:61`, GEN 3.1) — the serpent to the woman, quoting God's
  command to the couple (plural).

Deuteronomy addresses Israel as one people in `thou` and as a crowd in `ye`, often in the same chapter. The
counts by book (2026-10-17) show how much the person system weighs in the law books:

| Book | Words | thou | thee | thy | ye | you | your |
|---|---:|---:|---:|---:|---:|---:|---:|
| Genesis | 38 419 | 281 | 266 | 285 | 96 | 83 | 81 |
| Exodus | 32 672 | 415 | 160 | 195 | 135 | 91 | 69 |
| Leviticus | 24 282 | 96 | 25 | 119 | 221 | 126 | 127 |
| Numbers | 32 691 | 143 | 89 | 72 | 202 | 85 | 80 |
| Deuteronomy | 28 539 | 612 | 442 | 663 | 228 | 163 | 163 |

In Deuteronomy one word in 16 is a second-person singular pronoun.

In the tales, `thou` is a voice marker. A frog, a king, a fairy, a peasant in a translated folk tale say `thou`:
«But what wilt thou give me if I succeed in fetching thy plaything up again?» (`book:pg11027:1293`, *Grimm's
Fairy Stories*); «"Art thou sitting?" "I sit." "Dost thou spin?" "I spin."» (`book:pg22373:5543`, *Russian Fairy
Tales*). A narrator who switches to `thou` in the middle of modern English usually marks prayer, poetry, an oath
or an old saying.

## 3.3 Ye and you: case

`Ye` is the nominative plural, `you` the oblique. Abbott §236 says the distinction is «observed in our version of
the Bible» but «disregarded by Elizabethan authors», who use `ye` in questions, entreaties and rhetorical
appeals and `you` as subject freely («Therein, ye gods, you make the weak most strong», *Julius Caesar* I.3,
quoted by Abbott). JPS 1917 keeps the case distinction strictly (2026-10-17):

| Context | ye | you |
|---|---:|---:|
| before *shall, will, have, are, did, know, see, go, come, eat, keep, do, be* (subject) | 534 | 23 |
| after *said, say, speak, unto, to, give, with, before, for, upon, among, of* (object) | 16 | 303 |

So in JPS `ye` is nearly always a subject and `you` nearly always an object or oblique. The db agrees: `ye` is
`nsubj` 839 times, `nsubj:pass` 26, `obj` 4; `you` is `obl` 236, `obj` 153, `iobj` 61, `nmod` 50.

**Vocative `ye`.** «hear, ye sons of Jacob» (`jps:GEN:1600`, GEN 49.2) and «ye take too much upon you, ye sons of
Levi» (`jps:NUM:616`, NUM 16.7) put `ye` before a noun, like modern «you guys». The db has `ye` with relation
`det` on the noun in four sentences (`jps:GEN:1600`, `jps:DEU:885`, `jps:NUM:616`, `jps:DEU:928`). UD English
EWT 2.18 does it the other way round: in «you guys» the pronoun is the head and `guys` is its `nmod:unmarked`
(three sentences in `en_ewt-ud-train.conllu`). Following EWT, «ye sons of Jacob» is `ye` (vocative) ←
`sons` (`nmod:unmarked`); the `det` analysis should go.

## 3.4 Thee for thou, and other case slips

Abbott §§212–213: `thee` sometimes stands where `thou` is expected, after imperatives («Look thee», «Haste thee»)
and after `be` («I would not be thee»). In the biblical register the imperative-plus-`thee` construction is a
**reflexive or ethical dative**:

- «get thee into the land of Moriah» (`jps:GEN:577`, GEN 22.2) — *get thee* = go, `thee` reflexive object.
- «Make thee an ark of gopher wood» (`jps:GEN:163`, GEN 6.14) — *make thee* = make for yourself, `thee` is
  `iobj`/`obl` (benefactive), not the subject.
- «Take Me a heifer of three years old» (`jps:GEN:385`, GEN 15.9) — the same with `Me`.

The subject of these imperatives is not expressed; `thee` must not become `nsubj`. In the db `thee` is `obl` 324
times, `obj` 361, `iobj` 111, `nmod` 75 — never `nsubj`.

Abbott §241 notes the opposite case too: `thou` is often omitted after a verb in the 2nd person singular,
because the ending already says who («Hast eaten?»). The ending `-est`/`-st` is then the only subject marker
(chapter 4).

## 3.5 Mine and thine before vowels

`Mine` and `thine` are the old forms of the determiner before a vowel or `h`, like `an` before a vowel: «mine
eyes», «thine heir». Abbott §237 says Shakespeare uses both forms before vowels «with little distinction» and
that `mine`/`thine` tend to be the unemphatic forms. The 1769 KJV is regular about it; JPS 1917 modernises half
of the cases:

| Phrase | KJV Torah | JPS Torah |
|---|---:|---:|
| thine eyes | 21 | 24 |
| thine heart / thy heart | 22 / 5 | 0 / 27 |
| thine house / thy house | 13 / 12 | 0 / 26 |
| mine eyes | 3 | 3 |

JPS examples: «This man shall not be thine heir; but he that shall come forth out of thine own bowels shall be
thine heir» (`jps:GEN:380`, GEN 15.4). Predicative `thine`: «nor aught that is thine» (`jps:GEN:375`, GEN 14.23),
«And thy issue … shall be thine» (`jps:GEN:1581`, GEN 48.6).

In the db, `thine` has lemma `thine` 109 times, `thy` 3, `Thine` 3, while `mine` has lemma `mine` 27, `my` 19
and `Mine` 1. Neither is consistent. UD English EWT 2.18 lemmatizes `mine` as `my` 18 times out of 20
(`en_ewt-ud-train.conllu`). Proposal (chapter 13): `mine` → `my` and `thine` → `thy` in both uses, as EWT does;
lemma in lower case always.

## 3.6 His, it and its

Until about 1600 the genitive of `it` was `his`: «the fruit tree yielding fruit after his kind» (KJV Gen 1:11).
`Its` appears at the end of the 16th century; Abbott §228 writes that it «was not used originally in the
Authorized Version of the Bible, and is said to have been rarely used in Shakespeare's time», though common in
Florio's Montaigne (1603). A dialect `it` genitive also occurs: «The innocent milk in it most innocent mouth»
(*Winter's Tale* III.2, quoted in §228).

For Mova this is a **coreference trap** in the KJV and in older texts: `his` can point to a tree, a city, an
altar. In JPS 1917 the problem is gone: `its` 163, KJV 1 (chapter 1). For KJV-style texts the coreference module
must allow `his` → inanimate antecedent when no animate candidate agrees in gender.

## 3.7 Relatives and indefinites

| Form | Use | Example | Note |
|---|---|---|---|
| `which` for persons | old usage | KJV «Our Father which art in heaven» (Matt 6:9) | JPS Torah uses `who` 161 vs KJV 80 and `which` 759 vs 981: JPS replaces many personal `which` (Abbott §§265–268) |
| `the which` | relative with article | KJV 7, JPS 1 | `the` is part of the relative: `det` of `which` |
| `that` as relative after a personal antecedent | normal | «he that shall come forth» (`jps:GEN:380`) | `that` PRON, `nsubj` of the relative clause |
| `he that`, `he who` | generic «whoever» | «he that smiteth a man» | free relative: `he` is the head, the clause `acl:relcl` |
| `whoso`, `whosoever` | whoever | «whosoever slayeth Cain» (`jps:GEN:104`, GEN 4.15) | db: PRON, lemma lower-case 37, but capitalised lemma `Whosoever` 8 and `Whoso` 3 |
| `whatsoever` | whatever | | PRON |
| `aught` / `ought` | anything | «nor aught that is thine» (`jps:GEN:375`) | db: `aught` NOUN 12, PRON 3 — should be PRON (*anything*) |
| `naught` / `nought` | nothing | «shouldest thou therefore serve me for nought?» (`jps:GEN:866`, GEN 29.15) | db: NOUN 5, PRON 2 — PRON; the prag polarity list (`prag/src/feats.rs`) does not know it |
| `none` | no one, not any | «there is none else beside Him» (`jps:DEU:147`) | PRON with `PronType=Neg` |
| `who` for `whom` | case not marked | «Who told thee?» | Abbott §274 |
| `whom` after a preposition with object fronted | | «the man whom He had formed» (`jps:GEN:41`) | normal |

Abbott §244 also lists the **omission of the relative** in subject position («There is a fellow somewhat near the
door, he should be the man» style), which leaves a clause with no subject and no relative; it is rare in JPS.

## 3.8 Decisions Mova must make

1. **Lemma of `ye`.** `ye` → `ye` (db now, 879 tokens) or `ye` → `you` (as `thee` → `thou`). Choose one; the
   db's choice (`ye`) is defensible because `ye` is a separate nominative form, as `thou` is.
2. **`thee` → `thou`.** The db has 713 `thou` and 243 `thee` lemmas for the same form; **202 sentences** carry
   a `thee` lemma (2026-10-17). One rule fixes them all.
3. **Person on the archaic pronouns.** The db has `Person=3|Gender=Neut` on `thou` 115 times and `Person=3` on
   `thy` 20 times and `thine` 6 times — **98 sentences**. These features come from the graph parser's lexicon
   fill (chapter 12): when the parser changes the tag of a word, it copies the most frequent FEATS of that tag
   (`PRP` → the features of *it*). `prag/src/feats.rs` reads `Person` from FEATS to decide the person of the
   subject, so these 98 sentences enter the pragmatic layer as third-person sentences.
4. **`Style=Arch`.** 1 071 `thou` tokens have it, 309 do not; `thee` 164 of 956; `ye` 878 of 879. The feature
   should be on every archaic form or on none. Chapter 13 proposes «on every one», set by the lexicon, not by
   the teacher.

## 3.9 What the small model does now

`en-agreed.bin` (2026-10-15), run on the 6 105 Torah sentences (SLM now, 2026-10-17), UPOS of the archaic
pronouns:

| Form | Tokens | PRON | NOUN | VERB | ADJ | PROPN | other |
|---|---:|---:|---:|---:|---:|---:|---:|
| thou | 1 525 | 1 479 | — | — | — | 15 | ADV 16, X 15 |
| thee | 982 | 298 | **585** | **88** | — | 11 | |
| thy | 1 310 | 1 036 | — | — | **234** | 27 | DET 13 |
| thine | 117 | 4 | **97** | — | 6 | 10 | |
| ye | 859 | 803 | 43 | — | 13 | — | |

`thee` is a noun to the model 60 % of the time and `thine` 83 %. The reason is plain: UD English training data
has almost none of these words. In the 12 544 training sentences of EWT 2.18 there is one `thou`, three `thy`,
two `ye` and no `thee`, `thine`, `hath`, `doth` or `shalt`; `art` is the noun 14 times and the verb once. An
unknown lower-case word after a verb looks like an object noun. Of the 6 013 Torah
sentences aligned with the db, **821** have a UPOS error on one of these pronouns. These are not hard cases for a
lexicon: `thou`, `thee`, `thy`, `thine`, `thyself`, `ye` are never anything but pronouns in this register (the
article `ye` of printed «ye olde» does not occur). Chapter 13 proposes a closed-class override before the
tagger.
