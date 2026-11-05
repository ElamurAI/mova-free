# bigbench — exact-arithmetic library benchmark

A measurement-only crate: the same algorithms on `dashu` and on `num-bigint`/`num-rational`, to choose the exact-arithmetic library for `math`. It is not part of the product. The choice and the dependency table are in [math/README.md](../README.md).

```bash
cargo run --release -p bigbench
```
