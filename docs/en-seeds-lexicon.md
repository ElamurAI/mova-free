# English seeds: lexicon and valency (`ai/en/seeds/lexicon`)

A **seed** is a short Markdown note that states one fact of English grammar, with its sources. A checkable rule grows from it. This folder covers the **lexicon**: lists of verbs that license, or rule out, particular complements. These include bare vs *to*-infinitives, gerund vs infinitive objects, indirect objects, retained objects in the passive, verbs without a passive, copula-like verbs, raising and control, and stative verbs in the progressive. Morphology (tags, lemmas, inflection) lives in `ai/en/seeds/morph`.

## How seeds fit into `ai/en`

`ai/en` is a deterministic English parser written in Rust: tokenizer, PTB tagger, lemma/form morphology, dependency parser and word-order generator.

- **The lexicon tables are compiled; the seeds are not.** `ai/en/build.rs` compiles `data/lemmas.tsv` and `data/forms.tsv` (every form with its tag, lemma, UD frequency and source) into static open-addressing hash tables (`lexicon.bin`). It also writes word constants (`words.rs`) for the 5,000 most frequent lemmas plus `data/consts.txt`, so a rule in Rust code compares integers, not strings.
- **Rules are interpreted.** The ```` ```rule ```` blocks in these seeds are loaded from the seeds directory at runtime by the rule engine `en::expert`. The command `en expert-check <seeds dir> [--gold <file>] <file>` runs them over CoNLL-U, and annotation tools use them as hints for a reviewer ("token 5 breaks rule X"). The `cargo test` gate `seeds_all_parse` requires every rule to parse and every rule id to be unique. All lexicon rules have severity `warn`: a hit means "suspect: non-native text, a different sense of the verb, or an annotation error", not a certain error.

A lexicon rule usually binds a verb `v` by lemma list and a dependent by relation (`xcomp`, `obj`, `iobj`, `mark`, `aux`), then requires or forbids something:

```rule
rule: en.lexicon.copula-like-no-object
what: become/seem/appear/remain/stay/tend take no direct object (the predicative is xcomp)
match: v[lemma=become|seem|appear|remain|stay|tend]; o[rel=obj, head=v]
require: not o[rel=obj]
severity: warn
source: UD docs/_en/dep/cop.md; Jespersen MEG III ch. XVII
```

Fields: `match` (nodes `name[cond, …]` separated by `;`), `require` (clauses `name[…]`, `not name[…]`, `exists c[…]`, `none c[…]`, with ` or ` for alternatives), optional `unless` (exceptions), `severity` (`error` or `warn`) and `source` (required). Conditions include `lemma=`, `form=`, `upos=`, `xpos=`, `rel=`, `rel~` (relation by base), `head=`, `feats.X=V`, `feats.X` / `!feats.X`, word order (`before=`, `after=`, `next=`, `prev=`), `suffix=`/`prefix=`, and negation with `!=`. The full description is in the morphology seeds document.

## Seed format

Sections: **Gist**, **Conditions and exceptions**, **Examples** (`*` marks an ungrammatical example), **In UD** (how it looks in UD/EWT annotation), **Sources** ( Brown 1851, Poutsma, Jespersen's *Modern English Grammar* (MEG), Reed & Kellogg, VerbNet 3.4, PropBank 3.1, UD documentation, ACL Anthology ids), then zero or more ```` ```rule ```` blocks.

## Seeds

### verb-bare-infinitive
`ai/en/seeds/lexicon/verb-bare-infinitive.md`

**Gist.** After verbs of causing and permitting (*make, let, bid*, causative *have*) and of perception (*see, hear, feel, watch, notice, observe*), an infinitive with an object takes no *to*: *You make me blush*, *I heard him say so*. *Help* allows both (*help me (to) move*). In the passive, *to* returns: *He was made to wait*, *He was seen to go*.

