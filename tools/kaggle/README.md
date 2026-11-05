# kaggle — teacher models

Models the developer builds for Kaggle tasks. They are teachers only: their predictions may be given to Mova as
pseudo-labels (`world tabular … --teacher`) or used to find features, but what is submitted is always Mova's own
answer (`world tabular`, `world/src/tabular.rs`).

```
kaggle titanic <data dir> [--out submission.csv]
```

`<data dir>` holds the competition's `train.csv` and `test.csv`. The command prints the 10-fold cross-validation of three deterministic models (gender, a greedily learned decision list, woman-child groups) and the decision list learned on all of train, each prediction named by its rule. `--out` writes the woman-child-group predictions in the submission format; that file is a teacher's output for comparison and pseudo-labels, not what the project submits.

## Licence

Elamur Cosmic License ([LICENSE.md](../../LICENSE.md)). Third-party data keeps its own terms, as noted where it is used.
