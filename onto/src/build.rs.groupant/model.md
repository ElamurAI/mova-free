# onto-build

## descriptant

### onto-build

OEWN gzip/XML resolves member/sense IDs then relation targets then inverse pairs; FrameNet ZIP resolves frames and roles. finish sorts/deduplicates words and edges, keeps first word-sense occurrence, builds outgoing/incoming CSR and orders LU by word, descending annotations, ascending id.

## approximant

### Forward model: onto-build

OEWN gzip/XML resolves member/sense IDs then relation targets then inverse pairs; FrameNet ZIP resolves frames and roles. finish sorts/deduplicates words and edges, keeps first word-sense occurrence, builds outgoing/incoming CSR and orders LU by word, descending annotations, ascending id.

#### Domain

Finite input arrays/text with resource availability and the source-specific bounds and representation assumptions in [invariant](model.md#invariant). Exact categorical semantics; no executable percent surrogate is supplied. 1/2/5/10% caps need a meaningful metric and are not claimed.

#### Relations

W=sort(unique(all sense/member/LU words)); wid(w)=index_W(w). E=sort(unique(raw edges)); out_off[i]=|{(s,r,t)∈E:s<i}|; in_off[i]=|{(s,r,t)∈E:t<i}|. senses(w)=stable_unique(concat entry senses for w). LU order=(wid,-annotations,id). All casts to u32 assume representable cardinalities.

## invariant

# Invariant and cost

On representable valid inputs CSR edges are transposes. Ingestion errors leave partial mutations. finish checks endpoints but does not check taxonomy cycles. u32 pool/ID narrowing unchecked.

## Cost

O(B+W log W+E log E+L log L+sum s_w²), where repeated senses use linear contains. Memory O(retained text+concepts+edges+words+LUs).

No allocation, integer-overflow, malformed public-state or external-effect guarantee beyond explicitly described branches.

## composition

### Composition

Zip/gzip XML → Builder::finish → Graph → store::check. finish itself does not prove taxonomy acyclicity; caller gate remains distinct.

Child fragments specialize concrete byte scopes and share the parent syntax projection; they do not duplicate whole-file guarantees. No composition-wide approximation error budget is established.

## models

# Typed obernant catalog

Partial source-derived relations, not proofs. Complement excludes incompatible observations; reverse describes compatible antecedents.

## behavior_complement

Unpaired OEWN relation forbids successful oewn, not partial mutations. Out-of-range edge forbids successful finish; cycles not excluded by finish.

## reverse_reconstruction

Same Graph admits repeated edges, repeated dictionary mentions and senses, alternate LU annotation counts preserving order, and ignored XML relations; source bytes unrecoverable.

## csr_transpose

Each distinct(s,r,t) appears as outgoing(s,r,t) and incoming(t,r,s); n+1 offsets start zero.

## sense_stability

Word dictionary sorts independently of entry-order senses; duplicate Cid retained at first position.

## failure_precedence

Member resolution before sense resolution before relation target and inverse pair checks; no rollback.

## lu_priority

Stable order by Wid, descending annotated count, ascending LU id; annotation counts discarded.

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

Prospective review preceded inline test edits. Tests used bounded synthetic CPU fixtures only. On representable valid inputs CSR edges are transposes. Ingestion errors leave partial mutations. finish checks endpoints but does not check taxonomy cycles. u32 pool/ID narrowing unchecked.

No production optimization, live teacher, corpus, assessment or database mutation. Registry presence denotes partial modeling; syntax is lexical and generated twice identically.

### 2026-10-31 · tests

Heading `### Evidence, 2026-10-31` stays in tests as `### Evidence`.

Finite synthetic examples only. Full ingestion, live workers, real corpus, allocation/I/O fault injection, GPU and full-school behavior remain unexecuted. No universal or executable-approximant assurance. External receipts: octopus-wave29-ontocore-2026-10-31 (native-commands.json, source-test-map.json, outcomes.json, controls.json and numeric exit/log files).
