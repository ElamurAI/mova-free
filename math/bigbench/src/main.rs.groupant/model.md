# math-bigbench-main

## descriptant

### Source-bound description

best(n,f) returns LAST result and MIN duration across n invocations; n=0 panics. Product tree multiplies1..n for n>=1, splits width>=16. big2048 uses32 wrapping LCG steps, shift right1, set low bit. main runs large factorial/power/GCD/modpow/harmonic comparisons.

## approximant

### Forward relation

best(n,f) returns LAST result and MIN duration across n invocations; n=0 panics. Product tree multiplies1..n for n>=1, splits width>=16. big2048 uses32 wrapping LCG steps, shift right1, set low bit. main runs large factorial/power/GCD/modpow/harmonic comparisons.

Domain: Tests factorial1,2,7,17; harmonic3; pow3(8). fact(0) underflows and is outside domain; lo+hi overflow uncertified.

Executable Rust approximant unsupported: categorical/effect observations have no useful percentage metric yet. Future caps1/2/5/10% only; no automatic substitution.

## invariant

### Invariant and cost

Zero repetitions excludes best output. Main modpow comparison checks only last of100 outputs, not all bases.

Ordering: Elapsed collected before replacing output; cross-library assertions follow timed calls.

Cost: Product tree has O(n) factors with size-dependent multiplication; repeats multiply work. Conversion/GCD delegated; no timing performed this epoch.

## composition

### Composition and epoch memory

dashu/num library values compared via decimal representation, not type conversion.

Large benchmark main and large GCD/modpow unexecuted; no speed claim.

Partial syntax and finite tests do not establish all caller assumptions.

## models

## behavior_complement

Zero repetitions excludes best output. Main modpow comparison checks only last of100 outputs, not all bases.

## reverse_reconstruction

Returned pair admits any n-output sequence ending with result and durations having given minimum, not necessarily timing of returned result. Decimal arithmetic outputs discard computation path.

## failure_precedence

Elapsed collected before replacing output; cross-library assertions follow timed calls.

## representation_transport

dashu/num library values compared via decimal representation, not type conversion.

## resource_boundary

Product tree has O(n) factors with size-dependent multiplication; repeats multiply work. Conversion/GCD delegated; no timing performed this epoch.

## domain_boundary

Tests factorial1,2,7,17; harmonic3; pow3(8). fact(0) underflows and is outside domain; lo+hi overflow uncertified.

## tests

### Evidence

Selected actual-source checks: `cargo test -p bigbench wave28`. Exact commands, numeric statuses, normalized outcomes, compiled assertion controls and restoration receipts are in the external wave28 mathcontrol run.

Large benchmark main and large GCD/modpow unexecuted; no speed claim.

No live teacher/API, trained model, corpus/held-out, GPU, DB or expensive benchmark run. Parent syntax generated twice identically. Scope remains partial.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: prospective model before bounded synthetic tests; production behavior unchanged. Large benchmark main and large GCD/modpow unexecuted; no speed claim.

### 2026-10-31 · tests

Heading `### Evidence, 2026-10-31` stays in tests as `### Evidence`.
