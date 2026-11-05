# prag-groupant-wave15

## model

# Wave15 native joins

## forward

Seven tagged joins transport converted local lexicon roles, active/passive slots, sentence/token head identity, frame-deletion references, participle additions, oblique candidates and frame/role/schema observations through supplied inputs.

## invariant

Read.pred remains aligned to frames; (sentence,zero-based token) identifies a predicate and UD heads are one-based. Match.class/fclass/frame address the same retained local Lex. Entity slots, surviving frame maps and original token forms remain explicit.

## cost

Conversion scans supplied XML/TSV bytes plus ordered map work; matching visits candidate classes, ancestor frames and argument positions. Rule sites/atoms traverse supplied frame/token snapshots; deletion constructs one index map and traverses retained references. Rendering cost includes emitted bytes. Tiny fixtures establish no throughput estimate.

## composition

Each named test is a separate tagged relation. Composition requires identical Read frame indices, sentence indices, zero-based token positions, one-based UD heads, participant slots and the retained local Lex class/frame indices on both sides. Distinct tests do not constitute one end-to-end pipeline. Converted TSV feeds the same supplied local Lex; its Match indices are resolved against that Lex rather than global Match accessors. Frame edits and deletion operate on the same Read snapshot and remap its references.

## epoch-memory

# Wave15 prospective join review
Source inspection and external candidate harness only. No compile or execution by reviewer. Literal predictions below precede root measurements. These seven fixtures establish neither broad linguistic accuracy nor pipeline-wide proof. Existing frames_of_tree is exercised with supplied Word records; it can consult built-in lexicon/configuration, so the seventh test is not claimed environment-independent. No environment or mutable global override is changed. No read_with, active(), parser process, corpus, or teacher is used.

## Coordinate domains and API coupling
Word.head uses one-based UD IDs with zero denoting root; Tree, Read.pred, Site::Token, and Match indexes use zero-based coordinates. Read.pred is parallel to frames; each predicate pair is (sentence, token), not a token alone. Rule Ent role indexes 0..3 mean agent/patient/recipient/place; 4+k means oblique k. Frame links and Asked.frame address frame indices; rendering adds a presentation offset without changing stored coordinates.
Lex::from_tsv and Lex::best(&Clause, stable) own the lexicon identity of Match.class/fclass/frame. Match::class_id and Match::vframe resolve global lex(), so the harness deliberately indexes the supplied Lex instead. Converter row ordering determines IDs; distractor class index zero makes accidental global/index coupling visible. rules::select_rules(&str,&str)->Vec<Rule> signature confirmed by source: off selects none; proposed includes accepted and proposed; every other mode includes accepted only.

## Converter and lexical role join
Test converted_tsv_preserves_supplied_lex_role_coordinates: two supplied XML classes, one frame and two members produce counts (2,2,1). give frame Agent V Theme {to} Recipient yields Match coordinates (1,1,0), subj Agent, objs [Theme], pps [Recipient], score 8 with unknown animacy and empty frequencies, for stable false and true. Unknown verb gives None. Forward observation retains class/frame coordinates and ordered argument roles. Behavior complement excludes swapped object/PP roles and index zero provenance within this fixture universe. Reverse reconstruction includes equivalent XML formatting, other examples and unused lexical content; Match cannot uniquely recover XML. Cost: conversion proportional to XML events/output plus indexing, matching candidate frames across ancestors and argument slots; no time measurement.

## Active/passive coordinate join
Test supplied_matcher_transports_active_and_passive_role_coordinates: one supplied throw class with Agent V Theme maps active subject to Agent and object to Theme, score 4; passive subject to Theme and by to Agent, score 8. Empty restriction/frequency assumptions fix those scores. Complement excludes passive-subject Agent and passive object-vector entries invented from the subject. Reverse retains filler animacy/identity ambiguity; these role observations do not uniquely determine clauses. Cost: candidate scan and shape/argument allocation, with passive subject entering a temporary object list.

## Tree-head, action, rendering and schema join
Test sentence_token_pairs_select_head_agent_and_render_schema_act: two sentence trees reuse predicate indexes 1 and 2. Applying AgentFromHead at frame 3 selects Bob from sentence 1, while sentence 0 child stays empty; original input remains unchanged. KindSay(Ask) reaches Markdown speech: ask and schema ask. Complement excludes Alice as frame 3 agent and edits to frame 1. Reverse permits earlier child agents and kinds overwritten by actions; speech rendering loses tree/provenance details. Cost: apply_at clones Read, builds ordered predicate map and walks bounded ancestors; rendering follows entity/output size.

