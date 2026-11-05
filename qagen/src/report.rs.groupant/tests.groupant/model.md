# qagen-report-tests

## forward
Synthetic JSONL checks four token overflow columns, both float overflow fields, MAX rounding, negative history, 13 independent small-oracle lists and collect artifact preservation.

## invariant
Collect fixtures use V1 and empty paragraphs so no model loads; temp paths isolate writes.

## behavior-complement
Universe: finite supplied values and filesystem observations, return values, errors and writes in this scope. No live model/teacher/corpus. Empty-paragraph collect fixtures do not establish all nonempty gate/statistical joins.

## reverse-reconstruction
F^-1(y) contains all compatible inputs and initial states yielding y. Outcome lists constrain finite cases only. Failure names identify tested mutations, not a unique cause in every program. No complete inverse solver or unique recovery claim.

## cost
13 lists of length 0..12 plus constant overflow/edge/preflight fixtures.

## composition
append_call supplies ordinary JSONL consumed by read_calls; malformed record deliberately bypasses writer; sentinel tests join validation failures to unchanged bytes.

## oracle
Small token sums use independent closed form n(n-1)/2.

## boundary
Single u64::MAX accepted, doubled rejected; f64::MAX finite alone, doubled rejected; negative fractional history preserves rounding.

## artifact-witness
Malformed and overflowing calls each preserve all three sentinel files.

## mutation-gate
Compiled sum, finite-check, rounding and early-write mutations fail named assertions; source restored before positives.

## checks
Finite synthetic Rust tests in owned sources; broad offline qagen lib suite twice per profile, explicitly skipping lemma::tests::doubt_with_en_model. Compiled production mutations reach named assertion failures. Exact outcomes and hashes are in the file groupant ../model.md#evidence.

## epoch-memory

Prospective model saved before repairs. Old-code witnesses demonstrated token overflow, JSON null from float overflow, premature report replacement, and masked metadata errors. Ordinary output keys, finite negative history and legacy Option lookup semantics are preserved. Scheduler was not redesigned.

## limitations
Partial source models and finite observations, not proof of all concurrent/filesystem states. Syntax projection is partial; macros opaque. Allocation failure, races and crashes may prevent completion. No live teacher/model/corpus/database was used. No useful certified metric for a numerical surrogate of these error-sensitive operations was identified; executable approximant is absent, with no invented error budget or speed claim.

## History

### 2026-10-30 · epoch-memory

2026-10-30 prospective model saved before repairs. Old-code witnesses demonstrated token overflow, JSON null from float overflow, premature report replacement, and masked metadata errors. Ordinary output keys, finite negative history and legacy Option lookup semantics are preserved. Scheduler was not redesigned.
