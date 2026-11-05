# algorithm-linear-recurrence-tests

## descriptant

### Modeling

Original exact implementation/test harness. Partial syntax, forward relation, invariant, cost and typed views share this source identity. The combiner retains their assumptions and finite evidence; it does not generate semantic proofs. Macro bodies are opaque in the syntax projection. No useful approximate profile is selected, so no executable surrogate or percentage-error claim is introduced.

Prospective recurrence model: polynomial reduction trades O(n*k) sequential advancement for O(k^2 log(n+1)) work. No recurrence runtime speedup was measured. Prospective NTT model: three transforms should lose to schoolbook at16 and overtake by512 or2048; comparisons below test this hypothesis on supplied equal operand lengths only.

|Operand length|NTT median ns/call|Schoolbook median ns/call|NTT range|Schoolbook range|
|---|---|---|---|---|
|16|7006|602|5513..7901|467..748|
|128|24345|23031|19533..25922|10687..23220|
|512|54594|176265|53174..63443|170290..195138|
|2048|242709|2661481|242386..270817|2651024..2699745|

Five batches alternate timing order; each supplied size checks output equality before its batches, and the benchmark normalizes schoolbook operands once. Timings include allocations, result materialization and destruction. Hardware contention and small samples limit generalization; no universal crossover, training speed or full-course claim. The measurements support the prospective prediction: schoolbook wins at16 and128; NTT wins at512 and2048. Thus the observed crossover is bracketed by128 and512 only for these operands and this run. No warm-up phase was used; the reported spread includes startup/cache effects.

Sources: recurrence reduction follows classical polynomial algebra (von zur Gathen and Gerhard, Modern Computer Algebra, third edition,2013); finite-field FFT follows Pollard, Mathematics of Computation25 (1971),365-374. The code and fixtures are original.

## model

# Source-bound mathematical views

## Forward

Execute the finite deterministic fixtures; pass iff every oracle equality, boundary expectation and metamorphic assertion holds. This observes production calls through the test harness.

## Invariant

Canonical residues and input immutability; test observations require the declared domains. Arithmetic/resource boundaries are explicit below. Structural checks are not semantic proofs.

## Cost

Finite loops only. Iterative recurrence oracle uses O(n*k) time per call and O(n+k) words. Schoolbook uses O(a.len*b.len) products and O(a.len+b.len) words; finite vector corpus stores121 short vectors. No full training cost claim.

## behavior_complement

For the finite declared fixtures and terminating candidate implementations, U minus the harness pass/fail relation contains false acceptance and false rejection observations. Abort and nontermination are outside this finite observation universe; negative cases alone do not define this target.

Domain: this source's declared API or finite fixture schedule. Observation: returned values/errors and assertion outcomes; allocation environment remains external. Assumptions: integer arithmetic and classical algebra premises above. Status: partial mathematical specification with bounded evidence in model.md#evidence; no universal Rust proof or approximate substitution.

## reverse_reconstruction

An observed pass admits every implementation matching the finite fixtures, including arbitrarily different off-fixture behavior. A failing named assertion admits production defects or a defective oracle; the observation does not uniquely localize a cause.

Domain: this source's declared API or finite fixture schedule. Observation: returned values/errors and assertion outcomes; allocation environment remains external. Assumptions: integer arithmetic and classical algebra premises above. Status: partial mathematical specification with bounded evidence in model.md#evidence; no universal Rust proof or approximate substitution.

## oracle_independence

The recurrence oracle appends terms sequentially with lagged dot products, never polynomial reduction. The convolution oracle uses independent schoolbook index summation, never transforms. Shared primitive arithmetic is a remaining assumption; finite exhaustive inputs are explicitly bounded.

Domain: this source's declared API or finite fixture schedule. Observation: returned values/errors and assertion outcomes; allocation environment remains external. Assumptions: integer arithmetic and classical algebra premises above. Status: partial mathematical specification with bounded evidence in model.md#evidence; no universal Rust proof or approximate substitution.

## counterfactual_oracle

A compiled production mutation must fail a named assertion; independently, adding one to an oracle observation must also fail. Exact original source bytes are restored before positive rechecks and syntax binding. These controls establish sensitivity only to the selected changes.

Domain: this source's declared API or finite fixture schedule. Observation: returned values/errors and assertion outcomes; allocation environment remains external. Assumptions: integer arithmetic and classical algebra premises above. Status: partial mathematical specification with bounded evidence in model.md#evidence; no universal Rust proof or approximate substitution.

