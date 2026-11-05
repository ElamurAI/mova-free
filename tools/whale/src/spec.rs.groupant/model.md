# mova-whale-spec

## Descriptant

Pinned specification loader: prompt template, source kinds, admission, output contract, originality and bundle layout. Source: `tools/whale/src/spec.rs`. Public, domain-agnostic: concrete prompts, schemas, gate parameters and deployments come from pinned configuration.

## Forward approximant

Load `spec {path,sha256}` (absolute, hash-exact), parse `mova-shark-spec-v1` with unknown fields rejected, load the template pin (relative to the spec allowed), require each slot exactly once, validate bounds, labels, pointers, artifact names (no reserved/duplicate names) and manifest_extra keys. `prompt` splices metadata JSON and source text at template positions; `admission` normalizes licence names and applies exact admitted names and required flags.

## Invariant

No specification is built in; a loaded Spec carries the hashes of both files; rendered prompts are <= prompt_max_bytes; substituted text cannot create or move slots; admission local_only is true unless the licence is admitted and every required flag holds and the item is not local-only.

## Cost

O(spec + template bytes) per load; prompt rendering O(template + metadata + context).

## Composition

Every whale entry point loads the Spec from the run configuration; contract and publish read its rules.

## behavior_complement

Excluded: missing/relative/forged spec pins, altered template bytes, missing or repeated slots, unknown fields, unsupported adapters, invalid pointers or artifact names, manifest_extra overriding generic keys, deployment originality keys outside the corpus set. Counterexample: one appended byte in the template is refused (negative control shown red by disabling the hash check). Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

A Spec determines a unique template and contract; from a rendered prompt the metadata and source are recoverable only when they do not contain template text adjacent to slots (the split is ambiguous in general). From a normalized licence, many raw spellings are compatible. No unique inverse is claimed.
