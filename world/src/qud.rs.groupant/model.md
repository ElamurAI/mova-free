# world-qud

## model

# Preannotated QUD stack selection and accommodation

## forward

State=(open stack,given lemmas,last subject). Prepare tokens, push questions or select latest congruent, accommodate if none, then update given/last. One Step per input. No probabilistic ranking.

## invariant

Stack IDs are an ordered subsequence of pushed sentence IDs. Explicit answer targets an open question. Complete removes only winner; partial preserves stack. Implicit never pushes.

## cost

For S sentences, T tokens and Q open questions, conservative O(S Q T^2 + S T log(S T)) comparisons plus string/global animacy costs; output reasons can grow with rejected candidates.

## composition

Preparation feeds slot eligibility, then tracking applies completeness only after ranking. track adds external annotator/splitting. Tests use track_toks only; malformed heads remain trusted.

## epoch-memory

Predicted latest eligible and partial persistence; 46,656 bounded histories confirm. Generic-hedge prose overstates implementation: Who+maybe is complete. No production repair or continuous approximation metric proposed.

## checks

Five native tests: 46,656 six-step histories (279,936 transitions), uncertainty interaction, empty-content slot gate, four polarity preimages, ordered givenness.

## behavior-complement

Universe: admissible inputs and candidate projected outputs, fixed dependencies. Source-derived inner exclusion: outcomes contradicting the stated invariant/priority/eligibility constraints are outside R. These constraints are partial models of implementation, not a complement of linguistic truth. Finite tests support only their declared domain.

## reverse-reconstruction

B(y)={x in declared domain | F(x)=y}, including latent state and dependency snapshot. Retain all compatible antecedents; rejected rows/tokens and discarded projections create ambiguity. Full text/reasons/values narrow fibers compared with key/status projections. No complete solver, unique recovery or causal inference claimed.
## recency-priority

Winner=max eligible stack index regardless of completeness. Empty eligible set accommodates. Bounded oracle uses maximum eligible ID, not production search. Source-derived partial relation; checks above and explicit unexecuted gaps delimit evidence.

## slot-eligibility

Nonempty question content needs overlap plus slot; empty content waives overlap only. Proper names avoid global lookup; Unknown noun animacy is ineligible. Source-derived partial relation; checks above and explicit unexecuted gaps delimit evidence.

## uncertainty-partition

Complete is a branch property, not truth certainty. YesNo hedge is partial; definite Who slot ignores generic hedge. Source-derived partial relation; checks above and explicit unexecuted gaps delimit evidence.

## polarity-fiber

Project away text/reasons: sleep asserted/not/never/no yield same complete attachment and empty stack. Full Step has narrower fibers. Source-derived partial relation; checks above and explicit unexecuted gaps delimit evidence.

## givenness-order

Missing requested lemmas produce leakage if in answer else hallucination, retaining order and duplicates with exact string equality. Source-derived partial relation; checks above and explicit unexecuted gaps delimit evidence.

## recency-resolution

Listed pronouns use last subject without gender/number agreement. Questions update last too. Subjectless input leaves prior last intact. Source-derived partial relation; checks above and explicit unexecuted gaps delimit evidence.

## annotation-boundary

Tok::new lowercases, public fields and toks_of need not. First head-zero root and head=i+1 children are trusted; no parser accuracy claimed. Source-derived partial relation; checks above and explicit unexecuted gaps delimit evidence.

## lexical-dependency

Who nouns depend on sense::animacy → global categories/selection; conflicting animate/inanimate categories yield Unknown. Freeze dependency snapshot. Source-derived partial relation; checks above and explicit unexecuted gaps delimit evidence.

## evidence

### Evidence

Each syntax projection generated twice identically, then copied to syntax.json. A two-source isolated snapshot plus external temporary registry validates both file roots and six fragments; repeated structural reports match, with zero diagnostics and one intentionally missing temporary parent sidecar. Full-worktree check with only the two-source registry diagnoses unrelated ownership; no full-inventory validity is claimed here. changed.json reports both changed Rust files with missing=[]; ancestor integration belongs to root task.

Coverage: 10 typed root views plus three four-view fragments per file, 44 views total. Models are partial source relations and finite correspondences, not semantic ASTs, proofs, approximate speedups, or linguistic accuracy. Helper tests do not execute parser/teacher, full mining output, global hot state or I/O failures. Dependency hashes are explicit extension metadata; optional absent/runtime dependencies remain gaps.

## History

### 2026-10-30 · evidence

Heading `### Evidence — 2026-10-30` stays in evidence as `### Evidence`.

CPU native checks: cargo test -p world --lib wave8_models -- --test-threads=1, twice debug and twice release. Each run passes 10 tests (five per owned file), 218 filtered out. The four ordered test-name/result projections match exactly: SHA-256 df12ad1b6b325d1334fcda986aa2718c5fba2b44d0aaeca22cc6ccf3be83d7a2. Existing unrelated build warnings remain. No runtime throughput claim follows from test duration.

Negative control: Reverse newest-first selection to oldest-first; bounded_stack_matches_independent_latest_eligible_oracle fails a compiled assertion. Exit 101 with named FAILED and assertion diagnostic, not compiler failure. Saved source restored byte-for-byte; restored combined ten-test run passed. Raw mutant-qud.log SHA-256 038eb01757387f5f1d7f990f07e8fa32c71d09c453475a1cdc94161d10a0bb34.

Run artifacts are relative to $MOVA_DATA/runs/octopus-wave8-questions-2026-10-30/: debug-1.log, debug-2.log, release-1.log, release-2.log, *.normalized, mutant-qud.log, restored.log, changed.json. Large raw output is outside the repository.
