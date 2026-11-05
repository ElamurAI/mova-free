# Bounded Fish pages

Mova's animals and the Octopus, the large language model that develops Mova, are described in [the sea animals of Mova](../../docs/animals.md).

`mova-fish-pages run CONFIG` consumes a pinned plan through bounded immutable pages. `status CONFIG` reads the latest snapshot; it does not establish process liveness. The pager reuses `mova_fish_swarm::Supervisor` for paid attempts, leases, raw/typed bindings, deadline admission, complete-target validation and known-closed quarantine. It never imports annotations, trains a model or marks structural reviews as semantic gold.

Configuration is schema 1 with absolute canonical `root`, `plan:{path,sha256}`, `ready_root`, `fish_executable:{path,sha256}`, `not_after_unix`, optional `timeout_seconds` (default 300, 1..900), and `poll_seconds` (default 5, 1..60). Build in release mode. Each pinned file remains below 64 MiB; page queues remain bounded rather than enlarging that cap.

The plan is `{schema:1,pages:[{path,sha256}]}`. Each pinned intent is `{schema:1,id,sources:[sourceID]}` with 1..256 unique source IDs, and source ownership is disjoint across all intents. Intent IDs use at most 64 ASCII letters/digits/underscore/hyphen. The plan is read and ownership checked once per controller opening.

A producer atomically publishes `ready_root/INTENT_ID.json` with exactly `{schema:1,intent:{path,sha256},queue:{path,sha256},deferred:[{source_id,reason}],rejected:[{source_id,reason}]}`. The queue uses existing prepared Fish job JSONL. Actual pinned `Task.target_ids`, deferred and rejected IDs must form a disjoint complete partition of the intent; an oversized or malformed target must be explicitly accounted for, never silently dropped. Gap reasons are nonempty and at most 2048 UTF-8 bytes. Each task has 1..8 targets; concrete deployments may choose smaller batches. No page can introduce foreign IDs or make one ID both queued and deferred.

The pager creates each child configuration beneath `root/pages/INTENT_ID`, runs one bounded swarm step at a time and publishes a heartbeat while that step drains. PAUSE prevents the next paid attempt; an accepted attempt may finish. Only the pager's own child PAUSE marker is removed on resume. STOP and the wall-clock deadline also wait for the current step. Missing ready pages wait without replaying earlier work. Unknown or incomplete retained attempts precede every safe-boundary claim and stop the controller. A source-bound seal is written before the cursor advances; restart verifies sealed histories and recovers an interrupted seal/checkpoint update without repeating paid attempts.

Progress uses source IDs: `counts` separates validated, quarantined, deferred and rejected targets; `review_pending_targets` retains everything lacking a validated structural review. `sealed_counts` excludes the current page; `cursor` is its zero-based index and is also the producer high-water signal. `plan_exhausted` does not mean every source was reviewed: deferred/rejected/quarantined sources remain visible. Productive transitions add no fixed polling delay; idle/pause waits and active heartbeats are bounded to at most 5 seconds.

`quiescent(root,status,now)` is a read-only observer API. It binds kind/config/plan/intent/admission identities, actual source counters, sealed and current histories, and fresh PAUSE or a persistent terminal boundary. Terminal snapshots also require unheld parent/current-child leases. It does not prove semantic correctness, provider accounting, adversarial filesystem safety or absence of deliberately detached remote work. Completed sealed pages are trusted immutable during ordinary steps; all their histories are rechecked on restart and observer audit, not before every new job.

Fake-executor Rust tests cover source-counted pagination, no replay on restart, late-page waiting, explicit gaps, known-closed quarantine, orphan precedence, active drain/heartbeat, binding and checkpoint corruption; the swarm crate has its own checks. These checks use no live model, GPU, database, training or corpus assessment.

The pager republishes the swarm's `provider_busy_backoff` and `provider_busy_held` states (safe, model-call false, `details.provider_busy`) and keeps polling; it does not seal, advance or stop on them. A guard that stops the pool on `uncertain` therefore ignores a busy provider, while usage-limit failures still stop it.

## Build and test

A standalone workspace (own `Cargo.lock`), not built by the root `cargo build`:

```bash
cargo build --release --manifest-path tools/fish-pages/Cargo.toml
cargo test --release --manifest-path tools/fish-pages/Cargo.toml
```

The binary is `target/release/mova-fish-pages` in the target directory of that build; it runs `mova-fish-pages run CONFIG` and `mova-fish-pages status CONFIG` as described above.
