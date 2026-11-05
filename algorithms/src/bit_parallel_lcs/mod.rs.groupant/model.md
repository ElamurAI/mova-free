# algorithm-bit-parallel-lcs

## descriptant

### Description and epoch memory

Prospective cost: O(256+n+m) time and 256 u64 masks plus constant state, no heap allocation. Shorter input is at most 64; longer side has no arbitrary cap. No timings or speed claim. Retained observation: unsigned borrow must wrap. Empty and arbitrary byte inputs remain lawful. Classical reference in mod.rs: Allison and Dix (1986) for bit-string LCS. No copied implementation.

Call-level observation is an exact scalar/typed error. This source-derived specification is checked finitely, not formally proved. Allocator failure, process interruption and timing are outside the model.

Partial syntax excludes macro expansion and semantic resolution. Structural checks and finite fixtures do not establish a general proof. No approximant-error cap or automatic substitution is claimed.

## approximant

### Forward

Select shorter s and longer l, choosing first on ties. If |s|>64 return ShortSideTooLong{|s|}; otherwise return the exact maximum length of any shared byte subsequence. Empty slices return zero.

### Invariant

After a prefix of l, bit j is LCS(s[..j+1],prefix)-LCS(s[..j],prefix). Popcount gives the final row score. For byte mask M, x=M|state, y=(state<<1)|1; update state=x & !(x-y) with unsigned modulo-2^64 subtraction. x excludes excess high bits. Masks shift by 0..63; state shifts by one.

### Cost

O(256+n+m) time and 256 u64 masks plus constant state, no heap allocation. Shorter input is at most 64; longer side has no arbitrary cap.

Call-level observation is an exact scalar/typed error. This source-derived specification is checked finitely, not formally proved. Allocator failure, process interruption and timing are outside the model.

## models

# Typed models

Every view binds the source revision in manifest.json. Assumption: ordinary completed calls. Call-level observation is an exact scalar/typed error. This source-derived specification is checked finitely, not formally proved. Allocator failure, process interruption and timing are outside the model.

## behavior_complement

Universe: finite byte pairs and Result<usize,ShortSideTooLong>. Exclude success iff shorter length>64; otherwise exclude errors and every returned q differing from the maximum shared-subsequence length. In particular q cannot exceed either length. API supplies no subsequence witness. Status: concrete source-derived complement specification, with finite correspondence evidence only.

## reverse_reconstruction

Ok(q) retains every pair with min length<=64 and LCS=q. ab/ac and xy/xz both map to one. Zero admits empty slices and disjoint nonempty alphabets. Error r retains every pair with min length=r>64. Neither bytes, matching positions, lengths nor the choice of an optimal subsequence is uniquely recoverable. No complete executable reconstructor exists beyond finite oracle enumeration.

## boundary_partition

At x=0,y=1 wrapping borrow produces u64::MAX and the update restores zero. A full matching word sets bit63 safely. abc/axbyc scores3, while ab/ba scores1, distinguishing subsequence from substring or unordered matching. UTF-8 é/è share one byte and score1. Finite oracle: the named boundary, resource and exhaustive tests. This specialization is a partial source-derived relation.

## orientation_quotient

Swapping a,b preserves both result variants: shorter length r and LCS are symmetric. The implementation chooses the first slice as mask side only on equal lengths; that internal tie choice is erased by the scalar observation. For r<=64 the swapped pair returns the same exact score; for r>64 it returns the identical ShortSideTooLong{length:r}. The quotient fiber contains both orientations, which coincide only for equal input slices; no orientation can be recovered from the output. Domain: finite byte slices, no allocation assumptions required. Source-derived symmetry checked by exhaustive_lcs_against_matrix and length_cap_and_empty_domain.


## bit63_borrow_projection

For a reachable state S and byte mask M, define X=M|S, Y=(S<<1)|1 and S'=X & !(X-Y mod 2^64). Arithmetic observation is the full u64 word before popcount. When X=0 and Y=1, subtraction yields 2^64-1 and S'=0, rejecting a signed/checked-underflow interpretation. For 64 distinct mask bytes, a long prefix matching only the final byte gives S'=2^63 and score one; after all 64 ordered matches S'=2^64-1 and score64. Popcount merges these bit-location distinctions: scores alone cannot reconstruct the matching row increments. Source-derived reachable-state relation; top_bit_borrow_and_full_word supplies all three endpoint witnesses.


