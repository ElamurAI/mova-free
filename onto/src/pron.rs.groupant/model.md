# onto-pron

## descriptant

### onto-pron

gender_cue first female/male words independently; one class only returns cue, both/neither None. sense noun lookup→optional sentence WSD→core→first. info three property resolutions; persona=speak Yes && human!=Yes; own text then3 first ancestors for cue. pronouns ordered policy.

## approximant

### Forward relation

gender_cue first female/male words independently; one class only returns cue, both/neither None. sense noun lookup→optional sentence WSD→core→first. info three property resolutions; persona=speak Yes && human!=Yes; own text then3 first ancestors for cue. pronouns ordered policy.

#### Equations

persona=(speak=Yes)∧(human≠Yes); cue=Fem if F≠∅∧M=∅, Masc if M≠∅∧F=∅, otherwise None. AllowedPronouns=ordered decision table on(human,persona,gender).

#### Domain

Unknown/Undetermined map Tri Unknown with different reasons. No antecedent selection/plural agreement. Sentence WSD uses default prior.

Categorical behavior and effect boundaries have no validated percentage surrogate. Executable Rust approximant unsupported; 1/2/5/10% future caps require a useful metric first.

## invariant

### Invariant and domain

Unknown/Undetermined map Tri Unknown with different reasons. No antecedent selection/plural agreement. Sentence WSD uses default prior.

#### Limits

Finite synthetic fixtures are not semantic proof. Public invalid indices, allocation failures, huge counters, environment races and live effects outside explicit checks remain unverified.

## cost

### Cost approximation

Cue O(chars) fixed lexicons; info up to3 ancestry first edges plus3 Kb properties and optional IDF/WSD; wrappers recompute.

Symbolic costs only; no timing or throughput claim.

## composition

### Composition

persona=(speak=Yes)∧(human≠Yes); cue=Fem if F≠∅∧M=∅, Masc if M≠∅∧F=∅, otherwise None. AllowedPronouns=ordered decision table on(human,persona,gender).

Graph builder→store shape check→Kb; Kb→query→bridge/describe/pron. hs Parsed→hsmap preference/scoring/gates. Core SynMap last-duplicate vs Graph first-duplicate requires unique synset keys for interchangeable lookup. Syntax children below share parent projection; exact byte ranges bind original source semantics.

- cue-parser: F=first female cue,M=first male cue; output iff exactly one exists

- pronoun-policy: Ordered cases on(human,persona,gender) produce a fixed allowed singular pronoun slice

## models

## behavior_complement

Mixed male/female words exclude direct cue; missing noun excludes info; no seed cannot infer No. Nonhuman/nonpersona Neut excludes he/she.

## reverse_reconstruction

[it] compatible with Neut or Unknown absent human/persona; [he] requires Masc and human Yes/persona. Cue inputs erase case/punctuation and unconstrained noncue words.

## cue_ambiguity

Both classes return None; ancestor fallback may supply cue after ambiguous own text.

## persona_relation

Human Unknown with speak Yes is persona; human Yes is not persona.

## ordered_pronouns

human/persona Masc/Fem→he/she; human→he,she; persona→he,she,it; animal gender→it,he/she.

## sense_transport

Missing noun stops before WSD; sentence choose precedes first core/first fallback.


## tests

### Actual-code checks

- `pron::wave29_tests::cues_and_layer_persona`

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective model saved before edits; nine existing sources read fully. Added bounded inline tests; store reader overflow repaired only after compiled old-code failing witness. No live corpus, teacher, model or database effects executed. Source syntax generated twice identically; partial lexical projection only.

### 2026-10-31 · tests

#### Evidence 2026-10-31

Each selected test passed twice debug and twice release after exact mutation restoration (15 named suite checks; all four sorted outcome sets equal, exits0). Compiled source control: negative-pron.log (compiled source mutation assertion failure101). Controls compile successfully and fail an assertion with exit101. Store control is the original unsafe length addition before repair.

Finite graph/token/temp fixtures; public invalid states and unselected branches not exhausted

Whole CLI, live teacher/corpus/model/prior loading, filesystem failure/durability and huge inputs were not executed. No exact wrapper is advertised as a percentage surrogate. Added test blocks rustfmt-formatted independently; unrelated source formatting preserved. Strict broad Clippy not repeated: existing dependency/source warnings are outside this work.
