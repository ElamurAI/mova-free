# answer-span-predicate

## model

# answer-span-predicate

Subject: world/src/span.rs, whole-file SHA-256 in manifest, baseline 4279da541. Domain: finite acyclic valid trees with consistent child lists, valid indices and terminating dependencies. Observation: a single call return, not semantic truth or reachability. Hold Tree, ans::lemmas and types::FEELING dependencies fixed; parser/model loading and external caller wiring are gaps. All entries are source-derived partial models. Exact-target definitions are not executable inverses; finite tests cannot establish universal correspondence.

## forward

Scan caller predicates then token ranks; return first admissible minimum.

## invariant

Tie keeps earliest token; PROPN has rank 3.

## cost

O(p*n*L) with opaque morphology allocations.

## epoch-memory

Initial source-bound epoch. Preserve Num bypass, global Feel selection, early-return cleanup suppression, and verb_kids comment mismatch: code expands Xcomp only and checks no shared subject. No source changes or task-accuracy claim. Partial syntax has no semantic analysis or macro expansion.

## checks

Structural inventory checks hashes, references, types and node bounds only. Proposed oracle: enumerate small annotated acyclic trees and compare literal expected returns, including None and labels; mutate priority or echo inequality to obtain a red control. Any executed bounded evidence is recorded separately in root ../model.md#evidence; catalog examples remain proposed until recorded there.

## composition

Leaf scope shares immutable tree/question/helper contracts. Overlap with parent is not extra runtime work. Dependencies outside range remain opaque.

## behavior-complement

U=(Tree,preds) x Option<index>; R=find_pred. Inner U minus R excludes out-of-range outputs, rank-3 outputs and later equal-rank hits when earlier hits exist for first admissible predicate.

Assumptions/evidence/status: common domain above; source-derived partial relation at bound revision. Approximation and held-fixed inputs are stated in the relation. Oracle: finite literal fixtures exercising both membership and nonmembership against the actual function, preserving ambiguous preimages. No linguistic correctness or universal proof inferred.

## reverse-reconstruction

For Some(i), retain all lists with a first admissible predicate whose minimum (rank,index) hit is i; preceding entries may match only rank 3. None includes empty lists, no hits and rank-3-only hits. Morphology ambiguity remains.

Assumptions/evidence/status: common domain above; source-derived partial relation at bound revision. Approximation and held-fixed inputs are stated in the relation. Oracle: finite literal fixtures exercising both membership and nonmembership against the actual function, preserving ambiguous preimages. No linguistic correctness or universal proof inferred.

## lexical-rank

Fixed predicate and hit set: choose minimum (rank,index), requiring rank<3. Hit means equal lemma, equal form or ans::lemmas(form) membership. Changing POS can change winner without changing spelling.

Assumptions/evidence/status: common domain above; source-derived partial relation at bound revision. Approximation and held-fixed inputs are stated in the relation. Oracle: finite literal fixtures exercising both membership and nonmembership against the actual function, preserving ambiguous preimages. No linguistic correctness or universal proof inferred.

## suffix-equivalence

Any preds suffix after first accepted predicate is unobserved. Inserting predicates having no admissible matches before it preserves output. This is a sufficient equivalence family, not exhaustive.

Assumptions/evidence/status: common domain above; source-derived partial relation at bound revision. Approximation and held-fixed inputs are stated in the relation. Oracle: finite literal fixtures exercising both membership and nonmembership against the actual function, preserving ambiguous preimages. No linguistic correctness or universal proof inferred.

## wave6

# Native selection observations

Revision: exact source SHA in this manifest. Observations are single terminating calls. Tree fixtures are directly annotated, acyclic and index-valid; dependencies are compiled from this checkout. No parser or teacher is loaded. Metric is exact equality of optional indices/text and rule labels; zero tolerance. Exhaustive claims apply only to explicitly bounded domains, with no semantic accuracy or general Rust proof.

## finite-rank-fibers

Fix three forms/lemmas to ruby, preds=[ruby], rank tags {VERB,ADJ,NOUN,PROPN}. Domain is all 4^3=64 rank triples. A(r)=argmin (r_i,i) among r_i<3, or None if empty. B(j) contains every triple with r_j<3, each earlier rank strictly larger and each later rank at least r_j; B(None)={(3,3,3)}. The native test enumerates entire actual and oracle maps of fibers and compares equality, including earliest ties. This is complete for this finite tag set; AUX and morphology are outside the enumeration. Two additional literal caller-order observations show an earlier predicate noun outranks a later predicate verb. Test predicate_rank_complete_fibers_and_caller_order.

## History

### 2026-10-29 · epoch-memory

2026-10-29: initial source-bound epoch. Preserve Num bypass, global Feel selection, early-return cleanup suppression, and verb_kids comment mismatch: code expands Xcomp only and checks no shared subject. No source changes or task-accuracy claim. Partial syntax has no semantic analysis or macro expansion.

### 2026-10-30 · document

Heading `# Native selection observations, 2026-10-30` stays in document as `# Native selection observations`.
