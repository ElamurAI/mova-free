# math-arith-search

## descriptant

Scoped search fragment, source bytes [13770,22775).

quantities -> prepare -> subset search -> predict/gold -> Tree::exact. Float pruning and rounded keys can lose exact solutions; replay certifies selected arithmetic, not completeness. Relevance added after search changes cross-mask choice.

Inherited family observations are parent context; only the explicit byte range belongs to this fragment.

## approximant

prepare truncates extracted quantities then builds ordered pair/relevance features. search enumerates disjoint masks by population count and scores child trees plus crossing pairs. predict adds relevance; learn ticks before prediction and updates mismatches. Exact replay uses big Q and can reject rational zero division.

O(n²F) feature work, conservative O(3^n B² n²) search plus tree cloning and O(2^n B) retained candidates. Requires n<32 for masks; no enforced general resource cap.

Executable approximant gap: no useful percentage surrogate for exact arithmetic and categorical branch contracts. Existing float pruning/tolerances do not establish an approximation guarantee.

## invariant

Generated internal candidates exclude nonfinite, negative and >=1e12 values; leaves bypass that filter. Zero divisors exclude division. Beam truncation and round(value*1e6) dedup exclude valid exact alternatives too; no completeness or natural-language correctness claim.

For value y, antecedents are all texts, environments, weight states and retained trees whose exact replay equals y. Gold acceptance instead uses floating tolerance. Repeated values, skipped quantities and commutative operations prevent unique recovery.

Cost: O(n²F) feature work, conservative O(3^n B² n²) search plus tree cloning and O(2^n B) retained candidates. Requires n<32 for masks; no enforced general resource cap.

Finite fixtures do not prove all-domain correctness or resource behavior.

## models

## behavior_complement

Internal candidates admit finite [0,1e12) values and disjoint nonempty submasks. Dedup excludes later candidates sharing round(value*1e6) as i64 despite potentially distinct exact values. Leaves bypass the internal predicate; completeness is not claimed.

## reverse_reconstruction

For predicted tree t, score s and margin d, compatible states are feature-weight assignments retaining t first and next distinct-value score s-d. Text antecedents must yield its quantities/indices. These observations cannot uniquely recover text, weights or discarded candidates.

## beam_boundary

keep stops after len>=beam, so beam0 retains one available candidate; singletons survive max_leaves0. Popcount-sorted masks and lowest-bit partition determine traversal. Quantity count<32 is assumed.

## tie_policy

Stable descending score sort preserves generation order for ties; NaN compares Equal. Distinct next value uses abs(value-best)>1e-6, not rounded-key equality. No alternative means margin10.

## quantity_coordinates

Pair direction uses token positions; reversed subtraction/division use ps[j][i] and reversed label. Leaf relevance is added after subset beams, affecting final cross-mask ranking without influencing internal candidate pruning.

## learning_state

This fragment reads weights only. Gold selection prioritizes fewer leaves, or more permitted leaves for used-number selection, then score. Its tolerance1e-6*max(abs(gold),1) is not exact equality. Updates lie outside this scope.

## tests

Actual source module filter: arith::. Inspected synthetic module tests only, selected independently; numeric run statuses, sorted named outcomes and compiled controls retained in wave27-math external receipts. No full crate suite, corpus, teacher, GPU or training. See source-test-map.json for exact evidence and limitations.
