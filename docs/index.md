# Veilpick

Veilpick is a planned Rust engine for ontology-guided, autonomous web acquisition that turns an authorized collection goal into source-backed structured data.

## Current status

This repository currently defines the product and architecture contract. It does not yet publish an executable engine, browser integration, challenge resolver, package, hosted service, or release. Open pull requests and documentation are candidate evidence until they reach the protected default branch.

## Start here

- [Repository overview](../README.md)
- [Architecture decision index](adr/README.md)
- [ADR 0001: Automated challenge resolution](adr/0001-automated-challenge-resolution.md)
- [ADR 0002: Ontology-based, fully autonomous Rust scraping engine](adr/0002-ontology-based-autonomous-rust-engine.md)
- [ADR 0003: First-class stealth and ecosystem composition](adr/0003-stealth-and-ecosystem-composition.md)
- [Ask DeepWiki](https://deepwiki.com/ContextualWisdomLab/Veilpick)

## Product boundary

Veilpick owns the planned acquisition task loop, crawl frontier, task-local ontology use, governed acquisition and stealth orchestration, adaptive extraction, entity reconciliation, validation, and extraction provenance. It does not grant collection authority or publish organization-wide ontology truth.

## Publication boundary

This file is a documentation landing source. GitHub Pages availability is separate repository state and must be verified live before it is presented as published.
