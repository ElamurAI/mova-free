# onto-query

## descriptant

### onto-query

lookup explicit ID/POS→merged exact/lower/capitalized→first successful noun plural fallback. Context/core/prior rank retains within0.3. ask resolves first --layer before command. Property verdict preserves Kb distinction. parts sorted ancestor(depth,id), PartOf<=3, MemberOf/MadeOf depth0.

## approximant

### Forward relation

lookup explicit ID/POS→merged exact/lower/capitalized→first successful noun plural fallback. Context/core/prior rank retains within0.3. ask resolves first --layer before command. Property verdict preserves Kb distinction. parts sorted ancestor(depth,id), PartOf<=3, MemberOf/MadeOf depth0.

#### Equations

lookup=ordered_unique_senses(exact,lower,capitalized) ⊕ first successful noun singular candidate. rank retains s(c)≥max(s)-0.3. Parts=first unique(part,rel) across sorted ancestor owners and≤3 incoming part edges.

#### Domain

Valid lookup ordering/nonempty senses assumed. IsA miss No, parts miss Unknown. Dedup(part,rel) retains first owner/path. Ranking may load global prior; one-sense fixture avoids.

Categorical behavior and effect boundaries have no validated percentage surrogate. Executable Rust approximant unsupported; 1/2/5/10% future caps require a useful metric first.

#### Ranking score domain

Multi-candidate ranking claims additionally require finite per-candidate scores (finite prior shares and finite weighted sums). The prior parser can accept NaN: total_cmp may place NaN first, then every `score >= best - 0.3` comparison fails, producing an empty ranked vector; isa/has_part can subsequently panic at cands[0]. Nonfinite scores are an explicit unverified failure domain, not a successful unknown-answer policy. Executed query fixtures have one sense and bypass rank_candidates scoring/global prior loading.

## invariant

### Invariant and domain

Valid lookup ordering/nonempty senses assumed. IsA miss No, parts miss Unknown. Dedup(part,rel) retains first owner/path. Ranking may load global prior; one-sense fixture avoids.

#### Limits

Finite synthetic fixtures are not semantic proof. Public invalid indices, allocation failures, huge counters, environment races and live effects outside explicit checks remain unverified.

#### Ranking score domain

Multi-candidate ranking claims additionally require finite per-candidate scores (finite prior shares and finite weighted sums). The prior parser can accept NaN: total_cmp may place NaN first, then every `score >= best - 0.3` comparison fails, producing an empty ranked vector; isa/has_part can subsequently panic at cands[0]. Nonfinite scores are an explicit unverified failure domain, not a successful unknown-answer policy. Executed query fixtures have one sense and bypass rank_candidates scoring/global prior loading.

## cost

### Cost approximation

Lookup sense dedup O(S²); rank O(S log S); ancestry O(Va+Ea), repeated target BFS and broad3-level parts product costs; output-sized proof memory.

Symbolic costs only; no timing or throughput claim.

## composition

### Composition

lookup=ordered_unique_senses(exact,lower,capitalized) ⊕ first successful noun singular candidate. rank retains s(c)≥max(s)-0.3. Parts=first unique(part,rel) across sorted ancestor owners and≤3 incoming part edges.

Graph builder→store shape check→Kb; Kb→query→bridge/describe/pron. hs Parsed→hsmap preference/scoring/gates. Core SynMap last-duplicate vs Graph first-duplicate requires unique synset keys for interchangeable lookup. Syntax children below share parent projection; exact byte ranges bind original source semantics.

- lookup: Found=merge(exact,lower,capitalized) else first successful plural candidate for nouns

- meronymy: owners=sort ancestors(depth,Cid); walk incoming PartOf<=3, MemberOf/MadeOf at depth0

## models

## behavior_complement

Unknown word excludes guessed Boolean; no part witness excludes No; direct form excludes plural; MemberOf cannot recursively traverse.

## reverse_reconstruction

Found allows case/underscore aliases and plural forms with earlier fallback misses. Yes admits compatible paths; No constrains selected senses only; answers lose unseen paths/seeds/extra args.

## lookup_order

Explicit ID POS mismatch returns None immediately; adjective satellite same class.

## relation_completeness

IsA/verb/role No vs meronym Unknown on no witness; relation-specific closed world.

## proof_paths

BFS shortest, owners depth/id sorted, first(part,rel), depth<=3.

## parser_precedence

First --layer parsed before unknown/missing command; only first pair drained, extra args ignored.


## Ranking score domain

Multi-candidate ranking claims additionally require finite per-candidate scores (finite prior shares and finite weighted sums). The prior parser can accept NaN: total_cmp may place NaN first, then every `score >= best - 0.3` comparison fails, producing an empty ranked vector; isa/has_part can subsequently panic at cands[0]. Nonfinite scores are an explicit unverified failure domain, not a successful unknown-answer policy. Executed query fixtures have one sense and bypass rank_candidates scoring/global prior loading.

## tests

### Actual-code checks

- `query::wave29_tests::lookup_query_and_open_world_parts`

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective model saved before edits; nine existing sources read fully. Added bounded inline tests; store reader overflow repaired only after compiled old-code failing witness. No live corpus, teacher, model or database effects executed. Source syntax generated twice identically; partial lexical projection only.

### 2026-10-31 · tests

#### Evidence 2026-10-31

Each selected test passed twice debug and twice release after exact mutation restoration (15 named suite checks; all four sorted outcome sets equal, exits0). Compiled source control: negative-query.log (compiled source mutation assertion failure101). Controls compile successfully and fail an assertion with exit101. Store control is the original unsafe length addition before repair.

Finite graph/token/temp fixtures; public invalid states and unselected branches not exhausted

Whole CLI, live teacher/corpus/model/prior loading, filesystem failure/durability and huge inputs were not executed. No exact wrapper is advertised as a percentage surrogate. Added test blocks rustfmt-formatted independently; unrelated source formatting preserved. Strict broad Clippy not repeated: existing dependency/source warnings are outside this work.
