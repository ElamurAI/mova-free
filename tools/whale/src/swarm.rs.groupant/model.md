# mova-whale-swarm

## Descriptant

Durable multi-lane scheduling by measured worker milliseconds with pluggable discovery. Source: `tools/whale/src/swarm.rs`. Public, domain-agnostic: concrete prompts, schemas, gate parameters and deployments come from pinned configuration.

## Forward approximant

Context validates lanes (labels, roles survey/agent, shares summing to one in ppm), workers 2..8, estimate, private dirs. drive: manager lease, identity pin, recover unresolved reservations to uncertain plus PAUSE, loop: reap children into receipts with measured elapsed ms; storage floor PAUSE; bounded discovery into pending rows with a durable cursor; reserve lanes by largest share deficit of effective time (estimates for in-flight/unmeasured), build child configs per role, spawn children inheriting the lifetime lease; write status. QueueFileDiscovery reads a pinned JSONL queue from the cursor skipping known keys. start: pinned config/executable binding, spawn manager once.

## Invariant

Unresolved reservations become uncertain and are never replayed; discovery never exceeds pending capacity and advances the cursor only with inserted rows; lane choice equals the two-lane fraction rule when two lanes exist; changed config or executable needs a new root; survey payloads may override only listed keys.

## Cost

SQLite queries per poll O(lanes) plus O(candidates) inserts; queue discovery O(queue bytes) per poll (bounded 128 MiB).

## Composition

Uses whale::prepare for survey children and launches `whale`/`whale-research`; WhaleRoles exposes status/start to Crab.

## behavior_complement

Excluded: lanes not summing to one, unknown lanes or roles, candidate keys that are not hex, payloads over bound, discovery beyond capacity, launches under PAUSE/STOP, restart after a recorded start. Counterexample: equal deficits keep configured lane order (two-lane oracle test). Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

From charged milliseconds per lane the order of completions is not recoverable; a lane state count admits many schedules. Manager receipts pin each child result so terminal outcomes are reconstructible per key. No unique inverse is claimed.
