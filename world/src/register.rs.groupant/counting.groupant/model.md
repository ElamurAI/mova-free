# corpus-register-counting

## descriptant

# corpus-register-counting

counts and count_body, with events::book_body dependency.

## Forward

Fold readable bodies into additive lowercase histograms and totals. Alphabetic token boundaries precede byte-length admission.

## Invariant

For count_body, the total increment equals the sum of histogram increments without overflow. counts starts from zero, so returned total equals sum counts. Each increment represents one original alphabetic run with UTF-8 byte length > 1, keyed by its Unicode lowercase result. Admission precedes lowercase; combining characters may appear afterward. Token order is discarded.

## Cost

O(bytes+tokens log V), storage O(V+largest file).

## Composition

This fragment has no children. counts reads each path, applies events::book_body and accumulates count_body deltas. Failed reads add nothing. The root passes maps/totals to ranking; this fragment computes no z score or row order.

## Epoch memory

## Gaps

Syntax is source-bound and partial: macro expansion, names/types, cfg evaluation and external modules absent. Native fixtures exercise pure helpers and body slicing. Filesystem errors, invalid UTF-8 reads, formatted nonfinite rates and overflow remain unexecuted; large-count behavior has one targeted witness, not exhaustive coverage. No parser/annotator, sentence-splitting, live corpus, teacher or GPU execution is involved.

## catalog

# corpus-register-counting typed models

## nonbehavior

Type: behavior_complement.

U=(body,delta,total). Excludes digit-bearing admitted token and total unequal to sum deltas. Oracle native Unicode/count fixture.

## antecedents

Type: reverse_reconstruction.

Histogram admits all token sequences whose lowercase classes have exact multiplicities, plus discarded singleton ASCII and separators that do not merge tokens. Oracle proposed permutations.

## byte-length

Type: admission_partition.

Single multibyte alphabetic scalars pass; ASCII singleton letters fail. Lowercase expansion happens after test. Oracle é/É/İ/A fixture.

## additivity

Type: conservation_relation.

Separate bodies A then B give delta(A)+delta(B). Byte concatenation needs separator or a token can merge. Oracle native two-body fold.

## read-skip

Type: failure_projection.

Unreadable/non-UTF-8 path contributes zero, indistinguishable from empty body in count output. Attempted/read book counts unrecoverable. Oracle source-only.

## History

### 2026-10-30 · Epoch memory

2026-10-30. Native exact histograms confirm ASCII singleton exclusion, multibyte-letter admission and lowercase expansion. A second fixture checks marker stripping and separate-body addition. Review removed inherited event-role and ranking invariants. No executable approximation/error-cap claim.

### 2026-10-30 · document

Subject: source hash in manifest, epoch 2026-10-30. Scope: counts and count_body, with events::book_body dependency.. Domain: terminating Rust calls without integer overflow, narrowed below. Models are partial source-derived relations, not proofs. Complement is an inner bound in the stated input/output universe. Reconstruction retains all compatible antecedents, not a unique inverse. Oracle status is explicit below; ../model.md#tests records executed coverage.
