# mova-teacher — the shared Codex teacher transport

`mova-teacher` is the one way Mova's training code calls a teacher model: the Codex CLI (`codex exec`), authenticated with a ChatGPT subscription login. There is no Claude API route and no fallback to one. See [docs/training.md](../docs/training.md), "Current teacher transport".

## What it provides

- `model(COMPONENT_VAR)` — the teacher model: `MOVA_TEACHER_MODEL` if set, else the component's own variable, else `DEFAULT_MODEL` = `gpt-6.1-terra`.
- `validate_model(m)` — refuses any model whose name does not start with `gpt-` (legacy Claude requests must be prepared again).
- `ask(cwd, model, effort, prompt)` — one bounded call, no automatic retry. The prompt goes through stdin. Effort is `low`, `medium`, `high` or `xhigh` (callers default to `medium`). Each call runs in its own temporary directory under `cwd`, with a read-only sandbox, user config, rules, web search and every tool feature disabled. A line per call is appended to `cwd/codex-calls.jsonl` (model, effort, seconds, token usage).
- `exec_args(model, effort)` — the same isolated `codex exec` arguments, for callers that run Codex themselves (the Fish review execution in `tools/fish`).
- `decode_events(jsonl)` — turns Codex JSONL events into one answer with token usage. A failed turn, a tool use, a repeated or incomplete turn, or an item after completion is an error, never a training answer.
- `transient_reconnect(message)` / `transient_reconnects(jsonl)` — the one neutral error: Codex's stream-retry notice `Reconnecting... N/M (cause)` (exact Codex CLI 0.165.0 shape, 1 <= N <= M, no limit/quota text) before `turn.completed` is skipped by `decode_events`, so a turn that recovered and completed with an answer decodes normally. A reconnect followed by `turn.failed` or no completion, a notice after completion, and every other error stay errors. `transient_reconnects` counts notices for receipts.
- `CLEARED_ENV` — `ANTHROPIC_API_KEY`, `OPENAI_API_KEY`, `CODEX_API_KEY`, `OPENAI_BASE_URL` are removed from the child environment, so only the subscription login is used.

## Environment

| Variable | Default | Meaning |
|---|---|---|
| `MOVA_TEACHER_MODEL` | — | overrides every component's model |
| component variable | — | `ANNOT_MODEL`, `PRAG_MODEL`, `MATH_MODEL`, `ONTO_MODEL`, `CYCLE_MODEL`, `ENRICH_MODEL`, `MACULA_MODEL`, `BOOK_ANSWER_MODEL` |
| `MOVA_TEACHER_TIMEOUT_SECS` | 1800 | per call; a timeout kills the process group and is not retried |
| `MOVA_CODEX_BIN` | `codex` | the Codex executable |

Setup: install the Codex CLI and run `codex login --device-auth`.

## Users

`annot`, `prag` (and `qagen` through it), `math`, `onto`, `world`, `tools/cycle`, `tools/enrich`, `tools/macula`, `tools/whale`, and `tools/fish` (`exec_args` only).

## Tests

```bash
cargo test -p mova-teacher                       # decoder and argument tests, no call
cargo test -p mova-teacher -- --ignored          # one live call; needs the Codex login
```

## Limits

- Subscription dollar cost is not reported: replies carry `total_cost_usd: 0.0` and `cost_reported: false` for compatibility with older log parsers.
- Output is capped at 32 MiB per stream.
