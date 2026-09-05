# Bounded semantic frontier implementation plan

**Goal:** Supply Veilpick's first independently testable Rust acquisition-planning primitive without duplicating a transport or claiming an autonomous scraper is complete.

**Spec:** [ADR 0003](../../adr/0003-stealth-and-ecosystem-composition.md), section 6.

**Architecture:** A single library stores a bounded lifetime target ledger and a pending frontier. Explicit task concept hints determine stable priority. Targets are opaque identifiers supplied by an acquisition adapter, not URL authorizations. The frontier reports candidate selection, drained state, or exhausted attempt budget, never collection success.

**Tech stack:** Rust 1.98.1, edition 2024, standard library only, no runtime dependencies. The official 1.98.1 release fixes a 1.98.0 vtable miscompilation: https://blog.rust-lang.org/2026/09/03/Rust-1.98.1/ . No compiler compatibility beyond the pinned version is claimed.

## Limits and semantics

- Identifier input is nonempty, at most 256 UTF-8 bytes, and contains no whitespace or control characters. It is an opaque key; URI/URL parsing remains outside this module.
- At most 128 concept input entries, checked before deduplication. Required concepts are nonempty; discovery-only candidates may have no concept hints.
- Candidate and total attempt limits are each in 1..=4096. Candidate capacity is lifetime unique identity, not just pending length.
- Reject unknown concept hints without mutating queue, seen identities, or attempts.
- First accepted candidate for an identity is retained; duplicate admission returns false, including after dispatch.
- Priority is count of distinct declared task concepts, then discovery order. This explicit baseline is not learned ranking, OWL reasoning, automatic ontology induction, or semantic correctness evidence.
- Empty queue returns Drained. Nonempty queue after the total attempt cap returns BudgetExhausted without removing a candidate or consuming another attempt.
- Selected candidates are planning output only. Real authorization, scheduling/pacing, actual I/O, evidence admission, completeness validation, and recovery remain separate integration work.

## Task 1: Test-first frontier

Files: `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `src/lib.rs`, `tests/frontier_contract.rs`, `.github/workflows/rust.yml`.

- [x] Specify 12 behavioral regressions in `tests/frontier_contract.rs`, including bounded identity storage, stable ordering, and non-success exhaustion.
- [ ] Execute `cargo test --locked --all-targets` on the test-only head and confirm unresolved frontier API is the actual RED, not an unrelated toolchain or workflow failure.
- [ ] Implement `Candidate`, `FrontierLimits`, `FrontierError`, `FrontierStep`, and `CrawlFrontier` in `src/frontier.rs`; export them from `src/lib.rs` with complete rustdoc.
- [ ] Pass the focused tests and add hostile-boundary/error-display tests for the implemented API.
- [ ] Run `cargo fmt --all --check`, `cargo test --locked --all-targets`, `cargo clippy --locked --all-targets -- -D warnings`, and `RUSTDOCFLAGS='-D warnings' cargo doc --locked --no-deps` on one unchanged head.
- [ ] Update implementation status and CHANGELOG; retain Draft until executed verification and applicable repository review gates complete.

Expected positive behavior:

```rust,ignore
let mut queue = CrawlFrontier::new(
    &["urn:product:name", "urn:product:price"],
    FrontierLimits { max_candidates: 16, max_attempts: 8 },
)?;
queue.enqueue(Candidate::new("page-one", &["urn:product:price"])?)?;
match queue.next_candidate() {
    FrontierStep::Candidate(candidate) => assert_eq!(candidate.id(), "page-one"),
    _ => return Err("candidate was not scheduled".into()),
}
```

## Verification environment and truth boundary

The authoring container has no `cargo` or `rustc`; locating them in the available tool paths failed, and the official Rust download host could not be resolved from that container. This is an environment limitation, not a product-test RED. Repository CI is the execution path. A queued or absent CI run is not a passing test or an observed RED.

The first commit is intentionally test-only and has no frontier implementation. Do not promote this draft as a working Rust library, network client, stealth implementation, ontology engine, or challenge solver.
