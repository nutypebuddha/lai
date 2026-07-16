# Contributing to Athena

Thank you for your interest. Athena is early-stage and the contribution model is still taking shape.

## Philosophy

- **Formulas, not facts** — if you're adding a static lookup value, consider whether it should be a formula instead.
- **Determinism first** — all computations must be reproducible. No randomness, no ML, no non-deterministic dependencies.
- **Cross-domain by default** — every formula should reference at least one other domain.
- **Professional communication** — be clear, be concise, be constructive.

## Pull Requests

1. Keep changes focused. One PR = one concern.
2. Include tests for new formulas and formula chains.
3. Run `cargo test` and `cargo clippy` before submitting.
4. If adding a new formula tier, include at least 3 example formulas.

## Code Style

- Follow standard Rust formatting (`rustfmt`).
- Clippy must pass with no warnings.
- All public APIs must have doc comments.
- Prefer `anyhow` for error propagation; use `thiserror` for library errors.

## Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat(wheel): add Opposition aspect traversal
fix(bankai): handle division-by-zero in EvalEngine
docs(api): document traverse API return types
```

## Reporting Issues

- Use the GitHub issue tracker.
- Include a minimal reproduction case.
- Mention the formula chain or domain involved.
