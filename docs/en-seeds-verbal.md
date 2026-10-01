# English grammar seeds: verbs and clauses (`ai/en/seeds/verbal`)

A **seed** is a short Markdown note holding one piece of knowledge about English grammar, together with its source (a page of a reference grammar or a paper). The name follows the image of a fruit stone: the seed is the kernel from which a rule grows. Each seed has the same sections:

- **Gist** — what the grammar says, in 1–3 sentences;
- **Conditions and exceptions** — when the statement holds and when it does not;
- **Examples** — short English sentences (an asterisk `*` marks an ungrammatical one);
- **In UD** — how the phenomenon looks in Universal Dependencies 2.18 annotation (English Web Treebank conventions): UPOS, XPOS (Penn Treebank tags), FEATS, relations;
- **Sources** — the grammars, UD guidelines and papers the seed is based on;
- optionally, one or more machine-readable ```` ```rule ```` blocks.

The seed tree is split into sections: `morph` (inflection and word formation), `nominal` (the noun phrase), `verbal` (the verb and the clause — this folder), `lexicon` (valency) and `errors` (typical errors of parsers, language models and learners).

## How a seed becomes a rule

A ```` ```rule ```` block is written in a small declarative language and executed by the deterministic Rust rule engine `en::expert`:

```rule
rule: en.verbal.cop-be-only
what: the copula (cop) is only "be", with UPOS AUX
match: c[rel=cop]
require: c[lemma=be, upos=AUX]
severity: error
source: UD docs/_en/dep/cop.md
```

- `match` — one or more nodes, `name[condition, …]`; the engine tries every binding of the nodes to distinct words of a sentence. Conditions test `upos`, `xpos`, `lemma`, `form`, `feats.X=V`, `rel` (`rel~nsubj` matches the base relation with any subtype), `head`, word order (`before`, `after`, `next`, `prev`) and suffixes/prefixes; any condition can be negated (`!=`), and `feats.X=@s` compares a feature with another node.
- `require` — clauses that must all hold for every binding (`;` = and, ` or ` = alternatives). Clause forms: `node[…]`, `not node[…]`, `exists x[…]` (some other word exists), `none x[…]` (no such word).
- `unless` — exceptions: if any clause holds, the violation is dropped.
- `severity` — `error` (the annotation is certainly wrong) or `warn` (a suspicion worth checking).
- `source` — where the knowledge comes from (mandatory).

The engine loads the seed folders, parses every block into typed Rust structures (relations, tags and features are enums, not strings) and reports a parse error rather than silently ignoring a malformed rule; a test checks that every seed parses and that rule ids are unique. The CLI command `en expert-check <seeds> [--gold <gold>] <file.conllu>` checks a CoNLL-U file and prints up to three examples per rule (with `EN_EXPERT_LIST=1`, the full list of violations).

Typical uses: validating annotation produced by a parser, a language model or a silver pipeline; giving a correction pass hints such as "word 5 violates rule X"; and acting as hard constraints when parsing. The rules are precise but only see ungrammatical structure, so they are a check, not a complete parser.

Seeds without a rule block are still useful: they document a construction and the UD analysis expected for it. Relative paths in "see also" notes (`nominal/…`, `errors/…`, `lexicon/…`) point to sibling sections of the seed tree.

---

### absolute-construction
`ai/en/seeds/verbal/absolute-construction.md`

**Gist.** An absolute construction is a noun or pronoun with a participle that is grammatically unconnected to the rest of the sentence; it has its own "subject" and works as an adverbial (cause, time). Independent participial or infinitival phrases (*generally speaking*, *to confess the truth*) likewise have no link to the sentence subject.
**Conditions and exceptions.** The construction is often introduced by *with/without* (*With the kids in school, I have free time*) and may lack a verb altogether (*most of them women and children*). Brown required the nominative inside it (*he being absent*, not *him*). A dangling participle whose understood subject is not the sentence subject (*Walking home, the rain started*) is a stylistic flaw but receives the same analysis (`advcl`).
**In UD.** The head of the construction (participle, adjective or noun) is `advcl` of the main predicate; its "subject" is `nsubj`/`nsubj:pass` of that head; *with* is `mark`. A passive participle in the construction takes `nsubj:pass` without `aux:pass`.
**Examples.** *His master being absent, the business was neglected.* — *With some used as building material and others as urinals.* — *The attack killed 600 Iraqis, most of them women and children.*
**Sources.** Reed & Kellogg, *Higher Lessons in English* (1877), Lesson 44; Brown 1851, Rule VIII (nominative absolute) and Rule XX (participles); UD `docs/_en/dep/advcl.md`; UD `docs/_en/specific-syntax.md` (nonverbal predicates without a copula: absolutes).
**Rules.** None (descriptive seed).

### agreement-clausal-subject
`ai/en/seeds/verbal/agreement-clausal-subject.md`

**Gist.** When the subject is a whole clause or an infinitival or gerund phrase, the verb is third person singular: *To lie **is** base*, *That she lied **was** suspected*, *Taking a nap **relaxes** you*.
**Conditions and exceptions.** Several clausal subjects joined by *and* take the plural; joined by *or*, the singular.
**In UD.** A head with `csubj`/`csubj:pass` (with no coordinated conjuncts) is not `VBP` and has no `VBP` among its `cop`/`aux` dependents.
**Examples.** *Whether he lied **is** beside the point.* — *What he says **is** true.* — *\*To err are human.*
**Sources.** Brown 1851, Rule XIV Note III, Rule XVI Note VII, Rule XVII Note IV; UD `docs/_en/dep/csubj.md`.
**Rules.** `en.verbal.csubj-not-vbp`, `en.verbal.csubj-not-vbp-aux` (warn): a `VBP` predicate, copula or auxiliary with a clausal subject is suspicious unless the subject is coordinated.

### agreement-compound-subject
`ai/en/seeds/verbal/agreement-compound-subject.md`

