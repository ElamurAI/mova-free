# en-diag

## descriptant

### en-diag

Dependency-tree error geometry, relation classes, PP attachment and parser diagnostic report.

## approximant

### Forward approximant

depths follows each head chain while nonzero and steps<=n, stores root distance (cycles yield n+1). nonprojective(d) checks intervening words for ancestry under h, root arcs false. report decodes baseline, geometry, PP, k-best oracle then constrained class interventions; absent constrained tree reuses baseline and counts dead.

Domain/state: Head arrays (1-based arcs, zero root), relation/tag vectors, prediction beams and report counters.

Executable surrogate: unsupported. Categorical decisions, binary layouts and effects have no calibrated percentage metric here. No automatic substitution and no claimed 1/2/5/10 percent profile.

## invariant

### Invariant and cost

Depth vector length n+1 and d[0]=0. Valid in-range acyclic heads give finite distances <=n. Class partition is ordered match, NmodPoss is noun phrase class5 not PP class1. pct(a,0)=100a; all empty count ratios zero when numerator zero.

Cost: Geometry O(n²) depths, nonprojectivity O(n³) worst for n arcs and nested ancestry. Report adds parser costs for each width and ten class interventions. Space O(tokens+predictions+beam trees+report). Malformed head arrays can panic.

Limits: No trained-parser/report execution; PP and constrained-beam branches source-derived; Comment cycle-at-n differs from actual n+1; model records code behavior; Reported nonprojective ceiling text is not a proved bound

## composition

### Composition

depths follows each head chain while nonzero and steps<=n, stores root distance (cycles yield n+1). nonprojective(d) checks intervening words for ancestry under h, root arcs false. report decodes baseline, geometry, PP, k-best oracle then constrained class interventions; absent constrained tree reuses baseline and counts dead.

All dependent/head IDs are one-based except array position; root arc length is reported as1. Recall groups gold depth, precision groups predicted depth, avoiding invalid direct equality of denominators.

Composition is branch-selected, not a claim that every function runs in sequence. Inputs and effects cross the English library module boundary; consumer checks cannot establish delegated training, parser, dictionary, registry or filesystem behavior. The main CLI has source-scoped children for dispatch, XPOS, evaluation/context, text/conversion and persistence/comparison. Child coverage is lexical and parent relations remain partial.

## models

## behavior_complement

For in-range heads exclude depth outputs with wrong length, nonzero d[0], or steps>n+1. A root arc is excluded from nonprojective=true. No universal claim of detecting/correcting invalid trees.

## reverse_reconstruction

R(depths)=all in-range head arrays with those truncated path lengths; star siblings and different same-depth parents collide. Relation class inverse is the finite set of Rel variants matched to that class; class does not reconstruct exact subtype.

## cycle_bound

The <=n guard permits n+1 transitions, including a self-cycle n=1 producing depth2. It bounds cycles but never checks head<=n before indexing.

## pp_selection

PP requires gold obl/nmod except nmod:poss, nonroot head, first case child with IN/TO and nounlike/VERB gold head. Found requires predicted case arc and PP relation; right then requires correct head; missed-as-noun only credits gold nominal heads.

## constrained_fallback

No forced arcs uses baseline directly; parse_constrained(None) increments dead and uses baseline. Negative cascade fixed=other-before minus after may be negative; no monotonic improvement invariant.

## metric_coordinates

All dependent/head IDs are one-based except array position; root arc length is reported as1. Recall groups gold depth, precision groups predicted depth, avoiding invalid direct equality of denominators.

## tests

### Evidence

Checks: diag::wave27::cycles_empty_and_bins; diag::tests::depth_and_projectivity.

The selected suite contains 14 actual named Rust tests, each run twice debug and twice release with identical sorted named outcomes, all exit0. Build entrypoint additionally ran empty-directory, two-lemma synthetic dictionary and malformed-tag precedence cases twice per profile; outputs matched and malformed row exited101 without changing previous outputs. Compiled mutation control 3: compiler exit0, selected assertion run exit101, exact source restoration, positive rerun exit0. See external control-3.txt and compile/red/green logs.

Unexecuted: No trained-parser/report execution; PP and constrained-beam branches source-derived; Comment cycle-at-n differs from actual n+1; model records code behavior; Reported nonprojective ceiling text is not a proved bound. No corpus, training, model mutation, held-out set, GPU, teacher or API was run.

## prospective

### Prospective source-bound modeling

