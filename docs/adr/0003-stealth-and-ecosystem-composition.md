# ADR 0003: First-class stealth and ecosystem composition

- Status: Accepted (product decision; implementation and integration remain separately gated)
- Date: 2026-09-05
- Refines: [ADR 0002](0002-ontology-based-autonomous-rust-engine.md)
- Preserves: [ADR 0001](0001-automated-challenge-resolution.md)

## Context

The product owner confirmed that ontology, autonomy, and stealth remain equally necessary product dimensions. Ontology-based planning does not replace stealth, and an automated challenge resolver does not make stealth optional. The approved direction is to reuse ContextualWisdomLab components at their actual responsibility boundaries instead of building a parallel ecosystem inside Veilpick.

The canonical concept remains an ontology-based, fully autonomous website scraping engine built in Rust that requires no human involvement. Stealth is an explicit first-class acquisition capability within that concept, not a deprecated feature or an optional afterthought.

## Decision

### 1. Preserve three product dimensions

- **Ontology:** guide discovery, interpretation, extraction, entity/relation reconciliation, completeness assessment, and recovery.
- **Autonomy:** complete supported admitted tasks without human execution steps; plan, act, observe, validate, recover, and terminate within explicit budgets.
- **Stealth:** maintain a coherent presentation, transport/browser capability selection, session continuity, and request/navigation pacing to reduce avoidable detection and challenge escalation in authorized acquisition.

Challenge detection and automated resolution remain mandatory for v1's declared supported challenge classes under ADR 0001. Successful resolution does not substitute for stealth acceptance, and avoiding a challenge does not prove extraction correctness. No-human-involvement is not weakened to a hidden manual approval, site-specific ontology authoring, selector-repair, or solver queue.

### 2. Separate policy, application, and observed effectiveness

Veilpick owns task-level stealth requirements, acquisition strategy, session-use decisions, bounded pacing, and recovery orchestration. The integration must distinguish a requested profile, runtime capability admission, actual browser/transport application, and observed results. A metadata object, changed User-Agent, blocked fingerprint API, or command acknowledgement alone is not proof of stealth effectiveness.

OriginWeave is the intended owner of reusable presentation/fingerprint primitives, verified transport and browser application, and runtime evidence. Its bounded HTTP/1.1 exchange is not automatically equivalent to Chromium's network fingerprint or a complete browser acquisition path. A required unsupported capability must be reported explicitly; adapters must not silently downgrade and still claim the requested stealth profile.

Evaluate approved profiles on a versioned controlled support matrix with fixed runtime/model revisions and a matched baseline. Report challenge incidence, observed cross-surface consistency, verified extraction completion, failures, interventions, latency, and cost separately. A privacy improvement is not automatically an anti-detection improvement. This ADR supplies neither a universal undetectability claim nor a measured effectiveness result.

### 3. Compose through owning contracts

| Owner | Reuse direction | Boundary retained |
| --- | --- | --- |
| OriginWeave | Rust transport/browser/presentation adapters and runtime evidence | No copied HTTP/TLS stack; actual application and evidence remain distinct from profile metadata. |
| ConceptWeave | Evidence-bound semantic candidates and domain validation through a narrow semantic adapter | Its foundation is not a complete live ontology-induction engine or Veilpick planner. |
| contextual-orchestrator | Provider-neutral reasoning requests through its published gateway contract | Rust retains task state, budgets, admission, completion, and recovery; provider output is not authority. Multimodal benchmark code is not proof of a working Veilpick VLM integration. |
| context-graph-contracts | Versioned assertion/event interoperability and conformance vectors | Six truth/origin values are categories, not a confidence ladder. Preserve foreign assertions and temporal/provenance meaning. |
| RankWeave | Candidate-ranking reference semantics and differential-test fixtures | Do not copy a competing algorithm into Veilpick by default. An owner-maintained Rust core or service adapter requires its own reviewed decision and compatibility evidence. |
| LineageWeave | Reference pattern for offered-concept selection, exact evidence spans, and empty-result versus unavailable-result separation | Do not import its occupational vocabulary or assume its hierarchy traversal is valid for every ontology DAG. |
| semantic-data-portal | Optional catalog/graph output adapter | Catalog publication and its authorization are separate from local task completion; no cross-service database reads. |
| newsdom-api / mhtml-etl-gateway | Optional document/artifact adapters or parser conformance references | Check actual parser readiness, privacy and resource limits; Python services are not Rust-native in-process dependencies. |
| quarantine-sandbox-runtime | Candidate isolation boundary for untrusted artifact processors | Real isolation and cleanup evidence are required; planned or test-only controls are not available runtime guarantees. |
| Keyverse | Optional service-caller identity federation | Not a credential vault for arbitrary websites and not proof that an OriginWeave secret broker already ships. |

Veilpick remains the owner of crawl frontier, site exploration, task-local semantic planning, page-template discovery, adaptive extraction, semantic completeness and bounded replanning. Optional sinks and enterprise services must not become mandatory dependencies of standalone task execution.

### 4. Task ontology is not governed semantic publication

