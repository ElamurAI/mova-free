# School corpus format

One sentence = one CoNLL-U block. The file is plain CoNLL-U, so `en`, `annot` and the UD tools read it without changes.

## Block layout

```
# source = school lesson (ours)
# course = <course folder, e.g. chemistry>
# lesson = <lesson file, e.g. 013-rust-irons-slow-fire.lesson>
# annotation = <pipeline: draft, patch model, gates, batch id>
# text = <the sentence as it stands in the lesson>
# sent_id = school:<course>:<lesson>:<index in the lesson>
<token rows: ID FORM LEMMA UPOS XPOS FEATS HEAD DEPREL DEPS MISC>
<blank line>
```

- The tree (AST) is columns 7–8 (HEAD, DEPREL), as in UD 2.18 with Mova's registry. It stays in the same block as its sentence, always.
- `sent_id` is the key across all layers. Nothing is keyed by line number.

## Layers, added later

Pragmatics and other layers are added to the same block, so one file holds everything about a sentence:

- **Token-level labels** go in MISC as `Layer=value`, e.g. `Prag=speech-act:request` (layer name first, so layers never clash).
- **Sentence-level and structural labels** go in comment lines `# <layer> = <json>` placed after `# sent_id`, referencing tokens by ID, e.g. `# prag = {"act":"request","speaker":4,"addressee":"human"}`.
- A layer that has not run yet is simply absent. No empty placeholders.
- Layers never change FORM, the token count, or the tree unless the layer's own gate passes; tree changes come from a gated patch step only.

## Rules

- One block per sentence, in lesson order. No blocks without a tree.
- A sentence the gates reject is not written. Rejects are logged in the run folder, not in the corpus.
- To re-annotate a sentence, replace its block; never append a second block with the same `sent_id`.

## Dictionary lessons (`world/data/school/dictionary/`)

One lesson per word of the primary dictionary (`world/data/dictionary-schools/primary-school.v2.tsv`), file `NNN-<word>.lesson`, `course: dictionary`, `lesson: NNN` = the word's place in the generation order.

- Same line format as the other courses (`title`, `level`, `stars`, `explain`, `fact`, `exercise`, `test`, `exam`).
- **Questions differ from the older courses:** the options are shuffled (deterministic seed = lesson number) and the right one carries a leading `*`; it is not always first. Every question has exactly one `*` (checked for all lessons). The `world` lesson parser does not read `*` yet, and `dictionary` is not in `knowledge::course_names`, so nothing loads these lessons until the parser learns the marker.
- Built by `tools/dict-lessons` (Rust): `requests` / `requests-empty` → Batches API (Haiku 5.5, effort medium) → `post`; words with school sentences get up to 5 of them (by `sent_id`) as context.
- Words the model refused (swear words) have no lesson; their output is kept outside the repo.
