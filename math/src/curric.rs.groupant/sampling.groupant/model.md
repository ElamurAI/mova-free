# math-curriculum-sampling

## descriptant

### Source-bound description

Wrapping splitmix of SEED xor FNV(salt:id) determines key; stable sort keys then take k of cloned items.

## approximant

### Forward relation

Wrapping splitmix of SEED xor FNV(salt:id) determines key; stable sort keys then take k of cloned items.

Domain: Any cloneable finite slice; id callback may be effectful and is called once/item.

Executable Rust approximant unsupported: categorical/effect observations have no useful percentage metric yet. Future caps1/2/5/10% only; no automatic substitution.

## invariant

### Invariant and cost

k=0 excludes nonempty output; k>n cannot invent items. Key collision does not deduplicate.

Ordering: Clone/key all items before sorting/take, even k=0.

Cost: O(n log n)+hash bytes+cloning; O(n) keyed values. u64 wrapping explicit.

## composition

### Composition and epoch memory

prep later deduplicates ids after sampling, so sample length not necessarily written row count.

Source scope bytes[1367,2123). No corpus prep/load or held grading; arbitrary malformed numeric normalization not certified. Not every branch in this fragment executed.

Partial syntax and finite tests do not establish all caller assumptions.

## models

## behavior_complement

k=0 excludes nonempty output; k>n cannot invent items. Key collision does not deduplicate.

## reverse_reconstruction

Output prefix admits any compatible tail of keys; equal keys admit original order variations only when output remains consistent.

## failure_precedence

Clone/key all items before sorting/take, even k=0.

## representation_transport

prep later deduplicates ids after sampling, so sample length not necessarily written row count.

## resource_boundary

O(n log n)+hash bytes+cloning; O(n) keyed values. u64 wrapping explicit.

## domain_boundary

Any cloneable finite slice; id callback may be effectful and is called once/item.

## tests

### Evidence

Selected actual-source checks: `cargo test -p math --lib curric::`. Exact commands, numeric statuses, normalized outcomes, compiled assertion controls and restoration receipts are in the external wave28 mathcontrol run.

Source scope bytes[1367,2123). No corpus prep/load or held grading; arbitrary malformed numeric normalization not certified. Not every branch in this fragment executed.

No live teacher/API, trained model, corpus/held-out, GPU, DB or expensive benchmark run. Parent syntax generated twice identically. Scope remains partial.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: prospective model before bounded synthetic tests; production behavior unchanged. Source scope bytes[1367,2123). No corpus prep/load or held grading; arbitrary malformed numeric normalization not certified. Not every branch in this fragment executed.

### 2026-10-31 · tests

Heading `### Evidence, 2026-10-31` stays in tests as `### Evidence`.
