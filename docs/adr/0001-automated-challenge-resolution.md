# ADR 0001: Automated challenge resolution is a first-class Veilpick capability

- Status: Accepted (product decision; implementation not yet delivered)
- Date: 2026-09-05
- Governing product contract: [ADR 0002](0002-ontology-based-autonomous-rust-engine.md)

## Context

Veilpick's overarching concept is an ontology-based, fully autonomous website scraping engine built in Rust that requires no human involvement. Challenge resolution is a necessary subsystem of that end-to-end contract, not the product's primary identity. The ontology-based planning, extraction, validation, and recovery responsibilities are defined in ADR 0002.

Its stealth capability should reduce unnecessary bot-management challenges by maintaining coherent presentation identity, session continuity, request pacing, and browser/network behavior. Avoiding challenges is not sufficient as a product contract. A site can still present CAPTCHA, JavaScript/interstitial challenges, consent or interaction gates, authentication gates, or other challenge states. Requiring human intervention at that point would break unattended acquisition.

Veilpick is expected to integrate with large language model (LLM) and vision-language model (VLM) capabilities where semantic reasoning is useful. Challenge handling must be an explicit product boundary rather than an accidental error path or an external manual procedure.

OriginWeave is a related ContextualWisdomLab runtime and control-plane project. Its current and proposed contracts separate origin, resolved destination, transport, TLS identity, HTTP semantics, presentation identity, policy, browser actions, and evidence. Veilpick should consume those governed capabilities where appropriate rather than duplicate their authority boundaries.

## Decision

Automated challenge resolution is a mandatory Veilpick v1 capability for the declared supported challenge classes. A detection-only implementation, an empty resolver interface, or a human handoff does not satisfy this requirement. Implementation may proceed incrementally, but that sequencing does not defer the v1 acceptance gate.

Veilpick will:

1. minimize unnecessary challenge escalation through coherent presentation identity, session continuity, request pacing, and browser behavior;
2. detect challenge states explicitly rather than treating challenge pages as ordinary extraction results;
3. classify supported challenge families and retain bounded evidence sufficient for policy and post-condition decisions;
4. expose a typed resolution boundary with functioning deterministic, browser-interaction, LLM, VLM, or other explicitly configured strategies appropriate to the supported classes;
5. resolve supported challenges without human intervention during the task;
6. verify the observed post-condition after a resolution attempt before declaring the challenge resolved; and
7. return a typed unresolved-challenge outcome when no configured and authorized strategy can establish that post-condition within the task's budgets.

A successful solver invocation, model response, browser command acknowledgement, or interaction attempt is not proof of resolution. Resolution succeeds only when subsequent trusted observation establishes the expected challenge-free post-condition and the intended acquisition can resume.

The minimum trusted-observation contract records the observer's admitted authority; observation time and sequence; the original origin, request, and session correlation; the requested resource identity; and source-backed evidence for each expected post-condition. `ChallengeObservation` carries those facts. `ChallengeResolution` and `ChallengeDisposition` carry the expected condition, accepted freshness window, correlation result, and evidence reference. Missing, stale, cross-origin, cross-session, or wrong-resource evidence cannot produce `resolved`; it returns a typed unresolved or failed disposition.

The autonomous execution path must not pause awaiting human interaction. Any separate operator-assisted workflow is outside the autonomous success path, and an assisted run must never count as a successful autonomous run. Automatic failure is also not successful resolution.

## Architectural boundary

The intended flow is:

```text
ontology-guided acquisition plan
  -> challenge detection and classification
  -> bounded evidence and existing-authority check
  -> strategy selection
  -> automated challenge resolution
  -> trusted post-condition observation
       -> resolved: resume acquisition and validate the target content
       -> unresolved: bounded replan or typed terminal failure
```

The subsystem should expose typed contracts such as `ChallengeObservation`, `ChallengeKind`, `ChallengeResolver`, `ChallengeResolution`, and `ChallengeDisposition` without coupling the core domain to a particular model provider. These are proposed domain names, not implemented APIs.

