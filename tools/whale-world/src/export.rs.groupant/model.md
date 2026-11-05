# mova-whale-world-export

## Descriptant

Export of the Whales' finished descriptions into an editorial texts repository (`mova-whale-texts-v1`): per passed paper a main article (the last gate-passed round's description with footnotes after each run of sentences sharing their sources, and a bibliography), `seo.json`, shadows from the paper's dossier (Mova drafts, consultant drafts, earlier descriptions), an index page and a manifest. Dossier originals and web pages are listed by id, URL or file name and licence, never copied. Every Markdown file carries front matter with visibility, licence, provenance hashes and a seal (`export_sha256`). Source: `tools/whale-world/src/export.rs`.

## Forward approximant

batches.jsonl opened events (batch >= from_batch; later assignment wins) -> learner state papers/<doc>/status.json passed -> last round-NNN with gates.pass, description.txt, draft.json -> meta (library work, else batch-log title/licence) + refs.json -> cite: draft items -> source ids (paper for restated/structure items; knowledge provenance terminal/reread -> consult:<learner>, topic-corpus -> doc, pod:<learner>:<id> in that learner; Surname..YYYY matching a reference -> ref:<k>) -> runs of the same section = paragraphs -> plain text == description.txt ? sentence mode : paragraph mode -> markers after each run of equal source sets -> footnotes in first-use order ; dossier rows -> original/web listed, mova/sol/site-old shadows (distinct sha, main text excluded) -> front matter + seal -> put (unchanged | write via temp + rename | keep edited/foreign) -> index over manifest papers -> manifest.

## Invariant

The same inputs give the same bytes (no clock, BTreeMap/BTreeSet order, sorted rounds); a second run writes nothing; a Markdown file is replaced only while its seal and its manifest hash both hold, `seo.json` only while its manifest hash holds; nothing is deleted; the text of an original or a web page is never written; stripped of its markers, a sentence-mode body equals the description.

## Cost

One pass over the batch log, one status, gates, description and draft per paper, the learners' knowledge once per learner, the dossier rows and shadow texts, and one write per changed file. Measured on 83 papers: 6.7 s including the library load (about 10 s budget), 272 files, 4.7 MB.

## Composition

Parent: whale-world. Reads dossiers through `dossier::Dossier` (hash-checked reads; `exists` before `open`, so no dossier is created) and the library through `topic_corpus::library::Library` (`get`, `bibliography`, `resolve`, `bare_doi`). Called by `main` (`export-texts`). The texts repository's own conventions (main article, shadows, seo.json) are its contract; the exporter does not commit.

## behavior_complement

Does not run learners, ask a terminal, import dossiers, fetch, publish, commit, merge, delete files, or rewrite a file that a person edited or that it did not write; does not judge the quality of a description; does not export papers that never passed, rounds that failed, or texts outside the dossier and the round; does not follow citations inside shadow texts. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

From a main article: `description_sha256` and `draft_sha256` name the exact round files, `batch`, `round` and the describer name the learner state; removing the footnote markers recovers `description.txt` in sentence mode (checked in tests). From a shadow: `text_id` and `sha256` recover the dossier row and its exact text. From the manifest: every written file's hash, so an edit is detectable. Machine paths are reduced to file names and are recovered from the dossier row, not from the repository.

## History

### 2026-11-05 · Cost

One pass over the batch log, one status, gates, description and draft per paper, the learners' knowledge once per learner, the dossier rows and shadow texts, and one write per changed file. Measured 2026-11-05 on 83 papers: 6.7 s including the library load (about 10 s budget), 272 files, 4.7 MB.
