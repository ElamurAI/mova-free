# world-meanings

## model

# Lexical operation, numeric extraction and record overrides

Scope: partial source-bound relations over the observations named below. Sufficient memory and terminating dependencies assumed. This is not a universal proof or an executable percentage-error approximant.

## forward

entry_word lowercases, prefers exact record, then ordered suffix candidates of length>2. op_of selects frozen map when MOVA_LIVEOPS absent and map nonempty; exact '0' blocks fallback then becomes None. Live mode may retry euphemism base when its own operation is absent. operation_record counts opening lesson cues plus STRONG pupil cues excluding entry word; unique positive plurality wins. facts_in appends qualified numbers/stemmed units. improved_from keeps last lowercase-key row.

## invariant

Tied/all-zero cue counts produce None. Fixed resources/configuration yield deterministic projection. Cache key is original spelling, distinct from lowercase entry identity. TSV dates empty; extra fields ignored; last duplicate wins.

## cost

k lexical tokens: O(k) fixed-table comparisons plus four-element sort. Dictionary suffix lookup O(log D) comparisons each. Fact normalization/lookup add upstream cost; cache retrieval clones output. TSV O(B+r log D) comparison-weighted with stored accepted strings.

## composition

opchoice uses op_of for operation cues; mentions calls facts_of only when prior facts empty and MOVA_DICTFACTS present. op_in delegates to pure operation_record after lookup. facts depend on numbers::normalize and mentions::stem. No freeze command or dictionary write executed.

## behavior-complement

Type: behavior_complement. Subject: world-meanings. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Universe: supplied records/token streams/TSV and scoped outputs with fixed stemmer. Inner absence excludes an operation on tied counts, facts n<=1 or unapproved predecessor, retained earlier duplicate rows. Linguistic mistakes remain possible.

## reverse-reconstruction

Type: reverse_reconstruction. Subject: world-meanings. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. An operation identifies compatible cue vectors, not unique definitions. None includes silence and ties. Facts erase context; TSV erases comments/overrides/extra fields. Tests enumerate all 256 count vectors in 0..3 and explicit projection families.

## cue-plurality

Type: decision_relation. Subject: world-meanings. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Counts c in N^4 map to op_i iff c_i>0 and c_i>all other counts. Independent maximum/winner oracle covers 0..3 per operation; no lexical-quality guarantee.

## opening-loss

Type: information_loss. Subject: world-meanings. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Keep prefix before ' 2) ', then before semicolon/period; lowercase and alphanumeric-split, excluding entry token. Competing cues after these delimiters cannot affect observed operation.

## strong-asymmetry

Type: counterfactual_relation. Subject: world-meanings. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Pupil field admits only STRONG cues; lesson field admits all cues. Moving weak cues across fields can change counts. Native fixtures reject weak pupil cues and normalize uppercase strong cues.

## numeric-boundary

