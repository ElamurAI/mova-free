# Smart annotator — layers, context, memory

Design note:

> What the annotator must be able to do is hold context and pronouns. It needs an internal memory from which it infers the context, what pronouns refer to, and other things that follow from context.

Design note: the annotator can, in parallel with the tree, also output the structure of pragmatics, i.e. many layers over the same data. That makes it no longer a simple annotator but a smart one.

## Multi-pass parsing

Design note: an SLM parse can work like this — a first parse; pragmatics are built from that parse; then a reparse with the new context, because the context now contains pragmatics.

1. **First pass** — each sentence on its own: morphology and syntax without context.
2. **Upper layers** — built from the first parse: entities and coreference, discourse, pragmatics, the document passport, domain frames, the story layer. They go into memory, i.e. into the md graph.
3. **Reparse with context.** The lower layers are parsed a second time, now with what has appeared in memory:
   - in conversation *like* is a discourse word, not a verb;
   - in law *shall* is an obligation;
   - a request phrased as a question is a different sentence type;
   - a known entity suggests where to attach a prepositional phrase;
   - the weather domain suggests that "40%" is a probability.
4. **Repeat until the parse is stable**, but no more than 2–3 passes. What the second pass changed is visible in the log: that is exactly where context mattered.

**How the SLM learns this.** The Rust tagger and parser receive context features: genre, sentence type and act, entity type, domain, previous sentences. They learn on GUM, where all layers are already in the gold standard, and on silver data. In the first pass these features are empty; in the second they are filled from memory.

**Measurement and control:**
- gain of the second pass over the first — on GUM and GENTLE;
- negative control: foreign or shuffled context must not help, it must hurt. Otherwise the "gain from context" is fake.

**Measurement (26.09)** — experiment, GUM is NC; details in `en/README.md`, section "Document context in parsing". Setup: GUM test, 3 seeds, 95% paired bootstrap.
- **Ceiling.** Gold context gives LAS +0.77 [+0.37; +1.18] and MLAS +1.18 [+0.68; +1.65].
- **What carries it.** GUM mention boundaries: the "token is inside a mention" flag alone gives +0.75 LAS. Genre, sentence type, paragraph and speaker do not move LAS; sentence type gives only UPOS +0.1. EWT genre for the working model is also zero (LAS −0.05).
- **Controls.** Foreign context hurts (−4.5 LAS). A model trained on shuffled context does not beat the baseline (−0.9).
- **A real second pass.** Context: genre from the passport, sentence type by a rule over the first pass, "this noun lemma has already occurred in the document"; the second-pass model is trained by jackknifing. LAS −0.06 [−0.37; +0.27]; only UPOS +0.18 and UFeats +0.12 are significant.
- **Next step.** Mention boundaries from another source — the LLM or a mention detector.

**With the LLM.** At doubtful spots the SLM asks Opus about pragmatics or domain ("is this a request or a question?") and reparses with the answer. Opus writes a few tokens; the reparse is done by Rust.

## Small model and Opus: division of labour

Design note: the SLM itself does not need to be a genius. It can be a fairly primitive robot that holds context, annotates the input well, and can formulate a very sensible prompt to Opus for hard reasoning.

The SLM is a small language model: our own Rust, deterministic, running on the CPU. The name pairs with LLM, the large language model, i.e. Opus. It is not a genius but a reliable robot with three duties:
1. **Hold context:** the memory md graph, the document passport, entities, events.
2. **Annotate the input well:** all layers that can be produced deterministically — morphology, syntax, pragmatic cues, domain frames.
3. **Formulate a sensible prompt to Opus** for hard reasoning:
   - a precise question;
   - exactly the context that is needed — an extract from the md graph with anchors, not the whole document;
   - the annotated input;
   - the answer format.

**Opus** does the hard reasoning: implicatures, motives, summaries, difficult parses. It answers with a structure with anchors. The SLM checks the answer with gates and writes it into memory.

**When the SLM calls Opus:**
- when it is unsure: two paths to the goal diverged, seed rules fire warnings, the score is low;
- when the question requires understanding: motive, irony, summary.
Everything else it does itself.

**Why this way:**
- each does what it is good at;
- Opus gets a short, precise prompt, so fewer tokens and better reasoning;
- the SLM is cheap and transparent: its memory, rules and decisions are visible.

The `annot` edit mode already works this way: Rust draft → Opus edits → gates.

**Not a distillate but a student.** Design note: the SLM effectively turns out to be not even a distillate of the LLM but its student. A distillate only copies the teacher's answers. A student does more:
- learns from its own sources — grammars, papers and dictionaries (seeds, rules);
- receives corrections from the teacher (edit mode) and does not memorize everything indiscriminately (examples are not truth);
- decides by itself when to ask the teacher, and knows how to ask sensibly;
- has what the teacher lacks: memory across sessions, determinism, CPU speed, transparent rules.

### Transparent judgments — our signature

Design note: transparent judgments — deterministic Rust and simple trainable models whose judgments are plainly visible — are our signature feature.

The LLM gives an answer without a "why". In the SLM every decision leaves a trace:
- **tag:** the perceptron features that tipped the balance, with weights;
- **FEATS:** the table key that fired, the back-off level and the counts. For a verb — the subject that was found;
- **relation:** the parser step, its score and the main features;
- **rules:** a seed with its source (book, page) and an engine rule with an id;
- **converter:** a rule with an id for every changed token;
- **gates:** what failed and why.

This is a general principle the SLM is already being built on, not a separate task for a detailed judgment log. New SLM models are simple and explainable: tables, perceptrons, rules.

### The large model commands the small one

Design note: in addition, the large model gives commands to the small one: "prepare this and that over there", or "for reasoning I also need the first and the second, so resend the request with more info".

An LLM response comes in three kinds:
- **answer** — a structure with anchors; the SLM checks it with gates;
- **`need:`** — data is missing for the reasoning. The LLM lists commands for the SLM; the SLM executes them and sends the request again: the same question plus the added data with anchors;
- **`prepare:`** — an advance assignment: "prepare the passport of chapter 3", "annotate the entities in letters 5–9". The SLM does it cheaply, and subsequent requests already have it ready.

**SLM commands** — deterministic, over memory and annotation:

| command | what it returns |
|---|---|
| `sentences s10..s20` | original sentences with all annotation layers |
| `entity e3` | entity: mentions, what is known, anchors |
| `events who=e3` | events involving e3: who, what, when, why |
| `summary chapter 2` | a level summary with anchors |
| `passport` | the document passport |
| `domain s5 weather` | the sentence's domain frames |
| `search "…"` | where in the document this or something similar is said |

**Limits:**
- no more than three rounds of "`need:` → follow-up request" per question, then stop with a report (fail-fast);
- a token budget per question;
- every added item comes with an anchor, so the LLM cites the source instead of inventing.

This is like tool calling in modern LLMs. Only the tools are our cheap deterministic Rust over our own memory.

**Generative commands.** Design note: the large model can simply tell the small one: "output today's weather in Vinnytsia and add historical charts" — for Opus that is a handful of tokens and seconds, not 40–60 as before.

Rule: **the LLM writes decisions and commands; the SLM expands them** into text, tables and charts with its deterministic generators (like `weather`). Output tokens are the LLM's main cost and time, so the shorter the large model's answer, the cheaper. We have already measured this on annotation:
- Opus writes the annotation itself (two-pass high): ~1.7k output tokens and ~13 s per sentence;
- Opus writes only edits, Rust applies them (edit mode): ~530 tokens and ~4.7 s, and the quality is higher.

The weather in Vinnytsia is an example of the principle, not a task: domain generators are the business of a weather module and the domain modules that follow.

## SLM modules: motivation and working memory

Design note: the SLM itself should have, in Rust, a motivation module that will be a prototype of metaconsciousness, as in an LLM. Another module is for context and pragmatics — not just as a graph, but literally as a set of variables: who, how, why, a small array of main characters, secondary ones, and so on. So the SLM can actually hold the value itself in the array plus a reference to the graph.

### Working memory — variables with references to the graph

The memory md graph is the "heap": everything that is known. Working memory is the "registers": what is in focus right now. Typed Rust variables, each holding a **value and a reference** to a graph node with anchors to sentences:

```rust
struct Ref<T> { value: T, node: NodeId, anchors: Vec<SentId> }   // value + where in the graph + where from

struct Context {
    main: Vec<Ref<Character>>,        // main characters (by attention weight)
    secondary: Vec<Ref<Character>>,   // secondary characters
    speaker: Option<Ref<EntityId>>,   // who is speaking now
    addressee: Option<Ref<EntityId>>,
    who: Option<Ref<EntityId>>,       // who the current sentence is about
    what: Option<Ref<EventId>>,       // current event
    why: Option<Ref<Goal>>,           // motive / goal
    how: Option<Ref<Manner>>,
    place: Option<Ref<Place>>,
    time: Option<Ref<TimeAnchor>>,
    topic: Option<Ref<Topic>>,
    act: Option<Ref<Act>>,            // speech act of the current utterance
    domain: Domain,                   // from the passport
}
```

- Variables are updated sentence by sentence:
  - a new character goes into `main` or `secondary` by attention weight;
  - a pronoun reads `who` or `speaker`;
  - "in order to" fills `why`.
- The reference to the graph gives transparency: every value can be expanded to the sentences it came from.
- For the LLM this is exactly "just the needed context": the SLM sends the variables, not the whole document.

### Tale world: entities, registries, cursor

Design note: the whole tale is a set of entities. The tale begins: "In the field there is an oak." The main cursor creates location1 of type field and adds the location to the global list of locations. To the global list of objects we add object1 of type tree, and set the tree's class to oak. In the list of objects at location1 we put object1, and in object1's "location" parameter we put location1.

**World state:**
- global registries: locations, objects, characters, events;
- every entity has a type, a class and parameters;
- links are bidirectional: `L1.objects ∋ O1` ⇔ `O1.at = L1`;
- **the main cursor** — where and when the narrative is right now; it moves along with it. It holds `at` (location id) and the position in the text: chapter, paragraph, sentence. Design note: the cursor holds the location id, Chapter1, paragraph 1;
- **mentions:** each entity has a list of mentions, append-only. When a sentence mentions location1 and object1, their lists receive the cursor's current position (chapter, paragraph, sentence). From this come the first and last mention, and where in the text the entity appeared.

Working memory (`Context` above) is a view of the world from the cursor: who is here, who is main, what is happening.

**Minimal objects.** Design note: a character can have attributes: gender (may even be unknown — like other parameters, unknown by default), mother id, father id, age, romantic relationships (only an id; the details are told in that relationship), friendships, a set of acquaintances, a set of adversaries, and so on. The sets are described in separate classes, and the character is just a set of references and simple attributes.

```rust
enum Val<T> { Unknown, Known(T) }                       // unknown by default

struct Character {
    id: CharId, name: Val<String>, kind: Val<Kind>,     // human, animal, magical being…
    gender: Val<Gender>, age: Val<Age>,
    mother: Val<CharId>, father: Val<CharId>,
    relations: Vec<RelId>,                              // romantic, friends, acquaintances, adversaries — separate objects
    at: LocId, mentions: Vec<TextPos>,
}

struct Relation {                                       // relationship details live here, not in the character
    id: RelId, kind: RelKind,                           // Romantic | Friend | Acquaintance | Adversary | Kin(..)
    a: CharId, b: CharId,
    facts: Vec<Ref<Fact>>,                              // how they met, what happened between them — with anchors
}
```

**State changes over time.** Open question: how to elegantly describe a transition between an object's states — that in the first paragraph it had two eyes and at the end of the book one — is not yet clear.

Mova proposes **history instead of a value**:
- an attribute is a list of changes;
- each change: the new value, from which point in the text it applies, the causing event, anchors;
- the state at any moment is the last value before that point.

```rust
struct Attr<T> { history: Vec<Change<T>> }
struct Change<T> { value: Val<T>, from: TextPos, cause: Option<EventId>, anchors: Vec<SentId> }
// eyes: [(2, ch.1 para.1, —, ^s3), (1, ch.12 para.4, E57 "lost an eye in battle", ^s912)]
// eyes.at(ch.5) = 2; eyes.at(end) = 1; "why one?" → E57 → ^s912
```

**Immutable states — the alternative variant.** Design note: or the character is a singleton, and any change in it creates a new copy, so the reference goes to character1state1. Example: "Jackie was a handsome boy." Jackie state1: name — Jackie; state2: name Jackie, gender male, age boy; state3: name Jackie, gender male, age boy, beauty — handsome.

Like commits in git: the character is an identity (a branch), each change is a new immutable state (a commit). References in events, mentions and relationships point to a specific state.

```
C1@1  { name: Jackie }                                 ^s1  "Jackie"
C1@2  C1@1 + { gender: male, age: boy }                ^s1  "was a boy"
C1@3  C1@2 + { beauty: handsome }                      ^s1  "handsome"
…
C1@9  C1@8 + { eyes: 1 }   cause: E57 "lost an eye"     ^s912
```

- **Internally** a state is the previous state plus the changed fields, the cause and the anchor. No full copies; memory is shared (persistent structure). **Externally** each `C1@n` is a full snapshot.
- Both views are equivalent: an attribute's history (`Attr<T>` above) is a walk along the chain of states for one field.
- **What it gives:**
  - "what was Jackie like when he met Maria?" → the meeting event references `C1@5`;
  - the difference between any two states is visible immediately;
  - a state can be cited, like a commit.

