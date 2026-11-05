# onto-coresel

## descriptant

### onto-coresel

Per core and field: nonapplicable→n/a; risk any none dominates, else non-dash acts→known else unknown. Other fields require same lemma/property/synset; yes accepts, no rejects, refine>=0.7 accepts, absent verdict>=0.7 accepts. Numeric accepts sort yes/refine/confident then descending total_cmp confidence and truncate one; values dedup raw string.

## approximant

### Forward model: onto-coresel

Per core and field: nonapplicable→n/a; risk any none dominates, else non-dash acts→known else unknown. Other fields require same lemma/property/synset; yes accepts, no rejects, refine>=0.7 accepts, absent verdict>=0.7 accepts. Numeric accepts sort yes/refine/confident then descending total_cmp confidence and truncate one; values dedup raw string.

#### Domain

Finite input arrays/text with resource availability and the source-specific bounds and representation assumptions in [invariant](model.md#invariant). Exact categorical semantics; no executable percent surrogate is supplied. 1/2/5/10% caps need a meaningful metric and are not claimed.

#### Relations

For matching-synset row r with verdict v and c=parse_f64(conf) or0: accept(r)=(v=yes)∨(v=refine∧c≥0.7)∨(v absent∧c≥0.7). Numeric winner=first stable_sort(accepted,(confirmation_rank,-total_cmp(conf))); rank(yes,refine,confident)=(0,1,2). Nonapplicable precedes risk/acceptance; any risk none dominates other rows.

## invariant

# Invariant and cost

Core/FIELDS order preserved; risk none dominates. Imported numeric/risk strings may still fail compilation gate. Output written after construction, not atomic. Only helper execution claimed.

## Cost

O(input bytes+C*11+candidate scans+sum A log A) expected hashes; full output retained. parse_source repeatedly scans prefixes.

No allocation, integer-overflow, malformed public-state or external-effect guarantee beyond explicitly described branches.

## composition

### Composition

wdwalk::read_core and corevmm::{candidates,verdicts,key_of} feed selection; q_synsets aligns accepted/also nouns; output consumed by coregate/build script. Helpers tested; full selection file pipeline not executed.

Child fragments specialize concrete byte scopes and share the parent syntax projection; they do not duplicate whole-file guarantees. No composition-wide approximation error budget is established.

## models

# Typed obernant catalog

Partial source-derived relations, not proofs. Complement excludes incompatible observations; reverse describes compatible antecedents.

## behavior_complement

No verdict cannot select even at confidence1; unknown existing verdict blocks confident fallback. Wrong synset cannot contribute; nonapplicable bypasses candidates.

## reverse_reconstruction

Selected numeric result admits rejected and lower-ranked candidates; stable ties constrain first winner only. Unknown admits no candidates, explicit no, low/NaN confidence or blocked verdict.

## numeric_rank

yes overrides confidence including NaN; total_cmp can rank NaN above finite within yes. Stable equal keys retain input order.

## q_alignment

q_synsets requires accepted/also, noun suffix -n and wanted u32 Q; first accepted row wins, absent file empty.

## source_narrowing

First boundary Q/P digits parse u64 then cast u32/u16; oversized values wrap. Rank substring priority preferred before normal before deprecated.

## applicability

animal excludes material; magical only risk; family/role only mass,height,risk; unknown domain false. Some broad domain rules accept arbitrary field names.

## tests

### Evidence

Selected command: cargo test --offline --locked -p onto --bin onto -- --exact --test-threads=1 coresel::wave29::applicability_and_provenance

Named outcomes:

- coresel::wave29::applicability_and_provenance

Two debug and two release/optimized executions returned zero and identical sorted nonempty named outcomes. Source-generated syntax pairs matched byte-for-byte. 1 compiled source mutation rejected by named test, exact source restored and positive rerun succeeded.

- Control: "\"magical\" => field == \"risk\" -> \"magical\" => field != \"risk\"" → "coresel::wave29::applicability_and_provenance" (exit 101).

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective review preceded inline test edits. Tests used bounded synthetic CPU fixtures only. Core/FIELDS order preserved; risk none dominates. Imported numeric/risk strings may still fail compilation gate. Output written after construction, not atomic. Only helper execution claimed.

No production optimization, live teacher, corpus, assessment or database mutation. Registry presence denotes partial modeling; syntax is lexical and generated twice identically.

### 2026-10-31 · tests

Heading `### Evidence, 2026-10-31` stays in tests as `### Evidence`.

Finite synthetic examples only. Full ingestion, live workers, real corpus, allocation/I/O fault injection, GPU and full-school behavior remain unexecuted. No universal or executable-approximant assurance. External receipts: octopus-wave29-ontocore-2026-10-31 (native-commands.json, source-test-map.json, outcomes.json, controls.json and numeric exit/log files).
