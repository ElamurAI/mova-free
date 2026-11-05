# Pinned page scheduling

## domain
Subject: Config, plans, admit, Pager and quiescent in the adjacent Rust file. The domain is a cooperative filesystem with immutable pinned source plans, intents, prepared Task/profile artifacts, a conforming pinned executor and Unix leases. Observations include source-ID partitions, parent/child configuration identities, actual attempt histories, seals/cursor, status heartbeats and review-only counters. Corpus selection and held-out policy are producer obligations, not inferred from source names. The imported prepared-swarm audit is an explicit external dependency, not source owned by this leaf. Partial syntax omits macro expansion and semantic AST analysis.

## forward
Let I_0...I_(P-1) be immutable intent source sets. Opening verifies I_j intersect I_k is empty for distinct indices and 1<=|I_j|<=256. For current page c, admit a ready queue only when the pinned Task target sets Q and explicit deferred D/rejected R form I_c=Q disjoint-union D disjoint-union R. Reuse one prepared-swarm step at a time for Q. A source-bound completed page seal commits the intent/admission, previous seal hash and reread native history audit before c increases. Restart reconstructs the verified seal prefix; the current unsealed page retains its existing native attempt ledger. A missing next ready file waits without changing the prefix. Productive steps do not impose an idle polling delay.

## behavior-complement
Domain and observations are as above. A foreign, duplicate or unaccounted source excludes admission; one ID cannot appear in both a Task and a deferred gap. Unresolved retained attempts exclude every safe-boundary claim, even if STOP or deadline is present. Unbound queue/task/profile/executor bytes exclude a launch through the prepared child. Existing history forbids automatically replaying that paid attempt. The complement is partial: valid source bindings and native structure do not exclude shared semantic mistakes. Counterexamples to stronger claims: an empty all-deferred page is exhausted with zero reviews; a second unrelated configuration could target the same external corpus unless its producer ownership policy prevents it. Evidence: duplicate/partition/gap and orphan-first Rust controls.

## reverse-reconstruction
Given cursor c and verified seal prefix, compatible histories include different paid reply texts with the same source counts and different schedules that produce the same closed receipts. A seal constrains the compatible set by exact intent/admission/config/hash and audited receipt bytes; it does not recover a unique linguistic interpretation or a remote billing history. A reservation without valid closure still admits never-started, interrupted and remotely completed histories and cannot authorize replay. Counterexample: a known-closed invalid four-target reply and a valid four-target reply have equal attempted-source cardinality but different review states. Evidence: restart and quarantine controls; no universal crash or provider proof.

## invariant
Source identity is conserved at the admission partition. Counts separate validated structural review, known-closed quarantine, local deferral and local rejection. review_pending_targets=total_sources-validated_targets, so quarantine and preparation gaps remain pending review. Only exact closed native histories permit progress; annotation_gold and training authority are never granted. The current page alone is scanned during ordinary paid work, while restart/observer audits reread the completed prefix. Status is an observation, not process-liveness proof. Publication of active/unknown precedes starting the step thread, preventing an old paused snapshot from authorizing an asynchronous launch after an I/O failure.

## source-ownership
Type: set_partition. Domain: one immutable intent and its bounded ready queue/gap arrays. Formula: Q union D union R=I and all pairwise intersections are empty; target IDs are read from the actual pinned Task JSON. Global source uniqueness holds across pinned intents at opening. Observation: admission/nonadmission and distinct source counts. Counterexample: a declared gap for an already queued ID must fail even if every Task is otherwise well formed. Assumptions: collision resistance and immutable files; no theorem about the producer's corpus completeness. Evidence: exact partition and cross-page duplicate controls.

## replay-prefix
Type: monotonic_prefix. Domain: seals written before the atomic cursor checkpoint. The authoritative prefix is the maximal contiguous verified seal sequence. A stale checkpoint behind that prefix can be reconstructed without rerunning child attempts; a cursor ahead of verified seals or mismatched counters/hash is rejected. An unsealed page recovers through its original prepared-swarm receipts and lease. A future page with retained history is not ignored. Counterexample: a crash after sealing but before checkpoint publication need not repeat the paid page; an orphan beside completed records still prevents boundary claims. Evidence: no-replay restart and corrupt-checkpoint/closure tests. Power-loss semantics beyond synced files/directories remain assumptions.