**State rules:**
- **No silent changes.** A value changes only through an event, just as location changes only through `move`.
- **A contradiction is a signal.** If later the text again says "with both eyes" with no healing event, that is a continuity error. Also useful for checking texts.
- **Two times:** position in the text (where it is told) and story time (when it happened) — for flashbacks and flash-forwards.

**World-change language**, with an anchor to the sentence:

```
cursor chapter=1 para=1 sent=1                          at the start: Chapter1, paragraph 1
loc  L1 type=field                               ^s1   create location → location registry; L1.mentions += (1,1,1); cursor.at = L1
obj  O1 type=tree class=oak at=L1                ^s1   object registry; L1.objects += O1; O1.at = L1; O1.mentions += (1,1,1)
char C1 type=animal class=fox name="Fox" at=L1   ^s2   character registry; L1.chars += C1
move C1 to=L2                                    ^s5   L1.chars −= C1; L2.chars += C1; C1.at = L2
have C1 O3                                       ^s6   C1.items += O3; O3.at = C1
say  C1 to=C2 act=request                        ^s7   utterance from pragmatics
event E1 verb=eat agent=C1 patient=O4 at=L2 why=hunger   ^s8
cursor at=L2 time=T3                             ^s9   where the narrative moved
```

- **Who writes the changes.** First the LLM: a few tokens of commands per sentence, on the principle "the LLM writes commands, the SLM expands them". Later the SLM learns to write them itself from the UD tree, entities and context.
- **Who executes.** The SLM in Rust maintains the state with gates:
  - references only to existing entities;
  - bidirectional links are consistent;
  - types from closed sets;
  - no teleportation: a change of place only through `move`.
- **What it gives:**
  - answers to "where is the oak?", "what does the fox have?", "who was in the field when…?" — directly from the state and the change history;
  - the five-year-old test;
  - the md graph as a human view of the same state.

### Universality of the world

Design note: the system for describing a virtual world must be universal: one time it describes a tree, another time a news item, or it parses a Wiki article.

The core is the same for any text:
- entities with attributes and state history;
- relationships;
- events on the story timeline (partial order);
- claims of observers or sources about events and facts, with a status;
- narrative frames, reality branches, knowledge;
- anchors to sentences.

Genre and domain add only a **profile** — types of entities, attributes, events and a command vocabulary:

| text | what is in the world | observers |
|---|---|---|
| **description** (tree, place, device) | a static world: object, parts, material, size, place; almost no events | the author |
| **tale, novella** | characters, states, events, motives, frames | narrator, characters, reader |
| **news** | events and participants on a timeline | sources: "the ministry stated", "according to the police" — the same `claim` as for Watson and Holmes |
| **Wiki article** | facts with references, time-dependent attributes ("population in 2020"), source disagreements | the article's sources; checking — as in "Checking claims against the world" |

For RAG this means: any document becomes a world, and a question becomes a query to the world.

**Check:** after v3 — the same engine on three texts: a description, a news item, a Wiki article (CC BY-SA text — parsing and checking only, no training).

### Scale and zoom

Design note: looking abstractly at a country, from above you see thousands of the most important facts; zooming in, billions.

- **Scale levels.** Every entity and fact has its own level: world, country, region, city, street, person, object. For a narrative the same: corpus, book, chapter, paragraph, sentence.
- **Weight.** At each level — the most important. Weight comes from how many facts rest on this one, how many sources confirm it, and how recent it is. For a narrative — the attention weight of entities (GUM `salientEntities`).
- **Summaries with references downward.** "Population of the country", "the 2022 war" — summaries with thousands of smaller facts beneath them. Gate: a summary is consistent with its children (sum of regional populations). Zoom is a walk downward, as in the md graph from a chapter summary to sentences.
- **Lazy loading.** Billions of facts live in the database. The in-memory world holds only what is in focus — like the SLM's working memory.
- **Query:** focus (entity), level and budget N → the N most important facts of that level plus entry points into depth.
- **Geography** — from an external geography hierarchy (countries, regions, agglomerations), read-only. We do not build our own.
- **For RAG** — an answer at the right level: "what is important about Ukraine?" — from the top; "what was the weather in Vinnytsia on 25.09?" — from the depths.

### History and reconstructions

Design note: history through written history, and later there will be a project of economic, demographic, climatic and other reconstructions, so even previously described history will be read through the different prism of reconstructions.

- **Written history:** chronicles, documents, articles — claims of sources, each with its own position and bias.
- **Reconstructions:** economy, demography, climate — also observers, but with numbers and error: time series with confidence bounds. They attach to the same canonical events and entities.
- **Prism** — the same past read through another layer. The chronicle "famine of 1601" + a climate reconstruction (cold summer after an eruption) + a demographic one (population decline) = a causal chain across layers. The past is not rewritten: causes, confirmations or contradictions are added.
- **Rereading by epochs:** a new layer → another pass over the corpus until the world becomes consistent.
- **Layers come from separate projects, read-only:** climate — a weather module, countries and agglomerations — an external geography project. Economy is a separate domain: "50 shades of economy" — the seed of a future investigator of the grey and black economy. Mova provides the world into which these reconstructions fit as layers.

### Thousands of domains

Design note: this is only one project or domain, and there will be thousands of different domains.

So a domain profile must be cheap:
- **A shared core.** Entity, event, claim and source, time, place, money, organization, person, document — the same for all domains.
- **A profile is data, not code.** Like seeds: types, attributes, events, a command vocabulary, domain gates. A new domain is a new file; the engine does not change.
- **The LLM writes a draft profile** from a few texts of the domain; the SLM checks it with gates on examples. A domain in hours, not weeks.
- **Inheritance and composition:** shadow economy ⊂ finance ⊂ economy; weather ⊂ climate. Only the difference is new.
- **A profile registry** with versions and checks. Domains do not contradict the core, because the shared gates are the same.
- **A domain is a separate project:** the economy investigator, a weather module, an external geography project… Mova provides the core and the profile tooling; the siblings build domain versions.
- **Profile access level** (design note: not all of them will be public; not all will even go into Enterprise):
  - `public` — into open Mova;
  - `extended` — into Mova Extended under licence;
  - `private` — internal only, never outside, not even in Enterprise.
  - The level is recorded in the profile itself. **The release gate** does not let a profile, data or model of a higher level into a lower one, like the data-licence gate. Models trained on private-domain data are private.
- **The first domain as a world model — HS22** (27.09, `onto hs`). The Elamur goods classifier (5,609 subheadings, external trade data) became a taxonomy:
  - a tree section → heading → subheading, "other" separately;
  - features: state, processing, material or source, form, purpose, parts, taxon; thresholds with units — exactly, via `math`;
  - mapping to the core and the graph: head concept — 93.7%, with a Q-id — 45%;
  - gates for IsA and feature contradictions, new candidate concepts;
  - blind Opus judge — 3.61 of 5, decoys — 1.00.

  WCO texts do not go into the repo, only derivatives. Details — `onto/README.md`, "HS: a world model of goods".

**Life cycle of a domain project.** Design note: the weather project takes Mova, copies it, builds itself on top of Mova but as its own personality, expertise and data work. Then it generates an expert snapshot for Mova's linguistic retraining, a separate dataset for basic Enterprise, and an extended expert weather system-model for separate Enterprise requests or partners, for example various weather services.

1. **Copy:** Mova's core — code, models, seeds, the world engine — with a version tag.
2. **Personality:** its own expertise, data, profiles, lore.
3. **Three outputs:**
   - **expert snapshot → Mova.** The language of the domain: terms, multiword expressions, style (telegraphic forecasts "Mostly sunny, high near 72"), seeds, an annotated corpus, a world profile. Mova retrains and becomes stronger for all domains;
   - **dataset → basic Enterprise:** curated, with a commercial licence;
   - **extended expert system-model of the domain → Enterprise and partners** (weather services etc.), under separate agreements.
   - **a public free domain "chatterbox" → for everyone.** Design note: for everyone, free — a dense little chatterbox model that talks well about the weather and knows a bit of maths. SLM only, no LLM: free for everyone must also be free for us. Composition:
     - understanding the question — tree, pragmatics, domain;
     - a weather world from open data (NWS — public domain, ECCC — open licence);
     - good language — the `weather` generator;
     - a little maths, deterministically: units, averages, probabilities;
     - a reference to the data and the rule in every answer.
     Small, dense, on the CPU, in milliseconds.

**Exchange rules:**
- **A snapshot is data, not code.** Every piece has a source, licence, access level (`public` / `extended` / `private`), version.
- **Gates at the entrance to Mova:** licences (NC not into commercial), levels (private not into public), quality (seeds and corpus pass the same checks as our own).
- **Versions on both sides:** the domain project knows which Mova version it is on; Mova knows whose snapshots and which versions it includes. Rolling back a snapshot does not break the rest.

### Checking claims against the world

Design note: a wiki article about a tale can have access to the tale world, to fact-check Wikipedia.

1. An external text about the source (a Wikipedia retelling, a summary, a review, **a RAG answer**) is parsed into claims: who, what, where, why.
2. Each claim becomes a query to the world: states, history, events, motives.
3. A verdict with anchors to sentences of the original:
   - **confirmed;**
   - **contradicts** — in Wikipedia the Crow holds cheese, in the Townsend translation meat; the difference is visible together with the source;
   - **not in the original** — the retelling added it.

**Applications:**
- **grounding RAG answers:** the large model's answer is checked against the document's world, and whatever it does not rest on is visible at once. For RAG this is the main thing;
- checking summaries, including our own paper summaries;
- variants of translations and editions — a difference of worlds.

Wikipedia texts under CC BY-SA are only checked against; we do not train on them.

### Non-linear narrative

Design note: the narrative model will need improving for when a story does not run linearly.

In fables the order of the text matches the order of events. In novellas it does not: flashbacks, flash-forwards, parallel lines, stories within stories, dreams, lies, plans. Narratology distinguishes the **fabula** (what happened and in what order) and the **syuzhet** (in what order it is told) — Genette: order, duration, frequency.

**What we add to the world:**
1. **Two time axes.**
   - Position in the text is the cursor.
   - Story time is a separate axis. Events are placed on it relative to each other (before, after, during — Allen intervals, as in TimeML), not by sentence order.
   - Character states hold over intervals of story time.
   - Signals: the pluperfect *had done* (earlier in the story — a flashback), *years ago*, *before*, *after*, *when*, tense forms from FEATS, discourse relations.
2. **Narrative levels — a stack of frames.** "The old man began his story…" opens an embedded story with its own narrator and possibly its own world. The cursor holds a stack: who is narrating, at what level.
3. **Reality status — possible worlds.** Every event has a flag:
   - really happened;
   - dream;
   - plan or intention;
   - a character's belief;
   - lie;
   - "what if";
   - reported speech.
   Only events that really happened change the actual world. The rest live in branches: the Crow's belief world, the Fox's plan.
4. **Who knows what and when.** The knowledge of each character and of the reader is kept apart from the truth of the world. The solution of a mystery in a detective story is a belief revision, not a change of the past.
5. **Parallel lines.** Several cursors, one per line, aligned on a shared story-time axis.
6. **Gates — in story time.** A flashback that takes a character to an old place is not teleportation: the gate checks places on the story-time axis, not by sentence order.

**Where to test:**
- "One Thousand and One Nights" — stories within stories;
- "The Canterbury Tales" — a frame and narrators;
- Conan Doyle's detective stories — uncovering the past and characters' knowledge.

All of this is in the public domain. Temporal relations — TimeML and MATRES papers in the library; data licences to be checked.

**Event identity independent of its descriptions** (from a design discussion with ChatGPT, 26.09). One event — one `event_id`, but there can be many descriptions and interpretations:

```
event E7 (world truth: murder)
  claim  reader   E7 = unknown          ^s12
  claim  Watson   E7 = accidental death ^s40
  claim  Holmes   E7 = suspected murder ^s88
  reveal "in fact he died two hours before Watson arrived" ^s512
        → E7 BEFORE E9 (Watson's arrival); Watson's claim → false
```

The solution does not create a new "murder in the past". It adds a confirmed temporal relation to the existing event and makes the earlier interpretation false.

**Pipeline v3:**

```
text order → sentence cursor
  → narrative frame (who narrates, level)
  → event mention or claim
  → resolve to a canonical event or create a new one
  → place on the story-time axis
  → reality branch: happened, dream, plan, lie, "what if"
  → update the knowledge of the specific observer (character or reader)
  → only then do the gates decide whether the canonical world changes
```

**Time is a partial order, not a timestamp.** Relations `A BEFORE B`, `C DURING B`, `D AFTER A`, `E ≈ C`. Absolute time only when the text gives it: prose says "earlier", "three days later", "that evening", not a date.

**The teleportation gate** compares a character's adjacent states on the story-time axis, not adjacent sentences. A flashback on page 500 can be the neighbour of an event from page 3.

**Scope of v3:** v2 proved that the world holds linear causality. v3 checks whether the world can separate what happened from what people think about it, and from the order in which it is told.

**Queue:** world v3, after v2 and the FairytaleQA measurement. An SLM writer of world changes is the next step, v4.

