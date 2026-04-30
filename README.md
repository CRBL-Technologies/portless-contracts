# Portless Contracts

Shared API and transport contracts for Portless.

This repository is the source of truth for protobuf and schema files that are
consumed across the control plane, relay, daemon, and admin surfaces. Runtime
repositories should generate or vendor code from this repository instead of
hand-copying cross-repo request/response shapes.

## Layout

- `proto/` - versioned protobuf contracts.
- `gen/` - generated code output, ignored by git unless a generated artifact is
  intentionally published later.

## Secret Handling

This repository must not contain Cloudflare, Portainer, GHCR, Plex, or other
runtime secrets. GitHub repository secrets are intentionally not copied from any
runtime repository; add only the minimum CI credentials needed for contract
linting or publishing.

## Current Contracts

- `proto/portless/v1/control.proto` - relay control-plane lifecycle, config,
  certificate revocation, and quota stream contracts.

