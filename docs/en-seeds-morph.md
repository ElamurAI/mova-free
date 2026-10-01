# English seeds: morphology (`ai/en/seeds/morph`)

A **seed** is a short Markdown note that states one fact of English grammar, with its sources. A checkable rule grows from it. This folder covers **morphology**: inflection (number, degree, tense and person forms), the Penn Treebank (PTB) tags and Universal Dependencies (UD) features that encode those forms, lemmas, spelling changes, pronoun paradigms and word formation. Seeds in `ai/en/seeds/lexicon` cover verb valency instead.

## How seeds fit into `ai/en`

`ai/en` is a deterministic English parser written in Rust: tokenizer, PTB tagger, lemma/form morphology (`morph.rs`), dependency parser and word-order generator. Seeds feed it in two ways:

- **The lexicon is compiled; the seeds are not.** `ai/en/build.rs` compiles `data/lemmas.tsv` (the lemma heap) and `data/forms.tsv` (every form with its tag, lemma, UD frequency and source: UD, AGID or a stem) into static open-addressing hash tables (`lexicon.bin`). It also writes word constants (`words.rs`, e.g. `w::THE`) for the 5,000 most frequent lemmas plus the lemmas listed in `data/consts.txt`. At runtime nothing is read from disk, and a word check is an integer comparison. The morphology seeds describe the facts these tables and `morph.rs` must respect: which tag goes with which form and which lemma belongs to an irregular form.
- **Rules are interpreted.** The fenced ```` ```rule ```` blocks inside seeds are loaded from the seeds directory by the rule engine `en::expert`. The command `en expert-check <seeds dir> [--gold <file>] <file>` runs them over CoNLL-U, and annotation tools use them as hints ("token 5 breaks rule X"). The `cargo test` gate `seeds_all_parse` requires every rule in every seed to parse and every rule id to be unique. A rule that cannot be parsed is an error, never silently skipped.

## Seed format

Each seed has these sections:

- **Gist**: what the grammar says, in 1–3 sentences.
- **Conditions and exceptions**.
- **Examples**: short English examples, often tagged.
- **In UD**: how the phenomenon appears in UD 2.18 / EWT annotation (UPOS, XPOS, FEATS, relations).
- **Sources**: grammars (Sweet, Whitney, Jespersen, Kruisinga, Mätzner, Poutsma, Brown), Santorini 1990 (the PTB tagging guidelines), UD documentation and ACL Anthology paper ids.
- Zero or more ```` ```rule ```` blocks.

## The `rule` block

```rule
rule: en.morph.comparative-form
what: a form tagged JJR or RBR ends in -er (or is more, less, worse)
match: a[xpos=JJR|RBR, !feats.Typo]
require: a[suffix=er|more|less|worse]
severity: warn
source: Santorini 1990, §2, p. 1
```

- `match`: one or more nodes, separated by `;`. A node is written `name[cond, cond, …]`. The engine tries every binding of distinct words in the sentence that satisfies all the conditions.
- `require`: what must hold for each binding; if it does not, the binding is a violation. Clauses are separated by `;`, and ` or ` inside a clause gives alternatives. Clause forms: `name[…]` (a condition on a bound node), `not name[…]`, `exists c[…]` (some other word satisfies the conditions), `none c[…]` (no word does).
- `unless`: optional exceptions. These clauses use the same forms as `require`; if any of them holds, the violation is dropped.
- Conditions: `upos=`, `xpos=`, `lemma=`, `form=` (case-insensitive; `a|b` means "one of"); `feats.X=V`, `feats.X!=V`, `feats.X` and `!feats.X` (the feature is present or absent); `rel=` (exact relation) and `rel~` (relation by base, so `nsubj` also matches `nsubj:pass`); `head=v` and `head=0`; `before=`, `after=`, `next=`, `prev=` (word order); `suffix=` and `prefix=` on the form; `lemma.suffix=` and `lemma.prefix=`; negation of any condition with `!=`; `feats.X=@s` (same value as node `s`); `lemma=@form`.
- `severity`: `error` means the annotation is certainly wrong; `warn` means it is suspect.
- `source`: required. It says where the rule comes from.

Most morphology rules exclude tokens marked `Typo` (`!feats.Typo`), because a misspelled form keeps the annotation of the intended word.

## Seeds

### adj-as-noun
`ai/en/seeds/morph/adj-as-noun.md`

**Gist.** English adjectives do not inflect, but they can head a noun phrase in two ways. In *partial* conversion (*the rich, the poor, the English, the unknown*) the word takes no *-s* and no *a*, so it is still an adjective (ADJ, JJ, `Degree=Pos`, no `Number`). In *full* conversion (*a native → natives, a criminal, the ancients, his betters, goods*) it takes plural *-s* and *a*, so it is a noun (NOUN, NN/NNS with `Number`).

**Conditions and exceptions.** Test: if the word can take plural *-s* and *a/an*, it is a noun; if it cannot, but can be intensified (*the very rich*), it is an adjective. Nationality words ending in a sibilant (*the English, the French, the Dutch, the Chinese*) convert only partially, while others convert fully (*an American, Americans*). Language names (*He speaks Chinese*) are nouns (PROPN or NOUN in EWT). The prop-word *one* carries the noun endings (*a good one, the red ones*), and a noun used before a noun (*gold watch, silk thread*) stays a noun with `compound`.

**Examples.** *The **rich** get richer* (ADJ); *the **natives** of the island* (NOUN, NNS); *the **unknown*** (ADJ); *two **Americans*** (PROPN/NOUN with `Number=Plur`).

**Sources.** Sweet, *A New English Grammar* (NEG) I §§106–107, 179–180, 1035; Whitney §§76, 144, 196; Santorini 1990 §4.1; Jespersen, *Modern English Grammar* (MEG) II 11.3–11.5; Kruisinga II.3 §§1789–1803, 1850; Mätzner I pp. 270–272.

### adj-comparison-er-est
`ai/en/seeds/morph/adj-comparison-er-est.md`

**Gist.** Short adjectives form the comparative with *-er* and the superlative with *-est* (*big → bigger → biggest*, *happy → happier → happiest*), while long ones use *more/most*. The choice depends on length and sound shape, and in a transition zone both forms occur. The lemma of an *-er/-est* form is the positive (*bigger → big*).

**Conditions and exceptions.** Monosyllables, disyllables stressed on the last syllable (*polite, severe*) and many initially stressed disyllables, especially those in *-y, -ow, -le, -er* (*happy, narrow, simple, clever*), take *-er/-est*. Words in *-ful, -ish, -ive, -ous, -st*, participial adjectives in *-ed/-ing* and most words of three or more syllables take *more/most*. Spelling changes follow `spell-consonant-doubling`, `spell-silent-e` and `spell-y-to-i`. Flat adverbs compare the same way (*harder, sooner*). Only the suppletive *more, less, worse* and *most, least, worst* break the *-er/-est* shape.

**Examples.** *a **bigger** house; the **easiest** way; **nicer** weather; the **latest** news.*

**Sources.** Sweet NEG I §§1038–1039; Whitney §§199–200; Santorini 1990 §2; UD `docs/_en/feat/Degree.md`; Kruisinga II.3 §§1723–1729; Mätzner I pp. 273–275.

**Rules.** `en.morph.comparative-form` (warn): a JJR/RBR form ends in *-er* or is *more/less/worse*. `en.morph.superlative-form` (warn): a JJS/RBS form ends in *-est* or is *most/least/worst*.

### adj-comparison-periphrastic
`ai/en/seeds/morph/adj-comparison-periphrastic.md`

**Gist.** Long adjectives and nearly all *-ly* adverbs are compared with *more/most* (or *less/least*): *more difficult, most carefully, less expensive*. The adjective itself stays in the positive degree (JJ, `Degree=Pos`), and the degree is carried by *more/most*.

**Conditions and exceptions.** *More/most* before an adjective or adverb is an adverb (RBR/RBS, `advmod`). Before a noun (*more money*) it is a quantifying adjective (JJR/JJS, `amod`). Used alone, PTB tags it JJR as an object (*eat more*) and RBR as an adverbial (*relax more*). Double comparison (*more braver*) is now substandard; if it occurs, *braver* is still tagged JJR. Intensifying *most* (*a most interesting book*) is RBS. In *more than* (*more than ten*), EWT tags *more* ADJ with `ExtPos=ADV`.

**Examples.** *a **more** difficult task* (*more* RBR, *difficult* JJ `Degree=Pos`); *the **most** carefully planned trip*; ***less** expensive*.

**Sources.** Sweet NEG I §§1038–1039, 1041, 1524; Whitney §200; Santorini 1990 §4.2; UD `docs/_en/feat/Degree.md`; Kruisinga II.3 §§1727–1729, 1777–1778; Mätzner I pp. 280–282.

**Rules.** `en.morph.more-most-advmod-adv` (warn): *more/most/less/least* as `advmod` (without `ExtPos`) is ADV with RBR/RBS.

### adj-comparison-suppletive
`ai/en/seeds/morph/adj-comparison-suppletive.md`

**Gist.** A few very frequent words form their degrees from another root: *good/well → better → best*, *bad/badly/ill → worse → worst*, *much/many → more → most*, *little → less → least*. Some have double forms with different meanings: *farther/further*, *older/elder*, *later/latter*, *latest/last*, *nearest/next*.

**Conditions and exceptions.** Part of speech decides the lemma: ADJ *better/best* → *good*, ADV → *well*; ADJ *worse/worst* → *bad*, ADV → *badly* (EWT sometimes uses *bad*). *Elder/eldest* refers to family seniority only. *Latter* and *last* have their own lemma and the tag JJ. *Further* meaning "additional" is JJ with lemma *further*, while comparative *further* (*further away*) is RBR with lemma *far*. *Lesser* is a double comparative. In EWT *more, most, less, least* keep their own lemmas, while *fewer → few*, *later/latest → late* and *older → old*.

**Examples.** *a **better** plan* (ADJ, lemma *good*); *she sings **better*** (ADV, lemma *well*); ***worse** luck* (ADJ, *bad*); ***more** people* (ADJ JJR, lemma *more*).

**Sources.** Sweet NEG I §§1044–1052, 1525; Whitney §§202, 316; Santorini 1990 §§2, 4.1, 4.2; UD EWT 2.18 lemma practice; Kruisinga II.3 §§1731–1733.

**Rules.** `en.morph.better-best-adj` (error): ADJ *better/best* → lemma *good*. `en.morph.better-best-adv` (error): ADV → *well*. `en.morph.worse-worst-adj` (error): ADJ → *bad*. `en.morph.worse-worst-adv` (warn): ADV → *badly* or *bad*. `en.morph.more-most-lemma` (error): degree-tagged *more/most/less/least* keep their own lemma. `en.morph.farther-further-lemma` (warn): degree-tagged *further/farther/furthest/farthest* → *far*.

### adj-degree-tags
`ai/en/seeds/morph/adj-degree-tags.md`

**Gist.** PTB tags encode degree in the tag name: JJ/JJR/JJS for adjectives and RB/RBR/RBS for adverbs. The UD feature `Degree` repeats this, so tag and feature must agree: Pos ↔ JJ, Cmp ↔ JJR/RBR, Sup ↔ JJS/RBS.

**Conditions and exceptions.** Every JJ adjective has `Degree=Pos`. Among adverbs, only those with their own comparison forms have it (see `adv-flat-degree`); ordinary *-ly* adverbs have no `Degree`. In EWT an adjective inside a proper name keeps the tag NNP but gets UPOS ADJ and its degree (*Greater London*: ADJ, NNP, `Degree=Cmp`). Periphrastic comparison does not change the adjective's degree.

**Examples.** *young* JJ `Degree=Pos` — *younger* JJR `Degree=Cmp` — *youngest* JJS `Degree=Sup`; *sooner* RBR `Degree=Cmp`; *best* (adverb) RBS `Degree=Sup`.

**Sources.** UD `docs/_en/feat/Degree.md`, `docs/_en/pos/ADJ.md`; Santorini 1990 §2.

**Rules.** `en.morph.jj-degree-pos` (error): ADJ JJ → `Degree=Pos`. `en.morph.jjr-rbr-degree-cmp` (error): JJR/RBR → `Degree=Cmp`. `en.morph.jjs-rbs-degree-sup` (error): JJS/RBS → `Degree=Sup`. `en.morph.degree-cmp-tag` and `en.morph.degree-sup-tag` (error): the reverse direction, also allowing NNP. `en.morph.jj-is-adj` (error): JJ/JJR/JJS → UPOS ADJ (or X).

### adj-false-comparatives
`ai/en/seeds/morph/adj-false-comparatives.md`

**Gist.** Not every word in *-er* is a comparative, and not every word in *-est* is a superlative. JJR/RBR and JJS/RBS mark only forms with a live comparative or superlative meaning. Words like *other, proper, clever, bitter, honest, modest, earnest* are plain JJ.

**Conditions and exceptions.** Some roots simply end in *-er* (*proper, clever, bitter, eager, tender, sober, sinister, sheer, mere, severe, sincere*); these can still be compared (*cleverer*). Some former comparatives (*other, former, latter, upper, inner, outer, utter*) are now plain adjectives, used without *than*. The Latin comparatives (*superior, inferior, senior, junior, major, minor, prior, interior, exterior*) take *to*, not *than*, and are JJ `Degree=Pos`. Words in *-est* without superlative meaning (*honest, modest, earnest, manifest, west, interest*) are compared with *more/most*. Agent nouns in *-er* are a separate suffix. In PTB an *-er* form without clear "more" meaning is JJ (*further details*) or RB (*come by later*).

**Examples.** *the **other** side* (JJ); *the **former** president* (JJ); ***superior** to* (JJ); *an **honest** man* (JJ); *a **clever** trick* (JJ).

**Sources.** Sweet NEG I §§1038, 1043, 1527, 1746; Whitney §§202, 211; Santorini 1990 §§2, 4.1; Kruisinga II.3 §§1732, 1747, 1759–1761, 1772; Mätzner I pp. 278–280.

