# Event statistics of books — who did what to whom

`world events <out-dir> <book.txt>... [--threads N]` (world/src/events.rs) parses every sentence of a book and
counts events: agent, verb, patient, recipient (nouns only, pronouns skipped until coreference is wired in).
"The fox said to the crow" → `fox say →crow`; "the crow was silent" → `crow be silent`; "the cheese fell" →
`cheese fall`; "the tree held the crow" → `tree hold crow`.

Outputs, counts and ids only — no text, no trees, so it is allowed for books outside the public domain
(analysis logs; see `$MOVA_DATA/raw/gutenberg-restricted/README.md`):

- `books/<book>.json` — a primitive summary: main actors, interactions (`crow ~ fox`), top events;
- `books/<book>.events.tsv` — the search index: sentence id `<book>:<n>` → event (ids are stable within one
  version of the sentence split, `events::SPLIT`);
- `events.tsv` — corpus-wide events with counts and number of books;
- `cells.tsv` — observed verb × noun × role (SUBJ/OBJ/TO) counts: the empirical side of the absurdity matrix
  (`docs/absurdity.md`) — frequently observed cells are plausible, unseen ones are candidates for rating.

Uses: development analytics (what the corpus is about, what the reader meets most), a search index
(find sentences where a fox speaks to a crow), and the next waves of the absurdity matrix.

Publication: for public-domain books (EU/UA) the summaries and the sentence-id index are public and go into
the corpus; for the restricted books they stay internal: development analytics, and answers to specific user requests
as short summaries in our own words — the book is never shown and its sentences are never quoted. The two are always run
into separate output folders (`events-pd/`, `events-restricted/`) and never merged.
