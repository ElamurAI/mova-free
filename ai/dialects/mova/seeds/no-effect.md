# Changes with no effect on `mova` annotation — proposals

None of these changes touches English annotation. The decision is short, and the explanation is in the source seed.

| change | proposal | reason |
|---|---|---|
| [`expl:pass`, `expl:pv` — universal pages](../../ud-next/seeds/expl-pass-pv-docs.md) | accept as is | documentation only; English has no such subtypes. One thing is useful for the language-independent layer: this is a common subtype of three language groups |
| [validator data](../../ud-next/seeds/validator-data.md) | do nothing; repeat the comparison before 2.19 | no change for en; registry = 2.18 = snapshot of 24.09 |
| [`conllu_convert_uposf_to_xpos.pl`, treebank script](../../ud-next/seeds/tools-scripts.md) | do not take into the pipeline | a Perl tool with `Lingua::Interset`, while Mova's mechanics are Rust. As a one-off external cross-check of UPOS+FEATS ↔ PTB pairs (`en::penn`) — only on Mova's say-so. Script (`Latn`) — a line in the `treebanks/` cards |
| [list of English treebanks](../../ud-next/seeds/en-treebank-list.md) | accept as is | documentation |
| [other languages and the site](../../ud-next/seeds/other-languages.md) | accept as is | not English. For the future language-independent layer, note `flat:redup` (hy, axm) and `expl:rel` (egy) |
| [v1 labels in `specific-syntax.md`](../../ud-2.18/seeds/v1-labels.md) | not applicable | `mova` is v2; take examples from this file only with `dobj` → `obj` etc. replaced |

Mova decides.
