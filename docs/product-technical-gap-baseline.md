# Veilpick product and technical gap baseline

## Authority and status

This baseline is the continuation map for Veilpick's product contract. It is not release evidence.

- Protected default branch `develop` is the shipped repository authority. The
  documentation baseline from PR #1 is integrated there at `cdaae4519db95141b88080d23d3cbeab6cfca31b`.
- PR #1 is merged; it is no longer an open prerequisite.
- Draft PR #2 is the Proposed bounded semantic-frontier implementation lane at
  `611d8d44df598b19c66b7adc75599e3899fa72a0`, based directly on `develop`.
  Its predecessor code head passed the complete Rust lane; the current docs-only
  evidence repair must still be evaluated on its own exact head.
- Draft PR #3 is the Proposed access-challenge and site-reputation design lane;
  ADRs 0004 and 0005 remain Proposed and contain no runtime implementation.
- An open PR, passing predecessor head, or feature-branch integration is not a released capability.

The current product decision is an ontology-guided, fully autonomous web acquisition engine built in Rust for authorized collection tasks. Ontology, autonomy, and stealth remain independent acceptance dimensions.

## Bounded contexts and context map

| Bounded context | Veilpick responsibility | Integration boundary |
| --- | --- | --- |
| Acquisition Task | Admit an authorized goal, budgets, cancellation, and terminal outcome | Caller authority is evidence supplied to the task; Veilpick does not create permission. |
| Semantic Frontier | Maintain candidate identity, concept guidance, ordering, deduplication, and bounded exhaustion | Draft PR #2 implements the first standard-library-only slice; dispatch is not acquisition success. |
| Governed Acquisition | Select and orchestrate transport/browser capabilities, session use, pacing, and recovery | Released OriginWeave contracts may supply reusable runtime authority and evidence through an ACL. |
| Access Challenge Engine | Keep access/challenge state, budgets, supported-strategy admission, and verified post-conditions independent of the planner | Proposed ADR 0004 incubates the core in Veilpick; OriginWeave retains runtime/browser authority and Wardnet does not own outbound resolution. |
| Site Reputation | Evaluate immutable evidence snapshots across security, source reliability, access, and coverage without a universal score | Proposed ADR 0005 keeps this independently deployable; Wardnet can supply observations but not the decision. |
| Adaptive Extraction | Interpret pages, produce candidate records, and retain exact source evidence | Task-local inference is not published ontology truth. |
| Result Validation | Reconcile entities, validate required fields and relations, and report abstention/failure | Publication to catalogs or enterprise systems is an optional adapter operation. |

ConceptWeave may provide reviewed semantic contracts; contextual-orchestrator may provide bounded reasoning; context-graph-contracts may provide released assertion/provenance interoperability. None is mandatory infrastructure until a versioned contract, release, and consumer conformance evidence exist.

## Gap and action register

| ID | Gap | Current evidence | Required action and closure evidence | Status |
| --- | --- | --- | --- | --- |
| VP-G01 | Protected product/architecture baseline | [README](../README.md), [ADR index](adr/README.md), and this register are integrated on protected `develop` at `cdaae4519db95141b88080d23d3cbeab6cfca31b` through merged PR #1 | Preserve the protected baseline and update it in the same canonical lane as material product decisions | Integrated |
| VP-G02 | No released executable, package, or immutable version | No package or release is claimed in the README | Reproducible Rust build, SBOM/provenance, signed or otherwise verifiable immutable release, install and rollback path | Open |
| VP-G03 | Semantic frontier is not integrated | Draft PR #2 current head `611d8d44df598b19c66b7adc75599e3899fa72a0` contains the bounded implementation and 25 tests on a history-integrated `develop` base; predecessor code head `4f62c9d81b5811bfdd26de75640445345f75c32b` passed Rust run `36806174987` | Obtain exact-current-head hosted evidence and independent review, then integrate normally without treating a successful feature head as a release | Proposed |
| VP-G04 | Acquisition authority and runtime adapter are absent | [ADR 0003](adr/0003-stealth-and-ecosystem-composition.md) defines the boundary only | Versioned admitted-task contract, OriginWeave ACL/capability negotiation, denial/failure fixtures, consumer conformance tests | Open |
| VP-G05 | Stealth effectiveness is unmeasured | No controlled support matrix or matched baseline is released | Realistic authorized targets; fixed revisions; challenge incidence, cross-surface consistency, completion, failure, intervention, latency, and cost results | Open |
| VP-G06 | Adaptive extraction and ontology-guided replanning are absent | Product acceptance is documented; no implementation/release evidence exists | Source-bound extraction, task-local semantic model, reconciliation, completeness/abstention tests, provenance-preserving outputs | Open |
| VP-G07 | Automated challenge resolution is absent | [ADR 0001](adr/0001-automated-challenge-resolution.md) defines typed authority, retry, observation, and capability contracts | Supported-class matrix plus deterministic/browser/model strategies, post-condition evidence, zero-intervention successful runs, typed terminal failures | Open |
| VP-G08 | End-to-end product accuracy is unmeasured | No full acquisition experiment is released | Declare sampling design, target error, failure denominator, realistic cases, reproducibility, false continuation and extraction accuracy | Open |
| VP-G09 | Commercial dependency inventory is not yet applicable to a shipped graph | PR #1 adds no dependency; Draft PR #2 is standard-library-only | Before every dependency enters, verify provenance/license, forbid unapproved GPL/LGPL/AGPL or noncommercial intake, record NOTICE/attribution and SBOM obligations | Active control |
| VP-G10 | Operability and security evidence are incomplete | Security reporting and fail-closed principles are documented only | Threat model, secrets/credential-handle contract, audit events, resource limits, cancellation/recovery, container/runtime profile, incident and rollback procedures | Open |
| VP-G11 | Independent access-challenge engine is design-only | Draft PR #3 proposes [ADR 0004](adr/0004-independent-anti-bot-engine.md), a bounded context, contract shapes, acceptance cases, and an implementation plan | Implement test-first Rust state transitions and ports, real supported-class fixtures, durable budget/effect recovery, exact post-condition evidence, security/performance evidence, and an immutable release before consumer integration | Proposed |
| VP-G12 | Evidence-based site reputation is design-only | Draft PR #3 proposes [ADR 0005](adr/0005-evidence-based-site-reputation-engine.md), multidimensional evidence semantics, persistence boundaries, acceptance cases, and an implementation plan | Implement immutable snapshot/version/revocation semantics, PostgreSQL adapter and migrations, conformance/security/performance tests, independently deployable API, and an immutable release | Proposed |

## Decision and evidence rules

A gap becomes **Integrated** only when its canonical owner change reaches the protected branch and the exact unchanged head has required checks and review evidence. A capability becomes **Released** only when an immutable version and consumer-verifiable evidence exist. Draft, Proposed, blocked, or feature-branch work remains visible and is never counted as completion.

Every update to the PRD/TRD, ADRs, architecture, Context Map, API, database model, test contract, release, or material integration must update this register in the same authoritative lane. Contradictions are repaired at their owning source rather than hidden in customer-facing README copy.
