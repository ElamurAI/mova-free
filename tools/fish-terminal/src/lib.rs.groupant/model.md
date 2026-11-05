# private-fish-terminal-core

## descriptant

### private-fish-terminal-core

Configuration, immutable queue and job/report lifecycle. The operator assigns canonical disjoint workdirs and unique fixed sessions. Config hash fixes the run; queue hash fixes ordered task admission. Pending tasks are reserved before any terminal effect, and unresolved tasks block their Fish. A report advances only after unchanged inputs, stable idle observation and nonce/task/digest/schema checks. A separate root decision can free the Fish but never trains or imports.

Prospective state/observation reasoning is in the section "Prospective state and observation model" below. Native checks preserve counterexamples and explicit UI/source limitations. Concrete tasks and configurations remain private; root handles repository relocation/public export separately.

#### Independent review corrections

Domain: frozen run, writable durable journals and at most one unresolved task per Fish. Observation: read-only launch/ownership/capture, source pins and report paths before first reservation. Let E_f mean preflight error, R_f reservations and A_f terminal effects. Before reservation, E_f implies delta R_f=delta A_f=0. Terminal probe errors record terminal_unavailable; source/report refusal records refused_before_effect with task ID and a reason digest; other Fish remain eligible. After reservation, buffer/paste/Enter errors remain pass errors and the retained uncertainty is quarantined next pass without replay.

Counterexample: the old question-mark propagation let one missing session, changed source or predating report abort the whole pass and starve another Fish. Native two-Fish witnesses cover those cases, invalid report directories and a waiting-report terminal becoming unavailable. This is a scheduler error partition, not eventual-completion proof or permission to resubmit sent tasks. Persistence/config/job-journal failures remain global.
Domain: bounded Linux file reads and cooperative directory stability during checks. Observation: opened descriptor type and canonical report directory. Admission requires O_NONBLOCK | O_NOFOLLOW, then fstat(descriptor).is_file before read. A job report additionally requires a canonical nonsymlink .peta-fish-reports directory and nonce-bound path. Directory failure after dispatch quarantines rather than resets.

Counterexamples: O_NOFOLLOW alone blocks opening a FIFO before fstat; final-component symlink checks admit a regular JSON behind a symlinked parent. Native controls use an actual writerless FIFO and a valid nonce-bound report behind a symlink. Evidence is finite actual Rust execution; no formal proof or numeric approximation bound. This partial exclusion does not establish descriptor-relative traversal or malicious concurrent rename resistance.

Root separately completed two real Codex TUI tasks and received two structurally bound reports in the external private pilot run. Root rejected the water report for a human/species overclaim and free-form nonenum pragmatics; hearing was accepted only as a provisional syntax critique. No DB/training admission occurred. This is finite end-to-end evidence for the old c215 runtime, not semantic validation or a live smoke of this M1/M2/M3 binary. This implementation worker used no model call or real terminal session.

#### Prospective state and observation model

The trusted operator gives each Fish one unique peta-fish-* tmux session and disjoint canonical work directory. A frozen queue supplies bounded English prompts and input SHA-256 bindings. Launch is separate from monitoring; monitoring never creates/replaces a session. Session metadata is an ownership check, not authenticated proof against a hostile local user.

Forward state: pending -> dispatch_reserved -> paste_reserved -> enter_reserved -> waiting_report -> review_pending -> reviewed. Every external boundary is preceded by a synced journal update. A reserved state encountered on a later invocation is quarantined, never replayed. A timeout or changed input also quarantines. A report is admitted only with the task ID, deterministic nonce, input digest, bounded JSON result and unchanged source inputs; terminal idleness alone is insufficient. A review decision records accept/reject only; it never trains, imports a DB or changes code.

Behavior exclusion: no known unrelated/busy/modal terminal may receive a prompt; one unresolved task blocks its Fish; the configured lifetime cap counts all reservations including quarantine; report mismatch cannot advance to review. A repeat report/queue publication is byte-idempotent. Reverse reconstruction: a waiting state retains possible failed/pending terminal histories; a valid report constrains task/input identity, not scientific correctness or the exact model actions. No unique recovery from screen text is assumed.

Cost: one bounded capture/check per active Fish per poll, bounded queue/report bytes and input rehashing. At most one model task per Fish is outstanding. CLI watch is an independent Rust OS process. Reports enter a local durable review queue; this does not inject asynchronous messages into an inactive root chat.

Assumptions: trusted same-user filesystem/config, cooperative tmux and Codex UI, successful sync/rename semantics. No hostile-user safety, general exactly-once external effects, crash durability theorem, model-quality proof, automatic sandbox trust approval or automatic root review is claimed. Real TUI markers must be pinned to an actually observed idle screen; fake tests do not certify a live Codex release. Root owns live smoke and launch authorization.

Read-only preflight failure is Fish-local before reservation; uncertainty after reservation remains fail-closed without replay. Exact tmux targets exclude prefix fallback. Nonblocking no-follow descriptor opens and regular-file checks exclude FIFO reads; canonical nonsymlink report-directory checks exclude the observed parent-symlink counterexample. These are partial typed exclusions with cooperative-local assumptions.

## model

# Source-bound partial models

## Forward

Configuration, immutable queue and job/report lifecycle. The operator assigns canonical disjoint workdirs and unique fixed sessions. Config hash fixes the run; queue hash fixes ordered task admission. Pending tasks are reserved before any terminal effect, and unresolved tasks block their Fish. A report advances only after unchanged inputs, stable idle observation and nonce/task/digest/schema checks. A separate root decision can free the Fish but never trains or imports.

## behavior_complement

