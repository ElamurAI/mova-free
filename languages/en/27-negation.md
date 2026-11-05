# 27. Negation

English negates a clause with one small word, *not* (or its clitic *n't*), placed after the first auxiliary — and when there is no auxiliary, it calls in *do* to carry it. Around this core sit negative words (*no, nobody, nothing, never, neither, nor*), affixes (*un-, in-, -less*), polarity-sensitive words (*any, ever, yet, at all*) and a long tail of pragmatic effects (litotes, negative questions, negative raising). This chapter describes the forms, their analysis in Universal Dependencies (UD), what the corpora show, and how far Mova reads them today.

Contents:

1. Sentential negation: *not* and *n't*
2. Do-support and the auxiliary chain
3. Contractions and tokenization
4. Negation in non-finite clauses and imperatives
5. Negative words: determiners, pronouns, adverbs, connectives
6. Constituent (special) negation and the scope of *not*
7. Focus and contrastive negation
8. Negative polarity items and their licensers
9. Negative raising
10. Affixal negation
11. Double negation, negative concord, litotes
12. Negation in ellipsis and fragments
13. Jespersen's cycle: how English got here
14. Quick reference: negation in UD
15. In Mova
16. Sources

Related chapters: questions with negation in [26-questions.md](26-questions.md); ellipsis in [29-ellipsis-word-order-information-structure.md](29-ellipsis-word-order-information-structure.md); the pragmatics of negation (litotes, presupposition, scalar inferences) in [33-implicature-and-presupposition.md](33-implicature-and-presupposition.md); polarity and evaluation in [30-pragmatics-overview.md](30-pragmatics-overview.md).

---

## 1. Sentential negation: *not* and *n't*

### 1.1 The basic pattern

A finite clause is negated by placing *not* directly after the **first auxiliary** of the verb chain (Brown 1851, Part II, ch. VI, "Form of Negation"; Jespersen 1917, ch. 11). The auxiliary may be a modal, perfect *have*, progressive or passive *be*, the copula *be*, or dummy *do*:

(1) a. *Kennedy has **not** been killed.*
    b. *She will **not** go.*
    c. *The bird is **not** singing.*
    d. *The cake was **not** eaten.*
    e. *The fox is **not** hungry.* (copula)
    f. *I do **not** know.* (do-support, §2)

The order is fixed: *\*She not will go*, *\*She will go not* are ungrammatical in present-day English (but see archaic post-verbal *not* in §13).

### 1.2 Category and attachment in UD

| property | value | source |
|---|---|---|
| UPOS | `PART` | UD guidelines, `_en/pos/PART.md` |
| XPOS | `RB` | Santorini 1990; EWT practice |
| FEATS | `Polarity=Neg` | UD guidelines, `_en/feat/Polarity.md` |
| lemma | *not* (also for *n't*, *nt*, *n’t*) | EWT practice |
| relation | `advmod` | UD guidelines, `_en/dep/advmod.md` |
| head | the **predicate** (lexical verb, or the non-verbal predicate of a copula clause), not the auxiliary | `_en/pos/PART.md`; seed [`negation-not.md`](../../en/seeds/verbal/negation-not.md) |

The UD convention keeps function words as leaves: the auxiliary, the copula and *not* all hang from the content predicate. So in (1a) the tree is:

```
1  Kennedy  Kennedy  PROPN  5  nsubj:pass
2  has      have     AUX    5  aux
3  not      not      PART   5  advmod
4  been     be       AUX    5  aux:pass
5  killed   kill     VERB   0  root
```

Here *has*, *not* and *been* are all siblings under the root *killed*: `aux(killed, has)`, `advmod(killed, not)`, `aux:pass(killed, been)`.

The validator would tolerate negation on a function word, but English treebanks attach it to the predicate (seed [`negation-not.md`](../../en/seeds/verbal/negation-not.md), rule `en.verbal.negation-on-predicate`).

In a copula clause the predicate is the non-verbal word:

(2) *Yea, Dr. Mann ain't so great.* — EWT 2.18 `reviews-234261-0002`

```
4  Mann   Mann   PROPN  8  nsubj
5  ai     be     AUX    8  cop
6  n't    not    PART   8  advmod
7  so     so     ADV    8  advmod
8  great  great  ADJ    0  root
```

Note that *ain't* is split *ai* + *n't*, with *ai* lemmatized as *be* (§3).

### 1.3 Frequency

UD English EWT 2.18 (all three parts, 254 820 words; counted 2026-10-17) has these `Polarity=Neg` tokens:

| form | UPOS | relation | count |
|---|---|---|---:|
| *not* | PART | advmod | 1 196 |
| *n't* | PART | advmod | 826 |
| *nt* (in *dont, cant*) | PART | advmod | 94 |
| *no* | INTJ | discourse | 41 |
| *not* | PART | conj | 21 |
| *nor* | CCONJ | cc | 20 |
| *not* | PART | advcl | 11 |
| *no* | INTJ | root | 11 |
| *n’t* (curly apostrophe) | PART | advmod | 10 |
| *not* | PART | fixed | 10 |
| *not* | PART | cc | 7 |
| *neither* | CCONJ | cc:preconj | 7 |
| *not* | PART | root | 3 |

The negative determiners, pronouns and adverbs (*no, nothing, never*) do **not** carry `Polarity=Neg` in EWT; they carry `PronType=Neg` (§5). Mova's parse database (`$MOVA_DATA/parsedb/current.conllu`, 2026-10-17, 8.06 M words of school lessons, tales, dictionaries and maths) has 53 317 `Polarity=Neg` tokens, 6.6 per thousand words, against 8.8 per thousand in EWT — the web English of EWT negates more than the lessons and tales.

## 2. Do-support and the auxiliary chain

When the clause has no auxiliary, *not* cannot attach after a lexical verb; dummy *do* is inserted, carrying tense and agreement, and the lexical verb appears in the base form (Jespersen 1917, ch. 11; Huddleston & Pullum 2002, ch. 9 — wanted, see Sources):

(3) a. *The fox caught the hen.* → *The fox **did not** catch the hen.*
    b. *She knows.* → *She **doesn't** know.*

In UD, *do* is `AUX` with relation `aux`; the lexical verb is `VB` with `VerbForm=Inf`; *not* is `advmod` of the lexical verb:

(4) *I can't tell you how ominous I found Bush's performance in that interview.* — EWT 2.18 `weblog-juancole.com_juancole_20040708181175_ENG_20040708_181175-0027`

```
1    I     I     PRON  4  nsubj
2-3  can't _     _     _  _
2    ca    can   AUX   4  aux
3    n't   not   PART  4  advmod
4    tell  tell  VERB  0  root
5    you   you   PRON  4  iobj
```

Do-support is the same mechanism that builds questions (*Did you see it?*, [26-questions.md](26-questions.md)) and emphatic affirmatives (*I **did** see it*). Three lexical verbs historically avoid it:

| verb | without *do* (older, formal, British) | with *do* (usual) |
|---|---|---|
| *have* (possession) | *I haven't a clue.* | *I don't have a clue.* |
| *need* | *You needn't come.* (modal) | *You don't need to come.* (lexical) |
| *dare* | *He daren't say.* | *He doesn't dare (to) say.* |

The modal uses of *need* and *dare* are `AUX` (seed [`need-dare.md`](../../en/seeds/verbal/need-dare.md)).

## 3. Contractions and tokenization

### 3.1 The *n't* forms

*n't* is an enclitic on the auxiliary; with *will*, *shall*, *can* and *do* it changes the host (Jespersen 1917, ch. 11 "English verbal forms in n't" treats the history and pronunciation in detail). UD English splits every such word into a **multiword token** (MWT) of two syntactic words: the auxiliary and *n't*.

| written | words | lemmas | EWT 2.18 count (as MWT) |
|---|---|---|---:|
| *don't* | *do* + *n't* | do, not | 345 |
| *didn't* | *did* + *n't* | do, not | 91 |
| *can't* | *ca* + *n't* | can, not | 87 |
| *doesn't* | *does* + *n't* | do, not | 54 |
| *dont* (no apostrophe) | *do* + *nt* | do, not | 44 |
| *won't* | *wo* + *n't* | will, not | 42 |
| *wouldn't* | *would* + *n't* | would, not | 40 |
| *isn't* | *is* + *n't* | be, not | 34 |
| *haven't* | *have* + *n't* | have, not | 34 |
| *ain't* | *ai* + *n't* | be (or have), not | — |
| *cannot* | *can* + *not* | can, not | — |

The split is at the PTB boundary: *ca|n't*, *wo|n't* (Bies et al. 1995, the Treebank II bracketing guidelines, `books/b/bi/bies-1995-bracketing-guidelines-treebank-ii`). *Cannot* is written as one word but is also two syntactic words in EWT (*can* + *not*).

(5) *I and won't have …* — EWT 2.18: `17-18 won't` → `17 wo will AUX aux`, `18 n't not PART advmod`.

### 3.2 Mova

Mova's tokenizer (`en`, release model 0.4) splits these forms the same way. «I can't tell you.» comes out as *ca* (lemma *can*, `aux`) + *n't* (lemma *not*, `advmod`) + *tell* (root) — the EWT analysis exactly (Mova en 0.4, CPU, 2026-10-17). The frame reader also checks the auxiliary's form: an `aux` ending in *n't* or the form *cannot* negates the frame even if the clitic was not split ([`prag/src/frames.rs`](../../prag/src/frames.rs), `fn negated`).

## 4. Negation in non-finite clauses and imperatives

### 4.1 Infinitives, gerunds and participles

Non-finite verbs take *not* **before** them, without *do* (Brown 1851, "Form of Negation": infinitives and participles "take the negative first"):

(6) a. *She decided **not to go**.* — `advmod(go, not)`, `mark(go, to)`
    b. ***Not knowing** the way, they turned back.*
    c. *He was warned **not to be** late.*
    d. *to **not** attempt* — the split infinitive is ordinary in current usage (seed [`negation-not.md`](../../en/seeds/verbal/negation-not.md)).

### 4.2 Imperatives

The negative imperative always takes *do*, even with *be* and *have*: *Don't be silly*, *Don't have a cow*. In UD both *do* and the lexical verb are imperative:

(7) *Don't touch the stove!* (Mova en 0.4, CPU, 2026-10-17)

```
1  Do     do     AUX   3  aux      Mood=Imp|VerbForm=Fin
2  n't    not    PART  3  advmod   Polarity=Neg
3  touch  touch  VERB  0  root     Mood=Imp|VerbForm=Fin
4  the    the    DET   5  det
5  stove  stove  NOUN  3  obj
```

The letter of the UD English feature page would give *touch* `VerbForm=Inf` and put the mood only on *do*. The `mova` dialect follows the EWT majority instead (98 of 106 cases): the lexical verb keeps `Mood=Imp|VerbForm=Fin`, so *Go!* and *Don't go!* look alike on the verb that carries the event ([`dialects/mova/seeds.md#imperative-do`](../../dialects/mova/seeds.md#imperative-do)). The conversion back to the literal guideline is deterministic.

Archaic and biblical English negates the imperative with post-verbal *not* and no *do*:

(8) *Thou shalt not utter a false report; put not thy hand with the wicked …* — Torah (JPS 1917), `jps:EXO:648` in `$MOVA_DATA/corpora/judaism/parsedb/current.conllu`

In the parse database the second *not* hangs on *hand* (`advmod(hand, not)`) — a constituent-negation reading. By the convention of §1.2 it should be `advmod(put, not)`: the imperative is negated, not the noun. This is one of the archaic patterns the Torah drafts still get wrong (2026-10-17).

*Let's not* and *Don't let's* are the first-person plural negative imperatives; EWT treats *let* as the root VERB and *'s* as its object (`obj`).

## 5. Negative words: determiners, pronouns, adverbs, connectives

### 5.1 The paradigm

English has a family of words that negate by themselves, each with a positive and a polarity-sensitive partner (Jespersen 1917, chs. 6, 10; Curme 1931, *Syntax*, `books/c/cu/curme-1931-grammar-english-language-vol`):

| negative | non-assertive (NPI) | assertive | UPOS / FEATS in EWT | relation (typical) |
|---|---|---|---|---|
| *no* (det) | *any* | *some* | DET `PronType=Neg` | det |
| *none* | *any* | *some* | PRON `PronType=Neg` | nsubj, obj, root |
| *nobody, no one* | *anybody* | *somebody* | PRON `Number=Sing|PronType=Neg` | nsubj |
| *nothing* | *anything* | *something* | PRON `Number=Sing|PronType=Neg` | obj, nsubj |
| *nowhere* | *anywhere* | *somewhere* | ADV `PronType=Neg` | advmod |
| *never* | *ever* | *sometimes, always* | ADV `PronType=Neg` | advmod |
| *neither* (det/pron) | *either* | *both* | DET/PRON `PronType=Neg` | det, nsubj |
| *neither … nor* | *either … or* | *both … and* | CCONJ `Polarity=Neg` | cc:preconj, cc |
| *no* (answer) | — | *yes* | INTJ `Polarity=Neg` | discourse, root |
| *no* (degree: *no longer, no bigger*) | *any longer* | — | ADV | advmod |

EWT 2.18 counts (2026-10-17): *no* DET 323, *never* 175, *nothing* 83 across relations, *none* 17, *nobody* 7, *nowhere* 3, *neither* CCONJ 7 + DET 2 + ADV 2, *nor* 20. The split of features matters for any reader: **`Polarity=Neg` marks only *not/n't*, *nor/neither* and the answer *no*; the negative quantifiers are found by `PronType=Neg`.**

### 5.2 Examples

(9) *Nobody should feel a looser in Afghanistan …* — EWT 2.18 `weblog-blogspot.com_dakbangla_20041016151002_ENG_20041016_151002-0012` (`nsubj`)

(10) *I have never been and I will never be a Government official and I have no privileged information.* — EWT 2.18 `weblog-blogspot.com_alaindewitt_20060924104100_ENG_20060924_104100-0007` (*never* `advmod`; *no* `det` of *information*)

(11) *Neither the ISO board, which had established the price cap, nor the governor learned of Winter's actions …* — EWT 2.18 `email-enronsent07_01-0057`

```
1   Neither   neither   CCONJ  4   cc:preconj
4   board     board     NOUN   16  nsubj      DEPS 8:nsubj|16:nsubj
6   which     which     PRON   8   nsubj      DEPS 4:ref
8   established …       VERB   4   acl:relcl
13  nor       nor       CCONJ  15  cc
15  governor  governor  NOUN   4   conj       DEPS 4:conj:nor|8:nsubj|16:nsubj
16  learned   learn     VERB   0   root
```

The enhanced graph labels the conjunct `conj:nor` and copies the subject relation to *governor* (`16:nsubj`). It also gives *governor* `8:nsubj` — as if the governor too had established the cap. That edge is an over-propagation into the relative clause, which belongs to *board* alone; a reader that trusts DEPS blindly inherits the error.

### 5.3 Sentence-level effect

A negative quantifier in subject or object position negates the whole clause just as *not* does: *Nobody came* = 'it is not the case that anybody came'. Two tests (Klima 1964, wanted) show clausal negation: a positive tag (*Nobody came, **did** they?*) and *neither*-continuation (*Nobody came, and neither did Bill*). Compare constituent negation in §6.

### 5.4 Where the negative goes: negative attraction

English prefers to put the negation early, on the subject quantifier rather than on the verb: *Nobody came* is preferred to *\*Anybody didn't come*; *No one saw anything* rather than *\*Anyone saw nothing*. Jespersen (1917, ch. 6) calls this **negative attraction**: the negative is drawn to the first suitable word, and especially to an indefinite subject. With an object, both orders survive with a register difference: *I saw nothing* (more formal, narrative) vs *I didn't see anything* (more colloquial).

## 6. Constituent (special) negation and the scope of *not*

### 6.1 Clausal vs constituent negation

Jespersen (1917, ch. 5) separates **nexal** negation, which denies the link between subject and predicate (*He doesn't come today*), from **special** negation, which negates one notion (*never, unhappy, impossible, non-belligerent*). Modern grammar speaks of **clausal** vs **constituent** negation (Klima 1964; Huddleston & Pullum 2002). Silvennoinen's introduction to the 2025 edition notes that Jespersen's two classes mix formal and functional criteria, and that it is best to keep syntactic analysis (where the negator sits) apart from semantic analysis (what is denied) (Jespersen 1917/2025, introduction §4.2).

| test | clausal (*She didn't find any problem.*) | constituent (*She found no small problem.*, *Not long ago, she left.*) |
|---|---|---|
| positive tag question | *…, did she?* | *…, didn't she?* |
| *neither/nor* continuation | *…, and neither did he.* | *…, and so did he.* |
| licenses *any/ever* | yes | no |
| inversion when fronted | *Not once did she …* | *Not long ago she …* (no inversion) |

(12) a. *Not a single child cried.* — clausal: the negation sits on the subject phrase but scopes over the clause.
     b. *Not long ago, a fox lived here.* — constituent: only *long* is negated.

UD does not distinguish them by relation: *not* is `advmod` of whatever it modifies. In (12a) the EWT convention attaches *not* to the head of the phrase it negates when it is inside a nominal (*not a single child*), but parsers often hang it on the verb. Mova en 0.4 attaches *Not* in (12a) directly to *cried* (`advmod(cried, Not)`, CPU, 2026-10-17), which happens to give the right clausal reading to the frame reader.

### 6.2 Scope ambiguity

*Not* interacts with quantifiers, modals and adverbials, and English word order often leaves the scope open (Jespersen 1917, ch. 8; Horn 1989, wanted):

(13) a. *All the guests didn't leave.* — 'none left' (∀ > ¬) or 'not all left' (¬ > ∀); spoken English disambiguates by intonation.
     b. *You **mustn't** go.* — ¬ inside: 'you must not-go' (prohibition); *You **needn't** go.* — ¬ outside: 'it is not necessary'.
     c. *She didn't come **because** she was ill.* — either 'because ill, she stayed away' or 'she came, but not because she was ill'.
     d. *I didn't eat **some** of the cake.* — *some* resists scope under *not* ('there is some I did not eat').

| modal | *not* scopes over the modal | *not* scopes under the modal |
|---|---|---|
| *can, could* | *can't* = 'not possible/permitted' | — |
| *may* (epistemic) | — | *may not* = 'possibly not' |
| *may* (deontic) | *may not* = 'not permitted' | — |
| *must* | — | *mustn't* = 'obliged not to' |
| *need, have to* | *needn't, don't have to* = 'not obliged' | — |
| *will, shall* | — | *won't* = 'will not-do' |
| *should, ought to* | — | *shouldn't* = 'obliged not to' (weak) |

The UD tree is the same for every reading: scope is not in the dependency structure. A reader that wants scope needs rules over the lemma of the modal (the registry in `dialects/mova/seeds.md#modal-mood` lists each modal's function) and over the position of quantifiers.

## 7. Focus and contrastive negation

*Not* often denies one focused element and leaves the rest as background:

(14) a. *He doesn't smoke CIGARS, only cigarettes.* (Jespersen 1917, ch. 5)
     b. *I didn't say she STOLE it; I said she BORROWED it.*
     c. *The meeting is not on Monday but on Tuesday.*
     d. *Not John but Mary called.*

Jespersen (1917) already discussed **contrastive negation**, where the negator attaches to the focus constituent (14d) instead of the verb; the introduction to the 2025 edition notes that it was then "largely ignored" for decades (Silvennoinen, introduction to Jespersen 1917/2025, §4.3). The *not X but Y* pattern is coordination in UD: *Mary* is `conj` of *John*, *but* `cc`, *not* `advmod` of *John*. EWT has *not only … but (also)*:

(15) *But it is clear that not only did Bush slack off on his National Guard service, but he slacked off from his campaign work.* — EWT 2.18 `weblog-juancole.com_juancole_20040823064025_ENG_20040823_064025-0033`

Here fronted *not only* triggers subject–auxiliary inversion (*did Bush slack*), as every fronted negative with clausal scope does (§6.1, and [29-ellipsis-word-order-information-structure.md](29-ellipsis-word-order-information-structure.md) §8.3).

Focus is information structure, not syntax; see chapter 29 §§11–12 for focus and the question under discussion.

## 8. Negative polarity items and their licensers

### 8.1 What an NPI is

A **negative polarity item** (NPI) is a word or phrase that is acceptable only in certain "negative-like" contexts (Klima 1964; Ladusaw 1979; both wanted):

(16) a. *She didn't eat **anything**.* / *\*She ate **anything**.* (on the NPI reading)
     b. *Have you **ever** been there?* / *\*I have **ever** been there.*
     c. *I haven't seen him **at all** this week.* — cf. EWT 2.18 `email-enronsent34_01-0009` (*I haven't see him at all this week*)
     d. *It doesn't sound like he can receive visitors **yet**.* — EWT 2.18 `email-enronsent40_01-0022`
     e. *I don't live in Lake St. Louis **anymore** …* — EWT 2.18 `reviews-164580-0005`
     f. *He didn't **lift a finger** / **budge an inch** / **give a damn**.* — minimizers (idioms)

### 8.2 Licensing contexts

| licenser | example |
|---|---|
| clausal negation (*not, n't, never, no*-words) | *Nobody saw **anything**.* |
| *without*, *before* | *She left without saying **anything**.* |
| questions (yes/no and wh) | *Did **anyone** call?* |
| conditionals (*if*-clause) | *If **anyone** calls, tell me.* |
| restrictor of a universal | *Everyone who has **ever** met her likes her.* |
| comparatives, *too … to* | *She ran faster than **anyone**.* |
| negative-implicative verbs and adjectives | *I doubt/deny that **anyone** came; I'm reluctant to say **anything**.* |
| "incomplete negatives" *hardly, scarcely, few, little, only* | *Hardly **anyone** came; Only Mary has **ever** seen it.* |

Ladusaw's generalization (1979) is that NPIs are licensed in **downward-entailing** contexts: those that license inferences from a set to its subsets (*not … dogs* ⇒ *not … black dogs*). Jespersen's "incomplete negation" (*hardly, few*) belongs here (Jespersen 1917, ch. 4).

### 8.3 Free-choice *any*

*Any* also has a free-choice reading that needs no licenser: ***Any** child can do this.* It is stressed, often with a modal or generic. Readers must not take every *any* as a sign of negation.

### 8.4 NPIs in UD and in language models

UD treats NPIs as ordinary words: *any* is `DET` (`PronType=Ind`), *anything* `PRON`, *ever* and *yet* `ADV`, *at all* is a PP (`nmod` on a preceding noun, or `obl` on the verb). Licensing is invisible in the tree. In the BLiMP benchmark NPI licensing was among the hardest phenomena for language models, together with islands (Warstadt et al. 2020, TACL, https://aclanthology.org/2020.tacl-1.25; cited in seed [`double-negation.md`](../../en/seeds/errors/double-negation.md)).

Mova en 0.4 attaches *at all* in «She didn't eat anything at all.» as `nmod(anything, all)`, with *all* as DET (CPU, 2026-10-17). EWT 2.18 does the same: in `weblog-juancole.com_juancole_20040823064025_ENG_20040823_064025-0071` (*George didn't have any interest at all …*) *all* is `nmod` of *interest*, with *at* as `case`. The tree treats the polarity idiom as an ordinary PP on the nearest noun; its link to the negation is not represented.

## 9. Negative raising

Some verbs of opinion and expectation let a negation in the main clause be understood in the complement (Jespersen 1917, ch. 6, on attraction of the negative to the main verb; Horn 1989 calls it "neg-raising"):

(17) a. *I don't think he will come.* ≈ *I think he won't come.*
     b. *I don't believe she knows.* ≈ *I believe she doesn't know.*
     c. *I don't suppose you could help?*
     d. *I don't think so.* — EWT 2.18 `answers-20111107193815AAhaJjy_ans-0015`

Jespersen (1917, ch. 6) observes that sentences like (17a) "really mean" the negated complement, while with *hope* and *expect* English prefers the logical placement (*I hope he won't come*), unlike Danish. Neg-raising predicates include *think, believe, suppose, expect, imagine, want, seem, appear, be likely, intend, plan*; *know, say, hope, claim* are not (*I don't know that he will come* is not ≈ *I know that he won't*).

A test: a strict NPI that needs a clause-mate negation is licensed across a neg-raising verb: *I don't think he'll arrive **until** tomorrow* / *\*I don't claim he'll arrive until tomorrow*.

UD keeps the syntax literal: `advmod(think, n't)`, `ccomp(think, come)`. Mova en 0.4 parses (17a) exactly so. The frame reader then marks the **think** frame as negated and the quoted *come* frame as positive:

```
$ world frames "I don't think the fox will come."
#1 Say(other) i NOT think | says→#2
#2 Event fox come
```

(Mova, CPU, 2026-10-17.) The student that matches frames would therefore treat «the fox will come» as said, unless it knows that *think* is a neg-raising verb. **Not yet** implemented: a list of neg-raising verbs that moves the negation into the quoted frame.

## 10. Affixal negation

### 10.1 The prefixes and suffix

| affix | attaches to | meaning | examples |
|---|---|---|---|
| *un-* | adjectives, participles | 'not' (contradictory or contrary) | *unhappy, unknown, unexpected* |
| *un-* | verbs | reversal | *undo, unlock, untie* |
| *in-* (*im-, il-, ir-*) | Latinate adjectives | 'not' | *impossible, illegal, irregular* |
| *dis-* | adjectives, verbs, nouns | 'not', reversal | *dishonest, disagree, disorder* |
| *non-* | adjectives, nouns | 'not' (contradictory, neutral) | *non-violent, non-member* |
| *a-* | learned adjectives | 'without' | *amoral, atypical* |
| *-less* | nouns → adjectives | 'without' | *careless, harmless, endless* |

Jespersen (1917, ch. 13) gives a full historical account of *un-, in-, dis-, non-* and *a-*. Morphology is the subject of the 1x chapters of this manual; here only the syntactic consequences matter:

- **Affixal negation does not make a clause negative.** *She is unhappy, isn't she?* takes a negative tag (the clause is positive); *She isn't happy, is she?* takes a positive one. Affixal negatives do not license NPIs: *\*She is unhappy with anything.*
- **Contrary vs contradictory.** *Not happy* is weaker than *unhappy*: Jespersen (1917, ch. 5) notes that *she isn't very happy* and *she is very unhappy* differ greatly, while *she isn't happy* and *she is unhappy* are close. *Unhappy* is a contrary (there is a middle ground), *not happy* a contradictory.
- **Participles in un-** with no verb behind them (*unexpected, uninteresting*) are adjectives (ADJ), not passive verbs (seed [`participle-or-adjective.md`](../../en/seeds/verbal/participle-or-adjective.md)).

### 10.2 In UD

The affix is inside the word; UD has no feature for it in English. `Polarity=Neg` is **not** put on *unhappy* (the UD feature page limits `Polarity` to words that express negation as such). EWT 2.18 has no `Polarity` on any *un-* adjective.

(18) *This is not unknown to New Delhi.* — EWT 2.18 `weblog-blogspot.com_dakbangla_20050210141134_ENG_20050210_141134-0021` — litotes (§11.3): the tree shows one negation (*not*), the meaning has two.

### 10.3 In Mova

The frame reader takes *unhappy* as an attribute word and does not negate the frame: «The fox is unhappy.» → `Fact(is) fox be attrs=unhappy` (Mova, CPU, 2026-10-17). That is the right structural choice — the fact is positive — but the student has no link between *unhappy* and *not happy*, so a test option «The fox is not happy» does not match it. **Not yet**: an antonym or prefix lexicon at the attribute level. The irony module's polar lexicon ([`prag/src/irony.rs`](../../prag/src/irony.rs)) lists content-word lemmas by polarity, so *unhappy* counts as a negative word there, independently of syntax.

## 11. Double negation, negative concord, litotes

### 11.1 Standard English: one negation per clause

In standard English two negatives in one clause cancel (or are odd):

(19) a. *I didn't see nothing.* — non-standard; in the standard it would mean 'I saw something'.
     b. *I can't **not** go.* — deliberate: 'I must go' (two scopes: the modal is negated, and the complement is negated).
     c. *Nobody has nothing.* — 'everybody has something' (logical double negation, rare and marked).

### 11.2 Negative concord in non-standard varieties

Many varieties of English — African American English, Appalachian and southern US English, many British dialects, and older English up to Shakespeare's time — use **negative concord**: several negative words express a single negation (Wright 1905, *The English Dialect Grammar*, `books/w/wr/wright-1905-english-dialect-grammar-comprising`, on dialect negation; Jespersen 1917, ch. 7 "Double negation"):

(20) a. *I don't know nothing about it.* = 'I know nothing about it'
     b. *He never said nothing to nobody.*
     c. *Ain't nobody got time for that.* (with negative inversion)

Findlay et al. (2025, UDW, https://aclanthology.org/2025.udw-1.8) classify English as a language without negative concord in its standard, unlike Spanish. EWT 2.18 has two clear cases, both colloquial; one is

(21) *so, I don't think the bird don't trust you, but still don't like the cage.* — EWT 2.18 `answers-20111108071852AAxbh5F_ans-0018` (a doubled negation across *think*, plausibly a neg-raising echo)

UD annotates every negative word literally; the reading (concord or cancellation) is not in the tree. The `en` seed [`errors/double-negation.md`](../../en/seeds/errors/double-negation.md) flags *not* plus a `PronType=Neg` dependent on the same predicate as a warning, with exceptions for deliberate double negation and for quoted dialect speech.

Mova en 0.4 parses «Nobody saw nothing.» correctly as two negative pronouns (`nsubj`, `obj`). The frame reader, however, finds **no** negation in it (§15.2).

### 11.3 Litotes and weakened negatives

**Litotes** affirms by denying the opposite: *not bad* 'good', *not unknown* 'known', *no small achievement* 'a large achievement', *not a little surprised* 'very surprised'. Jespersen (1917, ch. 8) notes that negating the weak member of a scale (*not a little, not a few*) gives the strong one ('much', 'many'), while negating the strong member (*not much, not many*) gives the weak one ('little', 'few').

**Weakened negatives** carry little negative force: *Won't you have a glass of beer?* is close to *Will you have …?* (Jespersen 1917, ch. 9). Negative yes/no questions are biased: *Isn't he stupid!* asserts that he is (Jespersen 1917, ch. 4); see [26-questions.md](26-questions.md) and Trinh, Benz, Goodhue, Yatsushiro & Krifka (eds. 2025, *Biased questions*, `books/k/kr/krifka-2025-biased-questions-experimental-results`).

These are pragmatic effects of negation; they are treated in [33-implicature-and-presupposition.md](33-implicature-and-presupposition.md). Roitman (2023, ed., *Negatives and Meaning: Social Setting and Pragmatic Effects*, Stockholm University Press, CC BY 4.0, `books/r/ro/roitman-2023-negatives-meaning-social-setting`, https://library.oapen.org/handle/20.500.12657/62901) collects studies of negation in polemic, presupposition denial and its social effects.

## 12. Negation in ellipsis and fragments

When the predicate is elided, *not* can be the only thing left, and then it is promoted to head (UD guidelines, `_en/specific-syntax.md`, Ellipsis; seed [`negation-not.md`](../../en/seeds/verbal/negation-not.md)):

(22) *If not, is there someone else that will?* — EWT 2.18 `email-enronsent24_01-0023`

```
1  If     if     SCONJ  2  mark
2  not    not    PART   4  advcl      DEPS 4:advcl:if
4  is     be     VERB   0  root
5  there  there  PRON   4  expl
```

(23) *Not yet.* — EWT 2.18 `email-enronsent30_01-0018`: `advmod(yet, Not)`, *yet* is the root.

(24) *Why not just take your chances …?* — EWT 2.18 `answers-20111108082831AAco5PI_ans-0036`

| pattern | example | *not* is … |
|---|---|---|
| conditional remnant | *If not, …* | `advcl` (head of the elided clause) |
| answer fragment | *Not yet. / Certainly not.* | `advmod` of the remaining word, or root |
| *I hope/think/guess not* | *I hope not.* | `ccomp`/`advmod` of the verb (EWT varies; seed lists `ccomp`) |
| *whether or not* | *whether or not you agree* | `conj` of *or*'s first conjunct; EWT has 21 *not* as `conj` |
| stripping | *Mary went, but not John.* | `advmod` of the remnant *John*, which is `conj`/`orphan` (chapter 29 §3) |
| VP ellipsis | *Mary didn't leave, John did.* | — (the *not* stays with the antecedent) |

EWT has 21 *not* as `conj`, 11 as `advcl`, 7 as `cc`, 3 as `root` and 2 as `ccomp` (2026-10-17). The `en` rule `en.verbal.not-part-neg` therefore allows these relations for *not* in addition to `advmod`.

## 13. Jespersen's cycle: how English got here

Jespersen (1917, ch. 1) describes a recurring "fluctuation": the original negative adverb weakens, is felt insufficient, is strengthened by another word, and the strengthener then becomes the negative proper and may weaken in turn. The idea has since been called **Jespersen's cycle**, although, as Silvennoinen's introduction to the 2025 edition notes, Meillet (1912) and earlier scholars had observed it (Jespersen 1917/2025, introduction §4.1).

English went through these stages (Jespersen 1917, ch. 1):

| stage | form | period |
|---|---|---|
| 1 | *ic **ne** secge* — preverbal *ne* | Old English |
| 2 | *I **ne** seye **not*** — *ne* reinforced by *not* (< *nāwiht* 'nothing') | Middle English |
| 3 | *I say **not*** — *ne* lost, post-verbal *not* | late Middle English to Early Modern English |
| 4 | *I do **not** say* — *not* after the auxiliary, do-support | from the 16th–17th c. |
| 5 | *I **don't** say* — *not* cliticized as *n't* | Modern English |

Stage 3 survives in archaic and biblical style (*I know not*, *put not thy hand*, §4.2) and in fixed phrases (*I kid you not*, *if I mistake not*). Jespersen (1917, ch. 1) also noted the "natural tendency" to place the negative early, "very often immediately before the particular word to be negatived" — the modern position before the lexical verb fits this.

Later research found that Meillet's explanation by the speaker's wish for expressiveness fits the facts better than Jespersen's phonetic weakening, and that the strengthened negator first appears where the negated content is already active in the discourse (summary in Jespersen 1917/2025, introduction §4.1, citing van der Auwera 2009 and Hansen & Visconti 2009).

The Torah English trees in the data directory are a good test bed for stage 3 forms: *Ye shall not surely die* (`jps:GEN:64`) has ordinary stage-4 negation, while *put not thy hand* (`jps:EXO:648`) is stage 3.

## 14. Quick reference: negation in UD

| construction | example | analysis |
|---|---|---|
| auxiliary + *not* | *has not been killed* | `advmod(killed, not)`; *not* PART `Polarity=Neg` |
| do-support | *did not catch* | `aux(catch, did)`, `advmod(catch, not)`; *catch* `VerbForm=Inf` |
| contraction | *can't* | MWT `ca` (lemma can, `aux`) + `n't` (lemma not, `advmod`) |
| copula | *isn't happy* | `cop(happy, is)`, `advmod(happy, n't)` |
| negative imperative | *Don't go!* | `aux(go, Do)`, `advmod(go, n't)`; both `Mood=Imp` in `mova` |
| infinitive | *not to go* | `advmod(go, not)`, `mark(go, to)` |
| negative determiner | *no information* | `det(information, no)`; DET `PronType=Neg` |
| negative pronoun | *Nobody came* | `nsubj(came, Nobody)`; PRON `PronType=Neg` |
| *never* | *never caught* | `advmod(caught, never)`; ADV `PronType=Neg` |
| *neither … nor* | *Neither A nor B* | `cc:preconj(A, Neither)`, `cc(B, nor)`, `conj(A, B)`; EUD `conj:nor` |
| answer *no* | *No, I won't.* | `discourse(…, No)`; INTJ `Polarity=Neg` |
| remnant | *If not, …* | *not* is the head: `advcl` |
| affix | *unhappy* | no feature; one ADJ token |

---

## 15. In Mova

### 15.1 Parsing

- The `en` parser (release 0.4) follows EWT for *not/n't*: PART, `Polarity=Neg`, `advmod` of the predicate; contractions are split into syntactic words. The rules in [`en/seeds/verbal/negation-not.md`](../../en/seeds/verbal/negation-not.md) (`en.verbal.not-part-neg`, `en.verbal.negation-on-predicate`) and [`en/seeds/errors/double-negation.md`](../../en/seeds/errors/double-negation.md) (`en.errors.double-negation-pronoun`, `en.errors.double-negation-det`) check trees in the teacher loop.
- The `mova` dialect keeps `Mood=Imp|VerbForm=Fin` on the lexical verb of a *don't* imperative ([`dialects/mova/seeds.md#imperative-do`](../../dialects/mova/seeds.md#imperative-do)).
- Measured parses (Mova en 0.4, CPU, 2026-10-17), against UD:

| sentence | Mova en 0.4 | UD / EWT | verdict |
|---|---|---|---|
| *I can't tell you.* | ca/aux, n't/advmod, tell/root, you/iobj | same | right |
| *Don't touch the stove!* | Do/aux, n't/advmod of touch | same | right |
| *Nobody saw nothing.* | nsubj, obj | same | right |
| *I don't think he will come.* | advmod(think, n't), ccomp(think, come) | same | right |
| *Not a single child cried.* | advmod(cried, Not) | *not* on the nominal (EWT practice varies) | acceptable |
| *She didn't eat anything at all.* | nmod(anything, all), *all* DET | same as EWT (`nmod` on the noun) | right by EWT |
| *He is unhappy but not unkind.* | advmod(unkind, not), conj | same | right |

### 15.2 The frame reader

- A frame carries `neg: bool` ([`prag/src/frames.rs`](../../prag/src/frames.rs), `struct Frame`). `fn negated` sets it when a child of the predicate is *not, n't, never* or *no* as `advmod` (or `aux`), or when an auxiliary child ends in *n't* or is *cannot*. A bare phrase (an answer option) is negated if any of these lemmas occurs anywhere in it.
- Measured on short texts (`world frames`, CPU, 2026-10-17):

| text | frame | right? |
|---|---|---|
| *The fox never caught the hen.* | `Event fox NOT catch patient=hen` | yes |
| *The hen didn't see anything.* | `Event hen NOT see patient=anything` | yes |
| *Not a single bird sang.* | `bird NOT sing` | yes |
| *The midwives did not kill the boys.* | `Event midwife NOT kill patient=boy` | yes |
| *Nobody saw the fox.* | `Event nobody see patient=fox` | **no** — the negative subject is not seen as negation |
| *No child cried.* | `child cry` | **no** — the negative determiner sits on the subject, not on the verb |
| *I don't think the fox will come.* | `i NOT think`, quote `fox come` | literal; neg-raising not applied (§9) |
| *The fox is unhappy.* | `fox be attrs=unhappy` | positive fact; no link to *not happy* (§10.3) |

- **The student** ([`world/src/knowledge.rs`](../../world/src/knowledge.rs)) compares the question frame's `neg` with the memory frame's: a mismatch on the same verb sets the feature `NegDisagree`, which counts against the match (`fn start_weights`). On 2026-10-17 the diagnosis of the Torah course found that «the midwives did not kill the boys» lost to «they killed the baby boys»: the disagreeing negation weighed too little. A heavier weight (−8) was measured: Torah ★ 252 → 252, Torah exam 99 → 97, biology plain 791 → 770, a net loss, so it was **not kept** ([`docs/school.md`](../../docs/school.md); commit `f3e616747`). The conclusion recorded there is that the remedy is not the weight.
- **Implicatures** ([`prag/src/infer.rs`](../../prag/src/infer.rs)): scalar implicatures are not drawn under negation (*not all the guests left* does not implicate *some did not*); the function `negated` there is shared with the irony module.
- **Irony** ([`prag/src/irony.rs`](../../prag/src/irony.rs)): a negated polar word flips its sign (*not good* counts as negative), and an ended or absent situation (*no traffic*) is not a negative situation.
- **Pragmatic labels** in `$MOVA_DATA/parsedb/prag.tsv`: the column `polarity` (pos / neg / none) is the speaker's **evaluation**, not grammatical negation (schema in [`prag/src/schema.rs`](../../prag/src/schema.rs)). A sentence with *not* can be `polarity=pos` (*not bad at all!*) and one without it `neg`.

### 15.3 Not yet

- Negative quantifiers in subject or object (*nobody, nothing, no N, none*) do not negate the frame.
- No scope: quantifier–negation and modal–negation readings (§6.2) are not represented.
- No neg-raising list (§9); no NPI-licensing check; no treatment of *hardly, scarcely, few, little, only* as incomplete negatives.
- No antonym link between affixal and syntactic negation (§10.3).
- Negative concord in quoted dialect speech is read as two negations or none, depending on where the words attach.

## 16. Sources

- Bies, A., M. Ferguson, K. Katz, R. MacIntyre et al. 1995. *Bracketing Guidelines for Treebank II Style, Penn Treebank Project.* `books/b/bi/bies-1995-bracketing-guidelines-treebank-ii`.
- Brown, Goold. 1851. *The Grammar of English Grammars.* Part II, ch. VI, "Form of Negation". `books/b/br/brown-1851-grammar-english-grammars`.
- Curme, George O. 1931. *A Grammar of the English Language, vol. III: Syntax.* `books/c/cu/curme-1931-grammar-english-language-vol`.
- Findlay, J. Y. et al. 2025. Negation in Universal Dependencies (UDW 2025). https://aclanthology.org/2025.udw-1.8 (as cited in the `en` seed `errors/double-negation.md`).
- Jespersen, Otto. 1917. *Negation in English and Other Languages.* København: Høst. New edition, ed. B. Reynolds and P. Evans, with an introduction by Olli O. Silvennoinen, Language Science Press 2025 (Classics in Linguistics), CC BY 4.0, https://zenodo.org/records/14843487. `books/j/je/jespersen-1917-negation-english-other-languages`. Chapters cited: 1 (general tendencies, the cycle), 4 (indirect and incomplete negation), 5 (special and nexal negation), 6 (negative attraction), 7 (double negation), 8 (meaning of negation, scales), 9 (weakened negatives), 11 (verbal forms in *n't*), 13 (negative prefixes); introduction §§4.1–4.3.
- Trinh, Tue, Anton Benz, Daniel Goodhue, Kazuko Yatsushiro and Manfred Krifka (eds.). 2025. *Biased Questions: Experimental Results and Theoretical Modelling.* Language Science Press (Topics at the Grammar-Discourse Interface). `books/k/kr/krifka-2025-biased-questions-experimental-results`.
- Roitman, Malin (ed.). 2023. *Negatives and Meaning: Social Setting and Pragmatic Effects.* Stockholm University Press, CC BY 4.0. https://library.oapen.org/handle/20.500.12657/62901; `books/r/ro/roitman-2023-negatives-meaning-social-setting`.
- Santorini, Beatrice. 1990. *Part-of-Speech Tagging Guidelines for the Penn Treebank Project.* `books/s/sa/santorini-1990-part-speech-tagging-guidelines`.
- Universal Dependencies. English guidelines, snapshot 2026-09-24: `_en/pos/PART.md` (https://universaldependencies.org/en/pos/PART.html), `_en/feat/Polarity.md` (https://universaldependencies.org/en/feat/Polarity.html), `_en/dep/advmod.md`, `_en/specific-syntax.md` (ellipsis), `_en/feat/VerbForm.md` and `_en/feat/Mood.md` (imperatives).
- UD English EWT 2.18 (Silveira et al. 2014; CC BY-SA 4.0). Sentences cited by `sent_id`.
- Warstadt, A. et al. 2020. BLiMP: The Benchmark of Linguistic Minimal Pairs for English. TACL 8. https://aclanthology.org/2020.tacl-1.25.
- Wright, Joseph. 1905. *The English Dialect Grammar.* `books/w/wr/wright-1905-english-dialect-grammar-comprising`.
- Mova: [`en/seeds/verbal/negation-not.md`](../../en/seeds/verbal/negation-not.md), [`en/seeds/errors/double-negation.md`](../../en/seeds/errors/double-negation.md), [`en/seeds/verbal/need-dare.md`](../../en/seeds/verbal/need-dare.md), [`dialects/mova/seeds.md#imperative-do`](../../dialects/mova/seeds.md#imperative-do), [`dialects/mova/seeds.md#modal-mood`](../../dialects/mova/seeds.md#modal-mood), [`prag/src/frames.rs`](../../prag/src/frames.rs), [`prag/src/infer.rs`](../../prag/src/infer.rs), [`prag/src/irony.rs`](../../prag/src/irony.rs), [`prag/src/schema.rs`](../../prag/src/schema.rs), [`world/src/knowledge.rs`](../../world/src/knowledge.rs), [`docs/school.md`](../../docs/school.md).
- Wanted (not in the library, see the sources file): Klima 1964 "Negation in English"; Ladusaw 1979 *Polarity Sensitivity as Inherent Scope Relations*; Horn 1989 *A Natural History of Negation*; Huddleston & Pullum 2002 *The Cambridge Grammar of the English Language* (ch. 9); van der Auwera 2009 "The Jespersen cycles".
