# Groupants

This page describes groupants: the model nodes that sit beside Mova's Rust
code. It covers what a groupant holds, where it is stored, how groupants form a
tree with parent interaction models, how the registry and coverage are checked,
the external reference library in `$MOVA_GROUPANTS` with its publication rules,
and what the evidence does and does not establish. The mathematics of the
components (vidtvorants, typed obernants, the combiner, executable
approximants) is in [mathematical modeling](mathematical-modeling.md).
Typed relations between algorithm groupants and Mova sources are in
[correspondences](correspondences.md).

## What a groupant is

A groupant is the combinator node associated with a particular piece of code:

    G_v = {AST_v, A_v, {B_v,t,i}, J_v, D_v, tests_v, ...}

| component | role | manifest key |
|---|---|---|
| AST | structured representation of the covered Rust code, or an explicit gap | `ast` |
| A | vidtvorant: forward approximant/emulator in mathematical form | `approximant` |
| Rust approximants | executable vidtvorant surrogates with a 1%, 2%, 5% or 10% error cap, an explicit metric and a measured cost | `rust_approximants` |
| B family | typed obernants: behavior complement, reverse reconstruction and specialized variants | `obernants` |
| J | invariant or family of invariant constraints | `invariant` |
| D | descriptant: readable Markdown description and epoch memory | `descriptant` |

The word groupant names the whole node; the descriptant is one of its
components. The named components are core roles, not a closed set: a groupant
may also hold tests, cost models, diagrams, counterexamples, notes and any other
useful material. Missing components stay explicit gaps.

Every groupant must account for two typed families, in this order:

1. `behavior_complement`: an approximant of what the code does not do;
2. `reverse_reconstruction`: compatible inputs or states recovered from an
   output.

Several models of one type and additional specialized types are welcome. A
`not_materialized` record meets the accounting obligation without claiming
that a model exists; an `inapplicable` record needs a reason. Definitions and
laws are in [mathematical modeling](mathematical-modeling.md).

## Storage beside the source

For a source file `main.rs`, the groupant lives in the adjacent
`main.rs.groupant/` directory. The raw implementation stays in `main.rs`.

    main.rs
    main.rs.groupant/
      manifest.json
      model.md          # one Markdown file of blocks
      syntax.json
      ... other non-Markdown artifacts ...