**Conditions and exceptions.** *See* meaning "recognize, understand" takes *to be* (*I saw it to be so*). *Feel* takes a bare infinitive only for physical sensation; for opinion it takes *to* (*I feel it to be my duty*). In older English *know* also took a bare infinitive (*I have known him do it*). *Have* is left out of the rule because *have to* is a different construction. In UD the infinitive is `xcomp` (VB, `VerbForm=Inf`) of the verb: without `mark(to)` in the active, with `mark(to)` when the governing verb has `Voice=Pass`.

**Examples.** *They let him leave.* — *\*Nobody saw him to leave* (active: no *to*). — *He was heard to say so* (passive: with *to*).

**Sources.** Brown 1851, *The Grammar of English Grammars*, Rule XIX and Obs. 5, 8, 10, 11; Poutsma 1923, *The Infinitive…*, §§35, 43; Jespersen MEG V ch. XVIII.

**Rules.** `en.lexicon.bare-infinitive-active` (warn): an active *make/let/see/hear/watch/feel/bid/notice/observe* with a VB `xcomp` has no `mark` *to* on it. `en.lexicon.to-infinitive-after-passive` (warn): the same verbs (except *let*) with `Voice=Pass` require `mark` *to* on the VB `xcomp`.

### verb-copula-like
`ai/en/seeds/lexicon/verb-copula-like.md`

**Gist.** Verbs of becoming (*become, get, grow, turn, go, come, fall*: *go mad, fall ill, come true*) and of being or seeming (*seem, appear, look, sound, feel, remain, stay, keep, prove*) link the subject to an adjective or noun just as *be* does: *I became very upset*, *It remains a mystery*. In UD, though, only *be* is a copula. These are ordinary VERBs, and the predicative is `xcomp`, not `obj`, because the noun after *become* is not acted on.

**Conditions and exceptions.** Many of these verbs also have transitive senses (*prove a theorem, sound the alarm, keep a secret, feel the heat, get a letter, turn the page*, rare *That dress becomes you*). So the rule covers only those that almost never have a transitive sense: *become, seem, appear, remain, stay, tend*.

**Examples.** *She became a doctor* — `xcomp(became, doctor)`. — *It seems a good idea* — `xcomp`. — *He proved his identity* — `obj` (a different sense).

**Sources.** UD `docs/_en/dep/cop.md`; UD `docs/_en/specific-syntax.md` (functional control); Jespersen MEG III ch. XVII–XVIII; Reed & Kellogg, *Higher Lessons in English*, Lesson 29; PropBank 3.1 frames `become.xml`, `seem.xml`.

**Rules.** `en.lexicon.copula-like-no-object` (warn): *become, seem, appear, remain, stay, tend* have no `obj`.

### verb-gerund-object
`ai/en/seeds/lexicon/verb-gerund-object.md`

**Gist.** Some verbs take an infinitive complement; others take only a gerund (*-ing*): *I enjoy swimming*, not *\*I enjoy to swim*. Gerund-only verbs include verbs of admitting and denying (*admit, acknowledge, deny*), avoiding and stopping (*avoid, escape, finish, give up, leave off, quit, stop* "cease"), postponing (*postpone, put off, delay, defer*) and *mind, miss, risk, resist, resent, relish, imagine, fancy, contemplate, consider, suggest, practise, enjoy, appreciate, keep*.

**Conditions and exceptions.** *Stop to smoke* is a different construction: *to smoke* is a purpose clause (`advcl`), not a complement. *Try, encourage, urge* are listed among gerund verbs by Poutsma but also occur with an infinitive (in another sense or with a personal object), so they are not in the rule. Non-native text often breaks this pattern (*I enjoyed very much to study here*). In UD the gerund is `xcomp` with VBG (`VerbForm=Ger`); an `xcomp` with VB + `mark(to)` under these verbs is suspect.

**Examples.** *He carefully avoided giving the least sign.* — *I have just finished dusting.* — *\*She denied to know him.*

**Sources.** Poutsma, *A Grammar of Late Modern English* (GLME), Part II, ch. XIX §§17–18; Poutsma 1923, *The Gerund*, §§43–45; UD `docs/_en/feat/VerbForm.md`.

