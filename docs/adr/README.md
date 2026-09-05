# Architecture decision records

An accepted ADR records product/design authority; it does not prove implementation, passing tests, merge, or release. The decisions below reflect the product owner's direction on 2026-09-05. Repository integration remains subject to normal pull-request review.

| ADR | Decision | Status | Relationship |
| --- | --- | --- | --- |
| [0002](0002-ontology-based-autonomous-rust-engine.md) | Ontology-based, fully autonomous website scraping engine built in Rust | Accepted | Governing product contract |
| [0001](0001-automated-challenge-resolution.md) | Automated challenge resolution is a required capability | Accepted | Challenge subsystem under ADR 0002 |

ADR 0002 was recorded after the initial challenge discussion clarified the overarching product identity. The existing number 0001 is preserved; its acceptance language is aligned with the mandatory v1 automation requirement rather than treating the resolver as optional future work.

Implementation and dependency status must be recorded separately from design acceptance. Neither ADR claims a universal solver, working browser integration, or an implemented Rust engine.
