# world-dictstore: the runtime dictionary store

## forward

`Store::from_manifest(m, base, purpose)` folds the manifest's layers in order. For each layer: the state string maps to
admitted / draft / derived (`proposal` and anything else fail); the licence must be one of the public licences
(elamur-cosmic, cc0-1.0, public-domain) or the internal one (cc-by-sa-4.0), and an internal licence requires scope
internal; a shipped use (`Purpose::Release`) refuses every internal-scope layer. A built-in path (`builtin:pupils`,
`builtin:commands`) takes its compiled text, which must match its fixed state and public scope and pass `shippable`;
a file layer must name its 64-hex SHA-256, equal to the file's bytes, be UTF-8, name a review when admitted, and pass
`shippable` unless internal. Rows (`#` lines skipped, at least three columns) are appended to `entries[word]` with the
layer index. `Layer::knowledge` = admitted or `allow_unreviewed`. `knowledge(word)` filters `get(word)` by it;
`first_meaning` takes the first knowledge entry whose `first_sentence` is non-empty. `store()` loads `MOVA_DICT` once
(internal only with `MOVA_DICT_USE=internal`), exiting with status 2 on any refusal, else the built-in store.
A `module:NAME[@VERSION]/LAYER` path takes the module branch (`module_layer`): scope must be `module`, the module is
found among the roots (`mova_modules::find`, highest version unless named), the spec's licence must equal the
module's licence or class, an optional `sha256` must equal the module.json hash, a research-only module refuses a
release purpose, `admitted` needs the module to declare it, and the layer file is read through `read_verified`; the
layer carries `ModuleSource` (name, version, licence, class, attribution). `process_manifest` is `MOVA_DICT`'s manifest
or the built-ins, plus `module_specs(MOVA_MODULES)` (`all` or a list) when set. `derived_licence` lists the loaded
share-alike / non-commercial modules; `derived_header` turns it into a `# licence:` line; `guard_derived_write` refuses
such output inside a Git work tree or under a built-in file name. The fingerprint appends `module NAME@VERSION` for
module layers only, so stores without modules keep their fingerprints.
`admit(proposal, id, review)` requires the review's SHA-256 of the proposal bytes, a reviewer and date, disjoint
accept/reject sets that together equal the proposal's words, and writes one row per accepted word.

## invariant

- Determinism: entries are a `BTreeMap` keyed by lowercase word; each word's entries keep manifest layer order; the
  fingerprint hashes (id, state, licence, sha256) per layer in order.
- No CC BY-SA text loads for a shipped use from a repository-side layer, and no built-in text declares CC BY-SA or a
  non-commercial licence; built-in layers never come from a module.
- A module layer's bytes equal its module's SHA-256; a research-only module never loads for a release purpose.
- With a share-alike or non-commercial module loaded, retained outputs carry the mark and are not written into a Git
  work tree or under a built-in file name; without one, nothing changes (fingerprints included).
- A file layer's content equals the bytes its manifest pins.
- No proposal is ever a layer; an admitted file layer names its review.
- Only knowledge layers reach `knowledge` and `first_meaning`; `get` shows every loaded entry.

## cost

L layers, B total bytes, R rows: SHA-256 O(B), row split O(B), map insertion O(R log W) for W words. The built-in
pupils' layer is about 19.7 MB and 19 725 rows; built once per process on first use (`OnceLock`). Lookups O(log W).
`admit` O(P log P) for P proposal lines.

## composition

Callers: `knowledge::PragStudent::dict_kinds` (opt-in `PRAG_VN=student-dict`, question side only),
`knowledge::retention` (records `fingerprint()` in a saved state and refuses a state learned with another
dictionary), the `world dict` command. The store is read-only after load; the student's genus cache is a separate
process-wide map keyed by word under the fixed store.

## behavior-complement

The store does not: fetch or download layers; write or modify a layer except `admit`'s new exclusive file; merge or
rank senses across layers; check that a licence label is true beyond the CC BY-SA header test; decide whether a draft
is correct; authenticate a review (the reviewer field is a name, not a signature); expire or version entries within a
layer; read Nautilus proposals as knowledge; ship anything from `$MOVA_DATA`.

## reverse-reconstruction

From a fingerprint, the set of (id, state, licence, sha256) tuples is recoverable only by search over candidate
manifests (SHA-256 preimage resistance); from an entry, its layer index recovers the layer's provenance exactly. From an
admitted layer, the proposal id and its SHA-256 are in its header, and the accepted word set equals the rows; the
rejected words and their senses are not recoverable from the layer (only from the review record).

