# fish-executor-reservation

## model

# Attempt reservation and temporal boundary
## forward
Domain: a new caller-selected attempt directory. Validate local inputs and deadline, create directory privately, sync reservation, write and sync exact prompt and raw streams, check deadline again, then spawn at most one direct child for that attempt. Poll completion/timeout/output, retain diagnostics and sync terminal receipt.
## behavior_complement
An existing directory cannot launch another request through this run function. Expired deadline or oversize full prompt cannot launch. Timeout and malformed response cannot automatically retry. Counterexample to a stronger exclusion: a fresh second directory can bypass per-directory uniqueness unless the supervisor pins task reservations. Evidence: fake nonlaunch, replay and timeout controls.
## reverse_reconstruction
Reservation without receipt corresponds to both unsent and possibly sent histories. It cannot distinguish a local crash before spawn from interrupted post-spawn capture. A receipt narrows the history but does not recover remote accounting. Assumptions: local fsync behavior and trusted paths; no power-loss theorem.
## invariant
Reserve precedes spawn; receipt is terminal evidence before successful return. Active requests use a separate monotonic timeout; wall deadline governs new launch admission. Missing receipt is uncertainty, never evidence that no paid call occurred.
## cost
Bounded JSON/prompt file I/O plus one child lifetime and 50ms polling. Polling output detection may overshoot its nominal read bound; no disk-quota or real-time bound.
## composition
The outer supervisor owns immutable task queues, global uniqueness and process lifetime. This fragment owns one direct-child attempt. Their shared directory and identity protocol must be checked together; neither alone supplies exactly-once remote execution.
## descriptant
Partial artifacts are useful crash evidence. Recovery requires inspection and an explicitly new reviewed campaign decision, not automatic directory reuse.

## tests

Finite fake-CLI controls are shared with execute.rs tests. Source-level prefix ordering is inspected; no exhaustive crash, provider or semantic proof.
