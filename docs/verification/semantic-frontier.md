# Semantic frontier implementation: verification boundary

Initial source date: 2026-09-05. Verification refreshed: 2026-10-01.
Source predecessor: `26d94719943892b619349a4a0e0835c4cf5a46d4`.
This receipt separates local verification from exact-head hosted evidence.

## Delivered source

- `src/frontier.rs` implements validated candidates, immutable task concepts,
  lifetime deduplication, stable concept-priority/FIFO ordering, dispatch limits,
  and non-reflecting diagnostics. `src/lib.rs` exports the five public types.
- The original 12 tests remain byte-identical. Thirteen additional tests cover
  UTF-8 limits, raw-input boundaries, exact opaque identity, admission/dispatch
  interleaving, cumulative exhaustion, 81 priority combinations, full 4096-target
  capacity, and diagnostic privacy.
- `examples/frontier.rs` exercises synthetic planning only. Its expected order is
  `product-detail`, `product-name`, then `catalog-index`; this is an expected
  result, not an observed run in this environment.

## Historical hosted evidence

GitHub Actions run `34084356474`, job `101625442120`, checked out exact Draft PR #2
head `556b30b9eaac083ccb52cd6c5193411df0fb81e8` with Rust 1.98.1. All 25 tests
passed. `cargo fmt --all --check` then failed on `src/frontier.rs`,
`tests/frontier_boundaries.rs`, and `tests/frontier_contract.rs`; Clippy and rustdoc
did not execute. A passing test step did not make that head merge-ready.

## 2026-10-01 local repair evidence

The exact failing head reproduced the same rustfmt diff locally with Cargo and
rustc 1.98.1. Before formatting, `cargo test --locked --all-targets` passed all 25
tests, `cargo clippy --locked --all-targets -- -D warnings` passed, and
`RUSTDOCFLAGS='-D warnings' cargo doc --locked --no-deps` passed. The repair applies
only Rust 1.98.1 rustfmt output and evidence corrections; it does not change
frontier behavior, dependencies, or the pinned toolchain.

The repaired branch passed the complete command set below locally and on exact
unchanged hosted head `4f62c9d81b5811bfdd26de75640445345f75c32b`.
Push run `36806174987`, job `110190883719`, completed successfully with the
exact-source assertion, Rust 1.98.1 tests, rustfmt, Clippy, rustdoc, and the
tracked-mutation check:

```bash
cargo test --locked --all-targets
cargo fmt --all --check
cargo clippy --locked --all-targets -- -D warnings
RUSTDOCFLAGS='-D warnings' cargo doc --locked --no-deps
cargo run --locked --example frontier
```

Keep PR #2 Draft until all required governance checks and independent approval
exist. The exact hosted Rust lane is green, but a passing feature head, source
review, or model review is not protected-branch approval. The branch integrates protected `develop` through an
ordinary merge; no history rewrite, force-push, workflow, toolchain, lockfile,
runtime dependency, protected branch, release, or product behavior change is made.

Stealth, ontology induction, acquisition adapters, extraction, and supported-class
automated challenge resolution remain required product capabilities. This frontier
neither implements them nor weakens their acceptance gates. Its attempts count
local dispatches, not actual HTTP requests; an empty queue does not certify success.
