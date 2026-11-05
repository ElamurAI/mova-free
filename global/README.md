# global — the compiled global level

`global` is level 1 of the [three levels of logic](../docs/architecture.md): values (principles), concepts with their trigger words, shortcuts between concepts, verb classes, measures, selection lists and the absurdity matrix, compiled into Rust tables. Nothing is read from disk at runtime; every conclusion names the link and the seed file it came from.

## Sources

- `seeds/**/*.md` (`self`, `senses`, `shortcuts`, `values`, `wikidata`) — fenced blocks ```` ```principle ````, ```` ```concept ````, ```` ```link ```` (`from -> to : why`), ```` ```verbs ````, ```` ```measure ````, ```` ```selection ````. The seeds are explained in [docs/seeds/global-seeds.md](../docs/seeds/global-seeds.md).
- `data/absurdity-roles.tsv` (verb, noun, subject 0–4, object 0–4) — the absurdity matrix, see [docs/knowledge.md](../docs/knowledge.md); other `data/` files hold registers and SQL used to build seeds.
- `src/placement.rs` — the nonexclusive global/important-graph placement model of [docs/knowledge.md](../docs/knowledge.md).

## Build and test

```bash
cargo build --release -p global
cargo test -p global
```

`build.rs` is a gate: links only between declared concepts, no duplicates, valid matrix cells; a violation fails the build with an explanation. `GLOBAL_EXCLUDE="substring,…"` leaves out seeds whose path contains a substring — the ablation switch used by [tools/value](../tools/value/README.md).

## Limits

The tables are only as wide as the seeds; a concept with no seed is unknown to this level. Changing knowledge here means editing seeds and rebuilding.
