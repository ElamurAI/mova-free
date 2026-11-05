# treeagree — agreed UD trees (the teacher + the best open parsers)

The teacher's school silver stays as it is; next to it lives a parallel layer, `school-agreed.conllu`, where words change only by written rules (`src/rules.rs`) on the evidence of Stanza, Trankit (UD) and spaCy trf (ClearNLP). Nothing is fixed by hand sentence by sentence.

```
cargo build --release --offline
treeagree agree $MOVA_DATA/corpus/school/school-silver.conllu <parse run dir> \
  $MOVA_DATA/corpus/school/school-agreed.conllu <new agree run dir>
treeagree check <file.conllu>     # the verifier: gate violations per kind
treeagree dispute-ask <draft.conllu> <parsers dir> <prompts.jsonl> [sentences per prompt, default 8]
treeagree dispute-apply <draft.conllu> <parsers dir> <prompts.jsonl> <answers.jsonl> <out.conllu>
```

`dispute-ask` writes one JSONL prompt per group of sentences (`id`, `sents`, the fixed instructions and the variable part): each sentence once as numbered words, and for every non-punctuation word whose head the agreed draft, Stanza and Trankit dispute, a lettered choice («17 called: A → 3 word (acl) | B → 9 is (ccomp)»). The teacher answers «<n> <word> <letter>» only where it would change the draft. `dispute-apply` reads the answers (JSONL with `id` and `text`), applies the picks to the draft and writes the result; silence keeps the draft. How the witnesses are used: [parsers.md](../../docs/parsers.md#how-they-are-used).

The parser outputs come from a school parse run (runners in `<run>/bin/`).

## Rules (priority order; a word takes the first that fires)

| rule | what | evidence |
|---|---|---|
| R1 cc-next | `cc` on the left conjunct → the conjunct that follows it (UD 2) | Stanza or Trankit |
| R2 punct-next | comma/semicolon before a conjunct → that conjunct | Stanza or Trankit |
| R3 conj-first | chained `conj` → the first conjunct | Stanza and Trankit |
| R4 particle | `compound` on ADP/PART/ADV after its verb → `compound:prt` | the UD guideline |
| R5 strong | another head (and relation) | Stanza and Trankit agree, spaCy trf picks the same head |
| R6 punct-agree | another head for punctuation | Stanza and Trankit agree |
| R7 rel-agree | same head, another relation | Stanza and Trankit agree |
| R8 pos-agree | another UPOS + XPOS; FEATS and lemma from the silver itself (most frequent for form + tags, else for the tags) | Stanza and Trankit agree; rides on the word's other edit, or alone only if both parsers keep the teacher's base relation |

Never overridden: UPOS `X`, `flat`, `flat:foreign`, `fixed`, `goeswith`, `reparandum`. A conjunct with its own `cc` stays `conj`. A moved word keeps the teacher's subtype when the base relation is the same.

## Gates (`src/tree.rs`)

UD 2.18 registry for English, heads in range, one root and `root` ⇔ head 0, no cycles, projective `punct`, relation fits the UPOS (a relation and a tag change together or not at all; an edit refused with its retag is retried without it). Edits go one by one; an edit stays only if the tree keeps its shape and gains no violation the teacher's tree did not have. A swap (new root, cycle) is tried together with the pending edits of the words involved. Rejected edits go to `rejected.tsv` with the reason.

Changed words carry `Agree=<rule>|Haiku=<head>:<rel>` in MISC, retagged ones also `HaikuPos=<UPOS>:<XPOS>:<FEATS>`; the `# annotation` line lists the rules applied.

A lone retag requires both parsers to keep the teacher's base relation; otherwise a tag would change alone while the parsers want another relation too («when» SCONJ → ADV under `mark`).

## Licence

Elamur Cosmic License ([LICENSE.md](../../LICENSE.md)). Third-party data keeps its own terms, as noted where it is used.
