# world-tabular-rules

Cond, List, covers, learn, learn_k, learn_kc, predict, learn_rare; orchestration run/main_args

## forward

Greedy covering removes matched rows, stores support counts, then prediction selects first match. run/main_args adds IO, alignment and nested CV around the same helpers.

## behavior-complement

Universe U consists of input/output pairs for the scoped deterministic observations with fixed dependency state and successful allocation. N={(x,y) in U | y != F(x)} is the exact complement of the stated model. Correspondence with Rust is a source-derived hypothesis, supported only on named finite tests; unknown IO/dependency outcomes are not declared impossible.

## reverse-reconstruction

B(y)={x in D | F(x)=y}, retaining all compatible state/input antecedents. x belongs to B(F(x)); unique recovery requires injectivity, which the lossy projections lack. Bounded fibers are complete only within their enumerated input domain.

## rules-coupling

Type: constraint_relation. Rules are not invertible training summaries: many datasets induce the same conditions/default. IO, CV quality and CLI argument parsing are source-derived gaps. Domain is the named function scope at this source binding. Status: partial source-derived relation; native evidence only where explicitly named, otherwise a falsifiable prediction.

## invariant

World arrays have one entry per row; constructor links are symmetric, self-free and duplicate-free. Row attributes are deterministic BTreeMaps. Predictions use first-match priority and boolean default; no perfect-training-fit invariant. Numeric sorting requires absence of NaN for nonpanic behavior.

## composition

Greedy covering removes matched rows, stores support counts, then prediction selects first match. run/main_args adds IO, alignment and nested CV around the same helpers. Rules are not invertible training summaries: many datasets induce the same conditions/default. IO, CV quality and CLI argument parsing are source-derived gaps.

## cost

For N rows, H columns and B bytes, splitting is O(B). read_world numeric sorting O(H*N log N); identifier branch recomputes an alphabetic count for every row O(H*N^2). Group expansion bounds groups to <=12, plus linear duplicate membership scans. Let A attributes and R rules: candidate generation O(R*N*sum_{j=1..min(k,3)} choose(A,j)) times BTreeMap comparison costs; rare learning stops at 14 and only enumerates up to pairs. run invokes repeated learners over 10 outer/inner folds and 15 settings. IO costs unmeasured.

## checks

cargo test -p world --lib wave8_models -- --test-threads=1. File-specific filters md::wave8_models and tabular::wave8_models. Tests use native pure functions and synthetic World/Text records, never an annotator or teacher. Evidence and negative controls are in root ../model.md#evidence.

## epoch-memory

First epoch. Prediction: independent bounded encoding/projection observations agree exactly in debug and release, while a compiled production mutation fails the named oracle. Existing lossy behavior is retained explicitly. No numerical surrogate is attached: categorical encoding/report fidelity lacks a justified approximate metric.

## limitations

read_csv does not support quoted multiline records or validate width/quotes. run can panic on empty combined rows or malformed teacher/test indices, and output IDs are not CSV escaped. Numeric NaN may panic in partial_cmp unwrap. Features are inferred jointly on train/test, despite the old module prose claiming train-only. No live teacher, training corpus, file IO or CLI run is exercised.

## History

### 2026-10-30 · epoch-memory

2026-10-30 first epoch. Prediction: independent bounded encoding/projection observations agree exactly in debug and release, while a compiled production mutation fails the named oracle. Existing lossy behavior is retained explicitly. No numerical surrogate is attached: categorical encoding/report fidelity lacks a justified approximate metric.
