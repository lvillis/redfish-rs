#![cfg_attr(docsrs, feature(doc_cfg))]
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
//! - An async-first [`Client`] when an `async-tls-*` feature is enabled
//! - An optional [`BlockingClient`] when a `blocking-tls-*` feature is enabled
//! - Common Redfish services (service root, systems, chassis, managers, sessions)
//! - Platform services (accounts, events, tasks, updates)
//! - Registries/JsonSchemas discovery endpoints
//! - A safe and structured error model (`Error`)
//!
//! Most users will start with [`Client::builder`] or [`BlockingClient::builder`].

#[cfg(not(any(feature = "_async", feature = "_blocking")))]
compile_error!(
    "redfish requires at least one transport feature: enable an `async-tls-*` or `blocking-tls-*` feature"
);

#[cfg(all(
    feature = "_async",
    not(feature = "async-tls-rustls-ring"),
    not(feature = "async-tls-rustls-aws-lc-rs"),
    not(feature = "async-tls-native")
))]
compile_error!(
    "async transport requires one async TLS backend: enable `async-tls-rustls-ring`, `async-tls-rustls-aws-lc-rs`, or `async-tls-native`"
);

#[cfg(all(
    feature = "_async",
    any(
        all(
            feature = "async-tls-rustls-ring",
            feature = "async-tls-rustls-aws-lc-rs"
        ),
        all(feature = "async-tls-rustls-ring", feature = "async-tls-native"),
        all(feature = "async-tls-rustls-aws-lc-rs", feature = "async-tls-native")
    )
))]
compile_error!(
    "async transport requires exactly one TLS backend: choose only one of `async-tls-rustls-ring`, `async-tls-rustls-aws-lc-rs`, or `async-tls-native`"
);

#[cfg(all(
    feature = "_blocking",
    not(feature = "blocking-tls-rustls-ring"),
    not(feature = "blocking-tls-rustls-aws-lc-rs"),
    not(feature = "blocking-tls-native")
))]
compile_error!(
    "blocking transport requires one blocking TLS backend: enable `blocking-tls-rustls-ring`, `blocking-tls-rustls-aws-lc-rs`, or `blocking-tls-native`"
);

#[cfg(all(
    feature = "_blocking",
    any(
        all(
            feature = "blocking-tls-rustls-ring",
            feature = "blocking-tls-rustls-aws-lc-rs"
        ),
        all(feature = "blocking-tls-rustls-ring", feature = "blocking-tls-native"),
        all(
            feature = "blocking-tls-rustls-aws-lc-rs",
            feature = "blocking-tls-native"
        )
    )
))]
compile_error!(
    "blocking transport requires exactly one TLS backend: choose only one of `blocking-tls-rustls-ring`, `blocking-tls-rustls-aws-lc-rs`, or `blocking-tls-native`"
);

pub mod api;
mod auth;
mod client;
mod error;
mod transport;
pub mod types;
mod util;

pub use api::ActionResponse;
pub use auth::{Auth, Credentials, SessionToken};
#[cfg(feature = "_blocking")]
pub use client::BlockingClient;
#[cfg(feature = "_blocking")]
pub use client::BlockingClientBuilder;
#[cfg(feature = "_async")]
pub use client::Client;
#[cfg(feature = "_async")]
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
