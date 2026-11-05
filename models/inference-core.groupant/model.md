# inference-core

## model

# Partial inference interaction model

## Forward approximant

The registered observations cover execution E, cache C, frame adapter F, state queries Q, indexing I, spans A, world types T, lexical branch L and versioned store S. Most form a tagged union of API observations, not a serial pipeline. One actual local edge is C.parse -> E.parse_many for missing texts. F.read instead preflights tree::annotator and invokes pragmatic read_with through Tree::parse_cased. World command transitions, cached tree traversal and question-frame construction now have registered scoped models. Full text-to-world construction and reader sentence selection still require external dependencies and additional correspondence evidence.

Index scores are heuristics, not truth probabilities. Queries mix historical selection with current display labels. Spans observe a supplied tree and branch. Value judgments now use ordered fields/goals and prior-command harm; parallel store construction retains scheduling assumptions. These observations do not compose automatically into a deterministic answer function.

## Composition

Cache missing texts retain duplicates. After an exact-length parser result, the last occurrence of a key wins and every matching output clones it. Counters record attempts even on parser failure. finish separately checks identity and publication; rename/checksums do not imply locking, crash durability or authentic data.

The possible broader path E;U;I still needs independently validated text-to-world construction U and caller anchor/identity conditions. Q reads World directly; A receives a caller-selected Tree/QFrame. T exposes public representations wider than parser-established canonical subsets. L exposes separate number and command languages. Cross-tree school, catalog and pragmatic dependencies remain explicit assumptions.

## Invariant

Actual edges join the same text, parser identity, world references and observation scope. Valid one-based anchors, finite scores and bounded arithmetic are operation-specific. Branch ordering does not establish system determinism. AST/source integrity does not prove semantic correctness or dependency completeness.

## Epoch memory

The registered store branch adds version rows, references, dictionary lookup, indexed selection and derived sets/order caches. Cache publication directly calls its single-file helper. Store::open reads version rows and main separately, so concurrent append-only updates can produce a mixed captured view. Sampled query membership can change for unchanged sentence IDs. Changed-model sampled sets now reselect once and persist the complete vector; legacy unstamped-prefix refresh preserves query order. Pointwise carry retains its delta-completeness assumption. Mixed snapshot capture and other storage limitations remain open.

The semantic-delta epoch adds rule derivation D to the registered API union. A concrete data edge is stored records -> lexical feature index and, separately, scan_store -> packed rule input. The update equality gate now includes tag and normalized lemma, preserving changes that structure alone hides. Complete feature-change deltas remain conditional on valid stable tables, successful reads and publication. scan_store's separate latest-lemma lookup prevents treating the two reads as an atomic snapshot. The added derivation sidecar models existing search/packing boundaries; registering it does not demonstrate a general retained-learning loop.

The score/worker epoch strengthens separate branches rather than asserting a new parser-to-answer pipeline. Native index fixtures build public facts directly and retain the positive-sentence domain. Worker fixtures use closures, not a parser. Output order is distinct from callback purity; failed batches can preserve external effects while discarding owned results. The new worker test fragment records these observation boundaries.

The query/span epoch executes bounded projection examples. Update-history multiplicity, current labels, lexical resolution, attention partitions and normalized answer spans observe different information. Retain operation identity and the caller's valid-world/tree premises. Complete fibers concern specified finite domains; fallback and dispatch checks are selected examples. The parent does not infer correct sentence selection or semantic accuracy from these child checks.

The tree/question/world epoch registers those three representations and the separate corpus-statistics branch. A native command-language-to-world fixture observes four accepted commands and one missing-location rejection, including historical/current snapshots. This is narrower than natural-language understanding or universal transaction atomicity. Constructor-built child caches admit a cycle-cut argument; direct public word mutation can invalidate cache correspondence. Corpus branches preserve distinct token units, extraction policies and I/O effects.

## wave13-extension

Pragmatic feature/classifier/schema/inference observations compose with existing inference-core branches only through explicit identical dictionary snapshots, field class alphabets, row IDs and supplied schedules. Numeric feature IDs from distinct dictionaries cannot be substituted; missing-prefix start fallback cannot be treated as gold or future context. eval.rs fit_set/predict_seq/scores and data.rs Item are inspected unregistered dependencies, not newly owned leaves. Preserve established frame/lexical dependencies and avoid treating these tagged tests as a single CLI pipeline. Reverse composition retains all compatible shared dictionaries/schedules and antecedent rows; equal outputs do not recover training history or unique text.

## wave15-extension

Shared coordinates are Read frame i, Read.pred[i]=(sentence s, zero-based predicate token p), Tree.w[p] with one-based UD heads, participant slot k, and local Lex indices (class,fclass,frame). Conversion output TSV feeds that same local Lex; global Match accessors do not resolve local indices. Rule edits preserve sentence-qualified lookup; deletion maps retained frame references through one survivor map; rendering and schema views preserve selected roles/acts. The seven tagged fixtures are separate joins, not a general parser pipeline. Universe U consists of the seven tagged supplied-input fixtures and candidate observations at their declared APIs. R requires local Lex class/frame (1,1,0), ordered Agent/Theme/Recipient roles and score 8; active/passive subject roles Agent/Theme and scores 4/8; sentence-1 Bob copied at frame 3; survivor frame map 0->0,3->1 with unchanged predicate token pairs; only sentence-0 participle added; only oblique slot 5 resolved; and accepted/proposed role transfers with exact Frame JSON preservation. U minus R excludes swapped lexical roles, global-Lex index substitution, Alice copied across sentences, dangling deleted frame indices, altered token coordinates, adding the wrong sentence token, changing recipient while resolving an oblique, rejected/off rule execution and changed non-target Frame fields. This is a source-derived assertion relation, not a proven complement for all Rust paths; malformed domains and unexecuted branches remain unknown. Reverse composition retains compatible supplied XML/table, clause, frame/tree and rule states sharing these coordinates; discarded frames, unmatched words and original XML layout are not recovered. Cost is reached conversion, matching, rule/remap and rendering work; no whole-system bound follows.

Complement: Universe U consists of the seven tagged supplied-input fixtures and candidate observations at their declared APIs. R requires local Lex class/frame (1,1,0), ordered Agent/Theme/Recipient roles and score 8; active/passive subject roles Agent/Theme and scores 4/8; sentence-1 Bob copied at frame 3; survivor frame map 0->0,3->1 with unchanged predicate token pairs; only sentence-0 participle added; only oblique slot 5 resolved; and accepted/proposed role transfers with exact Frame JSON preservation. U minus R excludes swapped lexical roles, global-Lex index substitution, Alice copied across sentences, dangling deleted frame indices, altered token coordinates, adding the wrong sentence token, changing recipient while resolving an oblique, rejected/off rule execution and changed non-target Frame fields. This is a source-derived assertion relation, not a proven complement for all Rust paths; malformed domains and unexecuted branches remain unknown.

Reverse: For each tagged output y retain all supplied antecedents x satisfying its stated relation and shared coordinates. Match indices require the exact retained local Lex; roles/scores do not recover XML formatting, unused classes, examples, filler identity or animacy. Head copying and kind changes lose overwritten values. Deletion loses removed frames and incoming references while retaining survivor predicate coordinates. Added participle frames do not recover a complete tree; pronoun resolution leaves multiple compatible prior discourse lists. Full Frame JSON recovers that Frame on the lawful serialization domain, not the originating tree. Patient/recipient transfer is uniquely reversible only with occupied source, empty destination and all other state retained. Parent reverse composition existentially retains shared intermediate Lex/Read snapshots, never arbitrary independently chosen coordinates.

Reached API costs only. Bounded native evidence is recorded in model.md#wave15-evidence; prior evidence retained.

## wave16-extension

Shared coordinates are the supplied Frame list and explicit permutation, scorer role/link indices, one-based retained Item batch positions, closed Act alphabet and source/document/paragraph identity. Notation precedes scoring; numbered rows precede TSV/metrics; batching precedes prompt numbering. These are separate tagged joins. Exclude unremapped links, duplicate occurrence overcredit, row/class substitution and cross-source/wrapping context suppression. Reverse fibers retain omitted frame detail, raw row order/text and unprinted metadata ambiguity. Costs include reached parsing/alignment/maps/rendering only; no general pipeline proof.

Complement: Universe: supplied Frame notation/permutation, numbered silver rows and Item batches/prompts paired with observed counts/labels/text. Exclude unremapped links, true positives exceeding system/gold occurrence counts, exchanged one-based row/class identities, partial duplicate-identifier acceptance, and context suppression across source collisions or overflowing paragraph transitions. These separate tagged transitions invoke no external teacher/parser.

Reverse: Retain compatible Frames, notation, rows, Items and batches sharing observed coordinates. Notation omits frame detail; counts cannot recover source text; accepted labels discard row order/junk spelling; identical prompts cannot recover all metadata. Reverse composition retains the frame permutation, one-based Item position, act alphabet and ordered batch references; many antecedents remain.

Reached API costs only. Bounded native evidence is recorded in model.md#wave16-evidence; prior evidence retained.

## wave17-extension

The pragmatic-observation child binds the CLI/library boundary and supplied Codex events -> validated reply -> attempted-call accounting -> numbered labels over the same Item batch. Retain batch identity, attempted cardinality, row positions and source/voice policy. Decode rejection precedes accounting in the tested adapter. Attempted counts do not establish accepted labels; ledger values and normalized rows have nonunique antecedents. Cost includes reached decoding, projection, sequential ledger I/O and gate work. No live teacher, corpus acquisition, durability or school-learning claim follows. Question-generation belongs to the separate school-support subtree.

Complement: Universe: supplied event traces, caller metadata, isolated ledger and numbered label rows over an ordered Item batch. Exclude accounting after decode failure, exchanged source/row labels, accepted labels for missing or duplicated rows, and treating attempted sentence count as accepted labels. A valid suffix cannot recover an earlier rejected trace. No process/crash/concurrency or teacher guarantee is modeled.

Reverse: Retain all traces, metadata, rows and Item lists sharing the observed reply, accounting and labels. Row k plus the same batch recovers Item k-1; ledger cardinality alone cannot recover accepted rows or text. Full, partial and duplicate answers can share usage and attempted counts. Unchanged ledger bytes admit multiple failure histories; event decoding and label parsing lose spellings and discarded events.

Reached API costs only. Bounded native evidence is recorded in model.md#wave17-evidence; prior evidence retained.


## Algorithm-transfer composition

The syntax-validation, coreference-observation and spatial-inference children are separate operation-tagged observations. Head-vector adapters retain preflight and first-error priority; total-priority edges retain greedy identities; Hungarian scratch reset retains rounded objective calculation; directional adjacency retains nonempty all-path qualifier state. Algorithms remain under algorithm-library. Source/model-bound transfers reject cycle-member, globally optimal greedy-replacement and shortest-path-only substitutions. See ../mova.groupant/model.md#transfer-evidence and transfer-benchmark.md there. Costs include reached child work, allocation and adapters; whole-application throughput is unmeasured.

