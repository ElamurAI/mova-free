# cycle-main

## descriptant

### cycle-main

#### Descriptant

Prospective inspection identified this relation before additions: CLI validates Codex-only teacher for training commands then loads Cfg. pre/post flows skip existing artifact stages and call external tools; final trees append, check and optional train precede row. Helpers convert CoNLL-U to parser ids/tokens, extract log metrics, choose newest eligible model and report exact aligned-token changes.

The production implementation is unchanged; only selected inline tests are appended. Source-bound syntax is generated twice and compared. Native checks exercise bounded helpers; entry orchestration, teacher calls, models and corpora remain source-derived. Partial syntax is not semantic proof. Preserve failure prefixes and ambiguity when refining this model.

## approximant

# Forward relation

CLI validates Codex-only teacher for training commands then loads Cfg. pre/post flows skip existing artifact stages and call external tools; final trees append, check and optional train precede row. Helpers convert CoNLL-U to parser ids/tokens, extract log metrics, choose newest eligible model and report exact aligned-token changes.

## Composition

build supplies default path. pipeline calls ids_of/parser/log/report/model helpers, DB calls ids_of. jobs wraps draft commands. Child scopes separate parser representation from change-report observation; external models and teacher unresolved.

## catalog

# Typed observations: cycle-main

## behavior_complement

Invalid configured training teacher is rejected before Cfg load. parser_input count mismatch excludes file writes for ids/tokens but not directory creation. Token-form mismatch excludes that sentence's change counts.

Scope: the bound source interval and stated assumptions. Evidence: source inspection plus the checks explicitly named in model.md#tests; no finite check establishes all inputs.

## reverse_reconstruction

Report reconstructs only compatible counts/ranked relation transitions, not original trees; unreported sentences, words beyond top examples and lexical inputs remain ambiguous. Missing metrics NaN merges many malformed/missing logs.

Scope: the bound source interval and stated assumptions. Evidence: source inspection plus the checks explicitly named in model.md#tests; no finite check establishes all inputs.

## resume_artifacts

Each existing marker skips its stage; existence is not validation or a transaction. A final artifact can survive failure before table row and affect restart.

Scope: the bound source interval and stated assumptions. Evidence: source inspection plus the checks explicitly named in model.md#tests; no finite check establishes all inputs.

## parser_coordinates

Only ten-column rows without '-' or '.' id contribute forms; sent_id metadata enters independently. Equal counts validate cardinality, not semantic id/token pairing.

Scope: the bound source interval and stated assumptions. Evidence: source inspection plus the checks explicitly named in model.md#tests; no finite check establishes all inputs.

## model_publication

gparse_for maximizes (mtime,path) for model.safetensors excluding .part and round j≥k; unrelated names qualify. part_dir creates sibling suffix, train success precedes rename.

Scope: the bound source interval and stated assumptions. Evidence: source inspection plus the checks explicitly named in model.md#tests; no finite check establishes all inputs.

## log_schema

apply_metrics first '| final | Rust |' numeric cells indices 6/7, first cost marker and first ': +' integer; missing UAS/LAS/cost NaN, missing added zero. num_after consumes only digits/dots.

Scope: the bound source interval and stated assumptions. Evidence: source inspection plus the checks explicitly named in model.md#tests; no finite check establishes all inputs.

## tests

### Checks and limits

tests::changes_counts_moves; tests::part_dir_is_a_sibling; tests::metrics_from_logs; tests::parser_input_matches_ids. No round/training/report CLI.

Exact native outcomes and command metadata are retained in the external wave33 cycle receipt bundle. Selected names run twice in debug and release; syntax is independently regenerated twice. Compiled mutations and source restoration receipts distinguish behavior changes from assertion-only controls. These claims are finalized in model.md#evidence. No live teacher, real model/corpus, benchmark CLI or lesson-learning CLI is run.

## evidence

### Wave33 evidence

Across this nine-source slice, 14 exact native names passed twice in debug and twice in release (56 runs); sorted target-qualified outcomes agree. Six compiled behavior controls fail named assertions and exact restored bytes pass again. One exploratory parser-count control failed by unwrap panic; it was restored and replaced by a form-column assertion control. No assertion-only control is counted.

