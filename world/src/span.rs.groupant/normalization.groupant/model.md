# answer-span-normalization

## model

# answer-span-normalization

Subject: world/src/span.rs, whole-file SHA-256 in manifest, baseline 4279da541. Domain: finite acyclic valid trees with consistent child lists, valid indices and terminating dependencies. Observation: a single call return, not semantic truth or reachability. Hold Tree, ans::lemmas and types::FEELING dependencies fixed; parser/model loading and external caller wiring are gaps. All entries are source-derived partial models. Exact-target definitions are not executable inverses; finite tests cannot establish universal correspondence.

## forward

Sort/dedup, asymmetric edge trim, then content/echo gate; is_pron is a separate predicate.

## invariant

Interior EDGE tokens remain; valid indices required.

## cost

O(m log m+m^2) time, O(m) auxiliary content.

## epoch-memory

Initial source-bound epoch. Preserve Num bypass, global Feel selection, early-return cleanup suppression, and verb_kids comment mismatch: code expands Xcomp only and checks no shared subject. No source changes or task-accuracy claim. Partial syntax has no semantic analysis or macro expansion.

## checks

Structural inventory checks hashes, references, types and node bounds only. Proposed oracle: enumerate small annotated acyclic trees and compare literal expected returns, including None and labels; mutate priority or echo inequality to obtain a red control. Any executed bounded evidence is recorded separately in root ../model.md#evidence; catalog examples remain proposed until recorded there.

## composition

Leaf scope shares immutable tree/question/helper contracts. Overlap with parent is not extra runtime work. Dependencies outside range remain opaque.

## behavior-complement

U=(Tree,valid indices,qlem) x optional indices; R=clean. Inner U minus R excludes success unsorted/duplicate, content-free, majority-echo, leading EDGE/Cc/Punct/Mark or trailing EDGE/Cc/Punct. Non-EDGE trailing Mark is not excluded.

Assumptions/evidence/status: common domain above; source-derived partial relation at bound revision. Approximation and held-fixed inputs are stated in the relation. Oracle: finite literal fixtures exercising both membership and nonmembership against the actual function, preserving ambiguous preimages. No linguistic correctness or universal proof inferred.

## reverse-reconstruction

For J retain all index lists whose sorting/dedup and edge trim gives J and echo gate passes. Duplicates/permutations/trimmed edges arbitrary within that constraint. None fibers include no-content and majority-echo candidates.

Assumptions/evidence/status: common domain above; source-derived partial relation at bound revision. Approximation and held-fixed inputs are stated in the relation. Oracle: finite literal fixtures exercising both membership and nonmembership against the actual function, preserving ambiguous preimages. No linguistic correctness or universal proof inferred.

## echo-threshold

For c>0 content and e echoed content, success iff 2e<=c; equality c=2,e=1 passes. c=0 separately fails. Adding qlem membership can cross threshold; echo==c test is redundant when c>0.

Assumptions/evidence/status: common domain above; source-derived partial relation at bound revision. Approximation and held-fixed inputs are stated in the relation. Oracle: finite literal fixtures exercising both membership and nonmembership against the actual function, preserving ambiguous preimages. No linguistic correctness or universal proof inferred.

## pronoun-vacuity

is_pron is universal PRON/DET/PUNCT membership, true for empty indices. It is separate from clean: mixed PRON+NOUN is not pronoun-only but may fail echo. Selected subject/object/speech gates use it before cleanup.

Assumptions/evidence/status: common domain above; source-derived partial relation at bound revision. Approximation and held-fixed inputs are stated in the relation. Oracle: finite literal fixtures exercising both membership and nonmembership against the actual function, preserving ambiguous preimages. No linguistic correctness or universal proof inferred.

## wave6

# Native selection observations

Revision: exact source SHA in this manifest. Observations are single terminating calls. Tree fixtures are directly annotated, acyclic and index-valid; dependencies are compiled from this checkout. No parser or teacher is loaded. Metric is exact equality of optional indices/text and rule labels; zero tolerance. Exhaustive claims apply only to explicitly bounded domains, with no semantic accuracy or general Rust proof.

## finite-list-fibers

Fix tree [and/Cc, ruby/NOUN, jade/NOUN, ./Punct]. Domain D is every index list of length 0..4 over {0,1,2,3}, crossed with four question-membership masks over ruby/jade: 1364 calls. Let S be ascending unique input indices intersected with {1,2}. A(S,q) is Some(S) for S=[1] with ruby absent, S=[2] with jade absent, or S=[1,2] unless both present; otherwise None. For every output y, B_q(y)={L in D_lists:A(L,q)=y}; complete enumeration compares every member of each production fiber with this independently specified truth table, including None. Ambiguous duplicate/permuted/edge antecedents remain. Test cleanup_complete_bounded_fibers supplies exact bounded correspondence, not an unbounded reconstruction algorithm. Cost: 341 list evaluations per mask, maximum four indices each.

## marker-boundary-intervention

Fix q empty and tree [to/Mark, ruby/NOUN, and/Cc, jade/NOUN, to/Mark]. For all-index candidate plus duplicate ruby, normalization returns [1,2,3,4]: leading non-EDGE Mark disappears, trailing non-EDGE Mark and internal Cc survive. Subdomains [0,1] and [1,4] yield [1] and [1,4]. Model forbids symmetric Mark stripping on these calls (inner complement) and retains the original indices as compatible antecedents, without claiming these three lists exhaust the full fiber. Test cleanup_marker_asymmetry_and_internal_edges observes indices, so repeated spelling cannot hide boundary errors.

## History

### 2026-10-29 · epoch-memory

2026-10-29: initial source-bound epoch. Preserve Num bypass, global Feel selection, early-return cleanup suppression, and verb_kids comment mismatch: code expands Xcomp only and checks no shared subject. No source changes or task-accuracy claim. Partial syntax has no semantic analysis or macro expansion.

### 2026-10-30 · document

Heading `# Native selection observations, 2026-10-30` stays in document as `# Native selection observations`.