## Deletion and rendering join
Test deletion_reindexes_all_frame_links_but_preserves_token_coordinates: remove old frames 1 and 2 from four. Survivors old 0 and 3 become 0 and 1, pred remains [(0,2),(1,4)], roots [0], asked why at 1; quote [1], cause None, next 1, link parent 1. show(10) exactly Event _ say | says→#11 next→#11. Complement excludes dangling old indexes and remapped token IDs. Reverse admits arbitrary deleted content and vanished references, and therefore is many-to-one. Cost: scan frame/reference collections with ordered-set membership and a dense index map.

## Token-site append join
Test participle_site_addition_preserves_sentence_and_token_identity: broken cup and painted wall trees each expose token 0. AddNounPatient at (0,0) appends Event break/patient cup and pred (0,0); subsequent sites are Frame(0), Token(1,0). Rendering includes break/cup and excludes wall. Complement excludes selecting the same token in the other sentence or offering represented (0,0) again. Reverse of the observation admits unrecorded noun/tree details; the new frame is not a complete sentence reconstruction. Cost: site scan across tokens plus predicate lookup; action clones Read and appends one frame.

## Role-site resolution join
Test oblique_pronoun_site_resolution_changes_only_the_selected_role: recipient her and second oblique with her expose entity sites 2 and 5. Resolve(Before) at 5 changes only that oblique to alice and retains pron her/preposition with. Rendered trace exactly Event _ walk to=she(«her») near=tree with=alice(«her»); a repeated same resolution returns None. Complement excludes recipient/first-oblique changes. Reverse preserves multiple compatible earlier discourse lists and original entities with overwritten details. Cost: clone Read, construct predicate map, scan discourse candidates; output formatting follows retained entities.

## Extraction, selected rules and serialization join
Test extracted_frame_selected_rules_role_transfer_and_serde_preserve_identity: supplied Ada carries box tree yields one real frame with agent ada and patient box. Explicit accepted rule patient-to-to moves box to recipient; proposed reverse to-to-patient restores the complete original frame. Rejected drop is excluded and off selects none. Each selected action code parses back identically; Frame JSON deserializes to exact equality, including provenance/TAM. Complement excludes rejected execution, changed agent/predicate coordinates and serialization loss. Reverse role transfer is unique only under occupied-source/empty-destination preconditions and retained full state; serialized Frame preserves Frame fields but cannot recover the original tree or spelling beyond retained entity text. Cost: selected rules parse supplied text, apply_rule enumerates sites/features and builds predicate maps; JSON cost follows complete Frame size. Extraction cost is scoped to this supplied tree and built-in configuration, not live parsing.

## Oracle discipline
Each expected coordinate, role, count, selected rule ID, trace and score is literal and independent of implementation helper calculations. Equality assertions retain non-target fields; exclusions complement positive outcomes but are not the entire behavior-complement model. Root should preserve this prospective file and separately record compile fixes, measured outcomes, and counterexamples. Input domain requires parallel frames/pred and valid tree/frame references; malformed-coordinate behavior is outside these joins.


## checks

Seven named tests pass twice in debug and twice in release. The compiled wrong-sentence adapter fails its named assertion and exact bytes are restored. Partial syntax repeats identically; model.md#evidence records native and structural receipts. Macro bodies remain opaque; semantic AST and whole-system proof are absent.

## catalog

# Wave15 supplied-input relations

Source-derived specifications; bounded native evidence and repeated partial syntax are recorded in model.md#evidence. No general proof or complete inverse engine is claimed.

## behavior-complement

Type: behavior_complement. Named check: all seven groupant_wave15 tests.

