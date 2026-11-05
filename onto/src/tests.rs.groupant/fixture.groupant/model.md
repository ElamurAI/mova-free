# onto-tests-fixture

## descriptant

### onto-tests-fixture

Build17 OEWN concepts, sorted words, whale senses[15,8], CSR/transposed edges from fixed arrays; unwrap store::check.

## approximant

### Forward model: onto-tests-fixture

Build17 OEWN concepts, sorted words, whale senses[15,8], CSR/transposed edges from fixed arrays; unwrap store::check.

#### Domain

Finite input arrays/text with resource availability and the source-specific bounds and representation assumptions in [invariant](model.md#invariant). Exact categorical semantics; no executable percent surrogate is supplied. 1/2/5/10% caps need a meaningful metric and are not claimed.

#### Relations

Cid(synᵢ)=i; W=sort(unique(lemmas)); Wid(w)=binary_search_W(w). E=IsA∪InstanceOf∪PartOf fixture tuples; reverse(s,r,t)=(t,r,s). sense(whale)=[15,8] overrides default insertion order; check(Graph)=Ok required.

## invariant

# Invariant and cost

Scope bytes [375,3678) in the parent source. Failed store structural check cannot silently return Graph. Fixture does not cover arbitrary corpus distributions.

## Cost

Linear traversed scope input/returned bytes except lookup logarithmic comparisons; allocations follow actual result buffers. Parent whole-pipeline cost is not assigned to this fragment.

No allocation, integer-overflow, malformed public-state or external-effect guarantee beyond explicitly described branches.

## composition

### Composition

Scope output feeds its parent: Synthetic Graph → store → Kb → query/eval/pron/HS, with explicit fixture-only assertions; core completeness tests exercise parser without real core data.

Child fragments specialize concrete byte scopes and share the parent syntax projection; they do not duplicate whole-file guarantees. No composition-wide approximation error budget is established.

## models

# Typed obernant catalog

Partial source-derived relations, not proofs. Complement excludes incompatible observations; reverse describes compatible antecedents.

## behavior_complement

Failed store structural check cannot silently return Graph. Fixture does not cover arbitrary corpus distributions.

## reverse_reconstruction

Returned arrays admit other construction algorithms sharing same sorting/order; source code not invertible.

## sense_ambiguity

Whale person precedes animal to test downstream context reordering.

## coordinates

Cid is syn-list index; Wid sorted word position.

## transpose_fixture

Reverse source/target preserve Rel.

## bounded_domain

Fixed17 concepts; no I/O inside mini.

## tests

### Evidence

Selected command: cargo test --offline --locked -p onto --bin onto -- --exact --test-threads=1 tests::snapshot_round_trip_and_corruption tests::defaults_exceptions_most_specific_wins tests::layers_do_not_leak tests::nixon_diamond_is_undetermined_with_both_proofs tests::gates_are_not_mute tests::queries_answer_with_proofs_and_refuse_to_guess tests::scoring tests::frozen_hash_gate tests::world_command_fields tests::prop_names_parse tests::pronoun_api tests::core_completeness_gate tests::wsd_stems tests::hs_isa_gate

Named outcomes:

- tests::snapshot_round_trip_and_corruption
- tests::defaults_exceptions_most_specific_wins
- tests::layers_do_not_leak
- tests::nixon_diamond_is_undetermined_with_both_proofs
- tests::gates_are_not_mute
- tests::queries_answer_with_proofs_and_refuse_to_guess
- tests::scoring
- tests::frozen_hash_gate
- tests::world_command_fields
- tests::prop_names_parse
- tests::pronoun_api
- tests::core_completeness_gate
- tests::wsd_stems
- tests::hs_isa_gate

Two debug and two release/optimized executions returned zero and identical sorted nonempty named outcomes. Source-generated syntax pairs matched byte-for-byte. No dedicated compiled runtime control for this test-fixture source; production controls are mapped separately.



#### Prior isolation correction

Initial repeated query tests omitted ONTO_PRIOR; query ranking may initialize wsd::global_prior from ambient/default path. Whether that file existed was not established; initial query outcomes are not fully isolated evidence. The affected queries_answer_with_proofs_and_refuse_to_guess name was rerun twice debug and twice release with ONTO_PRIOR asserted absent before/after. All four pass. Native command specs now set this environment. See prior-isolation.json.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective review preceded inline test edits. Tests used bounded synthetic CPU fixtures only. Scope bytes [375,3678) in the parent source. Failed store structural check cannot silently return Graph. Fixture does not cover arbitrary corpus distributions.

No production optimization, live teacher, corpus, assessment or database mutation. Registry presence denotes partial modeling; syntax is lexical and generated twice identically.

### 2026-10-31 · tests

Heading `### Evidence, 2026-10-31` stays in tests as `### Evidence`.

Finite synthetic examples only. Full ingestion, live workers, real corpus, allocation/I/O fault injection, GPU and full-school behavior remain unexecuted. No universal or executable-approximant assurance. External receipts: octopus-wave29-ontocore-2026-10-31 (native-commands.json, source-test-map.json, outcomes.json, controls.json and numeric exit/log files).
