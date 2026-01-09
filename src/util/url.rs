use url::Url;

use crate::Error;

/// Parse and normalize a base URL.
///
/// - If the scheme is missing, `https://` is assumed.
/// - Query/fragment are stripped.
/// - Path is ensured to end with `/` so `Url::join` behaves predictably.
pub(crate) fn normalize_base_url(input: &str) -> Result<Url, Error> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(Error::invalid_config("Base URL cannot be empty"));
    }

    let candidate = if trimmed.contains("://") {
        trimmed.to_string()
    } else {
        format!("https://{trimmed}")
    };

    let mut url = Url::parse(&candidate)
        .map_err(|e| Error::invalid_config(format!("Invalid base URL '{candidate}': {e}")))?;

    match url.scheme() {
        "http" | "https" => {}
        other => {
            return Err(Error::invalid_config(format!(
                "Unsupported URL scheme '{other}' in base URL"
            )));
        }
    }

    if url.host_str().is_none() {
        return Err(Error::invalid_config("Base URL must include a host"));
    }

    if !url.username().is_empty() || url.password().is_some() {
        return Err(Error::invalid_config(
            "Base URL must not include embedded credentials (userinfo)",
        ));
    }

    url.set_query(None);
    url.set_fragment(None);

    // Ensure trailing slash.
    if !url.path().ends_with('/') {
        let mut path = url.path().to_string();
        path.push('/');
        url.set_path(&path);
    }

    Ok(url)
}

/// Join path segments onto a base URL.
///
/// Segments are treated as individual path segments (no slashes required).
pub(crate) fn join_segments(base: &Url, segments: &[&str]) -> Result<Url, Error> {
    let mut url = base.clone();
    url.set_query(None);
    url.set_fragment(None);

    {
        let mut segs = url.path_segments_mut().map_err(|_| {
            Error::invalid_config("Base URL cannot be used as a base (no path segments)")
        })?;
        for s in segments {
            if s.is_empty() {
                continue;
            }
            // Avoid accidentally interpreting leading slashes.
            let cleaned = s.trim_matches('/');
            if cleaned.is_empty() {
                continue;
            }
            segs.push(cleaned);
        }
    }

    Ok(url)
}

/// Resolve an arbitrary URI (absolute or relative) against `base`.
pub(crate) fn resolve_uri(base: &Url, uri: &str) -> Result<Url, Error> {
    let trimmed = uri.trim();
    if trimmed.is_empty() {
        return Err(Error::invalid_config("URI cannot be empty"));
    }

    if trimmed.contains("://") {
        Url::parse(trimmed)
            .map_err(|e| Error::invalid_config(format!("Invalid absolute URI '{trimmed}': {e}")))
    } else {
        base.join(trimmed)
            .map_err(|e| Error::invalid_config(format!("Invalid relative URI '{trimmed}': {e}")))
    }
}
