# qagen-report-accounting

## forward
Parse ordered Calls, check four u64 sums and finite seconds/cost prefixes, round seconds to tenths and cost to cents, preserve max-seconds fold from zero.

## invariant
Returned aggregate fields are finite and tokens do not wrap. Bypass decimal scaling above MAX/scale where floating spacing exceeds units.

## behavior-complement
Universe: finite supplied values and filesystem observations, return values, errors and writes in this scope. Out-of-range totals yield Err rather than null/modulo values. Finite negative historical values remain accepted.

## reverse-reconstruction
F^-1(y) contains all compatible inputs and initial states yielding y. Aggregates lose row attribution and subprecision values. Floating reordering may change sums, so only equal-rounded-output histories share a fiber. No complete inverse solver or unique recovery claim.

## cost
O(C) passes after O(JSONL bytes) parse; stored calls/text dominate space; rounding constant work.

## composition
read_calls parses finite JSON numbers; sequential sum checks every prefix; checked token closures propagate errors within JSON construction; rounded receives finite values.

## integer-sum
Each mathematical token total <=MAX succeeds exactly; larger totals error.

## floating-prefix
First nonfinite sequential total errors; later negative cancellation cannot rescue it.

## rounding
Ordinary (x*scale).round()/scale retained; extreme magnitudes return x to avoid infinity-to-null.

## historical-fields
Maximum starts at 0.0; caller supplies retries; JSON field names and negative history remain compatible.

## checks
Finite synthetic Rust tests in owned sources; broad offline qagen lib suite twice per profile, explicitly skipping lemma::tests::doubt_with_en_model. Compiled production mutations reach named assertion failures. Exact outcomes and hashes are in the file groupant ../model.md#evidence.

## epoch-memory

Prospective model saved before repairs. Old-code witnesses demonstrated token overflow, JSON null from float overflow, premature report replacement, and masked metadata errors. Ordinary output keys, finite negative history and legacy Option lookup semantics are preserved. Scheduler was not redesigned.

## limitations
Partial source models and finite observations, not proof of all concurrent/filesystem states. Syntax projection is partial; macros opaque. Allocation failure, races and crashes may prevent completion. No live teacher/model/corpus/database was used. No useful certified metric for a numerical surrogate of these error-sensitive operations was identified; executable approximant is absent, with no invented error budget or speed claim.

## History

### 2026-10-30 · epoch-memory

2026-10-30 prospective model saved before repairs. Old-code witnesses demonstrated token overflow, JSON null from float overflow, premature report replacement, and masked metadata errors. Ordinary output keys, finite negative history and legacy Option lookup semantics are preserved. Scheduler was not redesigned.
