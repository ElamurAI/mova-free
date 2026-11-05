# world — worlds, schools and students

`world` is where Mova keeps the world of a text and where its students live: the course pupil of the primary schools, the maths student, the readers of fables and tales, and the self-directed learners (`induce`, `selfplay`). This README describes the tale world in detail and lists the other commands of the binary; [`docs/architecture.md`](../docs/architecture.md) gives the overview, and [`docs/context.md`](../docs/context.md#tale-world-entities-registries-cursor) holds the specification it implements.

## The tale world

The SLM module specified in [`docs/context.md`](../docs/context.md#tale-world-entities-registries-cursor): «Tale world: entities, registries, cursor», «Minimal objects», «Immutable states — the alternative variant», «World-change language», «Non-linear narrative».

- The LLM writes world changes as commands anchored to sentences.
- The SLM (this crate, Rust) executes the commands through gates, maintains registries and immutable states, and answers questions from the state and history, without the teacher.

The change language has three layers, run by one executor. The layer names `v1`, `v2`, `v3` are also the names of the commands, tests and run files that belong to each layer:

| layer | what it holds | checked on |
|---|---|---|
| v1, the base | places, objects, characters, relations, events, utterances | Aesop fables (Project Gutenberg #21) |
| v2 | intentions, beliefs, joint agents, feeling sets, parts, quantity, negation, a second time axis (`about=`/`when=`/`before=`) | FairytaleQA tales (Xu et al., ACL 2022, `2022.acl-long.34`) |
| v3, the narrative layer | canonical events with identity, story time as a partial order, frames with their own worlds, reality branches, observers' claims and knowledge, reveals | «The Speckled Band», Scheherazade's frames |

A linear narrative without frames and placements is the special case of v3: v1 and v2 runs give byte-for-byte the same output on the v3 executor ([below](#linear-narrative-is-the-special-case)).

Two question readers answer natural-language questions from the state: reader v1 (`ans.rs`) and reader v2 (`index.rs`, `qframe.rs`, `span.rs`, `reader.rs`), which merges the state answer with a span of the anchor sentence and logs the reason for every choice. Narrative-layer questions are asked through formal `qa3` queries.

## How it works

### Model (`world.rs`, `types.rs`)

- **Identity and immutable states.** An entity is a branch, a state is a commit.
  - Inside `C1@n` there is only the delta from `C1@n-1`, the cause (`cause=E…`), the command and the place in the text. Shared memory: no full copies.
  - From outside, `Entity::snap(n)` gives a full snapshot.
  - Every command gives every affected entity exactly one new state. Old states are only appended and never change (test `old_states_stay_immutable`).
  - Events refer to specific participant states: `agent C1@3`.
- **Registries:** locations `L`, objects `O`, characters `C`, relations `R`, events `E`.
  - `Relation` is a separate class with its own states: `kind` changes with a cause, `facts` are anchored events. The hero holds only references in `relations`.
- **Bidirectional links:**
  - `L.chars ∋ C ⇔ C.at = L`;
  - `L.objects ∋ O ⇔ O.at = L`;
  - `C.items ∋ O ⇔ O.at = C`;
  - `L.places ∋ M ⇔ M.in = L` — nested locations;
  - `R.a/R.b = C ⇔ C.relations ∋ R`.
- **Cursor:** `at` (location), story time, chapter, paragraph, sentence. The paragraph is taken from the database and checked against `cursor para=`.
- **Mentions:** one per line anchor, append-only.
- **`Val::Unknown` by default.** A character snapshot shows unknown gender and age as `?`.

### Change language, base layer v1 (`lang.rs`)

One command per line, anchors `^sN` at the end. A line spanning two sentences has two anchors: `^s3 ^s4`.

| command | what it does |
|---|---|
| `cursor [chapter= para= sent=] [at=L] [time=]` | where the narrative moved |
| `loc L type= [class=] [name=] [in=L]` | a location, nested in another via `in=` |
| `obj O type= [class=] [name=] [at=L\|C]` | an object in a place or held by a character |
| `char C type= class= [name=] [at=L] [attributes…]` | a character |
| `move C to=L` / `move O to=L\|C` `[cause=E]` | change of place: a character goes; an item fell or was picked up |
| `have C O [cause=E]` | first possession of an item that has no place yet |
| `give C O to=C [cause=E]` | handing over an item |
| `set X key=value … [cause=E]` | an attribute |
| `rel R kind= a=C b=C [cause=E]` · `rel R kind= cause=E` · `rel R fact=E` | a relation: start, change, fact |
| `event E verb= [agent= patient= to= at= instr=] [why= how=]` | an event |
| `say C\|narrator [to=C] act= [indirect=0\|1] [sincere=0\|1] [means=] [why=] id=E` | an utterance; `act` is the closed set `prag::schema::Act` |

**Closed sets:**
- location, object and character types;
- relation kinds;
- `act`;
- attribute keys: `name gender age mother father trait feeling sleep freedom life goal belief status`;
- values of `gender age feeling sleep freedom life`.

Parsing is strict; a format error is a rejection. This rejects:
- an unknown command or key;
- a duplicate key;
- a value outside the set;
- unclosed quotes;
- a line without an anchor;
- text longer than 25 words.

### Layer v2: intentions, beliefs, parts (`lang.rs`, `world.rs`)

Registries: `P` — intentions, `B` — beliefs. Both are classes with their own immutable states, like relations. Bidirectional links:
- `C.plans ∋ P ⇔ P.owners ∋ C`;
- `C.beliefs ∋ B ⇔ B.owners ∋ C`;
- `X.parts ∋ O ⇔ O.part_of = X`.

| command / key | what it does |
|---|---|
| `plan P kind= who=C[+C] [to=C] what="…" [src=E]` | an intention is opened. Kinds: `promise plan agreement wish threat order task curse prophecy bargain bet` |
| `plan P status=kept\|broken\|dropped by=E` | the intention is closed by a fulfilment or violation event, once |
| `believe B who=C claim="…" [about=X] [from=E] [true=0\|1]` | a belief: the claim, what it is about, where it came from, whether it is true in the tale world |
| `believe B status=dropped by=E` · `believe B true=0\|1 by=E` | abandoned; truth established |
| `done C cause=E` | the goal is reached: `goal` becomes unknown, state `done` with a cause |
| `unset C <key> cause=E [why="…"]` | a goal abandoned, a feeling or status passed; `why` is stored in the state |
| `feel C +a +b -c [cause=E]` | feelings are a set: add and remove, the rest stays; `set C feeling=a,b` sets the whole set |
| `detach O to=L\|C cause=E` · `attach O to=C\|O cause=E` | a part is detached (lying or in hand) · attached |
| `mention X [Y…]` | a mention of existing entities with no changes and no cursor shift |
| `obj … part=C\|O` · `num=` | a body part or part of a thing; a quantity (digits or `some few several many all pair`) |
| `loc … here=0` | a place only mentioned: the narrative cursor does not move there |
| `event … agent=C1+C2 … neg=1 cause=E about=X when=<word> before=E` | joint agents (likewise `patient`, `to`); an event that did **not** happen; a causing event; about a past event, intention or entity; story time separate from place in the text; a memory (happened before E) |
| `say C1+C2 to=C3+C4 act=request+promise … cause= about= when=` | in chorus; up to 3 acts per utterance |
| `rel … kind=benefactor\|master\|parent\|ruler\|captor\|suitor\|mentor` | directed: "a is the <kind> of b" (the Mouse is the Lion's benefactor, the Lion is the debtor). Also `spouse` |

The closed sets include the general vocabulary of tales (places, items, 26 feelings).

A `feeling` with a single tag is a `Tag`, as in v1, so the v1 tests and fables give the same output.

### Executor gates

A command is applied to a copy of the world. A violation is a rejection; the world does not change (test `rejected_command_leaves_world_unchanged`).

| gate | when red |
|---|---|
| reference | the id is not in the registry; an id is created twice; `cause`/`fact` points to an event not yet written |
| type | the id is from the wrong registry (`move C1 to=C2`); the attribute does not belong to this registry |
| bidirectional link | after the write some link is not mutual: `have C2 O1` while the meat is still with C1 or lies on the ground; changing a relation endpoint |
| teleportation | `set … at=`; an item "jumps" from character to character without `give` |
| nearby | picking up, handing over, dropping is possible only in the same location or a nested one (`in=`) |
| silent change | a known attribute value or relation kind changes without `cause=E`. Unknown → known is not a change but a discovery (like "Jackie was a boy" in the specification) |
| continuity | `event at=L` while the agent is not there; `give` of an item that is not there; giving to oneself |
| anchor | an anchor outside the text; anchors go backwards; `cursor para=` does not match the database |

v2 gates (negative controls — `v2_gates_are_not_mute`, `v2_links_catch_injected_mismatch`, `v2_format_is_strict`):

| gate | when red (v2) |
|---|---|
| silent change | an intention closed without `by=E`; a belief changed without `by=E`; `done`/`unset`/`detach`/`attach` without a cause; a known feeling changed without `cause=`; relation direction (a ↔ b) changed without a cause; a known `num` changed without a cause |
| continuity | an intention closed a second time; `kept` by an event with `neg=1`; `move`/`give`/`have`/`detach`/`attach`/`done` through an event that did not happen; `done` without a goal; `unset` of an unknown; `feel -x` when x is absent; a belief abandoned a second time; `attach` of something already a part; `detach` of something that is not a part |
| link | a part moved by `move`, `give`, `have` without `detach`; a part also lying separately; `plans`, `beliefs`, `parts` without a counterpart |
| nearby | `detach`/`attach` far from the whole |
| reference | `src=`/`from=`/`by=`/`cause=`/`before=`/`about=` to something not yet written; an event referring to itself; `mention` of a nonexistent entity; a nonexistent joint agent |
| continuity (joint) | `event at=L` while at least one of the joint agents is elsewhere |

The link gate is not built into the executor.
- `have` and a relation endpoint change write literally per the specification: only the named sides.
- So a mismatch really does arise, and after every command the full check `check_links` catches it.

### Narrative layer v3 (`story.rs`, `narr.rs`)

Specification — `docs/context.md`: "Non-linear narrative" (event identity, the v3 pipeline, partial order of time), "Hierarchical and global worlds", "Checking claims against the world", "Immutable states".

Layers v1 and v2 hold linear causality. Layer v3 holds three things separately:
- **what happened** — canonical events with identity;
- **what is believed about it** — claims and observers' knowledge;
- **in what order it is told** — the text cursor versus the story time axis.

#### Architecture

**A canonical event is one.** An event gets an `E` id at its first mention.
- After that the text only refers to it: claim (`claim`), knowledge (`know`), reveal (`reveal`), time (`time`).
- A resolution does not create a new event. It adds a role, cause or time to the existing one and makes earlier interpretations false.

**Two axes.**
- Text order — the sentence cursor, as in v1/v2.
- Story time — a **partial order**, not a timestamp.
  - Each event has two endpoints: start and end.
  - `BEFORE` / `AFTER` / `DURING` / `SAME` (≈) are `<` or `≤` edges between endpoints. This is point algebra, the tractable fragment of Allen's interval algebra, as in TimeML.
  - A contradiction — a cycle with a strict edge — is a gate refusal.
  - Every "earlier" is transparent: the answer reveals the path of edges with reasons (frame line, `before=`, `time` on line N, `reveal`, frame boundary).
  - Absolute time — only in the words of the anchor sentence (`time E at="…"`).

**The frame's narrative line.**
- An event told without explicit placement goes on the line: told in sequence — happened in sequence.
- `before=` / `after=` / `during=` / `same=` take the event off the line and place it relative to another.
- Without frames and placements the whole narrative is a single line: that is v1/v2.
- A flash-forward or a recollection needs its own time declaration: a placement of the event, `time` or `reveal` about it. Without one, events told before a recollection do not count as flash-forwards.
- The cause of a state change is no later than the next event on the line (a soft edge): the LLM writes a consequence (`set C17 life=dead cause=E97`) where the narrative reaches it.

**Frame stack.** A frame is who tells, to whom, the level, **its own cursor and its own world**. The **narrative level** (Genette) is counted by narrators (`Story::narr_level`): a recollection by the same narrator is a time frame, not a new level. Frame nesting is a separate number.

| frame | own world | time axis | what passes into the parent world on close |
|---|---|---|---|
| root (level 0) | the document world | own | — |
| `world=same time=past` (testimony, recollection, explanation) | projection of the parent world: identity and structure of entities, **without** places and transient states (feeling, sleep, goal, status, life) | the same, everything in the frame is before `before=`/`src=` | new entities (identity, stable fields, place); for existing ones only what the parent does not know; the past does not overwrite the present |
| `time=now` | the parent's world and line | the same | — |
| `world=new` (story within a story) | a new empty world | **own**, separate | events are foreign: they can be talked about, but they do not cause changes in the listener's world |
| `branch=dream\|lie\|plan\|if` | a copy of the parent world | own | nothing (it did not happen) |

- An interrupted frame is continued with the same id: the same parent, level and narrator. Its own world and cursor carry on.
- Entities of the frame are reproduced in the parent world by change-language commands through the parent's gates. A refusal is a warning in the md, not a stop.

**Reality branches.** Branches: `real` (happened), `dream`, `plan`, `intent`, `lie`, `if`, `report`, `belief`.
- Only `real` events change the canonical world.
- An event not in the `real` branch is taken off the narrative line.
- An event that has already changed the world cannot be moved into a lie or a dream.

**Observers' knowledge.** An observer is a character, the reader (`reader`) or the narrator.
- **Claim `K`** — a separate registry with its own immutable states, like v2 intentions and beliefs. It has an interpretation (`interp`) and a status from the closed set `believes | doubts | wrong | knows`.
- The status is changed by an event (`by=E`) or a reveal.
- **Who knows what, and from which sentence:**
  - explicit `know … from=^sN`;
  - a reveal — to its addressees (`to=`), to the reader — always;
  - the reader — every told event from its anchor;
  - a frame's listeners — its events;
  - a frame's narrator — from its opening.
- **A reveal is a change of knowledge, not of the past.** `reveal` does not create an event and does not overwrite an existing role.

**Pipeline** (`World::apply`, exactly per the specification):
1. Sentence cursor (anchors, monotonicity).
2. Frame: top of the stack, its world and line.
3. Mention of an event or claim.
4. Resolution into a canonical event (id reference) or a new event.
5. Story time axis: nodes, a line or placement edge, past-frame boundaries.
6. Reality branch.
7. Observer knowledge.
8. Gates decide whether the canonical world changes. On refusal the world is unchanged: the command runs on a copy of the root together with the frame worlds.

#### Commands of layer v3 (`lang.rs`)

| command | what it does |
|---|---|
| `frame open F<n> narrator=C\|narrator [to=C+…\|reader] level=<n> [world=same\|new] [time=past\|now] [before=E] [src=E] [branch=…]` | frame: level one deeper than the current; `level=0` names the root narrator (once, as the first frame); the same id again continues an interrupted frame |
| `frame close F<n>` | close the deepest open frame; what passes into the parent world is in the table above |
| `claim K<n> who=C\|reader\|narrator about=E status=believes\|doubts\|wrong\|knows interp="…" [from=E]` | an observer's claim about an existing event |
| `claim K<n> status=… by=E` | the observer changed their mind because of an event |
| `time E1 BEFORE\|AFTER\|DURING\|SAME E2` | relation on the story axis (one world) |
| `time E at="…"` | time in the words of the anchor sentence — absolute time only from the text |
| `branch E dream\|plan\|intent\|lie\|if\|report\|belief\|real [by=E]` | the event's reality branch |
| `know who=C\|reader fact=E\|K\|P\|… from=^sN [how=saw\|heard\|told\|inferred\|revealed\|guessed] [by=E]` | who knows a fact and from which sentence |
| `reveal E [to=…] [by=E] [interp="…"] [wrong=K+…] [right=K] [agent= patient= instr=] [cause=E] [before=\|after=\|during=\|same=E]` | revealing the truth about an existing event |
| `event … after=E \| during=E \| same=E` | placement at creation, in addition to v2's `before=`; only one |

Closed sets: branches, claim statuses, knowledge sources, frame world and time. Parsing is strict:
- `frame open` lacks `narrator=` or `level=`;
- the narrator is the reader;
- `world=new` together with `time=`/`before=`;
- `branch=real` on a frame;
- a claim without `interp=`;
- `time` without a relation or with two;
- `know` without `from=^sN`, `who=narrator`;
- `reveal` with nothing revealed, or with the same `K` in both `wrong` and `right`;
- two placements on one event.

All of these are format refusals (`v3_format_is_strict`). `mention E…` remains a format error, as in v2: an existing event is mentioned by reference (`claim`, `know`, `reveal`, `time`), not as a new event.

#### Gates of layer v3

| gate | when red |
|---|---|
| frame | the level is not exactly one deeper; closing a frame that is not the deepest; closing the root; naming the root a second time; continuing with a different parent, level or narrator |
| story time | a cycle in the partial order (`time`, placement, reveal, frame boundary); an `at="…"` that is not in the anchor sentence |
| teleportation (in story time) | in the present, an event where the character was not just before on the story axis: a recollection without a time marker. In a past frame — the same inside its world, where the neighbours are the frame's states, not the present ones |
| branch | a state change by an event not of this branch (dream, lie, plan); `branch` of an event that has already changed the world; `know` of an event not in the `real` branch |
| world boundary | `time` or placement across worlds; a change in the listener's world caused by an event of a story within a story |
| event identity | a character's second death with a different cause — the same event twice; `reveal` overwrites an existing role |
| reference | `claim`/`know`/`reveal`/`time`/`branch` about something nonexistent; `reveal` of a new event ("a reveal does not create events"); a claim about a different event in `wrong=` |
| continuity | a dead character acts in the present |
| anchor | `know from=^sN` later than the command's sentence |
| silent change | a claim's status changed without `by=E` |

#### Linear narrative is the special case

- **Two executor modes.**
  - `World::new` — v2: linear narrative, v2 gates with the same names. The narrative layer is here too: v3 commands work.
  - `World::new_v3` — adds gates in story time:
    - teleportation on the story axis (in v2 this is continuity);
    - a dead agent in the present;
    - a second death.
- **Regression check** (`WORLD_V3=1` — v1/v2 commands on the v3 executor). From the same `cmds-*.txt`, the fables and the FairytaleQA tales are rebuilt with `ftqa-build` (worlds), `ftqa-eval` (reader v1) and `reader-eval` (reader v2); the worlds and answers must match the linear executor's output byte for byte (test `linear_v1_v2_is_special_case_of_v3`).

### Queries (`query.rs`, `qa.rs`)

`world query <doc> <dir> <query>`. Entity description — `C1`, `char:fox`, `obj:meat|flesh`, `loc:tree`. `a || b` — a second reading if the first gave no answer.

**State and place:**
- `state X ^sN` — the state of X at sentence N;
- `where X ^sN|@first|@last` — where X is;
- `holder O …` — who has O;
- `holders O` — the whole place history;
- `diff X a b` — the difference between `X@a` and `X@b` with steps and causes;
- `mentions X` — mentions;
- `attr X key T` — an attribute value;
- `cause X key=value` — when and why the attribute took the value.

**Events:**
- `chrono` — chronology;
- `say X [to=] [act=]` — utterances;
- `why verb=…` — event motives;
- `who verb=…` — who performed the event;
- `events agent=X from= to=` — a character's events.

**Relations:**
- `rel X Y` — relation history;
- `predict-rel X Y` — prediction from the last relation kind.

**Working memory** (`context_at`, the view from the cursor):
- main and secondary characters (by attention weight);
- speaker and addressee;
- what is happening and why;
- where and when.

**md graph:** `md.rs` builds a human view — the text, the cursor by sentence, registries, hero states as deltas, relations, chronology, working memory and gates.

### LLM (`vmm.rs`)

**Prompt:**
- the change language;
- closed sets;
- gates;
- an example on a different story (a girl and a dog). A test checks that the example passes all gates;
- the numbered sentences of the fable with paragraphs.

**Call:**
- per fable — one call via `prag::opus` (the teacher, empty directory, no tools);
- a retry — one, only if the format gate rejected lines, with the same prompt and the rejection explanation;
- existing `cmds-<doc>.txt` are not overwritten.

Accounting — `calls.jsonl`.

**Layer v2 prompt and calls** (`rules_v2`, `body_v2`, `call_v2`):
- **Prompt.** Language v2, closed sets, gates. An example (the invented «The Miller's Goose») shows all v2 commands; a test checks that it passes the gates. The LLM is warned that questions will be answered only from the state.
- **Chunks.** Up to 95 sentences per call, split at a section boundary closest to the middle. The second chunk receives the preceding text, a compressed world (registries, states, open intentions, latest events, cursor) and the next free ids.
- **Retry.** One per tale, only on a format rejection, and only for the rejected lines: `@N command` are inserted in their places. At most 3 calls per tale in total.

`world` depends on `prag` (`opus`, `schema::Act`, `data::duck_query`) and on `en` — only dictionary reads in code: lemmas of question words and past tense of verbs in answers.

**Layer v3 prompt and calls.** Parts of up to 95 sentences (`V3_PART`). The second and later parts receive the preceding text and a compact v3 world: entities of the current frame's world, frames with open ones marked, claims, off-line events, the latest events from all worlds, free ids from all worlds. The prompt forbids forward references (an event referred to before the line that writes it). One format repair per part, only for rejected lines. `v3-estimate` gives an upper cost bound before a run (800 output tokens per sentence), and a ceiling (`WORLD_V3_BUDGET`) is checked before every call.

### Question reader v1 (`ans.rs`)

Reader v1 answers natural-language questions from the state alone, with no teacher:
- parses the question: intent, subject, verb, a "time anchor" after `when/after/because`;
- looks up state facts by lemmas from the `en` dictionary; the question's sections take priority, as in FairytaleQA, where the model sees the section;
- builds the answer from slots: the event's `why`, the cause or consequence event, the feeling at the anchor time, goal, intention, belief, participant, place, number;
- no slot means **"not in state"**; a separate mark, **"anchor"**, is the sentence of the best fact found, as a fallback.

Reproducible evaluations bind the reader source, executable, model and inputs in their run artifacts (`freeze*.txt`); reader v1 (`ans.rs`) is the `freeze-2` reference column of the reader reports. A historical freeze does not establish the identity of the current build.

### Question reader v2 (`index.rs`, `qframe.rs`, `span.rs`, `reader.rs`)

Reader v2 is a set of separate modules next to v1, with no teacher calls. It works on already built worlds.

1. **A single fact index** (`index.rs`). Facts of a tale, each with sentence anchors, a cause (`cause`) and story order:
   - events: agent, object, addressee, place, instrument, `how`, `why`, `when`, `about`, negation;
   - utterances: speaker, addressee, acts, `means`, `why`;
   - intentions: kind, holders, text, toward whom, state and the fulfilling event; beliefs: proposition, about what, truth value;
   - attributes by state: traits, status, goal, feeling (only **added** tags), age, sex, sleep, freedom, life, number, name, place — with the `unset … why=` explanation;
   - objects in hand, parts, names of characters and places with `in=` ancestors, relations.

   Search uses lemmas from the `en` dictionary with field weights: predicate +3; subject in its role +3, elsewhere +1, absent −2; another participant +1.5; content word — the field weight (predicate 1.5; texts and participants 1; time and "about" 0.5); the question's section +3, adjacent +0.5, farther −1.5; for a prediction the next section +2.5.
2. **Parsing the question with a UD tree** (`qframe.rs`; the model is `WORLD_EN_MODEL` or `$MOVA_DATA/en/models/en.bin`).
   - A subordinate clause ("when/after/because/if/as soon as …") is a separate clause: the reader first looks up its fact — the time anchor — and then answers about the main clause.
   - Main clause: question word, auxiliary, predicate; participants come from the tree of the **declarative** sentence "subject + did + predicate + rest". The tagger knows such sentences better than lowercase questions.
   - Predicate repair with the `en` dictionary: the candidate must be a VERB in the declarative tree; "to V" is an xcomp of the previous candidate.
   - Names outside the dictionary (assipattle) are capitalized before tagging; "ashamed" is not (a familiar suffix).
   - Question type → index field: `feel` → feeling; `why` → why / causing event / goal / intention / traits of the loved one; `happen` → consequences (`cause=` anchors), otherwise the following events; `have` → objects **and parts**; `who` in the passive → patient; "who did X think" → belief; "what would happen if" → content of the utterance.
3. **Span of the anchor sentence** (`span.rs`). FairytaleQA answers to explicit questions are text spans. Predicate node (lemma from the question or from the fact) → the subtree of the role:
   - subject, object, place, time, cause (advcl because/for/to; "X, so V"; "so ADJ that"), manner;
   - feeling — only feeling words with an intensifier ("sorely troubled");
   - "what happened when X" — the main clause after the subordinate one, or the following coordinated predicates;
   - a question echo (a span made mostly of the question's own words) is not an answer.
4. **Merging** (`reader.rs`) — a transparent rule; the reason is written to the log of every answer:
   - fact score < 6 (`CONF`) or no fact → **baseline sentence**: "state not confident (score 4.0 < 6)", "state is silent";
   - otherwise, if the question's words cover ≥ 50% of the fact's anchor sentence (`COVER`; explicit **by our measure**, not by the dataset label) and a span is found → **span** with the rule name: "state: score 10.0 ≥ 6; question in sentence ^s38 at 50% → span [conj-after]";
   - otherwise → **fact field**: "state: score 12.5 ≥ 6 → fact field";
   - a feeling with a feeling word in the anchor sentence, and "what happened" — always a span; description and number — a field.

   A search without a predicate match and with fewer than two words is never confident.

### Evaluation data and protocol

- **FairytaleQA** (Xu et al., ACL 2022, `2022.acl-long.34`): Hugging Face `WorkInTheDark/FairytaleQA`, revision `655b20b`, Apache-2.0 according to the card; measurement only, never training (`$MOVA_DATA/raw/fairytaleqa/SOURCES.md`). The HF copy holds one row per question and section, so `ftqa.rs` reconstructs each tale from its sections; sections without a question are missing, and the text is lowercased and tokenized as in the paper. The LLM writes a tale's world without seeing its questions.
- **`ftqa-eval`, `reader-eval`** (`report.rs`, `rreport.rs`): ROUGE-L F1, max over the two references as in the paper (lowercase Latin letters and digits, no stemming); slices by question type and explicit/inferred; the share of "not in state"; a **baseline** — the sentence of the question's sections with the largest lemma overlap with the question; manual samples scored 1 / 0.5 / 0 plus whether the answer is in the state at all. Questions seen during development are excluded by `--exclude`/`--skip`; a reader is frozen (`freeze*.txt`, source hashes) before a blind run. Reader v2 controls: `--control shuffle` (facts of another tale) and `--control noindex` (must equal the baseline).
- **v3 testbeds** (Project Gutenberg, public domain): Conan Doyle, *The Adventures of Sherlock Holmes*, PG #1661, "VIII. The Adventure of the Speckled Band" (`speckled-band`, a detective story resolving the past); Andrew Lang (ed.), *The Arabian Nights Entertainments*, PG #128, from "The Arabian Nights" to "The Story of the Fisherman" (`arabian-nights`, frames within frames). They are fetched through the `sources` pipeline (manifest → plan → `curl -K` → verify), cut by `poly-prep` into sentences with `en::tok`, and used for measuring the world only. Headings of nested stories are separate sentences: they open frames.
- **v3 questions** (`questions-v3.tsv`) of the types time, knowledge, frames and control are written before the first LLM call, each with a formal `qa3` query and an expected answer, and hashed in `freeze.txt`. An event in a query is described by verb variants, a participant and a sentence range (`ev:die|death/char:julia@s85-160`), because ids are unknown before the run. `v3-eval` scores the expected words in order plus anchors within range.

### Commands

```
export PATH=$HOME/.cargo/bin:$PATH
cargo build --release        # from the repository root; all paths below are root-relative
# layer v1: fables
target/release/world prompt 2451                                   # prompt without a call
target/release/world vmm 2451 $MOVA_DATA/runs/<run>      # LLM → cmds-2451.txt
target/release/world build 2451 $MOVA_DATA/runs/<run> --questions world/data/questions.tsv
target/release/world query 2451 $MOVA_DATA/runs/<run> diff char:crow 1 7

# layer v2 on FairytaleQA
R=$MOVA_DATA/runs/<v2-run>
target/release/world ftqa-import $MOVA_DATA/raw/fairytaleqa/hf-test.csv $MOVA_DATA/raw/fairytaleqa
target/release/world ftqa-prompt golden-goose                  # v2 prompt without a call
target/release/world ftqa-vmm golden-goose $R                  # LLM v2 in chunks → cmds-golden-goose.txt
target/release/world ftqa-build golden-goose $R                # gates → world-golden-goose.md
target/release/world ftqa-eval $R --sample 10 --seed 2 --exclude $R/seen-questions.tsv
target/release/world ftqa-parse questions.tsv --frame          # question parsing (development)

# Question reader v2 (en model: WORLD_EN_MODEL or $MOVA_DATA/en/models/en.bin)
O=$MOVA_DATA/runs/<reader-run>
target/release/world reader-parse questions.tsv                # UD tree parsing (validation)
target/release/world reader-span-dev $O/valid [--show why]     # spans on validation, without the world
target/release/world reader-eval $R $O --dev $O/dev-questions.tsv   # development: seen questions only
target/release/world reader-ask golden-goose $R 4 [--facts]    # one (allowed) question in detail
target/release/world reader-eval $R $O --sample 10 --seed <seed> \
    --exclude $O/dev-questions.tsv --exclude $R/freeze-2/manual-sample.tsv --exclude $O/excluded-extra.tsv
target/release/world reader-eval $R $O --skip $O/dev-questions.tsv  # table on the non-dev questions
target/release/world reader-eval $R $O --control shuffle|noindex    # negative controls

# narrative layer v3
R=$MOVA_DATA/runs/<v3-run>
# sources: manifest → plan → curl → verification
target/release/sources plan $MOVA_DATA/raw $R/queues $R/sources.jsonl
curl -sS --fail -K $R/queues/curl-www.gutenberg.org.cfg --create-dirs
target/release/sources verify $MOVA_DATA/raw $R/sources.jsonl
# testbeds: cut out a story, sentences (en::tok)
target/release/world poly-prep $MOVA_DATA/raw/en-narr-pg1661-doyle-holmes/pg1661.txt \
    "VIII. THE ADVENTURE OF THE SPECKLED BAND" "IX. THE ADVENTURE OF THE ENGINEER’S THUMB" $R/stories/speckled-band.tsv
target/release/world poly-prep $MOVA_DATA/raw/en-narr-pg128-lang-arabian-nights/pg128.txt \
    "The Arabian Nights" "The Story of the Fisherman" $R/stories/arabian-nights.tsv
target/release/world v3-estimate $R speckled-band arabian-nights   # cost estimate before the run
target/release/world v3-prompt arabian-nights $R 1                # prompt of a part, no call
target/release/world v3-vmm arabian-nights $R                     # LLM in parts, cap WORLD_V3_BUDGET
target/release/world v3-build arabian-nights $R                   # gates → story-arabian-nights.md
target/release/world v3-query speckled-band $R order ev:die/char:julia ev:arrive/char:helen@s10-40
target/release/world v3-eval $R $R/questions-v3.tsv               # frozen questions → answers-v3.md
target/release/world v3-cost $R
WORLD_V3=1 target/release/world ftqa-build golden-goose $MOVA_DATA/runs/<v2-run>   # v1/v2 runs on the v3 executor
```

### Other commands

The binary has many more subcommands than the layers above. One line each, with the page that documents it (run from the repository root as `target/release/world <command> …`):

| commands | what they do | documented in |
|---|---|---|
| `babi`, `babi-contrast`, `babi-learn`, `babi-probe` | bAbI tasks 1–20, the contrast set, learning, one-sentence probe | [tests/README.md](../tests/README.md), [architecture.md](../docs/architecture.md) |
| `stepgame`, `stepgame-contrast` | StepGame relative positions and its contrast set | [tests/README.md](../tests/README.md) |
| `spartqa`, `spartqa-contrast` | SpartQA-Human spatial questions (NC data, measurement only) | [tests/README.md](../tests/README.md) |
| `school`, `treeread` | the school's maths student (`school actors cv`, `held`) and the tree reader for word problems | [tests/README.md](../tests/README.md), [architecture.md](../docs/architecture.md) |
| `knowledge` | knowledge courses: `course`, `retain`, `bench`, `diagnose`, lesson checks and safe edits | [school.md](../docs/school.md) |
| `lessons`, `meanings`, `numbers`, `mentions`, `opchoice`, `primers` | lessons as data, word meanings from the pupils' dictionary, number words, quantity mentions, choice of operation, arithmetic primers | module docs in `world/src/<name>.rs` |
| `quant-vmm`, `quant-stats`, `quant-check` | the quantitative layer: teacher-written world scripts for dataset problems, their statistics and a check of one script | module docs in `world/src/quant.rs` |
| `state`, `selfplay`, `self`, `brain`, `family`, `letters`, `tune`, `big-explore`, `big-judge`, `big-diff` | states with deltas and rollback, self-play, the brain, the family channel, big-text samples | [training.md](../docs/training.md) |
| `store` | the tree store of book sentences per model version | [release 0.3 notes](../releases/0.3.md) |
| `induce`, `rediscover`, `rediscover-random`, `repair-eval`, `conllu-split` | induced tree-repair rules, their evaluation, the even/odd tales split | [training.md](../docs/training.md) |
| `derive`, `derive-learn`, `derive-scan` | trees by logical derivation | [english.md](../docs/english.md) |
| `absurd-check`, `absurd-events`, `absurd-learn`, `absurd-repair`, `absurd-rerank`, `sense-check`, `idioms` | the absurdity matrix and what uses it | [knowledge.md](../docs/knowledge.md) |
| `events`, `events-show`, `book-answer`, `gaps` | event statistics of books, answering from a book's event index, the gap journal | [pragmatics.md](../docs/pragmatics.md) |
| `frames`, `qud` | pragmatic frames and questions under discussion | [pragmatics.md](../docs/pragmatics.md) |
| `domains`, `domains-eval`, `register-stats` | domains and registers of expressions | [pragmatics.md](../docs/pragmatics.md) |
| `read`, `read-*`, `values` | the reader of levels 1–3 and its checks | [architecture.md](../docs/architecture.md) |
| `nerves`, `trigger` | the nerve graph and triggers (words hooked to code) | [knowledge.md](../docs/knowledge.md) |
| `know`, `ideas`, `principles` | general statements, the global graph of ideas, derived principles | module docs in `world/src/<name>.rs` |
| `tabular` | tables (Kaggle-style tasks) read into the world with an explained decision list | [training.md](../docs/training.md) |
| `worldbook`, `worldbook-*` | world books of one topic with a citation per sentence | module docs in `world/src/worldbook.rs` |

## Limits

**The world.**
- `why` is free text: it does not refer to a goal or intention (`for=P`), and the LLM sometimes writes it as a tautology or a circumstance instead of a cause ("the tree was in the background"). Both the state and the readers then fail on causal questions.
- An object cannot lie inside an object (a chest with treasure).
- No nested beliefs ("X thinks that Y thinks").
- Feelings have no target (`about=` in `feel`).
- Narrative level and time frame: `Story::narr_level` counts levels by narrators, but the language still requires `level+1` for `frame open … time=past` even when the narrator does not change.
- No single view of states on the story axis: a character's states in a past frame live in the frame's world, and two frames about the same time are reconciled only through canonical events. The identity gate does not see a second death in a neighbouring past frame.
- No parallel lines (several cursors on a shared axis): `time=now` continues the parent's line.
- No global entities or epochs across tales ("Hierarchical and global worlds" in `docs/context.md`: `scope=global|book|tale`).
- The LLM writes the world changes; no SLM scribe writes frames or `time` from the UD tree yet.

**The readers.**
- Merging does not depend on the question type: for "who is X" a span can win over the character's name, and a subject span is taken even without `nsubj`.
- Name linking is literal: "the old woman" does not link to a character named "the old lady in green".
- A feeling is taken as "the first one caused by an event involving X", not by the time anchor; synonyms of feeling tags and answers (annoyed ↔ upset) are not matched.
- Narrative-layer questions have no natural-language reader: they are formalised by hand as `qa3` queries before the run.

**The evaluation.**
- ROUGE-L penalises synonyms ("afraid" / "terrified", "seven" / "7"); there is no teacher judge.
- The fable questions (`data/questions.tsv`), their queries and expected answers were written by the same agent as the prompt, and the closed sets were compiled with the fables in mind; the fable score (words of the expected answer, in order) is lenient.

## Tests

The tale-world tests are in `src/tests.rs` (v1, v2, reader v2) and `src/tests3.rs` (v3); `cargo test --release -p world` runs them with the rest of the crate's tests. Every gate has a negative control that must turn red.

- **v1:** the prompt example passes all gates (positive control); one negative control per gate (teleportation, reference, silent change, two-way link both via commands and via injection bypassing the executor, nearness, anchors, strict format and closed sets); old states stay immutable (`old_states_stay_immutable`); a rejected command leaves the world unchanged (`rejected_command_leaves_world_unchanged`); state queries and an honest "not in state"; the questions file and order-aware scoring; `real_vmm_output_and_its_mutations` — real LLM output for "The Fox and the Crow" (`data/cmds-2451-fox-crow.txt`) is clean and each mutation turns red on its own gate.
- **v2:** `prompt_example_v2_passes_all_gates` (the v2 example passes and the state is as intended); `v2_gates_are_not_mute` (every new command turns red at its own gate); `v2_links_catch_injected_mismatch` (mismatches in `plans`, `parts`, `owners` injected bypassing the executor); `v2_format_is_strict`; `real_vmm_v2_output_and_its_mutations` (real LLM v2 output, `data/cmds-v2-wen.txt`, and its mutations); `natural_questions_are_answered_from_state` (reader v1 on the v2 example, with "not in state" controls).
- **Reader v2:** `reader_parses_questions_with_ud` (type, subject, anchor clause, predicate repair); `reader_cuts_spans_from_anchor_sentences` (place, cause, feeling spans; a question echo is not an answer); `reader_v2_answers_from_state_and_controls` (without the index the answers equal the baseline; facts of another story give few state hits).
- **v3:** `prompt_example_v3_passes_all_gates` (the invented "The Cat and the Vase": own frame world and cursor, an event told late but happened early, transitive "before", claim statuses through a reveal, a lie off the line, the past not overwriting the present); `linear_v1_v2_is_special_case_of_v3`; `flashback_without_time_mark_is_teleport`; `claim_does_not_change_canonical_world`; `reveal_does_not_create_second_event`; `v3_gates_are_not_mute`; `v3_format_is_strict`; `time_is_a_partial_order` (transitivity, DURING, SAME, unknown stays unknown, closure, `sort`); `real_vmm_v3_output_and_its_mutations` (real LLM v3 output, `data/cmds-v3-merchant.txt`, and mutations caught at the identity, time, world boundary, frame and branch gates); `narrative_level_counts_narrators_not_frames`; `dream_frame_does_not_change_canonical_world`.

## Files

**Crate:**
- `src/types.rs` — ids, states, closed sets, values (registries `P`, `B`, intention and belief fields, `Tags`);
- `src/lang.rs` — parsing of the change language, layers v1–v3;
- `src/world.rs` — model, executor, gates;
- `src/query.rs` — state queries;
- `src/md.rs` — md graph (intentions, beliefs, event details);
- `src/qa.rs` — v1 formal queries;
- `src/vmm.rs` — prompts and the teacher calls (`rules_v2`, examples, parts, repair retry);
- `src/ftqa.rs` — FairytaleQA: CSV, reconstructing tales from sections, sentences;
- `src/ans.rs` — reader v1, baseline and ROUGE-L scoring;
- `src/report.rs` — evaluation of reader v1: slices, samples, manual verdicts;
- `src/tree.rs` — UD trees (`en::annotate`, model loaded once per process);
- `src/qframe.rs` — question parsing with a UD tree;
- `src/index.rs` — unified index of state facts and lemma search with field weights;
- `src/span.rs` — anchor-sentence span by the role of the question word;
- `src/reader.rs` — reader v2: state answer, merge with a reason, span development on validation;
- `src/rreport.rs` — evaluation of reader v2: controls, sample, manual verdicts;
- `src/story.rs` — v3 narration layer: story time axis (partial order, path explanation, closure), frames, event metadata, knowledge, reveals;
- `src/narr.rs` — v3 pipeline at the root: frames (own world: past projection, new world, branch; closing carries entities into the parent's world by commands through gates), events on the axis, `claim`/`time`/`branch`/`know`/`reveal`;
- `src/qa3.rs` — v3 queries (time, knowledge, frames) and scoring with anchors;
- `src/poly.rs` — testbeds: a story cut from a Gutenberg book → sentences (`en::tok`);
- `src/tests.rs`, `src/tests3.rs` — tests;
- `sql/fable.sql`;
- `data/questions.tsv` — fable questions, queries and expected answers;
- `data/cmds-2451-fox-crow.txt`, `data/cmds-v2-wen.txt`, `data/cmds-v3-merchant.txt`, `data/v3-merchant.tsv` — real LLM output (and, for v3, the text: Lang, PG #128, public domain) for the mutation tests.

**Run directory** (`$MOVA_DATA/runs/<run>/`, written by the commands above):
- `prompt-*`, `raw-*` (raw LLM output), `repair-*`, `cmds-<doc>.txt` (accepted commands), `calls.jsonl` (teacher-call accounting), `log-vmm-*.txt`;
- `world-<doc>.md` (v1/v2 md world), `story-<doc>.md` (v3 md world), `answers-<doc>.md`, `answers-v3.md`;
- `seen-questions.tsv`, `dev-questions.tsv` — questions seen during development, excluded from evaluation samples;
- `freeze*.txt`, `freeze-*/` — hashes of sources and model frozen before an evaluation, with its answers and manual verdicts;
- `report*.md`, `answers*.tsv`, `manual-sample.tsv`, `manual-verdicts.tsv` — evaluation reports, controls and manual samples;
- v3: `sources.jsonl`, `queues/`, `pages/` (sources and verification), `stories/*.tsv` (testbed sentences), `questions-v3.tsv`.

## Licence

Elamur Cosmic License ([LICENSE.md](../LICENSE.md)). Third-party data keeps its own terms, as noted where it is used.
