# kaggle — teacher models

Models the developer builds for Kaggle tasks. They are teachers only: their predictions may be given to Mova as
pseudo-labels (`world tabular … --teacher`) or used to find features, but what is submitted is always Mova's own
answer (`world tabular`, `world/src/tabular.rs`).

Titanic (2026-10-03, 10-fold CV on train): gender 78.68%, decision list 79.01%, woman-child groups 83.95%.

## Licence

Elamur Cosmic License ([LICENSE.md](../../LICENSE.md)). Third-party data keeps its own terms, as noted where it is used.
