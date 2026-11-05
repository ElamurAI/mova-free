# Model registry and groupant inventory

`mova-modelcheck` checks the model registry (`models/contracts.json`) and the groupant sidecars beside the Rust
sources. This README is the reference for its commands and file contracts; what groupants are and why is in
[docs/groupants.md](../../docs/groupants.md) and [docs/mathematical-modeling.md](../../docs/mathematical-modeling.md#registry-and-checks).

Run from the repository root:

    cargo run -p mova-modelcheck -- groupants . models/contracts.json NEW_JSON

The output must be new. The command writes the report before returning failure
for invalid sidecar declarations. Registry parsing/tree errors and unreadable Git
inventory fail before a report is available. No declared check command is run.

The inventory discovers adjacent SOURCE.rs.groupant/manifest.json for Git's
tracked and nonignored untracked Rust files. It hashes the actual source and
checks the declared subject and SHA-256, core component records, component
path/description file references, duplicate groupant IDs, and typed-family IDs.
File references resolve relative to the sidecar and must canonically remain
files inside the repository. Symlink escapes and missing files fail validation.
An optional registry.node must match the registry owner of that source.

A manifest is open JSON with schema 1, kind groupant, nonempty id/status,
source.path/source.sha256 and components. Core records are ast, approximant,
invariant and descriptant. components.obernants is an array of records with id,
type and status. The required types, in order, are behavior_complement and
reverse_reconstruction. Several entries per type and additional types are
accepted. A not_materialized record explicitly meets the accounting obligation
without claiming a model exists. An inapplicable record needs a reason.
Other statuses need a path or description artifact reference. Type-specific
domain, operator, assumptions, evidence and scope belong in those artifacts
and/or additional fields; their mathematical adequacy is not machine checked.

A component record (including typed family members and extra components) may
optionally select a catalog entry with `"path":"catalog.md","section":"Entry"`.
The section must be a nonempty single-line string without surrounding
whitespace. The path is required; description is not a substitute. Its confined
artifact must be readable UTF-8 and contain exactly one line `## Entry` outside
fenced code. Matching is case-sensitive, at column zero, with exactly one space
after the two hashes and no closing hashes or trailing whitespace added.
LF and CRLF are accepted. Missing or duplicate matching headings fail.

This is a narrow plain ATX H2 catalog contract, not full Markdown parsing:
backtick and tilde fences of at least three markers, indented by zero to three
ASCII spaces, are excluded. A closing fence uses the same marker, at least the
opening length, followed only by spaces/tabs. Backtick opening info cannot
contain backticks. An unclosed fence excludes the remaining lines. Catalogs
must use this flat layout; HTML blocks, container/list parsing, Setext headings,
inline markup interpretation and rendered anchor IDs are outside the contract.
Records without section retain their existing reference behavior, including
binary artifacts. Nested extension fields are not interpreted as selectors.

A `path` or `description` reference may also name a section inline as
`model.md#Section` (the text after the first `#`, same grammar: exactly one
plain `## Section` line outside fences). A record may carry both forms only when
they name the same section. A hash-bound reference (`path` with `sha256`, as in
`rust_approximants` evidence or the correspondence catalogue) to `file#Section`
digests only that section's bytes: from its heading line through the last
nonblank line before the next plain H2, without the final line terminator.
Edits elsewhere in the file leave such a binding fresh. The grammar lives in
`src/sections.rs` and is shared by the validators, the consolidation and
`mova-correspondence`.

A component or obernant whose `status` claims content (anything but
`not_materialized` or `inapplicable`) and that selects a section, by `section`
or inline in `path`/`description`, must find a body beyond the heading: a text
or fenced line, not only blank lines and sub-headings. A heading-only selection
is a diagnostic (`status … claims content, but '## S' … has no body beyond its
heading`); record a missing model as `not_materialized`.

Schema 1 reports registry_sha256, rust_files, sidecars, missing_sidecars,
unregistered_files, valid, diagnostics, files and logical_tree. Every file has
its actual hash, optional registry owner, sidecar location, original manifest,
required_types, diagnostics and status. available_partial means only that an
existing declaration passed these structural checks. Missing sidecars and
unregistered sources are migration backlog, not default errors. Group/root
registry nodes discover models/<id>.groupant/manifest.json, report the original
manifest, diagnostics, sidecar, status and required type coverage in logical_tree.
group_sidecars counts existing group artifacts (including malformed ones);
missing_group_sidecars counts group migration backlog. Group diagnostics
contribute to valid just like file diagnostics. No verification is inferred.

Group manifests use schema 1, kind groupant_group, id equal to the registry node,
a nonempty status, and the same open components and typed family as files.
children is exactly the immediate registry child-ID array, without duplicates;
ordering is immaterial. source_bindings is an array of {path, sha256} records,
one for every transitive registered Rust source, with no duplicates or extras.
Each path is repository-relative and each hash binds the actual current bytes.
Artifact references remain sidecar-relative. An optional registry.node must
match the group. Registry IDs used by discovery contain only ASCII letters,
digits, dot, underscore or hyphen, and cannot equal dot or dot-dot. Duplicate
manifest IDs across files and groups fail validation. Missing groups remain
backlog; declared stale bindings fail even if child file manifests are absent.

Unknown fields are retained in the manifest report. Only defined path and
description fields in component records and the registry path are interpreted
as file references; arbitrary extension strings are not paths. Ordering follows
sorted Git source paths, sorted registry IDs and original manifest array order.
Determinism requires unchanged input files; this is not a filesystem snapshot.

Other commands:

    cargo run -p mova-modelcheck -- inventory . models/contracts.json NEW_JSON [--require-all]
    cargo run -p mova-modelcheck -- changed . models/contracts.json BASE NEW_JSON
    cargo run -p mova-modelcheck -- sidecar-links . NEW_JSON
    cargo run -p mova-modelcheck -- relink-evidence . [--dry-run] NEW_REPORT
    cargo run -p mova-modelcheck -- extract-table LEAN_LOG NEW_TABLE
    cargo run -p mova-modelcheck -- publication-catalog STORE REVIEWS|- NEW_CATALOG
    cargo run -p mova-modelcheck -- publication-check REPO STORE CATALOG REL_MANIFEST [REVIEWS EXPECTED_REVIEWS_SHA] NEW_REPORT
    cargo run -p mova-modelcheck -- consolidate REPO DIR [--dry-run] NEW_REPORT
    cargo run -p mova-modelcheck -- consolidate-check REPO BASE NEW_REPORT
    cargo run -p mova-modelcheck -- undate REPO [--dry-run] NEW_REPORT

`inventory` always writes its report; with `--require-all` it then exits with an
error while any tracked Rust file has no model contract (the all-files gate).
`publication-catalog` and `publication-check` build and check the hash-bound
publication catalog of the external groupant library; see
[publication scope](../../docs/groupants.md#publication-scope).

`consolidate` merges the Markdown files of every `*.groupant/` sidecar below DIR
(`.` for all) into its `model.md`, one `## <stem>` block per former file, in the
order descriptant, model, approximant, invariant, cost, composition, catalog,
models, tests, evidence, prospective, then by name. A former file that some JSON
record selects by `section` stays verbatim under its block heading, so its own H2
sections stay addressable; other files are demoted so their top heading is H3. A
module `prospective.md` beside a `mod.rs.groupant` folds into that sidecar's
`prospective` block. A sidecar stays unmerged, and is reported, when two H2
anchors would coincide or a file ends inside a code fence. Every `.md` reference
to a former file in JSON string literals (edited in place, no key reordering)
and in Markdown links or `.groupant/` paths moves to `model.md#<stem>` (JSON
keeps a verbatim target's `section` field). Fresh hash-bound references are
rebound to the new bytes and the merged file gets a migration line; stale ones
keep their hash and are reported. The command never deletes: the report's
`remove` list is for `git rm`. `--dry-run` writes nothing but the report.
`consolidate-check` is the independent reverse against a Git base: every block
of each changed `model.md` must equal its former file read from BASE, up to
`.md` reference targets and a uniform heading shift, with nothing else but the
title, the migration line, block headings and blank lines.

`undate` applies the History rule of [docs/groupants.md](../../docs/groupants.md#storage-beside-the-source)
to every tracked groupant `model.md`: epoch-dated headings, dated paragraphs,
list items and record lines, and provenance (concrete `$MOVA_DATA/` paths, not
templates with a `<placeholder>`; 40+ hex digests; abbreviated digests ending in `…`) move verbatim into a final `## History`
section grouped as `### <date> · <origin section>`, oldest first. A section's
first dated heading before any undated prose stays in place under its undated
name with a History note quoting the original line; H2 sections that JSON
selects by name never move; fenced code is not inspected. In a section that a
JSON record of that file selects, a dated unit also stays, rewritten line by line
(`strip_line`: dates removed with their frames, dated code spans, digests and
concrete run paths replaced by `(see History)`; record lines such as `Date: …`
dropped); a selected section left heading-only is refilled from the newest
History group of that origin that restates to a nonempty body, and the report
lists refilled and residue sections. Each changed file is
verified (nonblank lines preserved as a multiset up to heading level, nothing
dated or provenance left outside History, added main-text lines only in selected
sections and only as `strip_line` images of History lines, History last) and
must be a fixed point of a second run. Fresh `path`/`sha256` bindings to changed files are then
rebound to fixpoint (sections for `model.md#S`, whole files otherwise, JSON
chains); stale ones are reported, never refreshed. `--dry-run` writes only the
report. Re-running on a tree that already has History is the identity; new
dated material merges into the existing groups.

Focused checks: `cargo test -p mova-modelcheck`; its negative controls include
stale hashes, missing artifacts, duplicate IDs and unsafe references.

The documentation link gate is `cargo test -p mova-modelcheck --test doc_links`:
every relative link and `#fragment` in the Git-tracked Markdown outside `peta/`
must resolve to an existing file and a heading or explicit anchor (GitHub slug
rules, fenced code skipped). External URLs are not fetched.

## Executable Rust approximant attachments

Optional components.rust_approximants retains the legacy contract when absent.
Each attachment requires a classification object with these string fields:

- metric_kind: scalar_relative, normalized_rank, objective_loss or probability_relative.
- method_kind: quantized_numeric, certified_sample, relaxation_certificate, floating_normalization or interval_certificate.
- assurance_kind: analytical, per_input_certificate or finite_observation.

Empirical bound_kind requires finite_observation assurance; analytical bound_kind
requires analytical or per_input_certificate assurance. Missing or unknown
classifications fail validation; extra fields remain open. These labels describe
the declared metric, method and assurance, without proving their adequacy.

Budget tiers are exactly 1, 2, 5 and 10 percent. The declared error must be finite,
nonnegative, within its tier (inclusive) and at most 10 percent. Existing hash, source,
evidence and sidecar confinement checks also apply.

## Source-bound Rust syntax projection

    cargo run -p mova-modelcheck -- syntax REPO SOURCE NEW_JSON

SOURCE is a normalized repository-relative .rs path discovered by Git (tracked
or nonignored untracked), with a canonical file target inside REPO. Absolute,
dot/dot-dot, ignored, missing, escaping and non-Rust source paths fail. Source
bytes must be UTF-8 and parse as a syn 2 file. No artifact is written on parsing
failure. NEW_JSON uses the same exclusive create_new behavior as other commands.

Schema 1 kind rust_syntax_projection records source.path, source.sha256 and
source.bytes for the exact bytes parsed, parser syn-2, machine-readable
limitations, range_unit utf8_bytes_half_open and an ordered nodes array.
Each node has contiguous id, parent (null only for the File root), category,
variant, start/end byte offsets and opaque. Node IDs follow deterministic syn
visitor preorder. Parents are the nearest projected ancestors, since many AST
fields are omitted. The File root spans all bytes; other ranges are nested
half-open UTF-8 boundaries. proc_macro2 scalar columns are converted to byte
offsets, including multibyte identifiers and CRLF. No absolute paths, timestamps,
random IDs or source text are embedded.

Projected categories are item, impl_item, trait_item, foreign_item, statement,
expression, pattern, type, block and macro. Variant identifies the syn enum
variant; statements distinguish Expr from Semi. Macro token bodies are opaque,
as are Verbatim/Unknown variants. Attributes, comments, punctuation and many
fine AST records are omitted. There is no macro expansion, name resolution,
type inference, cfg evaluation or loading of out-of-line modules. This is a
partial AST projection, not lossless syntax or compiler acceptance. Deep
recursion/allocation limits and concurrent filesystem mutation remain outside
the fixed-input guarantee.

A file groupant can opt into artifact integrity validation with:

    "ast": {"status":"partial", "format":"rust_syntax_projection", "path":"syntax.json"}

The artifact path is sidecar-relative and confined as usual. The groupants
command checks schema/kind, exact repository-relative source path and current
source SHA-256, byte length, parser/range unit, explicit limitations, contiguous
IDs, earlier parents and nested UTF-8 ranges. Rebinding only the manifest source
hash does not make a stale syntax artifact valid. This checks structural
integrity, not semantic correspondence of nodes to source; it does not reparse
or certify the projection. Group-level ASTs remain a separate unimplemented
composition. Components without this format retain their prior validation.

## Nested per-file groupants

A file manifest may declare local decomposition with
`"children":[{"id":"file-memory","path":"memo.groupant"}]`.
Each target is a direct child directory of the declaring sidecar, with exactly
one normal path component ending in `.groupant`. Directory aliases, symlinks,
absolute paths, separators, dot/dot-dot, duplicate paths and missing targets
are invalid. The child's `manifest.json` must be a regular file inside that
child directory, not a symlink. Undeclared directories remain open extensions.

Child manifests use schema 1, kind `groupant_fragment`, a nonempty id matching
the declaration, nonempty status, and the same components and required typed
obernants as file manifests. IDs are unique across all files, fragments and
materialized registry groups. `source.path` resolves relative to the child's
own directory to the SAME original Rust file; `source.sha256` binds that whole
file's current bytes. Source syntax projections receive the same integrity
checks as file projections.

Each fragment requires `coverage.description` as nonempty explanatory text.
Optional `coverage.byte_range: {"start":0,"end":100}` uses unsigned half-open
UTF-8 byte offsets in that original source. It must be nonempty, fit the file,
land on character boundaries and fit every ranged ancestor. A fragment below
a ranged ancestor must itself declare a range. Siblings may overlap and may
leave gaps; these checks establish neither disjointness nor full coverage.

A fragment may anchor its range to an existing partial syntax projection:

    "coverage": {
      "description": "selected operations",
      "byte_range": {"start": 100, "end": 240},
      "syntax_nodes": {"path": "../../syntax.json", "node_ids": [12, 37]}
    }

The projection path is relative to the fragment sidecar and uses the same
repository confinement as component references. The artifact must pass the
source-bound rust_syntax_projection validator for the SAME original Rust file
and its current whole-file hash. node_ids must be a nonempty array of distinct
unsigned integers, all present in the projection. byte_range is required and
must equal exactly the minimal enclosing half-open span: minimum selected start
through maximum selected end. ID order is irrelevant to bounds; gaps and
overlapping selected nodes are allowed. Existing UTF-8, nonempty-range and
ancestor-range constraints still apply. Model ancestry need not follow AST
ancestry, since models can cross-cut syntax. No partition or full coverage is
asserted. Manifests without syntax_nodes keep their prior behavior.

These anchors establish partial-syntax artifact and bound integrity only.
Validation does not reparse Rust, prove semantic correspondence or certify the
model. Freshness remains bound to the whole-file hash: editing any source bytes
requires refreshing the projection and reviewing node IDs and model assumptions.

Fragments recursively support the same children interface. Every manifest with
nonempty children requires a valid `components.composition` record with a
`path` or `description` artifact reference describing their interactions.
Here description retains the existing component meaning of a file reference;
it differs from the fragment's inline `coverage.description` text.
Traversal accepts at most 64 fragment levels below a file. A declaration beyond
that limit produces an error instead of further recursion. Invalid children
invalidate their ancestors and the report; absence of an entire file sidecar
retains its previous migration-backlog meaning.

Each file report adds a deterministic flat `subgroupants` array in declaration
preorder. Entries include parent ID, declared ID, depth (first child is 1),
sidecar, original manifest, diagnostics, status and required_types. Missing or
malformed targets appear as invalid entries with a null manifest. The report's
top-level `subgroupants` count counts visited declarations, including invalid
ones; declarations beyond the depth guard are diagnosed but not visited.
No verification is inferred. The inter-file registry stays unchanged: a file
leaf may contain this local decomposition.

## Registry overlays and group directories

A group node may set `groupant_path` to a repository-relative directory ending in `.groupant`. Source nodes keep adjacent sidecars and cannot override this path. Without the field, discovery still uses `models/<id>.groupant`. Existing canonical confinement and manifest identity checks apply to overrides; declaring a missing directory remains explicit backlog.

To combine a valid base with additional nodes:

```sh
cargo run --offline --locked -p mova-modelcheck -- compose-registry BASE_REGISTRY OVERLAY NEW_JSON
```

The overlay uses the Registry schema (`schema`, `root`, `nodes`) and may refer to base node IDs. The command validates the base, requires overlay schema 1, concatenates base nodes followed by overlay nodes, selects the overlay root and validates the complete ownership tree. Inputs are not changed; duplicates are errors, not replacements. Node order is preserved for fixed ordered inputs. Serialization may materialize existing default fields, so value preservation does not imply preservation of input JSON bytes. The output uses exclusive creation and is written only after validation.

A valid combined ownership tree does not prove runtime import compatibility, mathematical claims or native execution. Validate its groupant artifacts and run the relevant actual checks separately. An overlay with references into the base is intentionally not valid on its own. Keep local-only overlays and combined reports within their intended publication boundary.

## External reference library

`mova-modelcheck library REPO STORE RELATIVE_MANIFEST NEW_JSON` checks one algorithm or scholarly-work groupant from the separate external collection. `STORE` may be `-` to require `MOVA_GROUPANTS`. The collection is outside Git; only this public reader and its documentation belong in Mova. No `peta/` module, collection-wide scan, arbitrary command execution, paper download or hash refresh is involved. A valid artifact-integrity report is not a mathematical proof. See [the storage contract](../../docs/groupants.md).
