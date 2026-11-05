# global-build-auxiliary

## descriptant

### global-build-auxiliary

Auxiliary score/idiom tables and expression copy.

## approximant

### Forward relation

Read optional real cells into sorted map, derive lexical verb/noun IDs, emit. Read tale rows in input order with existing-pair/score/duplicate gates. Read closed idioms into sorted pair map validating kind; copy expressions bytes as UTF-8 text or empty.

#### Executable approximant gap

Exact categorical structure, identifiers, parser decisions and effects have no useful percentage metric established here. No executable surrogate, automatic substitution or 1/2/5/10% guarantee is claimed.

## invariant

### Invariant and domain

Real cells unique with scores0..4; tale keys subset of real. Tale order is not sorted by code. Expressions copy is separate write.

#### Cost and resources

O(B+R log R) plus output bytes; entire optional files read.

## cost

### Cost and resources

O(B+R log R) plus output bytes; entire optional files read.

## composition

### Composition

Auxiliary score/idiom tables and expression copy.

Read optional real cells into sorted map, derive lexical verb/noun IDs, emit. Read tale rows in input order with existing-pair/score/duplicate gates. Read closed idioms into sorted pair map validating kind; copy expressions bytes as UTF-8 text or empty.

The parent invokes scoped transformations through the actual functions named in their source ranges. Children partition useful observations, not a complete semantic proof; boundaries can omit helpers, tests and effects.

## models

## behavior_complement

Bad score/duplicate/unknown tale key and bad idiom kind panic; missing/unreadable optional files do not.

## reverse_reconstruction

IDIOMS drops count/source; absence and unreadable file collapse to empty. Tale sequence retains order, while real table sorts away it.

## score_domain

u8 parse failure defaults9, then >4 rejects; negative text rejects.

## tale_order

Reversed valid tale keys emit reversed rows; downstream binary search assumptions are not proven.

## closed_boundary

closed_data reads `$MOVA_PETA_DATA/global/<name>`, the directory taken from the environment, else from the line `MOVA_PETA_DATA=` of the manifest-relative `peta/paths.env`; with neither it names an absent file under OUT_DIR, so the tables read as empty (public checkout). The tables left the repository (Peta holds no data); the old sibling `peta/global/data` is no longer read.

## partial_effect

expressions may be written before later final global.rs write failure; no rollback.


## tests

### Evidence

Parent source checks (a fragment inherits only the behavior they actually exercise):

- wave29::block_and_code_boundaries
- wave29::isolated_seed_compile_and_duplicate_gate
- wave29::build_child

Selected kg tests: 15 named outcomes, twice debug/twice release, equal sorted outcomes. Standalone actual global/build.rs: 3 named outcomes per run, twice debug/twice optimized; build_child is normally no-op, parent fixture invokes it against isolated synthetic seeds. Eight compiled source mutations (one per source) each compiled successfully and exited101 on the focused assertion; exact source restoration followed by positive rerun.

This fragment's untested branches remain source-derived. No live graph, DB, corpus, teacher, GPU or held-out evaluation was used. Command specs and numeric exits are retained in the external wave29 graphlang receipt. Strict kg Clippy reports existing source lints; see receipt.

## History

### 2026-10-31 · descriptant

#### Modeling epoch — 2026-10-31

Source read before tests and models; retained prospective notes accompany the parent. Current evidence uses bounded invented inputs only. This is partial modeling, not proof or corpus accuracy.

### 2026-11-04 · closed_boundary

closed_data reads `$MOVA_PETA_DATA/global/<name>`, the directory taken from the environment, else from the line `MOVA_PETA_DATA=` of the manifest-relative `peta/paths.env`; with neither it names an absent file under OUT_DIR, so the tables read as empty (public checkout). 2026-11-04: the tables left the repository (Peta holds no data); the old sibling `peta/global/data` is no longer read.

### 2026-11-05 · public export

closed_data now emits rerun-if-changed itself, for peta/paths.env and for the resolved table only when each exists (expressions and idioms no longer emit their own). Cargo treats a watched absent path as always changed, so a public checkout would rebuild global on every build. Tables read and their contents are unchanged: an absent file still reads as empty. Checked by compiling the public export (cargo check, 2026-11-05).
