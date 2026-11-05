# Numeric token surfaces

## forward
Scope: lemma and num. Lowercase lemma; remove commas and dollar signs from form, parse f64, admit integral values, cast to i64. POS admission occurs in the reading child, not here.

## invariant
For small exactly representable finite integers, normalized decimal values are preserved. Fractional values and NaN/infinity fail the fractional-part predicate. This is not a general exact-i64 parser.

## cost
O(form bytes) allocation/filter plus floating parser; lowercasing scales with Unicode output bytes. No performance measurement.

## behavior-complement
A finite noninteger value cannot become Some through this predicate. Numeric-looking lemma with a nonnumeric form is not enough. Misplaced commas or dollar signs are not rejected structurally.

## reverse-reconstruction
1200 admits 1,200, $1200, 12e2 and 1200.0 among many surfaces. Casting also loses precision outside small exact values; no unique surface recovery is possible.

## boundary
f64 parsing plus saturating cast differs from checked integer parsing near i64 limits. The oracle domain is deliberately small; malformed currency syntax is not validated.

## composition
Reading subsequently filters POS, attaches noun and verb and adds context. A valid numeral alone says nothing about the counted unit or holder.

## checks
integral_number_surface_fibers exercises four 1200 forms, negative integer, fraction, NaN, infinity and word rejection. It calls real num without annotation.

## epoch-memory

Preserve existing float semantics rather than invent an exact-number claim. Partial syntax projection only; approximation error metric for semantic interpretation absent.

## History

### 2026-10-30 · epoch-memory

2026-10-30: preserve existing float semantics rather than invent an exact-number claim. Partial syntax projection only; approximation error metric for semantic interpretation absent.
