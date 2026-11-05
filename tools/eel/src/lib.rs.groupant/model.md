# mova-eel-controller

## descriptant

### Public native Eel continuation

The controller preserves the previously private working mechanism as reusable
public source. A deployment supplies versioned annotation data through TrainingData;
public code controls idle observation, durable reservation/cooldown, native trainer
invocation, outcome evidence and guarded artifact retention. No concrete private
paths/config/DB task payloads belong to this crate. Existing binary adoption is
separate from source migration and finite fake-executable wiring tests.

## model

# Idle continuation model

## Forward approximant

With immutable operator config, one Eel controller and a durable counter, first recover a known terminal round or pause an uncertain one. Honor durable cooldown, storage bounds and PAUSE/STOP. Probe the common GPU lease nonblocking; only empty compute-app output and GPU0 utilization below the configured threshold support an idle observation. Release that probe lease before calling trainer::run(round_config,true), so the trainer owns its own inherited lease and cannot self-deadlock. A lease race may defer the round without training.

Each unique round root records its index, proposed parent-weight hash and config hash. The reused trainer freezes a DB view, runs actual agreement, continues for one epoch at learning rate0.00003, enforces finite CUDA/checkpoint/local regression gates and an explicit fixed EWT LAS floor, then conditionally installs. The child deadline is configurable10..20seconds (default20); whole-round I/O/validation and device teardown can take longer. This is cooperative yielding between rounds, not instantaneous preemption of active kernels.

A successful or deferred terminal summary is retained outside the round artifact directory, then the counter advances and yields for at least2seconds (default5). Counter recovery advances a terminal round without resubmission. Resource failure/uncertainty pauses Eel and preserves its evidence. A typed finite quality rejection is a completed rejected experiment: retain its candidate/summary, keep current weights and permit only a newly planned experiment after cooldown. Other annotation/DB processes are not stopped. Proposed parent hashes can differ from the actual trainer parent if another trainer wins the lease gap; both observations remain distinct.

## Invariant and cost

No teacher/API call originates here. A busy or unknown GPU observation cannot submit a new trainer round. A reserved round without a trustworthy terminal receipt cannot replay. Trainer failure preserves its ordinary manual inspection boundary, including a possible post-install receipt failure. The fixed first developer set and anchor are config-pinned; repeated development selection is biased, not generalization.

The controller admits only a10GB free-space floor and default50GB configured run-root cap. These concurrent scans are guards, not filesystem quotas. Retention prunes only marked owned installed/deferred/below-threshold/rejected terminal experiment artifacts beyond the most recent eight safe candidates. Failed/uncertain/foreign/symlinked directories and any path referenced anywhere in config are preserved; inability to establish safety prevents deletion and can eventually pause at the cap. Summaries remain. Raw DB and source references are never cleanup targets under these premises.

Probe work costs two bounded driver queries plus model-weight hashing. Capacity/retention checks scan filesystem entries; model continuation retains the trainer's cost. Repeated round setup and development evaluation are not claimed free, and no throughput/fairness theorem is established.

## Composition

Eel idle observation -> release probe -> existing trainer GPU lease -> immutable agreement/learning pipeline -> durable Eel receipt -> counter/cooldown -> bounded artifact retention. Eel does not atomically transact SQLite, weight installation and its summaries. Existing counters/config bytes and root ownership are trusted local evidence, not cryptographic identity. Parent integration owns launch/migration; helper mocks exercise this source without real GPU work.

## Deliberate exploration after rejection

Round i uses seed_i=base_seed+i with checked overflow and GP_SAMPLE=0.15 by default, explicitly bounded to [0.01,0.25] for idle slices. This revisits verified data with changed sampling/order/dropout; the exact developer and absolute-anchor gates remain unchanged. The sampling fraction is a work budget, not a numerical output-error guarantee. Reservation and terminal summary retain round/parent/data generation plus seed/fraction/epoch/rate lineage. Rejected experiment i cannot automatically replay; experiment i+1 has a new seed. Three consecutive rejections raise durable cooldown to at least30seconds, while subsequent observed-idle rounds remain allowed. Rejected terminal artifacts join the owned eight-candidate retention policy; historical FAILED or uncertain rounds stay excluded. Repeated selection on the same developer sets remains biased and cannot promise eventual improvement.

