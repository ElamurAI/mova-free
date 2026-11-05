# prag-groupant-wave13

## model

# Wave13 pragmatic native joins

## forward

Seven bounded tests implement a tagged family: six-field actual tiny fitting/prediction/schema transport, dictionary snapshot equivalence, source-qualified predecessor identity, supplied schedule feedback, agreement voice projection, cue-to-dictionary adapter and score observation. Nine catalog views comprise the first behavior complement, second reverse reconstruction and seven specialized relations.

## invariant

Preserve slot-local dictionary coordinates, field class order, row/predecessor identities, supplied schedule, canonical means domain, character voice and present-class denominators. The context observer is a test double for eval's adapter, not a learned model. Actual constant-label fits prove transport, not held-out accuracy.

## cost

Finite four-row fitting for each kind, three/seven-item schedules and bounded score vectors. Compose reached APIs only; no parser acquisition, throughput claim or whole-pipeline error bound.

## composition

feats/model/schema join through actual public eval::fit_set and predict_seq; supplied data::Item establishes source identity. eval.rs and data.rs remain explicit unregistered dependencies, not covered leaves. agree is an existing registered consumer. Cue projection uses an explicit test adapter and does not exercise parser-backed private row projection. infer ancestry and gain determinism are leaf/source observations, not an invented CLI chain.

## epoch-memory

# Wave13 prospective pragmatic boundary models

Full-width aligned rows R and dictionary D give slot-specific IDs E_D(R), including UNSEEN and every Act/start preseeded in the prev slot. Field predictions must lie in the field's closed alphabet. Label TSV round trips preserve the tuple only with nonempty bounded TSV-safe means. IDs are dictionary-local.

predict_seq follows supplied idx order. Predecessor context comes from an already produced prediction, otherwise start. prev_of and rows must share Item index/source/document identity. Reversing the schedule may change outputs. A test observer Classifier isolates this wiring; separate tiny real-model tests cover classifier transport, not held-out learning quality.

Agreement consumes selected TSV fields: character voice protects speech form from narrator tree; ordinary voice allows tree-form and imperative-act adjustment. Agreement omits means, so its round trip covers only selected fields. Empty witnesses avoid optional policy differences.

Proposed native joins: three real classifiers through all six field alphabets and valid TSV; dictionary persistence/reindex through predictions; previous-act schedule/source identity through predict_seq; schema TSV through agreement voice policy; normalized cues through dictionary representation. Infer supplied-annotation rules are leaf-tested until an actual consumer join is exercised.

Expected cost: bounded existing lookup, fit/predict, schedule and TSV scans. No asymptotic or measured speed gain is predicted. Deterministic gain-ratio accumulation is a reproducibility repair. Confidence has classifier-specific meaning, not calibrated probability. Metrics require aligned vectors and in-range classes; empty input yields zero.

Complement universe: tagged rows/dictionaries/models/schedules/labels. Exclude known values becoming UNSEEN after reindex, tuple changes on valid TSV round trip, future predecessor predictions entering context, or narrator tree overriding character speech. Reverse fibers retain compatible rows/models/schedules/labels; class IDs and projected TSV lose spelling, training histories and means. No unique reconstruction.


## checks

cargo test -p prag --test groupant_wave13. Seven checks pass twice in debug and twice in release; the compiled wrong-schedule adapter is rejected and exact source restored. Partial syntax repeats identically. Evidence and structural-control receipts are in model.md#evidence. Macro bodies and semantic AST analysis remain absent.

## catalog

# Wave13 supplied-input relations

Nine source-bound partial views. Exact supplied domains and observation projections follow. Bounded native evidence is recorded in model.md#evidence; no universal inverse search or general proof is claimed.

## behavior-complement

Type: behavior_complement.

Universe U consists of tagged valid aligned rows, slot dictionaries with unique values, finite trained classifiers, unique valid schedules, closed labels and supported agreement tables. An inner source-derived approximation to U minus R excludes: a known encoded slot becoming UNSEEN after serialize/reindex; a predicted in-range field tuple changing under canonical TSV parse; feedback from an unvisited predecessor; cross-source predecessor links; narrator-tree replacement of character speech form; altered accuracy denominator or inclusion of absent classes in the macro-F1 mean. The finite checks witness these constraints only for their fixtures; malformed dictionaries/models and arbitrary TSV means are outside U.

## reverse-reconstruction

Type: reverse_reconstruction.

For an observed class tuple retain all full-width rows, dictionaries and fitted models that predict that tuple under the supplied schedule. Numeric feature IDs fix values only within a particular dictionary; UNSEEN is a fiber of all unrecorded strings. Schedule outputs retain all compatible prior predictions and fallback branches, not unique text. Agreement's output retains all compatible original labels/witness/tree inputs and every omitted means string. Neither successful serialization nor an equal class tuple reconstructs training history, raw spelling, parse trees or semantic intent. These are converse relations, not implemented universal searches.

## field-transport

Type: closed_class_transport.

For each field f with ordered class list C_f, predict class c then set gives get_f(set_f(l,c))=c for 0<=c<|C_f| while other fields remain. Our native fixture parses q/request/1/0/neg/character/- into literal [1,3,1,0,1,1], trains four identical supplied rows for each of three classifier kinds through fit_set and predicts all six fields through predict_seq, then checks the tuple and canonical TSV. This is real tiny classifier transport, not held-out learning accuracy. Shape, alignment and class bounds remain caller premises.

## dictionary-transport

Type: dictionary_snapshot_equivalence.

