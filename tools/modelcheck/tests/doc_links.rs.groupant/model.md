# docs-link-check

## descriptant

The public-documentation link gate. It lists every Git-tracked Markdown file outside `peta/`, extracts the relative
link destinations outside fenced code and inline code spans (`](dest)`, reference definitions, `href`/`src`
attributes), resolves each lexically against the linking file (a leading `/` against the repository root) and
requires the target to exist; a `#fragment` into a Markdown target must equal a heading slug (GitHub rules: lowercase,
letters, digits, `-` and `_` kept, a space becomes `-`, repeated slugs numbered `-1`, `-2`) or an explicit
`id`/`name` attribute. External URLs (`scheme://`, `mailto:`, `tel:`) are not fetched. The gate is a test, so it runs
with `cargo test -p mova-modelcheck --test doc_links`.

## Forward

`check(root, files)` returns `file:line: dest (reason)` for every destination whose path leaves `root`, does not
exist, or whose fragment is not an anchor of an existing Markdown target. `public_markdown_links_resolve` asserts the
list is empty over `git ls-files '*.md'` minus `peta/` and `tera/`.

## Invariant

Lines inside a fence opened by three or more backticks or tildes and closed by a bare fence of the same character
and at least the same length are neither links nor headings (CommonMark: a fence line with an info string does not
close). Anchors are computed from the target's own text only; the result is ordered by file list and line.

## Cost

One `git ls-files` call, one read per linking file and at most one read per distinct Markdown target (cached anchor
sets): linear in the bytes of the public Markdown plus `O(L log T)` set lookups for L links and T targets.

## behavior_complement

Not modeled: rendering. Setext headings, HTML block anchors other than `id="…"`/`name="…"`, link text spanning
lines, reference links whose label is defined in another file, autolinks without a scheme, and fragments into
non-Markdown targets are not checked. A path that exists only in an untracked or ignored file passes. Case-folding
file systems may accept a wrongly cased path that a case-sensitive host refuses.

## reverse_reconstruction

An empty report admits every repository whose relative targets exist and whose fragments are anchors; it does not
recover which links were intended, nor show that a link points at the right section. A reported line identifies one
destination string, but several defects (a moved file, a renamed heading, an unclosed fence above the heading) map
to the same report.

## Tests

`a_broken_path_and_a_missing_anchor_are_reported` is the negative control: a fixture with a missing file, a heading
hidden in a fence, a missing anchor and a path escaping the root yields exactly four reports, while valid slugs
(an em dash giving a double hyphen, a numbered duplicate, an explicit id, a same-file anchor), an inline code span,
a fenced link and an external URL yield none.
