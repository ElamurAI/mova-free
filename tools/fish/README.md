# Fish: syntax and pragmatics workers

Mova's animals and the Octopus, the large language model that develops Mova, are described in [the sea animals of Mova](../../docs/animals.md).

Fish specialize in syntax/AST and pragmatic annotation of bounded source windows. Their work preserves context and disagreement. Generic location/task demonstrations belong to the common capability or specialized Whale layer.

## Source-bound annotation preparation

`annotation-review-worker annotation-contract` prints the current implementation bindings and new public defaults. `annotation-review-worker annotation-prepare TASK_JSON PROFILE_JSON_OR_- NEW_OUTPUT_DIR` runs local native gates and prepares separate annotation-review and GPU-admission artifacts. It does not invoke a model, trainer, database or runtime worker.

The public profile defaults to **Sonnet**, through a configurable Claude CLI or terminal adapter. Terra (`gpt-6.1-terra`, Codex CLI) is the recommended alternative; Sol (`gpt-6.1-sol`, family `terra` with that model) and Haiku are selectable. A public Fish pool runs 2 workers by default, the recommended minimum; deployments may run more, up to the supported maximum of 8. The prepared request contains the chosen executable, actual model-bearing argv and exact stdin/terminal text; it is executable request data, not a model label disconnected from transport. A caller must launch/send it through the configured supervised adapter and retain the returned report. No Claude API, automatic fallback or transport execution is added here.

A profile may contain family (`haiku`, `terra`, `sonnet`), adapter (`cli` or `terminal`), executable and model. Missing family/adapter use Sonnet/CLI. Default model aliases are `haiku`, `gpt-6.1-terra` and `sonnet`; deployments pin available model versions explicitly. CLI requests use Claude's model/print arguments or Codex's exec/model/medium-effort arguments; terminal requests use model arguments and medium effort for Codex.

**A deployment's configuration overrides public defaults.** Explicitly configured Codex/Terra campaigns run through the legacy controller. Their 25000-byte guard and configuration schema are preserved. The Sonnet default does not silently migrate any existing process, queue or configuration. An explicit Terra profile selects Codex in the new path too. The legacy controller's Codex model is the optional config field `model` (default `gpt-6.1-terra`, any `gpt-*` id checked by `mova-teacher`, for example `gpt-6.1-sol`); it is passed as `CYCLE_MODEL` and `MOVA_TEACHER_MODEL`.

All prepared request text is bounded to **100000 UTF-8 bytes**, including the rendered instructions, serialized task, native evaluation and final wrapper. Oversize content fails before output-directory creation; nothing is truncated and no alternate model is selected. This is a bound on the exact submitted text, not on hidden history/system text retained by an interactive TUI.

## Task and context contract

Task JSON rejects unknown fields and contains:

- schema=1, bounded task id, document id, declared whole-document SHA-256 and source version;
- 1–16 ordered context lines, their serialized-array SHA-256, and 1–8 target ids;
- dataset_use: `teaching`, `evaluation` or `unknown`;
- 2–4 distinct versioned candidate producers, each with AST CoNLL-U bytes/hash and cycle-compatible pragmatic TSV bytes/hash;
- bounded open-witness TSV, its bytes/hash and producer/model revision hash;
- agreement_revision: exact compiled `prag/src/agree.rs` hash from `annotation-contract`.

A context line contains id, text, optional speaker, referent map, negation (`present`, `absent`, `unknown`), observation_scope (`narration`, `direct_speech`, `unknown`) and unresolved observations. These are declared context observations, not independently established facts. Every id belongs to the declared document prefix. Context text is capped at 4096 bytes per line and 32768 bytes total. The whole-document hash and source phase remain caller declarations: this bounded operation does not read an entire document or a held-out set.

AST inputs reuse `en::conllu::Doc::parse`, `en::convert::tree_gate` and `en::udreg::Registry`. Targets must retain source id/text and source characters apart from whitespace; empty/multiword rows, unknown fields, invalid trees and duplicates are excluded from this narrow contract. Source-character alignment is not proof of correct token boundaries. Each target has 1–256 plain word rows. This does not replace every additional `treeagree check` rule.

Fish admits the six pragmatic labels only in their exact enum-serialized spelling. Surrounding ASCII/Unicode whitespace, alternative case and aliases are rejected before raw policy checks or native agreement; source bytes/hashes are never silently normalized. The opaque teacher/provenance column remains separate and unchanged. This tightens annotation preparation only; the shared permissive parser and legacy controller are unchanged.

