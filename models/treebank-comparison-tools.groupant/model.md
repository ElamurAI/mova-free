# treebank-comparison-tools

## descriptant

### treebank-comparison-tools: wave33

Actual joins are proposal Edit positions into trial head/relation/retag vectors; baseline Bad equality into acceptance; Readme Item text/bib into Matcher with raw byte boundaries; Hit (Src,index,How,fuzzy) into collect Found and Card rendering; Counts into stats tuple and render totals. Synthetic readme_collect_render_join checks deduplication and rendering method/title. Parsecmp uses its own permissive reader and cannot substitute for treeagree read_ref: it skips range/dotted rows and uses sentinel heads where treeagree rejects nonsequential ten-column input.

This source-bound partial group organizes existing Mova code. Native evidence and excluded effects are recorded in source sidecars and the root wave33 report. Registration is not a universal proof or whole-system runtime claim.

## model

# treebank-comparison-tools

## forward

Tagged paths: treeagree read_ref→aligned View→propose→apply/no_worse→serialized tree; dispute path read_ref/read_wit→disputes→prompt→parse_answers→apply with group-local ids. Treebanks release::read→count/check_stats and Readme::parse gate feed Matcher::item→collect→Card render. Parsecmp independently maps aligned trees to Score and bracket multisets; it is not invoked by treeagree or treebanks.

## invariant

Tree agreement commits only shape-valid candidates whose full violation identities belong to original baseline, under valid equal lengths and indices. Reference collections preserve (Src,index) identity when rendering, deduplicate explicit reference keys and retain selected match method. Streaming release token count is words+fused-range_widths; rendered totals sum split Counts. Bracket intersection is multiplicity-aware and no larger than either count.

## cost

Treeagree per sentence conservatively O(e*(n²+e*n+registry)) for apply; next_conj can make proposal O(n³) on acyclic trees. Treebanks streams archive bytes but retains metadata, index, audit and card strings; candidate matching cost depends on trigram collisions and scans, not just bytes. Parsecmp pairwise reporting O(systems²*tokens), bracket multiset intersection O((a+b)log distinct_a). No measured performance gain claimed.

## composition

Actual joins are proposal Edit positions into trial head/relation/retag vectors; baseline Bad equality into acceptance; Readme Item text/bib into Matcher with raw byte boundaries; Hit (Src,index,How,fuzzy) into collect Found and Card rendering; Counts into stats tuple and render totals. Synthetic readme_collect_render_join checks deduplication and rendering method/title. Parsecmp uses its own permissive reader and cannot substitute for treeagree read_ref: it skips range/dotted rows and uses sentinel heads where treeagree rejects nonsequential ten-column input.

## domain

Small synthetic plain-word UD trees, aligned witness vectors with valid heads; positive grouping chunk size; finite in-memory README/reference/index/count/render fixtures; bounded temporary parsecmp file. All arithmetic nonoverflow, allocations available; graph proposals assume acyclic conjunction paths. No real release build, corpus, parser/model loading, teacher or network.

## limitations

Treeagree Registry::load is built-in validation, not annotation model loading. CLI effect ordering is source-derived, not a release run. Syntax is partial lexical projection, no semantic expansion. Unicode case expansion before URL offsets, malformed coordinate arrays, arbitrary cyclic input and arithmetic extremes remain gaps; neither counts nor parser agreement establishes linguistic truth.

## checks

- 25 exact selected tests, twice debug and twice release with sorted named outcomes
- treebanks wave33_tests::readme_collect_render_join actual Readme/Matcher/collect/Card join
- treeagree tests::swap_needs_both_edits and tests::retag_rides_with_the_relation actual proposal/gate joins
- parsecmp wave33_tests::score_and_bracket_multiset_boundaries and conllu_fixture_filters_and_missing_file
- Ten paired current-source syntax projections and ten exact byte-scoped children
- Compiled actual behavior mutations with byte-exact restoration and positive reruns; see fresh external receipts

## catalog

## behavior-complement

On admitted aligned shape-valid tree state, failed no_worse trial excludes candidate commit; treebank stats/meta rejection excludes all card writes. Parsecmp diagnostics exclude parser invocation and never repair trees. This inner exclusion is path-tagged; successful helper calls do not imply atomic release output.

## reverse-reconstruction

Output tree plus kept edits constrains compatible draft/witness/lex histories but loses rejected candidates and old MISC. Rendered reference list constrains selected indexed papers but not duplicate paragraphs or raw normalization fibers. Aggregate scores/bracket counts admit many trees; no unique cross-path inverse.

## edit-coordinate-transport

Rule Edit.pos and Dispute.word are one-based; Vec heads/rels index pos-1. Prompt sentence number first resolves through group-local ids before word choices. Equality of lengths alone does not establish forms/ids alignment, so these joins require positional correspondence.

## reference-identity-join

Matcher lifts ACL id to library (Src,index); collect coalesces by that pair and favors lower How/fuzzy tuple; Card resolves same index against unchanged Matcher. Mutating/reordering indexes between collect/render invalidates identity; synthetic join holds indexes fixed.

## stats-render-projection

Stats gate compares only sentences/tokens/words/fused, while render prints empty too. Equality of stats therefore cannot certify empty-node displayed count. Total is componentwise sum over existing splits; absent stats splits default zero and missing total rejects.

## reader-adapter-boundary

Treeagree requires ten columns/sequential ids and valid feature/relation parse; parsecmp accepts at least eight columns, skips ranges/empty nodes and defaults bad heads. A shared file is a safe adapter only on the intersection of these domains; a successful parsecmp read is insufficient evidence for treeagree validity.
