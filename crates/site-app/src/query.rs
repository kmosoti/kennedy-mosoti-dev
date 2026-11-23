//! Bounded query-string parser preventing allocation/CPU exhaustion on dynamic search routes.

/// Maximum raw URL query string length accepted on search routes (`?q=...`).
pub const MAX_RAW_QUERY_BYTES: usize = 256;
/// Maximum percent-decoded `q` byte length accepted on search routes.
pub const MAX_DECODED_QUERY_BYTES: usize = 128;
/// Maximum whitespace- or token-delimited search terms accepted in `q`.
pub const MAX_QUERY_TERMS: usize = 16;

/// Rejection reasons for bounded query parameter parsing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryParamError {
    RawQueryTooLong,
    DecodedValueTooLong,
    TooManyTerms,
    InvalidUtf8,
}

/// Parses and validates `key` from `query` using stack-only decoding before heap allocation.
///
/// Enforces:
/// - `query.len() <= MAX_RAW_QUERY_BYTES` (`256`)
/// - decoded value `<= MAX_DECODED_QUERY_BYTES` (`128`)
/// - valid UTF-8
/// - `<= MAX_QUERY_TERMS` (`16`) terms
///
/// Every matching `key` parameter in `query` is validated before allocating the first
/// non-empty value.
pub fn parse_bounded_query_param(
    query: &str,
    key: &str,
) -> Result<Option<String>, QueryParamError> {
    if query.len() > MAX_RAW_QUERY_BYTES {
        return Err(QueryParamError::RawQueryTooLong);
    }

    let mut selected: Option<([u8; MAX_DECODED_QUERY_BYTES], usize, usize)> = None;

    for pair in query.split('&') {
        let (k, v) = pair.split_once('=').unwrap_or((pair, ""));
        if k != key {
            continue;
        }

        let mut buf = [0u8; MAX_DECODED_QUERY_BYTES];
        let len = decode_into_stack(v, &mut buf)?;
        let decoded = std::str::from_utf8(&buf[..len]).map_err(|_| QueryParamError::InvalidUtf8)?;
        let trimmed = decoded.trim();
        if trimmed.is_empty() {
            continue;
        }

        if count_terms(trimmed) > MAX_QUERY_TERMS {
            return Err(QueryParamError::TooManyTerms);
        }

        if selected.is_none() {
            let start = decoded.len() - decoded.trim_start().len();
            let end = decoded.trim_end().len().max(start);
            selected = Some((buf, start, end));
        }
    }

    Ok(selected.map(|(buf, start, end)| {
        let slice = std::str::from_utf8(&buf[start..end]).expect("validated UTF-8 slice");
        slice.to_owned()
    }))
}

/// Extracts and validates a query parameter, returning `None` on absence or bound violation.
pub fn extract_query_param(query: &str, key: &str) -> Option<String> {
    parse_bounded_query_param(query, key).ok().flatten()
}

fn decode_into_stack(
    input: &str,
    out: &mut [u8; MAX_DECODED_QUERY_BYTES],
) -> Result<usize, QueryParamError> {
    let bytes = input.as_bytes();
    let mut i = 0usize;
    let mut written = 0usize;
    while i < bytes.len() {
        let decoded_byte = match bytes[i] {
            b'+' => {
                i += 1;
                b' '
            }
            b'%' if i + 2 < bytes.len() => {
                if let (Some(hi), Some(lo)) = (hex_val(bytes[i + 1]), hex_val(bytes[i + 2])) {
                    i += 3;
                    (hi << 4) | lo
                } else {
                    i += 1;
                    b'%'
                }
            }
            b => {
                i += 1;
                b
            }
        };
        if written >= out.len() {
            return Err(QueryParamError::DecodedValueTooLong);
        }
        out[written] = decoded_byte;
        written += 1;
    }
    Ok(written)
}

fn count_terms(trimmed: &str) -> usize {
    let ws = trimmed.split_whitespace().count();
    let alnum = trimmed
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|t| !t.is_empty())
        .count();
    ws.max(alnum)
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}
