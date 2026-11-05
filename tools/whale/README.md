# mova-whale — abstract Whale workers

🐋 Whales are among Mova's animals (Rust on Mova): giants that write big articles on special missions a deployment gives them through its own prompt and specification; the 🐙 Octopus that assigns their work is a large language model (LLM). See [the sea animals of Mova](../../docs/animals.md).

A Whale turns a pinned, immutable task queue into versioned bundles. For each task it:

1. extracts a bounded source context;
2. renders a pinned prompt template;
3. makes at most one teacher call through `mova-teacher` (Codex CLI, no fallback);
4. validates the reply against an output contract supplied by configuration;
5. runs the configured length and originality gates;
6. publishes an immutable bundle with receipts.

The crate also provides pool and swarm scheduling, status, pause/stop markers, a bounded
agent-session lane and the Crab `ExternalRoleHandler`.

The public code knows no concrete task domain. Prompt texts, output schemas, required
fields, length and originality parameters, source-kind choices, model selection, queues and
deployment paths come from configuration files given at run time. A deployment keeps them
outside this crate. Nothing in the crate depends on private code.

## Build and run

    cargo build --manifest-path tools/whale/Cargo.toml --offline --locked
    cargo test  --manifest-path tools/whale/Cargo.toml --offline --locked

The binary is `mova-whale`; every subcommand takes one absolute path to a JSON configuration file (`mova-whale <subcommand> CONFIG [--once]`):

| command | effect |
|---|---|
| `whale-prepare CONFIG` | pins items, extracts contexts, renders prompts and writes worker configs. No teacher call. |
| `whale CONFIG [--once]` | runs one disjoint worker. Makes at most one teacher call per task. |
| `whale-status CONFIG` | reads receipts and worker status |
| `whale-project-lengths CONFIG` | trims retained length-gate quarantines to a whole paragraph or sentence. No teacher call. |
| `whale-swarm CONFIG [--once]` | runs the persistent multi-lane manager |
| `whale-swarm-status CONFIG` | reads the swarm database and actor status |
| `whale-discover CONFIG` | previews the default queue-file discovery. Writes nothing. |
| `whale-research CONFIG` | runs one bounded agent session in an isolated Git worktree |
| `whale-research-status CONFIG` | reads an agent job's report or reservation |
| `crab CONFIG` | serves Crab with `WhaleRoles` installed |
| `serve CONFIG DEPUTY_CONFIG SOCKET` | the Whales as an English-speaking animal on a private socket (see «In English»). `STOP` in the socket directory ends it |
| `ask SOCKET ID [urgent]` | sends arbitrary English from stdin; prints the English answer, then the JSON reply |
| `paper CONFIG [--no-terminal]` | one Whale reads one paper once, writes its skeleton description, asks its terminal (see «Reading papers») |
| `learn CONFIG [--no-terminal]` | one Whale learns on one paper round after round until its own description passes the gates (see «Learning on a paper») |
| `learn CONFIG --replay ROOT...` | the same with a replay terminal: answers recorded in earlier state roots or `paper` run roots, by question; nothing is asked (measurement) |
| `learn-report STATE_ROOT...` | per paper: rounds, first/best/last words, rounds without gain, questions by kind, terminal calls, facts kept per answer, stop (read-only) |
| `extraction-check TEXT...` | a paper text's extraction quality against the default limits; `WHALE_PROSE=1` adds the prose words Mova reads |
| `batch-plan CONFIG` | closes every finished assignment and gives each free Whale its next paper (one batch each); writes the learners' configurations |
| `batch CONFIG [--no-terminal]` | the rolling runner (one per run root): each Whale with its own terminal takes its next paper as soon as its paper is final; collects review-only teaching candidates; `--no-terminal`: one dry pass |
| `epoch CONFIG [--epoch N] [--doc ID] [--no-terminal]` | one epoch of texts per document: Mova's long cited text from the paper and its neighbourhood, then the Whale's terminal writes from the given sources by path (see «Epochs of texts»); `--no-terminal`: Mova only |
| `epoch-status CONFIG [--epoch N]` | every document's state in an epoch (read-only) |

## In English

