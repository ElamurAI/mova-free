# Crab's deputy planner (`mova-octopus`)

The Octopus itself is a large language model (LLM); this crate is the Rust planner that
lets its deputy, Crab, understand the Octopus's English. The overview of how the animals talk is in
[docs/animals.md](../../docs/animals.md#crab-and-its-deputy-planner); this README holds the protocol, configuration and
handler detail.

**Crab is the deputy.** The `crab` binary of this crate is the public Crab angel with this planner
installed as its understanding:

```text
crab serve CRAB_CONFIG DEPUTY_CONFIG [COMMAND_CORPUS]
crab ask SOCKET REQUEST_ID [urgent] < any-text
crab exam DEPUTY_CONFIG CORPUS...
crab review-queue OBSERVED [REVIEWED...]
crab promote OBSERVED DECISIONS REVIEWED
crab exercises DEPUTY_CONFIG NEW_OUT REVIEWED...
crab candidates REVIEW_QUEUE
crab audit CORPUS
```

The caller supplies only arbitrary text and an optional priority; Crab decides what to execute, refuses the rest with
a review candidate, and never lets a prompt interrupt running work or acquire control operations. The standalone
`mova-octopus serve|angel` transport below remains available but is not deployed.

The deputy reads ordinary, unmarked English from stdin. It invokes Mova's existing English annotator and world tree reader, records their real UD nodes and deterministic question frame, and proposes a bounded concrete plan against externally configured capabilities. No task label, command grammar, JSON envelope or large model is required in the prompt.

```sh
cargo build --manifest-path tools/octopus/Cargo.toml --locked --offline -j2
mova-octopus plan "$OCTOPUS_CONFIG" development-001 < request.txt
mova-octopus run "$OCTOPUS_CONFIG" invocation-001 < request.txt
mova-octopus angel "$OCTOPUS_CONFIG" "$PRIVATE_DIR/deputy.sock"     # permanent: restarts the server after failures
mova-octopus serve "$OCTOPUS_CONFIG" "$PRIVATE_DIR/deputy.sock"     # one server run, no restarts
mova-octopus ask "$PRIVATE_DIR/deputy.sock" run invocation-002 < request.txt
```

`serve` listens on a Unix socket inside an owned 0700 directory (refused otherwise), creates it 0600 and answers
only peers with the owner's uid. A request is one NDJSON line of at most 16384 bytes:
`{"schema":1,"id":ID,"mode":"plan"|"run","prompt":TEXT,"priority":"urgent"|"normal","from":CALLER}` (priority and
`from` optional; `from` is a caller name such as `crab` or `whale`, 1–64 lowercase letters, digits and `-`). The prompt
is arbitrary ordinary text. `serve_requests`/`angel_requests` hand the handler the whole request, including the declared
caller and the connecting pid (SO_PEERCRED), so an animal can keep an allowlist and journal who asked; the caller
name is a declaration by a process of the owner, not authentication. Requests are answered one at a time through the same `Deputy::run`; waiting requests are
ordered urgent-first, then by arrival, and a running request is never interrupted, so a new prompt cannot destroy
unfinished earlier work. `STOP` in the socket directory ends the server after the current answer. `angel` holds a lease, restarts the server after a load failure, error or panic with a 1–60 s doubling backoff, records
`angel-status.json`, and stops only on `STOP`; a panicking handler fails only its own request. A prompt the deputy
cannot understand is an explicit refusal with a review candidate: material for teaching Mova, not for a workaround.

`Deputy::load` refuses to start while `WORLD_NO_REPAIR` is set: the native repair override is not admitted in this profile. Configuration pins an existing syntax model by SHA256, supplies capability IDs, entity aliases, lexical intent rules, installed handler names and trusted handler parameters. It also supplies ledger and review queue directories. Concrete assignments, deployment paths, sockets, credentials and private logs belong outside public Mova. The public engine has no fixed operator identity or task list.

Each capability contains `handler`, `parameters` and `intent`. The intent has `request_predicates`, `observations`, `entities`, and optional `subordinate_predicates` lexical lists. These are developer configuration over native analyses, not a language that the user must write. The planner records matched observations/entities and zero-based native request-head and observation-node witnesses. Native UD ancestry must connect a request head with an observation. This bounded skeleton resolver is experimental and does not establish general task understanding.

Public installed adapters currently provide bounded read-only Crab observations (`crab_observe`, trusted socket and status/read_reports operation) and trusted regular-log diagnosis (`log_tail`, trusted path). Rust applications can register further configured `CapabilityHandler` implementations through `Deputy::with_handler`; tasks need not rewrite the upstream planner. A new handler needs its own admission, semantics and tests. The existing executable has these two adapters; it does not execute arbitrary shell, write code, annotate data or train models.

Asking another animal (`relay.rs`): the installed handler `animal_ask` relays the quoted English of a prompt to
another animal's private socket and returns that animal's English answer. Its parameters are exactly
`{"animal": NAME, "socket": ABSOLUTE_PATH, "caller": CALLER}` and its one slot is `task` (kind `text`, 1..1),
checked when the configuration loads; the socket is never taken from a prompt. With entity `spider`,
"Ask Sea Spider: «Is the queue empty?»" sends "Is the queue empty?" as caller `crab` (request id = the invocation id,
so a repeated request returns the recorded answer) and waits at most 30 s. The same client path is the library
function `relay::ask_animal(socket, caller, id, english, urgent)` for animals that hold a configured socket (the
Whales). Tests cover configuration refusals and a round trip with a fake animal; removing the socket check turns the
configuration test red.

The `crab` binary (`src/bin/crab.rs`) embeds the deputy in Crab and installs two in-process handlers: `crab_observe` (status/read_reports) and `crab_control`. A `crab_control` capability has parameters `{"operation": "worker_start"|"worker_stop"|"worker_restart"|"worker_status", "worker": NAME}`, checked when the configuration loads. The prompt only selects a configured capability; operation and worker come from configuration, and Crab checks the worker name against its own list. Vocabulary is configuration too: for example `eel_start` (request predicates and observations `start`, `launch`; entity `eel`), `eel_restart` (`restart`), `eel_stop` (`stop`) and `eel_status` (observation predicates plus `status`, `run`, `work`). With such a configuration "Start Eel.", "Restart Eel, please.", "Is Eel running?" and "Stop Eel." map to the four operations, while negation, conditionals and unconfigured predicates ("Kill Eel.", "Delete Eel.") are still refused. Outside Crab the handler is not installed and such a plan fails closed.

The reply to `ask` is compacted to a 48 KiB budget inside Crab's 64 KiB response limit: it keeps the prompt hash, plan, failure, followups and results, but when it is too large each result `output` is replaced by `{"omitted": …}` (its `output_sha256` stays), and if it is still too large `followups` becomes `{"omitted": true}`. The full report stays in the deputy ledger (run mode).

## The deputy's question frame

Every observation carries `deputy_frame` (`frame.rs`) beside the world reader's `native_question_frame`: the mood
(`imperative`, `modal_request`, `polar`, `wh`, `declarative`), the subject node and the verb node, read from the native
parse. "Is Crab running normally?" is `polar | subj «crab» | verb run`; "Could you start Eel?" is
`modal_request | subj «you» | verb start` and is read as the imperative. When a present polar question is parsed with
the copula's subject as the root ("Is the documenter running?": root «documenter», «running» acl; "Is Eel busy?";
"Is the Nautilus running?": compound + -ing root), the frame is `repaired`: the subject noun is not a predicate and
the recovered verb or adjective is the clause head the resolver checks. Past questions ("Was Crab running?") are
never repaired and stay refused. A frame constrains the request head; it is not a parse-quality verdict.

