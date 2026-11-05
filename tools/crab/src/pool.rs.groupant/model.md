# crab-pool

## model

# crab-pool

## Descriptant

Work pools in Crab's flow configuration: members (a configured worker, an assignment file Crab writes, a report file the member writes) and jobs (priority, backlog probe, parts, licence line, opaque params). Crab leases one unit (job, part) to each idle member and journals every assignment and release. No member-specific code. Source: `tools/crab/src/pool.rs`.

## Forward approximant

tick(t): read every job's backlog (flow::read; unknown is None) and member report (<= 64 KiB). For each lease: job gone -> release; report for this lease says finished/refused -> done[unit] = backlog, release; stopped -> release; t >= expires -> release (expired); worker running and (report fresh: unix + lease > t, or t < assigned + lease) -> expires = t + lease, rewrite. done[unit] is dropped once its job's backlog reading changes. Waiting units: backlog > 0 known, not held, not done; sorted by priority desc, backlog desc, job, part. Idle members (worker running, no lease, report not stopped) in name order take them pairwise; lease id = pool-seq. check(): no unit held twice. status: English lines (`m works on job; the lease runs n s more.`, `m is idle.`, `job waits with a backlog of n.`) and a view; capacity: backlog totals per pool.

## Invariant

A member holds at most one lease and a unit at most one member (checked every tick); a job without parts > 1 is never on two members; a lease of a dead or silent member is not renewed past one period and expires; every assignment and release is journaled before the state is saved.

## Cost

Per tick and pool: one bounded read per job backlog and per member report, at most one atomic write per member; members <= 32, jobs <= 64, parts <= 16.

## Composition

Parent: `crab-orchestration`. Flow owns the pools (FlowConfig.pools, Flow.tick, status, kinds, capacity), so flow_status, the council digest and the capacity review show them; releases for stopped or refused members become attention items through the flow events. The member side is mova-nautilus-pool.

## behavior_complement

Never starts, stops or signals a worker; never assigns to a member whose worker is not running or whose report says stopped; never splits a job that does not declare parts; never interprets params; never enforces a job's licence (the member does). Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

pools.jsonl recovers every lease (member, job, part, id, expiry, backlog at assignment) and every release with its reason; pools.json the current leases and done units. Readings between ticks are not reconstructed.

## checks

pool::tests: one_member_per_job_and_priority_first (priority order, a held job never taken by the idle member, a splittable job one part each, check refuses two members on a unit), a_lease_expires_and_the_job_moves (renewal by fresh report, no move before expiry, move after, stale report does not renew after the first period), finished_and_stopped_end_the_lease, the_flow_shows_and_journals_the_pool (status lines, kinds, capacity, journal, state across reopen). Red controls (each red, restored): held filter removed (double assignment), expiry ignored, renewal while the worker is dead, a limit-stopped member given work.

Command: `cargo test --release` of the crate (tools/nautilus with MOVA_OCTOPUS_TEST_MODEL=(see History)). Finite fixtures; no terminal, teacher or model was called; not a proof.

### Checks

## History

### 2026-11-04 · document

#### Epoch 2026-11-04

First model, written with the source (the Nautilus work pool); no prospective prediction is claimed beyond the checks.

### 2026-11-04 · checks

Heading `### Checks (2026-11-04)` stays in checks as `### Checks`.

pool::tests: one_member_per_job_and_priority_first (priority order, a held job never taken by the idle member, a splittable job one part each, check refuses two members on a unit), a_lease_expires_and_the_job_moves (renewal by fresh report, no move before expiry, move after, stale report does not renew after the first period), finished_and_stopped_end_the_lease, the_flow_shows_and_journals_the_pool (status lines, kinds, capacity, journal, state across reopen). Red controls 2026-11-04 (each red, restored): held filter removed (double assignment), expiry ignored, renewal while the worker is dead, a limit-stopped member given work.

Command: `cargo test --release` of the crate (tools/nautilus with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin). Finite fixtures; no terminal, teacher or model was called; not a proof.
