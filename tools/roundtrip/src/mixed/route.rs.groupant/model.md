# rt-mixed-route

## descriptant

### Descriptant

#### Observed epoch

Bounded real helper checks in model.md#tests constrain these models; syntax is a partial lexical projection with no macro expansion, name resolution or proof. Production expressions unchanged; six inline test modules added. Compiled mutants are temporary and exactly restored. Existing malformed-input and nonfinite domains remain explicit, not silently repaired. Parent composes this scope with siblings without adding overlapping costs twice.

Finite synthetic helper evidence only; live corpora, parser/model loading, full orchestration and filesystem failure races excluded.

## approximant

### Forward relation

route_of maps En/Translit to En, Hebrew varieties to Heb, others Text. split merges neighboring equal routes except Hebrew/Text quote boundaries, expands exterior bounds and assigns last whitespace run as joint. join defaults missing residual value to one space.

Domain: Sorted nonoverlapping UTF-8 byte intervals within line; joints length covers every text index >=1. Arbitrary invalid coordinates can panic; joint requires 1<=k<pieces.len().

## invariant

### Invariant and cost

#### Invariant

Sorted nonoverlapping UTF-8 byte intervals within line; joints length covers every text index >=1. Arbitrary invalid coordinates can panic; joint requires 1<=k<pieces.len().

#### Cost

Token filtering O(S*T); split O(S+gap bytes) excluding repeated concatenation allocation; join O(total text+joint bytes).

#### Limits

Finite synthetic helper evidence only; live corpora, parser/model loading, full orchestration and filesystem failure races excluded. No percentage surrogate is implemented: categorical parsing/effect observations lack an admitted numerical error metric. Any future profile must define its metric/domain and use only1/2/5/10 percent caps.

## composition

### Local composition

route_of maps En/Translit to En, Hebrew varieties to Heb, others Text. split merges neighboring equal routes except Hebrew/Text quote boundaries, expands exterior bounds and assigns last whitespace run as joint. join defaults missing residual value to one space.

Children select narrow source functions with their exact byte intervals. Remaining branches stay in the file model; fragments are not exhaustive partitions. Calls and state joins follow the forward relation, not folder order. Shared scans/helpers are counted once in whole-file costs. Complement of composition is not union of complements; reverse composition retains compatible intermediates and caller premises.

## models

# Typed source-bound relations

Subject: exact source SHA-256 in manifest. Domain and resource premises are in model.md#invariant. Each model is partial, source-derived, over a single helper invocation or the described finite trace. Complements are proposed inner exclusions under those premises; reverse relations retain all compatible antecedents, with no unique inverse claim. The oracle is the named checks in model.md#tests; unexecuted cases remain hypotheses. No numerical approximation certificate.

## behavior_complement

Excluded: valid split loses exterior punctuation; noncanonical joint is recovered from None. Empty segment list with nonempty line returns Text piece.

## reverse_reconstruction

Texts plus exact joints reconstruct line uniquely; split route labels do not recover original language labels because many languages share route.

## byte_boundary

Segment coordinates are bytes, char_indices locates whitespace scalars but returns byte offsets.

## quote_partition

English merges across quote marks; Hebrew/Text preserve quote-separated pieces.

## shuffle_multiset

Shuffle permutes language labels across volume segments preserving coordinates and label multiset.

## empty_route

Empty line+segments gives zero pieces; nonempty no segments produces one Text piece including whitespace.

## tests

### Checks and evidence

Named source/helper checks: mixed::route::tests::pieces_cover_the_line.

Finite synthetic helper evidence only; live corpora, parser/model loading, full orchestration and filesystem failure races excluded.

## History

### 2026-10-31 · descriptant

#### Prospective epoch

2026-10-31 source review predicted: route_of maps En/Translit to En, Hebrew varieties to Heb, others Text. split merges neighboring equal routes except Hebrew/Text quote boundaries, expands exterior bounds and assigns last whitespace run as joint. join defaults missing residual value to one space. Resource model: Token filtering O(S*T); split O(S+gap bytes) excluding repeated concatenation allocation; join O(total text+joint bytes). External prospective.md predates test additions and preserves APIs, effect order, hazards and interaction hypotheses.

### 2026-10-31 · tests

Wave32 on2026-10-31:10 distinct exact tests passed twice debug and twice release, with equal sorted named outcomes. This node claims only its listed subset; empty subset means source review only. Five actual source-behavior mutants compiled, failed assertions, restored exact bytes/hash, and passed focused green reruns. No assertion-only mutants; existing swapped-input controls are separate. Receipt bundle: $MOVA_DATA/runs/octopus-wave32-rt_mixed-2026-10-31 (native-commands.json, source-test-map.json, mutation-results.json, *.status/*.log). No live corpus, teacher, frozen assessment, download or production data used.
