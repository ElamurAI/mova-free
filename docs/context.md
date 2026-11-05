# Context, layers and memory

This page describes how Mova keeps the context of a document: the context features of the parser, the division of
labour with the teacher, the tale world and its memory, the layers annotated over the same text, the SLM's
subsystems (mathematics, the mini-lab, the sandbox, the world model) and what is inferred from context. It names
what is implemented and where; what is not implemented is listed as a gap. The subsystem docs
([english.md](english.md), [pragmatics.md](pragmatics.md), [knowledge.md](knowledge.md), [school.md](school.md),
[training.md](training.md)) and the crate READMEs hold the full contracts.

## Multi-pass parsing

The tagger and parser in `en` take document context features in four groups (document genre; sentence type, current
and previous; paragraph start and speaker change; the entity status of the token) and evaluate them (`en/src/ctx.rs`, `en/src/ctxeval.rs`). The features are experimental; their
switches and evaluation modes are in
[the en README](../en/README.md#document-context-features-ctxrs-experimental). Foreign or shuffled context is the
control: it must hurt, or a "gain from context" is not real. A second pass can derive the context from the first (`ctx::derived`). GUM serves research evaluation only and does not
pass the saved-model training gate. A full automatic parse → all document layers → reparse loop is not implemented.

## Small model and the teacher: division of labour

Mova's native language processing is Rust, with CPU and optional GPU paths. External models serve distinct roles:
developer, teacher and consultant; they never answer Mova's assessments for it. In `annot` the SLM writes a draft
(tokens, tree, pragmatic cues), the teacher returns edits, and Rust gates check every edit before it is kept
([training](training.md)). Output tokens are the teacher's main cost, so it writes only edits and Rust applies them.
No general `need:`/`prepare:` protocol for a teacher to call SLM commands is implemented.

### Transparent judgments

Every SLM decision leaves a trace, across `en`, `coref`, `prag`, `onto` and `world`: for a tag, the perceptron
features and weights that tipped it; for FEATS, the table key, back-off level and counts; for a relation, the parser
step, score and main features; for rules, the seed with its source (book, page) or the engine rule id; for the
converter, a rule id per changed token; for gates, what failed and why. SLM models stay simple and explainable:
tables, perceptrons, rules, memory-based classifiers.

## The tale world

### Working memory

Working memory is `world::query::Context` (`world/src/query.rs`), a view of the tale world from the cursor at sentence N:
the main and secondary characters by attention (mentions), the speaker and addressee, what is happening, why (with
its event), where and when, each a reference into the world; `world/src/md.rs` renders it at the end of a text. Manner,
topic and the current speech act are not in it, and a working memory shared by every subsystem is not implemented.

### Tale world: entities, registries, cursor

Implemented in `world` ([world/README.md](../world/README.md)): registries, the cursor, mentions, immutable states,
the change language in layers v1–v3, gates and question readers.

"In the field there is an oak": the cursor creates location L1 (type field) in the location registry; object O1 (type
tree, class oak) goes into the object registry; L1's objects receive O1 and O1's `at` is L1.

- **Global registries:** locations, objects, characters, events; every entity has a type, a class and parameters.
- **Bidirectional links:** `L1.objects ∋ O1` ⇔ `O1.at = L1`.
- **The main cursor** — where and when the narrative is: `at` and the text position (chapter, paragraph, sentence).
- **Mentions:** an append-only list of cursor positions per entity, giving first and last mention.

Working memory is a view of the world from the cursor.

**Minimal objects.** A character is a set of references and simple attributes — gender, mother, father, age,
relations — each unknown by default (`enum Val<T> { Unknown, Known(T) }`). Relationships (romantic, friends,
acquaintances, adversaries, kin) are separate objects with their own anchored facts.

**Immutable states.** A character is an identity (like a git branch); each change creates an immutable state (like a
commit); events, mentions and relations point to a specific state.

```
C1@1  { name: Jackie }                                 ^s1  "Jackie"
C1@2  C1@1 + { gender: male, age: boy }                ^s1  "was a boy"
C1@3  C1@2 + { beauty: handsome }                      ^s1  "handsome"
C1@9  C1@8 + { eyes: 1 }   cause: E57 "lost an eye"     ^s912
```

Internally a state is its parent plus changed fields; externally each `C1@n` is a full snapshot that can be diffed and
cited.

**State rules:** no silent changes — a value changes only through an event, as place changes only through `move`; a
contradiction ("both eyes" again without healing) is a continuity error; two times — text position and story time.

**World-change language**, one command per line, with a sentence anchor:

```
cursor chapter=1 para=1 sent=1
loc  L1 type=field                               ^s1   L1.mentions += (1,1,1); cursor.at = L1
obj  O1 type=tree class=oak at=L1                ^s1   L1.objects += O1; O1.at = L1
char C1 type=animal class=fox name="Fox" at=L1   ^s2   L1.chars += C1
move C1 to=L2                                    ^s5   L1.chars −= C1; L2.chars += C1; C1.at = L2
have C1 O3                                       ^s6   C1.items += O3; O3.at = C1
say  C1 to=C2 act=request                        ^s7   utterance from pragmatics
event E1 verb=eat agent=C1 patient=O4 at=L2 why=hunger   ^s8
cursor at=L2 time=T3                             ^s9
```

The teacher writes the commands; the SLM executes them with gates: references only to existing entities, consistent
links, types from closed sets, no teleportation. Questions such as "where is the oak?", "what does the fox have?",
"who was in the field when…?" are answered from state and history. An SLM writer of world changes is not
implemented.

### Non-linear narrative

Layer v3 of the change language in `world` keeps canonical events, story time as a partial order, narrative frames,
reality branches, observers' claims and knowledge, and reveals. Narratology separates the **fabula** (what happened)
from the **syuzhet** (how it is told).

1. **Two time axes.** Text position is the cursor; story time is a separate axis where events are placed relative to
   each other (Allen intervals, as in TimeML).
2. **Narrative levels:** a stack of frames; "The old man began his story…" opens an embedded story with its own
   narrator.
3. **Reality status:** happened, dream, plan, belief, lie, "what if", reported speech. Only real events change the
   actual world; the rest live in branches.
4. **Who knows what and when:** each character's and the reader's knowledge is apart from the truth. Solving a mystery
   is belief revision, not a change of the past.
5. **Parallel lines are not supported:** there is no interface for several cursors on one story-time axis.
6. **Gates in story time:** the teleportation gate compares adjacent states on the story-time axis, not adjacent
   sentences.

**Event identity independent of its descriptions:**

```
event E7 (world truth: murder)
  claim  reader   E7 = unknown          ^s12
  claim  Watson   E7 = accidental death ^s40
  claim  Holmes   E7 = suspected murder ^s88
  reveal "in fact he died two hours before Watson arrived" ^s512
        → E7 BEFORE E9 (Watson's arrival); Watson's claim → false
```

**Pipeline v3 (the non-linear world):**

```
text order → sentence cursor → narrative frame (who narrates, level)
  → event mention or claim → resolve to a canonical event or create one
  → place on the story-time axis → reality branch
  → update the observer's knowledge → only then do gates decide whether the canonical world changes
```

Time is a partial order (`A BEFORE B`, `C DURING B`), with absolute time only when the text gives it. A linear
narrative is the special case.

### Hierarchical and global worlds

The tale executor supports nested narrative frames: an embedded story, dream or plan lives in its own frame world.
Global entities shared across tales (cross-document coreference, `scope=global|book|tale`) and epochs that reread a
collection until its global world stops changing are not implemented for tale worlds; the separate
[Whale world store](../tools/whale-world/README.md) indexes document dossiers by epoch.

### Thousands of domains

One domain world model exists: the HS22 goods classification read into an ontology (`onto hs`, `onto/src/hs.rs`;
[onto/README.md](../onto/README.md)). General domain profiles (types, attributes and a command vocabulary as data, per
domain) are not implemented. The `layers` conductor gives each domain module an access level (`layers::Access`: public, extended, private).

## SLM subsystems

Each subsystem has a deterministic fast core and a slower logical model that chooses and composes the steps for the
core.

### Mathematics

Implemented in `math` ([math/README.md](../math/README.md)): the exact core, the curriculum solver, the slow models
`math/src/arith.rs` and `math/src/steps.rs`; the quantitative world layer in `world/src/quant.rs`. School results:
[school.md](school.md#results).

- **Understanding:** "15% of 80", "72°F in Celsius" → UD tree → expression, or an honest "I don't understand".
- **Exact core:** rationals (overflow is an error), big integers, units with dimension checks and affine temperature,
  a symbolic expression tree with steps.
- **Independent checks** wherever possible: substitute the root, differentiate the integral, A·A⁻¹ = I, a second
  path. Checking is easier than generating, so the check is the gate.
- **Levels:** arithmetic and units; algebra and linear systems over rationals; analysis; linear algebra; number theory
  with certificates (Bézout, Miller–Rabin, Pocklington, Pollard–Brent, CRT); probability; first-order ODEs. Formal
  proofs go through Lean.

**The slow models.** `arith` picks the needed numbers and the operation at each lowest common ancestor (Roy & Roth
2015) with an averaged perceptron, and the core enumerates and recomputes trees exactly. `steps` is a transition system
like arc-standard parsing (SHIFT / SKIP / REDUCE with an operation, plus combining with deeper stack items, like SWAP
in Nivre 2009), a structured perceptron with beam and early update, trained on gold trees from equations and
calculator steps. Non-lexical features (verb classes, rates and unit dependencies, subject matching), a top-K reranker
(Collins 2000) and self-consistency for calibration sit on top. Bag-of-words features cannot see who has what; that
needs state tracking across sentences, which is the world.

**The quantitative layer of the world** (`world/src/quant.rs`): state `entity.thing → number`; actions set, add, sub,
mul, div, move, ask; each command bound to a sentence `@N` with a principle `k=` (given, total, gain, loss, transfer,
rate, compare, part, convert, combine, unit). The exact core executes; gates check that numbers come from the text so
far, only existing things change, there is an ask and the answer matches.

Physics and chemistry have no core of their own: units and dimension checks are in [math](../math/README.md), and
numerical and statistical operations in [mlab](../mlab/README.md).

### Mini-lab: a MATLAB/Octave-style language and built-in LaTeX

Implemented: `mlab` ([mlab/README.md](../mlab/README.md)) and `latex` ([latex/README.md](../latex/README.md)).

- **mlab:** a subset of the MATLAB/Octave language with its own Rust interpreter (`mlab run file.m`, `mlab -e "…"`):
  matrices with `end` indexing, masks, broadcasting, anonymous functions, `if/for/while/switch/try`.
- **Built-ins** in an extensible registry: linear algebra on `faer`, FFT, `integral`, `fzero`, `fminsearch`, `ode45`,
  polynomials. Gates on `\`, `inv`, `eig`, `svd`, `lu/qr/chol` check the defining identity and warn with a number.
- **Tables and statistics:** columnar `table` with grouping, pivot and joins; CSV/TSV/JSON with line/column errors;
  descriptive statistics, distributions, t/χ²/ANOVA/correlation tests and OLS. Every test prints hypotheses,
  assumptions, steps and conclusion; p-values are gated by seeded Monte Carlo under H0.
- **Licences:** behaviour comes from Octave's documentation (clean reimplementation), not its GPL code; the product
  does not use the trademark "MATLAB".
- **latex:** a command table → `build.rs` → enums; macros; unknown commands as `Unknown` nodes with positions;
  canonical LaTeX and plain-text printing; formula extraction from `.tex`; a bridge to `math::Expr`. No external TeX.

Exact mode, cell arrays, structs, plots and XLSX/ODS are not implemented.

### Execution sandbox: the SLM programs, bytecode, an in-memory file system

Implemented: the mlab register VM (`mlab/src/vm.rs`), the in-memory file system `vfs` ([vfs/README.md](../vfs/README.md))
and the WASI host `wasi-host`.

1. **Language VM + `vfs`:** mlab bytecode, files in memory. The VM is register-based, as in Lua 5, with
   superinstructions; whatever does not compile falls back to the tree walker, and an equivalence gate requires
   byte-identical output on both. `vfs` is a copy-on-write tree with constant-time snapshot and byte-exact restore,
   limits, a log and a virtual clock; escaping the root is an error; mlab uses `--fs mem` by default, `dir:<folder>`
   or, only explicitly, `real`.
2. **WASI host:** `wasi-host` runs WASM programs such as `mlab.wasm` against the same `vfs`.

Guest programs see an in-memory file system, a virtual clock, seeded randomness and no network, so a run is
reproducible. A coding curriculum, a shared bytecode for all languages, a virtual POSIX kernel, other language
frontends, runtime code-module generation and hot plugging are not implemented; [add-on modules](modules.md) are data
packages, not executable extensions.

### Reasoning and logic

Defaults with exceptions and proofs are in `onto`, derivation over the world in `world`
([knowledge.md](knowledge.md)). A general Horn-clause engine is not implemented.

### World model — separate from the narrative

Implemented in `onto` ([onto/README.md](../onto/README.md)): the ontology graph (OEWN, FrameNet), the Wikidata walker,
the compiled core (`onto/build.rs`, `onto/src/core.rs`), the core–graph gate (`onto/src/coregate.rs`), WSD
(`onto/src/wsd.rs`), `onto describe` and the pronoun API (`onto/src/pron.rs`).

That an apple is a fruit is world knowledge, not tale knowledge, so the world model is separate. **Two levels, like
TBox and ABox:** the world model holds abstractions shared by all texts (concepts and hierarchies, default properties,
parts and wholes, typical actions and consequences, roles); the narrative holds one text's individuals (Crow1@state3,
her cheese), each referencing a concept without copying it.

**Tale worlds are layers over the world model:** real world → genre (in a fable animals talk) → collection → tale,
with exceptions written at their own level. The most specific layer wins; conflicting defaults with no winner (the
Nixon diamond) are "undetermined" with both proofs.

**The core is compiled, the periphery is dynamic.**
- **The core** — a few hundred well-known concepts (fable animals, food, household, family, roles) as Rust types that
  `build.rs` builds into `enum CoreId` and static records from curated seeds, as the dictionary is compiled into `en`.
  Every "concept × field" cell is explicit; a missing field fails the build; every value has a source.
- **Properties are distributions:** `{mean, sd, min, max, n, unit}`; risks are probabilities. A specific lion exists
  only in a narrative, with values from the text or, flagged "assumed", from the distribution.
- **Periphery:** the rest of WordNet and Wikidata as a dynamic graph. A word maps to a core concept if there is one,
  otherwise to a graph node; a core–graph contradiction is a refusal, not a silent choice.
- **The Wikidata walker** stops the parent-taxon chain at the class rank (a whale is not a fish; a pike is) and maps
  synsets to items only with two supporting sources and a generality threshold.
- **WSD:** Lesk + corpus predominant sense (McCarthy 2004) + core + domain, avoiding first-sense errors ("oak — wood").
- **`onto describe`** gives concept descriptions with proofs; the **pronoun API** gives gender, animacy and humanness
  for coreference; tale-world classes reference ontology nodes, so "Can the crow fly?" is answered by inheritance.

Corpus-derived categories and arc plausibility as reparse features are not implemented.

## Layers over the same data

| layer | what it contains | where | gold standard |
|---|---|---|---|
| 1. tokens, sentences, paragraphs | boundaries | `en` | UD, GUM `newpar` |
| 2. morphology | UPOS, XPOS, FEATS, lemma | `en` | UD |
| 3. syntax | UD tree in the `mova` dialect ([dialect.md](dialect.md)) | `en` | UD |
| 4. entities and coreference | who is who, pronouns | `coref` | GUM, GENTLE `Entity=` |
| 5. discourse | cause, purpose, elaboration, contrast between clauses | not implemented | GUM `Discourse=` (eRST) |
| 6. pragmatics | speech act, what was meant, attitude | `prag` | GUM `s_type`, dialogue-act corpora |
| 7. document passport | genre and sentence type as context features | `en` (partly) | GUM `meta::*`, GENTLE and EWT genres |
| 8. time and place | deixis, dates from the document anchor | not implemented | — |
| 9. domain layer | frames, slots, quantities, units | not implemented | per domain |
| 10. story layer | who did what, to whom, when, where, why, per tale | `world`, `prag` frames | GUM (summaries, eRST), LitBank |

Each layer rests on the lower ones. GUM carries almost all layers, so each is checked against its own gold; GUM and
other NC data serve measurement only. Chapter, book and series memory across sessions is not implemented.

## Memory as an md graph

`world/src/md.rs` renders the tale world as Markdown with sentence anchors: `# Tale world: <title> (doc N)`, then
`## Text` (one line per sentence, `^s1` with its paragraph), `## Cursor`, the location and object registries,
`## Characters: immutable states` (each `C@n` with its delta, cause and anchor), `## Relations`, `## Event chronology`,
`## Working memory at the end` and `## Gates` (rejected lines and sentences without a command); layer v3 adds the
story time axis, reality branches, observer claims, knowledge, reveals and frame worlds. A human reads it as text;
every row points back to its sentences. Teacher-written level summaries and a graph-walk command are not implemented.

## Five-year-old test

Question generation is in `qagen` ([qagen/README.md](../qagen/README.md)) and FairytaleQA answering from the tale world
in `world/src/ftqa.rs`. On whole public-domain tales the check is whether Mova understands a tale as a five-year-old
does: who is who throughout, who wants what and why, what led to what. **Gold standard — FairytaleQA** (Xu et al., ACL
2022): children's tales with educator-written questions of seven types: character, setting, action, feeling, causal
relationship, outcome resolution, prediction.

**Training questions from the LLM.** For each paragraph the teacher writes questions and answers of these types,
flagged explicit or inferred, with anchors to the source sentences; `qagen` stores them in a `qa` table (document,
paragraph, question, type, answer, anchors, explicit or inferred) as training data. The texts are public
domain, so the data is clean for what is built.

## Document memory

Entity and mention memory is in `coref` ([coref/README.md](../coref/README.md)) and in the `world` registries: mentions,
number, gender and person from UD features and closed lists, animacy, names and variants. Gender and other sensitive
attributes are never guessed from a name. `coref` is a deterministic sieve
over UD trees; every link has a sieve id and evidence (`coref explain`). Information status and open ellipses are not
implemented.

## Document level: domain, style, narrator, purpose, mode

Genre and sentence type are context features of `en` (`en/src/ctx.rs`), from GUM document metadata and sentence
comments (`# meta::genre`, `# s_type`, `# speaker`, `# addressee`). A full document passport (domain, style, narrator,
purpose, mode as `# mova::` fields) is not implemented. Gender and other sensitive attributes of a narrator or speaker
are never guessed from a name.

## Pragmatics: what was meant

Implemented in `prag` ([prag/README.md](../prag/README.md); guide: [pragmatics.md](pragmatics.md)): per-sentence
labels, teacher silver (`prag/src/silver.rs`), three SLM models behind one interface (averaged perceptron, IGTree,
k-NN), irony candidates and the frames reader. What a speaker meant often differs from what was said: indirect acts,
implicatures ("I wouldn't buy it again" → a negative review), presuppositions, irony, hedging.

**Per-sentence labels** (the schema `prag` implements):

| field | values |
|---|---|
| `form` | `decl` · `q` yes/no question · `wh` · `imp` · `excl` · `frag` · `intj` · `other` |
| `act` | `assert` · `narrate` · `ask` · `request` · `offer` · `suggest` · `promise` · `thank` · `apologize` · `greet` · `agree` · `disagree` · `evaluate` · `express` · `warn` · `other` |
| `indirect` | yes/no: the form is not the usual one for the act ("Can you open the window?" — `q`, but `request`) |
| `hedge` | yes/no: *maybe, I think, sort of, perhaps* |
| `polarity` | `pos` · `neg` · `none`: the speaker's evaluation (negation has its own column) |
| `voice` | `narrator` · `character` (direct speech) · `none` |
| `means` | briefly, what was meant; written only by the teacher |

- **Data:** public-domain tales and Tatoeba (CC0); teacher silver with the preceding sentences as context.
- **Models**, one interface: an averaged perceptron per field (UD tree, words, context); **IGTree** (TiMBL) — memory as
  a prefix tree, classification as a walk whose path is the explanation; **k nearest neighbours**, explained by the
  most similar stored sentences. Model agreement is a confidence signal.
- **Checks:** held-out silver; `form` also against GUM `s_type` (measurement only, GUM is NC); shuffled labels must
  fall to chance.
- **Release boundary:** pragmatics is its own crate, so its data licences are tracked separately
  ([licence/DATA.md](../licence/DATA.md)).

Presuppositions and the `# mova::` output fields are partly implemented.

## What is inferred from context

1. **Pronouns and definite NPs:** he, it, they, this, the company → an entity in memory (`coref`).
2. **Relative pronouns:** the antecedent from `acl:relcl` (`en::ud::agreement`).
3. **Names:** "Barack Obama … Obama … the president" (`coref`).

Bridging, deixis and ellipsis are not implemented.

## Standards and data

- **Output format:** CorefUD — `Entity=` in MISC, bracket notation `Entity=(e3-person-new-…` … `e3)`, as in GUM and
  GENTLE; UD stays valid.
- **Gold standard:** GUM (many genres) and GENTLE (genres outside GUM, a transfer test); both NC, for measurement only.
- **Metrics** — as in CorefUD and CRAC: MUC (Vilain et al. 1995), B³ (Bagga & Baldwin 1998), CEAF-e (Luo 2005) →
  CoNLL F1, plus pronoun accuracy. Random antecedents must fail.
