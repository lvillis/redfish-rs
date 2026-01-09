use std::borrow::Cow;

use url::Url;

/// OData query options commonly used by Redfish implementations.
///
/// This type is intentionally conservative and string-based:
///
/// - It avoids baking in a particular schema model.
/// - It keeps API surface stable even as different vendors support different filters.
/// - All values are URL-encoded via `url::Url::query_pairs_mut()`.
///
/// Supported parameters:
///
/// - `$select`
/// - `$expand`
/// - `$top`
/// - `$skip`
/// - `$filter`
/// - `$orderby`
///
/// If you need vendor-specific parameters, use [`ODataQuery::with_pair`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ODataQuery {
    select: Option<String>,
    expand: Option<String>,
    top: Option<u64>,
    skip: Option<u64>,
    filter: Option<String>,
    orderby: Option<String>,
    extra: Vec<(String, String)>,
}

impl ODataQuery {
    /// Create an empty query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set `$select`.
    pub fn select(mut self, value: impl Into<String>) -> Self {
        self.select = Some(value.into());
        self
    }

    /// Set `$expand`.
    pub fn expand(mut self, value: impl Into<String>) -> Self {
        self.expand = Some(value.into());
        self
    }

    /// Set `$top`.
    pub fn top(mut self, value: u64) -> Self {
        self.top = Some(value);
        self
    }

    /// Set `$skip`.
    pub fn skip(mut self, value: u64) -> Self {
        self.skip = Some(value);
        self
    }

    /// Set `$filter`.
    pub fn filter(mut self, value: impl Into<String>) -> Self {
        self.filter = Some(value.into());
        self
    }

    /// Set `$orderby`.
    pub fn orderby(mut self, value: impl Into<String>) -> Self {
        self.orderby = Some(value.into());
        self
    }

    /// Add an arbitrary query pair.
    ///
    /// This is useful for OEM/vendor query extensions.
    pub fn with_pair(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.extra.push((key.into(), value.into()));
        self
    }

    /// Apply this query to a URL.
    ///
    /// The existing query string (if any) is preserved.
    pub fn apply_to_url(&self, mut url: Url) -> Url {
        {
            let mut qp = url.query_pairs_mut();

            if let Some(v) = &self.select {
                qp.append_pair("$select", v);
            }
            if let Some(v) = &self.expand {
                qp.append_pair("$expand", v);
            }
            if let Some(v) = self.top {
                qp.append_pair("$top", &v.to_string());
            }
            if let Some(v) = self.skip {
                qp.append_pair("$skip", &v.to_string());
            }
            if let Some(v) = &self.filter {
                qp.append_pair("$filter", v);
            }
            if let Some(v) = &self.orderby {
                qp.append_pair("$orderby", v);
            }

            for (k, v) in &self.extra {
                qp.append_pair(k, v);
            }
        }

        url
    }

    /// Return query key/value pairs in the order they will be applied.
    pub fn pairs(&self) -> Vec<(Cow<'static, str>, Cow<'_, str>)> {
        let mut out: Vec<(Cow<'static, str>, Cow<'_, str>)> = Vec::new();

        if let Some(v) = &self.select {
            out.push((Cow::Borrowed("$select"), Cow::Borrowed(v.as_str())));
        }
        if let Some(v) = &self.expand {
            out.push((Cow::Borrowed("$expand"), Cow::Borrowed(v.as_str())));
        }
        if let Some(v) = self.top {
            out.push((Cow::Borrowed("$top"), Cow::Owned(v.to_string())));
        }
        if let Some(v) = self.skip {
            out.push((Cow::Borrowed("$skip"), Cow::Owned(v.to_string())));
        }
        if let Some(v) = &self.filter {
            out.push((Cow::Borrowed("$filter"), Cow::Borrowed(v.as_str())));
        }
        if let Some(v) = &self.orderby {
            out.push((Cow::Borrowed("$orderby"), Cow::Borrowed(v.as_str())));
        }

        for (k, v) in &self.extra {
            // `extra` is owned by self; return borrowed str slices.
            out.push((Cow::Owned(k.clone()), Cow::Borrowed(v.as_str())));
        }

        out
    }
}