Type: constraint_relation. Subject: world-meanings. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. facts_in accepts i64 at i>=from only when i>0, n>1 and predecessor in is/has/have/of/(/equals/makes. Excluded or numeric next token yields empty unit. Native oracle covers 63 prefix-number pairs and 11 unit blockers.

## override-fiber

Type: information_loss. Subject: world-meanings. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Lowercase-key TSV insertion replaces all six fields. Extra fields/comments/malformed rows vanish; dates inserted empty. Native test checks all replacement fields and three extra-field equivalents.

## frozen-live-boundary

Type: configuration_relation. Subject: world-meanings. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Any presence of MOVA_LIVEOPS enables live routing. Otherwise exact frozen '0' prevents suffix fallback. OnceLock resources/caches are per-process. Routing is source-inspected; tests do not change environment or acquire dictionaries.

## checks

Native synthetic tests: cargo test -p world --lib meanings::wave9_models -- --test-threads=1. Test bodies define finite domains and fixtures. File-level model.md#evidence records repeated debug/release checks and a compiled negative control. No parser acquisition, teacher request, dictionary write or external assessment executed. Orchestration boundaries are source inspection only.

## epoch-memory

First scoped model. Retain ambiguous fibers, source-order rules and distinction between lexical evidence and truth. Tests observe actual helpers; they do not substitute surrogates into production. Macro token bodies in partial syntax remain opaque. Future updates must rebind exact source bytes and retain these limits. No performance or task-quality improvement inferred.

## evidence

### Native evidence

Operation oracle covers all 256 cue count vectors in 0..3; fact gate covers 63 prefix-number pairs, 11 unit blockers and offset identity; TSV checks duplicate precedence and three extra-field fibers. Frozen/live map routing and upstream entity anchoring are source-inspected; no dictionary file is read or written by these tests.

Pure helper extraction: operation_record isolates existing op_in cue projection without changing its logic. read.rs production behavior unchanged. No runtime bug repair or measured performance gain claimed.

## History

### 2026-10-30 · behavior-complement

Type: behavior_complement. Subject: world-meanings, 2026-10-30. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Universe: supplied records/token streams/TSV and scoped outputs with fixed stemmer. Inner absence excludes an operation on tied counts, facts n<=1 or unapproved predecessor, retained earlier duplicate rows. Linguistic mistakes remain possible.

### 2026-10-30 · reverse-reconstruction

Type: reverse_reconstruction. Subject: world-meanings, 2026-10-30. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. An operation identifies compatible cue vectors, not unique definitions. None includes silence and ties. Facts erase context; TSV erases comments/overrides/extra fields. Tests enumerate all 256 count vectors in 0..3 and explicit projection families.

### 2026-10-30 · cue-plurality

Type: decision_relation. Subject: world-meanings, 2026-10-30. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Counts c in N^4 map to op_i iff c_i>0 and c_i>all other counts. Independent maximum/winner oracle covers 0..3 per operation; no lexical-quality guarantee.

### 2026-10-30 · opening-loss

Type: information_loss. Subject: world-meanings, 2026-10-30. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Keep prefix before ' 2) ', then before semicolon/period; lowercase and alphanumeric-split, excluding entry token. Competing cues after these delimiters cannot affect observed operation.

### 2026-10-30 · strong-asymmetry

Type: counterfactual_relation. Subject: world-meanings, 2026-10-30. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Pupil field admits only STRONG cues; lesson field admits all cues. Moving weak cues across fields can change counts. Native fixtures reject weak pupil cues and normalize uppercase strong cues.

### 2026-10-30 · numeric-boundary

Type: constraint_relation. Subject: world-meanings, 2026-10-30. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. facts_in accepts i64 at i>=from only when i>0, n>1 and predecessor in is/has/have/of/(/equals/makes. Excluded or numeric next token yields empty unit. Native oracle covers 63 prefix-number pairs and 11 unit blockers.

### 2026-10-30 · override-fiber

Type: information_loss. Subject: world-meanings, 2026-10-30. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Lowercase-key TSV insertion replaces all six fields. Extra fields/comments/malformed rows vanish; dates inserted empty. Native test checks all replacement fields and three extra-field equivalents.

### 2026-10-30 · frozen-live-boundary

Type: configuration_relation. Subject: world-meanings, 2026-10-30. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Any presence of MOVA_LIVEOPS enables live routing. Otherwise exact frozen '0' prevents suffix fallback. OnceLock resources/caches are per-process. Routing is source-inspected; tests do not change environment or acquire dictionaries.

### 2026-10-30 · epoch-memory

2026-10-30: first scoped model. Retain ambiguous fibers, source-order rules and distinction between lexical evidence and truth. Tests observe actual helpers; they do not substitute surrogates into production. Macro token bodies in partial syntax remain opaque. Future updates must rebind exact source bytes and retain these limits. No performance or task-quality improvement inferred.

### 2026-10-30 · evidence

Heading `### Native evidence, 2026-10-30` stays in evidence as `### Native evidence`.

Final source SHA-256: cf20349ad626caf3b71c6ed93db488421a43fcc8c06e4abebf4a7ee5d8932c84.

Five tests in meanings::wave9_models; ten tests combined under cargo test -p world --lib wave9_models -- --test-threads=1. Twice debug and twice release: ten passed each. Restored debug pass: ten passed. All five normalized test-name/result lists identical (SHA-256 25f5e8fd801f3ec612171457cc6fdc27d27133def2a77c9c562aeee13ec5f8da). Existing compiler warnings retained; no full suite claim.

Compiled production control: changed strict winner > to >=; a positive tied maximum rejected. Named assertion failure: meanings::wave9_models::unique_plurality_oracle_and_fibers. This was a runtime assertion failure after compilation, not compiler rejection. Source restored byte-for-byte and all ten positives passed afterward. Mutant source SHA-256 c952abfde7e9f9434689d1fe519c52315eac9241f29984e712671ee332c265ea; raw mutant log SHA-256 c34a39a20252a3b149bbe0b9a0e5bce6f997ac0602eb25ee8f3692c22bbf66d1.

Syntax generated twice identically; SHA-256 a9d92c7b40474de6fd794ebb64b90a5fb3844bacaf2a8614de3048913a8c0628. Isolated copies of this exact source/sidecar pass groupants with no diagnostics. Full registry/ancestor integration is performed by the coordinating agent; a minimal registry against the full checkout deliberately leaves unrelated ownership diagnostics and is not counted as a passing full inventory.

Raw evidence lives outside repository under $MOVA_DATA/runs/octopus-wave9-reading-2026-10-30/.

- debug1.log: SHA-256 88068111cf5c531d673ba210a7521a7ebc4ff293962d0db82eb07204a60446ec

- debug2.log: SHA-256 16bfc7d8b0e1387047b6b752b15c726997e79883ae321a9e75ca5738810d74ed

- release1.log: SHA-256 f65b35a2385c348394e235f21c3e5f78485e4edbf1a395a3e106462b5ea476d2

- release2.log: SHA-256 c0bfe116e049f834ee7cb89286d9e8fa8e1d9c38f183198dc1ccfbd2958e358e

- restored.log: SHA-256 c154edb51986d3281a2282ed9989b42c9884f513d2015fec2cbd49c692c3d46d

### 2026-11-05 · school data outside the repository

The register of euphemisms is read at run time from courses/euphemisms/register.tsv of the school data instead of include_str!; without the school data the register is empty, so no record is dropped for it (a behaviour change only where the school is not installed).
