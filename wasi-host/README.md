# wasi-host — the WASI sandbox (`wasi-run`)

`wasi-host` runs `wasm32-wasip1` modules in a sandbox: the wasmi engine, WASI preview1 over the in-memory file system of [vfs](../vfs/README.md), a virtual clock, seeded randomness and no network. The binary is `wasi-run`.

Its design, commands, the gate and its limits are documented with the file system in [vfs/README.md](../vfs/README.md) (section "WASI host"); the place of the sandbox in Mova is in [docs/context.md](../docs/context.md), "Execution sandbox: the SLM programs, bytecode, an in-memory file system".

```bash
cargo build --release -p wasi-host
cargo test -p wasi-host
```
