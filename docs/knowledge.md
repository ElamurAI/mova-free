# Worlds and knowledge

This page describes where Mova keeps what it knows and how it uses it. It covers the knowledge contract (the skeleton and hard parameters, the important graph in RAM, the global graph), the absurdity matrix (a graded table of how plausible an event is, used to find parse errors and to switch on humor and irony), the nerve graph that ties words to code and worlds, and how Mova expands documents into layers for retrieval (RAG). Each section says what is implemented and what is design.

## The knowledge contract

Three complementary mechanisms, not three exclusive destinations for facts:

| component | role | update pattern |
|---|---|---|
| skeleton and hard parameters | mechanisms of understanding: representations, linguistic, spatial and logical operations, foundational world models and constraints | changed deliberately through code, model revision and retraining |
| important graph | operationally useful logical and informational knowledge resident in RAM; an active projection with prepared links and indexes | activated, refreshed and evicted as tasks and usefulness change |
| global graph | the durable information graph for human knowledge and knowledge derived by AI, especially Mova | records, versions and links knowledge continuously with provenance |

The crate `global` contains compiled knowledge; it is not the global information graph.

**The skeleton** supplies the mechanisms of understanding and the harder parameters; understanding is a capability, not the address where a fact is stored. General concepts and foundational knowledge (spatial frames, calendar operations, time, animals, people, AI concepts) can be compiled with their inference mechanisms; their instances, evidence and revisions stay in the global graph. Compiled does not mean eternally true: updating a seed, a Rust rule or a hard parameter is normal learning work, done with a versioned rebuild or retraining and regression evidence.

**The global graph** is the durable base. Its target scope is knowledge from people, documents and datasets, and conclusions derived by Mova and other AI; this is an architectural target, not a claim about the repository today. Each record keeps claim identity, source / document / edition / time scope, conditions, confidence or review status, revisions and contradiction or supersession links. An AI-derived claim also keeps its premises, derivation rule and model / skeleton version. Recording an assertion is not treating it as verified. Rival theories keep their named assumptions; later input never silently overwrites earlier evidence.

**The important graph** holds in RAM what the system needs for its work. Activating or compiling knowledge never removes its global record, and eviction from RAM never deletes it. Tentative reasoning may use working memory; accepted conclusions get durable global records. RAM copies need refresh and version checks when records or rules change; derived results whose premises change are invalidated or revalidated, keeping their history. Controversy, confidence, operational importance and abstraction are independent: a disputed theory can be durable without being in RAM, and becomes resident when useful.

**Placement policy.** `global::placement` returns independent decisions for an admitted knowledge candidate:

- always keep its canonical global record;
- make it eligible for the important graph when operationally important;
- propose a skeleton update for a reviewed general mechanism or foundation, still keeping its global record (never for instance-specific or contested claims; a theory's reusable operators may still be implemented while its disputed propositions stay explicit knowledge);
- keep alternatives when contested, independently of RAM residency.

The decisions are not exclusive. The finite model (three scopes × four Boolean attributes) is covered by tests for durable retention, nonexclusive compilation and residency, independent conflict metadata and instance scope.

**A model for future RAM selection.** For records i with expected use p_i, avoided fetch cost c_i, refresh cost r_i, resident size m_i and residency decision x_i under RAM budget M:

    maximize sum_i x_i * (p_i * c_i - r_i)
    subject to sum_i x_i * m_i <= M.

An initial surrogate, to be refined by inference dependencies and real tasks; no cache scheduler implements it yet.

**Implementation boundary.** The Rust recommendation API and this contract exist. The retained school state stores a course graph and role weights, without a global knowledge engine or a RAM residency manager. There is no automatic semantic classifier, all-knowledge ingestion or skeleton generator yet. Learning must record new knowledge and accepted derivations, activate useful knowledge, and separately propose slower mechanism changes; examinations read the resulting state without replaying lessons (see [school.md](school.md)).

## The absurdity matrix

A graded table of how plausible an event is: `0` normal, `1` slightly odd, `2` odd, `3` very strange, `4` absurd. The base is verb × noun cells with a score for the noun as subject (SUBJ) and as object (OBJ). Scores come from the teacher in batches (`$MOVA_DATA/runs/absurd-roles`) and are compiled by `build.rs` into level-1 knowledge in `global`, with integer ids and the source noted per cell. have/be/do are excluded on purpose.

One table, many consumers:

| consumer | use |
|---|---|
| parse error search | `world sense-check`, `world absurd-check`: an absurd tree is most likely a wrong tree |
| repair | `world absurd-repair` and `world::rerank::repairs`: the label repairs the matrix points at |
| rule induction | matrix scores are features of `world induce` and the judge of self-play on big texts ([training.md](training.md)) |
| irony and jokes | absurdity the text itself marks (dialogue, «laughed», a fairy-tale frame) is a genre signal, not an error |
| QA and generation | reject absurd generated questions and distractors |

### Modes

- **Debug mode** (parser and reader development): absurdity is the error finder. An absurd tree is first a suspected parse error, and the matrix is gold.
- **Working mode** (reading real text): the parse is trusted. Absurdity that survives the domain layer is a signal that the text may be joking or ironic: it switches on the humor / irony domain (see [pragmatics.md](pragmatics.md#expressions-by-their-real-meaning)), not a correction. The humor domain needs no absurdity layer of its own: absurdity is its trigger, not its knowledge.

### Domain scales

A cell is `(event, domain) → score`. A talking fox scores 3 in the real world and 0 in a fairy tale.

- `real` — the everyday world, the base layer;
- `tale` — fairy tales and fables (talking animals, magic objects, giants);
- other domains (`news`, `science`, `myth`, `children`, `technical` …) are added when a consumer needs them.

Domain layers are rated as **deltas** over `real`: only cells with a real score ≥ 2 are rated again with a domain prompt. The domain of a text comes from the lenses (fairy-tale frame, genre metadata). An event absurd even in its own domain is the strong signal (error, joke or irony); an event absurd only in `real` is the genre signal. The tale layer covers only part of the verbs so far.

### Error search

`world absurd-check <scores.json> <in.conllu> [<gold.conllu>] --out <dir>` logs every judgment (plausible and absurd, with the cell used) to `judgments.jsonl` and what the matrix does not know to `gaps.jsonl`. It is deterministic. The parse-error rate at the judged noun rises with the score, so the score is usable for error search; on gold tale trees most absurd judgments are talking animals, which the `tale` layer removes.

`world absurd-events <events.tsv> [--domain tale]` ranks corpus events (from `world events`, see [pragmatics.md](pragmatics.md#event-statistics-of-books)) by count among those the matrix finds absurd: frequent ones are systematic errors. What remains after the repairs is mostly not label errors: idioms and strict cells, attachment errors a label repair cannot fix («a man called Robert»), and sentence-split errors. These belong in the parser's training data, not in repairs.

### Repairs

`world absurd-repair <dev> <test>` chooses matrix-driven label repairs on dev. The repairs in use (`world::rerank::repairs`, wired into reader trees and `world events`):

- **inverted subject** — «'…,' said the cat», «then came a knight», «there lived a king»: obj → nsubj when the finite verb takes no object (matrix intransitivity, `global::absurd_intransitive`) or the cell is absurd as an object, the noun is a normal doer in the real or tale layer, there is no subject (an expletive does not block) and no xcomp/ccomp after the noun; names after speech verbs («cried Tom»); «quoth» as a speech verb;
- **bare oblique** — «went a long way», «waited a while»: obj → obl:unmarked for `measure_time` nouns after verbs that do not take them;
- **archaic adverbs** (whence, hither … read as nouns → ADV advmod; `archaic_adverbs` seed, two passes so that «Whence came this stranger?» ends with stranger as the subject);
- «there» read as a subject → expl.

The main value is cheaper development: the matrix points at the errors and says which fix fits. The rules learned by `world induce` find the inverted subject by themselves and do better.

### Idioms, light verbs, phrasal verbs

`world idioms <cells.tsv> <events.tsv> <wikt.tsv> --out <dir>` joins the Wiktionary expression base (`tools/wikt-idioms`) with corpus statistics — PMI of verb and object over dependency arcs (syntax-aware PMI after Bogdanova 2026; association over frequency, Dunn 2019) — and the matrix (literal reading odd or absurd). The result `idioms.tsv` (CC BY-SA, out of the free release) lists idioms (make one's way, pay attention), light verbs (take place, make an effort), collocations (tell a story) and phrasal verbs (give up, carry out). Corpus-only pairs are `candidate` and never suppress an alarm (the negative control caught «eat mouse» as a corpus «idiom»). In the matrix a dictionary-confirmed idiom, light verb or collocation is not literally absurd as an object, and with an idiomatic object the doer is not judged («the marriage took place»).

### Growth and limits

The matrix grows in steps, each a separate run that must pay off before the next:

| step | state |
|---|---|
| roles: verb × noun → SUBJ and OBJ scores | done for the current verb and noun lists |
| compile into `global`; graded `sense-check` | done |
| reranking: the parser's k best trees → the least absurd | measured, no gain |
| tale layer for cells with a real score ≥ 2 | part of the verbs |
| pairs: subject × verb × object for key verbs among animate nouns | not started |
| width: more nouns, verbs, prepositions, adjectives (noun × property) | not started |
| classes: unseen words scored through their class; the teacher only for disagreements | not started |
| humor / irony: a contrast set of jokes vs literal sentences measures the trigger | not started |

**Reranking gives no gain.** `world absurd-rerank <dev> <test> [--k 8]` scores the parser's k best trees (`en` `parse_kbest`) by `score − λ·penalty`. The beam's alternatives differ in structure, but the absurd errors are labels (an object for an inverted subject, an object for a bare oblique) that the beam never proposes, since it keeps one best label per arc. Coverage of verbs and nouns is the other main limit.

## The nerve graph

A design that ties words, code, worlds and the system's own events together. The implemented parts are named by their files; the rest is direction.

**Triggers — words hooked to code.** An English word is hooked to a trigger, a compiled Rust function of the skeleton (`world/src/triggers.rs`): «minute» fires the duration trigger, which knows a minute is sixty seconds and counts with it. The world of the trigger's topic is its explanation: the world's sentences that use the trigger's words, with their sources. The concept lives in the code; the world explains it.

**Events of the self activate words**, and the words call the events up (`<hot>/activations.tsv`, with time and duration): a chat message → message, chat; a training round → training, learning; a world gathered → world, knowledge. So the words of time («a minute», «just now») are tied to what the system lived through.

**The graph.** Nodes are lemmas and concepts (the triggers' words, the events, the parts and sentences of the worlds). Edges are a trigger's hooks, an event's words, words that share a world sentence, a sentence and its source. Activation spreads deterministically: a fixed number of steps, integer weights, ordered maps only.

| part | status |
|---|---|
| triggers of time (duration, cycles, clock, calendar); events of the self and their recall; the perception report (`world worldbook-perceive`) | implemented |
| the nerve graph from triggers, self events and the central manuals' sentences; spreading on a text; picking the world sentences that explain a trigger (`triggers::explain`, word overlap only as fallback) — `world/src/nerves.rs`; `world nerves "<text>"`, `world nerves --stats` (size and hash) | implemented |
| answering likely questions ahead between rounds (`know::anticipate`, `<hot>/anticipated.tsv`; the chat looks there first) | implemented |
| a rejected idea kept as a dormant hypothesis with the circumstances it failed in (`family.rs`, the negative list) | implemented |
| the graph changing in training: new nodes from gathered worlds, edges strengthened or cut by the guard sets | design |
| caches keyed by graph hash and input hash; often-walked paths compiled into shortcuts | design |
| finding a principle of the gathered worlds that matches a new situation by structure, and applying it | design |

**Fixed at work, alive in training.** The graph of a state is compiled with the state and does not change while it works; only training changes it. It is a function: the same input gives the same output whatever the hour, the load or the operating system. «Now» is an input; only ordered maps and sorted listings; threads combine in a fixed order; fixed-point integers, no platform `exp`/`ln`/`pow`; normalized text (NFC, line endings, `/` in paths). The gate runs the same input twice, also under load, and compares hashes; a planted `HashMap` order must turn it red. CI runs on Linux only.

## RAG: documents expanded into layers

Retrieval-augmented generation needs documents that answer more than their own text. Mova expands a document into a core wrapped in layers, each answering questions the core does not:

| layer | weather in a town on a date | text document (e.g. a developer diary) |
| --- | --- | --- |
| core | today's weather | the text itself |
| neighbors | towns and villages around | related entities and documents with shared terms |
| past | this date in past years, normals, records | earlier mentions, previous revisions |
| future | forecast | what is planned, consequences |
| mechanism | fronts, pressure centers | why: causal links |
| names | the town's spellings, airport and station codes | names, abbreviations, alternative spellings |

**What Mova provides without AI**, for English on standard annotation (`en`: tagger, in-code dictionary, parser), each as a separate zone or feature:

1. **POS-aware lemmas** (built → build, markets → market): a shared root without the noise of character n-grams.
2. **Phrases from the tree**: noun groups with compound and amod relations become terms (trade route, estate privileges).
3. **Neighbors**: documents that share key lemmas and phrases, presented as an index map, not as facts.
4. **Rule-based name shadow**: spelling variants, hyphens, abbreviations (TT-2 / TT2 / Tinto Talks 2) — the part of the shadow derivable without AI.
5. **Query analysis**: lemmas, phrases and parts of speech of English queries, without an AI rewriter.

A zone is enabled only when the bench shows a gain. The bench uses the same documents for every zone; a new zone must not worsen quote queries; paraphrased quote queries are made by Mova's own morphology, so the right answer is known in advance; and a lemma zone shuffled across documents must score worse (the negative control).

**Limits.** There is no RAG crate in the public repository: the zones are a design built from `en`'s parts. The retrieval bench, its documents (third-party diaries) and live bot queries are internal and not published; the live queries are too few for statistics. English only.
