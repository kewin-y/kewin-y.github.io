mod frontmatter;
mod site;
mod templates;

use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Context;

use crate::site::Blog;

const CONTENT_DIR: &str = "content";
const OUTPUT_DIR: &str = "output";
const PUBLIC_DIR: &str = "public";

pub fn to_blog_output_file(path: &PathBuf) -> PathBuf {
    let remainder = path.strip_prefix(CONTENT_DIR).unwrap_or(Path::new(path));
    let mut output_file = Path::new(OUTPUT_DIR).join(remainder);
    output_file.set_extension("html");
    output_file
}

fn is_md_file(path: &Path) -> bool {
    path.is_file() && path.extension().is_some_and(|ext| ext == "md")
}

fn is_image_file(path: &Path) -> bool {
    path.is_file()
        && path
            .extension()
            .is_some_and(|ext| ext == "png" || ext == "jpg")
}

fn write_all_blogs() -> anyhow::Result<Vec<Blog>> {
    let md_files = fs::read_dir(CONTENT_DIR)
        .with_context(|| format!("Failed to read files from {}", CONTENT_DIR))?;

    let mut blogs: Vec<Blog> = Vec::new();

    for entry in md_files {
        let entry = entry?;
        let path = entry.path();

        if !is_md_file(&path) {
            eprintln!("{} is not a markdown file. Skipping ..", path.display());
            continue;
        }
        if path.file_stem().is_some_and(|stem| stem == "index") {
            eprintln!("Skipping file with stem \"index\" ..");
            continue;
        }

        let markdown = fs::read_to_string(&path)
            .with_context(|| format!("Could not read markdown from {}", path.display()))?;
        let frontmatter = frontmatter::parse_frontmatter(&markdown)
            .with_context(|| format!("Could not parse frontmatter for {}", path.display()))?;
        let blog = site::build_blog(&frontmatter, &markdown);
        let output_file = to_blog_output_file(&path);

        // TODO: Probably should do a regex test for slugs
        let href = output_file
            .strip_prefix(OUTPUT_DIR)?
            .to_string_lossy()
            .into_owned();

        fs::write(output_file, blog.into_string())?;

        blogs.push(Blog { frontmatter, href });
    }

    blogs.sort_by(|a, b| b.frontmatter.date.cmp(&a.frontmatter.date));

    Ok(blogs)
}

fn copy_public_files() -> anyhow::Result<()> {
    // Copy index.css
    let index_css_path = Path::new(PUBLIC_DIR).join("index.css");
    let index_css_out_path = Path::new(OUTPUT_DIR).join("index.css");
    fs::copy(&index_css_path, &index_css_out_path).with_context(|| {
        format!(
            "Failed to copy {} to {} ..",
            &index_css_path.display(),
            &index_css_out_path.display()
        )
    })?;

    // Copy images
    let images_dir = Path::new(PUBLIC_DIR).join("images");
    let images_out_dir = Path::new(OUTPUT_DIR).join("images");

    fs::create_dir(&images_out_dir)
        .with_context(|| format!("Failed to create directory {}", images_out_dir.display()))?;

    let image_files = fs::read_dir(&images_dir)
        .with_context(|| format!("Failed to read images from {}", &images_dir.display()))?;

    for entry in image_files {
        let entry = entry?;
        let path = entry.path();

        if !is_image_file(&path) {
            eprintln!(
                "{} is not an image (png or jpg) file. Skipping ..",
                path.display()
            );
            continue;
        }

        if let Some(file_name) = path.file_name() {
            let output_file = images_out_dir.join(file_name);
            fs::copy(&path, &output_file).with_context(|| {
                format!(
                    "Failed to copy {} to {} ..",
                    &path.display(),
                    &output_file.display()
                )
            })?;
        }
    }

    Ok(())
}

fn write_index(index: maud::PreEscaped<String>) -> anyhow::Result<()> {
    let output_file = Path::new(OUTPUT_DIR).join("index.html");
    fs::write(&output_file, index.into_string())
        .with_context(|| format!("Failed to write index file ({})", &output_file.display()))
}

fn main() -> anyhow::Result<()> {
    // Prep
    if Path::is_dir(Path::new(OUTPUT_DIR)) {
        eprintln!("{} already exists. Removing ..", OUTPUT_DIR);
        fs::remove_dir_all(OUTPUT_DIR)?;
    }
    fs::create_dir(OUTPUT_DIR)?;

    // Site Creation
    copy_public_files()?;
    let blogs = write_all_blogs()?;
    write_index(site::build_index("Kevin Yu", blogs))?;

    eprintln!("Succesfully generated site at ./output");

    Ok(())
}
