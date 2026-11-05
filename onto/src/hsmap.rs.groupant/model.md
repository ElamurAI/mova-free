# onto-hsmap

## descriptant

### onto-hsmap

Candidates search longest NP and spelling variants, shorter preference-fit can override first nonfit. pick=WSD+state+chapter with >1e-9 replacement threshold. Build pass1 chapter distribution/pass2 scoring and constrained confirmations; ordered IsA fallback gates; sequential TSV derivative writes. judge/confirm invoke teacher and remain unexecuted.

## approximant

### Forward relation

Candidates search longest NP and spelling variants, shorter preference-fit can override first nonfit. pick=WSD+state+chapter with >1e-9 replacement threshold. Build pass1 chapter distribution/pass2 scoring and constrained confirmations; ordered IsA fallback gates; sequential TSV derivative writes. judge/confirm invoke teacher and remain unexecuted.

#### Equations

s(c)=WSD(c)+W_HS·1[lf(c)∈prefer]+W_CHAP·chapter_share(lf(c)); b_{i+1}=i if s_i>s_b+1e-9 else b. Qmap(s)=first accepted numeric row; Confirm(k)=last row.

#### Domain

Picks stay in supplied senses; confirmed c can retain old margin/why metadata. Tables trusted and may panic malformed; writes nontransactional.

Categorical behavior and effect boundaries have no validated percentage surrogate. Executable Rust approximant unsupported; 1/2/5/10% future caps require a useful metric first.

## invariant

### Invariant and domain

Picks stay in supplied senses; confirmed c can retain old margin/why metadata. Tables trusted and may panic malformed; writes nontransactional.

#### Limits

Finite synthetic fixtures are not semantic proof. Public invalid indices, allocation failures, huge counters, environment races and live effects outside explicit checks remain unverified.

## cost

### Cost approximation

Scoring O(S), WSD delegated; two mapped passes plus quadratic neighbour pairs per heading and feature scans; retains all rows/descriptions/output. No resource bound.

Symbolic costs only; no timing or throughput claim.

## composition

### Composition

s(c)=WSD(c)+W_HS·1[lf(c)∈prefer]+W_CHAP·chapter_share(lf(c)); b_{i+1}=i if s_i>s_b+1e-9 else b. Qmap(s)=first accepted numeric row; Confirm(k)=last row.

Graph builder→store shape check→Kb; Kb→query→bridge/describe/pron. hs Parsed→hsmap preference/scoring/gates. Core SynMap last-duplicate vs Graph first-duplicate requires unique synset keys for interchangeable lookup. Syntax children below share parent projection; exact byte ranges bind original source semantics.

- selection: score(c)=WSD(c)+1[preferred]+chapter_share(lf(c)); best replaces only >old+1e-9

- isa-gate: Verdict=first witness in ancestry,other-sense,part,taxon,modifier,missing-head,made,source,LCA

## models

## behavior_complement

Unaccepted Q cannot load; later accepted duplicate cannot replace first. Confirmation outside candidates cannot override. Direct ancestry excludes weaker fallback verdict.

## reverse_reconstruction

Pick preimage is score vectors selecting c under scan threshold with reported second/margin. TSV loses source text, dedup levels/bounds; loader histories not recoverable.

## score_ties

First best retained unless >1e-9 improvement; second total_cmp then low index; singleton9.99 sentinel.

## graph_verdict_order

Yes→OtherSense→Part→Taxon(first60)→Modifier→Unknown head→Made→Source→Near/No.

## mapping_identity

Q first accepted numeric row; confirmations last(phrase,chapter/state). live/food keys separate.

## effect_boundary

load_rows external CSV/model; build sequential output; teacher paths excluded; render excludes scores/verdicts.


## tests

### Actual-code checks

- `hsmap::wave29_tests::alignment_first_and_confirmation_last`
- `hsmap::wave29_tests::threshold_preferences_and_graph_join`

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective model saved before edits; nine existing sources read fully. Added bounded inline tests; store reader overflow repaired only after compiled old-code failing witness. No live corpus, teacher, model or database effects executed. Source syntax generated twice identically; partial lexical projection only.

### 2026-10-31 · tests

#### Evidence 2026-10-31

Each selected test passed twice debug and twice release after exact mutation restoration (15 named suite checks; all four sorted outcome sets equal, exits0). Compiled source control: negative-hsmap.log (compiled source mutation assertion failure101). Controls compile successfully and fail an assertion with exit101. Store control is the original unsafe length addition before repair.

Pure selection/graph gate/load temp helpers; live build/teacher/model/CSV unexecuted

Whole CLI, live teacher/corpus/model/prior loading, filesystem failure/durability and huge inputs were not executed. No exact wrapper is advertised as a percentage surrogate. Added test blocks rustfmt-formatted independently; unrelated source formatting preserved. Strict broad Clippy not repeated: existing dependency/source warnings are outside this work.
