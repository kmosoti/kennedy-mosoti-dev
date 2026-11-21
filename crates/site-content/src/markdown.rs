//! Markdown + Mermaid rendering delegating to Jacquard's `cockpit-wiki` engine.

use crate::model::HeadingOutline;

/// Rendered HTML plus extracted heading outline and site-relative internal links.
#[derive(Debug, Clone, Default)]
pub struct RenderedMarkdown {
    pub html: String,
    pub headings: Vec<HeadingOutline>,
    pub internal_links: Vec<String>,
}

/// Renders Markdown into safe semantic HTML via `cockpit_wiki::render_markdown`.
pub fn render_markdown(markdown: &str) -> RenderedMarkdown {
    let rendered = cockpit_wiki::render_markdown(markdown);
    let headings = rendered
        .toc
        .into_iter()
        .map(|h| HeadingOutline {
            level: h.level,
            id: h.id,
            text: h.text,
        })
        .collect();
    RenderedMarkdown {
        html: rendered.html,
        headings,
        internal_links: rendered.internal_links,
    }
}

/// Escapes text for safe HTML/XML body inclusion.
pub fn escape_html(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            other => out.push(other),
        }
    }
    out
}

/// Escapes text for safe double-quoted HTML/XML attributes.
pub fn escape_attr(input: &str) -> String {
    escape_html(input)
}
