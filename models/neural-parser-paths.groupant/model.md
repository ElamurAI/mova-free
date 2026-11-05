# neural-parser-paths

## descriptant

### neural-parser-paths: wave30

data::batch roles index (k*T+d)*T+p; Net attention uses B,H,T,dh and masks head/key columns. lib transposes arcs dependent-major into Eisner head-major and maps local decoded h to flattened k*T+h for Net::rels. An adapter using raw h for k>0 gathers another sentence's encoder state. Vocs labels require same ordering as model output dimensions; load files independently and missing weights are not consistency proof. CLI score zips and can hide length mismatches, so output-length invariants must be checked before claiming dataset coverage. Existing world GPU caller uses parse_forms; tokenization remains caller-owned. Tests are sibling observers, not runtime production stages.

This source-bound partial group organizes existing Mova code. Native evidence and excluded effects are recorded in source sidecars and the root wave30 report. Registration is not a universal proof or whole-system runtime claim.

## model

# neural-parser-paths

## forward

read -> Vocs/features -> padded Batch -> Net arcs[k,d,h] -> log-softmax -> transposed Eisner sc[h][d] rounded f32 recurrence -> local heads -> flattened k*T+h relation gather -> scattered tuples. Real max-plus optimality requires order-preserving arithmetic; finite f32 inputs alone do not imply it. Empty slots=[]/0; length>=max_len preserves original tuples/score0. Training loss=CEarc+CErel+0.5*(CExpos+CEupos), backward then AdamW; annotation adds lexical/postverbal edits.

## invariant

Admitted finite-intermediate Eisner recurrence produces one-root projective heads; first-ascending tie claims require finite candidate sums so initial -infinity is improved. Real optimality additionally requires candidate-order preservation. Coordinate transport distinguishes root keys from real loss rows. Empty/fallback tuples retain lengths/order, with fallback allowed to preserve invalid original trees. Training fused VJP preserves Q/K routes; inference fusion deliberately does not. Finite fixture scored/unscored equality is not a global rounding guarantee.

## cost

Encoder O(B*T*d+L*(B*T*d²+B*T²*d+B*T*d*f)); arc O(B*T*d*a+B*T*a²+B*T²*a); relation O(N*d*r+N*r²*K+N*r*K); tags O(N*d*(X+U)); preprocessing O(bytes+S log S+BT²), decoding sum O(n³). Attention storage B*H*T², encoder BTd, arc host transfer BT², plus parameters and training graphs/AdamW state. Independent dims H,a,r,K,X,U retained; BTd work persists at L=0. Integer products/allocations assumed representable. No speed claim.

## composition

data::batch roles index (k*T+d)*T+p; Net attention uses B,H,T,dh and masks head/key columns. lib transposes arcs dependent-major into Eisner head-major and maps local decoded h to flattened k*T+h for Net::rels. An adapter using raw h for k>0 gathers another sentence's encoder state. Vocs labels require same ordering as model output dimensions; load files independently and missing weights are not consistency proof. CLI score zips and can hide length mismatches, so output-length invariants must be checked before claiming dataset coverage. Existing world GPU caller uses parse_forms; tokenization remains caller-owned. Tests are sibling observers, not runtime production stages.

## domain

UTF-8 and valid Sent/Batch coordinates; Eisner square matrices with representable finite intermediate candidate sums for chart/backtrace tie guarantees, with special empty/singleton early branches. Exact-real maximum-tree interpretation additionally requires exact arithmetic or order-preservation across every rounded comparison. Net requires max_len>0, heads>0,d divisible heads, valid indices, resources and CUDA for selected ignored synthetic tests. No live model/corpus.

## limitations

CPU/GPU runtime and Candle kernels trusted beyond finite checks. CUDA tests serial on RTX4000 SFF Ada, compile cap89, two build jobs. Explicit CUDA Device::new_cuda calls do not depend on GP_CPU; CPU prediction uses GP_FUSED_SOFTMAX=0. Original model/corpus timing and optimizer experiments not executed. No full mathematical/semantic AST proof or percent-error surrogate.

## checks

- 17 exact selected tests, each twice debug and twice release; native-commands.json and sorted outcomes retained
- GELU 16001-point finite grid, value<5e-4, derivative<1e-3, finite_difference<1e-5
- Softmax widths1,3,17,33,257, views and masks: absolute forward/backward/finite-difference tolerance2e-6
- Tiny CUDA gradient routing and one synthetic optimizer step; no persisted model
- Compiled source mutants, exact restoration and selected positive reruns; see controls.json
- 13 paired syntax projections and source-bound nested fragment hashes; local augmented structural validation

## catalog

## behavior-complement

Within representable finite candidate sums, decoded tree cannot have multiple roots; no analogous guarantee is made for nonfinite recurrence defaults or copied overlength trees. Empty slots never enter reductions. Public training fusion uses differentiable softmax. Missing tensors preserve initialization, without rollback promise.

## reverse-reconstruction

Output preimages include discarded text fields, vocab/hash collisions and rounded f32 decoder decision regions; exact-real score cones describe them only under order-preservation. Softmax positive rows admit log(p)+constant; f32 zero can be underflow. Checkpoints omit optimizer moments, corpus and RNG history.

## coordinate-join

Batch row=k*T+i; tensor arc cell=(k*T+d)*T+h; decoder sc[h][d] receives that cell; relation head=k*T+h. Root index0 is local per sentence and never a real loss row.

## gradient-join

Train=true selects custom fused VJP or reference attention, retaining Q/K/V plus embedding/LN gradients; inference fused loses Q/K gradient while V still reaches loss. Checking any QKV gradient alone is insufficient.

## checkpoint-effects

CLI metadata writes precede epoch loop, checkpoints only strict improvements on full-dev/no-dev epochs, log follows. Failures may leave partial output. Selected synthetic optimizer mutates only ephemeral variables.

## score-metric

Scored prediction sums arc and raw relation plus half tag log probabilities before root/dep label rewriting. CLI percentage metrics count zipped tokens, whereas source tests assert prediction lengths explicitly; neither is normalized tree posterior.
