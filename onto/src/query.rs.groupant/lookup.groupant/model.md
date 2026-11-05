# onto-query-lookup

## descriptant

### onto-query-lookup

Found=merge(exact,lower,capitalized) else first successful plural candidate for nouns

## approximant

### Scoped forward

Found=merge(exact,lower,capitalized) else first successful plural candidate for nouns

#### Domain

This byte-scoped operator uses the parent valid-state domain plus the following local restrictions:

Form name retained from first matching variant

Append only Cid not already in result

ies→y; ves→f,fe; es strip; s strip

Pos.same_word_class handles satellites

No executable percentage surrogate established.

## invariant

### Local observations

Any direct match excludes plural fallback; explicit ID POS mismatch excludes lemma search

Form name retained from first matching variant

Append only Cid not already in result

ies→y; ves→f,fe; es strip; s strip

Pos.same_word_class handles satellites

## cost

### Scoped cost

Fixed spelling variants, linear sense scans with O(S²) uniqueness worst case.

Restriction to this operator's traversed inputs; parent total is not charged to every fragment. No standalone measurement.

## composition

### Parent join

lookup supplies the parent relation: Found=merge(exact,lower,capitalized) else first successful plural candidate for nouns

Output compatibility is conditional on the local domain and earlier-branch exclusions. Other parent branches are outside this fragment.

## models

## behavior_complement

Any direct match excludes plural fallback; explicit ID POS mismatch excludes lemma search

## reverse_reconstruction

Found fiber includes case/underscore spellings and plural spellings whose earlier candidates miss

## form_order

Form name retained from first matching variant

## unique_senses

Append only Cid not already in result

## plural_order

ies→y; ves→f,fe; es strip; s strip

## class_domain

Pos.same_word_class handles satellites


## tests

### Evidence

Parent model.md#tests identifies selected actual-code checks. Fragment is a source-scoped partial model; parent finite fixtures do not establish exhaustive branch coverage or every local equation.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Specialized source fragment bytes [2797,4749); shared parent AST. Excludes unrelated parent operations and added test module.
