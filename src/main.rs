mod frontmatter;
mod site;
mod templates;

use std::{
    fs,
    path::{Path, PathBuf},
};

const CONTENT_DIR: &str = "content";
const OUTPUT_DIR: &str = "output";
const PUBLIC_DIR: &str = "public";

pub fn to_blog_output_file(path: &PathBuf) -> PathBuf {
    let remainder = path.strip_prefix(CONTENT_DIR).unwrap_or(Path::new(path));
    let mut output_file = Path::new(OUTPUT_DIR).join(remainder);
    output_file.set_extension("html");
    output_file
}

fn write_all_blogs() -> anyhow::Result<Vec<site::BlogLink>> {
    let md_files: Vec<PathBuf> = fs::read_dir(CONTENT_DIR)?
        .flatten()
        .filter_map(|e| {
            if e.path().is_file()
                && !e.path().file_stem().map(|s| s == "index").unwrap_or(false)
                && e.path().extension().map(|ext| ext == "md").unwrap_or(false)
            {
                Some(e.path())
            } else {
                None
            }
        })
        .collect();

    let mut blogs: Vec<site::BlogLink> = Vec::new();

    for md_file in md_files {
        let markdown = fs::read_to_string(&md_file)?;
        let frontmatter = match frontmatter::parse_frontmatter(&markdown) {
            Some(fm) => fm,
            None => {
                eprintln!(
                    "Could not parse frontmatter for {:#?}. Skipping ...",
                    md_file.into_os_string().into_string()
                );
                continue;
            }
        };
        let blog = site::build_blog(&frontmatter, &markdown);
        let output_file = to_blog_output_file(&md_file);

        let href = output_file
            .strip_prefix(OUTPUT_DIR)?
            .to_string_lossy()
            .into_owned();

        fs::write(output_file, blog.into_string())?;

        let frontmatter::Frontmatter { title, date, .. } = frontmatter;

        blogs.push(site::BlogLink { title, date, href });
    }

    Ok(blogs)
}

fn write_index(index: maud::PreEscaped<String>) -> anyhow::Result<()> {
    let output_file = Path::new(OUTPUT_DIR).join("index.html");
    fs::write(output_file, index.into_string())?;
    Ok(())
}

fn copy_public_files() -> anyhow::Result<()> {
    // Copy index.css
    let index_css_path = Path::new(PUBLIC_DIR).join("index.css");
    let index_css_out_path = Path::new(OUTPUT_DIR).join("index.css");
    let _ = fs::copy(index_css_path, index_css_out_path)?;

    // TODO: Copy images
    let images_dir = Path::new(PUBLIC_DIR).join("images");
    let images_out_dir = Path::new(OUTPUT_DIR).join("images");

    let image_files: Vec<PathBuf> = fs::read_dir(images_dir)?
        .flatten()
        .filter_map(|e| {
            if e.path().is_file()
                && e.path()
                    .extension()
                    .map(|ext| ext == "png" || ext == "jpg")
                    .unwrap_or(false)
            {
                Some(e.path())
            } else {
                None
            }
        })
        .collect();

    fs::create_dir(&images_out_dir)?;

    for image_file in &image_files {
        if let Some(image_file_name) = image_file.file_name() {
            let output_file = images_out_dir.join(image_file_name);
            let _ = fs::copy(image_file, output_file)?;
        }
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    if Path::is_dir(Path::new(OUTPUT_DIR)) {
        eprintln!("{} already exists. Removing ...", OUTPUT_DIR);
        fs::remove_dir_all(OUTPUT_DIR)?;
    }

    fs::create_dir(OUTPUT_DIR)?;

    copy_public_files()?;

    let blogs = write_all_blogs()?;
    write_index(site::build_index("Kevin Yu", blogs))?;

    Ok(())
}
