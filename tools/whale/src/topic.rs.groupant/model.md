# mova-whale-topic

## Descriptant

The topic corpus in the learning loop: runs the frozen `mova-topic-corpus` binary (which depends on this crate, so it is a child process, never linked) to build an assigned paper's closed corpus and world, seed term facts into the Whale's knowledge before each round's questions, and write each round's cited description with a bibliography. Off unless configured. Source: `tools/whale/src/topic.rs`.

## Forward approximant

run(cmd, args): spawn EXECUTABLE cmd CONFIG args, drain stdout/stderr on threads, kill after timeout_seconds; {ok, ms, result JSON | exit + stderr tail | timeout}. build(paper) = build, then world when build succeeded. seed(paper, whale, state_root). cite(paper, round_dir, state_root, whale, out [--public]).

## Invariant

A child's failure or timeout is returned as a value and never stops learning; nothing is retried; the learner releases learner.lock only for the seed and stops (`lock_lost`) when it cannot take it back; paths are absolute.

## Cost

One child per call; build and world per assigned paper (minutes for a large neighbourhood); seed and cite per round (seconds each, loading the library and corpus).

## Composition

Parent: whale-workers. Used by batch.rs (plan: build on assignment, `topic_corpus` events) and learn.rs (seed at round start, cite after every round's files). The topic-corpus mechanism itself is tools/topic-corpus.

## behavior_complement

Not done: downloads, web pages, Sol drafts (manual steps of the topic-corpus README), checking the child's output beyond JSON parsing, caching between rounds. `cite` resolves pod items (`pod:…`) as the paper itself (it knows terminal and topic-corpus provenance only). Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

The journal holds each child's command, status, timing and result; the corpus and cited.md are on disk.

## History

### 2026-11-04 · document

#### Epoch 2026-11-04 (created)

Tested with a fake binary (a shell script whose seed refuses a held learner.lock); no live corpus run from the loop yet.
