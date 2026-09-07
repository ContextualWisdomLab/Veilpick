# Veilpick

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/ContextualWisdomLab/Veilpick)

**Ontology-guided, autonomous web acquisition—designed to turn an authorized collection goal into source-backed structured data.**

Veilpick is a planned Rust engine for resilient web acquisition. It is intended to discover relevant pages, understand their semantics, adapt extraction as sites change, reconcile entities, and validate what it collected without requiring a person to author and maintain a scraper for every site.

**Ontology. Autonomy. Stealth.** All three are first-class product dimensions.

> **Status: pre-release implementation foundation.** This branch adds one bounded Rust crawl-frontier planning primitive. It does not yet ship the autonomous engine, browser integration, stealth runtime, challenge resolver, package, hosted service, or end-to-end acceptance evidence.

## Why Veilpick

Conventional scrapers bind a collection workflow to selectors and page structure. When a site changes, the operator has to repair the implementation before collection can continue. Veilpick’s product direction is to keep the collection goal and evidence contract stable while the engine adapts discovery and extraction behind that boundary.

A completed supported task is intended to provide:

- structured records tied to their source evidence;
- explicit validation results instead of silent extraction success;
- task-local semantic adaptation without making unpublished ontology truth authoritative;
- recovery from supported page and presentation changes;
- an auditable boundary between collection intent, runtime actions, and delivered data.

Veilpick is for authorized acquisition. Operating authority, target policy, data rights, and required human approvals must be established before a task starts; autonomy does not expand permission.

## Intended experience

Given a collection goal and preconfigured operating authority, Veilpick is intended to:

1. discover relevant pages and maintain a crawl frontier;
2. construct or select task-local semantic guidance;
3. plan governed acquisition and session behavior;
4. interpret page semantics and extract candidate entities;
5. reconcile entities while retaining source provenance;
6. handle declared supported challenge classes;
7. verify post-conditions and deliver structured results.

For a declared supported task, users should not need to hand-author a site-specific scraper, select DOM nodes, repair selectors, build a per-site ontology, solve each supported challenge, or approve every runtime action. A user-supplied ontology is an optional input, not a prerequisite for autonomy.

Ontology guides the semantic acquisition loop; stealth remains a separate acquisition requirement. Neither ontology nor a solver replaces stealth, and successful acquisition does not prove extraction correctness.

## Get started with the current slice

This branch is source-only and has no published package or release. With the pinned Rust 1.98.1 toolchain installed, the bounded planning example and repository verification commands are:

```bash
cargo run --locked --example frontier
cargo test --locked --all-targets
cargo fmt --all --check
cargo clippy --locked --all-targets -- -D warnings
RUSTDOCFLAGS='-D warnings' cargo doc --locked --no-deps
```

The example is synthetic local frontier planning, not a website scrape. These commands describe the source contract; passing them does not prove the complete Veilpick product, a release, browser integration, stealth effectiveness, or permission to access a target.

## Current Rust slice

The standard-library-only frontier implementation is in [`src/frontier.rs`](src/frontier.rs), exported through `Candidate`, `CrawlFrontier`, `FrontierLimits`, `FrontierError`, and `FrontierStep`.

It:

- orders candidates by distinct declared concept hints and retains FIFO order for ties;
- preserves lifetime duplicate identities;
- enforces finite admission and dispatch budgets;
- treats targets as opaque caller-supplied references rather than discovering URLs or granting access authority;
- bounds identifiers to 256 UTF-8 bytes, raw concept input to 128 entries, and each frontier limit to `1..=4096`;
- rejects invalid input without mutating the frontier;
- keeps target and concept values out of debug output;
- represents `Drained` and `BudgetExhausted` as planning outcomes, not extraction receipts.

The repository specifies 25 behavioral tests, including 81 small priority combinations and the 4,096-target boundary. Compiler, test, formatter, lint, and documentation execution must be judged from the exact branch head; source inspection is not a substitute. See the [verification record](docs/verification/semantic-frontier.md).

## Product boundary

Veilpick owns the planned Rust task loop, crawl frontier, task-local ontology use, semantic planning, acquisition and stealth orchestration, adaptive extraction, entity reconciliation, challenge orchestration, validation, and extraction provenance.

It does not own:

- the caller’s legal authority, target permission, or data-use decision;
- publication of organization-wide ontology truth;
- general identity, secret, browser-policy, or model-provider control planes;
- a guarantee that an upstream service or proposed integration is available;
- correctness claims unsupported by observed post-condition evidence.

## Integration context

Veilpick is designed to remain useful as a standalone product while consuming released, versioned contracts when an eligible integration exists.

- [OriginWeave](https://github.com/ContextualWisdomLab/OriginWeave) is the intended reusable owner of governed transport/browser policy, presentation/fingerprint application, and runtime evidence. Veilpick retains task-level acquisition strategy, session-use, pacing, and recovery policy.
- [ConceptWeave](https://github.com/ContextualWisdomLab/ConceptWeave) may supply reviewed semantic candidate and domain contracts; task-local inference cannot bypass its publication lifecycle.
- [contextual-orchestrator](https://github.com/ContextualWisdomLab/contextual-orchestrator) may supply replaceable reasoning behind a bounded contract.
- [context-graph-contracts](https://github.com/ContextualWisdomLab/context-graph-contracts) may supply interoperable assertion and provenance contracts.

These are integration directions, not current working-capability claims. Optional catalogs, document processors, ranking services, and enterprise identity systems remain behind adapters rather than becoming mandatory dependencies.

## Quality and release gates

A Veilpick release must bind claims to exact-version evidence. At minimum, the shipped path needs:

- reproducible Rust build and package provenance;
- realistic authorized-target fixtures and post-condition checks;
- source-to-record provenance and failure/abstention behavior;
- security, dependency, license, and SBOM evidence;
- bounded resource, timeout, cancellation, and recovery behavior;
- integration contract and compatibility tests for every enabled external service;
- explicit supported and unsupported challenge classes.

The crawl-frontier slice is one planning component, not release evidence for the whole product. ADR decision status records product-owner acceptance; repository integration remains Proposed until the documentation reaches the protected branch through normal governance. No benchmark, compatibility, deployment, or release claim should be inferred from this README.

## Documentation

- [Bounded documentation index](docs/index.md)
- [ADR index](docs/adr/README.md)
- [ADR 0001: Automated challenge resolution](docs/adr/0001-automated-challenge-resolution.md)
- [ADR 0002: Ontology-based, fully autonomous Rust scraping engine](docs/adr/0002-ontology-based-autonomous-rust-engine.md)
- [ADR 0003: First-class stealth and ecosystem composition](docs/adr/0003-stealth-and-ecosystem-composition.md)
- [Semantic frontier implementation plan](docs/superpowers/plans/2026-09-05-semantic-frontier.md)
- [Semantic frontier verification record](docs/verification/semantic-frontier.md)
- [Change history](CHANGELOG.md)

## Contributing and support

Use [GitHub Issues](https://github.com/ContextualWisdomLab/Veilpick/issues) for reproducible product, documentation, and integration defects. Contributions should preserve the product boundary, keep unsupported capabilities explicit, and add tests and evidence for every new claim. Report security concerns through the [ContextualWisdomLab security policy](https://github.com/ContextualWisdomLab/.github/blob/main/SECURITY.md), not a public issue.

## License

Veilpick’s repository content is available under the [Apache License 2.0](LICENSE). Dependencies, generated artifacts, models, and external services retain their own terms and are not relicensed by this repository’s Apache-2.0 grant.
