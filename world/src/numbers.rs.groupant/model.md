# number-normalization

## descriptant

# number-normalization

## Forward

Tokenize with separators, lowercase, rank corrections, context-gate them, then read digit/word spans and emit normalized text plus Found.

## Invariant

Each Found consumes tokens; arithmetic assumes bounded nonoverflowing inputs. Internal separators of consumed word spans are discarded.

## Cost

Cold correction O(sum_v m*len(v)) DP work plus trigrams; fixed vocabulary, unbounded distinct-query cache. Tokenization is linear in text size.

## Composition

Correction uses original lowercase neighbors. Reading sees fixed tokens but not separators; rendering keeps final separator. Overlapping child costs cannot be blindly summed.

## Epoch memory

## catalog

# number-normalization typed models

## nonbehavior

Type: behavior_complement. Domain, observation and operator: U=bounded strings x (normalized text,Found vector); R=token/correct/read/render with fixed dictionary. Candidate inner bound of U minus R includes zero-token recognized spans, noncanonical ordinal suffixes in normalized output, or correction words outside vocabulary.

Evidence/falsification: Existing tests and probe literal cases. See root model.md#evidence for executed cases; proposed checks remain unexecuted. Status: partial source-derived model with bounded evidence only.

## antecedents

Type: reverse_reconstruction. Domain, observation and operator: F^-1(y) retains all texts with normalize(x)=y; observing text alone admits fifty five and fifty-five for 55. Found narrows fibers but loses internal separators, so no unique reconstruction.

Evidence/falsification: Probe finite alias fiber. See root model.md#evidence for executed cases; proposed checks remain unexecuted. Status: partial source-derived model with bounded evidence only.

## digit-spellings

Type: normalization_fiber. Domain, observation and operator: Digits remove all commas in initial digit/comma prefix; edge commas, overflow and suffix outside empty/st/nd/rd/th reject. Grouping and suffix agreement are unchecked: 1,2 and 12st canonicalize.

Evidence/falsification: Probe literal cases. See root model.md#evidence for executed cases; proposed checks remain unexecuted. Status: partial source-derived model with bounded evidence only.

## correction-rank

Type: argmin_preimage. Domain, observation and operator: Candidates minimize (half-edit distance,negative integer Dice,vocabulary position); threshold uses candidate byte length >=7 for 4, otherwise 2. Winning inputs satisfy all comparisons plus admission.

Evidence/falsification: Probe swaps/exact-word rejection; exhaustive rank check proposed. See root model.md#evidence for executed cases; proposed checks remain unexecuted. Status: partial source-derived model with bounded evidence only.

## context-admission

Type: predicate_partition. Domain, observation and operator: Correction blocks uppercase names outside sentence starts (token zero or immediately after a separator containing period, question mark or exclamation mark) and plural number inflections. Unknown dictionary forms otherwise pass; real forms need original-neighbor kind fit and punctuation boundary gate.

Evidence/falsification: Existing contextual test. See root model.md#evidence for executed cases; proposed checks remain unexecuted. Status: partial source-derived model with bounded evidence only.

## ordinal-barrier

Type: state_transition. Domain, observation and operator: read_words stops after ordinal and blocks non-scale ordinal after unit/teen as fractional continuation; normalize additionally guards previous a/unit/teen/digit.

Evidence/falsification: Existing fractions test. See root model.md#evidence for executed cases; proposed checks remain unexecuted. Status: partial source-derived model with bounded evidence only.

## quantity-observation

Type: projection_difference. Domain, observation and operator: quantity never corrects, returns first cardinal or once/twice/thrice, skips ordinal word spans and lacks normalize fraction guard. API outputs need not agree.

Evidence/falsification: Probe quantity literals. See root model.md#evidence for executed cases; proposed checks remain unexecuted. Status: partial source-derived model with bounded evidence only.

## renderer-domain

Type: bounded_roundtrip. Domain, observation and operator: Roundtrip is claimed only for 0<=n<3000 covered by existing tests. to_words uses thousand quotient with below1000 helper; full i64 rendering is outside model.

Evidence/falsification: Probe million boundary and existing loop. See root model.md#evidence for executed cases; proposed checks remain unexecuted. Status: partial source-derived model with bounded evidence only.

## suffix-fiber

Type: residue_preimage. Domain, observation and operator: For nonnegative n, st iff last digit 1 except residue11; nd and rd analogously except12/13; th remaining residues. Negative signed remainders excluded.

Evidence/falsification: Probe 0..1999. See root model.md#evidence for executed cases; proposed checks remain unexecuted. Status: partial source-derived model with bounded evidence only.

## separator-loss

Type: information_loss. Domain, observation and operator: Word parsing ignores separators; fifty. five can combine to55. Found.said joins tokens by spaces and output retains only final separator.

Evidence/falsification: Probe punctuation fixture. See root model.md#evidence for executed cases; proposed checks remain unexecuted. Status: partial source-derived model with bounded evidence only.

## cache-observation

Type: state_fiber. Domain, observation and operator: correct memoizes Option by exact string in Mutex BTreeMap; same result admits hit and miss states. Poison/allocation failures excluded; cache size has no global bound.

