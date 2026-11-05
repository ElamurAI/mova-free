# Coordinate projection and replacement

## forward

Get projects one of six categorical coordinates; Set replaces exactly that coordinate at c when 0<=c<n_f. For c>=n_f it panics before mutation, including Boolean fields. Means is not a learned field. classes gives the exact table order used by eval::labels_of and classifier class positions.

## behavior-complement

Over (f,L,c) paired with final Labels or panic, exclude normal return for c>=n_f; exclude outputs changing any other coordinate or means. For valid c exclude outputs with Get(f,L') != c. Invalid calls are programmer errors rather than new Result-returning API.

## reverse-reconstruction

Given f,c,L' after valid Set, compatible L retain every other coordinate/means from L' but the previous f coordinate ranges over n_f possibilities. Observing only Get additionally loses all other fields and arbitrary means. No unique previous state is recovered.

## invariant

Distinct-field valid setters commute and preserve means; same-field setters keep the last class. Public signatures and valid behavior stay unchanged. eval callers require classifier class<n_f; this source does not validate model training.

## coupling

Distinct-field valid setters commute and preserve means; same-field setters keep the last class. Public signatures and valid behavior stay unchanged. eval callers require classifier class<n_f; this source does not validate model training.

## composition

The closed table, coordinate and codec fragments share exact enums and field order. Classifier indices require the corresponding classes list; schema is not directly used by infer.rs. Agree uses string tables and does not validate this schema. Native tests exercise schema operations; caller inspection is not an executed cross-file join.

## cost

O(1) get/set, no allocations. Boolean guard adds one bounded comparison; classes allocates k references.

## checks

Six wave13_models tests plus one prior schema test run twice per debug/release profile; ../model.md#evidence at the file root binds receipts. Literal independent tables cover 4608 categorical rows, 34 valid coordinate updates, 12 invalid boundaries, 127 first-error masks, five word counts and Unicode whitespace fibers. JSON coverage executes Form variants and default Labels; arbitrary serde error cases are untested.

## epoch-memory

Prospective model recorded before edits. Old Boolean class 2 silently overwrote true with false; invalid_class_panics_before_mutation failed. The narrow guard preserves unit return and valid indices, consistently panicking before mutation. The prospective note estimated 36 valid classes; actual table sum is 34. The word-limit > to >= production mutant compiles and is rejected by rejection_order_and_word_boundary, then exact source is restored. No escaping/API redesign was introduced: noncanonical means remains an explicit transport limit. Syntax generation repeats exactly; macro bodies remain opaque, although methods are exercised natively. No live parser, teacher, corpus, network, GPU, training, filesystem-durability or full-system claim.

## History

### 2026-10-30 · epoch-memory

2026-10-30: prospective model recorded before edits. Old Boolean class 2 silently overwrote true with false; invalid_class_panics_before_mutation failed. The narrow guard preserves unit return and valid indices, consistently panicking before mutation. The prospective note estimated 36 valid classes; actual table sum is 34. The word-limit > to >= production mutant compiles and is rejected by rejection_order_and_word_boundary, then exact source is restored. No escaping/API redesign was introduced: noncanonical means remains an explicit transport limit. Syntax generation repeats exactly; macro bodies remain opaque, although methods are exercised natively. No live parser, teacher, corpus, network, GPU, training, filesystem-durability or full-system claim.
