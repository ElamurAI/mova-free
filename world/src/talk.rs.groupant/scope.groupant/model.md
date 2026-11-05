# world-talk-scope

## descriptant

### Cue predicates, ordered speech acts and politeness scores

For trimmed t and lowercase l, let first be its first nonempty alphanumeric token; affection is OR of AFFECTION matches, pos OR of PRAISE, neg OR of BLAME or phrase not good. Polarity=-1 if neg, else +1 if pos or affection, else 0. Select the first true act predicate: greet/SOM -> Greet; thank -> Thank; apology -> Apologize; ends ? -> Ask; affection -> Express; negative self-correction -> Disagree; nonzero polarity and first not REQUEST_START -> Evaluate; request-start or can/could you prefix -> Request; otherwise Assert. politeness returns sum of all fired fixed weights and strategy names in rule insertion order.

## approximant

### Cue predicates, ordered speech acts and politeness scores

For trimmed t and lowercase l, let first be its first nonempty alphanumeric token; affection is OR of AFFECTION matches, pos OR of PRAISE, neg OR of BLAME or phrase not good. Polarity=-1 if neg, else +1 if pos or affection, else 0. Select the first true act predicate: greet/SOM -> Greet; thank -> Thank; apology -> Apologize; ends ? -> Ask; affection -> Express; negative self-correction -> Disagree; nonzero polarity and first not REQUEST_START -> Evaluate; request-start or can/could you prefix -> Request; otherwise Assert. politeness returns sum of all fired fixed weights and strategy names in rule insertion order.

Categorical source relation; no executable percent surrogate or approximation error bound is claimed.

## invariant

Act predicates short-circuit in the listed order while polarity is computed independently. has_word uses substring matching for non-ASCII cues or phrases, token equality for ASCII single words. social is membership in Greet/Thank/Apologize/Express/Evaluate/Disagree. This scope ends before memory_path and excludes indirect_request, persistence, civil and clock functions.

Cost: Cue matching scans lowercase input once per fixed cue/strategy, O(B*C); politeness also materializes tokens and checks membership. acts delegates babi splitting then inclusive ! splitting. C is fixed by source tables; allocations and Unicode lowercase expansion are unbounded by an explicit cap.

## composition

This child produces Said/Act/politeness observations consumed by parent social replies and other callers. It ends before memory_path; parent memory, indirect capability, distress and calendar models are separate. Ordered cue classification is not an implementation of the later blocked-action gate.

## models

## behavior_complement

Hello? cannot become Ask because greeting wins; How much love? cannot become Express because question punctuation wins before affection. good but wrong cannot have positive polarity. Empty text is Assert. The scoped pure functions do not append memories, consult the clock, test blocked training state or execute requests.

## reverse_reconstruction

Act Ask admits How much love? and ordinary terminal questions only after greet/thank/apology predicates fail; it does not uniquely determine affection or polarity. Assert admits empty input and neutral declaratives. Polarity=-1 admits both criticism and My mistake self-correction, so polarity alone cannot recover act. A politeness score can have multiple fired-strategy combinations; strategy names add information but still do not recover words.

## cue_matching_geometry

ASCII cue good matches a complete alphanumeric token, not goodness; phrase not good uses substring matching. Emoji/non-ASCII cues are substrings. Cue scopes differ from nerves concept normalization.

## priority_relation

For predicate vector p, branch index=min{i:p_i=true}; lower-priority predicates may also be true without deciding the act. Affection wins over self-correction once terminal-question handling has failed.

## politeness_additivity

Score=Σ weight_i*indicator_i. please at first token contributes -30; please elsewhere contributes +49. Strategies overlap and their weights all add; this is not a refusal threshold or probability.

## sentence_social_transport

acts uses babi::split_sentences then splits each resulting piece inclusively on !, filters whitespace-only pieces and applies act_of. social then recognizes a closed act subset; it does not itself produce a reply or persist a record.


## tests

talk::wave28_dialogue::act_precedence_and_indirect_gate exercises Hello?, How much love?, My mistake, empty, and negative-over-positive polarity; its indirect_request assertions are outside this child. civil_negative_seconds_and_politeness exercises Please check versus Check please; its civil/distress assertions are outside this child. Memory effects are not tested.

Commands and repeated outcomes: [parent evidence](../model.md#tests), [source evidence](../evidence.json). No new execution was required for this model-only refinement.

## History

### 2026-10-31 · descriptant

2026-10-31 review epoch: replaced generic fragment wrappers with scoped transitions, concrete exclusions and compatible antecedents. The exact byte range/hash and shared parent AST are retained. This child produces Said/Act/politeness observations consumed by parent social replies and other callers. It ends before memory_path; parent memory, indirect capability, distress and calendar models are separate. Ordered cue classification is not an implementation of the later blocked-action gate.
