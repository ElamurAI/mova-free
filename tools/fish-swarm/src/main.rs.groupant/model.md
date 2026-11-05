# Prepared Fish command loop

## domain
Exactly two CLI arguments: run|status and canonical config path. The library owns queue and process semantics. No language/model provider choice is hardcoded. Trusted cooperative storage and a conforming executor are assumed.

## forward
Parse command and immutable configuration. Status reads the current snapshot and probes the cooperative lease; it does not dispatch. Run acquires Supervisor, repeatedly executes at most one step, exits on stopped/deadline_reached, fails on uncertain/failed, and otherwise sleeps min(config.poll_seconds,5). Queue exhaustion therefore waits without replay until a boundary. The forward vidtvorant is the command/state-to-loop-control mapping; language output is outside its observations.

## invariant
Status never becomes run. Failed or uncertain step cannot advance to another dispatch. Terminal campaign states exit; completed_review_only is merely a per-job state. Actual wait is at most5seconds subject to OS scheduling. Library errors propagate to stderr and process exit1 rather than implicit retry.

## cost
For k steps, cost is sum of library step costs plus k bounded sleeps; process startup and idle polling are not a measured throughput advantage.

## behavior-complement
Type behavior_complement. The command relation excludes unknown verbs, wrong arity, status-triggered execution and continuation after uncertainty/failure. Counterexample to treating every successful step as loop termination: completed_review_only and queue_exhausted continue polling. Evidence: source branch inspection plus the library's fake controls; CLI branch behavior is not separately integration-tested.

## reverse-reconstruction
Type reverse_reconstruction. Exit1 has compatible causes including config parse, lease failure, uncertain history and child failure; stderr refines but does not uniquely reconstruct system history. Exit0 has status-query or STOP/deadline preimages. No unique model computation or semantic success follows.

## heartbeat-wait
Type temporal_boundary. For configured p in1..60, nominal idle wait w=min(p,5). Thus w<=5, while observed heartbeat spacing can exceed w because scheduling and storage latency are external. Counterexample to a hard5second guarantee: blocking filesystem operations. Source-derived arithmetic, no hard real-time proof.

## checks
Native cargo test compiles this CLI; thirteen library controls test the delegated scheduling transitions. No real model or deployment is exercised.

## epoch-memory
A per-job completed state must never be admitted by a priority observer as campaign-terminal quiescence. Configurable provider/deadline values remain outside CLI identity.
