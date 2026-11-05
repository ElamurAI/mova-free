# value — what a component is worth, by ablation

`value` builds a copy of the system without one component, runs the same [bench](../bench/README.md) measurements in the copy and in the main checkout, writes a report, and deletes the copy.

## Run

```bash
cargo build --release -p value -p bench
target/release/value <name> --exclude <substring of seed paths>[,…] [--env K=V]… [--quick]
```

1. A git worktree `value-<name>` at `$MOVA_DATA/work/value-<name>` (its own `target`).
2. `global`, `math` and `world` are built there with `GLOBAL_EXCLUDE=<substrings>` (the matching seeds are not compiled in) plus any `--env K=V` that switches a module off.
3. `bench` (`--quick` passed through) runs in the copy and in the main checkout.
4. The report `$MOVA_DATA/runs/value/<name>.md`: metric, with, without, delta, and how many metrics the component helps or hurts. Timing columns (`*_s`) are left out.
5. The worktree, its branch and `$MOVA_DATA/runs/value-<name>` are deleted; only the report remains.

A report is a measurement of one run: keep it with the run's notes, not as a documentation page.

## Limits

- The main checkout is `~/Mova` and the `bench` binary is taken from `~/Mova/target/release`; both are fixed in the code.
- A `value-<name>` worktree or branch left from an earlier run is removed first.
- The verdict counts metrics; for gap and error columns lower is better, so read each delta by its metric.
- It inherits `bench`'s limits, including the SVAMP test column.
