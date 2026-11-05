# mova-whale-answer

## Descriptant

The Whales' English: deterministic sentences for describe, count, status (service and source), show and pause/resume, over mova_octopus::realize. Source: `tools/whale/src/answer.rs`.

## Forward approximant

Structured results (Outcome list, count/status/show/marker JSON) -> sentences with number agreement (noun_count, be), lists, could_not, utc/thousands; opening(text, max) drops Markdown headings and cuts at a sentence end.

## Invariant

Each sentence states only what its input says; unknown fields fall back to neutral words, never invented values.

## Cost

Linear in the result size.

## Composition

Parent: whale-workers; called by animal.rs Hands.

## behavior_complement

Not done: summarizing a description (show quotes its opening), languages other than English, paraphrase variety. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

From an answer one recovers the counts, ids, titles, states and paths it names; the structured JSON alongside it is the authority. Several inputs can give the same sentence (e.g. missing vs zero counts). No unique inverse is claimed.
