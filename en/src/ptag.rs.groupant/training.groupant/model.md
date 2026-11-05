# en-ptag-training

## descriptant

### en-ptag-training
Context lengths are checked before filtering tagged sentences. Frequent-word bypass excludes update-clock ticks; sparse cell totals integrate elapsed ticks, then final averaging divides by max(now,1). Robust wrong counts and environment influence suppressed updates.

## approximant

### Scoped forward relation
Context lengths are checked before filtering tagged sentences. Frequent-word bypass excludes update-clock ticks; sparse cell totals integrate elapsed ticks, then final averaging divides by max(now,1). Robust wrong counts and environment influence suppressed updates.
No executable percentage approximant attached.

## invariant

### Scoped assumptions and costs
Finite beam not global optimum; empty words empty; no supported candidates can panic at history lookup; epochs0 may mark every token suspect.
Inference O(n*b*(F+NT log NT)+n*b^2), histories O(n*b^2); training epochs*tokens*features; u32 clock unchecked.

## composition

### Parent composition
Context lengths are checked before filtering tagged sentences. Frequent-word bypass excludes update-clock ticks; sparse cell totals integrate elapsed ticks, then final averaging divides by max(now,1). Robust wrong counts and environment influence suppressed updates.
Parent combines this fragment with the other named source fragments and helpers outside their ranges. No coverage claim for unscoped code.

## models

## behavior_complement
Within this fragment's byte scope: Context lengths are checked before filtering tagged sentences. Frequent-word bypass excludes update-clock ticks; sparse cell totals integrate elapsed ticks, then final averaging divides by max(now,1). Robust wrong counts and environment influence suppressed updates.
Parent relation constraining compatible states: Finite beam not global optimum; empty words empty; no supported candidates can panic at history lookup; epochs0 may mark every token suspect.

## reverse_reconstruction
Within this fragment's byte scope: Context lengths are checked before filtering tagged sentences. Frequent-word bypass excludes update-clock ticks; sparse cell totals integrate elapsed ticks, then final averaging divides by max(now,1). Robust wrong counts and environment influence suppressed updates.
Parent relation constraining compatible states: Many weights/training sets/hash collisions share outputs; forced tags constrain outputs without identifying weights; averaging discards update history.

## forced_priority
Within this fragment's byte scope: Context lengths are checked before filtering tagged sentences. Frequent-word bypass excludes update-clock ticks; sparse cell totals integrate elapsed ticks, then final averaging divides by max(now,1). Robust wrong counts and environment influence suppressed updates.
Parent relation constraining compatible states: forced.get(i) tag precedes frequent known and may be unseen; missing forced entries fall back normally.

## averaging_clock
Within this fragment's byte scope: Context lengths are checked before filtering tagged sentences. Frequent-word bypass excludes update-clock ticks; sparse cell totals integrate elapsed ticks, then final averaging divides by max(now,1). Robust wrong counts and environment influence suppressed updates.
Parent relation constraining compatible states: now increments only non-known token decisions; averaged cell=(total+(now-stamp)*w)/max(now,1).

## beam_merge
Within this fragment's byte scope: Context lengths are checked before filtering tagged sentences. Frequent-word bypass excludes update-clock ticks; sparse cell totals integrate elapsed ticks, then final averaging divides by max(now,1). Robust wrong counts and environment influence suppressed updates.
Parent relation constraining compatible states: Stable descending scores choose earlier tag on ties; dedup retains first last-two-tag state; beam.max(1) excludes width0 emptiness.

## robust_environment
Within this fragment's byte scope: Context lengths are checked before filtering tagged sentences. Frequent-word bypass excludes update-clock ticks; sparse cell totals integrate elapsed ticks, then final averaging divides by max(now,1). Robust wrong counts and environment influence suppressed updates.
Parent relation constraining compatible states: EN_ROBUST_FROM defaults3; context seeded RNG environment delegated to ctx; wrong u8 saturates and can no longer equal epoch+1 past255.


## tests

### Bounded parent evidence
See ../model.md#tests. Actual check: ptag::wave27_tests::forced_tags_override_seen_and_zero_beam; shared source evidence, not independent exhaustive fragment testing.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: source-scoped partial model; parent tests provide finite shared evidence.

### 2026-11-04 · descriptant

2026-11-04 epoch (en-morph): source rebound to e7ca7a999b6b; byte range 8385..14201 → 8977..14793. The scope text is unchanged (moved only).