Universe U consists of the seven tagged supplied-input fixtures and candidate observations at their declared APIs. R requires local Lex class/frame (1,1,0), ordered Agent/Theme/Recipient roles and score 8; active/passive subject roles Agent/Theme and scores 4/8; sentence-1 Bob copied at frame 3; survivor frame map 0->0,3->1 with unchanged predicate token pairs; only sentence-0 participle added; only oblique slot 5 resolved; and accepted/proposed role transfers with exact Frame JSON preservation. U minus R excludes swapped lexical roles, global-Lex index substitution, Alice copied across sentences, dangling deleted frame indices, altered token coordinates, adding the wrong sentence token, changing recipient while resolving an oblique, rejected/off rule execution and changed non-target Frame fields. This is a source-derived assertion relation, not a proven complement for all Rust paths; malformed domains and unexecuted branches remain unknown.

## reverse-reconstruction

Type: reverse_reconstruction. Named check: all seven groupant_wave15 tests.

For each tagged output y retain all supplied antecedents x satisfying its stated relation and shared coordinates. Match indices require the exact retained local Lex; roles/scores do not recover XML formatting, unused classes, examples, filler identity or animacy. Head copying and kind changes lose overwritten values. Deletion loses removed frames and incoming references while retaining survivor predicate coordinates. Added participle frames do not recover a complete tree; pronoun resolution leaves multiple compatible prior discourse lists. Full Frame JSON recovers that Frame on the lawful serialization domain, not the originating tree. Patient/recipient transfer is uniquely reversible only with occupied source, empty destination and all other state retained. Parent reverse composition existentially retains shared intermediate Lex/Read snapshots, never arbitrary independently chosen coordinates.

## converted-local-lex

Type: converted_lexicon_argument_transport. Named check: converted_tsv_preserves_supplied_lex_role_coordinates.

Two XML classes (distractor first), two members and one frame convert to counts (2,2,1). Loading the exact TSV into local Lex and matching give yields (class,fclass,frame)=(1,1,0), subj Agent, objs [Theme], pps [Recipient], score 8, for stable false/true; absent verb yields None. Index the local Lex, never global Match accessors. Exclude class 0 provenance or swapped object/preposition roles. Reverse admits XML layout, examples and unused-table ambiguity. Cost is supplied conversion/indexing plus candidate ancestor-frame/argument traversal.

## active-passive

Type: voice_argument_coordinate_transport. Named check: supplied_matcher_transports_active_and_passive_role_coordinates.

For supplied throw Agent V Theme with unknown animacy, empty frequencies/restrictions, active subj=Agent, objs=[Theme], by=None, score=4; passive subj=Theme, by=Agent, objs=[], score=8. Passive subject occupies an internal object position but must not create an output object entry. Exclude subject Agent in the passive. Role observations retain filler identity/animacy and clause ambiguity. Cost is candidate shape and argument traversal.

## sentence-head

Type: sentence_qualified_head_action_transport. Named check: sentence_token_pairs_select_head_agent_and_render_schema_act.

Two supplied trees reuse token indices 1 and 2; Read.pred pairs [(0,1),(0,2),(1,1),(1,2)] bind four frames. AgentFromHead on frame 3 copies Bob from sentence 1, leaves frame 1 empty and original Read unchanged. KindSay(Ask) renders speech: ask and schema labels retain ask. Exclude Alice copied from sentence 0. Reverse retains overwritten agent/kind ambiguity and rendering loses tree details. Cost includes Read clone, ordered predicate map, bounded head traversal and output.

## deletion-map

Type: survivor_frame_reference_transport. Named check: deletion_reindexes_all_frame_links_but_preserves_token_coordinates.

Delete frame indices {1,2} from four: old 0/3 become 0/1. pred remains [(0,2),(1,4)], roots=[0], asked why->1; first frame quote=[1], cause=None, next=1, link parent=1. show(10) is Event _ say | says→#11 next→#11. Exclude stale frame references and renumbered token coordinates. Reverse retains arbitrary deleted frame content and vanished references. Cost scans frame/reference collections with ordered deletion-set membership and one dense remap.

## participle-append

Type: sentence_token_site_append. Named check: participle_site_addition_preserves_sentence_and_token_identity.

Supplied broken cup and painted wall trees expose sites Token(0,0),Token(1,0). AddNounPatient at (0,0) appends break with patient cup, no agent, and pred (0,0); sites become Frame(0),Token(1,0). Markdown contains break/cup and excludes wall. Exclude sentence-coordinate substitution or reoffering represented token (0,0). Reverse admits omitted tree/noun detail. Cost includes token/site scan, Read clone, predicate lookup and one append.

## oblique-slot

