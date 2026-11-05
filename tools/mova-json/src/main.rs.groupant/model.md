# mova-json CLI

## domain
argv of UTF-8 strings: a subcommand, positionals and the flags --str, --raw, --reformat, --add, --rebind-all, --paths (consumes following positionals) and `--` (ends flags).

## forward
parse_args splits flags from positionals; unknown `--x` flags are errors. run dispatches by subcommand and exact/minimum arity: check prints layout and exits 4 when not byte-identical; get prints the value in the file's layout (--raw: bare string); set/del/push/insert-before/remove-value/replace-str load for edit, apply the library operation, save and print a one-line result; rebind, merge3 (exit 3 on conflicts), rebind-groupant, jsonl-count (count TAB label, count descending), jsonl-filter (matching lines). Usage errors exit 2, other errors 1 with one message on stderr.

## invariant
VALUE arguments are JSON unless --str; a non-JSON value without --str is an error, never a silent string. No command runs a shell or anything but `git` (through the library).

## cost
O(argument bytes) plus the delegated library work and one file read/write.

## behavior-complement
Not done: reading values from stdin, batch scripts, globbing, in-place edits of files the library refuses, any network access.

## reverse-reconstruction
stdout lines identify the command's effect (index, count, deleted value); exit codes 1/2/3/4 partition error, usage, conflict and non-reproducible layout, but an exit 1 alone has several causes.

## flag-partition
Type: command_arity_partition. A value starting with `--` reaches the positionals only after `--`; otherwise it is refused as an unknown flag, so `--add-dir` is never misread.

## checks

CLI runs on copies of the private crab.json and deputy.json (not in the public tree): insert-before an argv anchor, remove-value, a same-value set leaving the file unchanged, replace-str there and back to identical bytes, del; jsonl-count/filter on a real Crab flow journal, two identical runs.

## epoch-memory

Thin transport over the library; a release build is installed for the Octopus outside the repository.

## History

### 2026-11-05 · checks

CLI runs on copies of the private crab.json and deputy.json (not in the public tree) (2026-11-05): insert-before an argv anchor, remove-value, a same-value set leaving the file unchanged, replace-str there and back to identical bytes, del; jsonl-count/filter on a real Crab flow journal, two identical runs.

### 2026-11-05 · epoch-memory

2026-11-05: thin transport over the library; a release build is installed for the Octopus outside the repository.
