# Veilpick product and technical gap baseline

## Authority and status

This baseline is the continuation map for Veilpick's product contract. It is not release evidence.

- Protected default branch `develop` is the shipped repository authority.
- PR #1 is the Proposed documentation and architecture integration lane.
- Draft PR #2 is the Proposed bounded semantic-frontier implementation lane.
- An open PR, passing predecessor head, or feature-branch integration is not a released capability.

The current product decision is an ontology-guided, fully autonomous web acquisition engine built in Rust for authorized collection tasks. Ontology, autonomy, and stealth remain independent acceptance dimensions.

## Bounded contexts and context map

| Bounded context | Veilpick responsibility | Integration boundary |
| --- | --- | --- |
| Acquisition Task | Admit an authorized goal, budgets, cancellation, and terminal outcome | Caller authority is evidence supplied to the task; Veilpick does not create permission. |
| Semantic Frontier | Maintain candidate identity, concept guidance, ordering, deduplication, and bounded exhaustion | Draft PR #2 implements the first standard-library-only slice; dispatch is not acquisition success. |
| Governed Acquisition | Select and orchestrate transport/browser capabilities, session use, pacing, and recovery | Released OriginWeave contracts may supply reusable runtime authority and evidence through an ACL. |
| Adaptive Extraction | Interpret pages, produce candidate records, and retain exact source evidence | Task-local inference is not published ontology truth. |
| Challenge Resolution | Detect, classify, resolve declared supported classes, and verify trusted post-conditions | Model/browser strategies cannot create authentication, consent, or network authority. |
| Result Validation | Reconcile entities, validate required fields and relations, and report abstention/failure | Publication to catalogs or enterprise systems is an optional adapter operation. |

ConceptWeave may provide reviewed semantic contracts; contextual-orchestrator may provide bounded reasoning; context-graph-contracts may provide released assertion/provenance interoperability. None is mandatory infrastructure until a versioned contract, release, and consumer conformance evidence exist.

## Gap and action register

| ID | Gap | Current evidence | Required action and closure evidence | Status |
| --- | --- | --- | --- | --- |
| VP-G01 | No protected product/architecture baseline | [README](../README.md) and [ADR index](adr/README.md) exist only in PR #1 | Integrate PR #1 through ordinary governance with exact-head documentation, security, licensing, and independent review evidence | Proposed |
| VP-G02 | No released executable, package, or immutable version | No package or release is claimed in the README | Reproducible Rust build, SBOM/provenance, signed or otherwise verifiable immutable release, install and rollback path | Open |
| VP-G03 | Semantic frontier is not integrated | Draft PR #2 contains the first bounded implementation and tests | Merge its prerequisite, retarget without rewriting history, obtain exact-head Rust/test/coverage/security evidence, then integrate normally | Proposed |
| VP-G04 | Acquisition authority and runtime adapter are absent | [ADR 0003](adr/0003-stealth-and-ecosystem-composition.md) defines the boundary only | Versioned admitted-task contract, OriginWeave ACL/capability negotiation, denial/failure fixtures, consumer conformance tests | Open |
| VP-G05 | Stealth effectiveness is unmeasured | No controlled support matrix or matched baseline is released | Realistic authorized targets; fixed revisions; challenge incidence, cross-surface consistency, completion, failure, intervention, latency, and cost results | Open |
| VP-G06 | Adaptive extraction and ontology-guided replanning are absent | Product acceptance is documented; no implementation/release evidence exists | Source-bound extraction, task-local semantic model, reconciliation, completeness/abstention tests, provenance-preserving outputs | Open |
| VP-G07 | Automated challenge resolution is absent | [ADR 0001](adr/0001-automated-challenge-resolution.md) defines typed authority, retry, observation, and capability contracts | Supported-class matrix plus deterministic/browser/model strategies, post-condition evidence, zero-intervention successful runs, typed terminal failures | Open |
| VP-G08 | End-to-end product accuracy is unmeasured | No full acquisition experiment is released | Declare sampling design, target error, failure denominator, realistic cases, reproducibility, false continuation and extraction accuracy | Open |
| VP-G09 | Commercial dependency inventory is not yet applicable to a shipped graph | PR #1 adds no dependency; Draft PR #2 is standard-library-only | Before every dependency enters, verify provenance/license, forbid unapproved GPL/LGPL/AGPL or noncommercial intake, record NOTICE/attribution and SBOM obligations | Active control |
| VP-G10 | Operability and security evidence are incomplete | Security reporting and fail-closed principles are documented only | Threat model, secrets/credential-handle contract, audit events, resource limits, cancellation/recovery, container/runtime profile, incident and rollback procedures | Open |

## Decision and evidence rules

A gap becomes **Integrated** only when its canonical owner change reaches the protected branch and the exact unchanged head has required checks and review evidence. A capability becomes **Released** only when an immutable version and consumer-verifiable evidence exist. Draft, Proposed, blocked, or feature-branch work remains visible and is never counted as completion.

Every update to the PRD/TRD, ADRs, architecture, Context Map, API, database model, test contract, release, or material integration must update this register in the same authoritative lane. Contradictions are repaired at their owning source rather than hidden in customer-facing README copy.
