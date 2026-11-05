# mlab-suite-parse-check

## descriptant

### mlab-suite-parse-check

Scenario text normalization and line comparator.

## approximant

### Scoped forward relation

Headers split on | (first three only); code ends at exact ---; expectations recognize >*, > and > text. Compare exact/prefix, return earliest mismatch.

#### Domain and gap

Source-generated states within resource limits. Categorical relation has no executable percent surrogate.

## invariant

### Scoped invariant

got empty has no lines; got LF has one empty line.

#### Cost

Linear text and output/expectation traversal, allocating scenario/code/line vectors. String prefixes cost compared bytes; mismatch short-circuits.

## models

# Scoped typed relations

## behavior_complement

Preamble without header excludes scenarios. Prefix matching excludes extra output lines. Comparator mismatch excludes matching any later line as a replacement for the first error.

## reverse_reconstruction

Different ignored preambles and extra header fields map to same scenario; prefix ab accepts abc and ab.

## blank_line

got empty has no lines; got LF has one empty line.

## delimiter

Whitespace around --- prevents delimiter recognition.

## missing_header

Text without ### header returns zero scenarios.

## first_mismatch

Comparison stops earliest mismatch and reports k+1.


## tests

### Evidence scope

Parent selected tests: wave30_synthetic_suite_boundaries, wave30_synthetic_gate_fail_fast. This fragment shares parent syntax and finite checks; no separate test count or complete fragment coverage claim. Assertion-control association and unexecuted paths are in parent model.md#tests and source-test-map.json.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Half-open source bytes [499,3525); bound by parent source hash and scoped byte hash. Model covers only this fragment; parent composition explains joins.
