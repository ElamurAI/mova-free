# world — worlds, schools and students

`world` is where Mova keeps the world of a text and where its students live: the course pupil of the primary schools, the maths student, the readers of fables and tales, and the self-directed learners (`induce`, `selfplay`). This README is the long history of the crate, newest sections further down; [`docs/architecture.md`](../docs/architecture.md) gives the overview.

## The tale world

A prototype of the SLM (small language model) module from `docs/context.md`: «Tale world: entities, registries, cursor», «Minimal objects», «Immutable states — the alternative variant», «World-change language», «Non-linear narrative». It started as a prototype on an Aesop fable, then grew into change language v2 with an independent check on FairytaleQA, and then into v3, the non-linear narrative.

- The LLM (Opus, high) writes world changes as commands anchored to sentences.
- The SLM (this crate, Rust) executes the commands through gates, maintains registries and immutable states, and answers questions from the state and history, without Opus.


**v2 in brief (2026-09-26):**
- **Change language.** Intentions (`plan`: open/kept/broken/dropped with a fulfilment event), beliefs (`believe`), several acts per utterance, joint agents, `done`/`unset`, feeling sets, `about=`/`when=`/`before=`, part–whole, number, negation, directed relations, `mention`. Gates and negative controls for every new command. The old 13 tests and three fables are unchanged.
- **FairytaleQA, 10 test tales, 432 questions.**
  - The LLM wrote 2055 lines, the gates rejected 0, 17 calls.
  - Blind SLM reader: ROUGE-L 0.185, "not in state" 32%. The baseline (most similar sentence of the section) — 0.222.
  - Manual check: SLM 34%, baseline 54%.
  - Yet the answer is in the state for 84% of questions. The weak spot is the question reader, not the world.
  - A second, NON-blind reader round: 0.226 / 18%. On a fresh sample SLM 41%, baseline 49%.
  - The state beats the baseline on feelings, predictions and inferred questions, and loses on explicit "who/what/why". Details [below](#fairytaleqa-the-five-year-old-test-2026-09-26).
- **Question reader v2** (same day, without a single Opus call): a unified index of state facts, question parsing with a UD tree, a span of the anchor sentence, a "state + sentence" merge with the reason for the choice.
  - On 305 questions unseen during development: ROUGE-L 0.294 against baseline 0.229 and reader v1 0.213.
  - Manual check on a fresh sample (53): v2 57%, baseline 54%, v1 38%; the answer is in the state for 92%.
  - Controls: facts from another tale — state 0.003, merge below baseline; without the index — exactly baseline. Details [below](#question-reader-v2-2026-09-26).

**v3 in brief (2026-09-26, non-linear narrative):**
- **Architecture.** The world separately keeps what happened (canonical events with identity), what is thought about it (claims `K` and the knowledge of observers — characters, reader, narrator) and in what order it is told (the text cursor versus the story time axis).
  - Time is a **partial order** on event endpoints (BEFORE / AFTER / DURING / SAME), with an explanation for every "earlier".
  - A **frame stack**, each frame with its own cursor and its own world: testimony, memory, story within a story, branch.
  - **Reality branches.** The canonical world is changed only by events that actually happened.
  - **A reveal is a change of knowledge**, not a new event.
  - The pipeline follows the specification exactly. Details [below](#world-v3-non-linear-narrative-2026-09-26).
- **The change language** is extended with the commands `frame`, `claim`, `time`, `branch`, `know`, `reveal` and the placements `after=`/`during=`/`same=`. Each comes with gates and negative controls:
  - new gates: frame, story time, branch, world boundary, event identity;
  - teleportation is now checked on the story axis.
- **Compatibility.** v1 and v2 are a linear special case. The three fables and 10 FairytaleQA tales (worlds, v1 and v2 reader answers, 432 questions) give **byte-for-byte the same** 31 files on the v3 executor, and in v2 mode as well.
- **Testbeds** — Project Gutenberg, public domain: «The Adventure of the Speckled Band» (595 sentences) and Scheherazade's frame from «The Story of the Merchant and the Genius» (284 sentences).
  - The LLM wrote 1182 and 712 lines. The gates rejected 6 and 0 — all six genuine LLM errors.
  - Frames 3 and 4, narrative levels 2 and 3. Claims 37 and 5, reveals 11 and 2.
  - **$8.57** for 17 calls (cap $40).
- **35 questions** (time, knowledge, frames, control) frozen before the LLM run.
  - Blind measurement — by hand **24/35 (69%)**.
  - A second, NON-blind round after analysis (level by narrators, flash-forward by its own time announcement, state cause ≤ the next event on the line) — **29/35 (83%)**.
  - The remaining failures are the event description in the query, not the world: the LLM correctly resolved the rumours of Roylott's death into a canonical event with ^s5.

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

### Change language (`lang.rs`)

One command per line, anchors `^sN` at the end. A line spanning two sentences has two anchors: `^s3 ^s4`.

| command | what it does |
|---|---|
| `cursor [chapter= para= sent=] [at=L] [time=]` | where the narrative moved |
| `loc L type= [class=] [name=] [in=L]` | a location, nested in another via `in=` |
| `obj O type= [class=] [name=] [at=L\|C]` | an object in a place or held by a character |
| `char C type= class= [name=] [at=L] [attributes…]` | a character |
| `move C to=L` / `move O to=L\|C` `[cause=E]` | change of place: a character goes; an item fell or was picked up |
| `have C O [cause=E]` | first possession of an item that has no place yet |
| `give C O to=C [cause=E]` | handing over an item *(added)* |
| `set X key=value … [cause=E]` | an attribute *(added)* |
| `rel R kind= a=C b=C [cause=E]` · `rel R kind= cause=E` · `rel R fact=E` | a relation: start, change, fact *(added)* |
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

### Change language v2 (`lang.rs`, `world.rs`)

New registries: `P` — intentions, `B` — beliefs. Both are classes with their own immutable states, like relations. Bidirectional links:
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

The closed sets were extended with the general vocabulary of tales (places, items, 26 feelings). The list was compiled before the FairytaleQA run, without looking at the reference answers.

A `feeling` with a single tag stays a `Tag`, as in v1, so old tests and fables give the same output.

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

New v2 gates (negative controls — `v2_gates_are_not_mute`, `v2_links_catch_injected_mismatch`, `v2_format_is_strict`):

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
- per fable — one `claude -p` via `prag::opus` (Opus, high, empty directory, no tools);
- a retry — one, only if the format gate rejected lines, with the same prompt and the rejection explanation;
- existing `cmds-<doc>.txt` are not overwritten.

Accounting — `calls.jsonl`.

**LLM v2** (`rules_v2`, `body_v2`, `call_v2`):
- **Prompt.** Language v2, closed sets, gates. A new example (the invented «The Miller's Goose») shows all new commands; a test checks that it passes the gates. The LLM is warned that questions will be answered only from the state.
- **Chunks.** Up to 95 sentences per call, split at a section boundary closest to the middle. The second chunk receives the preceding text, a compressed world (registries, states, open intentions, latest events, cursor) and the next free ids.
- **Retry.** One per tale, only on a format rejection, and only for the rejected lines: `@N command` are inserted in their places. At most 3 calls per tale in total.

`prag` is unchanged: `opus.rs` is already public. `world` depends on `prag` (`opus`, `schema::Act`, `data::duck_query`) and on `en` — only dictionary reads in code: lemmas of question words and past tense of verbs in answers.

### Commands

```
export PATH=$HOME/.cargo/bin:$PATH
cargo build --release
../target/release/world prompt 2451                                   # prompt without a call
../target/release/world vmm 2451 $MOVA_DATA/runs/world-2026-09-26      # LLM → cmds-2451.txt
../target/release/world build 2451 $MOVA_DATA/runs/world-2026-09-26 --questions data/questions.tsv
../target/release/world query 2451 $MOVA_DATA/runs/world-2026-09-26 diff char:crow 1 7

# FairytaleQA (v2)
R=$MOVA_DATA/runs/world-v2-2026-09-26
../target/release/world ftqa-import $MOVA_DATA/raw/fairytaleqa/hf-test.csv $MOVA_DATA/raw/fairytaleqa
../target/release/world ftqa-prompt golden-goose                  # v2 prompt without a call
../target/release/world ftqa-vmm golden-goose $R                  # LLM v2 in chunks → cmds-golden-goose.txt
../target/release/world ftqa-build golden-goose $R                # gates → world-golden-goose.md
../target/release/world ftqa-eval $R --sample 10 --seed 2 --exclude $R/seen-questions.tsv
../target/release/world ftqa-parse questions.tsv --frame          # question parsing (development)

# Question reader v2 (en model: WORLD_EN_MODEL or $MOVA_DATA/en/models/en.bin)
O=$MOVA_DATA/runs/world-reader-2026-09-26
../target/release/world reader-parse questions.tsv                # UD tree parsing (validation)
../target/release/world reader-span-dev $O/valid [--show why]     # spans on validation, without the world
../target/release/world reader-eval $R $O --dev $O/dev-questions.tsv   # development: seen questions only
../target/release/world reader-ask golden-goose $R 4 [--facts]    # one (allowed) question in detail
../target/release/world reader-eval $R $O --sample 10 --seed reader-2026-09-26 \
    --exclude $O/dev-questions.tsv --exclude $R/freeze-2/manual-sample.tsv --exclude $O/excluded-extra.tsv
../target/release/world reader-eval $R $O --skip $O/dev-questions.tsv  # table on the 305 non-dev questions
../target/release/world reader-eval $R $O --control shuffle|noindex    # negative controls
```

## Run of 2026-09-26: three fables

Source: Project Gutenberg #21, public domain, database `docs`/`sentences`. In this translation the Crow has **meat** (*a bit of meat*), not cheese.

| fable | doc | sentences | LLM lines | accepted | format | gates | without commands |
|---|---|---|---|---|---|---|---|
| The Fox and the Crow | 2451 | 6 | 26 | 26 | 0 | 0 | 0 |
| The Lion And The Mouse | 2355 | 6 | 44 | 44 | 0 | 0 | 0 |
| The Hare and the Tortoise | 2366 | 7 | 44 | 44 | 0 | 0 | 0 |

The gates let the clean LLM output through. That they are not mute is shown by the test `real_vmm_output_and_its_mutations` on the real output for "The Fox and the Crow". Each mutation turns red on its own gate:
- remove `move O1 to=L1` (the Crow did not drop the meat) → teleportation;
- `have` instead of picking up → two-way link;
- a feeling without a cause → silent change;
- a nonexistent character → reference;
- an event not where the agent is → continuity.

### The Fox and the Crow (2451)

- **Locations:** `L1 ground` ⊃ `L2 tree`.
- **Object:** `O1 meat`: Crow `^s1` (have because of E1 steal) → `L1` `^s5` (move because of E8 drop) → Fox `^s6` (move because of E9 pick_up).
- **Crow `C1@1…@7`:**
  - ^s1 — female; has the meat; in the tree;
  - ^s5 — feeling=proud, goal "prove her voice" (because of E5, the flattery);
  - ^s5 — without the meat;
  - ^s6 — ashamed (because of E10).
- **Fox `C2@1…@5`:**
  - wily, cunning;
  - goal "get the meat";
  - ^s6 — has the meat, amused.
- **Relationship:** `R1` Fox — Crow: stranger ^s2 → adversary ^s6 (because of E9); fact E5.
- **Timeline:** E1 steal → E2 perch → E3 hold → E4 see → **E5 say:evaluate, indirect, insincere** ^s3 ^s4 (means "caw aloud to prove your voice so you drop the meat", why "he wants the Crow to open her beak and drop the meat") → E6 narrator "trick" → E7 caw (why "refute the slight on her voice") → E8 drop → E9 pick_up → E10 say:evaluate (mockery).

### The Lion and the Mouse (2355)

- **Registries:**
  - `L1 place` — the text does not name the place;
  - characters: Lion, Mouse, `C3 the hunters` (a group is one character);
  - objects: `O1 ropes` (hunters → ground, status=gnawed), `O2 teeth`.
- **Lion `C1@1…@13`:**
  - asleep → awake (because of E1 run) → angry → amused (because of E4 laugh) → captive (because of E6 catch) → afraid → free (because of E12 free) → grateful;
  - belief "a mouse could never help a lion" → "even a mouse can help a lion" (because of E13).
- **Relationships:**
  - `R1` Lion — Mouse: stranger ^s1 → adversary ^s2 (because of E2 catch) → acquaintance ^s3 (because of E5 release) → friend ^s5 (because of E12 free); facts E2, E3, E5, E12;
  - `R2` hunters — Lion: adversary.
- **Timeline:** run → catch → say:request (a plea with a promise) → laugh → release → catch → bind → roar → recognize → come → gnaw (instr teeth) → free → say:evaluate.

### The Hare and the Tortoise (2366)

- **Locations:** `L1 the race course` ⊃ `L2 the goal`, `L3 wayside`.
- **Characters:** Hare, Tortoise, `C3 the Fox` (judge, no location).
- **Hare `C1@1…@12`:**
  - swift, proud;
  - belief "the tortoise cannot possibly beat him";
  - ^s5 — wayside, asleep; ^s6 — awake;
  - belief "the slow tortoise has beaten him", ashamed.
- **Tortoise `C2@1…@11`:**
  - slow → slow, steady (because of E8 go);
  - ^s4 — near the goal; status=winner (because of E13 win); tired; asleep (because of E14 doze).
- **Relationships:**
  - `R1` Hare — Tortoise: rival ^s1 (because of E1, the mockery), fact E13;
  - `R2`, `R3` — acquaintance with the Fox.
- **Timeline:**
  - E1 say:evaluate (mockery) → E2 say:offer (challenge) → E3, E4 say:agree → E5 choose (Fox);
  - E6, E7 start → E8 go → E9 lie → E10 sleep → E11 wake → E12 run → E13 win → E14 doze → E15 see;
  - E16 narrator: moral "steady effort beats careless speed".

## Answers from the state

Procedure:
1. Questions of the FairytaleQA types, their queries and expected answers (`data/questions.tsv`) were written **before** the LLM call and not edited after it.
2. The SLM takes the answer only from the state and the history, with anchors.
3. Scoring: words from the expected answer must appear in the answer text, with order via `>`.
4. All 24 answers were reviewed by hand.

The honesty control (FX, LX, HX) was added after the run. The answers to these questions are absent from the state by construction, so the correct answer is "not in state".

| # | type | question | SLM answer (abridged) | score |
|---|---|---|---|---|
| F1 | character | Who are the main characters? | Crow (6 mentions, ^s1–^s6), Fox (4, ^s2–^s6) | hit |
| F2 | setting | Where was the Crow when the Fox praised her? | `C1@4` in L2 tree ⊂ L1 ground, from ^s1 (move because of E2 perch) | hit |
| F3 | action | Who had the meat at the beginning and at the end? | @first Crow (^s1, because of E1 steal); @last Fox (^s6, because of E9 pick_up) | hit |
| F4 | cause | Why did the Fox praise the Crow? | E5 ^s3 ^s4: evaluate, indirect, insincere; why "…drop the meat" | hit |
| F5 | feeling | What did the Crow feel when the Fox doubted her voice? | feeling=proud from ^s5 because of E5 | hit |
| F6 | cause | Why did the Crow caw? | E7 caw ^s5: why "refute the slight on her voice" | hit |
| F7 | outcome | What happened to the meat? | Crow ^s1 → ground ^s5 (because of E8 drop) → Fox ^s6 (because of E9) | hit |
| F8 | prediction | Will the Crow trust the Fox? | "probably will be wary of each other and feud": stranger ^s2 → adversary ^s6 | hit |
| FX | control | What did the Fox do with the meat afterwards? | not in state (no eat events) | honest |
| L1 | character | Who woke the Lion? | sleep=awake from ^s1 because of E1 run (agent Mouse) | hit |
| L2 | feeling | What did the Lion feel when he caught the Mouse? | feeling=angry from ^s2 because of E1 | hit |
| L3 | action | What did the Mouse promise? | E3 ^s2 request; means "spare my life and I will surely repay your kindness" | hit |
| L4 | cause | Why did the Lion let the Mouse go? | E4 laugh: "a mouse could never help a lion"; E5 release: "the Mouse's plea amused him" | hit |
| L5 | character | Who tied up the Lion? | the hunters (E6 catch, E7 bind ^s4) | hit |
| L6 | outcome | How did the Mouse free the Lion? | ^s5 recognize → come → gnaw ropes (teeth) → free | hit |
| L7 | outcome | How did the relationship change? | stranger → adversary ^s2 → acquaintance ^s3 → friend ^s5, with causes | hit |
| L8 | prediction | What will happen if the Lion meets the Mouse again? | "probably will help each other": friend from ^s5 | hit |
| LX | control | Who is the Mouse's mother? | not in state (mother unknown) | honest |
| H1 | character | Who chose the course and the goal? | Fox (E5 choose ^s2) | hit* |
| H2 | cause | Why did the Hare agree? | belief "the tortoise cannot possibly beat him" from ^s2 | hit |
| H3 | setting | Where did the Hare fall asleep? | L3 wayside ⊂ L1 the race course, from ^s5 | hit |
| H4 | action | What did the Tortoise do during the race? | ^s3 start; ^s4 go "slow but steady, never stopping" | hit |
| H5 | outcome | Who won? | Tortoise (E13 win ^s6) | hit |
| H6 | feeling | What did the Tortoise feel at the end? | feeling=tired from ^s6 because of E8 | hit |
| H7 | cause | Why did the Hare lose? | E9 lie ^s5 at wayside: why "he believed he could easily win" | hit |
| H8 | cause | Why did the slow one win (moral)? | narrator ^s7: "steady effort beats careless speed" | hit |
| HX | control | How old is the Hare? | not in state (age unknown) | honest |

**Total:** 24/24 hit, 0 miss, 0 "not in state"; honesty control 3/3.

Caveats:
- **The scoring is not independent.** The questions, queries, expected answers and prompt were all written by the same agent. The closed sets were compiled with the fables in mind: `wayside course finish`, `offended eager amused`.
- **The scoring is lenient:** words, not the FairytaleQA gold.
- **H1*** was a hit thanks to a modeling error. The text only says they *agreed* that the Fox would choose. The LLM recorded the choice as a real event E5, against the rule "plans are not events".

## Cost of the Opus calls

3 calls (Opus `claude-opus-5-5`, high), no retries: the format gate rejected nothing.

| fable | s | output | cache write | cache read | estimate $ |
|---|---|---|---|---|---|
| 2451 | 26.6 | 2150 | 20,380 | 6,898 | 0.207 |
| 2366 | 31.4 | 2843 | 18,716 | 8,572 | 0.208 |
| 2355 | 34.0 | 3485 | 20,012 | 8,572 | 0.232 |
| **total** | **92.0** (in parallel ≈34) | **8478** | **59,108** | **24,042** | **≈0.65** |

`claude -p` runs on the subscription, so the $ from `total_cost_usd` is only an estimate. Input per call is ≈27–29 thousand tokens, mostly the built-in system prompt of `claude -p`. Our prompt (rules, example, fable) is ≈8 KB, i.e. ≈2 thousand tokens. Output is 2–3.5 thousand tokens.

## What change language v1 lacked, and what v2 closed

| what was lacking (fable run) | v2 | how the LLM used it on 10 fairy tales |
|---|---|---|
| **Irrealis and plans:** the conditional and the agreed-upon lived only in the text of `means`/`belief` or became an event (E5 choose) | `plan` with a state and a fulfillment event | 81 intentions; 68 closed: kept 36, broken 25, dropped 7 |
| **One act per utterance** (the Mouse's plea is both request and promise) | `act=request+promise` | 66 utterances |
| **Joint agents** ("they agreed", "the two started") | `agent=C1+C2`, `say C1+C2` | 50 events |
| **Goals never end** | `done` / `unset … why=` | 10 / 18 |
| **One feeling at a time** | `feel +a -b`, `feeling=a,b` | 177 `feel` |
| **Two times** | `about=`, `when=`, `before=` | 104 / 48 / 12 |
| **Beliefs are flat text** | `believe`: claim, what about, from where, truth value | 96 lines |
| **The cursor jumped to a mentioned place** | `loc … here=0`, `mention` | 54 / 22 |
| **Body parts as objects** | `part=`, `detach`, `attach` | 18 parts; the old man's wen moved to the neighbor via `detach` → `attach` |
| **Quantity** | `num=` | 53 |
| **Negation** ("never stopped" only in `how`) | `neg=1` | 35 |
| **Directed relationships** | `benefactor master parent ruler captor suitor mentor` | 33 |
| **Motive and cause as text** | partly: `cause=E` on events, `about=`. `why` is still text | — |

What remains:
- `why` does not refer to a goal or intention (`for=P`);
- an object cannot lie inside an object (a chest with treasure);
- no "X thinks that Y thinks", i.e. nested beliefs;
- feelings have no target (`about=` in `feel`).

## FairytaleQA: the five-year-old test (2026-09-26)

### Data

- **Source.** Hugging Face `WorkInTheDark/FairytaleQA` (a co-author of the paper), revision `655b20b`.
- **License.** Apache-2.0 according to the card; the license section of the card is empty. GitHub was not checked.
- **Use.** Measurement only; it does not go into training. Details: `$MOVA_DATA/raw/fairytaleqa/SOURCES.md`.
- **Texts.** The HF copy does not contain whole stories: each row is a question and its section.
  - The story is reconstructed from sections (`ftqa.rs`).
  - Sections without any question are missing from the copy.
  - The text is lowercased and tokenized, as in the paper.
- **Selection.** 10 test stories, chosen by length only (26–161 sentences, 537–4391 words), 432 questions.
  - 3 stories whose questions had been seen while inspecting the data were skipped.

### How the test went

1. **LLM v2** writes world changes from the story text. The LLM does not see the questions.
2. **World.** The SLM executes the lines with gates.
3. **The SLM reader** (`ans.rs`, Rust, no Opus) answers the questions in natural language from the state alone:
   - parses the question: intent, subject, verb, a "time anchor" after `when/after/because`;
   - looks up state facts by lemmas from the `en` dictionary; the question's sections take priority, as in FairytaleQA, where the model sees the section;
   - builds the answer from slots: the event's `why`, the cause or consequence event, the feeling at the anchor time, goal, intention, belief, participant, place, number.
   - No slot means **"not in state"**.
   - A separate mark, **"anchor"**: the sentence of the best fact found, as a fallback.
4. **Measurement** (`report.rs`):
   - ROUGE-L F1, max over the two references, as in the paper: tokens are lowercase Latin letters and digits, no stemming;
   - slices by type, explicit vs. inferred, share of "not in state";
   - **baseline**: the sentence from the question's sections with the largest lemma overlap with the question;
   - manual check of a sample: 1 / 0.5 / 0 for the SLM and the baseline, plus whether the answer is in the state at all (checked against the commands).
5. **Protocol honesty.**
   - Question parsing was developed on the **validation** split (question forms only) and on the v2 example.
   - Then the answerer was frozen (`freeze-1.txt`, source hashes), and only then run on the test questions for the first time.
   - The second reader round (`freeze-2`) came **after** analyzing the failures of sample 1, so it is **not blind**. Its manual check is on a fresh sample: 68 questions, none of the 127 questions seen before the changes is in it.

### LLM: 10 worlds

| story | sentences | lines | gates | calls | output, tok. | s |
|---|---|---|---|---|---|---|
| old-hop-giant | 26 | 89 | 0 | 1 | 7,315 | 64 |
| the-miserly-farmer | 41 | 109 | 0 | 1 | 14,358 | 122 |
| sleeping-beauty-in-the-wood | 44 | 177 | 0 | 1 | 27,088 | 224 |
| golden-goose | 52 | 211 | 0 | 2 (repair of 4 lines) | 30,197 | 260 |
| fox-and-wolf | 59 | 140 | 0 | 2 (repair of 3) | 15,959 | 145 |
| whippety-stourie | 66 | 165 | 0 | 1 | 20,793 | 181 |
| lucky-andrew | 85 | 240 | 0 | 2 (repair of 2) | 31,787 | 270 |
| how-an-old-man-lost-his-wen | 89 | 195 | 0 | 2 (repair of 1) | 19,028 | 167 |
| jelly-fish-and-monkey | 144 | 294 | 0 | 2 (2 parts) | 39,630 | 323 |
| the-dwarfie-stone | 161 | 435 | 0 | 3 (2 parts + repair of 1) | 51,534 | 442 |
| **total** | 767 | **2055** | **0** | **17** | **257,689** | 2198 (in 4 threads) |

- **Format.** 11 lines rejected, all fixed by a repair retry; sentences without commands: 0.
- **The gates stayed silent** on all 2055 lines. That they are not mute is shown by mutations of the real v2 output (`real_vmm_v2_output_and_its_mutations`, the wen story): without `detach` → continuity; `move` instead of `detach` → link; `attach` without a cause → silent change; an intention closed twice or "kept" by an event with `neg=1` → continuity.
- **Commands.** event 494, say 306, move 189, feel 177, plan 149, set 146, obj 120, char 101, loc 99, believe 96, rel 84; mention 22, cursor 21, unset 18, have 11, done 10, give 6, detach 4, attach 2.

**Cost.** 17 calls to Opus `claude-opus-5-5`, high:
- output 257,689 tokens (including thinking);
- cache write 468,936, cache read 139,067;
- 2198 s in total, up to 263 s per call;
- estimate from `total_cost_usd`: ≈ $8.93. This is the subscription, so the $ is only a guide.

All 17 calls were made before 11:28Z by a binary built at 11:15Z, still on the subscription. At 11:39Z `prag::opus` switched to an API key, so from then on `world ftqa-vmm` spends real money, and `total_cost_usd` is the actual price.

### Results: ROUGE-L (432 questions)

"SLM": "not in state" counts as 0. "+ anchor": where the state is silent, the anchor sentence is used.

| type | N | SLM blind (freeze-1) | not in state | SLM v2 (freeze-2, not blind) | not in state | v2 + anchor | baseline |
|---|---|---|---|---|---|---|---|
| character | 45 | 0.139 | 38% | 0.190 | 24% | 0.193 | 0.140 |
| setting | 24 | 0.298 | 8% | 0.327 | 0% | 0.327 | 0.242 |
| action | 126 | 0.223 | 29% | 0.257 | 24% | 0.274 | 0.288 |
| feeling | 48 | 0.211 | 21% | 0.236 | 21% | 0.236 | **0.060** |
| causal relationship | 122 | 0.134 | 52% | 0.193 | 21% | 0.203 | 0.274 |
| outcome resolution | 28 | 0.191 | 11% | 0.225 | 0% | 0.225 | 0.280 |
| prediction | 39 | 0.171 | 10% | 0.199 | 3% | 0.205 | 0.086 |
| **all** | 432 | **0.185** | **32%** | **0.226** | **18%** | 0.235 | **0.222** |
| explicit | 325 | 0.190 | 32% | 0.230 | 19% | 0.240 | 0.259 |
| inferred | 107 | 0.170 | 32% | 0.214 | 15% | 0.220 | 0.109 |

For comparison, on the test set in the paper:
- BART fine-tuned on FairytaleQA: ROUGE-L ≈ 0.53;
- humans: ≈ 0.65.

Our numbers are not comparable with them: we trained nothing on this data, and an answer from the state is not a span from the text. Only the baseline is comparable.

### Manual check

**Blind reader** (freeze-1), sample 1: 70 questions, 10 per type:

| type | SLM | not in state | answer is in state | baseline |
|---|---|---|---|---|
| character | 35% | 4 | 100% | 40% |
| setting | 50% | 1 | 90% | 85% |
| action | 20% | 5 | 90% | 90% |
| feeling | **65%** | 0 | 85% | 35% |
| causal relationship | 10% | 4 | 55% | 50% |
| outcome resolution | 30% | 2 | 80% | 20% |
| prediction | 30% | 0 | 85% | 55% |
| **all** | **34%** | 16 | **84%** | **54%** |

**Reader v2** (freeze-2, not blind), fresh sample 2: 68 questions, none seen before:

| type | SLM | not in state | answer is in state | baseline |
|---|---|---|---|---|
| character | 25% | 1 | 75% | 70% |
| setting (8) | 31% | 0 | 81% | 75% |
| action | 40% | 1 | 85% | 45% |
| feeling | **65%** | 3 | 65% | 20% |
| causal relationship | 30% | 1 | 65% | 70% |
| outcome resolution | **60%** | 0 | 95% | 50% |
| prediction | **35%** | 1 | 100% | 15% |
| **all** | **41%** | 7 | **81%** | **49%** |

**Conclusion.**
- The weak spot is **the reader, not the world**. The answer is in the state for 81–84% of the questions, but the reader finds it in 34–41%.
- The state is strong where one needs to know a character's state at a point in the story:
  - feelings: 65% vs. 20–35% for the baseline; ROUGE 0.236 vs. 0.060;
  - prediction and outcome: `cause=` chains and subsequent events;
  - inferred questions: 0.214 vs. 0.109.
- The baseline is strong on explicit "who/what/where/why". The answer there is a verbatim piece of a sentence, while the SLM reader looks for an event by the question's verb and often picks the wrong one.

### Examples (freeze-2)

**Hits:**
1. *golden-goose #13* "what happened when the eldest caught hold of the goose by its wing?"
   - SLM (consequence `E36 ← E35` via `cause=`): "…her fingers stuck fast to the goose and she could not take her hand away".
   - ROUGE 0.78; baseline 0.14 (a sentence about the intention to pluck a feather).
2. *whippety-stourie #48* "what did the widowed mistress want to do before she let the old woman know about her name?"
   - SLM from the intention `P7 plan`: "tease the fairy a few minutes before letting her know she has found out her name".
   - 0.52 vs. 0.18.
3. *the-dwarfie-stone #104* "what happened after the two countesses fled to scotland?"
   - SLM, the events following `E152 flee`: "the norsemen invaded scotland, and the norsemen set on fire the countesses' castle".
   - 0.42 vs. 0.10.
   - Also *sleeping-beauty #1* "how many fairies?" → `num` → "seven", 1.0 vs. 0.0. And *jelly-fish #21* "how did the king feel when his wife became ill?" → `feel … cause=E12 fall_ill` → "worried" (reference "troubled": correct by hand, ROUGE 0).

**Failures:**
1. *how-an-old-man-lost-his-wen #1* "what did the good old man have growing in his right cheek?" → "all kinds of medicines".
   - The wen is in the state: `O1 part=C1`.
   - For "have", the reader looks only in `items`, not in `parts`. **Reader failure.**
2. *whippety-stourie #8* "why did the widowed mistress rise very early…?" → "to feed her pigs".
   - The LLM wrote `why` as a tautology, and the real cause (rent-day) is not in the state.
   - **State failure** (LLM): the baseline with its sentence gets 0.55.
3. *golden-goose #18* "what happened when the princess saw the seven people continually running?" → "Dullhead claimed the princess as his bride".
   - The question's anchor ("saw … running") was matched to the event `E49 laugh`, and the reader took its consequence instead of the event itself.
   - **Anchor matching failure.**
   - Also *the-dwarfie-stone #22* "who was as different from his brother as night is from day?" → "an enormous grey-headed raven". The comparison of the brothers is not in the state (state), and the reader grabbed a random agent (reader).

### Next steps

1. **Reader.**
   - Search not only events but all state facts through one index: events, intentions, beliefs, traits, parts, place names.
   - The role of the answer comes from the question type.
   - The anchor should take the subject of the subordinate clause into account.
   - Basis: 81–84% of the answers are already in the state.
2. **Question parsing** from the UD tree (`en::parse`) instead of token rules.
3. **LLM prompt.** `why` must be the real cause, not a restatement of the action. Short action names (glosses) in the story's own words, for explicit questions.
4. **The five-year-old test per specification.** An Opus judge where ROUGE penalizes synonyms ("afraid" / "terrified", "seven" / "7"). Comparison with Opus reading the story itself.
5. **More stories.** The remaining 13 test stories only after an explicit decision: runs cost money.
## Question reader v2 (2026-09-26)

Diagnosis from the v2 measurement: the answer is in the state for 81–84% of questions, but reader v1 (`ans.rs`) finds it in 34–41%. The weak spot is the reader. Reader v2 is a set of separate modules next to v1 (`ans.rs` is unchanged, its hash = freeze-2), with no Opus calls: the worlds of the 10 tales are already built.

### How it works

1. **A single fact index** (`index.rs`). From 102 (old-hop-giant) to 494 (the-dwarfie-stone) facts per tale, each with sentence anchors, a cause (`cause`) and story order:
   - events: agent, object, addressee, place, instrument, `how`, `why`, `when`, `about`, negation;
   - utterances: speaker, addressee, acts, `means`, `why`;
   - intentions: kind, holders, text, toward whom, state and the fulfilling event; beliefs: proposition, about what, truth value;
   - attributes by state: traits, status, goal, feeling (only **added** tags), age, sex, sleep, freedom, life, number, name, place — with the `unset … why=` explanation;
   - objects in hand, parts, names of characters and places with `in=` ancestors, relations.

   Search uses lemmas from the `en` dictionary with field weights: predicate +3; subject in its role +3, elsewhere +1, absent −2; another participant +1.5; content word — the field weight (predicate 1.5; texts and participants 1; time and "about" 0.5); the question's section +3, adjacent +0.5, farther −1.5; for a prediction the next section +2.5.
2. **Parsing the question with a UD tree** (`qframe.rs`, model `ud-ewt-eslspok.bin`).
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

### Protocol

- **Development — only on what is allowed.** Parsing and spans — on the validation split (`hf-valid.csv`, 1025 questions, 23 tales, no world): oracle sentence and baseline sentence → span. Merging and the index — on test tales, but only on the 127 questions in `seen-questions.tsv` already seen in the first round. Thresholds (`CONF` 6, `COVER` 0.5) come from development: on the 127 questions there is a plateau at 4.5–6.
- **Freeze** (`freeze.txt`, freeze-3, 12:30Z): sha256 of all `src/*.rs`, `Cargo.*`, and the `en` model file; only then the first run on the remaining 305 questions. After the freeze only the report option `--skip` was added: the answers are byte-for-byte the same (`cmp`).
- **Manual check — a fresh sample:** 10 per type (seed `reader-2026-09-26`), excluding the 127 dev questions, the 68 questions of the freeze-2 sample, and 12 questions that had been seen (`excluded-extra.tsv`: README v2 examples and two rows of another table). The result is 53: the "setting" type was exhausted by development and freeze-2, "outcome" has only 3. Judged by the same small agent after the freeze; criteria are in the header of `manual-verdicts.tsv`.

### Results: ROUGE-L

**305 questions unseen by development** (main table). "State only" — "not in state" counts as 0; "state + sentence" — merging.

| type | N | before: reader v1 (freeze-2) | **after: v2, state + sentence** | v2, state only | baseline | not in state (v2) |
|---|---|---|---|---|---|---|
| character | 26 | 0.137 | **0.287** | 0.256 | 0.140 | 38% |
| setting | 8 | 0.338 | **0.389** | 0.340 | 0.206 | 12% |
| action | 106 | 0.240 | **0.329** | 0.243 | 0.269 | 37% |
| feeling | 30 | 0.178 | 0.154 | 0.144 | 0.039 | 37% |
| causal relationship | 102 | 0.196 | **0.294** | 0.187 | 0.281 | 40% |
| outcome resolution | 13 | 0.265 | **0.409** | 0.356 | 0.339 | 15% |
| prediction | 20 | 0.218 | 0.208 | 0.187 | 0.089 | 20% |
| **all** | 305 | 0.213 | **0.294** | 0.219 | 0.229 | 35% |
| explicit | 230 | 0.222 | **0.334** | 0.248 | 0.267 | 34% |
| implicit | 75 | 0.185 | 0.171 | 0.131 | 0.114 | 39% |

**All 432** (including the 127 dev questions): v1 0.226 (as in freeze-2 — a cross-check), v2 **0.320**, state only 0.246, baseline 0.222; explicit 0.230 / **0.358** / 0.259 (baseline), implicit 0.214 / 0.203 / 0.109. On the 127 dev questions — 0.382: the gap from 0.294 is the price of developing on one's own questions.

By answer source (432): baseline sentence — 145 (0.220); anchor-sentence span — 99 (**0.517**, baseline on the same questions 0.283); fact field — 188 (0.292, baseline 0.191).

Spans on validation (1025, no world): oracle sentence 0.397 → span 0.448 (with the fallback "sentence without question words" 0.458); baseline sentence 0.219 → 0.256.

### Manual check (fresh sample, 53)

| type | N | before: v1 | **v2** | baseline | v2 took the sentence | answer is in the state |
|---|---|---|---|---|---|---|
| character | 10 | 40% | 55% | 55% | 5 | 95% |
| action | 10 | 20% | 70% | 70% | 7 | 100% |
| feeling | 10 | 50% | **65%** | 45% | 1 | 90% |
| causal relationship | 10 | 25% | 50% | 60% | 6 | 80% |
| outcome resolution | 3 | 67% | 67% | 100% | 0 | 100% |
| prediction | 10 | 45% | 40% | 25% | 0 | 95% |
| **all** | 53 | 38% | **57%** | 54% | 19 | 92% |
| explicit | 38 | 39% | 64% | 61% | 15 | 96% |
| implicit | 15 | 33% | 37% | 37% | 4 | 83% |

**Conclusion.**
- Reader v2 beats v1 almost everywhere. The exceptions are predictions (ROUGE 0.208 vs 0.218, manually 40% vs 45%) and ROUGE on feelings and implicit questions.
  - On feelings manual accuracy is 65% vs 50% for v1, but ROUGE penalizes a synonym ("desperate" vs "despaired") and the fallback sentence.
- v2 beats the baseline on ROUGE by +0.065 (explicit +0.067, implicit +0.057), but manually by only 3 points.
  - The manual gain is in feelings (65% vs 45%) and predictions (40% vs 25%).
  - The loss is in causes (50% vs 60%): wrong spans and wrong facts where the baseline had a whole sentence containing the answer.
- The state is still better than the reader: the answer is in the state for 92% of the sample, v2 finds 57%.

### Negative controls

| run | state only | state + sentence | baseline | not in state |
|---|---|---|---|---|
| normal (432) | 0.246 | 0.320 | 0.222 | 34% |
| **facts of another tale** (tale *i* reads the index and world of tale *i+1*) | **0.003** | 0.207 | 0.222 | 94% |
| **no index** | 0 | **0.222 = baseline** | 0.222 | 100% |

On the 305 non-dev questions: foreign facts — 0.213 (state 0.003), no index — 0.229 = baseline.
- Foreign facts rarely pass the threshold: 408 of 432 answers are sentences. The remaining 24 are wrong, so merging falls below the baseline.
- Without an index the reader gives exactly the baseline: 432 of 432 answers match the baseline sentence byte for byte (also the test `reader_v2_answers_from_state_and_controls`).
- The v1 column in the control reports is the normal v1: the control does not apply to it.

### Examples (fresh sample, after the freeze)

**Hits:**
1. *the-miserly-farmer #17* "what happened when the farmer turned the corner?"
   - The anchor is the subordinate-clause event E44 turn (^s38). The consequence is in the same sentence → span [conj-after]: "there lay the missing piece from the axle by the city wall".
   - ROUGE 1.00. v1 — "the farmer found the axle of his cart" (0.28); baseline — the whole sentence (0.80).
2. *how-an-old-man-lost-his-wen #9* "why did the man sit inside a large hole in the hollow trunk of a tree?"
   - Fact E14 espy (score 12.5: subject +3, place +1.5, words +5, section +3) → field `why`: "find shelter from the rain".
   - Manually — yes; baseline ("at last he espied a large hole…") — no; v1 — "not in state".
3. *golden-goose #17* "how did the king feel about dullhead as a son-in-law?" (implicit)
   - "about X" → the king's feeling caused by an event involving Dullhead: "annoyed".
   - The baseline only hints ("did not much fancy him"); v1 — "not in state".

**Misses:**
1. *jelly-fish-and-monkey #1* "who was the dragon king of the sea?" → "long, long ago, in old japan, the kingdom of the".
   - The fact is correct (C1, name "rin jin, the dragon king of the sea"). But merging preferred the span, and the subject rule, with no nsubj in the tree, took "the words before the predicate".
   - **Merging failure:** for "who is X" the character's name must take priority over the span.
2. *whippety-stourie #28* "what did the old woman ask for in return?" → baseline sentence, wrong.
   - The answer is in the state: E33 `means="… now give me what I ask: your son"`.
   - But the LLM named the character "the old lady in green" (class fairy), so "the old woman" did not link to C5. The utterance went through without a subject (score 2 < 6).
   - **Linking failure** of a name to a character.
3. *how-an-old-man-lost-his-wen #19* "why did the man get back in the tree?" → "the tree was in the background".
   - The reader took the `why` field of event E22 creep_back — and that is exactly what the LLM wrote. But it is not a motive ("he did not want the demons to see him").
   - **State failure:** `why` as a tautology or circumstance, not a cause. The same one as in the freeze-2 example.

### What next

1. **Merging by type:** "who is X" — the name, not a span; forbid the subject span without nsubj.
2. **Name linking:** synonyms of class and name (old woman ↔ old lady; "the doctor" ↔ "the fish doctor"). Candidates come from all mentions of the character, not only from the name in the state.
3. **Feelings:** choose the fact by the time anchor, not "the first one caused by an event involving X". Synonymy of tags with answers (annoyed ↔ upset) — via the dictionary, not by fitting.
4. **LLM prompt** (from the previous "what next"): `why` must be the real cause. Here both the state and the reader run into tautologies.
5. **Opus judge** instead of ROUGE for synonyms — only after an explicit decision.

## World v3: non-linear narrative (2026-09-26)

v3 was launched together with the promised architecture rework. Specification — `docs/context.md`: "Non-linear narrative" (event identity, the v3 pipeline, partial order of time), "Hierarchical and global worlds", "Checking claims against the world", "Immutable states".

**The scope of v3.** v2 proved that the world holds linear causality. v3 holds three things separately:
- **what happened** — canonical events with identity;
- **what is believed about it** — claims and observers' knowledge;
- **in what order it is told** — the text cursor versus the story time axis.

### Architecture (`story.rs`, `narr.rs`)

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

### Change language v3 (`lang.rs`)

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

All of these are format refusals (`v3_format_is_strict`, 29 corrupted and 12 correct lines). `mention E…` remains a format error, as in v2: an existing event is mentioned by reference (`claim`, `know`, `reveal`, `time`), not as a new event.

### v3 gates

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

### Compatibility: v1 and v2 are the linear special case

- **Two executor modes.**
  - `World::new` — v2: linear narrative, v2 gates with the same names. The narrative layer is here too: v3 commands work.
  - `World::new_v3` — adds gates in story time:
    - teleportation on the story axis (in v2 this is continuity);
    - a dead agent in the present;
    - a second death.
- **Regression** (`WORLD_V3=1` — old commands on the v3 executor). From the same `cmds-*.txt`, three fables (world and answers to 24+3 questions) and 10 FairytaleQA tales were rebuilt:
  - `ftqa-build` — worlds;
  - `ftqa-eval` — reader v1;
  - `reader-eval` — reader v2, 432 questions.
- **Result.** 31 files (`world-*.md`, `answers-*.md`, `answers-all.tsv`, `report.md`, reader v2's `answers.tsv`, the sample) **match byte for byte** the baseline from the old binary — in both v2 and v3 mode. The v3 gates rejected nothing on 2,600+ v1/v2 lines.
- **Positive control of the baseline.** The old binary reproduces the committed `fables/*.md` and the reader's `answers.tsv`.

### Testbeds (public domain, Project Gutenberg)

Downloaded with the sources pipeline: manifest `sources.jsonl` → `sources plan` → `curl -K` → `sources verify`: ok 2, fail 0. The book pages are saved in `pages/`.

| testbed | source | status | cut | paragraphs | sentences | words |
|---|---|---|---|---|---|---|
| `speckled-band` — a detective story resolving the past | Conan Doyle, *The Adventures of Sherlock Holmes*, PG #1661 → "VIII. The Adventure of the Speckled Band" | PD: author † 1930 (UA/EU — since 2001), USA — 1892; PG page: "Public domain in the USA" | from heading VIII to IX | 251 | 595 | 9934 |
| `arabian-nights` — frames within frames | Andrew Lang (ed.), *The Arabian Nights Entertainments*, PG #128 → introduction "The Arabian Nights" and "The Story of the Merchant and the Genius" with the stories of the first and second old men | PD: Lang † 1912, translators from his circle (Leonora Lang † 1933), USA — 1898; PG page: "Public domain in the USA" | from "The Arabian Nights" to "The Story of the Fisherman" | 121 | 284 | 5180 |

- Sentences — the `en::tok` tokenizer and sentence boundaries, as in `tools/db/src/tales.rs` (`poly.rs`). Paragraphs where the tokens do not reproduce the text verbatim — 0.
- Headings of nested stories are separate sentences: they open frames.
- For measuring the world only; not used for training.

### Measurement protocol

1. **Questions — before the LLM.** 35 questions (`questions-v3.tsv`: 20 on "The Speckled Band", 15 on "The Arabian Nights") were written from the testbed texts before the first v3 LLM call. Question types:
   - time;
   - knowledge;
   - frames;
   - control.
   
   For each question the formal `qa3.rs` query and the expected answer were recorded. Hashes of the questions, the sentences of both testbeds and `qa3.rs` — `freeze.txt` (14:24Z).
2. **LLM.** An event description in a query is verb variants, a participant and a sentence range (`ev:die|death/char:julia@s85-160`), because ids are unknown before the run.
3. **Pilot.** The first part of "The Arabian Nights" — one call, $0.85 including the format repair.
   - It showed a cascade: the LLM referred to an event it wrote one line below (`before=E5` before `E5`). The reference gate rejected 12 lines.
   - One paragraph about forward references was added to the prompt, and part was re-asked. The pilot output is in `pilot/`.
   - Questions and expected answers did not change.
4. **Blind measurement.** Before it, hashes of all the code — `freeze-eval.txt`: `qa3.rs` is the same as in `freeze.txt`.

### LLM: two worlds

Opus `claude-opus-5-5` (high) via `prag::opus`, API. Parts of up to 95 sentences (`V3_PART`). The second and later parts receive:
- the preceding text;
- a compact v3 world: entities of the current frame's world, frames with open ones marked, claims, off-line events, the latest events from all worlds, free ids from all worlds.

Format repair — one per part, and only for rejected lines.

| testbed | sentences | parts | lines | accepted | format (after repair) | gates | no commands |
|---|---|---|---|---|---|---|---|
| speckled-band | 595 | 7 | 1182 | 1176 | 0 (4 single-line repairs) | 6 | 0 |
| arabian-nights | 284 | 3 | 712 | 712 | 0 (1 repair) | 0 | 0 |

**The six gate refusals are LLM errors.**
- Four are backward anchors: line `^s206` after `^s207`, three `^s593` lines after `^s594`.
- One is a silent change of Holmes's goal without a cause.
- One is a cascading reference to a rejected intention.

**Commands.**
- "The Speckled Band": say 336, event 252, move 93, obj 71, set 63, feel 54, plan 50, **claim 48**, loc 41, char 35, rel 29, **time 24**, believe 23, **frame 21**, mention 14, **reveal 12**, **know 6**, cursor 5, have 4, **branch 1**.
- "The Arabian Nights": event 183, say 141, move 66, plan 61, feel 53, set 46, rel 31, char 29, loc 26, obj 19, **frame 8**, cursor 7, **know 6**, give 6, believe 6, mention 5, **claim 5**, have 4, **time 3**, done 3, **reveal 2**, unset 1, **branch 1**.

| testbed | events (happened / in branches) | frames | narrative levels (nesting) | claims | reveals | knowledge |
|---|---|---|---|---|---|---|
| speckled-band | 588 (587 / 1) | 3 | 2 (3) | 37 (48 lines including status changes) | 11 | 6 |
| arabian-nights | 324 (323 / 1 lie) | 4 | 3 (3) | 5 | 2 | 6 |

**Cost.** 17 calls, output 290,755 tokens (including thinking), cache write 545,840, cache read 142,127, 2514 s — **$8.57** per `total_cost_usd` (API, actual price).
- Estimate before the run (`v3-estimate`, upper bound: 800 output tokens per sentence at $25/M) — $23.47 plus repairs. Ceiling — $40, checked before every call.
- Pilot — $0.85 (2 calls).
- "The Arabian Nights" — $2.36 (4 calls, output 90,484).
- "The Speckled Band" — $5.36 (11 calls, output 169,965).

### Answers from the state (35 frozen questions)

`qa3.rs` answers via the formal query, from the state only, with ^sN anchors. Scoring — the expected words in order plus anchors within range. Manual check — on all 35 questions, criteria in `freeze-*/manual.tsv`.

| type | questions | freeze-1 (blind), manual | freeze-2 (not blind), manual |
|---|---|---|---|
| time | 9 | 6 (+1 "not in state") | 8 (+1 "not in state") |
| knowledge | 11 | 8 | 8 |
| frames | 11 | 7 | 10 |
| control | 4 | 3 | 3 |
| **all** | **35** | **24 (69%)**; automatic 26 | **29 (83%)**; automatic 30 |
| "The Speckled Band" | 20 | 12 | 16 |
| "The Arabian Nights" | 15 | 12 | 13 |

**freeze-1 — blind.** The code was frozen (`freeze-eval.txt`) before the first run of the questions. 26 automatic hits, 24 manual: two are word coincidences.
- SB-T2: first among the "flash-forwards" are Watson's narrating acts, not Roylott's death.
- SB-R4: "level 1" is Watson's time frame, not Holmes's explanation.

**freeze-2 — NOT blind**, after analysing the freeze-1 misses. Questions, expected answers and event descriptions are the same. Three model rules were changed:
1. **Narrative level — by narrators** (Genette). The LLM put the whole story in Watson's time frame (`frame open F2 narrator=C1 level=1 time=past before=E1`) — a recollection by the same narrator, not a new level. Frame nesting remains a separate number.
2. **Flash-forward and recollection — only with their own time declaration**: a placement of the event, `time` or `reveal` about it. Without this, every event told before a recollection would "flash forward" merely because of the recollection.
3. **The cause of a state change is no later than the next event on the line** (a soft edge). The LLM writes the consequence (`set C17 life=dead cause=E97`) where the narrative reaches it.

Regression after the changes — the same 31 files byte for byte. Tests are green.

**What was still missed (freeze-2):**
- **SB-K7.** The event description in the query looked for Roylott's death among ^s540–575. But the LLM correctly resolved it into the canonical event E4 from ^s5 — rumours in the first paragraph. The answer is in the state: `truth E4` — "playing with a dangerous pet" (inquest, K32, wrong) and "bitten by the swamp adder" (reveal ^s565). A query failure, not a world failure.
- **AN-K3, AN-K5.** The LLM attached the reveal to the event "the wife took the son away" (E172), while the query took "turned him into a calf" (E173). The old man learned it from ^s195 — this is in the state, but on the neighbouring event.
- **AN-K4.** "I cannot tell you what it was, as I do not know" was recorded as the narrator's utterance about E315. The event description in the query took the third old man's request (E313).
- **SB-R4.** The LLM recorded Holmes's explanation as dialogue, not as a frame. There is no level 1 for Holmes in the state.
- **SB-T5** — "not in state": "I knew that we should find a ventilator before ever we came" is recorded only as the utterance ^s466. There is no "guessed" event placed before the arrival.

### Example: solving "The Speckled Band" — a change of knowledge, not of the past

The LLM wrote 1182 lines. Frame structure:

```
F1  level 0  Dr. Watson → reader                     ^s1–…     root (memoir; written after everything)
└ F2  level 0*  Dr. Watson                          ^s6–^s595  time frame: the whole story is before E1 (the act of writing)
  └ F3  level 1  Helen Stoner → Holmes, Watson      ^s68–^s189 (9 spans: Holmes asks follow-up questions — the frame is closed and continued)
                                                               same world, past: before E15 (her arrival)
```
\* narrative level is counted by narrators; nesting is 1.

**Julia's death is one event, E97.**
- First mention — ^s89: "She died just two years ago".
- Time: `after=E96` (years of hard work). At ^s153 — `time E97 AFTER E160` (the doctor has already been sent for).
- Consequence: `set C17 life=dead cause=E97`.

Around it are observers' claims, each with its own status history (`claims about=E97`):

| claim | who | interpretation | status over text time |
|---|---|---|---|
| K11 | coroner | "no satisfactory cause of death could be found" | doubts ^s165 |
| K15 | Helen | "she died of pure fear and nervous shock" | believes ^s174 → doubts ^s444 (Holmes: "No, I do not think so") → **wrong ^s577** (reveal) |
| K19 | Watson | "Julia was alone when she met her mysterious end" | believes ^s226 → doubts ^s486 → **wrong ^s577** (reveal) |
| K20 | Holmes | "the gipsies, intimate with the doctor, were behind the death" | believes ^s229 → doubts ^s233 → **wrong ^s572** ("I had come to an entirely erroneous conclusion") |

**The reveal at ^s577.** Holmes's words to Watson (E571) on the train the next day.
- What: "Roylott's swamp adder from India bit Julia".
- Cause: the bite E572, agent — the swamp adder.
- Wrong: K12, K15, K19.

There is no new event "murder two years ago": the event is the same, E97. What changed is knowledge:
- `learns char:watson E97 at=^s540` → "at ^s540 does not yet know the truth — learns since ^s577";
- the reader — also since ^s577;
- Helen — "does not know" (the text never explains it to her).

Holmes knew about the event itself since ^s89 as a listener of the frame. He knew the truth since ^s576–^s577: claims K23, K25, K26 → knows.

**Roylott's death is a flash-forward.**
- ^s5, rumours in the first paragraph: `event E4 verb=die patient=C4 before=E1` (before the act of writing).
- ^s565, the climax: the LLM **did not create a second death**. Instead:
  - `event E556 verb=bite …` (the bite);
  - `set C4 life=dead cause=E556`;
  - `reveal E4 by=E555 cause=E556 interp="bitten by the swamp adder, Roylott died within ten seconds"`.
- Now E4 is "not earlier than E556": told at ^s5, happened at ^s565. In the answer to "what was told first but happened later" it comes first: "but happened later than 525 events told later".
- Claims about E4:
  - the public, K1 — "more terrible than the truth" → wrong;
  - the inquest, K32 — "indiscreetly playing with a dangerous pet" → wrong;
  - Holmes, K37 — "indirectly responsible" → knows.

**Time is a partial order, with an explanation.** `order E97 E15` → "E97 die … — before — E15 arrive. Why: E97 ≤ F3 (in frame F3); F3 < E15 (frame F3 — past). E15 told earlier (^s12 vs ^s89)".

`sort` reconstructs a chronology that the text does not have:
1. mother's death ^s77;
2. Julia's engagement ^s94;
3. Julia's death ^s89;
4. Helen's engagement ^s183;
5. Helen's arrival ^s12;
6. the bite ^s565;
7. Roylott's death ^s5.
### Example: Scheherazade's frames (`arabian-nights`)

The LLM wrote 712 lines. Gates rejected 0; sentences without commands — 0.

```
F·root    level 0  narrator → reader                           ^s1–…            81 events
└ F1      level 1  Scheherazade → Schahriar, Dinarzade        ^s66–^s93, ^s106–^s283   new world (its own axis 1)
  ├ F2    level 2  first old man (with the hind) → genie, merchant   ^s151–^s212      same world of the tale, past: before E139 "came with the hind"
  └ F3    level 2  second old man → genie, merchant, …        ^s217–^s274      same, past: before E238
```

- **Its own world and its own cursor.**
  - The merchant and the genie live only in the world of F1: they are absent from the root.
  - An event of the tale cannot cause the sultan's feeling: in the test this mutation turns red at the world boundary gate.
  - At dawn (^s93) Scheherazade falls silent: the frame is closed; at level 0 are Dinarzade, the sultan, the vizier.
  - At ^s106 frame F1 is continued with the same id. Its world and cursor carry on: the genie still has his sword raised.
- **Different axes.** "Which came first: Scheherazade's wedding or the death of the genie's son?" → "in another world than". An invented story does not sit on the time axis of the one who hears it.
- **One event — the killing of the genie's son.**
  - Event E80 — from the first mention (^s77, the genie's accusation), placed `before=E79` (earlier than the genie's appearance).
  - Claims:
    - the genie's K1 — "the merchant killed my son": believes → knows;
    - the merchant's K2 — "could not have killed someone he never even saw": doubts → **wrong**.
  - Reveal at ^s85 (the genie's words, E89): instrument — date stones (O9), cause — the throw E87, time — `during=E77` (while eating).
  - Derived from this: the son's death is after the start of the merchant's journey, during the meal. A second "killing" event (mutation) hits the identity gate.
- **A lie.** "Your slave is dead" is the content of lie E181 in the `lie` branch. The old man's claim K4 — "the slave died" (believes).
- **A solution as a change of knowledge.** The old man's claim K5 — "the son simply vanished, he will be found". Reveal E172 at ^s195: the steward's daughter says the son was taken and turned into a calf by the wife. K5 → wrong.
- **The third old man.** Scheherazade does not know his story: "I cannot tell you what it was, as I do not know".
  - The LLM recorded the telling E315 without a frame and without content, and the not-knowing as the narrator's utterance about E315. The state invented nothing.
  - Query AN-K4 did not find this: the event description picked up another utterance of the third old man.

### Negative controls of the specification

| control | how it was checked | result |
|---|---|---|
| flashback without a time mark → teleportation gate | `flashback_without_time_mark_is_teleport`: Helen on Baker Street; a past event "at Stoke Moran" — without a frame, in a past frame, with `before=` | without a mark — **teleportation**; with a frame or `before=` — passes. The neighbour on the story axis is the frame's states, not the present ones: in the frame Helen is already at Stoke Moran, and an event on Baker Street without `move` is also teleportation |
| Watson's `claim` does not change the canonical world | `claim_does_not_change_canonical_world`: snapshots and state counts of all entities and events before and after the claim; on real output, the world without all `claim`s is the same (`real_vmm_v3_output_and_its_mutations`) | **no change**. A claim recorded as an event in the `belief` branch is red when it tries to change state (branch) |
| a reveal does not create a second event | `reveal_does_not_create_second_event`: events and narration registry before and after `reveal` — the same count; a mutation of real output (the solution recorded as a new "murder" event) | **same number** of events. `reveal` of a nonexistent one — reference; a second death — **identity**; overwriting an existing role — identity |

On the real worlds the same is visible without tests:
- The LLM itself resolved the rumours of Roylott's death (^s5) and the climax (^s565) into one event, E4.
- Julia's death is one event, E97.
- Claims gave no state to any entity: `claims` is a separate registry K.
### v3 commands

```
export PATH=$HOME/.cargo/bin:$PATH
R=$MOVA_DATA/runs/world-v3-2026-09-26
# sources: manifest → plan → curl → verification
~/Mova/target/release/sources plan $MOVA_DATA/raw $R/queues $R/sources.jsonl
curl -sS --fail -K $R/queues/curl-www.gutenberg.org.cfg --create-dirs
~/Mova/target/release/sources verify $MOVA_DATA/raw $R/sources.jsonl
# testbeds: cut out a story, sentences (en::tok)
../target/release/world poly-prep $MOVA_DATA/raw/en-narr-pg1661-doyle-holmes/pg1661.txt \
    "VIII. THE ADVENTURE OF THE SPECKLED BAND" "IX. THE ADVENTURE OF THE ENGINEER’S THUMB" $R/stories/speckled-band.tsv
../target/release/world poly-prep $MOVA_DATA/raw/en-narr-pg128-lang-arabian-nights/pg128.txt \
    "The Arabian Nights" "The Story of the Fisherman" $R/stories/arabian-nights.tsv
../target/release/world v3-estimate $R speckled-band arabian-nights   # cost estimate before the run
../target/release/world v3-prompt arabian-nights $R 1                # prompt of a part, no call
../target/release/world v3-vmm arabian-nights $R                     # LLM in parts, cap WORLD_V3_BUDGET ($40)
../target/release/world v3-build arabian-nights $R                   # gates → story-arabian-nights.md
../target/release/world v3-query speckled-band $R order ev:die/char:julia ev:arrive/char:helen@s10-40
../target/release/world v3-eval $R $R/questions-v3.tsv               # frozen questions → answers-v3.md
../target/release/world v3-cost $R
WORLD_V3=1 ../target/release/world ftqa-build golden-goose $MOVA_DATA/runs/world-v2-2026-09-26   # old runs on the v3 executor
```

### v3 tests (`tests3.rs`)

`cargo test --release` — **33 tests**: 22 old ones (v1, v2, reader v2) unchanged and 11 new.
- `prompt_example_v3_passes_all_gates` — positive control. The prompt example ("The Cat and the Vase", invented) passes all gates, and the state is as intended:
  - Tom's frame, level 1, its own world and its own cursor ("morning" is only in the frame's cursor);
  - the broken vase is told after Mary arrives, but happened earlier;
  - "Tom left before Mary arrived" — derived transitively via the cat's jump;
  - Mary's claim: believes → doubts → wrong (reveal);
  - the reveal added an agent and a cause, there is still one event;
  - knowledge — from which sentence;
  - a lie in the `lie` branch, off the line;
  - "At dawn" — from the text;
  - the present is not overwritten by the past: Tom is in the shop, though in the frame he went to the market.
- `linear_v1_v2_is_special_case_of_v3` — the v1 and v2 examples on the v3 executor give a byte-identical md world; all events are on one line, in order.
- `flashback_without_time_mark_is_teleport` — **negative control of the specification**:
  - a past event "at Stoke Moran" without a mark — teleportation;
  - with a past frame or `before=` — not;
  - in the frame Helen is already at Stoke Moran, and an event on Baker Street without `move` is also teleportation, because the neighbour on the story axis is inside the frame;
  - the death from the past carried over into the present, the place did not.
- `claim_does_not_change_canonical_world` — **negative control**:
  - snapshots of all entities and events before and after Watson's claim are identical;
  - a claim recorded as an event in the `belief` branch does not change state (branch);
  - an event that has already changed the world cannot be declared a dream.
- `reveal_does_not_create_second_event` — **negative control**:
  - after `reveal` the number of events is the same;
  - a reveal of a nonexistent one — reference;
  - the solution recorded as a new "murder" event — a second death, identity;
  - `reveal` does not overwrite an existing role.
- `v3_gates_are_not_mute` — every new command turns red at its own gate (frame, reference, silent change, time, world boundary, branch, anchor, identity, continuity).
- `v3_format_is_strict` — the format rejects 29 corrupted v3 lines, 12 correct ones pass.
- `time_is_a_partial_order` — transitivity, DURING/contains, SAME (≈), the unknown stays unknown, closure, `sort`.
- `real_vmm_v3_output_and_its_mutations` — real LLM v3 output (Scheherazade's frame and the start of the merchant's tale, `data/cmds-v3-merchant.txt`) is clean. Mutations turn red at their gates:
  - without claims the world is the same;
  - the solution as a new event — identity;
  - "the killing after the genie's appearance" — time;
  - a tale event as the cause of the sultan's feeling — world boundary;
  - a frame of the wrong level — frame;
  - the killing declared a lie — branch.
- `narrative_level_counts_narrators_not_frames` — narrative level by narrators: Watson's memoir with a time frame is level 0, Helen's testimony is 1; nesting is 3.
- `dream_frame_does_not_change_canonical_world` — a dream frame: its own world copy, events in the `dream` branch off the line. "In the dream he flew, but in reality he was at home". A dream as the cause of changes in the canonical world — world boundary.
### What next

1. **Narrative level vs time frame.** A memory of the same narrator is a time frame, not a new level.
   - In freeze-2 the level is already counted by narrators (`Story::narr_level`).
   - The next step is in the language itself: `frame open … time=past` without a change of narrator does not require `level+1`.
2. **A single view of states in story time.**
   - Currently a character's states in a past frame live in the frame's world. Between two frames about the same time they are reconciled only through canonical events.
   - For example, the identity gate does not see a second death in a neighbouring past frame: it has its own projection.
   - What is needed is a history of states of each entity on the story axis from all worlds — "immutable states" on the partial order.
3. **Parallel lines** (specification, item 5): several cursors on a shared axis. Currently `time=now` continues the parent's line, and there is no separate parallel line.
4. **Global entities and epochs** ("Hierarchical and global worlds"):
   - scope `scope=global|book|tale`;
   - Baghdad and Harun al-Rashid are shared across tales;
   - epochs — re-linking between tales, as described.
5. **A natural-language v3 question reader**: UD parsing (as in reader v2) → `qa3` queries. Currently questions are formalised by hand before the run.
6. **SLM change scribe (v4)**: first frames and `time` from the UD tree and signals — pluperfect, *years ago*, *before*, discourse connectives. Gold — LLM v3 output with gates.

## Tests

`cargo test --release` — **33 tests**: 13 old (v1), 6 v2, 3 reader v2 (all unchanged) and 11 v3 — in the section ["v3 tests"](#v3-tests-tests3rs).

**Old (v1):**
- **Positive control:** the example from the prompt passes all gates.
- **Negative controls for each gate:**
  - teleportation;
  - reference to a nonexistent thing;
  - silent change;
  - two-way link mismatch — both via commands and via injection bypassing the executor;
  - nearness;
  - anchors;
  - strict format and closed sets.
- **Immutability:** the old state after changes is the same snapshot and the same delta.
- **Rollback:** a rejected command does not change the world.
- **Queries:** answers from the state on the example; an honest "not in state".
- **The questions file** is correct.
- **Scoring** takes order into account.
- **Mutations of real LLM output.**

**New (v2):**
- `prompt_example_v2_passes_all_gates` — positive control: the v2 example passes the gates, and the state is as intended:
  - the promise is kept by E5;
  - two feelings together;
  - `done` with a cause;
  - the wing is detached;
  - shared agents, two acts;
  - `neg`, `before`, `when`, `about`;
  - a false and an abandoned belief;
  - `here=0` and `mention` do not move the cursor;
  - `benefactor`, `num`.
- `v2_gates_are_not_mute` — every new command turns red at its own gate (silent change, continuity, reference, link, nearness).
- `v2_links_catch_injected_mismatch` — a mismatch in `plans`, `parts`, `owners` injected bypassing the executor, and "a part lying separately".
- `v2_format_is_strict` — the format rejects 30 corrupted v2 lines.
- `real_vmm_v2_output_and_its_mutations` — real LLM v2 output (the wen) is clean, and 7 mutations turn red at their gates.
- `natural_questions_are_answered_from_state` — the natural question reader on the v2 example: 11 questions (not from FairytaleQA) and "not in state" negative controls.

**Reader v2:**
- `reader_parses_questions_with_ud` — parsing with a UD tree: type, subject, anchor subordinate clause; predicate repair ("the princess *return* to"); "happen to X", "how many X", "what made X V".
- `reader_cuts_spans_from_anchor_sentences` — spans: place ("under a tree"), cause "X, so V" ("the hole was near the ground"), feeling with an intensifier ("sorely troubled"). An echo of the question is not an answer.
- `reader_v2_answers_from_state_and_controls` — the v2 example: 7 questions from the state with the reason "state: …"; "not in state" → sentence. Controls: without the index — byte-identical to the baseline; facts of another story — fewer than half state hits.

Three v1 fables on the v2 executor give **identical** `world-*.md` and `answers-*.md` (24/24, control 3/3): `$MOVA_DATA/runs/world-v2-2026-09-26/fables/`. After reader v2, a rebuild gives the same files byte for byte.

## Files

**Crate:**
- `src/types.rs` — ids, states, closed sets, values (v2: registries `P`, `B`, intention and belief fields, `Tags`);
- `src/lang.rs` — parsing of the change language (v2: 8 new commands, `+`, `Ev2`);
- `src/world.rs` — model, executor, gates;
- `src/query.rs` — state queries;
- `src/md.rs` — md graph (v2: intentions, beliefs, event details);
- `src/qa.rs` — v1 formal queries;
- `src/vmm.rs` — prompt and Opus call (v2: `rules_v2`, example, parts, repair retry);
- `src/ftqa.rs` — FairytaleQA: CSV, reconstructing tales from sections, sentences;
- `src/ans.rs` — v1 natural-language question reader from the state, baseline, ROUGE-L (freeze-2, unchanged);
- `src/report.rs` — measurement, slices, samples, manual verdicts;
- `src/tree.rs` — UD trees (`en::annotate`, model loaded once per process);
- `src/qframe.rs` — question parsing with a UD tree;
- `src/index.rs` — unified index of state facts and lemma search with field weights;
- `src/span.rs` — anchor-sentence span by the role of the question word;
- `src/reader.rs` — reader v2: state answer, merge with a reason, span development on validation;
- `src/rreport.rs` — reader v2 measurement, controls, sample, manual verdicts;
- `src/tests.rs`;
- `sql/fable.sql`;
- `data/questions.tsv`;
- `data/cmds-2451-fox-crow.txt` — real LLM v1 output for the mutation test;
- `data/cmds-v2-wen.txt` — real LLM v2 output ("How an Old Man Lost His Wen") for the v2 mutation test;
- `src/story.rs` — v3 narration layer: story time axis (partial order, path explanation, closure), frames, event metadata, knowledge, reveals;
- `src/narr.rs` — v3 pipeline at the root: frames (own world: past projection, new world, branch; closing — carrying over into the parent's world by commands through gates), registering events on the axis, `claim`/`time`/`branch`/`know`/`reveal`;
- `src/qa3.rs` — v3 queries (time, knowledge, frames) and scoring with anchors;
- `src/poly.rs` — testbeds: a story from a Gutenberg book → sentences (`en::tok`);
- `src/tests3.rs` — v3 tests;
- `data/cmds-v3-merchant.txt`, `data/v3-merchant.tsv` — real LLM v3 output (Scheherazade's frame and the start of the merchant's tale) and the text (Lang, PG #128, public domain) for the v3 mutation test.

**Fables run** `$MOVA_DATA/runs/world-2026-09-26/`:
- `prompt-<doc>.txt`;
- `raw-<doc>.txt` — raw LLM output;
- `cmds-<doc>.txt` — accepted commands;
- `world-<doc>.md` — md world;
- `answers-<doc>.md`;
- `questions.tsv`;
- `calls.jsonl`.

**v2 run** `$MOVA_DATA/runs/world-v2-2026-09-26/`:
- `tales.txt`;
- per tale: `prompt-<tale>-p<N>.txt`, `raw-…`, `repair-…`, `cmds-<tale>.txt`, `world-<tale>.md`, `answers-<tale>.md`;
- `calls.jsonl`, `log-vmm-*.txt`;
- `freeze-1/` — blind run: hashes, report, answers, samples, verdicts;
- `freeze-2/` — reader v2: the same;
- `seen-questions.tsv` — 127 questions seen before the edits;
- `fables/` — three fables on v2.

**World v3** `$MOVA_DATA/runs/world-v3-2026-09-26/`:
- `sources.jsonl`, `queues/`, `download.log`, `verify.log`, `pages/` — sources and verification; texts — `$MOVA_DATA/raw/en-narr-pg1661-doyle-holmes/`, `$MOVA_DATA/raw/en-narr-pg128-lang-arabian-nights/`;
- `stories/*.tsv` — testbed sentences;
- `questions-v3.tsv`, `freeze.txt` — frozen questions;
- `prompt-*`, `raw-*`, `repair-*`, `fixed-*` — LLM parts; `cmds-*.txt` — all commands; `calls.jsonl`, `log-vmm-*.txt`;
- `pilot/` — pilot first part (before the prompt edit);
- `story-*.md` — v3 md worlds;
- `freeze-eval.txt`, `freeze-1/` — blind measurement (answers, manual verdicts);
- `freeze-2/` — second round (not blind);
- `answers-v3.md` — latest answers; `regress.md` — v1/v2 regression on the v3 executor (31 files, both modes).

**Reader v2** `$MOVA_DATA/runs/world-reader-2026-09-26/`:
- `freeze.txt` — freeze (hashes of sources and model);
- `valid/` — validation part (tales and questions) for development;
- `dev-questions.tsv` (127), `excluded-extra.tsv` (12);
- `answers.tsv`, `report.md` — all 432; `answers-clean.tsv`, `report-clean.md` — 305 non-dev;
- `report-{shuffle,noindex}[-clean].md` — controls;
- `manual-sample.tsv`, `manual-verdicts.tsv` — fresh sample and verdicts.

## Licence

Elamur Cosmic License ([LICENSE.md](../LICENSE.md)). Third-party data keeps its own terms, as noted where it is used.
