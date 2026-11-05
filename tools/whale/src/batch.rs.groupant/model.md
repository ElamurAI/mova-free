# mova-whale-batch

## Descriptant

A pod of Whales over papers with rolling assignment: each Whale works on its own paper and takes the next one as soon as its paper is final (passed, asked out, or its own deadline, half a day); a learner that ended unfinished keeps its paper and continues after a backoff; a usage limit pauses the whole pod; papers chosen deterministically from a pinned catalog (open licence, Markdown text, not local-only; priority ids, papers our code cites, then core and NLP topics by citations); one runner per run root; review-only teaching candidates for the papers school and the dictionary of scientific writing. Source: `tools/whale/src/batch.rs`.

## Forward approximant

plan_at(config, busy, assign, wait) at time t: for each open batch (opened without closed) with every assignment final — final_reason(status, deadline, t) = passed | EXHAUSTED reason of this assignment or under the current writer | deadline (stopped on it, or t >= deadline); busy Whales never final — append closed {all_passed | deadline if any outcome is deadline | exhausted, learners with outcome, retry = unpassed with reason, words, writer} and a `closed` line per Whale journal. Open assignment of a Whale = its one non-final assignment; start_at = t if never ended unfinished or ended by `stop`, else last_end + wait, last_end = later of the Whale journal's `ended` for that batch and a status of that assignment. pause_until = max over open, not busy assignments with last reason `limit` of end + wait. If assign and t >= pause_until: free Whales in slot order take their own retry (writer differs, text hash unchanged, not refused) first, then candidates(Pick, taken = every opened paper ∪ refused, rest); one opened event per assignment (number = count(opened) + 1, deadline t + hours, `rolling`), learn.json, `assigned` journal line, optional topic corpus build. roll(config, Roller): hold plan.lock for the runner's life; loop { plan_at(busy = running, assign = !STOP && !dry); start every non-running Whale with start_at <= t unless STOP or paused (thread: terminal(lc) then learn(lc, terminal); panics and errors become reason error/panicked); return stopped (STOP, none running), no_papers_left (none running, none held, no paper), idle (until_idle); wait one message or tick; on a message: `ended` journal line, collect(batch), dry = false }. once(config): plan, every held assignment's learner with NoTerminal in parallel, collect per batch, plan.

## Invariant

A paper is assigned at most once over all batches; a Whale holds at most one unfinished assignment (two is an error); a Whale is assigned only by the single planner holding plan.lock and only when it holds no unfinished assignment, so it never takes a paper another Whale holds or has had; an unfinished assignment ends only by a final reason (passed, asked out, deadline); during a pause no Whale is assigned or started; batches.jsonl and the Whale journals are append-only and fsynced; no terminal is shared; candidates come only from open-licensed papers and contain no terminal answer text; deterministic choice and numbering for a fixed catalog, cited set, priority and event log.

## Cost

A plan reads batches.jsonl, the Whale journals and one status.json per open assignment (milliseconds); assigning hashes and scans the catalog (338 MB, about 1.4 s) and reads the next candidates' texts for the length and extraction checks (refusals recorded once; 4.6 s on a copy of the live root). The runner plans once per learner end and per 10-second tick; with no paper left it does not scan again until a learner ends or a batch closes. Running costs the learners' cost in parallel; idle time per Whale between papers is one plan instead of the rest of the slowest Whale's batch.

## Composition

Parent: whale-workers; uses learn.rs (run, paper_status, EXHAUSTED, stop_file), knowledge.rs, terminal.rs (MovaTerminal, NoTerminal), paper.rs (open_licence, PaperSource), topic.rs; CLI in main.rs (batch-plan, batch); read by epoch.rs (documents from batches), whale-world export (passed papers) and the private Owls report.

## behavior_complement

