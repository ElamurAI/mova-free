# prag-opus

## model

# Teacher compatibility and accounting

Prospective model recorded before edits: external process P, pure projection A, journal J. Production ask is P then A; callers may compose J.write then J.read. P delegates to authoritative Codex transport and remains unexecuted. A copies supplied metadata/named fields with legacy defaults. J preserves finite records under successful sequential I/O; only absence is an empty read. Predictions: nonfinite writes fail before opening and symlink loops return errors.

Behavior complement: A cannot synthesize different token slots; J cannot return a successful partial prefix after malformed records. Reverse reconstruction: composed observations admit A's fieldwise fibers and J's equivalent decoded encodings. Neither elapsed time nor journal success reconstructs external events.

Composition: fragments communicate through Call. Deserialization defaults batch/cache-write fields; canonical decode_events fixture joins shared validation to A. J rejects nonfinite seconds even though A preserves supplied values. External transport, append validation/write and read parse/I/O have distinct errors. No process equivalence is established.

## Cost

Let A be answer bytes, S step bytes, R serialized record bytes, B journal bytes and D decoded owned storage. Projection allocates O(A+S) storage. For the fixed canonical schema, time is O(A+S+1). For arbitrary supplied objects with M outer keys and U usage keys, default serde_json BTreeMap lookup adds O(log(M+1)+log(U+1)) comparisons, so time is O(A+S+1+log(M+1)+log(U+1)). There are a constant number of lookups, not constant lookup cost. Comparisons are bounded by the fixed literal lookup-key lengths; different serde_json map configurations (such as preserve_order) require their own lookup-cost assumptions. Append costs O(R) serialization/storage plus unbounded OS latency; rejected nonfinite records do constant work before opening. Read costs O(B+D) time and storage, including allocation for the complete UTF-8 text. Composition adds these costs for reached stages; P (external teacher) is unmeasured and has no bound here. No wall-clock speed claim follows.

## Attempted count

Domain: caller-supplied n: usize, arbitrary supplied reply and downstream acceptance count a. Observation: Call.sentences. Operator T(n,v)=n; this is an identity projection of attempted sentence count, independent of answer lines and acceptance. Complement: T cannot silently replace n by parsed/accepted row count. Reverse fiber for observed k constrains n=k but leaves v and a unconstrained. Assumption: n denotes attempted sentences by caller contract; this module does not verify that interpretation. Evidence: canonical_decode_accounting_join fixes n=7 with the one-line answer 1 OK and observes 7. Downstream accepted-row conservation is outside this fragment.

## evidence

### Evidence

Old-code witness `old-append.log`: removing the finite check reproduces prior serialization behavior; `nonfinite_append_leaves_file_untouched` fails because NaN is accepted. Old-code witness `old-read.log`: restoring the prior exists precheck makes `symlink_loop_is_read_error` fail because ELOOP becomes empty success. Both repairs are local and independently witnessed.

Compiled production mutant `mutant-cache-slot.log` maps cache_read_tokens from input_tokens; `canonical_decode_accounting_join` fails at runtime with 10 instead of 3. Exact source bytes were restored before all final positive checks. These are runtime failures, not compiler failures. `verification.log` records restoration and cross-profile normalized equality. Existing unrelated prag warning remains (unused_mut in frames.rs:1382); no new production warning is introduced.

Partial syntax projections were generated twice with mova-modelcheck syntax and compared byte-for-byte. They contain 572 nodes; macro bodies remain opaque. The file has two scoped production fragments and explicit composition through Call. Scope excludes environment mutation, live ask, process launch, remote acquisition, concurrency, crash durability and external teacher correctness. Synthetic canonical events do not establish actual process execution behavior.

## History

### 2026-10-30 · document

Invariants/cost are scoped in fragments. Source-bound partial syn projection has opaque macros; it is not a semantic AST/proof. Evidence is finite and no speed or approximation budget claim is made. Epoch 2026-10-30: corrected stale provider/default comments, retained Opus compatibility API, added interpret_reply, repaired nonfinite JSON corruption and hidden filesystem errors. Configuration and process execution remain untested.

### 2026-10-30 · evidence

Heading `### Evidence — 2026-10-30` stays in evidence as `### Evidence`.

Six named production tests pass twice in debug and twice in release; normalized named outcomes are identical. Raw receipts: `$MOVA_DATA/runs/octopus-wave17-transport-2026-10-30/{debug1,debug2,release1,release2}.log`. The six tests cover canonical decode/accounting join, supplied default fibers and u64 maximum, ordered append/read with legacy defaults and escaped strings, nonfinite no-write behavior, malformed JSON/UTF-8 and directory errors, and Unix symlink-loop error propagation.

### Undated · Cost

Scope/source revision: opus.rs SHA-256 0169f72bddab55f9e493b85a86918b5f0adbcf5153c7c12725bb249f2ed5c136; source and native evidence unchanged from d229375e0. These are symbolic inspection-based costs, not measured complexity guarantees for external services.

### Undated · Attempted count

Approximation semantics: scoped exact structural relation over the stated domain, supported by finite named evidence and source inspection; no probabilistic bound or general proof. Source revision: opus.rs SHA-256 0169f72bddab55f9e493b85a86918b5f0adbcf5153c7c12725bb249f2ed5c136, native code/evidence d229375e0. Tests refer to model.md#evidence at the file groupant root.
