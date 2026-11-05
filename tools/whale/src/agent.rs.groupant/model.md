# mova-whale-agent

## Descriptant

One bounded agent session in an isolated Git worktree with configured prompt, scopes and protected files. Source: `tools/whale/src/agent.rs`. Public, domain-agnostic: concrete prompts, schemas, gate parameters and deployments come from pinned configuration.

## Forward approximant

parse: validate model/effort, absolute paths, full commit, branch under branch_prefix, pin counts, write directories (pinned source sidecars or pinned scope manifest directories), write files (pinned, matching write_file_rule). run: job operation lock, cached report or refuse unresolved reservation, render template (all slots once), verify pins at base, refuse non-new worktree/inside-repo/retention cap/storage floor, reserve, add worktree branch, run the transport (workspace-write sandbox, network off, credentials cleared, timeout/output limit -> uncertain), finalize (HEAD unchanged, pins, protected blob hashes, changed paths inside scopes, staged guard, patch and candidate commit), optionally archive the clean worktree, write report.

## Invariant

At most one agent call per job identity; any report is final (cached); production files with protected suffixes outside scopes are byte-identical in admitted candidates; nothing is merged or pushed; completion is not test evidence.

## Cost

One external agent session bounded by timeout and 8 MiB output; Git observations O(tree) for protected hashing.

## Composition

Launched by swarm agent lanes as `whale-research CONFIG`; standalone via CLI.

## behavior_complement

Excluded: writes outside scopes, symlinks, escaped or oversized files, protected byte changes (including assume-unchanged tricks), HEAD changes, guard rejection, altered template, forged scope pins, mismatched scope manifest fields, non-Codex models, branches outside the prefix. Counterexample: a hidden assume-unchanged edit to a protected file is detected by raw blob hashes. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

From a candidate commit and patch one reconstructs the exact changed files on the pinned base; from a quarantine reason only the failing gate is known; uncertain sessions admit several partial edit histories and are never replayed. No unique inverse is claimed.
