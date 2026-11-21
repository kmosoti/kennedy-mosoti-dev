//! TOML frontmatter extraction delimited by `+++` lines.

use serde::de::DeserializeOwned;

/// Splits `+++` TOML frontmatter from the Markdown body.
pub fn split_frontmatter<'a>(
    raw: &'a str,
    source_name: &str,
) -> Result<(&'a str, &'a str), String> {
    let normalized = raw.strip_prefix('\u{feff}').unwrap_or(raw);
    let rest = normalized
        .strip_prefix("+++\r\n")
        .or_else(|| normalized.strip_prefix("+++\n"))
        .ok_or_else(|| format!("{source_name}: missing opening `+++` frontmatter fence"))?;

    if let Some((fm, body)) = rest.split_once("\n+++\r\n") {
        return Ok((fm, body.trim_start_matches(['\r', '\n'])));
    }
    if let Some((fm, body)) = rest.split_once("\n+++\n") {
        return Ok((fm, body.trim_start_matches(['\r', '\n'])));
    }
    if let Some(fm) = rest.strip_suffix("\n+++") {
        return Ok((fm, ""));
    }
    Err(format!(
        "{source_name}: missing closing `+++` frontmatter fence"
    ))
}

/// Parses TOML frontmatter into `T` and returns `(parsed, body_markdown)`.
pub fn parse_document<T: DeserializeOwned>(
    raw: &str,
    source_name: &str,
) -> Result<(T, String), String> {
    let (fm, body) = split_frontmatter(raw, source_name)?;
    let parsed: T = toml::from_str(fm)
        .map_err(|err| format!("{source_name}: invalid TOML frontmatter: {err}"))?;
    Ok((parsed, body.to_owned()))
}
