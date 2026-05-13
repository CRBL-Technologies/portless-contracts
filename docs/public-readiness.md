# Public Readiness Checklist

This repository is intended to be auditable by developers who need to inspect
the Portless service contracts and generated bindings. Before making it public,
keep these checks green.

## Required Before Public Release

- Confirm all consumers depend on a tagged release, not a moving branch.
- Run protobuf formatting, linting, and breaking-change checks.
- Run `cargo fmt --check`, `cargo test`, and `cargo clippy -- -D warnings`.
- Run `GOWORK=off go test ./...` for generated Go bindings.
- Run a Rust advisory scan such as `cargo audit` or `cargo deny`.
- Run a repository secret scan, including git history, before changing
  visibility.
- Verify the repository contains no deployment configuration, runtime secrets,
  customer data, certificates, or private keys.

## Current Notes

- `proto/portless/v1/control.proto` is the canonical contract source.
- Go bindings in `gen/go/` are committed for Go consumers.
- Rust bindings are generated at build time from the committed protobuf file.
- Runtime configuration belongs in service repos, not this contracts repo.
