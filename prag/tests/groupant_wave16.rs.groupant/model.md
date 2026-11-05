# prag-groupant-wave16

## model

# Wave16 native joins

## forward

Three supplied joins transport frame notation/permutation into occurrence scores, numbered rows into Item/classes, and batch order into source-safe prompts.

## invariant

Retained frame permutation, one-based batch positions, closed act alphabet, source/document identity and checked consecutive paragraphs.

## cost

Notation bytes, greedy pair alignment, occurrence accounting, row parsing, ordered batches and emitted prompt bytes; external acquisition excluded.

## composition

Each named test is a separate tagged relation. Composition requires identical graph vertices, byte coordinates, moduli, insertion IDs and timeline steps; Mova joins retain frame permutations, one-based batch positions, closed class coordinates and source/document/paragraph identities. Distinct tests do not constitute one end-to-end pipeline. Independent finite or literal oracles remain distinct from cross-implementation equality.

## epoch-memory

# Wave16 prospective Mova joins

Three bounded supplied-input joins, designed before integrated runs. No teacher, parser process, corpus or IO execution. Built-in frame/lexical data may be reached only by existing functions; no learning/assessment accuracy claim.

1. Actual Frame.show output parses to scorer GFrames. A two-frame speech/event fixture is permuted with its frame links remapped. Literal role counts stay 9/9/9 and per-link counts 1/1/1. Adding duplicate system quote links contributes predictions without creating more correct occurrences than exist in gold (multiset intersection). Complement excludes dangling/unremapped links and tp>min(sys,gold). Reverse retains all frame detail omitted by notation; correct scores do not uniquely recover source text. Cost includes output bytes, O(G*S log(G*S)) greedy alignment and role/link accounting; no global optimum alignment claim.

2. Supplied Items and numbered silver rows compose through exact 1-based batch positions. Out-of-order rows resolve to their original Items; schema TSV transport preserves act classes; metric scores compare the same closed act alphabet. Missing rows remain partial rejection, duplicate rows reject entire batch. Complement excludes position/label substitution and partial acceptance after duplicate identifiers. Reverse loses original row ordering, junk spelling and raw label formatting. Costs follow prompt/row bytes plus ordered key maps and class counts; no model call occurs.

3. Batches preserve input order within each ascending batch ID. Prompt context suppression requires same source and document and checked consecutive paragraph arithmetic; cross-source collisions or MAX->MIN wrapping do not suppress context. Complement excludes dropped context for those disallowed transitions. Reverse cannot recover source metadata from identical prompt text. Cost is ordered batching and emitted prompt/context bytes; no end-to-end throughput measurement.


## checks

3 named tests pass twice in debug and twice in release. A compiled adapter control fails its named assertion and exact bytes are restored. Partial syntax repeats identically; model.md#evidence records native and structural receipts. Macro bodies remain opaque; semantic AST and whole-system proof are absent.

## catalog

# Wave16 supplied-input relations

Source-derived specifications; bounded native evidence and repeated partial syntax are recorded in model.md#evidence. No general proof or complete inverse engine is claimed.

## behavior-complement

Type: behavior_complement. Named check: all three Mova joins.

Universe: supplied Frame notation/permutation, numbered silver rows and Item batches/prompts paired with observed counts/labels/text. Exclude unremapped links, true positives exceeding system/gold occurrence counts, exchanged one-based row/class identities, partial duplicate-identifier acceptance, and context suppression across source collisions or overflowing paragraph transitions. These separate tagged transitions invoke no external teacher/parser.

## reverse-reconstruction

Type: reverse_reconstruction. Named check: all three Mova joins.

Retain compatible Frames, notation, rows, Items and batches sharing observed coordinates. Notation omits frame detail; counts cannot recover source text; accepted labels discard row order/junk spelling; identical prompts cannot recover all metadata. Reverse composition retains the frame permutation, one-based Item position, act alphabet and ordered batch references; many antecedents remain.

## notation-occurrences

Type: alignment_multiset_transport. Named check: notation_alignment_preserves_permuted_links_and_occurrence_bounds.

Actual Frame.show output parses to scorer GFrames. A two-frame speech/event fixture is permuted with its frame links remapped. Literal role counts stay 9/9/9 and per-link counts 1/1/1. Adding duplicate system quote links contributes predictions without creating more correct occurrences than exist in gold (multiset intersection). Complement excludes dangling/unremapped links and tp>min(sys,gold). Reverse retains all frame detail omitted by notation; correct scores do not uniquely recover source text. Cost includes output bytes, O(G*S log(G*S)) greedy alignment and role/link accounting; no global optimum alignment claim.

