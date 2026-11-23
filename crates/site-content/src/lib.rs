#![forbid(unsafe_code)]
//! Content loader, Markdown/Mermaid renderer, Atom/RSS feed builder,
//! and Contract §4–§6 validator for `kennedy.mosoti.dev`.

pub mod frontmatter;
pub mod loader;
pub mod markdown;
pub mod model;
pub mod privacy;
mod privacy_digests;
pub mod rss;
pub mod search;
pub mod validate;

pub use loader::load_from_dir;
pub use model::{
    BlogPost, Discipline, EvidenceClass, HeadingOutline, Identity, OperatingStage, ProfileLink,
    Project, ProjectStatus, Record, RecordKind, SiteContent, Skill, SkillsDoc, WikiArticle,
};
pub use rss::{SITE_BASE_URL, build_atom_feed, build_rss_feed, build_sitemap_xml};
pub use search::{ClientSearchEntry, WikiSearchHit, WikiSearchIndex};
pub use validate::validate_all;
