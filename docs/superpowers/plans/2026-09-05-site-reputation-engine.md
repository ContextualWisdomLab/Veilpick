# Site Reputation Engine Implementation Plan

> **For agentic workers:** Use the installed subagent-driven-development or executing-plans workflow when implementing these slices. The steps below are not executed by this design PR.

**Goal:** Independently return reproducible, scoped reputation assessments with provenance, coverage and lifecycle-aware evidence.

**Architecture:** A pure Rust evaluator reads immutable admitted snapshots. Separate provider/storage adapters prepare those snapshots; caller policy owns any action decision.

**Tech Stack:** Rust 2024, versioned typed contracts, PostgreSQL persistence adapter, deterministic replay and optional shadow-mode learned URL signals. No runtime dependency is added here.

**Spec:** [Shared design](../../design/access-and-reputation-engines.md) and [ADR 0005](../../adr/0005-evidence-based-site-reputation-engine.md).

## Global constraints

No `waf-ids-core`, `anti-bot-core` or planner-internal dependency. No network I/O in assessment. Security, source reliability and operational access remain separate; unknown is not safe. Initial per-query evidence ceiling is 256. No global unvalidated score or inherited IP-to-publisher trust. Wardnet is optional.

## Slice R1: Subject and evidence admission

Create `crates/site-reputation-core/{Cargo.toml,src/lib.rs,src/subject.rs,src/evidence.rs}` and that crate's `tests/admission.rs`. Adopt the reviewed semantic-frontier/root-workspace foundation independently of A1; do not create a competing foundation or require the anti-bot package to use this engine. Expose `ReputationEngine::assess` with the specification's query/snapshot/clock inputs and assessment/error outputs.

- [ ] First test different schemes/ports as different origins, IP versus host subjects, query-sensitive URL scope, malformed lifecycle, unsupported schema, missing provenance and forged tenant scope. None may widen subject or authority.
- [ ] Run `cargo test -p site-reputation-core --test admission`; record actual failures, implement bounded validation and repeat. Keep canonical normalization inside a reviewed adapter; normalization is not destination authorization.
- [ ] Commit the independently testable contract and matching documentation before integrating any feed.

## Slice R2: Lifecycle, coverage and aggregation

Create `src/{snapshot.rs,lifecycle.rs,assessment.rs}` and `tests/{lifecycle.rs,dimensions.rs}` inside the core crate. Inputs are admitted evidence versions; outputs are dimension states, included/excluded evidence and coverage leases.

- [ ] First encode these exact counterexamples: an empty snapshot is `Unknown`; complete requested provider coverage without a match is `NoKnownThreat`, never `Safe`; a still-valid threat survives another provider's outage; ten same-lineage reports do not become ten independent confirmations; CAPTCHA-only evidence changes access state but not reliability/security.
- [ ] Add permanent revocation followed by older-record replay, expiry, clock regression, contradictory evidence and 257-record overflow cases. Revocation cannot be undone; overflow cannot produce a clearance result.
- [ ] Run `cargo test -p site-reputation-core --test lifecycle --test dimensions`; record failures, implement deterministic reduction and rerun before committing.

## Slice R3: Storage and optional providers

Create `adapters/reputation-postgres/src/lib.rs`, `adapters/reputation-postgres/migrations/0001_evidence.sql`, `adapters/reputation-providers/src/{lib.rs,stix.rs,wardnet.rs}` and `tests/provider_storage_contract.rs`. Provider adapters produce admitted observations plus completeness metadata, not policy decisions.

- [ ] Test authenticated provider identity, source-version uniqueness, atomic snapshot/coverage publication, persistent revocation tombstones, duplicate/conflicting payloads, retention enforcement and cross-tenant reads with real PostgreSQL.
- [ ] Test a Wardnet row containing TTL but no authoritative observation anchor: it must return `InsufficientProvenance`, not a fresh threat. A SQL-injection signature or ingress-client event is not a destination-site finding. Fetching original evidence must use a separately governed adapter.
- [ ] Run provider/storage tests, implement minimal translations and transaction boundaries, and test Wardnet unavailable. A missing optional feed changes coverage, not service liveness. Commit with producer fixture versions and loss/omission documentation.

## Slice R4: Standalone, evaluation and composition

Create `apps/site-reputation-engine/src/main.rs`, `adapters/veilpick-reputation/src/lib.rs`, `tests/standalone_reputation.rs`, `tests/fixtures/reputation-manifest.json` and `docs/acceptance/reputation-results.md`.

- [ ] Test assessment without a crawler, source outage, replay reproducibility, cache key isolation, revocation invalidation and no credentials/body leakage. Test caller policy separately: a result must not create network permission.
- [ ] Lock domain/time-separated labeled fixtures and source lineages. Report feed-only and rule-only precision/recall, false positives, abstention/coverage, drift and resource consumption; a learned candidate requires a separate shadow evaluation and calibration report before activation.
- [ ] Run `cargo fmt --all --check`, `cargo test --locked --workspace`, `cargo clippy --locked --workspace --all-targets -- -D warnings` and service/PostgreSQL tests. Acquire exact-head hosted security/review evidence; do not infer it from local replay.
- [ ] Publish independent package/schema version and compatibility fixtures only through normal release governance. This design plan does not authorize merge or release.
