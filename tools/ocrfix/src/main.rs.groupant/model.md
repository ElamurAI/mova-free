# ocrfix-text-command

## descriptant

### ocrfix-text-command

#### Descriptant

Full source read preceded changes. Prospective model predicted branch/order, resource and information-loss behavior. Tests exercise actual helpers on bounded synthetic inputs; command workflows remain unexecuted. Production source prefix is preserved; only inline tests added.

Independent commands compose normalization/lexicon/channel correction, CER alignment, OCR masks, witness voting and line export. Corrector applies two-edit candidates and strict log-score improvement; CER normalizes letter streams then diagonal-first DP. External OCR renders then recognizes/masks then writes text, removes PNG after recognition closure.

Executable percentage approximation is unsupported: categorical parsing/effects have no useful certified metric here. Partial syntax excludes macro expansion and name resolution.

## approximant

### Forward relation

Independent commands compose normalization/lexicon/channel correction, CER alignment, OCR masks, witness voting and line export. Corrector applies two-edit candidates and strict log-score improvement; CER normalizes letter streams then diagonal-first DP. External OCR renders then recognizes/masks then writes text, removes PNG after recognition closure.

#### Domain

UTF-8 Hebrew text, finite positive lexicon total/oov and nonnegative finite costs for score interpretation; lengths/boxes bounded against u32/i32 overflow. Raw parsers do not enforce all premises.

## composition

### Local composition

The scoped alignment fragment models its declared source interval. Parent relation: Independent commands compose normalization/lexicon/channel correction, CER alignment, OCR masks, witness voting and line export. Corrector applies two-edit candidates and strict log-score improvement; CER normalizes letter streams then diagonal-first DP. External OCR renders then recognizes/masks then writes text, removes PNG after recognition closure.

Shared representation/invariant: Skeleton correction retains punctuation and unchanged original token if no improvement; altered skeleton drops points. Final-letter gate rejects internal final forms only. Alignment counts sum to DP distance under bounded u32 lengths. Command output is nontransactional. Child costs are included in parent costs, not added a second time. Other command paths are tagged alternatives; the child alone does not establish whole-file behavior.

## models

# Typed source models

All models are partial source-derived relations at this manifest snapshot. Scope is one call/trace under the stated domain; no universal semantic proof.

## behavior_complement

Short words below min_len cannot change; unattested/nonpositive candidate cannot win; mixed core scripts/brackets leave original token. OCR network/model/process effects are outside tests.

Domain: UTF-8 Hebrew text, finite positive lexicon total/oov and nonnegative finite costs for score interpretation; lengths/boxes bounded against u32/i32 overflow. Raw parsers do not enforce all premises. Observation: Skeleton correction retains punctuation and unchanged original token if no improvement; altered skeleton drops points. Final-letter gate rejects internal final forms only. Alignment counts sum to DP distance under bounded u32 lengths. Command output is nontransactional. Assumptions: bounded resources and source branch semantics. Oracle: selected actual-helper tests in model.md#tests constrain only their finite fixtures; remaining branches are source-derived.

## reverse_reconstruction

Skeleton maps many pointed/editorial texts to one consonant sequence. Corrected output admits unchanged originals or channel predecessors plus compatible lexicon scores; logs do not recover stripped marks.

Domain: UTF-8 Hebrew text, finite positive lexicon total/oov and nonnegative finite costs for score interpretation; lengths/boxes bounded against u32/i32 overflow. Raw parsers do not enforce all premises. Observation: Skeleton correction retains punctuation and unchanged original token if no improvement; altered skeleton drops points. Final-letter gate rejects internal final forms only. Alignment counts sum to DP distance under bounded u32 lengths. Command output is nontransactional. Assumptions: bounded resources and source branch semantics. Oracle: selected actual-helper tests in model.md#tests constrain only their finite fixtures; remaining branches are source-derived.

## channel_budget

Depth bounds edit path length2; cost gate c<=max+1e-9; best map stores strictly cheaper discovered paths; original removed even if reachable via cycle.

Domain: UTF-8 Hebrew text, finite positive lexicon total/oov and nonnegative finite costs for score interpretation; lengths/boxes bounded against u32/i32 overflow. Raw parsers do not enforce all premises. Observation: Skeleton correction retains punctuation and unchanged original token if no improvement; altered skeleton drops points. Final-letter gate rejects internal final forms only. Alignment counts sum to DP distance under bounded u32 lengths. Command output is nontransactional. Assumptions: bounded resources and source branch semantics. Oracle: selected actual-helper tests in model.md#tests constrain only their finite fixtures; remaining branches are source-derived.

## score_margin

