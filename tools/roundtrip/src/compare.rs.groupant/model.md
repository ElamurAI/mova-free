# rt-mixed-compare

## descriptant

### Descriptant

#### Observed epoch

Bounded real helper checks in model.md#tests constrain these models; syntax is a partial lexical projection with no macro expansion, name resolution or proof. Production expressions unchanged; six inline test modules added. Compiled mutants are temporary and exactly restored. Existing malformed-input and nonfinite domains remain explicit, not silently repaired. Parent composes this scope with siblings without adding overlapping costs twice.

Finite synthetic helper evidence only; live corpora, parser/model loading, full orchestration and filesystem failure races excluded.

## approximant

### Forward relation

Read residual(a), residual(b), normal(a), normal(b) in that failure order. Skip the header and empty lines; ignore residual rows shorter than seven columns and normal rows shorter than eight. Numeric parse failure becomes zero, while accepted nonfinite and signed values remain accepted. Residual duplicate full keys are last-wins; the first normal row fixes totals and later rows add class counts.

Enumerate sets from the before residual map and then before normal map, preserving each ordered-map traversal and omitting already seen sets. For each such set s, let K_s be the set union of (layer, class) keys from both residual maps. Define p(tokens)=0, p(tree)=1, p(frames)=2, p(world)=3 and p(other)=9. Iterate K_s in lexicographic order (p(layer), class, layer). This preserves known-layer priority and class order, and orders unknown layers deterministically by class then layer.

For each key, a missing entry contributes zero; an entry (bits, sentences) contributes bits/sentences when sentences>0 and zero otherwise. Emit one row when abs(before-after)>=0.05, with one-decimal rounding. Accumulate non-meta classes once per full key; emit totals only for tokens, tree, frames and world in that order. Unknown-layer totals are not emitted.

Normal starts come from before and require a matching after key. Normal class names form a sorted set union; changed normalized class counts use the same threshold. Ratio formulas, denominator clamps and normal formatting remain as implemented.

Domain: owned temporary TSVs and successful reads; permissive rows impose no closed layer vocabulary. Finite nonnegative metrics give the meaningful-ratio subdomain; the implementation does not validate finiteness. This is a partial source-derived relation, not a proof over every floating input.

## invariant

### Invariant and cost

#### Invariant

For each admitted before-domain set, residual iteration contains each full (layer, class) key from either snapshot exactly once. The tuple (p(layer), class, layer), with known priorities 0 through 3 and unknown priority 9, is injective over full keys: equality of sort tuples implies equality of both class and layer. Therefore sorting makes every duplicate adjacent, and dedup yields the set union.

Known-layer output keeps priority then class ordering. Unknown-layer output follows class then layer order. Distinct input row permutations that preserve each map's effective values produce identical output at fixed paths; this does not cover duplicate full residual keys with different last-wins values or normal rows with different first totals.

Read order, before-domain sets/normal starts, zero fallbacks, nonfinite acceptance, abs-change threshold, rounding and known-layer total policy are unchanged. The numeric child still covers only num. Owned temporary TSVs have at least seven/eight columns for admitted residual/normal rows.

#### Cost

Read O(file bytes); ordered-map updates O(rows log keys); set-vector membership can be quadratic. With K concatenated residual keys, sorting uses O(K log K) comparisons, dedup O(K), and retained keys O(K), ignoring variable string lengths. Sort keys now also clone the layer string; no measured speedup is claimed. Output cost is proportional to rendered rows and bytes.

#### Limits

Finite synthetic helper evidence only; live corpora, parser/model loading, full orchestration and filesystem failure races are excluded. No percentage surrogate is implemented: categorical parsing/effect observations lack an admitted numerical error metric. Any future profile must define its metric/domain and use only 1/2/5/10 percent caps.

## composition

### Local composition

The numeric child models num: parse f64 or return zero. Residual and normal readers compose that helper with their distinct row-width and duplicate policies. Read residual(a), residual(b), normal(a), normal(b) in that failure order; feed the four maps into before-domain selection, residual full-key union/order, ratio/threshold formatting and matching normal starts.

Within each set, sort concatenated residual keys by (known-layer priority or 9, class, layer), then deduplicate equal full keys. The sort key contains both identity fields, so local adjacent dedup implements the union required by the parent relation. This coupling is necessary: a comparator omitting layer cannot justify full-key dedup for unknown layers.

