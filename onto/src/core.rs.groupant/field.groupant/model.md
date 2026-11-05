# onto-core-field

## descriptant

### onto-core-field

Known(v)→Some(&v),true,true; Unknown(reason)→None,false,true; NotApplicable→None,false,false.

## approximant

### Forward model: onto-core-field

Known(v)→Some(&v),true,true; Unknown(reason)→None,false,true; NotApplicable→None,false,false.

#### Domain

Finite input arrays/text with resource availability and the source-specific bounds and representation assumptions in [invariant](model.md#invariant). Exact categorical semantics; no executable percent surrogate is supplied. 1/2/5/10% caps need a meaningful metric and are not claimed.

#### Relations

known(Known(v))=Some(&v); known(Unknown(_))=known(NotApplicable)=None. is_known(f)⇔∃v:f=Known(v). is_applicable(f)⇔f≠NotApplicable. Thus is_known⇒is_applicable, but converse fails.

## invariant

# Invariant and cost

Scope bytes [5772,6111) in the parent source. No field gives known=true and applicable=false; Unknown cannot return Some.

## Cost

O(1), no allocation.

No allocation, integer-overflow, malformed public-state or external-effect guarantee beyond explicitly described branches.

## composition

### Composition

Scope output feeds its parent: Generated CORE joins Graph by OEWN key/POS; check uses Kb::prop layer0 and ancestors. Query/describe can prefer core, but unknown values and lexfile warnings remain distinct.

Child fragments specialize concrete byte scopes and share the parent syntax projection; they do not duplicate whole-file guarantees. No composition-wide approximation error budget is established.

## models

# Typed obernant catalog

Partial source-derived relations, not proofs. Complement excludes incompatible observations; reverse describes compatible antecedents.

## behavior_complement

No field gives known=true and applicable=false; Unknown cannot return Some.

## reverse_reconstruction

None admits Unknown(any reason) and NotApplicable; pair(false,true) restricts Unknown but leaves reason free.

## empty_known

Known empty slices remain known and applicable; emptiness not ignorance.

## reason_loss

is_known/is_applicable discard Unknown reason without mutating it.

## borrow_transport

known borrows original T, no cloning or allocation.

## state_space

Three categorical states; no percentage surrogate metric.

## tests

### Evidence

Selected command: cargo test --offline --locked -p onto --bin onto -- --exact --test-threads=1 core::wave29::field_and_synset_semantics

Named outcomes:

- core::wave29::field_and_synset_semantics

Two debug and two release/optimized executions returned zero and identical sorted nonempty named outcomes. Source-generated syntax pairs matched byte-for-byte. 1 compiled source mutation rejected by named test, exact source restored and positive rerun succeeded.

- Control: "!matches!(self, Field::NotApplicable) -> matches!(self, Field::Known(_))" → "core::wave29::field_and_synset_semantics" (exit 101).

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective review preceded inline test edits. Tests used bounded synthetic CPU fixtures only. Scope bytes [5772,6111) in the parent source. No field gives known=true and applicable=false; Unknown cannot return Some.

No production optimization, live teacher, corpus, assessment or database mutation. Registry presence denotes partial modeling; syntax is lexical and generated twice identically.

### 2026-10-31 · tests

Heading `### Evidence, 2026-10-31` stays in tests as `### Evidence`.

Finite synthetic examples only. Full ingestion, live workers, real corpus, allocation/I/O fault injection, GPU and full-school behavior remain unexecuted. No universal or executable-approximant assurance. External receipts: octopus-wave29-ontocore-2026-10-31 (native-commands.json, source-test-map.json, outcomes.json, controls.json and numeric exit/log files).
