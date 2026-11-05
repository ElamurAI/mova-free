# mova-terminal

An animal's private LLM terminal. Every Mova animal instance that needs a large language model (a Whale, the Nautilus, a Sea Turtle, ...) holds its own interactive LLM session in its own tmux session. The code is shared; the AI is not: each terminal has its own owner marker, lock, task directory, answer directory and journal, nothing is shared between animals, and an animal never uses a Dolphin as its terminal (see [docs/animals.md](../../docs/animals.md)).

The session is held by `crab-tmux-worker` (`tools/crab`), which Crab starts like any other worker. This crate only asks: one bounded question at a time. The answer is evidence for the animal to read; it is never executed.

## Ask protocol

1. **Lock.** `ask` takes the exclusive lock `ROOT/terminal.lock`; a second asker is refused.
2. **No replay.** While an earlier question has no answer file, `ask` refuses. A question is never re-sent; an operator may `abandon` it.
3. **Checks before sending.** The pane is observed: exactly one window and one pane, the configured `pane_command`, the session option `@mova_crab_owner` equal to `owner_marker`, no working text (`esc to interrupt`, ...) or modal text (`do you want to`, `trust this`, `sign in`, ...) in the bottom 12 non-empty lines, and the idle marker among the last `idle_window` non-empty lines. States: `idle`, `busy`, `modal`, `absent`, `mismatch`. Anything but `idle` sends nothing.
4. **Unchanged check and bracketed paste.** The task file `ROOT/tasks/ID.md` is written and the journal records `reserved`; the task is loaded into a tmux buffer, the pane is observed again (it must still be idle and its screen unchanged), then the buffer is pasted bracketed (`paste-buffer -p`) and Enter is pressed. A changed screen journals `abandoned` and sends nothing.
5. **Answer file.** The task tells the LLM to write its complete answer, as plain text, to `ROOT/answers/ID.txt` and then print the idle marker. `answer` reads that file (a regular file, at most `answer_max_bytes`) and journals `answered` once, with the answer's SHA-256.
6. **Journal.** `ROOT/journal.jsonl` is append-only and fsynced: `reserved`, `sent`, `answered`, `abandoned`, each with the question id and time; question and task hashes, not texts.

## Profile

A JSON file (unknown fields are refused; all paths absolute):

| Field | Meaning |
|---|---|
| `animal` | The owning animal instance, e.g. `whale-1` (identifier, at most 64 characters). |
| `session` | The tmux session held for it. |
| `tmux` | Path of the tmux binary. |
| `socket` | Optional tmux server socket name (`-L`). |
| `pane_command` | `pane_current_command` of the live CLI: `node` for Codex, `claude` for Claude Code. |
| `owner_marker` | Value of `@mova_crab_owner` the holder set (`crab-tmux-worker --owner-marker`). |
| `idle_marker` | The line the LLM prints when ready, e.g. `TERMINAL READY`. |
| `idle_window` | Last non-empty lines searched for the marker (default 4, 1..=20). |
| `root` | Task, answer and journal directory; the LLM must be allowed to write `ROOT/answers`. |
| `question_max_bytes` | Default and maximum 100000. |
| `answer_max_bytes` | Default 400000, maximum 4000000. |
| `answer_timeout_seconds` | `ask --wait` limit, default 1800. |

## CLI

```text
mova-terminal status PROFILE
mova-terminal ask PROFILE [--wait]        (question on stdin)
mova-terminal answer PROFILE ID
mova-terminal abandon PROFILE ID REASON
```

Each prints one JSON line. `ask` prints the question id and answer path; `ask --wait` waits for the answer file and the idle marker and prints the answer. A timeout leaves the question pending; it is never re-sent.

## Holding a terminal

The role prompt is `prompts/animal-terminal.md` for a consultant the animal asks while doing its own work (it will not do that work for the animal), or `prompts/animal-teacher.md` for a teacher whose answers are review-only proposals (the Nautilus's senses, the Sea Turtle's lessons); the holder appends it to the command line with `--prompt-file` and `--var name=ANIMAL --var idle_marker=MARKER`. The session's working directory is the profile `root`.

**Opus (Claude Code), the public default.**

```text
crab-tmux-worker --lock LOCK --owner-marker MARKER --prompt-file .../animal-terminal.md \
  --var name=whale-1 --var idle_marker="TERMINAL READY" SESSION ROOT -- \
  claude --model opus --permission-mode acceptEdits --disallowedTools=Bash,WebFetch,WebSearch
```

Use the `=` form of `--disallowedTools`: the flag is variadic and would otherwise swallow the role prompt. Profile: `pane_command` `claude`. The first start in a new root shows Claude Code's folder-trust dialog; the terminal reports it as `modal` and sends nothing. Accept it once per root.

**Sol (Codex CLI), an alternative.**

```text
... SESSION ROOT -- codex -m gpt-6.1-sol -c model_reasoning_effort="medium" -s workspace-write -a never
```

Profile: `pane_command` `node`, `idle_window` 6 (Codex prints its footer below the marker).

## Tests

`cargo test --manifest-path tools/terminal/Cargo.toml --release --locked`: unit tests on screen classification and profile validation, and end-to-end tests against a fake LLM in a real tmux server on a private socket (tmux required).

## Limits

- The state is read by screen scraping of interactive TUIs: a CLI update that changes working, modal or footer text can misclassify the pane. The idle marker and the unchanged-screen check fail closed (nothing is sent), not open.
- One question at a time per terminal; no queue inside this crate.
- No streaming: the answer exists only once the LLM has written the answer file.
- The answer file is trusted only as far as the LLM's own write: its size and file type are checked, its content is not. It is evidence, never executed.
