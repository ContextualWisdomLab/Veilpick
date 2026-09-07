# Veilpick

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/ContextualWisdomLab/Veilpick)

**Ontology-guided, autonomous web acquisition—designed to turn an authorized collection goal into source-backed structured data.**

Veilpick is a planned Rust engine for resilient web acquisition. It is intended to discover relevant pages, understand their semantics, adapt extraction as sites change, reconcile entities, and validate what it collected without requiring a person to author and maintain a scraper for every site.

**Ontology. Autonomy. Stealth.** All three are first-class product dimensions.

> **Status: architecture and product contract only.** This repository does not yet ship an executable engine, browser integration, challenge resolver, package, hosted service, or release. The current documentation defines the acceptance boundary for implementation work.

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

## Get started

There is no install or quickstart command yet because no executable Veilpick artifact has been released. To evaluate or contribute to the direction:

1. start with the bounded [documentation index](docs/index.md);
2. review the product and v1 acceptance contract in [ADR 0002](docs/adr/0002-ontology-based-autonomous-rust-engine.md);
3. inspect the supported-challenge requirements in [ADR 0001](docs/adr/0001-automated-challenge-resolution.md);
4. use [ADR 0003](docs/adr/0003-stealth-and-ecosystem-composition.md) to distinguish Veilpick responsibilities from optional ecosystem integrations.

An install command belongs here only after a reproducible artifact and exact-version verification path exist.

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

ADR decision status records product-owner acceptance; repository integration remains Proposed until the documentation reaches the protected branch through normal governance. No benchmark, compatibility, deployment, or release claim should be inferred from this README.

## Documentation

- [Bounded documentation index](docs/index.md)
- [ADR index](docs/adr/README.md)
- [Product and technical gap baseline](docs/product-technical-gap-baseline.md)
- [ADR 0001: Automated challenge resolution](docs/adr/0001-automated-challenge-resolution.md)
- [ADR 0002: Ontology-based, fully autonomous Rust scraping engine](docs/adr/0002-ontology-based-autonomous-rust-engine.md)
- [ADR 0003: First-class stealth and ecosystem composition](docs/adr/0003-stealth-and-ecosystem-composition.md)

## Contributing and support

Use [GitHub Issues](https://github.com/ContextualWisdomLab/Veilpick/issues) for reproducible product, documentation, and integration defects. Contributions should preserve the product boundary, keep unsupported capabilities explicit, and add evidence for every new claim. Report security concerns through the [ContextualWisdomLab security policy](https://github.com/ContextualWisdomLab/.github/blob/main/SECURITY.md), not a public issue.

## License

Veilpick’s repository content is available under the [Apache License 2.0](LICENSE). Future dependencies, generated artifacts, models, and external services retain their own terms and are not relicensed by this repository’s Apache-2.0 grant.
