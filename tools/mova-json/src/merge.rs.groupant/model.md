# mova-json merge, rebind and groupant byte-range library

## domain
A git repository with `git` on PATH; JSON files in layouts the value library reproduces; for merge3 a merge in progress (MERGE_HEAD present); for rebind-groupant a base commit holding the previous source bytes and sidecar manifests under `FILE.groupant/`.

## forward
matching_blocks(a, b): longest common subsequence after trimming common prefix/suffix, O(n*m) table on the middle bounded by 40M cells, returned as maximal blocks. Merger.merge(base, ours, theirs): equal sides or one unchanged side resolve directly; objects merge key-wise (one-side delete of an unchanged value applies, delete vs change conflicts, theirs additions go to the sorted place in sorted objects, else after their predecessor); arrays whose elements all have unique (path,id,section,type,name) keys merge per key (path-sorted arrays keep sorted insertion); other arrays merge by diff3 hunks against base: disjoint hunks apply, identical hunks apply once, two pure insertions at one point concatenate ours-then-theirs only when the whole result keeps unique elements; anything else is a conflict and ours stands in. A both-sides change of `sha256` in an object with a `path` is settled by hashing the working-tree file (resolve_bound: `./`/`../` relative to the manifest directory, others repo root then manifest directory). merge3_file reads base/HEAD/MERGE_HEAD through git show, takes the layout of HEAD (else MERGE_HEAD), writes the working-tree file only with zero conflicts, optionally refreshes every {path, sha256} binding. rebind_manifest refreshes listed paths in source_bindings and with add inserts missing ones at the sorted place. rebind_groupant maps every sidecar coverage.byte_range through the line diff of base -> current source (exact inside unchanged lines; a start moves to the end of the previous unchanged block, an end to the start of the next), rehashes covered bytes and the source, and rebinds tracked `*.groupant/**/manifest.json` objects `{path: FILE, sha256: old}`.

## invariant
- merge3 never writes a file with a conflict; exit status 3 marks conflicts.
- Arrays never gain a repeated element by joining two insertions (argv safety: two different `--add-dir X` additions conflict).
- rebind_manifest changes only bindings of listed paths; unlisted stale hashes stay stale (no silent refresh); a listed bound file that is missing is an error.
- rebind_groupant reports every fragment whose covered text changed (TEXT CHANGED) instead of claiming the model still holds; obernant `revision`/`subject_revision` fields are not touched.

## cost
Merge is O(size of the three values) plus diff3 tables O(n*m) per unkeyed array (bounded; over the bound the array is a conflict). git show is three subprocesses per file. rebind_groupant is one git show, one line diff (bounded), one pass per sidecar manifest and a `git ls-files` scan reading only manifests whose text contains the old hash.

## behavior-complement
Not done: staging (`git add`), committing, resolving conflicts by preference, merging non-JSON files, rebinding `revision`/`subject_revision`, mapping ranges across intra-line edits exactly (an edited line maps to block boundaries), detecting moved code, checking that a model still describes changed text, touching contracts.json hashes.

## reverse-reconstruction
A clean merge output is compatible with many (base, ours, theirs) triples; with the three inputs retained in git, the result is reproducible (deterministic order rules). A rebound byte range does not recover the old range without the base commit; with it, the old range is the input and the report line records both.

## diff3-insertion-join

Type: partial_exclusion. Domain: unkeyed arrays changed on both sides at one insertion point. Excluded output: a merged array with a repeated element. Negative control: dropping the uniqueness check (filtering theirs by ours) merged `--add-dir A` and `--add-dir B` into `--add-dir A B` and turned merge3_true_conflicts_are_red red.

## checks

Unit tests: clean merges (keys, keyed nodes with delete, path-sorted bindings, disjoint sequence hunks, unique joint appends), true conflicts (scalar, delete vs change, keyed delete vs change, argv joint insertion, overlapping replacement) with a same-change control; offset mapping with an identity-map red control; rebind with stale-unlisted, missing-file and non-canonical red controls; rebind_groupant on a temporary git repo with TEXT CHANGED control; merge3_file inside a real conflicting git merge (hash conflict settled from the working tree, joint list insertion). Mutation controls run: raw escapes dropped, unsorted insert, delete-vs-change ignored, identity map, no rehash: each turned at least one test red.

## epoch-memory

Replaces scratch merge3.py, mergejson.py, bind.py and rebind.py. merge3.py was asymmetric and resolved true conflicts silently to one side; this version reports them. difflib's SequenceMatcher is replaced by an exact LCS, so ambiguous line matches can map differently from the Python helper.

## History

### 2026-11-05 · diff3-insertion-join

Type: partial_exclusion. Domain: unkeyed arrays changed on both sides at one insertion point. Excluded output: a merged array with a repeated element. Negative control: dropping the uniqueness check (filtering theirs by ours) merged `--add-dir A` and `--add-dir B` into `--add-dir A B` and turned merge3_true_conflicts_are_red red (2026-11-05).

### 2026-11-05 · checks

Unit tests: clean merges (keys, keyed nodes with delete, path-sorted bindings, disjoint sequence hunks, unique joint appends), true conflicts (scalar, delete vs change, keyed delete vs change, argv joint insertion, overlapping replacement) with a same-change control; offset mapping with an identity-map red control; rebind with stale-unlisted, missing-file and non-canonical red controls; rebind_groupant on a temporary git repo with TEXT CHANGED control; merge3_file inside a real conflicting git merge (hash conflict settled from the working tree, joint list insertion). Mutation controls run 2026-11-05: raw escapes dropped, unsorted insert, delete-vs-change ignored, identity map, no rehash: each turned at least one test red.

### 2026-11-05 · epoch-memory

2026-11-05: replaces scratch merge3.py, mergejson.py, bind.py and rebind.py. merge3.py was asymmetric and resolved true conflicts silently to one side; this version reports them. difflib's SequenceMatcher is replaced by an exact LCS, so ambiguous line matches can map differently from the Python helper.
