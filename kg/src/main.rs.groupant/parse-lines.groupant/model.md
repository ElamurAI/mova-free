# kg-main-parse-lines

## descriptant

### kg-main-parse-lines

Arbitrary TSV helper and synthetic Morph→parse→report composition.

## approximant

### Forward relation

Read valid first-tab rows until line read failure, load names, collect normalized vocab, Morph::load, then create output and emit ok/members or reject reason.

#### Executable approximant gap

Exact categorical structure, identifiers, parser decisions and effects have no useful percentage metric established here. No executable surrogate, automatic substitution or 1/2/5/10% guarantee is claimed.

## invariant

### Invariant and domain

Output has one row per retained TSV row; malformed no-tab rows skipped; synthetic2-row fixture retains ids and question rejection.

#### Cost and resources

O(input bytes)+two dictionary passes+parse costs; all rows retained before output.

## cost

### Cost and resources

O(input bytes)+two dictionary passes+parse costs; all rows retained before output.

## composition

### Composition

Arbitrary TSV helper and synthetic Morph→parse→report composition.

Read valid first-tab rows until line read failure, load names, collect normalized vocab, Morph::load, then create output and emit ok/members or reject reason.

The parent invokes scoped transformations through the actual functions named in their source ranges. Children partition useful observations, not a complete semantic proof; boundaries can omit helpers, tests and effects.

## models

## behavior_complement

No output is created before input/names/Morph succeed. ok does not imply render_check; no grammar completeness validation here.

## reverse_reconstruction

Output ignores malformed rows/read-error tail and original morphology alternatives; id and text retained for accepted rows.

## first_tab

Text may contain additional tabs because split_once retains suffix.

## read_failure

map_while(Result::ok) stops silently at first I/O error rather than propagating it.

## status_projection

Reject::code collapses detailed unknown/conjunction words into category labels.

## synthetic_scope

Actual tiny bz2 dictionary and names file are used; no stored graph is modified by this helper.


## tests

### Evidence

Parent source checks (a fragment inherits only the behavior they actually exercise):

- wave29::synthetic_reader_parse_and_sub_precedence

Selected kg tests: 15 named outcomes, twice debug/twice release, equal sorted outcomes. Standalone actual global/build.rs: 3 named outcomes per run, twice debug/twice optimized; build_child is normally no-op, parent fixture invokes it against isolated synthetic seeds. Eight compiled source mutations (one per source) each compiled successfully and exited101 on the focused assertion; exact source restoration followed by positive rerun.

This fragment's untested branches remain source-derived. No live graph, DB, corpus, teacher, GPU or held-out evaluation was used. Command specs and numeric exits are retained in the external wave29 graphlang receipt. Strict kg Clippy reports existing source lints; see receipt.

## History

### 2026-10-31 · descriptant

#### Modeling epoch — 2026-10-31

Source read before tests and models; retained prospective notes accompany the parent. Current evidence uses bounded invented inputs only. This is partial modeling, not proof or corpus accuracy.
