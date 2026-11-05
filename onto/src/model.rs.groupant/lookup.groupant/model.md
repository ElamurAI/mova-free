# onto-model-lookup

## descriptant

### onto-model-lookup

Binary search with lo inclusive/hi exclusive; lus_of selects [first word>=w, first word>w).

## approximant

### Forward model: onto-model-lookup

Binary search with lo inclusive/hi exclusive; lus_of selects [first word>=w, first word>w).

#### Domain

Finite input arrays/text with resource availability and the source-specific bounds and representation assumptions in [invariant](model.md#invariant). Exact categorical semantics; no executable percent surrogate is supplied. 1/2/5/10% caps need a meaningful metric and are not claimed.

#### Relations

Maintain 0≤lo≤hi≤|W| and candidate index in[lo,hi). mid=floor((lo+hi)/2); comparison updates lo=mid+1 or hi=mid. lus_of(w)=lus[lower_bound(word≥w)..lower_bound(word>w)]. Sorted inputs required.

## invariant

# Invariant and cost

Scope bytes [9944,10621) in the parent source. Empty dictionary returns None; absent word does not return insertion index.

## Cost

Linear traversed scope input/returned bytes except lookup logarithmic comparisons; allocations follow actual result buffers. Parent whole-pipeline cost is not assigned to this fragment.

No allocation, integer-overflow, malformed public-state or external-effect guarantee beyond explicitly described branches.

## composition

### Composition

Scope output feeds its parent: Builder constructs graph arrays, store validates/deserializes; Kb/query rely on sorted words/valid CSR. Core SynMap last duplicate differs from linear find first duplicate.

Child fragments specialize concrete byte scopes and share the parent syntax projection; they do not duplicate whole-file guarantees. No composition-wide approximation error budget is established.

## models

# Typed obernant catalog

Partial source-derived relations, not proofs. Complement excludes incompatible observations; reverse describes compatible antecedents.

## behavior_complement

Empty dictionary returns None; absent word does not return insertion index.

## reverse_reconstruction

Some(i) admits sorted dictionaries with exact s at i; LU slice admits arbitrary sorted surrounding words.

## empty_search

Absent sentinel still yields n_words0 through saturation.

## string_order

Case-sensitive str lexical comparison.

## lu_partition

Absent Wid gives empty slice; every equal-word LU included.

## resources

Search no buffers; logarithmic comparisons with variable string length.

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

Prospective review preceded inline test edits. Tests used bounded synthetic CPU fixtures only. Scope bytes [9944,10621) in the parent source. Empty dictionary returns None; absent word does not return insertion index.

No production optimization, live teacher, corpus, assessment or database mutation. Registry presence denotes partial modeling; syntax is lexical and generated twice identically.

### 2026-10-31 · tests

Heading `### Evidence, 2026-10-31` stays in tests as `### Evidence`.

Finite synthetic examples only. Full ingestion, live workers, real corpus, allocation/I/O fault injection, GPU and full-school behavior remain unexecuted. No universal or executable-approximant assurance. External receipts: octopus-wave29-ontocore-2026-10-31 (native-commands.json, source-test-map.json, outcomes.json, controls.json and numeric exit/log files).