Consume ConceptWeave candidate/validation contracts only where available. A task-local working ontology may remain inferred or proposed while guiding acquisition. Using that local model is not a transition to `Reviewed`, `Published`, or `Authoritative` in ConceptWeave.

Do not bypass, rename, or weaken ConceptWeave's governance lifecycle to obtain an autonomous loop. Shared semantic publication remains a separate authorized operation. A reviewed machine policy can be an integration option only when the owning product actually supports and validates it; do not invent that support in an adapter.

Concept selection must retain evidence for the concept-to-source mapping. Exact text occurrence proves an occurrence, not that a semantic interpretation is correct. Normalization, inference, conflict resolution, and directly observed source content remain distinguishable. Hierarchical pruning needs held-out recall tests before it becomes a generic acquisition strategy.

### 5. Dependency availability must be proved

The following are dated source observations from the approved repository investigation, not statements that all components are shipped or deployable:

| Repository / PR | Inspected revision | Observed availability |
| --- | --- | --- |
| [OriginWeave #37](https://github.com/ContextualWisdomLab/OriginWeave/pull/37) | `1e2f41072854edcdbaf0f9ecf14697a3bfd62195` | Open HTTP/1.1 implementation candidate. |
| [OriginWeave #229](https://github.com/ContextualWisdomLab/OriginWeave/pull/229) | `024f63690cf05cfe6f0d4a430f0e18ea8fd2c4d6` | Open presentation-identity foundation; not proof of browser-applied stealth. |
| [OriginWeave #233](https://github.com/ContextualWisdomLab/OriginWeave/pull/233) | `28d21c29f14a7584cc40390b248df49ea31f285d` | Merged into `feat/privacy-presentation-identity`, not protected `main`; branch integration is not product shipment. |
| [ConceptWeave #1](https://github.com/ContextualWisdomLab/ConceptWeave/pull/1) | `b538470c963e6524ddc0c3f652a46a4fc8265150` | Draft Rust domain foundation; source adapters and ontology induction are separate gaps. |
| [context-graph-contracts #4](https://github.com/ContextualWisdomLab/context-graph-contracts/pull/4) | `7503b7da50efcad8256e5b6d3214e438c829cd50` | Open shared-contract work; no assumed published package or integrated Rust binding. |

Before adopting an upstream component, independently re-read its integration branch, required API, active PR ancestry, license, supported toolchain, release artifact, and exact-revision tests. Pin an eligible integrated revision or released artifact and run consumer compatibility tests. A feature-branch merge, predecessor CI result, README claim, or callable stub is not dependency acceptance.

### 6. Implement in independently verifiable slices

The first Rust slice should establish a bounded semantic crawl frontier: deterministic candidate selection from explicit concept requirements, deduplication, bounded attempts, and explicit exhaustion outcomes. It must not call an exhausted frontier a successfully completed collection. It must not own URL authorization, DNS, transport, browser state, or ontology publication.

Later slices compose source-bound extraction admission, task-local ontology integration, actual acquisition adapters, measured stealth, and automated challenge resolution. This order does not remove any v1 requirement. A local planning core or fixture is not the fully autonomous product.

## Alternatives rejected

1. Replacing stealth with ontology or solving: loses an approved product dimension.
2. Booting every ecosystem service to scrape one site: destroys standalone operability.
3. Copying provider routing, HTTP/TLS, ranking, or semantic publication code into Veilpick: creates competing owners and drift.
4. Treating all merged upstream PRs as protected-main availability: confuses feature-stack integration with delivery.
5. Calling typed failure, a model answer, or a mock resolver autonomous success: removes the actual completion requirement.

## Consequences and verification

Ports must be narrow, evidence-preserving, and independently testable. Where an upstream implementation is not integrated, record the dependency gap rather than claim a working adapter. Runtime capability admission, semantic candidate use, transport evidence, extraction validation, and task completion are distinct contracts.

Documentation acceptance checks must preserve first-class stealth, the no-human-execution requirement, mandatory supported-class resolution, the feature-branch merge distinction, and the task-ontology/publication separation. Runtime and release acceptance additionally require real end-to-end execution; documentation checks cannot satisfy them.

## Source records

- OriginWeave. [Architecture at inspected protected main](https://github.com/ContextualWisdomLab/OriginWeave/blob/87c4daa1830bac5a5228b6036752ad5633232085/ARCHITECTURE.md).
- ConceptWeave. [Domain contract at inspected foundation head](https://github.com/ContextualWisdomLab/ConceptWeave/blob/b538470c963e6524ddc0c3f652a46a4fc8265150/crates/conceptweave-domain/src/lib.rs).
- Context Graph Contracts. [Interoperability semantics at inspected head](https://github.com/ContextualWisdomLab/context-graph-contracts/blob/7503b7da50efcad8256e5b6d3214e438c829cd50/README.md).
- LineageWeave. [ADR 0253: Catalog-bound occupational construct extraction](https://github.com/ContextualWisdomLab/LineageWeave/blob/83eba56149eb802cd63642c507c324c9976ec78e/docs/adr/0253-catalog-bound-occupational-construct-extraction.md).
