# modelcheck-links

## Descriptant

`links.rs` implements `mova-modelcheck sidecar-links REPO NEW_JSON` and `relink-evidence REPO [--dry-run] NEW_REPORT`. The check fails when Markdown inside a `*.groupant/` sidecar names a sibling or parent `.md`/`.json` file that does not exist, or a `file.md#<anchor>` whose heading is absent. The rewrite turns names of former sidecar files (merged into `model.md` by `consolidate`) into the `model.md#<section>` that now holds their block. Mechanics only: it changes link text, never model content.

## Forward

Inputs: Git-visible files of REPO (tracked and nonignored untracked), the set of paths deleted anywhere in Git history (`git log --diff-filter=D --no-renames`), every Markdown text and, for the rewrite, every JSON text.

1. `tokens`: maximal runs of `[A-Za-z0-9_./-]` ending in `.md` or `.json` (trailing sentence dots dropped), with an optional `#fragment`. A run directly after `$`, `~`, `:`, `@` or `*`, starting with `/`, or followed by `*` is external and skipped; so is a name starting with a dot.
2. `scan` per sidecar Markdown file: skip `syntax.json` (regenerable, absent from the public tree), a hash provenance record (the name followed by its SHA-256: `` `x.md`: `hex` ``, `x.md: hex`, `x.md, SHA256 hex`), a bare name listed as content of an external run (`$MOVA_DATA/… contains x.md` in one sentence), and a bare name that neither exists from the writing sidecar or an enclosing sidecar nor existed in history at a candidate location (JSON: unless some sidecar holds a file of that name). Paths with `/` are read from the writing directory and every ancestor directory. A missing file is a `missing_file` finding; an existing `.md` whose fragment matches no heading anchor is `missing_heading`.
3. `candidates`: the former location read from the writing sidecar, each enclosing sidecar outward, then the source directory of the outermost sidecar; a module `prospective.md` maps to its `mod.rs.groupant`.
4. `resolve`: the nearest candidate whose file existed in history decides: an H2 of its sidecar's `model.md` whose anchor is the stem, else that model's `## History` when a `### … · stem` group is there. Without any former file, an H2 of the writing sidecar's own `model.md`. `replacement` writes `model.md#<anchor>` relative to the writer; a whole Markdown link label (`[x.md](…)`) becomes the heading name.
5. `relink`: rewrites every resolvable finding, keeps unresolved ones and lists them; then the `undate` rebinder rebinds fresh hash-bound JSON references (`path` plus `sha256`) to the changed texts, a section's bytes for `model.md#<S>`, to fixpoint. Without `--dry-run` the changed files are written.

## Invariant

Only dangling token bytes change; every other byte of every file is kept. A rewritten reference always names an existing `model.md` and an existing H2 anchor, so the check accepts it; a second run rewrites nothing. A stale hash is never refreshed. All maps and sets are `BTreeMap`/`BTreeSet`; two runs on the same tree give byte-identical reports.

## Cost

One `git ls-files`, one `git log` of deleted paths, one read of every tracked Markdown file (and every JSON file for the rewrite). Per sidecar Markdown file a linear token scan; per token at most depth+2 candidate directories with ordered-set lookups and one H2 scan of a candidate `model.md`. Rebinding rescans the JSON literals once per round until fixpoint (at most 16).

## Tests

`links::tests::tokens_skip_external_and_keep_fragments`: environment-root, absolute, URL and glob tokens skipped; fragments kept; `.mdx` not a token; run-listing and hash-record rules with negative cases. `links::tests::check_is_red_then_relink_makes_it_green_except_unresolved`: a fixture repository with deleted `evidence.md` and `wave1-evidence.md`; the check is red with seven findings (bare, `../`, parent-qualified, History-only, link-label, missing relative file, missing heading); the rewrite produces exact expected texts (own-section decoy ignored in favour of the parent's former file), keeps the hash provenance record, rebinds the fresh section binding, leaves the stale one, the check is then red only for the two unresolvable references, and a second run rewrites nothing.

## behavior_complement

It does not delete files or lines, rewrite JSON reference strings, change prose qualifiers ("parent", "file-root"), change hash provenance records, invent sections (an unresolved name stays and is listed), check names outside `*.groupant/` directories, check bare names that never existed in the sidecar chain, render Markdown, refresh stale hashes or touch the external groupant store.

## reverse_reconstruction

The rewrite report lists file, line, former token and new token for each rewrite, so the former text is recoverable from it exactly. From the output alone, `model.md#<stem>` maps back only to the set of compatible former spellings (`<stem>.md`, `../<stem>.md`, a qualified bare name), and `model.md#history` to any former file with a History group of that stem; Git history holds the exact former text.

## Limits

Lexical rules: a run-directory listing written without `$MOVA_DATA/… contains` is not recognized; a bare name that never existed in the chain is presumed external; path tokens are accepted when they resolve from any ancestor directory. The interpretation of a bare name follows Git history, not the sentence's meaning.

## History

### 2026-11-05 · Tests

First repository run on the integration base plus this tool: `sidecar-links` red with 3,549 dangling references in 1,167 of 2,249 sidecar Markdown files (226 hash provenance records skipped). `relink-evidence`: two dry runs byte-identical; 3,541 rewrites in 1,163 files (3,539 to a former file's section, 2 to the writer's own section; most frequent targets `tests` 2,085, `evidence` 551, `approximant` 147, `prospective` 101); 12 fresh hash bindings rebound in 4 JSON files, none stale; a second run rewrote nothing. Eight references were left unresolved and fixed by hand (notation placeholders, a fixture path, a copied cross-file reference, a site-relative path, an implicit file-root JSON); the check is then green.