## finite_schedule

Recurrence enumerates sum(k=1..3)3^(2k)=819 seed/coefficient pairs, five moduli and17 indices:69,615 oracle comparisons. Convolution enumerates121 vectors of lengths0..4 over0,1,2 and all14,641 pairs. Deterministic finite scheduling gives repeatable identities, not unrestricted coverage.

Domain: this source's declared API or finite fixture schedule. Observation: returned values/errors and assertion outcomes; allocation environment remains external. Assumptions: integer arithmetic and classical algebra premises above. Status: partial mathematical specification with bounded evidence in model.md#evidence; no universal Rust proof or approximate substitution.

## evidence

### Finite evidence

Eight new tests pass, four per algorithm. With temporary local module exports, the existing library plus these tests passes306 unit tests twice debug and twice release with identical sorted test identities/results. Strict algorithms Clippy/all-targets and narrow rustfmt pass. Each implementation and each test source has a compiled assertion-failing mutation and byte-exact restoration; names and raw output are in external logs. No allocation refusal was injected, no maximal NTT array was allocated, and no global proof follows.

## validation

### Validation

Compiled control: Add1 modulo m to the independent iterative oracle result. Named assertion failure: linear_recurrence::tests::finite_iterative_oracle. The original bytes were restored and this named test passed again. The eight new tests and298 existing library tests pass twice per debug/release profile, with identical306 sorted identities. No root API join is claimed here; the root integrator owns exports and composition tests.

Four source projections were generated twice, byte-identically. Repeated groupants reports are identical; changed accepts the four new registered sources under an external registry scaffold. These check source/artifact declarations only.

Limits: maximal NTT storage and allocation refusal are not executed; arbitrary off-fixture behavior remains unverified. The recurrence order cap4096 is a resource policy. Exact algorithms have no surrogate error budget.

## History

### 2026-10-30 · descriptant

Heading `### Modeling epoch 2026-10-30` stays in descriptant as `### Modeling`.

### 2026-10-30 · evidence

Heading `### Finite evidence, 2026-10-30` stays in evidence as `### Finite evidence`.

External evidence under $MOVA_DATA/runs/octopus-wave10-numeric-2026-10-30; SHA-256 bindings:

- prospective.md: 98f42d52c02a986739b71d7e7fcd33bca123e7b3801c0af7c63712d982f6965e

- normalized.txt: f7d0728613a44a1396116d3443a289382d5a32af5749ddacea032f59a5c21692

- debug1.log: 9bcc67b5002fb501a7e1e0324822331180cc71674b93b6a56d2328e5d3ed93f5

- debug2.log: b290a5766bbec05a0496c3ccd8a5e827cf508abd498974f8ff4b1775bd584450

- release1.log: bee75b34a3e2d1a62307f5ac9838e90361c2d5102cc81d1b68726ce7521c8597

- release2.log: b47c313f67bea615fa649d502dc1a1eaa5348869f6576884c4f37b0de8e13063

- clippy.log: c4befea3c686d896b0b23d975cf532278bf6609c76e42421548f41d7e151aa7b

- timing.rs: e5bec634506beb46db5cb2996a9c22ef09ffbef9a27ef67ec190121c97ce74c0

- timing.log: 2e9c541523a0ed1679c0e5beb8cb98d936b1bf7f3681de39e2a232d37d95805f

- validation.log: 148d3f6e6a3427aac131893b057a8323fa1de3ea67afeaeb2e5eaa5a9e6c79f7

- mutant-linear_recurrence-tests.rs.log: 34cb767ff8a657bce29b63aae0c7165117208012fce1de2d7e0961022d2c0225

- restored-linear_recurrence-tests.rs.log: 7a9f9ed90b67e18f2d6ae64145df33a671339562b8c39606bdfd25f0566ec695

### 2026-10-30 · validation

Heading `### Validation, 2026-10-30` stays in validation as `### Validation`.

- groupants1.json SHA-256: 748ffa97af5b5a0ce4d9ec5ad660d9ad194fa1de4206f34b5d19ecd8b2c4308b

- groupants2.json SHA-256: 748ffa97af5b5a0ce4d9ec5ad660d9ad194fa1de4206f34b5d19ecd8b2c4308b

- changed.json SHA-256: 7df3f3facfd9a1a76658abac9663a4994faa8b2813813502c817df26fd679176

- structure.log SHA-256: cb5a49c3e9a8d1cfa1f7e19110072ec8a864b4a2feb69941e329009daf325a7a