Not done: starting terminals or Crab workers; clearing a question pending in a terminal (a terminal that keeps failing fails every backoff until the paper's deadline); interrupting a running learner except before its next round (`STOP`); moving a paper to another Whale or taking one another Whale holds or had; retrying a paper twice under the same writer; admitting candidates (a reviewer does); stopping an old one-batch-at-a-time runner (it holds no lifetime lock; a running one fails at plan.lock while the new runner holds it). Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

batches.jsonl gives every assignment with its text hash, opening time, deadline and how its batch closed with each learner's status, outcome and retry record; whales/<whale>.jsonl gives each Whale's assigned, started, ended (reason, rounds, words) and closed lines in order, so its timeline (idle gaps, backoffs, pauses) is recoverable; the learner configs and the Whales' state reconstruct each paper's rounds. Pause and backoff times are recomputed from these files, not stored.

## History

### 2026-11-04 · Cost

Opening a batch hashes and scans the catalog (338 MB, about 1.4 s, 2026-11-04) and hashes the chosen texts; running costs the learners' cost in parallel.

### 2026-11-04 · document

#### Epoch 2026-11-04 (keeping the pod busy)

Observed 2026-11-04: all five learners of batch 1 stopped `nothing_to_ask` within 40 minutes of opening, and the one-shot `batch` exited with the batch open until its 12-hour deadline (Crab worker `exited`, five idle learners). Chosen behaviour: a batch whose learners are all passed or asked out (`nothing_to_ask`, `round_limit`) closes at once as `exhausted`, recording every unpassed paper in `retry` with its learner, last words and the writer of its last round; the next batch opens immediately. A retry is assigned first, to the learner holding its knowledge, when the current `compose::WRITER` differs from the recorded one (old records without a writer count as different); under the same writer it is not retried again, so the catalog keeps moving. Learners stopped by a usage limit, timeout or terminal failure keep the batch open; the persistent `run` waits `wait_minutes` (30) and continues them (new questions only) until the deadline. `STOP` in the run root ends the loop; `--no-terminal` makes one pass. Rejected alternative: waiting for the deadline idles the pod for up to 11 hours per asked-out batch.

#### Epoch 2026-11-04 (Whale rename)

The engine was renamed from Sharks to Whales; a deployment renamed its learners (`shark-N` → `owl-N`) while batch 1 was open. Model: a slot S answers to the label set L(S) = {S.whale} ∪ S.formerly; an assignment, retry record or status with label l maps to the unique slot with l ∈ L(S) (validate keeps all labels, state roots and terminal identities pairwise distinct, so the map is a function). Records written before the rename carry the field `shark`; `util::legacy` reads `whale`, then `shark`. New records name the slot's current label. Knowledge stays keyed by `state_root`, not by label, so a rename with an unchanged `state_root` keeps it. Evidence: `batch::tests::renamed_learners_keep_their_open_batch_retries_and_state` (old field names and labels open a batch; new labels without `formerly` are refused — negative control; with `formerly` the open batch continues under the old state roots and both unpassed papers are retried with the renamed learners; a label both current and former is refused). Removing `formerly` matching or the legacy field reader turns it red (checked 2026-11-04). Format identifiers `mova-shark-*-v1` are unchanged.

#### Epoch 2026-11-04 (selection, extraction, pod, topic corpus)

`Selection`: priority, cited by our code, core topics, general NLP; by library then overall citations; never alphabetical; min_words of read prose; little-cited system descriptions skipped; refusals (`extraction`, `short`) recorded once, retries checked too. Learner configs carry pod, stall_rounds, batched, review, topic_corpus; plan builds the topic corpus on assignment. On a copy of the live run root the plan took peters-2018, linzen-2016, press-2023, kiperwasser-2016, marneffe-2019 instead of «a-2025…» workshop papers.

### 2026-11-05 · document

#### Epoch 2026-11-05 (selection shared)

`candidates` takes a `Pick` view (catalog, cited_dir, include_uncited, priority, exclude, extraction, selection) and is public; `BatchConfig::pick()` builds it, so a batch and an epoch (`epoch.rs`, documents `from: catalog`) select papers by the same rules. Behaviour of batches unchanged (batch::tests pass).

#### Epoch 2026-11-05 (rolling assignment)

Observed 2026-11-05: batch 32 opened 13:07 CET; four Owls passed in their first round within a minute; owl-1's terminal failed at 15:38 («Selected model is at capacity», no replay), so four Owls sat idle until the batch's 12-hour deadline. Chosen: rolling assignment. Each assignment is final by final_reason; a free Whale takes its next paper at once (own retry under a changed writer, else the selection), one batch per assignment, so the event format and its readers stay unchanged. Unfinished learners continue after wait_minutes within their paper's own deadline, then the paper closes as `deadline`; a `limit` pauses every Whale for wait_minutes. Rejected: a shared deadline per group (idles the pod), moving a failed paper to another Whale (its knowledge and asked questions live in the first Whale's state), retrying at once (fail-fast; the terminal needs time). Prediction before measuring: per-Whale idle time between papers falls from up to 12 h to one plan (seconds). Checked on two copies of the live run root (2026-11-05, release build, en.bin 2483516a…): batch-plan kept batch 32 open with owl-1 on mendelsohn-2021 (next start 30 min after its last failure) and gave owl-2…owl-5 papers 33–36 at once (vylomova-2016, madabushi-2021, reddy-2017, khondaker-2023; 5 short papers refused), 4.6 s; both copies chose identically. Tests: batch::tests rolling cases with a scripted learner and with the real learner plus a fake terminal; mutants red then restored: a terminal failure counted as final, no pause, no backoff, assignment only when the whole pod is free.

### 2026-11-05 · school data outside the repository

The school candidates' target label is school/courses/papers/ (the school data) instead of world/data/school/papers/.
