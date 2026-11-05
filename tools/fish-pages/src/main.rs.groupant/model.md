# Paging CLI

## domain
Subject: main/run of the adjacent Rust file. Inputs are exactly a command and immutable config path; observations are read-only status output, controller loop state, exit and nominal idle wait. External Config/Pager and executor behavior are delegated rather than reimplemented.

## forward
status loads the config and reads the retained status file. run opens the exclusive pager and repeatedly performs one bounded step. STOP/deadline terminate; uncertainty returns a nonzero diagnostic; productive job/page transitions immediately continue; other states wait min(poll_seconds,5) seconds. This loop does not import a database or authorize learning.

## behavior-complement
Invalid arity/command excludes a controller launch. An uncertain pager state excludes automatic continuation. STOP/deadline excludes another loop step. A read status does not establish PID liveness or semantic review quality. Counterexample: a retained paused snapshot may belong to an exited process; a caller must check its time and actual process separately. Evidence: source inspection plus the library's focused no-replay/boundary tests; CLI dispatch itself is not exhaustively tested.

## reverse-reconstruction
An observed clean exit is compatible with STOP or deadline; it does not reconstruct completed source counts without bound status/history. A printed JSON status has multiple compatible runtime histories and is not an inverse of execution. A nonzero exit can arise from config, I/O, unknown command or uncertain history; stderr and artifacts retain the distinction. Assumptions: unchanged linked library and trusted files. No unique process/provider inverse is claimed.

## productive-loop
Type: temporal_partition. Let S be a step state. The nominal additional wait is0 for job_completed/page_completed, min(poll,5) for other nonterminal/nonerror states, and absent after stop/deadline/uncertainty exit. This is a scheduling policy, not a hard real-time or throughput guarantee; step and OS delays remain. Counterexample: sleeping5seconds after every paid batch unnecessarily adds that cost to useful cycles, while removing idle waits could spin on a missing page. Evidence: direct branch inspection; source throughput is not measured by the fake-executor test duration.

## invariant
The CLI never changes immutable config/queue bytes and never retries an uncertain paid attempt. Status remains a historical observation. Training and semantic admission are not conveyed by a process exit.

## cost
One immutable config read plus linked pager cost; idle wait is at most5seconds nominal. Status reading scales with the bounded JSON file. No task accuracy/error bound.

## composition
Arguments -> Config -> Pager -> step/result state -> immediate progress, bounded idle wait or exit. The linked library supplies ownership, leases, proofs and native gates. This file alone does not establish their semantics.

## checks

Release compilation and source projection; eight linked fake Rust pager controls and thirteen preserved swarm controls passed. These finite local checks made no live model, GPU, DB or training calls.

## History

### 2026-11-02 · checks

2026-11-02: release compilation and source projection; eight linked fake Rust pager controls and thirteen preserved swarm controls passed. These finite local checks made no live model, GPU, DB or training calls.
