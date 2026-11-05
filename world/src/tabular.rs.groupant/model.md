# world-tabular

## model

# world-tabular

Permissive CSV records, transductive row-world features and deterministic explained decision lists.

## forward

split_csv is a quote-toggle scanner: commas split only outside quotes, doubled quotes within quoted mode emit one quote, unmatched quotes are tolerated. read_csv removes blank physical lines, takes the first as header, and parses each remaining physical line independently. read_world selects name (>half), numeric (>=90% parseable and >12 distinct), category (<=12 distinct), or identifier paths in that order. learn_kc greedily chooses Laplace accuracy above majority+0.02 with support/min and cap; predict returns the first covering rule else default. run aligns test columns, builds joint train/test features, tunes folds and optionally writes output.

## behavior-complement

Universe U consists of input/output pairs for the scoped deterministic observations with fixed dependency state and successful allocation. N={(x,y) in U | y != F(x)} is the exact complement of the stated model. Correspondence with Rust is a source-derived hypothesis, supported only on named finite tests; unknown IO/dependency outcomes are not declared impossible.

## reverse-reconstruction

B(y)={x in D | F(x)=y}, retaining all compatible state/input antecedents. x belongs to B(F(x)); unique recovery requires injectivity, which the lossy projections lack. Bounded fibers are complete only within their enumerated input domain.

## csv-fiber

Type: reverse_reconstruction. a, quoted a and either unmatched edge quote all parse to one a field. Reconstruction retains all such syntactic antecedents. Encoding each of three fields with surrounding quotes and doubled internal quotes roundtrips all 343 triples from seven LF-free field values. Domain is the named function scope at this source binding. Status: partial source-derived relation; native evidence only where explicitly named, otherwise a falsifiable prediction.

## line-boundary

Type: error_partition. split_csv accepts embedded LF as data, but read_csv splits physical lines before invoking it. Thus helper roundtrips do not establish file-level multiline CSV support. Blank lines are omitted, whitespace inside nonblank fields is retained. Domain is the named function scope at this source binding. Status: partial source-derived relation; native evidence only where explicitly named, otherwise a falsifiable prediction.

## feature-loss

Type: information_loss. Empty categorical values and literal unknown collapse. Numeric values collapse into four quantile bins; names discard given names and retain title meaning plus family/fare linkage. Native fixtures check empty/unknown equality and ranks at 12/13/20 distinct values. Domain is the named function scope at this source binding. Status: partial source-derived relation; native evidence only where explicitly named, otherwise a falsifiable prediction.

## threshold-partition

Type: constraint_relation. Name detection precedes numeric/category tests. Exactly 12 distinct values are categories; 13 finite distinct numeric values can enter quartile bins. Cuts use floor((m-1)*p) and strictly greater comparison, so values equal to a cut stay in the lower bin. Domain is the named function scope at this source binding. Status: partial source-derived relation; native evidence only where explicitly named, otherwise a falsifiable prediction.

## group-observation

Type: encoding_relation. group_attr filters links to women/children then known outcomes: zero known => none known; all true => all had it; all false => none had it; both => mixed. Nine ternary assignments over two peers test all partitions; own known label has no effect on constructor links. Domain is the named function scope at this source binding. Status: partial source-derived relation; native evidence only where explicitly named, otherwise a falsifiable prediction.

## selection-priority

Type: priority_relation. learn_kc maximizes smoothed majority precision, then fewer conditions, then lexicographically smaller conditions; ties predict false. predict preserves rule order. learn_rare maximizes positive precision then positive support then condition order and defaults false. Domain is the named function scope at this source binding. Status: partial source-derived relation; native evidence only where explicitly named, otherwise a falsifiable prediction.

## io-dependency

Type: dependency_boundary. run reads local teacher pseudo-label CSV only when provided; it does not call a remote teacher. Header alignment and features precede CV, while fold labels are restricted. The parent must distinguish feature transduction from outcome leakage and pure helpers from IO. Domain is the named function scope at this source binding. Status: partial source-derived relation; native evidence only where explicitly named, otherwise a falsifiable prediction.

## invariant

World arrays have one entry per row; constructor links are symmetric, self-free and duplicate-free. Row attributes are deterministic BTreeMaps. Predictions use first-match priority and boolean default; no perfect-training-fit invariant. Numeric sorting requires absence of NaN for nonpanic behavior.

## composition

Fragments share exact source state; helper output feeds rendering/features/rules in source order. Compose relations by existential intermediate state, preserving shared identities and caller options. Do not take independent products of projected fibers or add overlapping costs. Inline tests are additional root scope.

## cost

For N rows, H columns and B bytes, splitting is O(B). read_world numeric sorting O(H*N log N); identifier branch recomputes an alphabetic count for every row O(H*N^2). Group expansion bounds groups to <=12, plus linear duplicate membership scans. Let A attributes and R rules: candidate generation O(R*N*sum_{j=1..min(k,3)} choose(A,j)) times BTreeMap comparison costs; rare learning stops at 14 and only enumerates up to pairs. run invokes repeated learners over 10 outer/inner folds and 15 settings. IO costs unmeasured.

