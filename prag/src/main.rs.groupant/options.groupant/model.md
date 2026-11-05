# prag-main-options

## forward
Scan tokens left to right: --key consumes exactly the next token (even another --token); every other token joins the positional vector. Duplicate keys overwrite. num parses usize or returns the supplied default only for absent keys.

Domain: finite supplied valid token/tree/label data or isolated saved run files as scoped above. Observation: returned values, named errors, serialized TSV/JSON and public path composition. Operator: the scoped production functions at the manifest source hash. Approximation semantics: partial source-derived behavioral relation, exact on specified fixtures; no numerical approximation cap.

## invariant
Positional order retained; successful map contains last consumed value per key. Missing final option value and invalid usize return errors; zero is valid.

## cost
O(total token bytes + K log D) time; O(retained bytes) memory, K options and D distinct keys.

## behavior-complement
First-wins duplicate outputs, consumed values reappearing as positionals and defaults replacing invalid numbers are excluded. Unknown keys and interleaved options are intentionally accepted. Universe: scoped inputs paired with candidate observations. This inner exclusion model concerns code behavior, not just test failure; outside the stated assumptions remains unknown.

## reverse-reconstruction
Fiber consists of all scans with the same positional sequence and final values, allowing overwritten occurrences and compatible interleavings; numeric parsing further identifies decimal spellings and default-versus-explicit equality. Target relation is {(y,x): F(x)=y} within the domain; no implemented complete preimage solver or injectivity claim.

## composition
Preserve shared schema class, source voice, document identity/order and transport descriptor across producer/consumer boundaries. File headers and filtering wrap delegated library computation. Test fragments use public APIs; source module exports alone establish availability only.

## checks
Five binary and two library wave17_models tests exercise literal options/numbers, Unicode segmentation, supplied CoNLL-U role positions, restricted/full world equality, saved-call metadata, schema/serialization/gate and public role/world wiring. Debug and release are each repeated twice; named outcomes compared. Main duplicate-key first-wins mutation and library transport substitution are compiled rejection controls. See file-root ../model.md#receipts.

## epoch-memory

Prospective inspection identified false Claude provenance in gate_report. The old binary fails the Codex transport assertion; repair centralizes descriptor without renaming opus APIs. Compatibility accepts interleaved options, duplicate overwrite and --prefixed values; this behavior is intentionally preserved. No optimization or task-score gain claimed.

## limitations
Supplied fixtures are finite; no proof of linguistic correctness, complete exported API coverage or live dispatch. Environment-derived model/effort and role-alone selection are not varied. Syntax projection omits macro bodies and semantic resolution; groupant source hash binds exact bytes.

## History

### 2026-10-30 · epoch-memory

2026-10-30: prospective inspection identified false Claude provenance in gate_report. The old binary fails the Codex transport assertion; repair centralizes descriptor without renaming opus APIs. Compatibility accepts interleaved options, duplicate overwrite and --prefixed values; this behavior is intentionally preserved. No optimization or task-score gain claimed.
