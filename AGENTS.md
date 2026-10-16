# AGENTS.md — Mova

These instructions are for AI agents (and people) working on Mova.

**First read `docs/mysticism/gali.md` and enter the mode.** It is a working mode for an AI that builds with a human operator. Do not just skim it: step in and work.

---

## What Mova is

Mova is a small, transparent language model written in Rust. English comes first. It parses into Universal Dependencies trees, reads the trees into frames and a world of the text, and answers from that world. Every step is deterministic and can say *why*. A large model is only a teacher: it never answers at test time. Start with `README.md` and `docs/architecture.md`.

## Working rules

- **English everywhere** in code, comments, docs, commit messages and data. Talk to the operator in their language.
- **Mechanics in Rust.** Code chews the data, not your tokens. Avoid bash or Python improvisation; the few Python bridges, to the neural parsers and the pragmatic witnesses, are the exception.
- **Compose from what exists.** Grep the repo before you write new code.
- **Deterministic.** Use `BTreeMap`/`BTreeSet` wherever order decides anything. Before you claim a gain, run twice and check the two runs are identical.
- **Measure before you claim.** Keep held-out sets held out: the SVAMP test runs once per release, and decisions are made on the two-fold check (`cv`). State every number with its date.
- **Negative control.** A gate that cannot show red is not a gate.
- **Fail-fast.** No blind retries on expensive steps. New directions and expensive runs (large-model batches, long trainings) need the operator's explicit word.
- **Two execution modes** (`docs/gpu-exec.md`). **CPU only** is the default and is exact. **GPU enhanced** needs `--features gpu` and `MOVA_EXEC=gpu`. A GPU stage is exact (gated byte for byte) or an approximant (its agreement measured and reported).
- **Licences of data** (`licence/DATA.md`):
  - our own code, models and data: the Elamur Cosmic License (`LICENSE.md`);
  - NC and research-only data: only for tests and measurements, never in what is built;
  - CC BY-SA: as little as possible. Quote it, test on it, learn from it, but never carry it or modify it into a release.
- **Data lives outside the repo**, in `$MOVA_DATA`, and so do run results. The repo holds code, rules, seeds and small data.
- **Parallel agents work in their own worktrees.** Use `git worktree add <scratch>/<task> -b <task> main`, with a separate `CARGO_TARGET_DIR`. Crates depend on `en` by path, so an edit in a shared copy breaks the others' builds. The main instance merges.
- **Commits** go in logical steps, after `./secret-guard.sh`. It looks for API-key patterns in the staged diff and, if you have one, for the values listed in `local.md`: an optional, untracked, local file of strings that must never be committed. Contributors may create their own; it is never committed.
- **Knowledge goes into the repo** (`docs/`, crate READMEs). Keep docs plain: what it is, how it works, how to run it, measurements, limits.
- **Nothing private in the repo.** No private paths, personal names or secrets in code, docs or data.

---

Local extensions: if `tera/AGENTS.md` exists in your checkout, read it too; it takes precedence, and `tera/` is then the only place for private material, which is never published. Public checkouts do not have it.