Type: oblique_role_candidate_transport. Named check: oblique_pronoun_site_resolution_changes_only_the_selected_role.

Role indices 0..3 are agent/patient/recipient/place; 4+k selects oblique k. Recipient her at 2 and second oblique with her at 5 expose separate sites. Resolve(Before) at 5 changes only that oblique to alice, retains pron her and preposition with, and repeating returns None. Trace is Event _ walk to=she(«her») near=tree with=alice(«her»). Exclude recipient or first-oblique changes. Reverse admits multiple compatible discourse lists and overwritten entity fields. Cost includes clone, map construction, candidate scan and rendering.

## extract-rule-codec

Type: supplied_frame_rule_serialization_transport. Named check: extracted_frame_selected_rules_role_transfer_and_serde_preserve_identity.

Supplied Ada carries box Word tree extracts one frame agent ada/patient box. Explicit accepted patient-to-to moves box to recipient; proposed to-to-patient restores exact original Frame; rejected drop is excluded and off selects none. Selected Action codes parse identically and Frame JSON roundtrips exactly, retaining provenance/TAM and predicate (0,1). Exclude changed agent, rule-status leakage or field loss. Transfer inverse is unique only with occupied source/empty destination and retained other state; JSON cannot reconstruct original Tree. Cost includes supplied-tree extraction, rule text/site/feature traversal and JSON bytes. Extraction may read built-in lexicon/configuration; no live parser or environment mutation occurs.

## evidence

### Wave15 bounded evidence

139 selected native Mova checks pass twice in debug and twice in release with identical normalized named outcomes: 101 prior focused checks, 31 new unit checks (frames 8, rules 10, vn 6, vnconv 7) and seven new pragmatic joins. This is a selected suite, not the full repository suite; the ninety algorithms are unchanged and are not remeasured.

Four owner production mutations fail named assertions and their exact source bytes are restored. The root adapter changes only frame 3's predicate sentence from 1 to 0; it compiles and fails sentence_token_pairs_select_head_agent_and_render_schema_act (Alice rather than Bob), then exact source bytes are restored before positives. Compilation failures are not semantic rejection.

Witnessed repairs exclude unresolved right-hand heads from entity identity; reject invalid direct rule sites; terminate lexical parent chains before invalid/repeated indices; and avoid Unicode byte-boundary panic in malformed WordNet pointers. Pointer parsing retains the existing tolerant zero fallback: source zero can match any source word and target zero can select the first target word. Malformed pointers can still produce nominal rows; this is not validated WordNet input.

Pure production-path seams expose supplied lexicon tables/matching, VerbNet XML conversion and rule selection. Global VN recording behavior is preserved under fixed process configuration; concurrent environment changes are outside equivalence. Supplied Match coordinates index their own Lex, not global Match accessors. Extraction and rule features may consult compiled lexical resources/configuration. No live parser/teacher/corpus acquisition, filesystem conversion, GPU, whole-school, persistent adoption, general proof or speed/accuracy gain is claimed.

## History

### 2026-10-30 · epoch-memory

2026-10-30 prospective design retained below; no speed or accuracy gain is claimed.

### 2026-10-30 · evidence

Heading `### Wave15 bounded evidence, 2026-10-30` stays in evidence as `### Wave15 bounded evidence`.

Seven new native joins. Source SHA-256 b4a5a8101f5ea6a7bc74f63a3d1e6130b4d85e91ef9c5c886a7ace090e1f387f; twice-identical partial syntax SHA-256 73a72afd2ea4e17d24933674695e68661d0301d111e360f76c86b5e6b401b700.

Raw receipts: $MOVA_DATA/runs/octopus-wave15-integration-2026-10-30. native-evidence-hashes.json SHA-256 e62ef10e112b72d8cee51b62fc00d2b7182df368d9027a6dde8adc632d9608b3; all-normalized.txt ca00fc492f129d1b182df72826d3e547ba460d387fba9a9421805a4376f431dd; adapter-mutant.log 120c1c9e87c5351c8ff16aeefb4e400e3dbb00ae94b28ba0acf1b07125f53442; structural-controls/summary.json a50d4387189f34044579c0d068475201acf1f6e7ee8e3297c0db6909f3180975. Two structural controls reject stale parent-source bindings and wrong child identities, restoring exact bytes. Partial syntax has opaque macros and no semantic AST.
