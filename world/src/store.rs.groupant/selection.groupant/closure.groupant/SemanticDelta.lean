import SampleCarry

/- Abstract projection bridge only. The structural part, tag and resolved,
normalized lemma are parameters: this file does not verify a Rust projection,
normalization, dictionary lookup, feature extractor, query engine or cost.
A pointwise observer must use the same deterministic function in both states. -/
namespace SemanticDelta

structure Projection (Structure Tag Lemma : Type*) where
  structuralPart : Structure
  tag : Tag
  resolvedNormalizedLemma : Lemma
  deriving DecidableEq

variable {Id Value Structure Tag Lemma Result : Type*}

def Complete (P0 P1 : Id → Value) (C : Set Id) : Prop :=
  ∀ i, P0 i ≠ P1 i → i ∈ C

theorem complete_iff_outside (P0 P1 : Id → Value) (C : Set Id) :
    Complete P0 P1 C ↔ ∀ i, i ∉ C → P0 i = P1 i := by
  classical
  constructor
  · intro h i hi
    by_contra hn
    exact hi (h i hn)
  · intro h i hn
    by_contra hi
    exact hn (h i hi)

theorem pointwise_agrees_outside (P0 P1 : Id → Value) (C : Set Id)
    (h : Complete P0 P1 C) (observe : Value → Result) :
    ∀ i, i ∉ C → observe (P0 i) = observe (P1 i) := by
  intro i hi
  exact congrArg observe ((complete_iff_outside P0 P1 C).mp h i hi)

def selected (P : Id → Value) (query : Value → Prop) : Set Id :=
  {i | query (P i)}

theorem query_agrees_outside (P0 P1 : Id → Value) (C : Set Id)
    (h : Complete P0 P1 C) (query : Value → Prop) :
    ∀ i, i ∉ C → (i ∈ selected P0 query ↔ i ∈ selected P1 query) := by
  intro i hi
  change query (P0 i) ↔ query (P1 i)
  rw [(complete_iff_outside P0 P1 C).mp h i hi]

theorem carry_eq_of_complete (P0 P1 : Id → Value) (C : Set Id)
    (h : Complete P0 P1 C) (query : Value → Prop) :
    SampleCarry.carry (selected P0 query) (selected P1 query) C =
      selected P1 query :=
  (SampleCarry.carry_eq_iff_outside _ _ C).mpr
    (query_agrees_outside P0 P1 C h query)

/- Completeness is also necessary to support *every* pointwise predicate.
A single fixed predicate may legitimately need fewer changed IDs. -/
theorem complete_iff_all_queries (P0 P1 : Id → Value) (C : Set Id) :
    Complete P0 P1 C ↔
      ∀ query : Value → Prop,
        SampleCarry.carry (selected P0 query) (selected P1 query) C =
          selected P1 query := by
  constructor
  · intro h query
    exact carry_eq_of_complete P0 P1 C h query
  · intro h
    apply (complete_iff_outside P0 P1 C).mpr
    intro i hi
    have he := (SampleCarry.carry_eq_iff_outside _ _ C).mp
      (h (fun v => v = P0 i)) i hi
    have hp : P1 i = P0 i := he.mp rfl
    exact hp.symm

theorem projection_eq_iff (a b : Projection Structure Tag Lemma) :
    a = b ↔ a.structuralPart = b.structuralPart ∧
      a.tag = b.tag ∧ a.resolvedNormalizedLemma = b.resolvedNormalizedLemma := by
  cases a
  cases b
  simp only [Projection.mk.injEq]

/- In a fixed coarse structural fiber, precisely tag and lemma remain hidden.
This is a compatible-preimage relation, not a unique reconstruction claim. -/
theorem coarse_fiber (a b : Projection Structure Tag Lemma)
    (hs : a.structuralPart = b.structuralPart) :
    a = b ↔ a.tag = b.tag ∧
      a.resolvedNormalizedLemma = b.resolvedNormalizedLemma := by
  rw [projection_eq_iff]
  simp only [hs, true_and]

def original : Projection Nat Nat Nat := ⟨7, 3, 11⟩
def lemmaChanged : Projection Nat Nat Nat := ⟨7, 3, 12⟩
def tagChanged : Projection Nat Nat Nat := ⟨7, 4, 11⟩

theorem dropping_lemma_loses_information :
    (original.structuralPart, original.tag) =
      (lemmaChanged.structuralPart, lemmaChanged.tag) ∧
    original ≠ lemmaChanged := by decide

theorem dropping_tag_loses_information :
    (original.structuralPart, original.resolvedNormalizedLemma) =
      (tagChanged.structuralPart, tagChanged.resolvedNormalizedLemma) ∧
    original ≠ tagChanged := by decide

theorem coarse_complete_but_full_incomplete :
    Complete (fun _ : Unit => original.structuralPart)
      (fun _ : Unit => lemmaChanged.structuralPart) ∅ ∧
    ¬ Complete (fun _ : Unit => original) (fun _ : Unit => lemmaChanged) ∅ := by
  constructor
  · intro i h
    exact False.elim (h rfl)
  · intro h
    exact h () dropping_lemma_loses_information.2

theorem coarse_carry_can_fail :
    SampleCarry.carry
      (selected (fun _ : Unit => original) (fun p => p.resolvedNormalizedLemma = 11))
      (selected (fun _ : Unit => lemmaChanged) (fun p => p.resolvedNormalizedLemma = 11))
      ∅ ≠
    selected (fun _ : Unit => lemmaChanged) (fun p => p.resolvedNormalizedLemma = 11) := by
  intro h
  have he := Set.ext_iff.mp h ()
  simp [SampleCarry.carry, selected, original, lemmaChanged] at he

#print axioms carry_eq_of_complete
#print axioms complete_iff_all_queries
#print axioms coarse_fiber
#print axioms coarse_carry_can_fail
end SemanticDelta