## Public engine boundary

The subject is now the public mova-eel library, with TrainingData supplying
versioned data and optional priority selection. No private store module is linked.
Its actual continuation path calls the public trainer module, which invokes Mova
native treeagree and gparse; Rust owns lifecycle, validation and durable effects.
The private deployment adapter supplies concrete database and policy. Legacy owner
identifiers, counter/config hashes, installer lock names and receipt fields retain
compatibility. Existing running binaries do not change when source is migrated.

Terminal summaries add typed evidence and optional diagnostic identity. Deferred
and below-threshold are no-training observations. Rejected/installed classifications
need actual-parent plus candidate/installed and generation SHA-256 fields; proposed
parent alone cannot reconstruct training. A status string or advancing counter
cannot substitute for outcome-specific evidence. Partial historical summaries
remain readable as incomplete; classification does not authenticate local files.

The trainer's optional priority admission runs only between slices; ongoing GPU
work is never interrupted. A deployment-admitted AwaitingReview item blocks new
ordinary training and returns a deferred summary, preserving the rejection streak.
Reviewed immutable exports outrank ordinary DB polling/export and follow the same
GPU/gates/install path. Queue authority, deterministic ranking, pins and terminal
acknowledgement are deployment responsibilities. A failed terminal acknowledgement
leaves evidence and propagates uncertainty; no raw prompt becomes training data.

Every controller status emission carries `kind` = `mova_eel` and `config_sha256`,
the lowercase SHA-256 of the serialized parsed controller configuration: the same
value bound once in `identity.json`, not raw/pretty file bytes and not a round
configuration. A read-only observer can therefore pin the run identity. The field
identifies a configuration, not process liveness, authorship or training health.
Status files written by older binaries lack both fields and remain explicit
identity mismatches; adoption needs a deliberately installed new binary, and a
PAUSE marker is never removed by this change.

## catalog

# Typed Eel obernants

All models bind the exact source manifest. These are partial source-derived relations with finite mock/native evidence, not formal proofs or percentage approximants.

## behavior_complement

Domain: cooperating Linux lease users, operator-owned config/root and normal execution traces. Observe trainer callback submission. Busy common lease, observed compute apps, excessive utilization, unknown driver output, durable cooldown, storage cap or uncertain prior round exclude a new callback. This is an inner exclusion model for these paths, not a complement of all physical GPU activity. Counterexample: utilization0 at time t cannot exclude a new foreground launch at t+1. Oracle: busy-app/utilization/flock controls observe zero callback calls. Evidence: mock driver plus real flock tests; no instantaneous-preemption claim.

## reverse_reconstruction

Domain: durable counters, round ownership/reservations and retained summaries. These reconstruct round indices, config hashes and proposed/actual parent lineage, not original gradients or pruned logs/weights. A terminal receipt can advance a stale counter without replay; a reservation alone has many compatible execution prefixes and must pause. Counterexample to treating process death as no work: weights may already have changed before receipt failure. Oracle: uncertain round refuses submission; completed round advances and yields. Evidence: finite controller tests; missing histories remain missing.

## lease_gap_partition

Type: error_partition over scheduling observations. Domain: idle-probe completion followed by trainer's independent nonblocking lease acquisition. Outcomes distinguish busy-before-probe, observed-idle then deferred, completed training, and failed/uncertain execution. Probe release is required to avoid acquiring the same lease through a second open description. Counterexample to a single atomic scheduling claim: a foreground parser wins between probe and trainer. Oracle: mock callback successfully reacquires the shared lease, proving the probe descriptor was released; trainer waiting_for_gpu becomes an explicit deferred receipt. Evidence is finite; fairness and future idleness are unproved.

## retention_exclusion

