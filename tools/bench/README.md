# bench — one row of fixed measurements

`bench` runs a fixed set of measurements with no LLM call (frozen sets and cached verdicts) and appends one row — time, commit, metric after metric — to a TSV table. Comparing an idea: build the branch, run `bench`, compare the row with the previous one. [tools/value](../value/README.md) uses it for ablations.

## Run

```bash
cargo build --release -p bench -p world -p math
target/release/bench [--quick]     # --quick skips GSM8K, the longest measurement
```

Measurements (each a release binary under `$MOVA_ROOT/target/release`):

| Columns | Command |
|---|---|
| `svamp_%`, answered right/wrong, world-reader agreement | `mathsolve steps --set svamp --exact --epochs 20 --beam 32 --rerank 5 --world <scripts>` with `MATH_NOLEX=1` |
| `gsm8k_%` (not with `--quick`) | `mathsolve steps --set gsm8k …` |
| `ftqa_rougeL` | `world read-events` (FairytaleQA test) |
| `tales_EM`, `tales_F1`, `tales_pairs_EM` | `world read-train` on four tale books with their questions |
| `fables_verdicts`, `fables_gaps` | `world values` on three fable command files |
| `<name>_s` | seconds per measurement |

## Environment

| Variable | Default | Meaning |
|---|---|---|
| `MOVA_ROOT` | `~/Mova` | the checkout whose binaries run |
| `MOVA_DATA` | `~/mova-data` | inputs from dated runs under `$MOVA_DATA/runs/` |
| `BENCH_OUT` | `$MOVA_DATA/runs/bench.tsv` | the table; rows are appended, and a new header is written when the column set changes |

## Limits

- The inputs are fixed dated run directories under `$MOVA_DATA/runs/` (2026-09-26 and 2026-09-30); without them a column reads `?`.
- Numbers are taken from output lines by prefix; a changed output line gives `?`, not an error.
- The SVAMP column measures the SVAMP test split. Decisions are made on the two-fold `cv` check, and the test runs once per release (see [AGENTS.md](../../AGENTS.md)); do not use `bench` rows to tune.
