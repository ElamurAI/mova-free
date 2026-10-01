# English annotation-error seeds (`ai/en/seeds/errors`)

A **seed** is a short markdown note from which a rule or a piece of knowledge grows. Each seed in this
folder describes one typical error in English Universal Dependencies (UD) annotation. The error can sit
in the text (learner or native-speaker mistakes such as *a answer*, *has went*, *between you and I*) or
in the analysis (a wrong head, label, tag or feature, often made by parsers or LLM annotators). This page
gives a summary of each one.

Every seed has the same sections: **Gist**, **Conditions and exceptions**, **Examples** and **Sources**.
Many also have an "In UD" note that says how the phenomenon looks in a tree. Most seeds end with one or
more ```` ```rule ```` blocks. The expert system (`en/src/expert.rs`) compiles these blocks and checks
CoNLL-U trees against them. The target annotation style is UD English EWT 2.18.

## Rule blocks

```rule
rule: en.errors.cop-be
what: cop not on be — become/seem/get are VERB with xcomp
match: c[rel=cop]
require: c[lemma=be]
severity: error
source: UD _en/dep/cop.md
```

- `match:` names one or more nodes, each with conditions: `upos`, `xpos`, `lemma`, `form`,
  `feats.X=…`, `rel` (`rel~x` matches a relation and its subtypes), `head=<node>`, `before=`/`after=`,
  `next=`/`prev=`, `prefix=`/`suffix=`. Alternatives are separated by `|`, and `!=` negates a condition.
- The engine is deterministic. For every sentence it tries each way of binding the `match` nodes to
  different words.
- `require:` lists clauses, and all of them must hold. Within a clause, `or` gives alternatives.
  `exists x[…]` asks for a word, `none x[…]` forbids one, and `not` negates a clause.
- A binding that fails `require` is reported as a violation, unless at least one `unless:` clause holds.
- `severity:` is `warn` (often a text error, or a convention that varies) or `error` (always wrong in UD v2).

Some seeds have no rule of their own. They point to a rule in another seed folder (for example
`ai/en/seeds/nominal`), or they are background notes that explain the method.

---

### a-an-choice
`ai/en/seeds/errors/a-an-choice.md`

**Gist.** The indefinite article is *an* before a vowel sound and *a* before a consonant sound. This
includes vowel letters that are pronounced [j] or [w] (*a unit, a eulogy, a one*). What counts is the
sound of the word right after the article, which is not always the noun. Errors like *a answer* come
from typos and from learners.

**Conditions and exceptions.** The engine sees only spelling, so the rule fires only when nothing stands
between the article and the noun. Known false alarms: *a European*, *a one-off*, *an FBI agent*.
Abbreviations starting with M, N, R, S, X and silent *h* (*an hour, an honest*) are left out.

**Examples.** ✗ *a answer*, ✗ *a order*, ✗ *a Immigrant*; ✓ *an apple*, ✓ *a big apple*.

**Machine-readable part.** `en.errors.a-before-vowel` (warn): *a* directly before a noun that starts
with a vowel letter. `en.errors.an-before-consonant` (warn): *an* directly before a noun that starts
with a consonant letter.

**Sources.** Fowler, *Modern English Usage* (1926), "A, AN" §1; ERRANT (P17-1074, DET); BEA-2019
(W19-4406).

### adjectives-in-names
`ai/en/seeds/errors/adjectives-in-names.md`

**Gist.** Since UD 2.8, adjectives and verbs inside proper names keep their own part of speech (ADJ,
VERB), although the XPOS stays NNP. An adjective that modifies a noun is `amod`, not `compound`:
*United States* → amod(States, United). The `compound` analysis is left over from automatic PTB
conversion.

**Conditions and exceptions.** EWT keeps a few lexicalised pairs as `compound`: *top notch*,
*green tea*, *quarter end*. Since UD 2.8, the lemma of an adjective inside a name is capitalised.

**Examples.** *the United States* → amod(States, United); *Central Iowa* → amod(Iowa, Central);
*Islamist officers* → amod(officers, Islamist).

**Machine-readable part.** `en.errors.adj-not-compound` (warn): an ADJ attached to a noun as
`compound` should be `amod`.

**Sources.** Zeldes & Schneider (2023.udw-1.7); UD guidelines `en/dep/flat`.

### advcl-on-noun
`ai/en/seeds/errors/advcl-on-noun.md`

**Gist.** A clause that modifies a noun is `acl` (or `acl:relcl`). `advcl` modifies a predicate: a
verb, an adjective, or a nominal predicate with a copula. An `advcl` under a noun that has no copula
usually means the clause was attached to the nearest noun instead of the verb, or was given the wrong
label.

**Conditions and exceptions.** A nominal predicate with a copula may take `advcl` (*It was a mess when
we arrived*). In verbless fragments such as headlines and captions, the noun can be the root and take
`advcl`.

**Examples.** *a decision to leave* → acl(decision, leave); *I called when I arrived* →
advcl(called, arrived); *It was a mess when we arrived* → advcl(mess, arrived), because
cop(mess, was) is present.

**Machine-readable part.** `en.errors.advcl-on-noun` (warn): `advcl` under a NOUN or PROPN that has
no `cop` dependent.

**Sources.** UD guidelines `en/dep/advcl`, `en/dep/acl`; Bernhard et al. (2025.law-1.14, LLMs prefer
shorter arcs); 2023.udw-1.7.

### adverb-placement
`ai/en/seeds/errors/adverb-placement.md`

**Gist.** In English, an adverb normally does not go between the verb and its direct object:
✓ *I like football very much*, ✗ *I like very much football*. Learners whose language has freer word
order carry this habit over. ERRANT classes it as WO (word order).

**Conditions and exceptions.** A long ("heavy") object may move to the end of the clause: *I understand
better the risks that we face*. Frequency adverbs (*always, often, never*) go before the verb, so after
the verb they are already a word-order error.

**Examples.** ✗ *I like very much football.* ✓ *I understand better the risks we face.* (heavy object)

**Machine-readable part.** `en.errors.adverb-before-object` (warn): an `advmod` from a fixed list
(*much, well, often, always, never, usually, also, very*) stands after the verb and before a nominal
`obj`.

**Sources.** ERRANT (P17-1074, WO); Bryant et al. 2019 (W19-4406).

### attraction-of-phrase
`ai/en/seeds/errors/attraction-of-phrase.md`

**Gist.** This is agreement attraction. The subject is a singular noun with a plural *of*-phrase, and the
verb agrees with the nearer plural noun instead (*A pattern of arrests indicate…*). Fowler calls these
nouns "red herrings". Quantity nouns are the exception: with *a lot of*, *a number of*, *the majority
of*, *half of* and similar, the *of*-phrase decides the number.

**Conditions and exceptions.** The list of quantity and collective nouns is open (*lot, number,
majority, rest, half, percent, bunch, couple, variety, group, kind, sort, type, …*), and the rule skips
these lemmas. The rule cannot tell a coordinated subject, which is legitimately plural, from attraction.

**Examples.** ✗ *A pattern of arrests and seizures indicate…*; ✗ *The return to flight activities are
funded.*; ✓ *A number of people believe…*

**Machine-readable part.** `en.errors.attraction-of` (warn): a singular NOUN subject with a plural
`nmod`, and the verb is VBP. `en.errors.attraction-of-auxcop` (warn): the same pattern with a VBP
copula or auxiliary. Both rules exempt quantity and collective lemmas.

**Sources.** Fowler 1926, NUMBER §4 "Red herrings"; Zacharopoulos et al. (2023.emnlp-main.998); BLiMP
(2020.tacl-1.25); Garcia et al. (2025.depling-1.4).

### clausal-subject-csubj
`ai/en/seeds/errors/clausal-subject-csubj.md`

**Gist.** When the subject is a whole clause (*That he lied is obvious*, *Taking a nap will relax
you*), the relation is `csubj`, and it attaches to the verb of that clause. `nsubj` on a word tagged
VERB is almost always wrong. LLMs learn `csubj` late and use it inconsistently.

**Conditions and exceptions.** A verbal noun (*The swimming was fun*, NN) is `nsubj`. EWT tags
*following* in *the following is…* as VERB with `nsubj`. For *it*-extraposition, see
`tough-vs-extraposition`.

**Examples.** *Whether he lied is beside the point* → csubj(point, lied); *Taking a nap will relax
you!* → csubj(relax, Taking).

**Machine-readable part.** `en.errors.verb-subject-csubj` (warn): `nsubj` or `nsubj:pass` on a VERB.

**Sources.** UD guidelines `en/dep/csubj`; Matsuda et al. (2026.udw-1.1); Dönicke et al. (2020.udw-1.8).

### consistency-detection-methods
`ai/en/seeds/errors/consistency-detection-methods.md`

**Gist.** This background seed has no rule of its own. Errors in a treebank or in parser output leave
statistical traces, and several independent methods can find them. Each method suggests a different
kind of rule:
1. **Variation nuclei.** The same word pair in the same context gets different relations in different
   sentences.
2. **Arc direction.** Compare the share of right-pointing dependents per relation across treebanks of
   one language. This seed is where the direction rules come from: `flat` and `conj` point right,
   `compound` and `nummod` point left, and `cc` comes before its conjunct.
3. **UPOS-trigram divergence.** A symmetric KL measure between treebanks.
4. **Cross-parsing.** Train on one treebank, test on another, and read the confusion matrix.
5. **Robustness to small edits.** For example, swap a year in a sentence. Places where the parse
   changes are candidates for errors.

**Conditions and exceptions.** A rule that fires often on the gold standard is either wrong or is
catching errors in the gold standard itself. Real gold errors found this way include Case=Nom on *it*
after a preposition, `det` on *soldiers* in *an old soldiers' home*, and Number=Plur on *species*.

**Examples.** The English `compound` and `csubj` relations are among the least consistent across
treebanks. The `obl`/`nmod` pair is the top parser confusion in EWT and GUM.

**Sources.** de Marneffe et al. 2017 (W17-6514); Dickinson 2009; Dönicke et al. (2020.udw-1.8);
Aggarwal & Zeman (2020.tlt-1.9); Zeldes & Schneider (2023.udw-1.7); Kalpakchi & Boye (2021.udw-1.8).

### coordinated-subject-agreement
`ai/en/seeds/errors/coordinated-subject-agreement.md`

**Gist.** A subject made of two or more parts joined by *and* is plural: *Mother and children were
killed*. Agreeing with the nearest conjunct is an error. With *or*/*nor*, the verb agrees with the
nearest alternative instead.

**Conditions and exceptions.** These take the singular:
- one entity named with two words (*bread and butter is*, firm names, titles of works);
- several descriptions of one person (*Interim leader and front-runner Mahmoud Abbas talks*);
- *each/every X and Y*.

A verb placed before the subject may be singular (*There is a table and some chairs*), so the rule only
checks subjects that come before the verb.

**Examples.** ✗ *green curry and red curry is awesome* → ✓ *are*; ✗ *Mercury and Venus is conjunct*;
✓ *Mother or children are to die*.

**Machine-readable part.** `en.errors.coord-subject-verb` (warn): a subject with `conj` + `cc` *and*
before the verb, and the verb is VBZ. `en.errors.coord-subject-auxcop` (warn): the same with a VBZ
copula or auxiliary. Neither fires if a conjunct follows the verb or the subject has *each/every*.

**Sources.** Fowler 1926, NUMBER §2–3, 7, and IS §4; ERRANT (P17-1074, VERB:SVA); BLiMP
(2020.tacl-1.25).

### coordination-structure
`ai/en/seeds/errors/coordination-structure.md`

**Gist.** In UD v2, the first conjunct heads a coordination "X, Y and Z". Y and Z attach to X as
`conj`, and the conjunction attaches as `cc` to the conjunct that follows it. So `conj` always points
left to right, and `cc` comes before its head. SD and UD v1 attached `cc` to the first conjunct, and
old converters and models still do this.

**Conditions and exceptions.** A sentence-initial conjunction (*But he left*) is `cc` of the root and
still comes before its head. Paired conjunctions *either/both/neither* are `cc:preconj`.

**Examples.** *apples and pears* → conj(apples, pears), cc(pears, and); *He came, saw and conquered* →
conj(came, saw), conj(came, conquered), cc(conquered, and).

**Machine-readable part.** `en.errors.conj-rightward` (error): `conj` must follow its head.
`en.errors.cc-before-conjunct` (warn): `cc` must precede its head.

**Sources.** UD guidelines `en/dep/conj`, `en/dep/cc`; Peng & Zeldes (W18-4918); Przepiórkowski &
Woźniak (2023.acl-long.864); Grünewald et al. (2021.eacl-main.67); Maier et al. 2012 (coordination in
the PTB).

### copula-be-only
`ai/en/seeds/errors/copula-be-only.md`

**Gist.** In English UD, `cop` is only predicative *be*, and the nominal or adjectival predicate is the
head (*Bill is honest* → cop(honest, is)). Verbs like *become, seem, get, remain, look* are ordinary
VERBs with `xcomp`. With a verbal predicate, *be* is `aux` or `aux:pass`. The one exception is a clausal
predicate with `nsubj:outer` (*The problem is that…*): there *be* is `cop` of the clause's verb.

**Conditions and exceptions.** Existential *there is* has *be* as a VERB root (see
`existential-there`). Hyphenated former participles (*mouth watering*, *self-cleaning*) sometimes show
VERB + `cop` in EWT. Treebanks without lemmas (ESLSpok) give false alarms for the first rule.

**Examples.** *Bill got rich* → xcomp(got, rich); *Bill is speaking* → aux(speaking, is); *The
important thing is to keep calm* → nsubj:outer(keep, thing), cop(keep, is).

**Machine-readable part.** `en.errors.cop-be` (error): `cop` whose lemma is not *be*.
`en.errors.cop-on-verb` (warn): `cop` under a VERB with no `nsubj:outer` or `csubj:outer`.

**Sources.** UD guidelines `en/dep/cop`; Bernhard et al. (2025.law-1.14, perfect confused with copula).

### dep-last-resort
`ai/en/seeds/errors/dep-last-resort.md`

**Gist.** `dep` means "a relation exists, but it could not be determined". It is for odd constructions
and for failures of a converter or parser. The hand-checked EWT gold standard has almost none, so an
annotator that uses `dep` often is giving up on the analysis. LLMs do this on repetitions, ellipsis and
abbreviations.

**Conditions and exceptions.** GUM uses `dep` much more widely, for spoken fragments and, in older
releases, for number identifiers (*Page 3*). In EWT style, every `dep` is a reason to recheck the
parse. Ellipsis has its own mechanisms: `orphan` and promotion (see `gapping-orphan`).

**Examples.** *Then, as if to show that he could, …* → dep(show, if) (the guideline example).

**Machine-readable part.** `en.errors.dep-last-resort` (warn): every `dep` relation is flagged.

**Sources.** UD guidelines `en/dep/dep`; 2023.udw-1.7; Kellert et al. (2025.findings-emnlp.863).

### determiner-noun-number
`ai/en/seeds/errors/determiner-noun-number.md`

**Gist.** *this/that* and *a/an* go with singular nouns, and *these/those* go with plurals. A mismatch is
either an error in the text or a wrong Number feature on the noun. Learner corpora class these errors as
DET and NOUN:NUM.

**Conditions and exceptions.**
- The indefinite article is fine with a quantity phrase: *a few days*, *a good few*, *a full 20
  minutes*, *an estimated 850 people*, *a further 45*.
- Pronominal *those* (*those who*) is not `det`.
- The lemma of *those* is *that*, so the check uses the word form.

**Examples.** ✗ *this times*; ✗ *these question*; ✗ *a monthly breakfasts*; ✗ *an SAP identification
numbers*; ✓ *a few weeks*.

**Machine-readable part.** `en.errors.a-singular` (warn): *a* as `det` of a plural noun, unless a
quantity word (*few, good, full, estimated, further, extra, additional, mere, whopping, great, total*)
is attached to the noun. The demonstrative check is rule `en.nominal.dem-number` in
`ai/en/seeds/nominal/dem-number.md`.

**Sources.** BLiMP (2020.tacl-1.25, determiner–noun agreement); ERRANT (P17-1074); W19-4406; Fowler
1926, A, AN §2; Poutsma, *A Grammar of Late Modern English* III ch. XXVI, IV ch. XL.

### double-comparative
`ai/en/seeds/errors/double-comparative.md`

**Gist.** English forms the comparative either with a suffix (*happier*) or with *more/most* (*more
beautiful*), never with both: ✗ *more happier*, ✗ *most easiest*. ERRANT classes this as ADJ:FORM.

**Conditions and exceptions.** *more* as a quantity (*more people*) is out of scope. Only `advmod` on a
JJR/RBR counts. Archaic *most unkindest* is a stylistic exception.

**Examples.** ✗ *The customers are more happier.* → ✓ *happier*; ✓ *more beautiful*.

**Machine-readable part.** No rule here. The check is rule `en.nominal.double-comparison` in
`ai/en/seeds/nominal/double-comparison.md`. In UD the pattern is advmod(happier, more) with
*happier* as JJR.

**Sources.** ERRANT (P17-1074, ADJ:FORM); W19-4406.

### double-negation
`ai/en/seeds/errors/double-negation.md`

**Gist.** Standard English has one negation per clause: *I don't know anything* or *I know nothing*.
*not/n't* together with a negative pronoun or determiner in the same clause is negative concord (*I don't
know nothing*). It is normal in many dialects and in Slavic languages, but an error in the standard.

**Conditions and exceptions.** Deliberate double negation that makes a positive (*not unhappy*, *I can't
not go*) is fine. So is dialect speech quoted on purpose.

**Examples.** ✗ *I don't think nothing of it.* → ✓ *anything*; ✗ *didn't take no reason*.

**Machine-readable part.** `en.errors.double-negation-pronoun` (warn): a Polarity=Neg `advmod` and a
PronType=Neg dependent on the same predicate. `en.errors.double-negation-det` (warn): a Polarity=Neg
`advmod`, plus a PronType=Neg `det` inside an `obj`/`obl`/`nsubj` of the same predicate.

**Sources.** Findlay et al. (2025.udw-1.8); BLiMP (2020.tacl-1.25, NPI licensing).

### english-treebank-conventions
`ai/en/seeds/errors/english-treebank-conventions.md`

**Gist.** The 13 English UD treebanks (UD 2.18) differ in two ways: some lack certain columns, and some
annotate the same thing differently. A rule built on EWT can give false alarms elsewhere. The expert
system checks EWT 2.18 style, so on other treebanks compare violations with this list first.

**Conditions and exceptions.**
- Missing columns: no lemmas in ESLSpok, no FEATS in CHILDES, no XPOS in ATIS, CTeTex and LittlePrince,
  no multiword tokens in CTeTex, ESLSpok and LittlePrince.
- ATIS, ESLSpok, CHILDES and LittlePrince still use `:tmod`/`:npmod`.
- EWT vs GUM:

| phenomenon | EWT | GUM |
|---|---|---|
| *Sri Lanka, Hong Kong* | `compound`, head on the right | `flat`, head on the left |
| *Page 3* (before 2.11) | `nummod` | `dep` |
| *Marvel Consultants, Inc.* | head *Inc.* | head *Consultants* (`nmod:desc` since 2.15) |
| ADJ in names | often `compound` | `amod` |
| PP on a fragment root (*Good morning to all*) | `nmod` | `obl` |
| `dep` | rare | frequent |
| `iobj` outside double-object verbs | almost never | sometimes (*presented me*) |
| `orphan` outside a conjunct | never | sometimes |

Alignment across versions: 2.7–2.8 added MWTs for clitics, hyphens as separate tokens, and ADJ/VERB in
names. 2.10 moved proper names from `flat` to transparent syntax. 2.11 added `nsubj:outer` and revised
relative clauses, clefts and possessive lemmas. 2.15 introduced `:unmarked` and `nmod:desc`.

**Examples.** If a rule fires much more often on GUM than on EWT, that points to a convention difference,
not a faulty rule.

**Sources.** Zeldes & Schneider (2023.udw-1.7); Schneider & Zeldes (2021.udw-1.14); Dönicke et al.
(2020.udw-1.8).

### errant-taxonomy
`ai/en/seeds/errors/errant-taxonomy.md`

**Gist.** This background seed has no rule of its own. ERRANT classifies each text edit
deterministically from part-of-speech tags, lemmas and a dictionary. It has 25 main types (ADJ,
ADJ:FORM, ADV, CONJ, CONTR, DET, MORPH, NOUN, NOUN:INFL, NOUN:NUM, NOUN:POSS, ORTH, OTHER, PART, PREP,
PRON, PUNCT, SPELL, UNK, VERB, VERB:FORM, VERB:INFL, VERB:SVA, VERB:TENSE, WO). Each type combines with
an operation: M (missing), R (replaced) or U (unnecessary).

For verbs, ERRANT applies ordered tests: same in lower case → ORTH; not a word → SPELL; same lemma →
look at the form; one side VBG/VBN → FORM; VBD → TENSE; VBZ → SVA.

**Conditions and exceptions.** The types that leave a trace in a UD tree each have their own seed here:
- VERB:SVA (agreement), VERB:FORM (auxiliary chains), VERB:INFL (*goed*);
- DET and NOUN:NUM, ADJ:FORM (double comparison);
- PREP (verb prepositions), WO (adverb placement), PRON (pronoun case).

UD keeps text errors in the tree (see `literal-annotation-learner`). So the agreement and form rules in
this folder also work as grammatical-error detectors.

**Examples.** Verb rules from the paper: [iss → is] SPELL; [IS → is] ORTH; [has → is] VERB;
[being → is] VERB:FORM; [was → is] VERB:TENSE; [are → is] VERB:SVA.

**Sources.** Bryant et al. 2017 (P17-1074); Bryant et al. 2019 (W19-4406); Choshen et al.
(2020.conll-1.7, SERCL); Koyama et al. (2025.acl-long.1026, CTSEG).

### existential-there
`ai/en/seeds/errors/existential-there.md`

**Gist.** In *There is a ghost in the room*, *there* is a dummy subject: PRON, XPOS EX, relation
`expl`. The real subject is *ghost* (`nsubj`), and *is* is the root with UPOS VERB, not AUX and not
`cop`. The verb agrees with the subject that follows it (*There are two dogs*).

**Conditions and exceptions.** Locative *there* (*I went there*) is ADV with `advmod`. Colloquial
*there's* + plural is common, so the rule only warns. Fowler allows the singular before a coordinated
subject.

**Examples.** *There's a cow in the field* → expl('s, There), nsubj('s, cow), 's: VERB; ✗ *Is there any
tricks I could use?* → ✓ *Are there*.

**Machine-readable part.** `en.errors.there-be-verb` (warn): the head of expletive *there* must be
VERB. `en.errors.there-agreement` (warn): VBZ with expletive *there* and a plural `nsubj`. The rule
`en.nominal.ex-there` lives in `ai/en/seeds/nominal/there-expletive.md`.

**Sources.** UD guidelines `en/dep/expl`, `en/dep/cop`; Fowler 1926, NUMBER §2, 7; ERRANT (P17-1074,
VERB:SVA).

### flat-compound-direction
`ai/en/seeds/errors/flat-compound-direction.md`

**Gist.** An English compound noun has its head on the right (*phone book* → compound(book, phone)), so
`compound` dependents come before the head. `flat` is for headless names (*Hillary Rodham Clinton*):
every part attaches to the first word, so `flat` dependents always come after it. A `compound` after its
head, or a `flat` before its head, means a wrong head or a flipped analysis.

**Conditions and exceptions.** `compound:prt` (*made up*) follows the verb. Participles with particles
(*rusted out*) have `compound` after the head in EWT. A place name with the generic word first is `flat`
(*Lake Mead, Mount Everest*). With the generic word last it is `compound` (*Mirror Lake*). Treebanks
differ: for *Sri Lanka*, EWT has `compound` and GUM has `flat`.

**Examples.** *oil price futures* → compound(price, oil), compound(futures, price); *Carl XVI Gustaf* →
flat(Carl, XVI), flat(Carl, Gustaf).

**Machine-readable part.** No rule here. The checks are rules `en.nominal.flat-head-first`
(`ai/en/seeds/nominal/flat-names.md`) and `en.nominal.compound-head-final`
(`ai/en/seeds/nominal/compound.md`).

**Sources.** UD guidelines `en/dep/compound`, `en/dep/nmod-desc`, `en/dep/flat`; 2023.udw-1.7;
Schneider & Zeldes (2021.udw-1.14); Dönicke et al. (2020.udw-1.8).

### function-words-leaves
`ai/en/seeds/errors/function-words-leaves.md`

**Gist.** In UD, content words are heads. Function words (`aux`, `cop`, `case`, `mark`, `cc`, `det`,
`punct`) attach to them and have almost no dependents of their own. Only `fixed`, `goeswith`, `conj`
between function words (*before and after*) and a modifying `advmod` (*just before*) are allowed. An
auxiliary with a subject or object means the tree is upside down, in the PTB/SD style where the
function word is the head.

**Conditions and exceptions.** In VP ellipsis (*Mary will too*), *will* is the head of the clause, not
`aux` (see `vp-ellipsis-aux-head`). Coordinated function words linked by `conj` are allowed.

**Examples.** *He has eaten* → aux(eaten, has), nsubj(eaten, He), not nsubj(has, He); *in the house* →
case(house, in), det(house, the).

**Machine-readable part.** `en.errors.aux-cop-leaf` (error): `aux`/`aux:pass`/`cop` with a core or
content dependent. `en.errors.function-word-leaf` (warn): `cc`/`mark`/`case`/`det` with a content
dependent. `en.errors.punct-leaf` (error): `punct` with dependents.

**Sources.** UD guidelines `en/dep/aux_`, `en/dep/cop`, `en/dep/punct`; W15-2134; 2025.law-1.14.

### gapping-orphan
`ai/en/seeds/errors/gapping-orphan.md`

**Gist.** In *Marie went to Paris and Miriam to Prague*, the second verb is missing (gapping). UD
promotes one remnant (*Miriam*) to the verb's place, conj(went, Miriam), and attaches the other remnants
to it with `orphan`: orphan(Miriam, Prague). So the head of an `orphan` is always the promoted remnant:
a conjunct, or in fragments a root, `advcl` or `parataxis`.

**Conditions and exceptions.** `orphan` is not used when verbal material remains, as in right-node
raising or VP ellipsis. GUM identifies ellipsis more broadly and uses `orphan` in more contexts. In
learner text, the choice between ellipsis and asyndetic coordination depends on whether the sentence is
judged grammatical.

**Examples.** *Marie went to Paris and Miriam to Prague* → orphan(Miriam, Prague); *John bought and ate
an apple* → conj(bought, ate), obj(bought, apple), with no `orphan`.

**Machine-readable part.** `en.errors.orphan-head` (warn): the head of `orphan` must be
`conj`/`root`/`advcl`/`parataxis`.

**Sources.** UD guidelines `en/dep/orphan`; Corbetta et al. (2025.tlt-1.6); 2023.udw-1.7; W18-4918;
Dickinson & Ragheb 2015 (grammaticality in learner annotation).

### iobj-bare-nominal
`ai/en/seeds/errors/iobj-bare-nominal.md`

**Gist.** An English indirect object is only a bare nominal in a double-object construction: *She gave
me a raise* → iobj(gave, me), obj(gave, raise). A recipient with *to/for* (*gave it to me*) is `obl`,
because prepositional phrases are not core arguments in English. The indirect object stands between the
verb and the direct object.

**Conditions and exceptions.** The direct object can come earlier when it is fronted (*How much money
does the USA give NASA?*), so the order check only looks at an `obj` after the verb. Relative and
interrogative pronoun objects (*the book that I gave him*) are out of scope.

**Examples.** ✓ *give the children the toys* → iobj(give, children), obj(give, toys); ✓ *give the toys to
the children* → obl(give, children), case(children, to); ✗ iobj(give, children) together with
case(children, to).

**Machine-readable part.** `en.errors.iobj-no-case` (error): `iobj` with a `case` dependent.
`en.errors.iobj-before-obj` (warn): `iobj` after a nominal `obj` that follows the verb.

**Sources.** UD guidelines `en/dep/iobj`, `en/dep/obl`; Matsuda et al. (2026.udw-1.1).

### iobj-verb-classes
`ai/en/seeds/errors/iobj-verb-classes.md`

**Gist.** Only a limited class of verbs takes two bare objects:
- transfer: *give, hand, sell, pay, send, offer, owe, promise*;
- communication: *tell, ask, show, teach, write, email*;
- benefactive: *buy, get, make, find, save, cook*;
- persuading or informing with a clause: *convince, persuade, remind, warn, inform* + that/to (iobj +
  ccomp/xcomp in EWT).

*explain, describe, suggest, say, announce, mention, propose, introduce, donate* do not: ✗ *explain me the
rule* → ✓ *explain the rule to me*. LLMs make the opposite mistake and put `iobj` on the only object of
*help, let, call*.

**Conditions and exceptions.** The class is open: rare verbs like *envy, forgive, spare, fine, bill* also
take two objects. EWT uses `iobj` with *tell, ask, trust, notify* even without a direct object (*tell me
about it*), so the check looks at the verb, not at whether an `obj` is present. GUM uses `iobj` more
widely.

**Examples.** ✓ *She told me the news.* ✗ *Can you explain me this?* ✓ *Help me!* → obj(Help, me).

**Machine-readable part.** `en.errors.no-double-object` (warn): `iobj` under *explain, describe,
suggest, say, announce, mention, propose, introduce, donate, repeat, dedicate, confess*. The licensing
list is rule `en.lexicon.iobj-verb` in `ai/en/seeds/lexicon/verb-iobj-licensors.md`.

**Sources.** UD guidelines `en/dep/iobj`; ERRANT (P17-1074, VERB, PREP); 2026.udw-1.1.

### literal-annotation-learner
`ai/en/seeds/errors/literal-annotation-learner.md`

**Gist.** UD describes what is written, not what was meant: an ungrammatical sentence gets the tree of
its actual words ("literal reading", as in the TLE learner treebank). Spelling errors have their own
mechanism:
- the form stays as written;
- Typo=Yes is set;
- the lemma, tag and UPOS come from the intended word;
- the correction goes to MISC as CorrectForm.

Grammatical errors (agreement, verb form) do not get Typo. An LLM annotator tends to correct silently,
tagging what was meant. The agreement rules then stay silent and the text error is lost.

**Conditions and exceptions.** Deliberate non-standard forms (*should of*, *ain't*, *them boys*) are
Style, not Typo (see `vernacular-style`). A missing word is analysed as ellipsis. Extra words in speech
are `reparandum`. Whether a sentence counts as grammatical affects the choice between ellipsis,
coordination and listing, so this principle should be stated explicitly.

**Examples.** *He new it* → new: lemma *know*, VBD, Typo=Yes, CorrectForm=knew; *My wife know my
secret* → know: VBP, no Typo.

**Sources.** Berzak et al. (P16-1070, TLE); Masciolini et al. (2025.udw-1.17); Dickinson & Ragheb 2015;
Kyle et al. (2022.bea-1.7, SL2E).

### llm-annotator-failure-modes
`ai/en/seeds/errors/llm-annotator-failure-modes.md`

**Gist.** This background seed has no rule of its own. It lists the recurring ways LLMs fail at UD
annotation, each with a matching check in this folder:
1. Tags are better than trees, so check heads and relations first.
2. Arcs are too short: the model prefers the nearest head (see `obl-on-nominal`, `advcl-on-noun`,
   `vp-ellipsis-aux-head`).
3. Labels outside the tag set, including `det:poss` and old SD/UD v1 labels (`neg`, `dobj`,
   `nsubjpass`, `cc` on the first conjunct).
4. Rare relations (`iobj`, `csubj`) are unstable.
5. Inconsistency on repetitions, ellipsis and abbreviations.
6. Self-generated rules are too narrow and ignore word order.
7. Formal defects: invalid CoNLL-U, occasional cycles.
8. Accuracy drops as the number of clauses grows.
9. Silent correction of the text (see `literal-annotation-learner`).

**Conditions and exceptions.** Accuracy must be measured against an independent gold standard, not
against corrected LLM output. These help: a step-by-step order (UPOS → head → relation), tabular
output, examples chosen by similarity, and using the LLM as an editor of an existing parse. Mova's
"draft" mode works this way: the deterministic Rust annotator parses first, and an LLM corrects.

**Examples.** Typical LLM outputs: `det:poss` instead of `nmod:poss`; a prepositional phrase attached
to the nearest noun instead of the verb.

**Sources.** Bernhard et al. (2025.law-1.14); Machado & Ruiz (2024.propor-1.46); Kellert et al.
(2025.findings-emnlp.863); Matsuda et al. (2026.udw-1.1, 2025.iwpt-1.2); Ginn & Palmer
(2025.xllm-1.17); 2026.lrec-1.910.

### missing-article
`ai/en/seeds/errors/missing-article.md`

**Gist.** A singular count noun cannot stand bare. It needs an article, possessive, numeral or other
determiner (*I have a car*, not *I have car*). The superlative also needs *the*. Dropped articles are
among the most frequent errors of speakers whose language has no articles.

**Conditions and exceptions.** Telegraphic style in reviews and headlines (*Room was amazing*), fixed
phrases (*go to school, by car, in bed*), and mass readings (*give him room to progress*) are fine. The
rule covers only a few dozen clearly countable nouns used as subject or object.

**Examples.** ✗ *Now I have wife and son.*; ✗ *I have friend who drive from…*; ✗ *and did great job*.

**Machine-readable part.** `en.errors.bare-count-noun` (warn): an NN `obj`/`nsubj`/`nsubj:pass` from a
fixed list of count nouns (*car, book, friend, job, wife, …*) with no `det`, `nmod:poss`, `nummod` or
`det:predet`. It does not fire when the noun is sentence-initial (an article dropped at the start).

**Sources.** W19-4406 (DET); ERRANT (P17-1074, M:DET); Berzak et al. (P16-1070); Jespersen, *The
Philosophy of Grammar* (1924), on prosiopesis.

### missing-copula
`ai/en/seeds/errors/missing-copula.md`

**Gist.** English cannot drop *be*: *He is very happy*. Learners whose language has a zero present-tense
copula write ✗ *He very happy*. Under literal annotation, such a sentence has a nominal or adjectival
predicate with `nsubj` but no `cop`. The same tree also appears when an existing copula was attached to
the wrong word.

**Conditions and exceptions.** Telegraphic reviews and headlines (*Rooms clean.*) are fine. Absolute
constructions (*with the kids asleep*) are `advcl` and out of scope: the rule only checks `root`,
`conj`, `ccomp` and `parataxis`.

**Examples.** ✗ *Evidently, this a problem that…* → nsubj(problem, this) with no `cop`; ✓ *Rooms very
clean* (review style).

**Machine-readable part.** `en.errors.missing-copula` (warn): a NOUN/ADJ/PROPN predicate with a
preceding `nsubj` and no `cop`/`aux`. It does not fire if the sentence has no finite verb at all
(telegraphic style).

**Sources.** Berzak et al. (P16-1070); ERRANT (P17-1074, M:VERB); Dickinson & Ragheb 2015; UD guidelines
`en/dep/cop`.

### modal-do-to-base
`ai/en/seeds/errors/modal-do-to-base.md`

**Gist.** A modal (*can, will, must, should*), auxiliary *do* and the particle *to* require the base form
(VB): *can go*, *does not know*, *to see*. If another auxiliary comes in between (*could have gone*,
*will be going*, *to be told*), that auxiliary decides the form. Errors like *can goes*, *to going*,
*will meeting* are ERRANT VERB:FORM.

**Conditions and exceptions.** In an auxiliary chain, the last auxiliary before the lexical verb decides
its form. So if the verb is not VB, the rules require another auxiliary after the modal, *do* or *to*.
Dialectal *should of took* is covered in `vernacular-style`.

**Examples.** ✗ *We will meeting Rod's office.*; ✗ *it does has one*; ✓ *don't get married* (*get* is
`aux:pass`).

**Machine-readable part.** `en.errors.to-base`, `en.errors.modal-base`, `en.errors.do-base` (warn): a
VERB tagged VBN/VBG/VBD/VBZ/VBP with *to* (`mark`), an MD `aux` or *do* `aux`, and no further
auxiliary after it.

**Sources.** ERRANT (P17-1074, VERB:FORM); W19-4406; Koyama et al. (2025.acl-long.1026); UD guidelines
`en/dep/aux_`, `en/dep/mark`.

### much-many-countability
`ai/en/seeds/errors/much-many-countability.md`

**Gist.** *many* goes with plural count nouns (*many books*), and *much* goes with singular mass nouns
(*much time*). Mass nouns have no plural: *information, advice, furniture, equipment, knowledge,
homework, luggage, evidence, feedback, software* (✗ *informations*). These errors are typical of
speakers whose language treats these nouns as countable.

**Conditions and exceptions.** Adverbial *much* (*much bigger*, *not much*) is out of scope; only
`amod`/`det` on a noun counts. *many a man* is singular (literary). Nouns with established plurals in
some senses (*accommodations, damages, researches, staffs*) are not on the mass list.

**Examples.** ✗ *this has caused much difficulties*; ✗ *much monies*; ✗ *storage for your luggages*.

**Machine-readable part.** `en.errors.much-plural` (warn): *much* on a plural noun.
`en.errors.many-singular` (warn): *many* on a singular noun. The mass-noun plural check is rule
`en.nominal.mass-no-plural` in `ai/en/seeds/nominal/mass-nouns.md`.

**Sources.** ERRANT (P17-1074, NOUN:INFL, NOUN:NUM); W19-4406.

### negation-not
`ai/en/seeds/errors/negation-not.md`

**Gist.** The negative particle *not/n't* in English UD is PART with XPOS RB, relation `advmod` and
feature Polarity=Neg. Negative pronouns and determiners (*no, nothing, nobody, none*) carry
PronType=Neg. The SD/UD v1 label `neg` no longer exists. The features sit on the negative word, not on
the predicate, so they record form, not meaning.

**Conditions and exceptions.** *not* can have other relations: `cc` with ExtPos=CCONJ in *not only …
but*, `conj` in *whether or not*, and `fixed`. So the rule checks only UPOS, XPOS and the feature.
Treebanks without features (ESLSpok) fail it by convention.

**Examples.** *He did n't go* → advmod(go, n't); n't: PART, RB, Polarity=Neg. *no problem* →
det(problem, no); no: DET, PronType=Neg.

**Machine-readable part.** `en.errors.not-part` (error): every *not/n't* must be PART, RB,
Polarity=Neg.

**Sources.** Findlay et al. (2025.udw-1.8); UD guidelines `en/feat/Polarity`.

### nmod-on-predicate
`ai/en/seeds/errors/nmod-on-predicate.md`

**Gist.** This is the mirror image of `obl-on-nominal`: `nmod` is only for dependents of nouns. A
prepositional phrase on a verb is always `obl`, and on an adjective too (*afraid of sharks*). `nmod`
under a verb is left over from UD v1, which used one `nmod` for all prepositional phrases.

**Conditions and exceptions.** Quantity adjectives used as pronouns take `nmod`: *most of them*, *many
of the people*, *much of the time*, *few of us*.

**Examples.** *people afraid of sharks* → obl(afraid, sharks); *He talked about it* → obl(talked, it);
*most of them* → nmod(most, them).

**Machine-readable part.** `en.errors.nmod-on-verb` (error): `nmod` under a VERB.
`en.errors.nmod-on-adj` (warn): `nmod` under an ADJ other than *most, many, much, few, more, little,
less, least, several, enough*.

**Sources.** UD guidelines `en/dep/obl`, `en/dep/nmod`, English migration guidelines; 2023.udw-1.7.

### nominal-rels-on-verb
`ai/en/seeds/errors/nominal-rels-on-verb.md`

**Gist.** Some relations belong to the noun phrase by definition: `amod` (adjective on a noun), `acl`
(clause on a noun) and `case` (preposition in a nominal). If their head is a verb, either the relation
or the head's part of speech is wrong:
- a preposition before a gerund or clause is `mark`, not `case` (*by using it*);
- a clause on a verb is `advcl`, not `acl`;
- an adjective on a verb is `xcomp` (*got rich*) or `advmod`, not `amod`.

**Conditions and exceptions.** EWT tags *following* in *the following* as VERB with `det`. A work title
that is a clause (*"What We've Lost", published by…*) gives `acl` from a VERB.

**Examples.** *by using it* → mark(using, by); *the man sitting there* → acl(man, sitting); *He left,
crying* → advcl(left, crying).

**Machine-readable part.** `en.errors.acl-on-verb`, `en.errors.amod-on-verb`, `en.errors.case-on-verb`
(warn): the respective relation under a VERB.

**Sources.** UD guidelines `en/dep/case`, `en/dep/mark`, `en/dep/acl`, `en/dep/amod`; 2025.law-1.14.

### numbered-entities-nummod
`ai/en/seeds/errors/numbered-entities-nummod.md`

**Gist.** `nummod` is a numeral that says "how many": *3 sheep*, *forty dollars*. A number after a noun
is an identifier, not a quantity (*page 394*, *Route 66*, *World War II*). Under the UD 2.18 guidelines
it is `flat` from the generic word. So `nummod` after a noun is an error, and so is `nummod` on a
non-numeral (*several, many* are `amod` or `det`).

**Conditions and exceptions.** Money: in *$ 40*, `nummod($, 40)` is correct. The rule only checks NOUN
and PROPN heads. A phone number after *number:* is `appos`. Dates and addresses are `nmod:unmarked`.

**Examples.** *Sam ate 3 sheep* → nummod(sheep, 3); *page 394* → flat(page, 394); *door number 3* →
flat(door, number), flat(number, 3).

**Machine-readable part.** No rule here. The checks are rules `en.nominal.nummod-before-noun` and
`en.nominal.nummod-num` in `ai/en/seeds/nominal/nummod.md`.

**Sources.** UD guidelines `en/dep/nummod`, `en/dep/nmod-desc` (Numbered Entities); 2023.udw-1.7;
2021.udw-1.14; Kalpakchi & Boye (2021.udw-1.8).

### obj-without-preposition
`ai/en/seeds/errors/obj-without-preposition.md`

**Gist.** `obj` is a direct object without a preposition. As soon as the phrase has a `case`
preposition, it is `obl` on a verb or `nmod` on a noun, and the same holds for subjects. Prepositional
verbs (*depend on, look at, listen to, refer to*) have a semantic "object", but syntactically it is
`obl`.

**Conditions and exceptions.** Possessive *'s* is also `case`, but it attaches to the possessor
(`nmod:poss`), so the rule only counts ADP. Inversion with a fronted prepositional phrase (*Under the
bed is a box*) is `obl` + `nsubj` in EWT.

**Examples.** *I depend on you* → obl(depend, you), case(you, on); *Refer to our brochure* →
obl(Refer, brochure); ✗ obj(depend, you) with case(you, on).

**Machine-readable part.** `en.errors.obj-no-preposition` (error): `obj` with an ADP `case`.
`en.errors.subject-no-preposition` (warn): any `nsubj*` with an ADP `case`.

**Sources.** UD guidelines `en/dep/obj`, `en/dep/obl`, `en/dep/nsubj`.

### obl-on-nominal
`ai/en/seeds/errors/obl-on-nominal.md`

**Gist.** A prepositional phrase is labelled by what it attaches to: `obl` on a verb, adjective or
adverb, and `nmod` on a noun. So `obl` under a noun means either the phrase really belongs to the verb
(wrong head) or the label should be `nmod`. Confusing `obl` and `nmod` is the most common error of
English parsers, and it is almost always a prepositional-phrase attachment error (*eat a pizza with a
fork* vs *with anchovies*).

**Conditions and exceptions.**
- A nominal predicate with a copula is a legitimate head for `obl`.
- In verbless fragments, GUM attaches the phrase to the nominal root as `obl` (*Good morning to all*),
  while EWT uses `nmod`.
- Email headers in EWT (*X on 01/25/2002*) have `obl` without a copula.

**Examples.** *a preference for lilies* → nmod(preference, lilies); *we prefer lilies to daisies* →
obl(prefer, daisies); *eat a pizza with a fork* → obl(eat, fork), but *with anchovies* →
nmod(pizza, anchovies); *He was a teacher in Boston* → obl(teacher, Boston), because cop(teacher, was)
is present.

**Machine-readable part.** `en.errors.obl-on-nominal` (warn): `obl` under a NOUN/PROPN/PRON that has
no `cop`.

**Sources.** UD guidelines `en/dep/obl`, `en/dep/nmod`; Zeldes & Schneider (2023.udw-1.7); Peng &
Zeldes (W18-4918); 2025.law-1.14.

### one-subject-one-object
`ai/en/seeds/errors/one-subject-one-object.md`

**Gist.** A predicate has at most one subject (`nsubj`, `nsubj:pass`, `csubj`) and at most one direct
object. Two subjects mean the roles are mixed up. A typical case is an object relative clause (*the book
that the students read*): *that* is `obj` and *students* is `nsubj`, but an annotator makes both
subjects. Another case is a subordinate-clause subject attached to the main verb.

**Conditions and exceptions.** The outer subject of a copular sentence with a clausal predicate is
`nsubj:outer` and does not count (*The problem is that this has never been tried*). Coordinated subjects
are one `nsubj` plus `conj`.

**Examples.** *the book that the students read* → obj(read, that), nsubj(read, students); *The problem is
that this has never been tried* → nsubj:outer(tried, problem), nsubj:pass(tried, this).

**Machine-readable part.** `en.errors.one-subject` (error): a second subject on the same predicate.
`en.errors.one-object` (error): a second `obj` on the same predicate.

**Sources.** UD guidelines `en/dep/nsubj`, `en/dep/nsubj-outer`, `en/dep/obj`, `en/dep/cop`;
2023.udw-1.7; BLiMP (2020.tacl-1.25).

### overregularized-verbs
`ai/en/seeds/errors/overregularized-verbs.md`

**Gist.** Learners and children regularise irregular verbs with *-ed*: *goed, buyed, thinked, catched*
(ERRANT VERB:INFL). Under UD conventions and EWT practice, the wrong form stays as written. It gets
Typo=Yes, the lemma and tag of the intended word (VBD/VBN), and CorrectForm in MISC.

**Conditions and exceptions.** Real words that look like such forms (*seed, leaved, payed*) are not on
the list.

**Examples.** *I buyed it* → buyed: lemma *buy*, VBD, Typo=Yes, CorrectForm=bought; ✗ *We goed home*
without Typo=Yes.

**Machine-readable part.** `en.errors.overregularized-typo` (warn): a form from a fixed list (*goed,
comed, buyed, thinked, …, weared*) without Typo=Yes.

**Sources.** ERRANT (P17-1074, VERB:INFL); BLiMP (2020.tacl-1.25, irregular forms); Masciolini et al.
(2025.udw-1.17).

### passive-agent
`ai/en/seeds/errors/passive-agent.md`

**Gist.** In a passive clause, the *by*-phrase naming the agent is `obl:agent`. But *by* can also mark
means or a time limit (*transported by land*, *by the 1920s*), and then it is plain `obl`. The meaning
decides. A formal hint: an agent usually has an article or possessive (*by the dog*, *by his wife*),
while means is a bare noun (*by car*, *by hand*).

**Conditions and exceptions.** Temporal *by* with an article (*by the end of the year*) is `obl`. Proper
names and pronoun agents (*by John*, *by them*) are not checked, because they have no article.

**Examples.** *The cat was chased by the dog* → obl:agent(chased, dog); *Arms are transported by land*
→ obl(transported, land).

**Machine-readable part.** `en.errors.by-agent` (warn): in a passive (VBN with `aux:pass`), a plain
`obl` with `case` *by* and a `det`/`nmod:poss` should probably be `obl:agent`.

**Sources.** UD guidelines `en/dep/obl`, `en/dep/obl-agent`.

### passive-structure
`ai/en/seeds/errors/passive-structure.md`

**Gist.** The English passive is *be*/*get* + past participle (VBN). So `aux:pass` and `nsubj:pass` occur
only with a VBN head, and with `aux:pass` the subject must be `nsubj:pass`. Typical mistakes:
- the subject of an intransitive change-of-state verb (*The door opened*) is labelled `nsubj:pass`;
- *be* + an *-ed* adjective (*is interested*) is taken for a passive;
- the perfect is confused with a copula.

**Conditions and exceptions.** Reduced passives without an auxiliary are legitimate. Examples are
headlines (*Passport needed*) and absolutes (*many staffed by former officers*): `nsubj:pass` on a VBN
with no `aux:pass`.

**Examples.** *The cat was chased by the dog* → nsubj:pass(chased, cat), aux:pass(chased, was),
obl:agent(chased, dog); *The door opened* → nsubj(opened, door); *He got fired* → aux:pass(fired, got).

**Machine-readable part.** `en.errors.nsubjpass-vbn` (error): `nsubj:pass` with a non-VBN head.
`en.errors.auxpass-vbn` (error): `aux:pass` with a non-VBN head. `en.errors.auxpass-subject` (error):
`aux:pass` together with a plain `nsubj`.

**Sources.** UD guidelines `en/dep/nsubj-pass`, `en/dep/aux-pass`, `en/dep/cop`; Bernhard et al.
(2025.law-1.14); ERRANT (P17-1074, VERB:FORM).

### perfect-have-vbn
`ai/en/seeds/errors/perfect-have-vbn.md`

**Gist.** Perfect *have* requires a past participle: *has gone*, *have seen*. ✗ *has went*, ✗ *have saw*,
✗ *has go* confuse VBD and VBN, typically with irregular verbs (ERRANT VERB:FORM, VERB:INFL). If another
auxiliary follows *have* (*has been going*), that auxiliary decides the form.

**Conditions and exceptions.** *have to* ("must") and lexical *have* are VERB, not `aux`. For regular
verbs, VBD and VBN look the same (*missed*), so here the rule catches a tagging error, not a text error.

**Examples.** ✗ *He has went home.* ✓ *He has been working.*

**Machine-readable part.** `en.errors.have-vbn` (warn): a VERB tagged VB/VBD/VBZ/VBP/VBG with
perfect *have* as `aux` and no further auxiliary after it.

**Sources.** ERRANT (P17-1074); BLiMP (2020.tacl-1.25, irregular forms); UD guidelines `en/dep/aux_`.

### plural-subject-singular-verb
`ai/en/seeds/errors/plural-subject-singular-verb.md`

**Gist.** A present-tense verb agrees with its subject in number: *The results reduce*, not *reduces*.
A plural subject with a VBZ verb, copula or auxiliary has one of two causes:
- a real agreement error in the text (ERRANT VERB:SVA);
- the wrong word was picked as subject, often an attractor (the nearest noun instead of the true head,
  as in *The key to the cabinets*).

Both people and models make more errors when the attractor stands right before the verb.

**Conditions and exceptions.** These are fine with VBZ:
- plural-form names with singular meaning (*The United States goes*), so PROPN subjects are skipped;
- measures (*5 kg per gun means*);
- *as follows* with `nsubj:outer`.

Subjects after the verb (*there's lots of…*) are handled in `existential-there`.

**Examples.** ✗ *The results of the test reduces the gas.*; ✗ *My parents even plays with them.*;
✗ *what people knows about Philippines*.

**Machine-readable part.** `en.errors.plural-subject-vbz` (warn): a plural NOUN/PRON subject before a
VBZ verb. `en.errors.plural-subject-vbz-auxcop` (warn): the same with a VBZ copula or auxiliary. Both
are exempt when the subject has a `nummod` (measures: *Three hours isn't far*).

**Sources.** Fowler 1926, NUMBER §4; Zacharopoulos et al. (2023.emnlp-main.998); Wilson et al.
(2023.scil-1.24); BLiMP (2020.tacl-1.25); ERRANT (P17-1074), W19-4406.

### possessives-nmod-poss
`ai/en/seeds/errors/possessives-nmod-poss.md`

**Gist.** In English UD, possessive pronouns (*my, your, his, their, whose*) and the possessive *'s* are
`nmod:poss`. A possessive pronoun is PRON with Poss=Yes, and the possessor always comes before what it
possesses. English UD does not use `det:poss` (other languages do), but LLMs produce it.

**Conditions and exceptions.** A possessive as a conjunct (*his and her*) is `conj`. Independent *mine,
hers* take other roles (nsubj, obj). *a friend of mine* is `nmod` with case *of*.

**Examples.** *my office* → nmod:poss(office, my); *the president's office* →
nmod:poss(office, president), case(president, 's); *whose car* → nmod:poss(car, whose).

**Machine-readable part.** No rule here. The checks are rules `en.nominal.prp-poss-rel`
(`ai/en/seeds/nominal/poss-pronouns.md`) and `en.nominal.poss-before-head`
(`ai/en/seeds/nominal/nmod-poss.md`).

**Sources.** UD guidelines `en/dep/nmod-poss`, `en/dep/nmod`, `en/feat/Poss`.

### preposition-errors
`ai/en/seeds/errors/preposition-errors.md`

**Gist.** Prepositions are one of the most frequent learner error types (PREP). ERRANT distinguishes
three kinds:
- wrong (R:PREP): *depend of → depend on*, *arrive to → arrive at/in*;
- unnecessary (U:PREP): *discuss about → discuss*;
- missing (M:PREP): *listen music → listen to music*.

The preposition a verb takes is lexical knowledge that has to be stored per verb. In UD the error shows
up as `obl` with the wrong `case`, or as `obj` where `obl` is needed.

**Conditions and exceptions.** *depending on* is a fixed expression. In *arrived to find…*, *to* is
`mark` (purpose infinitive), not `case`, so it is not caught.

**Examples.** ✗ *Depends of what you want.*; ✗ *We discussed about the plan.*; ✗ *I listen music.*

**Machine-readable part.** `en.errors.depend-on` (warn): *depend* + `obl` whose `case` is not
*on/upon*. `en.errors.discuss-no-about` (warn): *discuss* + *about*. `en.errors.arrive-not-to` (warn):
*arrive* + *to*. `en.errors.listen-to` (warn): *listen* with an `obj`.

**Sources.** ERRANT (P17-1074, PREP); W19-4406; Koyama et al. (2025.acl-long.1026).

### progressive-be-vbg
`ai/en/seeds/errors/progressive-be-vbg.md`

**Gist.** Non-passive auxiliary *be* (`aux`) requires the *-ing* form: *is working*. A base or finite form
after *be* (✗ *they are find*, ✗ *he is hate*) is a text error (VERB:FORM) or a wrong annotation. The
only legitimate "be + infinitive" is *be to* with the particle *to* (*He was to leave at noon*). EWT
annotates *was* there as `aux` of the infinitive.

**Conditions and exceptions.** Passive *be* + VBN is `aux:pass` (see `passive-structure`). In *is being
done*, *is* is `aux`, *being* is `aux:pass`, and the head is VBN.

**Examples.** ✗ *they are find more interest in oil drilling*; ✓ *The United States was to cut its
level.*

**Machine-readable part.** `en.errors.be-progressive` (warn): a VERB tagged VB/VBD/VBZ/VBP with *be* as
`aux` and no *to* `mark`.

**Sources.** ERRANT (P17-1074, VERB:FORM); UD guidelines `en/dep/aux_`; 2025.law-1.14.

### pronoun-after-preposition
`ai/en/seeds/errors/pronoun-after-preposition.md`

**Gist.** A pronoun after a preposition is in the accusative (*for me*, *with them*), and so is the
second member of a coordination: *between you and me*. The hypercorrection ✗ *between you and I* is an
old error of educated speakers (Fowler also cites *let you & I try*). ERRANT classes it as PRON.

**Conditions and exceptions.** The check has two parts: a nominative pronoun with its own preposition,
and a nominative pronoun that is a conjunct in a phrase whose preposition attaches to the first
conjunct. It also catches wrong Case=Nom features on *it* in the gold standard.

**Examples.** ✗ *between you and I* → case(you, between), conj(you, I), I: Case=Nom; ✓ *between you and
me*.

**Machine-readable part.** `en.errors.nominative-after-preposition` (warn): a Case=Nom personal pronoun
with an ADP `case`. `en.errors.nominative-conjunct-after-preposition` (warn): a Case=Nom personal pronoun
as `conj` of a word that has a preceding ADP `case`.

**Sources.** Fowler 1926, ME; UD guidelines `en/feat/Case`; ERRANT (P17-1074, PRON).

### real-word-confusions
`ai/en/seeds/errors/real-word-confusions.md`

**Gist.** The most frequent real-word errors in web text are homophones: *their/there/they're*,
*to/too*, *then/than*, *where/were*, *your/you're*, *its/it's*. A spell checker misses them because each
is a real word. EWT annotates such a word as the intended word and sets Typo=Yes: *there* used for
*their* gets PRP$ and `nmod:poss`. So a form with another word's tag and no Typo=Yes is either a tagging
error or a missing Typo mark.

**Conditions and exceptions.** Only unambiguous form + tag pairs are checked. Adverbial *then* (*and
then*) and *to* as ADP/PART are legitimate. *your/you're* and *its/it's* involve tokenisation (multiword
tokens) and are left out.

**Examples.** *there car* → there: PRP$, nmod:poss, Typo=Yes, CorrectForm=their; *better then me* →
then: IN, case, Typo=Yes; *it was to hot* → to: ADV (RB), advmod, Typo=Yes.

**Machine-readable part.** `en.errors.there-as-their` (*there*/PRP$), `en.errors.their-as-there`
(*their*/EX or RB), `en.errors.to-as-too` (*to*/ADV), `en.errors.then-as-than` (*then*/IN),
`en.errors.where-as-were` (*where*/AUX). All are warn and require Typo=Yes.

**Sources.** EWT 2.18 practice (Typo=Yes + CorrectForm); Masciolini et al. (2025.udw-1.17); ERRANT
(P17-1074); W19-4406.

### relative-clause-agreement
`ai/en/seeds/errors/relative-clause-agreement.md`

**Gist.** When the relative pronoun (*who, which, that*) is the subject of the relative clause, the verb
takes the number of the noun the clause attaches to: *people who live*, *a man who lives*. A mismatch
means a text error, or that the clause was attached to the wrong noun (*the servant of the actress
who…*).

**Conditions and exceptions.** In *one of the best men that have ever lived*, the antecedent is *men*,
so the plural is correct (singular *has* is common but, per Fowler, a constant error). Collective nouns
(*the Taliban want*) agree by meaning.

**Examples.** ✗ *the people who lives here* → ✓ *live*; ✓ *He is one of the best men that have ever
lived.*

**Machine-readable part.** `en.errors.relcl-plural-antecedent` (warn): a plural antecedent, a
PronType=Rel subject, and the relative verb is VBZ. `en.errors.relcl-singular-antecedent` (warn): a
singular noun antecedent with a VBP relative verb. It is exempt for coordinated antecedents and for
quantity or collective nouns.

**Sources.** Fowler 1926, NUMBER; Garcia et al. (2025.depling-1.4); BLiMP (2020.tacl-1.25); ICLE-RC
(2025.law-1.16).

### relative-pronoun-in-clause
`ai/en/seeds/errors/relative-pronoun-in-clause.md`

**Gist.** Since UD 2.11, the relative pronoun (*who, which, that*) is a member of the relative clause
(`nsubj`, `obj`, `obl`, …). It attaches to the clause's predicate, and the clause is `acl:relcl` of the
antecedent. Relative *that* is PRON (WDT) with a role, not `mark`. `mark` *that* occurs only in
complement clauses (*the fact that he left*). A pronoun attached directly to the antecedent means an
inverted analysis.

**Conditions and exceptions.** Free relatives (*whatever deal you want*: *whatever* is `det` of *deal*)
and nested relatives are exceptions. The rule only checks a pronoun that stands between the antecedent
and the relative predicate.

**Examples.** *the man who left* → nsubj(left, who), acl:relcl(man, left); *the book that I read* →
obj(read, that), that: PRON; *the fact that he left* → mark(left, that), acl(fact, left).

**Machine-readable part.** `en.errors.relpron-inside-clause` (warn): a PronType=Rel PRON attached to
the antecedent, between it and the `acl:relcl`. The *that*-not-`mark` check is rule
`en.nominal.relative-that-not-mark` in `ai/en/seeds/nominal/relative-that.md`.

**Sources.** 2023.udw-1.7; UD guidelines `en/dep/acl-relcl`, `en/dep/mark`; ICLE-RC (2025.law-1.16);
Czerniak et al. (2025.tlt-1.12).

### singular-subject-plural-verb
`ai/en/seeds/errors/singular-subject-plural-verb.md`

**Gist.** A singular noun subject requires VBZ: *My wife knows*, not *know*. VBP with a singular noun
subject and no conjunct is a text agreement error or a wrong subject. A dropped third-person *-s* is
among the most frequent errors of non-native writers.

**Conditions and exceptions.** These agree by meaning:
- quantity nouns with a plural *of*-phrase (*a lot of people say*, *the majority want*, *the rest of us
  pay*);
- collective nouns in British usage (*Argentina intend*, *the family grumble*).

A coordinated subject (with `conj`) is skipped.

**Examples.** ✗ *this dentist want to pull the tooth*; ✗ *The coffee taste burnt.*; ✓ *A lot of people
say…*

**Machine-readable part.** `en.errors.singular-subject-vbp` (warn): a singular NOUN/PROPN subject before
a VBP verb, with no `conj`. `en.errors.singular-subject-vbp-auxcop` (warn): the same with a VBP copula or
auxiliary. Both exempt quantity lemmas (*lot, number, majority, rest, half, …*) and collective lemmas
(*family, team, government, committee, staff, police, …*).

**Sources.** ERRANT (P17-1074), W19-4406 (VERB:SVA); Fowler 1926, NUMBER; Wilson et al.
(2023.scil-1.24); Hobbs et al. (2026.conll-main.7); Poutsma, *GLME* III ch. XXVI; Curme 1931.

### subject-pronoun-case
`ai/en/seeds/errors/subject-pronoun-case.md`

**Gist.** The subject of a finite verb is nominative (*I, he, she, we, they*). ✗ *Me and my family are
moving*, ✗ *them came up with*, ✗ *her has a place* are colloquial or learner forms. In non-finite
constructions an accusative subject is normal (*for me to sit in*, *him stuttering*). So the check only
applies to predicates with VerbForm=Fin.

**Conditions and exceptions.** Reflexives as subjects (*Myself and Credit were calling*) are also
non-standard. A wrong Case=Acc on *it* in EWT is caught too.

**Examples.** ✗ *Me and my family are moving.*; ✓ *It would be OK for me to sit in.*; ✓ *I remember him
stuttering.*

**Machine-readable part.** `en.errors.finite-subject-nominative-auxcop` (warn): a Case=Acc PRON subject
before a finite copula or auxiliary. The verb-level check is rule `en.nominal.finite-subject-nom` in
`ai/en/seeds/nominal/pron-case-function.md`.

**Sources.** UD guidelines `en/feat/Case`; ERRANT (P17-1074, PRON); W19-4406; Poutsma, *GLME* IV ch.
XXXII §8.

### titles-and-company-suffixes
`ai/en/seeds/errors/titles-and-company-suffixes.md`

**Gist.** A title before a name (*Mr., Dr., Professor*) and a company suffix (*Inc., Corp., Ltd., LLC*)
are optional descriptors: drop them and the name is still grammatical. Since UD 2.15 they are
`nmod:desc` of the core name. Earlier they were `flat` or `compound`. `flat` is wrong because first and
last name then do not form a constituent. Against `compound`: a title agrees in number (*Presidents
Obama and Biden*), which compound dependents almost never do.

**Conditions and exceptions.** *Co. 1691* ("company no. 1691") and *Peters and Co.* (a conjunct) are not
descriptors, so *Co.* is excluded. A position with an article is `appos` (*the president, Joe Biden*). A
regnal number (*Elizabeth II*) is `flat`.

**Examples.** *Mr. Magoo* → nmod:desc(Magoo, Mr.); *Apple Inc.* → nmod:desc(Apple, Inc.); *JFK Jr.* →
nmod:desc(JFK, Jr.).

**Machine-readable part.** `en.errors.title-desc` (error): *Mr./Mrs./Ms./Dr.* (not a typo) right before
a PROPN must be `nmod:desc` of it. `en.errors.company-suffix-desc` (warn): *Inc./Corp./Ltd./LLC/PLC*
after a PROPN must be `nmod:desc`.

**Sources.** UD guidelines `en/dep/nmod-desc`; Schneider & Zeldes (2021.udw-1.14); 2023.udw-1.7.

### tmod-npmod-unmarked
`ai/en/seeds/errors/tmod-npmod-unmarked.md`

**Gist.** Bare nominal adverbials and modifiers (*We met last year*, *five days before the funeral*, *$5 a
share*) were split into `:tmod` (time) and `:npmod` (other) until UD 2.15. Since 2.15, EWT and GUM use a
single `:unmarked` subtype (`obl:unmarked`, `nmod:unmarked`). The old label comes from annotators
trained on older releases, or from treebanks that keep the old scheme.

**Conditions and exceptions.** For ATIS, ESLSpok, CHILDES and LittlePrince, the old labels are their
convention. Mova follows EWT 2.18, so for Mova they are errors. The CoNLL 2018 LAS metric ignores
subtypes, so this difference does not affect LAS.

**Examples.** *We met last year* → obl:unmarked(met, year); *IBM earned $5 a share* →
nmod:unmarked($, share); *five days before the funeral* → nmod:unmarked(funeral, days).

**Machine-readable part.** `en.errors.tmod-npmod` (error): any `obl:tmod`, `obl:npmod`, `nmod:tmod`,
`nmod:npmod`.

**Sources.** UD guidelines `en/dep/obl-unmarked`, `en/dep/nmod-unmarked`, `en/dep/nmod`; UD issues
#1028, #1094.

### tough-vs-extraposition
`ai/en/seeds/errors/tough-vs-extraposition.md`

**Gist.** Two similar sentences have different trees.
- **Extraposition** (*It is easy to make money*): the subject is the infinitive clause, *it* is an
  `expl` placeholder, and the infinitive is `csubj`.
- **Tough construction** (*This book is easy to read*): the subject is *book*, and the infinitive
  complements the adjective (`ccomp` in EWT, sometimes `advcl`). `csubj` is impossible there, because a
  subject already exists.

**Conditions and exceptions.** Tough adjectives: *easy, hard, difficult, impossible, tough, simple,
pleasant*. *too early to say* is a different construction with *too*.

**Examples.** *It is hard to say* → expl(hard, It), csubj(hard, say); *Fish are the easiest to take care
of* → nsubj(easiest, Fish), ccomp(easiest, take).

**Machine-readable part.** `en.errors.extraposed-infinitive-csubj` (warn): an ADJ with `expl` and a
following *to*-infinitive, where the infinitive must be `csubj`. `en.errors.tough-infinitive-not-csubj`
(warn): an ADJ with a nominal `nsubj` and a following *to*-infinitive, where the infinitive must not be
`csubj`.

**Sources.** UD guidelines `en/dep/csubj`, `en/dep/expl`; BLiMP (2020.tacl-1.25, tough vs raising).

### vernacular-style
`ai/en/seeds/errors/vernacular-style.md`

**Gist.** EWT separates errors (Typo=Yes) from deliberate colloquial or dialect forms (Style=Vrnc, Coll,
Slng):
- *should of took*: *of* is AUX, lemma *have*, VB, Style=Vrnc;
- *them boys*: *them* is DT (`det`), Style=Vrnc;
- *ain't*: *ai* has lemma *be*, Style=Vrnc;
- *cos/coz/cus*: lemma *because*, Abbr=Yes, Style=Vrnc;
- *ya, 'em*: Style=Coll.

An annotator that "corrects" these loses register information. One that misses *have → of* breaks the
tree, because *of* becomes a preposition without a noun.

**Conditions and exceptions.** The line between a typo and a deliberate form is blurry: *walkin, goin*
have both Style=Vrnc and Typo=Yes in EWT.

**Examples.** *he should of called back* → of: AUX, lemma *have*, VB, Style=Vrnc, aux(called, of);
*them apples* → them: DT, det, Style=Vrnc.

**Machine-readable part.** `en.errors.of-have-vernacular` (warn): *of* as AUX must have lemma *have* and
Style=Vrnc. `en.errors.them-det-vernacular` (warn): *them* as `det` must have Style=Vrnc.

**Sources.** EWT 2.18 practice (Style feature); Masciolini et al. (2025.udw-1.17).

### vp-ellipsis-aux-head
`ai/en/seeds/errors/vp-ellipsis-aux-head.md`

**Gist.** In *John will win gold and Mary will too*, the second *will* has lost its verb. Basic UD has no
empty nodes, so the auxiliary becomes the head of the second clause: conj(win, will₂), nsubj(will₂,
Mary). A wrong analysis attaches *will₂* as `aux` of the first *win*. That gives a long backward arc and
an auxiliary after its head, which never happens in ordinary clauses.

**Conditions and exceptions.** Inversion with a fronted participle or gerund: *Attached is the file* (an
`aux:pass`, not checked), *Sailing with the Roosevelt is…*, *Compounding this is…*. `orphan` is not used
in VP ellipsis.

**Examples.** *He can swim and I can too* → conj(swim, can₂), nsubj(can₂, I); ✗ aux(swim, can₂).

**Machine-readable part.** `en.errors.aux-before-verb` (warn): an `aux` that follows its VERB head.

**Sources.** UD guidelines `en/dep/orphan` ("In VP-ellipsis, we keep the auxiliary as the head");
Corbetta et al. (2025.tlt-1.6); Kellert et al. (2025.findings-emnlp.863); 2025.law-1.14.

### xcomp-no-subject
`ai/en/seeds/errors/xcomp-no-subject.md`

**Gist.** `xcomp` is an open complement: its subject comes from the main clause through control or
raising (*I want to leave*, *She seems happy*). A complement with its own subject is closed, so it is
`ccomp` (*I know that he left*, *He said he would go*). Parsers often confuse `xcomp` and `ccomp`.

**Conditions and exceptions.** With object control, the object belongs to the main verb, not to the
`xcomp` (*I want him to go* → obj(want, him), xcomp(want, go)). A *for*-clause (*for him to go*) has
`nsubj` in EWT and so is not `xcomp`.

**Examples.** *I want to go* → xcomp(want, go); *They made him leave* → obj(made, him),
xcomp(made, leave); *He said he would go* → ccomp(said, go), nsubj(go, he).

**Machine-readable part.** `en.errors.xcomp-no-subject` (warn): `xcomp` with an `nsubj`/`nsubj:pass`
dependent.

**Sources.** UD guidelines `en/dep/xcomp`, `en/dep/ccomp`; BLiMP (2020.tacl-1.25, control/raising);
de Marneffe & Manning 2008 (W08-1301).