## checks

cargo test -p world --lib wave8_models -- --test-threads=1. File-specific filters md::wave8_models and tabular::wave8_models. Tests use native pure functions and synthetic World/Text records, never an annotator or teacher. Evidence and negative controls are in root model.md#evidence.

## epoch-memory

First epoch. Prediction: independent bounded encoding/projection observations agree exactly in debug and release, while a compiled production mutation fails the named oracle. Existing lossy behavior is retained explicitly. No numerical surrogate is attached: categorical encoding/report fidelity lacks a justified approximate metric.

## limitations

read_csv does not support quoted multiline records or validate width/quotes. run can panic on empty combined rows or malformed teacher/test indices, and output IDs are not CSV escaped. Numeric NaN may panic in partial_cmp unwrap. Features are inferred jointly on train/test, despite the old module prose claiming train-only. No live teacher, training corpus, file IO or CLI run is exercised.

## evidence

### tabular correspondence evidence

#### Finite domains

Markdown: 259 unique strings over six scalars through length three, independent
scalar expansion oracle, exact two-element newline/space fiber, and a
non-idempotence witness. Empty v2/no-Story and v3 reports check fallback,
diagnostics and eight ordered sections/root suffix. These do not exercise
nonempty registry/state/history rendering or arbitrary qa3 behavior.

CSV: 343 quoted three-field records over seven LF-free fields roundtrip through
an independently defined encoder. Four encodings/unbalanced quote forms share
one parsed result; multiline helper acceptance is explicitly distinct from
physical-line file reading. Feature checks cover 12/13/20 distinct finite
numeric values, categorical unknown loss, raw age override, a three-person
family graph and nine ternary known-outcome assignments. A twenty-row decision
list witness checks tie priority, one-rule cap, default and first-match behavior.
These tests do not evaluate corpus quality, IO, full CV or external models.

#### Source and structural checks

#### Epoch correction and limits

The first debug run passed eight tests and failed the fallback fixture:
World::new initializes Story even in v2. The fixture now explicitly removes
Story; production behavior was correct and unchanged. The model conditions
fallback on Story absence, not v2 mode. Preserve that distinction in parent wiring.

read_csv does not support quoted multiline records or validate width/quotes. run can panic on empty combined rows or malformed teacher/test indices, and output IDs are not CSV escaped. Numeric NaN may panic in partial_cmp unwrap. Features are inferred jointly on train/test, despite the old module prose claiming train-only. No live teacher, training corpus, file IO or CLI run is exercised.
No live teacher, parser loading, GPU, training data or network operation was used.
Mathematical views are source-derived hypotheses supported by these finite
observations; no executable percentage-error surrogate is justified.

## History

### 2026-10-30 · epoch-memory

2026-10-30 first epoch. Prediction: independent bounded encoding/projection observations agree exactly in debug and release, while a compiled production mutation fails the named oracle. Existing lossy behavior is retained explicitly. No numerical surrogate is attached: categorical encoding/report fidelity lacks a justified approximate metric.

### 2026-10-30 · evidence

2026-10-30, first epoch. Production behavior is unchanged; only native tests were appended.
Source binding and partial syntax are in manifest.json and syntax.json. Fragment
ranges bind named function envelopes; gaps and cross-fragment calls are described
in their models. This is not semantic AST analysis or general verification.

#### Repeated profiles

Command: `cargo test -p world --lib wave8_models -- --test-threads=1`,
and the same command with `--release`. Two successful debug runs and two
release runs each passed nine tests (four md and five tabular). Normalizing
to ordered test identities and results yielded identical bytes in all four:
SHA-256 `7c21f5bccd10b254d87e87ac68a2a048ccb9841f27afb70034f84fd6539bf404`.
Durations and compiler warnings were excluded; no speed claim follows.

Relative external run names: `octopus-wave8-formats-2026-10-30/debug1-restored.log`,
`debug2.log`, `release1.log`, `release2.log` and corresponding
`.normalized` files. Raw logs are outside the repository.

#### Compiled controls

One compiled test binary contained two production mutations: cell LF replacement
space to underscore, and quartile comparison greater-than to greater-or-equal.
Named assertions `md::wave8_models::cell_bounded_oracle_and_loss_fibers` and
`tabular::wave8_models::numeric_quartiles_and_category_threshold` failed;
the remaining seven tests passed. Both sources were restored before any result
assertion; a subsequent native run passed all nine.
Raw `mutants.log` SHA-256:
`7f4658d1349eba594b5d73f11d047f592ef8941cec1f52143e75cd83284b71e3`.

Two independently generated syntax projections per file matched byte-for-byte.
This file's syntax SHA-256:
`f9a8d4e57f1a138f07882cc4d2356eea2954da606a1c9b6918f530b54c04c3b2`.
An external copy of the complete registry with these two entries found no
diagnostics in either file tree. Its only three diagnostics were the expected
root child-list mismatch and two missing transitive root source bindings.
Ancestor/registry integration belongs to the parent task; global validity is
not claimed here. A minimal registry was unsuitable because validation scans
other existing sidecars; that failed setup was replaced with the full copy.