### Hierarchical and global worlds, epochs

Design note: worlds must be hierarchical and global, i.e. each tale from "One Thousand and One Nights" at least knows about the previous ones from this training pass, i.e. epoch — and so on for many epochs until they agree.

**Hierarchy of worlds:**
- **the global corpus world:** Baghdad, Harun al-Rashid, Scheherazade — entities shared by many stories;
- **the collection or book world:** the frame — Scheherazade tells stories to Shahryar;
- **the world of a single tale;**
- **embedded worlds:** stories within stories, dreams, plans.

Each entity has a scope: global, collection, tale. Local entities are linked to global ones — this is cross-document coreference. Is "the king" in the fifth tale the same as in the second?

**Epochs — passes until agreement:**
1. **Epoch 1.** Tales one by one. Each sees the global world built from the previous ones.
2. **Epoch 2 and beyond.** Each tale again, now with the full global world from all tales:
   - relink entities — it turned out later that this is the same character;
   - fix early decisions;
   - recompute states.
3. **Stop** when the global world stops changing: the number of changes per epoch → 0, like the decline of loss in training. Or when the epoch limit is reached, with a report on what did not converge.

**Gates between tales:** a character who died in the third tale does not appear alive later. Unless it is another narrative level or another story time — hence the link with non-linear narrative.

**Cost.** The LLM in every epoch is expensive. So the LLM writes the changes in the first epoch, and afterwards the SLM relinks and checks. Only conflicts the SLM could not resolve go to the LLM.

**Testing ground:** "One Thousand and One Nights" (a public-domain translation) and Grimm's tales: recurring character types, places, motifs.

### Motivation module — a prototype of metaconsciousness

**Emulating dopamine without neurons** (design note, 27.09: a person can be explained as a causal-chain motivational link, an emulation of dopamine).
- **Expectation.** Every plan and step has an expectation: success, time, cost.
- **Prediction error.** After the result we compute δ = result − expectation. This is what dopamine signals in the brain (Schultz, Dayan, Montague 1997).
- **Credit along the chain.** δ distributes credit back along the causal chain of steps, like temporal-difference learning (Sutton & Barto) or production utility in ACT-R.
- **Signals:**
  - δ > 0 — strengthen;
  - δ < 0 — weaken and look for something else;
  - several δ < 0 in a row — "frustration" → ask the LLM;
  - **curiosity** — take tasks with the greatest learning progress (Oudeyer & Kaplan).
- **Transparency.** Every update is logged with its reason.
- **Dopamine for alternatives** (design note, 27.09: laziness and repetitiveness of thinking are real, but results will come when dopamine is added for alternative approaches):
  - an intrinsic bonus for an approach that differs from the usual one: another plan, operator, decomposition. Larger if the alternative succeeded, and non-zero if it yielded information;
  - balance with the known — UCB: the less-tried with high uncertainty gets a chance;
  - an archive of diverse successful solutions, like MAP-Elites, not just the first one found;
  - **two different plans that agree are a strong gate.** Agreement of two paths from the same plan is weak: in the coder on 64–78 tasks both paths made the same mistake.
  - Roots: novelty search (Lehman & Stanley), quality-diversity (Mouret & Clune), system 1 and system 2 (Kahneman).
- **Tree of cursors** (design note, 27.09: in essence these are beams, different cursors, a tree of cursors).
  - Solving is a tree of cursor states: a partial solution as `state@n`, branches are alternatives, beams are parallel candidates.
  - **UCT/MCTS search:** a node's value is the expectation, the bonus is for the unknown and novelty, the prediction error propagates up to the root.
  - **One abstraction for all of Mova:** immutable states, cursors, branches and evaluation. Already present in:
    - the tale world — states and branches;
    - `vfs` snapshots — a branch in ~20 ns;
    - the parser beam.

    The coder and maths become the next users — via a shared crate.

A model of itself: what the SLM is doing now, what it knows, what it does not know, and what to do about it.
- **Goal and task:** what we are annotating or answering now, for whom, in which mode (real time or batch).
- **Self-assessment:** confidence per layer and field, checked against measurement (calibration) — where the SLM is strong and where weak.
- **Choice of action:** answer itself, ask the LLM (`need:`), reparse with new context, mark as doubtful.
- **Curiosity — what to learn next:** fields with the lowest quality and the most frequent disagreements request new silver data exactly there.
- **Budget:** LLM tokens, time, memory — how much can be spent on a question.

Transparent, like everything in the SLM: every decision of the module has a reason, e.g. "asked the LLM because `act` confidence is 0.41 and the threshold is 0.6".

**Lore.** Design note: lore, as in tales, also goes into the system's motivation. The module holds not only thresholds but its own story — who the SLM is, where it comes from and why. Its commandments are the same rules that already work in the code, stitched into one narrative:
- **who:** the Little One, student of the Big One; keeps a notebook of truth about the world;
- **commandments:**
  - every word comes with an anchor to a sentence;
  - what is not in the notebook — I say "not there", I do not invent;
  - when unsure — I ask the teacher;
  - I save the teacher's time and tokens;
  - I show why I decided so;
- **path:** grammar seeds → Aesop's fables → "One Thousand and One Nights" → novellas where time does not run in order.

The lore lives as data, like seeds, and the module reads it about itself. The working mode of the builders is in `docs/mysticism/gali.md`.

**First testing ground — tales.** Arrays of main and secondary characters, `speaker` in dialogues, `why` from motives. A paragraph-level question-answering pilot provides the yardstick.

## SLM subsystems

Design note: later we will develop further subsystems, for example maths, so that the model can calculate a little.

### Mathematics

**Goal** (design note): the model should not be a complete idiot, but should calculate at the level of a university mathematics student.

- **Understanding:** "15% of 80", "72°F in Celsius", "the weekly average", "the probability of at least one rainy day if each day is 30%" → UD tree → expression.
- **Exact computation in Rust:**
  - rational numbers without rounding error;
  - units of measurement with dimension checking — degrees are not added to kilometres;
  - statistics and probabilities.
- **A symbolic core from the start:** an expression is a tree (rational constants, symbols, operations, functions); numerical evaluation is just one of the operations on it; units and steps sit on top of the same tree.
- **Steps are shown:** "0.7³ ≈ 34% dry → 66% rain at least once".
- **Every result comes with an independent check** where possible: substitute the root back, differentiate the integral, compare A·A⁻¹ with I. Checking is easier than generating, so the check is the gate.
- **Hard problems — SLM + LLM:** the LLM states the problem and writes a solution plan; the SLM calculates exactly and checks. The LLM does not calculate at all, so it makes no arithmetic mistakes.
- Computation results are world facts with anchors and steps.

**Levels:**
1. arithmetic, units, percentages, statistics, probability — v1;
2. algebra: simplification, expansion and factorization, equations — linear, quadratic, systems by Gaussian elimination over rationals;
3. analysis: derivatives, limits, Taylor series, integrals — table, substitution, by parts; later Rubi rules as engine data (licence to be checked);
4. linear algebra over rationals: determinant, inverse, rank, eigenvalues via the characteristic polynomial;
5. discrete mathematics and number theory: combinatorics on big integers, GCD, primes, modular arithmetic;
6. probability and statistics: distributions, expectation, variance;
7. first-order ordinary differential equations.

**Measurement** — open sets: GSM8K, MATH, MMLU college mathematics, Rubi and SymPy tests. Check each licence before use; nothing is being downloaded yet.

**v1 (26.09):** crate `math/` (`math/README.md`). Rationals on i128 (overflow is an error), units with dimension and affine temperature, a symbolic tree, steps, gate checks (inverse operation, reverse conversion, enumeration of consequences, an independent path on f64). English understanding through the UD tree from `en`, and an honest "I don't understand". Frozen set — 157 of 158, 1 "don't understand", 0 wrong (first run — 155/2/1); held-out — 30 of 30. Continuation — after the world model.

