# mova-hard-cases-control

## Descriptant

The lane's priority and limit share: owned PAUSE on yield_roots while a lane pager works (fresh heartbeat, non-terminal state), removed only as exact owned bytes; an uncertain lane pager stops lane and stop_roots; a generic limit-guard STOP stops the lane. Source: `tools/hard-cases/src/control.rs`.

## Forward approximant

step(now): working = #lane pagers with state not in {plan_exhausted, stopped, deadline_reached, uncertain}, heartbeat within fresh_seconds and no STOP; any lane uncertain -> STOP(lane + stop_roots), exit; stop_root STOP starting 'limit-guard' -> STOP(lane); working>0 and no PAUSE -> write marker; working=0 and PAUSE == marker -> remove.

## Invariant

A foreign PAUSE or STOP is never removed or rewritten; STOP files are only created; the guard exits after stopping on uncertainty.

## Cost

O(lane pagers + roots) file reads per poll (20 s).

## Composition

Cooperates with fish-pages PAUSE/STOP semantics and the generic pool's limit guard. Parent hard-cases.

## behavior_complement

Does not preempt an accepted attempt, prove liveness from a snapshot, resume a stopped pager or retry. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

Events are logged; the PAUSE marker names the lane root, so its owner is recoverable from the file.

## Tests

control::tests: lane work pauses t06 but not the operator-paused t07; stale heartbeat and plan_exhausted remove only the owned pause; red: uncertain lane pager stops lane, generic pagers and producer; a generic limit-guard stop reaches the lane. Finite fixtures and live samples; not a proof.

## History

### 2026-11-04 · Tests

control::tests (2026-11-04): lane work pauses t06 but not the operator-paused t07; stale heartbeat and plan_exhausted remove only the owned pause; red: uncertain lane pager stops lane, generic pagers and producer; a generic limit-guard stop reaches the lane. Finite fixtures and live samples; not a proof.
