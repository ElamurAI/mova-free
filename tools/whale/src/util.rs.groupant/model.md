# mova-whale-util

## Descriptant

Shared bounded file, hash, pin, lease, process-identity and storage helpers. Source: `tools/whale/src/util.rs`. Public, domain-agnostic: concrete prompts, schemas, gate parameters and deployments come from pinned configuration.

## Forward approximant

Pure helpers: SHA-256 of bytes/files; bounded reads (refuse > max); JSON save via synced temp+rename; create_new JSON/bytes; owned 0700 private directories; nonblocking flock leases (`try_lease` -> None when held); `Operation` mutex that explicitly unlocks on drop; /proc start-tick PID identity and liveness; statvfs free-space check; UTF-8 prefix on a character boundary; `pinned({path,sha256})` reading a regular file (absolute, or normal-relative to a base) and requiring an exact hash match.

## Invariant

A pinned read returns bytes only when SHA-256(bytes)=declared hash and the path is a regular file; `prefix(s,n)` is a prefix of s with byte length <= n on a char boundary; `create` never overwrites; `Operation` releases its lock even if descriptor copies survive; lifetime leases (`lease`) are released only when every inherited copy closes.

## Cost

O(file bytes) for hashing/pinned reads (1 MiB buffer); O(1) syscalls for leases, statvfs and /proc reads.

## Composition

Leaf: every other Whale module uses these helpers; `pinned` is the single hash gate for specifications, templates, queues and references.

## behavior_complement

Domain: arbitrary paths/values. Excluded: symlinked or non-regular pin targets, relative non-normal pins, oversized reads, mismatched hashes, non-owned or group/world-accessible private directories, overwrites by `create`. Counterexample: a pin whose file differs by one byte is refused (negative control in spec/whale/agent tests). Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

From a successful pinned read one recovers the exact bytes and their hash; from a hash alone the file is not recoverable (many preimages, SHA-256 not inverted). From `prefix(s,n)` all strings sharing that prefix are compatible antecedents. A `None` from `try_lease` admits any holder of the same open file description. No unique inverse is claimed.
