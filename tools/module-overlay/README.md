# External module overlays

Mova's animals and the Octopus, the large language model that develops Mova, are described in [the sea animals of Mova](../../docs/animals.md).

`mova-module-overlay` composes an approved public source tree with explicitly
bound replacement/addition files held in a separate customization directory.
It writes a fresh source tree and a deterministic receipt. It never edits
upstream, executes a build, launches a worker, or evaluates shell hooks.

Build this standalone tool with at most two jobs:

```sh
cargo build --manifest-path tools/module-overlay/Cargo.toml --jobs 2
```

Its three commands are:

```text
mova-module-overlay snapshot PUBLIC_SOURCE REVISION_LABEL NEW_BASE_JSON
mova-module-overlay check PUBLIC_SOURCE CUSTOM_ROOT MANIFEST_REL
mova-module-overlay stage PUBLIC_SOURCE CUSTOM_ROOT MANIFEST_REL NEW_TREE
```

`check` prints a structured base-drift report and exits 3 for an incompatible
base. Invalid manifests/unsafe inputs fail separately. `stage` requires an
unchanged bound base and a nonexistent output. Neither command runs Cargo.
The receipt provides explicit `cargo check` and `cargo build` argv with two jobs
for each declared Cargo manifest; run them deliberately from the composed root.

See [the architecture and upgrade workflow](../../docs/animals.md).
The checked-in [example](examples/) contains a tiny public upstream, external
customizations, exact hashes, an abstract Whale planning crate and an illustrative
Octopus task plan. The tiny upstream is not a Mova distribution.

To exercise it, choose a fresh output outside the example input directories:

```text
mova-module-overlay check tools/module-overlay/examples/upstream tools/module-overlay/examples/custom manifest.json
mova-module-overlay stage tools/module-overlay/examples/upstream tools/module-overlay/examples/custom manifest.json NEW_TREE
```

Then, from `NEW_TREE`, run the recipe explicitly:

```text
cargo check --manifest-path tools/example-whale/Cargo.toml --jobs 2
cargo test --manifest-path tools/example-whale/Cargo.toml --jobs 2
```

The Whale example plans four review steps. It does not fetch papers, generate
articles, check plagiarism, or start a service. Its Rust source is stored as a
text customization artifact and becomes `src/lib.rs` only in the composed tree.

`registry-entries.json` is a standalone component registry for the three Rust
owners and their composed parent. Its adjacent groupants and exact syntax
projections are source-bound. A main-repository integrator must attach the parent
node to the central ownership tree and update that tree's transitive bindings;
combining disconnected registry fragments is intentionally rejected. Standalone
component validation does not claim whole-Mova registration or verification.
