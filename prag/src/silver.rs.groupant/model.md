# prag-silver

## model

# prag-silver

## forward
Supplied Items -> ordered borrowed batches -> prompt and numbered answer gate -> accepted labels and item/batch rejection. Wrappers additionally call teacher and persist/aggregate files; those effects are source inspected only.

## invariant
Batch IDs sort; members retain input order; prompt/gate numbers are 1-based per batch, not key/ord. Codex teacher policy persists with no Claude fallback; legacy Opus naming grants no execution permission.

## cost
N items, K batches, B text bytes: grouping O(N log K), prompt O(B+N), gate O(B+N log N); external latency/IO unspecified.

## behavior-complement
Pure outputs accepting zero/duplicate numbers, reordering members, or suppressing context solely through cross-source doc collision are excluded. External durability/concurrency remains unexecuted. Universe: bounded supplied inputs and candidate observable outputs at this source revision. Partial source-derived inner exclusions with finite witness evidence; unsampled semantics remain unknown.

## reverse-reconstruction
Accepted labels lose text/context/rejected rows and keys. Recover keys only by joining exact batch references; prompts lose nonrendered Item metadata. Target is all x in the stated domain with F(x)=y, without unique recovery or an implemented complete solver.

## specialization
Shared identity: use the same batch vector for prompt numbering and gate numbering; numeric doc collisions across Src are not predecessor identity.

## composition
Children share source identity. Preserve frame indices through alignment/link mapping and batch references through prompt/gate numbering. Independent marginal inverses can lose shared correlations; overlap costs are counted once.

## checks
Six fbench and five silver wave16_models tests: finite canonical role oracle, independent ten-role counts, old-code witnesses and compiled controls. See receipts at file root.

## epoch-memory

Prospective inspection predicted predecessor defects and duplicate-edge recall inflation. Three old-code witnesses confirm these. Valid adjacency and unique links retain semantics. No score assertions or deduplication introduced. Historical metrics retain original semantics.

## limitations
Partial syntax is not expanded semantic AST; macros opaque. Scoped models and finite observations, not full-file verification or proof. No live parser, teacher, corpus, filesystem aggregate, GPU, full course, speed or approximation-cap claim. External effects and resource-extreme malformed inputs remain unexecuted.

## evidence

### Evidence scope

See root model.md#receipts for source and raw-log hashes. Pure supplied fixtures only; effects are source inspected.

## receipts

### Wave16 evidence

Eleven named wave16_models tests (six fbench, five silver) pass twice debug and twice release with identical normalized names/results. Original supplied tiny fixtures only. old.log records three failures on unchanged production: duplicate_links_are_occurrence_bounded, prompt_context_is_source_scoped, prompt_adjacency_does_not_wrap. fbench membership credit yielded tp=3 against gold=1; prompt suppressed cross-source context and overflowed at i64::MAX. Corrections preserve ordinary unique-link and valid same-source adjacency behavior.

Compiled production controls: fbench roles<2 changed to roles<3 fails alignment_ties_and_threshold; silver duplicate-number guard removed fails gates_partition_numbers_and_keep_junk_separate. Exact source bytes restored before the four positive runs. No teacher/live parser/corpus/GPU or run/collect/read_silver calls. No global environment changes. Existing unrelated frames.rs unused_mut warning remains.

Source staged before two identical syntax projections. Full external registry scaffold validates twice identically; changed registration passes without editing shared registry or parents. Two file roots and six fragments contain 24 declared typed views. Partial syntax is not semantic AST; model declaration is not proof.

## History

### 2026-10-30 · epoch-memory

2026-10-30: prospective inspection predicted predecessor defects and duplicate-edge recall inflation. Three old-code witnesses confirm these. Valid adjacency and unique links retain semantics. No score assertions or deduplication introduced. Historical metrics retain original semantics.

### 2026-10-30 · receipts

External evidence: $MOVA_DATA/runs/octopus-wave16-mova-2026-10-30.

#### SHA-256 receipts

- old.log: edc5ee1a9a60d4d0388a3a399a14b643593ebff159c3f06c7ea8d69dff3e8ba9
- mutant-fbench.log: 1da7cb3ccc043ad21b57bf45bfd0e78879a9c5440d2e69a2f218a245fae87316
- mutant-silver.log: a59d8fbd1dbc4063c229f6a760b5b3a4c810e0200cdca81292c1f4884f4c370f
- debug-1.log: 5e210c903b5345ac9f57e77ecd9ab41efbcb0c8cce2684bf24d1ffef5064c433
- debug-2.log: 958355cde5aa03b3ef8ba795ed39f2685d1a0a319387da6ebc17a121e71b728c
- release-1.log: 014601ab90414997d7dad60f45352284356b1ffe0e9f986a3e404735fc7ccbc0
- release-2.log: c049ef711fc353d394d6b4d6b8d9d5f592a9a2ef9425316da2551f407837488a
- normalized.txt: 20c3f1cb8f7854fe2d90f51a351b50a98894147ea991e2cfa90bbb84f8e321f1
- groupants-1.json: 662080ae53521982d655a55071f6306c7689d5049e971c2fa50051e708691c76
- groupants-2.json: 662080ae53521982d655a55071f6306c7689d5049e971c2fa50051e708691c76
- changed.json: 039a841bb953c81cd2f43a058ac68afc0fcdb306884582bbe6b292a83be1b106
- silver-syntax-1.json: 0de276010f5c3d219d2dce264abc333c60c14fddc6b57beb0a60cae77ec65d5c
- silver-syntax-2.json: 0de276010f5c3d219d2dce264abc333c60c14fddc6b57beb0a60cae77ec65d5c
- silver.rs: adf915c8d5e9059d3f58b20eca8d9b94721ee7fd9f39ac69f93775a6b5c856c9
