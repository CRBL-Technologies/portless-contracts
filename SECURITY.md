# Security Policy

## Reporting

Report vulnerabilities privately through GitHub Security Advisories for this
repository. Do not open public issues for suspected vulnerabilities, secrets, or
bypasses.

Include:

- affected version or commit
- reproduction steps
- expected and observed behavior
- impact assessment

## Sensitive Data

This repository should contain protobuf definitions, generated bindings, Rust
wrapper code, CI configuration, and documentation only.

Do not commit service tokens, API keys, deployment environment files,
certificates, private keys, customer data, or production configuration. Local
credential-like files are ignored, and CI scans git history with gitleaks.

## Compatibility

Contract changes can break consumers even when they do not expose secrets. Use
tagged releases, keep protobuf breaking-change checks green, and update
consumers intentionally after each release.
