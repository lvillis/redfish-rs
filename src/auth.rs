use std::fmt;

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use http::HeaderMap;
use http::header::{AUTHORIZATION, HeaderName, HeaderValue};
use secrecy::{ExposeSecret, SecretString};

use crate::Error;

/// Header name used by Redfish session-based authentication.
pub const X_AUTH_TOKEN: HeaderName = HeaderName::from_static("x-auth-token");

/// Username/password credentials.
#[derive(Clone)]
pub struct Credentials {
    username: String,
    password: SecretString,
}

impl Credentials {
    /// Create username/password credentials.
    pub fn new(username: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            password: SecretString::new(password.into().into_boxed_str()),
        }
    }

    /// Return the username.
    pub fn username(&self) -> &str {
        &self.username
    }
}

impl fmt::Debug for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Never print secrets.
        f.debug_struct("Credentials")
            .field("username", &self.username)
            .field("password", &"<redacted>")
            .finish()
    }
}

/// Redfish session token.
#[derive(Clone)]
pub struct SessionToken {
    token: SecretString,
}

impl SessionToken {
    /// Create a session token.
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: SecretString::new(token.into().into_boxed_str()),
        }
    }
}

impl fmt::Debug for SessionToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SessionToken")
            .field("token", &"<redacted>")
            .finish()
    }
}

/// Authentication configuration.
///
/// Note: This enum is marked `non_exhaustive` for forward compatibility.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub enum Auth {
    /// No authentication.
    #[default]
    None,
    /// HTTP Basic authentication.
    Basic(Credentials),
    /// HTTP Bearer authentication (Authorization: Bearer ...).
    Bearer(SecretString),
    /// Redfish session token authentication (X-Auth-Token).
    SessionToken(SessionToken),
}

impl Auth {
    /// No authentication.
    pub fn none() -> Self {
        Self::None
    }

    /// HTTP Basic authentication.
    pub fn basic(username: impl Into<String>, password: impl Into<String>) -> Self {
        Self::Basic(Credentials::new(username, password))
    }

    /// HTTP Bearer authentication.
    pub fn bearer(token: impl Into<String>) -> Self {
        Self::Bearer(SecretString::new(token.into().into_boxed_str()))
    }

    /// Redfish session token authentication (X-Auth-Token).
    pub fn session_token(token: impl Into<String>) -> Self {
        Self::SessionToken(SessionToken::new(token))
    }

    pub(crate) fn apply_headers(&self, headers: &mut HeaderMap) -> Result<(), Error> {
        match self {
            Auth::None => Ok(()),
            Auth::Basic(creds) => {
                let header_value = basic_auth_header_value(creds)?;
                headers.insert(AUTHORIZATION, header_value);
                Ok(())
            }
            Auth::Bearer(token) => {
                let mut value = String::from("Bearer ");
                value.push_str(token.expose_secret());

                let header_value = HeaderValue::from_str(&value).map_err(|e| {
                    Error::invalid_config(format!("Invalid bearer token for header: {e}"))
                })?;

                headers.insert(AUTHORIZATION, header_value);
                Ok(())
            }
            Auth::SessionToken(token) => {
                let header_value =
                    HeaderValue::from_str(token.token.expose_secret()).map_err(|e| {
                        Error::invalid_config(format!("Invalid X-Auth-Token value: {e}"))
                    })?;

                headers.insert(X_AUTH_TOKEN.clone(), header_value);
                Ok(())
            }
        }
    }
}

fn basic_auth_header_value(creds: &Credentials) -> Result<HeaderValue, Error> {
    // RFC 7617: Basic base64(username:password)
    let mut user_pass = String::with_capacity(creds.username.len() + 1 + 32);
    user_pass.push_str(&creds.username);
    user_pass.push(':');
    user_pass.push_str(creds.password.expose_secret());

    let encoded = BASE64_STANDARD.encode(user_pass.as_bytes());

    let mut value = String::from("Basic ");
    value.push_str(&encoded);

    HeaderValue::from_str(&value)
        .map_err(|e| Error::invalid_config(format!("Invalid basic auth header: {e}")))
}