Domain: the admitted configuration/input and cooperative local effects described above. Universe: transitions over the recorded job/session states and bounded observations. Under the trusted filesystem/config premises, no terminal rejected by the recognized busy/modal predicate, foreign ownership check or launch record passes dispatch; no known interrupted reservation is replayed; no mismatched report enters review; one unresolved task per Fish and the lifetime reservation cap are maintained. This is a partial exclusion over the implemented state machine, not a whole-model behavior complement.

## reverse_reconstruction

A valid review report constrains task ID, deterministic nonce, canonical task digest and observed input hashes. Several model executions, waiting durations and scientific reasoning paths can produce the same report. A reserved/uncertain stage retains both sent and not-sent terminal histories; it does not justify a unique replay decision.

## effect_partition

Failure partition: source mismatch before reservation returns without model dispatch; interruption at dispatch/paste/Enter is quarantined on the next invocation; missing report waits until the configured deadline; malformed or mismatched report quarantines; an explicit accept/reject only writes a decision. Atomic report/envelope writes are byte-idempotent, not an atomic transaction with the external model.

## Invariant

Configuration/task/source bindings remain explicit; unknown terminal/model effects are not converted to completed work. Review-only outputs do not mutate training, shared DB or production code. Exact byte identity and structural admission are separate from model truth.

## Cost

At most8Fish and1024tasks/reservations; queue/config/report128KiB,32input files per task, each128KiB. Each active pass performs bounded rehashing, JSON work and terminal observations; source I/O and external scheduling remain separate. Poll interval is between passes, not an instantaneous deadline guarantee.

## Checks

Eighteen native tests exercise actual core methods with fake terminal effects plus a real argv-only printf/sleep subprocess. Test report-content correctness is not linguistic quality; root's separate finite live smoke is described below.

Evidence status: source-derived partial model with bounded native controls. No formal proof, full semantic AST, percentage-error bound or live Codex quality guarantee.

## viewport_and_modal_partition

Domain: bounded captured text for the observed Codex0.165.0 inline UI, expected pane command and exact configured idle marker. Let end be the last non-whitespace line, and W the last16lines of the prefix ending there. Idle admission requires the marker in W and absence of recognized busy/setup indicators, including Update available plus Update now and Enter-continue actions. Removing only trailing blank viewport rows preserves visible content; it does not remove the intervening modal. Counterexample: an older idle marker can coexist with a default Update now action. The predicate does not establish that every future or translated modal is recognized. Evidence: padded baseline-red positive fixture, trim-only-red negative fixture and combined-green fixtures under actual core code.

## pre_effect_failure_partition

Domain: frozen run, writable durable journals and at most one unresolved task per Fish. Observation: read-only launch/ownership/capture, source pins and report paths before first reservation. Let E_f mean preflight error, R_f reservations and A_f terminal effects. Before reservation, E_f implies delta R_f=delta A_f=0. Terminal probe errors record terminal_unavailable; source/report refusal records refused_before_effect with task ID and a reason digest; other Fish remain eligible. After reservation, buffer/paste/Enter errors remain pass errors and the retained uncertainty is quarantined next pass without replay.

Counterexample: the old question-mark propagation let one missing session, changed source or predating report abort the whole pass and starve another Fish. Native two-Fish witnesses cover those cases, invalid report directories and a waiting-report terminal becoming unavailable. This is a scheduler error partition, not eventual-completion proof or permission to resubmit sent tasks. Persistence/config/job-journal failures remain global.

## report_artifact_exclusion

Domain: bounded Linux file reads and cooperative directory stability during checks. Observation: opened descriptor type and canonical report directory. Admission requires O_NONBLOCK | O_NOFOLLOW, then fstat(descriptor).is_file before read. A job report additionally requires a canonical nonsymlink .peta-fish-reports directory and nonce-bound path. Directory failure after dispatch quarantines rather than resets.

Counterexamples: O_NOFOLLOW alone blocks opening a FIFO before fstat; final-component symlink checks admit a regular JSON behind a symlinked parent. Native controls use an actual writerless FIFO and a valid nonce-bound report behind a symlink. Evidence is finite actual Rust execution; no formal proof or numeric approximation bound. This partial exclusion does not establish descriptor-relative traversal or malicious concurrent rename resistance.

## Separate root live evidence

Root separately completed two real Codex TUI tasks and received two structurally bound reports in the external private pilot run. Root rejected the water report for a human/species overclaim and free-form nonenum pragmatics; hearing was accepted only as a provisional syntax critique. No DB/training admission occurred. This is finite end-to-end evidence for the old c215 runtime, not semantic validation or a live smoke of this M1/M2/M3 binary. This implementation worker used no model call or real terminal session.

## configured_model

## History

### 2026-11-01 · descriptant

#### Observed terminal correction — 2026-11-01

Root's owned pre-task launch showed the idle marker above an update modal and more than20blank viewport rows. The original last16physical-lines predicate safely refused both that screen and an ordinary padded idle screen. Removing trailing blank padding alone would incorrectly admit the modal. Two native witnesses therefore separate viewport-padding normalization from action-modal exclusion: the old predicate fails the positive idle witness; the compiled trim-only intermediate fails the update-modal negative witness; the combined repair admits only the supported idle fixture. Generic weekly-limit warnings and /permissions help remain admissible. This is source/UI observation evidence for the declared release/layout, not a universal modal detector or a model-task completion test. No real terminal or runtime was changed by this implementation worker.

### 2026-11-04 · configured_model

Config.model defaults to DEFAULT_MODEL `gpt-6.1-terra`; Context::load accepts only `gpt-*` ids of at most 128 bytes without whitespace/control characters (complement: Claude ids and malformed ids are refused, no provider fallback). The field participates in the frozen config bytes like every other field. Checked by `configured_model_defaults_to_terra_and_refuses_non_codex_ids` (2026-11-04).
