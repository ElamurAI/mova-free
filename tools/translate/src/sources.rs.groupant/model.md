# translation-flow-sources

## descriptant

### translation-flow-sources

Kind selects Bible split by book, Docs split by document modulo10, TrainOnly always train. Bible discourse=(book,0), others=(group,document). Docs sampling keeps floor(document/10)%n==0; other kinds always sampled. Lessons sort courses/files and extract register-specific lines, treebank_texts associates preceding #text with encountered #sent_id, pool_texts enumerates tab-bearing lines into groups1000.

## approximant

### Forward relation

Kind selects Bible split by book, Docs split by document modulo10, TrainOnly always train. Bible discourse=(book,0), others=(group,document). Docs sampling keeps floor(document/10)%n==0; other kinds always sampled. Lessons sort courses/files and extract register-specific lines, treebank_texts associates preceding #text with encountered #sent_id, pool_texts enumerates tab-bearing lines into groups1000.

Scope: source-derived conditional transition model at the manifest SHA. Docs sampling n>0, identifiers fit u8/u16 to avoid cast collisions; treebank per-book counts below65536. Directory/order behavior source-derived only. The English-course predicate excludes grammar-* except grammar-en*, torah-he, queue, lessons.

## composition

### Composition

Scoped children are projections of this file, not independent executables. Main population filtering feeds aggregation; order/text helpers feed measure's exact decoder; many's agreement and storage read the same MTok with intentionally different punctuation treatment, streaming accounting consumes reader costs, clustering consumes normalized PairM. Uncovered branches remain source-derived; finite helper checks do not establish orchestration wiring.

## catalog

# Typed models

All entries bind the source SHA and optional byte range in manifest.json. Observation is one admitted helper call or the explicitly described orchestration trace. Relations are partial source-derived models, not universal proofs. Finite test names and unexecuted effects are in model.md#tests.

## behavior_complement

TrainOnly cannot select held-out split and Bible cannot be excluded by sampled even n0. Docs sample0 is outside domain and can panic. Nonmatching register lines never enter output map.

Domain and held-fixed assumptions: the invariant/forward documents for this exact scope. Status: partial source-derived relation; complement is a conditional inner exclusion, reverse retains ambiguity, other types specify the named projection/intervention. Oracle: relevant named synthetic tests when listed; otherwise source inspection only.

## reverse_reconstruction

A split label admits modulo residue classes; group cannot recover verse/unit coordinate. Pool output loses original ids and rejected lines. Treebank output can reflect empty text when sent_id precedes text, so it does not invert conventional CoNLL ordering generally.

Domain and held-fixed assumptions: the invariant/forward documents for this exact scope. Status: partial source-derived relation; complement is a conditional inner exclusion, reverse retains ambiguity, other types specify the named projection/intervention. Oracle: relevant named synthetic tests when listed; otherwise source inspection only.

## document_residues

Docs test3/8,dev5 on document index, independent group/book. Bible uses first coordinate; conflating them causes split leakage.

Domain and held-fixed assumptions: the invariant/forward documents for this exact scope. Status: partial source-derived relation; complement is a conditional inner exclusion, reverse retains ambiguity, other types specify the named projection/intervention. Oracle: relevant named synthetic tests when listed; otherwise source inspection only.

## sampling_stride

Docs admits documents in decadal bands numbered multiples of n; sampling is not every nth verse. n0 invalid only on Docs branch.

Domain and held-fixed assumptions: the invariant/forward documents for this exact scope. Status: partial source-derived relation; complement is a conditional inner exclusion, reverse retains ambiguity, other types specify the named projection/intervention. Oracle: relevant named synthetic tests when listed; otherwise source inspection only.

## register_projection

Plain consumes explain/fact, Talk test and Exam exam; question register strips answer suffix at first ' | ' and leading stars/spaces, losing answer text.

Domain and held-fixed assumptions: the invariant/forward documents for this exact scope. Status: partial source-derived relation; complement is a conditional inner exclusion, reverse retains ambiguity, other types specify the named projection/intervention. Oracle: relevant named synthetic tests when listed; otherwise source inspection only.

## coordinate_capacity

lessons casts course/document/line indices to u8/u16; treebank increments u16 counts. Outside bounded representation collisions or debug overflow differ from mathematical unbounded enumeration.

Domain and held-fixed assumptions: the invariant/forward documents for this exact scope. Status: partial source-derived relation; complement is a conditional inner exclusion, reverse retains ambiguity, other types specify the named projection/intervention. Oracle: relevant named synthetic tests when listed; otherwise source inspection only.


## tests

### Evidence and gaps

#### Native checks

- `sources::wave32::split_sample_and_course`

#### Controls

Five compiled source-behavior mutants: main gate counter suppressed; many strict chapter comparison relaxed; measure noncanonical-gap recording disabled; pages Meta filter reversed; blocks literal-change predicate reversed. All failed the corresponding assertion and passed after original bytes/hash restoration. Zero assertion-only mutants; zero surviving exploratory mutants.

#### Limitations

CLI argument/effect paths, live corpus/model/annotation/training, full Run::verse, page filesystem and reports remain source-derived only. Source syntax projections generated twice and byte-compared. No corpus-quality, persistent-learning, throughput or universal semantic coverage claim.

## invariant-and-cost

### Domain, invariant and cost

#### Invariant

Docs sampling n>0, identifiers fit u8/u16 to avoid cast collisions; treebank per-book counts below65536. Directory/order behavior source-derived only. The English-course predicate excludes grammar-* except grammar-en*, torah-he, queue, lessons.

#### Cost

Splits/groups O(1). Lessons O(bytes+courses log courses+files log files) and output map inserts; treebank O(bytes+Nlog books+Nlog output); pool O(bytes+Nlog N) map construction. External reads/lesson workflows not executed.

Integer additions/casts require representability; finite floating inputs where comparisons are modeled. These symbolic costs are not measurements of corpus throughput. No useful cheaper executable percentage surrogate established; future supported caps only 1/2/5/10%.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective predictions saved before edits in the external wave32 tr_flow receipts. Original production bytes preserved; only appended synthetic test modules added. Selected actual helpers exercised; five compiled behavior controls rejected with exact byte restoration and positive reruns. Source syntax projection is partial, not expanded semantic Rust. Diagnostic reconstruction with residual is not semantic understanding or persistent learning.

Docs sampling n>0, identifiers fit u8/u16 to avoid cast collisions; treebank per-book counts below65536. Directory/order behavior source-derived only. The English-course predicate excludes grammar-* except grammar-en*, torah-he, queue, lessons.

### 2026-10-31 · tests

These selected names ran twice debug and twice release with exact filtering and one test thread; raw logs and numeric status files are in the external octopus-wave32-tr_flow-2026-10-31 receipt directory. All test bodies inspected before selection. Inline cases use only synthetic in-memory records; the existing blocks filesystem test was not selected. Tests listed at file level do not execute every function or this entire scoped fragment.