The numeric child retains its exact byte interval. Remaining branches stay in the file model; fragments are not exhaustive partitions. Calls and state joins follow the forward relation, not folder order. Shared scans/helpers are counted once in whole-file costs. Complement of composition is not union of complements; reverse composition retains compatible intermediates and caller premises.

## models

# Typed source-bound relations

Subject: exact source SHA-256 in manifest. Domain and resource premises are in model.md#invariant. Each relation describes one compare invocation or its stated projection. Complements below are partial source-derived inner exclusions; they are not claims about untested filesystem races or panics. Reverse relations retain compatible antecedents and claim no unique inverse. model.md#tests identifies executed evidence; no numerical approximation certificate is claimed.

## behavior_complement

For successful reads in the admitted TSV domain, exclude an output that reports an after-only set, reports a normal start absent from after, or repeats a changed residual full key within one set. Exclude changed residual rows out of (known-layer priority, class, layer) order. The latter exclusions follow the injective sort tuple and full-key dedup; the former follow before-domain enumeration. Numeric parse errors do not necessarily error: they map to zero. This is an inner exclusion over output structure, not a complete complement of all rendered strings.

## reverse_reconstruction

A rendered output admits every TSV pair whose effective maps and omitted values satisfy the threshold, ratio and formatting constraints. Rounded values have many antecedents; overwritten duplicate rows and ignored short rows cannot be recovered. One emitted residual row represents one processed full key, but does not reveal how many original rows supplied that key. Permutations preserving all effective map values remain compatible: this includes last-wins residual values, first-row normal totals and accumulated normal class counts. Because floating addition is ordered, preserving the first totals alone does not preserve those class counts. Same totals need not mean the same examples or text.

## failure_precedence

An absent before residual file preempts every later read; filesystem context preserves the failing path.

## lossy_numbers

Invalid numeric text maps to zero; NaN is accepted and its comparisons fail thresholds. There is no universal finite-value guarantee.

## before_domain

Set enumeration and normal starts come from before only; residual keys form a full-key set union within those sets. An after-only residual key can appear when its set exists before.

## threshold_render

The 0.05 threshold precedes one-decimal rounding; formatted equality does not imply raw equality.

## residual_key_order

Hold one admitted set and both effective residual maps fixed. Project behavior to its processed key sequence: sort the full-key union by (p(layer), class, layer), with p(tokens/tree/frames/world)=0/1/2/3 and p(other)=9. This partial source-derived structural model excludes repeated keys and preserves known-layer ordering. Unknown layer ties are broken by their full layer strings after class comparison. The regression checks two unknown layers sharing a class and all four known layers, plus all four before/after insertion-order combinations. Unobserved strings remain supported by the source argument, not an exhaustive runtime proof.

The historical comparator omitted layer. Its collision fiber for (9,c) contains every unknown (layer,c) key. With identical before/after key sets containing alpha and beta, its stable sequence was alpha,beta,alpha,beta; adjacent dedup retained all four. This is the actual old-code witness in both red logs, not a behavior of the corrected comparator.

## residual_row_multiplicity

For one admitted set s and fixed effective maps, let U_s be their full-key union, and D(k) be the implemented abs(before-per(k)-after-per(k))>=0.05 predicate. Observe only changed residual class rows, excluding the separately emitted bold totals and normal rows. Their count is sum over k in U_s of indicator(D(k)), bounded by |U_s|; each individual full key has multiplicity zero or one. This partial source-derived cardinality projection applies even when NaN makes D false; meaningful finite ratios retain the narrower domain from model.md#invariant. It does not bound original TSV row counts because duplicate residual rows are overwritten by the reader.

The finite witness uses six union keys with before 2/2 and after 6/2, hence six changed class rows and one alpha/beta occurrence each. The regression's exact sequence asserts the cardinality and multiplicity oracle. The old comparator yields eight rows and falsifies this relation; rounding and thresholds remain unchanged by the repair.

## tests

### Checks and evidence

Named source/helper checks:

- compare::wave32_tests::lossy_rows_and_before_domain
- compare::wave37_tests::unknown_residual_layers_are_unique_and_ordered

