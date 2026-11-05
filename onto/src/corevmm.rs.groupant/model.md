# onto-corevmm

## descriptant

### onto-corevmm

Candidates require >=13 columns; sorted wd-confirm files last matching five-string key wins. pick accepts allowed properties, unseen keys, confidence>=0.45, dedups first key, sorts contested<0.8 first then lemma/property. Risk parser initializes valid-ID slot before act/harm/p validation. Live wrappers call teacher and append workers.

## approximant

### Forward model: onto-corevmm

Candidates require >=13 columns; sorted wd-confirm files last matching five-string key wins. pick accepts allowed properties, unseen keys, confidence>=0.45, dedups first key, sorts contested<0.8 first then lemma/property. Risk parser initializes valid-ID slot before act/harm/p validation. Live wrappers call teacher and append workers.

#### Domain

Finite input arrays/text with resource availability and the source-specific bounds and representation assumptions in [invariant](model.md#invariant). Exact categorical semantics; no executable percent surrogate is supplied. 1/2/5/10% caps need a meaningful metric and are not claimed.

#### Relations

Eligible(r)=prop∈VMM_PROPS∧key(r)∉seen∧parse(conf)≥0.45. pick=stable_sort(stable_unique_key(filter Eligible),(conf≥0.8,lemma,prop)). Risk slots initially None; valid ID/width maps None→Some([]), then valid action/harm/p appends, never clears.

## invariant

# Invariant and cost

Accepted risk p in[0,1], enums valid; NaN rejected. Some(empty) also means malformed valid-ID answer. Threads clamped1..2; append lock does not ensure deterministic completion order; write errors discarded.

## Cost

O(B+N log N) candidate selection; O(B+n+risks) response parsing. Live latency/cost external and unmeasured. Memory retains rows and prompts.

No allocation, integer-overflow, malformed public-state or external-effect guarantee beyond explicitly described branches.

## composition

### Composition

wdvmm response transport supplies strings parsed here; pick and verdict key feed coresel. Live ask, append failure and worker races are explicit gaps.

Child fragments specialize concrete byte scopes and share the parent syntax projection; they do not duplicate whole-file guarantees. No composition-wide approximation error budget is established.

## models

# Typed obernant catalog

Partial source-derived relations, not proofs. Complement excludes incompatible observations; reverse describes compatible antecedents.

## behavior_complement

pick excludes NaN/low and seen keys including no answer. Risk parser excludes NaN/out-of-range but does not enforce prompt cap3 or minimum0.001.

## reverse_reconstruction

Same parsed risks admit invalid ignored rows and case/whitespace variants. Some(empty) cannot reconstruct none versus malformed; dropped duplicate candidate keys unrecoverable.

## risk_slot

None means no valid-ID row with>=2 columns; Some([]) at least one such row. Later none never erases prior risks.

## threshold_order

Exactly0.45 accepted; exactly0.8 confident partition; same lemma/property ties stable, not confidence sorted.

## verdict_overwrite

Sorted paths then input lines, last key wins; directory-entry errors skipped.

## effect_boundary

risks truncates output header before worker calls; in-flight workers may complete after error. write_all errors ignored; no live execution.

## tests

### Evidence

Selected command: cargo test --offline --locked -p onto --bin onto -- --exact --test-threads=1 corevmm::wave29::candidate_threshold_order_and_dedup corevmm::wave29::risk_slot_is_distinct_from_valid_risk corevmm::t::parse_risk_answer

Named outcomes:

- corevmm::wave29::candidate_threshold_order_and_dedup
- corevmm::wave29::risk_slot_is_distinct_from_valid_risk
- corevmm::t::parse_risk_answer

Two debug and two release/optimized executions returned zero and identical sorted nonempty named outcomes. Source-generated syntax pairs matched byte-for-byte. 1 compiled source mutation rejected by named test, exact source restored and positive rerun succeeded.

- Control: "(0.0..=1.0).contains(&p) -> (0.0..=0.9).contains(&p)" → "corevmm::wave29::risk_slot_is_distinct_from_valid_risk" (exit 101).

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective review preceded inline test edits. Tests used bounded synthetic CPU fixtures only. Accepted risk p in[0,1], enums valid; NaN rejected. Some(empty) also means malformed valid-ID answer. Threads clamped1..2; append lock does not ensure deterministic completion order; write errors discarded.

No production optimization, live teacher, corpus, assessment or database mutation. Registry presence denotes partial modeling; syntax is lexical and generated twice identically.

### 2026-10-31 · tests

Heading `### Evidence, 2026-10-31` stays in tests as `### Evidence`.

Finite synthetic examples only. Full ingestion, live workers, real corpus, allocation/I/O fault injection, GPU and full-school behavior remain unexecuted. No universal or executable-approximant assurance. External receipts: octopus-wave29-ontocore-2026-10-31 (native-commands.json, source-test-map.json, outcomes.json, controls.json and numeric exit/log files).
