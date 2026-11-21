//! Combined Hallmark CSS cascade (`tokens.css` + `layout.css` + `components.css` + `site.css`).

/// Design tokens CSS layer.
pub const TOKENS_CSS: &str = include_str!("../../../style/tokens.css");
/// Spatial grid and responsive reflow CSS layer.
pub const LAYOUT_CSS: &str = include_str!("../../../style/layout.css");
/// UI controls, status pills, 2.5D stage, dialog, and popover CSS layer.
pub const COMPONENTS_CSS: &str = include_str!("../../../style/components.css");
/// Editorial prose, Mermaid diagram, and card surface CSS layer.
pub const SITE_CSS: &str = include_str!("../../../style/site.css");

/// Returns the concatenated stylesheet served at `/pkg/site.css`.
pub fn combined_stylesheet() -> String {
    let mut css = String::with_capacity(
        TOKENS_CSS.len() + LAYOUT_CSS.len() + COMPONENTS_CSS.len() + SITE_CSS.len() + 8,
    );
    css.push_str(TOKENS_CSS);
    css.push('\n');
    css.push_str(LAYOUT_CSS);
    css.push('\n');
    css.push_str(COMPONENTS_CSS);
    css.push('\n');
    css.push_str(SITE_CSS);
    css
}
