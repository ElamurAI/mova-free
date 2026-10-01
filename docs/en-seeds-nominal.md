# English grammar seeds: the noun phrase (`ai/en/seeds/nominal`)

A **seed** is a short Markdown note holding one piece of knowledge about English grammar, together with its source (a page of a reference grammar, a UD guideline or a paper). The name follows the image of a fruit stone: the seed is the kernel from which a rule grows. Every seed has the same sections:

- **Gist** — what the grammar says, in 1–3 sentences;
- **Conditions and exceptions** — when the statement holds and when it does not;
- **Examples** — short English phrases or sentences (✓ grammatical, ✗ ungrammatical, ~ colloquial or substandard);
- **In UD** — how the phenomenon looks in Universal Dependencies 2.18 annotation (English Web Treebank conventions): UPOS, XPOS (Penn Treebank tags), FEATS, relations;
- **Sources** — the grammars, UD guidelines and papers the seed is based on;
- optionally, one or more machine-readable ```` ```rule ```` blocks.

This folder, `nominal`, covers the noun phrase of the English parser `ai/en`: articles and other determiners, predeterminers and their order, adjectives in attributive and predicative use, noun number (count, mass, pluralia tantum, collectives), numerals, compounds and names (`compound`, `flat`, `nmod:desc`, `appos`), the possessive (`'s`, `nmod:poss`, double and group genitives), personal, possessive, reflexive, reciprocal, indefinite and relative pronouns, and clauses attached to nouns (`acl`, `acl:relcl`). Sibling folders are `morph` (inflection and word formation), `verbal` (the verb and the clause), `lexicon` (valency) and `errors` (typical errors of parsers, language models and learners).

## How a seed becomes a rule