## wave27-english-orchestration

main imports en modules declared by lib; license gate is only on train dispatcher, not a general evaluation gate. ctxeval consumes ctx::gold and ctx::Rng plus ud::Scores. diag calls parse baseline/kbest/constrained and uses gram enums. emode force/fix connect to annotation/tagger and embedded registry/dictionary; build output binary indices must match dict consumers (not independently validated here). Shape agreement by ID/token count does not prove form identity or valid trees.

## wave27-symbolic-arithmetic

expr supplies trees; eval joins rat and units with err taxonomy, then external fcheck SI comparison. arith uses big plus external memory NUM_WORDS. understand constructs Expr; solve consumes big helpers. Rat and big are not interchangeable representations: units remain checked i128.

## wave28-world-dialogue

ideas->nerves::norm, ideas->talk::now, talk->babi::split_sentences, main->vmm->lang/world. Act cue normalization and concept normalization are not equivalent adapters.

## wave29-world-rule-learning

family::change_line/change_of uses induce::Rule::machine/parse_rules, carrying scope separately. in_domain and apply_scoped transport changes without commutative-repair guarantee. big sample hand repairs precede induced rules; GPU counts only eligible correct words. worldbook::gather -> write -> brain_body feeds family brain; knowledge English serialization feeds state rules. retention_probe delegates Retained state, not legacy know cache.

## wave29-world-evaluation-boundaries

tests03's normalized phrasing relation must preserve above/below distinction while identifying agent renamings; load_txt reset is literal '1', not numeric equivalence. StepGame score ties map to left, so replacing missing model weights with abstention changes graph answers. bAbI relation adjacency uses subtraction, unlike StepGame's directly signed reciprocal graph; an adapter that copies edge orientation would invert displacement. Command parser errors happen before apply in fixtures; bAbI read increments t before parser failure and is not transactional. CLI configuration checks cannot establish legacy school failure/publication semantics.

## wave30-neural-parser-paths

data::batch roles index (k*T+d)*T+p; Net attention uses B,H,T,dh and masks head/key columns. lib transposes arcs dependent-major into Eisner head-major and maps local decoded h to flattened k*T+h for Net::rels. An adapter using raw h for k>0 gathers another sentence's encoder state. Vocs labels require same ordering as model output dimensions; load files independently and missing weights are not consistency proof. CLI score zips and can hide length mismatches, so output-length invariants must be checked before claiming dataset coverage. Existing world GPU caller uses parse_forms; tokenization remains caller-owned. Tests are sibling observers, not runtime production stages.

## wave31-rule-court-evaluation

Actual joins: rule::Rule nested Clause arrays are interpreted by court as condition AND-of-OR and exception OR-of-AND; juris::cover supplies No/Broad/Sure anchor relation, authority/severity determine ranking/outcomes. extract::rules_of output passes through attach_but_if then mark_disputes before loading/evaluation elsewhere; no direct corpus execution claimed. report::judge_all calls court choice_of_law/lattice/evaluate/tipping; absurd::flags calls court and report::harm_of. measure uses Rule and Jurisdiction but no verdict inference. lib declarations only expose APIs; tests are callers with hand-built state. CLI protocol hash observes disk bytes while report state uses in-memory rules, so equality is not implied.

## wave31-hebrew-corpus-adapters

Real joins: cue form/name->features->count/logp columns; tokenize byte spans->segments->talmud slices; MACULA Node0-based -> head1-based -> surface permutation -> emitted-row head mapping; macula html_text->tanakh NFC edition; read_sentence open_after->next open_before within book. Counterexamples: copying MACULA ordinal to Tanakh order misaligns books; treating Segment.end as talmud slice end loses punctuation; union of component complements is not complement of composed relation. L/M are tagged siblings only.

## wave32-hebrew-segmentation-lattice

Real joins: text::norm/units -> seg key and Piece semantics; conllu Token/Word -> gold and segmenter evaluation; lib::cut_with passed as callback by main lattice_cmd/choose_cmd; lib::segment_line calls split_in and cut_as; main score uses conllu heads and text normalization. Tagged independent paths: main split/prepare/Part partitioning, main score diagnostics, model TSV codec and annotate filesystem/subprocess adapter. Independent per-token lattice choices do not imply jointly optimal tree; positional merge count equality is not parser identity verification.

## wave32-roundtrip-layer-residuals

Real joins are pipeline -> frames/tree/world/tally, main -> pipeline/report, tests -> synthetic layer APIs. class supplies labels/costs. lib exposes API/path; reporting and path assembly are tagged independent from codec reconstruction. Root owns volume/mixed sibling joins.

## wave32-roundtrip-document-codecs

Real joins: blocks::concat→Volume; site_md::inline→vol::with_marks via Block::written; site_md::repeat→Copier and vol::residual_with mirrors copy decisions; site_code::encode_heb/decode_conllu→site_heb::predict_with/diff/apply; site_md::listed and run_blocks→heb::par_each. Independent tagged paths: site_learn CLI adaptation and heb::run/model parsing are not invoked by pure codec tests; numeric report helpers are projections, not part of reconstruction inverse. R1 is only a block-text residual: substituting it for full Volume layout loses literals. Site-code Chain requires same ordered canonical readings and resource state; leaf identities alone do not prove this full wiring.

## wave32-roundtrip-mixed-routing

Actual joins: sample::Span -> route/owner in mixed; heb::R1/R2/R3 -> mixed exact decode; link::rename/inverse -> world frame identity; diff::compare -> tally::Diverge; Run::merge -> Tally::merge -> report metrics. compare consumes independent residual/normal schemas, not mixed report::tsv's nine columns. These shared helper joins are not an invented sequential pipeline.

## wave32-translation-meta-codecs

Hebrew Reader train owns meta tokens through IBM then Strong override; owners feed HebEncoder train. meta MTok ctx_keys feed profile and HebEncoder; Fw/Rule remap their heads. Logic consumes event concepts via Lexicon and LCS. Strong is optional alignment input, not a translation oracle.

## wave32-translation-evaluation-pipeline

Real joins: bible::read_pool/sources text→corpus PVerse; main run_sys→measure::Run→Agg/VRec→block_measure and report::write; many→corpus/measure plus own pair metrics; pages receives wlc_jps VRec stage arrays from main and Hebrew encoder outputs. many::agree skips punctuation while block_meta includes it: their counts are not interchangeable. Report groups collapse layers. No invented call from report to many.

## wave33-math-library-ingestion

Real joins: raw::License::code and books::BookLicense::code match fetch's finite allowed strings; raw inputs match OAI gzip page naming; HTTP encode transports literal token bytes; robots uses Polite and fetch invokes Gate before downloading. Counterexample: GCS JSON numeric size becomes zero; it is not raw KB format. Book build's DNS means offline TSV construction is not a pure parser. SQL filling is sequential substitution, not DB semantics.

## wave33-cycle-teacher-diagnostics

Build MOVA_BIG_DEFAULT joins runtime nonempty MOVA_BIG precedence. Pipeline sends selected ids to main parser helpers, consumes db with_drafts and later updates DB before check/train/row. jobs wraps configured drafts. Teacher live test, bench main, lesson_smoke and twin ledger are explicitly independent paths; common diagnostics vocabulary is not a caller edge. Existing artifact markers may bypass stages, and failed collect can leave earlier DB commits.

## wave33-treebank-comparison-tools

Actual joins are proposal Edit positions into trial head/relation/retag vectors; baseline Bad equality into acceptance; Readme Item text/bib into Matcher with raw byte boundaries; Hit (Src,index,How,fuzzy) into collect Found and Card rendering; Counts into stats tuple and render totals. Synthetic readme_collect_render_join checks deduplication and rendering method/title. Parsecmp uses its own permissive reader and cannot substitute for treeagree read_ref: it skips range/dotted rows and uses sentinel heads where treeagree rejects nonsequential ten-column input.

## wave33-text-corpus-tools

Actual joins are within each tool; no cross-tool Rust caller is invented. A possible explicit artifact adapter maps Gutenberg fetched pg-id text into school book_paragraphs -> sentences, losing headers/underscores/short fragments. OCR output is Unicode line text and is not directly a school address TSV. Wiktionary page raw wikitext can be supplied to count/emit helpers only with edition/title/namespace context; these tools do not call one another. Tests exercise actual local helper joins only; cross-tool adapters remain source-derived.

## wave33-bibliography-value-tools

sources::bib public namespace is a concrete call edge. bib row's paper/open or open-nc/null metadata fields deserialize into Resource and FileRef. run_ids may fail before write; plan loads all before target effects. Merge accepts broader Value shapes than load: merge success is not sufficient load validation. Value main never calls these helpers; composition is tagged independent paths, not sequential execution.

## catalog

# Typed interaction models

All relations have the assumptions and dependency limits in model.md. These are
source-derived partial models; check references do not imply universal proof.

## excluded-stage-observations

Type: behavior_complement.

U contains tagged finite observations of registered APIs under child domains and fixed dependencies/schedules. An inner bound on U minus R excludes changed-cardinality successful worker mapping, reversed equal-score fact-number ordering and successful cache admission of partial parse batches. Child complements do not compose by simple union.

Evidence: scoped child source/probe records. Oracle: preserve tagged domains and compatible intermediate identities; no universal inverse is claimed.


Wave15 extension: Universe U consists of the seven tagged supplied-input fixtures and candidate observations at their declared APIs. R requires local Lex class/frame (1,1,0), ordered Agent/Theme/Recipient roles and score 8; active/passive subject roles Agent/Theme and scores 4/8; sentence-1 Bob copied at frame 3; survivor frame map 0->0,3->1 with unchanged predicate token pairs; only sentence-0 participle added; only oblique slot 5 resolved; and accepted/proposed role transfers with exact Frame JSON preservation. U minus R excludes swapped lexical roles, global-Lex index substitution, Alice copied across sentences, dangling deleted frame indices, altered token coordinates, adding the wrong sentence token, changing recipient while resolving an oblique, rejected/off rule execution and changed non-target Frame fields. This is a source-derived assertion relation, not a proven complement for all Rust paths; malformed domains and unexecuted branches remain unknown.


Wave16 extension: Universe: supplied Frame notation/permutation, numbered silver rows and Item batches/prompts paired with observed counts/labels/text. Exclude unremapped links, true positives exceeding system/gold occurrence counts, exchanged one-based row/class identities, partial duplicate-identifier acceptance, and context suppression across source collisions or overflowing paragraph transitions. These separate tagged transitions invoke no external teacher/parser.


Wave17 extension: Separate tagged domains. Universe: supplied event traces, caller metadata, isolated ledger and numbered label rows over an ordered Item batch. Exclude accounting after decode failure, exchanged source/row labels, accepted labels for missing or duplicated rows, and treating attempted sentence count as accepted labels. A valid suffix cannot recover an earlier rejected trace. No process/crash/concurrency or teacher guarantee is modeled. Universe: supplied Para/Sent/Pick/skip, versioned schema/prompt and independent TSV observations. Exclude skipped/ineligible selections, source-coordinate substitution for local p/anchors, invented labels, V2 explicit nonanchor/context spans and punctuation-changing spans. Prompt semantic constraints not enforced by the gate remain outside this complement; no acquisition/teacher behavior is inferred.