After adding layer as the final sort component, both named compare tests passed twice debug and twice release (two tests each run). The regression checks the explicit known/unknown row sequence, one occurrence of each unknown row and byte-identical whole reports for all four combinations of before/after row reversal at fixed paths. The external minimal Rust harness imports the actual compare.rs and depends only on the same anyhow version; it is not a rewritten compare implementation or a full roundtrip crate test. Cargo ran offline, locked, with two jobs and a private target directory.

Finite synthetic helper evidence only; live corpora, parser/model loading, full orchestration and filesystem failure races remain excluded. No teacher, GPU, download, frozen assessment, production data or full course was used.

## History

### 2026-10-31 · descriptant

#### Prospective epoch

2026-10-31 source review predicted: Read residual(a), residual(b), normal(a), normal(b) in that failure order; parse failure becomes0 while NaN parses. Residual duplicate key last-wins; normal first row fixes totals and later rows add class counts. Enumerate sets from before maps, show abs change>=0.05 and layer totals excluding Meta. Resource model: Read O(file bytes), ordered maps O(rows log keys), set vector contains can be quadratic; sorting residual union O(K log K); output O(rendered rows). External prospective.md predates test additions and preserves APIs, effect order, hazards and interaction hypotheses.

#### Wave37 prospective correction — 2026-10-31

The independent systems audit identified a conflict between the declared residual-key union and stable sorting by only (priority, class). Two unknown layers sharing a class remain interleaved across snapshots, so adjacent full-key dedup misses their repeats. Independent source review agreed; external prospective.md preserved the prediction before the production edit. The expected repair adds layer as the final sort component, preserving priority/class order and asymptotic cost while making the comparator injective over full keys.

#### Wave37 measured correction — 2026-10-31

The actual-source regression failed in both debug and release before the fix: alpha and beta each rendered twice. With (priority, class, layer), both compare tests passed twice per profile. The new fixture checks all four known-layer positions and deterministic output under four input-order combinations. This refutes the earlier union model for the old source's unknown-layer domain and supports the repaired relation on the bounded fixture; it is not whole-system verification. Historical Wave32 observations above remain historical and did not cover unknown-layer collisions.

The source model now states unknown-layer ordering explicitly. The residual_key_order typed relation records comparator/dedup coupling and the old collision fiber; residual_row_multiplicity projects the filtered union to one-or-zero rows per key. Numerical parsing and the numeric child are unchanged; its full-source fingerprint is refreshed with the parent. Evidence and scope are in model.md#tests. No performance improvement or percent-error bound is claimed.

### 2026-10-31 · tests

Wave32 on 2026-10-31: ten distinct exact tests passed twice debug and twice release, with equal sorted named outcomes. This node claims only its listed Wave32 subset. Five actual source-behavior mutants compiled, failed assertions, restored exact bytes/hash, and passed focused green reruns. Historical receipt bundle: $MOVA_DATA/runs/octopus-wave32-rt_mixed-2026-10-31.

Wave37 on 2026-10-31: the new regression failed against the original sort tuple in both debug and release (exit 101), observing two copies of the alpha changed row where one was required; the printed output also contains two beta rows. Values were before bits/sentences=2/2 and after=6/2. All four known layers occurred once in established order.

Receipts: $MOVA_DATA/runs/octopus-wave37-roundtrip-2026-10-31 (prospective.md, harness/Cargo.toml, harness/Cargo.lock, red-{debug,release}.{log,status}, green-{debug,release}-{1,2}.{log,status}, commands.md). Source bindings and syntax are checked separately; structural checks do not prove semantics.

### 2026-11-01 · descriptant

#### Paired diagnostic memory — 2026-11-01

The forward vidtvorant predicted a full-key union; the residual-row multiplicity exclusion required at most one changed row per full key. The collision fiber of the old sort tuple explained how unknown alpha/beta layers could violate that prediction without violating priority/class order. The native red fixture decided the discrepancy; adding layer to the comparator repaired the source. Preserve both the full-key identity and comparator assumptions when composing this node with report formatting. This explanation reuses the wave37 receipts above and claims no newly executed tests or timing gain. See the [paired-review account](../../../../docs/mathematical-modeling.md).

### 2026-11-05 · tests

2026-11-05: `lossy_rows_and_before_domain` no longer requires WAVE32_TEMP. When it
is unset the test uses its own directory `roundtrip-wave32-<pid>` under the system
temp dir (removed at the end); an explicit WAVE32_TEMP still wins. Compare code is
unchanged; the Wave32/Wave37 receipts above remain bound to the previous source hash.