An ambiguity refusal names its candidates (`failure.candidates`, also listed in the reason): "Is the Dolphin
running?" → `"dolphin" names 5 configured entities; say which one; candidates: claude, copilot, documenter, opus, sol`.
A group alias may cover any set of same-kind entities, for example `"nautilus": ["pompilius", "argonaut", …]`.

## Review candidates

The review queue keeps one file per prompt and failure stage, `<dedupe_key>.json`. The first candidate is kept as
written; `reproduced_by` lists every build (candidate id, source, model, configuration and native-source hashes,
reason, times seen) that refused the same prompt at the same stage. Report followups keep their per-build
`candidate_id`. Older one-file-per-build queues are grouped read-only by `crab candidates QUEUE`. An understood plan
whose execution failed is not a review candidate.

## Command school corpus

`commands.rs` keeps the corpus of the command school (`$MOVA_DATA/school/courses/commands`, the school data). With the optional `COMMAND_CORPUS` argument, `crab serve` appends every prompt it reads to that JSONL file, anonymized (absolute paths, e-mail addresses, URLs and long hexadecimal or digit runs become `<path>`, `<email>`, `<url>`, `<id>`), together with Crab's observed reading. A line may also carry a reviewed `expected` reading (a plan or a refusal). `crab exam DEPUTY_CONFIG CORPUS` replays every reviewed line through the planner in plan-only mode (nothing is executed) and prints passed/examined counts by kind, the unreviewed count and the failures. `crab audit CORPUS` fails if any line would still change under anonymization.

Observed readings are `plan`, `refuse` or `executed_failed` (an understood plan whose execution failed, such as a
worker start that met a conflict); an execution failure is never counted as a misunderstanding and cannot be a
reviewed expectation.

