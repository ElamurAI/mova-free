# crab-council

## model

# Crab council — source-bound model

## Descriptant

`council.rs` lets two or more named consultant Dolphins review Crab's supervision when its facts change
(or after the longest interval) and independently, combines their typed verdicts deterministically, and keeps the Octopus's attention queue.
Answers are evidence; only a unanimous allowed action is executed, through flow's bounds.

## Forward

tick(t): with an open round, close it when every asked member answered or t >= asked + member_timeout
(silent members recorded timed_out). Otherwise, at most every check_seconds: F = facts(), phi = H(stable(F))
(stable: probe conditions, over-high-water stacks, raised quarantine anomalies, digit-masked flow summary,
worker states, recent actions without times/values, refusals). due = (phi != phi_last and t >= last + min_spacing)
or t >= last + period. Not due -> skip; idle members < quorum -> skip (busy/stopped recorded); else last := t,
phi_last := phi and digest(F) of at most max_task_bytes is queued unchanged to each idle member (origin crab,
trigger council:<round>:<dolphin>); the others are recorded absent. A skip is journaled only when its reason
differs from the previous skip; repeats are counted in state.

close: answers A (asked members that answered); |A| < quorum -> Escalate(expired); else combine over A.

parse_verdict(answer): first non-empty line must be `VERDICT: <token> [reason]` with token in
{none, octopus, action:a:p (a in ACTIONS, p a probe), capacity:k:more|fewer:n (k a kind, 1 <= n <= 64)};
otherwise Malformed. combine(V): |V| < 2 or any Malformed or any Capacity or any Octopus -> Escalate; all None ->
JournalOnly; all equal Action -> Execute; else Escalate (disagreement). Execute goes through
flow::council_decision; a refusal or error escalates.

Attention: fp = H(source, facts) (canonical JSON; facts never contain time, id or round; free text digit-masked).
Known fp -> count += 1, last := t, newest summary kept in state.json (<= 512 fp, least recently seen dropped);
new fp -> one line appended to attention.jsonl. attention.md and view: the 16 most recently seen merged items
with repeat counts.

## Invariant

- A capacity verdict never reaches exec.
- Exec is called only when every member's verdict is the same allowed action on a configured probe.
- Every member of a round receives byte-identical digest text containing no member's answer.
- No consultation is queued over an active one: a busy member is absent from the round.
- Unanimity is over at least `quorum` (>= 2) answers; a timed-out member is evidence, never a vote.
- With unchanged phi at most one round per period; with changing phi at most one per min_spacing.
- An attention fingerprint appears at most once in attention.jsonl while it is kept in state.

## Cost

At most one consult entry per member per min_spacing (>= 60 s) and, with unchanged facts, per period
(>= 600 s); one digest composition and fingerprint per check_seconds; digest composition linear in probes; parsing
linear in the answer; attention view reads at most a 256 KiB tail.

## Composition

`angel.rs`: Config.council (requires flow), CouncilPod (availability from the Dolphin's worker state,
pending consultations, waiting entries and terminal idleness; enqueue into consult_queue; answers from
answered/dispatch_failed entries and their reports), supervision_tick, the `attention` operation and
refusal recording in ask_op.

## behavior_complement

Type: behavior_complement. Not done: executing anything named outside the typed first line; acting on a
single Dolphin's verdict; changing the number of animals; judging advice quality; acknowledging or deleting
attention items. Meeting on unchanged facts before the longest interval; overwriting a busy member's task; counting a timed-out member as a vote; appending a repeated attention fingerprint.

## reverse_reconstruction

Type: reverse_reconstruction. A journal line recovers the round, each member's verdict label, bounded reason
and answer hash, and the outcome; the full answers are recovered from the consult reports by hash. A verdict
does not determine the digest's facts beyond the stored digest hash and the attention item's numbers.

## checks

### Checks

Module tests: verdicts_parse_strictly, combination_escalates_everything_but_agreement,
rounds_skip_busy_switch_off_and_never_execute_capacity (fake pod: off, skipped, same digest to both,
waiting for answers, capacity escalated with both reasons and numbers, agreement executed, disagreement and
malformed escalated, expiry).

Live Dolphin terminals are not exercised; the idle-window terminal check is tested in angel
(idle_window_admits_tui_footers_but_not_a_longer_marker).

## History

### 2026-11-04 · document

#### Epoch 2026-11-04 (event-driven council, merged attention)

Live evidence (council journal 2026-11-04): eight rounds every 1800 s; six escalations repeated the same
octopus verdicts on unchanged t05/t07 quarantine facts, each appended as a new attention item; one round
expired after 3600 s while Sol stayed silent, and the two following rounds were skipped because Sol was busy.
Now rounds follow fact changes (min spacing 300 s, longest interval 1800 s), a busy member is absent instead of
blocking, a silent member times out after 600 s, and repeats merge in the attention state. Prospective
prediction: on a steady flow, at most one round per period (two per hour) and the attention file grows only on
new facts; with a changing flow the bound is twelve rounds per hour.

### 2026-11-04 · checks

Heading `### Checks (2026-11-04)` stays in checks as `### Checks`.

Red controls (each turned a test red, 2026-11-04): disagreement executed (equality check removed);
capacity executed (combine returns an action); malformed read as none. Removing only the capacity branch
stayed green: capacity still escalates through the disagreement branch, so the property holds structurally.

2026-11-04 (event-driven council): rounds_are_event_driven_and_never_execute_capacity,
a_quorum_runs_without_a_busy_member_and_closes_without_a_silent_one,
attention_merges_unchanged_facts_and_appends_changes, a_legacy_queue_is_seeded_into_the_merged_view.
Red controls (each turned its test red, 2026-11-04): attention merge removed (always append); event gating
removed (a round on every check); quorum replaced by "all members idle"; member timeout replaced by two periods.
