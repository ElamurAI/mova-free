# en-lexgen

## descriptant

### en-lexgen
AGID/UD TSV generator retaining old lemma positions

## approximant

### Forward relation
UD lowercased form triples aggregate first occurrence; skip missing tag/underscore. AGID first clean else first dirty, doubtful header no reverse; 3 verb fields copy past-part. Old heap kept; append UD frequency desc/lexical then AGID lexical. Lemmas flush before forms create/flush.

No executable Rust surrogate attached: no demonstrated metric for preserving these categorical, stateful or effect observations. Supported future caps are 1/2/5/10%, not a claim of current error.

## invariant

### Domain, invariant and cost
UD lowercased form triples aggregate first occurrence; skip missing tag/underscore. AGID first clean else first dirty, doubtful header no reverse; 3 verb fields copy past-part. Old heap kept; append UD frequency desc/lexical then AGID lexical. Lemmas flush before forms create/flush.
No atomicity; absent AGID fails before writes; duplicate old heap rows retained, last ID wins; no TSV escaping or u32 index cap.
O(bytes+U log U+A log A+output); retained rows/heap linear; two sequential buffered files.
Inputs must satisfy the indexing/generated-data assumptions stated by the typed catalog; allocation failure and overflow beyond bounded fixtures remain untested.

## composition

### Mathematical combiner
Source bytes and twice-generated syntax constrain this model. Forward relation: UD lowercased form triples aggregate first occurrence; skip missing tag/underscore. AGID first clean else first dirty, doubtful header no reverse; 3 verb fields copy past-part. Old heap kept; append UD frequency desc/lexical then AGID lexical. Lemmas flush before forms create/flush.
Reverse relation: Outputs admit input changes preserving counts/first occurrence; old heap prefix retained but flags recomputed; failures admit partially written files.
The ordered typed family records independent exclusions, compatible antecedents and source-specific boundaries. Fragment ranges compose through actual control flow; they do not cover all helpers.
Epoch memory: empty learned support, lossy projections and non-atomic effects cannot inherit stronger downstream guarantees.

## models

## behavior_complement
No atomicity; absent AGID fails before writes; duplicate old heap rows retained, last ID wins; no TSV escaping or u32 index cap.

## reverse_reconstruction
Outputs admit input changes preserving counts/first occurrence; old heap prefix retained but flags recomputed; failures admit partially written files.

## effect_order
Missing AGID read errors precede output directory/write; lemmas file truncates and flushes before forms is created, so later failure leaves partial update.

## append_identity
Existing heap order retained, duplicate strings map last index; newly observed lemmas append, never reorder old rows.

## variant_precedence
First clean comma variant wins; if no clean use first nonempty dirty; questionable POS clears clean for all forms.

## frequency_ties
UD lemmas sort descending frequency then lexical, lowercase companion appended immediately; AGID remainder sorted/deduped lexically.

## personal_token_exclusion
Domain: one token string. Excluded from the generated dictionary: addresses (e-mail, link, handle, telephone number) by `personal_token`; kept: words that merely contain `@` (`@`, `s@p`, `e@tg`, `assh@%$e`), bare domain names (`enron.com`, an organisation name), year ranges and decimals. Assumption: EWT's personal data appears as such address-shaped tokens; a personal name used as a word is not detected. Counterexample: a non-NANP number such as `+44 20 7946 0958` passes. Evidence: unit tests on positive and negative lists; not a proof.

## redaction_reverse
`redact` is not invertible: from `<redacted-N>` only the position N is recoverable, not the address. It is idempotent (a second run changes nothing) and preserves every non-personal lemma number and every kept form row byte for byte, in order.

## source_citation
`SOURCES` lists every input `en lexicon` reads (agid → letters a, b, r; ewt and eslspok → u) with the name its citation carries in data/LICENSE.md. `uncited(licence, forms)` returns the sources the licence text does not name and the source letters of forms.tsv no source yields; empty on the shipped files. Complement: it checks names, not citation correctness; a new treebank passed to `en lexicon` without a SOURCES row is not refused at run time, only by review. ESLSpok annotates no lemmas, so it yields no row although it is read and cited.

## tests

### Actual source checks
Named check: lexgen::wave27_tests::synthetic_files_preserve_ids_and_source_flags. Command: cargo test -p en --lib wave27_tests -- --test-threads=1 (also --release). Four runs: each 10 passed, 0 failed, numeric exit0; sorted named outcomes equal. This file contributes one named check per run.
Control present for this source: true. Hash zeroed byte contribution, text changed empty chrF to1, lexgen removed three-cell PastPart copy: each compiled and failed one assertion (exit101), then exact source restoration and ten-test positive rerun (exit0). Other sources have no compiled mutation claim.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: read complete source before prospective model and bounded tests. Production code retained byte-for-byte; only inline synthetic checks added. Coverage is partial, not a proof.

### 2026-10-31 · tests

See evidence.json and external octopus-wave27-lexical-2026-10-31 receipts. Static built dictionary read is part of the linked library; no corpus or external services used. Unknown support, malformed model/blob, resource failures, cycles and exhaustive Unicode behavior are unexecuted gaps.

### 2026-11-04 · descriptant

Epoch 2026-11-04: personal tokens. `personal_token` classifies e-mail addresses (also obfuscated `name...@host` and Lotus Notes `Name@DOMAIN`), links (`http://`, `https://`, `ftp://`, `mailto:`, `www.`), `@handles` and North American telephone numbers; `generate` skips UD tokens whose form or lemma is personal, and `redact` (CLI `en lexicon-redact`) rewrites an existing dictionary: a personal lemma line becomes `<redacted-N>` with flag `-`, and form rows with a personal form or a redacted lemma are dropped. Lemma numbers never move; the dictionary fingerprint changes, so models trained before need retraining.

### 2026-11-04 · tests

2026-11-04: lexgen::personal_tests::personal_tokens_are_addresses_not_words and redact_keeps_numbers_and_is_idempotent (cargo test -p en --lib personal_tests).

2026-11-04: lexgen::personal_tests::shipped_dictionary_holds_no_personal_tokens gates the shipped data/lemmas.tsv and data/forms.tsv; it was red (523 tokens) before `en lexicon-redact data` and is green after (415 lemmas redacted, 321 form rows dropped; a second run changes nothing).

### 2026-11-05 · tests

Named check (2026-11-05): `lexgen::personal_tests::licence_cites_every_source` — green on the shipped files; red controls: AGID renamed in the licence text → ["agid"]; a row with source letter q → ["letter q"].

### 2026-11-05 · public export

The personal_token test fixtures are invented (example.com/.org/.net addresses, 555 telephone numbers) instead of real-looking EWT/Enron addresses; the same shapes are asserted, so the tested relation is unchanged (cargo test -p en personal: 4 passed, 2026-11-05).