For a built dictionary D whose slot values are unique, let J be JSON values-only persistence and rebuild the skipped index by reindex. Then E_D(r)=E_reindex(J(D))(r); reserved acts and start remain known, while unseen spelling remains UNSEEN. Tiny independent fits on both snapshots preserve observed class, confidence bits and explanations. Actual encode mapping is checked, but agreement between fits is a transport observation rather than an independent prediction oracle. IDs from different insertion-order dictionaries are not interchangeable; arbitrary malformed serialized dictionaries are excluded.

## source-context

Type: source_document_predecessor.

For non-Tatoeba item i with ord>1, prev_of chooses the last supplied index with identical Src and doc and ord_i-1, if any. Duplicate coordinates remain last-wins; paragraph changes do not break this document relation. Composition with predict_seq uses the same index-to-row mapping. The seven-item mixed Tale/Gum/Tatoeba fixture asserts exact predecessor indices and resultant observed acts. Such source-scoped identity does not make corpus partitions semantically independent.

## schedule-context

Type: prefix_available_feedback.

For unique schedule pi, let O_t be already produced predictions. Row pi_t gets Act(O_t[p]).name only when p=prev[pi_t] is in O_t; otherwise it gets start. This overrides the supplied row's prev feature without mutating the row. A test observer exposes only this actual eval adapter: forward chain gives Ask/Request/Thank, reverse gives three Ask, suffix gives Ask/Request. It is not a learned classifier. No internal chronological sort, gold label access or future-context inference is asserted.

## agreement-projection

Type: voice_sensitive_label_projection.

Canonical schema TSV rows pass through agree::run and agree::table with empty witnesses and supplied imperative trees. Ordinary voice changes decl/assert to imp/request; character voice retains decl/assert and its emitted character value. Selected output fields remain parseable only after supplying an explicit means placeholder, since the agreement projection drops means. The finite check binds row IDs, voice and class names, not real speaker intent or live witness quality.

## cue-dictionary

Type: normalized_cue_projection.

Supplied forms I think thank you let us go normalize and greedy cue matching yields exactly (0,hedge),(2,thank),(4,suggest). Feeding the selected cue and hedge values into their designated dictionary slots preserves cue=thank and hedge=i while cue=i is UNSEEN. This is an explicit test adapter over public APIs; it does not execute the private parser-backed full feature row projection. Normalization and cue classes lose original spelling and unmatched words.

## score-union

Type: present_class_metric_relation.

For equal-length in-range gold/pred vectors and nclass=4, accuracy uses |gold| and macro-F1 averages only classes present in either vector. Gold [0,0,1,2] against [0,1,1,2] yields (3/4,7/9); against [3,1,1,2] yields (1/2,5/12); empty aligned vectors yield zero. Invalid lengths/classes are excluded because current eval::scores silently zips or omits them. A score pair has many compatible confusion matrices and cannot recover examples or establish generalization.

## evidence

### Wave13 bounded evidence

75 focused native checks pass twice in debug and twice in release with identical sorted names/results: 58 pragmatic unit checks (23 new source tests, eight existing owner tests and 27 previous pragmatic regressions), seven new pragmatic joins and ten previous world joins. These are selected tests, not full suites or corpus exams.

Four owner production mutations fail named assertions (kNN distance shell, schema word boundary, feature question flag, inference history window). The root wrong-schedule adapter also compiles and fails predecessor_schedule_and_subset_have_explicit_start_fallback; exact bytes are restored before positive runs. Owner sidecars preserve original red witnesses and receipts.

Field and metric assertions use literal independent values. Dictionary repetition is transport equivalence, not independent semantic accuracy. The context Observer is a test classifier; separate tiny real fits exercise the actual classifier path. eval/data remain unregistered source dependencies, malformed classifier/TSV states remain domain restrictions, and no live parser/teacher/GPU/corpus/full-school run or speed/accuracy gain is claimed.

## History

### 2026-10-30 · epoch-memory

2026-10-30 prospective design retained below. Tests distinguish independent literal class/metric oracles from transport equivalence and preserve existing parent models.

### 2026-10-30 · document

2026-10-30, base bf3790ad6b8ab9d3d4aa66d9c5be57738ceed66e. Supplied rows/annotations and tiny in-memory training fixtures only; no parser acquisition, corpus evaluation or teacher calls.

### 2026-10-30 · evidence

Heading `### Wave13 bounded evidence — 2026-10-30` stays in evidence as `### Wave13 bounded evidence`.

The harness source SHA-256 is 9991d8a61afc40be64d7828b5411d1cc55d8d49b750db50fe55f85f2c15d51d4. Two independently generated partial syntax artifacts are byte-identical, SHA-256 98747dbfda921f29f82ef34ac78048f70113407ec6c1b9218161df6c749ede2c. Partial syntax is not semantic AST.

External run bundle: $MOVA_DATA/runs/octopus-wave13-integration-2026-10-30. native-evidence-hashes.json SHA-256 5579f525ae5ec216d5ce51ebfc4f90ef1ad24910aca965984222c6c74c5a0337 binds raw native logs and normalized outputs; all-normalized.txt SHA-256 dc4afe9f976b96a8b612728d912982074166dab5f1fd3ea0c51912e6994e3328; adapter-mutant.log SHA-256 ede47849bfecb4f417b084e1397e25bdfd74ae1ba546d8c850618f9eb44f33f2. Structural controls summary SHA-256 a50d4387189f34044579c0d068475201acf1f6e7ee8e3297c0db6909f3180975 records stale-binding and wrong-child rejection with exact restoration.
