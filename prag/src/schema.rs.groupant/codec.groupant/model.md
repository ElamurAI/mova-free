# Label parse and row transport

## forward

Parse requires exactly seven columns then validates trimmed values in fixed order. Bits are exactly 0 or 1 after trim. Means is trimmed, nonempty, <=30 split_whitespace words; internal spacing is preserved. labels_tsv writes all fields joined by TAB and does not escape or validate means.

## behavior-complement

Over columns and Result<Labels,String>, exclude success whenever any validation predicate fails; exclude a later-field error when an earlier field is invalid. TSV outputs with escaped or normalized means are absent: writer copies it verbatim. This model does not assert arbitrary labels are safe TSV rows.

## reverse-reconstruction

All edge-whitespace variants of a valid column tuple share one parsed Labels. An error prefix fixes the first invalid position but leaves later columns unrestricted and earlier ones merely valid. Different over-limit texts can share the same word-count error; actual input cannot be recovered from an error.

## invariant

Transport roundtrip is exact for D from the parent. Counterfactual accepting 30 words with >= instead of > rejects the boundary witness. Fields supplied directly can contain TAB/newlines; row-level callers must maintain D. Serde roundtrip has no 30-word gate.

## coupling

Transport roundtrip is exact for D from the parent. Counterfactual accepting 30 words with >= instead of > rejects the boundary witness. Fields supplied directly can contain TAB/newlines; row-level callers must maintain D. Serde roundtrip has no 30-word gate.

## composition

The closed table, coordinate and codec fragments share exact enums and field order. Classifier indices require the corresponding classes list; schema is not directly used by infer.rs. Agree uses string tables and does not validate this schema. Native tests exercise schema operations; caller inspection is not an executed cross-file join.

## cost

O(total inspected bytes) parsing with short-circuit validation; means allocation proportional to its trimmed bytes; O(output bytes) TSV.

## checks

Six wave13_models tests plus one prior schema test run twice per debug/release profile; ../model.md#evidence at the file root binds receipts. Literal independent tables cover 4608 categorical rows, 34 valid coordinate updates, 12 invalid boundaries, 127 first-error masks, five word counts and Unicode whitespace fibers. JSON coverage executes Form variants and default Labels; arbitrary serde error cases are untested.

## epoch-memory

Prospective model recorded before edits. Old Boolean class 2 silently overwrote true with false; invalid_class_panics_before_mutation failed. The narrow guard preserves unit return and valid indices, consistently panicking before mutation. The prospective note estimated 36 valid classes; actual table sum is 34. The word-limit > to >= production mutant compiles and is rejected by rejection_order_and_word_boundary, then exact source is restored. No escaping/API redesign was introduced: noncanonical means remains an explicit transport limit. Syntax generation repeats exactly; macro bodies remain opaque, although methods are exercised natively. No live parser, teacher, corpus, network, GPU, training, filesystem-durability or full-system claim.

## History

### 2026-10-30 · epoch-memory

2026-10-30: prospective model recorded before edits. Old Boolean class 2 silently overwrote true with false; invalid_class_panics_before_mutation failed. The narrow guard preserves unit return and valid indices, consistently panicking before mutation. The prospective note estimated 36 valid classes; actual table sum is 34. The word-limit > to >= production mutant compiles and is rejected by rejection_order_and_word_boundary, then exact source is restored. No escaping/API redesign was introduced: noncanonical means remains an explicit transport limit. Syntax generation repeats exactly; macro bodies remain opaque, although methods are exercised natively. No live parser, teacher, corpus, network, GPU, training, filesystem-durability or full-system claim.
