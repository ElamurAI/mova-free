# cycle-db

## descriptant

### cycle-db

#### Descriptant

Prospective inspection identified this relation before additions: blocks takes first sent_id per double-newline block. update merges by id preserving positions, renames current before history/log append. agree picks strongest newest base, aggregates weighted head/relation votes and falls back if tree_ok fails. same_text whitespace-normalizes and copies lexicographically first matching id. training_file excludes heldout ids and takes first database occurrence.

The production implementation is unchanged; only selected inline tests are appended. Source-bound syntax is generated twice and compared. Native checks exercise bounded helpers; entry orchestration, teacher calls, models and corpora remain source-derived. Partial syntax is not semantic proof. Preserve failure prefixes and ambiguity when refining this model.

## approximant

# Forward relation

blocks takes first sent_id per double-newline block. update merges by id preserving positions, renames current before history/log append. agree picks strongest newest base, aggregates weighted head/relation votes and falls back if tree_ok fails. same_text whitespace-normalizes and copies lexicographically first matching id. training_file excludes heldout ids and takes first database occurrence.

## Composition

update establishes current/history/log consumed by agree/trainer; failures may leave inconsistent observations. same_text joins update then update_prag. training_file shares ids_of. Children isolate commit and vote boundaries.

## catalog

# Typed observations: cycle-db

## behavior_complement

Successful unique-id update preserves existing positions, appends new ids once. with_drafts excludes replacement of known DB ids but permits repeated new draft ids. No rollback guarantee.

Scope: the bound source interval and stated assumptions. Evidence: source inspection plus the checks explicitly named in model.md#tests; no finite check establishes all inputs.

## reverse_reconstruction

Current trees admit arbitrarily many replaced histories. Added/replaced counts constrain event multiplicity, not bodies. Voted tree admits many weighted multisets and invalid-vote fallback antecedents.

Scope: the bound source interval and stated assumptions. Evidence: source inspection plus the checks explicitly named in model.md#tests; no finite check establishes all inputs.

## vote_order

Best maximizes (weight,time,order). Candidate score=(sum weights,max time,max order), independent maxima possibly from different versions. Equal score chooses largest ordered key.

Scope: the bound source interval and stated assumptions. Evidence: source inspection plus the checks explicitly named in model.md#tests; no finite check establishes all inputs.

## tree_domain

Empty false; exactly one root, heads≤W and no self-head precede W+1-link traversals. Detached cycles reject despite one root.

Scope: the bound source interval and stated assumptions. Evidence: source inspection plus the checks explicitly named in model.md#tests; no finite check establishes all inputs.

## heldout_first_source

Scope: the bound source interval and stated assumptions. Evidence: source inspection plus the checks explicitly named in model.md#tests; no finite check establishes all inputs.

## effect_prefix

Directory creation precedes input read; old-current read errors become empty. Rename precedes second input read, history and log append. same_text prag follows tree commit.

Scope: the bound source interval and stated assumptions. Evidence: source inspection plus the checks explicitly named in model.md#tests; no finite check establishes all inputs.

## reviewed_selection

Behavior complement: conflicting or stale selected content excludes normal DB writes/training; full selected tags/heads cannot be replaced by ordinary votes. Reverse reconstruction: current alone admits many histories; complete before snapshot and transaction inventory constrain prior bytes under quiescence. SHA256 binds byte content, not semantics or operator identity.

## tests

### Checks and limits

No canonical DB, model, corpus, GPU, teacher or heldout evaluation is executed. Source-bound syntax and model declarations remain partial; no universal durability or semantic accuracy proof.

## evidence

### Wave33 evidence

Across this nine-source slice, 14 exact native names passed twice in debug and twice in release (56 runs); sorted target-qualified outcomes agree. Six compiled behavior controls fail named assertions and exact restored bytes pass again. One exploratory parser-count control failed by unwrap panic; it was restored and replaced by a form-column assertion control. No assertion-only control is counted.

Nine syntax projections were generated twice and byte compared. Production source prefixes match the base exactly; four files append only formatted cfg(test) blocks. Local groupants validation reports valid=true, zero diagnostics. Counts: nine source roots, ten scoped children, 114 typed views.

## invariant-and-cost

### Invariant and cost

Held-out filter: H held-out sentences, B export bytes: O(H log H) load, O(B + S log H) filter (one normalization per block); the exported set is a subset of the unfiltered one, identical when no source matches. No three-file transaction. Draft union uses snapshot known ids, not within-draft dedup. Vote requires one root and acyclic in-range nonself heads; fallback base is not validated. Weights 8/4/2/1 follow source names.

B bytes,S sentences,V versions,W words: O(B+S log S) merge, O(V W log V+W²) vote/tree check, output/history I/O. same_text O(B+S log S). train_dbs path dedup O(D²).

No executable percentage surrogate is claimed: effects, categorical acceptance and report text have no justified approximate metric here. Future profiles require a useful metric, zero case, reference and evidence at caps 1/2/5/10%.

#### Reviewed selection

One full-tree/prag policy batch overrides votes without changing source weights. Guards validate selected hashes/current equality before effects. Parsing is linear in bytes and sentence boundaries; policy adds full-file maps and selected structural checks. Explicit reviewed promotion/rollback require quiescent callers and use full snapshots, staging and journal; no cross-file atomicity.

## History

### 2026-10-31 · descriptant

Heading `#### Epoch 2026-10-31` stays in descriptant as `#### Descriptant`.

### 2026-10-31 · evidence

Heading `### Wave33 evidence — 2026-10-31` stays in evidence as `### Wave33 evidence`.

