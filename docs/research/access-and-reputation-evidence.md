# Access and reputation: research and repository evidence

Inspection date: 2026-09-05. This record distinguishes observed source facts, proposed design choices and unimplemented work. Sources justify semantic boundaries, not product effectiveness or benchmark results. Numbered references are shared by the two ADRs and the design specification.

## Repository observations

| Source | Inspected generation | Supported observation |
| --- | --- | --- |
| Wardnet README, AGENTS, CLAUDE and core model | `5829a0f08d78de464dd24393ce5d0f25fba9d126` | WAF/IDS/AI SOC/gateway ownership; `ThreatIndicator` and `DnsblEntry` are not complete site-reputation or challenge-resolution contracts |
| Veilpick product ADRs | PR #1, refreshed to `24ee7474f02457418b120018f4c6ba6c99de66b9` | Ontology-guided autonomous acquisition, first-class stealth/ecosystem ownership and mandatory functional supported-class challenge resolution; PR was open, not default-branch delivery |
| OriginWeave README and active HTTP candidate | protected main observed `87c4daa1830bac5a5228b6036752ad5633232085`; PR #37 observed `1e2f41072854edcdbaf0f9ecf14697a3bfd62195` | Runtime authority and bounded HTTP are separate from extraction; open candidate does not establish integration availability |
| OriginWeave presentation candidate | PR #229, `024f63690cf05cfe6f0d4a430f0e18ea8fd2c4d6`, as recorded in parent Veilpick ADR 0002 | Historical candidate reference only; this record does not independently establish its current head or applied browser behavior |

Primary repository links: [Wardnet core](https://github.com/ContextualWisdomLab/wardnet/blob/5829a0f08d78de464dd24393ce5d0f25fba9d126/crates/waf-ids-core/src/lib.rs), [Wardnet operating boundary](https://github.com/ContextualWisdomLab/wardnet/blob/5829a0f08d78de464dd24393ce5d0f25fba9d126/AGENTS.md), [Veilpick parent](https://github.com/ContextualWisdomLab/Veilpick/pull/1), [OriginWeave HTTP candidate](https://github.com/ContextualWisdomLab/OriginWeave/pull/37).

The parent advanced legitimately during design preparation and added ADR 0003. Its new README and ADR 0001-0003 are preserved; the new engine records use 0004 and 0005 to avoid competing numbers.

These snapshots are historical inspection anchors, not version pins for future Cargo dependencies. Re-read live refs, code, tests and governance at integration time. Wardnet PR #130 owns its existing product gap-baseline path; this design does not become a second writer for that document.

## Primary standards and academic references

[1] Nottingham, M., & Fielding, R. (2012). *Additional HTTP status codes* (RFC 6585), section 4. RFC Editor. DOI: [10.17487/RFC6585](https://doi.org/10.17487/RFC6585). [Full text](https://www.rfc-editor.org/rfc/rfc6585.html). Supports 429 classification and the distinction between response caching and our separate quota ledger.

[2] Fielding, R., Nottingham, M., & Reschke, J. (Eds.). (2022). *HTTP semantics* (RFC 9110), sections 9.2.2 and 10.2.3. RFC Editor. DOI: [10.17487/RFC9110](https://doi.org/10.17487/RFC9110). [Full text](https://www.rfc-editor.org/rfc/rfc9110.html). Supports idempotency-aware retries and both `Retry-After` forms. Durable reservation, reconciliation and budget ceilings are our design choices.

[3] Koster, M., Illyes, G., Zeller, H., & Sassman, L. (2022). *Robots Exclusion Protocol* (RFC 9309). RFC Editor. DOI: [10.17487/RFC9309](https://doi.org/10.17487/RFC9309). [Full text](https://www.rfc-editor.org/rfc/rfc9309.html). Robots directives do not grant access authorization; the design preserves both boundaries.

[4] OASIS Open. (2021). *STIX Version 2.1*, sections 3.2, 3.6 and 4.7. [OASIS Standard](https://docs.oasis-open.org/cti/stix/v2.1/os/stix-v2.1-os.html). Supports producer confidence, versioning, revocation and indicator validity. It specifies neither a site-reputation score nor calibrated maliciousness probability. Consumer freshness policy and cache invalidation are explicit local design decisions.

[5] Lebo, T., Sahoo, S., & McGuinness, D. (Eds.). (2013, April 30). *PROV-O: The PROV ontology*. W3C. [Dated recommendation](https://www.w3.org/TR/2013/REC-prov-o-20130430/). Supplies provenance vocabulary; attribution and derivation do not establish truth or independent corroboration.

[6] Guo, C., Pleiss, G., Sun, Y., & Weinberger, K. Q. (2017). On calibration of modern neural networks. In *Proceedings of the 34th International Conference on Machine Learning*, PMLR 70, 1321-1330. [Publisher record](https://proceedings.mlr.press/v70/guo17a.html). [arXiv:1706.04599](https://arxiv.org/abs/1706.04599). Supports evaluating calibration rather than treating model confidence as an empirical probability. Its experiments do not validate this proposed engine.

[7] Sahoo, D., Liu, C., & Hoi, S. C. H. (2017). *Malicious URL detection using machine learning: A survey* [Preprint]. [arXiv:1701.07179](https://arxiv.org/abs/1701.07179). Supports the distinction between known-list lookup and learned URL signals. It does not justify assuming that an unlisted URL is benign.

[8] Le, H., Pham, Q., Sahoo, D., & Hoi, S. C. H. (2018). *URLNet: Learning a URL representation with deep learning for malicious URL detection* [Preprint]. [arXiv:1802.03162](https://arxiv.org/abs/1802.03162). Provides a learned-URL baseline candidate, not a selected production model or evidence that source reliability can be inferred from URL strings.

## Research-to-design limitations and redistribution

The cited academic records establish motivation for separately evaluated learned signals and calibration. This PR does not reproduce their experiments, transfer their accuracy numbers, claim the newest research baseline, or assert that a CAPTCHA provider can always be solved. Our three-attempt, 90-second and bounded-evidence defaults are proposed engineering test-profile values, not literature-derived optima.

This change cites and summarizes the sources rather than copying paper PDFs. It does not establish redistribution permission for the exact PDF editions; open access alone is not treated as that permission. No third-party PDF is included. A future archive addition must verify the edition-specific redistribution license and retain its attribution before committing bytes.
