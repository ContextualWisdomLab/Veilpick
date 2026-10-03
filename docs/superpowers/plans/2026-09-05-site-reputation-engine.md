# Wardnet Reputation Consumer Integration Plan

> **For agentic workers:** Execute only after an immutable Wardnet contract/release exists. This design PR does not authorize owner implementation or a mutable cross-repository dependency.

**Goal:** Consume Wardnet-owned destination-security decisions without duplicating reputation policy, evidence lifecycle, storage, or transport authority.

**Architecture:** A Veilpick anti-corruption layer implements a narrow `ReputationPort`. Production binds only to a released Wardnet client/schema. Until then, the port is feature-disabled or supplied by a deterministic test double.

**Spec:** [Composition design](../../design/access-and-reputation-engines.md) and [ADR 0005](../../adr/0005-evidence-based-site-reputation-engine.md).

## Global constraints

No local reputation evaluator, provider ingestion, evidence database, learned URL model, standalone reputation service, Wardnet source import, shared database, cross-service SQL, or temporary-branch binding. An allow result does not grant network authority. Required reputation unavailability fails protected work closed.

## Slice C1: Released contract admission

- [ ] Wait for the Wardnet owner lane to merge and publish an immutable compatible schema/client release; record its version, digest, license, SBOM/provenance, and protected-branch evidence.
- [ ] Write failing consumer conformance tests first for unknown major schema, forged tenant/workload/purpose, wrong typed subject, expired/revoked generation, impossible action/reason combinations, and TTL-only insufficient provenance.
- [ ] Add only the minimum `ReputationPort` DTO/validation boundary needed to pass those fixtures. Do not reproduce owner reduction rules.

## Slice C2: Acquisition ACL

- [ ] Write failing tests for adverse deny, authorized unknown, unhealthy required-source state, owner outage, redirect/new peer, cancellation, and stale cache replay.
- [ ] Map the admitted envelope to Veilpick planning outcomes while preserving exact reason and evidence references. Recheck the owner decision and separate OriginWeave/EgressWeave authority before every side effect.
- [ ] Keep source-content reliability, extraction completeness, and challenge friction as separate Veilpick concepts; none may weaken a destination-security denial.

## Slice C3: Cache, audit, and operability

- [ ] Test cache keys across tenant, workload, purpose, typed subject, owner release, rule version, snapshot generation, and expiry. Revocation must invalidate affected entries.
- [ ] Test secret/body/query redaction, retention/deletion, owner latency/outage, rollback, and incident audit. Never store owner evidence through direct database access.
- [ ] Measure the released adapter under realistic concurrent workloads. If it serves a product request path, demonstrate the declared page/request p95 target without omitting failures or shrinking samples.

## Slice C4: Release and consumer bump

- [ ] Run the complete Veilpick test, lint, documentation, security, SBOM/provenance, contract, and end-to-end lanes on one unchanged head.
- [ ] Publish the Veilpick consumer version through normal governance, then record the exact Wardnet release compatibility and rollback path.
- [ ] Enable production composition only after owner release and consumer bump evidence are immutable. Open owner work, source URLs, branch archives, and copied fixtures are not substitutes.
