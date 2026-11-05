# wikt-page-text-command-xml-page

## descriptant

### wikt-page-text-command-xml-page

#### Descriptant

Full source read preceded changes. Prospective model predicted branch/order, resource and information-loss behavior. Tests exercise actual helpers on bounded synthetic inputs; command workflows remain unexecuted. Production source prefix is preserved; only inline tests added.

Event reader resets state on page start, marks revision start, reads title/ns/first ids, returns matching text or matching empty text. EOF returns None; XML/read_text/entity errors propagate.

Executable percentage approximation is unsupported: categorical parsing/effects have no useful certified metric here. Partial syntax excludes macro expansion and name resolution.

## approximant

### Forward relation

Event reader resets state on page start, marks revision start, reads title/ns/first ids, returns matching text or matching empty text. EOF returns None; XML/read_text/entity errors propagate.

#### Domain

UTF-8 rootless XML fragments, quick_xml configured check_end_names=false; tests exercise entities/ids/empty element only.

## models

# Typed source models

All models are partial source-derived relations at this manifest snapshot. Scope is one call/trace under the stated domain; no universal semantic proof.

## behavior_complement

Later contributor id cannot overwrite already filled rev; wrong title text cannot produce Some; EOF without matching text returns None.

Domain: UTF-8 rootless XML fragments, quick_xml configured check_end_names=false; tests exercise entities/ids/empty element only. Observation: Page state belongs to latest page start; title equality exact after entity decoding. Returning text does not validate remaining XML. Revision flag does not reset at revision end. Assumptions: bounded resources and source branch semantics. Oracle: selected actual-helper tests in model.md#tests constrain only their finite fixtures; remaining branches are source-derived.

## reverse_reconstruction

Returned Page constrains decoded selected fields only; compatible XML can differ in escapes, order of ignored metadata and content after return.

Domain: UTF-8 rootless XML fragments, quick_xml configured check_end_names=false; tests exercise entities/ids/empty element only. Observation: Page state belongs to latest page start; title equality exact after entity decoding. Returning text does not validate remaining XML. Revision flag does not reset at revision end. Assumptions: bounded resources and source branch semantics. Oracle: selected actual-helper tests in model.md#tests constrain only their finite fixtures; remaining branches are source-derived.

## page_reset

Start page clears title/in_rev and every Page field, preventing prior-page id/text carryover.

Domain: UTF-8 rootless XML fragments, quick_xml configured check_end_names=false; tests exercise entities/ids/empty element only. Observation: Page state belongs to latest page start; title equality exact after entity decoding. Returning text does not validate remaining XML. Revision flag does not reset at revision end. Assumptions: bounded resources and source branch semantics. Oracle: selected actual-helper tests in model.md#tests constrain only their finite fixtures; remaining branches are source-derived.

## id_first_write

id writes only empty destination, chosen by in_rev. Empty id content leaves slot empty for later id.

Domain: UTF-8 rootless XML fragments, quick_xml configured check_end_names=false; tests exercise entities/ids/empty element only. Observation: Page state belongs to latest page start; title equality exact after entity decoding. Returning text does not validate remaining XML. Revision flag does not reset at revision end. Assumptions: bounded resources and source branch semantics. Oracle: selected actual-helper tests in model.md#tests constrain only their finite fixtures; remaining branches are source-derived.

## empty_text_return

Empty text event with matching title returns current Page with empty text; nonmatching empty events ignored.

Domain: UTF-8 rootless XML fragments, quick_xml configured check_end_names=false; tests exercise entities/ids/empty element only. Observation: Page state belongs to latest page start; title equality exact after entity decoding. Returning text does not validate remaining XML. Revision flag does not reset at revision end. Assumptions: bounded resources and source branch semantics. Oracle: selected actual-helper tests in model.md#tests constrain only their finite fixtures; remaining branches are source-derived.

## early_return_scope

Return on matching text occurs before closing page/revision; malformed trailing chunk may remain unobserved.

Domain: UTF-8 rootless XML fragments, quick_xml configured check_end_names=false; tests exercise entities/ids/empty element only. Observation: Page state belongs to latest page start; title equality exact after entity decoding. Returning text does not validate remaining XML. Revision flag does not reset at revision end. Assumptions: bounded resources and source branch semantics. Oracle: selected actual-helper tests in model.md#tests constrain only their finite fixtures; remaining branches are source-derived.


## tests

### Evidence

Selected exact tests: ["wave33::xml_ids_text_and_empty_page"].

Each runs twice in debug and twice in release with normalized named outcomes compared. Compiled production-boundary mutants are recorded in the external wave33 text receipts with original/restored hashes and positive reruns. Existing negative assertions are separate from compiled mutants. No CLI, download, teacher, corpus or model execution. For this fragment, source-derived paths not reached by these tests remain unverified.

## invariant-and-cost

### Invariant and cost

#### Invariant

Page state belongs to latest page start; title equality exact after entity decoding. Returning text does not validate remaining XML. Revision flag does not reset at revision end.

#### Cost

O(chunk bytes until matching text), output and parser string allocations proportional to consumed fields; entire chunk already materialized.

Arithmetic and allocation premises are conditional. No timing or percentage surrogate claimed.

## History

### 2026-10-31 · descriptant

Heading `#### Epoch 2026-10-31` stays in descriptant as `#### Descriptant`.
