# algorithm-needleman-wunsch

## descriptant

### algorithm-needleman-wunsch

Maximize the sum of match/mismatch/gap signed i64 scores over all global monotone paths. Store one deterministic optimum, preferring diagonal then deletion then insertion at every traceback cell. Origin: Needleman-Wunsch, JMB 48 (1970).

Cell (i,j) is the best representable prefix score; all three candidate additions are checked before selection. Traceback steps consume one or both input bytes and never produce gap-gap. Every path terminates at (0,0).

Partial Rust syntax projection excludes macro expansion and semantic resolution. Finite tests do not prove general correctness or linear cost. No executable approximation profile: outputs are discrete exact objects and no useful certified surrogate metric is proposed.

## model

# algorithm-needleman-wunsch

## Forward

Maximize the sum of match/mismatch/gap signed i64 scores over all global monotone paths. Store one deterministic optimum, preferring diagonal then deletion then insertion at every traceback cell. Origin: Needleman-Wunsch, JMB 48 (1970).

## Invariant

Cell (i,j) is the best representable prefix score; all three candidate additions are checked before selection. Traceback steps consume one or both input bytes and never produce gap-gap. Every path terminates at (0,0).

## Cost

O((n+1)(m+1)) time and score/step storage, O(n+m) traceback space. Checked cell count precedes max_cells check and reservations. All candidate additions are checked, including losing candidates. Allocation errors are returned; positive gaps are supported.

## nonbehavior

Successful results cannot omit input bytes, contain gap-gap columns, or have a suboptimal score. This is global linear-gap byte alignment; there are no affine gaps, local alignment or sequence-specific assumptions.

Domain: finite byte inputs and stated successful-allocation/representability premises. Observation: public call or named internal state. Status: source-derived exact target relation with bounded evidence, not a general proof.

## preimages

Deleting None entries independently from each column side uniquely reconstructs the two inputs. A score alone admits many input pairs; even complete columns do not recover scoring parameters uniquely.

Domain: finite byte inputs and stated successful-allocation/representability premises. Observation: public call or named internal state. Status: source-derived exact target relation with bounded evidence, not a general proof.

## validation-order

Checked dimensions precede CellLimit and allocations. Boundary scores are checked before interior cells. ScoreOverflow may reject even when the mathematically best path fits, because every candidate addition is checked.

Domain: finite byte inputs and stated successful-allocation/representability premises. Observation: public call or named internal state. Status: source-derived exact target relation with bounded evidence, not a general proof.

## tie-order

Strict greater-than replacement preserves diagonal on a tie; deletion wins a deletion/insertion tie when diagonal is worse. This local traceback rule selects one optimal path, not a lexicographically minimum forward alignment.

Domain: finite byte inputs and stated successful-allocation/representability premises. Observation: public call or named internal state. Status: source-derived exact target relation with bounded evidence, not a general proof.

## prefix-table

A table cell retains every optimal path to its two prefixes, but only one predecessor is recorded. Discarded optimal alternatives cannot be recovered from that predecessor alone.

Domain: finite byte inputs and stated successful-allocation/representability premises. Observation: public call or named internal state. Status: source-derived exact target relation with bounded evidence, not a general proof.

## score-scaling

Multiplying all scores by a positive integer preserves the chosen path when every intermediate checked sum remains representable. Overflow changes the admissible domain, so this is a conditional law.

Domain: finite byte inputs and stated successful-allocation/representability premises. Observation: public call or named internal state. Status: source-derived exact target relation with bounded evidence, not a general proof.

## evidence

### Evidence

Three tests for this algorithm passed twice in debug and twice with rustc -O, serial execution and identical normalized identities. Compiled mutation failed the named assertion; source restored byte exactly and positive suite passed again. Two source-bound partial syntax projections were byte identical.

15 binary words through length 3 per side, 64 scoring triples: 14,400 independent complete-path oracle comparisons. Every returned alignment is reconstructed and rescored. Budget equality, empty input, signed overflow and tie ordering covered. Mutation changes deletion > to >=; deterministic_ties fails.

Partial Rust syntax projection excludes macro expansion and semantic resolution. Finite tests do not prove general correctness or linear cost. No executable approximation profile: outputs are discrete exact objects and no useful certified surrogate metric is proposed.

## History

### 2026-10-30 · descriptant

Prospective model saved before implementation in $MOVA_DATA/runs/octopus-wave18-strings-2026-10-30/prospective.md.

Observed 2026-10-30: 15 binary words through length 3 per side, 64 scoring triples: 14,400 independent complete-path oracle comparisons. Every returned alignment is reconstructed and rescored. Budget equality, empty input, signed overflow and tie ordering covered. Mutation changes deletion > to >=; deterministic_ties fails.

### 2026-10-30 · evidence

Heading `### Evidence, 2026-10-30` stays in evidence as `### Evidence`.

Source SHA-256: 817ed9aca7e61db40d395261402d71d1e89021cc43c8a416daed29e1f1d28124.

Logs under $MOVA_DATA/runs/octopus-wave18-strings-2026-10-30:

- needleman_wunsch-debug-1.log: 8d4acb18f47f99c70061ace93c309cde5b567b56829beeb55873314f84af8a94

- needleman_wunsch-debug-2.log: 8d4acb18f47f99c70061ace93c309cde5b567b56829beeb55873314f84af8a94

- needleman_wunsch-release-1.log: f97285beb3e11f9353ddbd41d02fecdd1e434e13150b658af956dc357be81cdd

- needleman_wunsch-release-2.log: 232e9b8d5e579914f5ea7eb082d36abdb2c8f32e7d50fd946ef33f8c7d2f3f76

- needleman_wunsch-mutant.log: 43d7781101026853ae32690e310a90f4724109bb06c5b9a84294e787362141c5

- needleman_wunsch-restored.log: 8d4acb18f47f99c70061ace93c309cde5b567b56829beeb55873314f84af8a94

- cargo-needleman_wunsch-debug-1.log: 01a44dab7b3e00376d0ce9e0142deaa327a1fac39c52e4538101b891baf2a7ed

- cargo-needleman_wunsch-debug-2.log: 01a44dab7b3e00376d0ce9e0142deaa327a1fac39c52e4538101b891baf2a7ed

- cargo-needleman_wunsch-release-1.log: 11d11331bc49b186089748e5b7b9e0550dc59ff424dffb200589b87f41882bf4

- cargo-needleman_wunsch-release-2.log: e773725f308c3f8e737ab40cb95a48d0389618ef883b7bd6a7dd9a0b7ec182fa

Cargo focused suites also passed twice in debug and twice in release with matching normalized outcomes. Strict library Clippy passed. Temporary exports restored. Clippy log SHA-256: a5668d2636de8c7e7c3d3a1b6aea1da32174b64bee750b6b6cdfca8afc01a77b.
