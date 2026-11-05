# onto-build-assembly

## descriptant

### onto-build-assembly

Consume CB/member/sense/LU arrays into sorted distinct word pool, SoA concepts, deduplicated transpose CSR and ordered LU rows.

## approximant

### Forward model: onto-build-assembly

Consume CB/member/sense/LU arrays into sorted distinct word pool, SoA concepts, deduplicated transpose CSR and ordered LU rows.

#### Domain

Finite input arrays/text with resource availability and the source-specific bounds and representation assumptions in [invariant](model.md#invariant). Exact categorical semantics; no executable percent surrogate is supplied. 1/2/5/10% caps need a meaningful metric and are not claimed.

#### Relations

Let E'=sort(unique(edges)); ∀(s,r,t)∈E': s<n∧t<n is required. out(i)={(r,t):(i,r,t)∈E'}, in(i)={(r,s):(s,r,i)∈E'}. off_out[i]=Σ(j<i)|out(j)|; off_in[i]=Σ(j<i)|in(j)|. Sense sequences use stable_unique; output words sort unique.

## invariant

# Invariant and cost

Scope bytes [21139,24771) in the parent source. Duplicate edge multiplicity cannot survive; invalid endpoint cannot succeed; cycles may survive.

## Cost

Dictionary/edge/LU sorting and per-word quadratic dedup as specified by parent; this scope performs no file I/O.

No allocation, integer-overflow, malformed public-state or external-effect guarantee beyond explicitly described branches.

## composition

### Composition

Scope output feeds its parent: Zip/gzip XML → Builder::finish → Graph → store::check. finish itself does not prove taxonomy acyclicity; caller gate remains distinct.

Child fragments specialize concrete byte scopes and share the parent syntax projection; they do not duplicate whole-file guarantees. No composition-wide approximation error budget is established.

## models

# Typed obernant catalog

Partial source-derived relations, not proofs. Complement excludes incompatible observations; reverse describes compatible antecedents.

## behavior_complement

Duplicate edge multiplicity cannot survive; invalid endpoint cannot succeed; cycles may survive.

## reverse_reconstruction

Graph admits builders repeating edges/senses and varying annotations while preserving final order.

## word_transport

Members resolve through binary search in sorted all; dictionary duplicates collapse.

## offset_units

String offsets are UTF-8 bytes cast u32, other offsets count elements.

## edge_gate

Endpoint checks occur after edge sort/dedup before CSR construction.

## empty_state

Empty builder returns zero concepts and sentinel offsets.

## tests

### Evidence

Selected command: cargo test --offline --locked -p onto --bin onto -- --exact --test-threads=1 build::wave29::assemble_csr_senses_and_lu_ties build::wave29::source_parser_helpers_and_bad_edge

Named outcomes:

- build::wave29::assemble_csr_senses_and_lu_ties
- build::wave29::source_parser_helpers_and_bad_edge

Two debug and two release/optimized executions returned zero and identical sorted nonempty named outcomes. Source-generated syntax pairs matched byte-for-byte. 1 compiled source mutation rejected by named test, exact source restored and positive rerun succeeded.

- Control: "edges.dedup(); -> /* negative control: retain duplicate edges */" → "build::wave29::assemble_csr_senses_and_lu_ties" (exit 101).

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective review preceded inline test edits. Tests used bounded synthetic CPU fixtures only. Scope bytes [21139,24771) in the parent source. Duplicate edge multiplicity cannot survive; invalid endpoint cannot succeed; cycles may survive.

No production optimization, live teacher, corpus, assessment or database mutation. Registry presence denotes partial modeling; syntax is lexical and generated twice identically.

### 2026-10-31 · tests

Heading `### Evidence, 2026-10-31` stays in tests as `### Evidence`.

Finite synthetic examples only. Full ingestion, live workers, real corpus, allocation/I/O fault injection, GPU and full-school behavior remain unexecuted. No universal or executable-approximant assurance. External receipts: octopus-wave29-ontocore-2026-10-31 (native-commands.json, source-test-map.json, outcomes.json, controls.json and numeric exit/log files).
