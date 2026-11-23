//! Core strongly typed content structures for biography, skills, records, Moiric projects,
//! wiki articles, and blog posts.

use serde::{Deserialize, Serialize};

/// Evidence classification for career records and skills (`CONTRACT.md` §4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EvidenceClass {
    Production,
    OperationalExposure,
    Developing,
    Independent,
}

impl EvidenceClass {
    /// Human-readable badge label printed on archive and biography views.
    pub fn label(self) -> &'static str {
        match self {
            Self::Production => "Production",
            Self::OperationalExposure => "Operational exposure",
            Self::Developing => "Developing",
            Self::Independent => "Independent",
        }
    }

    /// Short explanatory gloss for record detail headers.
    pub fn gloss(self) -> &'static str {
        match self {
            Self::Production => "Demonstrated inside a defined production domain.",
            Self::OperationalExposure => {
                "Real operational participation without claiming broad ownership."
            }
            Self::Developing => "Explicit engineering target, not yet an established strength.",
            Self::Independent => "Independent systems engineering and research.",
        }
    }

    /// CSS tone token attribute (`ok`, `info`, `warn`, `neutral`).
    pub fn tone(self) -> &'static str {
        match self {
            Self::Production => "ok",
            Self::OperationalExposure => "info",
            Self::Developing => "warn",
            Self::Independent => "neutral",
        }
    }
}

/// Record classification (`content/records/*.md`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RecordKind {
    Role,
    CaseStudy,
    Note,
    Direction,
    Project,
    Experiment,
}

impl RecordKind {
    /// Human-readable label for a record category.
    pub fn label(self) -> &'static str {
        match self {
            Self::Role => "Role",
            Self::CaseStudy => "Case study",
            Self::Note => "Note",
            Self::Direction => "Direction",
            Self::Project => "Project",
            Self::Experiment => "Experiment",
        }
    }
}

/// Lifecycle status for the five open-source Moiric project showcases.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProjectStatus {
    #[serde(rename = "Released Alpha")]
    ReleasedAlpha,
    #[serde(rename = "In Progress")]
    InProgress,
    #[serde(rename = "Architecture")]
    Architecture,
}

impl ProjectStatus {
    /// Human-readable lifecycle tag.
    pub fn label(self) -> &'static str {
        match self {
            Self::ReleasedAlpha => "Released Alpha",
            Self::InProgress => "In Progress",
            Self::Architecture => "Architecture",
        }
    }

    /// Semantic badge tone paired with geometric status icon.
    pub fn tone(self) -> &'static str {
        match self {
            Self::ReleasedAlpha => "ok",
            Self::InProgress => "info",
            Self::Architecture => "warn",
        }
    }
}

/// External profile link metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileLink {
    pub label: String,
    pub href: String,
    pub kind: String,
}

/// Discipline descriptor from `identity.md`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Discipline {
    pub id: String,
    pub title: String,
    pub summary: String,
    #[serde(default)]
    pub bays: Vec<String>,
}

/// Single step in the six-stage operating pattern.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatingStage {
    pub stage: String,
    pub line: String,
}

/// Identity frontmatter plus rendered prose (`content/identity.md`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Identity {
    pub name: String,
    pub headline: String,
    pub positioning: String,
    pub bio: String,
    pub location: String,
    #[serde(default)]
    pub links: Vec<ProfileLink>,
    #[serde(default)]
    pub disciplines: Vec<Discipline>,
    #[serde(default)]
    pub operating_pattern: Vec<OperatingStage>,
    #[serde(skip_deserializing)]
    pub body_md: String,
    #[serde(skip_deserializing)]
    pub body_html: String,
    #[serde(skip_deserializing)]
    pub internal_links: Vec<String>,
}

/// Skill entry (`content/skills.md`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Skill {
    pub tag: String,
    pub label: String,
    pub evidence: EvidenceClass,
    #[serde(default)]
    pub records: Vec<String>,
    #[serde(default)]
    pub boundary: Option<String>,
}

/// Skills document (`content/skills.md`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SkillsDoc {
    pub skills: Vec<Skill>,
    pub body_md: String,
    pub body_html: String,
}

/// Career/engineering record (`content/records/<slug>.md`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Record {
    pub slug: String,
    pub title: String,
    pub kind: RecordKind,
    pub evidence: EvidenceClass,
    #[serde(default)]
    pub machines: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub flagship: bool,
    #[serde(default)]
    pub period: Option<String>,
    pub summary: String,
    #[serde(default)]
    pub claim_boundary: Option<String>,
    #[serde(default)]
    pub links: Vec<ProfileLink>,
    #[serde(skip_deserializing)]
    pub body_md: String,
    #[serde(skip_deserializing)]
    pub body_html: String,
    #[serde(skip_deserializing)]
    pub internal_links: Vec<String>,
}

/// Curated open-source Moiric project page (`content/projects/<slug>.md`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Project {
    pub slug: String,
    pub name: String,
    pub tagline: String,
    pub status: ProjectStatus,
    pub version: String,
    pub repo: String,
    pub bay_id: String,
    pub role: String,
    pub summary: String,
    pub claim_boundary: String,
    #[serde(default)]
    pub highlights: Vec<String>,
    #[serde(skip_deserializing)]
    pub body_md: String,
    #[serde(skip_deserializing)]
    pub body_html: String,
    #[serde(skip_deserializing)]
    pub internal_links: Vec<String>,
}

/// Public technical wiki article (`content/wiki/<slug>.md`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WikiArticle {
    pub slug: String,
    pub title: String,
    pub category: String,
    pub updated: String,
    pub summary: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(skip_deserializing)]
    pub body_md: String,
    #[serde(skip_deserializing)]
    pub body_html: String,
    #[serde(skip_deserializing)]
    pub headings: Vec<HeadingOutline>,
    #[serde(skip_deserializing)]
    pub internal_links: Vec<String>,
}

/// Public engineering blog post (`content/blog/<slug>.md`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlogPost {
    pub slug: String,
    pub title: String,
    pub date: String,
    pub author: String,
    pub summary: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(skip_deserializing)]
    pub body_md: String,
    #[serde(skip_deserializing)]
    pub body_html: String,
    #[serde(skip_deserializing)]
    pub headings: Vec<HeadingOutline>,
    #[serde(skip_deserializing)]
    pub internal_links: Vec<String>,
}

/// Extracted heading entry for article table-of-contents navigation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeadingOutline {
    pub level: u8,
    pub id: String,
    pub text: String,
}

/// Complete validated content bundle loaded from `content/`.
#[derive(Debug, Clone, Serialize)]
pub struct SiteContent {
    pub identity: Identity,
    pub skills: SkillsDoc,
    pub records: Vec<Record>,
    pub projects: Vec<Project>,
    pub wiki: Vec<WikiArticle>,
    pub blog: Vec<BlogPost>,
}