The loop from observation to exam: `crab review-queue OBSERVED REVIEWED…` lists the observed texts no reviewed corpus
answers yet, with their readings and arrival counts. The reviewer writes decisions as JSONL
(`{"text", "expected", "privacy_review": "passed", "note"}`), and `crab promote OBSERVED DECISIONS REVIEWED` appends
the admitted ones (`origin: reviewed-observed`) only if the text was really observed, passed privacy review, is
anonymized, expects a plan or a refusal, and is not reviewed yet; it prints `training_authorized: false` — the exam and
the school use these entries, training does not. `crab exercises DEPUTY_CONFIG NEW_OUT REVIEWED…` derives
command-school exercises from reviewed single-capability plans: the determiner variant of the name ("Is Eel running?"
↔ "Is the Eel running?", "Start the Sea Turtle." ↔ "Start Sea Turtle.") and the same sentence about every other
entity whose capability has the same handler and parameters except the target ("Is the Sea Turtle running?" →
`turtle-status`); a name with a head noun ("the Sol Dolphin") is replaced only within its configured group. Each
exercise records `derived_from`; `crab exam` takes several corpora and replays them all.

For an illustrative configured deployment, “Have a look at Dolphin’s recent logs and ask Crab whether everything is running normally.” becomes two named capability calls. One calls the existing public Crab Unix protocol; the other examines a bounded trusted log tail. Alias vocabulary and transport destinations are configuration, not role identity in the generic planner.

The complete prompt is retained with its hash, exact model hash, implementation/dependency hashes, native syntax observations, typed plan and bound outputs. Native feature bitsets are decimal strings, preserving u128 values that cannot safely pass through serde_json::Value as numeric u64. The question reader's frame is separate from trained pragmatics: no trained pragmatic engine is invoked, and that gap is explicit. Structural UD admission is not a grammar-quality verdict.

An unsupported predicate, unadmitted tree, unresolved entity, negation, conditional, quotation or ambiguous competing capability prevents the entire execution. A review-only candidate identifies the failure stage without blaming the syntax parser for a valid unsupported action. Candidates are highest priority but are not gold: privacy, held-out admission and semantic review must precede any Fish/Eel or skeleton follow-up. No automatic database insertion or GPU learning occurs.

Candidate IDs hash the serialized array of source, model, prompt and configuration hashes, native source-hash map, repair profile and failure. Repair metadata is separate from source hashes. Repeated identical candidates retain identical bytes; historical artifacts are never silently rebound.

A run reserves its invocation ID durably before adapter calls. A completed repeat returns the previous receipt, not a fresh observation. A conflicting ID is refused. A reservation lacking a receipt remains uncertain and is never automatically replayed. Use a fresh invocation ID for a new observation; preserve the old receipt.

Log diagnosis reads at most 65536 bytes, returns at most 32 lexical error candidates and preserves byte offsets and a tail hash. It establishes no causal explanation or current health. Empty logs are no health evidence. Crab transport success likewise does not prove child health.

The tests cover positive prompts, refusal controls, u128 trace serialization, mocked Unix IPC, caching/conflict/crash uncertainty and bounded logging. Set `MOVA_OCTOPUS_TEST_MODEL` to an existing native model for the native test; without it the integration test explicitly reports its skip.

Source groupants and a three-node registry fragment are included. The integrating checkout adds the fragment to its authoritative registry and ancestor bindings.

For long logs, the byte window may start inside a UTF-8 character. The diagnostic
excludes its first partial line before decoding, preserves the original window
hash and reports `skipped_prefix_bytes` and `decoded_byte_start`. Invalid UTF-8
inside the admitted suffix still fails. Reads stop at the descriptor length
observed before reading; concurrent changes are not an atomic snapshot.

## Typed arguments

A capability may declare `slots`: `[{"name": "urls", "kinds": ["url"], "min": 1, "max": 16}]`,
kinds `url` (absolute http/https URL) and `host` (DNS name or IPv4 literal), kinds disjoint within a
capability, at most 16 arguments per prompt. Only when some capability declares slots
(`slots.rs`), every URL- or host-shaped word is validated strictly (http/https only, no user
information, a valid host, port 1–65535, printable ASCII; loopback, private, link-local and
reserved names such as `localhost`, `.local`, `.test` refused unless `slot_policy.allow_private_hosts`)
and replaced by a reserved proper-noun placeholder (`Zurla`, `Zhostb`, …) before sentence splitting
and parsing. An argument is bound only through the native parse: its placeholder must reach the
nearest request head of exactly one planned action through argument relations (obj, iobj, obl,
nmod, appos, conj, compound, flat, list, dep, nsubj), with no clause boundary (relative, adverbial,
complement clause, parataxis) and no exclusion dependent ("except", "instead", "besides", …) on the
way, and that action's capability must have a slot of its kind; slot counts must then fit. Otherwise
the whole prompt is refused with a review candidate at stage `argument_invalid`, `argument_unbound`,
`argument_ambiguous` or `argument_count`. Bound values appear in the plan as `arguments` with
`native_argument_nodes`. Configurations without slots read the text unchanged.

