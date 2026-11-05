# school-runtime

## model

# school-runtime

## forward

For fixed parser/resources/configuration, decode lesson fields; read facts into student S; answer from indexed frames; correct role weights on exercise mistakes; report tests/exams from the resulting state. Course execution is a lesson-reading diagnostic and is not a fresh-process retained examination.

## invariant

Every modeled update uses the same selected support as the decision. Gold labels enter exercise updates and reports, not answer(q,opts). Full parser semantics, frame integrity and persistence remain dependencies.

## cost

Parsing plus ingestion plus candidate/target comparisons plus exercise passes; the candidate sum counts list visits, not unique frame comparisons. No timing or speedup is inferred.

## checks

Existing knowledge:: tests exercise format, tails, trace, controls, witness correction, and exhaustive mask correspondence; child learning/retention tests exercise dependencies. See model.md#tests for executed scope. A proposed oracle below is not a completed check.

## epoch-memory

`course_dir` added. In the closed build (`peta` feature) the courses `wikipedia` and `wikipedia-<part>` resolve to `$MOVA_WIKISCHOOL/<name>`, by default `$MOVA_DATA/corpus/wikipedia/school/<name>` (CC BY-SA lessons of an internal tool not in the public tree, outside the repo); every other course, and every course in the free build, keeps `data/school/<name>`. Checked by `tail_tests::the_wikipedia_courses_stay_outside_the_repo` (a name merely containing the word stays public). Fragment ranges after the insertion moved by 862 bytes; no behavior of the course runner changed.

## behavior-complement

Type: behavior_complement. Domain and observation: Lesson decoding, retained-frame answer selection, witness learning and diagnostic course execution, as narrowed below. Operator/target: U consists of terminating single-call input/output/state transitions for the declared entry points, fixed configuration and valid internal indexes. Let R be actual Rust behavior. An inspected inner-bound hypothesis for U\R contains successful parse results lacking number/facts/exercises/tests, weight changes from count-decided answers, and current-support activation that retains old-only witnesses. This is a relation over transitions, not merely a negative-test list.

Approximation: source-derived scoped relation, conditional on valid internal state and fixed flags/resources; unmodeled behavior remains unknown. Evidence/oracle: Format and witness tests check examples; broader inference exclusions remain unproved.

## reverse-reconstruction

Type: reverse_reconstruction. Domain and observation: Lesson decoding, retained-frame answer selection, witness learning and diagnostic course execution, as narrowed below. Operator/target: For observation o, B(o)={ (S,input,config) | project(run(S,input,config))=o }. A score or answer has many compatible frame stores, weights and lesson histories. Restrict reconstruction by the child fibers without selecting one original. Neither parsed Lesson nor aggregate correctness is injective.

Approximation: source-derived scoped relation, conditional on valid internal state and fixed flags/resources; unmodeled behavior remains unknown. Evidence/oracle: Proposed oracle: enumerate small initial weight/witness states; preserve every pair giving the same observation.

## lesson-admission

Type: input_contract. Domain and observation: Lesson decoding, retained-frame answer selection, witness learning and diagnostic course execution, as narrowed below. Operator/target: For parsed fields L, accepted iff n>0 and facts, exercises and tests are nonempty; exams/course/title/level are not required. parse_fields errors precede this guard. Unknown fields error; duplicate scalar fields overwrite earlier values, repeated list fields append.

Approximation: source-derived scoped relation, conditional on valid internal state and fixed flags/resources; unmodeled behavior remains unknown. Evidence/oracle: Existing starred-option test checks question labels; complete parse admission Cartesian oracle remains proposed.

## candidate-budget

Type: resource_accounting. Domain and observation: Lesson decoding, retained-frame answer selection, witness learning and diagnostic course execution, as narrowed below. Operator/target: For each entity/alias/kind/verb posting list v, add its newest min(cap,|v|) entries into a BTreeSet. Candidate union size <= sum_v min(cap,|v|). Timing COST counters measure this sum across lookup visits, including overlaps, not distinct candidates.

