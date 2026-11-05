# treebanks-main

## descriptant

### Descriptant

build streams release, rejects all stats mismatches before metadata gate, then builds indexes and collects references. collect runs matcher, filters noncitations, deduplicates by sorted explicit ids/DOIs else normalized bib title/text, audits first item, merges found identities preferring lower How/fuzzy tuple. build writes cards/language/global indexes then run artifacts sequentially. No seeds or converter writes.

## approximant

# Forward approximant

build streams release, rejects all stats mismatches before metadata gate, then builds indexes and collects references. collect runs matcher, filters noncitations, deduplicates by sorted explicit ids/DOIs else normalized bib title/text, audits first item, merges found identities preferring lower How/fuzzy tuple. build writes cards/language/global indexes then run artifacts sequentially. No seeds or converter writes.

## Composition

Child fragments model one narrow helper each; their correctness alone does not establish orchestration.

The containing source wires this scoped relation to its callers. Treeagree proposal→gate→commit and treebanks parse→match→collect→render are actual joins; parsecmp is an independent diagnostic path. The external group model distinguishes these tags.

## catalog

# Typed models for treebanks-main

## behavior_complement

Stats/meta failure precedes card writes. An item with no hits and no bib/id/doi/year-qualified reference is not audited or tallied. Existing seeds/convert.md are never targeted by build.

Operator: inner exclusion on admitted domain. Evidence: source branch derivation plus model.md#tests; unexercised branches remain hypotheses. Oracle: evaluate the scoped actual helper on admissible fixtures and compare this observation. No universal proof.

## reverse_reconstruction

Found list admits many duplicate references and origins; first duplicate survives audit and tighter match wins per identity. Counts and rendered cards cannot reconstruct dropped raw paragraphs or order of all duplicates.

Operator: compatible antecedent relation, retaining ambiguity. Evidence: source branch derivation plus model.md#tests; unexercised branches remain hypotheses. Oracle: evaluate the scoped actual helper on admissible fixtures and compare this observation. No universal proof.

## identity_join

How scores order Url<Doi<Bibtex<Title, then false fuzzy before true; merge improvement does not replace earlier alts field.

Operator: specialized observation relation under the stated fixed inputs. Evidence: source branch derivation plus model.md#tests; unexercised branches remain hypotheses. Oracle: evaluate the scoped actual helper on admissible fixtures and compare this observation. No universal proof.

## gate_order

read archive errors precede stats errors, then missing metadata, then bibliography/library errors; writes begin only afterward.

Operator: specialized observation relation under the stated fixed inputs. Evidence: source branch derivation plus model.md#tests; unexercised branches remain hypotheses. Oracle: evaluate the scoped actual helper on admissible fixtures and compare this observation. No universal proof.

## excerpt_coordinates

Excerpt counts lowercase prefix characters then indexes original chars; Unicode case expansion is an explicit location fidelity gap.

Operator: specialized observation relation under the stated fixed inputs. Evidence: source branch derivation plus model.md#tests; unexercised branches remain hypotheses. Oracle: evaluate the scoped actual helper on admissible fixtures and compare this observation. No universal proof.

## write_prefix

Card README before papers, languages before INDEX, run dir and missing-ids before audit/review; later errors leave prefix effects.

Operator: specialized observation relation under the stated fixed inputs. Evidence: source branch derivation plus model.md#tests; unexercised branches remain hypotheses. Oracle: evaluate the scoped actual helper on admissible fixtures and compare this observation. No universal proof.

## tests

### Bounded evidence

- `tests::excerpts_and_citation_filter`
- `readme::tests::metadata_and_sections`
- `readme::tests::metadata_variants`
- `readme::tests::setext_and_caps_headings`
- `readme::tests::reference_items`
- `readme::tests::split_unmarked_citations_in_refs`
- `readme::tests::docs_family`
- `refs::tests::normalization_and_ids`
- `refs::tests::title_matching_positive`
- `refs::tests::title_matching_negative_controls`
- `refs::tests::ids_and_bibtex`
- `refs::year_tests::fuzzy_year_article_and_detex`
- `refs::line_tests::title_line_then_authors_line`
- `release::tests::counts_multiword_and_empty_nodes`
- `release::tests::broken_range_is_red`
- `release::tests::stats_gate_shows_red`
- `render::tests::formatting`
- `wave33_tests::readme_collect_render_join`

These names passed twice in debug and twice in release; the complete treebank/comparison slice has 25 target-qualified names and identical sorted outcomes in all four runs. Ten compiled behavior mutations (one per owned source) failed named runtime assertions; exact original/restored SHA-256 equality and positive reruns were checked. Zero assertion-only mutants, zero surviving exploratory mutants, zero production fixes. Fresh raw logs, numeric exits, commands and mutation hashes are external in the wave33 trees receipt set.

#### Scope and gaps

Only named finite assertions cover selected inputs; other branches and invalid-domain behavior remain source-derived. No corpus, network, downloads, annotation model, live teacher, full release build or site execution. The treebanks join is entirely in memory and parsecmp uses a tiny uniquely named temporary directory. Treeagree Registry::load uses the embedded UD registry. Tests are finite CPU observations, not universal semantic proof, title-match quality estimates or corpus accuracy.

## invariant-and-cost

### Invariant and cost

O(archive bytes plus matcher costs across items plus output bytes), storing all cards and audit strings. collect seen keys expected hash-map costs plus linear found search O(hits²); indexing and parsing costs delegated. Required numeric counts must fit usize/u64.

Stats/meta failure precedes card writes. An item with no hits and no bib/id/doi/year-qualified reference is not audited or tallied. Existing seeds/convert.md are never targeted by build.

The complement and specialized catalog constrain preserved coordinates and effect boundaries. Empty/malformed domains are only guaranteed where explicitly admitted. No whole-program cost bound follows from a helper bound.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective source reading identified conditional input alignment, information loss and failure order before tests. Existing production code is retained; added tests use actual helpers. Finite results update this source-bound model, not all parser/corpus semantics.

No executable percentage surrogate has a justified observation metric for these categorical/effectful APIs. Future profiles require metric, domain and real-reference evidence at caps 1/2/5/10%; no higher cap. Syntax is partial lexical Rust projection without macro expansion or semantic resolution.

### 2026-10-31 · tests

Heading `### Bounded evidence — 2026-10-31` stays in tests as `### Bounded evidence`.

Source `tools/treebanks/src/main.rs` at SHA-256 `2b8c709aa686031227a14bc8c4661402851293e65cb77760b071fa627034cd08`. Selected actual-source tests in `treebanks`:

### Undated · document

Subject: source SHA-256 `2b8c709aa686031227a14bc8c4661402851293e65cb77760b071fa627034cd08`; manifest supplies exact source/byte scope. Domain: admitted states in forward model, nonoverflow arithmetic and successful allocations. Observation: one terminating invocation, except explicitly described file trace. Source-derived partial relations; named finite tests constrain selected cases only. Each entry holds arguments and environment fixed except its stated intervention.
