# Veilpick

**An ontology-based, fully autonomous website scraping engine built in Rust that requires no human involvement.**

**Ontology. Autonomy. Stealth.** All three are first-class product dimensions.

> Product concept and accepted architectural direction, not a shipped-capability claim. This documentation baseline does not implement the Rust engine, browser-applied stealth, automated challenge resolution, or end-to-end acceptance evidence.

## Product contract

Given a collection goal and preconfigured operating authority, Veilpick is intended to discover relevant pages, construct or select a working ontology, plan acquisition, apply coherent stealth and session behavior, interpret page semantics, extract and reconcile entities, resolve supported challenges, recover from website changes, validate results, and deliver source-backed structured data without human intervention during execution.

Users must not have to hand-author a site-specific scraper, select DOM nodes, repair selectors, build a per-site ontology, solve a challenge, or approve each runtime action for a supported task to complete. A user-supplied ontology is an optional input, not a prerequisite for autonomy.

Ontology guides the complete semantic acquisition loop; stealth remains an explicit acquisition requirement. Neither ontology nor a solver replaces stealth, and successful stealth does not prove extraction correctness. Automated resolution remains mandatory for v1's declared supported challenge classes.

## Responsibility boundaries

Veilpick owns the Rust task loop, crawl frontier, task-local ontology use, semantic planning, acquisition/stealth orchestration, adaptive extraction, entity reconciliation, challenge orchestration, validation, and extraction provenance.

OriginWeave is the intended reusable owner of governed transport, browser, presentation/fingerprint application, policy, and runtime evidence. ConceptWeave supplies semantic candidate/domain contracts, contextual-orchestrator supplies replaceable reasoning, and context-graph-contracts supplies interoperable assertions. These integrations require actual eligible upstream implementations and consumer tests; they are not claimed as working by this document.

Task-local inferred semantics do not bypass ConceptWeave's review/publication lifecycle. Optional catalogs, document processors, ranking services, and enterprise identity systems stay behind adapters rather than becoming mandatory standalone dependencies. See ADR 0003 for the detailed ownership and availability matrix.

## Architecture decisions

- [ADR index](docs/adr/README.md)
- [ADR 0002: Ontology-based, fully autonomous Rust scraping engine](docs/adr/0002-ontology-based-autonomous-rust-engine.md) defines the product contract and v1 acceptance gates.
- [ADR 0001: Automated challenge resolution](docs/adr/0001-automated-challenge-resolution.md) defines the required challenge subsystem.
- [ADR 0003: First-class stealth and ecosystem composition](docs/adr/0003-stealth-and-ecosystem-composition.md) preserves stealth, assigns reuse boundaries, and distinguishes planned dependencies from integrated capabilities.

## License

Apache License 2.0. See [LICENSE](LICENSE).
