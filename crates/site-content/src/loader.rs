//! Path-traversal-safe filesystem loader for `content/`.

use crate::frontmatter::parse_document;
use crate::markdown::render_markdown;
use crate::model::{
    BlogPost, Identity, Project, Record, SiteContent, Skill, SkillsDoc, WikiArticle,
};
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Deserialize)]
struct SkillsFrontmatter {
    skills: Vec<Skill>,
}

/// Loads and parses the entire `content/` tree from `root`.
pub fn load_from_dir(root: &Path) -> Result<SiteContent, String> {
    let canonical_root = fs::canonicalize(root)
        .map_err(|err| format!("failed to resolve content root `{}`: {err}", root.display()))?;

    let identity = load_identity(&canonical_root)?;
    let skills = load_skills(&canonical_root)?;
    let records = load_records(&canonical_root)?;
    let projects = load_projects(&canonical_root)?;
    let wiki = load_wiki(&canonical_root)?;
    let blog = load_blog(&canonical_root)?;

    Ok(SiteContent {
        identity,
        skills,
        records,
        projects,
        wiki,
        blog,
    })
}

fn read_safe_file(root: &Path, rel: &Path) -> Result<String, String> {
    let rel_str = rel
        .to_str()
        .ok_or_else(|| "non-UTF-8 relative content path".to_owned())?;
    cockpit_wiki::validate_safe_rel_path(rel_str)
        .map_err(|err| format!("unsafe content path `{rel_str}`: {err}"))?;
    let full = root.join(rel);
    let canon = fs::canonicalize(&full)
        .map_err(|err| format!("failed to open `{}`: {err}", full.display()))?;
    if !canon.starts_with(root) {
        return Err(format!(
            "path `{}` escapes content root `{}`",
            canon.display(),
            root.display()
        ));
    }
    fs::read_to_string(&canon).map_err(|err| format!("failed to read `{}`: {err}", canon.display()))
}

fn sorted_md_files(root: &Path, subdir: &str) -> Result<Vec<PathBuf>, String> {
    let dir = root.join(subdir);
    let entries =
        fs::read_dir(&dir).map_err(|err| format!("failed to list `{}`: {err}", dir.display()))?;
    let mut files = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|err| format!("dir entry error in `{subdir}`: {err}"))?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("md") {
            let file_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .ok_or_else(|| format!("non-UTF-8 file name in `{subdir}`"))?;
            files.push(Path::new(subdir).join(file_name));
        }
    }
    files.sort();
    Ok(files)
}

fn load_identity(root: &Path) -> Result<Identity, String> {
    let raw = read_safe_file(root, Path::new("identity.md"))?;
    let (mut id, body_md) = parse_document::<Identity>(&raw, "content/identity.md")?;
    let rendered = render_markdown(&body_md);
    id.body_md = body_md;
    id.body_html = rendered.html;
    id.internal_links = rendered.internal_links;
    Ok(id)
}

fn load_skills(root: &Path) -> Result<SkillsDoc, String> {
    let raw = read_safe_file(root, Path::new("skills.md"))?;
    let (fm, body_md) = parse_document::<SkillsFrontmatter>(&raw, "content/skills.md")?;
    let rendered = render_markdown(&body_md);
    Ok(SkillsDoc {
        skills: fm.skills,
        body_md,
        body_html: rendered.html,
    })
}

fn load_records(root: &Path) -> Result<Vec<Record>, String> {
    let mut records = Vec::new();
    for rel in sorted_md_files(root, "records")? {
        let label = format!("content/{}", rel.display());
        let raw = read_safe_file(root, &rel)?;
        let (mut rec, body_md) = parse_document::<Record>(&raw, &label)?;
        let rendered = render_markdown(&body_md);
        rec.body_md = body_md;
        rec.body_html = rendered.html;
        rec.internal_links = rendered.internal_links;
        records.push(rec);
    }
    records.sort_by(|a, b| a.slug.cmp(&b.slug));
    Ok(records)
}

fn load_projects(root: &Path) -> Result<Vec<Project>, String> {
    let mut projects = Vec::new();
    for rel in sorted_md_files(root, "projects")? {
        let label = format!("content/{}", rel.display());
        let raw = read_safe_file(root, &rel)?;
        let (mut proj, body_md) = parse_document::<Project>(&raw, &label)?;
        let rendered = render_markdown(&body_md);
        proj.body_md = body_md;
        proj.body_html = rendered.html;
        proj.internal_links = rendered.internal_links;
        projects.push(proj);
    }
    let canonical_order = ["nomos", "fabrico11y", "metron", "jacquard", "hephaestus"];
    projects.sort_by_key(|p| {
        canonical_order
            .iter()
            .position(|&s| s == p.slug)
            .unwrap_or(usize::MAX)
    });
    Ok(projects)
}

fn load_wiki(root: &Path) -> Result<Vec<WikiArticle>, String> {
    let mut articles = Vec::new();
    for rel in sorted_md_files(root, "wiki")? {
        let label = format!("content/{}", rel.display());
        let raw = read_safe_file(root, &rel)?;
        let (mut art, body_md) = parse_document::<WikiArticle>(&raw, &label)?;
        let rendered = render_markdown(&body_md);
        art.body_md = body_md;
        art.body_html = rendered.html;
        art.headings = rendered.headings;
        art.internal_links = rendered.internal_links;
        articles.push(art);
    }
    articles.sort_by(|a, b| a.category.cmp(&b.category).then(a.title.cmp(&b.title)));
    Ok(articles)
}

fn load_blog(root: &Path) -> Result<Vec<BlogPost>, String> {
    let mut posts = Vec::new();
    for rel in sorted_md_files(root, "blog")? {
        let label = format!("content/{}", rel.display());
        let raw = read_safe_file(root, &rel)?;
        let (mut post, body_md) = parse_document::<BlogPost>(&raw, &label)?;
        let rendered = render_markdown(&body_md);
        post.body_md = body_md;
        post.body_html = rendered.html;
        post.headings = rendered.headings;
        post.internal_links = rendered.internal_links;
        posts.push(post);
    }
    posts.sort_by(|a, b| b.date.cmp(&a.date).then(a.slug.cmp(&b.slug)));
    Ok(posts)
}
