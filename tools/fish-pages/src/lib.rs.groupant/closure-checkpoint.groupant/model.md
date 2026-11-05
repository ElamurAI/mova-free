# Closure/checkpoint fragment

## domain
Source scope: sealed_prefix, Pager::step/status and quiescent in the parent Rust file. Observations are native closed histories, page seals, checkpoint/counters and safe physical boundaries. The shared partial syntax covers the source file; it is not a composed semantic AST.

## forward
Reuse imported native closed-receipt predicates; seal a fully closed current page before advancing its cursor. Reconstruct a verified prefix after restart and inspect the unsealed page's original attempts before any boundary. Active monitoring keeps parent status unsafe until the step thread drains. Read-only quiescence rechecks source counts and completed/current histories.

## behavior-complement
An orphan, false/null closure, changed identity, cursor ahead of seals, malformed checkpoint or inconsistent source count excludes progress/quiescence. STOP or expiry cannot erase uncertainty. Fresh paused+PAUSE differs from terminal state requiring unheld leases. Counterexample: an accepted model call can remain active after the wall cutoff, so expiry alone is not a no-call proof. Evidence: orphan-first, active-drain, actual flock and corrupt-state controls.

## reverse-reconstruction
A valid seal constrains the compatible local history to exact pinned intent/admission/receipt bytes, but still admits different remote execution and linguistic states. An unreceipted reservation has multiple compatible preimages and cannot authorize replay. A stale checkpoint behind an already synced verified seal can be rebuilt without repeating a paid page; a corrupt checkpoint is refused. Assumptions: cooperative immutable files and collision resistance; no remote billing/exactly-once theorem.

## invariant
Known-closed invalid reviews remain quarantine, not gold; preparation gaps remain pending review. Parent active status is published before starting the worker thread. Every claimed safe boundary follows history audit, while the ordinary job path avoids rereading all sealed pages.

## cost
Current-page audit is bounded by its queue/task/receipt bytes. Restart and observer checks scale with all completed history bytes; these costs are intentionally separate from ordinary paid steps. Scheduler and provider timing are outside a formal bound here.

## composition
Native closed attempt -> current-page summary -> source-bound seal -> checkpoint -> next source ownership. The public observer joins this evidence with fresh marker/identity and lease conditions; it never launches work.

## checks
Shares eight parent fake Rust controls, particularly restart, quarantine, orphan, active pause/drain, terminal lease and changed closure/checkpoint. These are finite interface observations, not semantic assurance.