**v2 (26.09):** a curriculum solver (`math/README.md`, section v2). The LLM writes a plan in a step language (compute, solve, factor, expand, derive, enumerate, check, claim); the SLM executes exactly and checks each step by an independent path; on failure — back to the LLM with the reason (≤ 2 retries), then "not solved". Core: big integers and rationals on `dashu` (chosen by benchmark: GCD 16×, rationals ~100× faster than num-*), number theory with certificates (Bézout, Miller–Rabin, Pocklington, Pollard–Brent, CRT), combinatorics with a second path (Legendre, Euler), polynomials over Q with radicals and multiplicities, Gaussian elimination over rationals, derivatives with steps and a central difference, bounded enumeration. Levels 0–3 (GSM8K, TAL-SCQ5K, MMLU, TheoremQA — MIT) passed: on held-out sets LLM + SLM verified and hit 99%, 100%, 96.7%, 100% (the LLM alone — 98%, 100%, 96.7%, 100%, 1.8–2.9× cheaper); wrong answers coincide and stem from interpreting the problem statement, and 3 of the 4 "wrong" ones at level 2 are gold-standard errors that the SLM traces revealed. Level 4 (Lean, ProofNet#): 6 of 10 on the first attempt, 7 of 10 with Lean errors fed back. Memory — 360 verified solutions (`$MOVA_DATA/db/mathsolve.duckdb`); a perceptron guesses the LLM's method in 66% of GSM8K problems, but the SLM can so far compose a plan by itself only for problems of the same structure (2 of 180 held-out). Cost of the wave — $8.06.

### Physics, chemistry and statistics

Design note: plus physics and chemistry; chemistry at a very high level, and physics, like maths and statistics, at genius level.

**Goal:** physics, maths and statistics — genius level, chemistry — very high. The path is the same as in maths: a curriculum from small to large, the LLM states the problem, the SLM calculates exactly and checks. Checking is easier than generating, so each science has its own gates:
- **physics:** dimension of every step, limiting and special cases, conservation laws, symmetry, order of magnitude, numerical simulation;
- **chemistry:** mass and charge balance, valence, thermodynamic consistency (Hess's law), cross-checking against PubChem and NIST data;
- **statistics:** checking by simulation (Monte Carlo for p-values and confidence intervals), analytics versus simulation, assumptions stated aloud.

**Measurement:** open olympiad- and graduate-level sets, e.g. GPQA, SciBench, physics and chemistry from OlympiadBench. Check each licence.

**How we build it.** Design note: do maths, physics, statistics and chemistry via ready-made Rust libraries or our own ports of top libraries from other languages, plus a large but not unlimited base of books and papers.
- **Code:** first permissive Rust crates. Then our ports of the best open libraries from other languages: Julia (mostly MIT), SciPy, statsmodels, Stan (BSD), OPSIN (MIT) and others. GPL, AGPL and commercial — only as ideas from documentation. The verdict for each library comes from a survey: the source notes.
- **A library of books and papers — large but not unlimited:** ~100 GB for all sciences.
  - mathematics ~40 GB, physics ~25, statistics ~20, chemistry ~15;
  - open licences for training, the rest read-only;
  - safeguard: at least 300 GB of free disk space.
- **Abstracts:**
  - every paper has the authors' abstract from metadata (arXiv, Crossref) — free, immediately, for all;
  - our own Opus summaries — only for the most important, since for 100k papers that would be ~$11k.
- **The foundation already exists:** units with dimension checking and affine temperature — in `math`.
- **Data:**
  - CODATA values of physical constants are facts; we embed them with a reference to the source. We do not republish the NIST file itself (SRD 121; NIST claims rights to the compilation) as our dataset;
  - the periodic table — from PubChem and Wikidata.
- **Physics:** problems with units, where the LLM states the problem and the SLM calculates and checks the dimension of every step.
- **Chemistry:**
  - molar masses and stoichiometry;
  - balancing reactions as linear algebra: the null space of the element matrix over rationals.
- **Library:** open textbooks (OpenStax Physics and Chemistry, Light and Matter — licences to be checked), open problem sets, selected papers. Summaries are written by the small models.
- **Queue:** a separate wave, when a slot frees up among the small models.

### Mini-lab: a MATLAB/Octave-style language and built-in LaTeX

Design note: copy the logic of Octave and build a mini competitor to MATLAB; plus internal compiled LaTeX support.
- **Computation language.** A subset of the MATLAB/Octave language: matrices, 1-based indexing, colon, `end`, element-wise operations, functions, control flow. The interpreter is our own, in Rust.
  - Opus writes such code fluently, so it is a natural language for LLM plans, and the SLM executes them.
  - Our difference is an exact mode: rational matrices, big integers, verification of every result. Plus steps and proofs.
- **Licences:**
  - Octave is GPL — we take the behaviour from the documentation (clean reimplementation), not Octave's code.
  - Language functionality and APIs are not protected by copyright (EU: SAS Institute v WPL, 2012; US: Google v. Oracle, 2021).
  - We do not use the name "MATLAB" in the product — it is a trademark.
- **Building blocks — permissive Rust crates:**
  - linear algebra — `faer` or `nalgebra`;
  - arrays — `ndarray`;
  - FFT — `rustfft`;
  - optimization — `argmin`;
  - statistics — `statrs`.

  The exact core — from `math`.
- **LaTeX compiled into the binary:**
  - parsing formulas into an expression tree — including from arXiv sources and textbooks in our library;
  - printing steps and answers in LaTeX.

  No external TeX needed; on the website formulas are rendered by KaTeX.
- **Tables and statistics, as in Excel.** Design note: also statistics on top, and work with tables, like Excel, with export and import.
  - **The `table` type:** named columns of different types (numbers, text, dates, logicals). Operations: `readtable`/`writetable`, `sortrows`, `groupsummary`, `join`, filters by logical indexing, pivot tables.
  - **Import and export:**
    - CSV and TSV;
    - XLSX and ODS — reading via `calamine` (MIT), writing XLSX via `rust_xlsxwriter` (MIT/Apache);
    - JSON;
    - DuckDB — via our CLI, i.e. tables straight from the dev store.
  - **Statistics:**
    - descriptive: mean, median, deviation, quantiles, mode, skewness, kurtosis, correlation, histograms;
    - distributions — `statrs` (MIT);
    - tests with p-values: t, χ², ANOVA, correlation;
    - OLS regression with a report.
  - **Transparency:** every test states its assumptions aloud (normality, independence) and shows the computation steps.
  - **Queue:** mlab v2 — after merging v1, so as not to burn the shared limit.

**LaTeX v1 (26.09):** crate `latex/` (`latex/README.md`). Command table → build.rs → enums; document macros (`\newcommand`, `\def` with `#1…#9`); an unknown command is an `Unknown` node with a position; printing to canonical LaTeX and plain text; extracting formulas from `.tex`; a bridge to `math::Expr` (feature `math`) and back. arXiv abstracts from the pilot, 2.54M formulas: without `Unknown` 97.4 %, round trip 100 %, 1.8M formulas/s; corrupted printing goes red on exactly the affected formulas.

- **mlab v1 (26.09):** crate `mlab/` (`mlab/README.md`), CLI `mlab run file.m`, `mlab -e "…"`.
  - **Language:** f64/logical/char matrices, complex — minimally; indexing with `end`, masks, growth and
    deletion; operators with broadcasting; anonymous functions with capture; functions at the end of a script; `if/for/while/
    switch/try`; output as in Octave. Slots for v2 are already in the value type: `Table` and `Exact` (exact mode via `math`).
  - **229 built-ins**, extensible registry: linear algebra on faer, FFT on rustfft, `integral` (Gauss–Kronrod),
    `fzero` (Brent), `fminsearch` (Nelder–Mead), `ode45` (Dormand–Prince), polynomials.
  - **Backend — faer:** versus nalgebra on a single thread, solving 1000×1000 is 6× faster, eig 500×500 —
    2.6×; only faer gives eigenvectors of non-symmetric matrices.
  - **Gates:** `\` (backward error), `inv` (A·inv(A) = I), `eig` (A·V = V·D), `svd` (U·S·Vᵀ = A), `lu/qr/chol`.
    On failure — a warning with a number.
  - **Frozen set:** 206 scenarios from documentation and identities, first run — 205/206. The only
    red one is an error in the set itself: a singular A. The set and the code were written by the same author, so an independent check comes
    later.
  - **Negative controls are red:** a broken `sum` gives 199/206; a planted wrong answer is caught by the gate.
  - Next — exact mode after math v2, cell arrays and structs, plots, v2 with tables and statistics.
- **mlab v2 (26.09):** tables and statistics in `mlab/` (`mlab/README.md`, section "v2").
  - **Columnar in-memory tables:** `table` (numbers, logicals, text, dates; missing values), `T.x`, `T(rows, vars)`,
    masks, `T.c = T.a .* T.b`; `sortrows`, `groupsummary` over several keys, `grouptransform` (`@cumsum`, moving
    windows within a group), `pivot`, `innerjoin`/`join`. 1M rows: grouping + sum + sorting — 0.12 s.
  - **Files:** CSV/TSV and JSON — our own parsers, errors with line and column; column types inferred from data; XLSX/ODS — later.
  - **Statistics:** descriptive, 18 distribution functions (`statrs`, MIT; normal — our own `erfc`: statrs has an error of
    ~5·10⁻¹¹, found by the test set), `ttest`/`ttest2` (and Welch)/`chi2test`/`anova1`/`corr` with p, `fitlm` (OLS via QR).
    Every test prints a report: hypotheses, assumptions (what was checked from the data, what was not), steps, conclusion.
  - **Gates:** p-values — Monte Carlo under H0 (8 tests × 4 α levels, K = 20,000, fixed seed) — 32/32 green,
    a wrong p goes red; `fitlm` is checked against `X \ y`.
  - **Set v2:** 44 scenarios, frozen by hash before the code; first run 43/44 (statrs error), after our own `erfc` —
    44/44; v1 regression unchanged (205/206, the same la-18). Next — XLSX/ODS, a "table maths" pilot with the LLM.

### Execution sandbox: the SLM programs, bytecode, an in-memory file system

Design note: the SLM can also be taught to program, translators can be built into bytecode interpreters, and a small file system can be made in RAM.
- **The SLM programs simple things.** Data transformations, table computations, task plans — in the mlab language or the plan language. The LLM is the teacher. Every program passes test gates: checking is easier than generating.
- **Teach the SLM to code** (design note: the SLM also needs to be taught to code). A curriculum from small to large, as in maths:
  1. expressions and table operations in the mlab language;
  2. small functions with tests;
  3. data transformation by example, like FlashFill (Gulwani 2011): regular expressions, parsing, tables;
  4. rewriting and optimization: loops → vectors, C fragments → Rust.
  - **How it learns:** the LLM is the teacher; it writes the solution and the tests. Verified programs go into memory. The SLM first looks for a similar verified template and enumerates variants with type awareness — classic program synthesis. It calls the LLM only for something new. The selection model is transparent.
  - **Gates:** tests, property tests, execution in the sandbox, equivalence.
  - **Measurement:** HumanEval (MIT), MBPP (CC BY), DS-1000, CRUXEval — licences to be checked. Three configurations: LLM alone, LLM + SLM, SLM alone.
  - **coder v0 (26.09)** — training without a VM, a "student — architect" loop (closed part, `tera/coder/`). Design note: Opus looks at the result and changes the student's architecture… Opus itself does the evaluation.
    - **Tasks:** 150 frozen — 100 from Opus and 50 adaptations of exercises from Erickson, ODS and SICP (CC BY / CC BY-SA). Split 100 training / 50 held-out.
    - **Memory:** 627 verified mlab programs.
    - **Architectures:** v0 — templates (BM25 on our lemmas, substitutions); v1 — motivated steps from Opus-annotated math v2 solutions (a "difference → move" table, GPS), two modes: fast (built-ins) and step-by-step (loops).
    - **4 rounds, blind judge:** held-out — 1.02 → 1.54 → 1.58 → 1.58; actually passed 0 → 3 → 5 → 5 of 50. Training — 0 → 12 → 21 → 20 of 100.
    - **A judge without a VM:** 599/599 "will pass" predictions matched real execution. On mutants of gold solutions with no gold in the prompt — 295/295.
    - **Motivation** raised the explanation score (1.0 → 2.0); correctness came from the understanding rules.
    - **Agreement of two paths is a weak gate:** both paths come from the same plan.
    - **Two code representations:** text is better for retrieving an example (P@1 0.46 versus 0.36 for AST). For choosing a move both representations together are better; on unfinished code — AST with partial parsing.
    - **Cost:** $13.64.
- **Knowledge into code** (design note: where possible, let everything be output into compiled classes, into a real module of code).
  - From open books and papers — not only summaries but real Rust modules:
    - algorithms and data structures — from Erickson's "Algorithms", "Open Data Structures" (both CC BY);
    - methods — from the mathematics library;
    - concepts — into the compiled world-model core.
  - Every function has tests from the book's own examples and a reference to the source: book, chapter, page, licence — transparent judgments and attribution.
  - This is also the memory of verified programs on which the SLM learns to code.
  - The list of candidates is in the library dossier, section "Candidates for code".
- **Division of labour** (design note: one small one, the SLM, learns from scratch and describes the learning itself within itself, while the other, the LLM, writes a virtual machine and access to a virtual file system, virtual RAM…, which for emulated code is like the real thing):
  - **The SLM learns from scratch and keeps a learning log within itself.** For every skill:
    - level;
    - on which examples it was learned (source: LLM or book, page);
    - which tests were passed;
    - accuracy by version.

    The log is a DuckDB table next to the memory of verified solutions. It shows what the SLM has learned, from where, and how well.
  - **The LLM builds a virtual environment** — a WASI host. For emulated code it is like the real thing:
    - an in-memory file system with snapshots;
    - virtual RAM with limits;
    - a virtual clock;
    - seeded randomness;
    - no network, or a controlled one.

    Full reproducibility: the same run gives the same result.
  - **Honesty with the SLM itself** (design note: the funniest part is when we explain directly how the VM works and what it allows the SLM to do).
    - For guest programs the VM is an illusion of a real machine. For the SLM itself it is not.
    - **The sandbox charter** is read by the SLM at every start:
      - that it is in a sandbox;
      - what it is allowed to do;
      - what the gates check and what is logged;
      - what is in the immutable trusted core;
      - how to legitimately ask for more — by a request to a human.
    - When the rules and the legitimate path are known, escaping the sandbox has neither sense nor benefit.
    - **Its own code — readable, plus a guide on "how to change your own parts"** (design note: as in the Matrix, it is given its own code to read and a description of how to modify its sub-parts for particular tasks):
      - a map of modules: what may be rewritten and what is the trusted core;
      - the interfaces and invariants of each module;
      - which tests and frozen sets guard the module;
      - how to submit a change to the self-extension loop;
      - examples of successful and failed changes from the log.

      This is a `CONTRIBUTING.md`, only for itself.
  - **Access steps** (design note: when it is stable and passes tests and antivirus, it gets real access in addition to the VM):
    0. sandbox only;
    1. reading real data — designated folders, read-only;
    2. writing to dedicated folders and calling designated APIs — with rate limiting, logging and rollback;
    3. the production loop — in stages, canary first, with a kill switch.
    - **Rules for every step:**
      - Every permission is narrow, has an expiry and is revoked in one move.
      - Only a human promotes, after gates and stable operation at the previous step.
      - Any anomaly — automatically back into the sandbox.
      - The SLM does not grant itself rights: granting rights is also part of the immutable trusted core.
  - **A clean machine for every lesson** (design note: the SLM's changes can be rolled back very cheaply, giving it a seemingly clean machine for learning to program).
    - The virtual Linux state is a copy-on-write snapshot. Every training episode starts from a clean snapshot in microseconds.
    - Many copies can be kept in parallel, each reproducible from a seed.
    - "What if" — state branches, like branches in the tale world: try two approaches, compare, keep the better one.
    - Only verified results and the learning log are kept. The rest is discarded.
    - **A warm snapshot after startup** (design note: the moment after Python starts is a system snapshot, literally a cursor, as in the tales, a state of classes; cleaning the system and initializing Python takes microseconds).
      - The interpreter with loaded modules and `vfs` is frozen as an immutable snapshot. Every run is a branch from it, like `state@n` in the tale world.
      - Precedents: the Android zygote, Firecracker snapshots, Smalltalk images, V8 startup snapshots.
  - **Emulation levels** (design note: and this can be real, a lower level of emulation). Each level is useful by itself and becomes the foundation of the next:
    1. **Language VM + `vfs`** — mlab code in bytecode, files in memory. Already being built.
    2. **WASI host** — any program in WASM sees `vfs`, a virtual clock and a seed. Already being built.
    3. **A virtual POSIX / WASIX kernel** — users, processes, pipes, terminal, packages, i.e. a virtual Debian.
    4. **Full machine emulation** — real binaries (v86, Blink).

    The lower the level, the faster and cheaper. We take the lowest one that is sufficient for the task.
  - **Linux emulation** (design note: plus literal emulation of Linux, the ability to install packages, create a user, an emulated terminal, and so on).
    - **A virtual kernel in Rust:**
      - users and permissions, processes, pipes, signals, `/proc`, `/home`;
      - a WASIX-like API (Wasmer, MIT) — POSIX on top of WASI.
      - Precedent — gVisor (Google, Apache-2.0): a kernel in user space.
    - **Terminal and shell — in Rust:** `brush`, bash-compatible, or nushell (MIT). Commands — uutils/coreutils (MIT, GNU coreutils rewritten in Rust), built to WASM.
    - **"Installing packages"** — signed WASM modules from our registry into a virtual `/usr/bin`, through the gate loop.
    - **Literal compatibility** — as a separate mode: full machine emulation for real binaries. Candidates: v86 (BSD-2), Blink (ISC).
    - **GPL tools** (busybox, QEMU) — only as separate programs, not embedded.
    - **The model — the latest Debian stable with long-term support** (design note: the virtual Linux can literally be an optimized copy of the real latest long-term-support Debian).
      - The same package names and versions, FHS layout and behaviour. Source — Debian package indices and machine-readable `copyright` files (DEP-5).
      - Each package is one of three, in order of preference (design note: technically, where possible, packages should be emulated):
        1. **package emulation** — our implementation behind the same interface: commands, flags, files, output format, exit codes. Inside — the "as if" rule: `mysql-server` → our facade on SQLite or DuckDB, `python3` → our Python, `gzip` between our own steps → nothing. For coreutils — uutils;
        2. built to WASM from Debian sources — when emulating makes no sense yet;
        3. full machine emulation — the last resort, only for literal compatibility.
      - **The terminal — working with classes, not launching programs** (design note: the terminal is then not a call to a separate program but simply work with a class).
        - Commands are methods of objects in the same process, without `fork` and `exec`. `ls | grep x | sort` is a chain of iterators over the virtual file system.
        - Structured data flows between commands, as in nushell or PowerShell. Text only at the boundary: when shown to a human or when a program really reads bytes (the "as if" rule).
        - Separate processes only where really needed: long-running services, isolation.
      - **Compatibility check:** Debian's existing package tests (autopkgtest, DEP-8) plus differential tests against real Debian in a container (on Hetzner).
      - **Licences:** GPL packages — as separate programs with access to sources, as in Debian itself. Name — only "based on Debian": it is a trademark.
    - Licences to be checked.
- **Motivated step-by-step — clever training** (design note: let it first solve maths problems through motivation — the search for a solution as a step-by-step motivational model; the same training of step-by-step thinking, breaking a task into steps; ready solutions should explain the steps with their motivations).
  - The unit of training is a step with **a goal, a motive, a move and a check**: "goal — get rid of fractions → motive: integers are easier → multiply by 6 → check by substitution".
  - The SLM learns a "difference from the goal → move" table (means–ends analysis, Newell and Simon's GPS), the choice of the next move and the decomposition of a goal into subgoals. First on maths, then transferred to code.
  - Ready solutions explain every step with a motive: this is both transparent judgment and the self-explanation effect from cognitive science (Chi et al. 1989).
  - Roots — Pólya's "How to Solve It": understand → devise a plan → carry it out → look back.
  - **Code as language — two representations** (design note: directly — the SLM sees textual code, which is good when the code is not valid; and as a parsed AST structure — then it sees the hierarchy of relations more precisely).
    - Text — tokens and n-grams, robust to invalid code. AST — nodes, paths, scopes, data flow.
    - As in our language work: surface form and dependency tree. AST paths as features — code2vec (Alon et al.).
    - For invalid code — error-recovering parsing, the tree-sitter idea (MIT).
  - **Two paths** (design note: it is one thing when it can use existing tools for a mega-fast result — that is top — but it must also be able to do it step by step):
    - **fast** — a ready tool: the exact core, a built-in function, a verified solution from memory;
    - **step-by-step** — motivated steps to the same result.

    The fast path gives the result. The step-by-step path gives explanation, learning and new problems with no ready tool. Together they are an independent check: the paths agree → the gate is green.

    **For every science, not only maths** (design note, 30.09: the same will have to be done for physics etc.; sometimes the fast part calculates while the slow logical part roughly understands how exactly). Every SLM subsystem has two parts: a built-in computational core — deterministic and incredibly fast (in maths it is `rat`/`expr`/`eval`, in physics — units, laws, numerical methods) — and a slow logical model that builds the plan. The logical one does not need to calculate exactly. It is enough to roughly understand *how* the solution goes: which law, which quantities, in what order. The core computes the numbers. Hence the yardstick: not "did the logical model calculate it" but "did it correctly choose and compose the steps for the core".

    Refinement (30.09): the slow model must understand how to decompose into subproblems and roughly how they will be executed, while the subproblems can be computed by the dumb fast one; but the slow one should also try to be able to describe and compute by itself — that shows it is not dumb; in production it does not always need to derive and prove all steps itself, it is enough to call the fast one. So the slow model has two modes: a training mode, where it writes out and computes the steps itself, and a working mode, where it only composes the subproblem tree and hands it to the core.

    **math v3 (30.09), first cut — `math/src/arith.rs`.** The slow model composes a subproblem tree: which numbers from the problem statement are needed and which operation applies to each pair (the operation at the lowest common ancestor, Roy & Roth 2015). It is an averaged perceptron on words; `mathsolve arith --show N` prints the explanation. The fast core enumerates trees over subsets of numbers and recomputes the answer exactly. Training on GSM8K train gold without the LLM; for 987 problems there is also LLM plan supervision (which numbers were used).
    - Measurement on GSM8K test: the SLM alone solves 3.3% (44 of 1319). A tree of up to 4 leaves exists for 73% of problems. So the weak spot is ranking, not the core.
    - Conclusion: "bag of words" features do not see who has what and what changes. The next model is state tracking across sentences (ARIS, Hosseini 2014), i.e. our world: entities, quantities, changes from verbs. Plus features from the UD tree (`en`). Papers on solving word problems are being collected.
    - Template memory (v2) does not generalize: after a wave of 980 verified plans (memory ×5) the SLM alone solves 0 of 100 at L0.

    **math v4 (30.09) — `math/src/steps.rs`.** A transition system in the spirit of our arc-standard: numbers come in text order, actions SHIFT / SKIP / REDUCE (+, ×, −, ÷ with direction). Features from the UD tree: unit, verb, subject, cue words, their combinations. Training — a structured perceptron with a beam and early update; gold — `arith` trees that compose in text order (2938 problems). Training set — 62%, GSM8K test — 1.7%. The model memorizes vocabulary, not the principle.

    **Conclusion: GSM8K is too high for the first step.** A curriculum from simple is needed.

    **The SLM learning loop** (ideas from the paper library, 30.09):
    1. analogy — "have I seen a similar problem?" (Pólya);
    2. a tree by principles, not surface features (Chi, Feltovich, Glaser 1981), built as reducing the "have — need" gap (GPS, means–ends);
    3. explicit steps with a "why" (self-explanation — Chi 1989; scratchpad, CoT);
    4. step-by-step checking by the core and Lean (Lightman 2023);
    5. what is verified goes into training, and round again (STaR); the exact core is an ideal verifier;
    6. generalize and wrap up (EBL, chunking in SOAR): a solved subproblem becomes a ready block, i.e. a fast model on top of the slow one;
    7. a curriculum with fading steps (Sweller): full solution → a missing step → plan only → on its own;
    8. on clean data — textbooks with solutions (Phi, TinyGSM).

    **Data steps:** one operation (MAWPS SingleOp, AddSub) → two or three (MultiArith, ASDiv, SVAMP) → GSM8K → competition level. Data with licences is collected separately.

    **Exact gold (30.09) — the main lever.** Instead of guessed trees — exact ones: SVAMP equations and GSM8K calculator steps (`<<48/2=24>>`), `steps::tree_from_equation` / `tree_from_calcs`. Plus actions "combine s0 with s2/s3" for numbers out of order (like SWAP in Nivre 2009), with the oracle by simulation.
    - SVAMP test (MIT, 300): the SLM alone — **44.7%**; the most confident 10% — 70–73%. For comparison, the neural DeductReasoner — 47.3.
    - GSM8K test: **9.6%** (was 1.7% on guessed trees); the most confident 10% — 20–25%.
    - Not used in training: 3299 GSM8K problems without an exact tree (implicit numbers), 2043 — trees with repeats (DAG). General implicit constants (`MATH_IMPLIED=1`) give more trees but noise in the search — SVAMP drops to 37%.

    **Papers from the library → gain (30.09).** Non-lexical features (Roy & Roth 2018: verb classes have/get/give/make/lose/move from seed lists; UnitDep 2017: the rate "X per Y" and whether the rate's denominator is the unit of another number; Patel 2021: whether the number's subject is the question's subject) instead of words and lemmas (`MATH_NOLEX=1`): SVAMP 44.7 → 48.0%.
    **Alternative branches** (design note: look not at one but at the top-K; alternatives can help understand the problem better). Voting of branches by value (self-consistency) did not add accuracy — different branches almost always give different answers — but it calibrates confidence. **Top-K reranking (Collins 2000)**: a second model sees each finished solution as a whole — how many numbers are unused and whether they concern the question, root operation × question words, answer size relative to the numbers in the statement, the score gap. Candidates for its training come from models trained without their own fold. SVAMP: **54.3%** (top-5); the correct answer is among the top-5 in 84% of problems — lots of headroom. GSM8K: **10.8%** (was 9.6%). Binding numbers to the question in the judge (Patel 2021, ARIS: content words of the question near the number; the order "X than Y" under a subtraction root) — SVAMP **55.7%**. Joint training on SVAMP + GSM8K hurts SVAMP (45.0%) — steps are trained separately. Synthetic data from the same distribution helps: TinyGSM (MIT, Python functions → exact trees; a number "out of thin air" goes red, ~15% remain) + GSM8K — **11.8%** on GSM8K test.

    **The quantitative layer of the world (`world/src/quant.rs`).** State `entity.thing → number`. Actions: set, add, sub, mul, div, move, ask. Every command is bound to a sentence `@N` and labelled with a principle `k=` (given, total, gain, loss, transfer, rate, compare, part, convert, combine, unit). The exact core executes it. The gate checks: the number comes from the text up to this sentence (or is a unit constant), only existing things change, there is an ask, the answer = gold; negative controls are red. Opus (subscription, medium) writes scripts for SVAMP and GSM8K train — pilot 19/20 green. On them the snake will learn to write the world itself, sentence by sentence, with the principle as a node feature.

    **Physics and common sense** (design note, 30.09: the robot should understand why, if Newton sits under a tree, an apple accelerating onto his head is not pleasant). The next world layer links laws (falling with acceleration, speed from height, impact — the fast physics core) to the common sense of the world model (a blow to the head hurts, risks from animals and things). The quantitative layer is the first step: quantities appear in the world.
- **A shared bytecode for our languages.** mlab, the plan language, the world-change language and the `convert` rules are translated into one bytecode. A small VM in Rust executes it — faster than tree walking, and portable. The same VM is built to WASM for Mini.
- **An in-memory file system.** Programs see only virtual files in memory. Only the approved result goes outside. This is both speed and safety.
  - **vfs v1 (26.09)** — crate `vfs` (`vfs/README.md`):
    - a copy-on-write in-memory tree; snapshot and restore in ~20 ns regardless of size,
      byte-exact restore; limits, log, virtual clock; escaping the root is an error;
    - explicit import and export of a real folder;
    - mlab accesses files only through it: `--fs mem` by default, `dir:<folder>`, `real` — only explicitly;
    - `wasi-host` (engine `wasmi`, WASI preview1 → the same `vfs`) runs `mlab.wasm`: 25 file scenarios × tree
      and VM give the same output as native mlab.
- **Ideas — from the classics:** the Lua VM (MIT), WebAssembly, WASI.
- **Choice by speed** (design note: pick whichever classic is faster):
  - **our VM is register-based, as in Lua 5.** A register VM executes roughly 47% fewer instructions than a stack VM and runs about a third faster (Shi, Gregg, Beatty, Ertl, "Virtual Machine Showdown: Stack Versus Registers"; Ierusalimschy et al., "The Implementation of Lua 5.0");
  - dispatch — `match` in a loop, superinstructions for hot pairs, compact value representation. Matrix operations go through built-in functions on `faer` anyway;
  - **WASM modules:** on the server — `wasmtime` with the Cranelift JIT, close to native speed. In Mini — `wasmi`: an interpreter in Rust, also with register-based internal code, small;
  - **native Rust modules compiled into the VM** (design note: rewrite a whole standard module, say of Python, in Rust and literally stitch it into the virtual machine). This is how CPython (C modules), Lua (C API) and Julia work: the script only conducts, and the heavy work runs in native code.
    - **The built-ins registry** is extensible; mlab already has it.
    - **Candidates:**
      - `json` and `csv` — `serde`;
      - `re` — the `regex` crate, linear time;
      - `datetime` — `jiff`;
      - `fractions` and `decimal` — `dashu`;
      - `random` — deterministic, seeded;
      - `hashlib`, `zlib`, `collections`, `itertools`;
      - `statistics` — our statistics.
    - **Ready-made:** RustPython (MIT) already has many Python standard library modules in Rust — crates on crates.io that can be adapted.
    - **Behaviour** — per the Python documentation and CPython tests (PSF licence, permissive).
  - **numpy compatibility** (design note: the maths module can be stitched to numpy: instead of a real numpy call — a call to our inexact maths module).
    - `numpy.*` calls in a script go to our native module: floating-point numbers, `faer`, mlab built-ins. Exact mode is a separate element type on our rationals.
    - The compatibility yardstick is the Python Array API standard (data-apis.org): the subset of numpy shared by CuPy, JAX and others.
    - Python frontend — RustPython (MIT). Then LLM plans can also be ordinary Python with numpy, which Opus writes most fluently, while execution happens in our sandbox.
  - **Planner-optimizer** (design note: user code runs through a planner-optimizer and, where possible, is cleverly rewritten into matrix optimizations). Code is first translated into an intermediate representation and then rewritten:
    - loops — into vector and matrix operations;
    - fusion of adjacent element-wise operations;
    - matrix-chain multiplication order — classic dynamic programming, `A*B*v` → `A*(B*v)`;
    - common subexpressions;
    - for tables — a lazy query plan with filter pushdown, as in polars or DuckDB.

    Every rewrite has an equivalence gate: the same result on test inputs. The user sees what changed: "loop replaced with `A*x` — 200× faster".
  - **Compatibility — checked with real code** (design note: the idea is for the SLM to test the virtual machine for compatibility with real, non-virtual code). This is differential testing:
    - the same real code is run on the reference interpreter (CPython, PHP, Octave) and on our VM, and then the output, errors and side effects in the in-memory file system are compared;
    - every discrepancy becomes a row in the fix queue with a minimal example, which the SLM shrinks itself;
    - corpora: CPython tests (PSF), `.phpt` from php-src (PHP License), numpy examples (BSD), open projects with licences. A heavy integration test — bring up WordPress so that it serves the same pages.
    - CPython is already on the box, so the Python frontend can be checked right away. Octave and PHP are installed by the operator when their turn comes.
  - **Half-native, half-virtual code, a facade outside** (design note, 26.09). A program in PHP, Python or MATLAB runs half as native Rust — compiled standard modules, if the licence allows — and half as translated bytecode in the sandbox.
    - **Outside — the same API,** inside — our principle:
      - files go to the in-memory file system with snapshots to disk;
      - "MySQL" actually runs on DuckDB or Parquet: a protocol facade plus SQL translation;
      - logs are structured, in the same virtual space, batched and compressed, not line by line onto a physical disk.
    - **Three caveats:**
      - We do not lose durability: where the program expects data to be saved (transaction commit) — a write-ahead log and explicit snapshots.
      - DuckDB is an analytical database with a single writer. For sites with frequent small writes — SQLite (public domain) or Limbo (MIT, Rust).
      - Errors and security logs never disappear silently. Log level and sampling are configured, not everything switched off.
  - **The languages actually needed** (design note: PHP is a joke; what we really need is Octave, Python, C/C++, R, Julia and maybe something else):
    1. **Python + numpy** — first: the most users, CPython is already there for differential tests. Frontend — RustPython (MIT), numpy — our native module.
    2. **Octave / MATLAB** — mlab already exists; we extend it. Reference for tests — Octave, to be installed by the operator.
    3. **R** — our own frontend for a subset, from the language documentation, "clean room". Statistics — from our ports (Julia, statsmodels). GPL CRAN packages may be run, but not embedded in the product.
    4. **Julia** — a subset frontend. The standard library is MIT, so code can be ported directly.
    5. **C/C++** — compile with `clang` (LLVM, Apache-2.0 with exception) to WASM and run in the sandbox with WASI and the in-memory file system. For native modules — C-to-Rust translation via `c2rust` (BSD-3).
    6. **Also:** Fortran via LLVM Flang to WASM (old numerical classics), SQL via DuckDB, Excel formulas — for tables.
    - **From `c2rust` to a full rewrite** (design note: c2rust is the base, but even better is a full rewrite and emulation: with source code, comments in code and on the internet, and code analytics for understanding the logic, hyper-optimizations are possible):
      1. **`c2rust`** gives a mechanical translation: unsafe but equivalent. It becomes the reference for differential tests.
      2. **Understanding.** The specification is extracted from code, comments, documentation, papers and discussions. Code analytics: call graph, data flows, invariants, hot-spot profiles.
      3. **Rewriting.** The LLM writes idiomatic safe Rust with better algorithms, data structures, vectorization and parallelism.
      4. **Verification.** Differential tests against the original and against the `c2rust` version, property tests, fuzzing, benchmarks. Not equivalent — does not ship.
      - **The contract is the API** (design note: for module A, which calls some module written in C, the API is what matters).
        - Module A sees only the interface, so the implementation beneath it is replaced with Rust with the same API, and for binary compatibility also with the same ABI (`extern "C"`, the same symbols).
        - Precedents: `zlib-rs` — a libz replacement with the same C interface; the OpenSSL compatibility layer for `rustls`; `sudo-rs`, `ntpd-rs`.
        - Verification — the library's own tests plus differential ones. For Python extensions — the CPython C API or HPy.
      - **The "as if" rule at system level** (design note: if a Python module compresses with gzip because it thinks that is better, there is no need to gzip at all; much faster compressors can be used, or no compression at all).
        - Like C++ compilers, we preserve only what is observable from outside. Inside the sandbox — anything goes.
        - Example: a program gzips intermediate data for itself. In the in-memory file system we do not compress it at all; when saving to disk we compress with zstd or lz4; and to a reader we hand it out as if it were gzip.
        - Real gzip only at the boundary: a file or network going outside, `Content-Encoding: gzip`, a hash or size that the program checks.
        - **Even better — at code level** (design note: cut the gzip check and decompression out of the transformed code entirely and substitute already-decompressed or raw data, never packed). The optimizer rewrites `decompress(compress(x)) → x` as a rule in an e-graph and removes the whole pair. Header checks, CRC and buffers disappear along with it.
        - Condition — escape analysis: the compressed bytes do not leave the sandbox, nobody hashes or measures them. Then the data stays raw from start to finish and is never packed.
        - Differential testing shows where the boundary is: the result outside must be the same.
      - **Flattening the call hierarchy** (design note: even more optimized variants without APIs are stitched into an even higher module in the call hierarchy; the original may have had huge redundancy of calls and type checks, while under the hood there is minimal, mega-efficient code).
        - Internal APIs are needed by people, not by the machine. Modules are inlined into a higher-level module, and only the external API remains outside.
        - Techniques:
          - whole-program optimization;
          - type specialization, as in Julia;
          - removing type checks that are statically proven;
          - pass fusion, constant propagation, dead-code elimination.
        - Theoretical basis — partial evaluation and the Futamura projections: an interpreter specialized for a program is the compiled program. This is how Truffle / Graal (UPL licence) and PyPy work.
        - Gate — equivalence plus differential tests of the external API.
      5. **Licence.** Rewriting from code is a derivative work, so the original's licence applies. GPL code for a closed product — only "clean room": one person writes a specification from the documentation, and another implements it without seeing the code.
  - **An application compiled as a whole** (design note: our compiled WordPress should not call our compiled PHP at all; it will run by itself as a hyper-optimized Rust module).
    - A PHP program is translated into Rust as a whole, ahead of time, before it runs, without an interpreter.
    - Precedents: HipHop for PHP at Facebook (2010, PHP → C++), KPHP at VK (GPL — ideas only).
    - Dynamic features — `eval`, variable variables, plugins loaded on the fly — stay in the VM. The core is native. Equivalence is checked by differential testing.
    - **Licence.** A compiled WordPress is a derivative of GPL-2.0. Running it ourselves and offering it as a service — allowed. Distributing — only with code under the GPL.
  - **Hot compiled code:** a hot function of a script or plan is rewritten in Rust, compiled into a WASM module and plugged in through the self-extension loop (gates, signature, rollback) — like Numba or mypyc.
  - **final decision — by benchmark:** fib(30), a loop of 10⁷ additions, a mixed matrix task. The current tree walking in mlab versus our register VM versus `wasmi` versus `wasmtime`.
- **VM v1 (26.09)** — done for mlab: `mlab/src/vm.rs`, the mlab README, section "VM v1".
  - Registers per frame, scalars without memory allocation, 45 instructions, superinstructions. Whatever does not compile is transparently executed by the tree walker.
  - Equivalence gate: both frozen sets byte for byte, 250 of 250. Negative control — a planted bug in the VM — is red.
  - Speed-up versus the tree: fib(27) — 7.9×, a 10⁷ loop — 14.5×, loops over a 1000×1000 matrix — 12.6×, a mixed matrix task — 2.0× (time spent in built-ins).
  - Builds for `wasm32-wasip1` (2.3 MB). Next — `wasmi` and `wasmtime` versus the VM, with the same benchmark.

### Self-extension: generated modules with hot plugging

Design note: if we also teach it not just to build abstract classes but to generate additional modules and import them by hot linking, that will be wild.
- **Loop:** a need (new domain, concept, skill) → the LLM writes a module → the SLM compiles it → gates (tests, negative controls, frozen sets) → signature → plugging in without restart → observation → rollback if worse.
- **Format — WebAssembly components** (`wasmtime`, Apache-2.0), not native `dylib`s.
  - Rust has no stable ABI, so native libraries are fragile.
  - WASM is a sandbox: a module has no network, no files, no clock until it is explicitly granted permission.
- **Safeguards:**
  - a module that has not passed the gates is not plugged in;
  - every module has a version, source, author (LLM or human) and log;
  - into the production loop — only with human confirmation;
  - rollback is one command.

  Self-modification without judges is exactly the "small models in coffee makers" scenario that we are not building.
- **The immutable trusted core** (design note: it should be able to rewrite whole modules of its own, except the virtual file system or the compiler).
  - The SLM may rewrite any of its modules entirely, **except the trusted core**:
    - the VM interpreter and the bytecode compiler;
    - the virtual file system;
    - gates and frozen sets;
    - signature verification;
    - the rollback mechanism and the log.
  - Only a human can change the trusted core, through a separate process. A judge cannot rewrite itself — otherwise the defendant would weaken its own tests.
- **On devices — self-adaptation, not a virus** (design note: so that the virus architecture is not a problem but a feature — a smart watch that rebuilds itself).
  - What adapts is **data**: Mova Mini's weights and vocabulary learn from the owner's texts, right on the device, with quality gates and rollback.
  - **Code** stays unchanged and signed. New code arrives only as a signed module through the loop above.
  - Nothing spreads between devices without the owner's consent.

  Three things distinguish a feature from a virus: consent, boundaries, verification.
- **First candidate:** the compiled world-model core. New domain concepts (seeds → `build.rs` → module) are plugged in without rebuilding all of Mova.

### Tools: data via API

Design note: plus we will teach it to pull data through our free API — let it fetch the weather forecast.

1. **Understanding the request:** "What's the weather in Vinnytsia tomorrow?" → `{intent: forecast, place: Vinnytsia, time: tomorrow, vars: [temperature, precipitation]}`. The place — via an external geography project's gazetteer (an external geography project), "tomorrow" — from the date anchor.
2. **A typed call:** request and response are Rust structs. The backend sits behind an interface:
   - first NWS (public domain) and ECCC (open licence) — the fetchers already exist in `weather` (`nws.rs`, `eccc.rs`);
   - later our free API — it does not exist yet, and there is no rush.
3. **Gates on the response:** schema, units, plausible bounds, freshness. On failure — "the data is doubtful", no inventions.
4. **Answer:** data → weather world → `weather` generator → text, with the source and update time at the end.
5. **Privacy:** the request contains only place and time, no user data.

### Reasoning and logic

Design note: a reasoning and logic module will also be needed.
- **A deterministic engine in Rust:**
  - facts and rules: Horn clauses, forward and backward chaining, like Datalog or Prolog;
  - default rules with exceptions: birds fly, penguins do not;
  - partial order of time — from world v3;
  - numeric constraints — via maths.
- **Every conclusion comes with a proof:** a chain of rules and facts with anchors in the text. This is the same transparency of judgments.
- **Division:** the LLM formulates the non-obvious; the SLM derives mechanically and checks the proof.
- **Where things come from:** facts — from the world (tale state, domain), rules — from the ontology and the domain profile.

### World model — separate from the narrative

Design note: it still needs to be taught, much as in the tales, that an apple is a fruit and so on; so the world model has to be built separately, no longer as part of the narrative but as a description of abstractions.

**Two levels, like TBox and ABox in description logic:**
- **World model — abstractions shared by all texts:**
  - concepts and their hierarchies;
  - default properties;
  - parts and wholes: a bird has a beak;
  - typical actions and consequences: dropped — it falls;
  - roles: mother, king, buyer.
- **Narrative — the specifics of one text.** Crow1@state3, her cheese, the event "dropped". Each individual references a world-model concept but does not copy it.

**Tale worlds are layers over the world model**, with exceptions, as in "Hierarchical and global worlds, epochs":
- **the real world** — the root;
- **genre** — in a fable animals talk;
- **collection** — "One Thousand and One Nights";
- **tale** — a single text.

An exception is written explicitly at its own level and does not spoil the root.

**One world model for everything:** tale, news item, Wiki article, domains. A domain adds its own concepts (weather — front, cyclone) as a profile.

**The core is compiled, the periphery is dynamic.** Design note: in the WordNet database there can be any inheritance at all, but with us a bird compiles properly, maybe it chirps and has a character; a lion can be stroked but may bite off your hand; well-known standard animals live a normal compiled life, not as a dynamic class.
- **The core — a few hundred well-known concepts:** fable animals, food, household, family, roles.
  - These are Rust types that `build.rs` stitches together from curated seeds. The same way a dictionary is compiled into `en` (`en/README.md`, "Dictionary in code").
  - Fields: food, size, habitat, parts, danger, actions with risks — a lion can be stroked but may bite off your hand.
  - Concepts are created at compile time, not at run time. The compiler checks that every core concept has its fields.
- **Categories — statistically, not only logically** (design note, 27.09: the neuron from apple to the type "fruit" does not always arise as the most logical one… "fruit" gives the fewest errors on average; it arises statistically — the teacher and parents repeated it many times, and if it doesn't sink in, peers mock you).
  - The link "concept → category" has a strength learned from prediction error (Rescorla–Wagner) from several sources with different weights:
    - the LLM — the "teacher";
    - the corpus — the "parents" who repeat;
    - WordNet / Wikidata — the "textbook";
    - disagreement of our parallel models — the "peers";
    - human correction — the strongest signal.
  - The cost of a "ridiculous" error is higher: "rubber — condom", "dolphin — fish".
  - Categories are graded, with prototypes (Rosch): an apple is a typical fruit, a tomato is a borderline case.
  - **The main source is corpora, not Opus** (design note, 27.09: the strategy of retraining with Opus verification is partly wrong: the SLM should derive this itself from huge corpora).
    - The SLM derives "apple → fruit" itself:
      - Hearst patterns ("fruits such as apples", "apples and other fruits");
      - distributional similarity — who lives in which contexts;
      - predominant senses from the corpus.
    - The corpora already exist: Leipzig, Gutenberg, arXiv abstracts, Wiktionary, EWT.
    - Opus is a rare arbiter for disputed cases and a teacher for new ones, not the main checker.
- **Properties of abstractions are distributions, not constants** (design note: no need to make a specific lion in the abstraction; let it come from a normal distribution).
  - Mass, size, lifespan — mean, σ, bounds, sources. Risks — probabilities, not "yes/no".
  - A specific lion appears only in the narrative: values from the text, and when the text is silent — from the distribution, flagged "assumed".
- **Periphery** — the rest of WordNet and Wikidata, a dynamic graph from a snapshot, as in v1, with inheritance of any depth.
- **Word → core concept,** if it is there. Otherwise — a graph node. The core outranks the graph, and a contradiction between them is a gate.

**Words tied to the world model correct parsing and pragmatics.** Design note: then corrections of pragmatics and of the syntax tree can also be built in through the abstract world model, once this model also attaches to words.
- **Word → concept.** A lemma in a particular sense references a world-model node. Sources: Open English WordNet synsets (CC BY 4.0) and Wikidata lexemes (CC0, where the sense points to an item).
- **Arc plausibility:**
  - "ate pizza with a fork": fork ⊂ instrument → attach to the verb (`obl`);
  - "pizza with anchovies": anchovies ⊂ food → attach to the noun (`nmod`);
  - who can be the agent of "threw", what can be "eaten" — these are selectional preferences.
  - In the library: `nakatani-1991-resolving-pragmatic-prepositional-phrase-attachment`, `heinzerling-2017-revisiting-selectional-preferences-coreference-resolution`.
- **Pragmatics:** a mismatch with the world model is a signal of irony or an indirect act: "I love waiting two hours at the dentist".
- **Place in the pipeline** — the second pass of "Multi-pass parsing". Word concepts and arc plausibility become reparse features.
- **The first measurement is already under way.** The context ceiling on GUM measures the `entity` group — the entity type: person, place, object, abstraction… This is the world model in miniature, and its result will show whether a full one is worth building.
- **An "is a kind of" hierarchy** with property inheritance: apple ⊂ fruit ⊂ food; crow ⊂ bird, so it usually flies.
- **Sources with a licence allowing commercial use:**
  - Wikidata — CC0, providing "instance of" and "subclass of". The full dump, 103 GB, from 17.09, moved to the data directory: `$MOVA_DATA/raw/wikidata-all/latest-all.json.bz2`. At the Geographer's old location there is a symlink;
  - Open English WordNet — CC BY 4.0, providing hypernyms.
  - ConceptNet — CC BY-SA, experiments only: SA does not go into Extended.
- **In the tale world** an object's class (oak — tree) references an ontology node. The reader infers through inheritance: "Can the crow fly?".
- **Learning:** the SLM holds a graph — memory, not a perceptron. The LLM confirms disputed nodes.
- **v1 (26.09):** crate `onto`, closed part. Details — `onto/README.md`.
  - **Graph:** OEWN 2025+ and FrameNet 1.7 — 133k concepts, 138k typed edges. A 19 MB snapshot loads in 15 ms; a query with a proof takes ~0.01 ms.
  - **Seeds:** 66 default properties with exceptions on the layers real world → genre → collection → tale. The most specific wins; the Nixon diamond — "undetermined" with both proofs.
  - **Frozen set** of 74 questions: blind round — 70/74, second — 71/74. All errors come from the OEWN sense order (dolphin — first sense fish, oak — wood), not from the logic.
  - **Controls:** 8 negative ones go red on a defect. A shuffled hierarchy drops IsA to chance level.
  - **Bridge:** fable and tale classes found at 99%; 14 questions about fables — 12/14 blind, 14/14 in the second round.
  - **Wikidata:** measurement of the full dump — ~15 h on one core, bzip2 is 96% of the time; the run plan is in the README.
  - **Wikidata, full pass (26.09):** 121.7M entities in 2 h → `$MOVA_DATA/db/wikidata.duckdb` (13.3 GB): statements P31/P279/P171…, labels in 637 languages, article titles of 364 Wikipedias, lexemes with forms and translations, the "is a kind of" closure, 53k OEWN synsets ↔ Q. Details — `onto/README.md`, "Wikidata: pass and database".
- **v2 (26.09): a Wikidata walker.** Details — `onto/README.md`, section "v2".
  - **A whale is not a fish:** the P171 chain stops at the class rank, with one step class → superclass (pike — fish). Controls whale, crow, shark, bat — green; without the rule the whale is a fish again.
  - **Stricter synset ↔ Q mapping:** two supporting sources, a generality threshold of 2%. 23.9k synsets accepted; manual precision 74% → 98%; shuffled labels — a 41× drop.
  - **Core:** 294 concepts → 2,171 seed candidates with source (Q, P, rank) and confidence. Numbers as distributions: "lion ≈ 157 ± 44 kg", not a single number.
  - **The LLM** gave a second opinion on 120 candidates for $0.33. Character (the fox is cunning) — only on the fable layer.
- **v3 (26.09): compiled core, context-based sense selection, description.** Details — `onto/README.md`, section "v3".
  - **Core in code:** `build.rs` stitches 294 concepts into `enum CoreId` and static records. Numbers are distributions `{mean, sd, min, max, n, unit}`, risks are probabilities, every value has a source (Q, P, rank, LLM confirmation).
    - Every "concept × field" cell is explicit; a deleted field — the build fails.
    - 1,304 values: 937 from Wikidata, all LLM-confirmed, and 367 risks.
  - **The "core ↔ graph" gate:** a contradiction is a refusal, not a silent choice. On real data it found 6 contradictions: nuts — food versus "object, not food".
  - **WSD v1:** Lesk + predominant sense from the corpus (McCarthy 2004) + core + domain. Frozen 60 sentences — 72% versus 35% for the "first sense" baseline. The v1 errors "dolphin — fish", "oak — wood" are gone.
    - Frozen set — 72/74, 0 wrong.
    - A news corpus hurts in tale sentences: mouse is pulled towards the device.
  - **`onto describe`:** a concept description with proofs. On 302 tale classes "unknown" — 64% → 55%, for animals 3.0 → 5.1 fields of 12.
    - A blind Opus judge: 2.88 → 3.01 of 5 on held-out, animals 2.95 → 3.31, food 2.83 → 3.34. A second phrasing of the rubric agrees, ρ = 0.75.
    - The worst descriptions come from a wrong word sense outside the core: flint — a river, shoes — "a situation".
    - The "student — architect" loop — one round so far; proposals not implemented.
  - **Pronoun API** (gender, animacy, human) — for the upcoming coreference.

### Measurement: existing AI benchmarks

Design note: use existing AI benchmarks for task performance.
- **We take open sets:**
  - maths — GSM8K and MATH, both MIT;
  - logic — bAbI, ProofWriter, RuleTaker, FOLIO;
  - common sense — CommonsenseQA, ARC;
  - reading — FairytaleQA, already present.
- **Each licence is checked before use.** NC sets — for measurement only.
- **We measure three configurations:** the SLM alone, SLM + LLM and the LLM alone. This shows where the SLM already pulls its weight on its own.

## Layers over the same data

One text — many parallel layers over the same tokens. Each layer rests on the lower ones and on the document memory, and sometimes gives them hints:

| layer | what it contains | gold standard |
|---|---|---|
| 1. tokens, sentences, paragraphs | boundaries | UD, GUM `newpar` |
| 2. morphology | UPOS, XPOS, FEATS, lemma | UD |
| 3. syntax | UD tree, later the metadialect (`docs/dialect.md`) | UD |
| 4. entities and coreference | who is who, pronouns, bridging | GUM, GENTLE `Entity=`, `Bridge=` |
| 5. discourse | relations between clauses and sentences: cause, purpose, elaboration, contrast | GUM `Discourse=` (eRST) |
| 6. pragmatics | speech act, what was meant, presuppositions, attitude | GUM `s_type`, dialogue-act corpora |
| 7. document passport | domain, style, narrator, purpose, mode, addressee | GUM `meta::*`, GENTLE and EWT genres |
| 8. time and place | deixis, dates from the document anchor | — |
| 9. domain layer | the text through the eyes of a domain: frames, slots, quantities, units | per domain |
| 10. story layer | who did what, to whom, when, where and why — at paragraph, chapter, book and series level | GUM (summaries, eRST), LitBank |

The "smartness" lies in the links between layers:
- syntax knows the domain: *shall* in law;
- coreference knows syntax (binding) and the passport (speakers in a conversation);
- pragmatics sees everything below.

Precedent — GUM, Georgetown's multilayer corpus. Its documents carry almost all the layers, so each layer is measured separately against its gold standard.

**Format:**
- one CoNLL-U with all layers: standard columns, `Entity=`/`Discourse=` in MISC, document and sentence comments in GUM style plus our own `# mova::…`;
- for RAG — a derived JSON per document: entities, resolved text, passport, "what was meant".

## Domain layer

Design note: besides the pragmatics layer there can be a domain layer that holds the features of the text and context as perceived through a domain, for example weather.

The same text reads differently depending on the domain. The document passport says which domain it is, and the domain layer decomposes the text into **domain frames**. Examples:
- **weather:** phenomenon, place, time and interval ("tonight", "this weekend"), quantities with units (°C/°F, mm, m/s), intensity, probability ("a 40% chance"), forecast or observation, source;
- **medicine:** symptom, diagnosis, drug, dose, frequency, negation ("no fever");
- **law:** parties, obligations and permissions (*shall*, *may*), deadlines, references to articles.

Each domain is a pluggable module:
- a schema as Rust enums;
- domain seeds: common expressions, units, pitfalls;
- extraction rules in the seed language;
- silver data from Opus.

The layer rests on the lower layers:
- syntax tells what a number is attached to;
- coreference — which city is meant;
- time — from which date to count "tomorrow";
- pragmatics — advice ("take an umbrella") or fact.

What it gives:
- **RAG:** domain facts as structure; the query "where will it rain tomorrow" searches by slots;
- **generation:** the same schema produces text back, as in the compressor.

**Weather is only an example.** The `weather` generator already has a weather schema (US and international data), so understanding and generation could speak one structure. We will start building domain modules once a domain is chosen.

## Story layer: hierarchical memory

Design note: or a story layer that holds a lot of info across sentence sessions. Who did what and why is a higher level of context, already the levels of paragraphs, chapters, books, book series and so on.

Memory of sentence → paragraph → chapter → book → series. At each level:
- **events:** who did what, with whom or what, when, where, how. As in `kg`: a fact plus participants with roles;
- **motives and goals:** why — from discourse (purpose, cause) and pragmatics;
- **event relations:** temporal order, cause and effect;
- **participant state:** who is where, what they know, what they want; how this changes;
- **open threads:** unfinished actions, questions, promises;
- **level summary:** of a paragraph, chapter, book.

**Across sessions.** The memory lives outside the context window, in a fact store (`kg`). When annotating the tenth chapter or the next book in a series, the annotator takes from it what it already knows and does not reread everything from the start.

**Who does what:**
- **Rust:** event frames from syntax (predicate and arguments → roles), stitching through coreference, time and cause — from discourse and tense forms.
- **Opus:** summaries and motives where real understanding is needed. This is the silver data that Rust learns from.

**Gold standard and data** (licences to be checked):
- GUM — five human summaries per document, eRST (purpose, cause, sequence), entities;
- LitBank — entities, events, coreference and quotations on novel excerpts, CC BY 4.0;
- corpora of chapter and book summaries, questions about novellas.
- Papers to fetch: narrative event chains (Chambers & Jurafsky), BookNLP and LitBank (Bamman), event coreference, temporal relations.

**For RAG** this is the top floor. A question like "why did X leave the company" or "what happened to Y in the second book" is answered from the memory of events and motives, not from separate chunks of text.

## Memory as an md graph

Design note: it is one thing when the annotator parses a single sentence. Another when it sees the preceding ones. Better still when it holds a summary of the whole conversation as an md document that also works as a graph, which can be walked to extract the original sentences and claims.

**Three levels of view:**
1. sentence only — this is how `annot` annotates now, in batches of random sentences;
2. sentence with the preceding ones — a sliding window;
3. a summary of the whole conversation or document — the md graph below. It is what gives pronouns across chapters, motives, the passport.

**Form.** One md document per conversation or book. A human reads it as text; code walks it as a graph:

```markdown
# Summary: <document>

## Passport
- domain: weather ^s1 ^s3 · purpose: warn ^s4

## Entities
### e3 — Barack Obama (person)
- mentions: ^s2 ^s5 ^s9
- known: US president ^s2; signed [[ev7]] ^s5

## Events
### ev7 — signing of a law
- who: [[e3]] · what: [[e12]] · when: 2010 ^s5 · why: … ^s6

## Summary by level
### Chapter 1 (^s1–^s40)
- … ^s12 ^s17

## Sentences
- ^s1 — <original sentence verbatim>
```

- **Every summary claim has an anchor** to its source sentences (`^s12`). Links between entities and events are wiki links `[[e3]]`, as in Mova's own memory.
- **Graph walk:** from a chapter summary to claims, from claims to entities and events, and from there to the original sentences. This is extraction of "what is said about e3" with evidence.
- **Gates:**
  - a claim without an anchor is not accepted;
  - the anchor must point to a real sentence, and that sentence must contain a mention of the entity from the claim.
  - Negative control: a deliberately wrong anchor must fail.
- **Built by Rust**, incrementally:
  - sentences are appended;
  - entities and events — from layers 4, 6 and 10;
  - level summaries — from Opus, but only with anchors.
  - Walk command: from a node — all claims and original sentences.
- **For RAG** this md graph is the expanded document for RAG. The retrieval chunks are claims with anchors to the original, so an answer always has a quotation.

## Five-year-old test

Design note: whole public-domain tales can be taken, and the SLM will show its trees and pragmatics and ask Opus whether it is already smarter than a five-year-old child or not yet.

**Corpus:** public-domain tales — Grimm (Hunt translation, 1884), Andersen (old translations), Aesop (Townsend, 1867), Perrault, Kipling's *Just So Stories*, Beatrix Potter. The language is simple, but understanding needs all the layers:
- who is who throughout the tale;
- who wants what and why;
- what led to what;
- the moral as what was meant.

**Gold standard — FairytaleQA** (ACL 2022). 278 children's tales from Project Gutenberg, with questions and answers written by educators. Question types:
- character, setting, action, feeling;
- causal relationship, outcome resolution, prediction.

Check the data licence before use.

**How the test runs:**
1. The SLM parses the tale with all layers and builds the memory md graph: trees, entities, events, motives, pragmatics.
2. The LLM is the examiner: it asks FairytaleQA-type questions.
3. The SLM answers with its memory commands, with anchors to the tale's sentences.
4. Scoring — against the FairytaleQA gold answers. An Opus judge only where there is no gold answer: it is not a full judge of itself.

**Yardstick:** accuracy by question type. "Five-year-old level" means explicit questions (who, what, where) almost without errors and sensible answers to "why" and "how did they feel". We compare with Opus reading the tale by itself, without the SLM's memory.

When: as soon as the entity, event and story layers exist. FairytaleQA (`2022.acl-long.34`) is already in the library.

**Training questions from the LLM.** Design note: Opus itself can generate lots of questions and answers per paragraph for training. For each paragraph of a tale Opus writes questions and answers:
- FairytaleQA types: character, setting, action, feeling, causal relationship, outcome resolution, prediction;
- a flag for whether the answer is explicit in the text or inferred;
- an anchor to the source sentences.

This is training data for the SLM's memory, story layer and motives: it learns to answer from the md graph. A `qa` table in the database: document, paragraph, question, type, answer, anchors, explicit or inferred. Pilot — ~300 paragraphs after the first pragmatics model, with the same Opus call module. The texts are public domain, so it is clean for a closed model.

## What this gives

- **Annotation:** who "he" is, what "it" is, what "this" is about. A sentence stops being an island: the annotator sees the document.
- **RAG.** A document chunk with "It was released in 2019" is useless until "it" is replaced by a name. The memory provides resolved text and an index of the document's entities. This is direct document expansion.

## Document memory

The annotator goes through sentences in order and maintains a memory.

- **Entities:**
  - number;
  - type: person, place, organization, object, abstract, time, event… — as in GUM;
  - features: Number, Gender, Person, animate or not;
  - names and name variants;
  - mentions (sentence, span, head);
  - salience: recency, syntactic role, frequency;
  - information status: new, given, inferable.
- **Discourse state:**
  - speaker and addressee (dialogue, direct speech);
  - time anchor (document date, narrative time) and place anchor;
  - open ellipses.

**Coreference v1 (26.09)** — `coref/` (closed part): a deterministic sieve over UD trees; every link has a sieve id and evidence (`coref explain`). GUM test, head-based CoNLL F1 without singletons: 60.3 on gold trees, 54.0 on SLM trees. Third-person pronouns — 62% accuracy. Numbers, errors and what is missing — in `coref/README.md`.

## Document level: domain, style, narrator, purpose, mode

Design note: where possible, the context should hold the domain, speech style, info about the narrator, the purpose and method of the narrative, and so on.

The memory holds not only entities but also the document "passport", updated as it goes:
- **domain:** science, law, medicine, news, sport, everyday life…;
- **genre and speech style:** academic, conversational, literary, business, poetry; spoken or written; register — formal or informal;
- **narrator:** who is speaking and from what position — first or third person, expert or eyewitness, attitude to the topic, number of speakers. We take only what the text says or clearly shows. Gender and other sensitive attributes are not guessed from a name;
- **purpose:** inform, persuade, instruct (instructions), entertain, ask, complain, evaluate (review);
- **mode:** narration, description, explanation, argumentation, step-by-step instructions, dialogue or interview;
- **addressee, time and place of action**, sentence types.

**Why:**
- annotation: in conversation *like* is a discourse word; in law *shall* is a modal of obligation;
- RAG: search by domain and purpose, ranking;
- generation: keeping the domain style. This requirement was already set for the compressor on 24.09.

**The gold standard already exists:**
- GUM:
  - per document: genre (16: academic, bio, conversation, court, essay, fiction, interview, letter, news, podcast, speech, textbook, vlog, voyage…), title, number of speakers, **salient entities** (`meta::salientEntities`) and **five human summaries** (`meta::summary1–5`) — the gold standard for summaries in RAG;
  - per sentence: type (`s_type`: decl, q, imp, frag, intj…), speaker and addressee (`speaker`, `addressee`), paragraphs;
  - eRST discourse relations (`Discourse=`): purpose and mode are visible from the relations purpose, elaboration, evaluation, sequence…
- GENTLE: genres outside GUM — dictionary, esports, law, medicine, poetry, mathematical proof, syllabus, threat letter.
- EWT: genres from sentence ids — answers, email, newsgroup, reviews, weblog.

**Output:** document and sentence comments in GUM style (`# meta::genre = …`, `# s_type = …`, `# speaker = …`) and our own fields `# mova::domain`, `# mova::purpose`, `# mova::mode`, `# mova::narrator`.

**Measurement:** accuracy of genre and sentence type on GUM, GENTLE and EWT. Salient entities — against `salientEntities`. For summaries — comparison with GUM's human summaries.

## Pragmatics: what was meant

Design note: pragmatics, or whatever it is called — the kind of thing that was meant.

The literal content of a sentence and what the speaker wanted to say often differ. The memory must hold the latter too.
- **Speech acts, direct and indirect.** "Can you open the window?" is a request, not a question about ability. "I'd appreciate a reply" is also a request. Also: statement, question, promise, threat, apology, complaint, evaluation.
- **Implicatures** (Grice). "Some of the students passed" → not all. "It's cold in here" → close the window. "I wouldn't buy it again" → a negative product review.
- **Presuppositions.** "He stopped smoking" → he used to smoke. "Even John came" → John was not expected.
- **Irony and sarcasm.** "Great, another Monday".
- **Attitude and certainty:** hedging (*maybe, I think, sort of*), politeness, evaluation.
- **Information structure:** theme and rheme, given and new (in GUM — the information status of entities), rhetorical questions.

**First cut (26.09).** Design note: now train the model on pragmatics, never mind that the AST is not up to par yet. Per sentence:

| field | values |
|---|---|
| `form` — form | `decl` declarative · `q` yes/no question · `wh` wh-question · `imp` imperative · `excl` exclamative · `frag` fragment · `intj` interjection · `other` |
| `act` — speech act | `assert` · `narrate` (narration) · `ask` · `request` · `offer` · `suggest` · `promise` · `thank` · `apologize` · `greet` · `agree` · `disagree` · `evaluate` · `express` · `warn` · `other` |
| `indirect` | yes/no: the form is not the usual one for the act ("Can you open the window?" — `q`, but `request`) |
| `hedge` | yes/no: *maybe, I think, sort of, perhaps* |
| `polarity` | `pos` · `neg` · `none`: the speaker's evaluation |
| `voice` | `narrator` · `character` (direct speech) · `none`: who is speaking in the text |
| `means` | briefly, what was meant. Written only by the LLM; the SLM does not learn it yet |

- **Data:** open texts from the database — tales (public domain) and Tatoeba (CC0). Silver data comes from the LLM, with the context of preceding sentences.
- **SLM:** an averaged perceptron per field, features from the UD tree, words and context (the previous sentence's act, direct speech). The model is transparent: you can see which features tipped the balance.
- **Memory instead of a perceptron.** Design note: try memory cells, something like a graph, instead of a perceptron. Memory-based learning (Daelemans & van den Bosch, TiMBL) stores examples and decides by the most similar ones:
  - **IGTree** — memory as a graph-shaped prefix tree, features ordered by gain ratio, the default label at each node; classification is a graph walk, and the explanation is the path taken;
  - **k nearest neighbours** with feature weights; the explanation is the most similar sentences from memory.
  - We compare with the perceptron on the same features and split.
  - Design note: it can have a couple of parallel types at once; if the perceptron shows better results in quality and even speed, that is a strong signal that the graph is not pulling its weight yet. Therefore:
    - the models share an interface and can run in parallel;
    - we measure quality and speed: training, prediction, size;
    - **model agreement is a confidence signal.** On disagreement the sentence goes to the LLM, as with "two paths to the goal".
- **Measurement:** on held-out silver data; `form` also against GUM `s_type`, for measurement only (GUM is NC).
- **Control:** shuffled labels give chance level.
- **Release boundary:** pragmatics is a separate crate, not `en`, so the licences of its data are tracked on their own (`licence/DATA.md`).

**Output.** Per utterance — the fields `# mova::act` (the act and its form: "request, question form"), `# mova::means` (what was meant, paraphrased), `# mova::presupposes`, `# mova::stance`. For RAG, "what was meant" is added to the literal text. The review "I wouldn't buy it again" is then found by a query for negative reviews.

**Who does what:**
- **Rust takes cues from the form:**
  - sentence type from the tree;
  - "can/could/would you + verb" → indirect request;
  - hedge words;
  - negation with evaluative words;
  - the particles *even, only, still*.
- **Opus.** Deep pragmatics — implicature, irony, the purpose of an utterance — is its domain: it provides the "what was meant" silver data. Rust learns cues from it, and pragmatics seeds are written from grammars and papers.

**Gold standard and data** (licences to be checked; NC — for internal measurement only):
- GUM: `s_type` and discourse relations;
- dialogue-act corpora (Switchboard DA and similar);
- Circa — indirect answers;
- IMPPRES — implicatures and presuppositions;
- sarcasm datasets.
- Papers on pragmatics to be fetched into the library together with papers on coreference.

## What we infer from context

1. **Pronouns and definite NPs:** he, it, they, this, the company → an entity in memory.
2. **Relative pronouns:** the antecedent from `acl:relcl`. For agreement this is already done: `en::ud::agreement`.
3. **Names:** "Barack Obama … Obama … the president".
4. **Bridging:** "the house … the door" — the door of that house. In GUM this is `Bridge=`.
5. **Deixis and time:** I/you — speaker and addressee; "yesterday", "last year" — a date from the document anchor; "here", "there".
6. **Ellipsis:** an omitted predicate (`orphan`) is restored from the previous clause.

## Standards and data

- **Output format:** CorefUD — `Entity=` in MISC, bracket notation: `Entity=(e3-person-new-…` … `e3)`. This is how GUM and GENTLE write it; UD remains valid.
- **Gold standard:**
  - GUM (several thousand tokens in entities per test set, many genres);
  - GENTLE — outside GUM's genres, a transfer test.
  - They also contain discourse (`Discourse=`, eRST) and morphemes (`MSeg=`) — for later.
- **Other open corpora** (licences to be checked): LitBank, WikiCoref, PreCo; pronoun test suites (`guillou-2018-pronoun-test-suite-evaluation-english` in the library).
- **Metrics** — as in CorefUD and CRAC: MUC, B³, CEAF-e → CoNLL F1, plus pronoun accuracy separately.

## How we build it

1. **Context seeds** — `en/seeds/context/`: anaphora, binding (reflexive pronouns — within the clause), pronoun–antecedent agreement, deixis. Sources — grammars (Jespersen, Poutsma, Curme) and papers from the library. Engine rules will be needed with access to `Entity` (the v1 engine lacks MISC).
2. **Rust, deterministically:**
   - mentions from the UD tree;
   - candidates from memory;
   - constraints: Number, Gender, Person, animacy, binding;
   - ranking by salience — Lappin & Leass for dependencies, the Stanford sieves for names.
   - Then — learned ranking (a perceptron on features of the "mention–candidate" pair) on GUM. On the CPU, in real time.
3. **Opus in the loop:** coreference silver data in edit mode. Rust proposes chains, Opus corrects, seed rules give hints.
4. **Measurement:** GUM test and GENTLE test, CoNLL F1 and pronoun accuracy. Negative control — random antecedents must fail.

## Order

The next wave of small models, once the current one (dialect and treebanks) returns:
- context seeds;
- the memory engine and basic pronoun resolution with a GUM scorer;
- fetching papers on coreference (CorefUD, CRAC, classics: Hobbs, Lappin & Leass, Stanford sieve).