**Rules.** `en.lexicon.gerund-verb-no-to-infinitive` (warn): *enjoy, avoid, escape, finish, mind, consider, suggest, deny, admit, acknowledge, risk, miss, imagine, fancy, contemplate, postpone, delay, defer, practise/practice, quit, appreciate, resist, resent, relish, keep* do not take a VB `xcomp` marked with *to*.

### verb-infinitive-object
`ai/en/seeds/lexicon/verb-infinitive-object.md`

**Gist.** Verbs of wishing, intending, deciding, trying and being able take a *to*-infinitive: *I want to go*, *We decided to stay*, *She managed to escape*, *He pretended not to see*. A gerund is impossible after them (*\*I want going*). Here the infinitive points forward in time relative to the main action, unlike the gerund.

**Conditions and exceptions.** Colloquial *want* + object + *-ing* (*You don't want it getting too warm*) is a different construction (object plus participle). *Seem/tend to be going* is a progressive infinitive, where the VBG has its own *to be*. Verbs that take both constructions (*begin, start, continue, like, love, hate, prefer*; with a change of meaning *remember, forget, stop, try, regret*) are not in the list. In UD a VBG `xcomp` under these verbs must have its own `aux`/`mark` (*to be going*); without them it is suspect.

**Examples.** *We hope to see you.* — *He refused to answer.* — *\*She decided going home.*

**Sources.** Poutsma 1923, *The Gerund*, §44; Poutsma GLME Part II ch. XIX §§19–20; Jespersen MEG V ch. XII.

**Rules.** `en.lexicon.infinitive-verb-no-gerund` (warn): a VBG `xcomp` of *want, hope, decide, agree, refuse, promise, plan, expect, manage, fail, afford, offer, pretend, seem, tend, wish, choose, deserve, threaten, learn* must have an `aux` or `mark` dependent.

### verb-iobj-licensors
`ai/en/seeds/lexicon/verb-iobj-licensors.md`

**Gist.** An indirect object (a recipient, addressee or beneficiary without a preposition) occurs only with verbs that allow two objects: *give him a book, tell them a story, buy her a present, bake me a cake*. These are verbs of transfer (*give, hand, lend, owe, sell, send, pay*), communication (*tell, show, teach, email, write*), benefaction (*buy, build, cook, fix, find, get, make*), cost and charging (*cost, charge, bill, fine*) and promising (*promise, offer, guarantee*). Speech and persuasion verbs that combine an addressee with a clause or infinitive also qualify (*inform, notify, remind, warn, advise, assure, convince, persuade, urge, ask, allow, cause*).

**Conditions and exceptions.** Since UD 2.12 the addressee stays `iobj` even without a direct object (*tell them*, *remind me of Vietnam*, *allow radicals to launch operations*), as long as the verb can in principle take a second object. Verbs without that possibility (*help, question*) take `obj` (*She helps her students to succeed*). EWT also uses `iobj` with *trust* (*trust me*) and *thank* (*thank god*); this is EWT convention, not grammar, and the rule may flag such cases for review. The list combines VerbNet double-object classes (filtered by EWT frequency) with speech verbs that take `ccomp`/`xcomp`; the full VerbNet list has 279 lemmas, and the rule uses 171.

**Examples.** *It took me two hours* (take). — *He quoted me a price* (quote). — *Do yourself a favour* (do). — *\*She explained me the rule* (*explain* takes no `iobj`: an error in the text or the annotation).

**Sources.** VerbNet 3.4 classes with frames "NP V NP-Dative NP", "NP V NP.beneficiary NP", "NP V NP NP" (give-13.1, send-11.1, bring-11.3, throw-17.1, slide-11.2, carry-11.4, get-13.5.1, steal-10.5, future_having-13.3, pay-68, bill-54.5, cost-54.2, build-26.1, create-26.4, preparing-26.3, performance-26.7, feeding-39.7, transfer_mesg-37.1.1, instr_communication-37.4); PropBank 3.1 (give.01, tell.01); UD `docs/changes.md` ("Sole iobj"); UD `docs/_u-dep/iobj.md`; Jespersen MEG III ch. XIV; ACL papers P17-1074, 2026.udw-1.1.

**Rules.** `en.lexicon.iobj-verb` (warn): the head of an `iobj` must be one of 171 listed lemmas (*accord, advise, afford, allocate, allow, answer, ask, … give, … tell, text, thank, throw, … wish, write*).

### verb-no-passive
`ai/en/seeds/lexicon/verb-no-passive.md`

**Gist.** Only verbs with a real affected object form a passive. Verbs describing an event, state or motion without an agent (*happen, occur, exist, arrive, arise, emerge, belong, consist, matter, seem, appear, remain*) have neither a direct object nor a passive (*\*It was happened*). Some verbs with an object do not passivize either: *cost, last, resemble* (*It costs two shillings*, but not *\*Two shillings are cost*).

**Conditions and exceptions.** In PropBank, the first rolesets of these verbs (happen.01, occur.01, exist.01, arrive.01, seem.01, remain.01, consist.01, matter.01, last.01, cost.01, resemble.01) have no agent role ARG0, which is a formal sign that there is no doer. *Die* takes a cognate object (*die a hero's death*), and *fall* has a passive-like form only in archaic *be fallen*, so both are excluded. Brown: the passive is formed only from active-transitive verbs. In UD these lemmas have no `Voice=Pass`, and *happen, occur, exist, arrive, arise, emerge, belong, consist, matter, seem, appear, remain* have no `obj`.

**Examples.** *What happened?* — *\*The accident was happened.* — *The book costs ten dollars* (no passive). — *\*It had only happened times* (error: *happen* with `obj`).

**Sources.** Jespersen MEG III §15.12 and ch. XVI "Transitivity"; Brown 1851, Part II, ch. VI ("Form of Passive Verbs"); PropBank 3.1 frames.

**Rules.** `en.lexicon.no-passive-verb` (warn): *happen, occur, exist, arrive, appear, seem, remain, stay, become, consist, matter, last, cost, resemble, belong, arise, emerge* have no `Voice=Pass`. `en.lexicon.unaccusative-no-object` (warn): *happen, occur, exist, arrive, arise, emerge, belong, consist, matter, seem, appear, remain* have no `obj`.

### verb-passive-retained-object
`ai/en/seeds/lexicon/verb-passive-retained-object.md`

**Gist.** A passive verb normally has no direct object, since the object has become the subject. An object is retained only with two-object verbs whose recipient has become the subject (*I was given a horse*, *He was told the truth*, *She was offered a job*, *We were charged a fee*) and in idioms like *was taken care of*. The predicative of naming and electing verbs (*He was elected president*, *was called a fool*) is `xcomp`, not an object.

**Conditions and exceptions.** *Titled/entitled/headed X* (titles) have `obj` in EWT. Any other `obj` under `Voice=Pass` is suspect. Either the verb is not a passive (a perfect without *been*), or the "object" is really `xcomp`/`obl`, or it is an error. In UD, `obj` under a `Voice=Pass` head is allowed only for two-object verbs, naming verbs and *take* (*take care*).

**Examples.** *I was given a horse* — `nsubj:pass(I)`, `obj(horse)`. — *Pakistan was gifted a slice of the territory.* — *The story was featured ARD* (suspect).

**Sources.** Jespersen MEG III ch. XV "Subject of Passive Verb"; Reed & Kellogg, *Higher Lessons in English*, Lesson 129; UD `docs/_en/dep/cop.md`; VerbNet 3.4 dative classes (see `verb-iobj-licensors`).

**Rules.** `en.lexicon.passive-object-verb` (warn): a `Voice=Pass` verb with an `obj` must be one of *give, send, tell, show, offer, bring, pay, ask, teach, lend, hand, owe, promise, grant, deny, allow, award, charge, cost, save, spare, forgive, refuse, assign, leave, provide, serve, feed, guarantee, bill, fine, email, inform, notify, call, name, title, entitle, head, elect, appoint, make, sell, buy, issue, afford, extend, gift, quote, take, read, write, mail*.

### verb-raising-control
`ai/en/seeds/lexicon/verb-raising-control.md`

**Gist.** Verbs with an infinitive "lend" it a subject in different ways. Raising verbs (*seem, appear, tend, happen, be likely*) have no agent of their own: *He seems to know* = "it seems that he knows". Subject-control verbs (*want, try, hope, decide, promise*): in *I want to go*, the one who wants is the one who goes. Object-control verbs (*ask, tell, persuade, convince, urge, order, allow, cause*): in *I asked him to stay*, *him* stays. Verbs of thinking and wanting with "object + infinitive" (*believe, consider, expect, want, find, like*): in *I believe him to be honest*, *him* is grammatically the main verb's object but semantically the infinitive's subject.

**Conditions and exceptions.** In EWT the person is `iobj` with speech and persuasion verbs that allow two objects (*ask, tell, allow, convince, urge, persuade, cause*) and `obj` with perception, causation and opinion verbs (*let, make, have, get, keep, find, want, help, see, consider*). Raising verbs have no passive and no direct object (see `verb-no-passive`, `verb-copula-like`). With *there* (*There seems to be a problem*) the expletive subject raises, and the notional subject stays with *be*. In UD all three types use `xcomp`; they differ in whether the main verb has an `obj`/`iobj`, and enhanced UD shows who the infinitive's subject is.

**Examples.** *Sue asked George to respond* — `xcomp(asked, respond)`. — *I consider him honest* — `obj(him)`, `xcomp(honest)`. — *The cat seems to be in pain* — `xcomp(seems, pain)`.

**Sources.** UD `docs/_en/dep/xcomp.md`, `docs/_u-dep/xcomp.md`, `docs/_en/specific-syntax.md` (functional control); PropBank 3.1 `seem.xml`, `want.xml`, `persuade.xml`; Poutsma GLME Part II ch. XIX §19; Jespersen MEG V ch. XVIII; Brown 1851, Rule XIX, Obs. 4.

**Rules.** None (descriptive seed).

### verb-stative-progressive
`ai/en/seeds/lexicon/verb-stative-progressive.md`

**Gist.** The progressive (*be + -ing*) describes an action that is ongoing and developing through will or activity. So verbs of state, knowledge, possession and attitude rarely take it: *I know*, not *\*I am knowing*; *It belongs to me*, not *\*is belonging*. Poutsma's groups: perception (*see, hear, smell, taste* in the literal sense, unlike *look, listen*), attitude (*like, love, hate, prefer, esteem*), desire (*desire, wish*), thought (*believe, know, suppose, understand*), possession and composition (*belong, contain, consist, possess, resemble*) and appearance (*seem, appear*).

**Conditions and exceptions.** With a nuance of activity the progressive is possible: *I'm seeing a doctor* (visiting), *You're being silly* (behaving), modern colloquial *I'm loving it*, *I've been meaning to call*. So the rule takes only the most stable stative verbs and only warns: either the text is unusual or the *-ing* is mis-annotated (for example, an adjective or gerund taken for a progressive). In UD, a VBG head with `aux` *be* should not have a lemma from the list.

**Examples.** *I know the answer.* — *\*I am knowing the answer.* — *She is being very kind* (copula with a sense of behaviour).

**Sources.** Poutsma 1921, *The Characters of the English Verb* (The Expanded Form), §§38–42; Jespersen MEG IV ch. XIV.

**Rules.** `en.lexicon.stative-progressive` (warn): a VERB tagged VBG with `aux` *be* is not one of *know, believe, understand, suppose, belong, contain, consist, own, possess, resemble, seem, prefer, matter, exist, deserve, recognize*.
