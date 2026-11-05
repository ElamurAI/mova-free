# 20. Syntax: the Universal Dependencies frame

This part of the manual describes English syntax in the analysis Mova uses: Universal Dependencies (UD), version 2.18 with the conventions of the English Web Treebank (EWT), plus the small set of decisions recorded as Mova's own dialect. This chapter gives the frame: what a dependency tree is, the full inventory of English relations with their frequencies, the enhanced graph, the Mova dialect, and where Mova's parser stands against the guidelines. Chapters 21–29 then take English syntax construction by construction.

Contents

1. [The chapters of the syntax part](#1-the-chapters-of-the-syntax-part)
2. [Dependency grammar in one page](#2-dependency-grammar-in-one-page)
3. [The CoNLL-U record](#3-the-conll-u-record)
4. [Word classes in brief](#4-word-classes-in-brief)
5. [The English relation inventory](#5-the-english-relation-inventory)
6. [English subtypes](#6-english-subtypes)
7. [Design decisions that shape every English tree](#7-design-decisions-that-shape-every-english-tree)
8. [The enhanced graph (EUD)](#8-the-enhanced-graph-eud)
9. [The Mova dialect](#9-the-mova-dialect)
10. [Mova's parser against the guidelines](#10-movas-parser-against-the-guidelines)
11. [Other representations of English syntax](#11-other-representations-of-english-syntax)
12. [How examples are cited in this manual](#12-how-examples-are-cited-in-this-manual)

---

## 1. The chapters of the syntax part

| file | what it covers |
|---|---|
| 20-syntax-overview.md | this chapter: the UD frame, relations, EUD, the Mova dialect, the parser |
| [21-clause-structure.md](21-clause-structure.md) | the simple clause: predicate types, subjects, objects, the verb chain, copula, expletives, clause types, adjuncts |
| [22-phrases.md](22-phrases.md) | noun, adjective, adverb and prepositional phrases; names, numbers, possessives, comparatives |
| [23-complementation.md](23-complementation.md) | complements of verbs, adjectives and nouns: `ccomp`, `xcomp`, control and raising, small clauses |
| [24-coordination-subordination.md](24-coordination-subordination.md) | coordination, parataxis, adverbial clauses and subordinators |
| [25-relative-clauses.md](25-relative-clauses.md) | relative clauses, relativizers, reduced and free relatives |
| [26-questions.md](26-questions.md) | yes/no, wh-, embedded, tag and other questions; exclamatives |
| [27-negation.md](27-negation.md) | negation, its scope, polarity items |
| [28-voice-and-passive.md](28-voice-and-passive.md) | passives, middles, causatives |
| [29-ellipsis-word-order-information-structure.md](29-ellipsis-word-order-information-structure.md) | ellipsis, word order, inversion, dislocation, clefts, information structure |

The pragmatics and discourse part (files 30–38) builds on these chapters; phonology, morphology and the lexicon are in files 0x and 1x of this folder.

## 2. Dependency grammar in one page

A dependency analysis links words, not phrases. Every word but one has exactly one **head**; the word without a head is the **root**. Each link carries a **relation** that says what the dependent is to its head: subject, object, modifier, marker and so on. The idea goes back to Tesnière's structural syntax and runs through Mel'čuk's Meaning–Text Theory and the Prague tradition; a compact modern introduction for parsing is Kübler, McDonald & Nivre (2009), `books/k/ku/kubler-2009-dependency-parsing`.

(1) *The fox flattered the crow.*

```text
root(ROOT, flattered)
nsubj(flattered, fox)      det(fox, The)
obj(flattered, crow)       det(crow, the)
```

UD adds three commitments that decide most English trees (de Marneffe et al. 2021, "Universal Dependencies", *Computational Linguistics* 47(2), https://aclanthology.org/2021.cl-2.11; Nivre et al. 2016, https://aclanthology.org/L16-1262; Nivre et al. 2020, https://aclanthology.org/2020.lrec-1.497):

1. **Content words are heads; function words are their dependents.** Auxiliaries, the copula, prepositions, subordinators, determiners and the infinitival *to* hang from the content word they serve. So in *has been eaten*, the head is *eaten*; in *in the garden*, the head is *garden* and *in* is its `case`.
2. **The same relation for the same function across languages.** English *of the king* and a genitive case ending in another language both make an `nmod`; the preposition only adds `case`.
3. **A shallow, surface-oriented tree.** The basic tree is a tree (one head per word); implicit relations such as the understood subject of an infinitive go into a second, optional layer, the enhanced graph (§8).

The first commitment is debated. Osborne & Gerdes (2019, "The status of function words in dependency grammar: A critique of Universal Dependencies", *Glossa* 4(1), CC BY 4.0, `papers/o/os/osborne-2019-status-function-words-dependency-grammar`) argue that syntactic tests favour function words as heads; Surface-Syntactic UD (SUD; Gerdes et al. 2018, https://aclanthology.org/W18-6008) makes exactly that choice and converts to UD and back. Mova treats UD, SUD and other schemes as "translations" of one internal annotation ([../../docs/dialect.md](../../docs/dialect.md), section *Why*).

## 3. The CoNLL-U record

A UD sentence is stored as CoNLL-U: one word per line, ten tab-separated columns, sentence-level comments starting with `#` (UD guidelines, `format.md`; https://universaldependencies.org/format.html).

| column | name | English example (*eaten* in *has been eaten*) |
|---|---|---|
| 1 | ID | `4` (words count from 1; `3-4` for a multiword token like *don't*; `5.1` for an empty node, enhanced only) |
| 2 | FORM | `eaten` |
| 3 | LEMMA | `eat` |
| 4 | UPOS | `VERB` |
| 5 | XPOS | `VBN` (Penn Treebank tag) |
| 6 | FEATS | `Tense=Past\|VerbForm=Part\|Voice=Pass` |
| 7 | HEAD | the ID of the head, `0` for the root |
| 8 | DEPREL | `root` |
| 9 | DEPS | the enhanced graph: `0:root` (or `_`) |
| 10 | MISC | anything else: `SpaceAfter=No`, `Entity=…`, Mova's agreement traces |

A real record from EWT (2): the token *Here's* is split into two syntactic words, and DEPS carries an extra subject edge for the relative clause.

(2) *Here's everything that flows from 5/1/01 forward.* — EWT 2.18 `email-enronsent38_01-0070` (columns ID FORM LEMMA UPOS HEAD DEPREL DEPS)

```conllu
1-2  Here's      _           _      _  _          _
1    Here        here        ADV    0  root       0:root
2    's          be          AUX    1  cop        1:cop
3    everything  everything  PRON   1  nsubj      1:nsubj|5:nsubj
4    that        that        PRON   5  nsubj      3:ref
5    flows       flow        VERB   3  acl:relcl  3:acl:relcl
6    from        from        ADP    7  case       7:case
7    5/1/01      5/1/01      NUM    5  obl        5:obl:from
8    forward     forward     ADV    7  advmod     7:advmod
9    .           .           PUNCT  1  punct      1:punct
```

Mova reads and writes CoNLL-U losslessly: writing without edits gives identical bytes on all 29 files of English UD 2.18 ([../../docs/dialect.md](../../docs/dialect.md), *Lossless reading*; code in [../../en/src/conllu.rs](../../en/src/conllu.rs)).

## 4. Word classes in brief

The full treatment of English word classes and features is in the morphology chapters (1x files). Syntax needs only the seventeen universal tags (UPOS) and how English uses them; EWT also keeps the Penn Treebank tag in XPOS (Santorini 1990, *Part-of-Speech Tagging Guidelines for the Penn Treebank Project*, `books/s/sa/santorini-1990-part-speech-tagging-guidelines`).

| UPOS | English members | typical relations |
|---|---|---|
| NOUN | *fox, idea, water* | `nsubj`, `obj`, `obl`, `nmod`, `compound` |
| PROPN | *Alice, Texas* | as NOUN; `flat` inside names |
| PRON | *I, it, who, everything, that* (relative) | as NOUN; `expl` for dummy *it*, *there* |
| VERB | lexical verbs | `root`, clausal relations |
| AUX | *be, have, do, will, can…* (closed list), the copula *be* | `aux`, `aux:pass`, `cop` |
| ADJ | *green, tired* | `amod`, predicate with `cop`, `xcomp` |
| ADV | *very, quickly, not* is PART | `advmod` |
| ADP | *in, of, by* | `case`; `compound:prt` for particles |
| DET | *the, a, this, all, every* | `det`, `det:predet` |
| NUM | *five, 2026* | `nummod` |
| CCONJ | *and, or, but, nor* | `cc` |
| SCONJ | *that, if, because, while* | `mark` |
| PART | *'s*, *not / n't*, infinitival *to* (only these) | `case`, `advmod`, `mark` |
| INTJ | *oh, well, yes, um* | `discourse` |
| PUNCT, SYM, X | punctuation, symbols, foreign or garbled | `punct`, varied |

The English AUX list is closed: *be, have, do, will, would, shall, should, can, could, may, might, must, ought, need, dare, get* (the last only in the get-passive) (UD guidelines `_en/pos/AUX_.md`, https://universaldependencies.org/en/pos/AUX_.html; Mova's seed [../../en/seeds/verbal/aux-closed-list.md](../../en/seeds/verbal/aux-closed-list.md)). PART holds only possessive *'s*, negative *not/n't* and infinitival *to* (UD guidelines `_en/pos/PART.md`).

## 5. The English relation inventory

UD 2.18 defines 37 universal relations; the English registry allows 58 labels, 37 universal and 21 subtypes, of which EWT 2.18 uses 51 (it has no `clf` or `flat:foreign`, and the old `:tmod`/`:npmod` are gone) (validator data `tools/data/deprels.json`, summarised in the research library's UD digest, `docs/papers/ud/ud-core.md` §6).

### 5.1 The universal table

The universal relations are organised by the kind of dependent (nominal, clause, modifier word, function word) and the kind of link (core argument, non-core dependent, dependent of a nominal) (UD guidelines `_u/dep/index.md`; https://universaldependencies.org/u/dep/index.html).

| | nominals | clauses | modifier words | function words |
|---|---|---|---|---|
| **core arguments** | `nsubj`, `obj`, `iobj` | `csubj`, `ccomp`, `xcomp` | | |
| **non-core dependents** | `obl`, `vocative`, `expl`, `dislocated` | `advcl` | `advmod`, `discourse` | `aux`, `cop`, `mark` |
| **nominal dependents** | `nmod`, `appos`, `nummod` | `acl` | `amod` | `det`, `clf`, `case` |

| other groups | relations |
|---|---|
| coordination | `conj`, `cc` |
| headless / multiword | `fixed`, `flat`, `compound` |
| loose joining | `list`, `parataxis` |
| special | `orphan`, `goeswith`, `reparandum` |
| other | `punct`, `root`, `dep` |

### 5.2 Frequencies in English

Counts over all of EWT 2.18 (train + dev + test: 16 622 sentences, 254 820 words) and over Mova's parse database (`$MOVA_DATA/parsedb/current.conllu`: school lessons, tales, dictionary and maths sentences, 325 765 sentences, 8 062 933 words), counted on 2026-10-17 with a one-line `awk` over column 8. The database is heavy in dictionary entries, which explains its high `punct` and `parataxis` and its low `flat` and `nmod:unmarked`.

| relation | EWT | EWT % | parse db | db % | what it is | chapter |
|---|---:|---:|---:|---:|---|---|
| `punct` | 29 682 | 11.65 | 1 596 907 | 19.81 | punctuation | — |
| `case` | 21 434 | 8.41 | 644 668 | 8.00 | preposition, possessive *'s* | 22 |
| `nsubj` | 19 888 | 7.80 | 621 236 | 7.70 | nominal subject | 21 |
| `det` | 19 371 | 7.60 | 695 686 | 8.63 | determiner | 22 |
| `root` | 16 622 | 6.52 | 325 765 | 4.04 | head of the sentence | 21 |
| `advmod` | 13 980 | 5.49 | 364 741 | 4.52 | adverbial modifier, *not* | 21, 27 |
| `obj` | 12 254 | 4.81 | 371 210 | 4.60 | direct object | 21, 23 |
| `amod` | 12 158 | 4.77 | 326 856 | 4.05 | adjectival modifier | 22 |
| `obl` | 10 817 | 4.24 | 404 473 | 5.02 | oblique nominal (PP adjunct or argument) | 21 |
| `conj` | 9 395 | 3.69 | 388 411 | 4.82 | conjunct | 24 |
| `compound` | 9 047 | 3.55 | 143 174 | 1.78 | noun–noun compound | 22 |
| `mark` | 8 633 | 3.39 | 199 763 | 2.48 | subordinator, infinitival *to* | 23, 24 |
| `nmod` | 8 442 | 3.31 | 184 296 | 2.29 | nominal modifier of a noun | 22 |
| `cc` | 8 292 | 3.25 | 292 496 | 3.63 | coordinating conjunction | 24 |
| `aux` | 8 066 | 3.17 | 140 067 | 1.74 | auxiliary | 21 |
| `cop` | 5 939 | 2.33 | 155 018 | 1.92 | copula *be* | 21 |
| `advcl` | 4 527 | 1.78 | 126 968 | 1.57 | adverbial clause | 24 |
| `nmod:poss` | 4 466 | 1.75 | 143 584 | 1.78 | possessor | 22 |
| `xcomp` | 3 831 | 1.50 | 96 653 | 1.20 | open clausal complement | 23 |
| `ccomp` | 2 540 | 1.00 | 70 518 | 0.87 | clausal complement | 23 |
| `flat` | 2 501 | 0.98 | 15 498 | 0.19 | names, dates | 22 |
| `acl:relcl` | 2 341 | 0.92 | 60 843 | 0.75 | relative clause | 25 |
| `parataxis` | 2 031 | 0.80 | 209 369 | 2.60 | loosely joined clause | 24 |
| `acl` | 1 831 | 0.72 | 42 632 | 0.53 | clause on a noun | 22, 23, 25 |
| `nummod` | 1 794 | 0.70 | 54 337 | 0.67 | numeric modifier | 22 |
| `appos` | 1 691 | 0.66 | 87 220 | 1.08 | apposition | 22 |
| `aux:pass` | 1 643 | 0.64 | 50 180 | 0.62 | passive auxiliary | 28 |
| `nsubj:pass` | 1 445 | 0.57 | 42 380 | 0.53 | passive subject | 28 |
| `nmod:unmarked` | 1 360 | 0.53 | 2 559 | 0.03 | bare-noun modifier of a noun | 22 |
| `obl:unmarked` | 1 244 | 0.49 | 21 862 | 0.27 | bare-noun adverbial (*every day*) | 21 |
| `discourse` | 1 066 | 0.42 | 55 171 | 0.68 | interjection, emoticon | 21, 38 |
| `list` | 1 061 | 0.42 | 491 | 0.01 | list items | 24 |
| `compound:prt` | 907 | 0.36 | 41 176 | 0.51 | verb particle | 21 |
| `iobj` | 795 | 0.31 | 15 532 | 0.19 | indirect object | 23 |
| `expl` | 756 | 0.30 | 12 280 | 0.15 | expletive *it*, *there* | 21, 29 |
| `fixed` | 644 | 0.25 | 14 155 | 0.18 | fixed expression | 22, 24 |
| `nmod:desc` | 439 | 0.17 | 291 | 0.00 | title or descriptor in a name | 22 |
| `obl:agent` | 376 | 0.15 | 5 920 | 0.07 | passive *by*-agent | 28 |
| `csubj` | 336 | 0.13 | 16 134 | 0.20 | clausal subject | 23 |
| `nsubj:outer` | 254 | 0.10 | 35 | 0.00 | outer subject of a clausal predicate | 21 |
| `det:predet` | 226 | 0.09 | 9 584 | 0.12 | predeterminer (*all the*) | 22 |
| `vocative` | 183 | 0.07 | 7 899 | 0.10 | addressee | 21, 38 |
| `goeswith` | 172 | 0.07 | 55 | 0.00 | parts of a split word | — |
| `advcl:relcl` | 141 | 0.06 | 473 | 0.01 | sentential relative | 24, 25 |
| `cc:preconj` | 98 | 0.04 | 1 135 | 0.01 | *both, either, neither* | 24 |
| `reparandum` | 49 | 0.02 | 12 | 0.00 | disfluency, self-repair | 37 |
| `orphan` | 29 | 0.01 | 22 | 0.00 | remnant in gapping | 29 |
| `csubj:pass` | 8 | 0.00 | 381 | 0.00 | passive clausal subject | 28 |
| `dislocated` | 7 | 0.00 | 329 | 0.00 | dislocated element | 29 |
| `dep` | 5 | 0.00 | 1 206 | 0.01 | unknown dependency | — |
| `csubj:outer` | 3 | 0.00 | 1 | 0.00 | outer clausal subject | 21 |

Two differences between the columns are informative rather than accidental:
- **`nsubj:outer`** (254 in EWT, 35 in the database): Mova's parser rarely predicts it, and the school sentences that need it are often misparsed (see 21 §5.4 for *The problem is that the door is locked*).
- **`nmod:unmarked`** (*Dallas, Texas*, *$5 a day*): the parser prefers `conj` or `appos` for *Dallas, Texas* (22 §2.9).

## 6. English subtypes

A subtype narrows a universal relation for a language (UD guidelines `_en/dep/index.md`; https://universaldependencies.org/en/dep/). English has these:

| subtype | of | meaning | example | chapter |
|---|---|---|---|---|
| `nsubj:pass` | `nsubj` | subject of a passive clause | *The cake **was eaten*** | 28 |
| `csubj:pass` | `csubj` | clausal subject of a passive | *That he lied was **proven*** | 28 |
| `aux:pass` | `aux` | passive auxiliary *be*/*get* | *was eaten* | 28 |
| `obl:agent` | `obl` | the *by*-agent of a passive | *eaten **by the fox*** | 28 |
| `nsubj:outer` | `nsubj` | outer subject when the predicate is a clause | ***The problem** is that…* | 21 |
| `csubj:outer` | `csubj` | outer clausal subject | ***What he said** is that…* | 21 |
| `acl:relcl` | `acl` | relative clause | *the book **that I read*** | 25 |
| `advcl:relcl` | `advcl` | sentential (clause-level) relative | *…, **which surprised us*** | 24, 25 |
| `nmod:poss` | `nmod` | possessor (*'s*, possessive pronouns) | ***her** cat*, ***the man's** beard* | 22 |
| `nmod:unmarked` | `nmod` | bare noun modifying a noun | *Dallas, **Texas***; *$5 **a day*** | 22 |
| `obl:unmarked` | `obl` | bare noun as adverbial | *came **last week*** | 21 |
| `nmod:desc` | `nmod` | title or descriptor before a name | ***Dr.** Smith* | 22 |
| `det:predet` | `det` | predeterminer | ***all** the pigs* | 22 |
| `cc:preconj` | `cc` | first part of a correlative | ***both** X and Y* | 24 |
| `compound:prt` | `compound` | verb particle | *sit **down**, give **up*** | 21 |
| `flat:foreign` | `flat` | foreign phrase | (not used in EWT 2.18) | — |

Two subtypes are recent. Since UD 2.15 the old `obl:tmod`/`obl:npmod` and `nmod:tmod`/`nmod:npmod` have been merged into `:unmarked`, because `tmod` encoded meaning (time) rather than syntax (UD issue #1028; UD guidelines `_en/dep/obl-unmarked.md`). EWT keeps the old distinction as `TemporalNPAdjunct=Yes` in MISC (1 077 tokens), and the Mova dialect keeps that MISC attribute so that converters to older treebanks can restore `:tmod` ([../../treebanks/en/ewt/seeds.md#mova-diff](../../treebanks/en/ewt/seeds.md#mova-diff)).

## 7. Design decisions that shape every English tree

These decisions recur in every chapter. They are stated once here; the chapters give the details.

| decision | consequence in English | where |
|---|---|---|
| copula is not the head | in *Alice was tired*, *tired* is the root, *was* is `cop`; only *be* is a copula | 21 §4 |
| auxiliaries are leaves | *will have been watching*: three `aux` on *watching* | 21 §3 |
| prepositions are `case` | *in the hall* → `case(hall, in)`; the PP attaches through its noun (`obl` or `nmod`) | 22 §5 |
| `obl` on predicates, `nmod` on nouns | *sat **on the mat*** (`obl`) vs *the cat **on the mat*** (`nmod`); the validator checks the boundary since UD 2.18 | 21, 22 |
| first conjunct is the head | in *tea and coffee*, *coffee* is `conj` of *tea*, *and* is `cc` of *coffee* | 24 |
| subordinators are `mark` of the clause's head | *because she wanted the cheese* → `mark(wanted, because)` | 24 |
| relative pronoun takes its role inside the clause | *the book **that** I bought*: `obj(bought, that)` | 25 |
| no empty nodes in the basic tree | gapping uses `orphan`; the enhanced graph adds an empty node | 29 |
| an auxiliary becomes the head when its verb is elided | *John did* → `did` is the head | 29 |
| clauses without a subject are open (`xcomp`) only if the subject is controlled | *wants **to buy*** → `xcomp`; *said **that she left*** → `ccomp` | 23 |

The relative order of authorities matters, because the English guideline pages are not all current. The research library's digest of the UD documentation recommends `changes.md` first, then the pages of individual labels, then the language overview: `_en/specific-syntax.md` still shows the pre-2.10 analysis of *The problem is that…* (copula as head with `ccomp`), which the 2.10 amendment replaced by `nsubj:outer` (`docs/papers/ud/ud-core.md` §6.1 in the research library; UD `docs/changes.md`, *multiple subjects*). Mova's seeds record each such conflict and the choice made ([../../dialects/mova/seeds.md#clausal-predicate](../../dialects/mova/seeds.md#clausal-predicate)).

## 8. The enhanced graph (EUD)

### 8.1 What it adds

The basic tree is deliberately shallow. The enhanced graph (DEPS column) makes implicit relations explicit and specialises labels; it descends from the collapsed and cc-processed Stanford Dependencies (de Marneffe & Manning 2008, *Stanford typed dependencies manual*, `books/m/ma/marneffe-2008-stanford-typed-dependencies-manual`) and from Schuster & Manning (2016, "Enhanced English Universal Dependencies: An Improved Representation for Natural Language Understanding Tasks", https://aclanthology.org/L16-1376). The UD guidelines name six kinds of enhancement (UD guidelines `_u-overview/enhanced-syntax.md`; https://universaldependencies.org/u/overview/enhanced-syntax.html):

| # | enhancement | English example | enhanced edges |
|---|---|---|---|
| 1 | empty nodes for elided predicates | *Sue likes coffee and Bill tea* | node `5.1` (*likes*) with `nsubj(5.1, Bill)`, `obj(5.1, tea)`; replaces `orphan` |
| 2 | incoming edges propagated to conjuncts | *Is the distinction accurate and valid?* | `nsubj(valid, distinction)` beside `nsubj(accurate, distinction)` |
| 3 | outgoing edges shared by conjuncts | *Transwestern will own and operate the interconnect* | `nsubj(operate, Transwestern)`, `obj(own, interconnect)` |
| 4 | subjects of controlled and raised clauses | *Attached please find the GE Guarantee* | `nsubj:xsubj(Attached, Guarantee)` |
| 5 | relative clauses | *everything that flows* | `ref(everything, that)`, `nsubj(flows, everything)` |
| 6 | case information in labels | *from 5/1/01* | `obl:from`, `nmod:of`, `conj:and`, `advcl:if` |

(3) EWT 2.18 `email-enronsent43_01-0005` — a shared subject and a shared object; the auxiliary *will* is not copied to *operate*:

```conllu
1  Transwestern  PROPN  3  nsubj  3:nsubj|5:nsubj
2  will          AUX    3  aux    3:aux
3  own           VERB   0  root   0:root
4  and           CCONJ  5  cc     5:cc
5  operate       VERB   3  conj   3:conj:and
6  the           DET    7  det    7:det
7  interconnect  NOUN   3  obj    3:obj|5:obj
8  .             PUNCT  3  punct  3:punct
```

(4) EWT 2.18 `email-enronsent26_02-0019` — the controlled participle *Attached* gets the object of *find* as its subject:

```conllu
1  Attached   VERB  3  xcomp      3:xcomp
2  please     INTJ  3  discourse  3:discourse
3  find       VERB  0  root       0:root
6  Guarantee  NOUN  3  obj        1:nsubj:xsubj|3:obj
```

### 8.2 Labels

An enhanced label has up to four parts: `universal[:subtype][:marker lemma][:case]`, or `…:xsubj`. The marker is the lowercased lemma of the `case` or `mark` dependent, with the words of a `fixed` expression joined by `_` (`obl:as_opposed_to`) (UD guidelines `_u-overview/enhanced-syntax.md`). The most frequent lexicalised labels in EWT 2.18 (DEPS, all splits, counted 2026-10-17):

| label | count | label | count | label | count |
|---|---:|---|---:|---|---:|
| `conj:and` | 6 716 | `obl:for` | 1 400 | `advcl:if` | 787 |
| `nmod:of` | 3 985 | `obl:with` | 1 077 | `nmod:for` | 766 |
| `nsubj:xsubj` | 3 713 | `obl:on` | 915 | `conj:but` | 732 |
| `obl:in` | 2 189 | `advcl:to` | 915 | `obl:at` | 651 |
| `obl:to` | 1 641 | `conj:or` | 898 | `acl:to` | 621 |

EWT 2.18 has 43 empty nodes and 1 514 `ref` edges in all. EWT, GUM and PUD carry all six enhancement types; CHILDES, GENTLE and GUMReddit carry all but empty nodes (`docs/papers/ud/ud-core.md` §4 in the research library, after UD `docs/survey-enhanced.md`).

### 8.3 What EUD does not settle

- **Enhanced++** (Schuster & Manning 2016) goes further — partitives and light nouns as `det:qmod` (*a lot of people*), multiword prepositions, copied nodes for coordinated prepositions — but is not part of the standard and should not appear in releases (UD guidelines `_u-overview/enhanced-syntax.md`).
- **Distributive vs collective** readings of shared dependents are not distinguished, and object sharing in coordinated verb phrases is not propagated when ambiguous (*she was reading or watching a movie*) (Schuster & Manning 2016).
- **How much of EUD follows from the basic tree.** Ek & Bernardy (2020, "How Much of Enhanced UD Is Contained in UD?", https://aclanthology.org/2020.iwpt-1.23, `papers/e/ek/ek-2020-how-much-enhanced-ud-is`) rebuild EUD from gold basic trees with patterns at ELAS 81.55 and EULAS 96.70: the labels are mostly predictable; the extra edges less so. At the IWPT 2020 shared task the best system reached ELAS 84.5 over 17 languages (Bouma et al. 2020, https://aclanthology.org/2020.iwpt-1.17).

### 8.4 EUD in Mova

Mova's parser writes `_` in DEPS: it produces no enhanced graph ([../../en/src/annotate.rs](../../en/src/annotate.rs), the CoNLL-U writer). The *meaning* of four enhancements is recovered later, in the pragmatics reader, as roles of frames rather than as graph edges ([../../prag/src/frames.rs](../../prag/src/frames.rs)):

| enhancement | what `prag::frames` does instead |
|---|---|
| 2, 3 — shared dependents of conjuncts | a `conj`, `xcomp`, `advcl` or `ccomp` predicate with no subject of its own takes the agent of the predicate above it ("a clause without its own subject shares the head's") |
| 4 — controlled subjects | subject control: the agent of the head; object control (*make X grow*, *tell X to go*): the head's patient becomes the agent |
| 5 — relative clauses | a relative pronoun stands for the noun its clause belongs to; the noun fills the clause's missing role |
| 6 — markers in labels | the preposition is kept with every oblique (`obls: with=queen`) and decides VerbNet role matching |

Empty nodes (enhancement 1) have no counterpart: a gapped clause gets no frame of its own (29 §2). Writing a real DEPS column, for export and for treebank conversion, is not done yet.

## 9. The Mova dialect

### 9.1 What it is

Mova annotates in its own UD dialect, `mova`: **UD 2.18 with EWT conventions**, plus changes that will come in UD 2.19 (`ud-next`), plus a few borrowings and normalisations, each recorded as a seed with its reason and evidence. Converters run in both directions, so `mova` output can be exported to EWT 2.18, older UD, or other treebanks' conventions ([../../docs/dialect.md](../../docs/dialect.md)). The selection rule is: take whatever is more consistent and more informative *and* converts to the standard without loss; a lossy target is only an export.

### 9.2 Decisions that touch English syntax

| seed | decision | why, in short |
|---|---|---|
| [clausal-predicate](../../dialects/mova/seeds.md#clausal-predicate) | *The problem is that…*: predicate head, `cop`, `nsubj:outer` (not the old copula-head + `ccomp`) | 2.10 amendment; EWT has 257 outer subjects and none of the old analysis |
| [depictives](../../dialects/mova/seeds.md#depictives) | *She entered the room sad*: `advcl` of the verb | 2.10 amendment; the English overview contradicts itself (`acl` vs `advmod`) |
| [discourse-adverbs](../../dialects/mova/seeds.md#discourse-adverbs) | `discourse` only for INTJ, numbering, emoticons; adverbs are `advmod` | the criterion is the word class; 2.19 definition |
| [iobj-sole](../../dialects/mova/seeds.md#iobj-sole) | `iobj` may stand alone (*remind me*) and next to `xcomp` (*ask X to go*); `obj` with *make/let/help X do* | 2.12 amendment; keeps addressee vs causee |
| [to-phrase-obl](../../dialects/mova/seeds.md#to-phrase-obl) | a *to*-phrase on a verb is `obl` | EWT 1 285 vs 0; the `nmod` in `_en/dep/iobj.md` is a v1 leftover |
| [imperative-do](../../dialects/mova/seeds.md#imperative-do) | *Don't go*: *go* has `Mood=Imp\|VerbForm=Fin` | the directive is visible on the lexical verb; 98 of 106 EWT cases |
| [subjunctive](../../dialects/mova/seeds.md#subjunctive) | *insist that he **go***, *if I **were***: `Mood=Sub` | a clause-level feature for meaning; lossless export |
| [modal-mood](../../dialects/mova/seeds.md#modal-mood) | modals keep `VerbForm=Fin` with no `Mood` | wait for UD issues #1118, #1155 |
| [wh-case](../../dialects/mova/seeds.md#wh-case) | *who/whom/whose* get `Case`; lemma *who* for *whom* | case shows the gap role in a relative clause |
| [spoken-subtypes](../../dialects/mova/seeds.md#spoken-subtypes) | no `discourse:filler`, `discourse:tag`, `conj:reform` yet | wait for the UniDive spoken-language guidelines |
| [features-context](../../dialects/mova/seeds.md#features-context) | features from context, but the form wins (*the team **are*** stays plural) | clarification No. 18 of `ud-next` |

Three normalisations are applied to all silver data: `det:poss` → `nmod:poss`; `:tmod`/`:npmod` → `:unmarked`; Number and Person of finite verbs from the subject (`annot::ann::ewt_rel`, `en::ud::agreement`; [../../docs/dialect.md](../../docs/dialect.md)). On EWT 2.18 each of them changes nothing; they exist for the teacher's annotations and for other treebanks ([../../treebanks/en/ewt/seeds.md#mova-diff](../../treebanks/en/ewt/seeds.md#mova-diff)).

### 9.3 Seeds as checkable grammar

Mova's English grammar is also written as **seeds**: short markdown pages under [../../en/seeds/](../../en/seeds/) (`verbal/`, `nominal/`, `lexicon/`, `errors/`, `morph/`), each with a description, examples and, where possible, a machine-checkable rule in the seed rule language ([../../docs/english.md](../../docs/english.md); engine [../../en/src/expert.rs](../../en/src/expert.rs)). A rule matches nodes of a tree and states requirements; a violation is reported with the rule's id. Examples relevant to syntax:

| seed | what it checks |
|---|---|
| [verbal/single-subject.md](../../en/seeds/verbal/single-subject.md) | at most one `nsubj`/`csubj` per predicate (outer subjects aside) |
| [verbal/copula-be-only.md](../../en/seeds/verbal/copula-be-only.md) | `cop` is always a form of *be* |
| [verbal/aux-flat-leaf.md](../../en/seeds/verbal/aux-flat-leaf.md) | auxiliaries have no dependents except in ellipsis |
| [verbal/open-complement-xcomp.md](../../en/seeds/verbal/open-complement-xcomp.md) | an `xcomp` has no subject of its own |
| [verbal/existential-there.md](../../en/seeds/verbal/existential-there.md) | *there* in existentials is `expl` |
| [nominal/relcl-after-head.md](../../en/seeds/nominal/relcl-after-head.md) | a relative clause follows its noun |
| [errors/function-words-leaves.md](../../en/seeds/errors/function-words-leaves.md) | function words do not take dependents |

The seeds are counters as well as gates: run over EWT, a general rule must give zero or a small residue, otherwise the rule itself is wrong (`treebanks/en/seeds-overview.md`).

## 10. Mova's parser against the guidelines

### 10.1 The parsers

- **`en`** (CPU, the default): an arc-standard transition parser with a beam of 8, averaged-perceptron tables, a separate step for the label; tagger and lemmatiser in the same binary ([../../en/src/parse.rs](../../en/src/parse.rs); after Nivre 2004, Zhang & Clark 2008, https://aclanthology.org/D08-1059, Zhang & Nivre 2011, https://aclanthology.org/P11-2033).
- **`gparse`** (GPU): a transformer encoder trained from scratch, biaffine heads, Eisner decoding ([../../gparse](../../gparse)).
- **Training data**: EWT and ESLSpok plus the school trees agreed by rules over several witnesses (Stanza, Trankit, spaCy) and checked by a tiered teacher ([../../docs/architecture.md](../../docs/architecture.md); [../../tools/treeagree](../../tools/treeagree)).

Measured on 2026-10-15 ([../../docs/architecture.md](../../docs/architecture.md)): the GPU parser trained on the agreed school reaches school LAS 88.5 and EWT LAS 85.7; the perceptron trained on the same data 84.5 and 80.7.

### 10.2 Where it departs from UD

Mova's parser was run on the examples of these chapters (model `en.bin` of release 0.4, CPU mode, 2026-10-17). Its systematic departures, each discussed in its chapter:

| construction | UD (EWT) analysis | Mova en 0.4 | chapter |
|---|---|---|---|
| *What did the cook throw at the baby?* | *throw* root, *did* `aux`, *What* `obj` | *did* root (VERB), *throw* `ccomp`, *What* `nsubj` | 26 |
| *John likes tea and Mary coffee.* | `conj(likes, Mary)`, `orphan(Mary, coffee)` | `compound(coffee, Mary)`, `conj(tea, coffee)` | 29 |
| *It is easy to please John.* | `expl(easy, It)`, `csubj(easy, please)` | `expl` + `xcomp(easy, please)` | 23, 29 |
| *The problem is that the door is locked.* | `nsubj:outer(locked, problem)`, `nsubj:pass(locked, door)` | two `nsubj:pass` on *locked* | 21 |
| *That the Queen was angry surprised nobody.* | `csubj(surprised, angry)`, `mark(angry, That)` | *surprised* as ADJ root, *That* `nsubj` | 21, 23 |
| *We elected her president.* | `obj(elected, her)`, `xcomp(elected, president)` | `nmod:poss(president, her)` | 21, 23 |
| *Sit down and listen to me, Alice!* | `vocative(Sit, Alice)` | `obl(Sit, Alice)` | 21 |
| *Whom were you talking to?* | `case(Whom, to)`, `obl(talking, Whom)` | `fixed(talking, to)` | 25, 26 |
| *Both my brothers live in Dallas, Texas.* | `nmod:unmarked(Dallas, Texas)` | `conj(Dallas, Texas)` | 22 |

Two of these are handled downstream: questions with do-support are put into statement order before parsing in the pragmatics reader (*What did the cook throw?* → *the cook threw what?*; [../../docs/pragmatics.md](../../docs/pragmatics.md), *Questions*), and tree-repair rules learned from errors (`world induce`, [../../docs/training.md](../../docs/training.md)) and the absurdity-matrix reranker fix some attachments before frames are read.

## 11. Other representations of English syntax

UD is one of several analyses Mova can convert to or compare against ([../../docs/dialect.md](../../docs/dialect.md), *Not just UD*):

| scheme | key idea | English resource | in Mova |
|---|---|---|---|
| Penn Treebank constituency | phrase-structure brackets with function tags and traces (Bies et al. 1995, `books/b/bi/bies-1995-bracketing-guidelines-treebank-ii`) | PTB WSJ (LDC, closed) | card in [../../treebanks/en/ptb](../../treebanks/en/ptb); constituency parsers as witnesses ([../../docs/parsers.md](../../docs/parsers.md)) |
| Stanford Dependencies | the ancestor of UD; prepositions collapsed into labels (de Marneffe & Manning 2008) | converted PTB | `dialects/` target planned |
| SUD | function words as heads, convertible to UD (Gerdes et al. 2018) | SUD versions of UD treebanks | planned converter |
| HPSG (ERG) | lexicalised feature structures and MRS semantics | ACE + ERG 2018 | witness, candidate for transparent rules ([../../docs/parsers.md](../../docs/parsers.md)) |
| CCG | categories and combinators | CCGbank (LDC) | EasyCCG, depccg as tests |
| Link Grammar | typed links from a hand-written dictionary | Link Grammar 5.12.5 | full linkage for 43.5% of school sentences (2026-10-15) |
| AMR / UMR | meaning graphs | AMR releases (LDC) | the frames of `prag` play this role |

The UD typology review by Croft & Nivre (2025, "Verbal Predication Constructions in Universal Dependencies", https://aclanthology.org/2025.cxgsnlp-1.6, `papers/c/cr/croft-2025-verbal-predication-constructions-universal-dependencies`) finds UD able to represent all major verbal predication constructions, but not fully coherent in its criteria for core arguments and in voice subtypes; several of its proposals bear on English `iobj` and passive analysis (23, 28).

## 12. How examples are cited in this manual

| tag | meaning | licence |
|---|---|---|
| `EWT 2.18 <sent_id>` | gold tree from UD English EWT 2.18 | CC BY-SA 4.0: quoted, never shipped as data |
| `GUM 2.18 <sent_id>` | gold tree from UD English GUM | CC BY-NC-SA 4.0: short fragments only, for illustration |
| `school:<course>:<lesson>:<n>` | Mova's school lesson sentence in the parse database (`$MOVA_DATA/parsedb/current.conllu`), agreed silver tree | ours |
| `book:pg<N>:<n>` | a sentence of a Project Gutenberg tale in the parse database | text public domain |
| `jps:<BOOK>:<n>` | the Torah in the JPS 1917 translation, Mova's tree (`$MOVA_DATA/corpora/judaism/parsedb/current.conllu`) | public domain |
| «Mova en 0.4, CPU, 2026-10-17» | output of `en annotate` with the release 0.4 model on a sentence written for this manual | ours |
| unmarked | constructed example | ours |

Arcs are written `relation(head, dependent)`. CoNLL-U excerpts drop columns that do not matter for the point; the column set is given above each excerpt.

## In Mova

- CoNLL-U reading and writing: [../../en/src/conllu.rs](../../en/src/conllu.rs); the annotator: [../../en/src/annotate.rs](../../en/src/annotate.rs); the parser: [../../en/src/parse.rs](../../en/src/parse.rs); UD features and metrics: [../../en/src/ud.rs](../../en/src/ud.rs); the relation registry: [../../en/src/udreg.rs](../../en/src/udreg.rs) and `en/data/ud-registry-en.tsv`.
- The dialect and converters: [../../docs/dialect.md](../../docs/dialect.md), [../../en/src/convert.rs](../../en/src/convert.rs), [../../dialects/](../../dialects/), [../../treebanks/en/](../../treebanks/en/).
- Seeds and the rule engine: [../../en/seeds/](../../en/seeds/), [../../en/src/expert.rs](../../en/src/expert.rs).
- Trees to frames (where enhanced information is used as meaning): [../../prag/src/frames.rs](../../prag/src/frames.rs).
- **Not yet**: no DEPS output; no empty nodes; no `TemporalNPAdjunct` prediction (the converter to `:tmod` treebanks guesses by lemma); `nsubj:outer` is rarely predicted.

## Sources

- de Marneffe, M.-C., Manning, C. D., Nivre, J., Zeman, D. 2021. Universal Dependencies. *Computational Linguistics* 47(2). https://aclanthology.org/2021.cl-2.11
- Nivre, J. et al. 2016. Universal Dependencies v1: A Multilingual Treebank Collection. LREC. https://aclanthology.org/L16-1262
- Nivre, J. et al. 2020. Universal Dependencies v2: An Evergrowing Multilingual Treebank Collection. LREC. https://aclanthology.org/2020.lrec-1.497
- Schuster, S., Manning, C. D. 2016. Enhanced English Universal Dependencies: An Improved Representation for Natural Language Understanding Tasks. LREC. https://aclanthology.org/L16-1376
- de Marneffe, M.-C., Manning, C. D. 2008. *Stanford typed dependencies manual*. `books/m/ma/marneffe-2008-stanford-typed-dependencies-manual`
- Universal Dependencies contributors. 2026. *Universal Dependencies guidelines* (snapshot 2026-09-24): `_u/dep/index.md`, `_u-overview/enhanced-syntax.md`, `_en/dep/*.md`, `_en/specific-syntax.md`, `changes.md`. https://universaldependencies.org/guidelines.html; `books/c/co/contributors-2026-universal-dependencies-guidelines`
- Silveira, N. et al. 2014. A Gold Standard Dependency Corpus for English. LREC. https://aclanthology.org/L14-1067
- Kübler, S., McDonald, R., Nivre, J. 2009. *Dependency Parsing*. `books/k/ku/kubler-2009-dependency-parsing`
- Osborne, T., Gerdes, K. 2019. The status of function words in dependency grammar: A critique of Universal Dependencies (UD). *Glossa* 4(1). https://doi.org/10.5334/gjgl.537; `papers/o/os/osborne-2019-status-function-words-dependency-grammar`
- Gerdes, K., Guillaume, B., Kahane, S., Perrier, G. 2018. SUD or Surface-Syntactic Universal Dependencies: An annotation scheme near-isomorphic to UD. UDW. https://aclanthology.org/W18-6008
- Ek, A., Bernardy, J.-P. 2020. How Much of Enhanced UD Is Contained in UD? IWPT. https://aclanthology.org/2020.iwpt-1.23; `papers/e/ek/ek-2020-how-much-enhanced-ud-is`
- Bouma, G. et al. 2020. Overview of the IWPT 2020 Shared Task on Parsing into Enhanced Universal Dependencies. https://aclanthology.org/2020.iwpt-1.17
- Croft, W., Nivre, J. 2025. Verbal Predication Constructions in Universal Dependencies. https://aclanthology.org/2025.cxgsnlp-1.6; `papers/c/cr/croft-2025-verbal-predication-constructions-universal-dependencies`
- Bies, A. et al. 1995. *Bracketing Guidelines for Treebank II Style*. `books/b/bi/bies-1995-bracketing-guidelines-treebank-ii`
- Santorini, B. 1990. *Part-of-Speech Tagging Guidelines for the Penn Treebank Project*. `books/s/sa/santorini-1990-part-speech-tagging-guidelines`
- Zhang, Y., Clark, S. 2008. A Tale of Two Parsers. EMNLP. https://aclanthology.org/D08-1059
- Zhang, Y., Nivre, J. 2011. Transition-based Dependency Parsing with Rich Non-local Features. ACL. https://aclanthology.org/P11-2033
- Research library digest: `docs/papers/ud/ud-core.md` (UD principles, versions, EUD, English registry), 2026-09-25.
- Repo: [../../docs/dialect.md](../../docs/dialect.md), [../../docs/architecture.md](../../docs/architecture.md), [../../docs/parsers.md](../../docs/parsers.md), [../../docs/pragmatics.md](../../docs/pragmatics.md), [../../dialects/mova/seeds.md](../../dialects/mova/seeds.md), [../../treebanks/en/ewt/seeds.md#mova-diff](../../treebanks/en/ewt/seeds.md#mova-diff).
