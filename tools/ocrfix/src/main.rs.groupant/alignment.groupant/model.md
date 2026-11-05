# ocrfix-text-command-alignment

## descriptant

### ocrfix-text-command-alignment

#### Descriptant

Full source read preceded changes. Prospective model predicted branch/order, resource and information-loss behavior. Tests exercise actual helpers on bounded synthetic inputs; command workflows remain unexecuted. Production source prefix is preserved; only inline tests added.

Initialize d[i,0]=i,d[0,j]=j; d[i,j]=min(diag+mismatch,up+1,left+1). Backtrace diagonal if equal, else deletion if equal, else insertion; accumulate gt->hyp substitution multiset.

Executable percentage approximation is unsupported: categorical parsing/effects have no useful certified metric here. Partial syntax excludes macro expansion and name resolution.

## approximant

### Forward relation

Initialize d[i,0]=i,d[0,j]=j; d[i,j]=min(diag+mismatch,up+1,left+1). Backtrace diagonal if equal, else deletion if equal, else insertion; accumulate gt->hyp substitution multiset.

#### Domain

Finite char slices with n,m and DP increments fitting u32 and allocation capacity.

## models

# Typed source models

All models are partial source-derived relations at this manifest snapshot. Scope is one call/trace under the stated domain; no universal semantic proof.

## behavior_complement

Identity slices exclude positive distance; empty gt excludes substitutions/deletions; empty hyp excludes substitutions/insertions.

Domain: Finite char slices with n,m and DP increments fitting u32 and allocation capacity. Observation: s+i+d=d[n,m]; n-d+i=m; sum(pair counts)=s. Exact comparisons on bounded u32 DP cells; diagonal priority fixes an alignment, not necessarily unique. Assumptions: bounded resources and source branch semantics. Oracle: selected actual-helper tests in model.md#tests constrain only their finite fixtures; remaining branches are source-derived.

## reverse_reconstruction

Counts and pair multiset admit many string pairs and placements. Given full DP and slices, chosen backtrace fixed, but minimal alternative alignments may exist.

Domain: Finite char slices with n,m and DP increments fitting u32 and allocation capacity. Observation: s+i+d=d[n,m]; n-d+i=m; sum(pair counts)=s. Exact comparisons on bounded u32 DP cells; diagonal priority fixes an alignment, not necessarily unique. Assumptions: bounded resources and source branch semantics. Oracle: selected actual-helper tests in model.md#tests constrain only their finite fixtures; remaining branches are source-derived.

## length_conservation

Every diagonal consumes1 each; deletion gt only; insertion hyp only, so m-n=i-d.

Domain: Finite char slices with n,m and DP increments fitting u32 and allocation capacity. Observation: s+i+d=d[n,m]; n-d+i=m; sum(pair counts)=s. Exact comparisons on bounded u32 DP cells; diagonal priority fixes an alignment, not necessarily unique. Assumptions: bounded resources and source branch semantics. Oracle: selected actual-helper tests in model.md#tests constrain only their finite fixtures; remaining branches are source-derived.

## tie_priority

At equal optimum diagonal beats deletion, deletion beats insertion; pair histogram is tie-policy dependent though distance is not.

Domain: Finite char slices with n,m and DP increments fitting u32 and allocation capacity. Observation: s+i+d=d[n,m]; n-d+i=m; sum(pair counts)=s. Exact comparisons on bounded u32 DP cells; diagonal priority fixes an alignment, not necessarily unique. Assumptions: bounded resources and source branch semantics. Oracle: selected actual-helper tests in model.md#tests constrain only their finite fixtures; remaining branches are source-derived.

## substitution_histogram

Only unequal diagonal increments both s and directed pair(gt,hyp); equal diagonal consumes without record.

Domain: Finite char slices with n,m and DP increments fitting u32 and allocation capacity. Observation: s+i+d=d[n,m]; n-d+i=m; sum(pair counts)=s. Exact comparisons on bounded u32 DP cells; diagonal priority fixes an alignment, not necessarily unique. Assumptions: bounded resources and source branch semantics. Oracle: selected actual-helper tests in model.md#tests constrain only their finite fixtures; remaining branches are source-derived.

## resource_boundary

Allocate(n+1)*(m+1) cells before filling; casts and increments require bounded lengths. Allocation failure/overflow are outside exact finite model.

Domain: Finite char slices with n,m and DP increments fitting u32 and allocation capacity. Observation: s+i+d=d[n,m]; n-d+i=m; sum(pair counts)=s. Exact comparisons on bounded u32 DP cells; diagonal priority fixes an alignment, not necessarily unique. Assumptions: bounded resources and source branch semantics. Oracle: selected actual-helper tests in model.md#tests constrain only their finite fixtures; remaining branches are source-derived.


## tests

### Evidence

Selected exact tests: ["tests::brackets_rejoined","tests::stream_and_cer","tests::channel_corrects_and_keeps","tests::negative_control_known_word_stays","wave33::channel_depth_alignment_and_masks"].

Each runs twice in debug and twice in release with normalized named outcomes compared. Compiled production-boundary mutants are recorded in the external wave33 text receipts with original/restored hashes and positive reruns. Existing negative assertions are separate from compiled mutants. No CLI, download, teacher, corpus or model execution. For this fragment, source-derived paths not reached by these tests remain unverified.

## invariant-and-cost

### Invariant and cost

#### Invariant

s+i+d=d[n,m]; n-d+i=m; sum(pair counts)=s. Exact comparisons on bounded u32 DP cells; diagonal priority fixes an alignment, not necessarily unique.

#### Cost

O((n+1)(m+1)) time and u32 memory, substitution map O(s log distinct); no linear-space claim.

Arithmetic and allocation premises are conditional. No timing or percentage surrogate claimed.

## History

### 2026-10-31 · descriptant

Heading `#### Epoch 2026-10-31` stays in descriptant as `#### Descriptant`.