## compatible-worlds

Type: reverse_reconstruction.

For a tagged observation retain every compatible input, world/history, typed value, lexical dependency and latent schedule. Join identities along actual call edges. Formatting, projection and duplicate-key overwrite lose information; unique full reconstruction is not established.

Evidence: scoped child source/probe records. Oracle: preserve tagged domains and compatible intermediate identities; no universal inverse is claimed.


Wave15 extension: For each tagged output y retain all supplied antecedents x satisfying its stated relation and shared coordinates. Match indices require the exact retained local Lex; roles/scores do not recover XML formatting, unused classes, examples, filler identity or animacy. Head copying and kind changes lose overwritten values. Deletion loses removed frames and incoming references while retaining survivor predicate coordinates. Added participle frames do not recover a complete tree; pronoun resolution leaves multiple compatible prior discourse lists. Full Frame JSON recovers that Frame on the lawful serialization domain, not the originating tree. Patient/recipient transfer is uniquely reversible only with occupied source, empty destination and all other state retained. Parent reverse composition existentially retains shared intermediate Lex/Read snapshots, never arbitrary independently chosen coordinates.


Wave16 extension: Retain compatible Frames, notation, rows, Items and batches sharing observed coordinates. Notation omits frame detail; counts cannot recover source text; accepted labels discard row order/junk spelling; identical prompts cannot recover all metadata. Reverse composition retains the frame permutation, one-based Item position, act alphabet and ordered batch references; many antecedents remain.


Wave17 extension: Preserve each tag and its own intermediates; no cross-tag bijection. Retain all traces, metadata, rows and Item lists sharing the observed reply, accounting and labels. Row k plus the same batch recovers Item k-1; ledger cardinality alone cannot recover accepted rows or text. Full, partial and duplicate answers can share usage and attempted counts. Unchanged ledger bytes admit multiple failure histories; event decoding and label parsing lose spellings and discarded events. Retain every input vector, metadata, raw sentence text, Pick/skip and response sharing selected identities, prompt/gate observations and quotas. The same retained batch plus p/anchor recovers source doc/para/sent_id; gate output alone cannot. Case/quote/whitespace normalization has many raw preimages, selection loses rejected content and quotas lose weight/cap details.

## sentence-anchors

Type: interface_obligation.

The caller must preserve source order and map valid one-based fact sentence anchors into world text. Fact.sent=0 is outside the normal scoring domain because subtracting one can underflow; valid local parsing does not establish this caller obligation.

Evidence: source inspection and the scoped child checks listed in model.md#tests.

## score-fibers

Type: score_preimage.

A total score has many predicate/participant/word/section decompositions. Duplicate other-participant or word-query entries can contribute repeatedly; no set-quotient equivalence is assumed.

Evidence: source inspection and the scoped child checks listed in model.md#tests.

## rank-fibers

Type: tie_order_preimage.

Search orders scores descending, then fact number ascending. Equal score and equal fact number preserve stable input order; a rank position cannot determine a unique source chronology or score decomposition.

Evidence: source inspection and the scoped child checks listed in model.md#tests.

## cost-decomposition

Type: resource_approximation.

Sum only reached parsing, unknown world-building, extraction and scoring/sorting work. Worker balance, allocator costs and parser model loading remain separate. Nested models of scoring must not be counted twice.

Evidence: source inspection and the scoped child checks listed in model.md#tests.

## cache-execution-join

Type: duplicate_write_projection.

For pre-call map M and request texts T, when refresh=false missing positions are computed from membership in M before any insertion; refresh=true selects every request position. Both preserve order and duplicates. Exact-length success inserts pairs in that order, so the last parsed occurrence of a key wins for every output occurrence. Cache equivalence to independent per-occurrence parsing requires an identity-scoped text-extensional parser and compatible entries already in M; cardinality alone is insufficient. Counters record pre-call hit/miss attempts even when the parser errors or returns the wrong length; those failures preserve M. finish separately checks identity. Native finite oracles cover duplicate-output preimages, membership complements, failed/retried histories and envelope/refresh boundaries.

Scope: finite tagged calls with child domains, dependencies and latent schedule retained. Evidence: source inspection; child tests record executed scope.

## frame-parser-boundary

Type: pipeline_non_equivalence.

frames::read preflights tree::annotator and uses read_with through parse_words/Tree::parse_cased rather than locally calling cache or exec. Shared configuration does not prove path equivalence, especially with GPU/repairs. Oracle: independent agreement fixtures before equating outputs.

Scope: finite tagged calls with child domains, dependencies and latent schedule retained. Evidence: source inspection; child tests record executed scope.

## historical-query-fiber

Type: mixed_time_projection.

where_at selects an object version at requested sentence; where_ver uses that version's sentence for holder location. Labels/place chains use current snapshots. Output does not uniquely reconstruct a consistent historical snapshot. Oracle: holder movement after the object's last version.

Scope: finite tagged calls with child domains, dependencies and latent schedule retained. Evidence: source inspection; child tests record executed scope.

## span-selection-boundary

Type: caller_selection_obligation.

span::extract sees one supplied tree/QFrame. Reader selection is external. Num bypasses predicate/clean/qlem; Feel may scan the whole tree; early rejected candidates can return None without alternatives. Oracle: controlled trees for bypass/rejection and separate selection checks.

Scope: finite tagged calls with child domains, dependencies and latent schedule retained. Evidence: source inspection; child tests record executed scope.

## typed-display-boundary

Type: representation_preimage.

Public types admit values beyond parser/constructor canonical subsets. Display collides across fields/values; Id round trips need a bounded domain. Preserve typed identities instead of rebuilding them uniquely from labels. Oracle: colliding variants and out-of-range IDs.

Scope: finite tagged calls with child domains, dependencies and latent schedule retained. Evidence: source inspection; child tests record executed scope.

## determinism-domain-meet

Type: latent_schedule_constraint.

Current value judgments use ordered fields/goals and prior-command harm under fixed tables. This local determinism does not establish group-wide determinism: parallel store build and independently read metadata retain scheduling/snapshot assumptions. Oracle: current finite value transitions and separate store concurrency/capture obligations.

Scope: finite tagged calls with child domains, dependencies and latent schedule retained. Evidence: source inspection; child tests record executed scope.

## cache-storage-join

Type: publication_trace.

Cache finish checks dependency identity, serializes an envelope, then calls store::write_atomic. Under single-writer and same-filesystem rename assumptions, success replaces one path. Failed writes can leave temporary bytes; same-PID names sharing a stem may alias. There is no fsync, multi-file transaction or stale-writer union. Store build/update/capture has separate ordering constraints and must not inherit stronger guarantees from this helper.

Oracle: child temporary-file replacement/failure probes and stale-writer examples. Actual concurrent races, durability and full store-build traces remain unexecuted.

## sampled-set-consistency

Type: materialized_view_consistency.

Under the store child's fixed-corpus, valid-files and single-writer assumptions, loading an outdated sampled set evaluates Q_new once, persists that exact vector and returns it. The relation binds the query, requested model, selected vector and written set bytes; it does not reconstruct which old sampled candidates were displaced. Current/stable stamps trust their stored IDs, so a stamp is not authentication of edited content. Pointwise ancestor carry still requires membership agreement outside the changed-ID union. Prefix refresh also preserves query vector order, which a set equality alone would not establish.

Oracle: source-level sampled carry regression, bounded query/persistence comparisons and legacy unstamped-prefix fixture. The set-algebra law and standalone finite sampling model provide separate mathematical evidence. No concurrent publication, delta-extraction completeness or end-to-end school claim follows.

## annotation-derivation-join

Type: annotation_projection_join.

Let P retain stored structure, tag and the parent-resolved lowercase lemma sequence. The updated equality helper admits unchanged records only when these components match, under valid enum/width and parent-table assumptions. Lexical indexing and packed derivation inputs observe different projections of P: subject features consume lemmas; rule scanning consumes lexical classes and finite-tag flags. Equal full projections imply equal deterministic pointwise observations, while equal indexed features need not imply equal tags or records. The Lean completeness bridge formalizes the abstract implication, not the Rust encoder.

The caller must preserve version/lemma identity. scan_store captures a version, then lemma_table separately rereads the latest reference; concurrent updates can break that association. Delta generation also relies on successful record reads, stable IDs and a valid writer. The bounded synthetic fixture exercises the actual equality seam, inherited records, index_full, selection and set carry with independently encoded records. It does not run annotation, the update writer or full rule learning. Earlier stored snapshots are not rewritten by the repair.

Oracle: omitted-lemma/tag source mutants, lexical query transition fixtures and the derivation child's class/packing probes. Complete source bindings include both store and derive; shared binding alone does not establish these runtime preconditions.

## history-display-boundary

Type: mixed_time_observation_boundary. History retains matching Set deltas including equal assignments; label uses current fields with priority/fallthrough; resolve applies a lexical rule; context partitions mention counts. A label or attention partition cannot identify a unique update history. Four native finite domains check these projections independently. Event/cursor fields are empty in context fixtures. Command validation restricts relation endpoints to characters; forged cyclic relations are outside terminating label evidence.

## span-projection-boundary

Type: normalized_span_preimage. Cleanup projects supplied token indices onto sorted, deduplicated, trimmed/filtered indices. Predicate selection and fallback retain separate priority rules. Finite cleanup/rank fibers include every enumerated compatible input; selected extraction fixtures expose shadowing and ties without exhaustive inverse claims. Valid Tree structure, chosen sentence, question lemmas and fixed dependencies are premises. Query correctness does not prove that callers select the right tree or answer span.

## tree-question-world-boundary

Type: validated_representation_boundary.

Tree cached graphs, QFrame syntax projections and World typed-command transitions retain different validity premises. The executed native join is lang::parse_all(program) -> world::run(text, parsed) -> historical/current snapshots. Its five-command fixture commits four commands and rejects a move to a missing location while preserving the fixture's last committed state. This does not prove all-error rollback, parser accuracy or a complete text-to-answer composition. Keep input text, command references and snapshot identity together. Child finite preimages do not become independent Cartesian factors across this shared state.

## cached-child-cycle-cut

Type: functional_graph_cycle_cut.

Constructor-built cached child graphs give each vertex at most one incoming edge. If a cycle reachable from traversal start i were first entered at c != i, c would have both the cycle predecessor and the distinct incoming path predecessor. Therefore every reachable cycle contains i. Tree::sub skips re-enqueueing i, terminating these traversals without an allocated visited set while retaining valid-tree outputs. An independent visited-set oracle checks 86,078 bounded head/root/cut cases. This is bounded correspondence evidence plus a source-level argument, not a machine-checked theorem. Direct mutation of public w can stale lengths, parents and caches; this change does not repair that representation boundary.

## current-history-display

Type: snapshot_display_join.

