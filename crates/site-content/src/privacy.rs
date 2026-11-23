//! Irreversible BLAKE3 domain-separated privacy/NDA and generic secret detectors (`CONTRACT.md` §5–§6).
//! Zero plaintext private identifiers or split literal fragments exist in this module.

use crate::privacy_digests::{
    CONTENT_POLICY_DIGESTS, EXCLUDED_CLIENT_HASHES, OPERATIONAL_PHRASE_DIGESTS,
};

/// Canonical redacted privacy failure message (never echoes matched text).
#[must_use]
pub fn privacy_error(loc: &str) -> String {
    format!("{loc}: matched prohibited privacy rule")
}

/// FNV-1a 64-bit word hash enforcing client-name exclusion without storing client literals.
fn fnv1a_64(word: &str) -> u64 {
    word.bytes().fold(0xcbf2_9ce4_8422_2325, |h, b| {
        (h ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3)
    })
}

fn hash_phrase_parts(hasher: &mut blake3::Hasher, parts: &[&str]) -> [u8; 32] {
    hasher.reset();
    hasher.update(b"km-privacy-v1:");
    for (idx, part) in parts.iter().enumerate() {
        if idx > 0 {
            hasher.update(b" ");
        }
        hasher.update(part.as_bytes());
    }
    *hasher.finalize().as_bytes()
}

fn hash_single_token(hasher: &mut blake3::Hasher, token: &str) -> [u8; 32] {
    hasher.reset();
    hasher.update(b"km-privacy-v1:");
    hasher.update(token.as_bytes());
    *hasher.finalize().as_bytes()
}

/// Returns `true` if `text` contains any prohibited operational identifier, workstation path,
/// private IPv4/ULA IPv6 range, or PEM/SSH/age key header/token.
#[must_use]
pub fn contains_operational_or_secret_leak(text: &str) -> bool {
    scan_text(text, false)
}

/// Returns `true` if `text` contains any Contract §5–§6 privacy, client, operational,
/// workstation, credential, or prohibited positioning violation.
#[must_use]
pub fn contains_content_policy_or_privacy_leak(text: &str) -> bool {
    scan_text(text, true)
}

fn scan_text(text: &str, include_content_policy: bool) -> bool {
    let lower = text.to_lowercase();
    if has_home_user_path(&lower)
        || has_prohibited_ipv4(&lower)
        || has_ula_ipv6(&lower)
        || has_ssh_public_key(text)
    {
        return true;
    }

    let tokens: Vec<&str> = lower
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .collect();

    if has_key_header_or_age_token(&tokens) {
        return true;
    }
    if include_content_policy
        && tokens
            .iter()
            .any(|w| EXCLUDED_CLIENT_HASHES.contains(&fnv1a_64(w)))
    {
        return true;
    }

    matches_digest_windows(&tokens, include_content_policy)
}

fn matches_digest_windows(tokens: &[&str], include_content_policy: bool) -> bool {
    let mut hasher = blake3::Hasher::new();
    let matches_any = |dig: &[u8; 32]| {
        OPERATIONAL_PHRASE_DIGESTS.contains(dig)
            || (include_content_policy && CONTENT_POLICY_DIGESTS.contains(dig))
    };

    for i in 0..tokens.len() {
        let tok = tokens[i];
        if matches_any(&hash_single_token(&mut hasher, tok)) {
            return true;
        }
        if tok.starts_with("age1")
            && tok.len() >= 8
            && let Some(prefix) = tok.get(..8)
            && matches_any(&hash_single_token(&mut hasher, prefix))
        {
            return true;
        }
        if i + 1 < tokens.len() && matches_any(&hash_phrase_parts(&mut hasher, &tokens[i..=i + 1]))
        {
            return true;
        }
        if i + 2 < tokens.len() && matches_any(&hash_phrase_parts(&mut hasher, &tokens[i..=i + 2]))
        {
            return true;
        }
        if i + 3 < tokens.len()
            && tokens[i..=i + 3]
                .iter()
                .all(|t| !t.is_empty() && t.len() <= 3 && t.bytes().all(|b| b.is_ascii_digit()))
            && matches_any(&hash_phrase_parts(&mut hasher, &tokens[i..=i + 3]))
        {
            return true;
        }
    }
    false
}

fn has_home_user_path(lower: &str) -> bool {
    let b = lower.as_bytes();
    let needle = b"/home/";
    let mut pos = 0usize;
    while let Some(rel) = lower[pos..].find("/home/") {
        let idx = pos + rel;
        let prev_ok = idx == 0 || !is_path_ident(b[idx - 1]);
        let after = idx + needle.len();
        let next_ok = after < b.len() && is_path_ident(b[after]);
        if prev_ok && next_ok {
            return true;
        }
        pos = after;
    }
    false
}

