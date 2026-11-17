# Agent Guidelines for byteforge

## Branching & Workflow Rules

These are hard rules. Do not deviate.

1. **NEVER push directly to `main`.** The `main` branch is protected. All changes must go through a pull request.
2. **An issue must exist before a PR.** Before creating any pull request, verify that a corresponding GitHub issue exists. If it does not, create the issue first (describe the problem/feature clearly), then reference it in the PR.
3. **Always work on a feature branch.** Create a branch off `main` using a descriptive name, e.g. `feat/tokenizer-cache`, `fix/decode-panic`, `ci/add-coverage`, `perf/bench-setup`.
4. **PRs must link the issue.** The PR body must include a reference such as `Closes #<issue-number>` so the issue is closed on merge.
5. **Do not commit directly to `main`, do not force-push, do not merge locally and push.** Open a PR and let it be reviewed.
6. Commit messages should follow the existing style in the repo (e.g. `feat: ...`, `fix: ...`, `ci: ...`, `perf: ...`).

## Required Checks Before Opening a PR

CI runs `cargo fmt`, `clippy`, and tests. Run these locally and make sure they pass before pushing:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

Optionally run benchmarks:

```sh
cargo bench
```

## Project Layout

- `src/encoding.rs` — encoding logic
- `src/decoding.rs` — decoding logic
- `src/preprocessing.rs` — input preprocessing
- `src/tokenizer.rs` — tokenizer core
- `src/training.rs` — training routines
- `src/lib.rs` — library entry point
- `benches/byteforge_bench.rs` — Criterion benchmarks

## Code Style

- Rust 2024 edition. Keep code idiomatic.
- No new comments unless they explain non-obvious intent.
- Follow existing module patterns and naming.
- Do not introduce new dependencies without justification in the PR.

## Pull Request Checklist

- [ ] A related issue exists and is referenced in the PR body
- [ ] Branch is based on latest `main`
- [ ] `cargo fmt --all -- --check` passes
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` passes
- [ ] `cargo test --all-features` passes
- [ ] PR targets `main` and never pushes to it directly
