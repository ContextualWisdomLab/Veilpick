# Anti-bot Engine Implementation Plan

> **For agentic workers:** Use the installed subagent-driven-development or executing-plans workflow when implementing these slices. The steps below are not executed by this design PR.

**Goal:** Independently run authorized acquisition through functioning automatic challenge resolution and verified continuation.

**Architecture:** A pure Rust state machine emits bounded effects. Trusted adapters enforce runtime authority, durable quota/effect ownership and provider evidence; Veilpick consumes the same standalone engine.

**Tech Stack:** Rust 2024, versioned typed contracts, OriginWeave integration adapters, PostgreSQL durability adapter and deterministic replay plus real-browser tests. No new runtime dependency is added here.

**Spec:** [Shared design](../../design/access-and-reputation-engines.md) and [ADR 0004](../../adr/0004-independent-anti-bot-engine.md).

## Global constraints

No `waf-ids-core`, `site-reputation-core` or planner-internal dependency. No runtime raw environment credentials. At most three resolution attempts per task and 90 seconds per challenge in the initial test profile, bounded further by task budgets. Functional supported-class CAPTCHA resolution and zero human intervention remain v1 requirements. A reported failure is not success.

## Slice A1: State and quota boundary

Create `crates/anti-bot-core/{Cargo.toml,src/lib.rs,src/state.rs,src/budget.rs}` and `tests/state_machine.rs` within that crate. Adopt the reviewed semantic-frontier/root-workspace foundation when integrated; do not fork the active foundation work into a competing workspace PR. Check the integrated runtime's compiler requirement and preserve the owning workspace/toolchain pin; do not copy Wardnet's workspace. `AntiBotEngine::advance` consumes the event/context contract in the specification and produces state plus proposed effects, not I/O.

- [ ] Encode the following replay cases first in `tests/state_machine.rs`: 200 challenge produces `ChallengeClassified`, not completion; attempt four produces `BudgetExhausted` without a dispatch; a 120-second server cooldown with 30 seconds remaining produces `Deferred`; a new session retains the same quota-group cooldown.
- [ ] Run `cargo test -p anti-bot-core --test state_machine`; record the actual failing assertions before adding the minimal transitions and ledger arithmetic.
- [ ] Implement checked counters, monotonic deadline comparisons and explicit unknown/denied states. Repeat the tests, then commit only this slice and its release record.

## Slice A2: Durable effects and governed runtime

Create `adapters/anti-bot-runtime/src/lib.rs`, `adapters/anti-bot-state-postgres/src/lib.rs`, `adapters/anti-bot-state-postgres/migrations/0001_effect_ledger.sql` and `tests/access_runtime_contract.rs`. The host consumes proposed effects and returns authenticated, attempt-bound observations. Store unique `(tenant, task, effect_id)` reservations, quota-group cooldown and lease generation before dispatch.

- [ ] First test restart after reservation, crash after ambiguous remote write, lease loss, cancellation and cross-origin/profile change. The expected outcomes are retained budget, reconciliation, no new dispatch and fresh authority checks respectively.
- [ ] Run the focused adapter tests against real PostgreSQL and a controlled runtime peer; record failures, implement the minimum durable transaction/fencing logic and rerun. An in-memory mock cannot establish restart correctness.
- [ ] Verify that no adapter reimplements DNS/TLS/HTTP authority or binds a mutable OriginWeave branch. Commit with exact dependency and runtime test evidence.

## Slice A3: Functional resolver and verification

Create `adapters/anti-bot-resolvers/src/{lib.rs,registry.rs}`, `crates/anti-bot-core/src/postcondition.rs`, `tests/fixtures/challenge-matrix.json` and `tests/challenge_e2e.rs`. The registry consumes admitted challenge observations and provider capabilities; the host returns fresh runtime observations, which are evaluated separately from provider answers.

- [ ] Lock at least one real supported CAPTCHA family and one JavaScript/interstitial family with exact provider/browser versions in the matrix before evaluating. Record supplied access authority and credentials separately from site-specific automation work.
- [ ] Add failing cases for provider-success with challenge still present, wrong session/origin, expired/replayed answer, truncated evidence, model prompt injection and unavailable provider. None may reach verified continuation.
- [ ] Implement functioning admitted strategies and run actual browser-to-provider-to-browser verification through resumed extraction. Record every outcome, cost, latency and human intervention. Mock-only success cannot satisfy this slice.
- [ ] Commit only after evidence differentiates verified resolutions, unsupported cases, failed tasks and infrastructure errors. Do not remove failing CAPTCHA classes from the locked matrix.

## Slice A4: Standalone and consumer acceptance

Create `apps/anti-bot-engine/src/main.rs`, `adapters/veilpick-access/src/lib.rs`, `tests/standalone_access.rs` and `docs/acceptance/anti-bot-results.md`. Both entry points consume the same versioned task/event contract; standalone execution supplies its own admitted runtime/provider adapters without the ontology planner.

- [ ] Test standalone challenge completion, provider outage, shared-origin concurrency, secret redaction and no-human execution. Include a matched applied-profile versus baseline stealth evaluation. Then test Veilpick goal completion and extraction validation separately.
- [ ] Run `cargo fmt --all --check`, `cargo test --locked --workspace`, `cargo clippy --locked --workspace --all-targets -- -D warnings`, and the real-browser suite under the declared profile. Apply the owning repositories' required coverage/security gates without weakening them.
- [ ] Publish versioned support, trial denominators, failures, latency/cost and exact head evidence; acquire hosted checks and review. Package release and default-branch merge are separate governed operations, not authorized by this plan.