For parsed commands C and fixed Text T, let R=world::run(T,C), E be an entity
in R.world, and D(v)=md::snap_str(R.world,E.id,v). Version selection observes
E.snap(v), while labels/place chains can consult the same world's current
registries. Therefore D(v) is a human projection of the selected state with
current naming dependencies, not a lossless historical snapshot serialization.
The full report couples R.accepted/rejects/uncovered and errors to the same Run;
it does not recompute acceptance or execute rejected commands.

The native parsed_world_history_joins_markdown_observations fixture parses five
commands, accepts four, rejects a missing-location move, checks Alice's old Hall
and current Garden states, and observes those names/counters/anchors in Markdown.
This checks actual lang->world->md wiring. It does not establish universal
rollback, arbitrary historical label stability, full Markdown escaping or
injective report reconstruction. The fixture contains three text anchors even
though the rejected command leaves the last valid state unchanged.

## quantity-observation-tags

Type: numeric_language_boundary.

Retain the operation tag when comparing opchoice candidates, quant exact
rationals and treeread scalar/schema answers. In the bounded nonnegative
integer domain, opchoice admits nonnegative subtraction and exact positive-
divisor division; quant can represent rational/negative outcomes beyond that
candidate domain. A serialized candidate expression evaluated through quant
must equal that candidate's integer value embedded as a rational, with valid
anchor and accepted literal membership. This is a one-way domain restriction,
not equivalence of the languages or of natural-language interpretations.

operation_candidates_join_exact_quantity_scripts enumerates all triples from
0..3 and compares candidate structures/values with independent widened
arithmetic, then parses and runs every returned expression at anchor one. Its
literal sentence supplies the operands. Zero division is absent from admissible
candidates; the integrator reports a compiled adapter control for division zero.
Final control counts/profile evidence are recorded by integration, not inferred
here. Quant's literal-prefix gate constrains numeric literal availability only:
it does not establish reference provenance, units or semantic relevance.
Origin errors can record diagnostics while permitting effects, unlike
evaluation/anchor failures. Keep state, answer and error trace together.
Treeread's f64-to-integer surface conversion, schema priority and bounded i64
arithmetic remain a separate observation tag.

## reader-question-ownership

Type: dependency_identity_join.

The answer-observation subtree owns reader, ans, qud and sense models; this
ownership is not a serial call chain. Reader calls ans helpers and external
qframe/index/span, while qud consults sense::animacy for eligible noun Who
fillers. Source-world facts, own-world text, selected tree and question frame
must retain their identities along the reader joins. Repaired checked anchor
lookup rejects zero/out-of-range tree indices but does not validate a slot's
output anchor or establish that two worlds refer to the same text.

question_frame_reader_join_preserves_multiset_coverage checks 512 supplied
Tree/QFrame combinations: repeated retained words count repeatedly in coverage,
whereas qlemmas collapses duplicates. Its expected counts and sets are independent.
sense_animacy_joins_who_question_eligibility checks four noun answers against
actual sense::animacy; that is a dependent wiring oracle, not independent
factual animacy evidence. Confidence is a selection heuristic, QUD completeness
is a slot/transition property, and neither is truth probability. ans event
sections are preferences, ans baseline sections are hard filters, and reader
section envelopes include gaps; do not impose one policy on every API.
QUD complete attachment removes only the selected latest eligible question;
partial attachment preserves the stack and can block older complete matches.

## restricted-csv-join

Type: restricted_format_equivalence.

Let encode surround each field with quotes and double internal quotes. For one
nonblank header and one encoded record whose fields contain no physical newline,
tabular::read_csv(path) and ftqa::csv(bytes) agree on header/row projections in
the 25 two-field fixtures over empty, plain, comma, quote and Unicode values.
The same read table passed to read_world maps empty categorical fields to the
literal unknown and leaves single-record links empty. This final projection
is lossy: original empty and literal unknown values need not be reconstructible.

The test tabular_and_fairytale_csv_agree_on_single_line_domain writes a temporary
file, calls the real two parsers, and checks independently specified fields and
attributes. It is actual bounded IO composition, not a general CSV equivalence.
tabular splits physical lines before its permissive quote scanner; multiline,
blank-line and malformed-quote/width policies are outside the equivalence
domain. ftqa now has an adjacent scoped file groupant under answer-observation;
its separate native fixtures do not establish full parser equivalence.

## narrative-time-boundary

Type: partial_order_world_identity.

Narrative time is a partial order over scoped event identities, not the source text position. Preserve observer, frame, world and canonical-ID mappings across registration and queries. Strict cycles are rejected; equal or incomparable times require their own observation tags. Public World::apply rollback and lower-level mutation have distinct transaction boundaries. Supplied-input joins check temporal operand direction, New-world closing and atomic rejection. They do not certify every merge path or malformed public mutable state. Reverse reconstruction admits multiple event graphs and histories compatible with the same answer.

## rerank-score-domain

Type: score_selection_domain.

Rerank preserves parser scores, penalties and development-selected lambda. NaN inputs and invalid penalties/lambda reject; infinite parser scores retain comparison behavior. The configured grid avoids extreme finite-product overflow; arbitrary magnitudes remain limited. Finite choice/bootstrap fixtures do not validate significance or held-out accuracy; parser/repair paths remain source-inspected.

## argument-score-domain

Type: configured_argument_judgment.

Supplied Word annotations preserve positions through Node projection, Matrix lookup, judge and sense argument reasons. Unsupported roles return None. A missing matrix cell can abstain without gap provenance; an unrated idiomatic object can bypass the known layer. The configured lexical judgment is not a universal semantic truth. Finite native joins exercise supplied annotations; parser and repair callers were inspected, not executed.

## inference-core-wave12-mind-input-partition

Type: supplied_story_transition.

Mind segments supplied text before sequential Story reads; heading/blank-only inputs project to no sentences and preserve the supplied Story identity/state. A trailing question selects ask rather than tell; response wording projects returned answers/gaps. Empty-input identity is executed in leaf checks. Nonempty parsing, socket handling, prompt loading and persistent state are source-inspected only. Equal replies cannot reconstruct input text, prior Story or lost gap provenance.

Wave12 source-derived view; the supplied domain and observation are above. Executed scoped integration evidence is recorded in world/tests/groupant_wave12.rs.groupant/model.md#evidence. Fixed dependencies and shared identities are premises; finite native joins do not establish full-pipeline behavior or recover unique input history.

## inference-core-wave13-pragmatic-identity-coupling

Type: shared_coordinate_composition.

Pragmatic feature/classifier/schema/inference observations compose with existing inference-core branches only through explicit identical dictionary snapshots, field class alphabets, row IDs and supplied schedules. Numeric feature IDs from distinct dictionaries cannot be substituted; missing-prefix start fallback cannot be treated as gold or future context. eval.rs fit_set/predict_seq/scores and data.rs Item are inspected unregistered dependencies, not newly owned leaves. Preserve established frame/lexical dependencies and avoid treating these tagged tests as a single CLI pipeline. Reverse composition retains all compatible shared dictionaries/schedules and antecedent rows; equal outputs do not recover training history or unique text.

Source-derived partial relation. Wave13 bounded integration evidence is recorded in model.md#wave13-evidence; prior evidence is retained separately.

## inference-core-wave15-frame-rule-lexicon-coordinates

Type: supplied_pragmatic_coordinate_composition.

Shared coordinates are Read frame i, Read.pred[i]=(sentence s, zero-based predicate token p), Tree.w[p] with one-based UD heads, participant slot k, and local Lex indices (class,fclass,frame). Conversion output TSV feeds that same local Lex; global Match accessors do not resolve local indices. Rule edits preserve sentence-qualified lookup; deletion maps retained frame references through one survivor map; rendering and schema views preserve selected roles/acts. The seven tagged fixtures are separate joins, not a general parser pipeline. Universe U consists of the seven tagged supplied-input fixtures and candidate observations at their declared APIs. R requires local Lex class/frame (1,1,0), ordered Agent/Theme/Recipient roles and score 8; active/passive subject roles Agent/Theme and scores 4/8; sentence-1 Bob copied at frame 3; survivor frame map 0->0,3->1 with unchanged predicate token pairs; only sentence-0 participle added; only oblique slot 5 resolved; and accepted/proposed role transfers with exact Frame JSON preservation. U minus R excludes swapped lexical roles, global-Lex index substitution, Alice copied across sentences, dangling deleted frame indices, altered token coordinates, adding the wrong sentence token, changing recipient while resolving an oblique, rejected/off rule execution and changed non-target Frame fields. This is a source-derived assertion relation, not a proven complement for all Rust paths; malformed domains and unexecuted branches remain unknown. Reverse composition retains compatible supplied XML/table, clause, frame/tree and rule states sharing these coordinates; discarded frames, unmatched words and original XML layout are not recovered. Cost is reached conversion, matching, rule/remap and rendering work; no whole-system bound follows.

Source-derived partial relation. Wave15 bounded integration evidence is recorded in model.md#wave15-evidence; prior evidence is retained separately.

## inference-core-wave16-scorer-silver-coordinates

Type: supplied_scoring_annotation_coordinates.

Shared coordinates are the supplied Frame list and explicit permutation, scorer role/link indices, one-based retained Item batch positions, closed Act alphabet and source/document/paragraph identity. Notation precedes scoring; numbered rows precede TSV/metrics; batching precedes prompt numbering. These are separate tagged joins. Exclude unremapped links, duplicate occurrence overcredit, row/class substitution and cross-source/wrapping context suppression. Reverse fibers retain omitted frame detail, raw row order/text and unprinted metadata ambiguity. Costs include reached parsing/alignment/maps/rendering only; no general pipeline proof.

Source-derived partial relation. Wave16 bounded integration evidence is recorded in model.md#wave16-evidence; prior evidence is retained separately.

## inference-core-wave17-teacher-accounting-label-coordinates

Type: teacher_annotation_boundary.

The pragmatic-observation child binds the CLI/library boundary and supplied Codex events -> validated reply -> attempted-call accounting -> numbered labels over the same Item batch. Retain batch identity, attempted cardinality, row positions and source/voice policy. Decode rejection precedes accounting in the tested adapter. Attempted counts do not establish accepted labels; ledger values and normalized rows have nonunique antecedents. Cost includes reached decoding, projection, sequential ledger I/O and gate work. No live teacher, corpus acquisition, durability or school-learning claim follows. Question-generation belongs to the separate school-support subtree.

Source-derived partial relation. Wave17 bounded integration evidence is recorded in model.md#wave17-evidence; prior evidence is retained separately.

## inference-core-wave26-en-src-conllu-rs-file_layers

Type: file_layers.

Layers are computed over all returned syntactic words: nonempty parsed known FEATS, lemma != underscore, form != underscore; present iff count*20>=W, inclusive 5%. read emits no sentence at W0; comment-only Doc sentences exist and all three flags become true because 0>=0. Refresh preserves existing flags rather than recomputing a file-wide decision.

Source relation: en/src/conllu.rs.groupant/model.md#file_layers. Native evidence remains in the source sidecar.

