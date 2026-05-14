# Portless Contracts

[![Contracts CI](https://github.com/CRBL-Technologies/portless-contracts/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/CRBL-Technologies/portless-contracts/actions/workflows/ci.yml?query=branch%3Amain)
[![Security](https://github.com/CRBL-Technologies/portless-contracts/actions/workflows/security.yml/badge.svg?branch=main)](https://github.com/CRBL-Technologies/portless-contracts/actions/workflows/security.yml?query=branch%3Amain)

Shared protobuf and generated API bindings for Portless.

This repo is the source of truth for cross-repo service contracts. Runtime repos
should consume this package or generated artifacts instead of hand-copying
request and response shapes.

## Public Use

This repository contains interface definitions and generated bindings only. It
does not need runtime credentials, deployment configuration, customer data, or
private environment files.

Consumers should depend on tagged releases, for example `v1.0.3`, rather than a
moving branch. Security reporting and sensitive-data guidance are in
[SECURITY.md](SECURITY.md).

## Versioning

Contract tags use `v1.0.x`. Treat tags as compatibility points for downstream
repos; update consumers only after the contracts CI, protobuf checks, and
security checks pass on `main`.

## Contents

- `proto/portless/v1/control.proto` - relay control, daemon status, certificates,
  quota, and lifecycle messages.
- `gen/go/` - committed Go protobuf/gRPC bindings.
- `src/` and `build.rs` - Rust crate wrapper; Rust bindings are generated at
  build time with vendored `protoc`.

## Requirements

- Rust `1.95.0`
- Go, for validating generated Go bindings
- `protoc`, `protoc-gen-go`, and `protoc-gen-go-grpc` only when regenerating
  committed Go code

## Check

```sh
cargo fmt -- --check
cargo test --locked
cargo clippy --locked -- -D warnings
cargo deny check advisories bans licenses sources
GOWORK=off go test ./...
```

`cargo deny` intentionally leaves duplicate-version warnings visible. Advisory,
license, and source-policy failures still fail the check.

## Regenerate Go Bindings

```sh
protoc -I proto \
  --go_out=gen/go --go_opt=paths=source_relative \
  --go-grpc_out=gen/go --go-grpc_opt=paths=source_relative \
  proto/portless/v1/control.proto
```

Tag contract releases (`v1.0.x`) before updating consumers.