**Rules.** `en.morph.false-comparative-er` (error): listed *-er*/Latin forms are never JJR/RBR. `en.morph.false-superlative-est` (error): *honest, modest, earnest, manifest, west, interest* (and negatives) are never JJS/RBS.

### adv-flat-degree
`ai/en/seeds/morph/adv-flat-degree.md`

**Gist.** Some adverbs have the same form as the adjective, without *-ly* (*work hard, run fast, come late, fly high*). These, plus *soon, often, well, badly, far, little*, have their own degree forms (*harder, fastest, sooner, better, worse, further, less*). Adverbs in *-ly* are compared only with *more/most*.

**Conditions and exceptions.** Pairs with different meanings each keep their own lemma: *hard/hardly, late/lately, most/mostly, near/nearly*. Colloquial speech also compares *-ly* adverbs with the plain form (*easier said than done*). *Seldom* has a rare *seldomer*, and *rather* is a frozen comparative. In UD only these adverbs carry `Degree=Pos` in the positive; other ADV tokens have no `Degree`.

**Examples.** *She works **hard*** (RB, `Degree=Pos`) — ***harder*** (RBR) — ***hardest*** (RBS); *come **soon*** — ***sooner***; *sing **well*** — ***better*** — ***best***.

**Sources.** UD `docs/_en/feat/Degree.md`; Sweet NEG I §§342, 1498–1499, 1524–1525; Whitney §§99, 313d, 316; Jespersen MEG VI 22.9; Kruisinga II.2 §§943–945, II.3 §1714; Mätzner I pp. 396–398.

**Rules.** `en.morph.rb-degree-flat-only` (warn): RB with `Degree` only for *hard, fast, late, long, high, easy, early, far, soon, low, close, well, badly, little, near, deep, quick, slow, loud, often, seldom*. `en.morph.rb-degree-pos` (error): RB with `Degree` has `Degree=Pos`.

### adv-ly
`ai/en/seeds/morph/adv-ly.md`

**Gist.** The suffix *-ly* productively turns adjectives (and participles) into adverbs (*quick → quickly, willingly, reportedly*). It is not a reliable sign of an adverb, though: added to a noun, it makes an adjective (*friendly, manly, daily*).

**Conditions and exceptions.** Spelling: consonant + *-y* → *-ily* (*happily*); *-le* → *-ly* (*simply, ably*); *-ll* → *-lly* (*fully*); *-ic* → *-ically* (*basically*, except *publicly*); also *truly, duly, wholly*. Adjectives in *-ly* (*friendly, lovely, lonely, ugly, silly, costly*) form no adverb (*in a friendly way*). *Daily, weekly, monthly, yearly, early, kindly, only* are both adjective and adverb, and syntax decides which. Endings *-ously, -ically, -fully, -lessly, -ably/-ibly, -tively/-sively, -ingly, -edly* give only adverbs. The lemma of a *-ly* adverb is the adverb itself (RB, no `Degree`).

**Examples.** *She spoke **quietly**. He is **friendly*** (ADJ). *The paper comes out **daily*** (ADV) / *a **daily** paper* (ADJ). ***Surprisingly**, it worked.*

**Sources.** Sweet NEG I §§341, 1500; Whitney §§94, 193, 313a; Santorini 1990 §§2, 4.1; UD EWT 2.18 lemma practice; Jespersen MEG VI 22.7–22.9; Kruisinga II.3 §§1709–1716; Mätzner I pp. 395, 441.

**Rules.** `en.morph.derived-adverb-suffix` (warn): words in *-ously, -ically, -fully, -lessly, -ably, -ibly, -tively, -sively, -ingly, -edly* are ADV (or PROPN/X). `en.morph.ly-adjective` (warn): *friendly, lovely, lonely, ugly, silly* etc. are ADJ (or NOUN/PROPN), not adverbs.

### det-articles
`ai/en/seeds/morph/det-articles.md`

**Gist.** The indefinite article is *a* before a consonant **sound** and *an* before a vowel **sound**. Pronunciation decides, not spelling: *an hour, an MP, an FBI agent* but *a university, a one-time offer, a European*. Both forms share the lemma *a*. *The* is the only definite article.

**Conditions and exceptions.** Silent *h* takes *an* (*an honest man, an heir*); for sounded *h* modern usage has *a* (*a hotel*), though older texts have *an hotel*. With abbreviations the name of the first letter decides (*an SMS, a UN report*). An adjective between article and noun decides (*an old car*). Text errors (*a apple*) do not change the annotation. In UD both are DET, DT, `det`: *a/an* have lemma *a* with `Definite=Ind|PronType=Art`, and *the* has `Definite=Def|PronType=Art`.

**Examples.** *a book, an apple, an hour, a university, the end.*

**Sources.** UD `docs/_en/feat/Definite.md`, `feat/PronType.md`, `docs/_en/pos/DET.md`; Santorini 1990 §2; Sweet NEG I §1137; Whitney §§220–221; Kruisinga II.2 §§1305–1307; Mätzner I p. 317.

**Rules.** `en.morph.article-indefinite` (error): DET *a/an* → lemma *a*, DT, `Definite=Ind`, `PronType=Art`. `en.morph.article-definite` (error): DET *the* → lemma *the*, DT, `Definite=Def`, `PronType=Art`.

### noun-genitive-group
`ai/en/seeds/morph/noun-genitive-group.md`

**Gist.** Possessive *'s* attaches to a whole phrase, at its end, not to a word: *the King of England's crown*, *somebody else's coat*, colloquial *the man I saw yesterday's son*, *my son-in-law's car*. So *'s* is a clitic, not a case ending.

**Conditions and exceptions.** Joint possession takes one *'s* (*John and Mary's house*); separate possession takes one per owner (*John's and Mary's houses*). Apposition: *my friend the hunter's rifle*. Time and measure: *two hours' drive*. Long groups are usually rephrased with *of*. In UD, *'s* (PART, POS) attaches by `case` to the **head** of the possessor phrase, which is `nmod:poss` of the possessed noun. So *'s* always follows its head but is not always next to it; in EWT about one *'s* in ten is separated from its head.

**Examples.** *the King of England's crown; somebody else's idea; my father-in-law's house.*

**Sources.** Sweet NEG I §§443, 1016–1017; Whitney §§137–138, 379; UD `docs/_en/dep/case.md`, `dep/nmod-poss.md`; Jespersen MEG VI 17.1–17.5; Kruisinga II.2 §§825, 831.

**Rules.** `en.morph.possessive-after-head` (error): a POS token comes after the head it attaches to.

### noun-genitive-s
`ai/en/seeds/morph/noun-genitive-s.md`

**Gist.** English nouns have two cases: common (*man, men*) and possessive (*man's, men's*). The possessive is written *'s* in the singular and in plurals without *-s* (*the boy's, the children's*), and as a bare apostrophe after plural *-s* (*the boys'*). In PTB and UD tokenization, *'s* and *'* are separate tokens.