Approximation: source-derived scoped relation, conditional on valid internal state and fixed flags/resources; unmodeled behavior remains unknown. Evidence/oracle: Proposed oracle: overlapping synthetic postings at cap 0/1/100; no course benchmark executed.

## anchor-ranking

Type: ranking_operator. Domain and observation: Lesson decoding, retained-frame answer selection, witness learning and diagnostic course execution, as narrowed below. Operator/target: Retain anchors with strength>=2; if enabled and current lesson has any, restrict to it; if any remaining strength>=3, drop strength=2; sort descending (strength,frame_index), take ten. This ordered pipeline differs from globally taking ten first.

Approximation: source-derived scoped relation, conditional on valid internal state and fixed flags/resources; unmodeled behavior remains unknown. Evidence/oracle: Proposed finite ranking oracle; parser-to-anchor correspondence remains missing.

## target-route

Type: route_quotient. Domain and observation: Lesson decoding, retained-frame answer selection, witness learning and diagnostic course execution, as narrowed below. Operator/target: When the same remembered frame is reached by several routes, BTreeMap target construction keeps the smallest Via discriminant. Reconstruct route histories as all nonempty offered route sets whose minimum equals the observed route; losing routes cannot be recovered uniquely.

Approximation: source-derived scoped relation, conditional on valid internal state and fixed flags/resources; unmodeled behavior remains unknown. Evidence/oracle: Proposed enumeration of subsets of seven routes; graph expansion scope remains partial.

## mask-strength

Type: kernel_equivalence. Domain and observation: Lesson decoding, retained-frame answer selection, witness learning and diagnostic course execution, as narrowed below. Operator/target: For u16 m, strength_mask(m)=2 popcount(m & role_bits_without_attr)+indicator(attr_bit). Other bits contribute zero. Equality with strength(ALL_FEATS selected by m) concerns set-valued align features; arbitrary duplicated feature lists and VerbClass are outside this equality.

Approximation: source-derived scoped relation, conditional on valid internal state and fixed flags/resources; unmodeled behavior remains unknown. Evidence/oracle: Executed existing exhaustive test covers all 65,536 masks; do not generalize to arbitrary Vec<Feat>.

## count-override

Type: decision_precondition. Domain and observation: Lesson decoding, retained-frame answer selection, witness learning and diagnostic course execution, as narrowed below. Operator/target: A computed quantity replaces frame choice iff a how-many/how-much path yields count c and exactly one option has quantity Some(c.value). Zero or multiple matching options leave prior choice/count unchanged. This is numerical matching, not unique linguistic truth.

Approximation: source-derived scoped relation, conditional on valid internal state and fixed flags/resources; unmodeled behavior remains unknown. Evidence/oracle: Existing witness skip test covers count decisions; full count-override oracle remains proposed.

## weight-gap

Type: local_update_identity. Domain and observation: Lesson decoding, retained-frame answer selection, witness learning and diagnostic course execution, as narrowed below. Operator/target: For fixed supported wrong choice c and right r, d=phi_r-phi_c and w'=w+d, where feature multiplicities count. Then (w'-w) dot d=||d||^2, assuming no i64 overflow and valid indexes. Recomputed witnesses may change, so this is not global monotone accuracy.

Approximation: source-derived scoped relation, conditional on valid internal state and fixed flags/resources; unmodeled behavior remains unknown. Evidence/oracle: Existing active-witness fixture checks +2 and an old-witness control gives -2.

## shuffle-fiber

Type: permutation_preimage. Domain and observation: Lesson decoding, retained-frame answer selection, witness learning and diagnostic course execution, as narrowed below. Operator/target: shuffled(seed,right,wrong) preserves the option multiset, applies fixed splitmix64-seeded swaps and reports the first equal-to-right position. With duplicate equal strings, this index does not identify the original labeled occurrence. Its preimage includes seeds/orders/duplicate identities compatible with that observation.

Approximation: source-derived scoped relation, conditional on valid internal state and fixed flags/resources; unmodeled behavior remains unknown. Evidence/oracle: Source-derived; proposed literal duplicate-options oracle, no distribution or uniformity claim.

## control-share

