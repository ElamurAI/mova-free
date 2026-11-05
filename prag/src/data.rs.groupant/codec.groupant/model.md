# prag-data-codec

## forward
Src serializes as lowercase tale/tatoeba/gum and maps to gutenberg-tales/tatoeba-eng-cc0/gum license keys. Item decoding defaults omitted ord to zero and ctx to empty; serialization omits empty ctx but emits ord. license_gate deduplicates source names before delegating.

## invariant
Unknown or wrong-case source names reject. Legacy omitted fields become explicit default state. Required Item fields remain required. License truth is delegated, not derived from source text.

## cost
Serialization and decoding scale with record bytes; source mapping O(1); license dedup O(K log K).

## behavior-complement
Serializing Tale as Tale, accepting unknown Src or inventing nonzero ord when omitted is excluded. Codec acceptance does not imply a valid chronology or genuine license provenance. Domain and observation are those of forward; this is a source-derived partial exclusion model, not a general proof.

## reverse-reconstruction
Decoded ord=0,ctx=[] admits both omitted and explicitly present JSON fields and many whitespace/key orders. Re-encoding normalizes these distinctions and cannot restore original bytes. The relation is all compatible antecedents, with no uniqueness claim.

## specialization
Source license mapping is a three-element exact table, independently tested with literals; legacy JSON demonstrates omission normalization.

## composition
Children share source identity and dictionary/sample or record indices. Parent order and domains constrain joins; leaf observations alone do not establish external orchestration. Overlapping scopes must not double-count costs.

## checks
Seven wave14_models tests: four evaluation and three data; supplied inputs only. Debug twice and release twice; compiled production mutation per source must fail a named assertion. Syntax generated twice identically. Receipts summarize final evidence.

## epoch-memory

First coverage of these previously unmodeled files. Prospective review predicted no valid-domain metric change and O(N) extra validation. Old source accepted mismatched vectors; the rejection witness failed (six other tests passed). Narrow assertions preserve tuple API and reject invalid domains. Data production behavior remains unchanged. Tiny two-row fitting exercises three learners without any training job.

## limitations
Partial syntax projection, opaque macros, no semantic AST or universal proof. No live parser, DB subprocess, external corpus, teacher, GPU or training job. External IO, report formatting, selection corner cases, malformed Set/Train and empty tale windows remain outside executed scope. No performance claim.

## History

### 2026-10-30 · epoch-memory

2026-10-30: first coverage of these previously unmodeled files. Prospective review predicted no valid-domain metric change and O(N) extra validation. Old source accepted mismatched vectors; the rejection witness failed (six other tests passed). Narrow assertions preserve tuple API and reject invalid domains. Data production behavior remains unchanged. Tiny two-row fitting exercises three learners without any training job.
