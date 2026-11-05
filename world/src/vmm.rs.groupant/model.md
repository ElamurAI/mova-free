# world-vmm

## descriptant

### world-vmm

Prompts concatenate versioned rules/examples and numbered story slices; continuation includes preceding text/world brief. parts chooses k=ceil(n/max), nearest later paragraph boundary to ideal cut; splice replaces each valid 1-based @N line in order and counts writes. call rejects done before I/O/teacher; v2 rejects >2 parts then allows one repair total; v3 prefers fixed then raw, estimates before missing raw call and repair, writes fixed without revalidation.

## approximant

Prompts concatenate versioned rules/examples and numbered story slices; continuation includes preceding text/world brief. parts chooses k=ceil(n/max), nearest later paragraph boundary to ideal cut; splice replaces each valid 1-based @N line in order and counts writes. call rejects done before I/O/teacher; v2 rejects >2 parts then allows one repair total; v3 prefers fixed then raw, estimates before missing raw call and repair, writes fixed without revalidation.

Exact categorical/effect semantics: no useful executable percent surrogate supplied. Future admissible caps 1/2/5/10% require defined metric/domain; no error bound claimed.

## invariant

Done files refuse calls. Splice does not validate commands or restrict to rejected lines; duplicate last wins. Empty text yields (1,0); nonempty/max=0 panics, lengths beyond u16 truncate. Paragraph cuts do not guarantee max-length parts; if no later boundary remains the fallback ideal can move backwards, yielding reversed/overlapping ranges. NaN budget does not enforce cap; dollar estimate is legacy heuristic, not current subscription price.

Cost: Prompt O(rule+example+story+brief bytes); partition O(n+k*boundaries); splice O(output+fix bytes); remote calls dominate unknown latency. At most 2 calls v1, 3 v2, up to 2 per part v3 absent resume; retries after failures can repeat effects.

## composition

1-based inclusive u16 ranges; nearest section cut may exceed max. Splice permits any in-range line and counts duplicates.

lib exposes modules; main activates state and dispatches. ideas uses nerves::norm and talk::now, store::write_atomic; talk uses babi splitting/prag Act. VMM uses lang::parse_all and world executor summaries before teacher continuation. These adapters preserve different coordinates (line number versus sentence anchor); splice acceptance does not establish execution validity. Pure parts/splice/prompt/estimate and done guard tempfile checks. No teacher invocation, DB/license query, live repair or finite-budget assurance.

## models

## behavior_complement

An existing done file excludes teacher invocation in call/call_v2/call_v3. A valid @N splice does not exclude syntactically or semantically invalid replacement commands. v2 rejects more than two parts before creating the run directory; v3 fixed-cache presence excludes a new raw/repair request for that part. Budget comparisons do not exclude NaN bypass or certify current subscription dollar cost.

## reverse_reconstruction

Spliced output admits arbitrary overwritten originals and duplicate replacement histories; count is not number of changed rows. Cached fixed output admits differing original raw/repair histories. Prompt strings retain selected sentence text but do not reconstruct entire world history.

## partition_and_repair

1-based inclusive u16 ranges; nearest section cut may exceed max. Splice permits any in-range line and counts duplicates.

## failure_precedence

Done files refuse calls. Splice does not validate commands or restrict to rejected lines; duplicate last wins. Empty text yields (1,0); nonempty/max=0 panics, lengths beyond u16 truncate. Paragraph cuts do not guarantee max-length parts; if no later boundary remains the fallback ideal can move backwards, yielding reversed/overlapping ranges. NaN budget does not enforce cap; dollar estimate is legacy heuristic, not current subscription price.

## resource_domain

Prompt O(rule+example+story+brief bytes); partition O(n+k*boundaries); splice O(output+fix bytes); remote calls dominate unknown latency. At most 2 calls v1, 3 v2, up to 2 per part v3 absent resume; retries after failures can repeat effects.

## effect_boundary

Pure parts/splice/prompt/estimate and done guard tempfile checks. No teacher invocation, DB/license query, live repair or finite-budget assurance.


## tests

### Bounded evidence

No teacher or DB; done guard terminates before teacher configuration

Names: ["vmm::wave28_dialogue::splice_coordinates_duplicates_and_invalid_rows","vmm::wave28_dialogue::parts_and_prompt_anchors","vmm::wave28_dialogue::done_guard_precedes_teacher_and_partition"]

Command: CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR=ISOLATED_TARGET MOVA_HOT_ROOT=FRESH_TEMP_DIRECTORY cargo test -p world --lib --bin world [--release] wave28_dialogue -- --test-threads=1

The fresh temporary directory basename must start with wave28-dialogue-. Never point this fixture at a real hot root. The binary test calls private helpers; it never invokes main or state activation.

Nine selected cases passed twice debug and twice release with identical sorted named outcomes. Only the named source behaviors are exercised.

Control: mutant-vmm: splice content. All four source mutants compiled and failed with exit 101; each was restored byte-for-byte and nine positive cases passed again. lib and main have no runtime mutant claim.

Six parent syntax projections were independently generated twice and compared. Child fragments reference parent syntax with exact byte ranges and hashes. Strict whole-world Clippy remains subject to existing warnings in untouched code. No live teacher, corpus, DB, model, GPU or school execution. No semantic proof or executable percent surrogate follows.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: source-derived partial model; prospective hypotheses retained in external receipt. Pure parts/splice/prompt/estimate and done guard tempfile checks. No teacher invocation, DB/license query, live repair or finite-budget assurance.

### 2026-10-31 · tests

Heading `### Bounded evidence — 2026-10-31` stays in tests as `### Bounded evidence`.
