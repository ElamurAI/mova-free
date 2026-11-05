# onto-model

## descriptant

### onto-model

Public SoA/CSR graph accesses off[i]..off[i+1]; binary word search needs sorted dictionary; LU lookup uses two word partition points. find_synset returns first matching OEWN key/POS. Enum byte decoders return None outside table.

## approximant

### Forward model: onto-model

Public SoA/CSR graph accesses off[i]..off[i+1]; binary word search needs sorted dictionary; LU lookup uses two word partition points. find_synset returns first matching OEWN key/POS. Enum byte decoders return None outside table.

#### Domain

Finite input arrays/text with resource availability and the source-specific bounds and representation assumptions in [invariant](model.md#invariant). Exact categorical semantics; no executable percent surrogate is supplied. 1/2/5/10% caps need a meaningful metric and are not claimed.

#### Relations

word(w)=words[word_off[w]..word_off[w+1]]; out(c)=out[out_off[c]..out_off[c+1]], analogously in/members/senses. word_id(s)=binary_search_W(s). find_synset(s)=min{i:srcᵢ=Oewn∧keyᵢ=parse_key(s)∧posᵢ=parse_pos(s)}, if any.

## invariant

# Invariant and cost

Offsets/dimensions/UTF8 boundaries and IDs assumed valid via external store::check; public mutation can violate them. Default graph has zero words/concepts; n_words saturating subtraction.

## Cost

Slices O(1), word lookup O(log W) string comparisons, LU lookup O(log L), synset O(C); rendering allocates; no validation cost inside accessors.

No allocation, integer-overflow, malformed public-state or external-effect guarantee beyond explicitly described branches.

## composition

### Composition

Builder constructs graph arrays, store validates/deserializes; Kb/query rely on sorted words/valid CSR. Core SynMap last duplicate differs from linear find first duplicate.

Child fragments specialize concrete byte scopes and share the parent syntax projection; they do not duplicate whole-file guarantees. No composition-wide approximation error budget is established.

## models

# Typed obernant catalog

Partial source-derived relations, not proofs. Complement excludes incompatible observations; reverse describes compatible antecedents.

## behavior_complement

Invalid enum byte cannot decode. word_id performs no case/plural normalization; malformed CSR may panic instead of Err.

## reverse_reconstruction

Lookup result constrains observed dictionary location only; unobserved arrays arbitrary. cid_str is not unique for duplicate keys; first matching Cid chosen.

## pos_quotient

Adj/Sat share word class symmetrically but distinct bytes/letters; Frame/Role lack from_wn input.

## csr_units

String offsets count UTF8 bytes; membership/senses/edges offsets count elements.

## lookup_order

word_id assumes lexical string order; lus_of requires nondecreasing Wid and returns all equal-word LUs.

## role_bits

Src::Role decodes bits1..2 as Core; bit0 independent FLAG_PLUS; all four combinations valid.

## tests

### Evidence

Selected command: cargo test --offline --locked -p onto --bin onto -- --exact --test-threads=1 model::wave29::representation_and_lookup_boundaries

Named outcomes:

- model::wave29::representation_and_lookup_boundaries

Two debug and two release/optimized executions returned zero and identical sorted nonempty named outcomes. Source-generated syntax pairs matched byte-for-byte. 1 compiled source mutation rejected by named test, exact source restored and positive rerun succeeded.

- Control: "self.word(mid as Wid).cmp(s) -> s.cmp(self.word(mid as Wid))" → "model::wave29::representation_and_lookup_boundaries" (exit 101).

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective review preceded inline test edits. Tests used bounded synthetic CPU fixtures only. Offsets/dimensions/UTF8 boundaries and IDs assumed valid via external store::check; public mutation can violate them. Default graph has zero words/concepts; n_words saturating subtraction.

No production optimization, live teacher, corpus, assessment or database mutation. Registry presence denotes partial modeling; syntax is lexical and generated twice identically.

### 2026-10-31 · tests

Heading `### Evidence, 2026-10-31` stays in tests as `### Evidence`.

Finite synthetic examples only. Full ingestion, live workers, real corpus, allocation/I/O fault injection, GPU and full-school behavior remain unexecuted. No universal or executable-approximant assurance. External receipts: octopus-wave29-ontocore-2026-10-31 (native-commands.json, source-test-map.json, outcomes.json, controls.json and numeric exit/log files).
