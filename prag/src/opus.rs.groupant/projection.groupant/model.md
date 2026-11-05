# Supplied response projection

Domain: supplied JSON Value, step string, batch/sentence usize and elapsed f64. Observation: returned answer and every Call field. Forward: copy metadata, string result or empty, numeric cost or zero, named unsigned token fields or zero. Invariant: no process/filesystem access; token slots retain key identity. Cost: O(answer bytes + step bytes + 1) for the fixed canonical schema; arbitrary extra keys add map lookup work as specified in the Cost section.

Behavior complement: altered metadata and different nondefault token slots are excluded. No lifecycle validation occurs here; decode_events owns that boundary. Reverse reconstruction: zero fields permit absent, mistyped or zero source values; extra JSON fields are unconstrained. Nondefault values constrain corresponding source fields. Supplied seconds are copied, including nonfinite values subsequently rejected by append.

## Cost

Let A and S be answer and step bytes, M outer object keys and U usage object keys. Copying strings allocates O(A+S) storage; token conversion and metadata copy cost O(1). The number of lookup operations is fixed, but default serde_json uses BTreeMap objects, giving O(log(M+1)+log(U+1)) key comparisons. Fixed literal lookup-key lengths bound each comparison cost. General supplied-Value time is therefore O(A+S+1+log(M+1)+log(U+1)); the fixed canonical schema reduces this to O(A+S+1). Enabling serde_json preserve_order changes the map implementation and requires separate lookup-cost assumptions. No JSON parsing, process execution, I/O or elapsed-time measurement is performed in this seam. This symbolic model excludes allocator costs and does not claim measured speed.

## Reply default fibers

Domain: arbitrary supplied JSON Value and fixed metadata. Observation: result string, cost and four token slots. Operator applies as_str/as_f64/as_u64 followed by empty/zero defaults. Complement: invalid types cannot become nonzero token counts; defaulted input does not produce a validation error here. Reverse fibers for zero include absent, null, wrong-type and numerical zero values; nonzero unsigned slots require their own corresponding key values. Extra keys remain free. Assumption: production calls pass decode_events output, while synthetic tests characterize wider compatibility behavior. Evidence: supplied_projection_default_fibers checks null, absent, negative/string fields and u64 maximum; canonical_decode_accounting_join checks named slots; cache-slot mutant fails that join.

## History

### 2026-10-30 · document

Assumptions: production input is an already validated canonical teacher reply; arbitrary synthetic values only characterize compatibility defaults. Evidence: canonical_decode_accounting_join and supplied_projection_default_fibers; compiled cache-slot mutant fails canonical join. Source hash bound in manifest. Epoch 2026-10-30: extracted pure seam without changing defaults. No approximation error or process behavior claim.

### Undated · Cost

Scope/source revision: opus.rs SHA-256 0169f72bddab55f9e493b85a86918b5f0adbcf5153c7c12725bb249f2ed5c136; source and native evidence unchanged from d229375e0. These are symbolic inspection-based costs, not measured complexity guarantees for external services.

### Undated · Reply default fibers

Approximation semantics: scoped exact structural relation over the stated domain, supported by finite named evidence and source inspection; no probabilistic bound or general proof. Source revision: opus.rs SHA-256 0169f72bddab55f9e493b85a86918b5f0adbcf5153c7c12725bb249f2ed5c136, native code/evidence d229375e0. Tests refer to ../model.md#evidence at the file groupant root.