**Conditions and exceptions.** Sibilant-final nouns, especially names, have both forms (*James's/James'*, *Socrates' wisdom*, *for conscience' sake*). Possessive pronouns have no apostrophe (*its, hers, yours, ours, theirs*), but *one's* does. *'s* after a noun may also be contracted *is/has* (*John's here*); then it is a verb (VBZ, lemma *be/have*, see `verb-clitics`). The possessive can stand without a following noun (*at my uncle's*, *a friend of John's*), and it attaches to whole groups (see `noun-genitive-group`). In UD *'s* / *'* (also *’s*, *’*) is PART, POS, lemma *'s*, `case`.

**Examples.** *the **boy's** bike; the **boys'** bikes; the **children's** toys; **James's** car.*

**Sources.** Sweet NEG I §§78, 111, 998, 1022; Whitney §§133–135, 142; Santorini 1990 §2; UD `docs/_en/dep/case.md`, `dep/nmod-poss.md`; Jespersen MEG VI 16.1, 16.8; Kruisinga II.2 §§751, 826–829; Mätzner I p. 243.

**Rules.** `en.morph.possessive-token` (error): a POS token is PART with lemma *'s* and relation `case`.

### noun-number-tags
`ai/en/seeds/morph/noun-number-tags.md`

**Gist.** PTB tags encode noun number: NN and NNP are singular, NNS and NNPS plural. The UD `Number` feature must agree with the tag. Number is decided by **agreement** with the verb and determiner, not by form alone.

**Conditions and exceptions.** Plurals without *-s* (*men, children, sheep*) are NNS, `Plur`. Nouns in *-s* with singular agreement (*news, linguistics*) are NN, `Sing`. Singular collectives (*the committee has voted*) are NN; *police, people, cattle* agree as plurals (NNS). Plural-only nouns (*clothes, scissors*) are NNS with `Number=Ptan`, which occurs only with NNS/NNPS. Measures (*10 minutes is not enough*) remain NNS, `Plur`.

**Examples.** *apple* NN — *apples* NNS; *London* NNP; *the Alps* NNPS; *sheep* NN or NNS depending on context.

**Sources.** UD `docs/_en/feat/Number.md`, `docs/_en/pos/NOUN.md`, `pos/PROPN.md`; Santorini 1990 §4.1; Jespersen MEG II 2.1; Kruisinga II.3 §§2136–2141.

**Rules.** `en.morph.nns-number-plur` (error): NNS/NNPS → `Number=Plur|Ptan`. `en.morph.nn-number-sing` (error): NN/NNP nouns → `Number=Sing`. `en.morph.ptan-plural-tag` (error): `Number=Ptan` → NNS/NNPS.

### noun-plural-compounds
`ai/en/seeds/morph/noun-plural-compounds.md`

**Gist.** In a compound noun the plural usually goes on the head word. If the head comes first and is followed by a prepositional phrase or adverb, the *-s* is inside: *sons-in-law, passers-by, hangers-on, commanders-in-chief*. If there is no head noun (the word names a bearer of a property), the *-s* goes at the end: *forget-me-nots, go-betweens, grown-ups, runaways*.

**Conditions and exceptions.** Noun + postposed adjective: formal *courts-martial, attorneys general*, colloquial *court-martials*. *Handful, spoonful, mouthful* are no longer felt as compounds (*handfuls*). Compounds in *-man* change the vowel (*Englishmen*). Colloquially the *-s* may move to the end (*son-in-laws*). A hyphenated compound is usually one token in EWT: NOUN, NNS, `Number=Plur`, with the singular compound as lemma (*passers-by → passer-by*).

**Examples.** *my **sisters-in-law**; the **passers-by**; the **runners-up**; two **forget-me-nots**.*

**Sources.** Sweet NEG I §§440–442, 1018–1019; Whitney §130; Jespersen MEG II 2.3, VI 17.8; Kruisinga II.2 §§763–773.

**Rules.** `en.morph.compound-plural-lemma` (warn): listed compound plurals (*mothers-in-law … courts-martial*) are NNS, `Plur`, with the singular compound as lemma.

### noun-plural-foreign
`ai/en/seeds/morph/noun-plural-foreign.md`

**Gist.** Latin and Greek loans often keep their native plural: *criterion → criteria, phenomenon → phenomena, analysis → analyses, stimulus → stimuli, formula → formulae, curriculum → curricula, appendix → appendices*. The general trend is towards regular *-s* (*formulas, forums, stadiums*). Sometimes the two plurals differ in meaning (*indexes/indices*, *geniuses/genii*).

**Conditions and exceptions.** Types: *-on → -a*; *-is → -es*; *-us → -i*; *-um → -a*; *-a → -ae*; *-ex/-ix → -ices*; *-eau → -eaux*; *-im* (*cherubim*). *Agenda* is now an ordinary singular, and *data* and *media* are often singular mass nouns; EWT annotates them by agreement, mostly NN `Sing` with lemma *data/media*. *Series, species* are the same in both numbers. Singular use of *criteria* or *phenomena* is a text error; the annotation then follows agreement, which is why the rules here are `warn`. In UD these plurals are NOUN, NNS, `Plur`, with the foreign singular as lemma.

**Examples.** *two **criteria**; the **analyses** show; **stimuli** of this kind; the **appendices**.*

**Sources.** Sweet NEG I §§1007–1015; Whitney §126; Santorini 1990 §4.2; Jespersen MEG II 2.6; Kruisinga II.2 §§775–782; Mätzner I pp. 223–233.

**Rules.** `en.morph.foreign-plural-number` (warn): listed foreign plurals are NNS, `Plur`. `en.morph.foreign-plural-lemma` (warn): their lemma is the foreign singular (*criteria → criterion*, *appendices → appendix* …).

### noun-plural-f-ves
`ai/en/seeds/morph/noun-plural-f-ves.md`

**Gist.** Some nouns in *-f/-fe* voice the consonant in the plural, spelled *-ves*: *wife → wives, life → lives, knife → knives, thief → thieves, leaf → leaves, loaf → loaves, half → halves, calf → calves, elf → elves, self → selves, shelf → shelves, wolf → wolves, sheaf → sheaves*. Other nouns in *-f* take plain *-s* (*roofs, chiefs, beliefs, proofs, cliffs, safes*).

**Conditions and exceptions.** Some nouns vary (*scarfs/scarves, hoofs/hooves, dwarfs/dwarves, wharfs/wharves*), and *staff* has both *staffs* and *staves*. Watch for verb homographs: *lives* (noun *life* / verb *live*), *leaves* (noun *leaf*, noun *leave* "time off", verb *leave*), *halves, shelves, calves* (verbs *halve, shelve, calve*); part of speech decides the lemma. The possessive does not voice (*wife's*). Some *-th* nouns voice only in speech (*paths, mouths*; *houses*).

**Examples.** *three **knives**; they saved many **lives*** (NOUN, lemma *life*); *she **lives** here* (VERB, lemma *live*); *autumn **leaves*** (lemma *leaf*).

**Sources.** Sweet NEG I §§999, 1001, 1290; Whitney §124a–b; Jespersen MEG VI 16.2–16.4; Kruisinga II.2 §§755–758; Mätzner I pp. 223–224.

**Rules.** `en.morph.ves-plural-lemma` (error): noun *knives, wives, wolves, halves …* → lemma in *-f/-fe*. `en.morph.lives-noun-lemma` (error): noun *lives* → *life*. `en.morph.lives-leaves-verb-lemma` (error): verb *lives/leaves* → *live/leave*. `en.morph.leaves-noun-lemma` (warn): noun *leaves* → *leaf* (or *leave*).

### noun-pluralia-tantum
`ai/en/seeds/morph/noun-pluralia-tantum.md`

**Gist.** Some nouns exist only in the plural, or their singular means something else. They are paired objects (*scissors, trousers, jeans, pants, pyjamas, tongs, pliers, binoculars, glasses* "spectacles") and collective or mass meanings (*clothes, goods, riches, outskirts, surroundings, whereabouts, belongings, proceedings, earnings, savings, remains, headquarters, barracks*). They take plural agreement (*my trousers are*) and are counted with *a pair of*.

**Conditions and exceptions.** Paired objects never lose *-s* as phrase heads, but the singular can appear as a premodifier (*trouser pocket, scissor kick*). Some have a singular with a different meaning (*good/goods, arm/arms, manner/manners, content/contents*). *Riches, alms, eaves* are old singulars in *-s* reanalysed as plurals, and *headquarters, barracks, means, crossroads* take either agreement. The lemma is the plural form itself (*clothes*, not *clothe*), with UD `Number=Ptan` (NOUN, NNS). EWT uses `Ptan` widely (*regards, troops, supplies, contents, grounds, finances, specifics*, decades like *1990s*), but EWT 2.18 treats *congratulations, belongings* as ordinary plurals with singular lemmas.

**Examples.** *Where are my **scissors**? The **clothes** are dry. We sell **goods** online. On the **outskirts** of town.*

**Sources.** UD `docs/_en/feat/Number.md`; Sweet NEG I §998, NEG II §§1979–1980; Whitney §129; Jespersen MEG II 5.73–5.77, VI 16.7; Kruisinga II.2 §§759, 808–819; Mätzner I pp. 233–241.

**Rules.** `en.morph.ptan-lemma` (warn): listed plural-only nouns have `Number=Ptan` and lemma = form.

### noun-plural-mutation-en
`ai/en/seeds/morph/noun-plural-mutation-en.md`

**Gist.** A few old nouns form the plural by changing the root vowel (*man → men, woman → women, foot → feet, tooth → teeth, goose → geese, mouse → mice, louse → lice*). Three keep the old ending *-en*: *ox → oxen, child → children, brother → brethren* ("fellow believers" only; otherwise *brothers*). These lists are closed.

**Conditions and exceptions.** Compounds in *-man/-woman* follow the pattern (*policemen, Englishmen*), but *German, human, talisman* take *-s* because their *-man* is not the word "man". A computer *mouse* has *mice* or *mouses*. In compounds the first element is usually singular (*toothbrush*), but *teeth/feet* do occur as modifiers (*teeth whitening*) and are still NNS. The lemma is the singular. These forms are always plural, so NN on *feet, teeth, men* is an error.

**Examples.** *two **men**; my **feet** hurt; the **children** played; a pair of **oxen**.*

**Sources.** Sweet NEG I §§1002–1004; Whitney §125; UD `docs/_en/feat/Number.md`; Jespersen MEG VI 11.1, 20.2; Kruisinga II.2 §§760–764.

**Rules.** `en.morph.mutation-plural-number` (warn): *men, women, feet, teeth, geese, mice, lice, children, oxen, brethren* are NNS, `Plur`. `en.morph.mutation-plural-lemma` (error): their lemma is the singular (*men → man*, *children → child* …).

### noun-plural-s-spelling
`ai/en/seeds/morph/noun-plural-s-spelling.md`

**Gist.** The regular plural ending *-s* has three pronunciations, depending on the last sound of the stem: [ɪz] after sibilants (*boxes, churches, judges*), [z] after vowels and voiced sounds (*days, dogs*), [s] after voiceless sounds (*cats, cliffs*). In spelling, [ɪz] is written *-es*, and final *-y* after a consonant becomes *-ies*. The possessive and the verb's 3rd person singular share this ending and these rules.

**Conditions and exceptions.** *-es* is written after *s, x, z, ch, sh* (*kisses, boxes, buzzes, matches*), and only *-s* is added after a silent *-e* (*horses, judges*). Consonant + *y* → *-ies* (*cities*), vowel + *y* → *-ys* (*days, valleys*), and proper names keep *y* (*the Kennedys, two Marys*). Frequent old words in *-o* take *-oes* (*potatoes, heroes, echoes*), while words in *-io*, short words and loans take *-os* (*ratios, photos, pianos, zeros*). Letters, digits and cited words often take an apostrophe (*P's and Q's, 9's*); decades are *the 1990s* (NNS, `Number=Ptan` in EWT). *-ves* plurals and irregular plurals have their own seeds. The lemma is the singular with its spelling restored (*cities → city, potatoes → potato, boxes → box*); forms like *citie, potatoe, boxe* are lemmatization errors.

**Examples.** *box → boxes; church → churches; city → cities; day → days; hero → heroes; photo → photos.*

**Sources.** Sweet NEG I §§1000, 1021; Whitney §§123–124, 128; Santorini 1990 §2; UD `docs/_en/feat/Number.md`; Jespersen MEG VI 16.1; Kruisinga I §§574–582; Mätzner I p. 224.

**Rules.** None (descriptive seed).

### noun-plural-zero
`ai/en/seeds/morph/noun-plural-zero.md`

**Gist.** Some nouns have the same form in singular and plural: *one sheep — two sheep*, *a deer — many deer*, *this species — these species*. Their number shows only through agreement with a determiner, numeral or verb, so the tag (NN or NNS) and `Number` are assigned from context. The lemma equals the form.

**Conditions and exceptions.** Animals: *sheep, deer, swine, moose, bison*. Fish: *fish, salmon, trout, cod* (*fishes* is used for different species), and hunting usage has *two brace of pheasants*. Latin nouns in *-es*: *species, series*; also *means, headquarters, crossroads, barracks* (EWT often marks these `Ptan`). Others: *aircraft, spacecraft, craft, offspring*. Measure words after a numeral take no *-s* (*two dozen eggs, five hundred people, a ten-pound note*, but *dozens of eggs*). Sibilant nationality words are invariable (*two Japanese, the Swiss*).

**Examples.** ***These** species are rare* (NNS, `Plur`). ***A** species of bird* (NN, `Sing`). *Two **aircraft** landed* (NNS). *The **sheep** is lost* (NN).

**Sources.** Sweet NEG I §§1004–1015; Whitney §§127, 144c, 214; Santorini 1990 §4.1; UD `docs/_en/feat/Number.md`; Jespersen MEG II 3.1–3.5, VI 19.8; Kruisinga II.2 §§783–794.

**Rules.** `en.morph.zero-plural-with-plural-det` (warn): a zero-plural noun with *these, those, several, many, few, both* is NNS, `Plur`. `en.morph.zero-plural-with-singular-det` (warn): with `det` *a, an, this, that, each, every* it is NN, `Sing`.

### noun-singular-in-s
`ai/en/seeds/morph/noun-singular-in-s.md`

**Gist.** Some nouns end in *-s* but are grammatically singular and agree with *is, has, this*: *news*, sciences and activities in *-ics* (*mathematics, physics, linguistics, economics, politics, athletics*), diseases (*measles, mumps*) and games (*billiards, darts*). Agreement, not the final *-s*, decides number. The lemma is the whole form with *-s*.

**Conditions and exceptions.** *News* is only singular now (*the news is good*), with lemma *news*, not *new*. *Politics, economics, ethics* can be plural meaning "views, practice" (*his politics are…*); EWT marks *politics, economics* as NNS `Ptan` and *mathematics* as NN `Sing`. *Ethics* as the plural of *ethic* (*work ethics*) is an ordinary plural with lemma *ethic*. *Series, species, means* are zero plurals, a different type.

**Examples.** *The **news** is on. **Linguistics** is my favorite subject. **Measles** is contagious.*

**Sources.** UD `docs/_en/feat/Number.md`; Sweet NEG I §§998, 1724; Whitney §129; Santorini 1990 §4.1; Jespersen MEG II 5.76–5.78; Kruisinga II.2 §808, II.3 §§2136–2141.

**Rules.** `en.morph.news-singular` (error): noun *news* is NN, `Sing`, lemma *news*. `en.morph.ics-lemma` (error): *mathematics, physics, … measles, mumps, billiards* keep the *-s* in the lemma.

### num-ordinal-mult
`ai/en/seeds/morph/num-ordinal-mult.md`

**Gist.** Apart from *first, second, third*, ordinals are formed with *-th* (*fourth, fifth, twentieth, hundredth*). In compounds only the last word becomes ordinal (*twenty-first*), and in digits they are written *1st, 2nd, 3rd, 4th*. The multiplicative adverbs are *once, twice* and literary *thrice*; beyond that the language uses *three times*.

**Conditions and exceptions.** Spelling: *fifth, twelfth* (v → f), *eighth*, *ninth*, *twentieth* (y → ie). *Second* is also a time-unit noun (*a second*) without `NumType`. Ordinals can be ADJ (*the third book*), ADV (*first, we…*) or NOUN in dates (*July 3rd*), always with `NumType=Ord` (plus `NumForm=Word` or `Combi`). Fractions (*a half, two thirds*) are mostly NOUN with `NumType=Frac`. *Once* meaning "as soon as" is SCONJ without `NumType`. *Once, twice* are ADV, RB, `NumType=Mult`, and cardinals are NUM, CD, `NumType=Card`.

**Examples.** *the **fifth** chapter; she came **first**; July **4th**; I called **twice**; **once** a week.*

**Sources.** Sweet NEG I §§1170–1176, 1504; Whitney §§215–218; Santorini 1990 §2; UD `docs/_en/feat/NumType.md`; Jespersen MEG VI 18.1, 24.4; Kruisinga II.3 §§1693, 1706–1710; Mätzner I pp. 288–289.

**Rules.** `en.morph.ordinal-form` (warn): `NumType=Ord` only on words in *-th/-st/-nd/-rd* that are ADJ, ADV or NOUN. `en.morph.ordinal-words` (warn): *first, third, fifth …* as ADJ/ADV have `NumType=Ord`. `en.morph.multiplicative-once-twice` (warn): ADV *once/twice/thrice* have `NumType=Mult`. `en.morph.cardinal-numtype` (error): NUM CD has `NumType=Card|Frac`.

### pron-demonstrative
`ai/en/seeds/morph/pron-demonstrative.md`

**Gist.** Demonstratives are the only English determiners that inflect for number: *this → these* (near), *that → those* (far). They stand before a noun (*this book*) or replace a noun phrase (*I like this*). The lemma of the plural is the singular (*these → this, those → that*).

**Conditions and exceptions.** Before a noun they are DET (`det`); standing alone they are PRON. In both cases the PTB tag is DT, with `PronType=Dem` and `Number=Sing/Plur`. *That* has three non-demonstrative uses: relative pronoun (PRON, WDT, `PronType=Rel`, no number), conjunction (SCONJ, IN) and degree adverb (*not that big*: ADV, RB). Only demonstrative *that* has `Number` and `PronType=Dem`. *Here, there* and expletive *there* (EX) are also `PronType=Dem`, without number. Colloquial *them* for *those* (*them apples*) is DET with `Number=Plur|PronType=Dem|Style=Vrnc`.

**Examples.** ***These** books are mine. I prefer **that**. **Those** who wait…*

**Sources.** UD `docs/_en/pos/DET.md`, `docs/_en/pos/PRON.md`, `feat/PronType.md`; Santorini 1990 §§2, 4.1; Sweet NEG I §§125, 177, 1128–1130; Whitney §§150, 166–168; Jespersen MEG II 2.22, 16.3.

**Rules.** `en.morph.dem-plural` (error): *these/those* are DT, `Plur`, `Dem`. `en.morph.dem-these-lemma`, `en.morph.dem-those-lemma` (error): lemmas *this*, *that*. `en.morph.dem-this-sing` (error): *this* is `Sing`, `Dem`. `en.morph.dem-that-det` (error): DET *that* is `Sing`, `Dem`.

### pron-indefinite
`ai/en/seeds/morph/pron-indefinite.md`

**Gist.** Compound pronouns combine *some-, any-, every-, no-* with *-one, -body* (person) or *-thing* (thing). They are written as one word (*someone, nothing*), except *no one*, and are always singular (*everyone is*, *nothing has changed*).

**Conditions and exceptions.** PTB tags them NN, but UD makes them PRON because they take no articles or premodifiers. *None* is a separate negative pronoun (`PronType=Neg`) without `Number`, since it agrees either way (*none of them is/are*). *No one* is two words: *no* is DET and *one* is PRON with `PronType=Neg`. The matching adverbs (*somewhere, anywhere, nowhere, everywhere*) are ADV with `PronType`. The lemma is the whole form. Features: `Number=Sing` (except *none*); `PronType=Ind` for *some-/any-*, `Tot` for *every-*, `Neg` for *no-* and *none*.

**Examples.** ***Someone** called. I didn't see **anything**. **Everybody** knows. **Nothing** happened. **None** of them came.*

**Sources.** UD `docs/_en/pos/PRON.md`, `feat/PronType.md`, `pos/ADV.md`; Santorini 1990 §4.1; Sweet NEG I §§1136, 1148–1157; Jespersen MEG II 16.6, 17.2–17.3; Kruisinga II.2 §§1339–1356.

**Rules.** `en.morph.indef-pron-upos` (error): these words tagged NN are PRON. `en.morph.indef-some-any` (error): `PronType=Ind`, `Sing`. `en.morph.indef-every` (error): `PronType=Tot`, `Sing`. `en.morph.indef-no` (error): *nobody, nothing, none* have `PronType=Neg`. `en.morph.indef-lemma` (error): lemma = whole form.

### pron-personal-case
`ai/en/seeds/morph/pron-personal-case.md`

**Gist.** Personal pronouns keep two cases: nominative *I, we, he, she, they* and accusative (objective) *me, us, him, her, them*; *you* and *it* do not change form. In UD, case is assigned **by form**, not by syntactic role. The lemma of an accusative form is the nominative.

**Conditions and exceptions.** Accusative forms in subject position or after *be* (*It's me*, *Me and him went*) stay `Case=Acc`, and nominative forms in object position (*between you and I*) stay `Case=Nom`. Only for *you* and *it* is case decided by position: subject → `Nom`, object or after a preposition → `Acc`. *Her* is either accusative (PRP) or possessive (PRP$, see `pron-possessive`). Archaic *thou/thee, ye* and colloquial *'em* follow the same pattern, and *'s* in *let's* is *us* (lemma *we*, `Acc`). UD: PRON, PRP, `PronType=Prs`.

**Examples.** ***She** saw **him**. **They** told **us**. It's **me**. Let**'s** go.*

**Sources.** UD `docs/_en/feat/Case.md`, `docs/_en/pos/PRON.md`; Santorini 1990 §2; Sweet NEG I §§141, 1084–1087; Whitney §§155, 158; Kruisinga II.2 §§964–984; Mätzner I pp. 293–295.

**Rules.** `en.morph.pron-nominative` (error): *I, we, he, she, they, thou, ye* are PRP, `Nom`, `Prs`. `en.morph.pron-accusative` (error): *me, us, him, them, thee, 'em* are PRP, `Acc`, `Prs`. `en.morph.pron-accusative-lemma` (error): *me/us/him/them* → *I/we/he/they*. `en.morph.pron-her-accusative` (error): PRP *her* → lemma *she*, `Acc`.

### pron-personal-paradigm
`ai/en/seeds/morph/pron-personal-paradigm.md`

**Gist.** Every personal pronoun form fixes person, and most also fix number. Gender is distinguished **only** in the 3rd person singular (*he, she, it* and their forms). Nouns have no gender and get no `Gender` feature in UD.

**Conditions and exceptions.** *You, your, yours* are 2nd person without number; only the reflexives show it (*yourself* singular, *yourselves* plural). *They, them, their* are always `Number=Plur`, even as singular *they*, because number is grammatical (*they are*). Generic *one* is 3rd singular without gender. Dummy *it* (*it rains*) is still `PronType=Prs`. UD: PRON, PRP or PRP$, `PronType=Prs` (emphatic reflexives `Emp`).

**Examples.** *I/me/my/mine/myself* — `Person=1|Number=Sing`; *we/us/our/ours/ourselves* — `Person=1|Number=Plur`; *he/him/his/himself* — `Person=3|Number=Sing|Gender=Masc`; *it/its/itself* — `Gender=Neut`; *they/them/their/theirs/themselves* — `Person=3|Number=Plur`.

**Sources.** UD `docs/_en/pos/PRON.md`, `feat/Person.md`, `feat/Number.md`, `feat/Gender.md`; Sweet NEG I §§1076–1088, 1101; Whitney §§155, 158, 160; Jespersen MEG II 2.23.

**Rules.** `en.morph.pron-1sg`, `en.morph.pron-1pl`, `en.morph.pron-2`, `en.morph.pron-3sg-masc`, `en.morph.pron-3sg-fem`, `en.morph.pron-3sg-neut`, `en.morph.pron-3pl` (error): person, number and gender for each form set above. `en.morph.gender-only-3sg` (error): any token with `Gender` is a 3rd-person singular PRON.

### pron-possessive
`ai/en/seeds/morph/pron-possessive.md`

**Gist.** Possessive pronouns have a dependent form before a noun (*my book, your car, her idea*) and an independent form that replaces a noun phrase (*the book is mine, yours is better*). The independent form adds *-s* (*yours, hers, ours, theirs*) or *-n* (*mine*, archaic *thine*). *His* and *its* have a single form for both roles.

**Conditions and exceptions.** *Her* is possessive (PRP$) before a noun and accusative (PRP) otherwise. *His* can be dependent (PRP$) or independent (PRP, without `Case`). *Its* is not *it's* (which is tokenized *it* + *'s*). Independent forms have no apostrophe (*yours*, not *your's*), and *whose* is covered in `pron-wh`. In UD the dependent form is PRP$ with `Case=Gen|Poss=Yes|PronType=Prs` plus person, number and gender, lemma = form, attached as `nmod:poss`. The independent form is PRP with `Poss=Yes|PronType=Prs` and **no** `Case`; its lemma is the dependent form (*mine → my, yours → your, hers → her, ours → our, theirs → their*).

**Examples.** ***My** car is older than **yours**. The idea was **hers**. The cat licked **its** paw.*

**Sources.** UD `docs/_en/feat/Poss.md`, `feat/Case.md`, `docs/_en/pos/PRON.md`; Santorini 1990 §§2, 4.1; Sweet NEG I §§1022, 1102–1103; Jespersen MEG II 16.21, VI 16.1, 20.3; Whitney §§165, 205–207; Mätzner I pp. 296–297.

**Rules.** `en.morph.prp-dollar-feats` (error): PRP$ is PRON with `Case=Gen`, `Poss=Yes`, `PronType=Prs`. `en.morph.poss-independent` (error): *mine, yours, hers, ours, theirs* are PRP, `Poss=Yes`, no `Case`, with the dependent form as lemma. `en.morph.poss-her-dependent` (error): PRP$ *her* has lemma *her*, `Gen`, `Poss=Yes`.

### pron-reflexive
`ai/en/seeds/morph/pron-reflexive.md`

**Gist.** Pronouns in *-self* (singular) and *-selves* (plural) are built on the possessive in the 1st and 2nd person (*myself, yourself, ourselves*) and on the accusative in the 3rd (*himself, themselves*; *herself, itself* are ambiguous). They have two uses: reflexive, as an object repeating the subject (*She hurt herself*), and emphatic (*She herself said so*).

**Conditions and exceptions.** Generic *oneself* is built on *one*. Dialectal *hisself, theirselves* are built on the possessive, and *themself* is used for singular *they*. Number follows the ending: *-self* is singular, *-selves* plural (*yourself* vs *yourselves*). UD: PRON, PRP, lemma = form, `Case=Acc|Reflex=Yes` plus person, number and gender; `PronType=Prs` when reflexive, `Emp` when emphatic. The UD guidelines give emphatic uses no `Case`/`Reflex`, but EWT 2.18 marks both uses, so the feature rule follows the data and is only `warn`.

**Examples.** *I cut **myself**. They enjoyed **themselves**. The president **himself** called.*

**Sources.** UD `docs/_en/feat/Reflex.md`, `feat/PronType.md`, `feat/Case.md`, `docs/_en/pos/PRON.md`; Santorini 1990 §2; Sweet NEG I §§1105, 1109–1114; Whitney §164; Kruisinga II.2 §§1024–1030.

**Rules.** `en.morph.reflexive-feats` (warn): PRON in *-self/-selves* is PRP with `Reflex=Yes`, `Case=Acc`, `PronType=Prs|Emp`. `en.morph.reflexive-number` (error): *-selves* is `Plur`. `en.morph.reflexive-number-sing` (error): *-self* is `Sing`. `en.morph.reflexive-lemma` (error): lemma = form.

### pron-wh
`ai/en/seeds/morph/pron-wh.md`

**Gist.** Interrogative and relative pronouns are the same words in two roles: in questions (*Who called?*) and at the start of relative clauses (*the man who called*). Only *who* inflects: *who* (nominative), *whom* (accusative), *whose* (possessive). *Which, what* and relative *that* are invariable.

**Conditions and exceptions.** In speech *whom* gives way to *who* even as an object (*Who did you see?*); the form *who* keeps lemma *who*, and its case follows position. *Whose* is used for people and things (*a house whose roof leaks*) and must not be confused with *who's*. *Which, what, whatever* are DET before a noun and PRON alone. *-ever* compounds behave like the base word (*whomever* → lemma *whoever*). Relative *that* is PRON, WDT, `PronType=Rel`. Tags: *who, whom, what* are WP; *whose* is WP$; *which, that, whatever* are WDT. `PronType=Int` in questions, `Rel` in relative clauses, and *whose* has `Poss=Yes`. The UD guideline gives *whom* lemma *who* with `Case=Acc`, but EWT 2.18 still has lemma *whom* without `Case` in some cases, so that rule is `warn`.

**Examples.** ***Whom** did you invite? The author **whose** book I read. **Which** do you want? The car **that** I bought.*

**Sources.** UD `docs/_en/pos/PRON.md`, `feat/PronType.md`, `pos/DET.md`; Santorini 1990 §§2, 4.1; Sweet NEG I §§211, 216, 1086; Whitney §§169–187; Kruisinga II.2 §§1045–1047, 1079, 1094; Jespersen MEG VI 16.1.

**Rules.** `en.morph.whose-poss` (error): *whose* is PRON, WP$, `Poss=Yes`, `PronType=Int|Rel`. `en.morph.wh-prontype` (error): WP/WP$/WDT tokens have `PronType=Int|Rel`. `en.morph.whom-acc` (warn): *whom* has lemma *who*, `Case=Acc`.

### spell-consonant-doubling
`ai/en/seeds/morph/spell-consonant-doubling.md`

**Gist.** Before vowel-initial endings (*-ed, -ing, -er, -est*, also *-y, -ish*), a final consonant doubles if it follows a single short **stressed** vowel: *stop → stopped, run → running, big → bigger, prefer → preferred, begin → beginning*. The doubling only marks the short vowel in spelling; the lemma has a single consonant (*stopped → stop*).

**Conditions and exceptions.** There is no doubling after an unstressed final syllable (*visited, offered, opened, developed, galloping, benefited*; *benefitted* also occurs), after two vowels or a long vowel (*rained, beating, cooler*), or with *w, x, y* (*snowed, fixed, playing*). British English doubles *l* even after an unstressed vowel (*travelled, cancelled, crueller*, but *paralleled*), while American English does not (*traveled*). A few words double in the US too (*kidnapped, worshipped, handicapped*). *-c* becomes *-ck-* before *-ed, -ing, -y* (*panicked, picnicking, trafficked, panicky*). Final *s* varies (*gases*; *buses/busses*; *focused/focussed*). A lemmatizer must remove **only one** letter, and only when the stem itself does not end in a double consonant (*missed → miss, added → add*).

**Examples.** *stopped → stop; running → run; bigger → big; preferred → prefer; travelled → travel; panicked → panic.*

**Sources.** Jespersen MEG VI 4.2, 16.1, 21.9; Kruisinga I §§572, 574, 581–583; Mätzner I pp. 273, 337.

**Rules.** `en.morph.doubling-verb-lemma` (error): listed doubled verb forms (*stopped, running, preferred, travelled, panicked …*) have the undoubled lemma. `en.morph.doubling-adj-lemma` (error): *bigger, hottest, sadder, thinnest …* have lemma *big, hot, sad, thin …*.

### spell-silent-e
`ai/en/seeds/morph/spell-silent-e.md`

**Gist.** Final silent *-e* (which marks a long vowel, as in *make, hope, nice*) drops before vowel-initial endings (*making, hoped, hoping, nicer, lovable*) and stays before consonant-initial ones (*hopeful, lovely, movement*). For lemmatization this means *hoping → hope* (with *-e*) but *hopping → hop* (no *-e*, doubled consonant).

**Conditions and exceptions.** *-ee, -ye, -oe* keep the *e* before *-ing* (*seeing, agreeing, dyeing* "colouring" vs *dying*, *hoeing, canoeing, eyeing*), as do *singeing* (vs *singing*), *swingeing* and *ageing/aging*. *-ie* becomes *-y-* before *-ing* (*lying, dying, tying, vying*). *-ce, -ge* keep *e* before *a, o* (*noticeable, manageable, courageous*). Stems in *-e* add only *-d* (*loved, agreed*). Adverbs: *truly, duly, wholly* but *solely, vilely*. *-able* varies (*likeable/likable*). The UD lemma is the stem with *-e* restored (*writing → write, used → use, larger → large, lying → lie*).

**Examples.** *making → make; hoping → hope; hopping → hop; nicer → nice; lying → lie; dyeing → dye; dying → die.*

**Sources.** Jespersen MEG VI 4.2, 21.9, 22.6, 22.8; Kruisinga I §§567–569, 573, 575; Mätzner I p. 337.

**Rules.** `en.morph.silent-e-verb-lemma` (error): *making, having, coming, writing, used, hoping …* have the lemma with *-e*. `en.morph.ie-ying-lemma` (error): *lying, dying, tying, vying, untying* → *lie, die, tie, vie, untie*.

### spell-y-to-i
`ai/en/seeds/morph/spell-y-to-i.md`

**Gist.** Final *-y* after a consonant is written *-i-* before *-s, -ed, -er, -est, -ly, -ness*: *try → tries, tried; city → cities; happy → happier, happiest, happily, happiness*. Before *-ing*, *y* stays (*trying, studying*), and after a vowel *y* does not change (*plays, played, days, gayer*). For lemmatization, *-ies, -ied, -ier, -iest, -ily* go back to *-y* (*studies → study*, not *studie*).

**Conditions and exceptions.** Vowel + *y*: *plays, keys, valleys, boys*, with the old exceptions *laid, paid, said*, *daily, gaily* (also *gayly*); *staid* is now an adjective. Proper names keep *y* (*the Kennedys, two Marys*). Monosyllabic adjectives vary (*drier/dryer, shyer, slyer*; *drily/dryly, shyly*). *-ey*: *monkeys, valleys, journeys*; *money → moneys/monies*. As adjectives, *married, hurried* have their own lemma, while as verbs they lemmatize to *marry, hurry*. EWT treats *supplies* "provisions" as plurale tantum (lemma *supplies*), so it is left out of the noun rule. The adverb *happily* keeps its own lemma.

**Examples.** *tried → try; studies → study; cities → city; carried → carry; happier → happy; happily → happily* (the adverb has its own lemma).

**Sources.** Jespersen MEG VI 4.2, 16.1, 22.8; Kruisinga I §§567, 572, 574; Mätzner I pp. 224, 273, 337; Sweet NEG I §1021.

**Rules.** `en.morph.y-to-i-verb-lemma` (error): verb forms *tried, studies, carried, applied …* → lemma in *-y*. `en.morph.y-to-i-noun-lemma` (error): nouns *cities, countries, companies …* → lemma in *-y*. `en.morph.y-to-i-adj-lemma` (error): JJR/JJS *happier, easiest, earlier …* → lemma in *-y*.

### verb-archaic-endings
`ai/en/seeds/morph/verb-archaic-endings.md`

**Gist.** Older English had two more personal endings: *-th/-eth* for the 3rd person singular (*he goeth, hath, doth, saith*) and *-st/-est* for the 2nd person singular with *thou* (*thou goest, hast, art, canst, wilt*). Today they survive in the Bible, prayers, proverbs and solemn or stylized text. For annotation, *-eth* equals modern *-s* (VBZ), and *-st* forms are 2nd person singular.

**Conditions and exceptions.** *Hath, doth, saith* lasted into the mid-18th century and were revived in solemn style, which also split *doth* (auxiliary) from *doeth* (main verb). 2nd person: *-est* contracts except after sibilants; *-t* forms are *art, shalt, wilt*; *-st* forms are *canst, mayst, darest, hast, dost, didst, hadst, wouldst, shouldst, couldst*; *wast* is indicative and *wert* subjunctive or poetic; *thou must* has no ending. Lemmas: *hath, hast → have*; *doth, dost, doeth, didst → do*; *art, wast, wert → be*; *saith → say*; *shalt → shall*; *wilt → will*; *canst → can*. *Wilt* is also a modern verb ("to droop"): as MD its lemma is *will*, as VERB it is *wilt*. Early Modern English also used *-s/-th* with plural subjects (*they hath*). In UD these forms may carry `Style=Arch`. The *-eth* forms are VBZ `Person=3|Number=Sing|Tense=Pres`, and *thou* forms are `Person=2|Number=Sing`. Archaic modals are MD with the modern lemma.

**Examples.** *He **hath** spoken* (VBZ, *have*). *Thou **art** mine* (VBP, *be*, `Person=2|Number=Sing`). *Thou **shalt** not kill* (MD, *shall*). *The Lord **giveth*** (VBZ, *give*).

**Sources.** Jespersen MEG VI 2.3–2.7, 3.6–3.7, II 2.24; Sweet NEG I §§1283, 1479–1490; Whitney §§243, 256, 273, 277–278.

**Rules.** `en.morph.archaic-eth-vbz` (warn): a verb ending in *-eth* is VBZ, 3rd singular, present. `en.morph.archaic-verb-lemma` (error): *hath, hast, hadst, doth, dost, doeth, didst, saith, art, wast, wert* → *have, do, say, be*. `en.morph.archaic-modal-lemma` (error): MD *shalt, wilt, canst, couldst, wouldst, shouldst, mayst, mightst* → modern modal lemma.

### verb-be-forms
`ai/en/seeds/morph/verb-be-forms.md`

**Gist.** *Be* is the most irregular English verb, with eight forms from different roots: present *am* (1sg), *is* (3sg), *are* (others); past *was* (1sg, 3sg), *were* (others and unreal conditions); *be* (infinitive, subjunctive); participles *been*, *being*. It is the only verb that distinguishes person or number in both present and past. The lemma is always *be*.

**Conditions and exceptions.** *Be* can be an auxiliary (AUX: *is going, was taken*), a copula (AUX, `cop`: *she is happy*) or, rarely in EWT, a main verb "exist, be located" (VERB: *there are…*). *Were* with a singular subject (*if I were*) is subjunctive: VBD, `Mood=Sub`. The contractions *'m, 're, 's* are *am, are, is* (see `verb-clitics`), and colloquial *ain't* splits into *ai* (lemma *be*) + *n't*. Non-standard *you was, they was, he were* are annotated by form. Tags: *am* VBP `Person=1|Number=Sing`; *is* VBZ; *are* VBP; *was* VBD `Number=Sing`; *were* VBD; *been* VBN; *being* VBG; *be* VB.

**Examples.** *I **am** here. She **is** ready. We **were** late. It has **been** done. You are **being** silly.*

**Sources.** UD `docs/_en/feat/Person.md`, `feat/Number.md`, `feat/Mood.md`, `docs/_en/pos/AUX_.md`; Santorini 1990 §§2, 4.1; Sweet NEG I §§1490–1491; Jespersen MEG VI 5.6; Kruisinga II.1 §§25, 29; Whitney §273.

**Rules.** `en.morph.be-am`, `en.morph.be-is`, `en.morph.be-are`, `en.morph.be-was`, `en.morph.be-were`, `en.morph.be-been`, `en.morph.be-being` (all error): each form of *be* (AUX or VERB) has lemma *be* and the tag listed above (*am* also `Person=1|Number=Sing`; *was* also `Number=Sing`).

### verb-clitics
`ai/en/seeds/morph/verb-clitics.md`

**Gist.** In informal language, auxiliaries and *not* contract onto the preceding word: *it's, we're, I'm, they've, she'd, you'll, isn't*. PTB and UD tokenization split the contraction off as a separate token, which gets the same lemma and features as the full form (*'re* = *are*, *n't* = *not*).

**Conditions and exceptions.** *'s* can be *is* (*it's cold*) or *has* (*it's been cold*), both VBZ with lemma *be* or *have*. It can also be the possessive (POS, see `noun-genitive-s`) or *us* in *let's* (PRON, lemma *we*). *'d* is *had* (VBD, *have*) before a participle (*I'd gone*) or *would* (MD) before a base form (*I'd go*). *'ll* is *will* (MD). *n't* splits off together with the *n* (*is|n't, do|n't*). Some stems change, and the remnant gets the full lemma: *ca|n't* (*can*), *wo|n't* (*will*), *sha|n't* (*shall*), *ai|n't* (*be* or *have*). *Cannot* splits into *can* + *not*. Colloquial fusions split too: *gonna → gon* (VBG, *go*) + *na* (TO, *to*), *wanna → wan* (*want*) + *na*, both parts with `Abbr=Yes`. In UD *n't* is PART, RB, lemma *not*, `Polarity=Neg`; *'m* is *be* VBP `Person=1|Number=Sing`; *'re* is *be* VBP; *'ve* is *have*; *'ll* and *wo* are *will* MD; *ca* is *can* MD.

**Examples.** *It**'s** fine. We**'re** late. I**'d** go. You**'ll** see. It is**n't**. I ca**n't**. I'm gon**na** try.*

**Sources.** UD `docs/_en/tokenization.md`, `docs/_en/pos/PRON.md`, `feat/Polarity.md`; Santorini 1990 §2; Sweet NEG I §§78, 1478–1493; Jespersen MEG V 23.1–23.2; Kruisinga II.1 §§20–25.

**Rules.** `en.morph.clitic-nt` (error): *n't* is PART, RB, lemma *not*, `Polarity=Neg`. `en.morph.clitic-s-verb` (error): verbal *'s* is VBZ, lemma *be|have*. `en.morph.clitic-s-us` (error): pronoun *'s* has lemma *we*, `Acc`, 1st plural. `en.morph.clitic-am` (error): *'m* is *be* VBP 1sg. `en.morph.clitic-re` (error): *'re* is *be* VBP. `en.morph.clitic-ve` (error): *'ve* is *have*. `en.morph.clitic-will` (error): MD *'ll/wo* is *will*. `en.morph.clitic-can` (error): MD *ca* is *can*. `en.morph.clitic-d` (error): MD *'d* is *would*. `en.morph.clitic-d-had` (error): VBD *'d* is *have*. `en.morph.clitic-gonna` (error): *gon* is VBG, lemma *go*.

### verb-ed-regular
`ai/en/seeds/morph/verb-ed-regular.md`

**Gist.** Regular verbs form both the past tense and the past participle with *-ed* (*call → called, want → wanted*). This is the only productive pattern: all new verbs follow it (*googled, texted*). The ending is pronounced [ɪd] after *t, d*, [d] after voiced sounds and [t] after voiceless sounds.

**Conditions and exceptions.** Spelling: stems ending in *-e/-ee* add only *-d* (*loved, agreed*); consonant + *y* → *-ied* (*tried*), vowel + *y* → *-yed* (*played*; exceptions *laid, paid, said*). A final consonant doubles after a short stressed vowel (*stopped, preferred*, but *visited, offered*), British English always doubles *l* (*travelled*; American *traveled*), and *-c* becomes *-cked* (*panicked*); see the `spell-*` seeds. Forms in *-t* exist beside *-ed* (more British): *burnt, learnt, dwelt, smelt, spelt, spilt, spoilt, dealt, dreamt, knelt, leant, leapt*, with the plain stem as lemma (*learnt → learn*). The old [ɪd] pronunciation survives in adjectives (*a learned professor, beloved, aged, blessed*), which are JJ with their own lemma, while the verb forms are VBD/VBN with lemma *learn*. *Past* is not *passed*. In UD: VBD (`Tense=Past|VerbForm=Fin|Mood=Ind`) or VBN (`Tense=Past|VerbForm=Part`), with the lemma spelled correctly (*stopped → stop, tried → try, loved → love, panicked → panic*).

**Examples.** *She **walked** home. It was **stopped**. We **tried**. I **learnt** it* (VBD, lemma *learn*).

**Sources.** Jespersen MEG VI 4.2–4.3, 4.5; Sweet NEG I §§1286–1287, 1295–1333; Whitney §§244, 246–249.

**Rules.** `en.morph.t-past-lemma` (error): VBD/VBN *burnt, learnt, dwelt … blest* → lemma *burn, learn, dwell … bless*.

### verb-have-do-forms
`ai/en/seeds/morph/verb-have-do-forms.md`

**Gist.** *Have* and *do* are irregular verbs with special forms. *Have*: *has*, past and participle *had*. *Do*: *does* (regular spelling, different vowel), past *did*, participle *done*. Both can be auxiliaries (*have gone*, *do you know*) or main verbs (*have a car*, *do homework*).

**Conditions and exceptions.** *Had* is VBD (*he had a car*, *she had left*) or VBN (*I have had enough*); syntax decides. *Did* is only VBD and *done* only VBN (in non-standard *he done it*, EWT tags by function). *Does* as the plural of *doe* is a noun and is not covered (only VERB/AUX are checked). Contractions: *'ve → have*, *'s → has*, *'d → had*. The lemma is *have* or *do*; AUX for auxiliary use, VERB for main use.

**Examples.** *She **has** left. We **had** fun. **Does** it work? I **did** it. It's **done**.*

**Sources.** UD `docs/_en/pos/VERB.md`, `docs/_en/pos/AUX_.md`, `feat/Number.md`; Santorini 1990 §§2, 4.1; Sweet NEG I §§1492–1493; Jespersen MEG VI 3.1, 4.1; Kruisinga II.1 §19.

**Rules.** `en.morph.have-has` (error): *has* → *have*, VBZ. `en.morph.have-had` (error): *had* → *have*, VBD/VBN. `en.morph.do-does` (error): *does* → *do*, VBZ. `en.morph.do-did` (error): *did* → *do*, VBD. `en.morph.do-done` (warn): *done* → *do*, VBN.

### verb-homograph-past
`ai/en/seeds/morph/verb-homograph-past.md`

**Gist.** Some irregular forms coincide with the base of **another** verb or with a noun: *found* (past of *find*; base of *found* "establish"), *saw* (*see*; *saw* "cut, a saw"), *lay* (*lie* "recline"; base of *lay* "put"). A lemmatizer that looks only at the form easily picks the wrong lemma. The tag disambiguates: as VBD/VBN the form belongs to the strong verb, as VB/VBP/NN to the homograph.

**Conditions and exceptions.** Pairs: *found* (find / found → founded), *saw* (see / saw → sawed), *fell* (fall / fell → felled), *lay* (lie / lay → laid), *rose* (rise / the flower), *bore* (bear / bore → bored), *wound* (wind / wound → wounded), *ground* (grind / ground → grounded), *bound* (bind / bound → bounded), *left* (leave / the direction), *felt* (feel / the fabric), *lit* (light), *spoke* (speak / a spoke), *stole* (steal / a stole), *tore, wore, dove*. The regular homograph verbs form their past in *-ed*, so *found* tagged VBD is always from *find*, and *wound* VBD always from *wind*. *Lie/lay* confusion is old and growing; annotation follows the form, so VBD *lay* has lemma *lie* even if "put" was meant. *Left, bound, found* as adjectives (*the left hand, bound to happen*) are ADJ with their own lemma.

**Examples.** *I **found** it* (VBD, lemma *find*). *They **found** a company* (VBP, lemma *found*). *He **lay** down* (VBD, lemma *lie*). *We **saw** the film* (VBD, lemma *see*).

**Sources.** Jespersen MEG VI 5.1, 5.3–5.4; Sweet NEG I §§1364, 1405, 1408; Whitney §§96, 262, 264.

**Rules.** `en.morph.homograph-past-lemma` (error): VBD/VBN *found, saw, fell, rose, bore, wound, ground, bound, left, felt, lit, spoke, stole, tore, wore, dove* → the strong verb's lemma. `en.morph.lay-past-of-lie` (error): VBD *lay* → *lie*.

### verb-invariable
`ai/en/seeds/morph/verb-invariable.md`

**Gist.** A group of verbs in *-t/-d* has one form for base, past and participle: *put, cut, set, hit, let, shut, cost, hurt, burst, cast, spread, split, quit, bet, rid, shed, thrust, wet*. Only context (subject, auxiliaries, tense of neighbouring verbs) decides between VB, VBP, VBD and VBN. The lemma always equals the form.

**Conditions and exceptions.** With a 3rd-person singular subject and no *-s* (*he put, it cost*), the verb is past tense (VBD), since the present would be *puts, costs* (VBZ). So *he/she/it* + an invariable verb tagged VBP is an error. After a modal, *to* or *do* the verb is VB; after *have* or in the passive it is VBN. Some verbs vary with *-ed* (*knit/knitted, quit/quitted, bet/betted, wet/wetted, sweat/sweated, broadcast(ed), forecast(ed)*; *shred* is now usually *shredded*). Frozen old participles (*dread moment, roast beef*) are adjectives. *Read* behaves as invariable in writing. Features: VBD `Tense=Past|VerbForm=Fin`, VBP `Tense=Pres`.

**Examples.** *Yesterday he **put** it here* (VBD). *They **cut** costs every year* (VBP). *It has **cost** a lot* (VBN). *Don't **hit** him* (VB).

**Sources.** Jespersen MEG VI 4.4; Sweet NEG I §§1344–1363; Whitney §253; Santorini 1990 §4.1.

**Rules.** `en.morph.invariable-3sg-not-vbp` (error): an invariable verb with subject *he/she/it* is not VBP.

### verb-md-modals
`ai/en/seeds/morph/verb-md-modals.md`

**Gist.** Modals (*can/could, may/might, shall/should, will/would, must, ought*) are defective. They take no 3rd-person *-s* (*he can*), have no infinitive, *-ing* form or participles, and always come first in the verb group. *Could, might, should, would* were historically past forms but are now separate words with their own meanings (*you should go* is not past).

**Conditions and exceptions.** The PTB tag is MD; in UD they are AUX. *Could, would, should, might* have **their own** lemma (*could → could*), not *can*. *Dare* and *need* can be modal (*he need not go*) or main verbs (*he needs to go*), and *ought* takes *to* but is still MD. The infinitive after a modal is bare (VB, `VerbForm=Inf`). Contractions: *'ll, wo → will*, *'d → would*, *ca → can*, *sha → shall*. Missing forms are supplied periphrastically (*be able to*, *have to*). Features: only `VerbForm=Fin` (no tense, mood, person or number); lemma = form except for contractions.

**Examples.** *She **can** swim. It **might** rain. You **should** go. We **must** leave. They **ought** to know.*

**Sources.** UD `docs/_en/pos/AUX_.md`, `feat/VerbForm.md`; Santorini 1990 §§2, 4.1; Sweet NEG I §§1477–1487; Jespersen MEG VI 3.8–3.9, 4.8, 4.9; Kruisinga II.1 §§19, 23; Whitney §§276–278.

**Rules.** `en.morph.md-feats` (error): MD is AUX with `VerbForm=Fin` and no `Tense`, `Mood`, `Person`, `Number`. `en.morph.modal-is-md` (error): an AUX with a modal lemma is tagged MD.

### verb-mixed-weak
`ai/en/seeds/morph/verb-mixed-weak.md`

**Gist.** Many irregular verbs are not strong but weak with complications: they add *-t* or *-d* and also change the stem vowel or consonant. Their past **coincides** with the participle (*kept, told, thought*), so only syntax decides VBD vs VBN. The lemma is the stem, often quite different from the form.

**Conditions and exceptions.** Groups (after Jespersen): *-t* with vowel change (*keep–kept, sleep, sweep, weep, creep, feel–felt, kneel, deal, dream, lean, leap, mean–meant*) or with devoicing (*leave–left, lose–lost, bereave–bereft, cleave–cleft*); *d → t* (*bend–bent, lend, send, spend, build–built*; also *went*, formerly the past of *wend*); *-d* with vowel change (*say–said, flee–fled, hear–heard, sell–sold, tell–told, shoe–shod*); *-d* with consonant loss (*have–had, make–made*); *-t* with deeper change (*bring–brought, think–thought, seek–sought, teach–taught, catch–caught, buy–bought, fight–fought*); vowel change only, past = participle (*feed–fed, lead–led, meet–met, read–read, speed–sped, bleed, breed; hold–held, stand–stood, sit–sat, shoot–shot, get–got, win–won*). *Catched* is non-standard, and *wrought* survives only as an adjective (*wrought iron*).

**Examples.** *She **kept** it* (VBD). *It was **kept** secret* (VBN). *I **thought** so. We **met** yesterday.*

**Sources.** Jespersen MEG VI 4.1, 4.5–4.9, 5.1; Sweet NEG I §§1293–1343; Whitney §§249–255.

**Rules.** `en.morph.mixed-verb-lemma` (error): VBD/VBN *kept, told, thought, brought, met, held …* → base lemma (*keep, tell, think, bring, meet, hold …*).

### verb-participle-adjectives
`ai/en/seeds/morph/verb-participle-adjectives.md`

**Gist.** When a verb develops two participle forms, the old *-en* form often detaches and becomes a prenominal adjective: *a drunken sailor* (but *he has drunk*), *a sunken ship*, *molten lava*, *swollen feet*, *a laden cart*, *clean-shaven*. Likewise *wrought (iron), gilt, roast (beef), dread (moment)*. These are ADJ (JJ, `Degree=Pos`) with their own lemma, not verb forms.

**Conditions and exceptions.** Some are adjective-only: *drunken, sunken, shrunken, molten, graven, bounden (duty), rotten, misshapen*. Others are mostly adjectives but can be verbal: *laden, hewn, strewn, mown, swollen, shaven, stricken*. Adjectives with old [ɪd] are covered in `verb-ed-regular` and `wf-ed-adjectives`. *Born* is the participle of *bear* only for birth in the passive (*he was born*), VBN with lemma *bear*; *borne* is used otherwise (*has borne five children*). *Proven* is mostly an adjective; as a participle it is VBN with lemma *prove*. Adverbs are built on the *-en* form (*brokenly, mistakenly*).

**Examples.** *a **drunken** brawl* (JJ); *he was **drunk*** (JJ, a state); *he has **drunk** it* (VBN); *a **sunken** ship* (JJ); *he was **born** in 1990* (VBN, lemma *bear*).

**Sources.** Jespersen MEG VI 4.4, 4.9, 5.3, 5.5, 5.7; Sweet NEG I §§1386, 1390, 1439, 1462; Whitney §§275, 455; Santorini 1990 §4.1.

**Rules.** `en.morph.participle-adjective-only` (warn): *drunken, sunken, shrunken, molten, graven, bounden, rotten, misshapen* are ADJ. `en.morph.born-lemma` (error): verb *born* is VBN with lemma *bear*.

### verb-strong-ablaut
`ai/en/seeds/morph/verb-strong-ablaut.md`

**Gist.** About two hundred verbs form the past and participle by changing the root vowel, often with *-n/-en* on the participle: *sing–sang–sung, write–wrote–written, take–took–taken, know–knew–known*. The class is closed and slowly losing members to the regular pattern. For many of these verbs **past and participle differ**, so the form itself suggests the tag: *went, saw, took* are VBD; *gone, seen, taken* are VBN. The lemma is the infinitive.

**Conditions and exceptions.** Modern groups (after Jespersen): three vowels without *-n* (*swim–swam–swum, begin, sing, ring, drink, sink; run–ran–run, come–came–come*); participle in *-n* (*drive–drove–driven, ride, write, rise; speak–spoke–spoken, break, steal, freeze, choose; take, shake; blow–blew–blown, grow, know, throw, fly, draw; give, eat, fall, see, do, beat–beat–beaten*); suppletive *go–went–gone*, *be–was/were–been*. *-n* always stays after a vowel or *r* (*seen, done, gone, drawn, known, born, sworn, torn, worn*) and never follows a nasal (*swum, begun*). Double forms: *got/gotten* (*gotten* common in the US), *proved/proven*, *showed/shown*, *woke/waked*, *hung/hanged* ("executed"), *born/borne*. Non-standard speech levels the forms (*I seen it, he done it, had went, have took*), and EWT tags by function (VBN after *have*, even for *went*). So "only VBN / only VBD" is a `warn`, while gross errors (VB, VBP, VBZ, VBG on these forms) are `error`.

**Examples.** *She **wrote** a letter* (VBD). *It was **written** in 1914* (VBN). *We **went** home* / *We have **gone** home.*

**Sources.** Jespersen MEG VI 4.1, 5.1–5.7; Sweet NEG I §§1284–1288, 1364–1456; Whitney §§257–275.

**Rules.** `en.morph.strong-participle-form-tag` (error): *gone, seen, taken, written …* are VBN or VBD. `en.morph.strong-participle-vbn` (warn): in standard use they are VBN only. `en.morph.strong-preterite-form-tag` (error): *went, saw, took, wrote …* are VBD or VBN. `en.morph.strong-preterite-vbd` (warn): in standard use they are VBD only.

### verb-vb-base-form
`ai/en/seeds/morph/verb-vb-base-form.md`

**Gist.** The bare verb stem (*work, go, be*) is the dictionary form. It is tagged VB in three roles: infinitive (*to go, can go, will go*), imperative (*Go!*) and present subjunctive (*I suggest that he go*). In the present indicative the same form is VBP.

**Conditions and exceptions.** Infinitive: after *to*, modals, *do* in questions and negatives (*did you go?*), and *let, make, see* (*let him go*). Imperative: no subject, sentence-initial (*Read the book!*), including *Don't go*, where *do* is also VB. Subjunctive: after *suggest, demand, insist that* and in formulas (*God save the Queen*); there is a subject but no *-s*. The infinitive and subjunctive of *be* is *be* (*that he be told*), while unreal *were* is VBD. UD features: infinitive has only `VerbForm=Inf` (no tense, mood, person or number); imperative has `Mood=Imp|VerbForm=Fin`; subjunctive has `Mood=Sub|Tense=Pres|VerbForm=Fin` plus `Number`/`Person` from the subject. EWT 2.18 annotates subjunctives this way.

**Examples.** *I want to **leave**. You must **try**. **Make** a sandwich! I suggest that he **see** a doctor.*

**Sources.** UD `docs/_en/feat/VerbForm.md`, `feat/Mood.md`, `feat/Tense.md`; Santorini 1990 §§2, 4.1; Sweet NEG I §1290; Whitney §234.

**Rules.** `en.morph.vb-verbform` (error): VB has `VerbForm=Inf|Fin`. `en.morph.vb-finite-mood` (error): finite VB has `Mood=Imp|Sub`. `en.morph.vb-infinitive-bare` (error): infinitive VB has no `Tense`, `Mood`, `Person`, `Number`.

### verb-vbd-past
`ai/en/seeds/morph/verb-vbd-past.md`

**Gist.** The simple past has one form for all persons and numbers (*I/you/he/they worked*): *-ed* for regular verbs, vowel change or other means for irregular ones (*went, sang, kept*). The PTB tag is VBD. Only *be* distinguishes number in the past (*was* vs *were*).

**Conditions and exceptions.** Where past and participle coincide (*worked, kept, made*), function decides: VBD without an auxiliary, VBN after *have* or in the passive (see `verb-vbn-participle`). *Were* in unreal conditions (*if I were rich*) is still VBD but `Mood=Sub`. *'d* is *had* (VBD) or *would* (MD). In non-standard text a past form can stand for a participle (*I have went*), and EWT then tags VBN by function. UD: VERB or AUX, VBD, `Mood=Ind|Tense=Past|VerbForm=Fin` (or `Mood=Sub`), and in EWT 2.18 also `Number` and `Person` from the subject (*he wanted*: `Number=Sing|Person=3`). The lemma is the infinitive.

**Examples.** *She **went** home. We **were** late. I**'d** already left* (*'d* = *had*). *If I **were** you…*

**Sources.** UD `docs/_en/feat/Tense.md`, `feat/VerbForm.md`, `feat/Mood.md`, `feat/Number.md`; Santorini 1990 §2; Sweet NEG I §1290; Jespersen MEG VI 5.6; Kruisinga II.1 §28.

**Rules.** `en.morph.vbd-feats` (error): VBD has `Tense=Past`, `VerbForm=Fin`, `Mood=Ind|Sub`. `en.morph.vbd-agreement-feats` (warn): VBD has `Number` and `Person` (EWT practice).

### verb-vbg-ing
`ai/en/seeds/morph/verb-vbg-ing.md`

**Gist.** The *-ing* form has one shape and several functions. It can be a participle (*she is reading, a sleeping child*), a gerund that keeps verbal structure (*I enjoy reading books*) or an ordinary deverbal noun (*the reading of the will, buildings*). PTB gives every verbal use the tag VBG; UD separates participle and gerund with `VerbForm`, and *-ing* nouns are NOUN.

**Conditions and exceptions.** Verb vs noun: with a direct object or adverb (*closing the plant, cooking well*) it is VBG; with an *of*-phrase, adjective, article or plural (*the closing of the plant, good cooking, buildings*) it is NN/NNS. Verb vs adjective: if it accepts *very*, degree or *un-* (*very interesting, uninteresting*), or follows *seem, become*, it is JJ. `VerbForm=Part` with `Tense=Pres` is used in the progressive with an auxiliary (*is going*), in participial clauses (*…, saying there were…*) and as a modifier. `VerbForm=Ger` (no `Tense`) is used in nominal positions without an auxiliary: subject, object, after a preposition (*I enjoyed working with you*). Spelling follows `spell-silent-e` and `spell-consonant-doubling` (*making, seeing, dyeing, singeing, lying, running*). Colloquial *goin', walkin* and *gon* (from *gonna*) are also VBG with the full lemma. UD: VERB or AUX (*being*), VBG, no person, number or mood, lemma = infinitive (*lying → lie*). An *-ing* noun is NOUN with its own lemma (*building*) and no `VerbForm`.

**Examples.** *He is **sleeping*** (Part). ***Swimming** is fun* (Ger). *a **sleeping** bag* (NN in a compound). *The **meeting** starts at five* (NN).

**Sources.** Sweet NEG I §§101, 324–329, 335, 1600; Whitney §§237–238, 447, 455; Santorini 1990 §4.1; UD `docs/_en/feat/VerbForm.md`; Jespersen MEG VI 21.9; Kruisinga II.1 §§139–151, I §§567, 569, 573; Mätzner I p. 453.

**Rules.** `en.morph.vbg-verbform` (error): verbal VBG has `VerbForm=Part|Ger` and no `Mood`, `Person`, `Number`. `en.morph.vbg-participle-tense` (error): VBG `Part` has `Tense=Pres`. `en.morph.vbg-gerund-no-tense` (error): VBG `Ger` has no `Tense`. `en.morph.vbg-form-ing` (warn): a VBG form ends in *-ing* (or colloquial *-in*, *-in'*).

### verb-vbn-participle
`ai/en/seeds/morph/verb-vbn-participle.md`

**Gist.** The past participle (*worked, taken, gone*) is non-finite, with no person, number or mood. It is used in the perfect (*have taken*), the passive (*was taken*) and as a modifier (*a broken cup*, while still verbal). The PTB tag is VBN.

**Conditions and exceptions.** For regular verbs VBN equals VBD (*worked*), and syntax alone decides: VBN after *have* and in the passive, VBD as a predicate without an auxiliary. Many strong verbs have a separate *-en/-n* or vowel-changed form (*taken, written, known, sung, begun*), which is VBN only (see `verb-strong-ablaut`). A participle that has become an adjective (*a very interested reader*, *broken glass* as a state) is ADJ/JJ; the adjective tests are *very*, degree and *un-* (*unbroken*). UD: VERB or AUX (*been*), VBN, `Tense=Past|VerbForm=Part`, plus `Voice=Pass` in the passive; lemma = infinitive (*taken → take*).

**Examples.** *It has **been** done. The cup was **broken** by the cat. **Written** in 1914, the book…*

**Sources.** UD `docs/_en/feat/Tense.md`, `feat/VerbForm.md`, `feat/Voice.md`; Santorini 1990 §§2, 4.1; Sweet NEG I §335; Whitney §§302–303, 309; Kruisinga II.1 §§64–69.

**Rules.** `en.morph.vbn-feats` (error): VBN has `Tense=Past`, `VerbForm=Part` and no `Mood`, `Person`, `Number`.

### verb-vbp-present-non3sg
`ai/en/seeds/morph/verb-vbp-present-non3sg.md`

**Gist.** In the present tense a verb has only two forms: *-s* for the 3rd person singular (*she works*) and the bare stem for everyone else (*I/you/we/they work*). The PTB tag for the bare present is VBP. So VBP is always a finite present indicative and **never** 3rd person singular.

**Conditions and exceptions.** The same bare stem is also the infinitive, imperative and subjunctive, which are tagged VB (see `verb-vb-base-form`). *Be* has *am* and *are* (and *'m, 're*) as VBP, while *is* is VBZ. Modals have no endings but are MD, not VBP. Non-standard *he don't, it taste good* keeps the form's tag (VBP) and takes features from the subject; this is a text error, not an annotation error, and the rule here does not flag it because it looks only at the word's own features. UD: VERB or AUX, VBP, `Mood=Ind|Tense=Pres|VerbForm=Fin` plus `Number` and `Person` from the subject (*I know*: `Sing|1`; *you know*: `Sing|2` or `Plur`; *they know*: `Plur|3`). `Person=3|Number=Sing` on VBP is impossible.

**Examples.** *I **know**. They **are** here. We **have** time. I**'m** late.*

**Sources.** UD `docs/_en/feat/Tense.md`, `feat/VerbForm.md`, `feat/Person.md`, `feat/Number.md`; Santorini 1990 §§2, 4.1; Sweet NEG I §1290; Whitney §§229–230.

**Rules.** `en.morph.vbp-feats` (error): VBP has `Tense=Pres`, `VerbForm=Fin`, `Mood=Ind`. `en.morph.vbp-not-3sg` (error): VBP is not `Person=3` + `Number=Sing`.

### verb-vbz-3sg
`ai/en/seeds/morph/verb-vbz-3sg.md`

**Gist.** The present tense has a single personal ending, *-s*, for the 3rd person singular (*he works, she goes, it has*); the PTB tag is VBZ. It sounds and is spelled like the noun plural (*works* is both verb and noun), so syntax decides the part of speech.

**Conditions and exceptions.** Pronunciation: [ɪz] after sibilants (*kisses, judges*), [s] after voiceless sounds (*hopes*), [z] otherwise (*loves, plays*). Spelling: *-es* after *s, x, z, ch, sh* (*passes, fixes, watches*); consonant + *y* → *-ies* (*tries*), vowel + *y* → *-ys* (*plays*); some *-o* verbs take *-oes* (*goes, does, echoes*). The irregular forms are *is* (be) and *has* (have); *does* and *says* are spelled regularly but pronounced differently. Modals and modal *need, dare* take no *-s* (they are MD). Archaic *-th/-eth* is also 3rd singular (see `verb-archaic-endings`), and non-standard *I says, they says* spreads *-s* to all persons. Contracted *'s* (*is/has*) is VBZ, and colloquial *ai* (from *ain't*) is VBZ with lemma *be* in EWT. UD: VERB or AUX, VBZ, `Mood=Ind|Number=Sing|Person=3|Tense=Pres|VerbForm=Fin`; lemma = infinitive.

**Examples.** *She **works** here. It **goes** well. He **has** left. It**'s** fine.*

**Sources.** Sweet NEG I §§1290, 1293, 1478, 1487, 1492–1493; Whitney §243; UD `docs/_en/feat/Person.md`, `feat/Number.md`, `feat/Tense.md`; Santorini 1990 §2; Jespersen MEG VI 3.1, 3.8–3.9; Kruisinga II.1 §§5, 19–23; Mätzner I p. 331.

**Rules.** `en.morph.vbz-feats` (error): VBZ has `Number=Sing`, `Person=3`, `Tense=Pres`, `VerbForm=Fin`, `Mood=Ind`. `en.morph.vbz-form-s` (warn): a VBZ form ends in *-s* (exception: *ai*).

### wf-adj-suffixes
`ai/en/seeds/morph/wf-adj-suffixes.md`

**Gist.** Adjectives are most often derived from nouns and verbs with suffixes: *famous, careful/careless, readable, economic, childish, noisy, wooden, troublesome*. *-ous* yields only adjectives; the other suffixes can be ambiguous, because nouns share some endings (*handful, animal, native*). The lemma is the adjective itself (ADJ, JJ/JJR/JJS).

**Conditions and exceptions.** *-ous* (*famous, dangerous, various*) is always an adjective, and *-ously* forms the adverb. *-ful* makes adjectives (*careful, forgetful*) but also measure nouns (*handful, spoonful*; plural *handfuls*). *-less* attaches to nouns (*careless*) and verbs (*countless*), but *unless* (conjunction) and *bless* (verb) do not contain it. *-able/-ible* attaches to almost any verb (*readable*, even *get-at-able*; *-ible* in Latin loans such as *audible*), though *table, cable, vegetable, variable* are nouns. *-ic/-ical* pairs differ in meaning (*economic/economical, historic/historical, comic/comical*), and *-ical* is used where there is a noun in *-ic(s)* (*statistical*). *-ish* forms nationalities (*English*), "like" (*childish*), "somewhat" (*reddish*) and *sixish*, but in *finish, publish, punish, establish* it is a different, borrowed *-ish*. Others: *-y* (*noisy*); *-en* for material (*wooden, golden*; colloquially a noun modifier is more common: *gold watch*); *-some* (*troublesome*); *-like* (*childlike*); *-ly* from nouns (*friendly*, see `adv-ly`); *-ed* from nouns (*talented*, see `wf-ed-adjectives`).

**Examples.** *a **famous** writer; a **careful** driver; **readable** text; an **economic** crisis; a **childish** joke.*

**Sources.** Jespersen MEG VI 13.3, 19.6, 19.7, 20.4, 22.3, 22.6, 23.1–23.3, 25.2; Sweet NEG I §§1606–1614, 1719–1755; Kruisinga II.3 §§1682–1705; Mätzner I pp. 436–438; Whitney §§91, 193.

**Rules.** `en.morph.suffix-ous-adj` (warn): a word ending in *-ous* is ADJ (or PROPN/X).

### wf-compounds
`ai/en/seeds/morph/wf-compounds.md`

**Gist.** A compound joins two stems into one word with its own meaning: *blackbird* (a species), not *black bird* (any black bird). English writes compounds solid (*blackbird, toothbrush*), hyphenated (*gas-mask, well-known*) or open (*tear gas, bus stop*); Jespersen calls the spelling "little short of chaotic". Better criteria are stress (usually one strong stress on the first part) and semantic isolation.

**Conditions and exceptions.** Types: noun + noun (*sunrise, bookseller*, the largest group); adjective + noun (*blackboard, greenhouse*); verb + noun (*pickpocket*); particle + noun (*outcry, bystander*); bahuvrihi, naming the bearer of a feature (*redcoat, paleface*); compound adjectives (*blue-eyed, heart-breaking, God-given, man-made, colour-blind, light-green*); compound verbs, often by back-formation (*housekeep, henpeck, browbeat*). The first part is usually singular even with plural meaning (*toothbrush, a five-pound note*), but it stays plural when that has its own meaning (*clothes-brush, newspaper, goods train*) and in official names (*the Aliens Act*). The plural goes on the head (see `noun-plural-compounds`). Hyphenated compounds are usually one token in EWT (*well-known, long-term*); PTB tags a hyphenated prenominal modifier JJ (*income-tax/JJ return*) and open parts separately (*income/NN tax/NN return*). Open compounds are two words in UD, the first a NOUN attached by `compound`. The lemma of a solid compound is the whole word (*toothbrushes → toothbrush*).

**Examples.** *a **blackbird**; a **toothbrush**; a **well-known** writer; the **bus** stop* (`compound`).

**Sources.** Jespersen MEG VI 8.1, 8.7–8.9, 9.1–9.7; Sweet NEG I §§63–68, 900–915, 1551–1556; Whitney §§102–105, 119, 194, 226; Santorini 1990 §4.1; UD `docs/_en/dep/compound.md`.

**Rules.** None (descriptive seed).

### wf-conversion
`ai/en/seeds/morph/wf-conversion.md`

**Gist.** An English word easily moves to another part of speech without a suffix: *a walk* (from *to walk*), *to bottle* (from *a bottle*), *to clean* (from *clean*), *to up the price*. Jespersen calls this a zero-suffix derivative. The converted word takes **all** formal marks of its new class: a noun takes articles and plurals (*three walks*), a verb takes *-s, -ed, -ing* (*bottles, bottled, bottling*). So syntax and inflection in the actual sentence decide the part of speech, not the dictionary.

**Conditions and exceptions.** Directions: noun → verb (*to bottle, to pigeon-hole, to short-circuit*); verb → noun (*have a look, give a push, three tries*); adjective → verb (*to calm, to dry, to empty*, even *to best*); adverb → verb (*to down, to out*); proper name → noun or verb (*a Plato, to boycott*). Some pairs differ in stress (*'record/re'cord, 'object/ob'ject, 'present/pre'sent, 'import/im'port*), voicing (*use* [s]/[z], *house*, *belief/believe, proof/prove, bath/bathe, advice/advise*) or vowel (*food/feed, blood/bleed, full/fill, song/sing*). Verbs are formed from the **singular** noun (exception *to dice*). The lemma follows the new part of speech (*walks* → *walk* as NOUN or VERB; *bottled* → *bottle*). An article signals a noun, so a word with *a/an/the* cannot be a finite or base verb (VB, VBP, VBZ, VBD). Participles with an article (*the following, the attached*) are borderline cases that EWT tags VBG/VBN.

**Examples.** *Let's take **a walk*** (NOUN). *They **walk** daily* (VERB). *She **bottled** the wine* (VERB, lemma *bottle*). *Prices **upped*** (VERB, lemma *up*).

**Sources.** Jespersen MEG VI 6.1, 6.8, 6.9, 7.1–7.2, 11.2–11.9, 12.1–12.5; Sweet NEG I §§105–106, 164, 887, 914–915; Whitney §§98–99, 225d.

**Rules.** `en.morph.article-not-on-finite-verb` (error): an article (`det`, `PronType=Art`) never attaches to a VERB/AUX tagged VB, VBP, VBZ or VBD.

### wf-ed-adjectives
`ai/en/seeds/morph/wf-ed-adjectives.md`

**Gist.** *-ed* added to a **noun** makes an adjective meaning "having X": *talented, bearded, moneyed*. With a modifier it forms a whole group: *blue-eyed, long-legged, bad-tempered, open-minded*. It looks like the participle ending, but there is no verb behind it (*to talent* does not exist), so these words are adjectives, not VBN. UD: ADJ, JJ, `Degree=Pos`, lemma = form, no `VerbForm` or `Tense`.

**Conditions and exceptions.** Some old words are pronounced [ɪd] (*talented, landed, wicked, wretched, rugged, ragged, crooked, naked*), while compounds have [d] (*blue-eyed*). *be-…-ed* forms (*bespectacled, bewigged, bemedalled*) are also adjectives. In borderline cases a verb does exist (*gifted, skilled, armed, winged*); then meaning decides, and a state or property reading is JJ. Spelling: *ivied, propertied, honeyed*.

**Examples.** *a **talented** singer; a **blue-eyed** boy; an **open-minded** person; a **bearded** man.*

**Sources.** Jespersen MEG VI 4.2, 24.1, 28.1; Sweet NEG I §§907, 1606; Whitney §194; Santorini 1990 §4.1.

**Rules.** `en.morph.ed-compound-adjective` (warn): words ending in *-eyed, -haired, -legged, -minded, -hearted, -handed, -sided, -shaped, -faced, -headed, -footed, -tongued, -skinned, -coloured/-colored, -tempered, -natured* are ADJ. `en.morph.ed-denominal-adjective` (warn): *talented, moneyed, salaried, wooded, bigoted, wicked, wretched, naked, rugged, ragged, jagged, sacred, bespectacled* (and variants) are ADJ.

### wf-negative-prefixes
`ai/en/seeds/morph/wf-negative-prefixes.md`

**Gist.** Negative prefixes turn a word (usually an adjective) into its opposite: *unhappy, impossible, dishonest, non-smoker*. The prefix does not change the part of speech. The negation is **lexical**, so in UD such words get no `Polarity=Neg`; only grammatical words (*not, n't, nor*, interjection *no*) do.

**Conditions and exceptions.** *Un-* is native and the most productive. It combines with adjectives and adverbs (*unkind, unfortunately*), their nouns (*unkindness, untruth*) and participles (*unfinished, unwilling*). With verbs it means "reverse the action" (*untie, undo, unlock, unpack*), so *unlocked* is ambiguous. *In-* goes with Latin stems and assimilates: *il-* before *l*, *im-* before *b, m, p*, *ir-* before *r* (*illegal, impossible, irregular*); compare *unable/inability, unjust/injustice*. Some forms are lexicalized (*infamous, invaluable* "priceless", *impertinent, indifferent*). Others: *dis-* (*dishonest, disagree*), *non-* (*nonsense, non-profit*), Greek *a-* (*amoral, atypical*), *mis-* "wrongly" (*misread*). A prefix written separately or split at a hyphen is sometimes a separate token with XPOS AFX in EWT (*non profit, mid 90s*), with inconsistent UPOS. The lemma keeps the prefix (*unhappy*, not *happy*).

**Examples.** *an **unhappy** child; it's **impossible**; a **dishonest** man; **non-profit** organisations.*

**Sources.** UD `docs/_en/feat/Polarity.md`, `docs/_en/pos/X.md`; Jespersen MEG VI 26.1–26.6; Sweet NEG I §§1584, 1587, 1642, 1652, 1658; Whitney §§100, 193d.

**Rules.** `en.morph.no-polarity-on-lexical-words` (error): ADJ, ADV, NOUN, PROPN and VERB tokens have no `Polarity`.

### wf-noun-suffixes
`ai/en/seeds/morph/wf-noun-suffixes.md`

**Gist.** Unlike a prefix, a derivational suffix usually **changes** the part of speech and determines it: *kindness, reality, socialism, childhood, creation, government, teacher*. So the word ending is a strong tagging cue: words in *-ness, -ity, -ism, -hood* are almost always nouns, and *-nesses, -ities, -isms, -tions, -ments* are plural nouns. The lemma is the singular derived word (*kindnesses → kindness*), not the base.

**Conditions and exceptions.** *-ness* is the most productive suffix: it attaches to any adjective or participle (*readiness, preparedness*), even to phrases (*up-to-dateness*), but is not used where a noun already exists (*possibility*). Special cases: *business* (not *busyness*), *wilderness*, and *witness, harness*, which are also verbs. *-ity* goes with Latin stems (*reality, ability*); *pity* can be a verb, and *quality* before a noun is sometimes JJ in EWT. *-ism/-ist* pair with *-ize* (*socialism, socialist, socialize*), and *-ist* can be an adjective (*racist*). *-hood, -ship, -dom*: *childhood, friendship, freedom* (*worship* is also a verb; *random, seldom* do not contain the suffix). *-tion/-sion/-ation, -ment, -ance/-ence, -al* name actions and results, and many have become verbs by conversion (*to question, to mention, to function, to comment*). Personal and diminutive suffixes: *-er/-or, -ee, -ess, -ist, -ant/-ent*; *-let, -ling, -ette*. *-th* from adjectives: *warmth, length, strength, depth, width, truth*.

**Examples.** *his **kindness**; the **reality**; **racism**; in my **childhood**; two **creations***.

**Sources.** Jespersen MEG VI 13.6, 14.1, 19.1, 19.3, 19.4, 19.9, 21.8, 22.2, 23.4–23.5, 24.4, 25.3; Sweet NEG I §§1597–1605, 1682–1718, 1733; Kruisinga II.3 §§1634–1681; Mätzner I p. 450; Whitney §§90–96, 118.

**Rules.** `en.morph.suffix-ness-noun` (warn): *-ness(es)* words are NOUN (or PROPN/X; VERB for *witness, harness*). `en.morph.suffix-ity-ism-hood-noun` (warn): *-ity, -ism, -hood* (and plurals) are NOUN, PROPN or X. `en.morph.suffix-tion-noun` (warn): *-tion/-sion* words are NOUN (or VERB by conversion, PROPN, X). `en.morph.derived-plural-noun` (warn): nouns in *-nesses, -ities, -isms, -tions, -sions, -ments, -ships, -hoods* are NNS, `Plur|Ptan`.

### wf-prefixes-class-changing
`ai/en/seeds/morph/wf-prefixes-class-changing.md`

**Gist.** Most English prefixes keep the part of speech, but three old ones change it. *A-* (from *on*) turns nouns and verbs into predicative adjectives or adverbs: *asleep, afloat, alive, ahead*. *Be-* makes verbs from nouns or adjectives (*befriend, belittle, becalm*) and *be-…-ed* adjectives (*bespectacled*). *En-/em-* makes verbs meaning "make so": *enlarge, enrich, empower, enable*.

**Conditions and exceptions.** *A-* adjectives (*asleep, afraid, alive, awake, aware, alike, alone, ablaze, afloat, adrift, aglow, ashamed*) appear only **after** a noun or as predicates (*the child is asleep, the people asleep*, not *an asleep child*). Before a noun another word is used: *a live broadcast*, *a sleeping child*. They form no *-ly* adverbs (*afraidly* is impossible). *Alone, alike, ahead, away, aside, apart* are often ADV. *Em-* is used before *b, p* (*embody, empower*), with the same meaning as the verbal suffix *-en* (*enrich* ~ *sweeten*). In UD the *a-* adjectives are ADJ (JJ) and, as noun modifiers, `amod` placed after the noun. *Be-* and *en-* derivatives are ordinary VERBs whose lemma includes the prefix (*befriend, enlarge*).

**Examples.** *The baby is **asleep*** (ADJ, predicate). *Children **asleep** in the car* (ADJ after the noun). *They **befriended** him* (VERB). *We need to **enlarge** it* (VERB).

**Sources.** Jespersen MEG VI 20.5, 22.9, 27.3, 27.4, 28.1; Sweet NEG I §§1506, 1572, 1588, 1653; Whitney §225b.

**Rules.** `en.morph.a-adjective-postposed` (warn): *asleep, afraid, alive, aware, awake, ashamed, ablaze, afloat, adrift, aglow, akin, averse, alike* as `amod` come after their noun.

### wf-prefixes-neutral
`ai/en/seeds/morph/wf-prefixes-neutral.md`

**Gist.** Most English prefixes only refine meaning and do **not** change the part of speech: *rewrite* (verb), *overconfident* (adjective), *co-pilot* (noun). So a derived word usually has the same tag as its base, and the lemma is the whole word including the prefix.

**Conditions and exceptions.** Productive native prefixes: *un-, mis-* (*misread*), *over-, under-, out-* (*overestimate, underpay, outrun*), *fore-* (*foresee*). Productive borrowed prefixes: *re-* (*reconsider, re-enter*), *pre-, post-, ante-*, *co-*, *anti-, counter-*, *sub-, super-, ultra-, hyper-*, *semi-, demi-, mini-, multi-, mono-, poly-*, *inter-, trans-*, *ex-* "former" (*ex-wife*), *vice-*, *pseudo-, neo-, auto-*. A hyphen is used where the word would otherwise clash with an older loan (*re-form/reform, re-sign/resign, re-cover/recover, re-mark/remark*). Stressed *re-* [riː] is productive, while unstressed [rɪ] in old words (*receive, repeat*) is no longer a separable prefix. *Pre-war, anti-war, post-2000* before a noun are modifiers (JJ), even though the base is a noun. A prefix written as a separate token gets XPOS AFX in EWT (see `wf-negative-prefixes`).

**Examples.** *I **rewrote** it* (VBD, lemma *rewrite*). ***pre-war** Europe* (JJ). *an **overconfident** player* (JJ). *the **co-author*** (NN).

**Sources.** Jespersen MEG VI ch. XXVII–XXVIII, 28.2; Sweet NEG I §§1584–1588, 1620–1680, 1668; Whitney §101; Santorini 1990 §4.1.

**Rules.** None (descriptive seed).

### wf-reduplication
`ai/en/seeds/morph/wf-reduplication.md`

**Gist.** A small group of words is formed by repeating a stem: unchanged (*pooh-pooh, goody-goody, so-so, bye-bye*), with a vowel change (*zigzag, chit-chat, ping-pong, dilly-dally, shilly-shally, tip-top, sing-song, willy-nilly, riff-raff*) or with rhyme on a changed first consonant (*hanky-panky, helter-skelter, higgledy-piggledy, hip-hop*). They express sound imitation, repeated action, contempt or playfulness. Usage decides the part of speech: *zigzag* is noun and verb, *so-so* adjective and adverb, *willy-nilly* adverb.

**Conditions and exceptions.** They are written solid (*zigzag*) or hyphenated (*ping-pong*); a hyphenated word is one token in EWT. The lemma is the whole word (*zigzagged → zigzag*), and reduplicated verbs inflect regularly (*shilly-shallied*). Emphatic repetition in speech (*very very good, a big big house*) is not word formation but two separate words. Tags and lemmas are those of an ordinary word of that class, with no special features.

**Examples.** *The path **zigzags** up the hill* (VERB). *a **so-so** film* (ADJ). *They play **ping-pong*** (NOUN).

**Sources.** Jespersen MEG VI ch. X, 10.2–10.4; Kruisinga II.3 §§1913–1918; Whitney §334.

**Rules.** None (descriptive seed).

### wf-shortening
`ai/en/seeds/morph/wf-shortening.md`

**Gist.** English readily shortens long words. **Clipping** usually keeps the beginning: *phone, exam, lab, ad, photo, bike, pub, vet, flu, maths/math*. Clipped words become ordinary nouns with their own plurals (*phones, exams*). **Back-formation** removes an imagined suffix to create a new word: *editor → edit, burglar → burgle, enthusiasm → enthuse, television → televise, housekeeper → housekeep, pease → pea*.

**Conditions and exceptions.** A clipped word is a separate lexeme whose lemma is the clipped form (*phones → phone*, not *telephone*). UD's `Abbr=Yes` is for written abbreviations (*Mr., govt, e.g.*, *u* for *you*), not for clippings that have become ordinary words: *phone, exam, info* have no `Abbr` in EWT. Sometimes the end (*phone, bus* from *omnibus*) or middle (*flu*) survives. Losing unstressed initial syllables gives separate words (*mend < amend, fence < defence, sport < disport*). Initialisms (*M.P., a.m., BBC*) are a separate type, tagged by function (NNP for names), and EWT marks them with `Abbr=Yes` inconsistently. Back-formation often changes the part of speech, typically noun → verb (*edit, burgle, babysit*).

**Examples.** *My **phone** died. Two **exams** left. She **edits** the paper* (VERB, lemma *edit*).

**Sources.** Jespersen MEG VI 4.4, 29.2–29.9; Kruisinga II.3 §§1904–1911; Mätzner I pp. 173, 177; Sweet NEG I §§998, 1001; UD EWT 2.18 practice.

**Rules.** `en.morph.clipping-lemma` (error): nouns *phone(s), exam(s), lab(s), ad(s), photo(s), flu, bike(s) …* have the clipped form as lemma. `en.morph.clipping-not-abbr` (warn): such established clippings have no `Abbr`.

### wf-verb-suffixes
`ai/en/seeds/morph/wf-verb-suffixes.md`

**Gist.** Verbs are derived from adjectives and nouns with a few suffixes meaning "make so, turn into": *realize, modernize, clarify, justify, widen, shorten, activate*. There are few productive verb suffixes, because English easily makes verbs without one (conversion: *to dry, to empty*; see `wf-conversion`).

**Conditions and exceptions.** *-ize/-ise*: *-ize* is American and Oxford spelling, *-ise* common British, and lemmas follow the spelling of the text. Some verbs are spelled only *-ise* (*advertise, advise, comprise, despise, devise, exercise, supervise, surprise*), and *size, prize, seize, wise, precise* do not contain the suffix. *-ify/-fy* (*clarify, justify, testify, intensify*; *-efy* in *liquefy, stupefy*) always marks a verb. *-en* (*widen, redden, sharpen, soften, darken, fasten*) attaches only after consonants that allow syllabic *n* (no *smallen*); *lengthen, strengthen* exist because there are no verbs *long, strong*; in *frighten, listen, happen* the suffix adds nothing. *-ate* comes from Latin participles (*separate, dedicate*); adjective *separate* is pronounced [-ɪt], the verb [-eɪt]. *-ish* in verbs (*finish, punish, publish, establish*) is borrowed and unproductive. UD: VERB in all forms; lemma = stem with suffix (*clarified → clarify, widening → widen*).

**Examples.** *We need to **realize** it. Please **clarify**. The road will **widen**. They **activated** the account.*

**Sources.** Jespersen MEG VI 4.4, 19.4, 19.5, 20.5, 24.8, 25.1; Sweet NEG I §§1616, 1754–1758; Kruisinga II.3 §§1627–1633, I §584; Mätzner I pp. 439, 473–474; Whitney §§95, 225a.

**Rules.** `en.morph.suffix-ify-verb` (warn): a word ending in *-ify* is VERB (or PROPN/X).
