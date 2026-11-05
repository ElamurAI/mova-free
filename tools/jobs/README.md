# jobs — a heavy-job runner with limits and a log

`jobs` runs one command, samples the whole process tree every second (peak RSS and CPU time from `/proc`), stops only that job when it passes its memory or time limit, and logs estimate against actual.

## Run

```bash
cargo build --release -p jobs
target/release/jobs run --name N [--mem-gb G] [--secs S] [--est-mem-gb E] [--est-secs T] -- command args…
target/release/jobs log
```

| Flag | Default | Meaning |
|---|---|---|
| `--name` | the command | job name in the log |
| `--mem-gb` | 16 | stop when the tree's RSS exceeds it |
| `--secs` | 7200 | stop when wall time exceeds it |
| `--est-mem-gb`, `--est-secs` | — | your estimate, logged next to the actual values |

A stop sends SIGTERM to the tree, then SIGKILL after 5 s; the exit code is then -9. Otherwise `jobs` exits with the command's code.

`jobs log` prints `$MOVA_DATA/runs/jobs.jsonl`: one JSON line per job with `name`, `cmd`, `est_mem_gb`, `est_secs`, `peak_mem_gb`, `cpu_secs`, `wall_secs`, `code`, `reason`.

## Limits

Linux only (`/proc`). Sampling is once per second, so a short spike can pass unseen. `$MOVA_DATA` defaults to `~/mova-data`.
