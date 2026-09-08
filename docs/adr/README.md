# Architecture decision records

An accepted ADR records product/design authority; it does not prove implementation, passing tests, merge, or release. These decisions reflect the product owner's direction on 2026-09-05. Repository integration remains subject to normal pull-request review.

| ADR | Decision | Decision status | Integration status | Relationship |
| --- | --- | --- | --- | --- |
| [0002](0002-ontology-based-autonomous-rust-engine.md) | Ontology-based, fully autonomous website scraping engine built in Rust | Accepted | Proposed | Governing product contract |
| [0001](0001-automated-challenge-resolution.md) | Automated challenge resolution is a required capability | Accepted | Proposed | Required challenge subsystem under ADR 0002 |
| [0003](0003-stealth-and-ecosystem-composition.md) | First-class stealth and ecosystem composition | Accepted | Proposed | Refines ADR 0002; preserves ADR 0001 and assigns reuse boundaries |

ADR 0002 followed the initial challenge discussion; numbering is preserved. ADR 0003 makes stealth explicit alongside ontology and autonomy, separates task-local ontology use from shared semantic publication, and corrects feature-branch merge versus protected-main availability.

Implementation and dependency status remain separate from design acceptance. No ADR is evidence of a universal solver, working browser integration, a verified external dependency, or a complete autonomous Rust engine.
