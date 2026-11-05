# world-school-student-state

## model

# world-school-student-state

## forward

Normalize tokens, remove stop words, names and counted things, create every ordered within-sentence pair, sort/deduplicate. TSV decode ignores comments/short rows, defaults malformed numbers to zero, and lets later rows replace current weights; a five-field duplicate leaves any earlier accumulator unchanged. Encode unions w/acc keys, retains all explicit accumulators including zeros, omits current-only zero rows, and fills orphan accumulator keys with zero current rows.

## invariant

For keys without tabs/newlines or a leading '#', encode/decode preserves the selected numeric table and acc.is_empty. Learned states with nonzero current or explicit accumulator rows roundtrip exactly. Arbitrary map shape can canonicalize zero-only current and orphan rows. Tolerant parsing is not validation or provenance authentication.

## cost

Pair construction O(sum sentence_length^2) strings then sorting; dictionary work is compiled lookup. Codecs O(K log K+B) encoding, O(R log K) decoding, O(K) retained maps.

## composition

Selection and update children share precisely these maps/features. Actual load/save call the codecs. Native learned-state roundtrip and both activation regressions run without filesystem access. Root discovery, write_atomic and crash behavior remain opaque.

## epoch-memory

Initial school source-bound epoch. Prospective model: keep O(1) arithmetic/partition cost, widen aggregation scalars, and expose actual codecs without filesystem effects. Old native witnesses found unchecked operation, score, marginal vote, partition and duplicate-vote arithmetic; legacy serialization synthesized zero accumulators. Review found that dropping explicit zero accumulators also changed the prediction table. Both activation directions now have named tests. No speedup, approximate-error profile or general semantic proof is claimed. Unchecked update/world arithmetic retains representability premises.

## checks

cargo test -p world --lib school::wave12_models -- --test-threads=1. Root ../model.md#evidence binds exact source/evidence hashes, repeated debug/release outcomes and a compiled production mutation. Named fixture domains appear in composition. Syntax is a source-bound partial projection; macros remain opaque. No semantic AST or live school execution.

## behavior-complement

Subject/revision: manifest hash and byte scope. Universe: supplied finite inputs, fixed dependencies, successful allocation and the explicit arithmetic premises of this scope. Inner absent-behavior relation: An explicit zero accumulator cannot disappear through encoding; a current-only row cannot gain a synthetic accumulator. Format-safe canonical numeric rows cannot change values. Arbitrary tab/newline or leading '#' keys remain outside exact structural roundtrip. These are source-derived constraints with finite named witnesses where indicated; external behavior remains unknown.

## reverse-reconstruction

Subject/revision: manifest hash. Operator: compatible preimage/converse, holding dependencies and admissibility fixed. All texts yielding the same feature set and TSV spellings yielding the same row maps. Case, number strings, omitted names and overwritten duplicates are lost. Canonical codecs are idempotent; arbitrary public map shape projects rather than uniquely inverts. Partial relational description, not a complete inverse search or a unique causal history.

## coupling

Subject/revision: manifest hash. Constraint relation on stage identities and ordered intermediate states: Selection and update children share precisely these maps/features. Actual load/save call the codecs. Native learned-state roundtrip and both activation regressions run without filesystem access. Root discovery, write_atomic and crash behavior remain opaque. Supplied native joins and source inspection are distinct evidence.

## observation-fiber

Subject/revision: manifest hash. Domain: scoped inputs with the declared observation. The information-loss relation equates inputs whose observation agrees: Empty acc and nonempty all-zero acc differ if w favors a nondefault operation. Row existence is retained information. Orphan acc keys become zero current rows without losing their answer contribution. Finite tests do not establish unrestricted completeness.

## wave34-retained-codec

Current nested child world-school-student-state-retained-codec: Decode lines in order: ignore # and widths below five; parse four current columns with invalid numbers mapped to zero; widths >=9 also replace accumulator columns. A later five-column duplicate replaces w but leaves an earlier acc row. Encode sorted union keys; emit nonzero w or any explicit acc, and only emit accumulator columns when present.

Parent interaction: Student load invokes this decoder; save creates a directory then calls write_atomic with this encoder. The parent selects acc globally when nonempty. Presence is therefore a control bit shared with selection, not merely a numeric zero. Filesystem publication and global paths are unexecuted.

Earlier checks and epoch observations above remain historical, not new execution receipts. See file-root ../model.md#wave34-evidence for current selections.

## wave34-composition

### Wave34 composition

Decode retained-codec rows in order before selection chooses the global accumulator/current table. Five-column duplicates can update current weights without replacing earlier accumulator rows. Existing selection and updates retain their identities. The encoder preserves activation while canonicalizing zero-only/orphan map structure; filesystem load/save failure order is outside supplied-string evidence.

The previous model.md and its composition remain historical epoch memory. Current scoped relations use the manifest source hash; partial syntax and finite helper tests do not prove semantic completeness.

## History

### 2026-10-30 · epoch-memory

2026-10-30 initial school source-bound epoch. Prospective model: keep O(1) arithmetic/partition cost, widen aggregation scalars, and expose actual codecs without filesystem effects. Old native witnesses found unchecked operation, score, marginal vote, partition and duplicate-vote arithmetic; legacy serialization synthesized zero accumulators. Review found that dropping explicit zero accumulators also changed the prediction table. Both activation directions now have named tests. No speedup, approximate-error profile or general semantic proof is claimed. Unchecked update/world arithmetic retains representability premises.
