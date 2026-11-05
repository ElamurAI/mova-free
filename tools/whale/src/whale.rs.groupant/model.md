# mova-whale-run

## Descriptant

One Whale run: pinned task queue, at most one teacher call per task, contract gates and immutable versioned bundles. Source: `tools/whale/src/whale.rs`. Public, domain-agnostic: concrete prompts, schemas, gate parameters and deployments come from pinned configuration.

## Forward approximant

prepare: load Spec and teacher identity, validate items (kind, format, binding rule, metadata), read pinned export rows, compute admission, extract context, render prompt, write per-task prompt/context/metadata and a manifest with disjoint worker indices (index mod workers). run_with: honor STOP/PAUSE and storage floor, take the worker lease, pick the first own task without receipt, refuse unresolved reservations, withhold local-only items when the kind says so, verify source/prompt/context hashes, create the reservation, call the teacher once, store the reply, then admit (identity check, verbatim guard, projection, validate, publish) into a completed or quarantined receipt. publish: originality on the raw reply, status rewrite, artifacts, staged directory renamed into store/<prefix>/<kind>/<sha(id)>/<version>. project_quarantined_lengths: re-gate retained length quarantines without a teacher.

## Invariant

Per task: at most one reservation and one teacher call; outcomes in {completed, quarantined, uncertain, local_only_no_remote_teacher}; uncertain or unreceipted reservations stop the worker; a published version directory is never overwritten; identity changes require a new run root; original receipts/replies are never modified by projection.

## Cost

Preparation O(sum source bytes + prompts); per task one teacher call plus O(reply) validation and O(corpus) originality; status O(tasks).

## Composition

Uses spec, source, contract, originality and util; launched by pool (workers) and swarm survey lanes (one-item runs).

## behavior_complement

Excluded: calls without teacher/spec configuration, altered template/spec, changed sources, duplicate leases, unknown receipt outcomes, replies from another model/provider, any contract/originality failure (quarantine, no reprompt). Counterexample: a failed transport records uncertain and a second run makes no call. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

A bundle manifest pins task, reply, spec and artifact hashes, so its reply and prompt are recoverable from the retained job directory; a receipt alone admits many replies; a quarantine reason partitions the failing gate but not the reply content. No unique inverse is claimed.