## closure-boundary
Type: process_effect_boundary. Observations: kind/config/plan identity, fresh status, parent marker, exact audited history and leases. Quiescence requires no active/unknown attempt and either fresh paused+PAUSE or persistent STOP/expired deadline with unheld parent/current-child leases. While a step thread drains, parent snapshots remain active with no-unresolved=false; child model-call uncertainty is preserved as null. Accepted work may finish after the wall cutoff within the child timeout. Counterexamples: a held terminal lease is not terminal proof; a stale paused heartbeat or removed kind identity cannot establish this boundary. Evidence: actual flock, active-drain heartbeat and orphan precedence tests. No proof about deliberately detached descendants or provider accounting is claimed.

## error-partition
Missing next admission means waiting; malformed admission or changed pins means refusal; explicit D/R means preparation gap without model call; known-closed invalid reply means quarantine and next distinct work; false/null/missing closure means uncertainty and stop. Native provider/transport errors are not ordinary inner-draft quarantine. Observation of one closed process is distinct from semantic resolution. Counterexample: an outer schema-valid reply may contain a double-encoded CoNLL-U string and must remain a rejected review rather than be silently decoded. Evidence: imported native closure predicate plus local fake-executor error controls; live provider behavior is outside these tests.

## source-counts
Type: source_cardinality. For a job targeting k sources, a complete native structural success contributes k validated sources; a closed complete-target rejection contributes k quarantined sources. D and R contribute zero validated sources. This is a declared observation metric, not an accuracy percentage or a claim of gold. Counterexample: one two-target job must not be reported as one reviewed source, and an all-deferred two-source page has review_pending=2 after exhaustion. Evidence: source-counted pagination, quarantine and gap tests.

## cost
Let N be all intended sources, P pages, M<=256 current sources, B current queue/task/receipt bytes and E one child inference lifetime. Startup ownership indexing costs O(N log N) plus pinned metadata reads; ordinary step auditing is O(B)+E with current-page scope. Heartbeat status totals traverse P intents, not their source text or completed histories. Restart and observer checks cost O(completed history bytes), deliberately outside every-job scheduling. Memory retains the intent ownership sets and one bounded page. No numerical runtime or semantic error bound is asserted; the imported native executor supplies its own time/output budgets.

## composition
CLI -> immutable Config/plan/intent ownership -> private producer's ready queue -> exact source partition -> prepared-swarm step thread -> pinned native executor -> native closed history -> page seal -> checkpoint -> next page. Producer annotation/held-out correctness, native semantic quality and observer policy remain distinct dependencies. The parent group owns this file and CLI only; the swarm is an external sibling model.

## checks

Eight fake Rust executor tests pass in1.51seconds; thirteen preserved prepared-swarm tests pass in11.59seconds, offline/locked with two build jobs. Tests cover restart/no replay, late ready pages, global/source partition, per-source counts, explicit gaps, known-closed quarantine, orphan-before-boundary, active drain/heartbeat, leases and malformed checkpoint/closure. Syntax is generated deterministically twice by the groupant guard. No live model, GPU, database, training or corpus assessment ran in these checks. Finite fixtures and source inspection are evidence, not a formal semantic or exactly-once theorem.

## provider-busy-state

## History

### 2026-11-02 · checks

2026-11-02: eight fake Rust executor tests pass in1.51seconds; thirteen preserved prepared-swarm tests pass in11.59seconds, offline/locked with two build jobs. Tests cover restart/no replay, late ready pages, global/source partition, per-source counts, explicit gaps, known-closed quarantine, orphan-before-boundary, active drain/heartbeat, leases and malformed checkpoint/closure. Syntax is generated deterministically twice by the groupant guard. No live model, GPU, database, training or corpus assessment ran in these checks. Finite fixtures and source inspection are evidence, not a formal semantic or exactly-once theorem.

### 2026-11-04 · provider-busy-state

Type error_partition (2026-11-04). When the inner swarm step returns provider_busy_backoff or provider_busy_held, the pager publishes the same state with active_model_call=false, no_unresolved_reservations=true and details.provider_busy (job, attempts, retry_after_unix), after the STOP/deadline/PAUSE boundary check. It does not advance the cursor, does not seal and does not stop the run loop: the loop polls. A limit guard that keys on "uncertain" therefore ignores provider-busy states, while usage-limit and other executor failures stay uncertain and stop the pager as before. Seals include the audit's provider_busy_attempts. Evidence: one fake pager test (busy refusal, restart during backoff, retry completes, seal records one busy attempt); nine pager tests pass.
