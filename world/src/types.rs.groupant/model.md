# world-types

## descriptant

# world-types

Pure identity/value operations and their source-bound partial syntax.

## Forward

Dispatch by operation and preserve Rust input/output variants. Parsing uses a restricted decimal grammar; names and displays project values; V::tags computes sorted support. Individual graphs are in model.md#catalog.

## Invariant

No operation changes a world store or validates entity existence. Canonical tag invariants hold for V::tags outputs, not arbitrary public V constructors. Constructor domains exceed parser domains. Strings are byte-exact and case-sensitive.

## Cost

Membership is linear in table size times string comparison cost. Normalization sorts n tags in O(n log n) comparisons then deduplicates. tag_list costs O(n) cloning for Tags. Formatting costs emitted bytes and temporary vector/string allocations. Id accepts at most seven ASCII bytes; is_word at most 32. No timing measured.

## Composition

Identity, vocabulary, routing and values share Reg, Field, Tag and V. key selects Field/Dom then key_ok constrains registry; external payload parsing is a dependency gap. Closed::tag supplies tags, V::tags normalizes, Display projects. Sequential R(x,z)=exists y: R1(x,y) and R2(y,z); reverse retains every compatible y. Parent complement is its own U minus R, not union of child complements. Fragment costs count actual calls once despite shared type dependencies. External language parsing, storage, branch execution and semantic identity remain unmodeled.

## Epoch memory

## catalog

# Typed models

## nonbehavior

U is the disjoint union over public pure operations of (operation, finite typed input, typed normal-return outcome). R is the union of graphs below. Target U minus R forbids accepting C0, returning duplicate normalized tags, or key_ok(O,Gender)=true. Source-derived exact target, not a proven complement; allocation aborts and formatter sink failure excluded.

Approximation/evidence: exact mathematical target under the stated domain, source-inspected correspondence; bounded executed coverage only as recorded in model.md#tests. Unsampled inputs remain unproved.

## antecedents

For operation o and result y recover all x with R(o,x,y). None from Id::parse has all rejected strings; None from V::tags has only empty input. Position short display hides coordinates, names hide variants. Preserve operation identity and every compatible antecedent.

Approximation/evidence: exact mathematical target under the stated domain, source-inspected correspondence; bounded executed coverage only as recorded in model.md#tests. Unsampled inputs remain unproved.

## id-language

Id::parse accepts exactly [LOCREPBFK][1-9][0-9]{0,5}. Strings are UTF-8 with no trimming, sign, lowercase registry, zero or leading zero. n is decimal value.

Approximation/evidence: exact mathematical target under the stated domain, source-inspected correspondence; bounded executed coverage only as recorded in model.md#tests. Unsampled inputs remain unproved.

## id-roundtrip

parse(display(id))=Some(id) exactly when 1<=n<=999999. Id::new and public fields admit every u32 including 0 and larger numbers. Display is injective on all Id values; parse does not validate construction.

Approximation/evidence: exact mathematical target under the stated domain, source-inspected correspondence; bounded executed coverage only as recorded in model.md#tests. Unsampled inputs remain unproved.

## sref-display

SRef displays id@ver for all u32 versions; equality compares both fields. This file supplies no SRef parser and does not establish entity/version existence.

Approximation/evidence: exact mathematical target under the stated domain, source-inspected correspondence; bounded executed coverage only as recorded in model.md#tests. Unsampled inputs remain unproved.

## position-fiber

TextPos short display is ^s followed by sent; each fiber has 65536 squared chapter/para pairs. long displays all three coordinates injectively; default is zero despite the one-based sentence comment.

Approximation/evidence: exact mathematical target under the stated domain, source-inspected correspondence; bounded executed coverage only as recorded in model.md#tests. Unsampled inputs remain unproved.

## closed-membership

For fixed static Closed c, tag(s) succeeds exactly for an equal c.all entry; comparison is case-sensitive. Error contains c.what and debug s. Tag equality uses string contents, losing Closed provenance.

Approximation/evidence: exact mathematical target under the stated domain, source-inspected correspondence; bounded executed coverage only as recorded in model.md#tests. Unsampled inputs remain unproved.

## closed-list

Closed::list joins entries by one space, preserving order and duplicates, not vertical bars. Public custom Closed permits spaces and empty entries, so general list recovery is ambiguous.

Approximation/evidence: exact mathematical target under the stated domain, source-inspected correspondence; bounded executed coverage only as recorded in model.md#tests. Unsampled inputs remain unproved.

## field-names

Field::name(status) fiber is {Status,Stage}; other names have singleton fibers. SetF::Children is places. key(status) chooses Status/Words, never Stage.

Approximation/evidence: exact mathematical target under the stated domain, source-inspected correspondence; bounded executed coverage only as recorded in model.md#tests. Unsampled inputs remain unproved.

## key-alias

17 spellings are accepted: name gender age mother father trait feeling sleep freedom life goal belief status num at to in. Last three share At/Place; unknown strings return None. This is not an inverse of Field::name.

Approximation/evidence: exact mathematical target under the stated domain, source-inspected correspondence; bounded executed coverage only as recorded in model.md#tests. Unsampled inputs remain unproved.

## key-permission

C accepts every Field; O accepts Name,Trait,Status,At,Num; L accepts Name,Trait,Status,At; all other registries reject. Combined admission is key(s)=Some(f,d) AND key_ok(reg,f); C permissiveness does not supply absent spellings.

