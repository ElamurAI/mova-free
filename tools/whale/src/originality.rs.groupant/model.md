# mova-whale-originality

## Descriptant

Configurable bounded local lexical originality gate against primary documents and a configured corpus. Source: `tools/whale/src/originality.rs`. Public, domain-agnostic: concrete prompts, schemas, gate parameters and deployments come from pinned configuration.

## Forward approximant

Tokenize (Unicode alphanumeric/combining-mark runs, lowercase); select narrative strings except configured exempt keys; index exact-run, shingle and short-gram windows of each document (primary full text plus configured .md/.txt corpus files, optionally through a hash-verified SQLite text cache); mark exact runs, shingle coverage union and near-copy windows; decision blocked_overlap if any exact run, near window >= near_percent, or shingle union >= aggregate bounds; pending_incomplete if any coverage reason; else clear_in_checked_corpus.

## Invariant

publication_allowed iff decision=clear_in_checked_corpus; any incompleteness (missing roots, bound exhaustion, symlink, unreadable file, incomplete primary) yields at least pending_incomplete; thresholds lie in documented bounds and the three gram sizes are distinct; the cache returns text only if its hash equals the source hash.

## Cost

O(D * (T_doc + T_out * sizes)) BTreeMap operations for D documents with T tokens; windows add O(T_out * |windows| * width); bounded by max_entries/max_bytes/max_document_bytes/max_output_bytes.

## Composition

Called by whale::publish on the raw reply before projection with the specification thresholds merged with deployment corpus keys (spec::originality_config).

## behavior_complement

Domain: narrative fields and configured documents. Excluded from 'clear': any exact run, near-copy window or aggregate overlap; any incomplete corpus. Not detected: semantic paraphrase, translation, copying from outside the configured corpus. Counterexample: substitutions breaking every exact run still block via near windows (lexical_near_copy_without_exact_run_blocks). Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

A receipt records matched spans (capped at 100), document hashes and thresholds; compatible narratives are all texts with the same token coverage pattern; the original text is not reconstructible from counts. A clear decision admits every narrative with no configured overlap in the checked corpus only. No unique inverse is claimed.