Candidate wins only ln(count)-ln(total)-cost > incumbent+1e-12; lexical candidate iteration resolves near-equal ties by first encountered incumbent.

Domain: UTF-8 Hebrew text, finite positive lexicon total/oov and nonnegative finite costs for score interpretation; lengths/boxes bounded against u32/i32 overflow. Raw parsers do not enforce all premises. Observation: Skeleton correction retains punctuation and unchanged original token if no improvement; altered skeleton drops points. Final-letter gate rejects internal final forms only. Alignment counts sum to DP distance under bounded u32 lengths. Command output is nontransactional. Assumptions: bounded resources and source branch semantics. Oracle: selected actual-helper tests in model.md#tests constrain only their finite fixtures; remaining branches are source-derived.

## mask_coordinate_join

Each Hebrew box uses maximal positive intersection mask box; ties use last max iterator occurrence. Contiguous line keys preserve physical grouping; empty keep still newline.

Domain: UTF-8 Hebrew text, finite positive lexicon total/oov and nonnegative finite costs for score interpretation; lengths/boxes bounded against u32/i32 overflow. Raw parsers do not enforce all premises. Observation: Skeleton correction retains punctuation and unchanged original token if no improvement; altered skeleton drops points. Final-letter gate rejects internal final forms only. Alignment counts sum to DP distance under bounded u32 lengths. Command output is nontransactional. Assumptions: bounded resources and source branch semantics. Oracle: selected actual-helper tests in model.md#tests constrain only their finite fixtures; remaining branches are source-derived.

## effect_cleanup_order

Correct/vote creates output dir before lex reads; OCR PNG removal occurs after recognition closure, but render failure returns first. Multi-file writes have no rollback.

Domain: UTF-8 Hebrew text, finite positive lexicon total/oov and nonnegative finite costs for score interpretation; lengths/boxes bounded against u32/i32 overflow. Raw parsers do not enforce all premises. Observation: Skeleton correction retains punctuation and unchanged original token if no improvement; altered skeleton drops points. Final-letter gate rejects internal final forms only. Alignment counts sum to DP distance under bounded u32 lengths. Command output is nontransactional. Assumptions: bounded resources and source branch semantics. Oracle: selected actual-helper tests in model.md#tests constrain only their finite fixtures; remaining branches are source-derived.


## tests

### Native evidence

Package/bin: `ocrfix`. Exact selected names:

- `tests::brackets_rejoined`
- `tests::stream_and_cer`
- `tests::channel_corrects_and_keeps`
- `tests::negative_control_known_word_stays`
- `wave33::channel_depth_alignment_and_masks`

All 5 selected checks passed twice in debug and twice in release, one exact name per invocation with one test thread. Numeric exits are zero; sorted named outcomes match across all four runs. Added test blocks were formatted before these receipts; original source prefix (including earlier tests) remains byte-identical.

One compiled production mutant changed `for _ in 0..depth` to `for _ in 0..0`. The new wave33 test compiled and failed its runtime assertion (exit 101). Exact original/restored SHA-256 equality and a restored positive exact test (exit 0) were verified. No assertion-only mutant is counted; existing negative assertions remain ordinary tests. No surviving compiled mutant is omitted.

The external wave33 text receipts retain native-commands.json, verified-outcomes.json, mutation-report.json, raw logs/statuses, paired syntax projections and prospective.md. No private filesystem path is part of this public model.

Scope: actual helpers on synthetic in-memory data only. UTF-8 Hebrew text, finite positive lexicon total/oov and nonnegative finite costs for score interpretation; lengths/boxes bounded against u32/i32 overflow. Raw parsers do not enforce all premises. The scoped fragment reuses these tests where its helper is reached; other declared paths remain source-derived. Macro expansion, full command/effect execution, corpus accuracy and universal correctness are not established.

## invariant-and-cost

### Invariant and cost

#### Invariant

Skeleton correction retains punctuation and unchanged original token if no improvement; altered skeleton drops points. Final-letter gate rejects internal final forms only. Alignment counts sum to DP distance under bounded u32 lengths. Command output is nontransactional.

#### Cost

Normalization O(B); two-depth channel frontier may expand O(branching^2) with string/BTreeMap costs. CER O((n+1)(m+1)) time/space; word alignment O((N+1)(M+1)*(L+1)^2) time plus O((N+1)(M+1)) matrix. Mask merge O(H*M*textlen+H), process latency unmeasured.

Arithmetic and allocation premises are conditional. No timing or percentage surrogate claimed.

## History

### 2026-10-31 · descriptant

Heading `#### Epoch 2026-10-31` stays in descriptant as `#### Descriptant`.

### 2026-10-31 · tests

Heading `### Native evidence — 2026-10-31` stays in tests as `### Native evidence`.
