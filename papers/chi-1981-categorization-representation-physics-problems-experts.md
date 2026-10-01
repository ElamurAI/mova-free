# Categorization and Representation of Physics Problems by Experts and Novices

**Authors:** Michelene T. H. Chi, Paul J. Feltovich, Robert Glaser · **Year:** 1981 · **Venue:** Cognitive Science 5(2)
**Link:** https://doi.org/10.1207/s15516709cog0502_2 (DOI 10.1207/s15516709cog0502_2)
**License of the paper:** unknown (publisher copyright; no open copy known)

## Summary
In a series of four experiments the authors asked physics experts and novices to sort, describe and plan solutions for mechanics problems. Novices grouped problems by surface features, such as inclined planes or springs, while experts grouped them by the underlying principle needed to solve them, such as conservation of energy or Newton's second law. The categories people use determine how they represent a problem and which knowledge they bring to it, so the difference in representation largely explains the difference in problem-solving skill. Protocols show that experts' knowledge is organised around principles linked to the conditions under which they apply. The paper is a classic of expertise research and of cognitive science of problem solving.

## How Mova uses it
- `ai/world/src/quant.rs` — the quantitative world layer for text word problems: each state change in a script is tied to a sentence (`@N`) and labelled with the principle it represents (`k=total`, `gain`, `loss`, `transfer`, `rate`, `compare`, `part`, `convert`, `combine`, `unit`, `given`), following the paper's lesson that experts group by principle rather than surface.
- Scripts are drafted by a large model, executed by the exact rational-number core, and only those that reproduce the reference answer using numbers from the text pass the gate; the small model then learns from verified scripts, including the principle labels.

## Effectiveness in Mova
Design inspiration for the principle labels (`k=`) in quantitative world scripts. The labels are part of the world-model learning that is measured as a whole, but the principle-labelling idea itself is not measured separately.

---

👨‍🔬💥
