# Architecture decision records

An accepted ADR records product/design authority; it does not prove implementation, passing tests, merge, or release. These decisions reflect the product owner's direction on 2026-09-05. Repository integration remains subject to normal pull-request review.

| ADR | Decision | Decision status | Integration status | Relationship |
| --- | --- | --- | --- | --- |
| [0002](0002-ontology-based-autonomous-rust-engine.md) | Ontology-based, fully autonomous website scraping engine built in Rust | Accepted | Integrated | Governing product contract |
| [0001](0001-automated-challenge-resolution.md) | Automated challenge resolution is a required capability | Accepted | Integrated | Required challenge subsystem under ADR 0002 |
| [0003](0003-stealth-and-ecosystem-composition.md) | First-class stealth and ecosystem composition | Accepted | Integrated | Refines ADR 0002; preserves ADR 0001 and assigns reuse boundaries |
| [0004](0004-independent-anti-bot-engine.md) | Independent outbound anti-bot access and challenge engine | Proposed | Proposed | Preserves ADR 0001-0003; excludes Wardnet ownership |
| [0005](0005-evidence-based-site-reputation-engine.md) | Consume Wardnet-owned destination reputation decisions | Proposed | Proposed | Removes competing ownership; keeps Veilpick consumer ACL and conformance boundaries |

ADR 0002 followed the initial challenge discussion; numbering is preserved. ADR 0003 makes stealth explicit alongside ontology and autonomy, separates task-local ontology use from shared semantic publication, and corrects feature-branch merge versus default-branch availability.

Implementation and dependency status remain separate from design acceptance. No ADR is evidence of a universal solver, working browser integration, a verified external dependency, or a complete autonomous Rust engine.

Proposed ADRs 0004 and 0005 share a [composition specification](../design/access-and-reputation-engines.md) and [research record](../research/access-and-reputation-evidence.md). The [anti-bot plan](../superpowers/plans/2026-09-05-anti-bot-engine.md) is owner-side Veilpick work; the [reputation plan](../superpowers/plans/2026-09-05-site-reputation-engine.md) is consumer integration only after a Wardnet immutable release. These records deliver no runtime implementation.