**Gist.** Subjects joined by *and* take a plural verb (*John and Mary **are** here*); subjects joined by *or*/*nor* agree with the nearest one (*Neither you nor I **am** concerned*, *Either the boys or the teacher **is** coming*).
**Conditions and exceptions.** The singular is used with *and* when the two names denote one whole (*Bread and butter **is** my breakfast*) or are preceded by *each, every, no*. *As well as, with, together with, not* do not form a compound subject (*Veracity, as well as justice, **is** our rule*). Texts often keep the singular in *there is a force and beauty*, so the rule only warns.
**In UD.** A compound subject is the first conjunct as `nsubj`, the others `conj` to it with `cc`. A `VBZ` verb whose `nsubj` has a `conj` via *and* is suspicious (except in sentences with `expl`).
**Examples.** *He and I are friends.* — *Their sense of humour and calmness **amazes** me* (singular despite *and* — a feature of the text). — *The ship, with all her furniture, was destroyed.*
**Sources.** Brown 1851, Rule XVI and Notes II–IV, Rule XVII and Note I; Reed & Kellogg, *Higher Lessons*, Lessons 20 and 142; UD `docs/_en/specific-syntax.md` (Coordination).
**Rules.** `en.verbal.vbz-and-subject` (warn): `VBZ` with a subject coordinated by *and*, unless the verb has an `expl`.

### agreement-indefinite-pronoun
`ai/en/seeds/verbal/agreement-indefinite-pronoun.md`

**Gist.** *Each, either, neither, one, everyone, everybody, someone, anybody, nobody, everything, something, nothing* are grammatically singular, even when followed by *of* + plural: *Each of you **is** entitled to his share*, *Neither of them **was** there*.
**Conditions and exceptions.** In colloquial speech *neither/either of* + plural often takes the plural (*Are either of you familiar…?*), which Brown calls an error. *None* can be singular or plural; *all, some, most* agree by meaning.
**In UD.** A subject with one of these lemmas does not occur with `VBP` (either as the head or via `cop`/`aux`).
**Examples.** *Each of those ten million **has** a family.* — *Nothing **changes**.* — *Are either of you familiar with this project?* (colloquial, non-agreeing).
**Sources.** Brown 1851, Rule XIV Notes IV and VI; Reed & Kellogg, *Higher Lessons*, Lesson 142.
**Rules.** `en.verbal.indefinite-subject-not-vbp`, `en.verbal.indefinite-subject-not-vbp-aux` (warn): such a subject with a `VBP` head, copula or auxiliary, unless the subject is coordinated.

### agreement-non-third
`ai/en/seeds/verbal/agreement-non-third.md`

**Gist.** The present form without *-s* (tag `VBP`: *go, have, are, do*) is used with *I, you, we, they* and plural subjects; third-person singular pronouns (*he, she, it, this, that*) need the *-s* form. *Am* goes only with *I*.
**Conditions and exceptions.** Coordinated subjects (*He and I **are***) are plural. The subjunctive (*that he **go***) is tagged `VB`, not `VBP`, so `VBP` with *he* usually means a mis-tagged subjunctive or a genuine error in the text (*it sound like*). Relative pronouns (*who, that*) have no person.
**In UD.** A pronoun subject with `Person=3|Number=Sing` (not coordinated) does not occur with a `VBP` head, copula or auxiliary. With *am*, the pronoun subject has `Person=1|Number=Sing`.
**Examples.** *They **are** here.* — *I **am** here.* — *\*He **see** someone* (error in the text). — *I insist that he **go*** (VB, subjunctive).
**Sources.** Brown 1851, Rule XIV and Note IV; Brown 1851, Part II, Ch. VI (Persons and Numbers); Reed & Kellogg, *Higher Lessons*, Lesson 142; UD `docs/_en/feat/Mood.md`; Santorini 1990 (Penn Treebank tagging guidelines), VBP.
**Rules.** `en.verbal.vbp-3sg-pronoun`, `en.verbal.vbp-aux-3sg-pronoun` (warn): `VBP` with a he/she/it subject; `en.verbal.am-first-person` (warn): *am* only with a first-person singular pronoun subject.

### agreement-notional
`ai/en/seeds/verbal/agreement-notional.md`

**Gist.** Sometimes the verb agrees with the meaning of the subject rather than its form. A collective noun conceived as a plurality of persons takes the plural (*The committee **were** divided*, mostly British). Quantity nouns with *of* (*a number of, a lot of, a couple of, a handful of, a majority of*) agree with what follows *of*; conversely, plural-form names, sums and measures take the singular (*Ten dollars **is** enough*, *The United States **is***).
**Conditions and exceptions.** *The number of X* is singular. Because the same form can be used with either number depending on the speaker's view, these cases are exceptions to the agreement rules, not errors.
**In UD.** The head of the subject is a quantity noun (*number, lot, couple*) with `Number=Sing`; `Number` follows the word form, not the meaning. The `agreement-*` rules therefore give false warnings here; lemmas such as *number, lot, couple, majority, handful, bunch, variety, wealth, total, rest, half, percent* are candidates for exclusion via `lemma!=`.
**Examples.** *A large number of them **are** Bangladeshis.* — *There **are** a wealth of references* (colloquial). — *Three hours **isn't** far.*
**Sources.** Brown 1851, Rule XV and Rule XIV Note II; Jespersen, *A Modern English Grammar* (MEG) II, chapters on number; UD `docs/changes.md` (Morphosyntactic Features: notional agreement).
**Rules.** None (descriptive seed; see also `ai/en/seeds/nominal/collective-nouns.md`).

### agreement-third-singular
`ai/en/seeds/verbal/agreement-third-singular.md`

**Gist.** A finite verb agrees with its subject in person and number. The third-person singular form (*goes, has, is, does*; tag `VBZ`) and *was* are used only with a singular subject. The verb agrees with the head of the subject, not its dependents: *The progress of his forces **was** impeded*.
**Conditions and exceptions.** Plural-form names (*The United States **is***), measures and sums (*Three hours isn't far*) and names of sciences (*Physics is*) take the singular. In inversion (*From that flows all the tributaries*) and colloquial *there's* + plural, agreement is often broken in the text itself. The rule therefore only warns: it catches either a wrongly attached subject or a textual disagreement.
**In UD.** A `nsubj`/`nsubj:pass` with `Number=Plur` does not occur with a `VBZ` predicate or with `VBZ`/*was* as `cop`/`aux` — except in sentences with `expl` (*there's*) or an outer subject (`nsubj:outer`, where *is* belongs to the outer clause).
**Examples.** *The car **is** red.* — *The list of items **is** long* (the subject is *list*, not *items*). — *\*The dogs barks* (error in the text).
**Sources.** Brown 1851, Rule XIV and Note II; Reed & Kellogg, *Higher Lessons*, Lesson 142; UD `docs/changes.md` (Morphosyntactic Features); Santorini 1990, VBZ.
**Rules.** `en.verbal.vbz-plural-subject`, `en.verbal.vbz-aux-plural-subject`, `en.verbal.was-plural-subject` (warn): singular verb, copula, auxiliary or *was* with a plural noun/pronoun subject; exceptions: `expl`, an outer subject, or a numeral modifier on the subject (measures and sums).

### aux-closed-list
`ai/en/seeds/verbal/aux-closed-list.md`

**Gist.** An auxiliary carries no meaning of its own but adds tense, aspect, voice or modality to the main verb. English auxiliaries form a closed list: *be, have, do*, passive *get*, and the modals *can, could, may, might, shall, should, will, would, must, ought, need, dare*. Any other word in auxiliary position is an annotation error.
**Conditions and exceptions.** Infinitival *to* is not an auxiliary (it is `mark`). *Let* (*Let us go*) is a full verb with an object, not an "imperative auxiliary". *Become, seem, keep* are not auxiliaries; they take `xcomp`. *Have to, be going to, used to* are separate verbs with an infinitive (see `semi-modals`).
**In UD.** `aux` and `aux:pass` have UPOS `AUX` and a lemma from the UD 2.18 auxiliary registry; `cop` is always `AUX` too (only *be*, see `copula-be-only`).
**Examples.** *Reagan has died.* — *Reagan might have been lying.* — *The book got stolen.* — but *I tried **to** finish it* (*to* is `mark`).
**Sources.** UD `docs/_en/dep/aux_.md`, `docs/_en/pos/AUX_.md`; UD auxiliary registry; UD 2.18 validator, test `rel-upos-aux`; Brown 1851, Part II, Ch. VI (definition of auxiliary, Obs. 4) and Rule XIX Obs. 10.
**Rules.** `en.verbal.aux-lemma` (error): `aux`/`aux:pass` lemma from the closed list; `en.verbal.aux-upos` (error): `aux`, `aux:pass` and `cop` have UPOS `AUX`.

### aux-flat-leaf
`ai/en/seeds/verbal/aux-flat-leaf.md`

**Gist.** Several auxiliaries in one predicate (*might have been lying*) do not form a chain; each attaches directly to the main verb. An auxiliary is a leaf: it has no dependents of its own.
**Conditions and exceptions.** Allowed dependents of an auxiliary: `fixed`, `goeswith`, `reparandum`, `conj` and `cc` (coordinated auxiliaries: *We can and will get there*), `punct`. Negation is attached to the predicate in English EWT (see `negation-not`). When the main verb is elided, the auxiliary becomes the head and may have a subject — it is then no longer `aux` (see `vp-ellipsis`). This rule is a permissive list, as in the UD validator; `ai/en/seeds/errors/function-words-leaves.md` gives the complementary prohibited list.
**In UD.** `aux(lying, might)`, `aux(lying, have)`, `aux(lying, been)` — not `aux(have, might)`. A node with `aux`/`aux:pass`/`cop` has no children other than those listed.
**Examples.** *By that time, the story **would have been** revealed.* — all three (*would, have, been*) depend on *revealed*.
**Sources.** UD `docs/_en/specific-syntax.md` (Auxiliaries; Function words attaching to predicates); UD 2.18 validator, test `leaf-aux-cop`; UD `docs/_en/dep/aux_.md`.
**Rules.** `en.verbal.aux-leaf` (error): dependents of `aux`/`aux:pass`/`cop` (other than negation) must be `goeswith`, `fixed`, `reparandum`, `conj`, `cc` or `punct`.

### aux-head-nonfinite
`ai/en/seeds/verbal/aux-head-nonfinite.md`

**Gist.** The first auxiliary carries finiteness (tense, person, number); the main verb after an auxiliary is non-finite: bare infinitive (*can go*), *-ing* participle (*is going*) or *-ed/-en* participle (*has gone, was taken*). An *-s* form or past tense after an auxiliary is impossible (*\*does goes, \*will went*).
**Conditions and exceptions.** If the text itself is wrong (*does has*), the annotation should reflect the form and the rule then flags a textual error, which is also useful. When the auxiliary goes with a nominal predicate (*is a doctor*), it is `cop` and the rule does not apply.
**In UD.** A verbal head of `aux`/`aux:pass` has XPOS `VB`, `VBG` or `VBN`, never `VBZ`, `VBP`, `VBD`, `MD`.
**Examples.** *She **will go**.* — *They **have eaten**.* — *He **was killed**.* — *\*He does goes* (error in the text).
**Sources.** Brown 1851, Part II, Ch. VI; UD `docs/_en/feat/VerbForm.md`; Reed & Kellogg, *Higher Lessons*, Lessons 131–132.
**Rules.** `en.verbal.aux-head-nonfinite` (error): a `VERB`/`AUX` with an auxiliary is not `VBZ|VBP|VBD|MD`.

### aux-no-object
`ai/en/seeds/verbal/aux-no-object.md`

**Gist.** *Have, do, be* can be auxiliaries or full verbs (*have* "possess", *do* "perform", *be* "exist"). An auxiliary has no object of its own; if *have/do* has an object (*I have a car*, *do the dishes*), it is `VERB`.
**Conditions and exceptions.** In ellipsis the auxiliary becomes the head and may "inherit" a fronted object of the elided verb (*how many we can [reach]*, *the things that we do [feel]*), so `AUX` with `obj` is possible. *Have to* (*I have to go*) is `VERB` with `xcomp`.
**In UD.** `AUX` with a dependent `obj`/`iobj` is suspicious — most likely it should be `VERB`.
**Examples.** *I have eaten* (AUX). — *I have a dog* (VERB). — *Do your homework* (VERB). — *Mary didn't leave, John did* (AUX as head, no object).
**Sources.** UD `docs/_en/pos/AUX_.md` (ambiguity with VERB); Brown 1851, Part II, Ch. VI, Obs. 4; UD `docs/_en/specific-syntax.md` (VP ellipsis).
**Rules.** `en.verbal.aux-no-object` (warn): a word with UPOS `AUX` has no `obj`/`iobj`.

### clausal-complement
`ai/en/seeds/verbal/clausal-complement.md`

**Gist.** A subordinate clause can be the complement of a verb or adjective: *He says **that you like to swim***, *I am certain **that he did it***, *I wonder **whether it works***. Such a clause has its own subject (or is direct speech, or a *to*-infinitive after verbs of speaking: *The boss said **to start digging***). The complementizer *that* can be omitted.
**Conditions and exceptions.** `ccomp` itself fills the direct-object slot, so it never co-occurs with `obj`; an addressee next to it is `iobj` (*I told **them** that I'm coming*). A clause after a noun (*the fact that…*, *no idea what…*) is not `ccomp` but `acl` of the noun: nouns take no clausal complements.
**In UD.** `ccomp` does not co-occur with `obj` of the same head.
**Examples.** *He says that you like to swim.* — *I told them that I'm planning to come* (iobj + ccomp). — *He had no idea what he was talking about* (acl of *idea*, not ccomp of *had*).
**Sources.** UD `docs/_en/dep/ccomp.md`; UD `docs/_en/specific-syntax.md` (Core arguments; Clausal core arguments); Reed & Kellogg, *Higher Lessons*, Lessons 71–72; Jespersen, MEG V, ch. XII.
**Rules.** `en.verbal.ccomp-no-obj` (warn): a head with `ccomp` has no `obj`.

### copula-be-only
`ai/en/seeds/verbal/copula-be-only.md`

**Gist.** When the predicate is a noun, adjective or prepositional phrase (*Bill is honest*, *Sue is a patriot*, *We are in the barn*), *be* only links subject and predicate — it is a copula. In English UD only *be* is a copula; *become, seem, remain, look, get, grow, turn* are ordinary verbs and their predicative is `xcomp`.
**Conditions and exceptions.** *Be* is not a copula when it means "exist" (*There is a cow in the field*; `VERB`), in progressive forms (`aux`), in the passive (`aux:pass`), or in quotative *be like* (*I was like, "What?"*; `VERB` with `compound:prt`). Logic textbooks (and Reed & Kellogg) call any asserting word a copula; UD is narrower.
**In UD.** `cop`: lemma *be*, UPOS `AUX`. The head of the clause is the predicative, and the subject attaches to it.
**Examples.** *Bill is honest* — `cop(honest, is)`. — *Bill got rich* — `xcomp(got, rich)`. — *I became very upset* — `xcomp`.
**Sources.** UD `docs/_en/dep/cop.md`; UD `docs/_en/specific-syntax.md` (Copular verbs; Functional control); Reed & Kellogg, *Higher Lessons*, Lesson 29; Jespersen, MEG III, ch. XVII–XVIII (Predicatives).
**Rules.** `en.verbal.cop-be-only` (error): `cop` has lemma *be* and UPOS `AUX`.

### copula-equative-order
`ai/en/seeds/verbal/copula-equative-order.md`

**Gist.** In equative clauses (*Rolfe's wife was Pocahontas*, *The title is Green Eggs and Ham*) both nouns are on equal footing. Reed & Kellogg suggested choosing the subject by meaning (what the reader already knows); UD uses a formal criterion: the subject is the first noun, the predicative the second.
**Conditions and exceptions.** The subject follows the predicative in inversion (*Among the weapons intercepted were two launchers*) and in questions (*Who is he?*). Brown: the verb between two nouns agrees with the first, "except when the terms are transposed" (*The wages of sin is death*).
**In UD.** For a `NOUN`/`PROPN` head with `cop`, a nominal `nsubj` stands to the left of the head; otherwise it is an inversion (check) or the roles are swapped.
**Examples.** *Words are wind.* — *Lizards are reptiles.* — *Among the strongest proponents of war were Sharon and his supporters* (inversion: nsubj after).
**Sources.** UD `docs/_en/specific-syntax.md` (Copulas); Reed & Kellogg, *Higher Lessons*, Lesson 29; Brown 1851, Rule XIV Note V.
**Rules.** `en.verbal.equative-subject-first` (warn): a nominal subject of a nominal copular head precedes the head.

### copula-predicate-head
`ai/en/seeds/verbal/copula-predicate-head.md`

**Gist.** UD makes the predicative (noun, adjective, prepositional phrase, pronoun), not *be*, the head of a copular clause, so the subject, negation and adverbials attach to it: *Bill is **honest*** — `nsubj(honest, Bill)`. A nominal or pronominal predicate cannot have a direct object — objects belong only to verbs (and *worth, like, unlike*).
**Conditions and exceptions.** A verb under a copula occurs only in a predicate clause (*The important thing is **to keep** calm*), which then has an outer subject `nsubj:outer`. In questions the wh-word is the head: *What is that?* — `cop(What, is)`.
**In UD.** A `cop` head with UPOS `NOUN`, `PROPN`, `PRON` or `NUM` has no `obj`/`iobj`; a `cop` head with UPOS `VERB` has `nsubj:outer` or `csubj:outer`.
**Examples.** *Sue is a true patriot.* — *The light is on* (head: the preposition *on*). — *The plan is to leave* (`nsubj:outer`).
**Sources.** UD `docs/_en/dep/cop.md`; UD `docs/_en/specific-syntax.md` (Copulas; Core arguments); Brown 1851, Rule VI; Reed & Kellogg, *Higher Lessons*, Lesson 29.
**Rules.** `en.verbal.cop-nominal-no-object` (error): a nominal copular predicate has no `obj`/`iobj`; `en.verbal.cop-verb-head-outer` (warn): a verb under a copula has an outer subject.

### core-objects
`ai/en/seeds/verbal/core-objects.md`

**Gist.** A direct object is a noun completing a transitive verb without a preposition (*Washington captured **Cornwallis***). Only verbs have objects (plus the adjectives *worth, like, unlike*: *It's worth **the money***). A predicate has at most one direct object; a second "object" is an indirect object (*gave **me** a raise*), an object predicative (*made him **king***, see `objective-complement`), or a bare-noun adverbial (*walked **three miles***, *arrived **this morning***).
**Conditions and exceptions.** Coordinated objects (*captured Cornwallis and his army*) are one `obj` with `conj`. Full-verb *have/do* with an object is `VERB`. In ellipsis the object may hang on an auxiliary that has become the head.
**In UD.** `obj`/`iobj` have a `VERB` head (`AUX` only in ellipsis) or an `ADJ` *worth/like/unlike*; a head has at most one `obj`.
**Examples.** *She gave me a raise* — `obj(gave, raise)`, `iobj(gave, me)`. — *They elected him president* — `obj(him)`, `xcomp(president)`. — *\*a man honesty* (a noun with obj is an error).
**Sources.** Reed & Kellogg, *Higher Lessons*, Lesson 28; Brown 1851, Rule V; UD `docs/_en/dep/obj.md`; UD `docs/_en/specific-syntax.md` (Core arguments); UD 2.18 validator, `too-many-objects`; Poutsma 1923, *The Infinitive…*, §81.
**Rules.** `en.verbal.single-object` (error): at most one `obj` per head; `en.verbal.object-head-predicate` (error): the head of `obj`/`iobj` is `VERB`, `AUX` or `ADJ`; `en.verbal.object-head-adjective` (warn): among adjectives only *worth, like, unlike* take objects.

### do-support
`ai/en/seeds/verbal/do-support.md`

**Gist.** In negation, questions, emphasis and ellipsis, when no other auxiliary is present, English inserts *do*: *I do not love*, *Did you see?*, *I **do** believe it*. After *do* the main verb is a bare infinitive. *Do* never combines with modals or perfect *have* (*\*does can*, *\*did have eaten*).
**Conditions and exceptions.** In the imperative *do* combines with *be* and the passive (*Don't be silly*, *Do be informed*, *did not get assigned*), so the head may be an adjective or a `VBN` participle (with `aux:pass`). Full-verb *do* (*do the work*) is `VERB` (see `aux-no-object`). Archaic "redundant" *do* (*all the beasts … do creep forth*) is also do-support.
**In UD.** *do* is `aux`, `AUX`; a verbal head has XPOS `VB` (or `VBN` with `aux:pass`), never `VBZ`, `VBP`, `VBD`, `VBG`; no other `aux` of the same head is a modal or *have*.
**Examples.** *He does not know.* — *Don't be evil.* — *\*He did not went.* — *\*She doesn't can swim.*
**Sources.** Brown 1851, Part II, Ch. VI (Form of Negation; Form of Question; Obs. 11 on expletive *do*); Poutsma 1923, *The Infinitive…*, §4c; Jespersen, MEG V, ch. XXIII (Negation); UD auxiliary registry.
**Rules.** `en.verbal.do-support-base` (error): the main verb after *do* is not finite and not `VBG`; `en.verbal.do-support-vbn-pass` (error): `VBN` with *do* only together with `aux:pass`; `en.verbal.do-support-no-modal` (error): *do* does not co-occur with modals or *have*.

### existential-there
`ai/en/seeds/verbal/existential-there.md`

**Gist.** *There is/are* states that something exists or is somewhere (*There is a ghost in the room*). *There* is not a place adverb but a formal placeholder in subject position; the notional subject follows the verb and the verb agrees with it (*There **are** magicians*). *Be* here is not a copula but a full verb "exist".
**Conditions and exceptions.** *There* also occurs with *seem, appear, exist, remain, come* (*There seems to be a problem*); the notional subject then attaches to *be* under `xcomp`. Colloquial *there's* + plural is textual disagreement. *There* meaning "in that place" (*Put it there*) is an adverb.
**In UD.** *there*: `PRON`, XPOS `EX`, `expl` (the tag itself is checked by `en.nominal.ex-there`). *Be* with expletive *there* has UPOS `VERB` (not `AUX`, not `cop`). The notional subject is `nsubj` of the same head, to the right of *there*.
**Examples.** *There's a cow in the field.* — *Is there a ghost?* — *There must be a reason* (expl with *be* plus a modal).
**Sources.** UD `docs/_en/dep/expl.md`, `docs/_en/dep/nsubj.md`, `docs/_en/dep/cop.md`, `docs/_en/pos/AUX_.md`; Reed & Kellogg, *Higher Lessons*, Lesson 44; Brown 1851, Part II, Ch. VI.
**Rules.** `en.verbal.existential-be-verb` (error): *be* with expletive *there* is `VERB`; `en.verbal.existential-subject` (warn): there is an `nsubj` after *there*.

### expletive-it
`ai/en/seeds/verbal/expletive-it.md`

**Gist.** *It* can be an empty placeholder: (1) extraposition, where the real clausal subject comes at the end (*It is important **that your students respect you***, *It's hard **to make money***); (2) clefts (*It was **Joseph Goebbels** who said that*); (3) weather and time (*It is raining*, *It's late*); (4) in object position (*I find it best not to think about that*).
**Conditions and exceptions.** In tough-constructions without *it* (*This problem is hard to solve*) the infinitive is `xcomp` of the adjective; with *it* it is `csubj`. Referential *it* (*I bought a car; it is red*) is `nsubj`. Rarely, *that* serves as the cleft placeholder (*that was 2 days ago that I called*).
**In UD.** Expletive *it* is `expl`, `PRON`. The same head usually has a clause (`csubj`, `csubj:pass`, `ccomp`, `xcomp`, `advcl`, `advcl:relcl`); without one it is weather *it* or an error. With an adjective and expletive *it*, the infinitive is `csubj`, not `xcomp`.
**Examples.** *It is rare to find such nice workers* (expl + csubj). — *It's John who we want to help* (expl + advcl:relcl). — *It is raining* (expl without a clause).
**Sources.** UD `docs/_en/dep/expl.md`; UD `docs/_en/specific-syntax.md` (Core arguments; Tough-constructions); UD `docs/_en/dep/acl-relcl.md` (It-clefts); UD `docs/_en/dep/csubj.md`; UD 2.18 validator, `rel-upos-expl`.
**Rules.** `en.verbal.expl-it-has-clause` (warn): expletive *it* has a clause at its head; `en.verbal.tough-expl-csubj` (warn): with an adjective and expletive *it*, no `xcomp`; `en.verbal.expl-is-pron` (error): `expl` is `PRON`; `en.verbal.expl-it-or-there` (warn): `expl` lemma is *it* or *there*.

### finite-verb-subject
`ai/en/seeds/verbal/finite-verb-subject.md`

**Gist.** Unlike pro-drop languages, an English finite verb almost always has an overt subject (*I came, I saw, I conquered*); even impersonal predicates take formal *it*/*there*. Subjectless finite verbs occur in the imperative (*Go!*), as the second of coordinated predicates (*He came and **saw***), and in colloquial initial omission (*Hope you're well*, *Looks like rain*).
**Conditions and exceptions.** A shared subject attaches only to the first coordinated predicate; the second `conj` has no subject (coordination structure: `ai/en/seeds/errors/coordination-structure.md`). In relative clauses the relative pronoun is the subject; in free relatives (*What irritates me…*) the relative word is the head and no subject is visible.
**In UD.** A `VERB` with `Mood=Ind` as `root`, `ccomp`, `advcl` or `parataxis` usually has `nsubj`/`nsubj:pass`/`csubj`/`csubj:pass` or `expl`. Absence means colloquial omission or a misattached subject.
**Examples.** *It reminds me of Vietnam.* — *Seems as if there is a clear distinction* (colloquial omission). — *He was tired and went home* (second predicate without nsubj is normal).
**Sources.** Brown 1851, Rule XIV Note VIII; Reed & Kellogg, *Higher Lessons*, Lessons 20 and 57; Jespersen 1924, *The Philosophy of Grammar*, pp. 142, 310 (prosiopesis); Curme 1931, §5 b, d; UD `docs/_en/dep/expl.md`, `docs/_en/specific-syntax.md` (Coordination).
**Rules.** `en.verbal.finite-has-subject` (warn): an indicative `VERB` as `root`/`ccomp`/`advcl` has a subject or `expl`, unless only punctuation, conjunctions, adverbs or interjections precede it (initial omission), the lemma is *thank*/*hope*, or the clause is marked by *as*/*than* (*as follows*); `en.verbal.parataxis-has-subject` (warn): the same for `parataxis`, also excused when the governing clause has a subject before it (*Staff is friendly, treat you as a friend*).

### gapping-orphan
`ai/en/seeds/verbal/gapping-orphan.md`

**Gist.** In coordinated clauses the predicate of the second is often omitted, leaving two remnants contrasted with the first clause: *Marie went to Paris and Miriam [went] to Prague*. The tree promotes one remnant (usually the subject) to the predicate position and attaches the others to it with the special relation `orphan`.
**Conditions and exceptions.** `orphan` is used only when the predicate is entirely missing; if an auxiliary remains, it is VP ellipsis (the auxiliary is the head, no `orphan`). Right-node raising (*John bought and ate an apple*) is ordinary coordination. The promoted remnant is usually `conj` (sometimes `parataxis`, or `advcl` in comparatives: *He buys companies like my mother [does] vegetables*).
**In UD.** The head of `orphan` has relation `conj`, `parataxis`, `root`, `csubj`, `ccomp`, `advcl`, `acl` or `reparandum` (with subtypes).
**Examples.** *Marie went to Paris and Miriam to Prague* — `conj(went, Miriam)`, `orphan(Miriam, Prague)`. — *He's not against gays in the bedroom, just at the altar.*
**Sources.** UD `docs/_en/dep/orphan.md`; UD `docs/_en/specific-syntax.md` (Gapping/Stripping; Right-node raising); UD 2.18 validator, `orphan-parent`; Reed & Kellogg, *Higher Lessons*, Lesson 57.
**Rules.** `en.verbal.orphan-parent` (warn): the head of `orphan` has one of the listed relations.

### gerund-participle
`ai/en/seeds/verbal/gerund-participle.md`

**Gist.** The *-ing* form has two roles. The gerund is a verb used as a noun: subject, object, after a preposition (*Swimming is fun*, *I enjoyed working with you*, *for keeping money*). The participle is a verb used as a modifier or adverbial, and in progressive forms (*a sleeping child*, *He left, saying nothing*, *is sleeping*). Modern grammars (CGEL) see one "gerund-participial" form; UD approximates the traditional split.
**Conditions and exceptions.** An *-ing* word that has become a noun (*the opening of the store*, *a building*) is `NOUN` without verbal features. The border between gerund and verbal noun is fuzzy (Poutsma §§53–55); the criterion is whether the word keeps verbal government (a direct object, an adverb).
**In UD.** `VBG`: `VerbForm=Ger` (no `Tense`) in nominal positions and without `aux`; `VerbForm=Part|Tense=Pres` with `aux` and in modifier/adverbial roles. A gerund never has `aux`.
**Examples.** *I enjoyed working with you* (Ger). — *I will be driving home* (Part). — *The opening was delayed* (NOUN).
**Sources.** UD `docs/_en/feat/VerbForm.md` (Ger, Part; VBG rules since v2.14); Poutsma 1923, *The Gerund*, §1 and §§49–55; Reed & Kellogg, *Higher Lessons*, Lesson 37; Jespersen, MEG V, ch. VIII–IX (The Gerund).
**Rules.** `en.verbal.gerund-no-aux` (error): `VerbForm=Ger` has no `aux`/`aux:pass` (missing `Tense` is checked by `en.morph.vbg-gerund-no-tense`).

### get-passive
`ai/en/seeds/verbal/get-passive.md`

**Gist.** Colloquial *get* with a past participle forms a passive (*The book got stolen*, *I got put on hold*) and is then an auxiliary. Elsewhere *get* is an ordinary verb: "become" with an adjective (*Bill got rich*), causative (*I got it fixed*), "receive" (*I got a letter*), *have got*.
**Conditions and exceptions.** Causative *get* + object + participle (*got it fixed*) is not a passive of *get*: *it* is the object, *fixed* is `xcomp`. *Get* has no auxiliary function other than the passive.
**In UD.** *get* as `AUX` is only `aux:pass` (or a head in ellipsis, `conj`).
**Examples.** *He got shot.* (AUX, aux:pass) — *Bill got rich.* (VERB + xcomp) — *I got it fixed.* (VERB + obj + xcomp).
**Sources.** UD `docs/_en/dep/aux_.md`; UD `docs/_en/pos/AUX_.md`; UD `docs/_en/specific-syntax.md` (Auxiliaries).
**Rules.** `en.verbal.get-aux-passive-only` (error): *get* with UPOS `AUX` is `aux:pass`, `conj` or `root`.

### imperative
`ai/en/seeds/verbal/imperative.md`

**Gist.** Commands and requests use the form identical to the bare infinitive (*Make a sandwich!*, *Be careful*). The subject is usually omitted; if present, it precedes the verb (*You go first*, *Somebody help me*). Negative and emphatic imperatives use *do* (*Don't go*, *Do come*); an imperative verb has no other auxiliaries (modals, *have*, passive *be*).
**Conditions and exceptions.** *Let's go* is an imperative of *let* (VERB) with object *us* and `xcomp`. In *Don't be silly* the imperative is *Do* (AUX) and *be* is a copula. For the main verb after imperative *do*, the UD `VerbForm` page prescribes `Inf`, while EWT 2.18 marks it `Mood=Imp|VerbForm=Fin` in about 98 of 106 cases; the rule below follows EWT and only warns.
**In UD.** Imperative: XPOS `VB`, `Mood=Imp`, `VerbForm=Fin`, no `Tense`. Its only possible auxiliary is *do*.
**Examples.** *Read the book!* — *Don't take that deal.* — *\*Must try!* (error: modal with an imperative).
**Sources.** UD `docs/_en/feat/Mood.md` (Imp); UD `docs/_en/feat/VerbForm.md`; Reed & Kellogg, *Higher Lessons*, Lessons 56 and 131; Brown 1851, Part II, Ch. VI (Form of Negation); Jespersen, MEG V, ch. XXIV (Requests).
**Rules.** `en.verbal.imperative-form` (error): imperative is `VB`, `VerbForm=Fin`, no `Tense`; `en.verbal.imperative-aux-do-only` (error): an imperative verb has no auxiliary other than *do*; `en.verbal.imperative-do-main-verb` (warn): the main verb with imperative *do* is also `Mood=Imp|VerbForm=Fin` (EWT practice).

### indirect-object
`ai/en/seeds/verbal/indirect-object.md`

**Gist.** The indirect object is the recipient or addressee without a preposition (*She gave **me** a raise*, *Tell **them** a story*); it precedes the direct object. The same role with a preposition (*gave it **to me***) is not an object but `obl`.
**Conditions and exceptions.** Since UD 2.12, `iobj` may be the only object if the verb allows a second one: *tell **them*** (cf. *tell them a story*), *tell them that the party is canceled* (with `ccomp`), *teach **her students** to write well* (with `xcomp`). In questions and relative clauses the direct object is fronted (*What did you give him?*). Verbs licensing `iobj` are listed in `ai/en/seeds/lexicon/verb-iobj-licensors.md`.
**In UD.** `iobj` stands to the left of a nominal `obj` of the same head.
**Examples.** *They gave me the trip as a gift.* — *I told them that I'm coming* (iobj + ccomp). — *How much notification would you give the customers?* (obj fronted).
**Sources.** UD `docs/changes.md` (Sole iobj, v2.12); UD `docs/_u-dep/iobj.md`; UD `docs/_en/dep/iobj.md`; UD `docs/_en/dep/obl.md`; Jespersen, MEG III, ch. XIV §14.11; PropBank 3.1, `give.01`.
**Rules.** `en.verbal.iobj-before-obj` (warn): `iobj` precedes a nominal `obj`.

### infinitive-coordination
`ai/en/seeds/verbal/infinitive-coordination.md`

**Gist.** When two infinitives with the same role are coordinated, *to* is often placed only before the first (*to read and write*). The second infinitive does not become finite; it shares the *to*.
**Conditions and exceptions.** With three or more infinitives *to* is mostly repeated before each; repetition stresses the separateness of the actions. After *but* and *than* different rules apply (Poutsma §§44–48).
**In UD.** The second infinitive is `conj` of the first, has no `mark(to)` of its own, and stays `VB` with `VerbForm=Inf`.
**Examples.** *My advice is to go to the website and click "contact us".* — *She wanted to read and (to) write.*
**Sources.** Poutsma 1923, *The Infinitive…*, §§53–55; Brown 1851, Rule XVII Note V; UD `docs/_en/feat/VerbForm.md` (Inf).
**Rules.** `en.verbal.conj-infinitive-shares-to` (warn): a `VB` conjunct of a *to*-infinitive is also `VerbForm=Inf`.

### modal-bare-infinitive
`ai/en/seeds/verbal/modal-bare-infinitive.md`

**Gist.** A modal (*can, may, must, will, shall*, etc.) requires a bare infinitive (*I can swim*, *you must go*). If more auxiliaries follow, the one after the modal is also an infinitive (*might **have** been*, *could **be** done*). A finite form after a modal is impossible, and two modals in a row are not used in the standard language.
**Conditions and exceptions.** *Ought* takes *to* (*ought to go*) but remains a modal `AUX`. Double modals (*might could*) occur in Southern US dialects, so the second rule only warns; more often, two modals on one predicate mean that one is attached to the wrong verb (from a different clause).
**In UD.** The modal is `aux` with XPOS `MD`; other `aux`/`aux:pass`/`cop` of the same head to its right are not `VBZ`, `VBP`, `VBD`.
**Examples.** *He should leave.* — *Reagan might have been lying.* — *\*He can goes.* — *\*The hope may … can be offered* (the second modal belongs to another clause).
**Sources.** Poutsma 1923, *The Infinitive…*, §4 and §§60–62; Brown 1851, Part II, Ch. VI (potential mood); UD `docs/_en/dep/aux_.md`.
**Rules.** `en.verbal.modal-next-nonfinite` (error): auxiliaries/copula after a modal are non-finite; `en.verbal.double-modal` (warn): two `MD` auxiliaries on one head.

### need-dare
`ai/en/seeds/verbal/need-dare.md`

**Gist.** *Need* and *dare* behave in two ways. As modals they have no *-s*, take no *do* and go with a bare infinitive, mostly in questions and negation (*Need I say more?*, *He need not go*, *How dare you?*). As full verbs they inflect and take *do* and *to* (*He needs to go*, *You don't need to shout*).
**Conditions and exceptions.** *Dare* after *do* is often without *to* (*don't dare go*) — that is full-verb *dare* with a bare `xcomp`. *I dare say* is a fixed phrase. *Need* with a bare infinitive is treated as an auxiliary (as in Brown and in UD).
**In UD.** Modal: `AUX`, `aux`, the head has no `mark(to)`. Full verb: `VERB`, the infinitive is `xcomp` with `mark(to)`.
**Examples.** *You needn't shout* (AUX). — *You need to shout* (VERB + xcomp). — *He dare not come* (AUX).
**Sources.** UD `docs/_en/pos/AUX_.md`; Brown 1851, Rule XIX Obs. 7 and Obs. 12–13; Poutsma 1923, *The Infinitive…*, §§6–15 (need) and §§16–31 (dare).
**Rules.** `en.verbal.need-aux-bare` (error): auxiliary *need/dare* — no *to* on the infinitive; `en.verbal.need-verb-to` (warn): full-verb *need* with a `VB` `xcomp` has *to*.

### negation-not
`ai/en/seeds/verbal/negation-not.md`

**Gist.** Clauses are negated with *not* (*n't*) placed after the first auxiliary (*I have **not** seen*, *She will**n't** go*); without an auxiliary, *do* is added (*I do not know*). Infinitives and participles take *not* before them (*not to go*, *not knowing*). *Not* scopes over the whole predicate, so in UD it attaches to the main verb (or predicative), not to the auxiliary.
**Conditions and exceptions.** *Not* as the remnant of an elliptical clause (*If not, …*, *Why not?*, *I hope not*) becomes a head or an adverbial. *Not only … but also* is `advmod`/`cc:preconj`; in *whether or not*, *not* attaches to *or*. The split infinitive *to not attempt* is normal. Other negative words (*never, no, nobody*) are separate parts of speech with `Polarity=Neg`.
**In UD.** *not/n't*: UPOS `PART`, XPOS `RB`, `Polarity=Neg`, `advmod` of the predicate. The validator allows negation on a function word, but English treebanks attach it to the predicate.
**Examples.** *Kennedy has not been killed* — `advmod(killed, not)`. — *Don't go* — `advmod(go, n't)`. — *If not, is there someone else?*
**Sources.** Brown 1851, Part II, Ch. VI (Form of Negation); Jespersen, MEG V, ch. XXIII (Negation); UD `docs/_en/pos/PART.md`; UD `docs/_en/feat/Polarity.md`; UD 2.18 validator (functional-leaves exception for negation).
**Rules.** `en.verbal.not-part-neg` (warn): *not* as `PART` has `Polarity=Neg` and one of the expected relations (`advmod`, or a head role in ellipsis, etc.); `en.verbal.negation-on-predicate` (warn): a negative `PART` attached to an auxiliary or copula should not be `advmod` there.

### objective-complement
`ai/en/seeds/verbal/objective-complement.md`

**Gist.** After *make, call, name, elect, appoint, consider, think, find, keep, leave, paint* the object also receives a predicate: *They made Victoria **queen***, *I consider him **a fool***, *Custom renders the feelings **blunt***. This is not a second object: *queen* says what Victoria became (Reed & Kellogg's objective complement, or factitive object).
**Conditions and exceptions.** With *as* (*We consider time **as** a sacred trust*, *regard X as Y*) EWT annotates the phrase as `obl` with `case(as)` (with the verb *being*, as `advcl` with `mark(as)`). In the passive the predicative remains (*He was elected **president*** — `xcomp`). Resultatives (*The pond froze **solid***) are also `xcomp`.
**In UD.** A predicative noun or adjective on the object is `xcomp` of the verb (not `obj`: there is no second `obj`, see `core-objects`).
**Examples.** *They made Victoria queen* — `obj(made, Victoria)`, `xcomp(made, queen)`. — *He painted the barn red.* — *Many Iraqis regard the date as a day of hope* (`obl` + `case as`).
**Sources.** Reed & Kellogg, *Higher Lessons*, Lesson 31 (Objective Complements); UD `docs/_en/dep/xcomp.md`; UD `docs/_en/specific-syntax.md` (Resultatives); Jespersen, MEG III, ch. XVII–XVIII (Predicatives).
**Rules.** None (descriptive seed).

### open-complement-xcomp
`ai/en/seeds/verbal/open-complement-xcomp.md`

**Gist.** Some verbs take a complement whose "subject" is borrowed from the main clause: *Sue asked George **to respond*** (George responds), *You like **to swim***, *I consider him **honest***, *She looks **beautiful***, *He painted the barn **red***. This is `xcomp`: always non-finite and without its own subject.
**Conditions and exceptions.** It covers raising verbs (*seem, appear, tend*), control verbs (*want, try, persuade*), copula-like verbs (*become, remain, look*) and resultatives (*paint red*, *make them martyrs*). When the subordinate clause has its own finite subject, it is `ccomp`. An `xcomp` with its own `nsubj` is an error, except for raising with *there* (*there seems to be a problem*: *problem* attaches to *be*).
**In UD.** `xcomp` has no `nsubj`/`nsubj:pass`/`csubj`/`csubj:pass`; a verbal `xcomp` is not `VBZ`, `VBP`, `VBD`, `MD`.
**Examples.** *He says that you like to swim* — `xcomp(like, swim)`. — *The cat seems to be in pain.* — *I became very upset.*
**Sources.** UD `docs/_en/dep/xcomp.md`; UD `docs/_en/specific-syntax.md` (Functional control; Resultatives); UD `docs/_u-dep/xcomp.md`; Jespersen, MEG V, ch. XVIII; PropBank 3.1, `seem.01`, `persuade.01`.
**Rules.** `en.verbal.xcomp-no-subject` (warn): `xcomp` has no subject of its own; `en.verbal.xcomp-nonfinite` (error): a verbal `xcomp` is non-finite.

### outer-subject
`ai/en/seeds/verbal/outer-subject.md`

**Gist.** The predicate of a copular clause can be a whole subordinate clause (*The problem is **that this has never been tried***, *The important thing is **to keep calm***). The predicate of the inner clause (*tried*, *keep*) then becomes the head, the copula attaches to it, and the subject of the outer clause is marked as "outer".
**Conditions and exceptions.** The `:outer` subtype is only for such predicate clauses. If the predicate is a noun or adjective (*The title is Green Eggs and Ham*, *That book is very good*), the subject is plain `nsubj`. In pseudoclefts (*What John did was to play tennis*) the subject *What* is also `nsubj:outer`.
**In UD.** Only heads that have a `cop` take `nsubj:outer`/`csubj:outer`.
**Examples.** *The problem is that this has never been tried.* — *To hike in the mountains is to experience the best of nature* (csubj:outer). — *It was because Bill is honest.*
**Sources.** UD `docs/_en/dep/nsubj-outer.md`, `docs/_en/dep/csubj-outer.md`, `docs/_en/dep/cop.md`; UD `docs/_en/dep/acl-relcl.md` (Pseudoclefts); UD 2.18 validator (outer subjects are not counted as inner).
**Rules.** `en.verbal.outer-subject-needs-cop` (warn): a head with an outer subject has a `cop`.

### participle-or-adjective
`ai/en/seeds/verbal/participle-or-adjective.md`

**Gist.** *-ed/-en* and *-ing* forms sit between verb and adjective. Verbal nature shows in tense, an object, an agent or adverbials (*the letters written yesterday*, *was worn by Joseph*); adjectival nature shows in comparison or intensifiers (*very tired, more interesting*), a negative *un-* without a matching verb (*unexpected*), or use after *seem, become*. Denominal *-ed* adjectives (*talented, skilled, gifted, kind-hearted*) are not participles at all (see `ai/en/seeds/morph/wf-ed-adjectives.md`).
**Conditions and exceptions.** A nearby *very* is not proof (*very much appreciated* — *very* modifies *much*). *Un-* also occurs on real verbs (*undo, unlock → unlocked*). *The coat was badly worn* is adjectival; *The coat was worn by Joseph* is passive.
**In UD.** Adjectival use: `ADJ` (`JJ`), with *be* as `cop`; verbal use: `VERB` (`VBN`/`VBG`), with *be* as `aux:pass`/`aux`. The intensifier *very* attaches to an `ADJ`, not a `VERB`.
**Examples.** *I am very interested* (ADJ). — *He was interested by the offer* (VERB, passive). — *a talented singer* (ADJ from the noun *talent*).
**Sources.** Poutsma 1923, *Participles*, §§7–13, 17, 21–22, 33, 35, 42; Reed & Kellogg, *Higher Lessons*, Lesson 129; UD `docs/_en/feat/Voice.md`.
**Rules.** `en.verbal.very-participle` (warn): *very* as `advmod` of a `VBN`/`VBG` verb suggests the word should be `ADJ`.

### passive-agent
`ai/en/seeds/verbal/passive-agent.md`

**Gist.** The doer in a passive clause is expressed by a *by*-phrase (*The cat was chased **by the dog***). Reed & Kellogg's test: if *by* + agent can be added to *be* + participle without changing the meaning, it is a passive; if not, it is an adjective after a copula (*The coat was badly worn*).
**Conditions and exceptions.** Not every *by* is an agent: *by the window* (place), *by Friday* (time) are plain `obl`. An agent can also occur with a passive participle without an auxiliary (*paintings eaten by moths*).
**In UD.** The agent is `obl:agent` with *by* as its `case`; the head is a verb with `Voice=Pass`.
**Examples.** *We were delighted by the snow.* — *It has been eaten by moths.* — *He sat by the fire* (not an agent; obl).
**Sources.** UD `docs/_en/dep/obl-agent.md`, `docs/_en/feat/Voice.md`; Reed & Kellogg, *Higher Lessons*, Lesson 129; Poutsma 1923, *The Infinitive…*, §85.
**Rules.** `en.verbal.agent-passive-by` (error): `obl:agent` only on a `Voice=Pass` head and with a `case` *by*.

### passive-aux
`ai/en/seeds/verbal/passive-aux.md`

**Gist.** The passive shows that the subject undergoes the action. It is built from *be* (less often colloquial *get*) and a past participle (*was killed*, *is being built*, *got stolen*), and only from verbs that take an object (*Caesar was slain*, but not *\*He was arrived*).
**Conditions and exceptions.** Verbs of similar meaning (*become, seem*) are never passive auxiliaries (*He became known* — `xcomp`). Passive infinitive *to be given*, passive perfect *has been given*, passive gerund *being punished*. A prepositional passive (*That matter was talked about*) is also a passive; the preposition is left without its noun.
**In UD.** The passive auxiliary is `aux:pass`, lemma *be* or *get*, UPOS `AUX`; its head is `VERB`, XPOS `VBN`, `Voice=Pass`.
**Examples.** *Kennedy was killed.* — *Kennedy got killed.* — *He was to be released before dawn* (*be* is aux:pass, *was* is aux).
**Sources.** Brown 1851, Part II, Ch. VI (Form of Passive Verbs); Reed & Kellogg, *Higher Lessons*, Lesson 129; Poutsma 1923, *The Infinitive…*, §58; UD `docs/_en/dep/aux-pass.md`, `docs/_en/feat/Voice.md`.
**Rules.** `en.verbal.passive-aux-lemma` (error): `aux:pass` is *be* or *get*; `en.verbal.passive-aux-head` (error): the head of `aux:pass` is `VERB`, `VBN`, `Voice=Pass`.

### passive-subject
`ai/en/seeds/verbal/passive-subject.md`

**Gist.** In the passive, the active object becomes the subject (*he loves her* → *she is loved (by him)*). Since this subject is not the doer, UD labels it separately — `nsubj:pass` (or `csubj:pass` for a clausal subject); conversely, a passive verb cannot have a plain `nsubj`.
**Conditions and exceptions.** The indirect object can also become the passive subject (*I was given a horse*; *horse* stays as object). Not every verb with an object has a passive (*cost, weigh, last, resemble*; see `ai/en/seeds/lexicon/verb-no-passive.md`). A passive subject can occur without an auxiliary — in absolute constructions (*with some used as building material*) and headlines (*Man arrested*) — so the third rule only warns.
**In UD.** Only `Voice=Pass` heads take `nsubj:pass`/`csubj:pass`; a `Voice=Pass` head has no `nsubj`/`csubj`; usually an `aux:pass` is present.
**Examples.** *Dole was defeated by Clinton.* — *That she lied was suspected by everyone* (csubj:pass). — *Armed militias, many staffed by former soldiers, …* (no aux:pass).
**Sources.** Jespersen, MEG III, ch. XV §15.11 (Subject of Passive Verb); Reed & Kellogg, *Higher Lessons*, Lesson 129; UD `docs/_en/dep/nsubj-pass.md`, `docs/_en/dep/csubj-pass.md`, `docs/_en/feat/Voice.md`.
**Rules.** `en.verbal.passive-subject-voice` (error): passive subjects only with `Voice=Pass`; `en.verbal.passive-no-active-subject` (error): a `Voice=Pass` verb has no `nsubj`/`csubj`; `en.verbal.passive-subject-aux` (warn): `nsubj:pass` usually comes with `aux:pass`, except in `advcl` heads, *with*-constructions and sentences with no finite verb.

### perfect-have
`ai/en/seeds/verbal/perfect-have.md`

**Gist.** Perfect forms are built from *have* and a past participle (*has gone, had seen, will have finished*). The perfect is not a passive: *Moths have eaten the painting* is active, although *eaten* has the same form as in the passive.
**Conditions and exceptions.** The archaic *be*-perfect (*He is come*) survives only in *be gone, be done*, usually as an adjective after a copula. *Have* with an infinitive (*have to go*) is neither perfect nor auxiliary (see `semi-modals`); *have* meaning "possess" is `VERB`. The passive perfect has both auxiliaries: *has **been** eaten* (`aux` + `aux:pass`).
**In UD.** *have* is `aux`, `AUX`; the head is `VERB` with XPOS `VBN` (or `VBG` via *been*: *has been lying*), never `VB`, `VBZ`, `VBP`, `VBD`. A perfect participle without `aux:pass` has no `Voice=Pass`.
**Examples.** *I have eaten the plums.* — *It has been eaten by moths* (passive: *been* present). — *\*She has go.*
**Sources.** Jespersen, MEG IV (Time and Tense), ch. III; Poutsma 1923, *Participles*, §8; UD `docs/_en/feat/Voice.md`; UD `docs/_en/pos/AUX_.md`.
**Rules.** `en.verbal.perfect-have-participle` (error): the main verb with perfect *have* is not `VB|VBZ|VBP|VBD`; `en.verbal.perfect-not-passive` (warn): a `VBN` with *have* and `Voice=Pass` must have `aux:pass`.

### progressive-be-ing
`ai/en/seeds/verbal/progressive-be-ing.md`

**Gist.** The progressive (Poutsma's "expanded") form is *be* + *-ing* participle (*I am writing*, *he was sitting*). It presents an action in progress, temporary or incomplete, and also a planned future (*I am leaving tomorrow*) or disapproving repetition (*you are always grumbling*).
**Conditions and exceptions.** Stative verbs (*know, believe, belong, contain*, literal *see, hear*) rarely take the progressive (see `ai/en/seeds/lexicon/verb-stative-progressive.md`). Progressive copula *be* means behaviour (*you are being clever*); *being* is then `cop` and the head is the adjective. An *-ing* form after *be* without progressive meaning may be an adjective (*The film is interesting*) — `cop` + `ADJ`.
**In UD.** *be* is `aux` (not `cop`); the head is `VERB`, XPOS `VBG`, `VerbForm=Part|Tense=Pres`. Any `VBG` with `aux` is a participle (`Part`), not a gerund.
**Examples.** *We are eating cake.* — *She must be coming now.* — *You are being very clever* (*being* is cop).
**Sources.** Poutsma 1921, *The Expanded Form*, §§1–38; Jespersen, MEG IV, ch. XII–XIV (The Expanded Tenses); Brown 1851, Part II, Ch. VI (Compound or Progressive Form); UD `docs/_en/feat/VerbForm.md`; UD `docs/_en/dep/cop.md`.
**Rules.** `en.verbal.progressive-vbg-feats` (error): a `VBG` verb with `aux` has `VerbForm=Part`.

### purpose-clause
`ai/en/seeds/verbal/purpose-clause.md`

**Gist.** An infinitive can be a complement of the verb (*I want **to help***) or an adverbial of purpose (*I came **to see** you* = in order to see). A complement is required by the verb itself; a purpose adverbial can be added to almost any action and replaced by *in order to*. A phrase with *in order to* is always an adverbial.
**Conditions and exceptions.** A purpose infinitive after verbs of motion (*come, go, sit down*) is `advcl`; *go to see* differs from *want to see*. An infinitive on a noun (*a house to live in*) is `acl`.
**In UD.** A clause with `mark(in)` + `fixed(order)` is not `xcomp`, `ccomp` or `csubj` (usually `advcl`).
**Examples.** *He talked to him **in order to secure** the account* — `advcl`. — *I came to tell you* — `advcl`. — *I tried to finish it* — `xcomp`.
**Sources.** UD `docs/_en/dep/advcl.md`; UD `docs/_en/dep/xcomp.md`; Reed & Kellogg, *Higher Lessons*, Lesson 40; Poutsma 1923, *The Infinitive…*, §3.
**Rules.** `en.verbal.in-order-to-advcl` (warn): an *in order to* clause is not `xcomp|ccomp|csubj`.

### questions-inversion
`ai/en/seeds/verbal/questions-inversion.md`

**Gist.** Yes/no questions use inversion: the first auxiliary precedes the subject (*Are you going?*, *Has he left?*); without an auxiliary, *do* is added (*Did you see it?*). In wh-questions the wh-word comes first but keeps its role in the clause: *What did you see?* (object), *Who called?* (subject, no inversion), *Where do you live?* (adverbial). In archaic and poetic style the main verb itself inverts (*Have you any thoughts?*, *What say ye?*).
**Conditions and exceptions.** If the wh-word is the subject or modifies it, word order is normal (*Which boy won?*). Indirect questions (*I wonder where he lives*) have no inversion. In questions with copular *be*, the wh-predicative is the head (*What is that?* — `cop(What, is)`, `nsubj(What, that)`).
**In UD.** Dependencies are the same as in the declarative; only order changes: a fronted `obj` stands left of the verb, `aux` left of `nsubj`. Word-order rules (e.g. `iobj-before-obj`) give false warnings on questions.
**Examples.** *What did you see?* — `obj(see, What)`, `aux(see, did)`. — *Who is he?* — *How much notification would you give?* (fronted object phrase).
**Sources.** Reed & Kellogg, *Higher Lessons*, Lesson 55 (Arrangement — Interrogative Sentences); Brown 1851, Part II, Ch. VI (Form of Question); Jespersen, MEG V, ch. XXV (Questions); UD `docs/_en/dep/cop.md`.
**Rules.** None (descriptive seed).

### semi-modals
`ai/en/seeds/verbal/semi-modals.md`

**Gist.** *Have to* (must), *be going to* (intend), *used to* (habitual past) are close to modals in meaning but are ordinary verbs with an infinitive: *have* inflects (*has to, had to*), takes do-support (*doesn't have to*) and has an infinitive (*will have to*). *Ought to*, by contrast, is a true modal.
**Conditions and exceptions.** *Be* in *be going to* is an ordinary `aux` of *going*. *Gonna* is tokenized as *gon* + *na*: *gon* is `VERB`, *na* is `PART` (mark). *Had better* is a special fixed combination; *better* is not a verb there.
**In UD.** *have/going/used* are `VERB` and the head; the infinitive is `xcomp` with `mark(to)`. Not `aux`.
**Examples.** *I have to leave.* — *I'm going to take a nap.* — *It used to snow here.* — *You ought to go* (*ought* is AUX, MD).
**Sources.** UD `docs/_en/pos/AUX_.md`; UD `docs/_en/specific-syntax.md`; Poutsma 1923, *The Infinitive…*, §5 Obs. II and §32; Brown 1851, Rule XVII Note XII.
**Rules.** `en.verbal.have-to-verb`, `en.verbal.going-to-verb`, `en.verbal.used-to-verb` (error): *have*, *going/gon* and *used* with a *to*-infinitive `xcomp` are `VERB`.

### single-subject
`ai/en/seeds/verbal/single-subject.md`

**Gist.** Each predicate has at most one subject. Apparent double subjects are coordinated (*John and Mary* is one subject with two parts), belong to another (subordinate) clause, or are parentheticals, vocatives or dislocated elements (*My brother, he never listens*).
**Conditions and exceptions.** The only legitimate double subject is a predicate clause in a copular sentence: *The problem is that this has never been tried* — outer subject *problem* is `nsubj:outer`, inner *this* is `nsubj:pass` (see `outer-subject`). Placeholder *there/it* is not a subject (`expl`).
**In UD.** Among the children of a node at most one has `nsubj`, `nsubj:pass`, `csubj` or `csubj:pass`.
**Examples.** *Clinton defeated Dole* — one `nsubj`. — *My brother, he never listens* — *brother* is `dislocated`, not a second subject.
**Sources.** UD 2.18 validator, `too-many-subjects`; UD `docs/changes.md` (Multiple Subjects, v2.10); UD `docs/_en/dep/nsubj-outer.md`; Brown 1851, Rules II and III.
**Rules.** `en.verbal.single-subject` (error): at most one inner subject per head.

### subjunctive
`ai/en/seeds/verbal/subjunctive.md`

**Gist.** The subjunctive presents an action as a demand, supposition or wish rather than a fact. The present subjunctive is the bare form even in the third person (*I suggest that he **see** a doctor*, *It is vital that she **be** present*); the past subjunctive is *were* for all persons (*If I **were** rich…*). So *he see*, *I were* are not agreement errors when they are subjunctive.
**Conditions and exceptions.** The "mandative" subjunctive follows verbs of demand and advice (*suggest, insist, ask that, recommend*); in conditional and concessive clauses (*if, though, whether*) it has largely been replaced by the indicative. Brown advised avoiding the subjunctive where a reader would take it for an agreement error.
**In UD.** EWT 2.18 marks the subjunctive: `VB` → `Mood=Sub|VerbForm=Fin|Tense=Pres` (with person and number), *were* → `Mood=Sub|Tense=Past` (the data distinguish it even though the `Mood` guideline page says it is not distinguished). A `VB` with `VerbForm=Fin` is either imperative or subjunctive (checked by `en.morph.vb-finite-mood`).
**Examples.** *…suggesting that we **go** out and fight them.* — *Whether such an offer **were** accepted…* — *If it were not so, I would have told you.*
**Sources.** UD `docs/_en/feat/Mood.md` (Sub); UD `docs/_en/feat/Tense.md`; Brown 1851, Rule XIV Notes IX–X; Reed & Kellogg, *Higher Lessons*, Lesson 131; Jespersen, MEG IV, ch. IX–X (Imaginative use of Tenses).
**Rules.** `en.verbal.subjunctive-present-feats` (error): a `VB` with `Mood=Sub` has `VerbForm=Fin` and `Tense=Pres`.

### subordinators-mark
`ai/en/seeds/verbal/subordinators-mark.md`

**Gist.** A subordinate clause is introduced by a conjunction: *that, whether, if* for complement clauses; *because, although, though, if, unless, while, since, until, before, after, as, once, so that* for adverbial clauses. In UD they are all `mark`, attached to the predicate of the subordinate clause (not the main one). So are infinitival *to* and a preposition introducing a clause (*on **whether** users are at risk*: *on* and *whether* are both `mark`).
**Conditions and exceptions.** `mark` is a function word: not a noun, pronoun or adjective. Relative pronouns (*who, which, that* in relative clauses) are not `mark` but clause members (`nsubj`, `obj`, …). A subordinator may only have an adverbial modifier (*just because*, *right after*), `fixed` (*so that*, *even though*) or coordination. The head of `mark` is a clause (`advcl`, `ccomp`, `csubj`, `acl`, `xcomp`, `root`, `conj`, `parataxis`), not a nominal object or subject. `ai/en/seeds/errors/function-words-leaves.md` gives the complementary prohibited list.
**In UD.** `mark` is not `NOUN`, `PROPN`, `ADJ`, `PRON`, `DET`, `NUM`, `AUX`, `INTJ` (unless it has `ExtPos`); children of `mark` are only `advmod`, `obl`, `fixed`, `goeswith`, `reparandum`, `conj`, `cc`, `punct` (and negation); `SCONJ` is usually `mark` (or `fixed`).
**Examples.** *Forces engaged in fighting after insurgents attacked* — `mark(attacked, after)`. — *He says that you like to swim* — `mark(like, that)`. — *because of the rain* — here *because* is `case` (a preposition), not `mark`.
**Sources.** UD `docs/_en/dep/mark.md`; UD `docs/_en/specific-syntax.md` (Complementizers, subordinating conjunctions and the infinitival marker); UD 2.18 validator, `rel-upos-mark`, `leaf-mark-case`; Reed & Kellogg, *Higher Lessons*, Lessons 63–65, 71–72, 100–107; Brown 1851, Part II, Ch. IX (Of Conjunctions), Rule XXII.
**Rules.** `en.verbal.mark-upos` (error): `mark` is not a nominal/pronominal/adjectival/auxiliary word; `en.verbal.mark-leaf` (error): `mark` has only the allowed dependents; `en.verbal.mark-head-clausal` (warn): the head of `mark` is not a nominal or function-word relation; `en.verbal.sconj-is-mark` (warn): `SCONJ` is usually `mark`.

### tense-two-forms
`ai/en/seeds/verbal/tense-two-forms.md`

**Gist.** English verbs inflect for only two tenses: present (*walks*) and past (*walked*). Future (*will walk*), perfect (*has walked*), progressive (*is walking*) and pluperfect (*had walked*) are combinations with auxiliaries, not word forms, so there is no "future tense" or "aspect" feature on the verb.
**Conditions and exceptions.** The past participle has `Tense=Past` and the progressive *-ing* participle `Tense=Pres`, although they do not express time as such (Poutsma considered the names "present/past participle" unfortunate). Modals have no tense at all (see `ai/en/seeds/morph/verb-md-modals.md`).
**In UD.** `Tense` is only `Pres` or `Past`; English verbs have no `Aspect` feature (it is absent from the UD 2.18 registry for `VERB`/`AUX`). No separate `Aspect` rule is needed: the FEATS reader already rejects unknown feature pairs.
**Examples.** *I had been there* — *had*: `Tense=Past`, *been*: `VerbForm=Part|Tense=Past`, and no `Tense=Pqp`. — *She will go* — *go* has no `Tense=Fut`.
**Sources.** UD `docs/_en/feat/Tense.md`; UD 2.18 feature registry; Jespersen, MEG IV (Time and Tense), ch. I–II; Poutsma 1923, *Participles*, §2; Brown 1851, Part II, Ch. VI (Tenses).
**Rules.** `en.verbal.tense-pres-past` (error): `Tense` is `Pres` or `Past`.

### to-before-ing
`ai/en/seeds/verbal/to-before-ing.md`

**Gist.** After some words *to* is a preposition and is followed by an *-ing* form (gerund), not an infinitive: *I look forward **to seeing** you*, *She objected to being called*, *used to working*, *with a view to reducing*. Infinitival *to* before *-ing* is possible only in the progressive infinitive with *be* (*to be going*).
**Conditions and exceptions.** Test: can the *-ing* form be replaced by a noun? *look forward to the trip* — yes, so *to* is a preposition. *I want to go* — *\*I want to the trip* — no, so it is the infinitival particle.
**In UD.** *to* as `PART` with a `VBG` head must have an `aux` between them; otherwise *to* is `ADP` (with `mark`, since it introduces a clause).
**Examples.** *I look forward to seeing you* (*to* is ADP). — *She seems to be going* (*to* is PART on *going* via *be*).
**Sources.** UD `docs/_en/pos/PART.md`; UD `docs/_en/dep/mark.md`; UD `docs/_en/feat/VerbForm.md` (Ger); Poutsma 1923, *The Gerund*, §§43–45.
**Rules.** `en.verbal.to-part-before-ing` (error): particle *to* on a `VBG` requires an auxiliary after *to*.

### to-infinitive-marker
`ai/en/seeds/verbal/to-infinitive-marker.md`

**Gist.** *To* before an infinitive is neither a preposition nor an auxiliary but an infinitive marker. It attaches to the verb it introduces, which is non-finite: *to go* (VB), *to be defanged* (VBN with *be*), *to have gone* (VBN with *have*), *to be going* (VBG with *be*).
**Conditions and exceptions.** An adverb (*to boldly go*) or negation (*to not attempt*) can stand between *to* and the verb. In ellipsis *to* becomes the head (*update whatever you need to*). Older grammars (Brown, Reed & Kellogg) called *to* a preposition governing the infinitive, and some analyses treat it as an auxiliary; UD adopts neither view.
**In UD.** *to*: UPOS `PART`, XPOS `TO`, relation `mark` (never `aux`, `case`). The head is `VB` with `VerbForm=Inf`; if the head is `VBN`, an `aux`/`aux:pass` stands between *to* and it; the head is never finite.
**Examples.** *I tried to finish it.* — *It should continue to be defanged.* — *Change anything you need to.* (*to* is the head).
**Sources.** UD `docs/_en/dep/mark.md`; UD `docs/_en/dep/aux_.md`; UD `docs/_en/pos/PART.md`; Brown 1851, Rule XVIII; Poutsma 1923, *The Infinitive…*, §2 and §58.
**Rules.** `en.verbal.to-not-aux` (error): particle *to* is never `aux`, `aux:pass`, `case`, `cop`, `compound:prt`; `en.verbal.to-head-nonfinite` (error): its head is not finite; `en.verbal.to-vb-infinitive` (error): a `VB` head is `VerbForm=Inf`; `en.verbal.to-vbn-needs-aux` (error): a `VBN` head needs *be*/*have* after *to*.

### vb-infinitive-by-context
`ai/en/seeds/verbal/vb-infinitive-by-context.md`

**Gist.** The base form (*go, be, take*; tag `VB`) differs grammatically by context. With *to*, a modal or another auxiliary (*to go, can go, did not go*), or as a complement of another verb (*let him go, made me cry*), it is an infinitive. When it forms a predicate on its own without an auxiliary, it is finite: imperative (*Go!*) or subjunctive (*that he go*).
**Conditions and exceptions.** Exception: with imperative *do*, EWT marks the main verb `Imp/Fin` (see `imperative`). A bare infinitive in `xcomp` (*let him go*, *help me move*) is also `Inf`.
**In UD.** `VB` with `aux`/`aux:pass` (other than imperative *do*) is `VerbForm=Inf`; `VB` as `xcomp` is always `VerbForm=Inf`.
**Examples.** *I have to leave* (Inf). — *He should leave* (Inf). — *Leave now!* (Fin, Imp). — *They let him leave* (Inf, xcomp).
**Sources.** UD `docs/_en/feat/VerbForm.md`; Brown 1851, Rules XVIII–XIX; Poutsma 1923, *The Infinitive…*, §1.
**Rules.** `en.verbal.vb-aux-infinitive` (error): a `VB` verb with a non-imperative auxiliary is `VerbForm=Inf`; `en.verbal.xcomp-vb-infinitive` (error): a `VB` `xcomp` is `VerbForm=Inf`.

### vp-ellipsis
`ai/en/seeds/verbal/vp-ellipsis.md`

**Gist.** English often omits a repeated verb with its dependents, leaving the auxiliary: *Mary didn't leave, John **did***; *I can't, but she **can***; *So please update whatever you need **to***. The tree then has no main verb and the auxiliary (or *to*) takes its place: subject, negation and adverbials attach to it.
**Conditions and exceptions.** If the verb is omitted and two clause members remain (*Marie went to Paris and Miriam to Prague*), it is gapping (see `gapping-orphan`). Coordinated auxiliaries on one verb (*We can and will get there*) are not ellipsis: *will* is `conj` of *can*. Brown required repeating differing forms in such places (*as he has [made] mine*).
**In UD.** The auxiliary head keeps UPOS `AUX` (or `PART` for *to*) and receives the relation of the omitted verb (`conj`, `ccomp`, `xcomp`, `root`, …), not `aux`.
**Examples.** *John will win gold and Mary will too* — `conj(win, will)`, `nsubj(will, Mary)`. — *Change anything you need to* — head *to*.
**Sources.** UD `docs/_en/specific-syntax.md` (VP ellipsis); UD `docs/_en/dep/orphan.md`; Brown 1851, Rule XVII Note IX; Reed & Kellogg, *Higher Lessons*, Lesson 57.
**Rules.** None (descriptive seed).

### wh-adverb-clause
`ai/en/seeds/verbal/wh-adverb-clause.md`

**Gist.** *When, where, why, how* introduce subordinate clauses (*He was upset **when** I talked to him*, *I know **where** she lives*), but unlike *because* or *if* they also play a role inside the clause — adverbial of time, place, reason, manner. So they are adverbs, not conjunctions.
**Conditions and exceptions.** In free relatives (*I looked **where** you were sitting*) *where* is the head and the clause attaches to it as `advcl:relcl`. The same holds for *whenever, wherever*.
**In UD.** *when/where/why/how/whenever/wherever* are `ADV` (XPOS `WRB`) with `advmod` (or the head of a free relative), never `mark`.
**Examples.** *He was upset when I talked to him* — `advmod(talked, when)`, `advcl(upset, talked)`. — *I looked where you were sitting* — `advmod(looked, where)`, `advcl:relcl(where, sitting)`.
**Sources.** UD `docs/_en/dep/acl-relcl.md`; UD `docs/_en/dep/advcl-relcl.md`; UD `docs/_en/dep/advcl.md`; Reed & Kellogg, *Higher Lessons*, Lessons 59–60, 63.
**Rules.** `en.verbal.wh-adverb-not-mark` (error): these wh-adverbs are never `mark`.
