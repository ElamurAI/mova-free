# mova-hard-cases-judge

## Descriptant

Recheck of the list with the deployed gparse (gparse view: learned / reopened), the fixed measurement sample, measurement of a model (pass rate, drafts' rate, silver UAS/LAS on harvested trees, EWT dev LAS) and the watch loop. Source: `tools/hard-cases/src/judge.rs`.

## Forward approximant

judge(tree, stanza) = roots | head_outside | grammar (world::induce::violations>0) | nonprojective | witness (UAS<0.8). sample: once, cases with drafts, measure split first then key, Stanza heads stored. measure(model) = gparse annotate (CPU) on the sample -> pass = no reason; attach() vs harvested trees; gparse eval on EWT dev. recheck = hard_case_list::recheck(gparse, all) with the judge; once per model id. watch: when the deployed weights' id differs from recheck.json -> measure then recheck.

## Invariant

The sample file is never rewritten; a measurement of the same model and sample is byte-identical per case (CPU); recheck never touches the en view; cases without tokens stay unjudged.

## Cost

One gparse CPU pass over the sample/list (~9 s for 131 sentences with EWT dev eval); one Stanza CPU pass per recheck.

## Composition

Uses hard-case-list, policy, harvest annotations, gparse and the Stanza bridge. Parent hard-cases.

## behavior_complement

Does not establish correctness (agreement with Stanza can be shared error); does not touch held-out test sets; does not change models. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

Per-case verdicts are kept in measure/runs/<label>-<model>.json; the model is identified by its weights' hash, not recoverable from the verdicts.

## Tests

judge::tests: green tree; red: two roots, head outside, two subjects, witness disagreement; attachment counts. Live: same model measured twice identical per case; drafts 0/131 (red by construction); an older model 68/131 vs deployed 66/131 shows the instrument moves. Finite fixtures and live samples; not a proof.

## History

### 2026-11-04 · Tests

judge::tests (2026-11-04): green tree; red: two roots, head outside, two subjects, witness disagreement; attachment counts. Live: same model measured twice identical per case; drafts 0/131 (red by construction); an older model 68/131 vs deployed 66/131 shows the instrument moves. Finite fixtures and live samples; not a proof.