## inference-core-wave26-en-src-conllu-rs-head_gate_boundary

Type: head_gate_boundary.

Both read and refresh parse raw HEAD using usize.parse().unwrap_or(0), so malformed raw strings can look like roots in the derived view. Doc keeps the raw string losslessly. Consumers needing trees must validate raw heads through tree_gate; Doc acceptance and a derived zero are not a certificate.

Source relation: en/src/conllu.rs.groupant/model.md#head_gate_boundary. Native evidence remains in the source sidecar.

## inference-core-wave26-en-src-conllu-rs-failure_precedence

Type: failure_precedence.

read opens/streams the file, skips short and range/decimal lines before relation parsing. Doc performs the whole lexical scan before refreshing sentences, so a later lexical error can precede an earlier semantic ID/relation error. Refresh checks each word ID before that word relation. I/O errors propagate, vector allocation may abort; no callbacks are invoked.

Source relation: en/src/conllu.rs.groupant/model.md#failure_precedence. Native evidence remains in the source sidecar.

## inference-core-wave26-en-src-conllu-rs-finite_checks

Type: finite_checks.

Four bounded tests cover synthetic byte round trips, lexical/semantic rejections and two repaired state transitions. Embedded full-treebank test is explicitly skipped. Passing finite fixtures neither proves the parser for every UTF-8 string nor observes external corpora.

Source relation: en/src/conllu.rs.groupant/model.md#finite_checks. Native evidence remains in the source sidecar.

## inference-core-wave26-en-tests-groupant_wave26-rs-root_basin

Type: composition_relation.

Type: composition_relation.

After root-count and raw-head preflight, tree_gate accepts iff every syntactic vertex shares cycle_id[0]. Since successor[0]=0, this cycle is exactly [0]. Exact first cycle error is min i>=1 with cycle_id[i]!=cycle_id[0]; tails into a nonroot cycle are included.

Source relation: en/tests/groupant_wave26.rs.groupant/model.md#root_basin. Native evidence remains in the source sidecar.

## inference-core-wave26-en-tests-groupant_wave26-rs-independent_walk

Type: oracle_relation.

Type: oracle_relation.

For each start, at most n head steps reach zero iff it terminates; otherwise a nonzero vertex repeats. This independent bounded-walk predicate corroborates cycle membership without using production decomposition state or tree-gate internals.

Source relation: en/tests/groupant_wave26.rs.groupant/model.md#independent_walk. Native evidence remains in the source sidecar.

## inference-core-wave26-en-tests-groupant_wave26-rs-error_order

Type: error_partition.

Type: error_partition.

Doc rejects malformed field count, nonconsecutive IDs and unknown relations before a gate call. Accepted representation can still contain malformed HEAD, range errors, relation/root mismatch, zero/multiple roots or cycles. Gate preflight scans all raw rows before cycle detection; a later malformed head therefore precedes an earlier self-loop.

Source relation: en/tests/groupant_wave26.rs.groupant/model.md#error_order. Native evidence remains in the source sidecar.

## inference-core-wave26-en-tests-groupant_wave26-rs-discarded_rows

Type: coordinate_relation.

Type: coordinate_relation.

Doc retains MWT range and decimal empty-node rows. words indexes only syntactic rows, while head coordinates refer to one-based syntactic words, never raw row offsets. Adding ignored rows preserves gate result and decomposition while changing raw row indices.

Source relation: en/tests/groupant_wave26.rs.groupant/model.md#discarded_rows. Native evidence remains in the source sidecar.

## inference-core-wave26-en-tests-groupant_wave26-rs-negative_control

Type: counterfactual_relation.

Type: counterfactual_relation.

Compile a harness adapter mutant replacing extend_from_slice(heads) by extending heads mapped with saturating_sub(1). The named coordinates_first_failure_and_threshold exact adapter assertion must fail; restore byte-identical source and rerun positively.

Source relation: en/tests/groupant_wave26.rs.groupant/model.md#negative_control. Native evidence remains in the source sidecar.

## inference-core-wave26-en-tests-groupant_wave26-rs-finite_coverage

Type: coverage_relation.

Type: coverage_relation.

Exhaustive small vectors cover all self-loops, multiple cycles and tails compatible with one root. Fixed 64/65/66 cases cross the production gate implementation threshold. Passing does not establish large-input correctness, arbitrary parser equivalence, allocation safety or formal proof.

Source relation: en/tests/groupant_wave26.rs.groupant/model.md#finite_coverage. Native evidence remains in the source sidecar.

## wave27-english-orchestration-context-transport

Doc nonempty sentence order→Ctx/docs; train_ctx filters tagged sentences and offsets document IDs; test_ctx validates pass1 IDs/counts; masking occurs after derivation/shuffle. ctxeval System loader checks IDs/counts but not forms. Crossing these adapters requires explicit order alignment, not just equal lengths.

## wave27-english-orchestration-effect-prefix

Build reads then writes lexicon/layout/words sequentially; folds creates dir then fold/rest files; train saves before reload comparison; conversion streams completed files before later failure. A failure may retain prefixes and no rollback is modeled.

## wave27-english-orchestration-coordinate-join

Token vector index i maps head ID i+1; zero is root. Context entity features use zero/no-layer, one/outside, >=2 encoded type/status. Build offsets distinguish byte offsets from record counts and symbol IDs; none are interchangeable.

## wave27-english-orchestration-uncertainty-join

Compile visibility of lib is not execution evidence. Fourteen finite tests and seven mutants falsify selected contracts only; unknown live paths and unsupported percentage approximants remain explicit. Bootstrap intervals are sampling summaries, not proofs of real-task improvement.

## wave27-symbolic-arithmetic-representation-transport

Q.base joins Rat value with Mono factor or affine offset; gate compares this SI observation with fcheck. big Q used in arith has no automatic unit or checked-i128 transport.

## wave27-symbolic-arithmetic-failure-order

Children precede parent validation; prior Ctx updates survive failure. Thermal kind rejection precedes numerical zero division. Units reject dimensions before scale arithmetic.

## wave27-symbolic-arithmetic-exact-replay

Selected arith Tree exact replay certifies only its arithmetic over retained quantities, not float beam completeness, learned relevance or gold tolerance equality.

## wave27-symbolic-arithmetic-resources

No general allocation/depth cap; subset masks assume n<32 and unit exponents assume representability. Existing probability enumeration bounded to16 only after count sum.

## wave28-world-dialogue-normalization-join

ideas words = set(filter_map(nerves::norm, alphanumeric tokens)); talk cue matching has different boundaries and cannot substitute for nerves normalization.

## wave28-world-dialogue-coordinate-join

VMM splice uses 1-based physical lines while prompt partitions use 1-based inclusive sentence anchors; changing line N does not imply changing sentence N.

## wave28-world-dialogue-effect-order

Main activates state before dispatch; VMM done refusal precedes partition and teacher; ideas atomic write failure is swallowed, unlike VMM file errors.

## wave28-world-dialogue-resource-join

Three diffusion frontiers bounded at 4000 coexist with unbounded total activation and whole-file ideas/memory loading; teacher cost has no measured time bound.

## wave29-world-rule-learning-rule-scope-join

Change carries Rule but not scope; family separately serializes scope and reattaches it. Replace retains each preexisting scope. Applying ordered labels need not commute, so independent candidate gains do not sum; combined child guard required.

## wave29-world-rule-learning-count-kernel-join

CPU Feats[si][i] nonzero21bit codes -> GPU flat codes plusu32 offsets and enum labelu8->u32; group dense index determines count slot. Correctness assumes offset/count widths and unique table keys; no GPU execution witness.

## wave29-world-rule-learning-brain-book-join

worldbook write emits<=3citation notes/fact, brain_body takes first note and<=12lines/part but returns total cited lines. family uses total as report and truncated body as stored knowledge; neither recovers all Fact attrs/support.

## wave29-world-rule-learning-effect-order

Family hot write precedes state transition/guard; ignored rollback errors prevent transaction claim. Retention probe rejects argc before reads, parses train before exam, saves before equality. Teacher failure and filesystem failure are not numerical approximation errors.

## wave29-world-evaluation-boundaries-coordinate-transport

bAbI offset from reference subtracts relation adjacency vectors; StepGame edge reference→subject adds Reader vector. Both mean subject minus reference but edge storage is opposite. Model each adapter explicitly; tests use c→a displacement(1,1) and C→A displacement(-2,0).

## wave29-world-evaluation-boundaries-transaction-boundary

Command fixture world parses all lines before any apply; tests record unchanged observed state on rejects. bAbI read increments t before annotator and may update subjects before unknown-class failure. CLI Command setup has no binary startup; successful setup does not certify failed model handling.

## wave29-world-evaluation-boundaries-observation-fibers

World test snapshots omit K for claim invariance and inspect only selected version/event observations for rollback. Partial time order preserves Unknown for incomparable events; no numeric duration inferred. Story state_lines is a different lossy projection and cannot be substituted.

## wave29-world-evaluation-boundaries-execution-scope

17 selected bounded tests and four compiled controls exercise helpers/state/grammar. The two integration sources' legacy process-running tests remain excluded, as do model-backed readers and stored corpus fixtures. Scratch cleanup is observed under normal OS success only.

## wave30-neural-parser-paths-coordinate-join

Batch row=k*T+i; tensor arc cell=(k*T+d)*T+h; decoder sc[h][d] receives that cell; relation head=k*T+h. Root index0 is local per sentence and never a real loss row.

## wave30-neural-parser-paths-gradient-join

Train=true selects custom fused VJP or reference attention, retaining Q/K/V plus embedding/LN gradients; inference fused loses Q/K gradient while V still reaches loss. Checking any QKV gradient alone is insufficient.

## wave30-neural-parser-paths-checkpoint-effects

CLI metadata writes precede epoch loop, checkpoints only strict improvements on full-dev/no-dev epochs, log follows. Failures may leave partial output. Selected synthetic optimizer mutates only ephemeral variables.

## wave30-neural-parser-paths-score-metric

Scored prediction sums arc and raw relation plus half tag log probabilities before root/dep label rewriting. CLI percentage metrics count zipped tokens, whereas source tests assert prediction lengths explicitly; neither is normalized tree posterior.

## wave31-rule-court-evaluation-clause-boolean-transport

Rule codec preserves nested grouping only on delimiter-safe canonical domain. Conditions use semicolon groups with pipe alternatives; exceptions semicolon groups with plus conjunctions. court chooses max condition Status and min exception Status. Swapping separators is a counterexample to correct wiring.

## wave31-rule-court-evaluation-duty-anchor-projection

An obligation anchored on its act records duty0 when positive and duty2 when negated; completed duty is skipped before cap/evaluation. Condition-anchored duty uses clause_status, with Unknown/Broad ->1. Verdict label uses first act_verb, not patient text; no retained matched-rule witness exists for skipped duty.

## wave31-rule-court-evaluation-lookup-to-precedence

Jurisdiction Cover selects anchor strength and potential Reading point; authority first substring supplies rank; severity last max-rank match supplies Liability severity. Evaluate applies chosen principle strict key comparison with earlier-body ties, then suspension. Changing one order changes winning trace even if same rule set.

