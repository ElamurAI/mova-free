# mova-correspondence — the relation-catalogue audit

`mova-correspondence` checks the typed finite formulas of [models/correspondences.json](../../models/correspondences.json) against their source bindings and the groupant registry, and reports coverage. It is an audit: it never proves a relation and never substitutes a formula for production code. Method: [docs/correspondences.md](../../docs/correspondences.md).

## Run

```bash
cargo build --release -p mova-correspondence
target/release/mova-correspondence scan REPO REGISTRY CATALOG NEW_JSON
# e.g. scan "$REPO" "$REPO/models/contracts.json" "$REPO/models/correspondences.json" "$RUN/scan-1.json"
```

`NEW_JSON` must not exist. The command prints the output path, the coverage, `"proof": false` and `"production_substitution": false`. Run it twice and compare the two reports before claiming a change.