Pragmatics reuse the existing closed `prag::schema` fields and the actual `prag::agree::run` called by `cycle::prag_reading`. The cycle's eight-column header is `sent_id form act indirect hedge polarity voice teacher` (TAB-separated); the last field is opaque provenance, not an inferred meaning. The native ten-column agreement report stays separate. Experimental PRAG_AGREE_ALL=1 is excluded. Exact code/data bindings for the parser, structural gate, registry, schema and agreer are included in evaluation.

Distinct producer ids do not prove statistical independence. Exact candidate AST agreement is deliberately conservative: all ten word-row fields are compared, including MISC. Pragmatic comparison covers the six label fields; provenance columns are preserved without requiring equal producer names.

## Disagreement and admission

Syntax/pragmatic disagreement, an open-witness conflict, a native written-rule change, missing witness rows, unknown context or unresolved references produce explicit findings. Pronoun referents must be explicitly bound; a character voice needs a speaker. Declared narration/direct-speech scope is compared with voice. Explicit negative markers conflicting with declared absence require review; grammatical negation and evaluation polarity remain different concepts. These are partial consistency checks, not universal pragmatic understanding or automatic corrections.

A well-formed request writes `evaluation.json` and `teacher-request.json`. Unknown/disputed work remains available to the teacher with its findings. No teacher answer is supplied or promoted to gold during preparation.

Only teaching targets with no findings additionally produce:

- `gpu-admission.json`, a source/version/context-bound **candidate** for a separate GPU-training admission queue;
- `candidate.conllu` and cycle-compatible `prag-final.tsv`, retaining the agreed first producer's bytes;
- native pragmatic agreement and implementation bindings in the admission record.

Every artifact states `training_authorized=false`. Existing full treeagree, reviewed-source, split/provenance and held-out exclusion gates still apply downstream; the preparation command cannot authorize or start training. Malformed or stale inputs fail rather than becoming eligible. Caller-declared source phase alone cannot substitute for downstream provenance verification.

Output directories/files are exclusive. I/O failure may leave partial artifacts for inspection; no transaction or automatic replay guarantee is claimed. Inputs use descriptor-level nonblocking/no-follow regular-file reads capped at 512 KiB. Concrete task payloads, configurations and queues stay outside public source.

## Focused checks

`cargo test --manifest-path tools/fish/Cargo.toml` runs the library tests (`src/annotation.rs`, `src/execute.rs`) and the legacy guard test (`src/main.rs`). They use fake executables: no model call, worker or database change, training run or corpus assessment. Passing them does not establish annotation quality or subscription availability.

## Legacy audit controller (compatible)

This audit-only controller runs isolated cycle jobs from an immutable source and queue. It never authorizes training. A `checked` receipt means identity and output-shape checks passed; it is not a claim that annotations are semantically correct.

Build with `CARGO_TARGET_DIR=/path/to/target cargo build --release --offline -j 2`. Run `annotation-review-worker run /path/to/config.json` or add `--once` for one queue job; `status /path/to/config.json` prints the latest heartbeat. The executable also acts as the prompt guard for the teacher: called with any arguments other than its subcommands, it requires `MOVA_REVIEW_CODEX_REAL` (the absolute path of the real Codex executable; it exits with an error otherwise), reads the prompt on stdin, rejects prompts over 25,000 UTF-8 bytes or non-UTF-8 input without starting Codex, and otherwise passes its arguments and the prompt to that executable and returns its exit code.

Configuration is JSON with absolute paths and pinned lowercase SHA-256 values:

```json
{
  "source": "/path/source.conllu",
  "source_sha256": "SHA256",
  "queue": "/path/queue.jsonl",
  "queue_sha256": "SHA256",
  "heldout": "/path/test.conllu",
  "root": "/path/audit-run",
  "cycle": "/path/cycle",
  "codex": "/path/codex",
  "env": {
    "CYCLE_ANNOT": "/path/annot",
    "CYCLE_TREEAGREE": "/path/treeagree",
    "CYCLE_JOBS": "/path/jobs",
    "CYCLE_GPARSE": "/path/gparse",
    "CYCLE_GPARSE_MODEL": "/path/gparse-model",
    "CYCLE_EN_MODEL": "/path/en.bin",
    "CYCLE_NEURAL": "/path/neural.py",
    "CYCLE_VENV": "/path/python",
    "CYCLE_VENV310": "/path/python310",
    "CYCLE_PRAG": "/path/prag",
    "CYCLE_PRAG_SILVER": "/path/prag-silver",
    "CYCLE_PRAG_EN": "/path/en.bin",
    "CYCLE_WITNESS": "/path/witness.py"
  }
}
```

