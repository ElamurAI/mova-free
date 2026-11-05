# mlab-backend-bench — linear-algebra backend comparison

A measurement-only crate comparing `faer` and `nalgebra` (1000×1000 multiplication and solve, 500×500 eigenvalues), single thread, best of 3. It prints a TSV table of timings and the residual checks of both backends. Why `mlab` uses faer is in [mlab/README.md](../README.md).

```bash
cargo run --release -p mlab-backend-bench
```