Type: ratio_complement. Domain and observation: Lesson decoding, retained-frame answer selection, witness learning and diagnostic course execution, as narrowed below. Operator/target: control_red(r,n,c,m) iff m=0 or c*n<r*m in u128 products. For m>0, the complement is c*n>=r*m. Strict equality is not red; m=0 is vacuously red, not evidence that a measured control degraded.

Approximation: source-derived scoped relation, conditional on valid internal state and fixed flags/resources; unmodeled behavior remains unknown. Evidence/oracle: Existing control test checks lower/equal/higher shares; overflow assumptions are platform usize<=64.

## assessment-boundary

Type: protocol_separation. Domain and observation: Lesson decoding, retained-frame answer selection, witness learning and diagnostic course execution, as narrowed below. Operator/target: run_course_lessons reads each lesson and calls learn_lesson(l,2) before after-tests/exams. Its scores therefore observe online diagnostic state. Fresh-process question-only retained examination is a distinct protocol owned by retention.rs. Assessment equivalence requires matched state, focus/config, seeds and inputs, not equal totals.

Approximation: source-derived scoped relation, conditional on valid internal state and fixed flags/resources; unmodeled behavior remains unknown. Evidence/oracle: Source inspection plus retention tests; no full course/teacher/GPU execution.

## composition

The format child maps text to Lesson; the course child consumes ordered lessons and student state; the decision child is repeatedly called within learning/testing. For intermediate states y, R_parent(x,z) requires the ordered child relations plus shared lesson IDs, options, weights, configuration and source provenance. Format success does not guarantee parser or inference success. Local question decoding is counted inside format, and witness correction inside decision, so cost estimates must not double-count them. Dictionary-course generation, ingestion indexes, kind reasoning, CLI dispatch, diagnostics, teacher-check and persistence internals remain partial or outside this decomposition. Parent complement targets its own U\R; it is not the union of child complements. Sequential reversal retains every compatible intermediate state. Failure prefixes may terminate before later stages. No partition of all source bytes is asserted.

## gap_diagnostic

## Epoch notes

## tests

### School runtime checks

#### executed-scope

The bounded CPU command is `cargo test -p world knowledge:: -- --test-threads=1`. Its actual result is recorded after completion below. It includes existing format/tail, citation, trace, control, school-math, learning and retention tests. No tests were added solely for inventory counts.

The mask test enumerates all 65,536 u16 inputs. The witness fixture checks a +2 active decision-gap update and an existing wrong-global-witness control yielding -2. Label, tail, citation, reporting and ratio tests include rejection/preservation controls. These are existing executed tests, not new independent proofs of every catalog entry.

#### limits

No full course, ignored benchmark, parser download, teacher call, GPU run, full training or held-out release evaluation. Proposed catalog oracles are explicitly prospective. Structural groupant validation checks declarations, source identity and bounds, not mathematics.

## wave34-composition

### Wave34 composition

The existing format/course/decision children remain. New citation/fact/field descendants refine format. Tail recognition precedes conditional parser dispatch in answer; preparse publication is a separate helper used by course/retention preparation, not a step after tail recognition. Both bind the same runtime state/config assumptions. Preparse cache preservation, error non-effects and reverse relations additionally require a callback that neither mutates the same trees RefCell nor leaves a borrow active at publication; FnOnce does not itself enforce this premise. Current synthetic callbacks satisfy this restriction. Course replay remains a diagnostic; no persistent-learning claim follows.

Reverse composition preserves compatible intermediate values and their shared ledger/course/cache constraints. Parent behavior complement is relative to its own coupled trace universe, not a union of child complements. Costs include each executed call once. Existing model.md/composition.md retain historical models and evidence.

## wave34-evidence

### Wave34 knowledge evidence

Ten new scoped groupants across two existing Rust sources, six source-specific typed views each. Five inline helper oracles added to knowledge.rs; retention uses five existing synthetic tests. Original source bytes preserved exactly; only the cfg(test) module is appended.

