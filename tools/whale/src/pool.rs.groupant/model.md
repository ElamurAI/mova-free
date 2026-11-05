# mova-whale-pool

## Descriptant

Crab-owned launch and observation of a prepared Whale worker cohort. Source: `tools/whale/src/pool.rs`. Public, domain-agnostic: concrete prompts, schemas, gate parameters and deployments come from pinned configuration.

## Forward approximant

Context pins the Crab root, master config hash, Whale executable hash and worker count (1..8). start: private pool dir under crab/whale-pools/<sha(master)>, short operation mutex, prepare or verify the pool manifest, observe existing journals, refuse PAUSE/STOP, take the lifetime workers lease, write launching journal and per-child reservations, spawn `exe whale CFG` in its own session with the lease descriptor inherited, record PID start ticks, mark launched. status/observe: PID identity, worker status, bounded receipt counts, uncertainty. marker: owned PAUSE/STOP files; resume removes only an owned PAUSE and refuses after STOP.

## Invariant

Each cohort launches at most once; prior journals are observed, never replayed; foreign markers are never modified; changed master/worker configs or executable are refused; lease held without live children reads uncertain.

## Cost

O(workers) spawns; status O(workers + min(receipts,128)).

## Composition

Called by WhaleRoles for Crab legacy pool control; prepares through whale::prepare; children run whale::run.

## behavior_complement

Excluded: relaunch after any journal, launch under PAUSE/STOP, overwrite of foreign markers, resume after STOP, operations during another operation, changed identities. Counterexample: a recorded `launching` journal with a reserved child yields state uncertain and no new process. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

From observed state one recovers launch phases and PID identities; an `uncertain` state admits several histories (crash between reservation and spawn, surviving descendant with the lease). No unique inverse is claimed.
