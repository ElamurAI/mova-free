# hebrew-segmentation-lattice

## descriptant

### hebrew-segmentation-lattice: wave32

Real joins: text::norm/units -> seg key and Piece semantics; conllu Token/Word -> gold and segmenter evaluation; lib::cut_with passed as callback by main lattice_cmd/choose_cmd; lib::segment_line calls split_in and cut_as; main score uses conllu heads and text normalization. Tagged independent paths: main split/prepare/Part partitioning, main score diagnostics, model TSV codec and annotate filesystem/subprocess adapter. Independent per-token lattice choices do not imply jointly optimal tree; positional merge count equality is not parser identity verification.

This source-bound partial group organizes existing Mova code. Native evidence and excluded effects are recorded in source sidecars and the root wave32 report. Registration is not a universal proof or whole-system runtime claim.

## model

# hebrew-segmentation-lattice

## forward

Raw text -> text::tokenize -> Model::keys/candidates_in -> cut_as -> CoNLL Token/Word. Optional lattice::variants uses contextual candidate labels; supplied scored readings join by sentence integer; choose takes g=score_alt-score_base+lambda*log_share+mu*(words_alt-words_base)>0 maxima per token; apply cuts the chosen labels. parser_view normalizes only word forms and resets UPOS; merge copies columns2..7 by flat position. Main score is an independent diagnostic consumer of matching token-position forms.

## invariant

Piece offsets count text::units, not UTF-8 bytes or scalar counts. Valid nonzero cuts sum to written units; H consumes zero, leading cuts relocate vowels. Finite nonnegative smoothing with positive denominators and representable integer counts supports score interpretation. Merge preserves base forms/token structure/MISC but may partially mutate before late count failure.

## cost

Let B bytes,U units,E model entries,P prefix alternatives,C prefix×suffix candidates,R parsed readings. For K=min(max_k,U), candidate context construction copies O(K²+sum_p K_p²) units: suffix contexts are rebuilt per prefix lacking accepted rest suffixes, K_p=min(max_k,remaining units). This is at most O((P+1)K²), weighted by unit byte lengths, plus O(PB) rest concatenation; lookup comparisons O(log E), sorting O(C log C), cuts/normalization remain additional. Lattice emits V=1+sum(C_i-1) readings per sentence and calls cut on every nonpunctuation token of every reading: V*N_np callbacks, each lib::cut_with callback recomputing candidates. Add initial contextual candidates and output/comment copying. choose costs O(T_R+B_R+R log S+R log N), where T_R sums reading token counts and B_R is scanned/parsed comment bytes; gains has the same scan terms and O(R log S) maps, with two n_words scans per retained alternative. CoNLL, projection and merge copy input/output text; external parser time/resources unknown.

## composition

Real joins: text::norm/units -> seg key and Piece semantics; conllu Token/Word -> gold and segmenter evaluation; lib::cut_with passed as callback by main lattice_cmd/choose_cmd; lib::segment_line calls split_in and cut_as; main score uses conllu heads and text normalization. Tagged independent paths: main split/prepare/Part partitioning, main score diagnostics, model TSV codec and annotate filesystem/subprocess adapter. Independent per-token lattice choices do not imply jointly optimal tree; positional merge count equality is not parser identity verification.

## domain

Bounded synthetic UTF-8/CoNLL strings, manually supplied tiny model TSVs/states and parsed score comments. Valid indices; nonempty callback outputs; k>=1 for nonpunctuation; every alternative has a base. Finite gain arithmetic and u32/usize/i64 representability. Explicit panic/error witnesses explore selected invalid boundaries without production repair.

## limitations

No external models, gparse annotate, real corpora, teacher, training, lesson workflows or GPU executed. Existing learn-based tests were inspected but not selected. Model::load handles synthetic in-memory TSV only. Filesystem/parser effects modeled by inspection. Source production prefixes remain byte-identical to base; added tests only.

## checks

- 14 exact named tests twice debug and twice release, matching sorted actual outcomes
- 6 compiled production-behavior mutants rejected with assertion failures and exact source restoration plus positive reruns
- 6 source syntax projections generated twice and byte-compared

## catalog

## behavior-complement

On admitted valid cuts consonant-checked results cannot alter unpointed written letters after removing H; finite nonpositive gains cannot leave lattice base; merge cannot copy FORM/MISC from parser. Empty-form fallback Cut(0) is explicitly outside the spelling guarantee.

## reverse-reconstruction

Final trees admit compatible raw texts, normalized forms, count tables, contextual score states and parser annotations under successive preimages. Whitespace/marks, old annotations, overwritten IDs and omitted TSV fallback are not recoverable uniquely; shared token and unit coordinates constrain composition fibers.

## piece-coordinate-join

text::units gives source slices; Cut advances unit counts while H inserts article and lead shifts selected vowels. CoNLL written tokens retain forms while words can include an unwritten H; comparing byte counts to Piece counts is invalid.

## context-lattice-join

Lex candidate p is updated previous then next via (n+K*p)/(total+K); lattice ratios use these contextual p values, clamp at1e-12 and serialize log ratio to four decimals. Choice consumes this rounded lr, not exact original p.

## parser-merge-join

parser_view clears UPOS and normalizes FORM; merge trusts equal sentence/word counts and copies2..7. Wrong equal-sized parser order may be accepted; failure on later sentence leaves prior updates.

## lattice-selection

variants changes one token at a time against base. Strict positive gain and strict incumbent comparison retain first equal winner; apply combines winners without validating joint parsed score. No global optimum follows.

## codec-effects

CoNLL write renumbers ID without remapping HEAD; TSV save omits fallback/lex/rest flags. annotate writes input before spawning and cleanup happens only on full success; main choose can emit study before choosing fails.

## History

### Undated · checks

- All original production prefixes checked against base cf00be5540a78c47038bcc8b72b6fcd8c22b355f
