# ADR 0001: Automated challenge resolution is a first-class Veilpick capability

- Status: Accepted
- Date: 2026-09-05

## Context

Veilpick is intended to be a Rust-native web acquisition, crawling, parsing, and adaptive extraction engine. Its stealth capability should reduce unnecessary bot-management challenges by maintaining coherent presentation identity, session continuity, request pacing, and browser/network behavior.

Avoiding challenges is not sufficient as a product contract. A site can still present CAPTCHA, JavaScript/interstitial challenges, consent or interaction gates, authentication gates, or other challenge states for reasons outside Veilpick's direct control. Requiring routine human intervention at that point would break unattended crawling and make challenge handling an operational bottleneck.

Veilpick is expected to integrate with LLM/VLM capabilities where semantic reasoning is useful. Challenge handling therefore needs to be represented as an explicit product boundary rather than an accidental error path or an external manual procedure.

OriginWeave is a related ContextualWisdomLab runtime and control-plane project. Its current and proposed contracts separate origin, resolved destination, transport, TLS identity, HTTP semantics, presentation identity, policy, browser actions, and evidence. Veilpick should consume those governed capabilities where appropriate rather than duplicate their authority boundaries.

## Decision

Automated challenge handling is a first-class Veilpick capability and part of the product scope.

Veilpick will:

1. attempt to minimize unnecessary challenge escalation through coherent presentation identity, session continuity, request pacing, and browser behavior;
2. detect challenge states explicitly rather than treating challenge pages as ordinary extraction results;
3. classify supported challenge families and retain bounded evidence sufficient for policy and post-condition decisions;
4. expose a typed challenge-resolution boundary capable of deterministic, browser-interaction, LLM, VLM, or other explicitly configured resolution strategies;
5. support automated resolution without requiring routine human intervention as the normal success path;
6. verify the observed post-condition after a resolution attempt before declaring the challenge resolved; and
7. return a typed unresolved-challenge outcome when no configured and authorized strategy can establish the required post-condition.

A successful solver invocation, model response, browser command acknowledgement, or interaction attempt is not proof of resolution. Resolution succeeds only when subsequent trusted observation establishes the expected challenge-free post-condition.

Human intervention may exist as an exceptional operational fallback, but it is not the primary product contract and does not satisfy automated challenge-resolution acceptance criteria.

## Architectural boundary

The intended flow is:

```text
acquisition
  -> challenge detection
  -> challenge classification
  -> policy / strategy selection
  -> automated challenge resolution
  -> trusted post-condition observation
       -> resolved: resume acquisition
       -> unresolved: typed failure
```

The challenge subsystem should expose stable typed contracts such as `ChallengeObservation`, `ChallengeKind`, `ChallengeResolver`, `ChallengeResolution`, and `ChallengeDisposition` without coupling the core domain to a particular model provider or solver implementation.

LLM/VLM-backed resolution is an implementation strategy behind this boundary, not the definition of the boundary itself. Deterministic strategies should remain usable when semantic model reasoning is unnecessary.

Veilpick must not silently reinterpret a challenge as successful content acquisition. Challenge observations, attempted resolutions, and post-condition results should remain distinguishable in evidence and diagnostics.

## OriginWeave integration

Veilpick should not recreate OriginWeave authority that is already available or being established there. In particular, OriginWeave's destination, exact TCP peer, authenticated TLS, bounded HTTP, presentation-identity, browser-action, policy, and evidence contracts should be reusable through narrow adapters when those contracts are available on an integrated OriginWeave generation.

Veilpick remains responsible for acquisition orchestration, crawl frontier management, parsing, adaptive extraction, challenge recognition, challenge-resolution strategy orchestration, and extraction-facing results. OriginWeave remains the natural owner of governed transport/browser authority and provenance where the two products are composed.

Veilpick must not bind its stable core API directly to an unmerged OriginWeave feature branch. Integration adapters may follow independently versioned contracts once their required OriginWeave dependencies are available on an appropriate integrated generation.

## Consequences

### Positive

- Unattended crawling remains the product objective even when challenge states occur.
- Stealth and challenge resolution become complementary rather than mutually exclusive capabilities.
- CAPTCHA and bot-management handling are not deferred into an undefined manual process.
- LLM/VLM support can be added behind a stable domain boundary.
- Post-condition verification prevents an attempted interaction from being misreported as successful resolution.
- OriginWeave can supply governed transport/browser/evidence primitives without making Veilpick a duplicate browser control plane.

### Costs and risks

- Challenge recognition and resolution require realistic browser-level integration tests in addition to parser tests.
- Challenge implementations and provider behavior can change independently of Veilpick.
- Automated resolution can fail and therefore requires explicit typed failure and bounded retry/reconciliation behavior.
- Model-backed strategies introduce latency, cost, nondeterminism, and additional evidence requirements.
- Authorization and site-policy constraints remain independent of technical ability; a resolution strategy must not itself create authority to access a resource.

## Acceptance implications

Veilpick's product roadmap must treat challenge handling as in-scope. Initial implementation may deliver the capability incrementally, but a production-readiness claim for unattended challenge-capable acquisition requires evidence for detection, classification, automated resolution, and post-condition verification on the supported challenge classes.

The absence of a universal solver for every possible challenge does not invalidate the architecture. Unsupported or unsuccessfully resolved challenges must fail explicitly rather than falling back silently to manual success, fabricated extraction, or unverified continuation.