Approximation/evidence: exact mathematical target under the stated domain, source-inspected correspondence; bounded executed coverage only as recorded in model.md#tests. Unsampled inputs remain unproved.

## tag-normalization

V::tags sorts and deduplicates; empty -> None, singleton support -> Tag, larger support -> sorted Tags. For output support S, inverse is every finite sequence with support exactly S. Direct public V::Tags bypasses canonicality.

Approximation/evidence: exact mathematical target under the stated domain, source-inspected correspondence; bounded executed coverage only as recorded in model.md#tests. Unsampled inputs remain unproved.

## value-display

Words joins commas, Tags joins tag strings, Text wraps guillemets without escaping. Arbitrary comma-bearing Words lose segmentation; Words([happy]) collides with Tag(happy). Unknown Val and Known(Words([?])) both display ?. tag_list clones Tags, makes singleton Tag, otherwise empty.

Approximation/evidence: exact mathematical target under the stated domain, source-inspected correspondence; bounded executed coverage only as recorded in model.md#tests. Unsampled inputs remain unproved.

## word-boundary

is_word accepts 1..32 ASCII bytes with initial lowercase letter and remaining lowercase/digit/_/-. TEXT_MAX_WORDS=25 is merely declared here; no text word-count enforcement occurs.

Approximation/evidence: exact mathematical target under the stated domain, source-inspected correspondence; bounded executed coverage only as recorded in model.md#tests. Unsampled inputs remain unproved.

## tests

### Checks and limits

Executed CPU observations: 270 registry/field pairs, with 39 admitted; 63 Id constructor/display/parse boundaries; 13 rejected spellings; all 121 tag sequences of length zero through four over three literal tags; 968 output-support relation/complement memberships. Every positive normalization result is checked for canonical idempotence. Auxiliary literal checks cover custom Closed membership/list/provenance loss, bypassed V::Tags canonicality, Words segmentation and variant collisions, Unknown display collision, key aliases, field alias, position loss/default and lexical boundaries. Two runs produce identical summaries.

Negative control: running the same probe with `--false-roundtrip` asserts the deliberately false universal Id display/parse identity and fails at Id(C,0), exit 101. This rejects a false formula; it is not a mutation test of every model.

The normalization fixture domain is finite (length <=4 and three tags); inverse membership retains every sequence within this bounded universe, not all finite vectors. Permission expectations use a separately written literal allowlist, although correspondence evidence is still finite testing rather than proof. Registry ordering, SRef encoding, complete vocabulary tables, all domain-routing cases, all errors, allocation failure and formatting sink failure are inspected or untested. No performance claim, full crate suite, external payload parser, storage existence or whole-inference verification is made.

The source-bound partial syntax projection and four fragment envelopes are structural evidence only. Run `mova-modelcheck groupants . models/contracts.json NEW_JSON`; an unregistered sidecar is allowed, and the parent integrates registry-node.json under inference-core. Regenerate syntax twice and compare bytes to check projection determinism. Models retain exact targets while marking their correspondence as partial/source-derived.

## evidence

### Recorded evidence

The standalone source-included CPU probe compiles with rustc and reports identically twice:

    permission=270 accepted=39 id_boundary=63 rejected_spellings=13 normalization=121 relation_complement_pairs=968

The deliberately false universal parse/display identity is rejected at C0, exit 101. No world rlib is used: the probe compiles the actual unchanged types.rs as its module. This isolates public pure mechanics; full world-crate compilation and dependency integration are not claimed.

The groupant report passes twice with byte-identical output: valid=true, diagnostics=[], 19 file sidecars and six group sidecars. Inventory counts include preexisting material. The new sidecar contains one file root and four fragments, 34 models total (14 + 4*5). The unchanged registry permits this unregistered sidecar. The changed-source report has changed=[] and missing=[] because only sidecar artifacts are added. Syntax regeneration is byte-identical; fragment ranges select projected top-level item envelopes, including any intervening comments/gaps. These checks validate structure and source bindings, not mathematics.

## History

### 2026-10-29 · Epoch memory

2026-10-29: retained constructor/parser mismatch, Status/Stage alias, key aliases, Closed list's actual space separator and public noncanonical V::Tags. These are model observations; source unchanged. Partial syntax does not expand macros, infer types or prove semantics. Prospective falsification: extend probes to arbitrary custom Closed tables and formatter errors after widening the observation universe.

### 2026-10-29 · document

Subject/revision: world/src/types.rs at manifest SHA-256, epoch 2026-10-29. Each relation observes normal returns on finite typed inputs. Assumptions: fixed source, ordinary allocation, successful formatting sink; no downstream validation. Status: source-derived partial correspondence, not semantic proof. Every formula defines its operator and explicit domain below. For every entry the falsification oracle is a literal input/output comparison with the actual source-included Rust API. Only coverage named in model.md#tests was executed; remaining relations are inspected hypotheses.

### 2026-10-29 · tests

Epoch 2026-10-29. `probe.rs.txt` is ordinary Rust with a source-path module binding to the unchanged adjacent types.rs. Compile with `rustc --edition=2021 --crate-name types_probe world/src/types.rs.groupant/probe.rs.txt -o "$MOVA_DATA/types-probe"` and run that executable. No world rlib, teacher, GPU, external data or active world state is required.

### 2026-10-29 · evidence

2026-10-29; baseline 4279da541; types.rs SHA-256 ac2d47f390463744e9b5d2e7b0a005352b62368e4e5d68bb1348a173a5c90037. Production Rust and shared registry unchanged.
