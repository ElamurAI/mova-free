# rule-derivation-scan

## descriptant

# rule-derivation-scan

## Forward

Book scan collects packed trees. scan_rounds samples every twentieth record, induces fixed lists, replays accumulated transformations and measures head agreement. Fix counts shortlist 2000 candidates before break scoring. Greedy admission excludes shared class or shared head/direction. Incremental learning caches condition/fix indexes and refreshes whole touched sentences.

## Invariant

Candidates have a first class atom and target within nearest three matching POS words. scan admission requires f/(f+b)>=0.8 and f>b+100. Pre-application counts and the heuristic noninterference predicate do not prove realized joint gain. Malformed trees/overflow excluded.

## Cost

W tokens, max length N, current rules T, list length L: each rederivation pass O(W*(L+T+1)*N). Candidate generation O(W*N) with bounded atoms. Break scoring worst-case O(W*C*N), C<=2000. Pre-shortlist F candidates cost O(F log F) sorting and unbounded-by-C first-pass memory. Mutexes, I/O and dictionary bytes add costs. Incremental refresh scales with whole touched sentences.

## Composition

Lists stay fixed across rounds; current heads rebuild from lists plus ordered transformations. Packed/class dictionaries remain paired. Work is chunked by 4096 records across max(threads,1) workers. Book completion order changes every-twentieth sampling; sorted candidate ties cannot remove that upstream dependence.

## Epoch memory

Initial epoch: 90 assertions over twelve extracted pure source items with minimal enum dependencies. Closed-class erasure mutant rejected. No full learner, parser, GPU, corpus, filesystem-training or external state execution. Source-derived models and proposed oracles are distinct from executed bounded probes. See ../model.md#tests.

## catalog

# rule-derivation-scan typed models

## nonbehavior

Type: behavior_complement.

For fixed dependency states and terminating scan traces, candidate inner absent behaviors include accepting f=100,b=0, taking more than per_round rules in a round, and running rounds+1 reports without early stop but claiming only rounds reports: loop is inclusive 0..=rounds. Source-inspected only; memory stop-gain and prior decision letter alter stopping/round count.

## antecedents

Type: reverse_reconstruction.

Observed report and ordered transformation strings admit all Packed corpora/dictionaries and initial memory states producing those aggregates and strings. Saved bytes omit per-word heads, rejected candidates, decision lists and store query/version. Scan transformations.tsv omits fix/break counts. Thus output is not a unique training checkpoint; retain lexical collisions and latent state. Proposed equal-aggregate corpus oracle unexecuted.

## candidate-search

Type: bounded_search.

Fixes arise at wrong non-root heads with k<=3 gold targets and no candidate cycle. Breaks count correct non-root heads with a different candidate target without the same cycle check. Top 2000 by fixes precede break scoring. Exact-real eligibility is f>=4*b and f>b+100, but Rust uses f64. Sort net gain, condition-count, Tr order, then greedily reject same first atom OR same(head,dir). This does not establish global best or realized gain. Proposed shortlist/tie fixtures unexecuted.

## persistence

Type: failure_boundary.

save writes report.txt then transformations.tsv using ordinary fs::write. Second-write error can leave a new report with stale/partial transformations. Saves follow agreement, early-stop text, learned rules and final exit. Success yields final strings; no atomic pair, fsync durability, replay loader or complete checkpoint follows. stage.note/decide also affect external state. Proposed injected second-write error not executed.

## schedule-cost

Type: schedule_dependence.

scan checks count>=limit once per acquired book, then adds the whole local batch; limits may overshoot. Completion order controls data append and first-encounter class IDs under mutexes, altering data[::20]. Determinism is conditional on fixed ordering/dependencies. Resource preimages retain W,N,T,F,C,threads and I/O rather than inferring corpus size from wall time. Current bounds uncalibrated; no runtime benchmark.

## History

### 2026-10-29 · document

Partial source-derived groupant, 2026-10-29. Full source inspected; this node specializes its explicit syntax envelope. Macro expansion and semantic AST are absent.

Subject: derive.rs SHA-256 2726211da9538f3f5c225292219dfda4d4fd6f15d6b68deaaaf53c834ba60842. Domain: finite admitted inputs with fixed dependency/schedule state. First type is a candidate inner behavior-complement bound in explicit input/output universe, second is an ambiguous compatible-preimage relation. Other models specialize observations or costs. See descriptant for assumptions and ../model.md#tests for evidence; structural acceptance is not semantic proof.
