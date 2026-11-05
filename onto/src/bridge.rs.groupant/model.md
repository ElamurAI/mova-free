# onto-bridge

## descriptant

### onto-bridge

Ordered read_world hashes UTF8 content before line extraction and keeps first filename/tale for duplicate content; char/obj/loc require type. map_class tries exact lookup, hyphen spellings, then final head; first context lexfile match else first sense. answer_question selects first tale entity, optionally a same-lexfile visible seed sense, then delegates query.

## approximant

### Forward relation

Ordered read_world hashes UTF8 content before line extraction and keeps first filename/tale for duplicate content; char/obj/loc require type. map_class tries exact lookup, hyphen spellings, then final head; first context lexfile match else first sense. answer_question selects first tale entity, optionally a same-lexfile visible seed sense, then delegates query.

#### Equations

T(class)=min{tier | lookup_tier(class) nonempty}; choose(F,ctx)=first{c∈F.senses:lf(c)=ctx} or F.senses[0]. Dedup(files)=first occurrence per SHA256(contents).

#### Domain

Read-only; tier precedes context fitness; duplicate filename/content order matters.

Categorical behavior and effect boundaries have no validated percentage surrogate. Executable Rust approximant unsupported; 1/2/5/10% future caps require a useful metric first.

## invariant

### Invariant and domain

Read-only; tier precedes context fitness; duplicate filename/content order matters.

#### Limits

Finite synthetic fixtures are not semantic proof. Public invalid indices, allocation failures, huge counters, environment races and live effects outside explicit checks remain unverified.

## cost

### Cost approximation

O(B) read/hash plus O(E) entities; coverage ordered maps O(E log E),400 sample cap; question scans entities/assertions plus query traversal.

Symbolic costs only; no timing or throughput claim.

## composition

### Composition

T(class)=min{tier | lookup_tier(class) nonempty}; choose(F,ctx)=first{c∈F.senses:lf(c)=ctx} or F.senses[0]. Dedup(files)=first occurrence per SHA256(contents).

Graph builder→store shape check→Kb; Kb→query→bridge/describe/pron. hs Parsed→hsmap preference/scoring/gates. Core SynMap last-duplicate vs Graph first-duplicate requires unique synset keys for interchangeable lookup. Syntax children below share parent projection; exact byte ranges bind original source semantics.

- class-mapping: class→first successful(exact,variants,head); sense=first context match else senses[0]

- world-ingest: Files→first unique content hash→used filename→typed char/obj/loc lines

## models

## behavior_complement

Duplicate-content later files cannot add entities; missing type cannot emit Ent; exact lookup success excludes Head even when head context matches.

## reverse_reconstruction

Ent preimages allow arbitrary ignored lines/fields; mapped Cid admits every exact/variant/head string consistent with earlier-tier failures. Verdict alone cannot reconstruct entity/tale.

## tier_precedence

Exact→Variant→Head→Missing; chief-crow with only crow uses Head.

## context_selection

First matching lexfile in senses wins; absent/unknown context falls back first and false.

## content_identity

SHA256 text identity keeps first filename; used includes files with zero valid entities.

## question_layer

tale layer or real fallback; visible nonreal same-lexfile seed may replace class sense.


## tests

### Actual-code checks

- `bridge::wave29_tests::mapping_tiers_and_content_dedup`

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective model saved before edits; nine existing sources read fully. Added bounded inline tests; store reader overflow repaired only after compiled old-code failing witness. No live corpus, teacher, model or database effects executed. Source syntax generated twice identically; partial lexical projection only.

### 2026-10-31 · tests

#### Evidence 2026-10-31

Each selected test passed twice debug and twice release after exact mutation restoration (15 named suite checks; all four sorted outcome sets equal, exits0). Compiled source control: negative-bridge.log (compiled source mutation assertion failure101). Controls compile successfully and fail an assertion with exit101. Store control is the original unsafe length addition before repair.

Finite graph/token/temp fixtures; public invalid states and unselected branches not exhausted

Whole CLI, live teacher/corpus/model/prior loading, filesystem failure/durability and huge inputs were not executed. No exact wrapper is advertised as a percentage surrogate. Added test blocks rustfmt-formatted independently; unrelated source formatting preserved. Strict broad Clippy not repeated: existing dependency/source warnings are outside this work.
