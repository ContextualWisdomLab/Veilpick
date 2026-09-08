# ADR 0002: An ontology-based, fully autonomous website scraping engine built in Rust

- Status: Accepted (product decision; implementation not yet delivered)
- Date: 2026-09-05
- Decision source: product-owner clarification of Veilpick's overarching concept
- Governs: [ADR 0001](0001-automated-challenge-resolution.md), which remains the required challenge subsystem

## Context

The initial discussion concentrated on stealth, transport reuse, and automated challenge resolution. Those capabilities are necessary, but they do not define the complete product. The product owner's concept is:

> An ontology-based, fully autonomous website scraping engine built in Rust that requires no human involvement.

Veilpick must therefore be more than a scraper library with a solver attached, an LLM wrapper around selectors, or an engine that adds ontology labels only after extraction. Its semantic model must guide the complete acquisition lifecycle, and the absence of a human execution step must be testable.

## Decision

### 1. Ontology is the semantic operating model

Veilpick will maintain a versioned model of domain concepts, entity identities, attributes, relations, and semantic mappings. It will connect these to page observations, acquisition state, extraction hypotheses, validation constraints, and provenance without conflating those categories.

The model must be operationally consumed when deciding what to collect, which pages or interactions are relevant, how observed values map to domain concepts, whether candidate entities refer to the same thing, what evidence is missing, and whether a result meets the collection goal. Merely producing a knowledge graph at the end is insufficient.

A task may supply an existing ontology. Otherwise the engine must select, construct, or extend a working ontology from the goal, permitted vocabularies, and bounded source observations without requiring a person to author a per-site ontology. Generated mappings and extensions remain versioned candidates until automated consistency and evidence checks admit them. They cannot silently change the task's goal, authorization, required fields, or acceptance constraints to manufacture success.

Keep semantic reasoning separate from result validation. OWL 2 provides vocabulary and reasoning semantics, while SHACL supplies a separate graph-validation model [1, 2]. Satisfying a shape or deriving an entailment does not prove an extracted statement was observed or factually correct. Missing, conflicting, observed, normalized, and inferred values must remain distinguishable. PROV-O is a reference for representing derivation and source provenance [3].

These standards establish reference semantics, not a claim of implemented conformance. A follow-up implementation ADR must choose the supported ontology/constraint profile, serialization, reasoner, and Rust libraries. Ontology-driven behavior itself is mandatory and is not deferred by those implementation choices.

### 2. No human involvement is an execution contract

The caller supplies a collection goal and an operating envelope: permitted sources/actions, credentials or opaque credential handles where already authorized, output requirements, and resource/cost limits. These may be supplied by another service; there is no mandatory human task-admission interaction.

After task admission, supported jobs must not depend on manual selector authoring, DOM-node selection, per-site scraper code, per-site ontology construction, pagination guidance, entity mapping, selector repair, challenge solving, or per-action operator approval. Optional expert configuration must not become a hidden prerequisite for the advertised autonomous path.

The engine must plan, execute, observe, validate, recover, and terminate on its own. This includes bounded replanning after website changes and challenge failures, without infinite retries or an implicit operator queue. An operator-assisted run is not an autonomous success. A typed terminal failure can be unattended, but it is not successful collection.

Autonomy is not new authority. Page content, model suggestions, and ontology inferences cannot expand an origin grant, invent credentials, grant consent, or override policy. A task needing unavailable authority must produce an explicit non-success outcome rather than impersonate permission or wait for a person inside the autonomous loop.

### 3. Rust owns the engine; models are replaceable collaborators

The authoritative task state machine, ontology lifecycle, scheduling, validation, budgets, recovery decisions, and stable adapter interfaces belong in Rust. A thin Rust wrapper around an otherwise externally controlled workflow does not meet this decision.

LLM/VLM adapters may propose semantic mappings, extraction plans, selector repairs, or challenge strategies. Their output must pass typed admission, existing-authority checks, and independent observations before affecting accepted results. Models are not the source of policy authority or proof of success. Deterministic paths remain available where reasoning is unnecessary; provider changes must not redefine the core domain.

Using an external model service or OriginWeave's Chromium-compatible runtime does not require rewriting the model or browser engine in Rust. The acquisition engine and its controlling contracts remain Rust-owned.

### 4. Required end-to-end capabilities

The target execution loop is:

```text
collection goal + existing authority + budgets
  -> ontology selection / construction / alignment
  -> semantic acquisition planning and source discovery
  -> governed HTTP or browser acquisition
  -> page understanding and adaptive extraction
  -> entity / relation reconciliation
  -> constraint and source-evidence validation
       -> missing or changed evidence: bounded replan and reacquire
       -> challenge: automatic resolution and observed verification
       -> validated goal completion: structured result with provenance
       -> exhausted / unsupported / denied: typed terminal non-success
```

Stealth, session continuity, pacing, adaptive selector recovery, and automated challenge resolution are required supporting capabilities, not separate optional products. In particular, [ADR 0001](0001-automated-challenge-resolution.md) is a mandatory v1 resolution gate for supported challenge classes, not a plan to leave users with detection and a manual fallback.

