# world-idioms

## model

# Offline dictionary projections and corpus idiom mining

## forward

Parse dictionary/MWE maps, merge by source priority, aggregate corpus counts, gate and classify, separately aggregate particles, sort detailed rows and emit final BTreeMap. Domain excludes count overflow and conditions on successful I/O and fixed global snapshot.

## invariant

Dictionary bypasses min/min_books but not OBJ count>=3. Corpus-only OBJ rows become candidate; corpus particles remain phrasal-verb. Map keys unique; detailed rows may duplicate a pair across branches.

## cost

B input bytes, U unique pairs, E event rows, R retained rows: O(B+(U+E)log(U+E)+R log R) excluding string lengths/global lookup; memory O(B+U+E+R). No measured speed claim.

## composition

Matrix::global().score uses EXISTING global idioms to suppress OBJ oddness; classification is not pure in supplied TSVs. Event export can overwrite OBJ pair; failures leave partial files. Runtime contextual matching is elsewhere.

## epoch-memory

Predicted first minimum dictionary rank and first MWE row; finite tests confirm. Limitations: substring flags, unconditional first-line discard, duplicate last pair versus summed marginals, absent phrasal book gate and unused k%1 set. Production unchanged.

## checks

Five native tests: 64 rank triples, eight phrase/POS contexts plus header, 80 MWE flag/type/title combinations, reversed order, four pair-key preimages. Full run/exports not executed.

## behavior-complement

Universe: admissible inputs and candidate projected outputs, fixed dependencies. Source-derived inner exclusion: outcomes contradicting the stated invariant/priority/eligibility constraints are outside R. These constraints are partial models of implementation, not a complement of linguistic truth. Finite tests support only their declared domain.

## reverse-reconstruction

B(y)={x in declared domain | F(x)=y}, including latent state and dependency snapshot. Retain all compatible antecedents; rejected rows/tokens and discarded projections create ambiguity. Full text/reasons/values narrow fibers compared with key/status projections. No complete solver, unique recovery or causal inference claimed.
## dictionary-order

Winner is argmin(rank,index) on eligible rows. Equal best rank retains earlier evidence; POS and lowercase/title filters apply before ranking. Source-derived partial relation; checks above and explicit unexecuted gaps delimit evidence.

## mwe-order

First eligible MWE row wins independent of type rank. Cross-source merge keeps Wiktionary idiom; MWE idiom replaces other evidence; nonidiom fills only missing. Source-derived partial relation; checks above and explicit unexecuted gaps delimit evidence.

## phrase-context

Wikt title has 2..4 space-separated words, alphabetic/apostrophe characters and sole nonfiller tail. No lexical verb check or literal/context disambiguation. Source-derived partial relation; checks above and explicit unexecuted gaps delimit evidence.

## licence-eligibility

contains(open) OR contains(copyleft), so reopened passes. Unknown/NC-only fails. Predicate does not certify legal status. Source-derived partial relation; checks above and explicit unexecuted gaps delimit evidence.

## threshold-partition

OBJ count>=3 AND (dictionary OR count>=min AND books>=min_books). Ordered kinds: dictionary idiom; odd light; odd PMI>=3; dictionary light/nonlight; nonodd unconfirmed PMI>=5. Matrix None is nonodd. Source-derived partial relation; checks above and explicit unexecuted gaps delimit evidence.

## count-projection

Duplicate pair counts replace while total/marginals sum all accepted lines. PMI log2(c_last*total/(verb_sum*noun_sum)) is conventional only under unique-row assumption. Source-derived partial relation; checks above and explicit unexecuted gaps delimit evidence.

## export-priority

Detailed rows: kind asc,count desc,verb/object asc; particles count desc,phrase asc. Final map pair order; particles overwrite same key. Determinism requires fixed dependency snapshot. Source-derived partial relation; checks above and explicit unexecuted gaps delimit evidence.

## lexical-dependency

Matrix::score consults global::absurdity_in, global::idiom and known layer; known idiom/light/collocation clamps literal OBJ to <=1. Re-mining has prior lexicon feedback. Source-derived partial relation; checks above and explicit unexecuted gaps delimit evidence.

## Epoch notes

## evidence

### Evidence

Each syntax projection generated twice identically, then copied to syntax.json. A two-source isolated snapshot plus external temporary registry validates both file roots and six fragments; repeated structural reports match, with zero diagnostics and one intentionally missing temporary parent sidecar. Full-worktree check with only the two-source registry diagnoses unrelated ownership; no full-inventory validity is claimed here. changed.json reports both changed Rust files with missing=[]; ancestor integration belongs to root task.

Coverage: 10 typed root views plus three four-view fragments per file, 44 views total. Models are partial source relations and finite correspondences, not semantic ASTs, proofs, approximate speedups, or linguistic accuracy. Helper tests do not execute parser/teacher, full mining output, global hot state or I/O failures. Dependency hashes are explicit extension metadata; optional absent/runtime dependencies remain gaps.

## dependency-notes

### Lexicon snapshot boundary

The direct identities in dependency-bindings.json supplement the source binding; modelcheck does not validate this extension. global/data/idioms.tsv is absent in this checkout, an explicitly supported cold build with no internal idiom table. global::hot reads optional GLOBAL_HOT files once through OnceLock; it can override cold matrix/idiom entries, including a none removal. Thus a source hash alone cannot bind all runtime behavior. No hot layer was read by the lexical-helper tests, and no run classification/export fixture was executed.

The dependency is actual Matrix::score behavior: for OBJ, known idiom/light-verb/collocation clamps an existing literal score to min(score,1); otherwise the literal or known layer is returned. An absent literal plus known idiom can return None at the clamp branch. Mining can therefore depend on its previous dictionary snapshot. A parent interaction test should freeze GLOBAL_HOT before process initialization and distinguish raw literal scores from suppression-aware scores. This is a proposed test boundary, not completed evidence.

## History

### 2026-10-30 · evidence

Heading `### Evidence — 2026-10-30` stays in evidence as `### Evidence`.

CPU native checks: cargo test -p world --lib wave8_models -- --test-threads=1, twice debug and twice release. Each run passes 10 tests (five per owned file), 218 filtered out. The four ordered test-name/result projections match exactly: SHA-256 df12ad1b6b325d1334fcda986aa2718c5fba2b44d0aaeca22cc6ccf3be83d7a2. Existing unrelated build warnings remain. No runtime throughput claim follows from test duration.

Negative control: Change dictionary replacement rank < to <=; dictionary_rank_matches_independent_first_minimum fails a compiled assertion. Exit 101 with named FAILED and assertion diagnostic, not compiler failure. Saved source restored byte-for-byte; restored combined ten-test run passed. Raw mutant-idioms.log SHA-256 ada10e05df3e582b8efc5c160eb27ce25389addd11362f94518a2ca8ae9d6834.

Run artifacts are relative to $MOVA_DATA/runs/octopus-wave8-questions-2026-10-30/: debug-1.log, debug-2.log, release-1.log, release-2.log, *.normalized, mutant-idioms.log, restored.log, changed.json. Large raw output is outside the repository.

### 2026-11-04 · Epoch notes

- Comments repointed to consolidated docs, 2026-11-04.