## wave31-rule-court-evaluation-count-to-report

count groups local assignment fibers and full global Cartesian assignments, while tipping explores only limited distinct-point alternatives. Report reachability within depth is a subset observation, not full-lattice exclusion; sensitivity compares one-tip verdicts to other counted verdicts.

## wave31-rule-court-evaluation-protocol-state-boundary

protocol hashes currently read disk rules.tsv, default empty on read failure, while lattice records use current in-memory Jurisdiction rules and caller-supplied report text. Coherence requires these independently supplied observations to refer to the same snapshot; FNV collisions and concurrent file changes defeat unique reconstruction.

## wave31-hebrew-corpus-adapters-byte-coordinate

Token boundaries are UTF-8 source offsets; Segmenter spans last token end but talmud slices to next start/text length. Punctuation stays preceding segment; no leading-neutral recovery law.

## wave31-hebrew-corpus-adapters-dependency-coordinate

Old word head h maps0->0 else p(h-1)+1, then emitted-row head first[h-1]+1. Valid heads and shared word sequence are required; interpreting Node0-based ids as heads is invalid.

## wave31-hebrew-corpus-adapters-book-code

MACULA library and Tanakh order differ, so cross-reader identity is book code plus chapter/verse, not integer ordinal. Missing/invalid coordinates map0 and unknown code39, creating noninjective fibers.

## wave31-hebrew-corpus-adapters-speech-carry

Prag read_sentence(row,open,rules) produces next open; quote recognition precedes closure/carry/bare. Book boundary resets externally. Langseg has no caller relationship to this speech state.

## wave31-hebrew-corpus-adapters-effect-boundary

Model save truncates before rows; CLI talmud writes original/English before optional map read; converter writes accumulated books/maps separately. No atomic rollback or no-read claim for audit guard. These effect traces are source-modeled, not executed.

## wave32-hebrew-segmentation-lattice-piece-coordinate-join

text::units gives source slices; Cut advances unit counts while H inserts article and lead shifts selected vowels. CoNLL written tokens retain forms while words can include an unwritten H; comparing byte counts to Piece counts is invalid.

## wave32-hebrew-segmentation-lattice-context-lattice-join

Lex candidate p is updated previous then next via (n+K*p)/(total+K); lattice ratios use these contextual p values, clamp at1e-12 and serialize log ratio to four decimals. Choice consumes this rounded lr, not exact original p.

## wave32-hebrew-segmentation-lattice-parser-merge-join

parser_view clears UPOS and normalizes FORM; merge trusts equal sentence/word counts and copies2..7. Wrong equal-sized parser order may be accepted; failure on later sentence leaves prior updates.

## wave32-hebrew-segmentation-lattice-lattice-selection

variants changes one token at a time against base. Strict positive gain and strict incumbent comparison retain first equal winner; apply combines winners without validating joint parsed score. No global optimum follows.

## wave32-hebrew-segmentation-lattice-codec-effects

CoNLL write renumbers ID without remapping HEAD; TSV save omits fallback/lex/rest flags. annotate writes input before spawning and cleanup happens only on full success; main choose can emit study before choosing fails.

## wave32-roundtrip-layer-residuals-head-coordinate-transport

R3 maps zero-based realized origins to one-based parsed heads; R4 maps local frame links to global indices then kept-event coordinates and back.

## wave32-roundtrip-layer-residuals-decoder-state

Realizer seen ids and off flags must match encode/decode passes; same output alone does not recover state.

## wave32-roundtrip-layer-residuals-empty-sentences

R4 empties plus XOR starts and nsents restore the original partition, while pipeline Unit changes reset groups.

## wave32-roundtrip-layer-residuals-diagnostic-cost

Diagnostic bits and equal-set averages are not actual compressed byte lengths or semantic-quality percentages.

## wave32-roundtrip-document-codecs-markup-coordinate-join

site_md::inline emits marks at current emitted UTF-8 byte length; vol::with_marks restores them. concat changes Text indices by block offset but does not change each block's local mark offsets. Swapping these coordinate spaces is invalid.

## wave32-roundtrip-document-codecs-hebrew-patch-join

site_code predicts Hebrew rows using lexicon/text/verse/given then site_heb::diff/apply. apply(diff(pred,lit))=lit requires ten fields and unique normalized pair keys; duplicate FEATS and empty string versus '_' are explicit gaps, not universal exactness.

## wave32-roundtrip-document-codecs-copy-history-join

repeat trials clone Copier and accept when len>=24 and cost<=0.25 literal_bits; residual_with reconstructs same previous prose order using remember/code. Different prior prose or disabled-repeat environment changes classification and cost; all history must agree.

## wave32-roundtrip-document-codecs-scheduler-report-join

par_each executes all indexed jobs and catches closure panics. run_blocks folds Results in input order, turns errors into one-volume red tally; row exact=exact_ok==1. Aggregation loses scheduling/error provenance and cannot reconstruct jobs from totals.

## wave32-roundtrip-mixed-routing-coordinate-transport

Spans and owner overlap use bytes, sentence cuts use char_indices byte positions, diff preview counts Unicode scalars, LCS and tree Take use token indices, tree heads one-based.

## wave32-roundtrip-mixed-routing-tagged-routing

En/Heb/Text are alternatives; Text stores whole token residual. Language classifier loss and route accuracy use distinct labels.

## wave32-roundtrip-mixed-routing-metric-denominators

Report bits per sampled item differs from parsed sentence chrF and language token accuracy. Meta excluded in summary bits yet present in TSV.

## wave32-roundtrip-mixed-routing-shared-helper-joins

diff updates Diverge consumed by Tally; compare is independent file comparison with before-only set domain. No direct mixed-TSV adapter claimed.

## wave32-translation-meta-codecs-owner-signature

Reader owners map each meta token to optional Hebrew index; HebEncoder emits only first encounter per owner, losing later assignments.

## wave32-translation-meta-codecs-head-coordinate

MTok uses1-based heads/root0; each structural transform must renumber positions after dropping/reordering, while span retains source coordinates.

## wave32-translation-meta-codecs-logical-gate

Delta gate compares sorted logical_words of bare concept and chunk, preserving duplicates and normalizing selected numerals. Event comparison is separate: permutations preserve verb-multiset gaps, while LCS pairing (including repeated-verb ties) can change contradictions and distance.

## wave32-translation-meta-codecs-normal-versus-residual

Hebrew measure computes ordinary text exact/chrf before diagnostic target residual reconstruction; gate_ok can hold for empty learned encoder.

## wave32-translation-evaluation-pipeline-population-transport

All target VRec keys versus source-present Agg keys differ; missing whole blocks in many contribute absent/level accounting separately from cross population.

## wave32-translation-evaluation-pipeline-cost-denominators

Pair-local1000(reader+tail)/max(target_words,1), then arithmetic mean over pairs differs from pooled numerator/denominator; class sums and page filtered sums use distinct cell universes.

## wave32-translation-evaluation-pipeline-effect-order

Eval rejects train before output creation; both commands load model after output creation; report writes sequential artifacts, optional pages later. Source-derived effect order only, no rollback promise.

## wave32-translation-evaluation-pipeline-coordinate-join

VKey split depends on Kind; corpus joined heads add offsets preserving0; measure block sentinel contributes an additional offset. Unknown boundaries prevent unique reverse segmentation.

## wave33-math-library-ingestion-license-transport

raw CcBy4 -> cc-by-4.0 and book CcBySa -> cc-by-sa are accepted finite fetch codes; raw None -> none and book Other -> other reject. This is policy transport, not license authenticity validation.

## wave33-math-library-ingestion-token-transport

OAI textual token extraction unescapes XML then HTTP encode percent-escapes bytes. Literal token percent must be escaped again; passing already serialized query syntax as a token is not equivalent to adding query parameters.

## wave33-math-library-ingestion-checkpoint-join

OAI gzip sync precedes state append; fetch successful file rename precedes manifest rewrite; Duck execution has separate process/DB effects. Failure between stages can leave prefix artifacts, so individual success does not imply transaction.

## wave33-math-library-ingestion-route-partition

CLI command and arity select one of OAI/raw/books/fetch/GCS/SQL paths. Their costs and errors form a tagged union; GCS listing is not a prerequisite for raw parsing or books. Main helper checks cover partition, not route I/O.

## wave33-cycle-teacher-diagnostics-artifact-order

Pipeline collect commits db before corpus append/check/enrich/train/row. A row is a terminal observation under untampered assumptions, but row absence does not imply no database effects. main model rename joins trainer availability only after subprocess success.

## wave33-cycle-teacher-diagnostics-heldout-identity

exclude returns heldout set separately from heldout∪inflight ids. feed uses union; training_file uses heldout only. Matching uses sent_id, so equal text with different ids is not excluded by this relation.

## wave33-cycle-teacher-diagnostics-diagnostic-boundary

Lesson smoke preparses assessment answers and learns exercises twice, bench reads frozen workload outputs, twin predicts synthetic costs. Neither reading correctness nor cost acceptance is an independent saved-state learning proof.

## wave33-cycle-teacher-diagnostics-budget-resource

Enrichment line budget caps reservations under lock and single-caller premise, not successful calls or dollars. jobs sampled RSS/time limit and legacy budget_n share allocation represent distinct resources and cannot be substituted.

## wave33-treebank-comparison-tools-edit-coordinate-transport

Rule Edit.pos and Dispute.word are one-based; Vec heads/rels index pos-1. Prompt sentence number first resolves through group-local ids before word choices. Equality of lengths alone does not establish forms/ids alignment, so these joins require positional correspondence.

## wave33-treebank-comparison-tools-reference-identity-join

Matcher lifts ACL id to library (Src,index); collect coalesces by that pair and favors lower How/fuzzy tuple; Card resolves same index against unchanged Matcher. Mutating/reordering indexes between collect/render invalidates identity; synthetic join holds indexes fixed.

## wave33-treebank-comparison-tools-stats-render-projection

Stats gate compares only sentences/tokens/words/fused, while render prints empty too. Equality of stats therefore cannot certify empty-node displayed count. Total is componentwise sum over existing splits; absent stats splits default zero and missing total rejects.

## wave33-treebank-comparison-tools-reader-adapter-boundary

Treeagree requires ten columns/sequential ids and valid feature/relation parse; parsecmp accepts at least eight columns, skips ranges/empty nodes and defaults bad heads. A shared file is a safe adapter only on the intersection of these domains; a successful parsecmp read is insufficient evidence for treeagree validity.

## wave33-text-corpus-tools-text-coordinate-transport

School verse marks are UTF-8 clean-string byte offsets; OCR edit alignments use char indices; Kaggle uses passenger u32 ids; Wiktionary streams use compressed byte offsets. An adapter must carry the coordinate tag and cannot reuse equal numeric values across spaces.

## wave33-text-corpus-tools-lossy-artifact-join

