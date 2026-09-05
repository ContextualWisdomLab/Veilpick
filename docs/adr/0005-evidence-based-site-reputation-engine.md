# ADR 0005: Independent evidence-based site reputation engine

- Status: Proposed; design-only, pending PR review
- Date: 2026-09-05
- Governing decision: [ADR 0002](0002-ontology-based-autonomous-rust-engine.md)
- Preserves: [ADR 0003](0003-stealth-and-ecosystem-composition.md), including ecosystem ownership
- Specification: [Engine design](../design/access-and-reputation-engines.md)
- Delivery plan: [Reputation implementation plan](../superpowers/plans/2026-09-05-site-reputation-engine.md)

## Context and product requirement

An autonomous collector needs to distinguish an unsafe destination, a poorly evidenced source, a currently inaccessible site and a merely unknown site. These are different questions. A CAPTCHA, 403, 429 or WAF deployment is not evidence that the site's content is false or its operator malicious. Conversely, successful access and a valid TLS connection do not establish source credibility.

Wardnet's gateway score, DNSBL entry and threat severity are scoped security observations, not a general site reputation model. At the inspected revision, `ThreatIndicator` stores value, type, severity, source and TTL; it does not itself carry the complete subject, observation-time, confidence and revocation lineage required here. Do not synthesize those missing facts. The exact inspected code and research are in [Research and evidence](../research/access-and-reputation-evidence.md).

The customer outcome is a reproducible, explainable assessment that the caller can use to prioritize acquisition, require corroboration, isolate risky content or decline an action under its own policy. The service must also run independently of Veilpick, Wardnet and any active crawler.

## Decision

### Independent ownership

Incubate `crates/site-reputation-core` and `apps/site-reputation-engine` as independently versioned packages in Veilpick. The core consumes immutable admitted evidence snapshots and produces assessments; it performs no HTTP, DNS, browser, model or database I/O. It must not depend on `anti-bot-core`, `waf-ids-core` or Veilpick planner/ontology implementation types. Feed ingestion, persistent storage and caller policy are separate adapters. Wardnet is an optional observation provider, not an availability prerequisite or decision authority.

Candidate ranking remains in the caller through the existing ranking owner boundary; this engine does not copy RankWeave algorithms. Any context-graph interoperability preserves truth/origin categories without turning them into a confidence ladder.

No global shared `common` crate will absorb the two engines' domain models. Each engine owns its external schema, and integration adapters explicitly translate between them. Shared technical utilities require a separate demonstrated reuse case, not name similarity.

### Subject and assessment model

| Dimension | Assessment meaning | Prohibited inference |
| --- | --- | --- |
| Security | Applicable threat evidence, coverage and conflicting findings for an exact URL/origin/host/IP subject | A DNSBL-listed shared IP automatically makes every hosted publisher malicious |
| Source reliability | Attributed provenance, identity evidence, corrections and independent claim-level corroboration within a declared topic/time scope | Popularity, TLS, an LLM opinion or shape validation proves factual truth |
| Operational access | Observed availability, challenge classes and cooldown state for a scoped measurement window | Difficult automation implies bad security or low editorial quality |
| Coverage/freshness | Which providers and observation periods support the result; missing, stale or unavailable inputs | No feed match means safe, or stale data becomes current because it was fetched again |

Return these dimensions separately, with reason codes and evidence references. V1 does not emit a universal 0-100 site score. For security, use `ThreatPresent`, `NoKnownThreat`, `Unknown` and `ConflictingEvidence`; `NoKnownThreat` means no match within explicitly complete requested provider coverage, not proof of safety. Reliability starts as `Unknown`, `Supported`, `Disputed` or `Mixed` against specific claims/topic windows. Access is descriptive and cannot cancel security evidence.

