# world-schooltools-editing

## model

# world-schooltools-editing

## forward
Pure apply_edits keeps newline structure, matches old option after stripping star prefix, rejects ambiguous/missing/forbidden replacements, rebuilds line preserving indentation and CR. read_edits groups validated rows by resolved file. edit writes successful file buffers unless dry; snapshot copies sequentially; restore checks line-count and index domain before write.

## invariant
A refused pure edit returns no edited text; sequential successful edits observe earlier replacements. Restore of selected valid lines leaves other lines unchanged on successful I/O. No cross-file transaction guarantee.

## cost
Repeated whole-line clones and field scans per edit plus whole text split/join; read/write linear file bytes. Snapshot memory/I/O is delegated to fs::copy.

## composition
Leaf projections share exact lesson identities, selected indices, text bytes and option ordering. Compose only with those shared values retained; isolated score/length observations cannot recover them. Source inspection identifies knowledge::main_args dispatch, schoolbench selected-lessons -> counts -> run_course_lessons -> trace scoring, and schooltools option_line -> canon -> tally/lint/edit. Tests execute native pure helpers and isolated snapshot/restore; no live course/parser/teacher call.

## epoch-memory
Initial coverage records tolerant parser/score semantics and per-file effect boundaries. Model review exposed the direct zero restore index panic, repaired after old-code failure. Benchmark score loop was extracted unchanged to a pure private helper to test actual classification and width/count gates. Lock constructor failure may leave a stale directory; argument failures can consume flags. These remain explicit source-inspected limitations.

## checks

Native module filters schoolbench:: and schooltools::, including eight new wave9 tests; independent finite rank/cadence/length/fraction oracles and a restore no-effect fixture. Raw evidence is external under (see History); root ../model.md#evidence binds outcomes. Macro bodies in syntax projection remain opaque; finite agreement does not certify unbounded input space.

## behavior-complement
Evidence/status: source-inspected relation with the bounded tests named under checks; unenumerated behavior remains unknown. Complement claims are inner constraints, reverse claims retain all compatible candidates rather than identifying a historical cause.

## reverse-reconstruction
Evidence/status: source-inspected relation with the bounded tests named under checks; unenumerated behavior remains unknown. Complement claims are inner constraints, reverse claims retain all compatible candidates rather than identifying a historical cause.

## refusal-fiber
Evidence/status: source-inspected relation with the bounded tests named under checks; unenumerated behavior remains unknown. Complement claims are inner constraints, reverse claims retain all compatible candidates rather than identifying a historical cause.

## fixture-boundary
Evidence/status: source-inspected relation with the bounded tests named under checks; unenumerated behavior remains unknown. Complement claims are inner constraints, reverse claims retain all compatible candidates rather than identifying a historical cause.

## wave34-edit-sequence

Current nested child world-schooltools-editing-edit-sequence: Start with split newline buffer. For each edit validate line bounds, option recognition, nonempty separator-free new text, exactly one old option match after stripping stars, actual change, five fields, right-answer protection and casefold uniqueness. Successful edits rebuild that line preserving CR/star position; errors accumulate and later edits still run. Return buffer only if no errors.

Parent interaction: edit reads rows in table order grouped by file, calls apply_edits per file and writes only Ok unless dry. This is per-file refusal, not all-files transaction; successful prior files can remain written if another fails.

Earlier checks and epoch observations above remain historical, not new execution receipts. See file-root ../model.md#wave34-evidence for current selections.

## wave34-restore-selection

Current nested child world-schooltools-editing-restore-selection: Resolve snapshot copy by parent-basename/file-basename. Read saved then current lines split on newline. Refuse unequal line counts, then refuse any zero or oversized requested index before writing. Copy selected saved lines in list order, count only actual differences, write whole joined buffer, report changes.

Parent interaction: line_numbers normally sorts/deduplicates CLI indices, but public restore_lines admits duplicates. snapshot supplies copy naming. Count/index validation precedes the replacement loop and final fs::write; write failures may partially affect disk, outside preflight invariant.

Earlier checks and epoch observations above remain historical, not new execution receipts. See file-root ../model.md#wave34-evidence for current selections.

## wave34-composition

### Wave34 composition

edit-sequence consumes edits in table order with tentative per-line updates and accumulating refusals. The command writes only a file-level Ok and continues across files; a later refusal does not undo earlier file writes. restore-selection is a separate snapshot path: saved read, current read, equal count gate, all-index gate, selected replacement loop, then one write. CLI line_numbers deduplicates but direct restore admits duplicates; no prefix write occurs for a valid-then-invalid selection.

The previous model.md and its composition remain historical epoch memory. Current scoped relations use the manifest source hash; partial syntax and finite helper tests do not prove semantic completeness.

## History

### 2026-10-30 · document

Source-bound epoch 2026-10-30. Every view below uses this exact source revision and finite inputs with sufficient allocation resources. Forward/absent/reverse relations observe only the stated projection; filesystem models assume fixed supplied dependencies and distinguish successful returns from partial effects. Status: partial source model, with bounded native evidence in ../model.md#evidence at the root sidecar. No approximation percentage, universal proof or school accuracy claim.

### 2026-10-30 · checks

Native module filters schoolbench:: and schooltools::, including eight new wave9 tests; independent finite rank/cadence/length/fraction oracles and a restore no-effect fixture. Raw evidence is external under $MOVA_DATA/runs/octopus-wave9-school-2026-10-30; root ../model.md#evidence binds outcomes. Macro bodies in syntax projection remain opaque; finite agreement does not certify unbounded input space.

### 2026-10-30 · behavior-complement

Type: behavior_complement. Subject: world-schooltools-editing, epoch 2026-10-30. Domain and observation: scoped inputs and projected outputs defined here. Operator/assumptions: For admitted pure successful edits, new text is nonempty and lacks bar/tab/newline/CR, exactly one old option matched, final case-folded options unique. Direct zero restore cannot write or index after guard.

### 2026-10-30 · reverse-reconstruction

Type: reverse_reconstruction. Subject: world-schooltools-editing, epoch 2026-10-30. Domain and observation: scoped inputs and projected outputs defined here. Operator/assumptions: For known edit sequence and result, compatible originals must replay successfully; overwritten option bytes/spacing may be lost unless old values and formatting saved. A snapshot restores only selected lines and cannot recover shifted line structures.

### 2026-10-30 · refusal-fiber

Type: reverse_reconstruction. Subject: world-schooltools-editing, epoch 2026-10-30. Domain and observation: scoped inputs and projected outputs defined here. Operator/assumptions: Same missing-option error admits many original lines and earlier tentative edits; diagnostics do not uniquely reconstruct input text. Boundary tests distinguish a return of Err from a returned partial string.

### 2026-10-30 · fixture-boundary

Type: test_relation. Subject: world-schooltools-editing, epoch 2026-10-30. Domain and observation: scoped inputs and projected outputs defined here. Operator/assumptions: restore-zero fixture uses create_dir with PID and test-specific name and RAII removal; never falls back to school_root. Existing edit tests use temporary files. Production file resolution and actual course content remain unexercised by new tests.
