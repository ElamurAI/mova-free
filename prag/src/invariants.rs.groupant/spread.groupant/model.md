# prag-invariants-spread

## forward

Spread adds only VerbNet frames with nonempty signature to (verb,signature)->class->ordered (sentence ID, one-based row) lists. counts returns populated groups and groups with multiple distinct class keys. hits selects largest occurrence count, ties to lexicographically smallest class, and emits every losing occurrence in group/class order and insertion order inside each class.

## invariant

No denominator divides counts: plurality counts occurrences, including duplicate add calls/IDs. Empty class remains a class when signature is nonempty. Single-class groups emit no hits. Different verb or signature yields independent groups; no global winner.

## cost

Add O(F(log G+log C)) map work plus copied strings; retained O(A) accepted occurrences. hits scans class counts and occurrence rows, plus repeated detail rendering per losing row. BTree ordering makes group/class output deterministic, not insertion-order independent within a class.

## composition

Public APIs sentence, Spread::add/counts/hits consume the same ordered frames. Native fixtures construct Tree/Frame/Anchor directly. The papers frames-check caller invokes frames_of_tree and sentence before ctx.new_line, then Spread::add on retained frames; caller corpus/teacher/persistence paths are inspected only.

## epoch-memory

Initial source-bound epoch. No production repair was warranted on the scoped valid-input domain. Independent finite frequency and dependency oracles support the existing smaller-class tie, occurrence counts and anchor exclusions. Malformed cached tree/anchor state, heuristic false positives beyond fixtures and lexical dependencies remain open. Costs are prospective source models; no optimization, timing gain or percentage-error guarantee is claimed.

## checks

cargo test -p prag --lib invariants:: -- --test-threads=1; the wave12_models submodule contains new finite checks. File ../model.md#evidence binds old witnesses, repeated profiles and compiled production mutations. Syntax projections are partial and macro bodies opaque.

## behavior-complement

Domain: finite inputs satisfying this fragment's forward premises and the parent API tag/state constraints; output is its scoped observation. Let J_fragment(x,y) impose the concrete relations: Spread adds only VerbNet frames with nonempty signature to (verb,signature)->class->ordered (sentence ID, one-based row) lists. counts returns populated groups and groups with multiple distinct class keys. hits selects largest occurrence count, ties to lexicographically smallest class, and emits every losing occurrence in group/class order and insertion order inside each class. Also require: No denominator divides counts: plurality counts occurrences, including duplicate add calls/IDs. Empty class remains a class when signature is nonempty. Single-class groups emit no hits. Different verb or signature yields independent groups; no global winner. The inner absent-behavior model is {(x,y): not J_fragment(x,y)}. This excludes wrong indices, selections or outputs, rather than treating a scope disclaimer as a behavioral predicate. Finite checks support selected cases; unknown external semantics remain outside this claim.

## reverse-reconstruction

counts=(g,a) retains all class allocations with g nonempty keys and a ambiguous keys; no class proportions can be recovered. Empty hits includes empty input and any single-class-per-group corpus. Reported losers constrain a winner but cannot reconstruct omitted winning IDs or all original frames.

Formally retain every parent-tagged input and compatible intermediate state satisfying the forward relation with the observed fragment output. Do not merge antecedents merely because their projected result agrees; no unique inverse or complete executed solver is claimed.

## intervention

Plurality tie counterfactual switches winner tie order to larger class: finite oracle rejects it. Sixty-four three-class histograms of counts 0..3 check frequency winners, original one-based rows and ignored empty signatures; three independent keys verify grouping.

All typed records bind the exact source revision and scope in manifest.json. Domains, observations and operators are above. Stable dependencies and successful termination are premises. These partial relations retain ambiguous antecedents; no universal solver, unique inverse or actual causal inference is supplied.

## History

### 2026-10-30 · epoch-memory

2026-10-30 initial source-bound epoch. No production repair was warranted on the scoped valid-input domain. Independent finite frequency and dependency oracles support the existing smaller-class tie, occurrence counts and anchor exclusions. Malformed cached tree/anchor state, heuristic false positives beyond fixtures and lexical dependencies remain open. Costs are prospective source models; no optimization, timing gain or percentage-error guarantee is claimed.
