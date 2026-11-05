# Cooperative priority scheduler

## domain
One trusted local scheduler root, a finite set of explicit ordinary campaign roots, and independently admitted jobs J. Each job is (id, sequence, admission commitment, payload). The callback is external; the scheduler cannot establish its semantic truth or authorization. No numerical approximation is introduced. Filesystem identities and cooperative locking are assumptions, not a hostile-process sandbox.

## forward
Let R be durable reservations, T terminal receipts, P owned pauses, and q a fresh caller observation that ordinary work reached its boundary. Order jobs by (sequence,id). STOP/PAUSE yields without dispatch. Any retained r in R without matching terminal blocks further scheduling even if the ready list has lost that job. Valid terminal identities are skipped. If no pending job remains, remove only exact owned P. Otherwise create owned P; if q=false, return awaiting_ordinary_boundary without reserving. If q=true, append reservation before callback; successful return appends terminal receipt. Callback failure retains reservation and pauses.

The source-derived vidtvorant is this state transition on metadata. It reproduces selected admission/effect observations; child computation is opaque. It is not a surrogate for parser or teacher output.

## invariant
For every callback start, a reservation was written first. An unresolved reservation is never automatically resubmitted. Only exact scheduler-owned pause bytes can be removed. Cooperative pauses do not interrupt an active child. Terminal artifacts do not authorize database ingestion or training.

## cost
For n ready jobs, r retained job directories, and w ordinary roots, admission is O(n log n + r + w) metadata operations plus serialized payload bytes and callback cost C. No wall-time advantage or fairness bound is measured; externally supplied latency can dominate.

## behavior-complement
Type behavior_complement. Domain is the declared local transition. Let A be admitted callback starts. A excludes STOP/PAUSE starts, starts with unresolved reservations, starts under changed terminal commitments, and starts with q=false. This is an effect exclusion model, not merely a list of negative tests. Counterexample to a universal no-side-effect claim: creating owned pauses is allowed while q=false. Evidence: finite native fixtures plus source branch inspection; no universal proof.

## reverse-reconstruction
Type reverse_reconstruction. A terminal receipt reconstructs compatible admission commitment and callback return value. Its preimage still contains multiple internal callback executions and timings; no unique original prompt, model reasoning or semantic validity follows. A reservation without terminal is compatible with not-started, active, completed-but-unrecorded and failed callbacks. Counterexample to retry-on-missing-receipt: the callback may already have produced irreversible effects. Evidence: uncertainty fixture; external child histories remain opaque.

## boundary-exclusion
Type effect_boundary. Observation q=false with pending valid job implies owned pauses may exist but new reservation and callback do not. Assumption: caller reports an actual recent quiescence observation. Counterexample to equating PAUSE-file presence with quiescence: an ordinary child was already in flight and has not returned to its loop. Native fixture verifies the first call only pauses and a later true observation permits execution.

## uncertainty-partition
Type error_partition. Separate known terminal, pending unreserved, reserved without terminal, and invalid changed identity. Only pending unreserved can start. Removing a case from the ready list does not convert uncertain history into idle work. Counterexample: empty queue plus retained unresolved reservation still fails. Finite checked evidence does not give crash-atomic filesystem/child semantics.

## checks
Six native tests verify deterministic priority, no terminal replay, foreign PAUSE/STOP preservation, active-boundary waiting, changed admission rejection, and uncertainty surviving queue removal. These are bounded local executions, not training or semantic correctness evidence.
