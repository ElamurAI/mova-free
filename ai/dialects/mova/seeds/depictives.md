# Depictives — `advcl` — proposal

**Proposal: accept** `advcl` for optional depictives, as in the 2.10 amendment, `_en/dep/advcl.md` and the EWT data. Do not take the outdated `acl` and `advmod` from `_en/specific-syntax.md` (`dialects/ud-2.18/seeds/depictives.md`).

**What exactly in `mova`.** *She entered the room **sad***, *came back **dead***, *came into office **obsessed** with Iraq* — the depictive adjective is `advcl` of the verb. The depictive's predicand (*she*), if visible, goes into EUD, not into the basic tree.

**Why.**
- **One answer instead of three.** `specific-syntax.md` contradicts itself: line 612 says `acl`, line 878 says `advmod`.
- **Matches the data and the standard:** EWT 2.18 has 22 depictives with `advcl`, 0 with `acl`, 0 with `advmod`.
- **More informative than `advmod`.** `advcl` shows that this is secondary predication, not manner.
- **Lossless:** export to 2.18 and to `ud-next` is identity.

**Effect on seeds.** Do not take the examples from `specific-syntax.md:878–890` (*unable*, *assured*) into seeds: in EWT they are annotated differently, as `parataxis` and VERB `advcl`.

**Origin.** `dialects/ud-2.18/seeds/depictives.md`; `2.18:UD docs/changes.md#optional-depictives`.

Mova decides.
