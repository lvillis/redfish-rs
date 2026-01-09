#![forbid(unsafe_code)]
#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::todo,
        clippy::unimplemented,
        clippy::dbg_macro
    )
)]

//! Production-grade Rust SDK for DMTF Redfish.
//!
//! This crate provides:
//!
//! - An async-first [`Client`] (Tokio-based)
//! - An optional [`BlockingClient`] behind the `blocking` feature
//! - Common Redfish services (service root, systems, chassis, managers, sessions)
//! - Platform services (accounts, events, tasks, updates)
//! - Registries/JsonSchemas discovery endpoints
//! - A safe and structured error model (`Error`)
//!
//! Most users will start with [`Client::builder`] or [`BlockingClient::builder`].

#[cfg(all(feature = "rustls", feature = "native-tls"))]
compile_error!("Features `rustls` and `native-tls` are mutually exclusive. Enable at most one.");

#[cfg(all(not(feature = "async"), not(feature = "blocking")))]
compile_error!("Enable at least one runtime mode: feature `async` (default) and/or `blocking`.");

pub mod api;
mod auth;
mod client;
mod error;
mod transport;
pub mod types;
mod util;

pub use api::ActionResponse;
pub use auth::{Auth, Credentials, SessionToken};
#[cfg(feature = "blocking")]
pub use client::BlockingClient;
#[cfg(feature = "blocking")]
pub use client::BlockingClientBuilder;
#[cfg(feature = "async")]
pub use client::Client;
#[cfg(feature = "async")]
pub use client::ClientBuilder;
pub use error::{Error, ErrorKind, RequestContext};

/// Common Redfish resource models and request types.
///
/// These are also available under [`crate::types`].
pub use types::{
    Account, AccountCreateRequest, AccountService, AccountUpdateRequest, Bios, BiosSettings,
    BiosSettingsUpdateRequest, Boot, BootSourceOverrideEnabled, BootSourceOverrideTarget, Chassis,
    Collection, ComputerSystem, ComputerSystemUpdateRequest, Drive, EthernetInterface,
    EthernetInterfaceUpdateRequest, EventService, EventSubscription,
    EventSubscriptionCreateRequest, EventSubscriptionUpdateRequest, JsonSchemaFile, LogEntry,
    LogService, Manager, ManagerNetworkProtocol, ManagerNetworkProtocolUpdateRequest,
    MessageRegistryFile, ODataQuery, OdataId, Power, ResetType, Role, Sensor, ServiceRoot, Session,
    SimpleUpdateRequest, SoftwareInventory, Storage, SubmitTestEventRequest, Task, TaskService,
    Thermal, UpdateService,
};

/// Convenience alias.
pub type Result<T> = std::result::Result<T, Error>;

/// Re-exported HTTP types used in the public API.
pub use http::{HeaderMap, HeaderName, HeaderValue, Method, StatusCode};

/// Retry policy configuration.
pub use util::retry::RetryPolicy;
