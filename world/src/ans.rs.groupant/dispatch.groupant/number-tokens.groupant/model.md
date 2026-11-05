# ASCII-space number rendering

## forward

Split on literal ASCII space. Each token parsed as usize in 0..20 maps to fixed word; otherwise preserve token. Join by one ASCII space, retaining empty split fields and thus original repeated spaces.

## invariant

A token 21 or punctuation-suffixed 7, cannot map to seven; tabs are not separators and no negative spelling parses as usize.

## behavior-complement

A token 21 or punctuation-suffixed 7, cannot map to seven; tabs are not separators and no negative spelling parses as usize.

## reverse-reconstruction

Each rendered numeric word admits its own spelling and every accepted usize spelling of its number, including leading zeros and plus sign. Preserve separator count; punctuation tokens recover themselves within this local mapping.

## parent-interaction

ans applies num_words to selected text only, then stores State; none returns empty text independently while retaining fallback anchor.

## boundary

Zero is included despite the old 1–20 comment. Conversion is idempotent over completed strings because number words do not parse as usize.

## information

Platform usize overflow leaves a token unchanged; portability of enormous digit strings depends on target width.

## cost

O(input bytes + output bytes) scan/parse/join and temporary token-string vector; output can expand bounded small-number tokens.

## checks

New spacing/sign fibers plus existing answer-projection fixture; no full intent dispatch.

Exact selected native boundary: `cargo test --offline --locked -p world --lib ans::wave34_models::number_rendering_spacing_and_fibers -- --exact --test-threads=1`. Repeated in debug and release; see the current file-level wave34 evidence and external native-receipts.json.

## assurance

The behavior-complement is an inner exclusion hypothesis over admissible input/output pairs, derived from branch order; not the complement of an incomplete model of the entire file. Reverse reconstruction is the compatible antecedent relation, retaining every modeled ambiguity rather than a chosen inverse. Parent-interaction is a composition relation, boundary a constraint relation, information an information/state projection, and cost a resource relation. The named checks are finite falsifiers; unexecuted branches remain source-inspected gaps. No parser/model/teacher/corpus/frozen assessment was run.

## epoch-memory

New child in wave34; no prior child renamed or counted. Parent historical wave8/wave9 evidence remains historical. Current raw receipts are in the external wave34 reasoning run; source syntax is partial, macros opaque, and no semantic AST or general proof is supplied.

## History

### 2026-10-31 · document

Wave34, 2026-10-31. Subject: world-ans-dispatch-number-tokens, current source SHA d8d94c891e347d2b640744b20c4662dc35064368e7d4d9253c4954b4297e1e09. Domain: one terminating supplied-input helper transition; valid indices, sufficient memory and finite strings/trees/worlds. Source-derived partial relations, not a verified universal theorem. All views below share this domain and exact manifest scope; specialized assumptions are stated locally. No executable percentage approximant is claimed.
