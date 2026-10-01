# The Mova dialect of Universal Dependencies (`ai/dialects/mova`)

**`mova`** is Mova's own internal variant of Universal Dependencies. Treebanks and annotation schemes that differ in conventions — UD releases, the customs of individual treebanks, SUD, Stanford Dependencies, classic Penn Treebank conversions — are converted *into* `mova`; the English tagger/parser `en` is trained on it; and its output is converted *out of* `mova` into any target (for example EWT 2.18 or the latest UD). Quality is measured against each treebank's own gold standard through the reverse converter.

**What `mova` consists of:**
- **Base:** UD 2.18 with English Web Treebank (EWT) conventions (the feature/relation registry is `en/data/ud-registry-en.tsv`).
- **Layers on top of the base:**
  - `ud-next` — guideline changes made after the 2.18 release that are expected in 2.19 (`dialects/ud-next/`);
  - selected borrowings from alternative dialects;
  - own normalizations: `det:poss` → `nmod:poss`; `:tmod`/`:npmod` → `:unmarked`; verb FEATS (`Number`, `Person`) taken from the subject.
- **Selection principle:** prefer the more consistent and more informative analysis, provided it can be converted to the standard without loss. A lossy scheme is only ever an export target, never the base.

## How converters work

Conversion rules are ```` ```convert ```` blocks in `convert.md` files under `treebanks/` and `dialects/` (and occasionally inside a seed). They use the same matching language as the English grammar seeds (`match`, `require`, `unless`; see `ai/en/seeds`), plus `from`/`to` (source and target dialect) and `set` actions that change a node's relation, UPOS, XPOS, lemma or features (`feats+=X=V`, `feats-=X`, `lemma=@form`, `feats+=X=@v` to copy a feature from another node):

```convert
rule: ewt.imperative-do-main
what: main VB with imperative "do" gets Mood=Imp|VerbForm=Fin
from: ewt
to: mova
match: v[xpos=VB, feats.VerbForm=Inf]; d[lemma=do, rel=aux, head=v, feats.Mood=Imp]
set: v[feats+=VerbForm=Fin; feats+=Mood=Imp]
source: EWT 2.18
```

Application is deterministic:
1. rules for a `from → to` pair run in file order (paths alphabetically) and block order within a file;
2. for each rule, all bindings are found on the current state of the sentence (all `require` clauses hold, no `unless` clause holds); action values are read from that same state, and then all actions are applied together;
3. two actions that set different values on the same field of the same word are an error (each FEATS feature and each MISC key counts as a separate field);
4. after all rules, a tree gate checks that heads are in range, there is exactly one root, `root` is used only on it, and there are no cycles.

Lines the actions do not touch are carried over byte for byte: comments, multiword tokens, empty nodes, DEPS, MISC, and features outside the known feature set. The engine is run as `en convert`; `en convert-check` reports what a rule set changes on a corpus.

## The seeds in this folder

Each seed in `dialects/mova/seeds/` records one `mova` convention where UD 2.18 guidelines, post-2.18 changes and EWT data could point in different directions. Each entry below states what `mova` does, its scope, examples, and how it relates to standard export. Status values: **adopted** (part of `mova`), **pending** (`mova` keeps current EWT practice until UD settles the question), **not applicable**.

Only three conventions change `mova` annotation relative to EWT 2.18: case and lemma on *who*-pronouns (`wh-case`), adverbs moved out of `discourse` (`discourse-adverbs`), and imperative main verbs that EWT marks `Inf` (`imperative-do`). The rest confirm that `mova` follows EWT data where the English guideline text is out of date.

---

### clausal-predicate
`ai/dialects/mova/seeds/clausal-predicate.md`

**Status.** Adopted.
**Gist.** When the predicate of a copular sentence is a clause, `mova` follows the UD 2.10 amendment and EWT: the predicate of the inner clause is the head, *be* is `cop`, and the subject of the outer clause is `nsubj:outer` (or `csubj:outer` for a clausal outer subject). The older analysis with the copula as head and the clause as `ccomp` is not used.
**Conditions and exceptions.** The copula stays `cop` in all sentences, with no "transitive copula" exception. *Be* with `ccomp` remains only for quotative *be like* and constructions such as *there are hints … that*. Export to UD 2.18 is identical; the older analysis can be derived deterministically from `nsubj:outer` + `cop` if a conversion to UD v1 or Stanford Dependencies needs it.
**Examples.** *The problem is that these sentences are difficult*: head *difficult*, *is* — `cop`, *problem* — `nsubj:outer`, *sentences* — `nsubj`.
**Sources.** UD 2.18 `docs/changes.md` (Multiple subjects); UD `docs/_en/specific-syntax.md` (older analysis); checked by `en.verbal.outer-subject-needs-cop` and `en.verbal.cop-verb-head-outer` (`ai/en/seeds/verbal`).

### depictives
`ai/dialects/mova/seeds/depictives.md`

**Status.** Adopted.
**Gist.** An optional depictive adjective (a secondary predicate describing a participant's state during the event) is `advcl` of the verb, as in the UD 2.10 amendment, `_en/dep/advcl.md` and EWT data. The older `acl` and `advmod` analyses are not used.
**Conditions and exceptions.** The depictive's own subject (e.g. *she*), where visible, is expressed in Enhanced UD, not in the basic tree. Examples in `_en/specific-syntax.md` involving *unable* and *assured* are annotated differently in EWT (`parataxis` and verbal `advcl`) and should not be used as depictive examples. Export to UD 2.18 and `ud-next` is identical.
**Examples.** *She entered the room **sad**.* — *came back **dead*** — *came into office **obsessed** with Iraq*.
**Sources.** UD 2.18 `docs/changes.md` (Optional depictives); UD `docs/_en/dep/advcl.md`; UD `docs/_en/specific-syntax.md`.

### discourse-adverbs
`ai/dialects/mova/seeds/discourse-adverbs.md`

**Status.** Adopted.
**Gist.** `discourse` is reserved for interjections, discourse particles, list numbering and emoticons, following the post-2.18 definition. An adverb is never `discourse` but `advmod`; a prepositional phrase is never `discourse` but `obl`.
**Conditions and exceptions.** `discourse` remains for INTJ (*oh, well, like, um, yes*), item numbering (NUM: *1.*, *(a)*), emoticons (SYM), *thanks*, *ps*. The criterion is the word class, visible without interpreting pragmatics. Conversion EWT/GUM → `mova` uses rule `mova.ud-next.discourse-adv` (`dialects/ud-next/convert.md`); `mova` → 2.18/`ud-next` needs no change, since the 2.18 guideline already excluded adverbs. On a round trip, the few EWT/GUM tokens annotated `discourse` on adverbs differ from the gold file; reverse-conversion reports should list them on a separate line. DEPS still carry `N:discourse` and need a DEPS action or an EUD rebuild.
**Examples.** `advmod`: *though, so, also, maybe, btw*; `obl`: *in other words*; `discourse`: *oh, well, um, yes, 1., (a)*.
**Sources.** UD `ud-next` `_u-dep/discourse.md`; UD 2.18 discourse guideline ("non-adverbial discourse markers"); gates `udnext.discourse-not-adv`, `udnext.discourse-not-pp` (error).

### feat-layer-ids
`ai/dialects/mova/seeds/feat-layer-ids.md`

**Status.** Adopted.
**Gist.** Any layered feature identifiers that `mova` introduces (for example, for agreement with several participants in other languages) are named with lowercase Latin letters only, `[a-z]+`, without digits.
**Conditions and exceptions.** The post-2.18 guideline requires this, although the validator still accepts digits; digits are expected to become an error. Nothing changes for English, which has no feature layers.
**Examples.** None in the seed (non-English illustration: an Old Georgian layer renamed `sauf2` → `dsauf`).
**Sources.** UD `docs/_u-overview/feat-layers.md`; UD validator `utils.py`.

### features-context
`ai/dialects/mova/seeds/features-context.md`

**Status.** Principle adopted; `Exponence` pending.
**Gist.** Feature values are taken from context, but where context and word form conflict, the form wins (post-2.18 clarification no. 18). `mova` keeps its "FEATS from the subject" normalization with an explicit limit: agreement never overrides an unambiguous form. `VBZ` is always `Number=Sing|Person=3`; `VBP` is never `Sing|3`; *was* is always `Sing`.
**Conditions and exceptions.** With notional agreement (*the team **are***, *a number of people **are***) the verb gets `Plur|3` from its form rather than a value the standard does not allow. `VBD` and `VBP` take `Number`/`Person` from the subject, and *you*/*it* get `Case` by position, as in EWT. The `Exponence` MISC attribute is not used until UD standardizes its values. Export to 2.18 is unchanged.
**Examples.** *the team are* → `Plur|3` on *are*; *a number of people are*.
**Sources.** UD `docs/changes.md` (Morphosyntactic features, clarification no. 18); UD issue #1233; gates `udnext.vbz-3sg`, `udnext.vbp-not-3sg` (applied to `en` output and silver data).

### imperative-do
`ai/dialects/mova/seeds/imperative-do.md`

**Status.** Adopted.
**Gist.** In an imperative with *do*, both *do* and the main `VB` verb have `Mood=Imp|VerbForm=Fin` — following EWT practice rather than the letter of `_en/feat/VerbForm.md` (which would give the main verb `Inf`). So *Go!* and *Don't go!* are annotated the same way on the lexical verb.
**Conditions and exceptions.** The minority of EWT tokens with `VerbForm=Inf` in this construction are aligned by the EWT → `mova` rule `ewt.imperative-do-main` (shown in the introduction); there is no reverse rule, so these tokens appear as a separate line in reverse-conversion reports. The literal `VerbForm.md` form is derived deterministically: a `VB` with `aux` *do* carrying `Mood=Imp` → `VerbForm=Inf`, `Mood` removed. *Don't* with `Mood=Ind` in imperatives is an EWT error.
**Examples.** *Don't worry.* — *Do not hesitate.* — *DO NOT GO HERE.* — *Don't cling.* — *don't let that fool you*.
**Sources.** UD `docs/_en/feat/VerbForm.md`; EWT 2.18 data; English seed `ai/en/seeds/verbal/imperative.md` (`en.verbal.imperative-do-main-verb`; `en.verbal.vb-aux-infinitive` excludes imperative *do*).

### iobj-sole
`ai/dialects/mova/seeds/iobj-sole.md`

**Status.** Adopted.
**Gist.** Following the UD 2.12 "Sole iobj" amendment and EWT (rather than the older text of `_en/dep/iobj.md` and `_en/specific-syntax.md`), `iobj` may be the only internal argument and may co-occur with `xcomp` when the verb takes an addressee or recipient.
**Conditions and exceptions.** `iobj` + `xcomp`: *ask/tell/allow/convince/urge/persuade/cause/trust/teach/remind/warn X to do*. `obj` + `xcomp`: *let, make, get, have, keep, find, want, help*. The boundary between the classes is set by the lexicon (`ai/en/seeds/lexicon/verb-iobj-licensors.md`, VerbNet), not by examples; *permit* and *recommend*, which have both analyses in EWT, follow the gold data until the lexicon rule decides. Export to EWT 2.18 is identical; `iobj` → `obj` (for the literal `_en` text) is deterministic.
**Examples.** *remind me* — *ask Bush* — *ask X to do* (`iobj`) vs *make X do* (`obj`).
**Sources.** UD 2.18 `docs/changes.md` (Sole iobj); UD `docs/_en/dep/iobj.md`; UD `docs/_en/specific-syntax.md`; VerbNet.

### modal-mood
`ai/dialects/mova/seeds/modal-mood.md`

**Status.** Pending (UD issues #1118, #1155).
**Gist.** Modals (`MD`) keep the EWT convention: `VerbForm=Fin` without `Mood`, `Number` or `Person`. `Mood=Pot/Nec/Cnd` is not added, and neither is `Mood=Ind`.
**Conditions and exceptions.** `Mood=Pot/Nec/Cnd` would make export invalid, since the English feature registry allows only `Imp|Ind|Sub`; `Mood=Ind` would wrongly assert indicative for *might*, *would*. The modal meaning is still available from the lemma: the auxiliary registry gives the function of each of the 16 auxiliary lemmas, and a clause-level "possibility/necessity" feature can be derived from it. The validator's `verbform-fin-without-mood` warning on these tokens is expected. Revisit when #1118 closes or `_en/feat/Mood.md` changes.
**Examples.** None in the seed beyond the modal lemmas (*can* — possibility, *must* — necessity, *would* — conditional).
**Sources.** UD `docs/_u-feat/Mood.md`; UD auxiliary data (`data.json`); UD issues #1118, #1155, #1233.

### no-effect
`ai/dialects/mova/seeds/no-effect.md`

**Status.** Accepted as is / not applicable.
**Gist.** A list of post-2.18 changes that do not affect English `mova` annotation: general documentation of `expl:pass` and `expl:pv`; validator data; the English treebank list; other languages and the website; UD v1 labels in `specific-syntax.md`.
**Conditions and exceptions.** The Perl script `conllu_convert_uposf_to_xpos.pl` is not part of the conversion pipeline (pipeline mechanics are in Rust). Examples taken from `specific-syntax.md` must have v1 labels replaced (`dobj` → `obj`, etc.). For a future language-independent layer, `expl:pass`/`expl:pv` (a subtype shared by three language groups), `flat:redup` and `expl:rel` are noted. Validator data are to be compared again before 2.19.
**Examples.** None.
**Sources.** Linked seeds in `dialects/ud-next/seeds/` and `dialects/ud-2.18/seeds/`.

### spoken-subtypes
`ai/dialects/mova/seeds/spoken-subtypes.md`

**Status.** Pending.
**Gist.** The spoken-language subtypes documented for French after 2.18 — `discourse:filler`, `discourse:tag`, `conj:reform` — are not used in `mova` for now.
**Conditions and exceptions.** They would be more informative (a filler *um* is not an interjection *oh*; a tag question is not ordinary parataxis) and could be exported losslessly by dropping the subtype (`discourse:filler` → `discourse`). They wait until the UniDive spoken-language working group's guidelines are merged into UD, or until spoken English treebanks are taken up; they are not in the English registry, so export would have to strip them.
**Examples.** *um* (filler) vs *oh* (interjection).
**Sources.** UD `ud-next` French spoken subtypes; UD issues #1290, #1280, #1273, #1289.

### subjunctive
`ai/dialects/mova/seeds/subjunctive.md`

**Status.** Adopted.
**Gist.** The subjunctive is marked `Mood=Sub`, as in EWT, GUM and ParTUT, rather than following the note in `_en/feat/Mood.md` that maps it to `Inf`/`Ind`. Present subjunctive (a bare `VB` without an auxiliary, with its own subject, after *suggest/insist/recommend that*, *it is vital that*, etc.): `Mood=Sub|Tense=Pres|VerbForm=Fin`, person and number from the subject. Past subjunctive *were* with a singular subject: `Mood=Sub|Tense=Past`.
**Conditions and exceptions.** Examples that EWT left as `Inf` are not corrected automatically; rule `ud218.subjunctive-as-inf` only suggests candidates for a correction pass. The literal `Mood.md` form is derived deterministically (`VB Sub` → `VerbForm=Inf` with `Mood`, `Tense`, `Number`, `Person` removed; `VBD Sub` → `Mood=Ind`), but the reverse is not deterministic, so `Sub` must live on the `mova` side, not the export side. Export to EWT 2.18 is identical.
**Examples.** *suggest that he see a doctor* — *it is vital that she be present* — *if I were* — *were I to*.
**Sources.** UD `docs/_en/feat/Mood.md`, `docs/_en/feat/Tense.md`; checked by `en.verbal.subjunctive-present-feats` (`ai/en/seeds/verbal/subjunctive.md`).

### to-phrase-obl
`ai/dialects/mova/seeds/to-phrase-obl.md`

**Status.** Adopted.
**Gist.** A *to*-phrase attached to a verb (e.g. a recipient: *gave it to me*) is `obl`, as in EWT and the universal `obl`/`nmod` guidelines. The `nmod` reading in `_en/dep/iobj.md` is a UD v1 leftover.
**Conditions and exceptions.** Examples taken from `_en/dep/iobj.md` into the English seeds annotate *to me* as `obl`. The validator checks the `nmod`/`obl` boundary (`obl-should-be-nmod`). Export is identical.
**Examples.** *to me* (`obl`).
**Sources.** UD `docs/_en/dep/iobj.md`; UD `docs/changes.md` (no. 17); universal `obl`, `nmod` pages.

### wh-case
`ai/dialects/mova/seeds/wh-case.md`

**Status.** Adopted (from `ud-next`).
**Gist.** *Whom* has lemma *who* (*whomever* → *whoever*). *Who*/*whoever* get `Case=Nom` or `Case=Acc` according to their syntactic function; *whom*/*whomever* are always `Acc`, since the form wins. *Whose* with a noun (`nmod:poss`) is `Case=Gen|Poss=Yes`; independent *whose* is `Poss=Yes`.
**Conditions and exceptions.** Case is taken from the base function of *who*; in a few free relatives and predicative uses it comes from the position inside the subordinate clause (edge cases described in `dialects/ud-next/convert.md`, rules `mova.ud-next.who-*`). Export to 2.18 uses deterministic reverse rules (`ud-next.who-case-drop`, `ud-next.whose-case-drop`, `ud-next.whom-lemma`): remove `Case` and restore the lemma from the form; an EWT round trip is identical. The English registry already allows `PRON Case=Nom|Acc|Gen` and `Poss=Yes`.
**Examples.** *the man **who** I saw* (`Acc`) vs *the man **who** saw me* (`Nom`).
**Sources.** UD `ud-next` `_en/pos/PRON.md`; UD issue #517; gates `udnext.whom-who-acc`, `udnext.who-case`, `udnext.whose-dependent` (error); hints `udnext.who-object-acc`, `udnext.who-subject-nom`, `udnext.whose-independent` (warn); `en.morph.whom-acc` (`ai/en/seeds/morph/pron-wh.md`).

### xpos-not-empty
`ai/dialects/mova/seeds/xpos-not-empty.md`

**Status.** Adopted.
**Gist.** XPOS is never an empty string: an unknown XPOS is written as `_`. This is enforced by the CoNLL-U writer for all export targets, not only `ud-next` (validator 0.2.8, `empty-string-in-xpos`, which 2.19 treats as a level-2 error).
**Conditions and exceptions.** Mainly relevant for future languages with free-string XPOS; in `en`, XPOS is always a Penn Treebank tag. The seed rule language cannot see raw columns, so the gate lives in the writer code, with a negative control: a line with empty XPOS must produce an error. `_` is also valid in 2.18, so there is no loss.
**Examples.** None.
**Sources.** UD validator 0.2.8 (`empty-string-in-xpos`).