## module-provenance

Relation layer ↦ ModuleSource: every entry's layer index gives the module name, version, licence, class and
attribution exactly; `derived_licence` is the ordered set of tainting (cc-by-sa, nc-research) module sources.

## licence-class

Constraint relation: (licence, scope, purpose) ↦ load or refuse. public licence ∧ public scope: loads for both
purposes. cc-by-sa-4.0 ∧ internal: loads only for Purpose::Internal. cc-by-sa-4.0 ∧ public: always refused. Unknown
licence: refused. Module layers: (class, purpose) ↦ nc-research ∧ release refused, otherwise loaded with scope module. A public-labelled file whose header declares CC BY-SA: refused.

## knowledge-gate

Decision relation: state × allow_unreviewed ↦ knowledge. admitted ↦ true; draft, derived ↦ allow_unreviewed;
proposal ↦ no layer at all.

## checks

`cargo test --release -p world --lib dictstore` (eight tests) and
`cargo test --release -p world --lib knowledge::retention::tests::word_meanings_are_retained_and_bound_to_the_dictionary`.
Negative control: with the CC BY-SA test in `shippable` disabled, `cc_by_sa_never_ships` fails; restored,
it passes.

## epoch-memory

Third epoch: extra columns. A layer's columns after the sixth are kept per entry as one text
(`Entry::extra`; `col`/`terms` by `EXTRA_COLUMNS` name), and the rows are moved into the store with the text dropped
first, so the 340 MB Wiktionary layer loads at about 1.5 GB peak RSS instead of 2.8 GB (measured with `world dict
stats(see History)rows` keeping no extra text, `extra_columns_are_kept` fails; restored, all
eight pass. The school layers have six columns, so every six-column consumer reads the same entries.

Second epoch: add-on module layers (docs/modules.md). Red controls: with `guard_derived_write` disabled,
with the NC check of `shippable` removed, and with the research-only purpose check removed, the new tests
`derived_output_of_a_share_alike_module_never_ships` / `module_layers_keep_provenance_and_licence_gates` fail;
restored, all seven pass.

First epoch. The Nautilus layers' meanings are byte-for-byte copies of the pupils' dictionary (2 759 of 2 759
Wikipedia-layer and 952 of 952 commands-layer meanings), so loading them changes nothing a meaning-reading consumer
sees; their new material is examples, counts and, later, admitted senses. The genus reading measured on the school
gates did not pass the controls (docs/training.md section 32).

## History

### 2026-11-04 · checks

`cargo test --release -p world --lib dictstore` (eight tests) and
`cargo test --release -p world --lib knowledge::retention::tests::word_meanings_are_retained_and_bound_to_the_dictionary`.
Negative control (2026-11-04): with the CC BY-SA test in `shippable` disabled, `cc_by_sa_never_ships` fails; restored,
it passes.

### 2026-11-04 · epoch-memory

2026-11-04, third epoch: extra columns. A layer's columns after the sixth are kept per entry as one text
(`Entry::extra`; `col`/`terms` by `EXTRA_COLUMNS` name), and the rows are moved into the store with the text dropped
first, so the 340 MB Wiktionary layer loads at about 1.5 GB peak RSS instead of 2.8 GB (measured with `world dict
stats`, 2026-11-04). Red control: with `rows` keeping no extra text, `extra_columns_are_kept` fails; restored, all
eight pass. The school layers have six columns, so every six-column consumer reads the same entries.

2026-11-04, second epoch: add-on module layers (docs/modules.md). Red controls: with `guard_derived_write` disabled,
with the NC check of `shippable` removed, and with the research-only purpose check removed, the new tests
`derived_output_of_a_share_alike_module_never_ships` / `module_layers_keep_provenance_and_licence_gates` fail;
restored, all seven pass.

2026-11-04, first epoch. The Nautilus layers' meanings are byte-for-byte copies of the pupils' dictionary (2 759 of 2 759
Wikipedia-layer and 952 of 952 commands-layer meanings), so loading them changes nothing a meaning-reading consumer
sees; their new material is examples, counts and, later, admitted senses. The genus reading measured on the school
gates did not pass the controls (docs/training.md section 32).

### 2026-11-05 · school data outside the repository

builtin:commands reads commands.tsv from knowledge::dictionaries_dir() at first use (OnceLock) instead of include_str!; empty without the school data. The built-in store test asserts the commands entries only when the text is non-empty; the pupils' layer stays compiled in. The fingerprint is still deterministic for a given installed file.
