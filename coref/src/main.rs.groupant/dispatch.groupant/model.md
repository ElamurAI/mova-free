# coref-main-dispatch

## descriptant

### coref-main-dispatch

Command arity and effect ordering.

## approximant

### Scoped forward relation

Command arity and effect ordering.

Parent transition model (restricted to this byte scope): args takes first argument as command; --trees/out/label consume next token with missing-value error; --exact boolean; all other tokens positional. Repeated values last-win. resolve requires1, score2, eval1+out, mentions1; explain accepts>=1 and uses second as optional id, ignoring further positionals. load reads gold then optional tree, retrees and builds documents. Eval creates directory before input load, then run outputs and report write.

## invariant

### Scoped invariant/cost

Resolve gold stripping is implicit in detector behavior, not removal before load. Score loads both files without --trees and checks document count only; heads use each file's tree. Mention diagnostics use zip and zero denominators can print NaN. CLI results depend on filesystem and may leave partial effects.

Argument bytes linear; load holds raw cloned input plus tree/document views. Resolve/eval costs delegated; mention_stats matching plus map counts and sorted top-k summaries. Whole input/output strings are buffered, no resource cap.

Scope: Command arity and effect ordering.. Parent bounds are conservative; no separate timing measured.

## models

Scope: Command arity and effect ordering.. Relations below are restricted to this fragment's transitions and retained parent state.

# Typed models

## behavior_complement

Invalid dispatch arity cannot execute chosen processing arm and returns usage after argument parsing. Missing flag value errors before usage. Eval input failure does not exclude a newly created output directory. Unknown option tokens are accepted as positionals, not rejected as unknown flags.

## reverse_reconstruction

For stdout y, compatible argv/files may vary in ignored flags, extra explain positionals and discarded data. load output admits any aligned tree source with matching replacement columns; output does not uniquely identify file bytes or path. Error messages constrain failing stage rather than full prior state.

## argument_grammar

Flags are recognized after command, values consume even flag-looking tokens; duplicate path/label last wins. No -- terminator.

## dispatch_effects

resolve prints raw updated text; explain may print multiple documents; score computes two policies; eval creates dir, writes diagnostics/output then report; mentions prints counts.

## load_transport

load(path,Some(t))=documents(retree(read(path),read(t))); gold read failure precedes tree read/alignment, documents validates resulting trees.

## diagnostic_zero

mention_stats head recall=100 hit/ng and precision=100 hit/np without zero guard; exact conditional denominator max(hit,1). Ties in top tables sort count descending then lexical key.

No executable percentage surrogate: categorical state/effect semantics currently lack a useful cheaper observation metric. Supported future caps are 1%, 2%, 5%, 10%; no profile is claimed here.

## tests

### Evidence

Safe synthetic command: env -u COREF_X -u COREF_MD -u PRAG_OFF cargo test -p coref tests:: -- --skip score::. Two debug and two release executions are retained externally with numeric exit status and sorted named outcomes. Selected checks:

- wave27_tests::synthetic_load_preserves_gold_and_rejects_mismatched_trees

See source-test-map.json and controls.json in the wave27 coref run receipts. The library declaration file has compilation evidence only. No corpus, teacher, GPU or speed measurement.

Partial scope: Arg parser/process dispatch and output formatting not fully executed; synthetic load test only. No broad CLI or I/O-failure control.

#### Measured receipts

Final source state: 13 named tests (12 library, 1 binary) passed twice debug and twice release, 52 total executed test instances; all four command exits0 and sorted named outcomes identical. Five added tests are four library boundary tests plus one main load test. The pre-existing score tests and groupant_transfers were filtered out.

Compiled actual-source control: wave27_tests::synthetic_load_preserves_gold_and_rejects_mismatched_trees. Compile exit0, assertion-run exit101, exact bytes restored, positive rerun exit0. Retained logs: control-main-compile.log, control-main-red.log, control-main-restored.log.

Nine source AST projections were generated twice identically. Shared ancestor declarations are deferred to root integration; local owned leaves have no structural diagnostics. No timings or general accuracy claim.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: scoped view of parent state; no independent completeness claim.

### 2026-10-31 · tests

Heading `### Evidence — 2026-10-31` stays in tests as `### Evidence`.
