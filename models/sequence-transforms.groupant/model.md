# sequence-transforms

## model

# sequence-transforms

## forward

Compute exact single-word global byte Levenshtein distance, train/apply an ordered byte-pair model, encode/decode bounded LZ77/LZW streams, and build/apply deterministic patience patches. Lossless reconstruction, token identity, edit count and distance are separately tagged.

## invariant

Decoding canonical encodings recovers original bytes on admitted bounds; malformed streams and output-budget failures remain errors. BPE tokens carry their exact ordered model; LZW freezes dictionary at4096 and has no packing/clear/end codes. Patience patches consume the source exactly and reconstruct the target without a minimality promise.

## cost

Myers O(pattern+text+256) time/constant storage. BPE training O(merges*n*log n), encoding O(merges*n), decoding O(model+tokens+output). LZ77 greedy encoding O(n*window*max_match), decoding O(tokens+output). LZW uses phrase-vector ordered maps with O(n*phrase_length*log4096) encoding and bounded dictionary phrase storage; decoding includes phrase copying. Patience conservative O(n³) time/O(n²) stack/storage, apply O(source+edits+output). No compression ratio or speed claim.

## composition

The five leaves are std-only; BPE/LZW use BTreeMap but no intermodule production calls. Codec composition takes decoded bytes or an explicit reversible u16 serialization, never an unchecked cast of token IDs to bytes. Patience apply reconstructs a target whose byte distance can be independently checked by Myers only when its chosen pattern fits64.

## epoch-memory

Wave25. Models retain real-arithmetic assumptions separately from native finite checks. Kernels are library tools, not automatically adopted into training or inference.

## checks

Named joins: bpe_codec_model_bound_join, patience_myers_edit_observations_join. See ../algorithm-library.groupant/model.md#wave25-evidence and wave25-validation.md there for integrated receipts. These separately tagged tests do not imply a serial runtime pipeline.

## limitations

All inputs are bytes, not Unicode scalar sequences or lines. Allocation may panic/abort except Myers allocation-free path. Decode admits valid noncanonical streams, so decode/encode is not a universal stream identity. Output caps and model/config identity must be propagated through every join. No callbacks or executable percentage profiles.

## catalog

# Typed family relations

Each relation keeps its own domain and assurance.

## behavior-complement

Type: behavior_complement.

Exclude a Myers value different from global unit-cost Levenshtein distance; canonical codec round trips changing bytes; invalid token/code/match accepted; decoded output exceeding cap; or a patch that does not fully consume matching source bytes. Exclude model-free BPE decoding, interpreting u16 IDs as bytes by truncation, and claiming patience edit count equals Levenshtein distance.

## reverse-reconstruction

Type: reverse_reconstruction.

Distance0 admits every equal byte pair and does not recover inputs. BPE literals[97,98] and merged[256] both decode ab under a model whose first merge is(97,98). LZ77 aaaa admits four literals or literal(a),match(1,3); LZW AAA admits[65,65,65] or[65,256]. A full patch's Equal/Delete projection recovers source and Equal/Insert projection recovers target, but target alone admits many sources/scripts.

## model-identity

Type: token_dictionary_transport.

BPE decode(encode_M(x),M)=x on the model/input/output domain. Ordered merges define token meanings; using a different learned model may produce different bytes without an authentication error. When composing with a byte codec, first expand tokens or serialize u16 losslessly with explicit format and length limits.

## canonicalization

Type: noninjective_stream_relation.

For valid LZ77/LZW/BPE streams, encode(decode(stream)) may choose a different canonical stream. Only decoded-byte equality is preserved absent canonicality assumptions. LZ77 configuration and BPE model additionally influence canonical encoding; stream length alone carries no compression-quality guarantee.

## lz77-overlap

Type: sequential_copy_relation.

LZ77 distance d references already emitted bytes one at a time, permitting length>d and self-repetition. Copying a fixed pre-match slice instead of evolving output changes semantics. Encoder ties prefer smallest distance; decoder accepts valid positive lengths below3 although encoder emits only matches>=3.

## lzw-dictionary

Type: synchronized_dictionary_relation.

Encoder and decoder begin with byte codes0..255, append previous phrase plus first byte of current phrase, and freeze at4096. Decoder's code==next_index case is previous+first(previous) only when prior phrase exists and dictionary not full. Adding clear codes or bit-packing is outside this API and cannot be inferred from a12-bit dictionary.

## patch-distance-rejection

Type: validity_without_optimality.

Patience uses prefix/suffix stripping, unique-byte anchors and deterministic LIS ties, with delete/insert fallback. Applying diff(old,new) must recover new. Its insertion/deletion count is a valid edit-script upper bound for unit-cost Levenshtein, not generally the minimum; substitutions cost1 in Myers but require delete+insert in this patch vocabulary.

## resource-domain-join

Type: bounded_composition.

A successful producer does not guarantee consumer admission: Myers requires one side<=64, patience diff<=4096, while codecs allow65536. BPE u16 serialization can double length and exceed byte-codec cap. Validate each transformed length and preserve first malformed-stream versus output-limit precedence; resource rejection is not an approximate answer.

## byte-coordinate-law

Type: alphabet_semantics.

Myers bit positions represent pattern bytes; repeated equal bytes set multiple positions and wrapping addition is the algorithm's bit recurrence. Unicode text must first choose an explicit byte encoding. Byte-level patches/tokenization do not preserve character boundaries or line-level patience semantics without an external adapter.

## wave25-evidence

# sequence-transforms: wave25 evidence

## checks (wave25-evidence)

Named joins: bpe_codec_model_bound_join, patience_myers_edit_observations_join. See ../algorithm-library.groupant/model.md#wave25-evidence and wave25-validation.md there for integrated receipts. These separately tagged tests do not imply a serial runtime pipeline.

## History

### 2026-10-30 · epoch-memory

Wave25, 2026-10-30. Models retain real-arithmetic assumptions separately from native finite checks. Kernels are library tools, not automatically adopted into training or inference.

### 2026-10-30 · checks

2026-10-30: this family adds 14 source-local tests and 2 independent composition observations. The full 175-module library passed 782 checks twice in debug and twice in release with identical normalized outcomes. All five production mutants in this family failed named assertions; exact restoration and positive checks followed. Root inspected the failure receipts. Tests are finite observations, not universal semantic or floating-point proofs.

### 2026-10-30 · checks (wave25-evidence)

2026-10-30: this family adds 14 source-local tests and 2 independent composition observations. The full 175-module library passed 782 checks twice in debug and twice in release with identical normalized outcomes. All five production mutants in this family failed named assertions; exact restoration and positive checks followed. Root inspected the failure receipts. Tests are finite observations, not universal semantic or floating-point proofs.
