# Cooperative Fish priority dispatch

Mova's animals and the Octopus, the large language model that develops Mova, are described in [the sea animals of Mova](../../docs/animals.md).

A reusable safe-boundary scheduler for already admitted annotation jobs. Deployment code supplies ordered ReadyJob values and an executor callback. The library does not review prompts, invent annotations, import a database or train a model.

At each dispatch, the scheduler holds its process lease, checks STOP/PAUSE, retains unresolved reservations, sorts by (sequence,id), and skips matching terminal receipts. Ready work creates ownership-marked PAUSE files for ordinary controllers. An in-flight ordinary job finishes normally. The caller must supply a fresh ordinary-quiescent observation before the callback can start. A false observation returns awaiting_ordinary_boundary without creating a job reservation.

The scheduler writes its durable reservation before invoking the executor. A callback error retains that reservation and owned pauses for investigation; it never retries uncertain work. A successful callback writes a terminal receipt. When no admitted ready job remains, the scheduler removes only its exact owned pause bytes. Foreign operator pauses remain untouched. Removing a case from the ready list does not erase a retained uncertain reservation.

This is cooperative process control on a trusted local filesystem. It does not prove instant preemption, global exactly-once side effects, fairness, semantic truth or training eligibility. Ordinary controllers must check their configured root/PAUSE between jobs. A caller reporting quiescence without actually observing it violates the API contract.

Run the bounded native checks with `cargo test --manifest-path tools/fish-priority/Cargo.toml --offline --locked -j2`. They exercise ordering, active-boundary waiting, foreign controls, changed admissions and uncertain no-replay behavior. No teacher or GPU operation is involved.
