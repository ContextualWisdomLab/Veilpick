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
- [x] Add `Candidate`, `FrontierLimits`, `FrontierError`, `FrontierStep`, and `CrawlFrontier` implementation source in `src/frontier.rs`; export them from `src/lib.rs` with rustdoc. Compilation remains unverified.
- [x] Add 13 boundary/error-display requirements in `tests/frontier_boundaries.rs` and the synthetic `examples/frontier.rs` consumer.
- [ ] Pass both focused test files on the implementation revision.
- [ ] Run `cargo fmt --all --check`, `cargo test --locked --all-targets`, `cargo clippy --locked --all-targets -- -D warnings`, and `RUSTDOCFLAGS='-D warnings' cargo doc --locked --no-deps` successfully on one unchanged head.
- [x] Update implementation-source status and CHANGELOG; keep Draft because executed verification is unavailable.

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

## Implementation-source refinements

Use `BTreeMap<(Reverse<usize>, usize), Candidate>` for pending priority/FIFO
ordering and a `BTreeSet<String>` lifetime identity ledger. Derive each order key
from the never-decreasing ledger size before admission, not the pending length.
Validate unknown hints before duplicate/capacity shortcuts. Retain original concept
order while deduplicating hints, and expose no frontier reset/clone operation.
Counters describe dispatches for this instance, not global task/network retries.

## Verification environment and truth boundary

The original test-only head is `26d94719943892b619349a4a0e0835c4cf5a46d4`.
Five local baseline copies were matched to its Git blob hashes before editing.
The authoring environment still has no `cargo` or `rustc`; attempts to retrieve
a toolchain did not succeed. Both the baseline and expanded test-first invocations
exited 127, before Rust execution. These are environment failures, not product RED.
The original push run `33948681099` / job `101259264294` was still queued when
rechecked during implementation.

The follow-up request advances implementation source rather than stopping at
interface-only scaffolding. The original 12 test requirements are retained and
the additional 13 tests were written before the production module. However,
RED/GREEN was not executed: do not call this a verified TDD cycle or passing library.
See [the verification record](../../verification/semantic-frontier.md).

No transport, browser-applied stealth, ontology induction, live model integration,
extraction, challenge solver, release, or merge is claimed by this increment.
