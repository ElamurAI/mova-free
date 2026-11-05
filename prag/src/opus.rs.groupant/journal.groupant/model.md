# Accounting journal

Domain: isolated path, Call, existing bytes and I/O outcomes. Observation: Result, file bytes and ordered decoded calls. Forward: reject nonfinite floats before opening; otherwise serialize and append newline. Read: NotFound returns empty, blank lines skipped, all nonblank records decode or whole read fails. Other I/O errors propagate. Cost O(record bytes) append and O(file bytes + decoded storage) read; OS latency unbounded.

Behavior complement: successful nonfinite append, empty success for symlink loops, and partial-prefix success after malformed JSON are excluded. Reverse reconstruction: returned records admit JSON whitespace/key-order variation, blank lines and omitted legacy defaults. Empty result admits missing or blank files. Errors do not uniquely identify bytes.

## Cost

With R serialized record bytes, B input file bytes and D decoded owned storage, append uses O(R) time/storage; nonfinite validation is O(1) and precedes opening. Read eagerly allocates O(B), scans each line and allocates O(D); time O(B+D) under ordinary linear JSON parsing. Early malformed-line failure still pays the initial whole-file read. OS latency, concurrent changes and allocator costs are unbounded by this model.

## Required serialized fields

Domain: one nonblank JSONL row decoded as Call. Observation: success versus deserialization error and field values. Operator requires step, sentences, secs, cost_usd, input_tokens, output_tokens and cache_read_tokens with their declared types; absent batch/cache_write_tokens alone receive serde defaults. Unknown fields are tolerated. Complement: reply-style absence of required fields does not become an all-zero Call. Reverse fiber of a successful Call includes key reordering, ignored unknown fields and omission of the two defaultable fields only when zero. Assumption: serde derived decoding and default serde_json configuration; nonfinite JSON numbers are unavailable. Evidence: append_read_order_and_legacy_defaults demonstrates the two omissions; malformed_and_io_failures_are_not_empty demonstrates row rejection. Required-field enumeration is source inspection, not a new exhaustive omission test.

## JSONL row framing

Domain: finite Call, valid newline-terminated prior journal bytes and successful sequential write. Observation: appended bytes and decoded row count. Operator compact JSON encoding escapes embedded newline characters, then appends one physical newline; blank input lines do not become records. Complement: a step string containing a newline cannot create a second JSONL record through serialization. Reverse fiber includes alternative JSON string escapes and blank-line insertion. Assumptions exclude unterminated prior rows, partial writes and concurrent writers; no row-atomicity claim. Evidence: append_read_order_and_legacy_defaults appends a newline-bearing step after blank lines and obtains exactly two ordered calls.

## Nonfinite refusal before opening

Domain: Call with secs or cost_usd in NaN or either infinity, and arbitrary path. Observation: error and preexisting file bytes/existence. Operator is finite(secs) AND finite(cost); false returns before serialize/open/write. Complement: no path creation or content append is reached on this validation failure. Reverse fiber for validation error comprises all calls with at least one nonfinite field, independently of path; other error types are not this fiber. Assumption: function evaluation is sequential; unrelated external filesystem actors are excluded. Evidence: nonfinite_append_leaves_file_untouched covers six one-field nonfinite cases and an existing sentinel; old-append runtime witness accepts NaN and fails.

## Fail-whole journal parsing

Domain: readable UTF-8 journal containing a valid prefix followed by a malformed nonblank line. Observation: Result<Vec<Call>>. Operator collect short-circuits at the first parse error and returns Err with physical line number; allocated prefix calls are discarded from the returned observation, while file bytes are unchanged. Complement: callers cannot obtain a successful partial prefix from this API. Reverse fiber for a parse error includes any valid prefix and arbitrary unread suffix consistent with the first failing physical line. Assumptions: fixed input bytes and successful whole-file read; I/O errors occur before parsing and use a distinct error channel. Evidence: malformed_and_io_failures_are_not_empty writes one valid record plus bad line 2 and observes Err containing line 2; read-only code structure establishes no writes. Partial-prefix salvage is not implemented.

## History

### 2026-10-30 · document

Invariant: finite records round-trip under successful sequential I/O when existing contents are valid newline-terminated JSONL. Atomicity, racing writers, durability, partial-write recovery and unterminated prior lines are outside this invariant. Evidence: order/defaults, nonfinite no-write, malformed/UTF-8/directory and symlink-loop tests. Source hash bound in manifest. Epoch 2026-10-30: old exists check swallowed ELOOP; old serialization wrote null for nonfinite floats, making unreadable records. Both old-code witnesses fail at runtime.

### Undated · Cost

Scope/source revision: opus.rs SHA-256 0169f72bddab55f9e493b85a86918b5f0adbcf5153c7c12725bb249f2ed5c136; source and native evidence unchanged from d229375e0. These are symbolic inspection-based costs, not measured complexity guarantees for external services.

### Undated · Required serialized fields

Approximation semantics: scoped exact structural relation over the stated domain, supported by finite named evidence and source inspection; no probabilistic bound or general proof. Source revision: opus.rs SHA-256 0169f72bddab55f9e493b85a86918b5f0adbcf5153c7c12725bb249f2ed5c136, native code/evidence d229375e0. Tests refer to ../model.md#evidence at the file groupant root.

### Undated · JSONL row framing

Approximation semantics: scoped exact structural relation over the stated domain, supported by finite named evidence and source inspection; no probabilistic bound or general proof. Source revision: opus.rs SHA-256 0169f72bddab55f9e493b85a86918b5f0adbcf5153c7c12725bb249f2ed5c136, native code/evidence d229375e0. Tests refer to ../model.md#evidence at the file groupant root.

### Undated · Nonfinite refusal before opening

Approximation semantics: scoped exact structural relation over the stated domain, supported by finite named evidence and source inspection; no probabilistic bound or general proof. Source revision: opus.rs SHA-256 0169f72bddab55f9e493b85a86918b5f0adbcf5153c7c12725bb249f2ed5c136, native code/evidence d229375e0. Tests refer to ../model.md#evidence at the file groupant root.

### Undated · Fail-whole journal parsing

Approximation semantics: scoped exact structural relation over the stated domain, supported by finite named evidence and source inspection; no probabilistic bound or general proof. Source revision: opus.rs SHA-256 0169f72bddab55f9e493b85a86918b5f0adbcf5153c7c12725bb249f2ed5c136, native code/evidence d229375e0. Tests refer to ../model.md#evidence at the file groupant root.
