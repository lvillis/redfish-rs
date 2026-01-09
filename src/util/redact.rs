use serde_json::Value;

const TRUNCATED_SUFFIX: &str = "...(truncated)";

pub(crate) fn redacted_body_snippet(body: &[u8], max_len: usize) -> Option<String> {
    if max_len == 0 {
        return None;
    }
    if body.is_empty() {
        return None;
    }

    // If the body is reasonably small, attempt JSON redaction first.
    let json_parse_limit = 256 * 1024;
    if body.len() <= json_parse_limit
        && let Ok(mut value) = serde_json::from_slice::<Value>(body)
    {
        redact_json_value(&mut value);

        if let Ok(mut s) = serde_json::to_string(&value) {
            truncate_in_place(&mut s, max_len);
            return Some(s);
        }
    }

    // Fall back to lossy UTF-8 snippet.
    let mut s = String::from_utf8_lossy(body).to_string();
    truncate_in_place(&mut s, max_len);
    Some(s)
}

fn truncate_in_place(s: &mut String, max_len: usize) {
    if s.len() <= max_len {
        return;
    }

    // If there's no room for a suffix, just truncate.
    if max_len <= TRUNCATED_SUFFIX.len() {
        let mut cut = max_len;
        while !s.is_char_boundary(cut) && cut > 0 {
            cut -= 1;
        }
        s.truncate(cut);
        return;
    }

    // Make room for the suffix.
    let mut cut = max_len - TRUNCATED_SUFFIX.len();
    while !s.is_char_boundary(cut) && cut > 0 {
        cut -= 1;
    }

    s.truncate(cut);
    s.push_str(TRUNCATED_SUFFIX);
}

fn redact_json_value(value: &mut Value) {
    match value {
        Value::Object(map) => {
            for (k, v) in map.iter_mut() {
                if is_sensitive_key(k) {
                    *v = Value::String("<redacted>".to_string());
                } else {
                    redact_json_value(v);
                }
            }
        }
        Value::Array(items) => {
            for item in items.iter_mut() {
                redact_json_value(item);
            }
        }
        _ => {}
    }
}

fn is_sensitive_key(key: &str) -> bool {
    // Case-insensitive match for common sensitive keys.
    let k = key.to_ascii_lowercase();
    matches!(
        k.as_str(),
        "password"
            | "passphrase"
            | "secret"
            | "token"
            | "access_token"
            | "refresh_token"
            | "x-auth-token"
            | "authorization"
            | "api_key"
            | "apikey"
            | "key"
    )
}
