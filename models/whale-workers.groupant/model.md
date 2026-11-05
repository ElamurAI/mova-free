# whale-workers

## Descriptant

Composition of the public `mova-whale` crate. A deployment supplies a pinned specification (prompt template, kinds, output contract, gate parameters, bundle layout), teacher identity, items or a candidate queue and paths; the crate supplies the mechanics.

## Forward

util pins and leases -> spec loads the pinned contract -> source extracts bounded context -> whale prepares tasks, reserves, calls the teacher once and admits the reply through contract and originality into an immutable bundle -> pool launches prepared workers once -> swarm schedules survey and agent lanes by measured time from a pinned queue -> agent runs bounded worktree sessions -> roles exposes pool/swarm to Crab -> main is the executable all children run.

## Invariant

Every external call is preceded by a durable reservation and followed by a terminal receipt or an uncertain state that blocks replay; every published artifact is immutable and hash-listed; every concrete choice is pinned by hash in configuration.

## Cost

Dominated by teacher/agent latency; local gates are linear in reply and corpus size; schedulers poll SQLite O(lanes).

## Composition

Leaves are file groupants under `tools/whale/src/*.rs.groupant`. Correct leaves do not prove the wiring: the interaction checks are the pool/swarm fixture tests that launch the real child entry points through fake executables, and whale tests that cross spec, contract, originality and publish.

## behavior_complement

Domain: all configured runs. The system does not: call a teacher without a pinned specification and explicit model; retry or reprompt; overwrite bundles; merge or push agent candidates; restart launched cohorts; admit writes outside configured scopes; claim correctness from structural gates. Counterexample: a forged template hash stops every entry point before any reservation.

## reverse_reconstruction

From a bundle manifest the task, reply, spec and template hashes are recoverable and the job directory retains the reply; from swarm status only per-lane counts and charged time are recoverable, admitting many schedules. Evidence status: source-derived with finite checks.

## History

### 2026-11-04 · document

#### Epoch 2026-11-04 (Sea Spider live: singleton and callers)

New child mova-whale-seaspider-client: optional seaspider_socket in the animal config; ask_sea_spider asks Sea Spider as caller whale. Off unless configured.

#### Epoch 2026-11-04

New children `mova-whale-pod` (pod-shared term knowledge), `mova-whale-review` (review hints from the Whale's own terminal) and `mova-whale-topic` (the topic-corpus binary as a child process). The pod adds a cross-Whale interaction: learners of one batch share terms through `pod/` (claims before asking, entries after answers).

### 2026-11-05 · document

#### Epoch 2026-11-05 (epochs of texts)

Children `mova-whale-claims`, `mova-whale-dossier`, `mova-whale-epoch`, `mova-whale-epoch-tests`: the epoch driver composes compose_epoch (writer), dossier (texts and neighbourhood), terminal (one question per document and epoch, holder.lock), claims (citations to given sources) and batch::candidates (selection). Interaction invariant: a document's epoch moves new → mova → sol_asking → done|saturated|sol_rejected, or stops at mova_short|mova_gates|uncertain|sol_<error>; no transition re-sends a question.
