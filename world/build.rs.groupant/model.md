# world-build

## descriptant

The build script of the `world` crate. It decides whether the legacy central manuals (`state::CENTRAL_MANUALS`,
feature `worlds`, also enabled by `peta`) are compiled in: only when the feature is on and the folder
`peta/world/legacy/worlds` exists beside the crate. It exists so that a public checkout, which has no `peta/`,
compiles with `--all-features` (2026-11-05).

## approximant

Forward relation: F(feature_worlds, dir_exists) emits `rustc-check-cfg=cfg(mova_legacy_worlds)` and
`rerun-if-changed=build.rs` always; then

- feature off: nothing more;
- feature on, folder present: `rerun-if-changed=<folder>` and `rustc-cfg=mova_legacy_worlds`;
- feature on, folder absent: one `cargo:warning` naming the absent folder, no cfg.

Composition: `cfg(mova_legacy_worlds)` selects the `include_str!` table of `state.rs` (otherwise the empty table)
and the test `nerves::time_words_reach_the_world_of_time`. The freesync export treats this cfg as false
(`PRIVATE_BUILD_CFGS`), since the export has no `peta/`.

## behavior_complement

It never reads the manuals, never checks that the 33 named files exist (a partial folder fails later at
`include_str!`), never sets the cfg without the feature, and never fails the build. Watching a directory notices
added or removed entries, not edits inside files (cargo's directory rule).

## reverse_reconstruction

From the emitted cfg one recovers (feature on ∧ folder present); from its absence only (feature off ∨ folder
absent), which the warning disambiguates: a warning means feature on and folder absent. The folder's contents are
not recoverable.

## invariant-and-cost

Invariant: `mova_legacy_worlds` ⇒ feature `worlds` ∧ the folder existed when the script ran. Cost: one environment
lookup and one `stat`; O(1).

## tests

Compiled with and without the folder: `cargo check -p world --features peta` in the private checkout (manuals in),
and `cargo check --all-features` on the public export (warning, empty table), 2026-11-05. No unit test: the script's
inputs are cargo's environment and the file system.