Ten exact named world::lib tests pass twice debug and twice release (40 positive invocations). Sorted target-qualified outcomes agree across all four profiles/trials. Three production mutants compile and are rejected by their named runtime tests: citation predicate inversion and preparse cardinality bypass fail direct assertions; removing the kind sentinel exemption causes a Result::unwrap panic on the unexpected provenance Err at retention.rs:500. The latter is a compiled behavior rejection via runtime panic, not a direct assertion rejection. Cross-review supersedes the original all-three-assertions wording while preserving the raw receipts. Each is restored byte-for-byte and its selected test passes again. Compile failures and assertion-only edits: zero. The initial citation pilot also passed.

Syntax for both sources generated twice and byte-compared. Current source hashes match syntax and every descendant manifest; shared syntax is not duplicated in children. Structural report has only two expected ancestor bindings for knowledge.rs (mova and self-study), for integration to refresh. No production repair or optimization was made.

No corpus, actual lesson, teacher, network, model loading, training, frozen suites or successful exam was run. Tests use literal strings, synthetic callbacks or hand-built graphs and temporary files. Lesson replay is not credited as retained knowledge. No observed-error percentage surrogate is justified; parser/inference behavior, teaching truth, cold-exam quality, I/O fault recovery and full semantic correspondence remain gaps. Existing models and epoch memories are retained as historical evidence.

## History

### 2026-10-29 · document

Source-bound partial models, epoch 2026-10-29. Subject: world/src/knowledge.rs at the manifest SHA-256. Observation: Lesson decoding, retained-frame answer selection, witness learning and diagnostic course execution. Every relation below concerns this frozen source/configuration. Structural validation does not establish semantic correctness.

### 2026-10-29 · epoch-memory

2026-10-29: inspected the existing implementation before modeling. Preserved current behavior, including lossy decoding, index-sensitive ties and conditional updates. Existing witness tests distinguish active-scope learning from a wrong global witness. Counts represent distinct observations, not independent proofs. No source changes, performance gain, full-course result or autonomous combiner is claimed. AST remains explicitly absent; ranges bind source bytes only. Parser resources, panic/allocation behavior and complete inference semantics remain gaps.

### 2026-10-29 · tests

#### actual-results-2026-10-29

The command above passed: 28 tests passed, 0 failed, 1 ignored (the explicitly ignored strength microbenchmark), 104 filtered out in the world unit-test binary; filtered integration targets ran zero tests. This includes tests in the nested learning/retention modules and is not a claim of 28 new school-runtime tests. Runtime test execution took 0.12 seconds after compilation; this is not a performance comparison.

The unchanged-source inventory was checked with `mova-modelcheck groupants . models/contracts.json NEW_JSON`. All artifact/range/type declarations passed; its sole diagnostic is the expected pending integration: `world/src/knowledge.rs.groupant: registry node does not match source ownership`. The owner node is provided in registry-node.json for insertion under self-study. The coordinator must register it and refresh ancestor bindings before claiming an overall valid inventory. Ancestor groupants and models/contracts.json were deliberately not edited in this isolated worktree.

### 2026-10-31 · wave34-evidence

Evidence SHA-256: native-receipts.json a3330bf15cd1b7901bac4a7fd4d7522a4c6b72bf02f08e17e07b541416acc496; mutation-receipts.json bc6c2f106735fa5f515618efc2138f08a363693fc0233db417836f0cfbda1151; production-delta.json f688ac9cb00e9d64eccac438b3463ce60cffd297bfd86b882b0b0c92c98500a1. External raw receipts are in $MOVA_DATA/runs/octopus-wave34-knowledge-2026-10-31.

### 2026-11-03 · epoch-memory

2026-11-03: `course_dir` added. In the closed build (`peta` feature) the courses `wikipedia` and `wikipedia-<part>` resolve to `$MOVA_WIKISCHOOL/<name>`, by default `$MOVA_DATA/corpus/wikipedia/school/<name>` (CC BY-SA lessons of an internal tool not in the public tree, outside the repo); every other course, and every course in the free build, keeps `data/school/<name>`. Checked by `tail_tests::the_wikipedia_courses_stay_outside_the_repo` (a name merely containing the word stays public). Fragment ranges after the insertion moved by 862 bytes; no behavior of the course runner changed.

### 2026-11-03 · gap_diagnostic