Two further optional intent fields: `addressee: true` makes the first entity alias the meant one
when no alias is said (a channel that reaches only one animal, such as Sea Spider), and
`native_lemma_observations` are matched by the native lemma only, never by the dictionary variants
of `world::ans::lemmas` (whose variants of «a» include «can»): for modals such as «can», «may».
Defaulted fields are not serialized, so configurations without them keep their hashes.

`observes: true` declares that a capability only reads state (a status, a count, a stored page, a
permission check). A question — a polar or wh frame, or any sentence that ends in a question mark
except a modal request ("Could you …?") — may select only such capabilities; an acting capability it
would otherwise select is withheld and, if nothing observing remains, the prompt is refused as
`pragmatic_ambiguity` ("Is Eel stopped?", "Did you stop Eel?", "Have the Whales resumed?" never
stop or resume anything; "Stop Eel." and "Could you stop Eel?" do). The default is acting, so an
undeclared capability refuses questions rather than acting on them. Loading refuses `observes` on the
deputy's own acting operations (`crab_control` start/stop/restart, `crab_consult` enqueue,
`animal_ask`); each animal's `check_capabilities` does the same for its own (Sea Spider fetch, Whale
describe/pause/resume, Nautilus draft, Turtle check/gap). A polar frame is read through its subject
whether or not the parse needed the frame repair: an observed subject noun admits the predicate ("Is
the queue empty?" with «empty» as root).

`realize.rs` is a small deterministic English realizer for answers (counted nouns, is/are,
was/were, has/have, past forms and participles from `en::morph::Morph`, lists, sentences, «I could
not … because …»). `socket::serve_prioritized` and `angel_prioritized` pass the request's priority
to the handler so an animal can order the work it queues; the transport order is unchanged.

A slot of kind `id` takes a catalog identifier: a word of lowercase ASCII letters and digits in at
least three hyphen-joined parts, with at least one letter and one digit (`smith-2024-tree-parsing`),
at most 128 bytes, masked as `Zsluga`, `Zslugb`, …. Identifiers are masked only when some capability
declares an `id` slot, so other configurations read such words as text. A copular wh-question whose
root is "what"/"which" ("What is the status of X?") is read through its nominal subject: the subject
noun must be a configured observation, as a root predicate would. An imperative that the parser
read as one noun phrase ("Show Crab's recent reports." → «show» a compound noun, «reports» the root) is
repaired by the deputy frame (`frame::repair_imperative`): with no verb, auxiliary or subject in the
sentence, a first word that is a bare singular base-form noun, a configured request predicate and the
head of no determiner, possessor or modifier becomes the imperative verb and the old root its object.
The observation keeps the native parse; the frame text says `imperative repaired`.

A slot of kind `text` takes a quoted span — «…», “…” or "…" — masked as a whole (`Ztexta`, …) before
anything else, so its words are never read as instructions and URLs inside it stay text; an unclosed or
nested quote, an empty span or one over 6000 bytes refuses the prompt (`argument_invalid`). An argument
of any kind introduced by a colon after the request ("Ask the Opus Dolphin: «…»", "Survey this paper:
paper-2023-gamma.") is that request's argument however the parser attached it (the 2026-11-05 en.bin
reads both as `vocative`): the word before the placeholder is a colon, only punctuation follows it,
nothing excluding hangs from it, and the nearest request head before the colon takes it, if its
capability has a slot of that kind (`slots::colon_head`; the
parse is tried first). A span before the request, a colon with no request before it, or words after
the span stay with the parse alone. The
`crab_consult` handler (installed by `crab`) hands such a bound `task` to Crab's consult queue for the
Dolphin named in its trusted parameters (`{"operation": "enqueue"|"state", "dolphin": NAME}`, or
`{"operation": "list"}`); see [Crab's Dolphins](../crab/README.md#dolphins). `group_aliases` maps a
collective word to its members' aliases (`{"dolphin": ["opus", "claude", "copilot", "documenter"]}`): the
singular word without a member's alias is refused as `pragmatic_ambiguity` unless a capability names that
word itself as its entity; an intent with `collective: true` binds its entity only in the plural ("Which
Dolphins are running?"). Questions with "which" are read like "what" questions.

Unit tests cover URL/host validation, masking, slot rules and the realizer. Planner behaviour with slots is
exercised against the native model by `tools/seaspider/tests/english.rs`.

