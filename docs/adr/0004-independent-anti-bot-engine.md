# ADR 0004: Independent Rust anti-bot access engine

- Status: Proposed; design-only, pending PR review
- Date: 2026-09-05
- Governing decisions: [ADR 0002](0002-ontology-based-autonomous-rust-engine.md) and [ADR 0001](0001-automated-challenge-resolution.md), preserved without weakening
- Preserves: [ADR 0003](0003-stealth-and-ecosystem-composition.md), including first-class stealth and ecosystem ownership
- Specification: [Engine design](../design/access-and-reputation-engines.md)
- Delivery plan: [Anti-bot implementation plan](../superpowers/plans/2026-09-05-anti-bot-engine.md)

## Context and terminology

In this acquisition product, **anti-bot engine** means handling a destination's bot-management mechanisms during an already-authorized autonomous acquisition. It does not mean an inbound bot detector or a WAF replacement. The technical package name `anti-bot-core` is paired with the explicit descriptor **outbound access and challenge engine** to prevent that ambiguity.

Wardnet's inspected `waf-ids-core` models threats, DNSBL entries and gateway enforcement; these are not browser sessions, access attempts or verified challenge resolutions. Reusing its scorer because it is written in Rust would couple unrelated decisions. OriginWeave already owns governed transport, browser and presentation-identity boundaries. Veilpick owns the ontology-guided collection goal and extraction acceptance. Inspected revisions and primary sources are recorded in [Research and evidence](../research/access-and-reputation-evidence.md).

## Product requirement

The customer outcome is verified goal completion without manual selectors, per-site repair or runtime human challenge solving. The engine must reduce unnecessary challenge escalation, recognize challenges, choose a functioning automatic resolution strategy, and verify that acquisition can resume. Zero human interventions is necessary but an unattended error is not success.

Functional CAPTCHA/bot-management resolution remains a mandatory v1 capability for a non-empty, versioned supported-class matrix. A trait, detector, mock resolver or human handoff alone fails that gate. Technical support is not a grant of access authority; authentication, legal consent and other missing permissions remain distinct non-success outcomes.

## Decision

### Ownership and packaging

Incubate an independently versioned `crates/anti-bot-core` and standalone `apps/anti-bot-engine` in Veilpick, with a consumer adapter rather than a dependency on Veilpick's planner internals. Repository co-location is an incubation choice, not merged domain ownership. Neither package may depend on `waf-ids-core`, `site-reputation-core`, or ontology/planner implementation types. The standalone entry point must execute the same state machine and accept injected runtime/provider ports; it must not require the whole scraper to run.

The core owns deterministic state transitions, access observations, strategy selection, attempt accounting and terminal classifications. Host adapters own I/O, clocks, durable leases, credentials, provider calls and evidence admission. OriginWeave remains the owner of network/browser/policy authority when composed. Only adapters may bind to verified integrated OriginWeave revisions. There is no mutable feature-branch dependency or copied HTTP/browser stack.

LLM/VLM requests use an adapter to contextual-orchestrator's owning gateway contract when an eligible integration exists; the engine does not copy provider routing. This does not assert current provider or VLM availability.

A later repository extraction must preserve package identity, contracts, conformance tests and independent releases. This ADR creates no repository, Rust crate or executable.

### Required behavior

| Requirement | Required result |
| --- | --- |
| AB-01 Coherent presentation | Select a supported presentation profile, retain its runtime application evidence and preserve session continuity. Profile metadata alone is not applied stealth. |
| AB-02 Pacing | Honor policy, origin/account quotas, `Retry-After` and a task-wide budget. New sessions, identities or egress addresses cannot reset a cooldown. |
| AB-03 Recognition | Separate ordinary content, CAPTCHA, JavaScript/interstitial, authentication, consent, rate limit, denial, outage and ambiguous content. A status code or vendor-looking text alone is insufficient attribution. |
| AB-04 Resolution | Execute functioning deterministic/browser/LLM/VLM strategies from an explicitly admitted, versioned capability registry; no hidden human solver. |
| AB-05 Verification | Re-observe the exact session, origin and document generation after an attempt; validate challenge disappearance and the requested target-content predicate. A solver answer or protocol ACK is not completion. |
| AB-06 Recovery | Reconcile ambiguous side effects before retry. Stop at explicit attempt, time, byte and monetary limits; never wait for a person in the autonomous path. |
| AB-07 Audit | Retain redacted observations, strategy/runtime versions, budget consumption and post-condition references. Never log cookies, credentials, answer tokens or raw model prompts containing protected data. |

