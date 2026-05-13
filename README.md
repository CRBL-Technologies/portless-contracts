# Portless Contracts

[![Contracts CI](https://github.com/CRBL-Technologies/portless-contracts/actions/workflows/ci.yml/badge.svg?branch=dev)](https://github.com/CRBL-Technologies/portless-contracts/actions/workflows/ci.yml?query=branch%3Adev)
[![Security](https://github.com/CRBL-Technologies/portless-contracts/actions/workflows/security.yml/badge.svg?branch=dev)](https://github.com/CRBL-Technologies/portless-contracts/actions/workflows/security.yml?query=branch%3Adev)

Shared API and transport contracts for Portless.

This repository is the source of truth for protobuf and schema files that are
consumed across the control plane, relay, daemon, and admin surfaces. Runtime
repositories should generate or vendor code from this repository instead of
hand-copying cross-repo request/response shapes.

## Layout

- `proto/` - versioned protobuf contracts.
- `gen/go/` - generated Go protobuf and gRPC code committed for Go consumers.
- `src/` + `build.rs` - Rust crate entrypoint; `tonic-build` generates Rust
  protobuf and gRPC code from `proto/` at crate build time.

## Secret Handling

This repository must not contain Cloudflare, Portainer, GHCR, Plex, or other
runtime secrets. GitHub repository secrets are intentionally not copied from any
runtime repository; add only the minimum CI credentials needed for contract
linting or publishing.

## Current Contracts

- `proto/portless/v1/control.proto` - relay control-plane lifecycle, config,
  certificate revocation, and quota stream contracts.

## Generation

Go artifacts are generated with:

```sh
protoc -I proto \
  --go_out=gen/go --go_opt=paths=source_relative \
  --go-grpc_out=gen/go --go-grpc_opt=paths=source_relative \
  proto/portless/v1/control.proto
```

Rust consumers depend on this repository as the `portless-contracts` crate; the
crate uses a vendored `protoc` during `cargo build`.
