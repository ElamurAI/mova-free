# Contrast sets

Contrast sets (Gardner et al. 2020) keep a task and change its surface, to see whether a model learned the task or
the quirks of one dataset.

| Folder | What | Mova | License |
|---|---|---|---|
| `babi/` | the 20 bAbI test sets (Weston et al. 2015, v1.2) with new names, places, things and synonym verbs (`world babi-contrast` makes them deterministically from the originals) | 99.7%, 20 of 20 tasks (the original: 99.8%) | CC BY 3.0, as bAbI; derived by Mova |
| `stepgame/phrasings-dev.tsv` | 40 phrasings of the nine StepGame relations that the StepGame generator never produced, including the reverse view ("B has A on its left"); used to build Mova's reader of new phrasings | 100% | ours, Apache-2.0 OR MIT |
| `stepgame/phrasings-test.tsv` | 31 more, written before that reader existed; the 2 that also occur in the StepGame training data are left out when measuring | 100.0% at every k=1..10 (the learned perceptron alone: 17.1%) | ours, Apache-2.0 OR MIT |

Each line of a phrasing file: `label<TAB>template`, the label is the relation of `{A}` to `{B}`.
`world stepgame-contrast <StepGame dir> 500 [--test]` builds chains of 1–10 steps from them (deterministic) and
checks first that no phrasing occurs in the training data.