## numbered-rows

Type: annotation_position_class_transport. Named check: numbered_silver_rows_keep_item_and_closed_class_identity.

Supplied Items and numbered silver rows compose through exact 1-based batch positions. Out-of-order rows resolve to their original Items; schema TSV transport preserves act classes; metric scores compare the same closed act alphabet. Missing rows remain partial rejection, duplicate rows reject entire batch. Complement excludes position/label substitution and partial acceptance after duplicate identifiers. Reverse loses original row ordering, junk spelling and raw label formatting. Costs follow prompt/row bytes plus ordered key maps and class counts; no model call occurs.

## batch-context

Type: source_safe_prompt_coordinates. Named check: batching_and_prompt_context_share_source_safe_paragraph_identity.

Batches preserve input order within each ascending batch ID. Prompt context suppression requires same source and document and checked consecutive paragraph arithmetic; cross-source collisions or MAX->MIN wrapping do not suppress context. Complement excludes dropped context for those disallowed transitions. Reverse cannot recover source metadata from identical prompt text. Cost is ordered batching and emitted prompt/context bytes; no end-to-end throughput measurement.

## evidence

### Wave16 bounded evidence

The complete 100-algorithm crate passes 504 checks (448 unit, 55 external composition, one doctest) twice in debug and twice in release. The 155 selected Mova checks also pass twice in each profile, with identical normalized named outcomes: 139 previous focused checks, 11 new unit checks, two existing fbench/silver tests and three new integration joins. This is a selected Mova suite, not full-system validation.

Ten new algorithm implementation mutants and two Mova production mutants fail named assertions in owner receipts, with exact restoration before positive runs. Root additionally compiles two adapter controls: wrong forest weight/cardinality correction and swapped one-based silver-row identity. Both fail their named assertions, then exact source bytes are restored. Compilation failure is not counted as semantic rejection.

Mova fixes bound duplicate-link credit by gold occurrences (multiset intersection) and retain context unless adjacent paragraphs share source/document identity and checked consecutive paragraph numbers. Old-code witnesses cover duplicate overcredit, cross-source suppression and overflow. Three root joins exercise notation/alignment/counts, row/item/class transport and batch/prompt coordinates. Unique-link semantics and public APIs remain unchanged.

Independent read-only review found no additional concrete defect. Coverage limits: graph/tree cap+1 checks do not establish maximum-size successful throughput; Li Chao scans an independent line envelope but shares widened arithmetic and value/ID ordering; the connectivity cap fixture does not stress worst-case interval storage. A 64-vertex clique representation cap does not make exponential search cheap.

The ten new implementations are exact within their declared domains; no new approximate Rust surrogate or Lean proof is asserted. No live parser/teacher/corpus acquisition, GPU, whole-school throughput or persistent-learning adoption was executed. No general semantic proof or training-speed gain follows from these checks.

## History

### 2026-10-30 · epoch-memory

2026-10-30 prospective design retained below; no speed or accuracy gain is claimed.

### 2026-10-30 · evidence

Heading `### Wave16 bounded evidence, 2026-10-30` stays in evidence as `### Wave16 bounded evidence`.

3 new native joins. Source SHA-256 e3b13f4d30c6df653b6cea0843c66ad119cb3f3de894fda7aef23b0aefc2eb4c; twice-identical partial syntax SHA-256 1acf6eebd1c5ac2416db4bae4cafc082b448a5fc9ad1e3542a80766ca8279978.

Raw receipts: $MOVA_DATA/runs/octopus-wave16-integration-2026-10-30. native-evidence-hashes.json SHA-256 f93920d688e961ce9dd02be865389cbe54db3093416464e379c650f16578841d; algorithm-normalized.txt 1335d34b718e04458dac54697f8822eb8f02b0f408dbb547852dfaa2d60a7164; mova-normalized.txt f0ca49c0b0f12e5febe6e41657b4d098e8918171918f8ae53bd96868ac5f9328; structural-controls/summary.json a50d4387189f34044579c0d068475201acf1f6e7ee8e3297c0db6909f3180975. Structural controls reject a stale parent source hash and a wrong child identity, then restore exact bytes. Owned partial syntax projections repeat identically; macro bodies remain opaque. Owner receipts remain adjacent to sources; formatting-only followups retain original owner native hashes, while root repeats bind final bytes.