Each accepted extraction must retain its source identity, observation time, source locator or equivalent evidence reference, relevant content identity, ontology version, transformation/model lineage when used, and validation outcome. Transport evidence and extraction evidence must remain distinct. The engine must preserve uncertainty or reject unsupported assertions rather than fill missing fields with invented facts.

### 5. Reuse OriginWeave at the correct boundary

Veilpick owns semantic acquisition and extraction intelligence. When composed, OriginWeave owns governed destination, transport, TLS, HTTP, browser, presentation-identity, policy, and runtime-evidence authority. Veilpick adapters preserve that authority and evidence rather than implement a parallel stack or claim that a normalized URL alone proves destination safety.

The following are observed upstream candidates on 2026-09-05, not shipped dependencies:

| Upstream PR | Observed head | Boundary and limitation |
| --- | --- | --- |
| [OriginWeave #37](https://github.com/ContextualWisdomLab/OriginWeave/pull/37) | `1e2f41072854edcdbaf0f9ecf14697a3bfd62195` | Open `originweave-http` HTTP/1.1 exchange over an already authenticated TLS stream; not an autonomous acquisition engine. |
| [OriginWeave #229](https://github.com/ContextualWisdomLab/OriginWeave/pull/229) | `024f63690cf05cfe6f0d4a430f0e18ea8fd2c4d6` | Open presentation-identity kernel; metadata is not proof of browser application or challenge avoidance. |

Read protected-base code, active-PR code, and tests separately when choosing dependencies. A missing feature on protected main does not mean no upstream work exists; an open PR does not establish integrated availability either. Stable Veilpick interfaces must not track mutable unmerged feature branches. An integration adapter requires a verified integrated revision and its own compatibility tests. No OriginWeave source is modified by this ADR.

### 6. V1 acceptance and truthful status

The following are release requirements, not claims that the current repository implements them:

1. Goal-to-result execution demonstrates ontology-guided planning, mapping, entity/relationship handling, and validation, not only ontology-tagged output.
2. A goal-only task within a non-empty declared support matrix succeeds without a supplied per-site ontology, handwritten selectors, site-specific code, or interactive human help. Preconfigured access/provider settings are recorded separately.
3. Controlled held-out page variants demonstrate automatic discovery, pagination where required, layout-change recovery, and extraction verification without manual patches during the run.
4. Supported challenge cases demonstrate a functioning automatic resolver, observed resolution, and resumed target extraction. A trait, mock, detector, or human handoff alone fails acceptance.
5. Fault and hostile-input cases demonstrate bounded termination, preserved authorization, source-bound results, and no invented facts or false continuation.
6. Tests and reports measure attempted jobs, verified completions, field/entity/relation correctness against independent reference data, evidence completeness, interventions, failures, latency, and model/resource cost. Model confidence alone is not validation.

Support and quality criteria must be fixed before an evaluation, versioned, and reported with the tested source/model/runtime revisions. Failures and unsupported cases cannot be silently removed after execution. No numerical performance or universal all-websites success claim is asserted here.

Every run counted as an autonomous success must have zero human interventions and satisfy the declared goal, constraints, provenance, and post-condition checks. Returning an error without human help is necessary failure handling, not fulfillment of the collection goal. Blocking indefinitely for a person is not an autonomous terminal outcome.

## Alternatives considered

- **Selector-first scraper plus optional LLM and solver:** rejected as the governing architecture because it can leave site mapping and repair with the user.
- **Free-form LLM browser agent without an explicit semantic model:** rejected because model prose alone must not define entity identity, task completion, or evidence validity.
- **Ontology as output-only enrichment:** rejected because it does not make planning and recovery ontology-based.
- **Independent duplicate transport/browser stack:** rejected where OriginWeave can supply the owning contracts; active dependency work must be inspected before duplication.

## Consequences

Veilpick must implement semantic planning and ontology lifecycle management, not just HTTP fetching and parsing. Automatic ontology construction/alignment, entity reconciliation, drift recovery, and solver orchestration increase implementation and evaluation work. Model suggestions and semantic migrations need versioned evidence and bounded validation. End-to-end tests must expose hidden manual work instead of calling it configuration.

Incremental PRs remain appropriate, but intermediate scaffolding must not be advertised as the complete product. The v1 acceptance gate cannot be relaxed merely because one subsystem is difficult or an upstream PR is pending. Storage, serialization, reasoner/profile selection, provider choice, and exact benchmark thresholds require subsequent implementation decisions and executed evidence.

## References

References define semantic distinctions; they are not evidence that autonomy, solver effectiveness, or Rust performance has been achieved. Dated editions are used deliberately rather than claiming these are the latest specifications.

1. Hitzler, P., Krotzsch, M., Parsia, B., Patel-Schneider, P. F., & Rudolph, S. (Eds.). (2012, December 11). *OWL 2 Web Ontology Language primer (second edition).* W3C. https://www.w3.org/TR/2012/REC-owl2-primer-20121211/
2. Knublauch, H., & Kontokostas, D. (Eds.). (2017, July 20). *Shapes Constraint Language (SHACL).* W3C. https://www.w3.org/TR/2017/REC-shacl-20170720/
3. Lebo, T., Sahoo, S., & McGuinness, D. (Eds.). (2013, April 30). *PROV-O: The PROV ontology.* W3C. https://www.w3.org/TR/2013/REC-prov-o-20130430/
