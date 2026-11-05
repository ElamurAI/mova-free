# modelcheck-consolidate

## Descriptant

`consolidate.rs` implements `mova-modelcheck consolidate REPO DIR [--dry-run] NEW_REPORT` and `consolidate-check REPO BASE NEW_REPORT`: the migration to one Markdown file per groupant (operator decision) and its independent text-preservation gate. Mechanics only: it moves text and references and never edits model content.

Pilot on `algorithms/`, then crate by crate, then the private tree (not in the public repository). Prospective prediction before the run: no heading collisions in `algorithms/` (stems are distinct lowercase words and most former files are wrapped); collisions expected only where a section-referenced file carries an H2 equal to another file's stem. Observed on `algorithms/`: 413 sidecars merged, 0 unmerged.

## Forward

Inputs: Git-visible files (tracked and nonignored untracked) of REPO; sidecars are directories named `*.groupant` below DIR with direct `.md` children; a module `prospective.md` beside a `mod.rs.groupant` sidecar folds into it.

1. A former file is verbatim when some JSON record pairs a `path` resolving to it with a `section` naming exactly one of its H2 headings; otherwise it is wrapped.
2. A sidecar merges unless a former file ends inside a code fence, a fold-in has links, or a verbatim prospective meets a fold-in. Colliding verbatim H2 headings are renamed `X (stem)` (see the unique-headings epoch). A sidecar whose only Markdown file is `model.md` is left alone.
3. `model.md` = `# <manifest id>`, the migration line when a fresh hash-bound reference is rebound, then one `## <stem>` block per former file in the fixed order (descriptant, model, approximant, invariant, cost, composition, catalog, models, tests, evidence, prospective, then by name). Verbatim text follows its heading unchanged; wrapped text is demoted so its top heading is H3 (H4 under the `### Groupant prospective` / `### Module prospective` split when both prospective files exist).
4. Every `.md` token resolving to a former file is rewritten: the basename becomes `model.md` (relative from the former file's directory), with `#<stem>` for wrapped targets in JSON and for every target in Markdown when no fragment is present. JSON string literals are replaced in place (no key reordering); keys `file` (evidence run directories) are never rewritten.
5. A JSON object with `path` and `sha256` bound to a former file: fresh → rebound to the new bytes (a section's bytes for `model.md#<S>`, the whole file otherwise); stale → path moved, hash kept, reported. Fresh bindings to Markdown files that changed only by moved links, and to rewritten JSON files one level deep, are rebound and reported; deeper chains are reported.
6. Without `--dry-run`, it writes `model.md` files and edited files; it never deletes. The report's `remove` list is for `git rm`.

## Invariant

For every merged sidecar, each former file's text equals its block body up to `.md` reference targets and a uniform heading shift; nothing else is added except the title, the migration line, block headings and blank lines. All maps are `BTreeMap`/`BTreeSet`; reports of two runs on the same tree are identical. A stale hash is never refreshed. Unmerged sidecars and their references are untouched.

## Cost

One `git ls-files`, one read of every sidecar Markdown file and of every JSON file containing `.md`, one read of every Markdown file in the repository: O(total bytes) scanning with O(token count · log F) map lookups for F former files. The check adds one `git diff` and one `git cat-file --batch` stream.

## Tests

`consolidate::tests::merges_rewrites_rebinds_and_checks`: exact merged text (title, migration line, verbatim and demoted blocks, fenced pseudo-heading kept), manifest and docs rewrites, section rebinding, dry-run determinism and no writes, idempotent second run, check valid; negative controls: one changed word and one invented line make the check fail. `consolidate::tests::collisions_and_open_fences_stay_unmerged`: anchor collision and an open fence leave the sidecar unmerged. Real runs: `consolidate-check` against the integration base for each committed step.

## behavior_complement

It does not merge child groupant directories into their parents, rewrite Rust string literals, follow symlinks outside Git's list, read untracked ignored files, rewrite `.md` mentions without `.groupant/` in Markdown prose, rebase links of moved fold-ins (those sidecars stay unmerged), refresh stale hashes, edit the external store `$MOVA_GROUPANTS` or decide anything about model correctness.

## reverse_reconstruction

Given a merged `model.md` and the list of former stems, each block body maps back to its former file: verbatim bodies are the file itself, wrapped bodies minus the uniform heading shift (the minimum level found above 3, or the recorded prospective split) give the file; `.md` reference targets are recoverable only up to the rewrite map (former basename → `model.md#<stem>`), which is injective within a sidecar because stems are unique. Blank-line padding at block ends and the original trailing newline are not recoverable.

## text_preservation

`consolidate-check` is the executable reverse: for each sidecar changed against BASE, it splits `model.md` at the H2 lines that name former stems, and requires every block body to equal its former file (read from BASE) after normalizing `.md` references, for one heading shift in 0–3; the preamble may hold only the title, the migration line and blank lines; every stem has exactly one block; no other Markdown file remains in the sidecar. Bounded executed: the fixture tests above and the real migration steps (counts in the commit messages).

## History

### 2026-11-04 · Descriptant

`consolidate.rs` implements `mova-modelcheck consolidate REPO DIR [--dry-run] NEW_REPORT` and `consolidate-check REPO BASE NEW_REPORT`: the migration to one Markdown file per groupant (operator decision, 2026-11-04) and its independent text-preservation gate. Mechanics only: it moves text and references and never edits model content.

Epoch 2026-11-04: pilot on `algorithms/`, then crate by crate, then the private tree (not in the public repository). Prospective prediction before the run: no heading collisions in `algorithms/` (stems are distinct lowercase words and most former files are wrapped); collisions expected only where a section-referenced file carries an H2 equal to another file's stem. Observed on `algorithms/`: 413 sidecars merged, 0 unmerged.

### 2026-11-04 · document

#### Epoch 2026-11-04 link-only changes

First real failure of the gate, on `annot/`: a `model.md` of an already single-file sidecar elsewhere (`algorithms/src/bfs/mod.rs.groupant`) changed only by a moved link, and `consolidate-check` treated it as a merge of one former file. Correction: a sidecar is checked only when some non-`model.md` former file left it. Regression fixture: the `model.md` of the fixture sidecar `b.rs.groupant` in `merges_rewrites_rebinds_and_checks`, which links into the merged sidecar.

#### Epoch 2026-11-04 unique headings

Operator decision: every sidecar ends with one `model.md`, so heading collisions no longer leave a sidecar unmerged. Rule: block headings (`## stem`) never move; for an anchor shared by verbatim H2 headings, the first in block order keeps its name unless a block heading holds the anchor, and every other one becomes `## X (stem)` with the stem of its own former file. `section` fields that name a renamed heading follow it, and hash-bound section references bind the renamed section. A sidecar is left unmerged only if an anchor still collides after renaming. `consolidate-check` reverses the rename per block (`## X (stem)` → `## X` for the block's own stem). Previous run on the older tree: 25 sidecars stayed unmerged for collisions. Fixture: `collisions_rename_and_open_fences_stay_unmerged` (rename, section-field follow, check valid; wrong suffix as negative control).