Nine syntax projections were generated twice and byte compared. Production source prefixes match the base exactly; four files append only formatted cfg(test) blocks. Local groupants validation reports valid=true, zero diagnostics. Counts: nine source roots, ten scoped children, 114 typed views.

## invariant-and-cost

### Invariant and cost

parser_input requires id count=nonempty token groups, creates dirs before validation; model .part excluded from selection and renamed after successful train. changes compares exact token form arrays, denominator max(words,1). Existing files signal completion without content validation.

Parser/report O(bytes+words+distinct changes log distinct changes); model selection O(directory entries). Wrapper cost sums subprocess costs and filesystem bytes, unmeasured. Stages can leave durable prefixes on failure.

No executable percentage surrogate is claimed: effects, categorical acceptance and report text have no justified approximate metric here. Future profiles require a useful metric, zero case, reference and evidence at caps 1/2/5/10%.

## History

### 2026-10-31 · descriptant

Heading `#### Epoch 2026-10-31` stays in descriptant as `#### Descriptant`.

### 2026-10-31 · evidence

Heading `### Wave33 evidence — 2026-10-31` stays in evidence as `### Wave33 evidence`.

See model.md#tests for this source's precise executed or representation-only coverage. Native evidence does not apply to ignored live teacher or lesson execution. No teacher/API, model/corpus, learning or benchmark CLI was run. Receipts retain numeric statuses, exact names, log/source hashes, original backups, unique mutation replacements and restoration assertions. The external bundle is named octopus-wave33-cycle-2026-10-31.

### 2026-11-01 · descriptant

#### Epoch 2026-11-01

Added two local reviewed DB commands; existing round/teacher/training paths retain their dispatch behavior. Reviewed orchestration is modeled in cycle-reviewed and the parent composition.

### 2026-11-01 · document

#### Reviewed DB CLI — 2026-11-01

Dispatch promote-reviewed TREES PRAG DECISIONS and rollback-reviewed HASH to local reviewed module. CYCLE_DB determines target. Cfg load remains ahead of dispatch and requires configured MOVA_BIG; these commands never invoke teacher/model training.

### 2026-11-05 · descriptant

#### Held-out sources — 2026-11-05

Cfg.heldout lists the development and held-out sources (EWT dev/test, GUM dev/test, ESLSpok dev/test, school dev, CYCLE_TEST, the Wikipedia-school corpus) as optional, plus CYCLE_HELDOUT entries as required; heldout() loads and logs them. The pre-flow fine-tune trains on filtered copies of its inputs and GP_ALWAYS file. db-export OUT writes the filtered training export of CYCLE_DB.

#### Annotator inputs — 2026-11-05 (retrain-clean)

annot_train(c, extra, dir) writes CYCLE_BASE plus extra (the cycle's agreed corpus) through db::filtered_inputs into <dir>/base-inputs/<i>/<file name> and returns those paths; empty CYCLE_BASE stays empty and loads no held-out source. Both flows give annot drafts and annot batch-prepare (whose state keeps the files for its FEATS tables) only these copies; the pre-flow fine-tune rebuilds its base+agreed list for its own filter. heldout-filter OUT FILE... writes filtered copies of any CoNLL-U files with the configured Heldout and prints kept/dropped per file. Behavior complement: no annotator training file of a round carries a held-out id or normalized text; reverse: a copy admits every source file that differs only in held-out blocks. Measured 2026-11-05: school-agreed-train.conllu 39 586 → 39 242 (335 school dev ids, 9 by text), the 15.10 school-db.conllu 48 289 → 43 520 (335 school dev, 4 425 school test, 9 by text), EWT train −557 and ESLSpok train −129 (short texts such as greetings equal to a dev/test or Wikipedia-school sentence).

### Undated · document

Subject: tools/cycle/src/main.rs, SHA-256 c901ffdda1a67fa69ed553c5d0c8e7e83d1aa0c23ce8a7bc74d53f30e5c5b108. Domain: bounded source inputs, declared filesystem/environment state and normal allocations. Each view is a partial source-derived relation, not a universal proof. Observation is one helper call or the explicitly described trace. Checks are in model.md#tests; unexecuted effects remain gaps.
