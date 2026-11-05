# mova-nautilus-pool

## model

# mova-nautilus-pool

## Descriptant

Nautilus as a member of a Crab work pool: reads the lease Crab writes into its assignment file between steps, works one item of the assigned layer (part of parts) with the job's shared ledger, and writes its report and the job's backlog file; seeds a ledger from an older journal once. Source: `tools/nautilus/src/pool.rs`.

## Forward approximant

step(now): current(assignment) = the lease if it names a job and now < expires_unix (another member's lease is an error; a released or missing file is none) -> none: report idle; layer = params.layer; unconfigured: report refused; a draft layer not made: animal::make natively, report working; else Config::job(layer, part, parts) and work::Loop{ledger = LEDGERS/<layer>/ledger.jsonl, member} .step -> backlog(layer) writes LEDGERS/<layer>/backlog.json (open items of the whole job) -> report state = finished (loop idle) | stopped (limit) | working. run: steps every SECS (x4 when idle/waiting/stopped/refused) until STOP.

## Invariant

The assignment is read only between steps, so a question is never cut by a job switch; an expired lease is no work; a job's ledger is the only shared state between members; reports and backlog files are written atomically.

## Cost

Per step: one bounded assignment read (64 KiB), the loop's step, one ledger read and one layer parse for the open count, two small atomic writes.

## Composition

Parent: `nautilus`. Uses animal (config, make), work (Loop with ledger), teacher (terminal). Counterpart of crab-pool in tools/crab: Crab writes the assignment and reads the report and backlog file.

## behavior_complement

Never works a lease that expired or names another member; never writes the assignment file; never shares a teacher terminal or journal between members; never asks a question already asked for the job by any member. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

The ledger recovers, per job, every item key asked or closed, by which member, and the question hashes; the report recovers the member's last lease and state. No unique inverse is claimed.

## tests

### Checks

pool::tests: a_job_moves_between_members_without_asking_again (25 weak rows, member A asks two questions, its lease expires, member B asks only the last five words; three questions in all; the layer file bytes unchanged; proposals in 9 columns), without_the_ledger_a_moved_job_asks_again (red control), expired_foreign_and_released_assignments_are_no_work, a_member_never_works_inside_the_layer_folder, a_shipped_layer_refuses_cc_by_sa_input.

## History

### 2026-11-04 · document

#### Epoch 2026-11-04

First model, written with the source (the Nautilus work pool); no prospective prediction is claimed beyond the checks.

#### Epoch 2026-11-04 (senses)

Tests only: the emode fixture uses words of three letters with two examples using them and batch 10; the replay terminal answers word, pick, difference.

### 2026-11-04 · tests

Heading `### Checks (2026-11-04)` stays in tests as `### Checks`.

Command: `cargo test --release` of the crate (tools/nautilus with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin). Finite fixtures; no terminal, teacher or model was called; not a proof.

2026-11-04 (senses): cargo test --release (0.4 en.bin), tools/nautilus 37 unit + 2 English tests, two identical runs; tools/turtle 9 + 2 pass.
