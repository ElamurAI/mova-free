# agent-mova-cli

## model

# Typed capability CLI model
## Forward
Domain: argv and external bounded JSON. Observation: process status, stdout report and output artifacts. Dispatch recognizes capabilities, example, trace, prepare-synthetic-recall, recall and collect only at their exact arities. JSON decoding and the typed library determine all stateful semantics.
## Composition
CLI reads task/index bytes with the bounded descriptor helper, decodes explicit public types and calls the matching capability. Recall takes both state and parent report paths. No command constructs a shell string, invokes a teacher or schedules a daemon. Preparation's four location alternatives are explicitly synthetic; general callers build RecallTask directly.
## Invariant
Unknown command or wrong arity returns usage failure. A successful trace/collection stdout corresponds to the returned typed report; exclusive artifact failures return nonzero. The wrapper cannot upgrade uncertainty to supported and has no automatic retry.
## Cost
Argv storage is O(argument bytes); task/index reads are capped at 256 KiB. Native operation and stdout serialization dominate later cost. No CLI throughput comparison is claimed.
## Descriptant
This file exposes a bounded synchronous protocol, not the persistent role runner. External queue reservations, task policies and private inputs belong to the caller. Linux native snapshot compatibility follows the library. Capability enumeration lists only implemented operations.
## behavior_complement
Domain: argv dispatch. Excluded successful operations include unrecognized command strings, wrong arity, JSON decode failure and any returned library error. Counterexample to no-side-effects-on-error: trace may create partial artifacts before a later I/O failure. Evidence: source inspection and finite successful/negative CLI probes, not exhaustive argv enumeration.
## reverse_reconstruction
Domain: stdout/status/artifacts. A successful JSON report identifies a capability and typed result, but many JSON whitespace encodings and path spellings can yield it. Usage errors likewise admit many unrelated argv antecedents. No unique argv reconstruction is claimed. Evidence: source-derived preimage model.
## retention_lineage
Recall argv must provide task, state, parent trace report and new report destination. Removing the parent pin cannot use the recall branch at valid arity. A valid four-argument shape still does not prove matching bytes: the library checks them. Evidence: two fresh recall processes and wrong-state negative control.
## context_exclusion
Trace JSON carries bounded source identity and speaker markers to the library without dropping them. This wrapper supplies no dialogue interpretation. Counterexample to a parser-quality claim: a source window may be syntactically valid yet semantically ambiguous. Evidence: actual speaker-tagged process and conservative library model.
## error_partition
Read/decode failure precedes capability invocation; native returned uncertainty is a successful process carrying explicit uncertainty; native Result::Err is a failing process. These observations must not be collapsed into a Boolean semantic-success label. Evidence: unknown-subject successful/uncertain output versus wrong-hash failing process.

## checks

### Checks and evidence

This is finite synthetic evidence. No teacher call, full corpus, held-out assessment, daemon restart or learned general planner was exercised. No general semantic proof or speed comparison is supplied. Initial yanked-dependency lock resolution and three local compile errors were corrected before successful builds; their external logs remain preserved.

## History

### 2026-11-01 · checks

2026-11-01: offline locked two-job debug build; five unit tests passed. Two fresh CLI traces and two fresh recalls were byte-identical. Trace added one graph frame and changed zero weights; recall changed neither. Unknown subject and speaker-tagged context were uncertain, without checkpoint publication. Wrong state/window hashes and native outcome substitution were rejected. Native table collection retained three reports including uncertainty and reproduced identical bytes.