- build.rs assembles dictionary blobs and constants. index has power-of-two capacity >=2n and linear probing, zero sentinel, 1-based stored indices; src_bits unions four known source bits. main reads missing/unreadable dictionaries as empty, parses tags before lemma/count, merges UD lower-lemma paradigms and writes three outputs sequentially after OUT_DIR lookup. Test actual helpers and invoke compiled build main only in isolated synthetic directories. Overflow and large-id casts remain gaps.
- ctx.rs preserves absent layers, emits stable group codes, stacks entity openings before token observation and closes afterward; gold carries genre across newdoc while resetting previous type/speaker, skips empty tokens after state updates. derived checks sentence count then noun lemma recurrence by document. Setup rejects training pred before eval/groups/source/dependency checks. Rng wrapping mix and bounded modulo; shuffled resizes entities and rejects same-document donors, with unbounded rejection-loop worst case. Existing two GUM tests must not run. Safe existing entity and gold/shuffle fixtures plus added empty/cycle/precedence cases.
- ctxeval.rs load reads each file then checks sentence count, id and token count (not forms), rejects no runs last. report assumes nonempty systems and runs; bootstrap with zero resamples panics, floating means of empty samples can be NaN. Metrics percent units, sample standard deviation, quantile indices floor(.025B), ceil(.975B)-1. Add bounded pure helper tests plus temporary System::load and report fixtures. Invalid arbitrary public System shapes are not certified.
- diag.rs class partitions relations; depths cycles return n+1 despite comment saying n; invalid heads panic. nonprojective walks intervening words; report delegates parser work and constrained failures fall back to original parse. Synthetic depth/crossing tests only, no model training. Report ceiling prose is not a mathematical guarantee.
- emode.rs first matching closed entry wins force, tagged finds alternate thine reading; contextual guards precede ending dictionary resolution; fix filters feats through loaded embedded registry, only be keeps cop. postverbal_not skips modern continuations, auxiliaries, already-attached and newly-created cycle cases; preexisting cycles bounded to n+1 steps, invalid aligned arrays/heads can panic. Execute safe existing closed-word and negation tests; dictionary-sensitive early-return tests excluded.
- lib.rs solely declares 26 public modules: compile/name visibility is the only source-owned boundary, no independent runtime semantics or control claimed.
- license.rs sequentially strips ordered suffixes; empty paths yield empty source. Table is embedded and collects duplicate keys last; gate first denied/unknown path fails, empty train succeeds without opening files. Existing safe path-only gate test plus empty and failure-precedence fixtures.
- main.rs dispatches arguments and effectful delegated work. split searches separator globally and may panic when separator precedes from. xpos_of has ordered morphology precedence. folds rejects k<2 before read, preserves per-sentence raw content but resets document blank_before/final_newline; creates directory then writes fold/rest sequentially without rollback. test_ctx None returns before any read. train license gate precedes missing output validation; model save precedes reload verification. Nested fragments will cover dispatch, pure/file helpers, context/evaluation, conversion, persistence. Add bounded helper/folds/xpos/temp tests; training/model/teacher execution excluded.

Expected costs: context O(bytes + mentions*stack depth), tree rules O(n²) worst case; diagnostics O(n²) tree traversal plus delegated decode; report O(B*n*seeds) bootstrap plus token metrics; build linear probing O(n²) worst case and binary/strings space; CLI costs sum delegated stages with sequential partial effects. Compiled mutation controls target index sentinel, context mode, bootstrap statistics, cycle bound, archaic force, license denial and XPOS precedence. Exact restoration verified byte-for-byte. No useful percentage surrogate for categorical decisions, effects or visibility; executable approximants explicitly unsupported.

## History

### 2026-10-31 · descriptant

2026-10-31 modeling epoch: inspected complete existing source before changes; added bounded inline tests where missing. Production behavior unchanged. The model records existing failure boundaries, including panics, lossy projections and partial effects. A registered leaf is partial coverage, not a proof.

### 2026-10-31 · tests

Heading `### Evidence — 2026-10-31` stays in tests as `### Evidence`.

Receipts: $MOVA_DATA/runs/octopus-wave27-control-2026-10-31/. verify.rs and controls.rs preserve exact commands; *.exit files preserve numeric statuses. Paired syntax projections were identical. Strict Clippy exits101 on pre-existing en/build.rs:51 manual_is_multiple_of before library checking; no lint-clean claim. Only appended test modules formatted; existing source formatting retained.

### 2026-10-31 · prospective

Heading `### Prospective source-bound modeling — 2026-10-31` stays in prospective as `### Prospective source-bound modeling`.

Read all eight sources at base e90d56b1d87092a22ce7a05ce8352fe70cf151d8 before edits. Scope is English orchestration under inference-core. No training, corpus or external services will execute.