`mova-whale serve CONFIG DEPUTY_CONFIG SOCKET` runs the Whales as a persistent animal, like
[Sea Spider](../seaspider/README.md#in-english). A prompt is arbitrary English plus an optional
priority (`urgent` or normal); there is no command language. Mova's native deputy
(`mova_octopus::Deputy`) reads it: Mova's annotator, the world tree reader and the configured
capabilities. Sources reach a capability only as typed arguments bound through the parse: an
absolute URL or a catalog identifier (slot kind `id`, a hyphen-joined lowercase slug with a letter
and a digit such as `smith-2024-tree-parsing`; see `tools/octopus/README.md`, «Typed arguments»).
The answer is English realized deterministically (`answer.rs` over `mova_octopus::realize`). No
language model writes or interprets the answer.

Capabilities (handler `whale`, parameter `operation`; vocabulary in `deputy.example.json`):

| operation | slot | example prompts | answer |
|---|---|---|---|
| `describe` | `sources`: 1–8 URLs or ids | "Describe this paper: smith-2024-tree-parsing.", "Please survey ID.", "Survey this paper: URL.", "Could you summarize ID?" | queued at once: «I have queued 1 paper for description (octopus-r1-0): ID (“Title”). I am starting on it now.» Already described, waiting, failed before, or not in the catalog: said, nothing queued |
| `count` | — | "How many papers have you described?" | «I have described 12 papers. 1 paper is being described now and 2 waiting.» plus failed and local-only attempts |
| `status` | `source`: optional | "What is the status of ID?", "Tell me the status of URL.", "Are you busy?" | «ID is waiting; 1 request is ahead of it.», «I am describing ID now (since …).», «I described ID at …: 4,512 words. The bundle is in …», a failed attempt with its reason |
| `show` | `source`: one | "Show the description of ID.", "Show me the summary of URL." | «My description of ID (“Title”) has 4,512 words; I wrote it at …. It begins: “…”. The full text is …/description.md. The bundle is in ….» |
| `pause` / `resume` | — | "Pause the Whales.", "Resume the Whales." | writes or removes `PAUSE` in the run root: no new description starts; running ones may finish |

Refused (nothing is queued; the deputy keeps a review candidate and the answer says «I did not
act on this request because …»): negation ("Do not describe ID."), conditions ("If you can,
describe ID."), unsupported verbs ("Delete ID.", "Stop the Whales." — stopping is Crab's), a
missing or surplus source ("Describe the paper.", two sources to `show`), a source outside the
requested action's parse ("Describe ID because ID2 has a copy."), declarative narration, and
questions about an acting operation ("Is ID described?", "Are the Whales paused?", "Have the
Whales resumed?"): only `count`, `status` and `show` (declared `observes`) answer questions. A
sentence-initial "Survey ID." read as a noun is repaired to the imperative, and an identifier
after a colon ("Survey this paper: ID.") belongs to the request.

How it works:

- **Configuration** (`animal.example.json`, unknown fields refused): `run_root` (private 0700),
  `executable {path, sha256}` (the frozen `mova-whale` that runs each request), optional `codex`
  (handed to the children as `MOVA_CODEX_BIN`), `survey` (the survey run skeleton: `spec`,
  `teacher`, `store`, optional `originality`, `metadata_export`, `storage_floor_bytes`; it must not
  set `run_root`, `items`, `workers`, `max_items` or `worker_index`), `catalog {path, sha256, kind,
  id_field, path_field, title_field, licence_field, formats, url_fields, url_prefixes}`, `workers`
  (1–4), `max_waiting` (1–256) and `noun` (the word answers use, default `source`).
- **Catalog.** A pinned JSONL file, read once at start (its hash is checked; duplicate ids are
  refused). An id names a row; a URL names a row through `url_fields` (exact) or `url_prefixes`
  (`prefix` + a row field, e.g. `https://doi.org/` + `doi`). A source outside the catalog is not
  fetched: the answer says it is not in the catalog.
- **The queue stays pinned.** A request is appended to `requests.jsonl` in the run root (one
  fsynced line per event: `queued`, `started`, `launched`, `finished`). Each admitted request
  becomes its own new one-item Whale run in `runs/<job>/survey` — `whale::prepare` pins the item,
  then `mova-whale whale CONFIG --once` runs as a child of the pinned executable with at most one
  teacher call. No existing queue, manifest or bundle is rewritten; the specification's gates and
  receipts apply unchanged. This is the request inbox the service's scheduler admits.
- **No replay.** `started` is journaled before preparation. After a restart a started request
  without `finished` is resolved from its run's terminal receipt when one exists, otherwise it is
  recorded `uncertain` and never relaunched. A request that was only `queued` (nothing prepared, no
  call) waits again. A source with a failed attempt (quarantined, uncertain, preparation failure)
  is not attempted again by a new prompt; it needs review and a new run root.
- **Priority without preemption.** `workers` request slots; waiting requests start urgent first,
  then by arrival. A prompt never interrupts a running description.
- **Answers never wait for work.** Describe prompts are queued and answered at once; questions
  are answered from the journal, the shared state and the bundle store (`store/<store_prefix>/<kind>/
  <sha256(id)>/<version>/`). "How many … described" counts sources with at least one complete
  bundle version in the store, whoever wrote it.
- **Singleton.** `service.lock` in the run root and the socket angel's `angel.lock`; a second
  service on the same run root or socket is refused. `STOP` in the socket directory ends serving;
  the service then waits for running children before exiting.
- **Crab.** The service is an ordinary Crab worker (`crab-worker.example.json`): `argv` from
  Crab's trusted config, `stop_marker` = the socket directory's `STOP`. Whales do not start by default: the
  example has `autostart: false`; only an explicit deployment config or an explicit English request
  to Crab starts them. Set `stop_grace_seconds`
  above the teacher timeout (default 1800 s): a child killed during its call is later recorded
  `uncertain`, never repeated.
- **Asking Sea Spider.** Optional `seaspider_socket` (absolute) in the animal configuration; unset
  by default, and then the Whales do not talk to Sea Spider. Set, `seaspider::ask_sea_spider(cfg,
  id, english, urgent)` sends one English request to Sea Spider as caller `whale` through
  `mova_octopus::relay::ask_animal` and returns its English answer; Sea Spider's own allowlist
  decides whether it answers and journals the request. No Whale capability calls it yet.

## Reading papers

`mova-whale paper CONFIG` runs one Whale on one paper and asks its private terminal
(`mova-terminal`, `tools/terminal`); `--no-terminal` journals the questions without asking.
`mova-whale paper-check CONFIG...` validates several Whale configs side by side. The model is
found through `WORLD_EN_MODEL` (or `$MOVA_DATA/en/models/en.bin`).

- **Reading through Mova** (`reader.rs`). PDF small-capital splits are rejoined first («S TACK P
  TR» -> «STACK PTR»: where the joined word is known, where the split repeats in the text, and next
  to such a join; a lone «A» or «I» before an acronym stays). Markdown headings give sections
  (references, bibliography and acknowledgements are not read as content; appendices are);
  paragraphs are split into sentences. Not content, and neither restated nor asked about
  (`furniture`, counted per reason in `filtered`): bibliography entries (with or without a venue;
  after two author-year entries in a part, its short continuations too), page furniture
  (addresses, venue and page lines, footnote markers), name lists, table rows (mostly numbers or
  codes), maths residue (formula symbols, single letters) and bracketed citation lists. Author and
  cited names (`names`) are never unknown terms; unknown terms are counted under their normalized
  form (`knowledge::norm_term`: case-folded, plural acronyms and nouns in the singular, so
  «MRFs»/«MRF» and «treebanks»/«Treebank» are one term), asked and explained in the paper's own
  most frequent spelling (`forms`). `WHALE_READ_DEBUG=1` prints every filtered sentence and refused
  clause.
  Every sentence is parsed by Mova's annotator (`world::tree::Tree::parse`) and reduced to clause
  skeletons (`world::read::skeleton`). Words Mova's dictionary does not know (`en::morph::known_form`),
  apart from cited author names, are counted.
- **Writing its own description.** An opening from the reading's own counts and most frequent
  nouns, then per section «In the part «…», the paper says this.» and the section's skeletons as
  plain sentences, each bound to its source sentence. Over budget, every section keeps the same
  share of its skeletons (evenly spaced); a short reading is not padded. The description is written
  and journaled before any question, and no answer text ever enters it.
- **Questions** (at most `max_questions`, 4 by default): unknown terms met in at least two
  sentences (acronyms first), then the longest sentence without a skeleton. Each is asked once; a
  usage limit, timeout or failure ends consulting cleanly, with no retry. Mova re-reads every
  answer and writes only its skeletons to `glossary.txt`.
- **Files** in a new private `run_root`: `journal.jsonl` (fsynced: start, read, draft, question,
  answer or no_answer, finish), `description.txt`, `reading.json`, `questions.jsonl`, `glossary.txt`,
  `teaching.jsonl` (every unknown term and every sentence without a skeleton, with its answer
  state, paper and licence; `open_school_admissible` only for CC BY, CC0 or public domain without
  NC, ND or SA terms), `metrics.json`.
- **Metrics:** paper and description words, readable and described sections (coverage), sentences
  read, unparsed and without skeleton, traceability (every content word of a written sentence
  occurs in its source sentence), the longest and median verbatim run shared with the source,
  unknown word types, questions selected, asked and answered, and how consulting ended.

### The private terminal interface (`terminal.rs`)

    pub trait PrivateTerminal {
        fn animal(&self) -> &str;                               // == the Whale's name
        fn ask(&mut self, q: &Question) -> Result<Answer, AskError>;
    }
    Question { id, kind: "term" | "sentence", subject, text /* includes context */, context: Vec<String> /* <= 3 sentences */ }
    Answer   { text, elapsed_ms, terminal_id: Option<String> /* mova-terminal question id */ }
    AskError::Limit(reason) | Timeout(reason) | Failed(reason) | Unavailable

A Whale config names its terminal as `"terminal": {"profile": ABSOLUTE_PROFILE_JSON, "poll_ms":
2000}`: a `mova-terminal` profile (`tools/terminal/README.md`) whose `animal` is the Whale itself.
`run` refuses a terminal of another animal, and holds `ROOT/holder.lock` of the profile for the
whole run (`TerminalLease`), so a second Whale cannot attach; `mova-terminal`'s own
`terminal.lock` serializes each question. `paper-check` refuses configs that share a Whale, run
root, profile file, session, owner marker or terminal root. Which model the session runs is the
holder's command line (`crab-tmux-worker … -- claude --model opus …` by default, Codex with a
`gpt-` model where configured), not the Whale's.

`MovaTerminal` is the real implementation, a thin adapter: `terminal::ask(profile, text)` sends
the question (refused, with nothing sent, unless the pane is idle and owned, and while an earlier
question is unanswered), then it polls `terminal::answer` every `poll_ms` and reads the screen
through `terminal::observe`. A usage or rate-limit message on the screen (`LIMIT_TEXT`) returns
`Limit` at once; `answer_timeout_seconds` without an answer returns `Timeout`. In both cases the
question stays pending in the terminal's journal, is never re-sent, and the Whale asks nothing
more. An answer counts only when its file exists and the pane is idle again. `NoTerminal` (dry run) and `FakeTerminal` (tests) start and ask nothing.

## Learning on a paper

`mova-whale learn CONFIG` (schema `mova-shark-learn-v1`) keeps one Whale on one paper, round
after round, until its own description passes every gate. The words come from Mova's reading and
from what the Whale has learned; never from padding, never from the terminal's text.

**State** (the Whale's private `state_root`, kept across papers and batches):
`knowledge/items.jsonl` (retained facts with provenance), `knowledge/asked.jsonl` (every question
key ever sent, answered or not), `knowledge/answers/SHA.txt` (answer texts, kept only for the
overlap gate), and per paper `papers/<id>/journal.jsonl`, `status.json` (`learning`, `passed` or
`stopped` with its reason, first start, deadline) and `round-NNN/` (`description.txt`,
`draft.json` with every sentence's basis, `gates.json`, `questions.jsonl`, `teaching.jsonl`,
`metrics.json`). `learner.lock` allows one learner per Whale; the terminal lease one Whale per
terminal.

**A round:** read (once per invocation) → (topic corpus: seed) → write with all retained knowledge
(and the pod's) → gates → (review hints when the round gained little, or before a pass) → pass:
stop.
Otherwise at most `max_questions` questions (4) about gaps never asked before: unknown terms the
Whale has no facts for (half the slots, acronyms first), a part it could not restate at all, the
longest prose sentences it could not restate. Each is asked once through its own terminal; Mova
re-reads the answer and retains only typed facts (`knowledge.rs`): an acronym's expansion, «X is
a Y», «X means Y», «X does Y» (object phrases of at most 5 words, below the overlap gate's 6), and
for sentence and part questions subject–verb–object readings whose every word occurs in the
source. Acronyms the paper defines itself («Grammatical Error Correction (GEC)») are understood
natively and never asked. The next round writes with the new knowledge.

**One call per round** (`batched`, default): the round's questions go to the terminal as one
numbered request asking for answers in shapes Mova's answer reader keeps, built from the source's
own words: «X stands for …» (an abbreviation), «X is a …», «X does …» for a term, «In the paper's
words: …» followed by simple subject–verb–object sentences that reuse the sentence's words for a
sentence or a part. The answer is split by number (`knowledge::split_answer`) and each part read
for its own question; every question key is marked asked on its own. Facts that only name a
category («Goldberg is a surname», «the label»: `knowledge::bare`) are flagged for review
(`review.jsonl` of the pod or the store), never kept or written.

**The pod** (`pod`, a shared directory for all learners of a batch; `pod.rs`): one store keyed by
normalized term with provenance per Whale, paper, item and answer. A learner publishes its term
knowledge and asked terms at its start and after every round, claims a term (under `pod.lock`)
before asking it, and asks nothing another Whale has claimed or knows; the writer explains with
its own items first, then the pod's (one sentence per fact relation, each bound to its item), and
the overlap gate covers every answer behind the pod's facts.

**Review hints** (`review`, `review.rs`): when a round gains fewer than `min_gain_words` (150) words
over the best earlier draft, and once before a draft passes, the Whale's own terminal receives the
description and the text of the parts it describes (requests of at most `max_bytes`, 100,000
bytes, wrappers included; parts that do not fit go to further requests) and answers with hint lines
`MISSING: part | what`, `WRONG: quote | why`, `UNCLEAR: quote | why`, `REPHRASE: part | how`,
`ORDER: advice`. Hints are the paper's working material: `papers/<id>/hints/round-NNN.json` and
`.md`, indexed by (description hash, paper hash) in `index.jsonl`, so the same pair is never asked
twice. They guide the Whale's own questions (terms and sentences a missing/wrong/unclear hint
mentions are asked first; journal `hints` lists what they chose and what was left aside —
rephrase and order hints, since the writer restates in the paper's order); their text never
enters a description: hint answers are kept with the terminal's answers, so the overlap gate
refuses any copy.

**Topic corpus** (`topic_corpus`, off by default; `topic.rs`): the frozen `mova-topic-corpus`
binary (`tools/topic-corpus`; a child process, since that crate depends on this one) seeds term
facts from the paper's closed topic corpus before each round's questions — the corpus explains
before the terminal is asked; the learner releases `learner.lock` for the seed only and stops
(`lock_lost`) if it cannot take it back — and writes `round-NNN/cited.md`, the round's description
with citations and a bibliography (`public` refuses closed sources). Journal `seeded`, `cited`. A
child's failure is journaled and never stops learning.

**Stops:** `passed`; `limit`, `timeout` or `failed` at the terminal (no retry; a later
invocation continues with new questions only — a sent question is never sent again);
`no_terminal` (`--no-terminal`: questions journaled, not asked); `nothing_to_ask`; `stalled`
(`stall_rounds`, 3, rounds in a row that gained no word over the best earlier draft after a round
that learned no fact the writer can use); `lock_lost`; `stop` (`stop_file` present before a round — a batch sets its run root's `STOP`; not final, the next invocation continues);
`round_limit` (`max_rounds`, 24 by default); `deadline` (`hours` from the first round, 12 by
default, or `deadline_unix` if earlier). A paper stopped `nothing_to_ask`, `stalled` or
`round_limit` under the current writer version is not reopened (journal `not_reopened`). Another
stopped paper assigned again with a later deadline (a batch's retry) is reopened: the new deadline, `max_rounds` counted from there, journal `reopened`. `import` names earlier `paper` run roots of the same
Whale: their answered questions become knowledge, every question they sent counts as asked.

**The writer** (`clause.rs`, `compose.rs`). Whole clauses from Mova's UD tree instead of
skeletons: determiners, prepositions, auxiliaries and negation stay; coordinated verbs, side
clauses and subject relative clauses become sentences of their own with their shared subject or
antecedent; the authors' «we / our / us» becomes «the authors / their / them»; bracketed citations
are left out; «X was made by Y» becomes «Y made X» where the verb form does not change. A clause
may share at most 12 words in a row with its source: adverbial clauses and obliques are moved
(fronted or put last) before optional modifiers are left out; negation is never left out; a clause
that cannot be brought under the cap is not used. «X not only does A, but (can) also B»
keeps both halves (the «but» half the parse hangs as an adverbial clause is the second half of the
coordination; «not only» is left out of the first, never «only» alone). Table and figure captions
are restated as «Table N shows …»; a sentence whose whole parse yields nothing is tried by its comma segments.
The description: an opening (title, size, parts, topics), per part a lead that connects it to the
previous one with its size and topics, its restated clauses in source order, each term explained
right after its first use from the Whale's knowledge (««GEC» is short for Grammatical Error
Correction.», «I understand «UAS» as the percentage of words.»), sentences learned through a
question, the numbers the part reports, and a closing. Standalone title-case lines between prose
open parts (PDF subsection titles).

**Gates** (`gates.rs`; each has a red control in its tests): `words` (`min..=max`),
`near_duplicates` (Jaccard of content-word sets ≥ 0.8), `verbatim` (≤ 12 tokens in a row shared
with anywhere in the paper; a structural sentence's «…» quotation of a title is exempt),
`answer_overlap` (no 6-token sequence of any terminal answer, unless the paper has it),
`traceability` (every content word belongs to the sentence's basis — its source sentences, its
knowledge item, or the counts and titles it states — or to the fixed frame vocabulary
`compose::FRAME`; a term may appear in another form of the same normalized term),
`structure_share` (structural sentences ≤ 20 % of the words), `grammar` (every restated sentence,
parsed again by Mova on its own, is a whole clause: `clause::grammatical` refuses a bare
subordinated «-ing» form at the end — «…, while maintaining.» —, a participle after a determiner
read as the verb — «This architecture all the previously derived subtrees» —, a plural noun read
as the verb — «A number of parsing steps linear in …» —, a sentence without a finite verb by
parse, tag or dictionary, and a closing function word; the writer applies it before it keeps a
sentence). Parameters live in the configuration (`gates`).

The writer restates admitted source clauses and retained facts. Output length depends on how much of the source the
reader covers and on the configured bounds; asking the terminal does not guarantee a passing description. When a
description is short or ungrammatical, inspect its rejection diagnostics and source-bound mappings: Mova's parser
gives some long scientific sentences a wrong root, formulas and tables are not restated, and modifiers left out for
the verbatim cap shorten clauses.

Tests (`cargo test`, model required where they read): `pod::tests` (one term, one
question across the pod; bare facts to review), `learn::tests` (one call per round; `stalled` with
a red control at a high `stall_rounds`; an asked-out paper not reopened; review hints cached,
stored, used for questions, kept out of descriptions, with a copied hint failing the overlap gate;
the replay terminal), `review::tests`, `topic::tests`, `batch::tests` (rolling assignment with a scripted
learner: a passed Whale takes the next paper at once while a failed one keeps its own; a failed
paper given up at its deadline; a usage limit pausing every Whale; one runner per root, `STOP`, a
restart continuing the open assignment; with the real learner and a fake terminal a restarted
runner asks nothing again; selection by relevance and
length, never alphabetical; broken extractions refused once, red with the limits off; topic corpus
built, seeded with the lock released and cited, with a fake binary), `reader::filter_tests`
(furniture, names, repair, extraction, each with sentences that must stay), `knowledge::norm_tests`,
`clause::correlative_tests` and `gates::tests::grammar_gate_refuses_fragments` (red on the ma-2018
fragments). Negative controls, red then restored: the grammar gate always passing, the pod claim
ignored, the stall off, the extraction check off; a terminal failure counted as final, no pod pause, no
backoff, and assignment only when the whole pod is free (the old batch rule), each red on the rolling tests.

Tests: `learn::tests` (rounds retain and reuse knowledge and never ask twice across invocations;
a limit stops cleanly; a low bound passes without a question; a past deadline starts no round;
two runs identical), `gates::tests` (every gate red on its control), `batch::tests`,
`knowledge::tests`, `clause::tests`, and `tests/terminal_adapter.rs` (an answer counts only when
the terminal is idle again). Negative controls, red then restored: ignoring the asked set (a term
asked three times), answers pasted into the draft, the adapter returning on the answer file alone.

## Epochs of texts

`mova-whale epoch CONFIG [--epoch N] [--doc ID] [--no-terminal]` (schema `mova-whale-epoch-v1`,
`epoch.rs`) gives every document a growing set of texts. An epoch is just an increasing number;
no epoch is special. Per document and epoch N:

1. **Mova writes** (`compose::compose_epoch`, version `compose::EPOCH_WRITER`): first the
   description of the original exactly as `learn` writes it (`compose`), then what the document's
   neighbourhood says about its key terms (its most frequent nouns, its acronyms, the terms Mova did
   not know). The neighbourhood is the document's own texts of all earlier epochs (consultant
   texts, older and web descriptions, Mova's own) and the nearest documents of the global world,
   nearest first; each text is read whole by Mova's reader. Neighbour sentences are taken in
   rounds — every text's most relevant sentence, then every text's second — so the nearest texts
   all speak before one dominates; a sentence must speak of `mova.min_relevance` (2) key terms.
   Each clause passes the writer's guard (at most `verbatim_max` tokens in a row shared with the
   paper or any neighbour text except Mova's own earlier texts, no terminal-answer overlap, the
   grammar gate) and the near-duplicate check; nothing is padded. Every sentence ends with its
   citations (`[n]`) and the text closes with `## References` (number, entry, citation id).
   Gates: `mova.words` (`min..=max`), verbatim, near duplicates, every sentence cited and every
   citation resolved, structure share. A text below `min` is recorded `mova_short` (other gate
   failures `mova_gates`), not registered, and nothing is asked for that document.
2. The text is registered in the document's **dossier** (`dossier.rs`, kind `mova`).
3. **The Whale's own terminal writes** (unless `--no-terminal` or no `sol` step): one question from
   a pinned prompt template (`{{SOURCES}}`, `{{MIN_WORDS}}`, `{{MAX_WORDS}}` exactly once;
   `{{TITLE}}`, `{{DOC}}`, `{{EPOCH}}`, `{{WHALE}}` optional; example: `epoch-sol.example.md`)
   listing numbered sources **by path**, never pasted: [1] the original, [2] Mova's text of this
   epoch, then the document's earlier texts (newest epoch first) and the key text of each nearest
   document (its latest consultant text, else its original; `sol.neighbour_texts`, 12). With
   `stage_inputs` (default) the files are copied into the terminal's root (`inputs/<doc>/e<N>/`),
   which every terminal sandbox may read. Only hard limits cut the list (`max_inputs` 64,
   `max_input_bytes` 8 MB); what is left out is journaled. The answer must cite only the given
   numbers (`claims::check`: a citation or reference entry outside them, or no citation at all,
   refuses the text) and have `sol.words` words; a refused text stays in the run root
   (`sol-answer.md`) and is never registered.
4. The terminal's text is registered as kind `sol`, never public. Its sentences become claims
   (`claims.rs`), each followed to the sentences of the sources it cites: `sourced` (a cited source
   holds a sentence with content-word Jaccard ≥ `min_support`, 0.2), `weak`, `uncited`. With
   `topic_corpus` the text is also imported into the paper's closed topic corpus (`sol-import`,
   `sol-claims`), where its claims are followed into the library.
5. **Stop rule:** an epoch whose terminal text adds no sourced claim earlier epochs lacked and is
   not longer than the best earlier one marks the document `saturated` (a record, not an error);
   later epochs skip it.

**Dossier** (`dossier`, rows `{id, doc, epoch, kind: original|mova|sol|site-old|web, source,
licence, licence_class, public_ok, sha256, words, created_unix, inputs, citations: {count,
unresolved}}`, files `<doc>/texts/<epoch>-<kind>-<n>.md`): `{"kind": "world", "executable",
"config"}` uses the dossier store of the global world's tool through its CLI (`dossier list|text|add
CONFIG …`; its originals may be compressed, so texts are read through it); `{"kind": "files",
"root"}` keeps this module's own files (`texts.jsonl` appended under `texts.lock`, each file written
once). Either way the same text of the same epoch and kind is registered once, the original is
registered as epoch 0 when missing, and generated texts are never `public_ok` (a store that marks
one public is an error).

**Neighbourhood** (`world`, the integration point with the global world, queried at the epoch
being written, so it sees the texts of earlier epochs): `{"kind": "world", "executable", "config",
"radius", "max_docs"}` runs the world tool's `world neighbourhood CONFIG DOC --radius R --max-docs N
--epoch E` (documents, distances, visible texts), `world related` (edge kinds of the direct
neighbours) and `world neighbourhood … --content` (every sentence of every visible text, rebuilt as
one text per id); the document itself comes back at distance 0, and its texts other than the
original are its own material. `topic_corpus` reads a paper's closed topic corpus (`graph.json`
order: references, similar works, same authors, citing works; library texts from its registry);
`fixture` reads `DIR/<doc>.json` (`{"documents": [{doc, title, relations, distance, licence,
public_ok, texts: [{id, kind, epoch, path}]}]}`); `none`. Without a distance-0 entry the
document's own texts come from the dossier. The driver does not build the world: before epoch N the world tool's
`world build --epoch N` must have read the texts of earlier epochs.

**Configuration** (unknown fields refused, absolute paths): `whale`, `run_root`, `epoch`,
`documents` (`{"from": "batches", "path", "batches": [N…], "whales": [labels]}` — the papers
batches assigned, in their order; `{"from": "catalog", …}` — the batch selection
`batch::candidates` over a pinned catalog, never alphabetical; `{"from": "list", "items"}`),
`max_documents` (5), `dossier`, `world`, `mova {words, gates, knowledge (a Whale state root whose
facts explain terms), min_relevance}`, optional `sol {terminal, prompt {path, sha256}, words,
stage_inputs, max_inputs, max_input_bytes, neighbour_texts, min_support}` and `topic_corpus`.

**State and resumption.** `run_root/epoch-N/journal.jsonl` (fsynced) and per document
`run_root/epoch-N/docs/<doc>/` (`status.json` written before every step, `draft.json`,
`gates.json`, `mova.md`, `sol-prompt.md`, `sol-inputs.json`, `sol-answer.md`, `claims.jsonl`);
`run_root/docs/<doc>/state.json` (per epoch: words, new sourced claims; `saturated_at`) and
`claims.keys`. A registered Mova text is not written again; `sol_asking` without an answer becomes
`uncertain` and is never re-sent. A terminal limit, timeout or failure stops the run (`sol_limit`,
`sol_timeout`, `sol_failed`; final for that document's epoch: the question may be pending in the
terminal); nothing is retried. `epoch.lock` allows one driver per run root; a live run holds the
terminal's `holder.lock` for the whole run, like a batch learner, so the two never share it.

Tests (`epoch::tests`, `claims::tests`, `dossier::tests`; fake terminal, fixture neighbourhood):
the length gate keeps a short text out and asks nothing (control: a low minimum registers and
asks); a citation to an unknown source is refused; a consultant text is never public (control: an
original is); a resumed epoch writes and asks nothing again; `sol_asking` becomes `uncertain`
without a question; a limit stops the run and is final; epoch 2 reads and cites epoch 1 and
saturates on the same answer (control: a longer answer does not); unknown fields are refused; two
runs write the same text; the world tool (a fake CLI) gives the dossier and the neighbourhood, and
Mova cites the neighbour and the document's own older description. Negative controls, red then
restored: accepting unknown citations, ignoring the document's own texts from the world,
re-asking after `sol_asking`, generated texts public, saturation off.

## Batches

`mova-whale batch-plan CONFIG` and `mova-whale batch CONFIG [--no-terminal]` (schema
`mova-shark-batch-v1`) run a pod of Whales over papers with rolling assignment: each Whale works
on its own paper, and as soon as that paper is final it takes the next one; no Whale waits for
another.

- `batches.jsonl` in `run_root` (append-only, fsynced): `opened` (assignments with each text's
  SHA-256 pinned at opening, deadline) and `closed` (`all_passed`, `exhausted` or `deadline`, with
  every learner's status and outcome, and the unpassed papers in `retry`). A paper is assigned at
  most once.
- Choice (deterministic, never alphabetical; `selection`): a pinned JSONL `catalog` (`id`,
  `text_path`, `title`, `licence`, optional `local_only_field`, `concepts`, `topics`, citation
  counts); open licence only (CC BY, CC0, public domain; no NC, ND, SA), a Markdown text, not
  local-only. Order: the `priority` ids, then papers our code cites (`<id>.md` in `cited_dir`), then
  (with `include_uncited`) papers of a core topic (`core_concepts`: syntax, UD, parsing,
  semantics, discourse, morphology, learner language, reading …), then general NLP
  (`nlp_concepts`); within each rank by citations inside our library (`library_citations_field`,
  `our_citations`), then overall (`citations_field`, `cited_by`); the id only breaks ties. Papers
  of no listed topic are not taken (`require_core`); a shared-task system description
  (`system_patterns` in the title) needs `min_citations_system` (50) citations unless our code
  cites it. A paper, new or a retry, needs `min_words` (6,000 by default) words of restatable
  prose — the sentences Mova reads after the content filter — and a healthy extraction
  (`extraction`: spacing residue ≤ 2 per 1,000 words, unknown words ≤ 12 %, truncated paragraphs
  ≤ 30 %, words before the first section heading ≤ 40 %); otherwise a `refused` event (reason
  `short` or `extraction`, its measures, «re-extract») is recorded once and the paper is passed
  over.
- With `pod`, `stall_rounds`, `batched`, `review` and `topic_corpus`, the learners' configurations
  carry the same settings; with `topic_corpus` (`build_on_assign`) the plan builds each assigned
  paper's topic corpus and small world (`topic_corpus` events; a failure is recorded, the learner
  then learns without them).
- `batch-NNN/<whale>.learn.json`: the learner configuration (the batch's deadline; `import` from
  the Whale's slot).
- A slot (`whales[]`: `whale`, `state_root`, `terminal`, `import`, `formerly`) may list the
  learner's earlier labels in `formerly`: an open batch, a retry or knowledge recorded under such a
  label belongs to the slot, so a deployment can rename its learners without losing their state.
  The learner's knowledge is kept under `state_root`, not under its label.
- Rolling assignment (`plan_at`). An assignment is final (`final_reason`) when its paper passed,
  was asked out (`nothing_to_ask`, `round_limit`, `stalled`) in this assignment or under the
  current writer, or its deadline (`hours`, 12, from the assignment; the learner's first round
  follows at once) has come. Its batch then closes (`all_passed`, `exhausted` or `deadline`) and
  the Whale is free. A free Whale, in slot order, takes its next paper at once: its own retry
  first (below), else the next eligible paper of the selection that no Whale holds or has had.
  Each such assignment is a batch of its own (`opened` with one assignment, `"rolling": true`);
  a batch opened before with several assignments closes when all of them are final, while its
  finished Whales already work on new papers. A Whale holds at most one unfinished assignment.
- A learner that ended unfinished — a usage limit, a timeout, a terminal failure (for example a
  provider at capacity), an error — keeps its paper and continues it with new questions only
  `wait_minutes` (30) after it ended, until that paper's deadline; then the paper is given up
  (`deadline`, recorded in `retry` with its last reason) and the Whale moves on. A usage limit
  pauses the whole pod (one subscription): `wait_minutes` from the latest `limit` of an open
  assignment no paper is assigned and no learner starts. A learner stopped by `STOP` (`stop`)
  continues at once in the next runner.
- Every unpassed paper is recorded in its `closed` event's `retry` list with its learner, words
  and the writer version of its last round (`compose::WRITER`); when that learner is next free it
  takes the paper again first (its knowledge and asked questions stay; the paper is reopened with
  the new deadline), once the writer version differs. Under the same writer it is not retried.
- Journals: `whales/<whale>.jsonl` per Whale (`assigned`, `started`, `ended` with the reason,
  `closed` with the outcome); the learner's journal per paper under its state root.
- `batch` is the one runner of a run root: it holds `plan.lock` (a second runner or a `batch-plan`
  is refused), starts each Whale's learner in its own thread with its own terminal when it may
  start, and plans again whenever a learner ends (otherwise every 10 seconds; the catalog is not
  scanned again while no paper is left). `STOP` in the run root: no learner starts, running
  learners stop before their next round (`stop`, not final), and the runner returns once none
  runs. A restarted runner continues from `batches.jsonl`, the Whales' journals and the learners'
  states; nothing asked is asked again. It ends when no paper is left and no Whale holds one.
  `--no-terminal` makes one dry pass (every open assignment's learner up to its first question).
  After each learner run the runner writes its batch's
  `candidates/batch-NNN-school.jsonl` (questions for the papers school `courses/papers/` of the school data, `$MOVA_DATA/school`
  with Mova's own answer) and `candidates/batch-NNN-dictionary.jsonl` (terms with Mova's
  explanation and the paper's own context sentence) — only from open-licensed papers, never a
  terminal answer, every record `review_required`.
- Readers of `batches.jsonl` (the texts export of `whale-world`, epochs with
  `{"from": "batches"}`, the Owls' report) read `opened` events and their `assignments` as before;
  several batches can be open at once.

## Specification contract (`spec`, schema `mova-shark-spec-v1`)

A run configuration must contain `"spec": {"path": ABSOLUTE, "sha256": HEX}`. There is no
built-in specification. The specification file is JSON with unknown fields rejected:

- `schema`: must be `mova-shark-spec-v1`.
- `id`: a label naming the specification (1–64 ASCII letters, digits, `-` or `_`).
- `prompt_template {path, sha256}`: the path may be relative to the specification file. The
  template must contain `{{METADATA_JSON}}` and `{{SOURCE_TEXT}}` exactly once each. Slots
  are located in the template, never in substituted text.
- `prompt_max_bytes` (default 64000) and `context_max_bytes` (default 48000).
- `prompt_metadata_fields`: the metadata fields shown to the teacher. Strings are cut to
  1000 bytes.
- `kinds.<kind>` sets, for each kind of source:
  - `formats`: item format → adapter (`text` or `epub`);
  - `metadata`: `inline` (item fields) or `export` (pinned JSONL rows keyed by `id`);
  - `export_path_field`: optional;
  - `binding {prefix, suffix}`: optional; requires a normal relative `binding` on each item;
  - `admission`: optional licence rule (`licence_field`, `admitted`, `ignored_suffixes`,
    `prefix_rewrites`, `require_true`, `require_false`). Names are matched exactly after
    normalization (lowercase, unified dashes, single spaces);
  - `withhold_teacher_when_local_only`.
- `output`: the output contract.
  - `max_reply_bytes`.
  - `verbatim_run_tokens`: 0 disables the check, otherwise 2..256 tokens.
  - `length {pointer, unit: words|unicode_scalars, min, max, profile, forbid, project_oversized}`.
  - `fields`: a list of `equals`, `text`, `string_list` and `object_list` rules. An
    `object_list` rule can also set `text_fields`, `string_list_fields`, `enum_fields`,
    `distinct_by` and `must_contain`.
- `originality {enabled, non_narrative_fields, thresholds}`. The thresholds are
  `exact_run_tokens`, `aggregate_shingle_tokens`, `near_gram_tokens`, `near_window_tokens`,
  `near_percent`, `aggregate_min_tokens` and `aggregate_percent`. Each has a default, and
  each is bounded.
- `bundle`: the bundle layout. It sets `store_prefix`, `manifest_kind`, `manifest_status`
  and `id_prefix`. Artifacts come from `text_artifacts` and `list_artifacts`, each
  `{path, pointer}`, and from `json_artifacts`, each `{path, fields, constants}`.
  `status_rewrites` downgrades teacher-claimed statuses and keeps the original claim.
  `metadata_artifact` and `metadata_fields` set the metadata file. `manifest_extra` cannot
  override a generic manifest key.

A word is a whitespace-delimited token containing at least one Unicode alphanumeric
character. With `project_oversized`, a reply above `max` is first cut to its last whole paragraph,
else sentence, whose count is within bounds; without such a boundary, or below `min`, it is a
length-gate quarantine. A changed length rule needs a new `profile` identifier. Old editions keep their
receipts and are never reinterpreted.

## Run configuration (`whale`, `whale-prepare`, `whale-status`, `whale-project-lengths`)

Required fields:

- `run_root` and `store`: absolute paths. The store must be outside Git.
- `spec`: the pin described above.
- `teacher {model, effort}`: no model is built in. The model must pass
  `mova_teacher::validate_model`. Effort is one of low, medium, high or xhigh.
- `items`: a list of `{kind, id, path, sha256, format}`, each with optional `binding`,
  `title`, `licence`, `local_only` and `metadata`. The list must fit `max_items`
  (default 64, maximum 256).

Optional fields:

- `workers`: 1..8 workers.
- `metadata_export {path, sha256}`.
- `originality`: corpus keys only. These are `comparison_roots`, `comparison_extensions`,
  `cache_path` and the `max_*` bounds.
- `storage_floor_bytes`: default 10 GB.

The run identity hashes the configuration together with the specification and template
hashes. A changed specification, template, item or teacher therefore needs a new run root.
A changed template or specification after preparation is refused before any call.

## Swarm configuration (`whale-swarm`)

Required fields:

- `run_root`: a private 0700 directory.
- `executable`: the frozen `mova-whale` binary, pinned by hash at start.
- `workers`: 2..8.
- `lanes`: a list of `{name, share, role: survey|agent, child_config, payload_overrides}`.
  The shares must sum to one.

Optional fields are `estimated_job_ms`, `poll_seconds`, `discovery_poll_seconds`,
`discovery_limit`, `pending_per_worker` and `storage_floor_bytes`.

The default discovery reads `candidates {path, sha256}`: a pinned JSONL queue of
`{key, lane, payload}` rows, consumed from a durable cursor. An embedding binary can supply
another `Discovery` through `swarm::run_with`.

Scheduling picks the lane with the largest deficit between its target share and its
effective worker time, counted in milliseconds. In-flight jobs and unmeasured uncertain jobs
are charged the estimate. Ties keep the configured lane order. With two lanes this is exactly
the rule "choose the second lane if its time is below share × total". This is a long-run
target, not an instantaneous guarantee.

The two roles launch different children:

- A `survey` lane prepares a one-item run from `child_config`, the payload `item` and the
  admitted overrides, then runs `whale CONFIG --once`.
- An `agent` lane merges the payload into `child_config` and runs `whale-research`. Its
  branch is `branch_prefix` + key. When `repository` is configured, the payload is rebound
  to the current clean HEAD only while every pinned file still matches. Otherwise the job
  is marked superseded and no call is made.

## Agent configuration (`whale-research`)

Required fields:

- `run_root`, `repository`, `base_commit` (40 hex digits), `worktree`, `branch` and
  `branch_prefix`;
- `codex`, `model` and `effort`;
- `prompt_template {path, sha256}`: the template contains every slot of `agent::SLOTS`
  exactly once;
- `source_bindings`;
- `allowed_write_dirs`;
- `protected_suffixes`.

Optional fields are `reference_bindings`, `scope_bindings`, `scope_manifest_equals`,
`sidecar_suffixes`, `allowed_write_files` together with `write_file_rule`, `staged_guard`,
`source_coverage`, `reference_metadata_only`, `reference_gap`, `task`, `timeout_seconds`,
`max_retained_worktrees`, `retain_successful_worktree`, `target_dir` and
`storage_floor_bytes`.

The admission rules:

- A write directory must be a pinned source's sidecar (source path + suffix) or the
  directory of a pinned scope manifest.
- A write file must be pinned and match `write_file_rule`.
- After the session, every tracked file with a protected suffix outside the write scopes
  must still hash to its base blob. Index flags are not trusted.
- The candidate passes the configured staged guard and is committed on its own branch. It
  is never merged or pushed.

## Crab

`mova_whale::WhaleRoles` implements `mova_crab::angel::ExternalRoleHandler`. Construct it
with `WhaleRoles::new()`, or get an `Arc` with `WhaleRoles::handler()`, and install it with
`mova_crab::angel::serve_with_roles(config, WhaleRoles::handler())`.

- `status` and `start` delegate to the swarm when Crab has a swarm configuration, and to the
  pool otherwise.
- `marker` (pause, resume, stop) handles the pool. Crab handles swarm markers itself.

The pool reads Crab's `run_root`, `whale_config` and `whale_executable`. The swarm start
reads `whale_swarm_config` and `whale_swarm_executable`.

## Compatibility identifiers

Existing configurations, run roots and pinned specifications may carry `shark` names, so the engine accepts them as
aliases of the `whale` ones; new files are written with the `whale` names.

- subcommands `shark`, `shark-prepare`, `shark-status`, `shark-project-lengths`, `shark-swarm`,
  `shark-swarm-status`, `shark-discover`, `shark-research`, `shark-research-status` (same as the `whale*` ones);
- JSON fields `shark` (learner label in paper, learn and batch-slot configurations, knowledge items, batch events and
  retry records) and `sharks` (the batch configuration's slot list);
- Crab configuration keys `shark_config`, `shark_executable`, `shark_swarm_config`, `shark_swarm_executable`, the
  wire operations `start_sharks` and `shark_status`, the role `sharks`, and the deputy capability handler name `shark`.

The format identifiers are fixed: the schemas `mova-shark-spec-v1`, `mova-shark-run-v1`, `mova-shark-paper-v1`,
`mova-shark-learn-v1`, `mova-shark-batch-v1` and `mova-shark-pool-v1`, the pool owner marker prefix
`mova-shark-pool:`, its `shark_root` field and the Crab pool directory `shark-pools`. They are format names, not
display names.

## Invariants

- One reservation per task before the call. A failed or unknown call is receipted
  `uncertain` and pauses the worker. An unreceipted reservation is never resubmitted.
- Invalid replies are terminal quarantines. There is no reprompt.
- Bundles are written to a staging directory and renamed into an immutable version
  directory. Existing versions are never overwritten.
- PAUSE and STOP prevent new admission. Active work may finish. STOP is terminal for a pool.
- Launch intents are observed, never replayed. There is no automatic restart.

## Checks

`cargo test`: the unit tests above plus the service tests (`animal.rs`: priority
orders waiting work and never preempts; duplicates, failed sources and unknown sources queue
nothing; PAUSE blocks admission; a started request without a receipt is recorded uncertain and not
relaunched after a restart; a second service on the run root is refused), the realizer tests
(`answer.rs`) and the length gate on a fixture specification with 4,000..5,000-word bounds
(3,999 and 5,001 refused, 4,000 and 5,000 admitted, 5,002 with a sentence end
projected to 5,000, 5,001 without one refused). `MOVA_WHALE_SPEC_CHECK=PATH[:SHA256]` also runs
that gate against a deployment specification, and `MOVA_WHALE_ANIMAL_CHECK=CONFIG:DEPUTY` validates
a service configuration and its catalog pin; both report a skip when unset.

English (`tests/english.rs`, with `MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin`; a skip
without it): 18 prompts map to the expected capabilities and arguments and 10 refusals fail at the
expected stage with a review candidate. The served test runs the real `mova-whale` child for each
request against a fake Codex executable and a fixture specification with 4,000..5,000-word
bounds: a status question is answered within 1.5 s while a description runs; refused prompts
queue nothing; an urgent request starts after the running one and before an earlier normal one;
4,500 words completes, 3,999 words and 5,001 words without a sentence end are quarantined; count,
status, show, pause, resume and a repeated request id are answered as expected.

Negative controls, each turned red and then restored: arrival-only ordering (priority
test red); recovery ignoring `started` lines, so an attempted request would wait again (restart
test red); length bounds widened by one word (bounds test red); dropping "not"/"if" from the
deputy's scope check (planner and served tests red: "Do not describe …" queued work).

Terminal adapter (`tests/terminal_adapter.rs`, tmux required; a fake bash LLM in a
real tmux server on a private socket, never a model): the adapter returns the answer file; a
usage-limit screen returns `Limit` within seconds and a following question is refused as a replay;
a full Whale run asks through its own terminal and keeps the answer out of the description.
Negative control: ignoring the limit text makes the limit test wait out the timeout (red).

Paper reading: a fixture paper with a fake terminal: the draft is journaled before
the first question; the answer never reaches the description; a usage limit ends consulting;
references are not read; teaching records carry licence admissibility; configs sharing a terminal
are refused. Negative controls, red then restored: appending answers to the description; ignoring
the limit (asking on); a lease that does not lock.

## Limits

- Gates are structural and lexical. They do not establish scientific correctness,
  semantic originality or translation independence.
- Traceability of `paper` skeletons is complete by construction (they keep the source's own
  words); it shows binding, not understanding.
- A clear originality receipt covers only the configured local corpus.
- EPUB extraction ignores layout, images and MathML semantics.
- Tests use fake teachers and fake executables. They never call a model.
- Process tests serialize on a test-only mutex, because a child forked by a concurrent test
  briefly inherits every open descriptor.
- No live run is part of this crate's checks.
- The English service understands only the configured vocabulary through the deputy's bounded
  skeleton reader; unusual phrasings are refused, not guessed. Answers are English only.
- The service describes catalog sources only; it does not fetch URLs (Sea Spider does).
- An opening quoted by `show` is the description's first sentences, not a summary written by Mova.
- `paper` writes skeletons (telegraphic: «We classify them according new based framework»).
  `learn` writes whole clauses; neither mode guarantees a configured description length. Its restatements are syntactic (splitting,
  perspective, moving and leaving out modifiers), not abstraction; a clause can lose an obligatory
  oblique («a new rule-based framework relies solely») or keep a misparse. Sentences Mova parses
  with a wrong root are not restated; they are the material of the papers school, not a defect to
  pad over.
- Retained facts are evidence with provenance, not truth; their objects can be vague («I
  understand «EWT» as a collection.»). Sentence readings learned from an answer are kept only in
  the source's own words and are often refused by the overlap gate.
- Epoch texts: the neighbourhood part restates other works' sentences clause by clause (syntax, not
  abstraction); extraction noise of old PDFs can pass the grammar gate. A document whose reading
  has no readable part has no key terms and gets no neighbourhood text. Claim support is lexical
  overlap with a cited source, not entailment. A refused or failed terminal step is final for that
  document's epoch in its run root.