Potential Gutenberg text -> school book cleanup strips header/footer, first line behavior, underscores and short cut fragments; reverse is a preimage including lost text. Raw Wiktionary Page.text requires edition/title context to enter count/emit helpers and does not imply six-column idiom TSV equivalence.

## wave33-text-corpus-tools-effect-prefix-order

Each effectful command admits successful prefixes before later failure: pool before verses/counts, earlier Gutenberg rows before write failure, earlier requested pages before missing/parse failure. Pure helper successes do not establish atomic publication or remote availability.

## wave33-text-corpus-tools-determinism-domain

BTreeMap rule conditions and explicit lexical tie keys stabilize selected paths; En reports sort ties by key, while Ukrainian morph/emptydef equal-count rows lack tie keys. Fixed synthetic helper outcomes cannot certify those unexecuted report orders or external subprocess schedules.

## wave33-bibliography-value-tools-manifest-transport

bib row status open/open-nc lies within OPEN; files null size -> parse_size(None); URL suffix .pdf derives id.pdf on safe canonical path. Actual-helper join fixture checks these interfaces.

## wave33-bibliography-value-tools-admission-vs-merge

merge(Value) accepts missing/nonarray files and repeated within-row URLs; load(Resource) schema/duplicate checks can reject. Therefore merge success cannot transport validity unconditionally.

## wave33-bibliography-value-tools-verification-gap

plan accepts present matching-size bytes or unknown-size bytes without hashing; check adds digest/magic constraints. Same-size non-PDF may disappear from queue yet fail verify.

## wave33-bibliography-value-tools-independent-value

Value's sh/bench/report owns independent environment and subprocess traces; no caller edge to bibliography exists. Costs and evidence remain separate instead of invented cross-product proof.

## tests

### Evidence and limits

- world exec focused order/cardinality and panic tests
- Fact-index source-derived score/rank contracts; scoped probes if present in child evidence
- Structural source-bound groupant inventory

The integrated inventory checks child identities and transitive source hashes.
It does not execute model text or certify semantic composition. No new end-to-end
process trace is implied by registering this group. Executed checks and exact
scope are recorded in the child sidecars and integration worklog.

The expanded slice adds source-included cache, number/value, type and span fixtures, plus extracted query/coverage function bodies with dependency stubs. Exact scopes and negative controls are in child evidence files. Seven new partial syntax projections and their fragment envelopes are source-bound. These are different evidence strengths; no full parser/reader course run is implied. Historical mixed-field observations refuted unconditional deterministic sorted judgments at baseline1e63aaf0a; current finite tests verify canonical traversal and prior-command eligibility.

This epoch passes the fresh seven-test world values subset and a separate 1,280-case full-judgment oracle. Three parent source mutants fail. Store fixtures pass 59 assertions twice, including a sampled-set carry disagreement with full reselection; the store implementation remains unchanged. Concurrent snapshot capture is source-derived, not executed. Cache-to-write_atomic composition remains scoped to the file helper.

The sampled-set repair supersedes the historical failure above: four native source tests pass, including 256 sampled query variants and legacy-prefix ordering; disabling the new guard fails three tests. The extracted store probe now passes 62 assertions twice. Separate finite oracles and the Lean carry/preimage law have explicit abstract scopes. Integration refreshes ancestor bindings; timing evidence is in the store current-evidence.md.

Semantic-delta evidence: eight native store tests pass, including four new identity cases; omission mutants reject losing either lemma or tag. The synthetic integration uses independent record packing and does not run update's parser/writer. A finite closure model and abstract Lean law are separate from source correspondence. The derivation sidecar records its 90-assertion helper probe and false closed-class control. No live training, full course or concurrent snapshot experiment is added.

## wave13-evidence

### Wave13 bounded evidence

75 focused native checks pass twice in debug and twice in release with identical sorted names/results: 58 pragmatic unit checks (23 new source tests, eight existing owner tests and 27 previous pragmatic regressions), seven new pragmatic joins and ten previous world joins. These are selected tests, not full suites or corpus exams.

Four owner production mutations fail named assertions (kNN distance shell, schema word boundary, feature question flag, inference history window). The root wrong-schedule adapter also compiles and fails predecessor_schedule_and_subset_have_explicit_start_fallback; exact bytes are restored before positive runs. Owner sidecars preserve original red witnesses and receipts.

Field and metric assertions use literal independent values. Dictionary repetition is transport equivalence, not independent semantic accuracy. The context Observer is a test classifier; separate tiny real fits exercise the actual classifier path. eval/data remain unregistered source dependencies, malformed classifier/TSV states remain domain restrictions, and no live parser/teacher/GPU/corpus/full-school run or speed/accuracy gain is claimed.

## wave14-evidence

### Wave14 bounded integration evidence

464 algorithm tests (416 unit, 47 external, one doc) and 101 selected Mova checks pass twice in debug and twice in release with identical normalized named outcomes. Mova checks comprise the previous 75 and 26 reached checks: eval 4, data 6, frames_md 7, tune 4 and five new joins. These selected Mova checks do not represent the complete Mova suite.

Ten new algorithm production mutations and eight false-oracle mutations fail named assertions in owner receipts. Four Mova production mutations fail named assertions. Both root adapters compile and fail their intended named assertions: zeta_product_inverts_to_or_convolution (wrong inverse stage) and batched_items_keep_source_identity_through_classification (wrong source window). Exact source bytes are restored before positive runs. Compilation failures do not count as semantic rejection.

Strict all-target algorithms Clippy and algorithms formatting pass. Historical world/prag warnings remain. No benchmark, throughput, GPU, teacher/parser acquisition, full-school run, held-out accuracy, persistent adoption or general proof is claimed. Tiny supplied-input fitting exercises actual classifier calls. Tuning validation participates in candidate choice and is not an untouched final test.

## wave15-evidence

### Wave15 bounded integration evidence

139 selected native Mova checks pass twice in debug and twice in release with identical normalized named outcomes: 101 prior focused checks, 31 new unit checks (frames 8, rules 10, vn 6, vnconv 7) and seven new pragmatic joins. This is a selected suite, not the full repository suite; the ninety algorithms are unchanged and are not remeasured.

Four owner production mutations fail named assertions and their exact source bytes are restored. The root adapter changes only frame 3's predicate sentence from 1 to 0; it compiles and fails sentence_token_pairs_select_head_agent_and_render_schema_act (Alice rather than Bob), then exact source bytes are restored before positives. Compilation failures are not semantic rejection.

Witnessed repairs exclude unresolved right-hand heads from entity identity; reject invalid direct rule sites; terminate lexical parent chains before invalid/repeated indices; and avoid Unicode byte-boundary panic in malformed WordNet pointers. Pointer parsing retains the existing tolerant zero fallback: source zero can match any source word and target zero can select the first target word. Malformed pointers can still produce nominal rows; this is not validated WordNet input.

Pure production-path seams expose supplied lexicon tables/matching, VerbNet XML conversion and rule selection. Global VN recording behavior is preserved under fixed process configuration; concurrent environment changes are outside equivalence. Supplied Match coordinates index their own Lex, not global Match accessors. Extraction and rule features may consult compiled lexical resources/configuration. No live parser/teacher/corpus acquisition, filesystem conversion, GPU, whole-school, persistent adoption, general proof or speed/accuracy gain is claimed.

## wave16-evidence

### Wave16 bounded integration evidence

The complete 100-algorithm crate passes 504 checks (448 unit, 55 external composition, one doctest) twice in debug and twice in release. The 155 selected Mova checks also pass twice in each profile, with identical normalized named outcomes: 139 previous focused checks, 11 new unit checks, two existing fbench/silver tests and three new integration joins. This is a selected Mova suite, not full-system validation.

Ten new algorithm implementation mutants and two Mova production mutants fail named assertions in owner receipts, with exact restoration before positive runs. Root additionally compiles two adapter controls: wrong forest weight/cardinality correction and swapped one-based silver-row identity. Both fail their named assertions, then exact source bytes are restored. Compilation failure is not counted as semantic rejection.

Mova fixes bound duplicate-link credit by gold occurrences (multiset intersection) and retain context unless adjacent paragraphs share source/document identity and checked consecutive paragraph numbers. Old-code witnesses cover duplicate overcredit, cross-source suppression and overflow. Three root joins exercise notation/alignment/counts, row/item/class transport and batch/prompt coordinates. Unique-link semantics and public APIs remain unchanged.

Independent read-only review found no additional concrete defect. Coverage limits: graph/tree cap+1 checks do not establish maximum-size successful throughput; Li Chao scans an independent line envelope but shares widened arithmetic and value/ID ordering; the connectivity cap fixture does not stress worst-case interval storage. A 64-vertex clique representation cap does not make exponential search cheap.

The ten new implementations are exact within their declared domains; no new approximate Rust surrogate or Lean proof is asserted. No live parser/teacher/corpus acquisition, GPU, whole-school throughput or persistent-learning adoption was executed. No general semantic proof or training-speed gain follows from these checks.

## wave17-evidence

### Wave17 bounded integration evidence

212 selected Mova checks pass twice in debug and twice in release, with identical normalized named outcomes: 155 prior focused checks, 33 new source-unit checks, 17 additional existing synthetic QA checks and seven new joins (three pragmatic, four question-generation). Root qagen library runs explicitly skip lemma::tests::doubt_with_en_model. The 100-algorithm library and its prior evidence are unchanged and are not remeasured in this epoch.

Seven existing sources gain models, with 21 nested fragments and 81 typed views; the two harnesses add 11 views, the new question-generation group adds six, and existing parents add five. Final inventory: 329/762 Rust sources, 433 pending (350 public,83 closed), 659 nodes,16 groups,314 fragments,3676 views,1109 type labels,319 partial syntax projections. Filename coverage in prag is complete; semantic coverage is scoped. Six qagen sources remain in the backlog.

Witnessed repairs: exact u128 Hamilton quotient/remainders remove the >2^53 weight-order error and usize sum overflow; checked prompt adjacency prevents i64 wrap/panic; QA anchors reject empty list components and paragraph labels permit at most one P/p prefix; accounting refuses nonfinite fields before opening and propagates non-NotFound I/O errors; run reports identify Codex transport. No broad option-compatibility change or transport rename was made.

Owner evidence records eight old-code failure witnesses and 14 compiled production mutants, followed by exact restoration and positive runs. Two root compiled adapter controls additionally fail named assertions: reversed Item/label identity and a wrong sentence-source mapping. Compiler errors are not counted as mutation rejection. Stale group bindings and wrong child identities are structurally rejected and restored exactly.

Independent source review found no further production counterexample within declared domains and refined costs: quota initialization/cap scanning remains at zero rounds; string comparisons/cloning and general JSON-object lookup work are explicit. The small quota oracle covers 45,056 tuples; this finite evidence is not a Lean proof or complete input verification. Native code and syntactic projections remain separate from models.

A gate worker also invoked a broader 26-test legacy run containing an optional local-model test; captured evidence does not establish whether that model branch ran or returned early. It is documented separately and excluded from the root 212-check receipt. Root checks use supplied fixtures and isolated temporary files, with no live teacher, parser model, corpus/database acquisition, GPU, whole-school or adoption workflow. No speed gain or new approximation-error budget is claimed.