Type: partial_exclusion. Let S be round directories whose marker, terminal-success receipt, ordinary-file tree and absence of config-referenced paths establish cleanup eligibility. Deletion is a subset of the oldest max(0,|S|-8) members; failed/uncertain/foreign/referenced paths are outside S. Domain assumes cooperating owned artifacts with no hostile check/delete race. Counterexample to age-only deletion: an old successful directory contains the configured source DB. Oracle: retention test preserves that DB, failed/uncertain directories and eight other safe candidates while pruning one eligible artifact directory. Summaries survive separately. This does not reconstruct deleted artifacts or establish a secure filesystem sandbox.

## absolute_developer_anchor

Type: partial_exclusion specialized to repeated continuation. Under the unchanged first pinned developer set, Eel requires explicit finite A and the trainer rejects installation below A, even if each local loss is under0.5points. Example85.44->85.00->84.60 passes successive local tolerances but fails A=84.94 at the second step. Oracle: trainer absolute_anchor_prevents_cumulative_local_gate_drift. This constrains Eel installations on that observed developer metric; other controllers and unseen-data quality are outside the relation. No generalization or universal cumulative-error guarantee follows.

## planned_experiment_complement

Type: partial_exclusion. Domain: owned durable round index i, unchanged gate definitions and declared experiment tuple E_i=(parent,DB generation,base_seed+i,fraction,epoch,rate). A rejected E_i is terminal; another E_i is excluded from automatic resubmission, while a new planned index changes the seed and is a distinct trial. No failed-quality candidate can install. Counterexample to treating every rejection as an infrastructure failure: finite developer regression is expected search evidence, without file corruption or an uncertain child. Oracle: rejection advances counter without PAUSE, next round has a different seed, old weights remain, and three-rejection cooldown persists. No independence, eventual success or unseen-data guarantee follows from changing seeds.

## terminal_evidence_partition

Domain: historical/current local terminal Eel or trainer receipt JSON. Observation:
outcome, actual and proposed lineage fields. Assumptions: declared local receipts
are interpreted, not authenticated. Relation: deferred or below_threshold implies
NoTrainingRecorded; rejected requires generation/actual-parent/candidate digests
for CandidateRejectionRecorded; installed requires generation/actual-parent/
installed digests for InstallationRecorded; otherwise lineage is incomplete.
Counterexample: round1709 with proposed-parent only and reject_streak9 is deferred,
not a tenth training rejection. Evidence: finite Rust evidence controls and the
public entrypoint fake-native pipeline; no operational training by this review.

## priority_wait_complement

Domain: an admitted priority diagnostic returned by TrainingData at a slice
boundary. Observation: waiting_for_review status and resulting Eel summary.
Assumptions: backend only admits authorized actionable pending work. Relation:
AwaitingReview excludes ordinary status/export/native training for that slice;
its Eel summary is deferred and preserves prior rejection streak. Counterexample:
arbitrary unreviewed failures must not be treated as admitted urgent work; doing
so can starve the ordinary queue. Evidence: explicit pending-priority Rust fixture;
no global fairness theorem, runtime deployment or automatic annotation claim.

## tests

### Finite checks

Seven inherited Eel controls exercise leases, idle probes, crash reservations,
retention, cooldown and new seeded experiments. Public trainer tests add the full
public Eel entrypoint through fake driver/agree/native parser, actual file weight
installation, digest lineage and cooldown no-replay. Evidence unit controls keep
partial/deferred/failed outcomes distinct. All tests use synthetic inputs/fake
executables; they do not launch operational CUDA or prove timing/fairness.

## History

### 2026-11-02 · tests

Three status-identity controls: a paused `run(..., once)` with root or local PAUSE
emits `kind` = `mova_eel` and `config_sha256` equal to `identity.json`, leaves the marker bytes
unchanged, creates no round and never touches training data (panic backend); a
configuration change changes the emitted identity; a literal kind and a fixed
digest of a fixed configuration pin the wire contract independently of the
production helpers. Removing the fields from the writer turns the first two red
(exit 101, 2026-11-02). No direct Eel-to-Crab integration test exists; observer
compatibility rests on source inspection of its exact-equality check.
