//! View modules for the Cockpit shell, interactive islands, and editorial SSR pages.

#[cfg(feature = "ssr")]
pub mod icons;
#[cfg(feature = "ssr")]
pub mod inspector_rail;
pub mod island_cockpit;
pub mod iso_math;
#[cfg(feature = "ssr")]
pub mod iso_model;
pub mod iso_project;
#[cfg(feature = "ssr")]
pub mod iso_stage;
#[cfg(feature = "ssr")]
pub mod layout;
#[cfg(feature = "ssr")]
pub mod pages_bio_archive;
#[cfg(feature = "ssr")]
pub mod pages_projects;
#[cfg(feature = "ssr")]
pub mod pages_wiki_blog;
#[cfg(feature = "ssr")]
pub mod project_bays;
pub mod stage_dom;