The manifest is open JSON: schema 1, kind `groupant`, nonempty id and status,
`source.path` and `source.sha256`, and `components`. Extra fields and files are
permitted; the example layout is a convention, not a closed schema. A groupant
keeps one Markdown file, `model.md`, with blocks: `## descriptant`, `## model`,
`## approximant`, `## invariant`, `## cost`, `## composition`, `## catalog`,
`## tests`, `## evidence`, `## prospective` and others as needed. Child groupants stay nested directories, each with its own
`model.md`. Curated
contents are versioned in the repository; large raw run outputs stay in
`$MOVA_DATA` and are referenced. Keep source identity, revision and artifact
statuses, so the existence of a folder is never mistaken for completed
mathematical components. A concrete example is the
[assessment descriptant](../tools/enrich/src/assessment.rs.groupant/model.md#descriptant).

The main blocks state what is now and carry no dates. Dated material — epoch
headings such as `Review epoch …`, dated paragraphs, list items and record lines,
and provenance such as source digests, receipts and `$MOVA_DATA` run roots —
lives in one final `## History` section, verbatim, in groups
`### <date> · <origin section>`. Groups run oldest first: History is
append-only, so a new epoch adds at the end and a re-run leaves older groups
untouched. A section's lead block keeps its place under an undated name
(`Design epoch …` → `Design`) and History keeps a note quoting the original
heading. Paper years, versions, toolchain build dates and data snapshot dates
are not epoch dates; fenced code is not inspected.

A section that a component record selects keeps its current substance. Its
dated sentences stay with the date removed mechanically (`(…)`, `as of …`,
`Epoch …:` frames; a digest or run root becomes `(see History)`), and History
keeps them verbatim; only record lines such as `Date: …` leave entirely. A
selected section left heading-only takes its current body from the newest
History group of that section, restated the same way. An unselected core block
whose only text was dated stays as an empty heading.
`mova-modelcheck undate REPO [--dry-run] NEW_REPORT` applies the rule to every
groupant `model.md`, verifies each file (lines preserved as a multiset, nothing
dated left outside History, rewritten lines only in selected sections and only
as images of History lines, idempotent) and rebinds fresh hash-bound
references; see the [modelcheck README](../tools/modelcheck/README.md).

A component record selects a block with `"path":"model.md#tests"` or
`"path":"model.md","section":"Forward"`. The file must contain exactly one
plain `## tests` (or `## Forward`) heading outside fenced blocks; missing or
duplicate headings fail. A record whose status claims content (anything but
`not_materialized` or `inapplicable`) must select a section with a body beyond
its heading; a heading-only section fails, and a missing model is recorded as
`not_materialized` instead. A hash-bound record (`path` with `sha256`) to
`model.md#Section` binds only that section's bytes, so editing one block does
not stale the bindings of the others. This is a narrow grammar, not Markdown
rendering. Details are in the
[modelcheck README](../tools/modelcheck/README.md).

`mova-modelcheck consolidate REPO DIR` merges a sidecar's Markdown files into
`model.md` blocks and moves every reference; `consolidate-check` verifies
against Git that each block is its former file. A sidecar whose block headings
would collide keeps several files; the validators accept both forms.

`mova-modelcheck sidecar-links REPO NEW_JSON` fails when Markdown inside a
sidecar names a sibling or parent `.md`/`.json` file that does not exist
(read from the sidecar, its enclosing sidecars and their source directory), or
a `file.md#anchor` whose heading is absent. A bare name counts only when such a
file exists or once existed in that sidecar chain (Git history), so a run
directory's `native-commands.json` or `prospective.md` is not a sidecar
reference; `syntax.json` may be absent, and a name followed by its SHA-256 is a
provenance record of the former file, kept as written.
`mova-modelcheck relink-evidence REPO [--dry-run] NEW_REPORT` rewrites such
former-file names to the `model.md#section` holding their block now
(`evidence.md` → `model.md#evidence`, `../tests.md` → `../model.md#tests`, a
module `prospective.md` → `mod.rs.groupant/model.md#prospective`, a dated block
moved by `undate` → `model.md#history`), rebinds fresh hash-bound references
and lists what it could not resolve.

## The tree of groupants

The root groupant describes the whole system. Its children describe
subsystems; their children describe smaller interactions and individual code
units. Each level keeps the same extensible form at its own abstraction.

    Mova groupant
      self-study groupant
        assessment groupant
        retained-learning groupant
        review/transport groupants
      knowledge-memory groupants
      mathematical-modeling groupants

The AST represents covered code; child links represent the modeling
decomposition. Group and root manifests live in `models/<registry-id>.groupant/`.
They bind their exact immediate children plus every transitive registered Rust
source and its hash, so a changed descendant invalidates the binding even when
that leaf has no sidecar yet.

A parent's approximant, obernants and invariants combine child models with
interaction constraints: shared state, execution order, conditions, effects and
failures. For child relations R1 and R2, a sequential parent model is

    Rparent(x,z) iff there exists y: R1(x,y) and R2(y,z).

Real composition also needs coupling constraints and shared-state
assumptions. The parent's reverse relation preserves every compatible
intermediate state. Its behavior complement is relative to the parent's own
universe and is not in general the union of child complements. Overlapping
children must not count the same work twice in cost estimates. Correct leaves
do not prove their wiring.

## Trees inside a Rust file

A file groupant may own its own hierarchy. The whole local tree stays inside
the adjacent sidecar; the Rust file is not split.

    learning.rs
    learning.rs.groupant/
      manifest.json
      model.md
      execution.groupant/
        manifest.json
        model.md
        memo-policy.groupant/
          manifest.json
          model.md
      accounting.groupant/
        manifest.json
        model.md

A file or fragment manifest declares its immediate children:

    "children": [
      {"id": "learning-execution", "path": "execution.groupant"}
    ]

Rules for fragments:

- A child manifest uses schema 1 and kind `groupant_fragment`. It binds the same
  whole Rust source by relative path and SHA-256, with a nonempty
  `coverage.description`.
- Optional `coverage.byte_range` holds UTF-8 byte offsets of a nonempty
  half-open interval `[start,end)`. Offsets are snapshot anchors, not proof
  that the model matches those bytes. A source edit invalidates the binding;
  recheck ranges and assumptions before refreshing it.
- Child folders are immediate subdirectories named `<name>.groupant`. IDs are
  unique across the inventory. A nonempty child list requires a
  `components.composition` record that references an artifact.
- Descendants of a range-scoped fragment declare ranges inside it. Siblings may
  overlap and need not cover the parent, when that is stated explicitly.
- Validation recurses up to 64 fragment levels; deeper declarations are
  invalid.

The inter-file registry keeps one owner per Rust file, so a file node can stay
a registry leaf while its sidecar owns a local tree. Fragment nodes do not
increase the number of modeled files.

## Source-bound syntax

    mova-modelcheck syntax REPO SOURCE NEW_JSON

parses one Git-visible Rust file into a deterministic partial AST projection.
Item, statement, expression, pattern and type nodes carry parent IDs and UTF-8
byte ranges. Macro bodies are opaque; there is no name resolution, type
inference, cfg evaluation or external-module loading. It is not a lossless
syntax representation.

A file component sets `ast.status=partial`,
`ast.format=rust_syntax_projection` and `ast.path=syntax.json`. Validation
checks the current source path, hash and bytes and basic node integrity;
rebinding only the manifest hash cannot make stale syntax current. A fragment
may select projection nodes:

    "coverage": {
      "description": "Memo declaration and implementation",
      "byte_range": {"start": 886, "end": 1702},
      "syntax_nodes": {"path": "../../syntax.json", "node_ids": [63, 67]}
    }

Node IDs belong to that source snapshot. The range must equal the smallest
enclosing interval of the selected nodes, `[min(start),max(end))`, and may
contain gaps. The validator does not reparse the source to certify the
artifact. Fragments without a projection keep range-only scopes.

Projections are regenerable, and the public tree does not carry them: an absent `syntax.json` is rebuilt in memory
from the current source by `groupants` and validated like a stored one (counted as `syntax_projections_regenerated`
in the report), so a stale source hash still fails. To write one back to disk:

    mova-modelcheck syntax . path/to/file.rs /tmp/syntax.json
    mv /tmp/syntax.json path/to/file.rs.groupant/syntax.json

for each file whose manifest names `ast.path=syntax.json`.

## Registry and checks

`models/contracts.json` is the public registry of model nodes: one owner per
Rust file, groups and the root. The models it registers are partial. Where a
local overlay of further nodes exists, compose it first; the public registry
stays usable on its own.

| command | what it checks |
|---|---|
| `mova-modelcheck groupants REPO REGISTRY NEW_JSON` | source hashes, core component records and references, duplicate IDs, required typed families, nested fragments, group sidecars; missing sidecars reported separately |
| `mova-modelcheck inventory REPO REGISTRY NEW_JSON [--require-all]` | registration; with `--require-all` fails while any tracked Rust file has no owner |
| `mova-modelcheck changed REPO REGISTRY BASE NEW_JSON` | the changed/new sources since `BASE` and their bindings |
| `mova-modelcheck compose-registry BASE OVERLAY NEW_JSON` | forms a combined registry from the public one and a local overlay |
| `mova-modelcheck syntax REPO SOURCE NEW_JSON` | writes a source-bound partial syntax projection |

Outputs must be new files. The validator never runs declared check commands.
Full usage and the negative controls are in the
[modelcheck README](../tools/modelcheck/README.md).

`inventory` enumerates tracked and non-ignored untracked Rust files, hashes
their contents, and emits the logical model tree and the directory coverage
tree; build targets and ignored third-party or generated files are not owned
source, while tracked generated Rust still needs a data or generator contract.
File-local fragment trees are inventoried separately and do not count as file
coverage. `mova-modelcheck extract-table LEAN_LOG NEW_TABLE` takes a table
from a successful Lean log and rejects partial, duplicate, out-of-domain or
failed-proof output. No gate executes registry check strings or calls a
teacher, and the report never infers verification from a model's presence or
a test name. Keep model declarations, bounded model checks, code/model
correspondence, formal proofs and runtime measurements distinct, with source
and registry fingerprints beside the evidence.

When you change or add a Rust file:

1. Register it in `models/contracts.json` and create or update its
   `<file>.groupant/` (and syntax projection, if the file has one).
2. Update parent composition models whose assumptions the change touches.
3. Run `changed` from the integration base, then the relevant focused tests.

## Coverage

Coverage means "registered owned Rust sources with a valid sidecar". It is
checked, not recorded by hand:

1. Compose the registry if a local overlay exists.
2. Run `inventory --require-all`, `groupants` and `changed` against it.
3. Require zero missing file and group sidecars as well as valid declarations.
   `inventory` alone only checks registration.

`models/coverage-backlog.json` is a dated snapshot of aggregate counts and
registry digests; new or changed sources need a fresh inventory and fresh
bindings. An empty registration queue does not close documented semantic gaps.
Static or compile-only evidence and unexecuted external paths stay visible in
the local contracts.

## Epoch memory

The descriptant carries the useful Markdown from earlier modeling epochs:
model descriptions, hypotheses, surprises, measurements, counterexamples,
rejected attempts and open questions, each with its source/model revision and
evidence status. For a diagnosis, record the question, domain and observation,
the forward prediction, the specialized obernant, the counterexample, the
correction and the evidence status; keep failed intermediate repairs and
unsupported domains. A parent descriptant also records observation loss and
interface assumptions.

The combiner can use a groupant at epoch k, its children and new evidence to
propose the next groupant and a new descriptant record; earlier epochs stay
available. This is the intended feedback structure; an autonomous combiner
loop is not implemented. See [mathematical modeling](mathematical-modeling.md).

## Lessons from model reviews

Forward models, typed obernants and independent native witnesses together have
exposed real runtime defects and overstated models. The details stay in the
adjacent sidecars; the diagnosis method and the rules that repeatedly found
defects are in
[mathematical modeling](mathematical-modeling.md#diagnosis-with-vidtvorants-and-obernants).

## External reference library

General algorithm explanations and scholarly-work groupants live outside Git
in `$MOVA_GROUPANTS`. The directory holds only groupant materials:
explanations, mathematical models, bibliographic records, code/test snapshots,
evidence and indexes. Do not initialize a Git repository there or copy the
collection into this repository; the ignored root `groupants/` path prevents an
accidental local copy. Public Mova owns the interaction code and has no
dependency on private code.

```text
store.json
index.json                         # algorithm description inventory
algorithms/<module>/
  manifest.json
  description.md
  bibliography.json
  source.rs.txt
  tests.rs.txt
  evidence.json
  worked-check.rs.txt
  worked-examples.txt
  models/...
papers/index.json
papers/<work-id>/
  manifest.json
  bibliography.json
  description.md
  use-model.md
<store-prefix>/<source-kind>/<id-hash>/<version-hash>/   # generated surveys
```

**Algorithm editions** keep the canonical Mova node ID and bind both the Rust
source and the adjacent model manifest by SHA-256. They add original prose, a
verified bibliography, source/test snapshots and worked examples to a snapshot
of the canonical components. They are explanatory editions, not additional
source owners: the adjacent development groupant remains authoritative for code
and model updates, and its descriptant stays separate. Copied partial syntax,
finite checks and arguments keep their original assurance labels. The index
distinguishes reviewed description drafts from pending ones; a missing
description stays missing, without placeholder prose.

**Paper nodes** start from works actually used by the code. Their models
describe Mova's concrete use of the work; full-paper ASTs and formalizations
stay explicitly missing, and each node records whether full text, selected open
sections or only bibliographic metadata were consulted.

**Generated surveys**, written by the [Whales](animals.md), use immutable
paths under the store prefix their specification names and never overwrite canonical bundles, indexes or reviewed
articles. A survey declares its profile, bibliography, hypotheses, typed
obernants and evidence gaps; the current word profile `words_4000_5000_v1`
admits 4,000–5,000-word descriptions. A generated survey is not a completed
formal groupant, and a generated converse must be checked before it is relied
on: a finite counterexample against the real Rust module can refute it.

Verbatim model Markdown uses a declared `markdown_links` map. Repository
locators are provenance, not website routes; renderers must not resolve them
against the store or guess a domain.

### Reading one manifest

```sh
cargo build --offline --locked --release -p mova-modelcheck
target/release/mova-modelcheck library . "$MOVA_GROUPANTS" \
  algorithms/longest_common_subsequence/manifest.json "$RUN/lcs-library.json"
target/release/mova-modelcheck library . - \
  papers/hirschberg-1975/manifest.json "$RUN/hirschberg-paper.json"
```

`-` takes the directory from `MOVA_GROUPANTS`. `RUN` is an existing external
run directory and each output must be new. The reader validates one bounded
manifest and its declared assets: path confinement, artifact hashes, declared
repository source bindings, survey description length and the explicit
unverified/missing-proof status. It never executes code, follows a declared
command, downloads a paper or refreshes a stale binding. It does not validate
indexes, Markdown maps, cross-bundle edges, licences or mathematical claims.
A dependency list needs a receipt for each listed manifest against the same
checkout. A worked harness is compiled in its own external directory after all
manifests in its `requires_bundles` list validate.

### Writing and maintenance

- Write original explanations: motivation, input/output domains, derivation,
  invariants, examples, error order, cost, typed converse/complement models and
  application boundaries. Substance sets the length.
- Store original explanations, not copied paper text, figures or library
  summaries. Our code and explanations follow the project licence; scholarly
  works keep their rights.
- Keep a paper's claim, a source-derived argument, a finite observation and a
  formal proof as separate evidence. An error percentage needs a metric and an
  admitted domain; time complexity is not a measured speedup.
- Before reusing a bundle, compare source and canonical-model hashes and review
  what changed. A source-only update makes an edition stale; the reader rejects
  it until reviewed. Never refresh historical hashes silently.
- Existing snapshot paths stay stable, because worked checks, cross-bundle
  dependencies and research receipts refer to them.

## Publication scope

Code-associated groupants, external copies and snapshots, and paper models of
Mova's implementation are implementation material, not global website
content. Storage outside Git does not change that, and copying or renaming an
implementation wrapper does not make it a general exposition.

Code-model publication and global exposition publication are separate outputs:
a publisher renders Mova's code groupants from the committed `model.md` files
and lists only admitted entries of the global library. A publisher selects the
source scope explicitly; storage location alone grants no publication permission.

A separate `publication.json` catalog in the store pins each manifest by
SHA-256. Its authoritative field is `entries[].library_scope`: `global`,
`mova_implementation` or `unreviewed`. A missing entry, an unknown scope or a
stale hash never means global. Immutable manifests and historical indexes are
not rewritten to add a label.

Global admission requires all of:

- explicit `library_scope=global`;
- `editorial_status=independently_reviewed`;
- explicit redistribution rights;
- disjoint hash-bound review evidence under `reviews/`;
- current integrity validation;
- a nonempty `allowed_artifacts` list (the label alone never authorizes copying
  every file in a bundle).

The supported paper-survey edition permits only its declared description
narrative; raw source bytes, code and syntax artifacts are excluded. Other
exposition kinds are not yet supported. Unclassified outputs stay out of pages,
search, sitemap and cross-links.

```sh
# conservative catalog; can only deny
mova-modelcheck publication-catalog "$MOVA_GROUPANTS" - "$RUN/publication.json"
mova-modelcheck publication-check . "$MOVA_GROUPANTS" "$RUN/publication.json" \
  algorithms/a_star/manifest.json "$RUN/a-star-denied.json"

# positive admission with a caller-pinned review policy
mova-modelcheck publication-catalog "$MOVA_GROUPANTS" "$REVIEWS" "$RUN/reviewed-catalog.json"
mova-modelcheck publication-check . "$MOVA_GROUPANTS" "$RUN/reviewed-catalog.json" \
  "$MANIFEST" "$REVIEWS" "$EXPECTED_REVIEWS_SHA" "$RUN/admission.json"
```

`EXPECTED_REVIEWS_SHA` comes from trusted configuration, never from the catalog.
Review files use schema 1 and kind `groupant_publication_reviews`. A zero exit
status means the check completed, including ordinary denials: consumers must
read `public_export_allowed`, `allowed_artifacts` and the version pins. Positive
admission recomposes the bounded store inventory, so new manifests invalidate a
positive catalog until it is regenerated, and one malformed or stale entry can
deny the whole positive batch. Editorial review does not change a generated
artifact's mathematical assurance. The trusted local filesystem, bounded
inventory and editorial authority are assumptions; this is not a proof of
authorship or scientific truth.

## Evidence limits

- Structural counts, partial syntax, declared relations and finite native
  checks are different from semantic proof. `available_partial` means only that
  a declaration passed structural checks.
- Hashes detect drift; they do not prove that a model matches the code.
- Supplied-input joins do not exercise the live parser, teacher, corpus
  acquisition, persistence or journal-backed optimization.
- No school, held-out accuracy, GPU, training-throughput, approximation-budget,
  automatic-substitution or whole-system result follows from the models.
- Specific mutant sensitivity is not universal fault detection. Bounded CPU
  microbenchmarks keep both gains and regressions and do not measure full-Mova
  throughput.
- A group's numerical error budget needs a composition argument; leaf
  percentages alone do not add. Unmodeled categorical behavior and side effects
  stay explicit gaps.
- Macro expansion, semantic AST analysis, complete reverse-reconstruction
  models and the general combining engine are open work.