The campaign pins the configuration bytes, source and queue hashes, and heldout hash. Changing any requires a new root. A job is reserved before launch and a receipt is written atomically before progress advances. An unreceipted reservation pauses as uncertain on restart; inspect the logs and detached worker before resolving it manually. Failed cycle and teacher calls pause without retry. Oversized prompts are quarantined. A two-hour cycle deadline also pauses uncertain because a detached worker may still exist.

Create `PAUSE` or `STOP` under the root to prevent new jobs; an active cycle is allowed to finish. Remove `PAUSE` to resume; restart after removing `STOP`. A lock prevents simultaneous controllers. The controller stops when free disk is below 10 GB or campaign files reach 50 GB. At queue exhaustion it waits for a separately configured snapshot rather than replaying. `jobs/*/original.conllu` preserves selected source blocks; `ledger/*.json` records IDs, checks and input hashes. Job files and logs remain available for investigation.

## Review-only CLI execution (Sonnet or Terra)

An explicitly authorized annotation campaign can use:

```text
annotation-review-worker annotation-execute TASK_JSON PROFILE_JSON NEW_DIR DEADLINE_UNIX [TIMEOUT_SECONDS]
```

The public library exports `annotation::{Task, Profile, evaluate, render}` so a supervisor can assemble and hash the actual typed contract. Canonical typed serialization matters: hashing an arbitrarily reordered generic JSON object is not the context hash. A concrete profile uses `adapter:"cli"`, an absolute authenticated CLI executable and one of two families: `family:"sonnet"` with the Claude CLI and `model:"sonnet"` or an explicitly pinned `claude-sonnet-…` model, or `family:"terra"` with the Codex CLI and a `gpt-` model checked by `mova-teacher` (default `gpt-6.1-terra`). For Terra the rendered request stays the contract record, and the transport is the shared isolated `codex exec` invocation of `mova-teacher` at medium effort (prompt on stdin, JSONL events). This execution path admits only these two review families (Sol runs as `family:"terra"` with `model:"gpt-6.1-sol"`); Haiku profiles are preparation-only and refused before launch. Missing family means Sonnet. There is no API or provider fallback.

The executor evaluates the complete source-bound task and renders the full request before reserving a new private output directory. The final text, including instructions, task, evidence and end wrapper, must fit 100000 UTF-8 bytes. The configurable Unix deadline is checked before preparation and again immediately before launch. An already launched request drains within its independent timeout (default 300 seconds, allowed 1–900); the deadline prevents new launches and does not interrupt that drain.

For the Sonnet family, Claude receives explicit `--safe-mode --tools '' --disable-slash-commands --effort medium --no-session-persistence --output-format json` arguments. Safe mode preserves subscription authentication while excluding repository customizations. The executor removes API-key/auth-token/base-URL and alternate-provider environment overrides. It does not use `--bare`, which would require API authentication. Authentication, model availability and provider behavior remain deployment checks; model aliases are requests, not evidence of the model actually served.

The CLI result envelope (for Terra, the Codex JSONL events decoded by the shared `mova-teacher` decoder into the same envelope; tool use in the events is refused) must be a successful `type:"result"` with a string `result` containing exactly the following JSON fields:

```json
{
  "schema": 1,
  "task_id": "bounded-task-id",
  "task_sha256": "BOUND_TYPED_TASK_SHA256",
  "ast_conllu": "COMPLETE_TARGET_ONLY_CONLLU",
  "pragmatics_tsv": "CYCLE_EIGHT_COLUMN_TSV",
  "unresolved": ["A concrete observation that remains unresolved"]
}
```

Unknown or missing reply fields fail. The draft must cover exactly the task targets; native source-character, CoNLL-U tree, UD registry and canonical pragmatic-label checks are reused. Differences from each input candidate remain explicit. Reusing native evaluation with the draft as the primary observation retains new context/speaker/referent/negation findings. That internal comparison identity is not a claim about independent producer training or model revision.