2026-11-03/04: `MOVA_GAPDIAG=1` prints why the student finds no support for an option (stderr only;
`PragStudent::gap_diag`); frame indexing moved unchanged from `read` into `remember_frame`. Traces on ten courses are
byte-identical with the diagnostic off or on; `knowledge::gap_tests` fails when the reached-gated branch is disabled.
Measured and removed experiments (light-verb reading, own-line/near support, question-entity modifiers) are recorded
in docs/training.md §28.

### 2026-11-04 · forward

Executable vidtvorant: the child [student-vidtvorant.groupant](student-vidtvorant.groupant/model.md) reproduces the pupil's chosen option, understanding and answer time from word bags with the code's decision structure (measured 2026-11-04: 67.2% choice agreement out of fold on ten courses, 26–30× faster per answer; no 1/2/5/10% cap met).

### 2026-11-04 · cost

Measured 2026-11-04, one P-core thread (i5-13500, taskset, release, en.bin 0.4, host load average 9–34), ten primary courses: parse ahead ≈ 0.6–0.9 s model load + 9–20 k words/s (18–20 k at the lower load) and takes 40–70% of a course; reading 4–9%; two exercise passes (recomputation path) 14–27%; answers 1.0–1.5 ms median per question (p95 ≤ 2.9 ms), about 600–950 questions/s once parsed; peak resident memory 0.95–1.09 GiB. Time per answer grows ≈ (memory lines)^0.1, (question words)^0.5, (option words)^0.3 (least squares on ln ns, `student-vidtvorant.groupant`). Runs: `$MOVA_DATA/runs/mova-speed-2026-11-04`.

### 2026-11-04 · gap_diagnostic

2026-11-04 (Sea Turtle): the rule of `world knowledge check` moved unchanged into `pub fn lesson_problem(&Lesson) ->
Option<String>` (the same condition and the same «✗ …» line), called by the check and by `tools/turtle` (`check::lesson`),
so the two checks cannot drift. `world knowledge check commands` still prints «30 lessons in «commands», 0 with a
problem». Fragment ranges lie before the change and keep their bytes.

### 2026-11-04 · Epoch notes

- Comments repointed to consolidated docs, 2026-11-04.

### 2026-11-04 · tests

#### actual-results-2026-11-04

`cargo test --release --offline -p world --lib knowledge` passed: 40 passed, 0 failed, 1 ignored (the strength microbenchmark). New: `knowledge::gap_tests::the_gap_diagnostic_names_why_an_option_found_no_support` (an option frame supported only by a remembered frame about none of the question's things is `not-about` at line distance 1; an unknown verb is `verb-unknown`; control: the same frame as a target about the question is `reached-gated`). Negative control run: with the reached-gated branch disabled the test fails (`about-other` against `reached-gated`).

#### lesson-problem-2026-11-04

`cargo run --offline -p world --bin world -- knowledge check commands`: 30 lessons, 0 with a problem (as before).
The rule is exercised through `tools/turtle` (`check::tests::controls_turn_red`: 11 exercises flagged; red control:
`check::lesson` without `lesson_problem` turns three Turtle tests red, restored).

### 2026-11-05 · public export

Doc comment of course_dir: the writer of the Wikipedia courses is named as an internal tool not in the public tree. No code change.

### 2026-11-05 · school data outside the repository

The courses are read from the school data outside the repository: school_data() = $MOVA_SCHOOL_DATA, else $MOVA_DATA/school ($MOVA_DATA defaults to ~/mova-data); courses_root() = school_data()/courses, dictionaries_dir() = school_data()/dictionaries; root() = courses_root(). The release archive school-0.5 installs there (mova fetch 0.5 school). Behaviour complement: no fallback to the repository's old world/data/school; without the data the courses are absent (course() reports the missing folder). The dictionary course still reads the compiled-in world/data/dictionary-improved.tsv.
course() fails with an install hint when the courses root itself is missing (the school is not installed); a missing single course folder (a placeholder) still reads as an empty course. Controls: world tests pass with the school data installed (466 lib + integration, 2026-11-05); with MOVA_SCHOOL_DATA pointing nowhere the lib tests still pass and the course-running integration tests (parser_failures, school_bench) fail with that message.