Evidence/falsification: Probe repeated calls. See root model.md#evidence for executed cases; proposed checks remain unexecuted. Status: partial source-derived model with bounded evidence only.

## typo-domain

Type: enumeration_scope. Domain, observation and operator: typo_check generates deletion, duplication and adjacent swaps, per-word deduplicates, filters exact/dictionary words; despite doc wording no alphabet-neighbor substitution generator exists.

Evidence/falsification: Source inspection; independent enumeration proposed. See root model.md#evidence for executed cases; proposed checks remain unexecuted. Status: partial source-derived model with bounded evidence only.


## tests

### Checks

Root model.md#evidence records direct real-source Rust tests and bounded probes, including false-formula rejection. Catalog checks marked proposed remain unexecuted. Syntax/manifest validation checks identity and structure only. No GPU, live teacher, full application run or general proof.

## evidence

### Executed evidence

Four tests pass twice: two existing source tests plus bounded_numeric_oracles and correction_and_control. Existing loops cover 3000 cardinals and 2999 ordinals. New checks cover 2000 suffix residues, five normalization literals, four quantity literals, three edit distances, three correction rejections and repeated cache calls. FALSE_FORMULA=1 rejects false transposition weight 2 (actual 1), exit 101.

Observed boundaries: fifty. five becomes 55 and Found.said is fifty five; 1,2 becomes 12; 12st becomes 12th. These are current behavior, not desirable-language guarantees. Initial million-rendering expectation was wrong: actual output is ten hundred thousand because unit(10) exists. Corrected probe expectation without changing production; initial failure remains in numbers-initial-boundary.log. Negative/arbitrarily large i64 rendering, overflow paths, full correction ranking and full punctuation semantics remain unproved.

#### Reproduction and provenance

probe.txt is Rust source kept as a sidecar artifact. Compile from repository root using rustc --edition=2024 --test --crate-name numbers_groupant_probe world/src/numbers.rs.groupant/probe.txt, adding -L dependency=DEPS and --extern dependencies from a default-CPU build. Numbers needs en and anyhow; values needs global. Run the binary with --nocapture --test-threads=1. Set FALSE_FORMULA=1 and select correction_and_control --exact --nocapture for the false-formula control; expect exit101.

Current numbers.rs/values.rs were directly compiled through #[path] modules, including original tests. Dependencies en/global/anyhow reuse the prior octopus-groupants-anchored default-CPU build; world fingerprint records features=[] at baseline 5794fd531. Git diff from that baseline to current baseline, excluding sidecars, found no changes in en, global, Cargo.lock or either source. Generated dictionary/global contents are that historical build snapshot, not newly rebuilt or proven against live external data. No world rlib reuse is needed.

#### Structural scope

Each root has twelve entries and three four-entry fragments: twenty-four models and four nodes per file; forty-eight models and eight nodes together. First entries target behavior complement U minus R; second entries preserve ambiguous reconstruction fibers. Partial syntax supplies source-bound fragment envelopes. Macro expansion, name resolution, semantic AST and universal correspondence remain gaps.

mova-modelcheck groupants passes unchanged registry: 20 file sidecars, six registered groups, 512 missing file sidecars, no diagnostics. New roots await parent registration through registry-node.json. changed against 4279da541 reports no changed production Rust. Structural success is not behavioral verification. No GPU, training, live teacher or active-data mutation.

## History

### 2026-10-29 · Epoch memory

2026-10-29: source-derived initial model, source hash 41e5c848b76cd85ba99f84746d76099707d968957fb1b8774e8a35def2b23ec6. Scope envelope [0,22566) is anchored to selected partial syntax nodes, not semantic proof or full coverage. Runtime source unchanged. AST omits macros expansion, name resolution, inferred types and cfg evaluation. Generated dictionary/global tables are fixed dependencies not bound by this file hash. Catalog false-formula controls and boundary observations refine claims without modifying production.

### 2026-10-29 · document

Epoch 2026-10-29; subject numbers.rs, SHA-256 41e5c848b76cd85ba99f84746d76099707d968957fb1b8774e8a35def2b23ec6. Domain is the bounded calls/traces stated below; observation horizon is one invocation unless explicitly a trace. Fixed compiled global/dictionary tables, successful allocation, unpoisoned locks and nonoverflowing arithmetic are assumptions. Every relation is a partial source-derived specification; Rust correspondence beyond recorded checks remains open. Behavior complements target U minus R and restrictions are candidate inner bounds, not proven complements. Reverse relations preserve all compatible antecedents. Hash traversal is an explicit hidden input for values.

### 2026-10-29 · evidence

Epoch 2026-10-29; baseline 4279da541. Runtime source unchanged.

External artifacts: $MOVA_DATA/runs/octopus-next-lexical-2026-10-29/numbers-positive.log, numbers-repeat.log, numbers-negative-confirmed.log, dependency-diff.txt, verification.txt, numbers-syntax-repeat.json, groupants.json and changed.json. Test observations repeat identically excluding harness elapsed-time summary. No performance claim. Syntax repeats byte-for-byte.