The subject discriminant is mandatory: `Url`, `Origin`, `Host`, `IpAddress`, or `PublisherIdentity`. A canonical HTTP(S) origin includes scheme, normalized host and effective port. URL-scoped evidence retains a protected resource identity; stripping its query must not broaden it to all pages. Domain/IP/publisher association is a separately evidenced relation, never string-based transitive trust. Public-suffix aggregation may be used for reporting, not authorization or automatic reputation inheritance.

### Evidence admission, aggregation and expiry

Every admitted item binds schema version, tenant/visibility scope, subject and scope, provider/source record identity, source version, observation/collection times, validity interval, provenance lineage, source confidence if supplied, sharing restrictions and lifecycle state. Provider trust is configured independently of the evaluated site; unauthenticated site self-claims remain untrusted evidence.

STIX 2.1 confidence is the producer's confidence in its data, with absence meaning unspecified. It is not a calibrated maliciousness probability. Its revocation and indicator-validity semantics inform the adapter [4]. Preserve source version and permanent revocation tombstones; an older record or replay cannot resurrect a revoked indicator. An omitted STIX `valid_until` is permitted by STIX, but this consumer applies a separately recorded maximum freshness policy rather than claiming the source supplied an expiry.

Aggregation is deterministic: validate subject and producer; enforce visibility; resolve source versions and revocations; apply freshness; group duplicated/derived records by lineage; evaluate each dimension; emit the evidence set, excluded-item reasons, coverage and rule version. Do not sum repeated syndicated reports as independent confirmations. Conflicting credible evidence remains visible and cannot be averaged into a reassuring scalar. A favorable source-reliability or accessibility record cannot erase an applicable threat.

For each result, expiry is no later than the earliest relevant evidence expiry, provider-coverage expiry and consumer maximum age. A provider outage invalidates complete current coverage when that provider's coverage lease expires; any still-valid positive threat remains visible. A revocation transaction advances the snapshot generation and invalidates affected cached results. Unknown or expired observations never become benign observations.

### Source reliability is not a truth oracle

Veilpick may contribute extraction provenance and claim-validation outcomes, but they remain observations with collector/model/ontology lineage. Independent corroboration must exclude copies and derived articles from the same source lineage. Positive evidence for one topic or historical period does not transfer to unrelated topics or future claims. PROV-O supplies reference vocabulary for derivation and attribution, not proof that the source statement is true [5].

No political, demographic, geographic or commercial popularity proxy is used as an automatic credibility score. Corrections and disputes are versioned evidence updates; normal source refresh and model-drift handling must not require a per-task human queue. Optional human-authored source evidence is labeled as such and does not replace the autonomous runtime contract.

### Models and policy

Start with explicit evidence rules. URL classification may later add model-generated threat hypotheses, never replace observations with invented certainty. Malicious URL research motivates testing learned signals alongside feeds rather than assuming blacklist completeness [7, 8]. Probability claims require independent labels, domain/time-separated evaluation and calibration; neural network confidence is not automatically calibrated [6]. These sources do not establish Veilpick's accuracy.

The caller chooses `Proceed`, `RequireCorroboration`, `IsolatedProbe`, `Defer` or `Deny` through its policy adapter. Those are not network capability grants. A reputation result cannot authorize a socket, ignore robots, supply consent, execute page scripts or release credentials. Unknown sites can receive only a separately authorized, secret-free bounded probe; otherwise the task terminates with explicit non-success.

## Alternatives and consequences

Reusing Wardnet's score is rejected because it loses subject and decision semantics. One scalar blending security, reliability and friction is rejected because good accessibility could mask danger. A permanent blacklist-only engine is rejected because absence of known evidence is not safety. Mandatory synchronous calls to Wardnet are rejected because an optional feed should not control service availability.

This decision adds evidence storage, lifecycle/tombstone handling, provenance deduplication and coverage accounting. It deliberately accepts more `Unknown` results instead of inventing assurance. Release acceptance requires deterministic replay, revocation/outage tests, tenant isolation, independent deployment, and dimension-separation counterexamples. The current PR contains design documents only, not a feed connector, model, scoring implementation or production deployment.
