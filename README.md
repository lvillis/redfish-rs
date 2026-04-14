# redfish-rs

<a href="https://crates.io/crates/redfish">
  <img src="https://img.shields.io/crates/v/redfish.svg" alt="crates.io version">
</a>

Production-grade Rust SDK for **DMTF Redfish** (async-first, optional blocking).

Goals:

- Clean public API (no backend-specific HTTP client types in public signatures)
- Strong diagnostics (HTTP status, request-id, safe body snippet)
- Security-by-default (auth redaction, no accidental token logging)
- Conservative retries (respects `Retry-After`)
- Testable (wiremock-based tests)

## Install

```toml
[dependencies]
redfish = "0.3.1"
```

By default, the crate enables `async-tls-rustls-ring`.

### Cargo features

- `async-tls-rustls-ring` (default): async client (Tokio-based) over rustls + ring
- `async-tls-rustls-aws-lc-rs`: async client (Tokio-based) over rustls + aws-lc-rs
- `async-tls-native`: async client (Tokio-based) over system-native TLS
- `blocking-tls-rustls-ring`: enable `BlockingClient` over rustls + ring
- `blocking-tls-rustls-aws-lc-rs`: enable `BlockingClient` over rustls + aws-lc-rs
- `blocking-tls-native`: enable `BlockingClient` over system-native TLS
- `tracing`: emit `tracing` spans for requests

## Quick start (async)

```rust,no_run
use redfish::{Auth, Client};

#[tokio::main]
async fn main() -> Result<(), redfish::Error> {
    let client = Client::builder("https://bmc.example.com")?
        .auth(Auth::basic("admin", "password"))
        .build()?;

    let root = client.service_root().get().await?;
    println!("RedfishVersion = {}", root.redfish_version);

    let systems = client.systems().list().await?;
    println!("Systems members = {}", systems.members.len());

    Ok(())
}
```

## Quick start (blocking)

Enable a blocking transport feature:

```toml
redfish = { version = "0.3.1", default-features = false, features = ["blocking-tls-rustls-ring"] }
```

```rust,no_run
use redfish::{Auth, BlockingClient};

fn main() -> Result<(), redfish::Error> {
    let client = BlockingClient::builder("https://bmc.example.com")?
        .auth(Auth::basic("admin", "password"))
        .build()?;

    let root = client.service_root().get()?;
    println!("RedfishVersion = {}", root.redfish_version);

    Ok(())
}
```

## Session login (X-Auth-Token)

Many Redfish implementations support session-based auth at:

- `POST /redfish/v1/SessionService/Sessions`

The response typically includes:

- `X-Auth-Token` (session token)
- `Location` (session resource URI; delete it to logout)

This crate supports that flow via `client.sessions().create(...)`.

## Coverage

Redfish is a large standard. This crate provides:

- Core services: ServiceRoot, Systems, Chassis, Managers
- Auth flows: SessionService (session login/logout)
- Platform services: AccountService, EventService, TaskService, UpdateService
- Registries/JsonSchemas discovery endpoints

For everything else (OEM extensions, less-common resources), you can always fall back to
`client.get_uri::<serde_json::Value>(...)` or the more general `client.request_json_value(...)`
and gradually add typed models as needed.

### Member-oriented helpers

In addition to collection services (e.g. `client.systems().list()`), this crate provides
member-oriented helpers for common sub-resources:

- `client.system("1").get_bios().await?`
- `client.system("1").patch_bios_settings(...).await?`
- `client.manager("1").get_network_protocol().await?`
- `client.chassis_member("1").get_power().await?`

These helpers intentionally model only the most stable/common fields, while still preserving
vendor/OEM fields via `extra` maps.

## MSRV / toolchain

- Rust 1.94.1 (see `rust-toolchain.toml`)
- Edition 2024

## License

Licensed under either of:

- Apache License, Version 2.0
- MIT license

at your option.
