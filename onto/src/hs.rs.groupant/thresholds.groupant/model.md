# onto-hs-thresholds

## descriptant

### onto-hs-thresholds

endpoint=x*factor; complete last interval on opposite missing side with same attr/base

## approximant

### Scoped forward

endpoint=x*factor; complete last interval on opposite missing side with same attr/base

#### Domain

This byte-scoped operator uses the parent valid-state domain plus the following local restrictions:

cc→m³ factor1/1000000; kg/g rescale

below(h,l)=h<l OR(h=l AND NOT both inclusive)

Rat::mul error drops hit, not float approximation

overlaps requires caller equal attr/base/basis

No executable percentage surrogate established.

## invariant

### Local observations

No comparator excludes hit; touching open endpoint excludes overlap

cc→m³ factor1/1000000; kg/g rescale

below(h,l)=h<l OR(h=l AND NOT both inclusive)

Rat::mul error drops hit, not float approximation

overlaps requires caller equal attr/base/basis

## cost

### Scoped cost

O(T²) nearest-attribute/unit scans; rational arithmetic cost depends on integer width.

Restriction to this operator's traversed inputs; parent total is not charged to every fragment. No standalone measurement.

## composition

### Parent join

thresholds supplies the parent relation: endpoint=x*factor; complete last interval on opposite missing side with same attr/base

Output compatibility is conditional on the local domain and earlier-branch exclusions. Other parent branches are outside this fragment.

## models

## behavior_complement

No comparator excludes hit; touching open endpoint excludes overlap

## reverse_reconstruction

Any token/comparator/unit sequence normalizing to same endpoint pair is compatible

## unit_relation

cc→m³ factor1/1000000; kg/g rescale

## topology

below(h,l)=h<l OR(h=l AND NOT both inclusive)

## scaling_failure

Rat::mul error drops hit, not float approximation

## dimension_domain

overlaps requires caller equal attr/base/basis


## tests

### Evidence

Parent model.md#tests identifies selected actual-code checks. Fragment is a source-scoped partial model; parent finite fixtures do not establish exhaustive branch coverage or every local equation.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Specialized source fragment bytes [13087,24748); shared parent AST. Excludes unrelated parent operations and added test module.