Challenge observations and outcomes participate in the ontology-guided execution state so the planner can distinguish a missing datum, a changed page, and an unresolved interaction gate. No observation or inferred ontology relation grants network, authentication, consent, or browser-action authority.

LLM/VLM-backed resolution is an implementation strategy, not the definition of the boundary. Deterministic strategies remain usable when semantic reasoning is unnecessary. All strategies share explicit attempt, elapsed-time, resource, and model-cost budgets; they cannot start unbounded retries or silently introduce a human solver.

Evidence may leave the runtime for an LLM/VLM provider only when the admitted task explicitly authorizes that provider and transmission purpose. The strategy must use an allowlisted provider, minimize and redact evidence before transmission, block credentials, cookies, authorization headers, and runtime-owned handles, and apply declared retention and logging controls. The request records the provider, redaction policy, transmitted evidence references, and response provenance. Provider output remains untrusted. If these controls or consent are absent, the provider strategy is unavailable and resolution returns a typed unresolved or failed disposition without a paid or unapproved bypass.

Veilpick must not reinterpret a challenge as successful content acquisition. Challenge observations, attempted resolutions, and post-condition results remain distinguishable in evidence and diagnostics. Page content and model output remain untrusted inputs, and protected credentials stay behind runtime-owned handles.

## OriginWeave integration

Veilpick should not recreate OriginWeave authority that is already available or being established there. Destination, exact TCP peer, authenticated TLS, bounded HTTP, presentation-identity, browser-action, policy, and runtime-evidence contracts should be reusable through narrow adapters when available on a verified integrated generation.

Veilpick owns ontology-guided acquisition orchestration, crawl frontier management, parsing, adaptive extraction, challenge recognition and resolution orchestration, extraction validation, and extraction-facing results. OriginWeave owns governed transport/browser authority and associated runtime evidence where composed. Veilpick must add its own extraction provenance without upgrading runtime evidence into proof that an extracted statement is true.

Veilpick must not bind its stable core API directly to an unmerged OriginWeave feature branch. Integration adapters may follow independently versioned contracts after their required dependencies reach an appropriate integrated generation. ADR 0002 records the relevant active upstream PRs without claiming they have shipped.

## Consequences

### Positive

- Unattended acquisition remains the product objective when challenges occur.
- Stealth and resolution become complementary capabilities under one autonomous engine.
- CAPTCHA and bot-management handling are not deferred into an undefined manual process.
- Model strategies remain replaceable, while semantic execution state and verification remain engine-owned.
- Post-condition verification prevents an interaction attempt from being reported as resolution.
- OriginWeave primitives can be reused without creating a competing browser control plane.

### Costs and risks

- Recognition and resolution require realistic browser-level tests, not only parser or mocked-provider tests.
- Site and provider behavior can change independently of Veilpick.
- Model strategies add latency, cost, and nondeterminism that must be measured.
- Ambiguous side effects require reconciliation before retry rather than blind redispatch.
- Authorization and site-policy constraints remain independent of technical ability; a strategy cannot create permission to access a resource. Authentication and consent gates must retain their distinct authority requirements.

## Acceptance implications

V1 acceptance requires a non-empty, versioned supported-challenge matrix and executed end-to-end evidence for detection, classification, automated resolution, post-condition verification, and resumed target extraction. CAPTCHA/bot-management handling remains explicitly in scope; it must not be silently removed from that matrix to obtain a passing result.

An interface-only, detection-only, mock-only, or manually assisted path is not sufficient. Results must report attempted tasks, verified completions, incorrect continuations, unsupported cases, terminal failures, latency, cost, and human interventions. The intervention count for every run claimed as autonomously successful must be zero.

No universal success claim is made for every possible challenge. Unsupported or unsuccessfully resolved cases must terminate explicitly rather than fabricate extraction, continue without verification, or wait indefinitely for a person. Such outcomes do not satisfy the successful-resolution acceptance criterion.