The `ai/en/build.rs` script compiles the lexicon (`data/lemmas.tsv`, `data/forms.tsv`) into static hash tables and word constants; it does not read seeds. The ```` ```rule ```` blocks are loaded and parsed by the deterministic Rust rule engine `en::expert` (`ai/en/src/expert.rs`) into typed structures (tags, relations and features are enums, not strings). A `cargo test` gate checks that every seed parses and that rule ids are unique; a malformed rule is a parse error, never silently skipped.

A rule block looks like this:

```rule
rule: en.nominal.det-before-head
what: a determiner (det, det:predet) precedes its head
match: n[]; d[rel~det, head=n]
require: d[before=n]
severity: error
source: Poutsma GLME I Ch. VIII §150; UD en det, det:predet
```

- `rule` — a unique id (`en.nominal.…` for this folder).
- `what` — a one-line human description.
- `match` — one or more nodes, `name[condition, …]`, separated by `;`. The engine tries every binding of the nodes to distinct words of a sentence. Conditions test `upos`, `xpos`, `lemma`, `form`, `feats.X=V` / `feats.X` (feature present) / `!feats.X` (absent), `rel` (`rel~nsubj` matches the base relation with any subtype), `head`, word order (`before`, `after`, `next`, `prev`), `suffix`/`prefix` of the form and `lemma.suffix`/`lemma.prefix`. Any condition can be negated with `!=`; `a|b` means "one of"; `feats.X=@s` requires the same value as node `s`.
- `require` — clauses that must all hold for every binding (`;` = and, ` or ` = alternatives). Clause forms: `node[…]`, `not node[…]`, `exists x[…]` (some other word with these conditions exists), `none x[…]` (no such word).
- `unless` — exceptions: if any clause holds, the violation is dropped.
- `severity` — `error` (the annotation is certainly wrong) or `warn` (a suspicion worth checking).
- `source` — where the knowledge comes from (mandatory).

The CLI command `en expert-check <seeds-dir> [--gold <gold.conllu>] <file.conllu>…` runs all rules over CoNLL-U files and prints how often each rule fired, with up to three examples per rule (`EN_EXPERT_LIST=1` prints the full list of violations). Typical uses: validating annotation from a parser, a language model or a silver pipeline; giving a correction pass hints such as "word 5 violates rule X"; and constraining the parser.

Seeds without a rule block still document a construction and the UD analysis expected for it. Some seeds point to a rule that lives in another folder (for example `errors/…`).

Abbreviations used in the sources: **Poutsma GLME** = H. Poutsma, *A Grammar of Late Modern English* (1904–1929), volume (I–IV), chapter, section; **Curme 1931** = G. O. Curme, *A Grammar of the English Language*, vol. III *Syntax* (1931); **Santorini 1990** = B. Santorini, *Part-of-Speech Tagging Guidelines for the Penn Treebank Project*; **UD en** = the Universal Dependencies English guidelines (`docs/_en/…`), **UD u** = the universal guidelines (`docs/_u-dep/…`); identifiers such as `P17-1074` or `2023.udw-1.7` are ACL Anthology ids.

---

### a-adjectives
`ai/en/seeds/nominal/a-adjectives.md`

**Gist.** Adjectives with the old prefix *a-* (once a preposition: *on sleep* → *asleep*) — *afraid, asleep, awake, alive, alike, alone, aware, ablaze, ashamed, akin* — are predicative and almost never stand before a noun. Before a noun other words are used: *a sleeping child, a live fish, a lone rider, a frightened man*.
**Conditions and exceptions.** After the noun they are possible: *the only man alive, the bravest man alive*. With an intensifier they occasionally occur attributively (*a wide-awake child*). Poutsma also lists *ill* among predicative-only adjectives.
**Examples.** ✓ *The baby is asleep.*; ✓ *the greatest poet alive*; ✗ *an afraid dog*.
**In UD.** Such an ADJ as a prenominal `amod` is suspicious.
**Rules.** `en.nominal.a-adj-not-prenominal` (warn): an *a-*adjective that is `amod` of a noun must follow it.
**Sources.** Poutsma GLME III, Ch. XXVIII §8 b (after Onions, *Advanced English Syntax* §25).

### a-an-sound
`ai/en/seeds/nominal/a-an-sound.md`

**Gist.** *A* is used before a word beginning with a consonant **sound**, *an* before a vowel sound. What matters is the pronunciation of the next word, not its first letter.
**Conditions and exceptions.** *a unit, a European, a ewe, such a one* (initial [j] or [w]); *an hour, an heir, an honest man, an honour* (silent *h*); letter names and abbreviations go by sound: *an MP, an FBI agent, a UN report*. Before an unstressed *h-* syllable, nineteenth-century British usage had *an historical novel, an hotel*; today *a historical* prevails. Archaic: *an union, an universal, such an one*. Nonstandard *a* before a vowel (*a apple*) is an error in the text.
**Examples.** *a university*; *an hour*; *an X-ray*; *a history*, but older *an historian*.
**In UD.** *a* and *an* share the lemma and features, so the annotation does not record the choice: this is a check of the text, not of the annotation.
**Rules.** None in this seed; checking needs word adjacency and pronunciation (e.g. a pronouncing dictionary such as CMUdict).
**Sources.** Poutsma GLME III, Ch. XXXI §3 with notes II–V.

### absolute-genitive
`ai/en/seeds/nominal/absolute-genitive.md`

**Gist.** A genitive can stand without the noun it belongs to. Poutsma distinguishes three cases: the noun has just been mentioned (*My car is faster than John's*); a house, shop, firm or church is understood (*at the baker's, at my aunt's, St Paul's*); the genitive is predicative (*The book is John's*).
**Conditions and exceptions.** A classifying genitive (*a lady's umbrella*) is hardly ever used without its noun; the noun is repeated: *a gentleman's umbrella and a lady's umbrella*. Replacing it with *that of John* for a simple name is "not in the spirit of the language" (Poutsma).
**Examples.** *We met at Mary's.*; *I ate at McDonald's.*; *This pen is Tom's.*
**In UD.** The possessor noun takes over the role of the whole phrase (`obl`, `nsubj`, `obj`, predicate root), and `case('s)` stays attached to it.
**Rules.** None; such a POS is legitimate and differs from `nmod:poss` only by the missing noun.
**Sources.** Poutsma GLME III, Ch. XXIV §45–49; Curme 1931 §10 II.

### absolute-possessives
`ai/en/seeds/nominal/absolute-possessives.md`

**Gist.** Independent ("absolute", Poutsma) possessive pronouns replace a whole "possessive + noun" phrase: *This book is mine* (= my book), *Yours is better.* They never stand before a noun.
**Conditions and exceptions.** After a noun with an article, demonstrative or numeral, possession is expressed with *of* + the independent form: *a friend of mine, that dog of yours, two books of his* (double genitive). *His* and *its* have one form for both roles. Archaic *mine eyes* is the old prenominal form before a vowel.
**Examples.** *Yours is better.*; *a friend of mine*; *The choice is theirs.*
**In UD.** PRON, XPOS PRP, `Poss=Yes|PronType=Prs` + Person/Number, **without** `Case`; the lemma is the dependent form (*mine* → *my*, *yours* → *your*). The relation follows the role in the phrase or clause (`nsubj`, `obj`, `nmod` with `case` *of*, predicate), never `nmod:poss`.
**Rules.** `en.nominal.abs-poss-feats` (error): *mine/yours/hers/ours/theirs* are PRP with `Poss=Yes` and no `Case`. `en.nominal.abs-poss-not-det` (error): they are never `nmod:poss`.
**Sources.** Poutsma GLME IV, Ch. XXXIII §1, §21–23; Curme 1931 §57 5 "Substantive Forms of Possessive Adjectives"; UD en `feat/Poss`.

### adj-as-noun
`ai/en/seeds/nominal/adj-as-noun.md`

**Gist.** An adjective can name a class of people or an abstraction without a noun: *the rich, the poor, the wounded, the unknown, the English*. Poutsma calls this partial conversion: the word takes no plural or genitive *-s* and denotes all such people. Full conversion yields a real noun with plural, article *a* and numerals: *a native — natives, valuables, sweets*.
**Conditions and exceptions.** Nationality words in a sibilant (*the English, the French, the Swiss, the Chinese*) are partially converted; those in *-an* (*Americans, Germans*) are full nouns. A partially converted adjective takes a plural verb: *The rich are different.*
**Examples.** *The poor get poorer.*; *the best of friends*; *the natives* (already a noun).
**In UD.** A partially converted adjective stays ADJ (JJ, `Degree`): it takes `det` and the role of the phrase (`nsubj`, `obj`, `nmod`…), but has no `Number` (the UD 2.18 English registry does not allow `Number` on ADJ) and no `nummod`. With plural *-s* or a numeral it is a NOUN. *English* as a language name is PROPN in EWT.
**Rules.** `en.nominal.adj-no-number` (error): an ADJ has no `Number`. `en.nominal.adj-no-nummod` (warn): a numeral attached to an ADJ (*three whites*) signals a full noun.
**Sources.** Poutsma GLME III, Ch. XXIX §1, §13–15; Curme 1931 §57 "Substantive Function of Adjectives"; UD 2.18 English feature registry.

### adj-before-article
`ai/en/seeds/nominal/adj-before-article.md`

**Gist.** When an adjective is modified by *so, as, too, how, however*, the indefinite article comes **after** the adjective: *so big a house, too good a chance, as good a scholar as he, how long a wait*. Without such a modifier an adjective cannot precede the article.
**Conditions and exceptions.** A parallel order with the adjective after the noun exists: *a power so strong* (Curme). In *such a big house*, *such* is a predeterminer and the adjective follows the article. *Quite a, rather a* are predeterminers. Poutsma: likewise *no* + comparative (*no better a man*).
**Examples.** ✓ *too costly a sacrifice*; ✓ *so harsh an answer*; ✗ *big a house*.
**In UD.** `amod(house, big)`, `det(house, a)`, `advmod(big, so)`: the adjective is to the left of the `det`.
**Rules.** `en.nominal.adj-before-article` (warn): an `amod` adjective before *a/an* of the same noun must have an `advmod` *so/as/too/how/however/this/that/no*.
**Sources.** Poutsma GLME I, Ch. VIII §151, §153; Curme 1931 §10 I 1.

### adj-position
`ai/en/seeds/nominal/adj-position.md`

**Gist.** An attributive adjective in English precedes its noun (Curme: "adherent"). It follows the noun (Curme: "appositive") when it has its own dependents (*a man proud of his son*, *a plan so stupid that…*), when several adjectives are joined by a conjunction (*a laugh musical but malicious*), and after pronouns in *-thing/-body* (*something new*). Such a postposed adjective works like a reduced relative clause.
**Conditions and exceptions.** Fixed postpositives (often French in origin): *court martial, Poet Laureate, Postmaster General, President elect, the sum total, from time immemorial, God Almighty*. Adjectives in *-able/-ible* after a superlative or *only*: *the best style possible, the only person available*. Native ones: *the amount due, the people involved/concerned/present, five years old*. An adjective with a dependent can precede the noun only in *so/too/as + adjective + a + noun* (separate seed) or with hyphens (*an easy-to-read book*).
**Examples.** ✓ *a proud man*; ✓ *a man proud of his son*; ✓ *the people involved*; ✗ *a proud of his son man*.
**In UD.** `amod` (ADJ) is normally to the left of its head; to the right of a NOUN it usually has its own dependents or is a fixed postpositive (*available, involved, old, possible, due, left*…).
**Rules.** `en.nominal.postposed-adj-has-deps` (warn): an `amod` adjective after its noun must have a dependent, unless it is a listed fixed postpositive (*involved, concerned, present, due, left, old, dead, dear, elect, simple, total, martial, politic, laureate, general, immemorial, incarnate, almighty, minor*) or ends in *-able/-ible*.
**Sources.** Curme 1931 §10 I 1 and §10 I 1 a; Poutsma GLME III, Ch. XXVIII §6; UD en `dep/amod`.

### adnominal-noun-singular
`ai/en/seeds/nominal/adnominal-noun-singular.md`

**Gist.** A noun modifying another noun (*shoe shop, apple tree*) is usually singular even when the idea is plural: *toothbrush, a three-volume novel, a five-foot rope, a ten-year-old boy, rose cultivation*.
**Conditions and exceptions.** Fixed plural exceptions: *sales manager, sports car, arms race, savings account, systems analyst, customs officer*; with a plural head, *women writers, gentlemen farmers*. A number + measure used as a modifier also lacks *-s*: *a 2 year old*, *a two-hour delay*.
**Examples.** *a five-dollar bill*; *a two-hour meeting*; *sales figures* (exception).
**In UD.** The modifier is a `compound` before the head.
**Rules.** None; legitimate plural exceptions are too many, so number inside a `compound` is only a hint for a human reviewer.
**Sources.** Poutsma GLME III, Ch. XXV §31–35; Poutsma GLME I, Ch. IV §8 (*iron bedstead*, *Gladstone bag* — not apposition).

### appos
`ai/en/seeds/nominal/appos.md`

**Gist.** An appositive is a second noun phrase that names the same referent as the first and specifies it: *Sam, my brother, arrived*; *the Australian Broadcasting Corporation (ABC)*. Poutsma distinguishes identity (*Mr. Lloyd George, the late Prime Minister*), kind with genus (*the planet Mars*) and quantity with substance (*a dozen collars, a little wine*); Curme distinguishes loose apposition (with commas) and close apposition (*my brother John*).
**Conditions and exceptions.** In UD `appos` is used only for identity of two full noun phrases. Quantity with substance uses other relations (*a couple cookies* — `nmod:unmarked`); a noun modifier (*iron bedstead*) is `compound`; a clause attached to a noun (*the fact that…*) is `acl`, not `appos`; *the city of Rome* is `nmod` with *of*.
**Examples.** *Sam, my brother,* → `appos(Sam, brother)`; *Bill (John's cousin)* → `appos(Bill, cousin)`.
**In UD.** `appos` is always to the right of its anchor; the dependent is mostly NOUN, PROPN or NUM. An `appos` clause with the conjunction *that* is suspicious (should be `acl`).
**Rules.** `en.nominal.appos-right` (error): the appositive follows the word it specifies. `en.nominal.appos-not-content-clause` (warn): an `appos` with `mark` *that/whether* should be `acl`.
**Sources.** Poutsma GLME I, Ch. IV §4–8; Curme 1931 §10 III "Apposition"; UD en `dep/appos`, `dep/nmod-unmarked`.

### articles
`ai/en/seeds/nominal/articles.md`

**Gist.** English has two articles: definite *the* and indefinite *a/an*. *An* is not a separate word but the form of *a* before a vowel sound, so both share one lemma.
**Conditions and exceptions.** An article is a determiner of a noun or of an adjective used as a noun (*the rich*). The letter *a* as a name or list label (*a)*, *Type A*, *vitamin A*) is not an article: it is NOUN, SYM or LS.
**Examples.** *the book*; *a book*; *an apple*; *an hour*.
**In UD.** *the*: DET, DT, `Definite=Def|PronType=Art`. *a/an*: DET, DT, `Definite=Ind|PronType=Art`, lemma `a`. Relation `det`.
**Rules.** `en.nominal.the-feats` (error): *the* is DET DT with `Definite=Def`, `PronType=Art`. `en.nominal.a-an-feats` (error): *a/an* is DET DT with lemma `a`, `Definite=Ind`, `PronType=Art`.
**Sources.** Poutsma GLME III, Ch. XXXI §1–3; UD en `feat/Definite`, `feat/PronType`.

### attributive-only
`ai/en/seeds/nominal/attributive-only.md`

**Gist.** Some adjectives occur only with a noun and are never predicates: *main, mere, sheer, utter, chief, principal, lone, former, latter, very* (*the very man*), and relational adjectives that stand for a genitive or adverb (*a daily paper, a wooden house, a medical school*). Poutsma: nobody says *The journals are daily* — one says *The journals appear daily*.
**Conditions and exceptions.** In *The main thing is…*, *main* modifies a noun; it is not a predicate. *Former, latter* without a noun are converted (*the latter is…*), which is not predicative use. *Live* "broadcast live" can be a predicate (*The show is live*), so it is not on the list.
**Examples.** ✓ *the main road*; ✓ *sheer luck*; ✗ *The road is main.*; ✗ *His luck was sheer.*
**In UD.** Such an ADJ with a `cop` child is suspicious.
**Rules.** `en.nominal.attributive-only-adj` (warn): *main/mere/sheer/utter/chief/principal/lone* have no `cop` dependent.
**Sources.** Poutsma GLME III, Ch. XXVIII §7.

### a-with-plural
`ai/en/seeds/nominal/a-with-plural.md`

**Gist.** *A/an* comes from "one" and goes with a singular count noun. With a plural it is possible only through a quantity word that turns the group into "one portion": *a few days, a great many people, a good two hours, a further 45 days*.
**Conditions and exceptions.** In *a hundred people* the article belongs to the numeral (= *one hundred*); in *a lot of people*, to *lot*. A measure taken as a whole: *a three months*, *a twelve months* (literary). In a classifying genitive (*a children's story*) the article belongs to *story*, not to *children*.
**Examples.** ✓ *a few days*; ✓ *a good many years*; ✓ *a further 45 days*; ✗ *a books*.
**In UD.** `det(a)` on a noun with `Number=Plur` almost always co-occurs with an `amod` *few/many* or a `nummod` on the same noun.
**Rules.** The rule is `en.errors.a-singular` in `ai/en/seeds/errors/determiner-noun-number.md`.
**Sources.** Poutsma GLME III, Ch. XXVI §17; Poutsma GLME IV, Ch. XL §58–62 (*a few*), §89 (*a many*); UD en `dep/nmod-poss` (*a children's story*).

### classifying-genitive
`ai/en/seeds/nominal/classifying-genitive.md`

**Gist.** A genitive can be individual — "whose exactly" (*John's car*) — or classifying — "what kind, for whom, how much" (*a children's story, a women's college, a fool's errand, a day's work, two weeks' notice*). The classifying genitive describes a kind, like an adjective (Poutsma).
**Conditions and exceptions.** Therefore an article with a classifying genitive belongs to the head, not the possessor: in *a children's story*, *a* agrees with singular *story*, not with *children*. Genitives of measure belong here too: *a stone's throw, an hour's drive*.
**Examples.** *a children's book*; *a bachelor's degree*; *a day's journey*; *two hours' drive*.
**In UD.** `det(story, a)`, `nmod:poss(story, children)`, `case(children, 's)`. So *a/an* is never the `det` of a plural possessor.
**Rules.** `en.nominal.classifying-genitive-det` (error): a plural `nmod:poss` has no `det` *a*.
**Sources.** Poutsma GLME III, Ch. XXIV §7, §22–23, §40–44; Curme 1931 §10 II "Descriptive genitive", "Genitive of measure"; UD en `dep/nmod-poss`.

### collective-nouns
`ai/en/seeds/nominal/collective-nouns.md`

**Gist.** Collective nouns of the first type (*family, team, government, committee, army, party*) are singular in form but name a group; when the members are meant, verb and pronoun may be plural, especially in British English: *The committee are divided.* Collectives of the second type (*people, cattle, police, clergy, vermin*) are always plural in meaning: *these people, a hundred people*.
**Conditions and exceptions.** UD `Number` follows the form, not the meaning: *family* is `Sing` even with *are*, so subject–verb agreement cannot be checked strictly for collectives. *People* "nation" has the plural *peoples*.
**Examples.** *My family is/are here.*; *The police have arrived.*; *Many people were there.*
**In UD.** *people* is NOUN NNS `Number=Plur`. *Police* is annotated inconsistently (NNS Plur or NN Sing). First-type collectives are NN Sing.
**Rules.** `en.nominal.people-plural` (warn): the noun *people* is NNS with `Number=Plur`.
**Sources.** Poutsma GLME III, Ch. XXVI §6–7 and Ch. XXV §27–28 (*people, cattle*); Curme 1931 §8 1 d and Ch. XXVI "Collective Nouns".

### compound
`ai/en/seeds/nominal/compound.md`

**Gist.** English readily puts a noun before a noun as a modifier: *phone book, oil price, shoe shop, college friend*. The head is the last word, the modifier precedes it: *a phone book* is a book, not a phone. Poutsma treats such a noun as a substitute for a missing adjective (*an iron bedstead, Ceylon tea, a Gladstone bag*), not as apposition.
**Conditions and exceptions.** Chains have internal structure: *[oil price] futures*. Proper names with the generic word last (*Wall Street, Stanford University, Mirror Lake*) are also `compound` with a right-hand head; the reverse order (*Lake Mead, Mount Everest, Hotel California*) is `flat`. Names with a preposition (*Bank of America*) use ordinary syntax (`nmod`).
**Examples.** *phone book* — `compound(book, phone)`; *Wall Street* — `compound(Street, Wall)`; *Lake Mead* — `flat(Lake, Mead)`.
**In UD.** `compound` precedes its nominal head. The subtype `compound:prt` (verb particle) is a different matter.
**Rules.** `en.nominal.compound-head-final` (error): a `compound` dependent of a NOUN/PROPN precedes its head.
**Sources.** UD en `dep/compound`, `dep/nmod-desc` (place names); Poutsma GLME III, Ch. XXII §1–3; Poutsma GLME I, Ch. IV §8; ACL Anthology 2023.udw-1.7, 2020.udw-1.8.

### content-clause-acl
`ai/en/seeds/nominal/content-clause-acl.md`

**Gist.** Some nouns (*fact, idea, hope, belief, news, question, fear, doubt, rumor*) take a clause that spells out their content: *the fact that he left*, *the question whether we can do it*. This is not a relative clause: there is no gap, and *that* is a conjunction (it cannot be replaced by *which*). Curme calls it an "attributive substantive clause" in appositional relation.
**Conditions and exceptions.** Introduced by *that, whether*, after *fear* by *lest*, colloquially by *as*; it can be reduced to an infinitive (*the time to act*, *his plan to go*). Content clauses are mostly finite. A finite `acl` without `mark` can also be legitimate: omitted colloquial *that* (*the fact those horses lost…*), *no matter what…*, a quotation used as a modifier (*a bang-your-head-against-the-wall moment*); hence there is no rule against a missing `mark`.
**Examples.** *the hope that he may recover*; *the rumor that she quit*; *no doubt that she was lovely*.
**In UD.** The clause head is `acl` of the noun (not `acl:relcl`, `appos` or `ccomp`); *that* is SCONJ with `mark`. A `mark` *that* under `acl:relcl` is an error.
**Rules.** `en.nominal.relcl-no-mark-that` (error): an `acl:relcl` has no `mark` *that/whether* — such a clause is a content clause (`acl`).
**Sources.** Curme 1931, Ch. XIII §23 I "Attributive Substantive Clause"; UD en `dep/acl` (content clauses, as in CGEL); Poutsma GLME II, Ch. XV §1 (substantive clauses).

### dem-number
`ai/en/seeds/nominal/dem-number.md`

**Gist.** *This/that* go with singular nouns, *these/those* with plurals: *this book, these books*. These are the only English determiners that inflect for number.
**Conditions and exceptions.** Colloquial *these kind of men* is agreement by sense, *kind of* being felt as a modifier (Curme: colloquial in Britain, substandard in America). A measure as a single whole: *that ten years*, *this three weeks* (cf. *a three months*). Pluralia tantum take *these/those*: *these trousers*.
**Examples.** ✓ *this book*, *these books*; ~ *these kind of things*; ✗ *this books*; ✗ *those car*.
**In UD.** DET, lemma *this* (forms this/these) or *that* (that/those), `PronType=Dem`, `Number=Sing|Plur`. The head's number is its `Number` (`Plur` or `Ptan` for plural).
**Rules.** `en.nominal.dem-number` (warn): a demonstrative `det` and its noun have the same `Number`, except that a plural demonstrative may go with a `Ptan` noun.
**Sources.** Poutsma GLME IV, Ch. XXXVI §1; Curme 1931, Ch. XXVI §59 7 "Plural of Kind, Sort"; Poutsma GLME III, Ch. XXVI §17; ACL Anthology 2020.tacl-1.25 (BLiMP, determiner–noun agreement); P17-1074 (NOUN:NUM error type).

### det-before-head
`ai/en/seeds/nominal/det-before-head.md`

**Gist.** An article, demonstrative or quantifier (*the, a, this, every, no, some*) precedes the noun it determines. Only numerals, adjectives and noun modifiers can come between them. The determiner itself is always a separate function word, DET.
**Conditions and exceptions.** *All, both, each* after a noun or pronoun (*the boys all left*, *we both agree*, *see you all*) are no longer determiners but "floating" quantifiers, annotated differently in UD (e.g. `nmod:unmarked`). Numerals and possessives are not `det` but `nummod` and `nmod:poss`.
**Examples.** *the old house*; *every other day*; *no such thing*; ✗ *house the*.
**In UD.** UPOS DET, relation `det` (subtype `det:predet` for *all the*, *such a*) to the nominal head, always to its left.
**Rules.** `en.nominal.det-before-head` (error): a `det`/`det:predet` precedes its head. `en.nominal.det-is-det` (error): the `det` relation carries only DET.
**Sources.** Poutsma GLME I, Ch. VIII §150 (article, pronoun and numeral precede adjectives); UD en `dep/det`, `dep/det-predet`, `dep/nmod-unmarked` (*See you all*).

### det-prontype
`ai/en/seeds/nominal/det-prontype.md`

**Gist.** English determiners form a closed list, and each has a fixed semantic type: demonstrative (*this, that*), total — "all, every" (*all, every, each, both*), indefinite (*some, any, another, either, such*), negative (*no, neither*), interrogative and relative (*what, which, whatever*).
**Conditions and exceptions.** *Many, much, few, little, several, enough* are determiners in grammars (and CGEL), but in UD English they are ADJ with `amod`. Cardinal numerals are NUM with `nummod`. *Each* in *each other* is reciprocal (`PronType=Rcp`); see the reciprocal seed.
**Examples.** *this car* (Dem); *every day* (Tot); *some water* (Ind); *no idea* (Neg); *what time* (Int).
**In UD.** DET; XPOS DT, PDT for predeterminers, WDT for *what/which*. `PronType` follows the lemma; demonstratives also have `Number`.
**Rules.** `en.nominal.det-prontype-tot` (error): DET *all, every, both* → `PronType=Tot`. `en.nominal.det-prontype-each` (error): *each* → `Tot` or `Rcp`. `en.nominal.det-prontype-ind` (error): *some, any, another, either, such, quite, half* → `Ind`. `en.nominal.det-prontype-neg` (error): *no, neither* → `Neg`. `en.nominal.det-prontype-dem` (error): *this/that* → `Dem` with `Number`.
**Sources.** Poutsma GLME IV, Ch. XXXVI §1 (demonstratives), Ch. XXXVII §7–8 (*such*), Ch. XL (*all* §1–7, *any* §16, *both* §26–28, *each* §35–36, *every* §51, *neither* §109–111, *no* §114–115); CGELBank (EWT in CGEL analysis: *many, several* and numerals as category D); UD en `feat/PronType`.

### distributive-singular
`ai/en/seeds/nominal/distributive-singular.md`

**Gist.** *Every, each, either, neither* and *another* consider items one at a time, so the following noun is singular: *every day, each student, either side, another chance*. The verb with such a subject is singular too (Curme).
**Conditions and exceptions.** With a numeral or *few* the group becomes the unit: *every two weeks, every few days, another five years* — then the noun is plural. *Each* after a pronoun is a different construction (*they each got one*).
**Examples.** ✓ *every day*; ✓ *each student*; ✓ *every three hours*; ✗ *every days*; ✗ *each students*.
**In UD.** DET with `det`; the head has `Number=Sing`.
**Rules.** `en.nominal.distributive-sing` (warn): a plural noun with `det` *every/each/either/neither/another* must have a `nummod` or `amod` (e.g. *few*).
**Sources.** Poutsma GLME IV, Ch. XL §35–36 (*each*), §51 (*every*), §109–111 (*neither*); Curme 1931 §8 1 e.

### double-comparison
`ai/en/seeds/nominal/double-comparison.md`

**Gist.** Comparison is formed either with *-er/-est* (Poutsma: "terminational") or with *more/most* (Poutsma: "periphrastic"). Both together — *more happier, most unkindest* — occur only in older authors (Shakespeare: *the most boldest*) and in substandard speech.
**Conditions and exceptions.** In *more beautiful* the adjective itself stays in the positive degree; the degree is carried by *more*. Fossilized old double forms are no longer felt as such: *near*, *foremost, hindmost* (Curme). Compound adjectives: *more kind-hearted* and *kinder-hearted*.
**Examples.** ✓ *happier*; ✓ *more careful*; ~ *more happier* (substandard).
**In UD.** *more/most* are ADV (RBR/RBS) with `advmod` to the ADJ; the ADJ itself is `Degree=Pos`.
**Rules.** `en.nominal.double-comparison` (warn): an ADJ/ADV with `advmod` *more/most* is in the positive degree (or is an ADV not tagged RBR/RBS).
**Sources.** Poutsma GLME III, Ch. XXX §2, §29–30; Curme 1931 §53–54 "Older comparison, pleonasm"; ACL Anthology P17-1074 (ADJ:FORM), W19-4406.

### double-genitive
`ai/en/seeds/nominal/double-genitive.md`

**Gist.** When a noun already has an article, demonstrative or numeral, the possessor comes after it, with *of* and in the genitive at the same time: *a friend of my father's, that remark of Tom's, a play of Shakespeare's*. Pronouns use the independent form: *a friend of mine*. Poutsma calls this the "pleonastic genitive".
**Conditions and exceptions.** Most natural after *a, this/that, any, some, no* or a numeral. Meaning may differ: *a picture of John* depicts John, *a picture of John's* belongs to John.
**Examples.** *a cousin of Mary's*; *this idea of yours*; *two books of his*.
**In UD.** The possessor after *of* is `nmod` (not `nmod:poss`, which only precedes the head), with two `case` dependents: *of* on the left and *'s* on the right; a pronoun like *mine* is `nmod` with `case(of)`.
**Rules.** `en.nominal.double-genitive-nmod` (error): a possessor with both `case` *of* and POS *'s* is not `nmod:poss`.
**Sources.** Poutsma GLME III, Ch. XXIV §33–34; Poutsma GLME IV, Ch. XXXIII §23; Curme 1931 §10 II 1 b "Double Genitive".

### flat-names
`ai/en/seeds/nominal/flat-names.md`

**Gist.** A multiword personal name (*Hillary Rodham Clinton*) has no grammatical head inside, so UD builds it "flat": all words depend on the first. The same applies to names that begin with a type word (*Lake Mead, Mount Everest, Hotel California*), numbered identifiers (*page 394, Route 66, World War II*) and foreign names with function words (*Ludwig van Beethoven*). Poutsma notes that compound names made of "non-significant" parts behave as one unit (e.g. no article).
**Conditions and exceptions.** A title before a name (*Mr., Dr., President*) is not `flat` but `nmod:desc`; likewise *Jr., Inc.* after a name. Names with transparent structure (*Natural Resources Conservation Service, the King of Sweden*) use ordinary syntax.
**Examples.** *Barack Obama* → `flat(Barack, Obama)`; *Mr. Smith* → `nmod:desc(Smith, Mr.)`; *Route 66* → `flat(Route, 66)`.
**In UD.** `flat` is always to the right of its head. *Mr./Mrs./Ms.* are PROPN with `nmod:desc` to the left of the name.
**Rules.** `en.nominal.flat-head-first` (error): `flat` dependents follow the head. The title rule is `en.errors.title-desc` in `ai/en/seeds/errors/titles-and-company-suffixes.md`.
**Sources.** UD u `flat`; UD en `dep/flat`, `dep/nmod-desc` (personal names, numbered entities); UD validator (flat is left-to-right); Poutsma GLME III, Ch. XXXI §29–30.

### group-genitive
`ai/en/seeds/nominal/group-genitive.md`

**Gist.** In compound names and phrases, *'s* is placed after the last word of the whole group: *the King of England's power, my father-in-law's house, Julius Caesar's death, someone else's car*. But the possessor is the head of the group (*King, Caesar, someone*), not the last word. Curme: *King's of England* would sound like a plural, so the *-s* moved to the end.
**Conditions and exceptions.** Joint possession takes one *'s* at the end (*John and Mary's house*); separate possession takes one on each (*John's and Mary's books*). Before about 1500 one said *the King's property of England* (Curme).
**Examples.** ✓ *the King of England's*; ✓ *John Smith's car*; ✗ *the King's of England*.
**In UD.** `case('s)` attaches to the head of the possessor group: in *John Smith's* to *John* (*Smith* is `flat` of *John*), in *someone else's* to *someone*. So *'s* never hangs on a `flat` or `compound` part of a name. With coordination EWT attaches *'s* to the last conjunct (`conj`).
**Rules.** `en.nominal.group-genitive` (error): the head of POS *'s* is neither a `flat` nor a `compound` dependent.
**Sources.** Poutsma GLME III, Ch. XXIV §3–4; Curme 1931 §10 II 1 d "Group Genitive"; UD en `dep/nmod-poss`.

### hundred-thousand
`ai/en/seeds/nominal/hundred-thousand.md`

**Gist.** After a numeral, *hundred, thousand, million, dozen* take no *-s*: *two hundred people, three dozen eggs* — they are part of the number. The *-s* forms (*hundreds of people, thousands of years, dozens of times*) are nouns governing an *of*-phrase.
**Conditions and exceptions.** *A hundred* ≈ *one hundred*: the article replaces *one*. In a complex number the smaller member specifies the larger: *four thousand* — "four" attached to "thousand".
**Examples.** ✓ *five hundred dollars*; ✓ *hundreds of dollars*; ✗ *five hundreds dollars*.
**In UD.** *hundred* within a number is NUM CD with `nummod` to the noun; the multiplier is `compound`: `compound(hundred, two)`. *hundreds, thousands, millions, billions, dozens* are NOUN NNS `Number=Plur`, with the singular as lemma.
**Rules.** `en.nominal.hundreds-noun` (error): *hundreds/thousands/millions/billions/dozens* are NOUN with `Number=Plur`. `en.nominal.complex-number-compound` (warn): a NUM has no NUM `nummod` dependent (the smaller member is `compound`).
**Sources.** Poutsma GLME IV, Ch. XLII §1–2; Poutsma GLME III, Ch. XXV §28 (*a hundred people*); UD en `dep/compound` (numbers: *four thousand*).

### indefinite-compounds
`ai/en/seeds/nominal/indefinite-compounds.md`

**Gist.** Pronouns built from *some-, any-, no-, every-* + *-one, -body, -thing* are noun-like singular words. An adjective modifying them comes **after**: *something new, anyone else, nothing special, everything English*.
**Conditions and exceptions.** The first part sets the type: *some-/any-* indefinite, *every-* total, *no-* negative. *No one* is written separately (in UD *no* is DET, *one* PRON). The verb is singular (*Everyone is here*), while the referring pronoun is often *they*. Poutsma explains *-body, -thing* as "prop-words" of indefinite pronouns.
**Examples.** *something new*; *anybody else*; *Nobody knows.*; *everything you need*.
**In UD.** PRON, XPOS NN, `Number=Sing` + `PronType=Ind` (*some-, any-*), `Tot` (*every-*), `Neg` (*no-*). A modifier is `amod` to the right.
**Rules.** `en.nominal.every-compound-tot` (error): *everyone/everybody/everything* → PRON NN, `Sing`, `Tot`. `en.nominal.some-any-compound-ind` (error): *someone/somebody/something/anyone/anybody/anything* → PRON NN, `Sing`, `Ind`. `en.nominal.no-compound-neg` (error): *nobody/nothing* → PRON NN, `Sing`, `Neg`. `en.nominal.adj-after-indef-pronoun` (warn): an `amod` of a PRON NN follows it.
**Sources.** Curme 1931 §10 I 1 (a single adjective after an indefinite pronoun), §8 1 e; Poutsma GLME IV, Ch. XLIII §27–28, §35–36 (*body, thing* as prop-words); UD en `dep/amod` (*Anything else*).

### mass-nouns
`ai/en/seeds/nominal/mass-nouns.md`

**Gist.** Material and abstract nouns (Poutsma: "material and abstract nouns") are conceived without boundaries, so they have no plural and take no indefinite article. For some this is strict: *information, advice, furniture, equipment, luggage, baggage, homework, feedback*. A portion is named with *a piece of*: *a piece of advice*.
**Conditions and exceptions.** Many mass nouns become countable in another sense — kinds or portions: *wines, teas, a coffee*, *a knowledge of Latin*, literary *researches*. For the words listed above there is no such standard use; *informations, advices, an equipment* are typical non-native errors.
**Examples.** ✓ *some advice*; ✓ *a piece of furniture*; ✗ *an information*; ✗ *furnitures*.
**In UD.** NOUN NN with `Number=Sing`; a `det` *a/an* with them is suspicious.
**Rules.** `en.nominal.mass-no-a` (warn): *information, advice, furniture, equipment, luggage, baggage, homework, feedback* have no `det` *a*. `en.nominal.mass-no-plural` (warn): these and *knowledge, evidence, software, hardware* are `Number=Sing` and do not end in *-s* (*informations* is an error in the text).
**Sources.** Poutsma GLME III, Ch. XXV §22–24; Curme 1931, Ch. XXVI "Plural of Names of Materials", "Plural of Abstract Nouns"; ACL Anthology P17-1074 (NOUN:INFL, *informations* → *information*).

### news-singular
`ai/en/seeds/nominal/news-singular.md`

**Gist.** Some nouns end in *-s* but are grammatically singular: *news*, names of sciences in *-ics* as disciplines (*mathematics, physics, linguistics*), diseases (*measles*) and games (*billiards*). The verb is singular: *The news is good.*
**Conditions and exceptions.** *Politics, economics* can also be plural (*His politics are dubious*); in EWT they are NNS with `Number=Ptan`. *News* is always singular (no *a news*; one says *a piece of news*).
**Examples.** *No news is good news.*; *Mathematics is hard.*; *Measles is contagious.*
**In UD.** *news* is NOUN NN `Number=Sing`. Names of sciences are NN Sing or NNS Ptan, depending on use.
**Rules.** `en.nominal.news-sing` (error): *news* is NN with `Number=Sing`.
**Sources.** Poutsma GLME III, Ch. XXVI §12–13 (plural nouns construed as singulars); Curme 1931, Ch. XXVI "Plural Used as Singular".

### nmod-desc
`ai/en/seeds/nominal/nmod-desc.md`

**Gist.** A "bare" description before a name — a position, title or role without an article (*President Obama, spokesman John Smith, actor Martin Sheen*) — is a descriptor. A description with an article, possessive or numeral (*my friend Nick, the poet Burns*) is a full apposition. Poutsma describes the similar type *the planet Mars, the man Moses*, where the first word alone does not give the full meaning.
**Conditions and exceptions.** A descriptor can be dropped without harming the grammar. Suffixes *Jr., Inc., LLC, Corp.* after a name are descriptors as well (to the right).
**Examples.** *Dr. Smith* → `nmod:desc(Smith, Dr.)`; *my friend Nick* → `appos(friend, Nick)`; *Apple Inc.* → `nmod:desc(Apple, Inc.)`.
**In UD.** `nmod:desc` has no `det`, `nmod:poss` or `nummod`; its head is a PROPN. A description with a determiner before the name is `appos` from the description to the name.
**Rules.** `en.nominal.desc-bare` (error): an `nmod:desc` has no `det`, `nmod:poss` or `nummod`. `en.nominal.desc-head-propn` (warn): the head of `nmod:desc` is PROPN.
**Sources.** UD en `dep/nmod-desc`; Poutsma GLME I, Ch. IV §6 b; Curme 1931 §10 III "Close apposition".

### nmod-poss
`ai/en/seeds/nominal/nmod-poss.md`

**Gist.** A possessive modifier (*John's, my, the company's*) precedes what is possessed. After the noun, possession is expressed with *of*: *the office of the president*. Curme: the *s*-genitive prevails with living beings, *of* with inanimates.
**Conditions and exceptions.** A possessor noun without *'s* signals a missing apostrophe in the text (*the players wives*). The double genitive *a friend of John's* follows the noun but is `nmod` with *of* (separate seed). Pronouns *my, your, his* are `nmod:poss` without a clitic.
**Examples.** ✓ *Marie's book* — `nmod:poss(book, Marie)`; ✓ *my book*; ✓ *the office of the president* — `nmod(office, president)`.
**In UD.** `nmod:poss` is always to the left of its head. A NOUN/PROPN/NUM in `nmod:poss` has a POS child.
**Rules.** `en.nominal.poss-before-head` (error): `nmod:poss` precedes its head. `en.nominal.poss-noun-needs-s` (warn): a nominal `nmod:poss` has a POS (*'s* / *'*) dependent.
**Sources.** UD en `dep/nmod-poss`, `dep/nmod`; Poutsma GLME III, Ch. XXIV §7–11; Curme 1931 §10 II 1.

### no-none
`ai/en/seeds/nominal/no-none.md`

**Gist.** *No* stands only before a noun (*no money, no friends*): it is a negative determiner and cannot be used alone. Its independent counterpart is *none* (*None of them came*), for persons *nobody/no one*. Poutsma: *none* is the "absolute form of *no*", as *mine* is of *my*.
**Conditions and exceptions.** *No* can also be an adverb before a comparative (*no better, no longer*) and an interjection (*No, thanks*). After *none* the verb can be singular or plural.
**Examples.** ✓ *no idea*; ✓ *none of us*; ✗ *no of us*; ✓ *no longer* (adverb).
**In UD.** *no* before a noun: DET DT `PronType=Neg`, relation `det`; adverb: ADV RB `advmod`; interjection: INTJ UH `Polarity=Neg`. *None*: PRON NN `PronType=Neg`.
**Rules.** `en.nominal.no-det` (error): DET *no* is `det` with `PronType=Neg`. `en.nominal.none-pron` (warn): *none* is PRON with `PronType=Neg`.
**Sources.** Poutsma GLME IV, Ch. XL §114–115 (*no*), §135–137 (*none*); UD en `feat/Polarity`, `feat/PronType`.

### noun-number-xpos
`ai/en/seeds/nominal/noun-number-xpos.md`

**Gist.** The number of an English noun is visible in its form: singular is tagged NN (proper NNP), plural NNS (NNPS). The `Number` feature must agree with the tag.
**Conditions and exceptions.** Pluralia tantum (*trousers, goods, clothes*) are NNS with `Number=Ptan`. Invariable *sheep, fish, species, aircraft* get their number from context. Number follows the form, not the meaning: *family, police* meaning a group of people keep the number of their form.
**Examples.** *book* NN Sing; *books* NNS Plur; *the Alps* NNPS Plur; *clothes* NNS Ptan.
**In UD.** NOUN/PROPN with `Number=Sing|Plur|Ptan`.
**Rules.** `en.nominal.nn-sing` (error): NN/NNP → `Number=Sing`. `en.nominal.nns-plur` (error): NNS/NNPS → `Number=Plur` or `Ptan`.
**Sources.** Poutsma GLME III, Ch. XXV §1–14 (plural formation); Santorini 1990 (NN, NNS, NNP, NNPS); UD en `feat/Number`.

### np-order
`ai/en/seeds/nominal/np-order.md`

**Gist.** Modifiers before a noun come in a fixed order: predeterminer (*all, both*) → determiner or possessive (*the, these, my*) → numeral (*two, three*) → adjectives → noun modifier → head: *all my three old college friends*. Poutsma: article, pronoun and numeral precede adjectives and noun modifiers; numerals precede adjectives (*two great men*).
**Conditions and exceptions.** *First, last, next* and words of similar meaning (*other, only, same, past, top, final, additional, further, full, whole*) come **before** the cardinal: *the first three days, the last two years, the other two, an additional 20 minutes*; Poutsma also allows *the three first months*. An adjective with *so/as/too/how* precedes the article (*so big a house*, separate seed). A classifying genitive allows an adjective before it (*a new children's book*).
**Examples.** ✓ *the two old men*; ✓ *the next five years*; ✗ *great two men*; ✗ *two the men*.
**In UD.** For one noun: `det:predet` < `det`/`nmod:poss` < `nummod` < `amod` < `compound` < head.
**Rules.** `en.nominal.det-before-nummod` (error): a `det` precedes a prenominal `nummod` of the same noun. `en.nominal.det-before-compound` (warn): a `det` precedes a `compound` of the same noun. `en.nominal.adj-before-numeral` (warn): only *first/last/next* and similar listed adjectives may precede a prenominal `nummod`.
**Sources.** Poutsma GLME I, Ch. VIII §150, §156–157; Curme 1931 §10 I 1.

### numeral-noun-number
`ai/en/seeds/nominal/numeral-noun-number.md`

**Gist.** After *one* (1) the noun is singular; after *two* and higher numbers it is plural: *one book, two books, forty dollars*.
**Conditions and exceptions.** A measure used as a modifier stays singular: *a two-hour delay*, *a 5 year old*. *Percent/per cent* is invariable: *80 percent*. Abbreviated units (*5 lb., 8 GB*) have no number. *Dozen, hundred, thousand* after a numeral take no *-s*: *two dozen eggs*. In *one or two days* the noun agrees with the nearer *two*.
**Examples.** ✓ *one day*; ✓ *three days*; ✓ *a three-day trip*; ✓ *20 percent*; ✗ *two day*.
**In UD.** `nummod` (NUM) → head NOUN with `Number`. A measure in a modifier position (*2 year old*) is `obl:unmarked`.
**Rules.** `en.nominal.one-singular` (warn): a noun with `nummod` *one/1* is `Number=Sing`. `en.nominal.numeral-plural` (warn): a noun with `nummod` *two…ten, twelve, twenty, dozen, hundred, thousand* in a subject, object, oblique, nominal-modifier, root or conjunct role (and not an abbreviation) is `Plur` or `Ptan`.
**Sources.** Poutsma GLME IV, Ch. XLII §1–3; Poutsma GLME III, Ch. XXV §28, §31.

### nummod
`ai/en/seeds/nominal/nummod.md`

**Gist.** A numeral that counts things (*three books, forty dollars*) is a distinct kind of modifier. It precedes the noun, together with other determiner-like words (Poutsma: numerals precede adjectives).
**Conditions and exceptions.** After a currency sign the number is on the right: *$ 40* — `nummod($, 40)`. A number used as an identifier after a noun is not a quantity: *page 394, Route 66, World War II* — `flat`. Dates and addresses (*October 8, 1963*) are `nmod:unmarked`. Ordinals (*first, third*) are ADJ with `amod`. In a complex number the smaller member is `compound` of the larger (see the *hundred* seed).
**Examples.** ✓ *three books* — `nummod(books, three)`; ✓ *room 101* — `flat(room, 101)`; ✗ `nummod(page, 394)`.
**In UD.** `nummod` → NUM, to the left of a NOUN/PROPN (to the right only with the SYM *$*). NUM as `amod` is suspicious.
**Rules.** `en.nominal.nummod-num` (error): a `nummod` dependent is NUM. `en.nominal.nummod-before-noun` (error): a `nummod` of a NOUN/PROPN precedes it. `en.nominal.num-not-amod` (warn): a NUM is never `amod`.
**Sources.** UD en `dep/nummod`, `dep/nmod-desc` (numbered entities), `dep/nmod-unmarked` (dates); Poutsma GLME IV, Ch. XLII §3, §7, §11–12; Poutsma GLME I, Ch. VIII §156; ACL Anthology 2023.udw-1.7.

### ordinals
`ai/en/seeds/nominal/ordinals.md`

**Gist.** *First, second, third, 21st* name a position in a sequence, not a quantity. Syntactically they behave like adjectives: they follow the article (*the second day*) and can be intensified (*the very first*).
**Conditions and exceptions.** *First* can be an adverb (*First, we…* — ADV RB). *Second* as a noun means a unit of time. Ordinals and *last, next* can precede a cardinal: *the first three chapters* (more usual) and *the three first*.
**Examples.** *the second day*; *his 21st birthday*; *the first three weeks*.
**In UD.** ADJ, XPOS JJ, `Degree=Pos|NumForm=Word|NumType=Ord` (digit forms: `NumForm=Combi`, *2nd*), relation `amod`, not `nummod`.
**Rules.** `en.nominal.ordinal-not-nummod` (error): a word with `NumType=Ord` is not `nummod`. `en.nominal.ordinal-adj-feats` (warn): ADJ *first…tenth* have `NumType=Ord`.
**Sources.** Poutsma GLME IV, Ch. XLII §11–13; Poutsma GLME I, Ch. VIII §157; UD en `feat/NumType`.

### own
`ai/en/seeds/nominal/own.md`

**Gist.** The adjective *own* reinforces possession and stands only after a possessive pronoun or genitive: *my own house, John's own idea, a room of one's own*. Without a possessive *own* is not used (✗ *an own house*, a calque of German *ein eigenes Haus*).
**Conditions and exceptions.** The fixed sports term *own goal* is a compound noun. In *on my own, of their own, a room of one's own*, *own* stands without a noun (Poutsma: a partially converted adjective), but it still has a possessive.
**Examples.** ✓ *her own car*; ✓ *their very own*; ✓ *on my own*; ✗ *an own car*.
**In UD.** *own* is ADJ JJ `amod`; its head has an `nmod:poss`. Without a noun (*on my own*), *own* is an ADJ head (`obl`, `nmod`) with its own `nmod:poss`.
**Rules.** `en.nominal.own-needs-possessor` (warn): a noun with `amod` *own* has an `nmod:poss`. `en.nominal.own-alone-needs-possessor` (warn): *own* as `obl/nmod/obj/nsubj` has its own `nmod:poss`.
**Sources.** Poutsma GLME III, Ch. XXIV §38–39 and Ch. XXIX §28; Poutsma GLME IV, Ch. XXXIII §18–20.

### pluralia-tantum
`ai/en/seeds/nominal/pluralia-tantum.md`

**Gist.** Some nouns exist only in the plural. These are things made of two identical halves (*trousers, jeans, pants, scissors, pliers, glasses* "spectacles") and collective or abstract names (*clothes, goods, thanks, contents, surroundings, whereabouts, headquarters, premises, outskirts, belongings, proceedings*). In this sense they have no singular.
**Conditions and exceptions.** Paired items are counted with *pair*: *a pair of scissors*. Some have a singular with a different meaning: *glass* "material" — *glasses* "spectacles"; *content* — *contents*; *good* — *goods*. *Headquarters, means, whereabouts* can also take a singular verb.
**Examples.** *These trousers are new.*; *Thanks a lot.*; *The goods were shipped.*
**In UD.** NOUN NNS with `Number=Ptan` (the UD 2.18 English registry allows Ptan for NOUN and PROPN). The lemma is the plural form itself: *clothes*, *goods*. Decades (*1970s*) are also Ptan.
**Rules.** `en.nominal.ptan-lemmas` (warn): NNS nouns with lemma *trousers, jeans, pants, scissors, pliers, tongs, clothes, goods, outskirts, premises, whereabouts, headquarters, surroundings, belongings, proceedings* have `Number=Ptan`.
**Sources.** Poutsma GLME III, Ch. XXV §18–21; Curme 1931, Ch. XXVI "Nouns without a Singular"; UD 2.18 English feature registry (NOUN `Number=Ptan`).

### poss-clitic
`ai/en/seeds/nominal/poss-clitic.md`

**Gist.** The possessive (genitive) of an English noun is formed with the clitic *'s* (*John's, men's*) or with a bare apostrophe for plurals in *-s* (*the boys'*) and old names in *-s* (*Jesus', Socrates'*). In modern English only this "s-genitive" and the *of*-construction are alive (Curme).
**Conditions and exceptions.** *It's* = *it is*, not a possessive; *its* is a pronoun without an apostrophe. Names in a sibilant: *Jones's* and *Jones'* — spelling varies. The old "his-genitive" *John his book* (Curme) is now substandard.
**Examples.** *the boy's hat*; *the boys' hats*; *children's shoes*; *Mrs. Adams's wrapper* (Curme).
**In UD.** *'s* or *'* is a separate token: PART, XPOS POS, lemma `'s`, relation `case` to the head of the possessor group, to its right. The possessor is `nmod:poss` of the possessed noun.
**Rules.** `en.nominal.pos-clitic` (error): a POS token is PART with lemma `'s`, relation `case`, after its possessor.
**Sources.** Poutsma GLME III, Ch. XXIV §1–2; Curme 1931 §10 II 1; UD en `dep/nmod-poss`; Santorini 1990 (POS).

### poss-pronouns
`ai/en/seeds/nominal/poss-pronouns.md`

**Gist.** The dependent ("conjoint", Poutsma) possessive pronouns *my, your, his, her, its, our, their* are the genitive of the personal pronouns. They stand only before a noun and do not combine with an article (*a friend of mine*, not *a my friend*).
**Conditions and exceptions.** *His* is both dependent and independent (*The car is his*). *Her* is ambiguous: possessive (*her book*, PRP$) and object (*I saw her*, PRP). *Its* ≠ *it's*. A possessive before a gerund is its subject (*I object to his coming*).
**Examples.** *my book*; *their house*; *I object to his coming.*
**In UD.** PRON, XPOS PRP$, `Case=Gen|Poss=Yes|PronType=Prs` + Person/Number/Gender; relation `nmod:poss`, with a gerund `nsubj`. The lemma is the form itself (*my, your, his, her, its, our, their*).
**Rules.** `en.nominal.prp-poss-feats` (error): PRP$ is PRON with `Case=Gen`, `Poss=Yes`, `PronType=Prs`. `en.nominal.prp-poss-rel` (warn): PRP$ and WP$ are `nmod:poss` (or `nsubj`/`nsubj:pass` of a gerund, or `conj`); `det:poss` is not used in English.
**Sources.** Poutsma GLME IV, Ch. XXXIII §1–2, §7 and Ch. XXXII §2; UD en `feat/Poss`, `dep/nmod-poss`.

### predeterminers
`ai/en/seeds/nominal/predeterminers.md`

**Gist.** A few words stand **before** an article or possessive pronoun: *all, both, half* before *the/my/these*; *such, what, quite, rather, many* before the indefinite article.
**Conditions and exceptions.** Without a central determiner, *all, both* are ordinary `det` (*all people*, *both sides*). *Such* without an article is an adjective (*such people*: ADJ `amod`). *Many a* + singular is a literary distributive (*many a man*). *Half* can also be a noun (*the first half*). Poutsma: after *all/both* the definite article is often dropped (*all day*, *both hands*).
**Examples.** *all the time*; *both my parents*; *half an hour*; *such a pity*; *what a mess*; *quite a few*.
**In UD.** Relation `det:predet`, UPOS DET, XPOS PDT (*all, both, half, such, quite, many*) or WDT (*what*). `det:predet` precedes the `det`/`nmod:poss` of the same noun and nearly always co-occurs with it (exceptions such as *all those*, where the head is the pronoun itself).
**Rules.** `en.nominal.predet-before-det` (error): `det:predet` precedes the central `det`/`nmod:poss`. `en.nominal.predet-needs-det` (warn): a noun with `det:predet` also has a `det` or `nmod:poss`. `en.nominal.predet-xpos` (error): `det:predet` is DET with XPOS PDT or WDT.
**Sources.** Poutsma GLME I, Ch. VIII §155–156; Poutsma GLME III, Ch. XXXI §18; Poutsma GLME IV, Ch. XL §88 (*many a*); UD en `dep/det-predet`; Santorini 1990 (PDT).

### predicative-not-amod
`ai/en/seeds/nominal/predicative-not-amod.md`

**Gist.** An adjective is attributive — with a noun, without a verb (*a sick man*) — or predicative — through a copula (*The man is sick*) or as a predicative complement (*I found him sick*). Poutsma treats these as the main functions of the adjective.
**Conditions and exceptions.** In UD a predicative adjective with *be* becomes the head of the clause: subject and copula depend on it. With *find, make, consider* it is `xcomp`. Hence an adjective with a `cop` or a subject cannot be `amod`.
**Examples.** *The house is old.* → `nsubj(old, house)`, `cop(old, is)`; *an old house* → `amod(house, old)`.
**In UD.** `amod` has no `cop` or `nsubj` children.
**Rules.** `en.nominal.amod-no-cop` (error): an `amod` has no `cop` and no `nsubj*` dependent.
**Sources.** Poutsma GLME III, Ch. XXVIII §6; UD en `dep/amod`; UD u `cop`.

### pron-case-function
`ai/en/seeds/nominal/pron-case-function.md`

**Gist.** The nominative (*I, he, she, we, they*) is for the subject of a finite verb. The objective (*me, him, her, us, them*) is for objects, after prepositions, for the subject of a *for*-infinitive and in "detached" positions not tied to a finite verb: *It's me*, *Me too*, *taller than me*.
**Conditions and exceptions.** Colloquial speech often uses the objective for the nominative when the pronoun is not directly next to a finite verb; *Me and him went* is substandard. The opposite slip is hypercorrection: *between you and I*. UD annotation records the form (case = form), so in such sentences the annotation is correct; the rule only prompts checking the relation.
**Examples.** ✓ *She saw me.*; ✓ *for him to go*; ~ *Me and her went* (substandard); ~ *between you and I* (hypercorrection).
**In UD.** A `Case=Nom` pronoun as `obj`, `iobj`, `obl`, `nmod` is suspicious; a `Case=Acc` pronoun as subject of a finite verb is substandard text (but correct for subjects of infinitives and participles).
**Rules.** `en.nominal.nom-not-object` (warn): a `Case=Nom` PRON is not `obj`, `iobj`, `obl*` or `nmod*`. `en.nominal.finite-subject-nom` (warn): a PRP/WP with `Case` that is subject of a finite verb is `Case=Nom`. See also `en.errors.finite-subject-nominative-auxcop` in `ai/en/seeds/errors/subject-pronoun-case.md`.
**Sources.** Poutsma GLME IV, Ch. XXXII §4–12; Curme 1931 §8 IV "Case"; UD en `feat/Case`; ACL Anthology P17-1074 (PRON).

### pron-paradigm
`ai/en/seeds/nominal/pron-paradigm.md`

**Gist.** Personal pronouns are the only English words with a full case paradigm: nominative (*I, he, she, we, they*), objective (*me, him, her, us, them*), possessive (*my, his…*). In *you* and *it* nominative and objective have merged. Gender is distinguished only in the 3rd person singular: *he* masculine, *she* feminine, *it* neuter.
**Conditions and exceptions.** *They* is plural in form even when it refers to one person ("singular they"): UD keeps `Number=Plur`. *You* has no number (singular and plural merged), while *yourself/yourselves* do. *Thou, thee, thy* are archaic (`Style=Arch`). The *'s* in *let's* is *us* (lemma *we*, `Case=Acc`).
**Examples.** *I saw him.*; *She met us.*; *They know it.*
**In UD.** PRON, XPOS PRP (possessives PRP$); `PronType=Prs`, `Person`, `Number` (except *you*), `Gender` (3rd person singular only), `Case=Nom|Acc|Gen`. The lemma is the nominative: *me* → *I*, *him* → *he*, *us* → *we*, *them* → *they*.
**Rules.** `en.nominal.pron-nom-forms` (error): *I, he, she, we, they* → PRP, `Case=Nom`, `Prs`. `en.nominal.pron-acc-forms` (error): *me, him, us, them* → PRP, `Case=Acc`, `Prs`. `en.nominal.pron-masc` / `en.nominal.pron-fem` / `en.nominal.pron-neut` (error): *he/him/his/himself*, *she/her/hers/herself*, *it/its/itself* → 3rd person singular with the matching `Gender`. `en.nominal.pron-plur` (error): *we/us/our/ours/ourselves*, *they/them/their/theirs/themselves* → `Number=Plur`. `en.nominal.pron-first-sing` (error): *I/me/my/mine/myself* → `Person=1`, `Number=Sing`. `en.nominal.you-no-number` (error): *you/your/yours* → `Person=2`, no `Number`.
**Sources.** Poutsma GLME IV, Ch. XXXII §1–2, §10, §20 and Ch. XXXV §11 (*you* replacing *thou*); Curme 1931 §3 (only pronouns have a distinct nominative); UD en `feat/Case`, `feat/Person`, `feat/Gender`, `feat/Number`.

### prop-word-one
`ai/en/seeds/nominal/prop-word-one.md`

**Gist.** To let an adjective or determiner stand without a noun, English adds *one/ones*: *the red one, a big one, these ones, the ones I like*. This is not the numeral "one" but a substitute for a count noun (Poutsma: "prop-word").
**Conditions and exceptions.** Only for count nouns: of milk one says *the white*, not *the white one*. After a possessive it is usually not used (*my one* is colloquial). Curme regards *one* here rather as a suffix that nominalizes the adjective; for annotation it is a noun.
**Examples.** *I want the blue one.*; *Which ones?*; *the one on the left*.
**In UD.** NOUN (NN *one*, NNS *ones*), `Number=Sing|Plur`; it takes `det`/`amod` and the role of the whole phrase. The numeral *one* (NUM) has no `det`/`amod`; *ones* is always NOUN.
**Rules.** `en.nominal.num-one-no-det` (warn): NUM *one* has no `det*` or `amod` dependent. `en.nominal.ones-noun` (error): *ones* is NOUN with `Number=Plur`.
**Sources.** Poutsma GLME IV, Ch. XLIII §1–3; Curme 1931 §57 1 "Use of the Suffix One".

### quantity-adj-number
`ai/en/seeds/nominal/quantity-adj-number.md`

**Gist.** *Many, several, few, both* count separate items, so the noun is plural: *many people, few chances, both hands*. *Much* (and *little* meaning "not much") measures mass, so it goes with a singular mass noun: *much time, little money*.
**Conditions and exceptions.** *Many a man* is the literary distributive *many a* with a singular (`det:predet`; see predeterminers). *Little* meaning "small" is an ordinary adjective: *little kids*. Colloquial *much more people* occurs, but it is a deviation.
**Examples.** ✓ *many people*; ✓ *several times*; ✓ *much time*; ✗ *much books*; ✓ *many a day* (literary).
**In UD.** *many, several, few, much, little* are ADJ JJ with `amod` (not DET); *both* is DET with `det`.
**Rules.** `en.nominal.count-quantifier-plural` (warn): a noun with `amod`/`det` *many, several, few, both* is `Plur` or `Ptan`. `en.nominal.much-singular` (warn): a noun with `amod` *much* is `Sing`.
**Sources.** Poutsma GLME IV, Ch. XL §26–30 (*both*), §57–62 (*few*), §64–68 (*little*), §85–90 (*many*, *many a*), §91–93 (*much*); Poutsma GLME III, Ch. XXV §22.

### reciprocal
`ai/en/seeds/nominal/reciprocal.md`

**Gist.** *Each other* and *one another* express a mutual action among several participants: *They love each other.* The combination is inseparable and behaves as a single object pronoun.
**Conditions and exceptions.** A plural subject or several subjects are required. The genitive is *each other's* (*each other's houses*). Historically *each* was the subject and *other* the object (*They each loved the other*), but now both words stand together and act as one.
**Examples.** *We help each other.*; *They looked at one another.*; *each other's names*.
**In UD.** The first word is the head with `ExtPos=PRON|PronType=Rcp` (*each* DET DT, *one* PRON CD); the second (*other* ADJ JJ, *another* DET) is `fixed`. The whole unit takes the relation of its role (`obj`, `obl`, `nmod:poss`).
**Rules.** `en.nominal.reciprocal-fixed` (error): in *each other* / *one another* with `fixed`, the first word has `PronType=Rcp` and `ExtPos=PRON`.
**Sources.** Poutsma GLME IV, Ch. XL §37–39; UD en `feat/PronType` (Rcp), `feat/ExtPos`; UD u `fixed`.

### reduced-relatives
`ai/en/seeds/nominal/reduced-relatives.md`

**Gist.** A relative clause is often reduced to a participle or infinitive (Curme: "abridgment"): *the man [who is] sitting there*, *a book [that was] written in 1900*, *a chance to win*, *a bagel to eat*. A participle with dependents follows the noun; a single participle usually precedes it, like an adjective (*a sleeping child*).
**Conditions and exceptions.** Curme: a participle with noticeable verbal force follows the noun (*a man shot*); as a property it precedes (*an unheard-of crime*). UD distinguishes infinitives: with a gap corresponding to the noun (*a bagel to eat*) — `acl:relcl`; without such a gap (*a way to get my discount*) — `acl`.
**Examples.** *sites offering booking* → `acl(sites, offering)`; *a parakeet named Cookie* → `acl`; *a suggestion to make* → `acl:relcl`.
**In UD.** `acl` with `VerbForm=Part|Ger|Inf` follows the noun (rare left-hand cases are mostly quoted modifiers such as *a bang-your-head-against-the-wall moment*). A participle before the noun is `amod`.
**Rules.** `en.nominal.acl-after-head` (warn): an `acl` of a NOUN/PROPN/PRON follows its head.
**Sources.** Curme 1931, Ch. XIV §23 II 11 "Abridgment of Relative Clause", §10 I 1; Poutsma GLME II, participle clauses §2; UD en `dep/acl`, `dep/acl-relcl` (infinitival relatives).

### reflexive-agreement
`ai/en/seeds/nominal/reflexive-agreement.md`

**Gist.** A reflexive pronoun repeats the subject of its verb, so it agrees with it in person and number: *I hurt myself*, *They blamed themselves*, *We enjoyed ourselves*.
**Conditions and exceptions.** Agreement is with the subject of the same verb (within one clause). In the imperative the subject *you* is not expressed (*Help yourself!*). With singular *they* — *themselves* or the rare *themself*. Pronouns like *everyone, someone* (PRON NN, not PRP) take *themselves* or *himself/herself*.
**Examples.** ✓ *We enjoyed ourselves.*; ✓ *She blamed herself.*; ✗ *He hurt myself.*
**In UD.** A subject PRP and a reflexive `obj`/`iobj` of the same verb share person and number. The rule compares features of two nodes (`feats.Person=@s`), so one rule covers all persons and numbers.
**Rules.** `en.nominal.reflexive-agreement` (warn): a reflexive `obj/iobj/obl` has the same `Person` as the PRP subject of its verb, and the same `Number` unless the subject has no `Number` (*you*) or is *they*.
**Sources.** Poutsma GLME IV, Ch. XXXIV §3–4.

### reflexive
`ai/en/seeds/nominal/reflexive.md`

**Gist.** Pronouns in *-self/-selves* have two roles. Reflexive: an object coreferent with the subject (*He hurt himself*). Emphatic (intensifying): "himself, in person", attached to a noun or subject (*The king himself came*, *I did it myself*).
**Conditions and exceptions.** In writing the roles cannot always be told apart; the criterion is function. The reflexive occupies an object or prepositional-object slot (`obj`, `iobj`, `obl`); the emphatic attaches to a noun or verb without a preposition. Colloquial *myself* for *I/me* (*John and myself went*) is nonstandard.
**Examples.** *She blamed herself.* (reflexive); *Einstein himself was there.* (emphatic); *Do it yourself.* (emphatic).
**In UD.** PRON PRP, `Case=Acc|Reflex=Yes` + Person/Number/Gender. Reflexive: `PronType=Prs`; emphatic: `PronType=Emp`, relation `nmod:unmarked` (with a noun) or `obl:unmarked` (with a verb). *Yourself* is `Number=Sing`, *yourselves* `Plur`.
**Rules.** `en.nominal.self-reflex` (error): PRON in *-self* → `Reflex=Yes`, `Case=Acc`, `Sing`. `en.nominal.selves-reflex` (error): PRON in *-selves* → `Reflex=Yes`, `Case=Acc`, `Plur`. `en.nominal.emphatic-self-rel` (warn): a `PronType=Emp` reflexive is `nmod:unmarked`, `obl:unmarked` or `appos`. `en.nominal.unmarked-self-emp` (error): a reflexive in `nmod:unmarked`/`obl:unmarked` is `PronType=Emp`.
**Sources.** Poutsma GLME IV, Ch. XXXIV §1–4, §29–31; Curme 1931 §56 D "Intensifying myself, himself"; UD en `dep/nmod-unmarked` (iv), `feat/Reflex`, `feat/PronType`.

### relative-that
`ai/en/seeds/nominal/relative-that.md`

**Gist.** *That* in a relative clause (*the book that I read*) is a relative pronoun, like *which/who*: it fills the subject or object slot of the clause. Unlike *who/which*, it cannot be preceded by a preposition: *the house in which I live*, or *the house that I live in* (stranded preposition), but not *in that I live*.
**Conditions and exceptions.** *That* is always clause-initial, so it is impossible where the relative word must be preceded by a preposition, *all/both* or a participle (Poutsma). *That* introduces mainly restrictive clauses; in non-restrictive ones (with a comma) *who/which* are used, and old *that* there is archaic. CGEL treats *that* as a subordinator, UD as a pronoun.
**Examples.** ✓ *the man that I met*; ✓ *the house that Jack built*; ✓ *the tool that I work with*; ✗ *the tool with that I work*.
**In UD.** PRON, XPOS WDT, `PronType=Rel`; its relation is its role in the clause (`nsubj`, `obj`, `nsubj:pass`, `obl` with a stranded preposition…), not `mark`. A preposition (`case`) of relative *that* is only to its right.
**Rules.** `en.nominal.relative-that-not-mark` (error): *that* before the head of an `acl:relcl` is PRON and not `mark`. `en.nominal.relative-that-no-preposition` (error): a `case` dependent of relative *that* follows it.
**Sources.** Poutsma GLME IV, Ch. XXXIX §12, §16–17, §34; Poutsma GLME I, preface (*that* as a relative pronoun); UD en `dep/acl-relcl` (relativizers), `dep/mark`; CGELBank (*that* as subordinator); ACL Anthology 2023.udw-1.7.

### relcl-after-head
`ai/en/seeds/nominal/relcl-after-head.md`

**Gist.** A relative clause (*the man who came, the book I read*) follows the noun it modifies and contains a gap corresponding to that noun: *the book [that I read ___]*. Poutsma divides such clauses into restrictive (no comma, they select the referent) and non-restrictive (with a comma, they add a fact); Curme into "restrictive" and "descriptive".
**Conditions and exceptions.** The head is usually a noun or pronoun (*those who, something that, all that*), sometimes an adjective used as a noun (*the best that…*). A clause referring to a whole sentence (*…, which was a bad idea*) is `advcl:relcl`, not `acl:relcl`.
**Examples.** *the book that I read*; *people who care*; *everything you said*.
**In UD.** The clause head is `acl:relcl` of the noun, always to its right. The relative pronoun takes its role in the clause (`nsubj`, `obj`, `obl`…). The head is NOUN, PRON, PROPN, rarely DET, ADJ, NUM, SYM.
**Rules.** `en.nominal.relcl-after-head` (error): an `acl:relcl` follows its head. `en.nominal.relcl-head-nominal` (warn): the head of `acl:relcl` is NOUN, PROPN, PRON, NUM, DET, ADJ or SYM.
**Sources.** Poutsma GLME II, Ch. XVI §1; Poutsma GLME IV, Ch. XXXIX §6; Curme 1931, Ch. XIV §23 II 6 "Descriptive and Restrictive"; UD en `dep/acl-relcl`.

### relcl-subject
`ai/en/seeds/nominal/relcl-subject.md`

**Gist.** A relative pronoun can be omitted (Curme: "asyndetic relative clause") when it is an object (*the man [whom] I saw*), the object of a stranded preposition (*the house [that] I live in*) or a predicate (*the man [that] he is*). When it is the subject, omission is impossible in standard English: *the man who came*, not *the man came*.
**Conditions and exceptions.** In colloquial and dialect speech a subject pronoun drops after *there is / here is* (*There's a man wants to see you*). In a clause with *there*, *there* takes over the subject role (*the problems there are*). Hence a finite relative clause on a noun has an `nsubj` (relative pronoun or its own subject) or an `expl` *there*.
**Examples.** ✓ *the book I read* (`nsubj` = I); ✓ *the man who came* (`nsubj` = who); ~ *There's a man wants you* (colloquial).
**In UD.** A finite `acl:relcl` on a NOUN/PROPN without `nsubj`/`expl` signals a broken text or annotation.
**Rules.** `en.nominal.finite-relcl-subject` (warn): a finite `acl:relcl` of a NOUN/PROPN has a dependent `nsubj`, `nsubj:pass`, `nsubj:outer`, `csubj`, `csubj:pass` or `expl`.
**Sources.** Poutsma GLME IV, Ch. XXXIX §27–29; Curme 1931, Ch. XIV §23 II 10 "Asyndetic Relative Construction"; UD en `dep/acl-relcl` (reduced relative clauses).

### single-det
`ai/en/seeds/nominal/single-det.md`

**Gist.** A noun has at most one central determiner: an article, demonstrative, possessive or quantifier. *The my book*, *a this car* are impossible. To combine an article with possession one says *a friend of mine*, *this idea of yours*.
**Conditions and exceptions.** Predeterminers *all, both, half, such, what, quite* precede the central determiner (*all the, both my, such a*); this is a separate relation `det:predet`, not a second `det`. *Many, few, other, same* after a determiner are adjectives (*the many, a few, the other*). Archaic *these my children* (Poutsma) is a rare literary exception.
**Examples.** ✓ *my old car*; ✓ *all my friends*; ✓ *a friend of mine*; ✗ *the my car*; ✗ *a his friend*.
**In UD.** A noun has at most one `det` child and never both a `det` and a pronominal `nmod:poss`.
**Rules.** `en.nominal.single-det` (warn): a noun has at most one `det`. `en.nominal.det-with-poss-pron` (warn): a noun with a pronominal `nmod:poss` has no `det`.
**Sources.** Poutsma GLME IV, Ch. XXXIII §11, §23; Poutsma GLME I, Ch. VIII §155–156; Curme 1931 §10 II 1 b (double genitive).

### there-expletive
`ai/en/seeds/nominal/there-expletive.md`

**Gist.** In existential sentences (*There is a cat on the roof*) *there* is a formal subject without the meaning "in that place"; the real subject (*a cat*) follows the verb. Poutsma: unstressed *there* "takes over the subject role to the ear".
**Conditions and exceptions.** It differs from the locative adverb *there* by being unstressed and not answering "where?". Both can occur in one sentence: *There's a dog there.*
**Examples.** *There are three options.*; *There seems to be a problem.*
**In UD.** Existential: PRON, XPOS EX, `PronType=Dem`, relation `expl`; the noun subject is `nsubj`. Locative *there* is ADV RB `advmod`.
**Rules.** `en.nominal.ex-there` (error): an EX token is PRON with `PronType=Dem` and relation `expl`.
**Sources.** Poutsma GLME IV, Ch. XXXIX §28 d; UD en `dep/expl`; Santorini 1990 (EX).

### who-whom-whose
`ai/en/seeds/nominal/who-whom-whose.md`

**Gist.** A relative pronoun takes its case from its role in the clause (Curme): *who* — subject, *whom* — object or prepositional object, *whose* — possessive. *Who/whom* are for persons, *which* for things, *that* for both (Poutsma).
**Conditions and exceptions.** In speech *who* for *whom* is normal (*the man who I saw*, Curme). *Whom* as a subject (*the man whom I think is guilty*) is hypercorrection. *Whose* is also used for things (*a house whose roof leaks*).
**Examples.** *the boy whom I trusted*; *the boy whose knife was lost*; *the boy to whom I gave it*.
**In UD.** *who, whom*: PRON WP, `PronType=Rel` (in questions `Int`); *whose*: PRON WP$, `Poss=Yes`, relation `nmod:poss`; *which*: PRON WDT or DET (*which book*).
**Rules.** `en.nominal.whom-not-subject` (warn): *whom* is not `nsubj*`. `en.nominal.whose-poss` (warn): PRON *whose* is WP$ with `Poss=Yes` and relation `nmod:poss`.
**Sources.** Poutsma GLME IV, Ch. XXXIX §4, §9–12; Curme 1931, Ch. XIV §23 II 7–8 "Personality and Form", "Case of Relative".

### you-it-case
`ai/en/seeds/nominal/you-it-case.md`

**Gist.** *You* and *it* have one form for nominative and objective, so case is determined by function: the subject of a finite verb is nominative; objects and prepositional objects are objective.
**Conditions and exceptions.** The subject of an infinitive after *for* (*for you to decide*) and the subject of a gerund are objective. Formal *it* (`expl`) as subject is nominative, as object (*I find it hard to…*) objective.
**Examples.** *You* (Nom) *saw it* (Acc). *It* (Nom) *hit you* (Acc). *It is time for you* (Acc) *to go.*
**In UD.** EWT assigns `Case` to *you/it* by function: subject of a finite verb `Case=Nom`; `obj`, `iobj`, `obl`, `nmod` `Case=Acc`. A mismatch here is an annotation error, not a text error.
**Rules.** `en.nominal.you-it-nom` (error): *you/it* as subject of a finite verb → `Case=Nom`. `en.nominal.you-it-acc` (error): *you/it* as `obj`, `iobj`, `obl`, `obl:unmarked`, `obl:agent`, `nmod`, `nmod:unmarked` → `Case=Acc`.
**Sources.** Poutsma GLME IV, Ch. XXXII §4, §8, §10; Curme 1931 §3; UD en `feat/Case`.