fn is_path_ident(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-')
}

fn has_prohibited_ipv4(lower: &str) -> bool {
    let mut hasher = blake3::Hasher::new();
    let b = lower.as_bytes();
    let mut i = 0usize;
    while i < b.len() {
        if b[i].is_ascii_digit()
            && (i == 0 || (!b[i - 1].is_ascii_digit() && b[i - 1] != b'.'))
            && let Some((ip_slice, octets, end)) = parse_ipv4_at(lower, i)
        {
            let [o0, o1, _, _] = octets;
            let is_private_or_reserved = o0 == 10
                || (o0 == 172 && (16..=31).contains(&o1))
                || (o0 == 192 && o1 == 168)
                || (o0 == 169 && o1 == 254)
                || (o0 == 100 && (64..=127).contains(&o1));
            if is_private_or_reserved
                || OPERATIONAL_PHRASE_DIGESTS.contains(&hash_single_token(&mut hasher, ip_slice))
            {
                return true;
            }
            i = end;
            continue;
        }
        i += 1;
    }
    false
}

fn parse_ipv4_at(text: &str, start: usize) -> Option<(&str, [u8; 4], usize)> {
    let b = text.as_bytes();
    let mut pos = start;
    let mut octets = [0u8; 4];
    for (idx, slot) in octets.iter_mut().enumerate() {
        let d_start = pos;
        while pos < b.len() && b[pos].is_ascii_digit() && pos - d_start < 3 {
            pos += 1;
        }
        if pos == d_start || (pos < b.len() && b[pos].is_ascii_digit()) {
            return None;
        }
        *slot = text[d_start..pos].parse::<u8>().ok()?;
        if idx < 3 {
            if pos >= b.len() || b[pos] != b'.' {
                return None;
            }
            pos += 1;
        }
    }
    if pos < b.len() && (b[pos].is_ascii_digit() || b[pos] == b'.') {
        return None;
    }
    Some((&text[start..pos], octets, pos))
}

fn has_ula_ipv6(lower: &str) -> bool {
    let b = lower.as_bytes();
    let mut i = 0usize;
    while i + 5 < b.len() {
        let boundary = i == 0 || (!b[i - 1].is_ascii_hexdigit() && b[i - 1] != b':');
        if boundary
            && b[i] == b'f'
            && matches!(b[i + 1], b'c' | b'd')
            && b[i + 2].is_ascii_hexdigit()
            && b[i + 3].is_ascii_hexdigit()
            && b[i + 4] == b':'
        {
            if b[i + 5] == b':' {
                return true;
            }
            let mut j = i + 5;
            let mut hex_len = 0usize;
            while j < b.len() && b[j].is_ascii_hexdigit() && hex_len < 4 {
                j += 1;
                hex_len += 1;
            }
            if hex_len > 0 && j < b.len() && b[j] == b':' {
                return true;
            }
        }
        i += 1;
    }
    false
}

fn has_key_header_or_age_token(tokens: &[&str]) -> bool {
    for i in 0..tokens.len() {
        let t = tokens[i];
        if t.starts_with("age1")
            && t.len() >= 20
            && t[4..]
                .bytes()
                .all(|c| matches!(c, b'0'..=b'9' | b'a'..=b'z'))
        {
            return true;
        }
        if i + 2 < tokens.len() && t == "age" && tokens[i + 1] == "secret" && tokens[i + 2] == "key"
        {
            return true;
        }
        if t == "begin" {
            let max_end = (i + 6).min(tokens.len());
            for j in (i + 1)..max_end {
                if j + 1 < max_end && tokens[j] == "private" && tokens[j + 1] == "key" {
                    return true;
                }
            }
        }
    }
    false
}

fn has_ssh_public_key(text: &str) -> bool {
    let mut prev_was_ssh_algo = false;
    for part in text.split(|c: char| {
        c.is_whitespace() || matches!(c, '"' | '\'' | ',' | '{' | '}' | '[' | ']' | '<' | '>')
    }) {
        if part.is_empty() {
            continue;
        }
        if prev_was_ssh_algo {
            let is_b64 = part.len() >= 20
                && part
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'+' | b'/' | b'='));
            if is_b64 {
                return true;
            }
        }
        prev_was_ssh_algo = matches!(
            part,
            "ssh-ed25519" | "ssh-rsa" | "ssh-dss" | "ecdsa-sha2-nistp256" | "ecdsa-sha2-nistp384"
        );
    }
    false
}
