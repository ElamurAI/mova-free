# en-ud-metrics

## descriptant

### ud: metrics

Scores compares optional gold tags/UPOS, universal/full features, lemmas and exact/base arcs. Content CLAS/MLAS/BLEX require attachment correctness, with sorted function-child feature tuples for MLAS. Zero main4 denominator returns0, display absent denominator uses dash.

## approximant

### metrics forward scope

Scores compares optional gold tags/UPOS, universal/full features, lemmas and exact/base arcs. Content CLAS/MLAS/BLEX require attachment correctness, with sorted function-child feature tuples for MLAS. Zero main4 denominator returns0, display absent denominator uses dash.

Composition rejoins the parent at its explicit state/representation boundaries; guarantees apply only to this byte range, not unrelated parent branches.

## invariant

### State and cost

Ud maps hashed backoff keys to UPOS/FEATS counts. View carries form/tag/relation/child flags/agreement. Scores accumulates denominators and exact/base label/content counts.

Missing UPOS gold cannot count as UPOS hit. No FEATS layer means no FEATS denominator increments and no MLAS hit. Out-of-range heads are ignored by children; malformed parallel arrays still may panic elsewhere. Unknown table keys cannot manufacture trained features.

kids_of O(n^2) scans plus n bounded ancestor walks; agreement repeated per verb can be O(n^2). Scores functional child collection/sorting may cost O(n^2 log n) across content words. Training count u32 and metric usize arithmetic have no checked overflow guarantee.

## models

## behavior_complement

Missing UPOS gold cannot count as UPOS hit. No FEATS layer means no FEATS denominator increments and no MLAS hit. Out-of-range heads are ignored by children; malformed parallel arrays still may panic elsewhere. Unknown table keys cannot manufacture trained features.

## reverse_reconstruction

Given predicted UPOS/FEATS, compatible antecedents include all Views and tables selecting those values via any sufficient backoff key; output alone cannot recover key level/counts. Scores admit many gold/pred sequences with identical counts; main4 percentages further collapse scale and zero denominators.

## agreement_scope

Subject precedence is own nsubj/csubj then expl; aux/cop use head, conj climbs, relative pronoun may use acl:relcl antecedent. Coordination and first-conj form enter hash.

## feature_ties

Count threshold2 is on total examples, not winning count. UPOS lower index wins equal counts; FEATS lower bitset wins. Hash collisions remain possible.

## metric_units

main4 returns percentages, zero denominators=>0. row uses em dash for absent denominators and content F1 with max1 denominators.

## coordinate_flags

heads1..n/root0; children returns zero-based indices. Subject/aux/passive/fixed bits1/2/4/8; conj propagation uses own snapshot, chain bounded n.

## tests

### Evidence

Compiled source mutants cover construct rounded-floor boundary, parse root legality and store checksum gate. Other sources have no compiled source mutation claim. Preflight empty-model nonempty tagging failed in ptag; synthetic fixtures adjusted without production changes. Existing corpus/seeds tests not selected.

Partial source-bound model, not full proof. No useful executable percentage surrogate established for these categorical decisions/effects; budgets1/2/5/10% require a future observation metric. No corpus/API/GPU/teacher, allocation fault injection, adversarial size exhaustion or broad accuracy claim.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31. Exact byte range/hash in manifest. Partial lexical AST is shared with parent; no macro expansion, semantic proof or executable percent surrogate. Parent tests exercise selected reachable paths, not every statement in this fragment.

### 2026-10-31 · tests

Actual source test: ud::wave27::bounded_contract. Command: cargo test -p en --lib wave27 -- --test-threads=1 (debug twice, release twice). External evidence: $MOVA_DATA/runs/octopus-wave27-structure-2026-10-31. Nine focused tests, each in its owned production module; no copied implementation.
