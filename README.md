# Veilpick

**An ontology-based, fully autonomous website scraping engine built in Rust that requires no human involvement.**

> Product concept and accepted architectural direction, not a shipped-capability claim. This repository currently records design decisions; the Rust engine and its end-to-end acceptance evidence are not implemented by this documentation change.

## Product contract

Given a collection goal and preconfigured operating authority, Veilpick is intended to discover relevant pages, construct or select a working ontology, plan acquisition, interpret page semantics, extract and reconcile entities, resolve supported challenges, recover from website changes, validate results, and deliver source-backed structured data without human intervention during execution.

Users must not have to hand-author a site-specific scraper, select DOM nodes, repair selectors, build a per-site ontology, solve a challenge, or approve each runtime action for a supported task to complete. A user-supplied ontology is an optional input, not a prerequisite for autonomy.

Ontology is the engine's semantic operating model, not merely a label attached to its output. Stealth, adaptive extraction, LLM/VLM reasoning, and automated challenge resolution serve this overarching autonomous acquisition contract.

## Responsibility boundaries

Veilpick owns the Rust execution loop, ontology lifecycle, semantic planning, acquisition orchestration, extraction, entity reconciliation, challenge-resolution orchestration, validation, and extraction provenance.

When composed with OriginWeave, its adapters consume OriginWeave's governed transport, browser, presentation-identity, policy, and runtime-evidence contracts rather than duplicating those authority boundaries. An open upstream PR is a dependency candidate, not an integrated capability.

## Architecture decisions

- [ADR index](docs/adr/README.md)
- [ADR 0002: Ontology-based, fully autonomous Rust scraping engine](docs/adr/0002-ontology-based-autonomous-rust-engine.md) defines the overarching product contract and v1 acceptance gates.
- [ADR 0001: Automated challenge resolution](docs/adr/0001-automated-challenge-resolution.md) defines the required challenge subsystem under that contract.

## License

Apache License 2.0. See [LICENSE](LICENSE).
