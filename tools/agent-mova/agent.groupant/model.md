# agent-mova-native

## model

# Capability interaction model
## Forward
A runner invokes a typed capability with externally owned task/configuration. Trace produces native inference plus reviewed-by-native-analysis episode retention; recall reuses a pinned checkpoint in a fresh process; collection maps report envelopes to deterministic declared-outcome groups and an ordered review queue.
## Composition
The CLI and library share exact serde types. Model the product of task identity, source window, graph state, report and evidence references. A proposed scheduler may compose these relations only after validating hashes, actor/task lineage, domain compatibility and publication status. The current crate does not supply scheduler fairness or exactly-once execution.
## Invariant
Source-scoped retention and declared external reviews remain distinct. No review queue is admitted as training truth. Unknown/failed reports survive collection; graph state is immutable during recall. A successful child file model alone does not establish an autonomous role.
## Cost
Total cost is the sum of reached native trace/recall costs plus bounded collection, sharing no assumed amortization. Cold start and kind-graph initialization remain real costs. No universal speedup estimate.
## Descriptant
First supported clients are Fish native source-window work and Crab report systematization. Public reusable protocols carry no private concrete tasks. Other roles are future clients, not implemented migrations. Finite source-grounded pilot evidence supports one location-episode loop only.
## behavior_complement
Excluded composition claims: report existence implies semantic truth; grouping supported labels means factual admission; a retained frame implies learned tool planning; successful recall implies source replay. Those are outside the implemented relation. Counterexample to a general learned executor: the only inference question is a location question with four explicit alternatives. Evidence: source and bounded synthetic graph/cold-recall probes.
## reverse_reconstruction
The review artifact preserves envelopes and artifact hashes, allowing a reviewer to recover compatible input identities when bytes remain available. It cannot recover omitted source passages, hidden task history or author authenticity from digest alone. Multiple source episodes may share the same retained conclusion. Evidence: inspected noninjective interfaces, not a reconstruction theorem.
## retention_lineage
Parent trace report and state pins join trace to cold recall. Native Fish refs bind envelope identities/outcomes to actual report bytes; external evidence refs bind bytes only and remain declared manifest metadata. A stale or relabeled native artifact breaks the join. Evidence: actual outcome-substitution rejection and cold restore.
## context_exclusion
A flagged speaker/first-person window cannot silently become retained semantic knowledge in this capability. Unmarked ambiguity remains an explicit limit. Parent composition must not erase findings or promote a teacher opinion to fact. Evidence: queue preservation and context controls.
## error_partition
Each exclusive effectful operation can leave a partial directory after I/O failure. A future persistent runner must preserve such attempts and inspect checkpoints rather than replay blindly. Completed synthetic artifacts are not evidence of crash atomicity. Source-derived limitation; no crash injection performed.

## artifact_identity_binding
Domain: a declared envelope m=(id,actor,kind,outcome,source,findings), artifact bytes b and reference digest h. Observation: collection acceptance and review queue. The predicate H(b)=h binds bytes but says nothing about m. For native_fish_report, acceptance additionally requires m=projection(decode_FishReport(b)) and schema=1. Counterexample to hash-only admission: retain b and its correct digest while replacing uncertain with supported, or id native-a with native-b. The hash still passes; the native projection check rejects before collection publication. Focused Rust controls exercise both mutations and an unchanged positive control. For external_review_evidence, decoding is intentionally absent: m remains a caller declaration and the stored artifact format preserves that uncertainty. Neither relation authenticates authorship or semantic truth. This is a source-derived necessary-condition model with finite executable evidence, not a proof of all report correctness.

## checks

### Checks and evidence

This is finite synthetic evidence. No teacher call, full corpus, held-out assessment, daemon restart or learned general planner was exercised. No general semantic proof or speed comparison is supplied. Initial yanked-dependency lock resolution and three local compile errors were corrected before successful builds; their external logs remain preserved.

## History

### 2026-11-01 · checks

2026-11-01: offline locked two-job debug build; five unit tests passed. Two fresh CLI traces and two fresh recalls were byte-identical. Trace added one graph frame and changed zero weights; recall changed neither. Unknown subject and speaker-tagged context were uncertain, without checkpoint publication. Wrong state/window hashes and native outcome substitution were rejected. Native table collection retained three reports including uncertainty and reproduced identical bytes.