## tests

### Native evidence

Four named tests per module, twice debug and twice release with identical normalized outcomes. Independent full DP oracle over 3969 pairs per algorithm, plus byte, empty, threshold, cap and bit63 fixtures. Two compiled production mutants invert substitution equality and remove the bitset complement; named boundary assertions fail. Exact restoration precedes positive runs. Strict all-target Clippy and narrow formatting pass. receipt.json binds source and raw evidence hashes. Partial syntax and structural checks run twice. No live parser, teacher, corpus, GPU or performance run.

#### Wave16 integrated CPU timing

The [bounded timing receipt](../../../../models/algorithm-library.groupant/model.md#wave16-string-timings) records two external executions against the existing exact DP reference, source/binary/log hashes and unchanged outputs. Its fixed synthetic domains do not establish training or whole-system throughput.

## correspondence

# Byte LCS correspondence models

## Admission boundary and error fibers

Type: `failure_partition`. Observe the full result variant and error payload.
For all byte contents the source partitions input lengths exactly as

    r <= 64: B(a,b) = Ok(L_byte(a,b))
    r > 64:  B(a,b) = Err(ShortSideTooLong { length: r }).

Thus observing `Err{length:q}` reconstructs precisely pairs with
`min(|a|,|b|)=q>64`, and no pair for `q<=64`. For fixed `q>64`, the length fiber
is `{(q,t):t>=q} union {(t,q):t>=q}`; byte contents remain unconstrained. A large
answer or one long input does not trigger rejection. For every valid `N>=64`,
lengths `(64,N)` admit success; `(65,N)` reject exactly when `N>=65`.
Swapping the inputs preserves variant and payload.

If one side is empty, zero masks and zero initial state keep every processed
long byte at zero: `x=0`, `y=1`, `x & !(x-y mod 2^64)=0`. The output is `Ok(0)`
for any other length. This implementation still initializes the fixed mask
array and iterates over the long side; it has no source-level empty early
return. Equal empty results across implementations do not imply equal work.
With `r>64`, rejection occurs before mask construction and both processing
loops, independently of whether the bytes would otherwise match.

Finite oracles: [length_cap_and_empty_domain](../tests.rs) checks both
orientations at 65/66, success at 64/100, and empty/100.
`top_bit_borrow_and_full_word` checks score 64 and position 63. The bridge
[unicode_characters_are_not_bytes_and_bitparallel_boundary](../../../../tools/correspondence/tests/lcs_bridges.rs)
compares 64/100 with English LCS and observes 65/100 rejection while generic
LCS still agrees with English LCS. The full length partition and empty-loop
trace are source-reviewed, not exhaustively enumerated.

## Renaming preserves positional masks and reachable states

Type: `state_transport`. Fix input positions and lengths. Let `h` be injective
on the bytes occurring in either input (a permutation of all 256 values is a
sufficient stronger condition). For the chosen short side `s`, define

    M_c = sum over j with s[j]=c of 2^j.

When `r<=64`, the renamed mask lookup at each long-input position is identical:
`M'_{h(long[i])}=M_{long[i]}`. Both runs start at state zero; applying the same
unsigned wrapping recurrence therefore gives the identical full `u64` state
after every corresponding prefix, not merely equal popcounts. Orientation and
64-byte admission are unchanged. At `r>64`, both runs return the same error
before masks exist; no rejected-state trajectory is claimed.

If a map merges distinct occurring bytes it can change a mask and the result:
`[0]/[1]` scores zero but mapping both to zero scores one. Consequently the
transport premise requires equality reflection as well as preservation. Byte
renaming can also produce invalid UTF-8, which remains within this API's domain.
A score does not reveal original byte labels or all positional mask information.

[exhaustive_lcs_against_matrix](../tests.rs) covers bytes 0 and 255 and
`top_bit_borrow_and_full_word` covers 64 distinct byte values and the top state
bit. [subsequence_not_substring_and_fibers](../tests.rs) contains equal-score
renaming witnesses `ab/ac` and `xy/xz`. These check outputs; general renaming
of the internal trajectory is a source-derived induction, not instrumented
state evidence or a claim of universal property testing.

## Accepted extremal fibers and partial histogram exclusion

Type: `reverse_reconstruction`. Hold byte lengths `n,m` fixed with `r<=64`.
The exact successful extreme fibers are

    B^-1(Ok(0)) = {(a,b): support(a) and support(b) are disjoint}
    B^-1(Ok(r)) = {(a,b): shorter is a subsequence of longer}.

With `r=0`, both fibers contain all fixed-length pairs; with equal nonzero
lengths, the upper fiber requires identical byte sequences. At `r>64` every
successful fiber is empty, including `Ok(0)` for disjoint byte alphabets.
A result alone does not determine lengths or positions: `ab/ac` and `xy/xz`
both give `Ok(1)`, while empty-side pairs and disjoint nonempty pairs give
`Ok(0)` on the accepted domain.

For accepted pairs let `H=sum_c min(count_a(c),count_b(c))` over 256 bytes.
Every successful score satisfies `0<=L_byte<=H<=r`. The exclusion `Ok(q)` with
`q>H` is only an inner bound on absent success behavior, not a full complement
or a percentage approximation. For `ab/ba`, `H=2` but the score is one, so the
false output `Ok(2)` survives the bound. Rejection must be modeled separately:
using `H` to predict `Ok(0)` for disjoint 65/65 byte inputs is wrong.

[exhaustive_lcs_against_matrix](../tests.rs) checks independent matrix agreement
and length bounds on 3,969 binary-word pairs. `length_cap_and_empty_domain`
and `subsequence_not_substring_and_fibers` check empty and ambiguous fibers;
`top_bit_borrow_and_full_word` checks an accepted full-length match and a
64-byte distinct reversal with score one. The bridge
[histogram_bound_is_a_partial_excluded_relation_not_the_value](../../../../tools/correspondence/tests/lcs_bridges.rs)
checks that all three finite forward rows avoid its partial exclusion and
retains a non-forward survivor. These are witnesses of the declared target,
not a complete executable preimage generator.

## Byte observation and cross-API bridge domain

Type: `observation_domain`. For identical byte slices with `r<=64`, ordinary
completed calls satisfy the proposed scalar bridge

    B(a,b) = Ok(longest_common_subsequence::length(a,b))
           = Ok(en::text::lcs(a,b)).

The generic algorithm requires `Eq`; the English API accepts `PartialEq`.
Here both operate on the same lawful `u8` equality. Their row schedules and
allocation behavior differ from this fixed-mask routine. The bridge concerns
only scalar observations on the common domain. At `r>64` it cannot unwrap
`B`; the other APIs may return a scalar without this cap. Their allocation and
callback failures do not become `ShortSideTooLong` errors.

A token or character interpretation must be supplied before calling this byte
API. `é/è` shares UTF-8 byte 0xC3 and returns `Ok(1)`, whereas its Unicode scalar
LCS is zero. For `éa/aé`, scalar LCS is one and byte LCS is two. Neither UTF-8
encoding nor decoding preserves this observation unit. The cap counts bytes,
not characters: 33 repeated `é` scalars encode 66 bytes and two such inputs
reject, although character LCS is 33. This latter cap witness is derived from
the source rule, not included in the named fixtures. A one-byte-per-token
embedding preserving and reflecting equality is valid only when at most 256
occurring token classes need distinct labels. No normalization or grapheme
segmentation is supplied.

[subsequence_not_substring_and_fibers](../tests.rs) checks `é/è` directly.
In [lcs_bridges.rs](../../../../tools/correspondence/tests/lcs_bridges.rs),
`unicode_characters_are_not_bytes_and_bitparallel_boundary` checks `éa/aé`
and the numeric cap; `actual_en_algorithm_bitparallel_and_oracle_agree_on_small_byte_domain`
compares all three actual APIs and an independent subsequence oracle on 3,969
binary pairs. `catalogue_forward_rows_bind_actual_outputs` checks seven finite
observations; `common_contract_makes_ordinary_and_algorithm_rows_finite_equal`
reports finite coincidence and rejects incompatible assumptions. These bridge
sources were reviewed for wave37; integrated execution and final cross-source
hashes belong to the root receipt. No whole-domain replacement is certified.

## Validation and remaining limits

Existing source and forward-model bytes are preserved. The wave37 integration
bridge was reviewed but not run in this isolated baseline worktree; the root
integration receipt owns those executions and ancestor bindings. General
alphabet/state transport, trace counts and full fibers remain source-derived.
No corpus, training, live model, GPU, speedup, numerical approximation budget
or complete-system correctness claim follows.

## History

### 2026-10-30 · descriptant

2026-10-30: original implementation and independent matrix oracle. Select shorter s and longer l, choosing first on ties. If |s|>64 return ShortSideTooLong{|s|}; otherwise return the exact maximum length of any shared byte subsequence. Empty slices return zero.

Model refinement, 2026-10-30: two additional scoped relations distinguish observation loss, validation or objective changes using the existing fixtures. This epoch declared five typed views. Source, native checks and partial syntax were unchanged; the added declarations were not new executed tests or a proof.

### 2026-10-31 · descriptant

#### Correspondence refinement, 2026-10-31

Four [source-reviewed relations](model.md#correspondence) refine error and extremal
fibers, injective renaming of positional masks and reachable states, and the
byte-only bridge to generic and English LCS. The 64/65 limit applies to the
shorter byte length. Empty inputs yield zero while this implementation still
scans the long side, unlike an early-return implementation. UTF-8 character
counts and byte counts are different input units. Histogram exclusion is partial
and restricted to accepted calls; it cannot replace the admission rule.

The source reference review removed unrelated band-width/Ukkonen commentary
from this description; the authoritative source cites Allison and Dix. Existing
forward-model bytes, source, syntax and historic receipts remain unchanged.
Existing finite tests and reviewed bridge references constrain these additional
relations; general renaming of every internal state remains source-derived.
No complete reconstructor, universal substitution or percentage surrogate is
claimed.

### 2026-10-31 · document

Source review, 2026-10-31. Subject: [length](../mod.rs), SHA-256
`7cace61982a54fac084c8306581c460a10945a7b2f10f8d43e7478babb1365fc`.
The existing [approximant](model.md#approximant) and [typed models](model.md#models) remain
unchanged. Let `B(a,b)` be the returned `Result`, `r=min(|a|,|b|)`, and
`L_byte(a,b)` the maximum shared byte-subsequence length. The domain is finite
valid byte slices and ordinary completed calls. No Unicode validity is required.
The exact symbolic relations below are source-derived; finite native tests
provide bounded correspondence evidence rather than an all-input proof.

### 2026-10-31 · Validation and remaining limits

Worker validation, 2026-10-31: compile each unchanged algorithm module with
`rustc --edition=2024 --test`, once normally and once with `-O`; run each binary
twice with `--test-threads=1`. All four existing tests per module pass in every
run (eight distinct checks); normalized named outcomes match across repetitions
and profiles. This invokes the actual module and its adjacent test source, not
a copied implementation. No new executable property is inferred from prose.

The existing `mova-modelcheck groupants` binary validates the full unchanged
registry plus these sidecars twice; reports match byte-for-byte, SHA-256
`2ab01879fc6282dab0f5ff3c871b9a8fffbb63d44918a1819bf7616bca1a3c2f`.
A temporary nonexistent H2 selector in the bit-parallel manifest is rejected
with `valid:false`; its report SHA-256 is
`81a00d6ebb1b4a1cec0a073067f7abcae434e583af53df89d290938b3bdf0939`.
The exact positive manifest is restored. This control tests reference checking,
not mathematical adequacy. Raw logs and reports are under
`$MOVA_DATA/runs/octopus-wave37-lcs-groupants-2026-10-31`.

### 2026-11-04 · document

Migration: consolidated into model.md, 2026-11-04, operator decision; fresh hash-bound references to the former files were rebound to the sections below.