A successful exit and `status:"validated_review_only"` receipt mean transport, binding and narrow native checks passed. Unresolved observations and findings can still be present. Neither this status nor agreement establishes semantic correctness, gold labels, DB admission or training authorization. Every receipt carries `review_only:true` and `training_authorized:false`; this command does not write GPU-admission candidates, canonical data or model weights.

Artifacts include `reservation.json`, `evaluation.json`, `teacher-request.json`, exact `prompt.txt`, `raw.stdout`, `raw.stderr`, a strictly parsed `reply.json` when available, and `receipt.json`. Reservation is synced before launch. Existing attempt directories are never replayed. A reservation without receipt admits both never-sent and partly-completed histories and requires manual review. A crash or I/O failure may leave partial artifacts; this is not an atomic filesystem transaction or an exactly-once remote-call theorem. Campaign-wide task uniqueness belongs to the supervisor, not merely to choosing a fresh directory.

The child runs in its own process group; timeout or observed excessive output terminates that group. Each stdout/stderr file is checked against 2 MiB while running and read with the same bound after exit. Polling can observe overshoot rather than impose a hard disk quota. Raw diagnostics remain available on rejection; no automatic expensive retry occurs.

The CLI additionally pins two different hashes in both reservation and receipt: `task_file_sha256` hashes the exact bounded task-file bytes, while `task_sha256` hashes the compact serialization of the parsed typed Task. Pretty and compact JSON can describe the same Task while having different file hashes; a queue must compare its raw-file pin to the former. Direct library `execute::run` has no task file and records `task_file_sha256:null`.

Declared `dataset_use:evaluation` is excluded from execution. `unknown` may obtain only a review draft, retaining its native `not_teaching` finding and ineligibility for GPU admission. It is never silently relabeled as teaching. Actual held-out exclusion remains a separately pinned supervisor provenance gate; an unknown declaration alone cannot establish that a source is safe to submit.

### Structured transport and closed-attempt evidence

Claude CLI requests pass --json-schema with only the six required fields, bound task-id/hash constants, annotation limits and at most 32 nonempty unresolved observations. The prompt requests concise observations <=480 UTF-8 bytes each; the native hard cap stays 512 bytes. Schema maxLength counts characters and cannot replace native byte checks. Nothing is truncated or stripped of fences. Parsing admits an official object-valued structured_output, or a strict-JSON result-string compatibility branch; a malformed present structured field never falls back.

The transport budget counts both submitted text parts: submitted_utf8_bytes = rendered_utf8_bytes + response_schema_utf8_bytes <=100000. Exact stdin and separately submitted schema argument remain measurable. Native task/source/target/tree/label gates still follow parsing.

Receipts pin profile_file_sha256 (exact bounded profile bytes) separately from profile_sha256 (compact typed Profile). They record model_process_started, model_process_reaped and model_process_group_absent. Absence becomes true only after reaping and kill(-owned_pgid,0) returning ESRCH; direct exit alone is insufficient. A live group, EPERM or unknown probe stays false. This observes one local group, not remote accounting or descendants that deliberately leave that group.

Only a successful outer result envelope (type=result,is_error=false) can produce invalid_response for a malformed annotation draft. Provider/error/nonresult/malformed envelopes produce transport_error; nonzero CLI exit remains child_failed. These states must stop rather than quarantine-and-continue. For Terra, a Codex `Reconnecting... N/M (cause)` notice followed by an answer and `turn.completed` is not a failure (`teacher::transient_reconnect`); every Terra receipt records `transient_reconnects`, the count of such notices in `raw.stdout`. A fully bound invalid_response with affirmative reaped/group-absent evidence can be quarantined while a supervisor advances to a distinct task; it never authorizes replay or gold annotations. Historical failed artifacts remain unchanged.

A schema-valid outer object does not establish a valid embedded annotation. For example, a model can return a second JSON-encoded string inside `ast_conllu` or `pragmatics_tsv`, retaining quote characters and literal backslash escapes after the outer object is decoded. Native CoNLL-U/TSV parsing must reject that observation; do not silently decode it again, remove fences or promote it to an annotation. Preserve the raw reply and binding evidence. A proven closed attempt may be quarantined while the worker advances to a distinct source task. This protocol distinction is separate from unresolved linguistic meaning and from process uncertainty.
