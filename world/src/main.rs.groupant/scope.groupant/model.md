# world-main-scope

## descriptant

### Environment path resolution and world-loading helpers before main

home=PathBuf(HOME or empty); data=MOVA_DATA or home/mova-data; db=MOVA_DB or data/db/mova.duckdb; tmp=WORLD_TMP or temp_dir/world; ftqa_dir=FTQA or data/raw/fairytaleqa. v3_mode iff WORLD_V3 is exactly 1. ftqa_part and v3_part parse usize or default to 95. dir_at(args,i)=PathBuf(args[i]) if present, else error. ftqa_text sorts directory file stems, locates name, assigns its 1-based catalogue ordinal and reads its TSV. load variants obtain Text first, then read cmds, parse_all, set total=accepted_parse_lines+format_errors and run_mode/run_v3.

## approximant

### Environment path resolution and world-loading helpers before main

home=PathBuf(HOME or empty); data=MOVA_DATA or home/mova-data; db=MOVA_DB or data/db/mova.duckdb; tmp=WORLD_TMP or temp_dir/world; ftqa_dir=FTQA or data/raw/fairytaleqa. v3_mode iff WORLD_V3 is exactly 1. ftqa_part and v3_part parse usize or default to 95. dir_at(args,i)=PathBuf(args[i]) if present, else error. ftqa_text sorts directory file stems, locates name, assigns its 1-based catalogue ordinal and reads its TSV. load variants obtain Text first, then read cmds, parse_all, set total=accepted_parse_lines+format_errors and run_mode/run_v3.

Categorical source relation; no executable percent surrogate or approximation error bound is claimed.

## invariant

A present empty folder is accepted as an empty PathBuf. WORLD_V3 values other than exactly 1 select false. Parsed zero part sizes are retained, not clamped. load returns parse-error rows alongside the executor result. State activation is outside this byte scope and is modeled only by the parent.

Cost: Path helpers scan/copy environment values. ftqa_text sorts N directory stems in O(N log N) plus TSV read; load reads command bytes and delegates parser/executor cost. Directory, UTF-8, TSV and world-executor failure behavior is delegated; no corpus-scale bound is established.

## composition

The parent main calls these helpers after activating state; activation is not in this fragment's byte range. The helpers adapt environment/argv into paths and Text, then command text into parsed lines plus format errors before world execution. The fragment does not justify an effect-free CLI.

## models

## behavior_complement

Exclude a missing-folder success, an error solely because a supplied folder string is empty, or v3=true for WORLD_V3=true/01. A failed Text read cannot be followed by the command-file read in a load helper, because ? returns first. This scope contains no state activation or teacher call.

## reverse_reconstruction

dir_at result p admits every argument vector with p at index i, regardless of other arguments; its missing-folder error admits all vectors of length <=i. A default part size 95 admits absent, unparsable, or explicit 95 environment values. total=2 admits two parsed commands, one command plus one format error, or two format errors; total alone loses that distinction.

## environment_fallback

Fallback occurs on missing/non-Unicode environment access or numeric parse failure, not empty present strings. Explicit empty MOVA_DATA is an empty path; explicit part=0 is valid usize parsing.

## catalogue_coordinate

ftqa_text uses sorted file stems as a catalogue, not a stable intrinsic document ID. Adding a preceding stem can shift a tale's ordinal; duplicate stems can share the first position.

## loader_failure_order

ftqa/poly/database Text acquisition precedes reading cmds; command file read precedes parse_all; execution consumes only parsed lines while errors remain returned separately.

## mode_transport

load and load_ftqa use run_mode(t,lines,v3_mode()); load_poly always uses run_v3. poly_text overrides document/title for two hardcoded names; other names use doc=0 and TSV title.


## tests

wave28_dialogue::folder_argument_missing_and_empty exercises actual dir_at for absent, empty and relative paths and usage error. All environment resolver and loader branches are source-derived only. main/state activation and corpus/database reads were not executed.

Commands and repeated outcomes: [parent evidence](../model.md#tests), [source evidence](../evidence.json). No new execution was required for this model-only refinement.

## History

### 2026-10-31 · descriptant

2026-10-31 review epoch: replaced generic fragment wrappers with scoped transitions, concrete exclusions and compatible antecedents. The exact byte range/hash and shared parent AST are retained. The parent main calls these helpers after activating state; activation is not in this fragment's byte range. The helpers adapt environment/argv into paths and Text, then command text into parsed lines plus format errors before world execution. The fragment does not justify an effect-free CLI.
