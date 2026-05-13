# Portless Contracts

[![Contracts CI](https://github.com/CRBL-Technologies/portless-contracts/actions/workflows/ci.yml/badge.svg?branch=dev)](https://github.com/CRBL-Technologies/portless-contracts/actions/workflows/ci.yml?query=branch%3Adev)
[![Security](https://github.com/CRBL-Technologies/portless-contracts/actions/workflows/security.yml/badge.svg?branch=dev)](https://github.com/CRBL-Technologies/portless-contracts/actions/workflows/security.yml?query=branch%3Adev)

Shared protobuf and generated API bindings for Portless.

This repo is the source of truth for cross-repo service contracts. Runtime repos
should consume this package or generated artifacts instead of hand-copying
request and response shapes.

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