See model.md#tests for this source's precise executed or representation-only coverage. Native evidence does not apply to ignored live teacher or lesson execution. No teacher/API, model/corpus, learning or benchmark CLI was run. Receipts retain numeric statuses, exact names, log/source hashes, original backups, unique mutation replacements and restoration assertions. The external bundle is named octopus-wave33-cycle-2026-10-31.

### 2026-11-01 · descriptant

#### Epoch 2026-11-01

Implemented explicit content-bound reviewed selection, separate train/dev authorization, full-tag/head preservation after agree, and no-separator normalization with preamble ownership. A real old-vote negative control demonstrates why a current-only correction is lost. Legacy update effects outside reviewed selection still lack a multi-file transaction. Bounded tests do not establish semantic quality.

### 2026-11-01 · document

#### Epoch 2026-11-01 current relation

blocks now separates each sent_id and carries contiguous preamble comments to its following sentence. Current writes validate persistent reviewed selection; update/prag reject conflicting selected content. agree applies the entire selected tree after its ordinary source-weight vote. training_file intersects explicit train decisions with heldout exclusions, and strict primary policy rejects unreviewed secondary DBs. Reviewed promotion/snapshot/rollback lives in reviewed.rs; legacy update remains nontransactional.

### 2026-11-01 · evidence

#### Epoch 2026-11-01

Native synthetic reviewed selection test now executes actual db::agree conflict/overlay paths, normal update/prag conflict rejection, training split/heldout exclusions and interrupted rollback. Separate joined-preamble regression protects newdoc/version/source metadata. Current suite results are retained in the external DB policy report; historical wave33 evidence above is unchanged.

### 2026-11-04 · descriptant

#### Epoch 2026-11-04

agree is agree_with(db, CYCLE_VOTE_MISC=1). With the margin on, every word line (not ranges or empty nodes) of a voted sentence gets MISC Vote (share of the weight behind its kept (head, relation)), VoteHead (behind its head) and VoteAlt (other heads, largest share first, at most 4, shares ≥ 0.005 before two-decimal rounding), replacing earlier vote items and keeping other MISC items; this holds for the voted tree and for the fallback tree. With it off, current.conllu is byte for byte as before (reviewed promotion pins current-tree hashes).

### 2026-11-05 · descriptant

#### Epoch 2026-11-05

Leak found 2026-11-04: training_file excluded only the school test ids, so the staged parse DB export carried all 335 school dev sentences (and 7 school test ids in an older export built with other databases). Added Heldout: one BTreeSet of ids and one of normalized texts (norm_text: lowercase alphanumerics; a text without any keeps its non-space characters) loaded from CoNLL-U files or a Wikipedia-school corpus directory (every article of split.tsv). training_file_from drops a sentence whose id or normalized text is held out, counts and logs the drop; filtered_inputs applies the same test to other training files. Measured on the staged DB (791 385 sentences, primary DB only): 3 248 dropped (school dev 335 and school test 2 712 by id, 201 more by text: school 5, question 109, book 49, dict 38), 788 137 kept; two runs byte-identical.

#### Kept file names — 2026-11-05 (retrain-clean)

filtered_inputs writes <dir>/<i>/<file name> instead of <dir>/<i>-<file name>, so en's licence gate (which reads the source from the file name) accepts a filtered copy; filtered_counts also returns the kept and dropped counts per file and logs both.

### 2026-11-05 · document

#### Epoch 2026-11-05 current relation

training_file(db, held) = training_file_at(db, held, db/train-db.conllu) = training_file_from([db] + train_dbs, held, out): blocks in DB order, first DB policy strictness, per-DB reviewed allowlist, then held.hits(id, block) drops (counted, logged), then first-id dedup. Output = {b : allowed(b) ∧ ¬held.hits(b) ∧ first occurrence}. Heldout.load fails on a missing required source and records a missing optional one as absent.

### 2026-11-05 · heldout_first_source

Explicit DB first then configured paths; held-out ids and normalized texts are excluded before seen insertion (2026-11-05). Behavior complement: no exported block has a held-out id or a normalized text equal to a held-out sentence's. Reverse: a dropped count admits many held-out sets and DB contents; normalization is many-to-one, so a train sentence differing from a held-out one only in case, spacing or punctuation is also dropped (deliberate over-exclusion), and two different texts can share a key.

### 2026-11-05 · tests

db::heldout_tests (2026-11-05): normalized_text_ignores_case_spacing_and_punctuation; dev_sentences_never_reach_the_training_export (the negative control: a dev id and the dev text under another id are dropped, a near text stays; with an empty Heldout both reach the export, so the gate shows red; a missing required source fails; filtered_inputs keeps the same set); wikipedia_corpus_contributes_every_split. Mutation check: hits reduced to ids only fails both export tests. Legacy db::tests and db::wave33_tests remain. reviewed::tests now executes real agree old-vote conflict and selected full-tree preservation, conflicting update/prag rejection, train/dev/heldout filtering, duplicate/hash/header/phase rejection and interrupted snapshot rollback on synthetic temporary DBs. joined_preambles_belong_to_following_sentence checks metadata ownership without blank separators. Run with one test thread due to existing process-environment fixtures.

### Undated · document

Subject: tools/cycle/src/db.rs, SHA-256 34f08db359c4d620f91bb9b5a586653ff948019524e4b21ff1b21f37c413c5b9. Domain: bounded source inputs, declared filesystem/environment state and normal allocations. Each view is a partial source-derived relation, not a universal proof. Observation is one helper call or the explicitly described trace. Checks are in model.md#tests; unexecuted effects remain gaps.