RFC 6585 defines rate limiting and RFC 9110 defines `Retry-After`; our cross-session quota ledger is a design choice that preserves those signals, not a claim that either RFC specifies this engine [1, 2]. Robots rules and access authorization remain separate [3]. Reference numbers resolve in the research record.

### State machine

```text
Admitted -> Planned -> RuntimeAuthorized -> Observed
Observed -> ContentCandidate -> ExtractionValidation
Observed -> ChallengeClassified -> StrategyAdmitted -> Attempted -> Reobserved
Reobserved -> VerifiedResolved -> ContentCandidate
Reobserved -> ReconcileRequired | BoundedReplan | TerminalFailure
Observed -> Cooldown | AuthorityUnavailable | Unsupported | TerminalFailure
```

`VerifiedResolved` requires newly admitted runtime evidence bound to the attempt, not a caller-supplied success boolean. The integration host authenticates the observation producer; Rust types or serialized evidence IDs alone cannot prove that a browser event occurred. Extraction validation remains Veilpick-owned and can still reject content after a resolved challenge.

A durable reservation must be recorded before dispatch. Crash recovery retains possibly consumed attempts/cost and reconciles uncertain effects. Retrying a POST-like challenge action merely because a timeout occurred is forbidden. Session identity changes require runtime authorization and cannot expand the original origin/action grant.

### Strategy policy and support matrix

Keep deterministic and semantic strategies behind the same admission and evidence contract. The initial implementation must demonstrate: a real browser JavaScript/interstitial path; at least one declared CAPTCHA family with a functioning unattended resolver; a server-enforced rate-limit/cooldown case; and distinct denied/unsupported/authentication/consent cases. The exact provider, browser and challenge family versions belong in the locked benchmark manifest before execution. Declaring all CAPTCHA unsupported to pass v1 contradicts ADR 0001.

Models propose bounded actions or answers. The host validates origin, action, budget and output shape before dispatch. Provider credentials are opaque registry handles. Provider failure cannot silently call an unconfigured provider or human service. A challenge token is origin/session/attempt-bound, short-lived and never reusable as general authorization.

## Alternatives

Embedding a solver in Wardnet is rejected because ingress detection and outbound resolution have different subjects and lifecycle authority. A free-form LLM-controlled browser loop is rejected because model output cannot own budgets, permission or success. Removing stealth and retaining only a resolver is rejected: prevention and resolution are complementary. Copying OriginWeave's transport to avoid pending dependencies is rejected; integrations remain blocked until the owning capability is available and verified.

## Acceptance and consequences

The declared supported matrix must pass real browser/provider end-to-end cases through resumed, validated extraction, with zero human interventions per claimed success. Keep the matrix fixed before the run and report all attempted tasks, supported failures, unsupported cases, false continuations, interventions, latency and cost. A finite test run with no observed false continuations is not a universal zero-risk claim.

Stealth acceptance is separate from solver acceptance: compare an applied approved profile with a matched baseline under the same controlled matrix, recording cross-surface consistency, challenge incidence, validated extraction, latency and cost. A successful solver or blocked fingerprint API does not prove stealth effectiveness.

Costs include provider variability, durable distributed quota state, browser fixtures and independent post-condition checking. Initially conservative concurrency can reduce throughput; optimization must not weaken quota sharing or evidence admission. The [shared design](../design/access-and-reputation-engines.md) specifies failure handling and budgets. This design PR delivers no solver, stealth effectiveness, Rust performance result or production readiness claim.
