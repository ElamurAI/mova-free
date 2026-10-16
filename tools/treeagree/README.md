# treeagree — agreed UD trees (Haiku + the best open parsers)

Haiku's school silver stays as it is; next to it lives a parallel layer, `school-agreed.conllu`, where words change only by written rules (`src/rules.rs`) on the evidence of Stanza, Trankit (UD) and spaCy trf (ClearNLP). Nothing is fixed by hand sentence by sentence.

```
cargo build --release --offline
treeagree agree $MOVA_DATA/corpus/school/school-silver.conllu $MOVA_DATA/runs/parse-school-2026-10-15 \
  $MOVA_DATA/corpus/school/school-agreed.conllu $MOVA_DATA/runs/agree-school-2026-10-15
treeagree check <file.conllu>     # the verifier: gate violations per kind
```

The parser outputs come from the school parse run (`docs/parsers.md` § School corpus run, runners in `<run>/bin/`).

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
| R8 pos-agree | another UPOS + XPOS; FEATS and lemma from the silver itself (most frequent for form + tags, else for the tags) | Stanza and Trankit agree; rides on the word's other edit, or alone only if both parsers keep Haiku's base relation |

Never overridden: UPOS `X`, `flat`, `flat:foreign`, `fixed`, `goeswith`, `reparandum`. A conjunct with its own `cc` stays `conj`. A moved word keeps Haiku's subtype when the base relation is the same.

## Gates (`src/tree.rs`)

UD 2.18 registry for English, heads in range, one root and `root` ⇔ head 0, no cycles, projective `punct`, relation fits the UPOS (a relation and a tag change together or not at all; an edit refused with its retag is retried without it). Edits go one by one; an edit stays only if the tree keeps its shape and gains no violation Haiku's tree did not have. A swap (new root, cycle) is tried together with the pending edits of the words involved. Rejected edits go to `rejected.tsv` with the reason.

Changed words carry `Agree=<rule>|Haiku=<head>:<rel>` in MISC, retagged ones also `HaikuPos=<UPOS>:<XPOS>:<FEATS>`; the `# annotation` line lists the rules applied.

## School, 15.10

44 011 sentences, 1.17 M words, 6 s. Changed 82.6 k words (7.1%) in 75% of sentences: R1 2 707, R2 1 961, R3 2 115, R4 167, R5 25 141, R6 12 308, R7 25 996, R8 12 201 alone; 22.0 k words retagged in all (NOUN → PROPN 3.7 k for names like Mole, Europa, Exodus; VBD/VB → VBN/VBP 3.2 k; quote XPOS 2.9 k). Head agreement with Stanza 85.5 → 89.3, Trankit 83.2 → 86.8. Gate violations: Haiku 9 298 → agreed 5 652 (non-projective punct 6 296 → 3 341; what is left is Haiku's own, no rule had evidence).

Spot checks (~60 edits): R5 ≈ 80% right (the rest ambiguous; dialect words like «aboot» fool the parsers), R6 and R7 ≈ 90% (typical R7: `conj`/`advcl` → `parataxis` after `;` or `:`, `obj` → `xcomp`, `amod` → `compound` with ADJ → NOUN), R8 ≈ 85% (misses: plural/singular of mass nouns like «graffiti», «leafcutter»).

The first R8 run let a tag change alone while the parsers wanted another relation too («when» SCONJ → ADV under `mark`): lone retags now require both parsers to keep Haiku's base relation.

## Licence

Elamur Cosmic License ([LICENSE.md](../../LICENSE.md)). Third-party data keeps its own terms, as noted where it is used.
