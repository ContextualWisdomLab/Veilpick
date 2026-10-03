# ADR 0005: Consume Wardnet-owned site reputation decisions

- Status: Proposed consumer boundary; pending canonical owner release and PR review
- Date: 2026-09-05
- Corrected ownership evidence: merged [Wardnet PR #171](https://github.com/ContextualWisdomLab/wardnet/pull/171)
- Canonical owner proposal: [Wardnet PR #173](https://github.com/ContextualWisdomLab/wardnet/pull/173) or its verified successor
- Governing Veilpick decisions: [ADR 0002](0002-ontology-based-autonomous-rust-engine.md) and [ADR 0003](0003-stealth-and-ecosystem-composition.md)
- Specification: [Access and reputation composition](../design/access-and-reputation-engines.md)
- Delivery plan: [Reputation consumer integration plan](../superpowers/plans/2026-09-05-site-reputation-engine.md)

## Context

Veilpick must distinguish an adverse destination-security decision, an unknown destination, source-content reliability, and operational access friction. A CAPTCHA, 403, 429, valid TLS session, or successful page load does not establish destination safety or factual truth.

Merged Wardnet PR #171 assigns destination maliciousness/reputation policy, security-evidence lifecycle, organizational admission policy, and SOC accountability to Wardnet while keeping outbound challenge handling outside Wardnet. Wardnet PR #173 is the current Proposed owner-side design. The previous Veilpick proposal for its own site-reputation core therefore created a competing writer.

## Decision

Wardnet is the canonical writer for destination-security reputation decisions and their evidence lifecycle. Veilpick owns only a consumer port and anti-corruption layer that translate an immutable released Wardnet decision envelope into acquisition planning inputs. Veilpick must not implement a parallel reputation evaluator, provider-ingestion pipeline, PostgreSQL evidence store, learned malicious-URL model, standalone reputation service, or cross-service SQL query.

Until Wardnet publishes an immutable compatible contract and release, Veilpick may use only a feature-disabled port or a bounded test double. It must not import Wardnet source, read its database, bind a temporary branch, or treat open PR #173 as runtime authority. A protected workflow that requires a reputation decision fails closed when the released authority, evidence health, tenant binding, or audit receipt is unavailable.

A future consumer envelope must bind at least:

- schema and decision/rule versions;
- authenticated tenant, workload, purpose, and exact typed destination subject;
- assessment state, policy action, reason code, evidence-health state, and coverage;
- observation/as-of time, expiry, revocation/snapshot generation, and source-lineage references;
- canonical owner release identity and integrity evidence.

Unknown is not safe. An allow decision is not transport authorization, proof that a socket was opened, or evidence that extraction succeeded. OriginWeave/EgressWeave retain their respective runtime and egress authority. Every redirect, coalesced authority, DNS re-resolution, or actual peer change requires the owning transport checks and a destination decision for the new exact subject.

Veilpick retains source-content validation in its own product domain: extraction provenance, claim-level corroboration, completeness, abstention, and result validation. Those signals cannot be promoted into Wardnet's destination-security decision or a universal site score. Operational access friction remains with ADR 0004's access/challenge boundary and cannot lower destination-security severity.

## Consumer acceptance

Before enabling the port, conformance fixtures must prove exact-subject isolation, cross-tenant rejection, unknown and adverse outcomes, required-source degradation, expiry, revocation, replay, malformed/unknown schema, TTL-only insufficient provenance, redirect/peer changes, and outage behavior. Provider confidence is never reinterpreted as calibrated maliciousness probability. Diagnostics expose reason/evidence references and counts without credentials, raw protected content, or tenant leakage.

The integration must be test-first, use a versioned released schema/client, retain an owner-release-to-consumer-version compatibility record, and obtain exact-head security/review evidence. Cache keys include every authorization and revision dimension; cache lifetime cannot exceed the owner envelope expiry, and revocation invalidates affected entries.

## Alternatives and consequences

Keeping an independent Veilpick reputation engine is rejected because it conflicts with the protected Wardnet ownership boundary and creates two security-policy writers. Copying Wardnet logic or reading its database is rejected because it bypasses release governance and couples lifecycle state. Treating Wardnet as an optional feed is rejected for protected decisions because the canonical owner, not Veilpick, decides whether evidence is sufficient.

This repair preserves the valid requirements from the earlier proposal as consumer conformance constraints while moving implementation, storage, model evaluation, and security-policy authority to Wardnet. No current Wardnet PR is claimed as released, and this PR still delivers no runtime integration.
