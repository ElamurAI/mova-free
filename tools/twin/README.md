# mova-twin — prospective cost predictions, measured runs, online calibration

`mova-twin` predicts a workload's wall time from structural features, runs the real command, records the prospective error and updates its coefficients by recursive least squares (ridge around an explicit prior). It models cost only — not answers, correctness or learning. The model and its update equations: [docs/training.md](../../docs/training.md#the-cost-twin).

## Run

```bash
cargo build --release -p mova-twin
# copy tools/twin/example.json and fill in absolute paths
target/release/mova-twin predict SPEC.json [PARENT_RUN]
target/release/mova-twin run SPEC.json NEW_RUN [PARENT_RUN]
target/release/mova-twin report RUN...
```

- `predict` prints the forecast (seconds, observation count, whether the features lie outside the observed box) from the prior or from a parent run's updated state.
- `run` creates `NEW_RUN` (it must be new), copies and hashes the executable, runs it with the spec's arguments, environment and timeout (no shell), checks the declared outputs, and writes `forecast.json`, `observation.json`, `stdout.txt`, `stderr.txt` and the updated state.
- `report` prints one JSON row per run: features, forecast, actual seconds, absolute relative error, accepted, errors; it refuses a run whose evidence hashes changed.

The spec (`schema: 1`) names the family, declared context (hardware, revision, warm/cold), executable, cwd, args, env, the env axes allowed to vary in a cohort, inputs, `feature_source`, features, prior, ridge, timeout and output checks.

## Limits

No forgetting factor and no transfer across changed binaries, data or hardware. The covariance is inverse information, not a calibrated interval. Feature choice is an explicit hypothesis of each experiment.
