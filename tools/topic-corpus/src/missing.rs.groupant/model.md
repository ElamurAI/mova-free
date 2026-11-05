# mova-topic-corpus-missing

## Descriptant

Stage 2: the cited works the library lacks (ids, else titles), looked up in the local OpenAlex snapshot by the library's `papers openalex lookup`, ranked by citations, routed to a legal open copy (ACL Anthology PDF, arxiv.org/pdf, OpenAlex best-OA PDF outside shadow libraries) and written as a new request run directory for the library's stream driver (manifest chunk, list-only works, ranking, run.sh with ROBOTS=1 HOST_SKIP=1 PDFDROP=1). Nothing is started. Source: `tools/topic-corpus/src/missing.rs`.

## Forward approximant

external(paper bibliography + bibliographies of library references and similar works) -> keys acl/arxiv/doi/title -> one lookup call -> title keys re-keyed by the record's ids when the year agrees -> route -> sort by (cited_by, neighbourhood count, by paper, key) -> write_request (disk guard >= 300 GB, new dir only, at most max_requests rows).

## Invariant

No shadow-library host is ever queued; a request directory is never rewritten; ACL licence by year (CC BY 4.0 from 2016, CC BY-NC-SA 3.0 before).

## Cost

One snapshot scan (~10-20 s, 12 threads in the papers binary) plus linear work in candidates.

## Composition

Parent: `topic-corpus`.

## behavior_complement

Does not download, does not start or resume streams, does not touch stopped streams, does not use the OpenAlex API. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

request.json keeps every candidate with its route and reason; the original bibliography lines are in the library. No unique inverse is claimed.
