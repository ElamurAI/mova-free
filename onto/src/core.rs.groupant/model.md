# onto-core

## descriptant

### onto-core

Field Known returns value/known/applicable; Unknown only applicable; NotApplicable neither. Generated CoreId indexes CORE. Lemma lookup replaces underscores/lowercases; synset lookup finds first static match. Graph synset map retains last duplicate key; check rejects missing synsets/lemma mismatch/domain contradictions and ancestral part/material.

## approximant

### Forward model: onto-core

Field Known returns value/known/applicable; Unknown only applicable; NotApplicable neither. Generated CoreId indexes CORE. Lemma lookup replaces underscores/lowercases; synset lookup finds first static match. Graph synset map retains last duplicate key; check rejects missing synsets/lemma mismatch/domain contradictions and ancestral part/material.

#### Domain

Finite input arrays/text with resource availability and the source-specific bounds and representation assumptions in [invariant](model.md#invariant). Exact categorical semantics; no executable percent surrogate is supplied. 1/2/5/10% caps need a meaningful metric and are not claimed.

#### Relations

state(Known(v))=(true,true); state(Unknown(r))=(false,true); state(NotApplicable)=(false,false). SynMap[(key,pos)]=last matching OEWN Cid. Domain contradiction for record r is ∃(p,b)∈domain_expect(r.domain): Kb.prop(c,p,0)=Known(b); unknown does not satisfy this predicate.

## invariant

# Invariant and cost

Static fields preserve Known([]) versus Unknown versus NotApplicable. core check uses real layer0 and accepts missing property knowledge. Public Graph indexing requires validity. Lexfile mismatch only warning.

## Cost

Static lookups O(Ccore); synset map O(Cgraph) expected; check adds Kb ancestry/property costs per record and value; display allocation proportional output.

No allocation, integer-overflow, malformed public-state or external-effect guarantee beyond explicitly described branches.

## composition

### Composition

Generated CORE joins Graph by OEWN key/POS; check uses Kb::prop layer0 and ancestors. Query/describe can prefer core, but unknown values and lexfile warnings remain distinct.

Child fragments specialize concrete byte scopes and share the parent syntax projection; they do not duplicate whole-file guarantees. No composition-wide approximation error budget is established.

## models

# Typed obernant catalog

Partial source-derived relations, not proofs. Complement excludes incompatible observations; reverse describes compatible antecedents.

## behavior_complement

Unknown cannot yield known value; NotApplicable cannot count applicable. Unknown graph property does not constitute contradiction. Lexfile mismatch alone cannot enter check errors.

## reverse_reconstruction

field_state(false,true) admits every Unknown reason; known state omits value. Formatted three-significant-digit values admit rounding intervals; original f64/provenance not uniquely recoverable.

## duplicate_synset

synset_map collect last duplicate wins while Graph::find_synset scans first. Equality requires unique key/POS assumption.

## domain_contradiction

Animal forbids Animate=false,Living=false,Human=true; plant forbids Living=false,Animate=true; magical imposes none.

## part_ancestry

Only part/material checked against self/ancestors if mapped synset exists; unresolved values skipped.

## numeric_display

sig zero→0, decimal places clamp0..6 based log10 magnitude then trailing zeros removed; rendering not numeric validation.

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

Prospective review preceded inline test edits. Tests used bounded synthetic CPU fixtures only. Static fields preserve Known([]) versus Unknown versus NotApplicable. core check uses real layer0 and accepts missing property knowledge. Public Graph indexing requires validity. Lexfile mismatch only warning.

No production optimization, live teacher, corpus, assessment or database mutation. Registry presence denotes partial modeling; syntax is lexical and generated twice identically.

### 2026-10-31 · tests

Heading `### Evidence, 2026-10-31` stays in tests as `### Evidence`.

Finite synthetic examples only. Full ingestion, live workers, real corpus, allocation/I/O fault injection, GPU and full-school behavior remain unexecuted. No universal or executable-approximant assurance. External receipts: octopus-wave29-ontocore-2026-10-31 (native-commands.json, source-test-map.json, outcomes.json, controls.json and numeric exit/log files).
