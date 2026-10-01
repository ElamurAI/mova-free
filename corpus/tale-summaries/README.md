# Tale summaries

One summary per paragraph of the books in [../tales](../tales/), one JSON object
per line: `book`, `par` (paragraph number), `story` (story number in the book),
`summary`.

**How made:** produced for Mova by a large language model reading each
paragraph; the paragraph text itself is in `../tales` and is not repeated here.

**License:** ours, Apache-2.0 OR MIT.

**How Mova uses it:** as a reference when checking Mova's own skeleton
summaries of paragraphs (`ai/world`, `world read` / `world check`).
