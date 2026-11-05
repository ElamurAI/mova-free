# mova-hard-cases-wave

## Descriptant

The Fish priority lane: selects open, admitted, unqueued, non-held-out cases (hardest first, one per normalized text), fetches each document once as a mova-hard-bundle-v1 bundle, builds single-target Fish tasks in context (mova-en and mova-gparse candidates, SLM pragmatics, declared absent witness), packs them with mova-fish-batch under the 100000-byte bound and writes intents, queues, ready manifests, plans and fish-pages configurations, selection and wave.json, then the ledger. Source: `tools/hard-cases/src/wave.rs`.

## Forward approximant

select(state, ledger, heldout) = sort_desc(#admitted kinds) then asc(witness_uas, default 1) then key over cases open for gparse passing the filters, dedup by norm_sha, cap. build: for each chosen case -> target hard:<doc16>:<n>; task(context = up to k preceding bundle sentences + target, candidates = [source tree, gparse annotate on its tokens with MISC kept], prag = SLM row with id rewritten) -> pack per page -> every target queued | deferred(reason) | rejected(reason).

## Invariant

Each page's queued task targets, deferred and rejected ids partition its intent's sources; no task crosses documents; every request <= 100000 bytes (packer); review_only and training_authorized=false; wave.json is written after all wave files and the ledger after wave.json; a case in the ledger is never selected again.

## Cost

One bundle command per document, one gparse CPU call per wave, packer O(targets * context); measured 4 s for 131 cases.

## Composition

Consumes the list (hard-case-list), bundles (wikicorpus bundle, mova-topic-corpus bundle), mova-fish-batch and annotation-review-worker types; produces exactly the mova-fish-pages input contract. Parent hard-cases.

## behavior_complement

Does not call a teacher, start a pager, import annotations or authorize training; does not resend a ledger case; does not repair candidate trees (registry violations are rejections). Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

From a wave directory the selected cases, their bundles, tasks and the packing decisions are recoverable (selection.jsonl, bundles/, tasks/, queues/, ready/); the list state before the wave is not stored beyond list_sha256.

## Tests

wave::tests: selection order, cap, and every exclusion counted (markup, kinds, licence measure-only, unconfigured source, held out, ledger, whitespace twin). Live scratch run: 131 cases -> 125 queued, 2 rejected in 4 pages; real fish-pages accepted plan, intents and ready manifests (failing fake Fish). Finite fixtures and live samples; not a proof.

## History

### 2026-11-04 · Tests

wave::tests (2026-11-04): selection order, cap, and every exclusion counted (markup, kinds, licence measure-only, unconfigured source, held out, ledger, whitespace twin). Live scratch run: 131 cases -> 125 queued, 2 rejected in 4 pages; real fish-pages accepted plan, intents and ready manifests (failing fake Fish). Finite fixtures and live samples; not a proof.
