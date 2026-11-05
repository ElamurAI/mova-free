# Closed vocabularies

## forward

For each enum E, fixed ordered table T_E maps variants to lowercase names; parse(s) is Some(T_E inverse(s)) iff s is exactly a table name, without trimming. idx equals table position; Display equals name. Serde derives use Rust variant spellings, with no lowercase rename.

## behavior-complement

Over strings times Option(E), exclude Some(e) whenever s differs from e.name(), and None whenever s is a table name. For the serialization observation exclude lowercase TSV names as accepted JSON enum strings. Macro-generated methods execute in tests despite opaque macro syntax.

## reverse-reconstruction

On accepted exact names the variant is uniquely recovered. The None output has every non-table string as a compatible input (empty, uppercase and padded names included); it has no unique inverse. idx is invertible only within the enum's finite index domain.

## invariant

Case/padding intervention can turn exact parse success into None, while parse_labels trims first. JSON roundtrip uses the derived variant alphabet; it cannot be replaced by the TSV name alphabet.

## coupling

Case/padding intervention can turn exact parse success into None, while parse_labels trims first. JSON roundtrip uses the derived variant alphabet; it cannot be replaced by the TSV name alphabet.

## composition

The closed table, coordinate and codec fragments share exact enums and field order. Classifier indices require the corresponding classes list; schema is not directly used by infer.rs. Agree uses string tables and does not validate this schema. Native tests exercise schema operations; caller inspection is not an executed cross-file join.

## cost

O(length(s)) bounded-name comparison, O(1) enum index/name and fixed table storage; serialization cost includes emitted bytes.

## checks

Six wave13_models tests plus one prior schema test run twice per debug/release profile; ../model.md#evidence at the file root binds receipts. Literal independent tables cover 4608 categorical rows, 34 valid coordinate updates, 12 invalid boundaries, 127 first-error masks, five word counts and Unicode whitespace fibers. JSON coverage executes Form variants and default Labels; arbitrary serde error cases are untested.

## epoch-memory

Prospective model recorded before edits. Old Boolean class 2 silently overwrote true with false; invalid_class_panics_before_mutation failed. The narrow guard preserves unit return and valid indices, consistently panicking before mutation. The prospective note estimated 36 valid classes; actual table sum is 34. The word-limit > to >= production mutant compiles and is rejected by rejection_order_and_word_boundary, then exact source is restored. No escaping/API redesign was introduced: noncanonical means remains an explicit transport limit. Syntax generation repeats exactly; macro bodies remain opaque, although methods are exercised natively. No live parser, teacher, corpus, network, GPU, training, filesystem-durability or full-system claim.

## History

### 2026-10-30 · epoch-memory

2026-10-30: prospective model recorded before edits. Old Boolean class 2 silently overwrote true with false; invalid_class_panics_before_mutation failed. The narrow guard preserves unit return and valid indices, consistently panicking before mutation. The prospective note estimated 36 valid classes; actual table sum is 34. The word-limit > to >= production mutant compiles and is rejected by rejection_order_and_word_boundary, then exact source is restored. No escaping/API redesign was introduced: noncanonical means remains an explicit transport limit. Syntax generation repeats exactly; macro bodies remain opaque, although methods are exercised natively. No live parser, teacher, corpus, network, GPU, training, filesystem-durability or full-system claim.