## History

### 2026-10-29 · Epoch memory

2026-10-29: the earlier expansion exposed duplicate cache overwrite, attempt counters, mixed-time queries, span bypasses, display collisions and schedule-dependent judgment. This epoch fixes value command-history eligibility and ordering; other runtime branches are unchanged. Child syntax projections and bounded Rust probes locate assumptions. World construction, parser accuracy, reader selection, crash recovery and full-course behavior remain outside this evidence.

### 2026-10-30 · document

#### Bounded observation joins, 2026-10-30

The registered API union now includes Markdown/tabular projections, quantitative
mechanics and answer/QUD observations. These are operation-tagged branches,
not one end-to-end inference algorithm. Four new catalog views retain actual
lang/world/md snapshot coupling, opchoice-to-quant restricted arithmetic,
reader/QUD dependency identities and single-line CSV agreement. ftqa's CSV API
is an exercised but unregistered dependency; its missing sidecar remains a gap.

The complement universe extends to these tags with policy-specific exclusions:
a projected accepted/rejected count must agree with its supplied Run; an
admitted small-domain arithmetic candidate must evaluate to its exact rational
embedding; coverage must preserve occurrence counts; the restricted encoded CSV
fixtures must preserve fields before categorical unknown projection. These are
partial source/fixture exclusions, not assertions about every malformed input
or linguistic truth. Reverse relations retain operation tags, text/frame/world
identity, literal-prefix context, errors and source tags. Shared intermediate
states must agree; neither numerical equality nor matching rendered strings
identifies their original provenance.

Reader's repaired tree lookup uses checked subtraction, while downstream slot
output can retain an invalid supplied anchor. Index/ans event scoring still
requires its own valid-anchor domain. Literal-prefix admission constrains
numbers rather than reference provenance. QUD's sense join checks policy wiring
against current animacy, not independent facts. These distinctions prevent
local guards and child correctness from becoming false parent guarantees.

The integrator reported the initial six-test debug groupant_joins pass and a
compiled division-zero adapter control. Final test counts, repeated profiles
and normalized evidence belong to integration. No parser, live teacher, GPU
or full natural-language pipeline is credited to these bounded joins.

### 2026-10-30 · tests

#### Cache relation correspondence, 2026-10-30

Six native cache tests pass in debug/release, including four new finite correspondence tests: 120 batch cases, 2768 behavior-complement memberships, complete bounded duplicate-output fibers, 32 two-step histories and envelope/refresh boundaries. A compiled OR-to-AND production mutant is rejected. Missing-set identity is measured before insertion, refresh selects every position, and failed calls can change counters without changing the map. Direct-parser equivalence retains its text-extensional premise. Tests use local closures; no real parser or GPU performance claim is introduced. Global world Clippy has existing modulo_one/never_loop errors elsewhere; these runs are not a clean whole-world lint result.

#### Fact ranking and worker correspondence, 2026-10-30

Six index tests and six exec tests pass in debug/release. Index's independent half-unit oracle covers 432 score cases, ranking/filter/tie behavior, role/field multiplicity, section cases and joint label/score fibers. A +3-to-+4 predicate mutant fails. Four new worker tests cover finite fibers, panic prefixes, drop accounting and hidden callback state; an extracted compiled reversed-iteration mutant fails. Index extraction/dictionary quality and real CPU/GPU parsing remain untested. Positive fact anchors and pure callback premises remain explicit. Whole-world Clippy has known unrelated errors; no blanket clean-world lint claim is made.

#### Native query/span epoch, 2026-10-30

Four query tests and six span tests pass twice in both profiles, alongside six exec and six index tests. Query's 397 rows compare independent finite fibers; span covers 1364 cleanup cases and 64 predicate rank cases plus selected wiring/fallback assertions. Compiled half-threshold/echo-threshold mutants fail native assertions. These are directly constructed worlds and annotated Trees; no parser, real school corpus, GPU or live teacher is exercised. Parent projection models organize child evidence without asserting an end-to-end answer pipeline.

#### Native tree, command, world and corpus epoch, 2026-10-30

The final focused world run passes 68 tests twice in debug and twice in release, with identical normalized test identities. Of these, 43 new tests belong to tree, qframe, world, lang, events, register, poly and triggers; 25 retain selected state/query/span/exec/index regression coverage. The native command parser-to-world fixture is an executed cross-file join. Corpus file I/O and live trigger activation remain narrower, source-inspected obligations. Compiled leaf mutations and the old large-count implementation are rejected by targeted assertions. These checks neither run the full world suite nor establish GPU throughput, Lean equivalence or language quality.

#### Integrated native evidence, 2026-10-30

The integrated checkout passes 138 focused native tests twice in debug and twice in release, with identical normalized test identities/results. This includes 61 new leaf tests, six new actual-API joins and 71 selected existing regressions. The six joins all pass after the reversed-operand control was restored. Leaf source controls, the original defective branches and the late-invalid-index-filter mutation fail the named assertions in their respective evidence. All registered child identities and transitive hashes are reconciled in the final manifests. These checks do not execute a full reader/parser, teacher, school corpus or GPU benchmark, and do not constitute a whole-world lint result.

### 2026-10-30 · wave13-evidence

Heading `### Wave13 bounded evidence — 2026-10-30` stays in wave13-evidence as `### Wave13 bounded evidence`.

The harness source SHA-256 is 9991d8a61afc40be64d7828b5411d1cc55d8d49b750db50fe55f85f2c15d51d4. Two independently generated partial syntax artifacts are byte-identical, SHA-256 98747dbfda921f29f82ef34ac78048f70113407ec6c1b9218161df6c749ede2c. Partial syntax is not semantic AST.

External run bundle: $MOVA_DATA/runs/octopus-wave13-integration-2026-10-30. native-evidence-hashes.json SHA-256 5579f525ae5ec216d5ce51ebfc4f90ef1ad24910aca965984222c6c74c5a0337 binds raw native logs and normalized outputs; all-normalized.txt SHA-256 dc4afe9f976b96a8b612728d912982074166dab5f1fd3ea0c51912e6994e3328; adapter-mutant.log SHA-256 ede47849bfecb4f417b084e1397e25bdfd74ae1ba546d8c850618f9eb44f33f2. Structural controls summary SHA-256 a50d4387189f34044579c0d068475201acf1f6e7ee8e3297c0db6909f3180975 records stale-binding and wrong-child rejection with exact restoration.

### 2026-10-30 · wave14-evidence

Heading `### Wave14 bounded integration evidence, 2026-10-30` stays in wave14-evidence as `### Wave14 bounded integration evidence`.

Raw receipts: $MOVA_DATA/runs/octopus-wave14-integration-2026-10-30. native-evidence-hashes.json SHA-256 d95a91417adeb7f8e755d26f900b6508ea17759291407f33153917a42396280f; algorithm-adapter-mutant.log d2279a28cc474377b1d9321e15d4c779b0af8507f81ac39e83338bc9084a02a2; mova-adapter-mutant.log 60567fa4d984ac58834be268cdb221a37caf30bd5d07e9a3d6a2d89c16fdac87; mova-all-normalized.txt fb492ea8097fd74fe50cdceb5d61cb49efc74baa6cb152f7699e546dcbb3ac20; structural-controls/summary.json a50d4387189f34044579c0d068475201acf1f6e7ee8e3297c0db6909f3180975. The structural controls reject stale parent-source bindings and wrong child identities, restoring exact bytes. Source syntax remains partial, with opaque macros and no semantic AST.

### 2026-10-30 · wave15-evidence

Heading `### Wave15 bounded integration evidence, 2026-10-30` stays in wave15-evidence as `### Wave15 bounded integration evidence`.

Raw receipts: $MOVA_DATA/runs/octopus-wave15-integration-2026-10-30. native-evidence-hashes.json SHA-256 e62ef10e112b72d8cee51b62fc00d2b7182df368d9027a6dde8adc632d9608b3; all-normalized.txt ca00fc492f129d1b182df72826d3e547ba460d387fba9a9421805a4376f431dd; adapter-mutant.log 120c1c9e87c5351c8ff16aeefb4e400e3dbb00ae94b28ba0acf1b07125f53442; structural-controls/summary.json a50d4387189f34044579c0d068475201acf1f6e7ee8e3297c0db6909f3180975. Two structural controls reject stale parent-source bindings and wrong child identities, restoring exact bytes. Partial syntax has opaque macros and no semantic AST.

### 2026-10-30 · wave16-evidence

Heading `### Wave16 bounded integration evidence, 2026-10-30` stays in wave16-evidence as `### Wave16 bounded integration evidence`.

Raw receipts: $MOVA_DATA/runs/octopus-wave16-integration-2026-10-30. native-evidence-hashes.json SHA-256 f93920d688e961ce9dd02be865389cbe54db3093416464e379c650f16578841d; algorithm-normalized.txt 1335d34b718e04458dac54697f8822eb8f02b0f408dbb547852dfaa2d60a7164; mova-normalized.txt f0ca49c0b0f12e5febe6e41657b4d098e8918171918f8ae53bd96868ac5f9328; structural-controls/summary.json a50d4387189f34044579c0d068475201acf1f6e7ee8e3297c0db6909f3180975. Structural controls reject a stale parent source hash and a wrong child identity, then restore exact bytes. Owned partial syntax projections repeat identically; macro bodies remain opaque. Owner receipts remain adjacent to sources; formatting-only followups retain original owner native hashes, while root repeats bind final bytes.

### 2026-10-30 · wave17-evidence

Heading `### Wave17 bounded integration evidence, 2026-10-30` stays in wave17-evidence as `### Wave17 bounded integration evidence`.

Raw receipts: $MOVA_DATA/runs/octopus-wave17-integration-2026-10-30. native-evidence-hashes.json SHA-256 2aee840e096dcaf3d558e55c95b86f6a6623506aa620351c1e9d009ba29e94e5; mova-normalized.txt e7d315a6d937998d03e2cdc2555ba4fd96137ef61ef873104329ca632b697274; structural-controls/summary.json a50d4387189f34044579c0d068475201acf1f6e7ee8e3297c0db6909f3180975. Focused formatting passes. Cargo Clippy completes with warnings; this is not a warning-free strict-lint claim. Macro bodies remain opaque in partial syntax. External child dependencies are not inferred to have complete models from these joins.

### 2026-11-01 · document

#### Public Crab retained-annotation boundary — 2026-11-01

The crab-orchestration child composes generic control, disjoint annotation workers, version ingestion, frozen teacher/witness export and terminal review envelopes. Concrete deployment handlers and model activation remain separate. Ownership is a tree; call/artifact dependencies still cross file boundaries. Source/input hashes, exact revision/token joins, train/dev policy and a common lease inode are required wiring constraints. A committed SQLite version is not an activated model, and a successful GPU/dev observation is not semantic gold or a global generalization proof. See the child's explicit snapshot and DB/model phase models.
